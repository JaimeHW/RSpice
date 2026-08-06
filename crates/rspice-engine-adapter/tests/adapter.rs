//! End-to-end tests driving the packaged executor exactly as the cloud
//! worker and the release probe do: cleared environment, one request on
//! standard input, one JSON verdict on standard output, results under a
//! pre-created `results/` directory.

use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use rspice_engine_adapter::wire::{
    EngineArtifact, EngineRequest, EngineRevision, digest_hex, revision_content_digest,
    simulation_request_digest,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// The exact deterministic request the release workflow's probe replays; its
/// digests are pinned upstream and must never be regenerated here.
fn release_smoke_request() -> Vec<u8> {
    serde_json::to_vec(&json!({
        "protocol_version": 3,
        "simulation_run_id": "019f76ae-0000-7000-8000-000000000703",
        "circuit_id": "019f76ae-0000-7000-8000-000000000701",
        "attempt": 1,
        "request_digest_version": 1,
        "request_sha256": "62ed89a0528762860da1f9897b6ecd35e0fa5c6e7eb7568f575034f92c6d7d1c",
        "analysis": {"kind": "operating_point"},
        "revision": {
            "id": "019f76ae-0000-7000-8000-000000000702",
            "schema_version": 1,
            "content_digest_version": 2,
            "content_sha256": "88767794c6fcedd647712673b43e3f4c409697cd4e4a133377ba45af4dd25bcc",
            "document": {"components": [], "schema": "rspice-circuit-v1"},
            "artifacts": []
        }
    }))
    .expect("serialize the pinned release smoke request")
}

struct Job {
    root: PathBuf,
}

impl Job {
    /// A fresh working directory shaped like the worker's job sandbox: the
    /// probe pre-creates `artifacts/` and `results/` before launch.
    fn new(name: &str) -> Self {
        let root =
            Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("{name}-{}", std::process::id()));
        if root.exists() {
            std::fs::remove_dir_all(&root).expect("reset job directory");
        }
        std::fs::create_dir_all(root.join("artifacts")).expect("create artifacts directory");
        std::fs::create_dir_all(root.join("results")).expect("create results directory");
        Self { root }
    }

    /// Stages one artifact under its canonical relative path and returns its
    /// validated manifest entry.
    fn stage_artifact(&self, id: Uuid, file_name: &str, content: &str) -> EngineArtifact {
        let directory = self.root.join("artifacts").join(id.to_string());
        std::fs::create_dir_all(&directory).expect("create artifact directory");
        std::fs::write(directory.join("object"), content).expect("write artifact bytes");
        let sha256: [u8; 32] = Sha256::digest(content.as_bytes()).into();
        EngineArtifact {
            id,
            kind: "model_library".to_owned(),
            file_name: Some(file_name.to_owned()),
            content_type: "text/plain".to_owned(),
            sha256: digest_hex(&sha256),
            size_bytes: content.len() as u64,
            path: format!("artifacts/{id}/object"),
        }
    }

    fn run(&self, request: &[u8]) -> Output {
        self.run_with(request, &[], &[])
    }

    fn run_with(&self, request: &[u8], set: &[(&str, &str)], unset: &[&str]) -> Output {
        use std::io::Write as _;

        let mut command = Command::new(env!("CARGO_BIN_EXE_rspice-engine-adapter"));
        command
            .current_dir(&self.root)
            .env_clear()
            .env("RSPICE_ENGINE_PROTOCOL_VERSION", "3")
            .env("RSPICE_ENGINE_INPUT", "stdin-json")
            .env("RSPICE_ENGINE_OUTPUT", "stdout-json")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        // Process creation on Windows needs the system root even under the
        // cleared-environment contract; the worker's Unix launch clears all.
        if let Some(system_root) = std::env::var_os("SystemRoot") {
            command.env("SystemRoot", system_root);
        }
        for (name, value) in set {
            command.env(name, value);
        }
        for name in unset {
            command.env_remove(name);
        }
        let mut child = command.spawn().expect("spawn the adapter");
        child
            .stdin
            .take()
            .expect("adapter stdin")
            .write_all(request)
            .expect("write the request");
        child
            .wait_with_output()
            .expect("collect the adapter output")
    }
}

fn build_request(document: Value, analysis: Value, artifacts: Vec<EngineArtifact>) -> Vec<u8> {
    let circuit_id = Uuid::from_u128(0x11);
    let revision_id = Uuid::from_u128(0x22);
    let revision_digest =
        revision_content_digest(1, &document, &artifacts).expect("revision digest");
    let request_digest =
        simulation_request_digest(circuit_id, revision_id, &revision_digest, &analysis)
            .expect("request digest");
    serde_json::to_vec(&EngineRequest {
        protocol_version: 3,
        simulation_run_id: Uuid::from_u128(0x33),
        circuit_id,
        attempt: 1,
        request_digest_version: Some(1),
        request_sha256: digest_hex(&request_digest),
        analysis,
        revision: EngineRevision {
            id: revision_id,
            schema_version: 1,
            content_digest_version: Some(2),
            content_sha256: digest_hex(&revision_digest),
            document,
            artifacts,
        },
    })
    .expect("serialize the request")
}

fn parse_stdout(output: &Output) -> Value {
    assert!(
        output.status.success(),
        "adapter exited with {:?}; stderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr),
    );
    serde_json::from_slice(&output.stdout).expect("stdout must be one JSON document")
}

fn measurement<'a>(response: &'a Value, name: &str) -> &'a Value {
    response["result_manifest"]["measurements"]
        .as_array()
        .expect("manifest measurements")
        .iter()
        .find(|entry| entry["name"] == name)
        .unwrap_or_else(|| panic!("measurement {name} missing"))
}

#[test]
fn the_release_smoke_request_succeeds_deterministically() {
    let job = Job::new("release-smoke");
    let first = job.run(&release_smoke_request());
    let response = parse_stdout(&first);
    assert_eq!(response["status"], "succeeded");
    assert_eq!(response["result_manifest"]["format"], "rspice-result-v1");
    assert_eq!(
        response["result_manifest"]["analysis_kind"],
        "operating_point"
    );
    assert!(response.get("result_artifacts").is_none());
    assert_eq!(
        std::fs::read_dir(job.root.join("results"))
            .expect("read results")
            .count(),
        0,
        "a metadata-only run must leave results/ empty"
    );

    let second = job.run(&release_smoke_request());
    assert!(second.status.success());
    assert_eq!(
        first.stdout, second.stdout,
        "the same request must produce byte-identical responses"
    );
}

#[test]
fn a_resistive_divider_operating_point_reports_exact_measurements() {
    let job = Job::new("divider-op");
    let deck = "resistive divider\nV1 in 0 DC 10\nR1 in out 1k\nR2 out 0 1k\n.op\n.end\n";
    let request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
        json!({"kind": "operating_point"}),
        Vec::new(),
    );
    let response = parse_stdout(&job.run(&request));
    assert_eq!(response["status"], "succeeded");

    let out = measurement(&response, "v(out)");
    assert_eq!(out["unit"], "V");
    assert_eq!(out["sample_count"], 1);
    assert_eq!(out["series_sha256"], Value::Null);
    let value: f64 = out["value_decimal"]
        .as_str()
        .expect("canonical decimal")
        .parse()
        .expect("decimal parses");
    assert!((value - 5.0).abs() < 1e-9, "v(out) was {value}");
}

#[test]
fn includes_resolve_from_manifested_artifacts_only() {
    let job = Job::new("include-artifact");
    let artifact = job.stage_artifact(Uuid::from_u128(0x44), "leg.lib", "R2 out 0 1k\n");
    let deck =
        "divider with included leg\nV1 in 0 DC 10\nR1 in out 1k\n.include \"leg.lib\"\n.op\n.end\n";
    let document = json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck});

    let bound = build_request(
        document.clone(),
        json!({"kind": "operating_point"}),
        vec![artifact],
    );
    let response = parse_stdout(&job.run(&bound));
    assert_eq!(response["status"], "succeeded");
    let value: f64 = measurement(&response, "v(out)")["value_decimal"]
        .as_str()
        .expect("canonical decimal")
        .parse()
        .expect("decimal parses");
    assert!((value - 5.0).abs() < 1e-9, "v(out) was {value}");

    let unbound = build_request(document, json!({"kind": "operating_point"}), Vec::new());
    let response = parse_stdout(&job.run(&unbound));
    assert_eq!(response["status"], "failed");
    assert_eq!(response["failure_code"], "netlist.include_unresolved");
}

#[test]
fn a_transient_run_declares_and_writes_its_waveform_artifact() {
    let job = Job::new("rc-transient");
    let deck = "rc lowpass step response\n\
                V1 in 0 PULSE(0 1 0 1u 1u 1m 2m)\n\
                R1 in out 1k\n\
                C1 out 0 1u\n\
                .tran 10u 1m\n\
                .end\n";
    let request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
        json!({"kind": "transient"}),
        Vec::new(),
    );
    let response = parse_stdout(&job.run(&request));
    assert_eq!(response["status"], "succeeded", "response: {response}");

    let artifacts = response["result_artifacts"]
        .as_array()
        .expect("declared result artifacts");
    assert_eq!(artifacts.len(), 1);
    assert_eq!(artifacts[0]["path"], "results/transient-1.csv");
    assert_eq!(artifacts[0]["content_type"], "text/csv");
    let written = job.root.join("results").join("transient-1.csv");
    let content = std::fs::read_to_string(written).expect("declared artifact must exist");
    assert!(
        content.starts_with("time,"),
        "csv header: {}",
        &content[..40]
    );

    let out = measurement(&response, "v(out)");
    assert!(out["sample_count"].as_u64().expect("sample count") > 10);
    assert!(out["series_sha256"].is_string());
}

#[test]
fn wrong_analysis_kind_for_the_deck_is_a_bounded_failure() {
    let job = Job::new("kind-mismatch");
    let deck = "divider\nV1 in 0 DC 10\nR1 in out 1k\nR2 out 0 1k\n.op\n.end\n";
    let request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
        json!({"kind": "transient"}),
        Vec::new(),
    );
    let response = parse_stdout(&job.run(&request));
    assert_eq!(response["status"], "failed");
    assert_eq!(response["failure_code"], "analysis.directive_missing");
}

#[test]
fn an_unparseable_deck_is_a_bounded_failure() {
    let job = Job::new("parse-failure");
    let deck = "broken deck\nR1 in\n.op\n.end\n";
    let request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
        json!({"kind": "operating_point"}),
        Vec::new(),
    );
    let response = parse_stdout(&job.run(&request));
    assert_eq!(response["status"], "failed");
    assert_eq!(response["failure_code"], "netlist.parse_error");
}

#[test]
fn launch_contract_violations_exit_nonzero_without_a_response() {
    let job = Job::new("launch-contract");

    let missing_flag = job.run_with(&release_smoke_request(), &[], &["RSPICE_ENGINE_INPUT"]);
    assert_eq!(missing_flag.status.code(), Some(10));
    assert!(missing_flag.stdout.is_empty());

    let delegating = job.run_with(
        &release_smoke_request(),
        &[(
            "RSPICE_ENGINE_SOLVER_PATH",
            "/opt/rspice/worker/bin/rspice-solver",
        )],
        &[],
    );
    assert_eq!(delegating.status.code(), Some(10));
    assert!(delegating.stdout.is_empty());
}

#[test]
fn a_tampered_request_exits_nonzero_without_a_response() {
    let job = Job::new("tampered-request");
    let mut request: Value =
        serde_json::from_slice(&release_smoke_request()).expect("request JSON");
    request["analysis"]["kind"] = json!("transient");
    let output = job.run(&serde_json::to_vec(&request).expect("request bytes"));
    assert_eq!(output.status.code(), Some(12));
    assert!(output.stdout.is_empty());
}

#[test]
fn component_info_states_the_reviewed_identity() {
    let output = Command::new(env!("CARGO_BIN_EXE_rspice-engine-adapter"))
        .arg("component-info")
        .output()
        .expect("run component-info");
    assert!(output.status.success());
    let info: Value = serde_json::from_slice(&output.stdout).expect("component info JSON");
    assert_eq!(info["component"], "rspice-engine-adapter");
    assert_eq!(info["engine_name"], "rspice");
    assert_eq!(info["runtime_mode"], "self_contained");
    assert_eq!(info["protocol_versions"], json!([3]));
}

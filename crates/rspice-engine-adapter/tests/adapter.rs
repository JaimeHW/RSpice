//! End-to-end tests driving the packaged executor exactly as the cloud
//! worker and the release probe do: cleared environment, one request on
//! standard input, one JSON verdict on standard output, results under a
//! pre-created `results/` directory.

use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use rspice_engine_adapter::axis_execution_document::{
    AxisAnalysisKind, AxisAssignmentKind, AxisExecutionDocument,
};
use rspice_engine_adapter::fft_result_document::{
    FFT_RESULT_DOCUMENT_CONTENT_TYPE, FftCompatibilityMode, FftPhysicalType, FftSourceKind,
    FftUnit, TransientFftResultDocument,
};
use rspice_engine_adapter::result_document::{
    AnalogAnalysisKind, AnalogResultDocument, AnalogSignalKind, RESULT_DOCUMENT_CONTENT_TYPE,
    SignalUnit, SignalValues,
};
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

fn typed_result(job: &Job, response: &Value, file_name: &str) -> AnalogResultDocument {
    let path = format!("results/{file_name}");
    let descriptor = response["result_artifacts"]
        .as_array()
        .expect("declared result artifacts")
        .iter()
        .find(|artifact| artifact["path"] == path)
        .unwrap_or_else(|| panic!("typed result descriptor {path} missing"));
    assert_eq!(descriptor["content_type"], RESULT_DOCUMENT_CONTENT_TYPE);
    let content = std::fs::read_to_string(job.root.join(&path)).expect("read typed result");
    AnalogResultDocument::from_json(&content).expect("typed result validates")
}

fn typed_result_at_path(job: &Job, path: &str) -> AnalogResultDocument {
    let content = std::fs::read_to_string(job.root.join(path)).expect("read typed result path");
    AnalogResultDocument::from_json(&content).expect("typed result path validates")
}

fn typed_fft_result(job: &Job, response: &Value, file_name: &str) -> TransientFftResultDocument {
    let path = format!("results/{file_name}");
    let descriptor = response["result_artifacts"]
        .as_array()
        .expect("declared result artifacts")
        .iter()
        .find(|artifact| artifact["path"] == path)
        .unwrap_or_else(|| panic!("typed FFT result descriptor {path} missing"));
    assert_eq!(descriptor["content_type"], FFT_RESULT_DOCUMENT_CONTENT_TYPE);
    let content = std::fs::read_to_string(job.root.join(&path)).expect("read typed FFT result");
    TransientFftResultDocument::from_json(&content).expect("typed FFT result validates")
}

fn signal<'a>(
    document: &'a AnalogResultDocument,
    canonical_name: &str,
) -> &'a rspice_engine_adapter::result_document::SignalDocument {
    document
        .signals
        .iter()
        .find(|signal| signal.canonical_name.eq_ignore_ascii_case(canonical_name))
        .unwrap_or_else(|| panic!("signal {canonical_name} missing"))
}

fn axis_execution(response: &Value) -> AxisExecutionDocument {
    AxisExecutionDocument::from_value(response["result_manifest"]["axis_execution"].clone())
        .expect("strict axis execution contract validates")
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
    assert_eq!(response["result_manifest"]["format"], "rspice-result-v1");

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

    let document = typed_result(&job, &response, "operating_point-1.result.json");
    assert_eq!(document.analysis.kind, AnalogAnalysisKind::OperatingPoint);
    assert_eq!(document.analysis.id, "op-001");
    assert_eq!(document.point_count, 1);
    assert_eq!(signal(&document, "v(out)").unit, Some(SignalUnit::Volt));
    assert_eq!(
        signal(&document, "i(v1)").kind,
        AnalogSignalKind::BranchCurrent
    );
    assert!(
        document
            .signals
            .iter()
            .any(|signal| signal.kind == AnalogSignalKind::DeviceObservable),
        "core DC observables must not be dropped"
    );
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
    assert_eq!(artifacts.len(), 2);
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

    let document = typed_result(&job, &response, "transient-1.result.json");
    assert_eq!(document.analysis.kind, AnalogAnalysisKind::Transient);
    assert_eq!(document.axes[0].unit, Some(SignalUnit::Second));
    assert_eq!(
        signal(&document, "i(v1)").kind,
        AnalogSignalKind::BranchCurrent
    );
}

#[test]
fn invalid_explicit_transient_tmax_is_a_bounded_configuration_failure() {
    let job = Job::new("invalid-transient-tmax");
    let deck = "invalid TMAX\nV1 in 0 1\nR1 in 0 1k\n.tran 1u 1m 0 0\n.end\n";
    let request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
        json!({"kind": "transient"}),
        Vec::new(),
    );
    let response = parse_stdout(&job.run(&request));
    assert_eq!(response["status"], "failed", "response: {response}");
    assert_eq!(response["failure_code"], "analysis.invalid_configuration");
    assert_eq!(
        std::fs::read_dir(job.root.join("results"))
            .expect("read results")
            .count(),
        0
    );
}

#[test]
fn transient_fft_artifacts_preserve_parent_order_ragged_bins_units_and_metrics() {
    let job = Job::new("transient-fft");
    let deck = "typed transient FFT results\n\
                V1 out 0 SIN(0 1 1k)\n\
                R1 out 0 1k\n\
                .options fft fftout=1\n\
                .tran 1u 1m\n\
                .tran 2u 1m\n\
                .fft v(out) np=8 format=unorm window=rect freq=1k\n\
                .fft i(V1) np=16 window=hann freq=1k\n\
                .end\n";
    let request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
        json!({"kind": "transient"}),
        Vec::new(),
    );
    let response = parse_stdout(&job.run(&request));
    assert_eq!(response["status"], "succeeded", "response: {response}");
    assert_eq!(
        response["result_manifest"]["typed_fft_result_schema"],
        json!({"name": "rspice-transient-fft-result", "version": 1})
    );
    assert_eq!(
        response["result_artifacts"]
            .as_array()
            .expect("result artifacts")
            .len(),
        6
    );

    let first = typed_fft_result(&job, &response, "transient-1.fft.result.json");
    let second = typed_fft_result(&job, &response, "transient-2.fft.result.json");
    assert_eq!(first.parent_analysis.id, "tran-001");
    assert_eq!(second.parent_analysis.id, "tran-002");
    for document in [&first, &second] {
        assert_eq!(document.result_count, 2);
        assert_eq!(document.results[0].analysis_id, "fft-001");
        assert_eq!(document.results[1].analysis_id, "fft-002");
        assert_eq!(
            document.results[0].parent_analysis_id,
            document.parent_analysis.id
        );
        assert_eq!(document.results[0].source.kind, FftSourceKind::Probe);
        assert_eq!(document.results[0].source.authored_output, "V(OUT)");
        assert_eq!(
            document.results[0].authored.compatibility_mode,
            FftCompatibilityMode::HspiceCompatible
        );
        assert_eq!(
            document.results[0].signal.physical_type,
            FftPhysicalType::Voltage
        );
        assert_eq!(document.results[0].signal.unit, Some(FftUnit::Volt));
        assert_eq!(
            document.results[0]
                .metrics
                .as_ref()
                .expect("unnormalized metrics")
                .units
                .fundamental_magnitude,
            Some(FftUnit::Volt)
        );
        assert_eq!(
            document.results[1].signal.physical_type,
            FftPhysicalType::Current
        );
        assert_eq!(
            document.results[1].signal.unit,
            Some(FftUnit::Dimensionless)
        );
        assert_eq!(
            document.results[1]
                .metrics
                .as_ref()
                .expect("normalized metrics")
                .units
                .fundamental_magnitude,
            Some(FftUnit::Dimensionless)
        );
        assert_eq!(document.results[0].spectrum.bins.len(), 5);
        assert_eq!(document.results[1].spectrum.bins.len(), 9);
        assert!(
            document
                .results
                .iter()
                .all(|result| result.metrics.is_some())
        );
    }
}

#[test]
fn failed_fft_execution_publishes_no_waveform_fft_or_staging_artifact() {
    let job = Job::new("transient-fft-failure");
    let deck = "unresolvable transient FFT output\n\
                V1 out 0 SIN(0 1 1k)\n\
                R1 out 0 1k\n\
                .tran 1u 1m\n\
                .fft v(missing) np=8\n\
                .end\n";
    let request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
        json!({"kind": "transient"}),
        Vec::new(),
    );
    let response = parse_stdout(&job.run(&request));
    assert_eq!(response["status"], "failed", "response: {response}");
    let entries = std::fs::read_dir(job.root.join("results"))
        .expect("read empty results directory")
        .collect::<Result<Vec<_>, _>>()
        .expect("collect results directory");
    assert!(
        entries.is_empty(),
        "failed run published artifacts: {entries:?}"
    );
}

#[test]
fn dc_result_preserves_axis_voltage_current_and_device_observables() {
    let job = Job::new("divider-dc");
    let deck = "divider sweep\nV1 in 0 DC 0\nR1 in out 1k\nR2 out 0 1k\n.dc V1 0 1 0.5\n.end\n";
    let request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
        json!({"kind": "dc_sweep"}),
        Vec::new(),
    );
    let response = parse_stdout(&job.run(&request));
    assert_eq!(response["status"], "succeeded", "response: {response}");
    let document = typed_result(&job, &response, "dc_sweep-1.result.json");
    assert_eq!(document.analysis.kind, AnalogAnalysisKind::DcSweep);
    assert_eq!(document.point_count, 3);
    assert_eq!(document.axes[0].unit, Some(SignalUnit::Volt));
    assert!(matches!(
        signal(&document, "v(out)").values,
        SignalValues::Real { .. }
    ));
    assert_eq!(signal(&document, "i(v1)").unit, Some(SignalUnit::Ampere));
    assert!(
        document
            .signals
            .iter()
            .any(|signal| signal.kind == AnalogSignalKind::DeviceObservable)
    );
}

#[test]
fn ac_result_preserves_complex_voltage_and_branch_current() {
    let job = Job::new("rc-ac");
    let deck = "rc ac\nV1 in 0 DC 0 AC 1\nR1 in out 1k\nC1 out 0 1u\n.ac LIN 2 1k 2k\n.end\n";
    let request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
        json!({"kind": "ac_small_signal"}),
        Vec::new(),
    );
    let response = parse_stdout(&job.run(&request));
    assert_eq!(response["status"], "succeeded", "response: {response}");
    let document = typed_result(&job, &response, "ac_small_signal-1.result.json");
    assert_eq!(document.analysis.kind, AnalogAnalysisKind::AcSmallSignal);
    assert!(matches!(
        signal(&document, "v(out)").values,
        SignalValues::Complex { .. }
    ));
    assert!(matches!(
        signal(&document, "i(v1)").values,
        SignalValues::Complex { .. }
    ));
    let SignalValues::Complex { samples } = &signal(&document, "v(out)").values else {
        unreachable!()
    };
    assert!(
        samples
            .iter()
            .flatten()
            .any(|sample| sample.imaginary != 0.0)
    );
}

#[test]
fn noise_result_preserves_complex_solution_densities_and_contributors() {
    let job = Job::new("divider-noise");
    let deck = "divider noise\nV1 in 0 DC 0 AC 1\nR1 in out 1k\nR2 out 0 1k\n.noise V(out) V1 LIN 2 1k 2k\n.end\n";
    let request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
        json!({"kind": "noise"}),
        Vec::new(),
    );
    let response = parse_stdout(&job.run(&request));
    assert_eq!(response["status"], "succeeded", "response: {response}");
    let document = typed_result(&job, &response, "noise-1.result.json");
    assert_eq!(document.analysis.kind, AnalogAnalysisKind::Noise);
    assert!(matches!(
        signal(&document, "v(out)").values,
        SignalValues::Complex { .. }
    ));
    assert_eq!(
        signal(&document, "output_noise_density").unit,
        Some(SignalUnit::VoltSquaredPerHertz)
    );
    assert!(document.signals.iter().any(|signal| {
        signal.canonical_name.starts_with("noise(r1)")
            && signal.canonical_name.ends_with("output_density")
    }));
}

#[test]
fn step_wraps_transient_with_canonical_namespaces_and_no_extra_op() {
    let job = Job::new("step-transient");
    let deck = "stepped transient\n\
                .param rval=1k\n\
                V1 in 0 PULSE(0 1 0 1u 1u 20u 50u)\n\
                R1 in out {rval}\n\
                C1 out 0 1n\n\
                .step param rval list 1k 2k\n\
                .tran 1u 20u\n\
                .end\n";
    let request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
        json!({"kind": "transient"}),
        Vec::new(),
    );
    let response = parse_stdout(&job.run(&request));
    assert_eq!(response["status"], "succeeded", "response: {response}");
    assert_eq!(response["result_manifest"]["format"], "rspice-result-v2");
    assert_eq!(
        response["result_manifest"]["measurements"],
        json!([]),
        "axis measurements must live under their coordinate/analysis provenance"
    );
    let execution = axis_execution(&response);
    assert_eq!(execution.analysis_kind, AxisAnalysisKind::Transient);
    assert_eq!(execution.coordinate_count, 2);
    assert_eq!(execution.execution_count, 2);

    let mut coordinate_ids = std::collections::HashSet::new();
    let mut paths = std::collections::HashSet::new();
    for run in &execution.runs {
        assert!(coordinate_ids.insert(run.coordinate_id.clone()));
        assert_eq!(run.assignments.len(), 1);
        assert_eq!(run.assignments[0].kind, AxisAssignmentKind::Step);
        assert_eq!(run.analyses.len(), 1, "no implicit OP may be added");
        assert_eq!(run.analyses[0].analysis_id, "tran-001");
        assert!(!run.analyses[0].measurements.is_empty());
        for path in &run.analyses[0].artifacts {
            assert!(paths.insert(path.clone()));
            assert!(path.contains(&run.coordinate_namespace));
            assert!(path.contains("tran-001"));
        }
    }
    assert_eq!(paths.len(), 4);
}

#[test]
fn temp_wraps_ac_and_repeated_directives_keep_stable_ordered_ids() {
    let job = Job::new("temp-repeated-ac");
    let deck = "temperature AC\n\
                V1 in 0 AC 1\n\
                R1 in out 1k\n\
                C1 out 0 1u\n\
                .temp 25 75\n\
                .ac LIN 2 1k 2k\n\
                .ac LIN 3 2k 4k\n\
                .end\n";
    let request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
        json!({"kind": "ac_small_signal"}),
        Vec::new(),
    );
    let response = parse_stdout(&job.run(&request));
    assert_eq!(response["status"], "succeeded", "response: {response}");
    let execution = axis_execution(&response);
    assert_eq!(execution.analysis_kind, AxisAnalysisKind::AcSmallSignal);
    assert_eq!(execution.coordinate_count, 2);
    assert_eq!(execution.execution_count, 4);
    let mut artifacts = std::collections::HashSet::new();
    for run in &execution.runs {
        assert_eq!(run.assignments[0].kind, AxisAssignmentKind::Temperature);
        assert_eq!(
            run.analyses
                .iter()
                .map(|analysis| analysis.analysis_id.as_str())
                .collect::<Vec<_>>(),
            ["ac-001", "ac-002"]
        );
        for analysis in &run.analyses {
            for path in &analysis.artifacts {
                assert!(artifacts.insert(path.clone()), "duplicate path {path}");
            }
        }
    }
    assert_eq!(artifacts.len(), 8);
}

#[test]
fn dc_step_and_noise_temperature_axes_retain_every_coordinate() {
    let dc_job = Job::new("step-dc");
    let dc_deck = "stepped DC\n\
                   .param load=1k\n\
                   V1 in 0 0\n\
                   R1 in 0 {load}\n\
                   .step param load list 1k 2k\n\
                   .dc V1 0 1 1\n\
                   .end\n";
    let dc_request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8": dc_deck}),
        json!({"kind": "dc_sweep"}),
        Vec::new(),
    );
    let dc_response = parse_stdout(&dc_job.run(&dc_request));
    assert_eq!(dc_response["status"], "succeeded", "{dc_response}");
    let dc_execution = axis_execution(&dc_response);
    assert_eq!(dc_execution.analysis_kind, AxisAnalysisKind::DcSweep);
    assert_eq!(dc_execution.coordinate_count, 2);
    assert!(
        dc_execution
            .runs
            .iter()
            .all(|run| run.analyses[0].analysis_id == "dc-001")
    );

    let noise_job = Job::new("temp-noise");
    let noise_deck = "temperature noise\n\
                      V1 in 0 AC 1\n\
                      R1 in out 1k\n\
                      R2 out 0 1k\n\
                      .temp 0 100\n\
                      .noise V(out) V1 LIN 1 1k 1k\n\
                      .end\n";
    let noise_request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8": noise_deck}),
        json!({"kind": "noise"}),
        Vec::new(),
    );
    let noise_response = parse_stdout(&noise_job.run(&noise_request));
    assert_eq!(noise_response["status"], "succeeded", "{noise_response}");
    let noise_execution = axis_execution(&noise_response);
    assert_eq!(noise_execution.analysis_kind, AxisAnalysisKind::Noise);
    assert_eq!(noise_execution.coordinate_count, 2);
    let densities = noise_execution
        .runs
        .iter()
        .map(|run| {
            let path = run.analyses[0]
                .artifacts
                .iter()
                .find(|path| path.ends_with(".result.json"))
                .expect("typed noise artifact");
            let document = typed_result_at_path(&noise_job, path);
            let SignalValues::Real { samples } = &signal(&document, "output_noise_density").values
            else {
                panic!("noise density must be real")
            };
            samples[0].expect("noise density sample")
        })
        .collect::<Vec<_>>();
    assert!(
        densities[1] > densities[0],
        "thermal noise must increase with TEMP: {densities:?}"
    );
}

#[test]
fn step_only_operating_point_executes_the_canonical_implicit_op() {
    let job = Job::new("step-implicit-op");
    let deck = "implicit stepped OP\n\
                .param rval=1k\n\
                V1 in 0 1\n\
                R1 in 0 {rval}\n\
                .step param rval list 1k 2k\n\
                .end\n";
    let request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
        json!({"kind": "operating_point"}),
        Vec::new(),
    );
    let response = parse_stdout(&job.run(&request));
    assert_eq!(response["status"], "succeeded", "response: {response}");
    let execution = axis_execution(&response);
    assert_eq!(execution.analysis_kind, AxisAnalysisKind::OperatingPoint);
    assert!(execution.runs.iter().all(|run| {
        run.analyses.len() == 1 && run.analyses[0].analysis_id == "implicit-op-001"
    }));
}

#[test]
fn axisless_deck_without_analysis_executes_implicit_op_in_scalar_v1_shape() {
    let job = Job::new("scalar-implicit-op");
    let deck = "implicit scalar OP\nV1 in 0 1\nR1 in 0 1k\n.end\n";
    let request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
        json!({"kind": "operating_point"}),
        Vec::new(),
    );
    let response = parse_stdout(&job.run(&request));
    assert_eq!(response["status"], "succeeded", "response: {response}");
    assert_eq!(response["result_manifest"]["format"], "rspice-result-v1");
    assert!(response["result_manifest"].get("axis_execution").is_none());
    assert!(
        response["result_artifacts"]
            .as_array()
            .expect("scalar artifacts")
            .iter()
            .any(|artifact| artifact["path"] == "results/operating_point-1.result.json")
    );
}

#[test]
fn conditional_topology_and_analysis_signature_fail_closed_without_artifacts() {
    for (name, deck, failure_code) in [
        (
            "conditional-topology",
            "conditional topology\n\
             .param mode=0\n\
             V1 in 0 1\n\
             R1 in out 1k\n\
             .if (mode==1)\n\
             R2 out 0 2k\n\
             .else\n\
             R3 out 0 3k\n\
             R4 out 0 4k\n\
             .endif\n\
             .step param mode list 0 1\n\
             .op\n\
             .end\n",
            "results.conditional_topology",
        ),
        (
            "conditional-analysis",
            "conditional analysis\n\
             .param mode=0\n\
             V1 in 0 AC 1\n\
             R1 in 0 1k\n\
             .if (mode==0)\n\
             .ac LIN 2 1k 2k\n\
             .else\n\
             .tran 1u 10u\n\
             .endif\n\
             .step param mode list 0 1\n\
             .end\n",
            // The materializer's own mismatch is typed by the engine now, so it
            // arrives with the engine's category rather than a coarse adapter code.
            "engine.materialization_mismatch",
        ),
    ] {
        let job = Job::new(name);
        let request = build_request(
            json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
            if name == "conditional-topology" {
                json!({"kind": "operating_point"})
            } else {
                json!({"kind": "ac_small_signal"})
            },
            Vec::new(),
        );
        let response = parse_stdout(&job.run(&request));
        assert_eq!(response["status"], "failed", "response: {response}");
        assert_eq!(
            response["failure_code"], failure_code,
            "response: {response}"
        );
        assert_eq!(
            std::fs::read_dir(job.root.join("results"))
                .expect("read results")
                .count(),
            0
        );
    }
}

#[test]
fn alter_and_mixed_signal_axes_are_explicitly_unsupported() {
    for (name, deck, kind, failure_code) in [
        (
            "alter-axis",
            "ALTER deck
V1 in 0 1
R1 in 0 1k
.op
.alter second
R1 in 0 2k
.end
",
            "operating_point",
            // Source-variant expansion is a capability the engine declines, and
            // the wire code says so instead of naming an adapter-local axis rule.
            "engine.unsupported_capability",
        ),
        (
            "mixed-axis",
            "mixed axis
.param r=1k
V1 in 0 1
R1 in 0 {r}
.step param r list 1k 2k
.tran 1u 10u
.end
",
            "mixed_signal",
            // The adapter's own contract refuses this axis before the engine sees it.
            "analysis.axis_unsupported",
        ),
    ] {
        let job = Job::new(name);
        let request = build_request(
            json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
            json!({"kind": kind}),
            Vec::new(),
        );
        let response = parse_stdout(&job.run(&request));
        assert_eq!(response["status"], "failed", "response: {response}");
        assert_eq!(response["failure_code"], failure_code, "response: {response}");
    }
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

/// A node with no DC path to ground has no operating point to report. The
/// solver's conditioning shunt can always produce a number for it, so the
/// refusal has to be explicit, and it has to reach the wire as a stable code
/// rather than as a success carrying a fabricated bias.
#[test]
fn a_node_with_no_dc_path_to_ground_is_a_bounded_failure() {
    let job = Job::new("no-dc-path");
    let deck = "floating node\nI1 0 out DC 1m\nC1 out 0 1u\n.op\n.end\n";
    let request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
        json!({"kind": "operating_point"}),
        Vec::new(),
    );
    let response = parse_stdout(&job.run(&request));
    assert_eq!(response["status"], "failed");
    assert_eq!(response["failure_code"], "engine.circuit_error");
}

/// The same deck with ngspice's `RSHUNT` shunt resistor runs, because the
/// shunt is a real element the author sized: 1 mA through 1 GOhm is 1 MV.
#[test]
fn rshunt_restores_the_dc_path_and_the_run_succeeds() {
    let job = Job::new("rshunt");
    let deck = "shunted floating node\n\
                I1 0 out DC 1m\nC1 out 0 1u\n.options rshunt=1e9\n.op\n.end\n";
    let request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
        json!({"kind": "operating_point"}),
        Vec::new(),
    );
    let response = parse_stdout(&job.run(&request));
    assert_eq!(response["status"], "succeeded");
    assert_eq!(
        measurement(&response, "v(out)")["value_decimal"],
        "9.999999999999999e5"
    );
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
    assert_eq!(
        info["result_schemas"],
        json!([
            "rspice-analog-result-v1",
            "rspice-transient-fft-result-v1",
            "rspice-axis-execution-v1"
        ])
    );
}

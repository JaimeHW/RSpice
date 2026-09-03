//! End-to-end tests driving the packaged executor exactly as the cloud
//! worker and the release probe do: cleared environment, one request on
//! standard input, one JSON verdict on standard output, results under a
//! pre-created `results/` directory.

use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use rspice_core::abort_signal::ImmediateAbort;
use rspice_core::execution::result_document::{ScalarValue, SeriesValues};
use rspice_core::execution::{
    ANALYSIS_RESULT_DOCUMENT_SCHEMA, ANALYSIS_RESULT_DOCUMENT_VERSION, AnalysisResultDocument,
    AnalysisResultKind, MappingStatus, ResultSignal, SignalUnit, analysis_result_capability,
};
use rspice_engine_adapter::axis_execution_document::{
    AxisAnalysisKind, AxisAssignmentKind, AxisExecutionDocument,
};
use rspice_engine_adapter::document::CircuitContent;
use rspice_engine_adapter::execute::{
    CANCELLED_FAILURE_CODE, DEFAULT_SOLVE_BUDGET, execute_with_abort,
};
use rspice_engine_adapter::fft_result_document::{
    FFT_RESULT_DOCUMENT_CONTENT_TYPE, FftCompatibilityMode, FftPhysicalType, FftSourceKind,
    FftUnit, TransientFftResultDocument,
};
use rspice_engine_adapter::result_artifact::result_document_content_type;
use rspice_engine_adapter::wire::{
    EngineArtifact, EngineRequest, EngineResponse, EngineRevision,
    INTEGRITY_ENGINE_PROTOCOL_VERSION, digest_hex, revision_content_digest,
    simulation_request_digest,
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// The exact deterministic request the release workflow's probe replays; its
/// digests are pinned upstream and must never be regenerated here.
fn release_smoke_request() -> Vec<u8> {
    serde_json::to_vec(&json!({
        "protocol_version": INTEGRITY_ENGINE_PROTOCOL_VERSION,
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
            .env(
                "RSPICE_ENGINE_PROTOCOL_VERSION",
                INTEGRITY_ENGINE_PROTOCOL_VERSION.to_string(),
            )
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

    /// Run one deck under one analysis kind and return the parsed response.
    fn execute(&self, deck: &str, kind: &str) -> Value {
        let request = build_request(
            json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
            json!({"kind": kind}),
            Vec::new(),
        );
        parse_stdout(&self.run(&request))
    }

    fn results_are_empty(&self) -> bool {
        std::fs::read_dir(self.root.join("results"))
            .expect("read results")
            .count()
            == 0
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
        protocol_version: INTEGRITY_ENGINE_PROTOCOL_VERSION,
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

fn typed_result(job: &Job, response: &Value, file_name: &str) -> AnalysisResultDocument {
    let path = format!("results/{file_name}");
    let descriptor = response["result_artifacts"]
        .as_array()
        .expect("declared result artifacts")
        .iter()
        .find(|artifact| artifact["path"] == path)
        .unwrap_or_else(|| panic!("typed result descriptor {path} missing from {response}"));
    assert_eq!(descriptor["content_type"], result_document_content_type());
    typed_result_at_path(job, &path)
}

fn typed_result_at_path(job: &Job, path: &str) -> AnalysisResultDocument {
    let content = std::fs::read_to_string(job.root.join(path))
        .unwrap_or_else(|error| panic!("read typed result {path}: {error}"));
    AnalysisResultDocument::from_json(&content).expect("typed result validates")
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

fn signal<'a>(document: &'a AnalysisResultDocument, canonical_name: &str) -> &'a ResultSignal {
    document
        .signals()
        .iter()
        .find(|signal| {
            signal
                .descriptor()
                .canonical_name()
                .eq_ignore_ascii_case(canonical_name)
        })
        .unwrap_or_else(|| {
            panic!(
                "signal {canonical_name} missing; document has {:?}",
                document
                    .signals()
                    .iter()
                    .map(|signal| signal.descriptor().canonical_name())
                    .collect::<Vec<_>>()
            )
        })
}

fn scalar_value(document: &AnalysisResultDocument, name: &str) -> ScalarValue {
    document
        .scalars()
        .iter()
        .find(|scalar| scalar.name().eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("scalar {name} missing"))
        .value()
        .clone()
}

fn axis_execution(response: &Value) -> AxisExecutionDocument {
    AxisExecutionDocument::from_value(response["result_manifest"]["axis_execution"].clone())
        .expect("strict axis execution contract validates")
}

//=============================================================================
// Family corpus: the registry is the test input
//=============================================================================

/// Which registry declaration a family is expected to carry, without pinning
/// the note text a reviewer may reword.
#[derive(Debug, PartialEq, Eq)]
enum DeclaredStatus {
    Mapped,
    Partial,
    Unsupported,
}

fn declared_status(status: MappingStatus) -> DeclaredStatus {
    match status {
        MappingStatus::Mapped => DeclaredStatus::Mapped,
        MappingStatus::Partial(_) => DeclaredStatus::Partial,
        MappingStatus::Unsupported(_) => DeclaredStatus::Unsupported,
    }
}

/// What this build declares for one core result family.
enum FamilyExpectation {
    /// A deck that exercises the family, the wire kind that selects it, and
    /// the canonical analysis tag its result must carry.
    Runs {
        request_kind: &'static str,
        analysis_tag: &'static str,
        deck: &'static str,
        /// `Partial` when the family runs but a documented subset of results
        /// cannot be published; the registry note says which.
        declared: DeclaredStatus,
    },
    /// The family is published as a typed child of another family's result
    /// rather than as its own request, so the registry declares it partial.
    Attached {
        request_kind: &'static str,
        parent_request_kind: &'static str,
        parent_artifact: &'static str,
        deck: &'static str,
    },
    /// The family is refused by name, and the refusal explains why.
    Refused { request_kind: &'static str },
}

const DIVIDER: &str = "resistive divider\n\
                       V1 in 0 DC 10\n\
                       R1 in out 1k\n\
                       R2 out 0 1k\n";
const RC: &str = "rc lowpass\n\
                  V1 in 0 DC 0 AC 1\n\
                  R1 in out 1k\n\
                  C1 out 0 1u\n";
const RF: &str = "periodic fixture\n\
                  V1 in 0 SIN(0 1 1G)\n\
                  R1 in out 1k\n\
                  C1 out 0 1p\n";

/// One deck plus refusal per family, matched exhaustively so a new core
/// result family cannot ship without an adapter decision recorded here.
fn family_expectation(kind: AnalysisResultKind) -> FamilyExpectation {
    match kind {
        AnalysisResultKind::OperatingPoint => FamilyExpectation::Runs {
            request_kind: "operating_point",
            analysis_tag: "op-001",
            declared: DeclaredStatus::Mapped,
            deck: "resistive divider\nV1 in 0 DC 10\nR1 in out 1k\nR2 out 0 1k\n.op\n.end\n",
        },
        AnalysisResultKind::DcSweep => FamilyExpectation::Runs {
            request_kind: "dc_sweep",
            analysis_tag: "dc-001",
            declared: DeclaredStatus::Mapped,
            deck: "divider sweep\nV1 in 0 DC 0\nR1 in out 1k\nR2 out 0 1k\n.dc V1 0 1 0.5\n.end\n",
        },
        AnalysisResultKind::Ac => FamilyExpectation::Runs {
            request_kind: "ac_small_signal",
            analysis_tag: "ac-001",
            declared: DeclaredStatus::Mapped,
            deck: "rc ac\nV1 in 0 DC 0 AC 1\nR1 in out 1k\nC1 out 0 1u\n.ac LIN 2 1k 2k\n.end\n",
        },
        AnalysisResultKind::Transient => FamilyExpectation::Runs {
            request_kind: "transient",
            analysis_tag: "tran-001",
            declared: DeclaredStatus::Mapped,
            deck: "rc transient\nV1 in 0 PULSE(0 1 0 1u 1u 1m 2m)\nR1 in out 1k\nC1 out 0 1u\n\
                   .tran 10u 1m\n.end\n",
        },
        AnalysisResultKind::Noise => FamilyExpectation::Runs {
            request_kind: "noise",
            analysis_tag: "noise-001",
            declared: DeclaredStatus::Mapped,
            deck: "divider noise\nV1 in 0 DC 0 AC 1\nR1 in out 1k\nR2 out 0 1k\n\
                   .noise V(out) V1 LIN 2 1k 2k\n.end\n",
        },
        AnalysisResultKind::Distortion => FamilyExpectation::Runs {
            request_kind: "distortion",
            analysis_tag: "disto-001",
            declared: DeclaredStatus::Mapped,
            deck: "diode distortion\nV1 out 0 DC 0.5 DISTOF1 1m 0\nD1 out 0 DM\n\
                   .model DM D(IS=1e-12 N=1 CJO=0 TT=0)\n.disto DEC 2 1k 10k\n.end\n",
        },
        AnalysisResultKind::TransferFunction => FamilyExpectation::Runs {
            request_kind: "transfer_function",
            analysis_tag: "tf-001",
            declared: DeclaredStatus::Mapped,
            deck: "divider transfer function\nV1 in 0 DC 10\nR1 in out 1k\nR2 out 0 1k\n\
                   .tf V(out) V1\n.end\n",
        },
        AnalysisResultKind::Stability => FamilyExpectation::Runs {
            request_kind: "stability",
            analysis_tag: "stb-001",
            // A three-pole loop crosses -180 degrees, so both margins are
            // finite. The registry records why an unconditionally stable loop
            // cannot be published.
            declared: DeclaredStatus::Partial,
            deck: "three-pole loop\nE1 eo 0 ctrl 0 -1000\nVPROBE eo x 0\n\
                   R1 x n1 1k\nC1 n1 0 159.154943091895n\n\
                   R2 n1 n2 1k\nC2 n2 0 159.154943091895n\n\
                   R3 n2 ctrl 1k\nC3 ctrl 0 159.154943091895n\n\
                   .stb dec 5 10 10meg probe=vprobe\n.end\n",
        },
        AnalysisResultKind::Sensitivity => FamilyExpectation::Runs {
            request_kind: "sensitivity",
            analysis_tag: "sens-001",
            declared: DeclaredStatus::Mapped,
            deck: "divider sensitivity\nV1 in 0 DC 10\nR1 in out 1k\nR2 out 0 1k\n\
                   .sens V(out)\n.end\n",
        },
        AnalysisResultKind::PoleZero => FamilyExpectation::Runs {
            request_kind: "pole_zero",
            analysis_tag: "pz-001",
            declared: DeclaredStatus::Mapped,
            deck: "rc pole zero\nV1 in 0 DC 0 AC 1\nR1 in out 1k\nC1 out 0 1u\n\
                   .pz in 0 out 0 vol pz\n.end\n",
        },
        AnalysisResultKind::MonteCarlo => FamilyExpectation::Runs {
            request_kind: "monte_carlo",
            analysis_tag: "mc-001",
            declared: DeclaredStatus::Mapped,
            deck: "divider monte carlo\nV1 in 0 DC 10\nR1 in out 1k\nR2 out 0 1k\n\
                   .mc 3 SEED 7 GAUSS 0.01\n.end\n",
        },
        AnalysisResultKind::HarmonicBalance => FamilyExpectation::Runs {
            request_kind: "harmonic_balance",
            analysis_tag: "hb-001",
            declared: DeclaredStatus::Mapped,
            deck: "rf harmonic balance\nV1 in 0 SIN(0 1 1G)\nR1 in out 1k\nC1 out 0 1p\n\
                   .hb 1g\n.end\n",
        },
        AnalysisResultKind::Pss => FamilyExpectation::Runs {
            request_kind: "pss",
            analysis_tag: "pss-001",
            declared: DeclaredStatus::Mapped,
            deck: "rf periodic steady state\nV1 in 0 SIN(0 1 1G)\nR1 in out 1k\nC1 out 0 1p\n\
                   .pss fund=1g\n.end\n",
        },
        AnalysisResultKind::Pac => FamilyExpectation::Runs {
            request_kind: "pac",
            analysis_tag: "pac-001",
            declared: DeclaredStatus::Mapped,
            deck: "rf periodic ac\nV1 in 0 SIN(0 1 1G)\nR1 in out 1k\nC1 out 0 1p\n\
                   .pss fund=1g\n.pac dec 2 1k 10k input=v1 out=v(out)\n.end\n",
        },
        AnalysisResultKind::Envelope => FamilyExpectation::Runs {
            request_kind: "envelope",
            analysis_tag: "env-001",
            declared: DeclaredStatus::Mapped,
            deck: "rf envelope\nV1 in 0 SIN(0 1 1G)\nR1 in out 1k\nC1 out 0 1p\n\
                   .hb 1g\n.envelope tstop=1n\n.end\n",
        },
        AnalysisResultKind::SParameters => FamilyExpectation::Refused {
            request_kind: "s_parameters",
        },
        AnalysisResultKind::PortNoise => FamilyExpectation::Refused {
            request_kind: "port_noise",
        },
        AnalysisResultKind::PNoise => FamilyExpectation::Refused {
            request_kind: "pnoise",
        },
        AnalysisResultKind::Fourier => FamilyExpectation::Refused {
            request_kind: "fourier",
        },
        AnalysisResultKind::Fft => FamilyExpectation::Attached {
            request_kind: "fft",
            parent_request_kind: "transient",
            parent_artifact: "tran-001.fft.result.json",
            deck: "attached transient FFT\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n\
                   .tran 1u 1m\n.fft v(out) np=8 freq=1k\n.end\n",
        },
    }
}

/// The registry declaration for a family and what this build actually does
/// must agree, family by family, with no wildcard arm on either side.
#[test]
fn every_result_family_matches_its_engine_adapter_capability_declaration() {
    for kind in AnalysisResultKind::ALL {
        let declared = analysis_result_capability(kind).engine_adapter;
        match family_expectation(kind) {
            FamilyExpectation::Runs {
                request_kind,
                analysis_tag,
                deck,
                declared: expected,
            } => {
                assert_eq!(
                    declared_status(declared.scalar),
                    expected,
                    "{kind:?} runs but the registry declares {:?}",
                    declared.scalar
                );
                let job = Job::new(&format!("family-{}", kind.tag()));
                let response = job.execute(deck, request_kind);
                assert_eq!(
                    response["status"], "succeeded",
                    "{kind:?} deck failed: {response}"
                );
                let document =
                    typed_result(&job, &response, &format!("{analysis_tag}.result.json"));
                assert_eq!(
                    document.result_kind(),
                    kind,
                    "{kind:?} published a {:?} document",
                    document.result_kind()
                );
                assert_eq!(document.analysis().tag(), analysis_tag);
                assert_eq!(document.schema(), ANALYSIS_RESULT_DOCUMENT_SCHEMA);
                assert_eq!(document.schema_version(), ANALYSIS_RESULT_DOCUMENT_VERSION);
                assert!(
                    document.topology_fingerprint().is_some(),
                    "{kind:?} published no topology identity"
                );
                let namespaces = document
                    .namespaces()
                    .unwrap_or_else(|| panic!("{kind:?} published no namespaces"));
                assert!(namespaces.output.contains(analysis_tag), "{namespaces:?}");
                assert!(
                    namespaces.checkpoint.contains(analysis_tag),
                    "{namespaces:?}"
                );
                for axis in document.axes() {
                    assert_eq!(axis.values().len(), document.point_count());
                }
                for signal in document.signals() {
                    assert_eq!(signal.values().len(), document.point_count());
                    assert_eq!(
                        signal.descriptor().value_type(),
                        match signal.values() {
                            SeriesValues::Real { .. } =>
                                rspice_core::execution::SignalValueType::Real,
                            SeriesValues::Complex { .. } =>
                                rspice_core::execution::SignalValueType::Complex,
                            SeriesValues::Logic { .. } =>
                                rspice_core::execution::SignalValueType::Logic,
                        },
                        "{kind:?} signal descriptor and samples disagree"
                    );
                }
            }
            FamilyExpectation::Attached {
                request_kind,
                parent_request_kind,
                parent_artifact,
                deck,
            } => {
                assert_eq!(
                    declared_status(declared.scalar),
                    DeclaredStatus::Partial,
                    "{kind:?} is published attached but the registry declares {:?}",
                    declared.scalar
                );
                let refusal_job = Job::new(&format!("attached-refusal-{}", kind.tag()));
                let refusal = refusal_job.execute(&format!("{DIVIDER}.op\n.end\n"), request_kind);
                assert_eq!(refusal["status"], "failed", "{kind:?}: {refusal}");
                assert_eq!(refusal["failure_code"], "analysis.unsupported_kind");

                let job = Job::new(&format!("attached-{}", kind.tag()));
                let response = job.execute(deck, parent_request_kind);
                assert_eq!(
                    response["status"], "succeeded",
                    "{kind:?} parent deck failed: {response}"
                );
                let document = typed_fft_result(&job, &response, parent_artifact);
                assert!(!document.results.is_empty());
            }
            FamilyExpectation::Refused { request_kind } => {
                assert_eq!(
                    declared_status(declared.scalar),
                    DeclaredStatus::Unsupported,
                    "{kind:?} is refused but the registry declares {:?}",
                    declared.scalar
                );
                let job = Job::new(&format!("refused-{}", kind.tag()));
                let response = job.execute(&format!("{DIVIDER}.op\n.end\n"), request_kind);
                assert_eq!(
                    response["status"], "failed",
                    "{kind:?} must be refused: {response}"
                );
                assert_eq!(response["failure_code"], "analysis.unsupported_kind");
                let detail = response["failure_detail"]
                    .as_str()
                    .expect("a refusal explains itself");
                assert!(
                    detail.len() > 40,
                    "{kind:?} refusal must name the missing contract: {detail}"
                );
                assert!(job.results_are_empty());
            }
        }
    }
}

/// Every family that runs also runs at every coordinate of a `.STEP` and a
/// `.TEMP` axis, publishing one typed document per coordinate under the
/// canonical namespace the planner assigned.
#[test]
fn every_runnable_family_executes_at_every_step_and_temperature_coordinate() {
    for kind in AnalysisResultKind::ALL {
        let FamilyExpectation::Runs {
            request_kind,
            analysis_tag,
            deck,
            ..
        } = family_expectation(kind)
        else {
            continue;
        };
        for (label, axis_card, axis_kind) in [
            (
                "step",
                ".param axisparam=1\n.step param axisparam list 1 2\n",
                AxisAssignmentKind::Step,
            ),
            ("temp", ".temp 25 75\n", AxisAssignmentKind::Temperature),
        ] {
            let axis_deck = deck.replace(".end\n", &format!("{axis_card}.end\n"));
            assert!(
                axis_deck.contains(axis_card),
                "{kind:?} deck has no .end to attach the {label} axis to"
            );
            let job = Job::new(&format!("axis-{label}-{}", kind.tag()));
            let response = job.execute(&axis_deck, request_kind);
            assert_eq!(
                response["status"], "succeeded",
                "{kind:?} failed on a {label} axis: {response}"
            );
            let execution = axis_execution(&response);
            assert_eq!(execution.coordinate_count, 2, "{kind:?} on {label}");
            assert_eq!(execution.execution_count, 2, "{kind:?} on {label}");
            for run in &execution.runs {
                assert_eq!(run.assignments.len(), 1);
                assert_eq!(run.assignments[0].kind, axis_kind);
                assert_eq!(run.analyses.len(), 1);
                assert_eq!(run.analyses[0].analysis_id, analysis_tag);
                let artifact = run.analyses[0]
                    .artifacts
                    .iter()
                    .find(|artifact| artifact.result_kind == kind.tag())
                    .unwrap_or_else(|| panic!("{kind:?} published no {label} coordinate document"));
                let document = typed_result_at_path(&job, &artifact.path);
                assert_eq!(document.result_kind(), kind);
                let coordinate = document
                    .coordinate()
                    .unwrap_or_else(|| panic!("{kind:?} coordinate document is unplaced"));
                assert_eq!(coordinate.id().to_string(), run.coordinate_id);
            }
        }
    }
}

/// An unconditionally stable loop has no phase crossover, and the shared
/// stability payload has no representation for the infinite margin that
/// produces. The run fails closed with the reason rather than publishing a
/// fabricated finite margin, and the registry records the gap.
#[test]
fn an_unconditionally_stable_loop_is_refused_rather_than_given_a_margin() {
    let job = Job::new("stb-infinite-margin");
    let response = job.execute(
        "single-pole loop\nE1 eo 0 ctrl 0 -1000\nVPROBE eo x 0\nR1 x ctrl 1k\n\
         C1 ctrl 0 159.154943091895n\n.stb dec 2 10 10meg probe=vprobe\n.end\n",
        "stability",
    );
    assert_eq!(response["status"], "failed", "{response}");
    assert_eq!(response["failure_code"], "results.schema_mismatch");
    assert!(
        response["failure_detail"]
            .as_str()
            .expect("a refusal explains itself")
            .contains("margin"),
        "the refusal must name the margin it cannot represent: {response}"
    );
    assert!(job.results_are_empty());
}

//=============================================================================
// Release and request contract
//=============================================================================

#[test]
fn the_release_smoke_request_succeeds_deterministically() {
    let job = Job::new("release-smoke");
    let first = job.run(&release_smoke_request());
    let response = parse_stdout(&first);
    assert_eq!(response["status"], "succeeded");
    assert_eq!(response["result_manifest"]["format"], "rspice-result-v3");
    assert_eq!(
        response["result_manifest"]["analysis_kind"],
        "operating_point"
    );
    assert!(response.get("result_artifacts").is_none());
    assert!(
        job.results_are_empty(),
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
fn a_superseded_protocol_request_is_refused_without_a_response() {
    let job = Job::new("superseded-protocol");
    let mut request: Value =
        serde_json::from_slice(&release_smoke_request()).expect("request JSON");
    request["protocol_version"] = json!(INTEGRITY_ENGINE_PROTOCOL_VERSION - 1);
    let output = job.run(&serde_json::to_vec(&request).expect("request bytes"));
    assert_eq!(
        output.status.code(),
        Some(12),
        "a drifted protocol is a controller fault, not a customer result"
    );
    assert!(output.stdout.is_empty());

    let mut future: Value = serde_json::from_slice(&release_smoke_request()).expect("request JSON");
    future["protocol_version"] = json!(INTEGRITY_ENGINE_PROTOCOL_VERSION + 1);
    let output = job.run(&serde_json::to_vec(&future).expect("request bytes"));
    assert_eq!(output.status.code(), Some(12));
    assert!(output.stdout.is_empty());
}

#[test]
fn the_removed_mixed_signal_kind_names_the_transient_that_replaced_it() {
    let job = Job::new("mixed-signal-removed");
    let response = job.execute(
        "mixed-signal deck\nV1 in 0 PULSE(0 1 0 1u 1u 1m 2m)\nR1 in out 1k\nC1 out 0 1u\n\
         .tran 10u 1m\n.end\n",
        "mixed_signal",
    );
    assert_eq!(response["status"], "failed", "{response}");
    assert_eq!(response["failure_code"], "analysis.unsupported_kind");
    let detail = response["failure_detail"]
        .as_str()
        .expect("a refusal explains itself");
    assert!(
        detail.contains("transient"),
        "the refusal must name the kind to request instead: {detail}"
    );
}

#[test]
fn a_resistive_divider_operating_point_reports_exact_measurements() {
    let job = Job::new("divider-op");
    let response = job.execute(&format!("{DIVIDER}.op\n.end\n"), "operating_point");
    assert_eq!(response["status"], "succeeded");
    assert_eq!(response["result_manifest"]["format"], "rspice-result-v3");
    assert_eq!(
        response["result_manifest"]["typed_result_schema"],
        json!({
            "name": ANALYSIS_RESULT_DOCUMENT_SCHEMA,
            "version": ANALYSIS_RESULT_DOCUMENT_VERSION,
            "content_type": result_document_content_type(),
        })
    );

    let out = measurement(&response, "signal:v(out)");
    assert_eq!(out["unit"], "V");
    assert_eq!(out["sample_count"], 1);
    let value: f64 = out["value_decimal"]
        .as_str()
        .expect("canonical decimal")
        .parse()
        .expect("decimal parses");
    assert!((value - 5.0).abs() < 1e-9, "v(out) was {value}");

    let document = typed_result(&job, &response, "op-001.result.json");
    assert_eq!(document.result_kind(), AnalysisResultKind::OperatingPoint);
    assert_eq!(document.point_count(), 1);
    assert_eq!(
        signal(&document, "v(out)").descriptor().unit(),
        &SignalUnit::Volt
    );
    assert_eq!(
        signal(&document, "i(v1)").descriptor().kind(),
        rspice_core::execution::SignalKind::Current
    );
    let rspice_core::execution::ResultPayload::Op(payload) = document.payload() else {
        panic!("an operating-point document carries an operating-point payload")
    };
    assert!(
        !payload.observables.is_empty(),
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
    let value: f64 = measurement(&response, "signal:v(out)")["value_decimal"]
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

//=============================================================================
// Per-family result content
//=============================================================================

#[test]
fn a_transient_run_declares_and_writes_one_typed_waveform_artifact() {
    let job = Job::new("rc-transient");
    let response = job.execute(
        "rc lowpass step response\nV1 in 0 PULSE(0 1 0 1u 1u 1m 2m)\nR1 in out 1k\nC1 out 0 1u\n\
         .tran 10u 1m\n.end\n",
        "transient",
    );
    assert_eq!(response["status"], "succeeded", "response: {response}");

    let artifacts = response["result_artifacts"]
        .as_array()
        .expect("declared result artifacts");
    assert_eq!(
        artifacts.len(),
        1,
        "one typed document replaces the CSV and the analog document"
    );
    assert_eq!(artifacts[0]["path"], "results/tran-001.result.json");
    assert_eq!(artifacts[0]["content_type"], result_document_content_type());

    let out = measurement(&response, "signal:v(out)");
    assert!(out["sample_count"].as_u64().expect("sample count") > 10);
    assert!(out["series_sha256"].is_string());

    let document = typed_result(&job, &response, "tran-001.result.json");
    assert_eq!(document.result_kind(), AnalysisResultKind::Transient);
    assert_eq!(document.axes()[0].name(), "time");
    assert_eq!(document.axes()[0].unit(), &SignalUnit::Second);
    assert_eq!(
        signal(&document, "i(v1)").descriptor().kind(),
        rspice_core::execution::SignalKind::Current
    );
}

#[test]
fn dc_result_preserves_axis_voltage_current_and_device_observables() {
    let job = Job::new("divider-dc");
    let response = job.execute(
        "divider sweep\nV1 in 0 DC 0\nR1 in out 1k\nR2 out 0 1k\n.dc V1 0 1 0.5\n.end\n",
        "dc_sweep",
    );
    assert_eq!(response["status"], "succeeded", "response: {response}");
    let document = typed_result(&job, &response, "dc-001.result.json");
    assert_eq!(document.result_kind(), AnalysisResultKind::DcSweep);
    assert_eq!(document.point_count(), 3);
    assert_eq!(document.axes()[0].name(), "sweep:v1");
    assert_eq!(document.axes()[0].unit(), &SignalUnit::Volt);
    assert!(matches!(
        signal(&document, "v(out)").values(),
        SeriesValues::Real { .. }
    ));
    assert_eq!(
        signal(&document, "i(v1)").descriptor().unit(),
        &SignalUnit::Ampere
    );
    let rspice_core::execution::ResultPayload::Dc(payload) = document.payload() else {
        panic!("a DC document carries a DC payload")
    };
    assert_eq!(payload.sweep_variable, "v1");
    assert!(
        payload
            .observables
            .iter()
            .all(|observable| observable.values.len() == 3),
        "every observable keeps one value per sweep point"
    );
}

#[test]
fn a_nested_dc_sweep_retains_both_authored_sweep_axes() {
    let job = Job::new("nested-dc");
    let response = job.execute(
        "nested divider sweep\nV1 in 0 DC 0\nV2 bias 0 DC 0\nR1 in out 1k\nR2 out bias 1k\n\
         .dc V1 0 1 0.5 V2 0 1 1\n.end\n",
        "dc_sweep",
    );
    assert_eq!(response["status"], "succeeded", "response: {response}");
    let document = typed_result(&job, &response, "dc-001.result.json");
    let names: Vec<&str> = document.axes().iter().map(|axis| axis.name()).collect();
    assert_eq!(
        names,
        ["sweep:v1", "sweep:v2"],
        "the outer sweep coordinate must not be dropped"
    );
    assert_eq!(document.point_count(), 6);
}

#[test]
fn ac_result_preserves_complex_voltage_and_branch_current() {
    let job = Job::new("rc-ac");
    let response = job.execute(&format!("{RC}.ac LIN 2 1k 2k\n.end\n"), "ac_small_signal");
    assert_eq!(response["status"], "succeeded", "response: {response}");
    let document = typed_result(&job, &response, "ac-001.result.json");
    assert_eq!(document.result_kind(), AnalysisResultKind::Ac);
    let SeriesValues::Complex { samples } = signal(&document, "v(out)").values() else {
        panic!("AC voltages must stay complex")
    };
    assert!(
        samples
            .iter()
            .flatten()
            .any(|sample| sample.imaginary != 0.0)
    );
    assert!(matches!(
        signal(&document, "i(v1)").values(),
        SeriesValues::Complex { .. }
    ));
    // Complex measurements keep both components rather than collapsing to a
    // magnitude the document never stored.
    assert_eq!(measurement(&response, "signal:v(out).re")["unit"], "V");
    assert_eq!(measurement(&response, "signal:v(out).im")["unit"], "V");
}

#[test]
fn noise_result_preserves_the_contribution_catalog_and_densities() {
    let job = Job::new("divider-noise");
    let response = job.execute(
        "divider noise\nV1 in 0 DC 0 AC 1\nR1 in out 1k\nR2 out 0 1k\n\
         .noise V(out) V1 LIN 2 1k 2k\n.end\n",
        "noise",
    );
    assert_eq!(response["status"], "succeeded", "response: {response}");
    let document = typed_result(&job, &response, "noise-001.result.json");
    assert_eq!(document.result_kind(), AnalysisResultKind::Noise);
    assert_eq!(document.axes()[0].unit(), &SignalUnit::Hertz);
    let rspice_core::execution::ResultPayload::Noise(payload) = document.payload() else {
        panic!("a noise document carries a noise payload")
    };
    assert!(
        !payload.contributions.is_empty(),
        "per-device noise contributions must not be dropped"
    );
}

#[test]
fn a_transfer_function_result_names_its_output_and_input() {
    let job = Job::new("divider-tf");
    let response = job.execute(
        "divider transfer function\nV1 in 0 DC 10\nR1 in out 1k\nR2 out 0 1k\n.tf V(out) V1\n.end\n",
        "transfer_function",
    );
    assert_eq!(response["status"], "succeeded", "response: {response}");
    let document = typed_result(&job, &response, "tf-001.result.json");
    let rspice_core::execution::ResultPayload::Tf(payload) = document.payload() else {
        panic!("a .TF document carries a transfer-function payload")
    };
    assert!(payload.input.eq_ignore_ascii_case("v1"), "{payload:?}");
    let ScalarValue::Real { value: Some(gain) } = scalar_value(&document, "transfer_gain") else {
        panic!("the transfer function reports a real gain")
    };
    assert!((gain - 0.5).abs() < 1e-9, "gain was {gain}");
}

#[test]
fn a_monte_carlo_result_retains_every_trial_and_its_statistics() {
    let job = Job::new("divider-mc");
    let response = job.execute(
        "divider monte carlo\nV1 in 0 DC 10\nR1 in out 1k\nR2 out 0 1k\n\
         .mc 4 SEED 7 GAUSS 0.01\n.end\n",
        "monte_carlo",
    );
    assert_eq!(response["status"], "succeeded", "response: {response}");
    let document = typed_result(&job, &response, "mc-001.result.json");
    assert_eq!(document.result_kind(), AnalysisResultKind::MonteCarlo);
    let rspice_core::execution::ResultPayload::MonteCarlo(payload) = document.payload() else {
        panic!("a Monte Carlo document carries a Monte Carlo payload")
    };
    assert!(!payload.statistics.is_empty());
    assert!(
        payload
            .statistics
            .iter()
            .all(|statistic| statistic.samples.len() == 4),
        "every trial must be retained, not just its summary"
    );
}

#[test]
fn a_pac_result_names_the_pss_it_linearized_around() {
    let job = Job::new("rf-pac");
    let response = job.execute(
        &format!("{RF}.pss fund=1g\n.pac dec 2 1k 10k input=v1 out=v(out)\n.end\n"),
        "pac",
    );
    assert_eq!(response["status"], "succeeded", "response: {response}");
    let document = typed_result(&job, &response, "pac-001.result.json");
    assert_eq!(document.result_kind(), AnalysisResultKind::Pac);
    assert_eq!(
        document.parent_analysis().map(|parent| parent.tag()),
        Some("pss-001".to_owned()),
        "a PAC result must name its periodic operating point"
    );
}

#[test]
fn an_envelope_result_names_the_harmonic_balance_carrier() {
    let job = Job::new("rf-envelope");
    let response = job.execute(
        &format!("{RF}.hb 1g\n.envelope tstop=1n\n.end\n"),
        "envelope",
    );
    assert_eq!(response["status"], "succeeded", "response: {response}");
    let document = typed_result(&job, &response, "env-001.result.json");
    assert_eq!(document.result_kind(), AnalysisResultKind::Envelope);
    assert_eq!(
        document.parent_analysis().map(|parent| parent.tag()),
        Some("hb-001".to_owned())
    );
}

#[test]
fn an_ac_sensitivity_sweep_is_refused_with_its_typed_reason() {
    let job = Job::new("sens-ac");
    let response = job.execute(
        "divider AC sensitivity\nV1 in 0 DC 10 AC 1\nR1 in out 1k\nR2 out 0 1k\n\
         .sens V(out) AC DEC 2 1k 10k\n.end\n",
        "sensitivity",
    );
    assert_eq!(response["status"], "failed", "response: {response}");
    assert_eq!(response["failure_code"], "analysis.unsupported_form");
    assert!(job.results_are_empty());
}

//=============================================================================
// Transient FFT
//=============================================================================

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
    let response = job.execute(deck, "transient");
    assert_eq!(response["status"], "succeeded", "response: {response}");
    assert_eq!(
        response["result_manifest"]["typed_fft_result_schema"],
        json!({
            "name": "rspice-transient-fft-result",
            "version": 1,
            "content_type": FFT_RESULT_DOCUMENT_CONTENT_TYPE,
        })
    );
    assert_eq!(
        response["result_artifacts"]
            .as_array()
            .expect("result artifacts")
            .len(),
        4,
        "two transients each publish one typed result and one FFT bundle"
    );

    let first = typed_fft_result(&job, &response, "tran-001.fft.result.json");
    let second = typed_fft_result(&job, &response, "tran-002.fft.result.json");
    assert_eq!(first.parent_analysis, "tran-001");
    assert_eq!(second.parent_analysis, "tran-002");
    for document in [&first, &second] {
        assert_eq!(document.result_count, 2);
        assert_eq!(document.results[0].analysis_id, "fft-001");
        assert_eq!(document.results[1].analysis_id, "fft-002");
        assert_eq!(
            document.results[0].parent_analysis_id,
            document.parent_analysis
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
            document.results[1].signal.physical_type,
            FftPhysicalType::Current
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
    let response = job.execute(
        "unresolvable transient FFT output\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n\
         .tran 1u 1m\n.fft v(missing) np=8\n.end\n",
        "transient",
    );
    assert_eq!(response["status"], "failed", "response: {response}");
    assert!(job.results_are_empty(), "failed run published artifacts");
}

//=============================================================================
// Run axes
//=============================================================================

#[test]
fn step_wraps_transient_with_canonical_namespaces_and_no_extra_op() {
    let job = Job::new("step-transient");
    let response = job.execute(
        "stepped transient\n\
         .param rval=1k\n\
         V1 in 0 PULSE(0 1 0 1u 1u 20u 50u)\n\
         R1 in out {rval}\n\
         C1 out 0 1n\n\
         .step param rval list 1k 2k\n\
         .tran 1u 20u\n\
         .end\n",
        "transient",
    );
    assert_eq!(response["status"], "succeeded", "response: {response}");
    assert_eq!(response["result_manifest"]["format"], "rspice-result-v3");
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
        for artifact in &run.analyses[0].artifacts {
            assert!(paths.insert(artifact.path.clone()));
            assert!(artifact.path.contains(&run.coordinate_namespace));
            assert!(artifact.path.contains("tran-001"));
            assert_eq!(artifact.schema, ANALYSIS_RESULT_DOCUMENT_SCHEMA);
            assert_eq!(artifact.schema_version, ANALYSIS_RESULT_DOCUMENT_VERSION);
            assert_eq!(artifact.result_kind, "tran");
            assert_eq!(artifact.content_type, result_document_content_type());
        }
    }
    assert_eq!(paths.len(), 2);

    // Every referenced document decodes and carries the coordinate identity
    // its manifest entry claims.
    for run in &execution.runs {
        for artifact in &run.analyses[0].artifacts {
            let document = typed_result_at_path(&job, &artifact.path);
            let coordinate = document.coordinate().expect("an axis result is placed");
            assert_eq!(coordinate.label(), run.coordinate_namespace);
            assert_eq!(coordinate.id().to_string(), run.coordinate_id);
        }
    }
}

#[test]
fn temp_wraps_ac_and_repeated_directives_keep_stable_ordered_ids() {
    let job = Job::new("temp-repeated-ac");
    let response = job.execute(
        "temperature AC\n\
         V1 in 0 AC 1\n\
         R1 in out 1k\n\
         C1 out 0 1u\n\
         .temp 25 75\n\
         .ac LIN 2 1k 2k\n\
         .ac LIN 3 2k 4k\n\
         .end\n",
        "ac_small_signal",
    );
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
            for artifact in &analysis.artifacts {
                assert!(
                    artifacts.insert(artifact.path.clone()),
                    "duplicate path {}",
                    artifact.path
                );
            }
        }
    }
    assert_eq!(artifacts.len(), 4);
}

#[test]
fn dc_step_and_noise_temperature_axes_retain_every_coordinate() {
    let dc_job = Job::new("step-dc");
    let dc_response = dc_job.execute(
        "stepped DC\n\
         .param load=1k\n\
         V1 in 0 0\n\
         R1 in 0 {load}\n\
         .step param load list 1k 2k\n\
         .dc V1 0 1 1\n\
         .end\n",
        "dc_sweep",
    );
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
    let noise_response = noise_job.execute(
        "temperature noise\n\
         V1 in 0 AC 1\n\
         R1 in out 1k\n\
         R2 out 0 1k\n\
         .temp 0 100\n\
         .noise V(out) V1 LIN 1 1k 1k\n\
         .end\n",
        "noise",
    );
    assert_eq!(noise_response["status"], "succeeded", "{noise_response}");
    let noise_execution = axis_execution(&noise_response);
    assert_eq!(noise_execution.analysis_kind, AxisAnalysisKind::Noise);
    assert_eq!(noise_execution.coordinate_count, 2);
    let densities = noise_execution
        .runs
        .iter()
        .map(|run| {
            let artifact = &run.analyses[0].artifacts[0];
            let document = typed_result_at_path(&noise_job, &artifact.path);
            let rspice_core::execution::ResultPayload::Noise(_) = document.payload() else {
                panic!("noise coordinate must publish a noise document")
            };
            let SeriesValues::Real { samples } = signal(&document, "onoise_spectrum").values()
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
    let response = job.execute(
        "implicit stepped OP\n\
         .param rval=1k\n\
         V1 in 0 1\n\
         R1 in 0 {rval}\n\
         .step param rval list 1k 2k\n\
         .end\n",
        "operating_point",
    );
    assert_eq!(response["status"], "succeeded", "response: {response}");
    let execution = axis_execution(&response);
    assert_eq!(execution.analysis_kind, AxisAnalysisKind::OperatingPoint);
    assert!(execution.runs.iter().all(|run| {
        run.analyses.len() == 1 && run.analyses[0].analysis_id == "implicit-op-001"
    }));
}

#[test]
fn axisless_deck_without_analysis_executes_implicit_op_in_scalar_shape() {
    let job = Job::new("scalar-implicit-op");
    let response = job.execute(
        "implicit scalar OP\nV1 in 0 1\nR1 in 0 1k\n.end\n",
        "operating_point",
    );
    assert_eq!(response["status"], "succeeded", "response: {response}");
    assert_eq!(response["result_manifest"]["format"], "rspice-result-v3");
    assert!(response["result_manifest"].get("axis_execution").is_none());
    let document = typed_result(&job, &response, "implicit-op-001.result.json");
    assert_eq!(document.result_kind(), AnalysisResultKind::OperatingPoint);
    assert!(
        document.coordinate().is_none(),
        "an axisless run has no shared-deck coordinate"
    );
}

#[test]
fn conditional_topology_and_analysis_signature_fail_closed_without_artifacts() {
    for (name, deck, kind, failure_code) in [
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
            "operating_point",
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
            "ac_small_signal",
            // The materializer's own mismatch is typed by the engine now, so it
            // arrives with the engine's category rather than a coarse adapter code.
            "engine.materialization_mismatch",
        ),
    ] {
        let job = Job::new(name);
        let response = job.execute(deck, kind);
        assert_eq!(response["status"], "failed", "response: {response}");
        assert_eq!(
            response["failure_code"], failure_code,
            "response: {response}"
        );
        assert!(job.results_are_empty());
    }
}

#[test]
fn alter_axes_are_explicitly_unsupported() {
    let job = Job::new("alter-axis");
    let response = job.execute(
        "ALTER deck\nV1 in 0 1\nR1 in 0 1k\n.op\n.alter second\nR1 in 0 2k\n.end\n",
        "operating_point",
    );
    assert_eq!(response["status"], "failed", "response: {response}");
    // Source-variant expansion is a capability the engine declines, and the
    // wire code says so instead of naming an adapter-local axis rule.
    assert_eq!(response["failure_code"], "engine.unsupported_capability");
}

//=============================================================================
// Bounded failure vocabulary
//=============================================================================

#[test]
fn invalid_explicit_transient_tmax_is_a_bounded_configuration_failure() {
    let job = Job::new("invalid-transient-tmax");
    let response = job.execute(
        "invalid TMAX\nV1 in 0 1\nR1 in 0 1k\n.tran 1u 1m 0 0\n.end\n",
        "transient",
    );
    assert_eq!(response["status"], "failed", "response: {response}");
    assert_eq!(response["failure_code"], "analysis.invalid_configuration");
    assert!(job.results_are_empty());
}

#[test]
fn wrong_analysis_kind_for_the_deck_is_a_bounded_failure() {
    let job = Job::new("kind-mismatch");
    let response = job.execute(&format!("{DIVIDER}.op\n.end\n"), "transient");
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
    let response = job.execute(
        "floating node\nI1 0 out DC 1m\nC1 out 0 1u\n.op\n.end\n",
        "operating_point",
    );
    assert_eq!(response["status"], "failed");
    assert_eq!(response["failure_code"], "engine.circuit_error");
}

/// The same deck with ngspice's `RSHUNT` shunt resistor runs, because the
/// shunt is a real element the author sized: 1 mA through 1 GOhm is 1 MV.
#[test]
fn rshunt_restores_the_dc_path_and_the_run_succeeds() {
    let job = Job::new("rshunt");
    let response = job.execute(
        "shunted floating node\nI1 0 out DC 1m\nC1 out 0 1u\n.options rshunt=1e9\n.op\n.end\n",
        "operating_point",
    );
    assert_eq!(response["status"], "succeeded");
    assert_eq!(
        measurement(&response, "signal:v(out)")["value_decimal"],
        "9.999999999999999e5"
    );
}

#[test]
fn an_unparseable_deck_is_a_bounded_failure() {
    let job = Job::new("parse-failure");
    let response = job.execute("broken deck\nR1 in\n.op\n.end\n", "operating_point");
    assert_eq!(response["status"], "failed");
    assert_eq!(response["failure_code"], "netlist.parse_error");
}

#[test]
fn an_unmapped_authored_card_fails_the_whole_request_without_writing_results() {
    // Leaving a card this build cannot publish unexecuted would drop authored
    // intent from the response, so the request is refused instead.
    for (cards, card, analysis_id) in [
        (".sp dec 2 1k 10k\n", ".SP", "sp-001"),
        (
            ".hb 1g\n.pnoise dec 2 1 1k out=v(out)\n",
            ".PNOISE",
            "pnoise-001",
        ),
        (".four 1g v(out)\n", ".FOUR", ""),
    ] {
        let job = Job::new("unmapped-card");
        let deck = format!("{RF}.tran 10p 1n\n{cards}.end\n");
        let response = job.execute(&deck, "transient");
        assert_eq!(response["status"], "failed", "{response}");
        assert_eq!(response["failure_code"], "analysis.unsupported_kind");
        let detail = response["failure_detail"]
            .as_str()
            .expect("a refusal explains itself");
        assert!(
            detail.contains(card),
            "refusal must name the card: {detail}"
        );
        if !analysis_id.is_empty() {
            assert!(
                detail.contains(analysis_id),
                "refusal must name the analysis instance: {detail}"
            );
        }
        assert!(response.get("result_artifacts").is_none());
        assert!(
            job.results_are_empty(),
            "a refused request must write no results"
        );
    }
}

/// A table-driven `.AC DATA=` card is the same AC family with the frequency
/// grid and per-row parameter overrides coming from the deck's own table.
#[test]
fn a_table_driven_ac_card_publishes_the_ordinary_ac_document() {
    let job = Job::new("ac-data-table");
    let response = job.execute(
        "AC DATA deck\n\
         .PARAM RVAL=1k\n\
         I1 out 0 AC 1\n\
         R1 out 0 {RVAL}\n\
         .DATA points\n\
         + FREQ RVAL\n\
         + 10 1k\n\
         + 100 2k\n\
         .ENDDATA\n\
         .AC DATA=points\n\
         .END\n",
        "ac_small_signal",
    );
    assert_eq!(response["status"], "succeeded", "{response}");
    let document = typed_result(&job, &response, "ac-001.result.json");
    assert_eq!(document.result_kind(), AnalysisResultKind::Ac);
    assert_eq!(document.point_count(), 2, "one point per authored row");
    let rspice_core::execution::result_document::AxisValues::Real { values } =
        document.axes()[0].values()
    else {
        panic!("a frequency axis is real")
    };
    assert_eq!(values, &[10.0, 100.0]);
}

/// The table-driven `.NOISE ... DATA=` card is the same noise family.
#[test]
fn a_table_driven_noise_card_publishes_the_ordinary_noise_document() {
    let job = Job::new("noise-data-table");
    let response = job.execute(
        "noise DATA deck\n\
         .GLOBAL_PARAM mag=1 phase=0\n\
         V1 in 0 DC 0 AC {mag} {phase}\n\
         R1 in out 1k\n\
         R2 out 0 1k\n\
         .NOISE V(out) V1 DATA=points\n\
         .DATA points\n\
         + mag phase HERTZ\n\
         + 2 20 10\n\
         + 1 10 1\n\
         .ENDDATA\n\
         .END\n",
        "noise",
    );
    assert_eq!(response["status"], "succeeded", "{response}");
    let document = typed_result(&job, &response, "noise-001.result.json");
    assert_eq!(document.result_kind(), AnalysisResultKind::Noise);
    assert_eq!(document.point_count(), 2, "one point per authored row");
}

/// The request selects one family; the deck may author several, and the
/// others are simply not the requested run.
#[test]
fn a_deck_authoring_several_families_runs_only_the_requested_one() {
    let job = Job::new("multi-family-deck");
    let response = job.execute(
        "several authored families\nV1 in 0 DC 1 AC 1\nR1 in out 1k\nC1 out 0 1u\n\
         .op\n.tran 10u 1m\n.ac LIN 2 1k 2k\n.end\n",
        "ac_small_signal",
    );
    assert_eq!(response["status"], "succeeded", "{response}");
    let artifacts = response["result_artifacts"]
        .as_array()
        .expect("declared result artifacts");
    assert_eq!(
        artifacts.len(),
        1,
        "only the requested family publishes: {response}"
    );
    assert_eq!(artifacts[0]["path"], "results/ac-001.result.json");
    let document = typed_result(&job, &response, "ac-001.result.json");
    assert_eq!(document.result_kind(), AnalysisResultKind::Ac);
}

/// `.PAC` may attach to a `.HB` carrier, but the shared result document
/// accepts only a `.PSS` parent for the pac family. Publishing without the
/// link would drop the provenance that says which large-signal solution the
/// small-signal response was taken around.
#[test]
fn a_pac_card_attached_to_a_harmonic_balance_carrier_is_refused() {
    let job = Job::new("pac-hb-upstream");
    let response = job.execute(
        &format!("{RF}.hb 1g\n.pac dec 2 1k 10k input=v1 out=v(out)\n.end\n"),
        "pac",
    );
    assert_eq!(response["status"], "failed", "{response}");
    assert_eq!(response["failure_code"], "analysis.unsupported_form");
    let detail = response["failure_detail"]
        .as_str()
        .expect("a refusal explains itself");
    assert!(
        detail.contains("pac-001") && detail.contains("hb-001"),
        "{detail}"
    );
    assert!(job.results_are_empty());
}

//=============================================================================
// Launch contract
//=============================================================================

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

    let superseded_launch = job.run_with(
        &release_smoke_request(),
        &[("RSPICE_ENGINE_PROTOCOL_VERSION", "3")],
        &[],
    );
    assert_eq!(
        superseded_launch.status.code(),
        Some(10),
        "a protocol-3 deployment must be refused before a request is read"
    );
    assert!(superseded_launch.stdout.is_empty());
}

/// The solve budget is a launch input the worker owns, not a compiled-in
/// constant, and the two ways of getting it wrong are told apart: a budget
/// that expires is a customer outcome on stdout, a budget that does not parse
/// is a controller fault on the exit status.
#[test]
fn the_solve_budget_is_a_launch_input_with_a_bounded_outcome() {
    let job = Job::new("solve-budget");
    let request = build_request(
        json!({"schema": "rspice-circuit-v1", "netlist_utf8":
            "budget exhaustion\n\
             V1 in 0 PULSE(0 1 0 1n 1n 1u 2u)\n\
             R1 in out 1k\n\
             C1 out 0 1n\n\
             .tran 1n 10m\n\
             .end\n"}),
        json!({"kind": "transient"}),
        Vec::new(),
    );

    let expired = job.run_with(
        &request,
        &[("RSPICE_ENGINE_SOLVE_BUDGET_SECONDS", "0.001")],
        &[],
    );
    assert_eq!(expired.status.code(), Some(0));
    let response = parse_stdout(&expired);
    assert_eq!(response["status"], "failed");
    assert_eq!(
        response["failure_code"], "engine.time_limit",
        "an expired budget is a time limit, not a cancellation"
    );

    for malformed in ["0", "-5", "abc", ""] {
        let refused = job.run_with(
            &request,
            &[("RSPICE_ENGINE_SOLVE_BUDGET_SECONDS", malformed)],
            &[],
        );
        assert_eq!(
            refused.status.code(),
            Some(10),
            "budget {malformed:?} must be a launch-contract violation rather than a \
             silent fall back to the default"
        );
        assert!(refused.stdout.is_empty());
    }
}

/// Every family stops on the same cancellation label, so an operator reading
/// the wire cannot tell one analysis's cancellation from another's.
#[test]
fn every_family_reports_the_same_cancellation_label() {
    for kind in AnalysisResultKind::ALL {
        let (request_kind, deck) = match family_expectation(kind) {
            FamilyExpectation::Runs {
                request_kind, deck, ..
            } => (request_kind, deck),
            FamilyExpectation::Attached {
                parent_request_kind,
                deck,
                ..
            } => (parent_request_kind, deck),
            FamilyExpectation::Refused { .. } => continue,
        };
        let job = Job::new(&format!("cancel-{}", kind.tag()));
        // A budget that is already spent at the first abort poll makes the
        // stop deterministic instead of a race against a real solve.
        let output = job.run_with(
            &build_request(
                json!({"schema": "rspice-circuit-v1", "netlist_utf8": deck}),
                json!({"kind": request_kind}),
                Vec::new(),
            ),
            &[("RSPICE_ENGINE_SOLVE_BUDGET_SECONDS", "0.000001")],
            &[],
        );
        let response = parse_stdout(&output);
        assert_eq!(response["status"], "failed", "{kind:?}: {response}");
        assert_eq!(
            response["failure_code"], "engine.time_limit",
            "{kind:?} must stop on the shared deadline label: {response}"
        );
        assert!(
            job.results_are_empty(),
            "{kind:?} published artifacts after stopping"
        );
    }
}

/// A caller-requested stop is reported as a cancellation, not as a deadline,
/// for every family, and no family declares an artifact on the way out.
///
/// The label is asserted in process because the packaged binary's stop request
/// is a signal there is no portable way to send from a test. The deadline test
/// above is the mid-solve counterpart: every family reaching
/// `engine.time_limit` proves its runner honours the abort source once the
/// solve has started.
#[test]
fn every_family_reports_a_caller_stop_as_a_cancellation() {
    for kind in AnalysisResultKind::ALL {
        let (request_kind, deck) = match family_expectation(kind) {
            FamilyExpectation::Runs {
                request_kind, deck, ..
            } => (request_kind, deck),
            FamilyExpectation::Attached {
                parent_request_kind,
                deck,
                ..
            } => (parent_request_kind, deck),
            FamilyExpectation::Refused { .. } => continue,
        };
        let execution = execute_with_abort(
            &json!({"kind": request_kind}),
            &CircuitContent::Deck {
                expanded_netlist: deck.to_owned(),
            },
            "test",
            &ImmediateAbort,
            DEFAULT_SOLVE_BUDGET,
        );
        match &execution.response {
            EngineResponse::Failed { failure_code, .. } => assert_eq!(
                failure_code, CANCELLED_FAILURE_CODE,
                "{kind:?} mislabelled a caller stop"
            ),
            EngineResponse::Succeeded { .. } => {
                panic!("{kind:?} reported success after a caller stop")
            }
        }
        assert!(
            execution.artifacts.is_empty(),
            "{kind:?} declared artifacts after a caller stop"
        );
    }
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
    assert_eq!(
        info["protocol_versions"],
        json!([INTEGRITY_ENGINE_PROTOCOL_VERSION])
    );
    assert_eq!(
        info["result_schemas"],
        json!([
            format!("{ANALYSIS_RESULT_DOCUMENT_SCHEMA}-v{ANALYSIS_RESULT_DOCUMENT_VERSION}"),
            "rspice-transient-fft-result-v1",
            "rspice-axis-execution-v1",
        ])
    );
}

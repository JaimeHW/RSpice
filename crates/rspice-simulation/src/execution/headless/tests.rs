use super::*;
use crate::execution::PreparedRunAuthorization;
use crate::execution_artifact::ExecutionArtifactEnvelope;
use crate::execution_options::SpecExecutionOptions;
use crate::preparation::QueuedAnalysis;
use crate::results::SimulationResult;
use crate::runner::SimulationRunner;
use rspice_app_types::product::AnalysisInstanceId;
use rspice_core::NoAbort;
use rspice_core::abort_signal::CountingAbort;
use rspice_core::netlist::SealedSourceEdge;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use rspice_simulation_contract::config::FftRequest;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

const DIVIDER: &str = "Headless divider\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.end\n";

fn origin() -> PathBuf {
    std::env::temp_dir()
        .join("rspice-headless-portable")
        .join("circuit.cir")
}

fn task(spec: AnalysisSpec, analysis_line: String) -> HeadlessTaskRequest {
    HeadlessTaskRequest {
        instance_id: AnalysisInstanceId::new(),
        label: spec.run_type().display_name().into(),
        dependencies: Vec::new(),
        analysis: QueuedAnalysis {
            spec,
            config: None,
            spec_options: SpecExecutionOptions::default(),
            analysis_line,
            numeric_override: None,
        },
    }
}

fn op() -> HeadlessTaskRequest {
    task(AnalysisSpec::dc_op(), ".op".into())
}

fn execute(snapshot: PreparedRunSnapshot) -> Vec<SimulationResult> {
    let dispatch = PreparedRunAuthorization::default()
        .authorize_campaign_member(snapshot)
        .unwrap();
    let mut artifacts = HashMap::new();
    let mut results = Vec::new();
    let mut runner = SimulationRunner::new();
    for task in dispatch.into_tasks() {
        let identity = (
            task.snapshot_digest(),
            task.instance_id(),
            task.source_revision(),
            task.config_digest(),
        );
        runner
            .start_prepared(
                task.resolve_dependency_artifacts(&artifacts).unwrap(),
                false,
            )
            .unwrap();
        let deadline = Instant::now() + Duration::from_secs(20);
        let result = loop {
            if let Some(result) = runner.poll_result() {
                break result.unwrap();
            }
            assert!(
                Instant::now() < deadline,
                "prepared headless task did not finish"
            );
            std::thread::sleep(Duration::from_millis(1));
        };
        // Only the producer with recorded spectra has an FFT handoff. A
        // standalone transient is not an empty trajectory dependency.
        if matches!(&result, SimulationResult::Transient { spectra, .. } if !spectra.is_empty())
            && let Some(artifact) = ExecutionArtifactEnvelope::from_transient_result(
                identity.0,
                identity.1,
                identity.2,
                identity.3,
                &result,
                &[],
                true,
            )
            .unwrap()
        {
            artifacts.insert(identity.1, artifact);
        }
        results.push(result);
    }
    results
}

fn voltage(result: &SimulationResult) -> f64 {
    assert!(matches!(result, SimulationResult::DcOp(_)));
    result
        .measurement("V(out)")
        .expect("divider output voltage")
}

#[test]
fn headless_snapshot_is_repeatable_and_executes_through_authorized_dispatch() {
    let path = origin();
    let request = op();
    let first = prepare_headless_run(
        HeadlessRunInput::new(DIVIDER, &path, vec![request.clone()]),
        &NoAbort,
    )
    .unwrap();
    let second = prepare_headless_run(
        HeadlessRunInput::new(DIVIDER, &path, vec![request]),
        &NoAbort,
    )
    .unwrap();
    assert_eq!(first.digest(), second.digest());
    assert_eq!(
        first.prepared_run_receipt().unwrap(),
        second.prepared_run_receipt().unwrap()
    );
    assert!((voltage(&execute(first)[0]) - 0.5).abs() < 1e-10);
}

#[test]
fn sealed_include_closure_reaches_the_worker_and_authenticates_unused_library_bytes() {
    let path = origin();
    let library = path.with_file_name("models.lib");
    let source = "Sealed library\n.lib models.lib nominal\nV1 in 0 1\n.end\n";
    let request = op();
    let prepare = |unused: &str| {
        let contents = format!(
            ".lib nominal\nR1 in out 1k\nR2 out 0 1k\n.endl nominal\n.lib spare\n{unused}\n.endl spare\n"
        );
        let bundle = SealedSourceBundle::try_new_with_edges(
            [(path.clone(), source.into()), (library.clone(), contents)],
            [SealedSourceEdge {
                owner: path.clone(),
                requested_path: "models.lib".into(),
                target: library.clone(),
            }],
        )
        .unwrap();
        let mut input = HeadlessRunInput::new(source, &path, vec![request.clone()]);
        input.resolver = HeadlessSourceResolver::Sealed(bundle);
        prepare_headless_run(input, &NoAbort).unwrap()
    };
    let first = prepare("* original unused section");
    let second = prepare("* changed unused section");
    let metadata = first.metadata();
    assert_eq!(metadata.sealed_source_dependencies.len(), 1);
    let dependency = &metadata.sealed_source_dependencies[0];
    assert_eq!(dependency.selected_section(), Some("nominal"));
    assert!(dependency.source().contains("original unused section"));
    assert_eq!(metadata.source_digest, second.metadata().source_digest);
    assert_ne!(metadata.receipt_digest, second.metadata().receipt_digest);
    assert_ne!(first.digest(), second.digest());
    assert!((voltage(&execute(first)[0]) - 0.5).abs() < 1e-10);
}

struct NativeFixture(PathBuf);
impl NativeFixture {
    fn new() -> Self {
        let directory =
            std::env::temp_dir().join(format!("rspice-headless-{}", AnalysisInstanceId::new()));
        std::fs::create_dir(&directory).unwrap();
        Self(directory)
    }
}
impl Drop for NativeFixture {
    fn drop(&mut self) {
        for name in ["circuit.cir", "divider.inc"] {
            let _ = std::fs::remove_file(self.0.join(name));
        }
        let _ = std::fs::remove_dir(&self.0);
    }
}

#[test]
fn native_dependencies_are_captured_once_and_sealed_resolution_has_no_host_fallback() {
    let fixture = NativeFixture::new();
    let path = fixture.0.join("circuit.cir");
    let include = fixture.0.join("divider.inc");
    let source = "Native library\n.include divider.inc\nV1 in 0 1\n.end\n";
    std::fs::write(&path, source).unwrap();
    std::fs::write(&include, "R1 in out 1k\nR2 out 0 1k\n").unwrap();
    assert!(
        prepare_headless_run(HeadlessRunInput::new(source, &path, vec![op()]), &NoAbort).is_err()
    );
    let request = op();
    let capture = || {
        let mut input = HeadlessRunInput::new(source, &path, vec![request.clone()]);
        input.resolver = HeadlessSourceResolver::Native {
            include_search_paths: vec![],
        };
        prepare_headless_run(input, &NoAbort).unwrap()
    };
    let first = capture();
    std::fs::write(&include, "R1 in out 1k\nR2 out 0 3k\n").unwrap();
    let second = capture();
    assert_ne!(first.digest(), second.digest());
    assert!((voltage(&execute(first)[0]) - 0.5).abs() < 1e-10);
    assert!((voltage(&execute(second)[0]) - 0.75).abs() < 1e-10);
}

#[test]
fn source_capture_preserves_abort_and_resource_errors() {
    let path = origin();
    assert!(matches!(
        prepare_headless_run(
            HeadlessRunInput::new(DIVIDER, &path, vec![op()]),
            &CountingAbort::new(0)
        ),
        Err(HeadlessPreparationError::Aborted)
    ));
    let source = format!("Long circuit\n{}\n.end\n", "* a source line\n".repeat(4096));
    assert!(matches!(
        prepare_headless_run(
            HeadlessRunInput::new(&source, &path, vec![op()]),
            &CountingAbort::new(8)
        ),
        Err(HeadlessPreparationError::Aborted)
    ));
    let mut input = HeadlessRunInput::new(DIVIDER, &path, vec![op()]);
    input.preparation_limits.max_netlist_bytes = 8;
    assert!(matches!(
        prepare_headless_run(input, &NoAbort),
        Err(HeadlessPreparationError::ResourceLimit(
            ResourceLimitError {
                resource: ResourceKind::NetlistBytes,
                limit: 8,
                ..
            }
        ))
    ));
}

#[test]
fn source_errors_retain_parser_locations_and_refuse_competing_plans() {
    let path = origin();
    assert!(matches!(
        prepare_headless_run(
            HeadlessRunInput::new("broken\nR1\n.end\n", &path, vec![op()]),
            &NoAbort
        ),
        Err(HeadlessPreparationError::Parse(_))
    ));
    for card in [".op", ".fft V(out) NP=16", ".control\nop\n.endc"] {
        let source = DIVIDER.replace(".end", &format!("{card}\n.end"));
        let error =
            prepare_headless_run(HeadlessRunInput::new(&source, &path, vec![op()]), &NoAbort)
                .unwrap_err();
        assert!(error.to_string().contains("circuit-only"), "{error}");
    }
}

#[test]
fn unsealed_inputs_and_task_source_injection_fail_before_dispatch() {
    let path = origin();
    for line in [".include missing.inc", ".end", ".control\nop\n.endc"] {
        let mut request = op();
        request.analysis.analysis_line = line.into();
        assert!(
            prepare_headless_run(
                HeadlessRunInput::new(DIVIDER, &path, vec![request]),
                &NoAbort
            )
            .is_err(),
            "{line}"
        );
    }
    let source = "unsealed waveform\nV1 out 0 PWL FILE=missing.txt\nR1 out 0 1k\n.end\n";
    assert!(
        prepare_headless_run(HeadlessRunInput::new(source, &path, vec![op()]), &NoAbort).is_err()
    );
}

#[test]
fn fft_cards_reach_only_their_bound_transient_and_publish_its_exact_spectrum() {
    let path = origin();
    let source = "Scoped spectra\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.end\n";
    let transient = |stop_time| {
        task(
            AnalysisSpec::Transient {
                stop_time,
                step_time: 1e-5,
                start_time: 0.0,
                max_timestep: Some(1e-5),
                uic: false,
            },
            format!(".tran 1e-5 {stop_time}"),
        )
    };
    let short = transient(0.0005);
    let long = transient(0.001);
    let fft_request = FftRequest {
        start: Some(0.0),
        stop: Some(0.001),
        points: 16,
        ..Default::default()
    };
    let mut fft = task(
        AnalysisSpec::Fft {
            request: fft_request.clone(),
        },
        fft_request.to_card(),
    );
    fft.dependencies.push(long.instance_id);
    let snapshot = prepare_headless_run(
        HeadlessRunInput::new(source, &path, vec![short, long, fft]),
        &NoAbort,
    )
    .unwrap();
    let results = execute(snapshot);
    let SimulationResult::Transient { spectra: short, .. } = &results[0] else {
        panic!("short transient")
    };
    let SimulationResult::Transient { spectra: long, .. } = &results[1] else {
        panic!("long transient")
    };
    let SimulationResult::Fft { spectrum, .. } = &results[2] else {
        panic!("FFT")
    };
    assert!(short.is_empty());
    assert_eq!(long.len(), 1);
    assert!(Arc::ptr_eq(spectrum, &long[0]));
}

#[test]
fn generated_transient_noise_settings_are_present_in_the_dispatched_source() {
    let path = origin();
    let spec = AnalysisSpec::TransientNoise {
        stop_time: 1e-5,
        step_time: 1e-7,
        start_time: 0.0,
        max_timestep: 1e-7,
        seed: Some(123),
        noise_fmax: 1e6,
        noise_fmin: None,
        scale: 1.0,
        uic: false,
    };
    let card = crate::analysis_preparation::build_transient_noise_command(&spec).unwrap();
    let snapshot = prepare_headless_run(
        HeadlessRunInput::new(DIVIDER, &path, vec![task(spec, card.clone())]),
        &NoAbort,
    )
    .unwrap();
    let dispatch = PreparedRunAuthorization::default()
        .authorize_campaign_member(snapshot)
        .unwrap();
    assert!(
        dispatch
            .tasks()
            .next()
            .unwrap()
            .executable_netlist()
            .contains(&card)
    );
}

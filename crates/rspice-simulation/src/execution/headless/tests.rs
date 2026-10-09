use super::*;
use crate::execution::PreparedRunAuthorization;
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
    let mut pending = dispatch.into_tasks();
    while let Some(task) = pending.pop_front() {
        let identity = task.instance_id();
        let dispatch = task.resolve_dependency_artifacts(&artifacts).unwrap();
        let producer = dispatch.artifact_producer().unwrap();
        runner.start_prepared(dispatch, false).unwrap();
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
        if let Some(artifact) = producer.capture(&result, &pending).unwrap() {
            artifacts.insert(identity, artifact);
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

#[test]
fn captured_includes_use_the_expanded_budget_after_root_admission() {
    let path = origin();
    let included = path.with_file_name("divider.inc");
    let source = "Expanded budget\n.include divider.inc\nV1 in 0 1\n.end\n";
    let contents = format!(
        "{}R1 in out 1k\nR2 out 0 1k\n",
        "* retained library comment\n".repeat(8)
    );
    assert!(contents.len() > source.len());
    let mut input = HeadlessRunInput::new(source, &path, vec![op()]);
    input.preparation_limits.max_netlist_bytes = source.len();
    input.preparation_limits.max_expanded_source_bytes = 4096;
    input.resolver = HeadlessSourceResolver::Sealed(
        SealedSourceBundle::try_new_with_edges(
            [(path.clone(), source.into()), (included.clone(), contents)],
            [SealedSourceEdge {
                owner: path.clone(),
                requested_path: "divider.inc".into(),
                target: included,
            }],
        )
        .unwrap(),
    );
    let prepared = prepare_headless_run(input, &NoAbort).unwrap();
    assert!((voltage(&execute(prepared)[0]) - 0.5).abs() < 1e-10);
}

#[test]
fn hierarchy_and_generated_task_decks_obey_preparation_limits() {
    let path = origin();
    let hierarchical =
        "Bounded hierarchy\n.subckt cell a b\nR1 a b 1k\nR2 a b 2k\n.ends\nX1 in 0 cell\n.end\n";
    let mut input = HeadlessRunInput::new(hierarchical, &path, vec![op()]);
    input.preparation_limits.max_flattened_elements = 1;
    assert!(matches!(
        prepare_headless_run(input, &NoAbort),
        Err(HeadlessPreparationError::ResourceLimit(
            ResourceLimitError {
                resource: ResourceKind::FlattenedElements,
                requested: 2,
                limit: 1,
            }
        ))
    ));

    // Source admission alone is insufficient: task cards are materialized
    // later, and that final deck needs the same hierarchy policy.
    let mut request = op();
    request
        .analysis
        .analysis_line
        .push_str("\nRextra in out 3k");
    let mut input = HeadlessRunInput::new(DIVIDER, &path, vec![request]);
    input.preparation_limits.max_flattened_elements = 3;
    assert!(matches!(
        prepare_headless_run(input, &NoAbort),
        Err(HeadlessPreparationError::ResourceLimit(
            ResourceLimitError {
                resource: ResourceKind::FlattenedElements,
                requested: 4,
                limit: 3,
            }
        ))
    ));
}

#[test]
fn generated_card_separators_count_towards_the_expanded_source_budget() {
    let path = origin();
    let request = op();
    let mut input = HeadlessRunInput::new(DIVIDER, &path, vec![request]);
    // The card also adds a newline. The final deck must be checked, not just
    // the sum of the original source and the card's content.
    input.preparation_limits.max_expanded_source_bytes = DIVIDER.len() + ".op".len();
    assert!(matches!(
        prepare_headless_run(input, &NoAbort),
        Err(HeadlessPreparationError::ResourceLimit(
            ResourceLimitError {
                resource: ResourceKind::ExpandedSourceBytes,
                ..
            }
        ))
    ));
}

#[test]
fn hierarchy_cancellation_retains_its_type_through_preparation_errors() {
    let parsed = rspice_core::Netlist::parse(DIVIDER).unwrap();
    let failure = validated_parsed_hierarchy_with_limits_and_abort(
        &parsed,
        ResourceLimits::default(),
        &CountingAbort::new(2),
    )
    .unwrap_err();
    assert!(failure.is_aborted());
    assert!(matches!(
        HeadlessPreparationError::from(failure),
        HeadlessPreparationError::Aborted
    ));
}

fn periodic_tasks() -> Vec<HeadlessTaskRequest> {
    let seed = op();
    let mut carrier = task(
        AnalysisSpec::Pss {
            method: rspice_simulation_contract::analysis_spec::PssMethod::Shooting,
            fundamental_freq: 1e3,
            tone_sources: vec!["V1".into()],
            tstab_periods: 1,
            points_per_period: 32,
            tolerance: 1e-6,
            oscillator_mode: false,
            oscillator_node: None,
            num_harmonics: 1,
            integration_method: None,
            tstab: 0.0,
            max_iterations: 20,
            abstol: 1e-12,
            damping: 1.0,
            max_period_change: 0.1,
            verbose: false,
        },
        ".pss FUND=1k HARMS=1 POINTS=32 TSTABPERIODS=1".into(),
    );
    carrier.dependencies.push(seed.instance_id);
    let mut spectrum = task(
        AnalysisSpec::PssSpectrum { num_harmonics: 1 },
        "* retain the exact PSS spectrum".into(),
    );
    spectrum.dependencies.push(carrier.instance_id);
    vec![seed, carrier, spectrum]
}

#[test]
fn shared_artifact_capture_executes_an_op_pss_spectrum_chain() {
    let path = origin();
    let source = "Periodic handoffs\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.end\n";
    let snapshot = prepare_headless_run(
        HeadlessRunInput::new(source, &path, periodic_tasks()),
        &NoAbort,
    )
    .unwrap();
    let results = execute(snapshot);
    assert_eq!(results.len(), 3);
    assert!(matches!(&results[0], SimulationResult::DcOp(_)));
    assert!(matches!(
        &results[1],
        SimulationResult::Transient {
            periodic_state: Some(_),
            ..
        }
    ));
    let SimulationResult::Ac {
        frequencies,
        waveforms,
        ..
    } = &results[2]
    else {
        panic!("PSS spectrum result");
    };
    assert!(frequencies.contains(&1000.0));
    assert!(!waveforms.is_empty());
}

#[test]
fn shared_artifact_capture_rejects_foreign_consumers_and_invalid_results() {
    let path = origin();
    let source = "Bounded handoffs\nV1 out 0 SIN(0 1 1k)\nR1 out 0 1k\n.end\n";
    let requests = periodic_tasks();
    let prepare = |source: &str| {
        let snapshot = prepare_headless_run(
            HeadlessRunInput::new(source, &path, requests.clone()),
            &NoAbort,
        )
        .unwrap();
        PreparedRunAuthorization::default()
            .authorize_campaign_member(snapshot)
            .unwrap()
    };
    let mut tasks = prepare(source).into_tasks();
    let producer = tasks
        .pop_front()
        .unwrap()
        .resolve_dependency_artifacts(&HashMap::new())
        .unwrap()
        .artifact_producer()
        .unwrap();
    let foreign = prepare(&source.replace("1k\n.end", "2k\n.end"));
    let result = SimulationResult::DcOp(Box::new(crate::results::DcOpResult {
        mna_node_names: vec!["OUT".into()],
        mna_solution: vec![f64::NAN],
        ..Default::default()
    }));
    assert!(matches!(
        producer.capture(&result, foreign.tasks()),
        Err(crate::prepared_dependency::ExecutionArtifactError::StaleSnapshot { .. })
    ));
    assert!(
        producer.capture(&result, &tasks).is_err(),
        "a nonfinite OP solution must not become a dependency artifact"
    );
    assert!(
        producer
            .capture(&result, std::iter::empty())
            .unwrap()
            .is_none()
    );
}

use super::*;
use crate::product::{AnalysisInstanceId, ObjectRevision};
use crate::simulation::dialog::McVariationSource;
use crate::simulation::multi_run::AnalysisSpec;
use crate::simulation::runner::study::{
    StudyRunConfig, monte_carlo::checkpoint::StudyMonteCarloCheckpoint,
};
use crate::simulation::runner::worker_contract::*;
use crate::simulation::runner::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

const DECK: &str = "Checkpoint dispatch\n.param r=1k\nV1 in 0 1\nR1 in out {r}\nR2 out 0 1k\n.mc 3 uniform 0.2 seed 37\n.end\n";
fn fixture() -> (SimulationRequest, NetlistInput) {
    let config = crate::simulation::AnalysisConfig::dc_op();
    (
        SimulationRequest::Spec {
            spec: Box::new(AnalysisSpec::MonteCarlo {
                variation_source: McVariationSource::ParameterTolerance,
                params: vec![],
            }),
            options: Box::new(SpecExecutionOptions {
                study_base: Some(StudyRunConfig {
                    postprocess: None,
                    constraints: vec![],
                    objective_terms: vec![],
                    instance_id: AnalysisInstanceId::new(),
                    source_revision: ObjectRevision::INITIAL,
                    analysis_line: config.to_spice(),
                    analysis: config.into(),
                    numeric_options: ".OPTIONS GMIN=0".into(),
                    measurements: vec!["scalar:V(out)".into()],
                    histogram_bins: 5,
                }),
                mc_checkpoint: Some(MonteCarloCheckpointRequest {
                    publish_every: 1.try_into().unwrap(),
                    trial_range: Some(0..1),
                    resume: None,
                }),
                ..Default::default()
            }),
        },
        NetlistInput {
            netlist: DECK.into(),
            source_path: None,
            project_veriloga_runtimes: Default::default(),
            dependencies: Default::default(),
            environment: None,
            stream_transient_samples: false,
        },
    )
}
fn policy(request: &mut SimulationRequest) -> &mut MonteCarloCheckpointRequest {
    let SimulationRequest::Spec { options, .. } = request else {
        unreachable!()
    };
    options.mc_checkpoint.as_mut().unwrap()
}
fn execute(
    request: SimulationRequest,
    input: NetlistInput,
    abort: Arc<AtomicBool>,
    streams: RunStreams,
) -> Result<crate::simulation::results::SimulationResult, SimulationError> {
    run_simulation_thread_with_progress_observer(
        request,
        input,
        Arc::new(Mutex::new(
            crate::simulation::status::SimulationProgress::default(),
        )),
        abort,
        streams,
    )
}
fn captured_request() -> (SimulationRequest, NetlistInput, Vec<u8>) {
    let (request, input) = fixture();
    let abort = Arc::new(AtomicBool::new(false));
    let cancel = abort.clone();
    let queue = Arc::new(Mutex::new(None));
    let result = execute(
        request.clone(),
        input.clone(),
        abort,
        RunStreams {
            monte_carlo_checkpoint: Some(queue.clone()),
            checkpoint_observer: Some(Arc::new(move |_| {
                cancel.store(true, Ordering::SeqCst);
                Ok(())
            })),
            ..Default::default()
        },
    );
    assert!(matches!(result, Err(SimulationError::Aborted)));
    let bytes: Arc<[u8]> = queue
        .lock()
        .unwrap()
        .take()
        .expect("checkpoint precedes cancellation");
    assert_eq!(
        StudyMonteCarloCheckpoint::from_bytes_with_limits(
            &bytes,
            ResourceLimits::default(),
            &NoAbort
        )
        .unwrap()
        .completed_trials(),
        1
    );
    (request, input, bytes.to_vec())
}

/// One real trial shared by controller and project-persistence boundary tests.
pub(crate) fn completed_checkpoint_fixture() -> (
    SimulationRequest,
    Arc<[u8]>,
    crate::simulation::results::SimulationResult,
) {
    let (request, input) = fixture();
    let queue = Arc::new(Mutex::new(None));
    let result = execute(
        request.clone(),
        input,
        Arc::new(AtomicBool::new(false)),
        RunStreams {
            monte_carlo_checkpoint: Some(queue.clone()),
            ..Default::default()
        },
    )
    .unwrap();
    let bytes = queue.lock().unwrap().take().unwrap();
    (request, bytes, result)
}

#[test]
fn monte_carlo_checkpoint_dispatch_transfers_cancelled_population_and_resumes_missing_trials() {
    let (mut request, input, bytes) = captured_request();
    policy(&mut request).trial_range = Some(0..3);
    policy(&mut request).resume =
        Some(MonteCarloCheckpointInput::from_bytes(bytes.clone()).unwrap());
    let wire = WorkerRequest::from_runner_parts(17, &request, &input).unwrap();
    let mut transport = WorkerRequestTransport::from_request(wire.clone()).unwrap();
    assert_eq!(transport.byte_buffers, [bytes]);
    assert!(transport.buffers.is_empty());
    let json = serde_json::to_string(&transport.request).unwrap();
    assert!(!json.contains("\"bytes\""));
    transport.request = serde_json::from_str(&json).unwrap();
    let restored = transport.into_request().unwrap();
    assert_eq!(restored, wire);
    let (request, input) = restored.into_runner_parts();
    let queue = Arc::new(Mutex::new(None));
    let calls = Arc::new(AtomicUsize::new(0));
    let observed = calls.clone();
    let result = execute(
        request,
        input,
        Arc::new(AtomicBool::new(false)),
        RunStreams {
            monte_carlo_checkpoint: Some(queue.clone()),
            checkpoint_observer: Some(Arc::new(move |_| {
                observed.fetch_add(1, Ordering::Relaxed);
                Ok(())
            })),
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(calls.load(Ordering::Relaxed), 2);
    let bytes: Arc<[u8]> = queue.lock().unwrap().take().unwrap();
    let retained = StudyMonteCarloCheckpoint::from_bytes_with_limits(
        &bytes,
        ResourceLimits::default(),
        &NoAbort,
    )
    .unwrap();
    assert_eq!(retained.completed_indices().collect::<Vec<_>>(), [0, 1, 2]);
    let crate::simulation::results::SimulationResult::MonteCarlo {
        runs_completed,
        member_measurements,
        ..
    } = result
    else {
        panic!("MC result")
    };
    assert_eq!(runs_completed, 3);
    assert_eq!(member_measurements.len(), 3);
}

#[test]
fn monte_carlo_checkpoint_dispatch_rejects_missing_corrupt_duplicate_and_over_budget_inputs() {
    let (mut request, input, bytes) = captured_request();
    policy(&mut request).resume = Some(MonteCarloCheckpointInput::from_bytes(bytes).unwrap());
    let wire = WorkerRequest::from_runner_parts(18, &request, &input).unwrap();
    let good = WorkerRequestTransport::from_request(wire).unwrap();
    for mutation in 0..4 {
        let mut transfer = good.clone();
        match mutation {
            0 => transfer.byte_buffers.clear(),
            1 => {
                transfer.byte_buffers[0][0] ^= 1;
            }
            2 => transfer.byte_buffers.push(transfer.byte_buffers[0].clone()),
            _ => {
                let WorkerSimulationRequest::Spec { options, .. } =
                    &mut transfer.request.request.request
                else {
                    unreachable!()
                };
                options.mc_checkpoint.as_mut().unwrap().resume = None;
            }
        }
        assert!(transfer.into_request().is_err());
    }
    assert!(validate_worker_request_checkpoint_lengths(0, &[usize::MAX]).is_err());
    assert!(
        validate_worker_request_checkpoint_lengths(
            16_777_216,
            &[ResourceLimits::default().max_external_data_bytes]
        )
        .is_err()
    );
    assert!(validate_worker_request_checkpoint_lengths(0, &[0]).is_err());
    let error = execute(
        request,
        input,
        Arc::new(AtomicBool::new(false)),
        RunStreams::default(),
    )
    .unwrap_err();
    assert!(error.to_string().contains("destination"));
}

#[test]
fn monte_carlo_checkpoint_dispatch_preparation_identity_binds_capture_range_and_input() {
    use crate::simulation::controller::QueuedAnalysis;
    use crate::simulation::execution::PreparedTask;
    let (mut request, _, bytes) = captured_request();
    let digest = |request: &SimulationRequest| {
        let SimulationRequest::Spec { spec, options } = request else {
            unreachable!()
        };
        PreparedTask::new(
            AnalysisInstanceId::new(),
            ObjectRevision::INITIAL,
            vec![],
            "MC",
            QueuedAnalysis {
                spec: *spec.clone(),
                config: None,
                spec_options: *options.clone(),
                analysis_line: ".mc 3 uniform 0.2 seed 37".into(),
                numeric_override: None,
            },
        )
        .config_digest()
    };
    let original = digest(&request);
    policy(&mut request).publish_every = 2.try_into().unwrap();
    assert_ne!(original, digest(&request));
    policy(&mut request).publish_every = 1.try_into().unwrap();
    assert_eq!(original, digest(&request));
    policy(&mut request).trial_range = Some(1..2);
    assert_ne!(original, digest(&request));
    policy(&mut request).trial_range = Some(0..1);
    policy(&mut request).resume = Some(MonteCarloCheckpointInput::from_bytes(bytes).unwrap());
    assert_ne!(original, digest(&request));
}

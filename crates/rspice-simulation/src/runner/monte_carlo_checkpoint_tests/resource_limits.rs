//! Captured policies govern fresh trials, decoded resumes, and publication.
use super::*;

fn bounded_limits() -> ResourceLimits {
    let mut limits = ResourceLimits::default();
    limits.max_matrix_unknowns = 32;
    limits.max_analysis_points = 5;
    limits.max_batch_runs = 3;
    limits.max_result_values = 1000;
    limits.max_external_data_bytes = 64 * 1024;
    limits.max_parallel_workers = 1;
    limits
}

fn run_and_capture(
    request: SimulationRequest,
    input: NetlistInput,
) -> (
    Result<crate::results::SimulationResult, SimulationError>,
    Option<Arc<[u8]>>,
) {
    let queue = Arc::new(Mutex::new(None));
    let result = execute(
        request,
        input,
        Arc::new(AtomicBool::new(false)),
        RunStreams {
            monte_carlo_checkpoint: Some(queue.clone()),
            ..Default::default()
        },
    );
    let bytes = queue.lock().unwrap().take();
    (result, bytes)
}

#[test]
fn monte_carlo_custom_limits_survive_worker_transport_and_cached_resume() {
    for selected_base in [false, true] {
        for variation in McVariationSource::ALL {
            let (mut request, mut input) = fixture();
            let SimulationRequest::Spec { spec, options } = &mut request else {
                unreachable!()
            };
            **spec = AnalysisSpec::MonteCarlo {
                variation_source: variation,
                params: vec![],
            };
            if !selected_base {
                options.study_base = None;
            }
            options.mc_histogram_bins = Some(5);
            if variation == McVariationSource::DeckStatistics {
                input.netlist = input.netlist.replace("r=1k", "r={unif(1k,0.2)}");
            }
            policy(&mut request).trial_range = Some(0..3);
            let (expected, _) = run_and_capture(request.clone(), input.clone());
            let expected = WorkerSimulationResult::try_from(expected.unwrap()).unwrap();
            input.execution_limits = bounded_limits();
            let wire = WorkerRequest::from_runner_parts(71, &request, &input).unwrap();
            let transport = WorkerRequestTransport::from_request(wire).unwrap();
            let (restored_request, restored_input) =
                transport.into_request().unwrap().into_runner_parts();
            assert_eq!(restored_input.execution_limits, input.execution_limits);
            let (actual, bytes) = run_and_capture(restored_request, restored_input);
            assert_eq!(
                WorkerSimulationResult::try_from(actual.unwrap()).unwrap(),
                expected
            );
            policy(&mut request).resume =
                Some(MonteCarloCheckpointInput::from_bytes(bytes.unwrap().to_vec()).unwrap());
            let (resumed, bytes) = run_and_capture(request, input);
            assert_eq!(
                WorkerSimulationResult::try_from(resumed.unwrap()).unwrap(),
                expected
            );
            let retained = StudyMonteCarloCheckpoint::from_bytes_with_limits(
                &bytes.unwrap(),
                bounded_limits(),
                &NoAbort,
            )
            .unwrap();
            assert_eq!(retained.completed_indices().collect::<Vec<_>>(), [0, 1, 2]);
        }
    }
}

#[test]
fn monte_carlo_rejects_tight_solver_batch_and_publication_limits_with_typed_errors() {
    for selected_base in [false, true] {
        for resource in [
            "matrix_unknowns",
            "analysis_points",
            "batch_runs",
            "result_values",
            "external_data_bytes",
        ] {
            let (mut request, mut input) = fixture();
            let SimulationRequest::Spec { options, .. } = &mut request else {
                unreachable!()
            };
            if !selected_base {
                options.study_base = None;
            }
            options.mc_histogram_bins = Some(5);
            policy(&mut request).trial_range = Some(0..3);
            let mut limits = bounded_limits();
            match resource {
                "matrix_unknowns" => limits.max_matrix_unknowns = 1,
                "analysis_points" => limits.max_analysis_points = 4,
                "batch_runs" => limits.max_batch_runs = 2,
                "result_values" => limits.max_result_values = 1,
                "external_data_bytes" => limits.max_external_data_bytes = 1,
                _ => unreachable!(),
            }
            input.execution_limits = limits;
            let (result, published) = run_and_capture(request, input);
            let error = result.unwrap_err();
            assert!(
                matches!(&error, SimulationError::ResourceLimit { resource: actual, requested, limit }
                if actual == resource && requested > limit),
                "base={selected_base}, {resource}: {error:?}"
            );
            assert!(
                published.is_none(),
                "failed admission must not publish a checkpoint"
            );
        }
    }
}

#[test]
fn monte_carlo_resume_decode_uses_captured_byte_and_value_limits() {
    let (mut request, mut input, bytes) = captured_request();
    policy(&mut request).resume =
        Some(MonteCarloCheckpointInput::from_bytes(bytes.clone()).unwrap());
    for resource in ["external_data_bytes", "result_values"] {
        input.execution_limits = bounded_limits();
        match resource {
            "external_data_bytes" => {
                input.execution_limits.max_external_data_bytes = bytes.len() - 1
            }
            "result_values" => input.execution_limits.max_result_values = 1,
            _ => unreachable!(),
        }
        let (result, published) = run_and_capture(request.clone(), input.clone());
        let error = result.unwrap_err();
        assert!(
            matches!(&error, SimulationError::ResourceLimit { resource: actual, requested, limit }
            if actual == resource && requested > limit),
            "{error:?}"
        );
        assert!(published.is_none());
    }
    let checkpoint = policy(&mut request).resume.as_ref().unwrap();
    assert_eq!(
        checkpoint
            .decode_with_limits(
                ResourceLimits::default(),
                &rspice_core::abort_signal::ImmediateAbort
            )
            .unwrap_err(),
        SimulationError::Aborted
    );
}

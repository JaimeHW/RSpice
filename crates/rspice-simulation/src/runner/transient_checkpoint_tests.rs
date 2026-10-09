//! Mixed transient continuation through the product request and worker transport.
use super::worker_contract::*;
use super::*;
use crate::transient_checkpoint::{TransientCheckpointInput, TransientCheckpointRequest};
use rspice_core::{NoAbort, ResourceLimits};

const MODEL: &str = r#"
`include "disciplines.vams"
module worker_checkpoint_divider(p, n, qdiv);
    inout p, n;
    electrical p, n;
    output qdiv;
    reg clk, qdiv;
    initial clk = 1'b0;
    initial qdiv = 1'b0;
    always #5 clk = ~clk;
    always @(posedge clk) qdiv <= ~qdiv;
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

fn fixture() -> (SimulationRequest, NetlistInput) {
    let file = "worker_checkpoint.vams";
    let module = "worker_checkpoint_divider";
    let bundle = rspice_veriloga::VirtualSourceBundle::new(
        file,
        [rspice_veriloga::VirtualSourceFile::new(file, MODEL)],
    )
    .unwrap();
    let compilation = rspice_veriloga::VerilogACompiler::new(
        crate::compilation::unified_runtime_compiler_options(),
    )
    .compile_virtual_runtime(
        &bundle,
        module,
        rspice_veriloga::VirtualCompileLimits::default(),
    )
    .unwrap();
    let runtime = crate::veriloga::PreparedVerilogARuntime::try_from_virtual_compilation(
        format!("__rspice_project__/checkpoint/{file}"),
        rspice_app_types::canonical::content_digest("checkpoint-test-source/v1", MODEL.as_bytes()),
        module.into(),
        &compilation,
    )
    .unwrap();
    let mut deck = format!(
        "Mixed worker continuation\nX1 p 0 qdiv {module}\nRp p 0 1meg\nR1 qdiv out 1k\nC1 out 0 10p\n.tran 1n 200n\n.end\n"
    );
    crate::netlist_preparation::append_project_veriloga_directive(
        &mut deck,
        runtime.source_key(),
        runtime.module_name(),
    );
    (
        SimulationRequest::Spec {
            spec: Box::new(AnalysisSpec::Transient {
                stop_time: 200e-9,
                step_time: 1e-9,
                start_time: 0.0,
                max_timestep: Some(1e-9),
                uic: false,
            }),
            options: Box::new(SpecExecutionOptions {
                tran_checkpoint: Some(TransientCheckpointRequest {
                    times: vec![100e-9],
                    resume: None,
                }),
                ..Default::default()
            }),
        },
        NetlistInput {
            netlist: deck,
            source_path: None,
            project_veriloga_runtimes: crate::veriloga::PreparedVerilogARuntimeSet::try_new(vec![
                runtime,
            ])
            .unwrap(),
            measurement_references: Default::default(),
            dependencies: Default::default(),
            environment: None,
            stream_transient_samples: false,
            execution_limits: ResourceLimits::default(),
        },
    )
}

fn policy(request: &mut SimulationRequest) -> &mut TransientCheckpointRequest {
    let SimulationRequest::Spec { options, .. } = request else {
        unreachable!()
    };
    options.tran_checkpoint.as_mut().unwrap()
}

fn execute(
    request: SimulationRequest,
    input: NetlistInput,
    abort: Arc<AtomicBool>,
    streams: RunStreams,
) -> Result<SimulationResult, SimulationError> {
    run_simulation_thread_with_progress_observer(
        request,
        input,
        Arc::new(Mutex::new(SimulationProgress::default())),
        abort,
        streams,
    )
}

fn captured() -> (SimulationRequest, NetlistInput, Vec<u8>) {
    let (request, input) = fixture();
    let abort = Arc::new(AtomicBool::new(false));
    let cancel = abort.clone();
    let queue = Arc::new(Mutex::new(None));
    let mc_queue = Arc::new(Mutex::new(None));
    let result = execute(
        request.clone(),
        input.clone(),
        abort,
        RunStreams {
            transient_checkpoint: Some(queue.clone()),
            monte_carlo_checkpoint: Some(mc_queue.clone()),
            transient_checkpoint_observer: Some(Arc::new(move |_| {
                cancel.store(true, Ordering::SeqCst);
                Ok(())
            })),
            ..Default::default()
        },
    );
    assert!(
        matches!(result, Err(SimulationError::Aborted)),
        "{result:?}"
    );
    assert!(mc_queue.lock().unwrap().is_none());
    let bytes: Arc<[u8]> = queue
        .lock()
        .unwrap()
        .take()
        .expect("published before cancellation");
    (request, input, bytes.to_vec())
}

fn assert_continuation(baseline: &SimulationResult, resumed: &SimulationResult, saved_time: f64) {
    let SimulationResult::Transient {
        time: expected_time,
        waveforms: expected,
        events: expected_events,
        ..
    } = baseline
    else {
        panic!("transient")
    };
    let SimulationResult::Transient {
        time,
        waveforms,
        events,
        ..
    } = resumed
    else {
        panic!("transient")
    };
    let seam = expected_time
        .iter()
        .position(|t| t.to_bits() == saved_time.to_bits())
        .unwrap();
    assert_eq!(time, &expected_time[seam..]);
    assert_eq!(waveforms.len(), expected.len());
    for (name, actual) in waveforms {
        let expected = &expected[name];
        assert_eq!(actual.x_values, expected.x_values[seam..], "{name}");
        assert_eq!(
            actual
                .y_values
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            expected.y_values[seam..]
                .iter()
                .map(|v| v.to_bits())
                .collect::<Vec<_>>(),
            "{name}"
        );
    }
    assert!(!events.digital.is_empty());
    assert_eq!(events.digital.len(), expected_events.digital.len());
    for actual in &events.digital {
        let expected = expected_events
            .digital
            .iter()
            .find(|e| e.node_name == actual.node_name)
            .unwrap();
        let held = expected
            .points
            .iter()
            .rev()
            .find(|p| p.time_s <= saved_time)
            .unwrap();
        assert_eq!(actual.points[0].time_s, saved_time);
        assert_eq!(actual.points[0].value_code, held.value_code);
        assert_eq!(
            actual
                .points
                .iter()
                .filter(|p| p.time_s > saved_time)
                .collect::<Vec<_>>(),
            expected
                .points
                .iter()
                .filter(|p| p.time_s > saved_time)
                .collect::<Vec<_>>()
        );
    }
}

#[test]
fn transient_checkpoint_worker_roundtrip_resumes_mixed_events_and_extended_horizon() {
    let (mut request, input, bytes) = captured();
    let saved = crate::transient_checkpoint::decode_bytes(&bytes, input.execution_limits, &NoAbort)
        .unwrap();
    policy(&mut request).resume =
        Some(TransientCheckpointInput::from_bytes(bytes.clone()).unwrap());
    policy(&mut request).times = vec![150e-9];
    let wire = WorkerRequest::from_runner_parts(73, &request, &input).unwrap();
    let mut transfer = WorkerRequestTransport::from_request(wire.clone()).unwrap();
    assert_eq!(transfer.byte_buffers, [bytes]);
    assert!(transfer.buffers.is_empty());
    let metadata = serde_json::to_string(&transfer.request).unwrap();
    assert!(!metadata.contains("\"bytes\""));
    transfer.request = serde_json::from_str(&metadata).unwrap();
    let received = transfer.into_request().unwrap();
    assert_eq!(wire, received);
    let (request, input) = received.into_runner_parts();
    for stop in [200e-9, 400e-9] {
        let mut resumed_request = request.clone();
        let SimulationRequest::Spec { spec, .. } = &mut resumed_request else {
            unreachable!()
        };
        let AnalysisSpec::Transient { stop_time, .. } = spec.as_mut() else {
            unreachable!()
        };
        *stop_time = stop;
        let mut baseline_request = resumed_request.clone();
        policy(&mut baseline_request).resume = None;
        let baseline_queue = Arc::new(Mutex::new(None));
        let baseline = execute(
            baseline_request,
            input.clone(),
            Arc::new(AtomicBool::new(false)),
            RunStreams {
                transient_checkpoint: Some(baseline_queue.clone()),
                ..Default::default()
            },
        )
        .unwrap();
        let resumed_queue = Arc::new(Mutex::new(None));
        let resumed = execute(
            resumed_request,
            input.clone(),
            Arc::new(AtomicBool::new(false)),
            RunStreams {
                transient_checkpoint: Some(resumed_queue.clone()),
                ..Default::default()
            },
        )
        .unwrap();
        assert_continuation(&baseline, &resumed, saved.time);
        if stop == 200e-9 {
            let mut resume_only = request.clone();
            policy(&mut resume_only).times.clear();
            let resumed_without_sink = execute(
                resume_only,
                input.clone(),
                Arc::new(AtomicBool::new(false)),
                RunStreams::default(),
            ).unwrap();
            assert_continuation(&baseline, &resumed_without_sink, saved.time);
            // Complete pending process, bridge, solver and controller state agrees too.
            let decode = |q: &crate::transient_checkpoint::CheckpointQueue| {
                crate::transient_checkpoint::decode_bytes(
                    q.lock().unwrap().as_ref().unwrap(),
                    input.execution_limits,
                    &NoAbort,
                )
                .unwrap()
            };
            assert_eq!(decode(&baseline_queue), decode(&resumed_queue));
        }
    }
    for change in 0..3 {
        let mut request = request.clone();
        let mut changed_input = input.clone();
        match change {
            0 => {
                changed_input.netlist = changed_input
                    .netlist
                    .replace("R1 qdiv out 1k", "R1 qdiv out 2k")
            }
            1 => {
                let SimulationRequest::Spec { spec, .. } = &mut request else {
                    unreachable!()
                };
                let AnalysisSpec::Transient { uic, .. } = spec.as_mut() else {
                    unreachable!()
                };
                *uic = true;
            }
            _ => policy(&mut request).times = vec![saved.time / 2.0],
        }
        let queue = Arc::new(Mutex::new(None));
        assert!(
            execute(
                request,
                changed_input,
                Arc::new(AtomicBool::new(false)),
                RunStreams {
                    transient_checkpoint: Some(queue.clone()),
                    ..Default::default()
                }
            )
            .is_err()
        );
        assert!(queue.lock().unwrap().is_none());
    }
}

#[test]
fn transient_checkpoint_worker_rejects_corrupt_duplicate_and_over_budget_transfers() {
    let (mut request, input, bytes) = captured();
    policy(&mut request).resume =
        Some(TransientCheckpointInput::from_bytes(bytes.clone()).unwrap());
    let wire = WorkerRequest::from_runner_parts(74, &request, &input).unwrap();
    let good = WorkerRequestTransport::from_request(wire.clone()).unwrap();
    for mutation in 0..7 {
        let mut transfer = good.clone();
        match mutation {
            0 => transfer.byte_buffers.clear(),
            1 => transfer.byte_buffers[0][0] ^= 1,
            2 => transfer.byte_buffers.push(transfer.byte_buffers[0].clone()),
            3 => transfer.request.request.request = wire.request.clone(), // duplicate inline bytes
            4 => {
                transfer
                    .request
                    .request
                    .execution_limits
                    .max_external_data_bytes = bytes.len() - 1
            }
            5 => transfer.request.request.execution_limits.max_result_values = 1,
            _ => {
                let WorkerSimulationRequest::Spec { options, .. } =
                    &mut transfer.request.request.request
                else {
                    unreachable!()
                };
                options.mc_checkpoint =
                    Some(crate::monte_carlo_checkpoint::MonteCarloCheckpointRequest {
                        publish_every: 1.try_into().unwrap(),
                        trial_range: None,
                        resume: None,
                    });
            }
        }
        assert!(
            transfer.into_request().is_err(),
            "accepted mutation {mutation}"
        );
    }
    assert!(validate_checkpoint_request_lengths(&wire, usize::MAX, &[bytes.len()]).is_err());
    let mut limited = input.execution_limits;
    // Packed input fits, but expanded text / retained parser storage must fit too.
    limited.max_external_data_bytes = bytes.len();
    assert!(crate::transient_checkpoint::decode_bytes(&bytes, limited, &NoAbort).is_err());
    let aborted = rspice_core::abort_signal::AtomicAbort::new();
    aborted.set();
    assert!(matches!(
        crate::transient_checkpoint::decode_bytes(&bytes, input.execution_limits, &aborted),
        Err(SimulationError::Aborted)
    ));
}

#[test]
fn transient_checkpoint_request_identity_and_destination_failures_are_preserved() {
    let (mut request, input) = fixture();
    let digest = |request: &SimulationRequest| {
        let SimulationRequest::Spec { spec, options } = request else {
            unreachable!()
        };
        crate::execution_identity::analysis_config_digest(
            ".tran 1n 200n",
            spec,
            None,
            options,
            None,
        )
    };
    let original = digest(&request);
    policy(&mut request).times = vec![120e-9];
    assert_ne!(digest(&request), original);
    policy(&mut request).times = vec![100e-9];
    assert_eq!(digest(&request), original);
    let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let observed = calls.clone();
    let result = execute(
        request.clone(),
        input.clone(),
        Arc::new(AtomicBool::new(false)),
        RunStreams {
            transient_checkpoint_observer: Some(Arc::new(move |_| {
                observed.fetch_add(1, Ordering::Relaxed);
                Err(SimulationError::ResourceLimit {
                    resource: "snapshot_destination".into(),
                    requested: 2,
                    limit: 1,
                })
            })),
            ..Default::default()
        },
    );
    assert!(
        matches!(result, Err(SimulationError::ResourceLimit { resource, .. }) if resource == "snapshot_destination")
    );
    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert!(
        execute(
            request,
            input,
            Arc::new(AtomicBool::new(false)),
            RunStreams::default()
        )
        .unwrap_err()
        .to_string()
        .contains("destination")
    );
}

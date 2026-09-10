//! Transfer-buffer round trips, authentication, and ingress limits.

use super::*;

#[test]
fn worker_transport_extracts_every_retained_pss_numeric_array_from_metadata() {
    let response = WorkerResponse {
        id: 78,
        outcome: WorkerOutcome::Success(Box::new(WorkerSimulationResult::Pss {
            measurements: Vec::new(),
            operating_point: super::super::tests::retained_pss_operating_point(),
        })),
    };
    let transport = WorkerResponseTransport::from_response(response.clone()).unwrap();
    let metadata = serde_json::to_string(&transport.response).unwrap();
    assert!(
        metadata.len() < 4_096,
        "PSS samples leaked into worker metadata"
    );
    assert!(!metadata.contains("\"Inline\""));
    assert_eq!(transport.buffers.len(), 8);

    let WorkerOutcomeTransport::Success(WorkerSimulationResultTransport::Pss {
        operating_point: periodic,
        ..
    }) = &transport.response.outcome
    else {
        panic!("expected retained PSS transport metadata")
    };
    assert!(matches!(
        periodic.result_time,
        WorkerF64Series::Buffer { .. }
    ));
    assert!(matches!(
        periodic.result_waveforms[0].values,
        WorkerF64Series::Buffer { .. }
    ));
    assert!(matches!(
        periodic.analysis_monodromy[0],
        WorkerF64Series::Buffer { .. }
    ));
    assert!(matches!(
        periodic.shooting_state,
        WorkerF64Series::Buffer { .. }
    ));

    assert_eq!(transport.into_response().unwrap(), response);
}

#[test]
fn worker_pss_branch_currents_round_trip_and_reject_tamper() {
    use rspice_core::{Engine, Netlist};
    let deck = Netlist::parse(
        "PSS worker currents\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\nC1 out 0 1p\n.end\n",
    )
    .unwrap();
    let point = Engine::default()
        .run_pss_operating_point_with_abort(
            &deck,
            rspice_core::analysis::PssConfig::new(1e6)
                .with_points_per_period(32)
                .with_tstab_periods(0),
            &rspice_core::abort_signal::NoAbort,
        )
        .unwrap();
    let response = WorkerResponse {
        id: 178,
        outcome: WorkerOutcome::Success(Box::new(WorkerSimulationResult::Pss {
            measurements: Vec::new(),
            operating_point: point.clone(),
        })),
    };
    let transport = WorkerResponseTransport::from_response(response.clone()).unwrap();
    let WorkerOutcomeTransport::Success(WorkerSimulationResultTransport::Pss {
        operating_point,
        ..
    }) = &transport.response.outcome
    else {
        panic!("PSS transport");
    };
    assert_eq!(operating_point.result_branch_waveforms[0].node_name, "V1");
    let WorkerF64Series::Buffer { buffer, .. } = operating_point.result_branch_waveforms[0].values
    else {
        panic!("branch samples must be transferable");
    };
    assert_eq!(
        transport.buffers[buffer],
        point.analysis().result.branch_waveforms[0].values
    );
    assert_eq!(transport.clone().into_response().unwrap(), response);
    let display = simulation_result_from_worker_pss(Vec::new(), point);
    let SimulationResult::Transient {
        waveforms,
        periodic_state: Some(point),
        time,
        ..
    } = display
    else {
        panic!("PSS display");
    };
    assert_eq!(waveforms["I(V1)"].y_unit, "A");
    validate_pss_display_contract(&time, &waveforms, &point).unwrap();
    let mut tampered = transport;
    tampered.buffers[buffer][0] += 1.0;
    assert!(
        tampered
            .into_response()
            .unwrap_err()
            .contains("numerical payload does not match")
    );
}

#[test]
fn worker_transport_extracts_and_authenticates_dc_op_mna_solution() {
    let configuration = super::super::tests::nondefault_op_config();
    let response = WorkerResponse {
        id: 79,
        outcome: WorkerOutcome::Success(Box::new(WorkerSimulationResult::DcOp {
            configuration,
            validated_startup_directives: 0,
            mna_node_names: vec!["out".to_owned()],
            mna_branch_names: vec!["V1".to_owned()],
            mna_solution: vec![1.25, -0.001],
            node_voltages: HashMap::from([("out".to_owned(), 1.25)]),
            branch_currents: HashMap::from([("V1".to_owned(), -0.001)]),
            device_report: None,
        })),
    };
    let mut transport = WorkerResponseTransport::from_response(response.clone()).unwrap();
    let metadata = serde_json::to_string(&transport.response).unwrap();
    assert!(!metadata.contains("\"Inline\""));
    assert!(
        metadata.contains("\"mna_solution\":{\"Buffer\""),
        "MNA state must be represented only by a transferable buffer reference"
    );
    assert!(metadata.contains("\"previous_state\":{"));
    assert!(!metadata.contains("\"solution\":[1.25"));
    assert_eq!(
        transport.buffers,
        vec![vec![1.25, -0.001], vec![1.25, -0.001]]
    );
    assert_eq!(transport.clone().into_response().unwrap(), response);

    let mut tampered = transport.clone();
    tampered.buffers[1][0] = 1.5;
    assert!(
        tampered
            .into_response()
            .unwrap_err()
            .contains("payload digest")
    );

    let mut nonfinite = transport.clone();
    nonfinite.buffers[1][0] = f64::NAN;
    assert!(nonfinite.into_response().is_err());

    let mut previous_state_tamper = transport.clone();
    previous_state_tamper.buffers[0][0] = 1.5;
    assert!(
        previous_state_tamper
            .into_response()
            .unwrap_err()
            .contains("previous-state solution digest")
    );

    let WorkerOutcomeTransport::Success(WorkerSimulationResultTransport::DcOp {
        mna_solution, ..
    }) = &mut transport.response.outcome
    else {
        panic!("expected DC OP transport")
    };
    *mna_solution = WorkerF64Series::Buffer {
        buffer: 1,
        len: MAX_WORKER_F64_VALUES + 1,
    };
    assert!(transport.into_response().unwrap_err().contains("exceeding"));
}

#[test]
fn worker_request_ingress_limits_are_checked_before_copy() {
    assert_eq!(checked_worker_request_numeric_total(10, 0, 20).unwrap(), 30);
    assert!(
        checked_worker_request_numeric_total(0, 0, MAX_WORKER_F64_VALUES + 1)
            .unwrap_err()
            .contains("buffer 0")
    );
    assert!(
        checked_worker_request_numeric_total(MAX_WORKER_F64_VALUES, 1, 1)
            .unwrap_err()
            .contains("more than")
    );
    assert!(
        validate_worker_request_transfer_buffer_lengths([MAX_WORKER_F64_VALUES, 1])
            .unwrap_err()
            .contains("more than")
    );
}

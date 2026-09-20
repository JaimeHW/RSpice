//! QPXF editor, native interchange and worker requests drive the same core service.
use super::*;
use crate::simulation::plan::{
    QpssDraft, QpxfSidebandSelection, QpxfSourceSelection, QuasiPeriodicTransferDraft,
};
use crate::simulation::runner::worker_contract::WorkerAnalysisSpec;
use rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod as Method;
use rspice_core::engine::{QpxfFrequencyAxis, QpxfInputLattices, QpxfOutput, QpxfSources};
fn authored() -> QuasiPeriodicTransferDraft {
    QuasiPeriodicTransferDraft {
        explicit_frequencies: "-100, 0, 117".into(),
        frequency_axis: QpxfFrequencyAxis::Output,
        source_selection: QpxfSourceSelection::Named,
        input_sources: "Iprobe\nV1".into(),
        current_output: true,
        output_branch: "L1".into(),
        sideband_selection: QpxfSidebandSelection::Explicit,
        input_lattices: "1, -1, 0; 0, 0, 0".into(),
        output_lattice: "1, -1, 0".into(),
        group_delay: true,
        group_delay_magnitude_floor: "1e-8".into(),
        linear_method: Method::Krylov,
        krylov_restart: "16".into(),
        krylov_cycles: "12".into(),
        linear_tolerance: "2e-11".into(),
        ..Default::default()
    }
}
#[test]
fn qpxf_controls_survive_saved_draft_worker_card_and_authenticated_service() {
    let draft: QuasiPeriodicTransferDraft =
        ron::from_str(&ron::to_string(&authored()).unwrap()).unwrap();
    let spec = draft.to_spec().unwrap();
    let worker = WorkerAnalysisSpec::try_from(&spec).unwrap();
    let restored: AnalysisSpec =
        serde_json::from_str::<WorkerAnalysisSpec>(&serde_json::to_string(&worker).unwrap())
            .unwrap()
            .into();
    assert_eq!(restored, spec);
    let card = restored.qpxf_card().unwrap();
    assert_eq!(AnalysisSpec::from_qpxf_card(&card).unwrap(), spec);
    let mut producer = QpssDraft::default();
    producer.tones = "1k, 1414.2135623730951, 1732.0508075688772".into();
    producer.harmonics = "1, 1, 1".into();
    producer.relative_tolerance = "3e-8".into();
    let deck = "QPXF Studio\nV1 in 0 DC 1\nIprobe 0 out DC 0\nR1 in out 1k\nL1 out 0 1m\nC1 out 0 100n\n.temp 42\n.end\n";
    let point = svc_runner::run_qpss_analysis_with_source_path_and_abort(
        deck,
        producer.to_spec().unwrap().driven_qpss_config().unwrap(),
        None,
        &rspice_core::NoAbort,
    )
    .unwrap()
    .operating_point;
    let result = svc_runner::run_qpxf_analysis_from_qpss_with_source_path_and_abort(
        deck,
        &card,
        &point,
        None,
        &rspice_core::NoAbort,
    )
    .unwrap();
    let request = &result.metadata.request;
    assert_eq!(
        result.metadata.operating_point_identity,
        point.retained_identity()
    );
    assert_eq!(request.frequencies_hz, [-100.0, 0.0, 117.0]);
    assert_eq!(
        request.output,
        QpxfOutput::BranchCurrent {
            branch: "L1".into()
        }
    );
    assert_eq!(
        request.input_sources,
        QpxfSources::Named(vec!["Iprobe".into(), "V1".into()])
    );
    assert_eq!(
        request.input_lattices,
        QpxfInputLattices::Explicit(vec![vec![1, -1, 0], vec![0, 0, 0]])
    );
    assert_eq!(request.linear.method, Method::Krylov);
    assert_eq!(request.linear.restart, 16);
    assert_eq!(request.linear.max_cycles, 12);
    assert_eq!(request.linear.relative_tolerance, 2e-11);
    assert_eq!(request.group_delay_magnitude_floor, 1e-8);
    assert!(request.group_delay);
    assert_eq!(result.transfers.len(), 4);
    for transfer in &result.transfers {
        assert!(transfer.group_delay.is_some());
        for (i, value) in transfer.values.iter().enumerate() {
            let jw = rspice_core::Complex64::new(
                0.0,
                std::f64::consts::TAU * result.metadata.output_frequencies_hz[i],
            );
            let expected = if transfer.input_lattice == [1, -1, 0] {
                rspice_core::Complex64::new(
                    if transfer.input_source == 0 {
                        1.0
                    } else {
                        0.001
                    },
                    0.0,
                ) / (rspice_core::Complex64::ONE
                    + (rspice_core::Complex64::new(0.001, 0.0) + jw * 1e-7) * jw * 0.001)
            } else {
                rspice_core::Complex64::ZERO
            };
            assert!((*value - expected).norm() < 1e-9);
        }
    }
    assert!(
        svc_runner::run_qpxf_analysis_from_qpss_with_source_path_and_abort(
            &deck.replace("R1 in out 1k", "R1 in out 2k"),
            &card,
            &point,
            None,
            &rspice_core::NoAbort
        )
        .is_err()
    );
}
#[test]
fn qpxf_controls_restore_legacy_defaults_and_only_parse_active_fields() {
    let defaults = QuasiPeriodicTransferDraft::default();
    let legacy = defaults.to_spec().unwrap();
    let mut json = serde_json::to_value(&legacy).unwrap();
    json["Qpxf"].as_object_mut().unwrap().remove("controls");
    assert_eq!(
        serde_json::from_value::<AnalysisSpec>(json).unwrap(),
        legacy
    );
    let old: QuasiPeriodicTransferDraft = serde_json::from_value(serde_json::json!({"sweep":defaults.sweep,"input_source":"V1","output_node":"out","output_ref":"0","input_lattice":"0, 0","output_lattice":"0, 0","group_delay":false})).unwrap();
    assert_eq!(old.to_spec().unwrap(), legacy);
    let mut draft = authored();
    draft.sweep.start = "unfinished".into();
    draft.sweep.stop = "unfinished".into();
    draft.sweep.points = "unfinished".into();
    draft.input_source = String::new();
    draft.output_node = String::new();
    draft.output_ref = String::new();
    draft.input_lattice = "unfinished".into();
    draft.linear_method = Method::Direct;
    draft.krylov_restart = "unfinished".into();
    draft.krylov_cycles = "unfinished".into();
    draft.group_delay = false;
    draft.group_delay_magnitude_floor = "unfinished".into();
    assert!(draft.to_spec().is_ok());
    draft.source_selection = QpxfSourceSelection::AllIndependent;
    draft.input_sources = "V1\nv1".into();
    draft.sideband_selection = QpxfSidebandSelection::AllRetained;
    draft.input_lattices = "unfinished".into();
    draft.max_orders = "unfinished".into();
    assert!(draft.to_spec().is_ok());
    draft.sideband_selection = QpxfSidebandSelection::MaxOrders;
    assert!(draft.to_spec().is_err());
    draft.max_orders = "1, 0, 2".into();
    assert!(draft.to_spec().is_ok());
    draft.source_selection = QpxfSourceSelection::Named;
    assert!(draft.to_spec().is_err());
    draft.input_sources = "X one:V\nIprobe".into();
    assert!(draft.to_spec().is_ok());
    draft.group_delay = true;
    assert!(draft.to_spec().is_err());
    draft.group_delay = false;
    draft.linear_tolerance = "unfinished".into();
    assert!(draft.to_spec().is_err());
    draft.linear_tolerance = "1e-10".into();
    draft.current_output = false;
    assert!(draft.to_spec().is_err());
    draft.current_output = true;
    draft.explicit_frequencies.clear();
    assert!(draft.to_spec().is_err());
}

//! Verify every driven QPSS control through draft, worker and engine boundaries.
use super::*;
use crate::simulation::plan::QpssDraft;
use crate::simulation::runner::worker_contract::WorkerAnalysisSpec;
use rspice_core::analysis::quasi_periodic::QuasiPeriodicSampling;
use rspice_core::engine::QpssInitialState;

fn authored() -> QpssDraft {
    QpssDraft {
        tones: "1k, 1414.213562373095".into(),
        harmonics: "1, 1".into(),
        max_iterations: "19".into(),
        relative_tolerance: "1e-8".into(),
        current_absolute_tolerance: "2e-13".into(),
        voltage_absolute_tolerance: "3e-10".into(),
        max_backtracks: "7".into(),
        max_mixing_order: "1".into(),
        collocation_points: "8, 16".into(),
        source_tones: "V1=1; I1=2".into(),
        linear_method: rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Krylov,
        krylov_restart: "16".into(),
        krylov_cycles: "12".into(),
        linear_tolerance: "2e-11".into(),
        dc_initialization: true,
        ..Default::default()
    }
}

#[test]
fn qpss_controls_survive_draft_worker_and_real_engine_execution() {
    let draft = authored();
    let draft: QpssDraft = ron::from_str(&ron::to_string(&draft).unwrap()).unwrap();
    let spec = draft.to_spec().unwrap();
    let worker = WorkerAnalysisSpec::try_from(&spec).unwrap();
    let restored: AnalysisSpec =
        serde_json::from_str::<WorkerAnalysisSpec>(&serde_json::to_string(&worker).unwrap())
            .unwrap()
            .into();
    assert_eq!(spec, restored);
    let config = restored.driven_qpss_config().unwrap();
    assert_eq!(
        config.grid.sampling,
        QuasiPeriodicSampling::Exact(vec![8, 16])
    );
    assert_eq!(config.grid.max_mixing_order, Some(1));
    assert_eq!(config.solver.max_iterations, 19);
    assert_eq!(config.solver.relative_tolerance, 1e-8);
    assert_eq!(config.solver.current_absolute_tolerance, 2e-13);
    assert_eq!(config.solver.voltage_absolute_tolerance, 3e-10);
    assert_eq!(config.solver.max_backtracks, 7);
    assert_eq!(
        config.solver.linear.method,
        rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Krylov
    );
    assert_eq!(config.solver.linear.restart, 16);
    assert_eq!(config.solver.linear.max_cycles, 12);
    assert_eq!(config.solver.linear.relative_tolerance, 2e-11);
    assert_eq!(config.initial_state, QpssInitialState::DcOperatingPoint);
    assert_eq!(config.source_tones[0].source, "V1");
    assert_eq!(config.source_tones[1].tone, 1);
    let netlist = "QPSS card controls\nV1 input 0 DC .1 AC .2 30\nR1 input out 1k\nR2 out 0 1k\nC1 out 0 1u\nI1 0 out DC 0 AC .001 -20\n.end\n";
    let directive = config.to_spice().unwrap();
    let executable = netlist.replace(".end", &format!("{directive}\n.end"));
    let result = run_spec_request(
        &EngineBridge::new(),
        restored,
        SpecExecutionOptions::default(),
        &executable,
        None,
        &ResolvedExecutionDependencies::default(),
        &rspice_core::NoAbort,
    )
    .unwrap();
    let SimulationResult::Qpss {
        operating_point, ..
    } = result
    else {
        panic!("QPSS dispatch returned a different result family")
    };
    let data = crate::services::simulation_runner::qpss_data_from_operating_point_with_abort(
        operating_point,
        &rspice_core::NoAbort,
    )
    .unwrap();
    assert_eq!(data.operating_point.config(), &config);
    assert_eq!(data.tuples.len(), 3);
    assert!(data.frequencies.windows(2).all(|pair| pair[0] < pair[1]));
    let output = &data
        .spectra
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case("V(out)"))
        .unwrap()
        .1;
    for (tuple, drive) in [
        (
            [1, 0],
            rspice_core::Complex64::from_polar(0.2 / 1e3, 30.0_f64.to_radians()),
        ),
        (
            [0, 1],
            rspice_core::Complex64::from_polar(0.001, -20.0_f64.to_radians()),
        ),
    ] {
        let index = data
            .tuples
            .iter()
            .position(|value| value.as_slice() == tuple)
            .unwrap();
        let admittance = rspice_core::Complex64::new(
            0.002,
            std::f64::consts::TAU * data.frequencies[index] * 1e-6,
        );
        assert!((output[index] - drive / admittance).norm() < 1e-9);
    }
    assert!(
        data.spectra
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case("I(V1)"))
    );
}

#[test]
fn qpss_controls_validate_inactive_and_legacy_fields() {
    let mut default = QpssDraft::default();
    default.harmonics = "1, 1".into();
    let legacy = default.to_spec().unwrap();
    let mut json = serde_json::to_value(&legacy).unwrap();
    json["Qpss"].as_object_mut().unwrap().remove("controls");
    let restored: AnalysisSpec = serde_json::from_value(json).unwrap();
    assert_eq!(legacy, restored);
    let legacy_draft: QpssDraft = serde_json::from_value(serde_json::json!({
        "tones":"1G, 1.001G", "harmonics":"1, 1", "max_iterations":"100", "relative_tolerance":"1e-6", "autonomous":false,"oscillator_node":""
    })).unwrap();
    assert_eq!(legacy_draft.to_spec().unwrap(), legacy);
    let mut duplicate = default.clone();
    duplicate.tones = "1G, 1G".into();
    assert!(duplicate.to_spec().is_err());
    default.collocation_points = "2, 2".into();
    assert!(default.to_spec().unwrap_err().contains("2H+1"));
    default.collocation_points.clear();
    default.max_backtracks = "0".into();
    assert!(default.to_spec().is_ok());
    default.source_tones = "V1=1,V1=1".into();
    assert!(default.to_spec().is_err());
    let mut autonomous = authored();
    autonomous.autonomous = true;
    autonomous.oscillator_node = "out".into();
    assert!(
        autonomous
            .to_spec()
            .unwrap()
            .driven_qpss_config()
            .unwrap_err()
            .contains("autonomous")
    );
}

#[test]
fn qpss_controls_direct_mode_retains_inactive_krylov_draft_buffers() {
    use rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod as Method;
    let mut draft = authored();
    draft.linear_method = Method::Direct;
    draft.krylov_restart = "unfinished".into();
    draft.krylov_cycles = "unfinished".into();
    draft.linear_tolerance = "unfinished".into();
    let config = draft.to_spec().unwrap().driven_qpss_config().unwrap();
    assert_eq!(config.solver.linear.method, Method::Direct);
    assert_eq!(draft.krylov_restart, "unfinished");
    draft.linear_method = Method::Krylov;
    assert!(draft.to_spec().is_err());
}

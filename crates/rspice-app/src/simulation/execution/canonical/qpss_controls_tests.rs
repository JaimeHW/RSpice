//! Every authored QPSS control participates in prepared request identity.
use super::*;
use crate::simulation::plan::QpssDraft;
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
        dc_initialization: true,
        ..Default::default()
    }
}

#[test]
fn qpss_controls_change_request_identity() {
    let spec = authored().to_spec().unwrap();
    let digest = |value: &AnalysisSpec| {
        super::analysis_config_digest(".qpss", value, None, &SpecExecutionOptions::default(), None)
    };
    let original = digest(&spec);
    let changes: &[fn(&mut crate::simulation::multi_run::QpssControls)] = &[
        |c| c.current_absolute_tolerance *= 2.0,
        |c| c.voltage_absolute_tolerance *= 2.0,
        |c| c.max_backtracks += 1,
        |c| {
            c.linear.method =
                rspice_core::analysis::quasi_periodic::QuasiPeriodicLinearMethod::Krylov
        },
        |c| c.linear.restart += 1,
        |c| c.linear.max_cycles += 1,
        |c| c.linear.relative_tolerance *= 2.0,
        |c| c.max_mixing_order = None,
        |c| c.sampling = QuasiPeriodicSampling::Oversample(vec![3]),
        |c| c.initial_state = QpssInitialState::Zero,
        |c| c.source_tones[1].tone = 0,
    ];
    for change in changes {
        let mut next = spec.clone();
        let AnalysisSpec::Qpss { controls, .. } = &mut next else {
            unreachable!()
        };
        change(controls);
        assert_ne!(digest(&next), original);
    }
}

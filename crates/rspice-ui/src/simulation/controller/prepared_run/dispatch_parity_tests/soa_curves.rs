use super::*;
use crate::services::{safety::*, simulation_runner::SoaRuleConfig};
use crate::simulation::{
    dialog::soa::{SoaConfig, SoaDialogState},
    plan::AnalysisDraft,
    runner::worker_contract::WorkerAnalysisSpec,
};

#[test]
fn soa_current_envelope_studio_dispatch_retains_limits_and_saved_configuration() {
    for duration in [None, Some(2e-11)] {
        let curve = SoaCurrentEnvelope::test_fixture();
        let config = SoaConfig {
            stop_time: 1e-9,
            step_time: 1e-10,
            check_vgs_max: false,
            check_vds_max: false,
            check_vbe_max: false,
            check_vce_max: false,
            rules: vec![SoaRuleConfig {
                duration_mode: Default::default(),
                minimum_duration_s: duration,
                current_envelope: Some(curve.clone()),
                power_derating: None,
                voltage_basis: Default::default(),
                parameter: SoAParameter::Id,
                max_value: 0.015,
                devices: vec!["X1.M1".into()],
                models: vec![],
            }],
            ..Default::default()
        };
        let draft = SoaDialogState::from_config(&config);
        let encoded = ron::to_string(&draft).unwrap();
        let draft: SoaDialogState = ron::from_str(&encoded).unwrap();
        assert_eq!(draft.to_config().unwrap(), config);
        let mut short = draft.clone();
        short.rules[0].current_envelope.pulse_width = "0.5n".into();
        assert!(short.to_config().is_err());
        let mut state = preflight_ready_state();
        let id = only(&mut state, &[AnalysisKind::Soa])[0];
        plan_mut(&mut state)
            .edit(id, |body| *body = AnalysisDraft::Soa(draft))
            .unwrap();
        let queue = compiled_queue(&state).unwrap();
        let mut declaration = queue[0].queued_analysis().clone();
        let wire = WorkerAnalysisSpec::try_from(&declaration.spec).unwrap();
        let wire: WorkerAnalysisSpec =
            serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
        assert_eq!(AnalysisSpec::from(wire.clone()), declaration.spec);
        declaration.spec = AnalysisSpec::from(wire);
        let deck = "Current curve\nVg g 0 1\nVd d 0 PWL(0 .5 1n 12)\nX1 d g cell\n.subckt cell d g\nM1 d g 0 0 nm W=1u L=1u\n.model nm NMOS LEVEL=1 VTO=0 KP=.02\n.ends\n.end\n";
        let deck = crate::services::simulation_runner::splice_before_terminal_end_card(
            deck,
            &declaration.analysis_line,
        );
        let run = crate::simulation::runner::pvt_point_evidence::run_declaration(
            &deck,
            "SOA curves",
            declaration,
            27.0,
            SavePolicy::RetainEngineProducedResults,
            &[],
        )
        .unwrap();
        let retained = &run.analyses[0];
        assert!(retained.success, "{:?}", retained.error_message);
        retained.validate_retained_evidence().unwrap();
        let Some(crate::state::AnalysisResultPayload::Soa {
            source_history: _,
            evaluations,
            violations,
        }) = &retained.result_payload
        else {
            panic!("SOA")
        };
        assert_eq!(evaluations.len(), 1);
        assert_eq!(evaluations[0].envelope.as_ref().unwrap().curve, curve);
        assert_eq!(evaluations[0].duration.is_some(), duration.is_some());
        assert!(!violations.is_empty());
        let trace = |name: &str| retained.waveforms.iter().find(|w| w.name == name).unwrap();
        let volts = trace("SOA_ID_CURVE_VOLTAGE(X1.M1)");
        let limits = trace("SOA_ID_CURVE_LIMIT(X1.M1)");
        let stress = trace("SOA_ID(X1.M1)");
        assert_eq!(volts.unit.as_deref(), Some("V"));
        assert_eq!(limits.unit.as_deref(), Some("A"));
        for i in 0..volts.y.len() {
            let v = volts.y[i];
            let peak = (0.04_f64 * 0.02).sqrt();
            let expected = (if v > 10.0 {
                0.0
            } else if v <= 1.0 {
                peak * v
            } else {
                peak / v
            })
            .min(0.015);
            assert!(
                (limits.y[i] - expected).abs() < 1e-14,
                "{v}: {} != {expected}",
                limits.y[i]
            );
            let current = if v < 1.0 {
                0.02 * (v - v * v / 2.0)
            } else {
                0.01
            };
            assert!((stress.y[i] - current).abs() < 1e-8);
        }
        assert!(limits.y.contains(&0.0));
        for name in ["SOA_ID_CURVE_LIMIT(X1.M1)", "SOA_ID_CURVE_VOLTAGE(X1.M1)"] {
            let mut altered = retained.clone();
            let wave = altered
                .waveforms
                .iter_mut()
                .find(|w| w.name == name)
                .unwrap();
            // Use an interior point below the cap so voltage tampering changes its limit.
            let i = volts.y.iter().position(|v| *v > 2.0 && *v < 9.0).unwrap();
            std::sync::Arc::make_mut(&mut wave.y)[i] *= 1.2;
            assert!(altered.validate_retained_evidence().is_err());
        }
        let mut missing = retained.clone();
        missing
            .waveforms
            .retain(|w| w.name != "SOA_ID_CURVE_VOLTAGE(X1.M1)");
        assert!(missing.validate_retained_evidence().is_err());
        let mut simulation = crate::state::SimulationState::default();
        simulation.runs.push(run.clone());
        simulation.next_run_id = 2;
        simulation.active_run_idx = Some(0);
        simulation.active_analysis_idx = Some(0);
        let saved = crate::io::project_io::ProjectSimulationResults::from_state(&simulation);
        let saved: crate::io::project_io::ProjectSimulationResults =
            serde_json::from_str(&serde_json::to_string(&saved).unwrap()).unwrap();
        assert_eq!(
            saved.into_simulation_state().unwrap().runs[0].analyses[0].result_payload,
            retained.result_payload
        );
    }
}

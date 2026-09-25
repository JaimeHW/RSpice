//! App integration tests for the portable Envelope initializer draft.

use rspice_simulation_contract::envelope_draft::EnvelopeInitializationState;
use rspice_simulation_contract::envelope_initialization::{
    EnvelopeInitializationConfig, EnvelopeShootingIntegration,
};

#[cfg(test)]
mod tests {
    use super::super::{EnvelopeConfig, EnvelopeDialogState, EnvelopeInitialPeriodicSolve};
    use super::*;

    #[test]
    fn envelope_initializer_controls_survive_drafts_and_legacy_omission() {
        for method in [
            EnvelopeInitialPeriodicSolve::HarmonicBalance,
            EnvelopeInitialPeriodicSolve::PeriodicSteadyState,
        ] {
            let mut config = EnvelopeConfig::default();
            config.modulation_sources = vec!["VMOD".into()];
            config.initial_periodic_solve = method;
            config.initialization.max_iterations = 73;
            config.initialization.reltol = 2.5e-7;
            config.initialization.abstol = 3e-13;
            config.initialization.damping = 0.8;
            config.initialization.verbose = true;
            if method == EnvelopeInitialPeriodicSolve::HarmonicBalance {
                config.initialization.hb_min_damping = 0.025;
                config.initialization.hb_oversample = 4;
                config.initialization.hb_collocation_points = Some(33);
                config.initialization.hb_gmres_restart = 19;
                config.initialization.hb_use_krylov = true;
                config.initialization.hb_source_stepping = true;
                config.initialization.hb_exact_jacobian = false;
            } else {
                config.initialization.pss_stabilization_periods = 7;
                config.initialization.pss_stabilization_time = 12.5e-6;
                config.initialization.pss_points_per_period = Some(512);
                config.initialization.pss_integration = EnvelopeShootingIntegration::Gear2;
            }
            let draft = EnvelopeDialogState::from_config(&config);
            let json: EnvelopeDialogState =
                serde_json::from_str(&serde_json::to_string(&draft).unwrap()).unwrap();
            let ron: EnvelopeDialogState = ron::from_str(&ron::to_string(&draft).unwrap()).unwrap();
            assert_eq!(json.to_config().unwrap(), config);
            assert_eq!(ron.to_config().unwrap(), config);
            assert!(config.to_spice().contains("initializer="));
            if method == EnvelopeInitialPeriodicSolve::PeriodicSteadyState {
                let resolved = json.to_config().unwrap();
                assert_eq!(
                    resolved
                        .initialization
                        .pss_config(&resolved.carrier_tones, resolved.harmonic_order as usize)
                        .unwrap()
                        .effective_tstab(),
                    12.5e-6
                );
                let mut old_controls = serde_json::to_value(&draft).unwrap();
                old_controls["initialization"]
                    .as_object_mut()
                    .unwrap()
                    .remove("pss_stabilization_time");
                let old_controls: EnvelopeDialogState =
                    serde_json::from_value(old_controls).unwrap();
                let restored = old_controls.to_config().unwrap();
                assert_eq!(
                    restored
                        .initialization
                        .pss_config(&restored.carrier_tones, restored.harmonic_order as usize)
                        .unwrap()
                        .effective_tstab(),
                    7e-6
                );
            }
            let mut old = serde_json::to_value(&draft).unwrap();
            old.as_object_mut().unwrap().remove("initialization");
            let old: EnvelopeDialogState = serde_json::from_value(old).unwrap();
            assert_eq!(
                old.to_config().unwrap().initialization,
                EnvelopeInitializationConfig::default()
            );
        }
    }

    #[test]
    fn envelope_initializer_only_parses_controls_used_by_the_selected_method() {
        let mut state = EnvelopeInitializationState::default();
        state.hb_oversample = "unfinished".into();
        assert!(state.to_config(0).is_err());
        assert!(state.to_config(1).is_ok());
        state.reltol = "unfinished".into();
        assert!(state.to_config(2).is_ok());
        state = EnvelopeInitializationState::default();
        state.pss_stabilization_time = "unfinished".into();
        assert!(state.to_config(0).is_ok());
        assert!(state.to_config(1).is_err());
        state.pss_stabilization_time = "12.5u".into();
        assert!((state.to_config(1).unwrap().pss_stabilization_time - 12.5e-6).abs() < 1e-18);
    }
}

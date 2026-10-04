//! Application-to-worker PVT configuration regression.

use super::*;

#[cfg(test)]
mod pvt_base_tests {
    use super::*;
    use rspice_simulation::sweeps::CornerBaseMode;
    use rspice_simulation_contract::worker_run_config::WorkerCornerBaseMode;

    #[test]
    fn pvt_base_transient_window_survives_configuration_and_worker_transport() {
        let mut state = AppState::default();
        state.sim_setup.tran.stop = "1m".into();
        state.sim_setup.tran.step = "10u".into();
        let temperature = rspice_simulation_contract::temp_draft::TempConfig {
            base_analysis: rspice_simulation_contract::temp_draft::TempBaseAnalysis::Transient,
            ..Default::default()
        };
        let corner = rspice_simulation_contract::corner_config::CornerConfig::default();
        let sealed = state
            .model_library_manager
            .seal_execution_sources_for_plan(&state.sim_setup.model_bindings)
            .unwrap();
        assert!(matches!(
            rspice_simulation::analysis_preparation::temp_run_config_from_dialog(
                &state.sim_setup,
                &temperature
            )
            .unwrap()
            .base_mode,
            CornerBaseMode::Transient { .. }
        ));

        state.sim_setup.tran.start = "200u".into();
        state.sim_setup.tran.max_step = "2u".into();
        state.sim_setup.tran.uic = true;
        for mode in [
            rspice_simulation::analysis_preparation::temp_run_config_from_dialog(
                &state.sim_setup,
                &temperature,
            )
            .unwrap()
            .base_mode,
            rspice_simulation::analysis_preparation::corner_run_config_from_dialog(
                &state.sim_setup,
                &corner,
                &sealed,
            )
            .unwrap()
            .base_mode,
        ] {
            let packet = WorkerCornerBaseMode::from(&mode);
            let json = serde_json::to_string(&packet).unwrap();
            let restored: WorkerCornerBaseMode = serde_json::from_str(&json).unwrap();
            assert_eq!(restored, packet);
            let CornerBaseMode::TransientWindow {
                stop_time,
                step_time,
                start_time,
                max_timestep,
                uic,
            } = CornerBaseMode::from(restored)
            else {
                panic!("all transient settings must be retained")
            };
            for (actual, expected) in [(stop_time, 1e-3), (step_time, 1e-5), (start_time, 2e-4)] {
                assert!((actual / expected - 1.0).abs() < 1e-14);
            }
            assert_eq!(max_timestep, Some(2e-6));
            assert!(uic);
        }

        // Invalid inherited fields must fail on the study form as they do on Transient.
        state.sim_setup.tran.max_step = "-2u".into();
        assert!(
            rspice_simulation::analysis_preparation::temp_run_config_from_dialog(
                &state.sim_setup,
                &temperature
            )
            .is_err()
        );
        assert!(
            rspice_simulation::analysis_preparation::corner_run_config_from_dialog(
                &state.sim_setup,
                &corner,
                &sealed
            )
            .is_err()
        );
    }
}

//! Verify a real periodic run across configuration, worker transport and project retention.
use super::*;
use crate::simulation::multi_run::FrequencySweep;
use crate::simulation::runner::worker_contract::WorkerSimulationResult;
use crate::state::{
    AnalysisType, SimulationRun, SimulationRunLifecycle, SimulationRunProvenance, SimulationState,
};
use rspice_core::analysis::s_param::PeriodicPortNoiseReference;

fn request(hb: bool, reference: Option<PeriodicPortNoiseReference>) -> AnalysisSpec {
    if hb {
        AnalysisSpec::Hbsp {
            start_freq: 1e4,
            stop_freq: 1e4,
            points_per_unit: 1,
            sweep: FrequencySweep::Linear,
            ports: periodic_ports(),
            max_sideband: 1,
            reltol: 1.0e-3,
            abstol: 1.0e-12,
            mixed_mode: false,
            noise_parameters: true,
            noise_reference: reference,
        }
    } else {
        AnalysisSpec::Psp {
            start_freq: 1e4,
            stop_freq: 1e4,
            points_per_unit: 1,
            sweep: FrequencySweep::Linear,
            ports: periodic_ports(),
            max_sideband: 1,
            reltol: 1.0e-3,
            abstol: 1.0e-12,
            mixed_mode: false,
            noise_parameters: true,
            noise_reference: reference,
        }
    }
}

#[test]
fn periodic_port_noise_survives_execution_worker_and_project_round_trip() {
    let netlist = "periodic port noise transport\n\
        P1 p1 0 SIN(0 0 1Meg) PORT=1 Z0=50\n\
        R1 p1 p2 50\nC1 p1 0 1e-18\n\
        P2 p2 0 SIN(0 0 1Meg) PORT=2 Z0=50\n.end\n";
    for hb in [false, true] {
        let dependencies = if hb {
            transferred_hb_dependencies(netlist)
        } else {
            transferred_pss_dependencies(netlist)
        };
        let reference = PeriodicPortNoiseReference {
            input_sideband: -1,
            output_sideband: -1,
            image_sideband: Some(1),
            reference_temperature_kelvin: 300.15,
            termination_temperature_kelvin: 0.0,
            ..Default::default()
        };
        let result = run_spec_request(
            &EngineBridge::new(),
            request(hb, Some(reference)),
            SpecExecutionOptions::default(),
            netlist,
            None,
            &dependencies,
            &rspice_core::NoAbort,
        )
        .unwrap();
        let SimulationResult::Ac {
            waveforms,
            frequencies,
            measurements,
            ..
        } = &result
        else {
            panic!("periodic network result");
        };
        assert_eq!(frequencies, &[1e4]);
        assert!((waveforms["PN_F"].y_values[0] - 2.0).abs() < 1e-7);
        assert!((waveforms["PN_Rn"].y_values[0] - 50.0).abs() < 1e-5);
        assert!((waveforms["PN_Fdsb"].y_values[0] - 2.0).abs() < 1e-7);
        assert_eq!(waveforms["Cw1_2[k=-1,m=-1]"].y_unit, "W/Hz");
        for signal in ["S21", "S21[k=-1,m=-1]"] {
            assert_eq!(waveforms[signal].y_unit, "1");
            let converted = result
                .study_measurement(&format!("last:{signal}"))
                .unwrap()
                .value_in_unit("%")
                .unwrap()
                .unwrap();
            assert!((converted - waveforms[signal].y_values[0] * 100.0).abs() < 1e-12);
        }
        for (name, target, expected) in [
            ("periodic_noise_carrier_hz", "MHz", 1.0),
            ("periodic_noise_reference_temperature_kelvin", "degC", 27.0),
        ] {
            let value = result
                .study_measurement(name)
                .unwrap()
                .value_in_unit(target)
                .unwrap()
                .unwrap();
            assert!((value - expected).abs() < 1e-10);
        }

        assert_eq!(
            measurements
                .iter()
                .find(|m| m.name == "periodic_noise_input_sideband")
                .unwrap()
                .value,
            Some(-1.0)
        );
        let worker = WorkerSimulationResult::try_from(result).unwrap();
        let wire = serde_json::to_string(&worker).unwrap();
        let received =
            SimulationResult::from(serde_json::from_str::<WorkerSimulationResult>(&wire).unwrap());
        let retained = crate::simulation::controller::SimulationController::new()
            .convert_to_analysis_result_with_metadata_owned(
                received,
                if hb {
                    AnalysisType::Hbsp
                } else {
                    AnalysisType::Psp
                },
                "periodic port noise",
            );
        assert_eq!(retained.validate_retained_evidence(), Ok(()));
        let covariance = retained
            .waveforms
            .iter()
            .find(|w| {
                w.complex
                    .as_ref()
                    .is_some_and(|c| c.source_name == "Cw1_2[k=-1,m=-1]")
            })
            .unwrap();
        assert_eq!(covariance.unit.as_deref(), Some("W/Hz"));
        let expected_covariance = -4.0 / 9.0 * rspice_core::constants::K_BOLTZMANN * 300.15;
        assert!(
            (covariance.complex.as_ref().unwrap().real[0] / expected_covariance - 1.0).abs() < 1e-7
        );
        let original_digest = retained.result_data_digest();
        let mut altered = retained.clone();
        altered
            .measurements
            .iter_mut()
            .find(|m| m.name == "periodic_noise_reference_temperature_kelvin")
            .unwrap()
            .value = Some(290.0);
        assert_ne!(altered.result_data_digest(), original_digest);
        let mut run = SimulationRun::new(1);
        run.mark_running().unwrap();
        run.add_analysis(retained);
        run.finish_lifecycle(SimulationRunLifecycle::Completed)
            .unwrap();
        run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .unwrap();
        let mut state = SimulationState::default();
        state.runs = vec![run].into();
        state.next_run_id = 1;
        state.active_run_idx = Some(0);
        state.active_analysis_idx = Some(0);
        let snapshot = crate::io::project_io::ProjectSimulationResults::from_state(&state);
        let json = serde_json::to_string(&snapshot).unwrap();
        let restored =
            serde_json::from_str::<crate::io::project_io::ProjectSimulationResults>(&json)
                .unwrap()
                .into_simulation_state()
                .unwrap();
        let result = &restored.runs[0].analyses[0];
        assert_eq!(result.validate_retained_evidence(), Ok(()));
        assert_eq!(result.result_data_digest(), original_digest);
        let temperature = result.scalar_evidence("periodic_noise_reference_temperature_kelvin");
        assert!((temperature[0].value_in_unit("degC").unwrap().unwrap() - 27.0).abs() < 1e-10);
        assert!(temperature[0].value_in_unit("V").is_err());
    }
}

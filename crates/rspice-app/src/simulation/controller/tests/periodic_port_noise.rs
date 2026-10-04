//! Periodic port-noise evidence survives a checked generated run and project reload.
use super::AppState;
use crate::io::project_io::ProjectSimulationResults;
use crate::simulation::plan::AnalysisKind;
use crate::state::{AnalysisType, SimulationState};
use crate::state::{ComponentType, Point};
use rspice_simulation_contract::analysis_draft::AnalysisDraft;
use rspice_simulation_contract::drafts::FrequencySweepDraft;
use rspice_simulation_contract::hb_draft::{HbConfig, HbDialogState};
use rspice_simulation_contract::periodic_network_draft::{
    NetworkPortDraft, PeriodicNetworkDraft, PeriodicNetworkNoiseDraft,
};
use rspice_simulation_contract::pss_draft::{PssConfig, PssDialogState};

fn network(hb: bool) -> AppState {
    let mut state = AppState::default();
    state.sim_setup.run_set = rspice_simulation_contract::run_set::RunSetState::reference_only();
    for (index, (kind, name, value, params, nodes)) in [
        (
            ComponentType::RfPort,
            "P1",
            "0",
            "port=1 z0=50 pwr=0 freq=1Meg",
            vec!["p1", "0"],
        ),
        (
            ComponentType::RfPort,
            "P2",
            "0",
            "port=2 z0=50 pwr=0 freq=1Meg",
            vec!["p2", "0"],
        ),
        (ComponentType::Resistor, "R1", "50", "", vec!["p1", "p2"]),
        (ComponentType::Capacitor, "C1", "1e-18", "", vec!["p1", "0"]),
        (ComponentType::Ground, "GND", "", "", vec!["0"]),
    ]
    .into_iter()
    .enumerate()
    {
        let id = state
            .schematic
            .add_component(kind, Point::new(100 + index as i32 * 200, 100));
        let component = state
            .schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .unwrap();
        component.name = name.into();
        component.value = value.into();
        component.params = params.into();
        let terminals = rspice_design::schematic::component_edit::legacy_terminal_points(component);
        assert_eq!(terminals.len(), nodes.len());
        for (terminal, node) in terminals.into_iter().zip(nodes) {
            let end = Point::new(
                terminal.x,
                terminal.y + if terminal.y < 100 { -20 } else { 20 },
            );
            state.schematic.add_wire(vec![terminal, end]).unwrap();
            state.schematic.add_net_label(end, node.into());
        }
    }
    let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
    for id in plan
        .instances()
        .iter()
        .map(|instance| instance.id())
        .collect::<Vec<_>>()
    {
        plan.set_enabled(id, false).unwrap();
    }
    let (op, _) = plan.insert(AnalysisKind::OperatingPoint).unwrap();
    let carrier_kind = if hb {
        AnalysisKind::HarmonicBalance
    } else {
        AnalysisKind::Pss
    };
    let (carrier, _) = plan.insert(carrier_kind).unwrap();
    plan.edit(carrier, |draft| {
        *draft = if hb {
            AnalysisDraft::HarmonicBalance(HbDialogState::from_config(&HbConfig {
                fundamental_freq: 1e6,
                num_harmonics: 8,
                reltol: 2.5e-7,
                ..Default::default()
            }))
        } else {
            AnalysisDraft::Pss(PssDialogState::from_config(&PssConfig {
                fund_freq: 1e6,
                num_harmonics: 8,
                tone_sources: vec!["P1".into(), "P2".into()],
                ..Default::default()
            }))
        };
    })
    .unwrap();
    plan.bind_dependency(carrier, AnalysisKind::OperatingPoint, op)
        .unwrap();
    let (consumer, _) = plan
        .insert(if hb {
            AnalysisKind::Hbsp
        } else {
            AnalysisKind::Psp
        })
        .unwrap();
    plan.edit(consumer, |draft| {
        let network = PeriodicNetworkDraft {
            sweep: FrequencySweepDraft {
                start: "1e4".into(),
                stop: "1e4".into(),
                points: "1".into(),
                sweep: 2,
            },
            ports: ["p1", "p2"]
                .map(|node| NetworkPortDraft {
                    node_pos: node.into(),
                    node_neg: "0".into(),
                    z0: "50".into(),
                })
                .into(),
            max_sideband: "1".into(),
            reltol: "1e-3".into(),
            abstol: "1e-12".into(),
            mixed_mode: false,
            noise_parameters: true,
            noise: PeriodicNetworkNoiseDraft {
                report_parameters: true,
                input_sideband: "-1".into(),
                output_sideband: "-1".into(),
                image_sideband: "1".into(),
                reference_temperature: "300.15".into(),
                termination_temperature: "0".into(),
                ..Default::default()
            },
        };
        *draft = if hb {
            AnalysisDraft::Hbsp(network)
        } else {
            AnalysisDraft::Psp(network)
        };
    })
    .unwrap();
    plan.bind_dependency(consumer, carrier_kind, carrier)
        .unwrap();
    state
}

#[test]
fn periodic_port_noise_project_round_trip_preserves_covariance_and_temperature_units() {
    for hb in [false, true] {
        let run = crate::simulation::controller::test_execution::run_generated_batch(
            network(hb),
            rspice_results::run::SimulationRunLifecycle::Completed,
        );
        let kind = if hb {
            AnalysisType::Hbsp
        } else {
            AnalysisType::Psp
        };
        let retained = run
            .analyses
            .iter()
            .find(|analysis| analysis.analysis_type == kind)
            .unwrap();
        let original_digest = retained.result_data_digest();
        let mut state = SimulationState::default();
        state.retained.next_run_id = run.id;
        state.retained.runs = vec![run].into();
        let snapshot = crate::io::capture_simulation_results(&state);
        snapshot.validate().unwrap();
        let json = serde_json::to_string(&snapshot).unwrap();
        let restored = crate::io::simulation_state_from_results(
            serde_json::from_str::<ProjectSimulationResults>(&json).unwrap(),
        )
        .unwrap();
        let run = &restored.retained.runs[0];
        run.validate_provenance().unwrap();
        let result = run
            .analyses
            .iter()
            .find(|analysis| analysis.analysis_type == kind)
            .unwrap();
        assert_eq!(result.validate_retained_evidence(), Ok(()));
        assert_eq!(result.result_data_digest(), original_digest);
        let temperature = result.scalar_evidence("periodic_noise_reference_temperature_kelvin");
        assert!((temperature[0].value_in_unit("degC").unwrap().unwrap() - 27.0).abs() < 1e-10);
        assert!(temperature[0].value_in_unit("V").is_err());
        let covariance = result
            .waveforms
            .iter()
            .find(|wave| {
                wave.complex
                    .as_ref()
                    .is_some_and(|complex| complex.source_name == "Cw1_2[k=-1,m=-1]")
            })
            .unwrap();
        assert_eq!(covariance.unit.as_deref(), Some("W/Hz"));
        let expected_covariance = -4.0 / 9.0 * rspice_core::constants::K_BOLTZMANN * 300.15;
        assert!(
            (covariance.complex.as_ref().unwrap().real[0] / expected_covariance - 1.0).abs() < 1e-7
        );
    }
}

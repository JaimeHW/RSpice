//! A Periodic AC reads the periodic solve its "Uses" field names: bound to a
//! harmonic-balance analysis its card states `from=hb` and it runs around
//! that solution; bound to a PSS its card states `from=pss` and it runs
//! around the shooting orbit.

use super::RunTestHost;
use crate::simulation::controller::SimulationController;
use crate::simulation::controller::test_execution::run_generated_batch;
use crate::simulation::plan::{AnalysisDraft, AnalysisKind};
use crate::state::{AnalysisType, ComponentType, Point};
use rspice_results::run::SimulationRunLifecycle;
use rspice_simulation_contract::hb_draft::{HbConfig, HbDialogState};
use rspice_simulation_contract::pss_draft::{PssConfig, PssDialogState};

/// `V1` (1 mV at 1 MHz) into R1 = 1k with C1 = 159 pF to ground, a harmonic
/// balance and a PSS of it, and a Periodic AC from `V1` to `out` bound to
/// the one of `kind`.
fn driven_rc_reading(kind: AnalysisKind) -> (RunTestHost, String) {
    let mut state = RunTestHost::default();
    state.model.sim_setup.run_set =
        rspice_simulation_contract::run_set::RunSetState::reference_only();
    for (index, (component_kind, name, value, nodes)) in [
        (
            ComponentType::VoltageSourceSin,
            "V1",
            "SIN(0 0.001 1Meg)",
            vec!["in", "0"],
        ),
        (ComponentType::Resistor, "R1", "1k", vec!["in", "out"]),
        (
            ComponentType::Capacitor,
            "C1",
            "159.154943091895p",
            vec!["out", "0"],
        ),
        (ComponentType::Ground, "GND", "", vec!["0"]),
    ]
    .into_iter()
    .enumerate()
    {
        let id = state
            .model
            .schematic
            .add_component(component_kind, Point::new(100 + index as i32 * 200, 100));
        let component = state
            .model
            .schematic
            .document_mut_for_test()
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .unwrap();
        component.name = name.into();
        component.value = value.into();
        let terminals = rspice_design::schematic::component_edit::legacy_terminal_points(component);
        assert_eq!(terminals.len(), nodes.len());
        for (terminal, node) in terminals.into_iter().zip(nodes) {
            let end = Point::new(
                terminal.x,
                terminal.y + if terminal.y < 100 { -20 } else { 20 },
            );
            state.model.schematic.add_wire(vec![terminal, end]).unwrap();
            state.model.schematic.add_net_label(end, node.into());
        }
    }
    let plan = state.model.sim_setup.analysis_plan.as_mut().unwrap();
    for id in plan
        .instances()
        .iter()
        .map(|instance| instance.id())
        .collect::<Vec<_>>()
    {
        plan.set_enabled(id, false).unwrap();
    }
    let (hb, _) = plan.insert(AnalysisKind::HarmonicBalance).unwrap();
    plan.edit(hb, |draft| {
        *draft = AnalysisDraft::HarmonicBalance(HbDialogState::from_config(&HbConfig {
            fundamental_freq: 1e6,
            num_harmonics: 8,
            ..Default::default()
        }));
    })
    .unwrap();
    let (pss, _) = plan.insert(AnalysisKind::Pss).unwrap();
    plan.edit(pss, |draft| {
        *draft = AnalysisDraft::Pss(PssDialogState::from_config(&PssConfig {
            fund_freq: 1e6,
            num_harmonics: 8,
            tone_sources: vec!["V1".into()],
            ..Default::default()
        }));
    })
    .unwrap();
    let (pac, _) = plan.insert(AnalysisKind::Pac).unwrap();
    plan.edit(pac, |draft| {
        let AnalysisDraft::Pac(draft) = draft else {
            unreachable!()
        };
        draft.start_freq = "1e5".into();
        draft.stop_freq = "1e7".into();
        draft.num_points = "2".into();
        draft.max_sideband = "1".into();
        draft.input_source = "V1".into();
        draft.output_node = "out".into();
    })
    .unwrap();
    let carrier = if kind == AnalysisKind::HarmonicBalance {
        hb
    } else {
        pss
    };
    plan.bind_dependency(pac, kind, carrier).unwrap();
    assert!(
        plan.validation_issues().is_empty(),
        "{:?}",
        plan.validation_issues()
    );
    let draft = plan.instance(pac).unwrap().draft().clone();
    let card = SimulationController::new()
        .analysis_draft_directive(&state.model, &draft)
        .unwrap();
    (state, card.to_lowercase())
}

#[test]
fn a_periodic_ac_states_and_runs_around_the_solve_it_uses() {
    for (kind, from) in [
        (AnalysisKind::HarmonicBalance, "from=hb"),
        (AnalysisKind::Pss, "from=pss"),
    ] {
        let (state, card) = driven_rc_reading(kind);
        assert!(card.contains(from), "{kind:?}: {card}");
        let run = run_generated_batch(state, SimulationRunLifecycle::Completed);
        let pac = run
            .analyses
            .iter()
            .find(|analysis| analysis.analysis_type == AnalysisType::Pac)
            .unwrap_or_else(|| panic!("{kind:?}: no Periodic AC in {:?}", run.analyses));
        assert!(!pac.waveforms.is_empty(), "{kind:?}: no PAC waveform");
    }
}

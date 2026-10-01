//! Authored divider and corner-plan inputs for application integration tests.

use super::AppState;
use crate::simulation::plan::AnalysisDraft;
use crate::simulation::run_set::{RunSetDimension, RunSetDimensionKind, RunSetState};
use crate::state::{ComponentType, Point};

pub(crate) fn divider() -> AppState {
    let mut state = AppState::default();
    for (index, (kind, name, value, nodes)) in [
        (ComponentType::VoltageSourceAc, "VDD", "1", vec!["vdd", "0"]),
        (ComponentType::Resistor, "R1", "1k", vec!["vdd", "out"]),
        (ComponentType::Resistor, "R2", "1k", vec!["out", "0"]),
        (ComponentType::Capacitor, "C1", "1p", vec!["out", "0"]),
        (ComponentType::Ground, "GND", "", vec!["0"]),
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
        if name == "VDD" {
            component.params = "dc=1.8".into();
        }
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
    state.sim_setup.run_set = RunSetState::reference_only();
    state.sim_setup.reference_pvt.temperature_celsius = 27.0;
    let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
    for id in plan
        .instances()
        .iter()
        .map(|instance| instance.id())
        .collect::<Vec<_>>()
    {
        plan.set_enabled(id, false).unwrap();
    }
    state
}

pub(crate) fn run_set_divider(draft: AnalysisDraft, temperatures: &[&str]) -> AppState {
    let mut state = divider();
    let mut supply = RunSetDimension::new(
        "fixture-supply",
        RunSetDimensionKind::Supply,
        &["1.62", "1.8"],
        1,
    );
    supply.source = "netlist-source:VDD".into();
    state.sim_setup.run_set.dimensions = vec![
        supply,
        RunSetDimension::new(
            "fixture-temperature",
            RunSetDimensionKind::Temperature,
            temperatures,
            1,
        ),
    ];
    let plan = state.sim_setup.analysis_plan.as_mut().unwrap();
    let (analysis, _) = plan.insert(draft.kind()).unwrap();
    plan.edit(analysis, |target| *target = draft).unwrap();
    state
}

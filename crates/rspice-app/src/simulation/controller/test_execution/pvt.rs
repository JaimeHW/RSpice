//! Authored divider and corner-plan inputs for application integration tests.

use super::RunTestHost;
use crate::simulation::plan::AnalysisDraft;
use crate::state::{ComponentType, Point};
use rspice_simulation_contract::corner_config::CornerConfig;
use rspice_simulation_contract::run_set::{
    NETLIST_SUPPLY_SOURCE_PREFIX, RunSetBudgets, RunSetComposition, RunSetCompositionMode,
    RunSetDimension, RunSetDimensionKind, RunSetState,
};

pub fn divider() -> RunTestHost {
    let mut state = RunTestHost::default();
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
            .model
            .schematic
            .add_component(kind, Point::new(100 + index as i32 * 200, 100));
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
            state.model.schematic.add_wire(vec![terminal, end]).unwrap();
            state.model.schematic.add_net_label(end, node.into());
        }
    }
    state.model.sim_setup.run_set = RunSetState::reference_only();
    state.model.sim_setup.reference_pvt.temperature_celsius = 27.0;
    let plan = state.model.sim_setup.analysis_plan.as_mut().unwrap();
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

pub fn run_set_divider(draft: AnalysisDraft, temperatures: &[&str]) -> RunTestHost {
    let mut state = divider();
    let mut supply = RunSetDimension::new(
        "fixture-supply",
        RunSetDimensionKind::Supply,
        &["1.62", "1.8"],
        1,
    );
    supply.source = "netlist-source:VDD".into();
    state.model.sim_setup.run_set.dimensions = vec![
        supply,
        RunSetDimension::new(
            "fixture-temperature",
            RunSetDimensionKind::Temperature,
            temperatures,
            1,
        ),
    ];
    let plan = state.model.sim_setup.analysis_plan.as_mut().unwrap();
    let (analysis, _) = plan.insert(draft.kind()).unwrap();
    plan.edit(analysis, |target| *target = draft).unwrap();
    state
}

/// Build a run set from an executable corner configuration.
///
/// Test-only since the Corner draft stopped declaring a space of its own:
/// nothing in the product now turns a `CornerConfig` back into a run set,
/// because the run set is what produced it. It survives as the way a test
/// states a space the way the engine sees it.
///
/// It reads the axes only: an explicit point list is what a space looks
/// like once it has stopped being an axis composition, and there is no axis
/// form to recover it into.
#[must_use]
pub fn from_corner_config(config: &CornerConfig) -> RunSetState {
    let mut state = RunSetState {
        revision: 1,
        sequence: 4,
        dimensions: Vec::new(),
        composition: RunSetComposition {
            mode: if config.full_matrix {
                RunSetCompositionMode::Cartesian
            } else {
                RunSetCompositionMode::Zipped
            },
            excluded_points: std::collections::BTreeSet::new(),
            ..RunSetComposition::default()
        },
        budgets: RunSetBudgets::default(),
        preview: None,
        receipts: Vec::new(),
        history: Vec::new(),
        future: Vec::new(),
    };

    let sections: Vec<String> = config
        .process_corners
        .iter()
        .map(|corner| corner.short_name().to_owned())
        .collect();
    state.dimensions.push(RunSetDimension::new(
        "dimension-process",
        RunSetDimensionKind::ProcessSection,
        &sections.iter().map(String::as_str).collect::<Vec<_>>(),
        1,
    ));

    let supplies: Vec<String> = config.voltages.iter().map(f64::to_string).collect();
    let mut supply = RunSetDimension::new(
        "dimension-supply",
        RunSetDimensionKind::Supply,
        &supplies.iter().map(String::as_str).collect::<Vec<_>>(),
        1,
    );
    if !config.supply_source_names.is_empty() {
        supply.source = format!(
            "{}{}",
            NETLIST_SUPPLY_SOURCE_PREFIX,
            config.supply_source_names.join(",")
        );
    }
    // A single supply value is not a sweep: it is the deck's own value, and
    // enabling an axis for it would report a dimension the run does not
    // actually vary.
    supply.enabled = config.voltages.len() > 1;
    state.dimensions.push(supply);

    let temperatures: Vec<String> = config.temperatures.iter().map(f64::to_string).collect();
    state.dimensions.push(RunSetDimension::new(
        "dimension-temperature",
        RunSetDimensionKind::Temperature,
        &temperatures.iter().map(String::as_str).collect::<Vec<_>>(),
        1,
    ));

    state
}

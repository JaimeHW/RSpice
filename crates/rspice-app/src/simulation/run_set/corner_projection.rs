//! App integration fixture for authored corner spaces.

use super::{
    NETLIST_SUPPLY_SOURCE_PREFIX, RunSetBudgets, RunSetComposition, RunSetCompositionMode,
    RunSetDimension, RunSetDimensionKind, RunSetState,
};
use crate::simulation::dialog::corner::CornerConfig;

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
#[cfg(test)]
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

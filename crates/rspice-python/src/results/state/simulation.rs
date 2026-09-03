//! Pickle state of a solved DC operating point.
//!
//! `SimulationResult` is also the point type a DC sweep, a `.STEP` run and a
//! `.TEMP` run are made of, so every one of those families round-trips through
//! this one codec.

use super::*;

/// Rebuild a core `SimulationResult` from its Python-visible state.
///
/// `SimulationResult::new` leaves its private observable index empty, and
/// core's observable lookup falls back to a linear scan in exactly that
/// case, so DC observables still resolve after a round-trip.
pub(crate) fn rebuild_simulation_result(state: SimulationResultState) -> SimulationResult {
    let (node_voltages, node_names, branch_currents, branch_names, dc_observables) = state;
    let mut result =
        SimulationResult::new(node_voltages.len().saturating_sub(1), branch_currents.len());
    result.node_voltages = node_voltages;
    result.node_names = node_names;
    result.branch_currents = branch_currents;
    result.branch_names = branch_names;
    result.dc_observables = dc_observables;
    result
}

/// Complete Python-visible state of a core `SimulationResult`.
pub(crate) type SimulationResultState = (
    Vec<f64>,
    Vec<String>,
    Vec<f64>,
    Vec<String>,
    Vec<(String, f64)>,
);

pub(crate) fn simulation_result_state(result: &SimulationResult) -> SimulationResultState {
    (
        result.node_voltages.clone(),
        result.node_names.clone(),
        result.branch_currents.clone(),
        result.branch_names.clone(),
        result.dc_observables.clone(),
    )
}

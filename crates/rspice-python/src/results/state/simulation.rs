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

/// Which of the rebuilt result's nodes carry events rather than a voltage.
///
/// The mask is a separate pickle field rather than part of the state tuple
/// above, so a pickle written before this rule existed still loads: it simply
/// carries no mask, and a result with no event domain has none to carry. A
/// code outside the two domains is read as "no event domain", which is what
/// an older or foreign writer means by it.
pub(crate) fn restore_event_only_nodes(result: &mut SimulationResult, codes: Option<Vec<u8>>) {
    let Some(codes) = codes else {
        return;
    };
    result.set_event_only_nodes(
        codes
            .into_iter()
            .map(|code| match code {
                1 => Some(EventOnlyNetKind::Digital),
                2 => Some(EventOnlyNetKind::Real),
                _ => None,
            })
            .collect(),
    );
}

/// The event-only mask as the pickle records it, or `None` when the result has
/// no event domain at all — the common case, which then costs the payload
/// nothing.
pub(crate) fn event_only_node_state(result: &SimulationResult) -> Option<Vec<u8>> {
    let mask = result.event_only_nodes();
    if mask.iter().all(Option::is_none) {
        return None;
    }
    Some(
        mask.iter()
            .map(|kind| match kind {
                Some(EventOnlyNetKind::Digital) => 1u8,
                Some(EventOnlyNetKind::Real) => 2u8,
                None => 0u8,
            })
            .collect(),
    )
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

//! The rail's Results badge: results the reader has not seen yet.

use crate::state::SimulationState;
use crate::workbench::state::Workspace;

/// The badge on a workspace's button: one on Results while it is not showing
/// and its results have changed since it last was, nothing anywhere else.
pub(in crate::workbench) fn unseen_result_count(
    seen_version: u64,
    simulation: &SimulationState,
    workspace: Workspace,
    active: bool,
) -> usize {
    usize::from(
        workspace == Workspace::Results
            && !active
            && simulation.retained.has_results()
            && seen_version != simulation.view.data_version,
    )
}

/// Results as they stand are seen: the badge stays off until they change.
pub(in crate::workbench) fn mark_results_seen(
    seen_version: &mut u64,
    simulation: &SimulationState,
) {
    *seen_version = simulation.view.data_version;
}

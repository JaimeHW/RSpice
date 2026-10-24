//! The rail's Results badge: results the reader has not seen yet.

use crate::workbench::AppState;
use crate::workbench::state::Workspace;

/// The badge on a workspace's button: one on Results while it is not showing
/// and its results have changed since it last was, nothing anywhere else.
pub(in crate::workbench) fn unseen_result_count(
    state: &AppState,
    workspace: Workspace,
    active: bool,
) -> usize {
    usize::from(
        workspace == Workspace::Results
            && !active
            && state.model.simulation.retained.has_results()
            && state.results.results_seen_version != state.model.simulation.view.data_version,
    )
}

/// Results as they stand are seen: the badge stays off until they change.
pub(in crate::workbench) fn mark_results_seen(state: &mut AppState) {
    state.results.results_seen_version = state.model.simulation.view.data_version;
}

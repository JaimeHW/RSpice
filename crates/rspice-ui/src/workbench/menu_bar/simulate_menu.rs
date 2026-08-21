//! The Simulate menu.

use crate::workbench::app_state::AppState;

/// Prepare the solver options for editing.
///
/// The draft is refreshed from the currently-effective options so the Solver &
/// convergence page opens on what the next run would actually use, never on a
/// stale draft left behind by an abandoned edit.
pub(crate) fn open_simulation_options(state: &mut AppState) {
    state.sim_setup.options_draft =
        crate::simulation::dialog::OptionsDialogState::from_options(&state.sim_setup.options);
}

//! The Simulate menu.

use crate::workbench::app_state::AppState;

/// Prepare the solver options for editing.
///
/// The draft is refreshed from the currently-effective options so an editor
/// opens on what the next run would actually use, never on a stale draft left
/// behind by an abandoned edit. Which surface presents it is the caller's
/// decision.
pub(crate) fn open_simulation_options(state: &mut AppState) {
    state.sim_setup.options_draft =
        crate::simulation::dialog::OptionsDialogState::from_options(&state.sim_setup.options);
    state.sim_setup.options_errors.clear();
}

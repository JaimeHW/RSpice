use crate::common::app::AppState;

/// Open the simulation options dialog with a fresh draft of the
/// currently-effective options.
pub(crate) fn open_simulation_options(state: &mut AppState) {
    state.sim_setup.options_draft =
        crate::simulation::dialog::OptionsDialogState::from_options(&state.sim_setup.options);
    state.sim_setup.options_errors.clear();
    state.sim_setup.options_open = true;
}

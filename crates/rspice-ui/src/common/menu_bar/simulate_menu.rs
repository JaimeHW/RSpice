
use crate::common::app::AppState;







pub(crate) fn open_simulation_options(state: &mut AppState) {
    state.dialogs.simulation_options_state =
        crate::simulation::dialog::OptionsDialogState::from_options(
            &state.dialogs.simulation_options_config,
        );
    state.dialogs.simulation_options_errors.clear();
    state.dialogs.simulation_options = true;
}

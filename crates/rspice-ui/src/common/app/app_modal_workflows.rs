use egui::Context;

use super::{AppState, RSpiceApp, apply_component_property_edits};

fn apply_properties_dialog_result(
    state: &mut AppState,
    result: crate::properties::dialog::PropertiesDialogResult,
) -> bool {
    match result {
        crate::properties::dialog::PropertiesDialogResult::Apply(id, props) => {
            apply_component_property_edits(state, id, props)
        }
        crate::properties::dialog::PropertiesDialogResult::Cancel => false,
        crate::properties::dialog::PropertiesDialogResult::None => false,
    }
}

impl RSpiceApp {
    pub(super) fn process_component_properties_dialog(&mut self, ctx: &Context) {
        let result = crate::properties::dialog::render_properties_dialog(
            ctx,
            &mut self.state.property_editor,
        );
        let _ = apply_properties_dialog_result(&mut self.state, result);
    }

    pub(super) fn process_model_browser_dialog(&mut self, ctx: &Context) {
        let _ = crate::properties::model_browser::render_model_browser(
            ctx,
            &mut self.state.model_browser_state,
            &self.state.model_library_manager,
        );
    }
}

use egui::Context;

use super::RSpiceApp;

impl RSpiceApp {
    pub(super) fn process_model_browser_dialog(&mut self, ctx: &Context) {
        let _ = crate::properties::model_browser::render_model_browser(
            ctx,
            &mut self.state.model_browser_state,
            &self.state.model_library_manager,
        );
    }
}

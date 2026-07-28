//! Symbol definition dialog rendering.

use egui::Context;

use crate::workbench::RSpiceApp;

#[cfg(target_arch = "wasm32")]
use super::source_picker;

impl RSpiceApp {
    pub(crate) fn render_symbol_definition_dialogs(&mut self, ctx: &Context) {
        #[cfg(target_arch = "wasm32")]
        source_picker::consume_symbol_source_result(&mut self.state.dialogs.symbol_import);
        self.render_symbol_import_dialog(ctx);
        self.render_symbol_parameter_form_dialog(ctx);
    }
}

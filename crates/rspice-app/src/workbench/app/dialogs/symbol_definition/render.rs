//! Symbol definition dialog rendering.

use egui::Context;

use super::import_workflow::render_symbol_import_dialog;
use super::parameter_form_workflow::render_symbol_parameter_form_dialog;
use super::{SymbolImportDialogState, SymbolParameterFormDialogState};
use crate::workbench::app_state::Model;

#[cfg(target_arch = "wasm32")]
use super::source_picker;

/// Draw the symbol definition dialogs over their drafts and the model: the
/// import first, then the parameter form.
pub(in crate::workbench) fn render_symbol_definition_dialogs(
    symbol_import: &mut SymbolImportDialogState,
    parameter_form: &mut SymbolParameterFormDialogState,
    model: &mut Model,
    ctx: &Context,
) {
    #[cfg(target_arch = "wasm32")]
    source_picker::consume_symbol_source_result(symbol_import);
    render_symbol_import_dialog(symbol_import, model, ctx);
    render_symbol_parameter_form_dialog(parameter_form, model, ctx);
}

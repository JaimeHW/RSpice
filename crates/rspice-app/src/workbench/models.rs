//! The models feature: the Models & PDKs workspace, the device-model editor,
//! measurement correlation and unsigned PDK authoring.
//!
//! What the workspace keeps between frames is declared here, beneath the
//! session aggregate that holds the models slot and the wire that persists
//! it, so both name it without reaching up into the surfaces.

pub(crate) mod workspace_state;

use crate::workbench::app_state::Model;
use crate::workbench::slots::ModelsState;
use rspice_app_workbench::workbench::app::dialogs::model_browser::render_model_browser;

/// The standalone model browser the Models menu opens, over every loaded
/// model.
pub(crate) fn render_model_browser_dialog(
    ctx: &egui::Context,
    models: &mut ModelsState,
    model: &Model,
) {
    let _ = render_model_browser(
        ctx,
        &mut models.model_browser_state,
        &model.model_library_manager,
    );
}

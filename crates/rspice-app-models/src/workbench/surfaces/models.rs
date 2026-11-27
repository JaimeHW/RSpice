//! Model catalog, PDK sections and authenticated includes.

mod manager;

use egui::Ui;

use crate::workbench::framework::feature::Cx;
use crate::workbench::models::ModelsFeature;

impl ModelsFeature {
    /// The Models & PDKs workspace on the stage.
    pub fn show_workspace(cx: &mut Cx<'_, ModelsFeature>, ui: &mut Ui) {
        manager::show(ui, cx);
    }
}

#[cfg(test)]
mod scale;

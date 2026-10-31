//! Model catalog, symbol contracts, PDK sections, authenticated includes, and
//! the source-owned model qualification and release gate.

mod manager;
mod qualification_actions;

use crate::workbench::framework::ports::QualificationGate;
use crate::workbench::models::qualification::*;
use qualification_actions::*;

use egui::Ui;

use crate::state::ModelBoundSymbolDefinition;

use super::super::{RouteTransitionSource, SurfaceId, SurfaceRoute};
use crate::workbench::documents::model_editor::{self, ModelEditorSection};
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::framework::feature::Cx;
use crate::workbench::models::ModelsFeature;
use crate::workbench::models::commands::ModelsCommand;

impl ModelsFeature {
    /// The Models & PDKs workspace on the stage.
    pub fn show_workspace(cx: &mut Cx<'_, ModelsFeature>, ui: &mut Ui) {
        manager::show(ui, cx);
    }
}

fn symbol_parameter_form_label(definition: &ModelBoundSymbolDefinition) -> String {
    let sections = &definition.parameter_form.sections;
    let field_count = sections
        .iter()
        .map(|section| section.fields.len())
        .sum::<usize>();
    match sections.as_slice() {
        [] => "not defined".to_owned(),
        [section] if !section.label.trim().is_empty() => section.label.trim().to_owned(),
        _ => format!("{} sections · {field_count} fields", sections.len()),
    }
}

#[cfg(test)]
mod scale;
#[cfg(test)]
mod tests;

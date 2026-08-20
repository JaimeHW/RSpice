//! Model catalog, symbol contracts, PDK sections, authenticated includes, and
//! the source-owned model qualification and release gate.

mod manager;
mod qualification;

use qualification::*;
/// What the rest of the surfaces may read about model qualification. The glob
/// above is this module's own working vocabulary and stays private to it.
pub(super) use qualification::{ModelGateFact, QualificationGate, model_gate_facts};

use std::collections::{BTreeMap, BTreeSet};

use egui::Ui;
use sha2::{Digest as _, Sha256};

use crate::diagnostics::ConsoleMessage;
use crate::state::ModelBoundSymbolDefinition;
use crate::state::model_library::{
    DeviceModel, ModelCorrelationState, ModelLibrary, ModelQualificationState,
    ModelSourceEvidenceBinding, QualificationAnalysis, QualificationPlatform, short_digest,
};
use crate::workbench::RSpiceApp;
use crate::workbench::app::{
    open_create_model_bound_symbol_dialog, open_symbol_import_dialog,
    open_symbol_parameter_form_dialog,
};

use super::super::{RouteTransitionSource, SurfaceId, SurfaceRoute};
use crate::workbench::commands::{CommandAvailability, vocabulary::Command};
use crate::workbench::documents::model_editor::{self, ModelEditorSection};

pub fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    manager::show(ui, app);
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

fn model_key(library: &str, item: &str) -> String {
    format!("{library}\u{1f}{item}")
}

#[cfg(test)]
mod control_ratchet;
#[cfg(test)]
mod scale;
#[cfg(test)]
mod tests;

//! What the models feature's tests build from: the feature on its own over
//! a new session, the catalog fixtures its pages read, and the reader of
//! what a frame painted.

use std::path::PathBuf;

use egui::Color32;

use crate::product::ObjectRevision;
use crate::state::model_library::{DeviceModel, ModelLibrary, ModelType};
use crate::state::pdk_config::PdkConfig;
use crate::state::{Component, ComponentType, ModelsPage, Point};
use crate::workbench::app_state::Model;
use crate::workbench::documents::model_editor::{ModelEditorParts, save_open_candidate};
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::slots::ModelsState;
use crate::workbench::state::WorkbenchSession;
use rspice_app_state::workbench::workspace::restore_model_library_manager;

use super::ModelsFeature;

/// The models feature on its own, over what a new session starts with:
/// the model over the empty PDK configuration a test session reads, and a
/// new session's workbench.
pub(crate) fn harness() -> FeatureHarness<ModelsFeature> {
    FeatureHarness::with_model(
        Model::session_start(PdkConfig::default(), None, restore_model_library_manager),
        WorkbenchSession::default(),
    )
}

/// The parts opening a model in the editor writes, over the harness.
pub(crate) fn model_editor_parts(
    harness: &mut FeatureHarness<ModelsFeature>,
) -> ModelEditorParts<'_> {
    harness
        .state
        .model_editor_parts(&mut harness.model, &mut harness.wb, &harness.platform)
}

/// Save the model editor's open candidate as one guarded project revision,
/// in the feature's context, as the Model Editor's own save does.
pub(crate) fn save_open_model_candidate(
    harness: &mut FeatureHarness<ModelsFeature>,
) -> Result<ObjectRevision, String> {
    let cx = &mut harness.cx();
    save_open_candidate(
        &mut cx.state.model_editor_parts(cx.model, cx.wb, cx.platform),
        &mut cx.runs.preflight,
        cx.handles.run_controller,
    )
}

/// A kit holding two bins of `nch` and the card `nch_25`, and one NMOS
/// instance per entry of `models`, `M1` onward, each naming that model, with
/// the catalog on show.
pub(crate) fn kit_project(project: &mut Model, state: &mut ModelsState, models: &[&str]) {
    project.model_library_manager.clear();
    let mut kit = ModelLibrary::new("kit");
    for (name, line, length) in [
        ("nch.1", 10, (0.1e-6, 1e-6)),
        ("nch.2", 20, (1e-6, 10e-6)),
        ("nch_25", 30, (0.1e-6, 10e-6)),
    ] {
        let mut card =
            DeviceModel::new(name, ModelType::Nmos).with_geometry(length.0, length.1, 1e-6, 10e-6);
        card.spice_type = Some("NMOS".to_owned());
        card.file_path = Some(PathBuf::from("kit.lib"));
        card.source_line = Some(line);
        kit.add_model(card);
    }
    project.model_library_manager.add_library(kit);
    state.models_page = ModelsPage::Models;

    project.workspace.ensure_active_buffer();
    let mut schematic = project
        .workspace
        .active_schematic()
        .map(|source| source.clone_editor())
        .expect("an active schematic exists");
    for (index, model) in models.iter().enumerate() {
        let id = index as u64 + 1;
        let mut component =
            Component::new(id, ComponentType::Nmos, Point::new(index as i32 * 40, 0));
        component.name = format!("M{id}");
        component.params = format!("model={model}");
        schematic.document_mut_for_test().components.push(component);
    }
    project.workspace.save_active_schematic(&schematic);
    project.schematic = schematic;
}

/// Every string one frame painted, with the colour it was painted in.
///
/// The catalog's cells are painter text, so reading the shapes back is the
/// only way to judge what a row says — and, for the status column, what it
/// means by the tone it says it in.
pub(crate) fn painted_text(output: &egui::FullOutput) -> Vec<(String, Color32)> {
    fn walk(shape: &egui::epaint::Shape, out: &mut Vec<(String, Color32)>) {
        match shape {
            egui::epaint::Shape::Text(text) => out.push((
                text.galley.text().to_owned(),
                text.galley
                    .job
                    .sections
                    .first()
                    .map_or(Color32::TRANSPARENT, |section| section.format.color),
            )),
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    walk(shape, out);
                }
            }
            _ => {}
        }
    }
    let mut found = Vec::new();
    for clipped in &output.shapes {
        walk(&clipped.shape, &mut found);
    }
    found
}

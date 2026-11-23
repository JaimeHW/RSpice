//! The project inspector: what the Overview, Library Manager, Dependencies
//! and History pages show while nothing on them is selected.
//!
//! The project's name, description and top cell are edited in place and
//! applied as the project's properties are applied anywhere, each change one
//! step of Undo; which configuration runs use is set as the Configuration
//! page sets it. Where the project file is, its revision and when it was
//! saved are facts. A project open read-only shows every field as a fact,
//! its refusal on hover.

use std::path::{Path, PathBuf};

use egui::{Key, Ui};
use rspice_ui_kit::panels::section_header;

use crate::workbench::design_system::{
    property_row, property_row_choice, property_row_input, property_row_path,
};
use crate::workbench::project::configuration_state::{ConfigurationSelection, used_for_runs};
use crate::workbench::project::health;
use crate::workbench::project::properties::{
    ProjectPropertiesEdit, project_name, project_properties_refusal, saved_at,
};

use super::{ProjectInspector, ProjectInspectorAction};

/// How long a read of when the project file was saved stands before the
/// file is asked again: a save can land at any time, and the panel draws on
/// every frame.
const SAVED_READ_SECONDS: f64 = 2.0;

pub(super) fn project_properties(
    ui: &mut Ui,
    inspector: &ProjectInspector<'_>,
) -> Option<ProjectInspectorAction> {
    let model = inspector.model;
    let project = &model.workspace.content.project;
    let properties = project.properties();
    let refusal = project_properties_refusal(model);
    let mut edit = ProjectPropertiesEdit::default();

    section_header(ui, "Project", None);
    edit.name = edited_text(
        ui,
        "Name",
        &properties.name,
        (refusal, inspector.focus_name),
        |text| project_name(text).err(),
    );
    edit.description = edited_text(
        ui,
        "Description",
        &properties.description,
        (refusal, false),
        |_| None,
    );
    // A path is told apart by its end, so it is cut in the middle.
    property_row_path(
        ui,
        "Location",
        &health::location(model, inspector.lifecycle.platform)
            .unwrap_or_else(|| "Not saved".to_owned()),
    );
    property_row(ui, "Revision", &project.revision().get().to_string());
    if let Some(saved) = project
        .path
        .as_deref()
        .and_then(|path| saved_lately(ui, path))
    {
        property_row(ui, "Saved", &saved);
    }

    section_header(ui, "Setup", None);
    let top = format!("{}/{}", properties.root_library, properties.top_cell);
    match refusal {
        Some(refusal) => {
            property_row(ui, "Top cell", &top).on_hover_text(refusal);
        }
        None => {
            edit.top =
                property_row_choice(ui, "Top cell", "project-inspector.top", &top, true, || {
                    model
                        .project_top_candidates()
                        .into_iter()
                        .map(|(library, cell)| {
                            let name = format!("{library}/{cell}");
                            ((library, cell), name)
                        })
                        .collect()
                })
                .filter(|(library, cell)| {
                    (library, cell) != (&properties.root_library, &properties.top_cell)
                });
        }
    }
    let use_for_runs = runs_use(ui, inspector, refusal);
    let technology = project.technology_binding().map_or_else(
        || "None".to_owned(),
        |binding| binding.display_label().to_string(),
    );
    property_row(ui, "Technology", &technology);

    use_for_runs.or_else(|| (!edit.is_empty()).then_some(ProjectInspectorAction::Properties(edit)))
}

/// Which configuration runs use, chosen as the Configuration page chooses it.
fn runs_use(
    ui: &mut Ui,
    inspector: &ProjectInspector<'_>,
    refusal: Option<&'static str>,
) -> Option<ProjectInspectorAction> {
    let catalog = &inspector.model.workspace.content.configuration_sets;
    let shown = catalog
        .active()
        .map_or("Default", |configuration| configuration.name());
    const LABEL: &str = "Configuration for runs";
    if let Some(refusal) = refusal {
        property_row(ui, LABEL, shown).on_hover_text(refusal);
        return None;
    }
    property_row_choice(ui, LABEL, "project-inspector.runs", shown, true, || {
        std::iter::once((ConfigurationSelection::Default, "Default".to_owned()))
            .chain(catalog.configurations().iter().map(|configuration| {
                (
                    ConfigurationSelection::Configuration(configuration.id()),
                    configuration.name().to_owned(),
                )
            }))
            .collect()
    })
    .filter(|target| !used_for_runs(catalog, *target))
    .map(ProjectInspectorAction::UseForRuns)
}

/// A property edited in place: the project's `value` while the field rests,
/// what is typed while it is edited. Leaving the field takes the typed text
/// when it differs from `value` and `refuse` has nothing against it; Escape
/// puts the value back. While `refuse` objects, the field is marked and says
/// why on hover. A read-only project shows `value` as a fact, `read_only` on
/// hover. A field asked to take the keys (`focus`) takes them as it draws.
pub(super) fn edited_text(
    ui: &mut Ui,
    label: &str,
    value: &str,
    (read_only, focus): (Option<&'static str>, bool),
    refuse: impl Fn(&str) -> Option<String>,
) -> Option<String> {
    if let Some(read_only) = read_only {
        property_row(ui, label, value).on_hover_text(read_only);
        return None;
    }
    let id = ui.id().with(("project-inspector.typed", label));
    let mut text = ui
        .data(|data| data.get_temp::<String>(id))
        .unwrap_or_else(|| value.to_owned());
    let refusal = refuse(&text);
    let response = property_row_input(ui, label, &mut text, refusal.is_some());
    if focus {
        response.request_focus();
    }
    let response = match &refusal {
        Some(refusal) => response.on_hover_text(refusal),
        None => response,
    };
    if response.has_focus() {
        ui.data_mut(|data| data.insert_temp(id, text));
        return None;
    }
    ui.data_mut(|data| data.remove::<String>(id));
    let escaped = ui.input(|input| input.key_pressed(Key::Escape));
    (response.lost_focus() && !escaped && refusal.is_none() && text.trim() != value.trim())
        .then_some(text)
}

/// When the project file at `path` was saved, read from the file at most
/// once every [`SAVED_READ_SECONDS`].
fn saved_lately(ui: &Ui, path: &Path) -> Option<String> {
    let id = egui::Id::new("project-inspector.saved");
    let now = ui.input(|input| input.time);
    let held = ui.data(|data| data.get_temp::<(PathBuf, f64, Option<String>)>(id));
    if let Some((read_path, read_at, saved)) = held
        && read_path == path
        && now - read_at < SAVED_READ_SECONDS
    {
        return saved;
    }
    let saved = saved_at(path);
    ui.data_mut(|data| data.insert_temp(id, (path.to_path_buf(), now, saved.clone())));
    saved
}

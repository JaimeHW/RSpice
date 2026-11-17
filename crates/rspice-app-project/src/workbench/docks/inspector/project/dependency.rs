//! The inspector beside the Dependencies page: the selected library, with
//! the status the rail and the table give it, and every fact about it the
//! table has no room for: where it is read from and on whose authority, its
//! digest, its models and the files it was read from.

use egui::Ui;
use rspice_ui_kit::panels::{panel_inset, section_header};

use crate::state::model_library::ModelSourceAuthority;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::design_system::{property_row, property_row_path, property_row_problem};
use crate::workbench::project::dependency_status::{
    DependencyKind, LibraryStatus, UNSET, dependency_rows, shown_path,
};

use super::{ProjectInspector, ProjectInspectorAction, inspector_action};

pub(super) fn project_dependency(
    ui: &mut Ui,
    inspector: &ProjectInspector<'_>,
) -> Option<ProjectInspectorAction> {
    let model = inspector.model;
    // The rail's rows, so the status read here is the one the rail and the
    // table show for this library.
    let rows = dependency_rows(ui.ctx(), model);
    let row = inspector
        .dependency_selection
        .as_ref()
        .and_then(|selected| rows.iter().find(|row| &row.library == selected));

    if let Some(row) = row {
        section_header(ui, row.library.kind.label(), None);
        property_row(ui, "Name", &row.library.name);
        status_rows(ui, "Status", &row.status);
        if row.source != UNSET {
            property_row_path(ui, "Source", &row.source);
        }
        if row.version != UNSET {
            property_row(ui, "Version", &row.version);
        }
        match row.library.kind {
            DependencyKind::Design => design_facts(ui, model, &row.library.name),
            DependencyKind::Model => model_facts(ui, model, &row.library.name, &row.digest),
        }
        section_header(ui, "Actions", None);
        let label = match row.library.kind {
            DependencyKind::Design => "Open in Library",
            DependencyKind::Model => "Open in Models",
        };
        if inspector_action(ui, label) {
            return Some(ProjectInspectorAction::OpenDependency(row.library.clone()));
        }
    } else {
        section_header(ui, "Library", None);
        detail_text(ui, "No library selected");
    }
    None
}

/// What a design library's row has no column for: the technology it is
/// drawn for, and whether it can be changed.
fn design_facts(ui: &mut Ui, model: &crate::workbench::app_state::Model, name: &str) {
    let Some(library) = model.library_manager.get_library(name) else {
        return;
    };
    if !library.technology.is_empty() {
        property_row(ui, "Attached technology", &library.technology);
    }
    property_row(
        ui,
        "Access",
        if library.read_only {
            "Read only"
        } else {
            "Editable"
        },
    );
}

/// What a model library's row has no column for: on whose authority it is
/// read, its digest, how many models it holds and every file it was read
/// from, each path whole.
fn model_facts(ui: &mut Ui, model: &crate::workbench::app_state::Model, name: &str, digest: &str) {
    let Some(library) = model.model_library_manager.get_library(name) else {
        return;
    };
    let access = match library.source_authority {
        ModelSourceAuthority::BuiltIn => "Ships with RSpice",
        ModelSourceAuthority::External => "Linked to its file",
        ModelSourceAuthority::RetainedImport { .. } => "Kept as imported",
        ModelSourceAuthority::ProjectOwned { .. } => "Project-owned",
    };
    property_row(ui, "Access", access);
    if digest != UNSET {
        property_row(ui, "Digest", digest);
    }
    property_row(ui, "Models", &library.models.len().to_string());
    if library.source_closure.is_empty() {
        return;
    }
    section_header(ui, "Files", Some(&library.source_closure.len().to_string()));
    for pin in &library.source_closure {
        detail_text(ui, &shown_path(&pin.path));
    }
}

/// A status as one row, its mark in front of its word, and what more a
/// problem has to say wrapped under it. A sound status's detail is a count
/// of files or where the library comes from, which the rows under it state.
fn status_rows(ui: &mut Ui, label: &str, status: &LibraryStatus) {
    let problem = status.mark.problem();
    property_row_problem(ui, label, status.label, problem);
    if let (Some(_), Some(detail)) = (problem, status.detail.as_deref()) {
        detail_text(ui, detail);
    }
}

/// Words that may run longer than a property row, wrapped inside the
/// panel's inset rather than elided.
fn detail_text(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let inset = panel_inset(ui).round().clamp(0.0, f32::from(i8::MAX)) as i8;
    egui::Frame::new()
        .inner_margin(egui::Margin::symmetric(inset, 3))
        .show(ui, |ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(text)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
}

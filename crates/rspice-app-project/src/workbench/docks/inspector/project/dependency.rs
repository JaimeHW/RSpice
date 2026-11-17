//! The inspector beside the Dependencies page: the selected library, with
//! the status the rail and the table give it, and the project's technology.

use egui::Ui;
use rspice_ui_kit::panels::{panel_inset, section_header};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::design_system::{property_row, property_row_path, property_row_problem};
use crate::workbench::framework::command_registry::ids;
use crate::workbench::project::dependency_status::{
    DependencyKind, LibraryStatus, UNSET, dependency_rows, technology_status,
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
        if row.library.kind == DependencyKind::Design
            && model
                .library_manager
                .get_library(&row.library.name)
                .is_some_and(|library| library.read_only)
        {
            property_row(ui, "Access", "Read only");
        }
    } else {
        section_header(ui, "Library", None);
        detail_text(ui, "No library selected");
    }

    section_header(ui, "Project", None);
    status_rows(ui, "Technology", &technology_status(model));

    section_header(ui, "Actions", None);
    let mut action = None;
    if inspector_action(ui, "Library manager") {
        action = Some(ProjectInspectorAction::Execute(ids::MODELS_CATALOG));
    }
    action
}

/// A status as one row, its mark in front of its word, and what more it has
/// to say wrapped under it.
fn status_rows(ui: &mut Ui, label: &str, status: &LibraryStatus) {
    property_row_problem(ui, label, status.label, status.mark.problem());
    if let Some(detail) = status.detail.as_deref() {
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

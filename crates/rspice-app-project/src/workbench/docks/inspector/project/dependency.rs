//! The inspector beside the Dependencies page: the selected dependency's
//! properties, with the status the table gives it and every fact the table
//! has no room for. A design library kept in a folder of its own takes its
//! access here, as a choice.

use egui::Ui;
use rspice_ui_kit::panels::{panel_inset, property_row_combo, section_header};

use crate::state::model_library::ModelSourceAuthority;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::app_state::Model;
use crate::workbench::app_state::model::LibraryMode;
use crate::workbench::design_system::{property_row, property_row_path, property_row_problem};
use crate::workbench::project::dependency_status::{
    DependencyKind, DependencyRow, LibraryStatus, PROJECT_FILE_SOURCE, UNSET, dependency_rows,
    shown_path,
};
use crate::workbench::project::library_dialogs::library_files::{
    LibraryFileIntent, library_file_blocker,
};

use super::{ProjectInspector, ProjectInspectorAction};

pub(super) fn project_dependency(
    ui: &mut Ui,
    inspector: &ProjectInspector<'_>,
) -> Option<ProjectInspectorAction> {
    let model = inspector.model;
    // The table's rows, so the status read here is the one the table shows
    // for this dependency.
    let rows = dependency_rows(ui.ctx(), model);
    let Some(row) = inspector
        .dependency_selection
        .as_ref()
        .and_then(|selected| rows.iter().find(|row| &row.library == selected))
    else {
        section_header(ui, "Dependencies", None);
        detail_text(ui, "No dependency selected");
        return None;
    };
    section_header(ui, row.library.kind.label(), None);
    match row.library.kind {
        DependencyKind::Technology => technology_facts(ui, model, row),
        DependencyKind::Design => return design_library_facts(ui, model, row),
        DependencyKind::Model => model_facts(ui, model, row),
        DependencyKind::IncludePath => file_facts(ui, row, "Path"),
        DependencyKind::Deck | DependencyKind::Include => file_facts(ui, row, "Name"),
    }
    None
}

/// A path the row names, cut from the middle in mono; one the project file
/// holds is named in words.
fn source_row(ui: &mut Ui, label: &str, source: &str) {
    if source == PROJECT_FILE_SOURCE || !looks_like_path(source) {
        property_row(ui, label, source);
    } else if source != UNSET {
        property_row_path(ui, label, source);
    }
}

fn looks_like_path(text: &str) -> bool {
    text.contains(['/', '\\', '.'])
}

/// The technology's facts: what it is, where its runs read their models
/// from, the PDK package it was attached from, and the catalogue and the
/// engine it runs against.
fn technology_facts(ui: &mut Ui, model: &Model, row: &DependencyRow) {
    let Some(binding) = model.workspace.content.project.technology_binding() else {
        status_rows(ui, "Status", &row.status);
        return;
    };
    property_row(ui, "Technology", &binding.display_label());
    status_rows(ui, "Status", &row.status);
    property_row(ui, "Model library", binding.model_library());
    property_row_path(ui, "Main model file", &shown_path(binding.root_source()));
    let files = binding.source_closure().len();
    property_row(
        ui,
        "Model files",
        &format!("{files} file{}", if files == 1 { "" } else { "s" }),
    );
    match binding.pdk_package() {
        Some(pin) => {
            property_row(
                ui,
                "PDK package",
                &format!("{} {}", pin.package_id(), pin.revision()),
            );
            property_row(
                ui,
                "Process",
                &format!("{} nm \u{00b7} {}", pin.process_node_nm(), pin.stack_name()),
            );
            property_row(ui, "Publisher", pin.publisher_id());
            property_row(
                ui,
                "Manifest digest",
                &short_digest(&pin.manifest_digest().to_string()),
            );
            property_row(
                ui,
                "Archive digest",
                &short_digest(&pin.archive_digest().to_string()),
            );
        }
        None => {
            property_row(ui, "PDK package", "Not attached");
        }
    }
    let libraries = model.model_library_manager.library_count();
    let models = model.model_library_manager.total_model_count();
    property_row(
        ui,
        "Model catalog",
        &format!(
            "{libraries} librar{} \u{00b7} {models} model{}",
            if libraries == 1 { "y" } else { "ies" },
            if models == 1 { "" } else { "s" }
        ),
    );
    property_row(
        ui,
        "Execution",
        crate::state::ExecutionTarget::current().label(),
    );
}

/// A digest by its first characters, which tell two apart; the whole is in
/// the exported manifest.
fn short_digest(digest: &str) -> String {
    digest.chars().take(8).collect()
}

/// A design library: where it is read from, the folder that resolves to
/// here, its access and the technology it is drawn for. The access of one
/// kept in a folder of its own is a choice, through the library's own
/// Make editable or Make read-only; one the project file holds states it.
fn design_library_facts(
    ui: &mut Ui,
    model: &Model,
    row: &DependencyRow,
) -> Option<ProjectInspectorAction> {
    let name = &row.library.name;
    property_row(ui, "Name", name);
    status_rows(ui, "Status", &row.status);
    source_row(ui, "Source", &row.source);
    if let Some(folder) = row.resolved.as_deref() {
        property_row_path(ui, "Resolves to", folder);
    }
    let asked = access_row(ui, model, name);
    if let Some(library) = model.library_manager.get_library(name)
        && !library.technology.is_empty()
    {
        property_row(ui, "Attached technology", &library.technology);
    }
    asked
}

/// Access as a choice for a library kept in a folder of its own, as a fact
/// for one the project file holds. A change the library cannot take now is
/// refused with its reason under the pointer.
fn access_row(ui: &mut Ui, model: &Model, name: &str) -> Option<ProjectInspectorAction> {
    let read_only = model
        .library_manager
        .get_library(name)
        .is_some_and(|library| library.read_only);
    let current = if read_only {
        LibraryMode::ReadOnly
    } else {
        LibraryMode::Editable
    };
    let Some(reference) = model.library_manager.reference(name) else {
        property_row(ui, "Access", current.label());
        return None;
    };
    let current = if reference.mode.is_editable() {
        LibraryMode::Editable
    } else {
        LibraryMode::ReadOnly
    };
    let other = match current {
        LibraryMode::Editable => LibraryMode::ReadOnly,
        LibraryMode::ReadOnly => LibraryMode::Editable,
    };
    let intent = LibraryFileIntent::SetMode(name.to_owned(), other);
    let blocker = library_file_blocker(model, &intent);
    let options = [LibraryMode::Editable, LibraryMode::ReadOnly]
        .map(|mode| (mode.label().to_owned(), mode.label().to_owned()));
    let mut chosen = current.label().to_owned();
    let shown = ui.scope(|ui| {
        property_row_combo(
            ui,
            "Access",
            ("workbench.project.dependency.access", name),
            &mut chosen,
            &options,
            blocker.is_none(),
        )
    });
    if let Some(blocker) = blocker.as_deref() {
        shown.response.on_hover_text(blocker);
    }
    (shown.inner && chosen != current.label())
        .then_some(ProjectInspectorAction::LibraryFile(intent))
}

/// What the Library page's inspector states of a design library beside its
/// source: the folder it resolves to here, its access and the technology it
/// is drawn for.
pub(super) fn design_facts(ui: &mut Ui, model: &Model, name: &str) {
    let Some(library) = model.library_manager.get_library(name) else {
        return;
    };
    if let Some(folder) = model.library_folder(name) {
        property_row_path(ui, "Resolves to", &shown_path(folder));
    }
    property_row(
        ui,
        "Access",
        if library.read_only {
            LibraryMode::ReadOnly.label()
        } else {
            LibraryMode::Editable.label()
        },
    );
    if !library.technology.is_empty() {
        property_row(ui, "Attached technology", &library.technology);
    }
}

/// A model library: where it is read from and on whose authority, its
/// digest, its models and the files it was read from, each path whole.
fn model_facts(ui: &mut Ui, model: &Model, row: &DependencyRow) {
    property_row(ui, "Name", &row.library.name);
    status_rows(ui, "Status", &row.status);
    source_row(ui, "Source", &row.source);
    if row.version != UNSET {
        property_row(ui, "Version", &row.version);
    }
    property_row(ui, "Access", row.access);
    let Some(library) = model.model_library_manager.get_library(&row.library.name) else {
        return;
    };
    let authority = match library.source_authority {
        ModelSourceAuthority::BuiltIn => "Ships with RSpice",
        ModelSourceAuthority::External => "Linked to its file",
        ModelSourceAuthority::RetainedImport { .. } => "Kept as imported",
        ModelSourceAuthority::ProjectOwned { .. } => "Project-owned",
    };
    property_row(ui, "Authority", authority);
    if row.digest != UNSET {
        property_row(ui, "Digest", &row.digest);
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

/// An include search path, a deck or an include: the path as the project or
/// the deck writes it, what it resolves to here, and whether it is there.
fn file_facts(ui: &mut Ui, row: &DependencyRow, written: &str) {
    match row.library.kind {
        DependencyKind::Deck => property_row(ui, written, &row.library.name),
        _ => property_row_path(ui, written, &row.library.name),
    };
    status_rows(ui, "Status", &row.status);
    if row.library.kind == DependencyKind::Deck {
        source_row(ui, "Source", &row.source);
    }
    match row.resolved.as_deref() {
        Some(resolved) => property_row_path(ui, "Resolves to", resolved),
        None => property_row(ui, "Resolves to", UNSET),
    };
}

/// A status as one row, its mark in front of its word, and what more there
/// is to say wrapped under it: what a problem needs to act on it, or which
/// line names a file. A library's sound status counts its files, which the
/// rows under it state.
pub(super) fn status_rows(ui: &mut Ui, label: &str, status: &LibraryStatus) {
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

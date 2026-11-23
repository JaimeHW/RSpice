//! The inspector beside the Library page: the selected view, when it was
//! last saved, what its cell and its library are for, edited in place, and
//! where the library lives, its checks and the schematics that place it, or
//! the selected model when the built-in Verilog-A library is open. What can
//! be done with a row is its menu's and the page bar's.

use egui::Ui;
use rspice_app_state::veriloga_library::BuiltinVerilogAModel;
use rspice_ui_kit::panels::section_header;

use crate::state::{CellViewRef, ViewType};
use crate::workbench::app_state::DesignCheckStatus;
use crate::workbench::app_state::Model;
use crate::workbench::app_state::model::{LibraryFile, OnDisk};
use crate::workbench::design_system::{
    property_row, property_row_path, property_row_problem, property_row_wrapped,
};
use crate::workbench::project::dependency_status::{
    DependencyLibrary, PROJECT_FILE_SOURCE, library_status, view_exception,
};
use crate::workbench::project::library_dialogs::library_files::lock_holder;

use super::dependency::{design_facts, status_rows};
use super::properties::edited_text;
use super::{ProjectInspector, ProjectInspectorAction};

/// The selection in the built-in Verilog-A library. Nothing here acts: a
/// model's row menu copies it or opens its file.
fn builtin_model_selection(ui: &mut Ui, model: Option<&BuiltinVerilogAModel>) {
    section_header(ui, "Selection", None);
    if let Some(model) = model {
        property_row(ui, "Module", model.module);
        property_row(ui, "Version", model.version);
        property_row_wrapped(ui, "Summary", model.summary);
    }
    property_row(ui, "Access", "read-only");
}

/// Why a description in `library` stays as it is, if it does: the project
/// is open read-only, or the library is.
fn description_refusal(model: &Model, read_only: bool) -> Option<&'static str> {
    if model.safe_mode.project_read_only() {
        Some("The project is open read-only.")
    } else if read_only {
        Some("The library is read-only; make it editable from its menu to change this.")
    } else {
        None
    }
}

/// A description edited in place, under its own name in the panel so the
/// cell's and the library's never share what is typed.
fn description(
    ui: &mut Ui,
    salt: &str,
    value: &str,
    refusal: Option<&'static str>,
) -> Option<String> {
    ui.push_id(salt, |ui| {
        edited_text(ui, "Description", value, (refusal, false), |_| None)
    })
    .inner
}

pub fn project_library(
    ui: &mut Ui,
    inspector: &ProjectInspector<'_>,
) -> Option<ProjectInspectorAction> {
    if inspector.builtin_library_open {
        builtin_model_selection(ui, inspector.builtin_model);
        return None;
    }
    let model = inspector.model;
    let library = model.library_manager.current_library();
    // A library kept in a folder of its own is sourced from a path; one the
    // project file holds says so in words.
    let library_path = library
        .and_then(|item| model.library_manager.reference(&item.name))
        .map(|reference| {
            reference
                .path
                .shown(rspice_design::library::folder::HostPlatform::current())
        });
    // What the page's find hides is not the page's selection: the band
    // asks for a view, and so does this panel.
    let cell = model
        .library_manager
        .current_cell()
        .filter(|_| inspector.library_shown.cell);
    let view = model
        .library_manager
        .current_view()
        .filter(|_| inspector.library_shown.view);
    let open_target = match (model.library_manager.current_library(), cell, view) {
        (Some(library), Some(cell), Some(view)) => Some((
            CellViewRef::new(&library.name, &cell.name, &view.name),
            view.view_type,
        )),
        _ => None,
    };
    let usage = open_target
        .as_ref()
        .map(|(reference, _)| model.cell_view_consumers(reference));
    let (checks_meta, checks) = match open_target.as_ref() {
        Some((reference, ViewType::Schematic | ViewType::Testbench)) => {
            match model.design_check_status(reference, inspector.facts) {
                DesignCheckStatus::Current(receipt) => {
                    let summary = receipt.result.summary();
                    (
                        None,
                        Some(format!(
                            "{} errors \u{00b7} {} advisories",
                            summary.critical + summary.errors,
                            summary.warnings + summary.info
                        )),
                    )
                }
                DesignCheckStatus::NotRun => (Some("not run"), None),
                DesignCheckStatus::Stale(_) => (Some("out of date"), None),
                DesignCheckStatus::Unavailable { reason, .. } => {
                    (Some("unavailable"), Some(reason))
                }
            }
        }
        // A symbol or a source view has no schematic checks to report, and a
        // row saying so is a row about nothing.
        Some(_) | None => (None, None),
    };

    // The library browser beside this panel shows the library, the cell and
    // the view; what the panel adds is the view's type, what the cell is for
    // and where the library lives.
    let read_only = library.is_some_and(|item| item.read_only);
    let refusal = description_refusal(model, read_only);
    let mut asked = None;
    section_header(ui, "Selection", read_only.then_some("read-only"));
    property_row(
        ui,
        "View type",
        view.map_or("\u{2014}", |item| item.view_type.caption()),
    );
    // The path whole, with the cell's description and the file the view is
    // kept in.
    if let Some((reference, _)) = &open_target {
        property_row_path(
            ui,
            "Path",
            &format!(
                "{}/{}/{}",
                reference.library, reference.cell, reference.view
            ),
        );
    }
    if let (Some(item), Some(cell)) = (library, cell)
        && let Some(text) = description(ui, "cell", cell.description.trim(), refusal)
    {
        asked = Some(ProjectInspectorAction::LibraryDescription {
            library: item.name.clone(),
            cell: Some(cell.name.clone()),
            description: text,
        });
    }
    if let Some(file) = view.and_then(|item| item.file_path.as_ref()) {
        property_row_path(ui, "File", &file.display().to_string());
    }
    if let Some(saved) = view.and_then(|item| item.saved_at).and_then(saved_time) {
        property_row(ui, "Saved", &saved);
    }
    // A view kept in a library's own folder: its revisions, and who holds
    // its file. Another holder's lock is broken from the view's menu.
    if let (Some(library), Some(cell), Some(view)) = (library, cell, view)
        && model.library_manager.is_referenced(&library.name)
    {
        let file = LibraryFile::cell_view(&cell.name, view);
        if let Some(exception) = view_exception(model, &library.name, &cell.name, view) {
            property_row_problem(ui, "Status", &exception.label, exception.mark.problem());
        }
        if let Some(revision) = revision_text(model, &library.name, &file) {
            property_row(ui, "Revision", &revision);
        }
        if let Some(lock) = model.library_lock_elsewhere(&library.name, &file) {
            property_row(ui, "Lock", &lock_holder(lock));
        } else if model.held_library_lock(&library.name, &file).is_some() {
            property_row(ui, "Lock", "Held here");
        }
    }

    section_header(ui, "Library", None);
    if let Some(item) = library
        && let Some(text) = description(ui, "library", item.description(), refusal)
    {
        asked = Some(ProjectInspectorAction::LibraryDescription {
            library: item.name.clone(),
            cell: None,
            description: text,
        });
    }
    if let Some(item) = library {
        status_rows(
            ui,
            "Status",
            &library_status(model, &DependencyLibrary::design(&item.name)),
        );
    }
    // Cut from the middle, so the folder the library is kept in stays in
    // view however narrow the panel: as written, then where that resolves
    // here, its access and its technology.
    match &library_path {
        Some(path) => property_row_path(ui, "Source", path),
        None => property_row(ui, "Source", PROJECT_FILE_SOURCE),
    };
    if let Some(item) = library {
        design_facts(ui, model, &item.name);
    }
    // A view nobody has edited is in its normal state.
    if view.is_some_and(|item| item.modified) {
        property_row(ui, "State", "modified");
    }

    section_header(ui, "Validation", checks_meta);
    if let Some(checks) = &checks {
        property_row(ui, "Checks", checks);
    }
    // One row, naming the consumers rather than counting them and then
    // naming the first of them underneath.
    if let Some(usage) = usage.filter(|usage| !usage.is_empty()) {
        property_row(ui, "Used by", &usage.join(", "));
    }

    asked
}

/// A file's revisions: the one loaded here, the one on disk when it moved
/// on, and the one the project was saved with when that differs.
fn revision_text(model: &Model, library: &str, file: &LibraryFile) -> Option<String> {
    let session = model.library_files.library(library)?.files.get(file)?;
    let loaded = session.loaded.map(|loaded| loaded.revision);
    let mut parts = vec![match loaded {
        Some(revision) => format!("{revision} loaded"),
        None => "Not loaded".to_owned(),
    }];
    if let OnDisk::Changed(on_disk) = &session.on_disk {
        parts.push(format!("{} on disk", on_disk.revision));
    }
    if let Some(saved) = model
        .library_manager
        .reference(library)?
        .saved_revision(file)
        .filter(|saved| Some(saved.revision) != loaded)
    {
        parts.push(format!("saved with {}", saved.revision));
    }
    Some(parts.join(" \u{00b7} "))
}

/// When a save last wrote a view with a change, on the reader's clock, as
/// the project inspector states the project's own save.
fn saved_time(unix_ms: u64) -> Option<String> {
    let stamp = crate::time_compat::local_date_time(unix_ms)?;
    Some(format!(
        "{:04}-{:02}-{:02} {:02}:{:02}",
        stamp.year(),
        u8::from(stamp.month()),
        stamp.day(),
        stamp.hour(),
        stamp.minute()
    ))
}

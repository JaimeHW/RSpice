//! The inspector beside the Library page: what opening the selected view
//! does, when it was last saved, what its cell and its library are for and
//! where the library lives, its checks and the schematics that place it, or
//! the selected model when the built-in Verilog-A library is open.

use egui::Ui;
use rspice_app_state::veriloga_library::BuiltinVerilogAModel;
use rspice_ui_kit::panels::section_header;

use crate::state::{CellViewRef, ViewType};
use crate::workbench::app_state::DesignCheckStatus;
use crate::workbench::app_state::Model;
use crate::workbench::app_state::model::{LibraryFile, OnDisk};
use crate::workbench::design_system::{property_row, property_row_path, property_row_problem};
use crate::workbench::project::commands::{LibraryCommand, ProjectCommand, ProjectRequest};
use crate::workbench::project::dependency_status::{
    DependencyLibrary, library_status, view_exception,
};
use crate::workbench::project::library_dialogs::library_files::{LibraryFileIntent, lock_holder};

use super::dependency::{design_facts, mode_action, status_rows};
use super::{ProjectInspector, ProjectInspectorAction, inspector_action_enabled};

/// The selection in the built-in Verilog-A library. Nothing here acts: the
/// page's band owns what can be done with a model.
fn builtin_model_selection(ui: &mut Ui, model: Option<&BuiltinVerilogAModel>) {
    section_header(ui, "Selection", None);
    if let Some(model) = model {
        property_row(ui, "Module", model.module);
        property_row(ui, "Version", model.version);
    }
    property_row(ui, "Access", "read-only");
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
    let library_path = library
        .and_then(|item| model.library_manager.reference(&item.name))
        .map_or_else(
            || "Project file".to_owned(),
            |reference| {
                reference
                    .path
                    .shown(rspice_design::library::folder::HostPlatform::current())
            },
        );
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
    let view_type = view.map_or("—", |item| item.view_type.caption());
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
    // the view; what the panel adds is what opening the selection does, what
    // the cell is for and where the library lives.
    section_header(
        ui,
        "Selection",
        library
            .is_some_and(|item| item.read_only)
            .then_some("read-only"),
    );
    property_row(ui, "View type", view_type);
    // The band cuts the path to the width its controls leave; here it is whole,
    // with the cell's description and the file the view is kept in.
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
    if let Some(cell) = cell
        && !cell.description.trim().is_empty()
    {
        property_row(ui, "Description", cell.description.trim());
    }
    if let Some(file) = view.and_then(|item| item.file_path.as_ref()) {
        property_row_path(ui, "File", &file.display().to_string());
    }
    if let Some(saved) = view.and_then(|item| item.saved_at).and_then(saved_time) {
        property_row(ui, "Saved", &saved);
    }
    // A view kept in a library's own folder: its revisions, and who holds
    // its file. Another holder's lock can be broken from here.
    let mut break_lock = None;
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
            break_lock = Some(LibraryFileIntent::BreakLock {
                library: library.name.clone(),
                file,
            });
        } else if model.held_library_lock(&library.name, &file).is_some() {
            property_row(ui, "Lock", "Held here");
        }
    }

    section_header(ui, "Library", None);
    if let Some(description) = library
        .map(|item| item.description())
        .filter(|description| !description.is_empty())
    {
        property_row(ui, "Description", description);
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
    property_row_path(ui, "Source", &library_path);
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

    section_header(ui, "Actions", None);
    let open = inspector_action_enabled(ui, "Open selected view", open_target.is_some());
    // Library properties in the library column, Cell properties beside a
    // cell or a view: the page's Properties command, on what it acts on.
    let properties = inspector_action_enabled(ui, "Properties\u{2026}", library.is_some());
    let mode = library.and_then(|item| mode_action(ui, model, &item.name));
    let broken = match break_lock {
        Some(intent) if inspector_action_enabled(ui, "Break lock\u{2026}", true) => Some(intent),
        _ => None,
    };
    if let Some(intent) = mode.or(broken) {
        return Some(ProjectInspectorAction::LibraryFile(intent));
    }
    // No "Where used & impact…": it wrote the consumers to the console, and
    // the "Used by" row above names them in place. No "Revision history…"
    // either — it is a project-level dialog, and the Project overview
    // inspector is where the project's own actions live.
    if properties {
        return Some(ProjectInspectorAction::Execute(
            ProjectCommand::Library(LibraryCommand::Properties).key(),
        ));
    }
    open_target.filter(|_| open).map(|(reference, view_type)| {
        ProjectInspectorAction::Shell(ProjectRequest::OpenView(reference, view_type))
    })
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
/// Project properties states the project's own save.
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

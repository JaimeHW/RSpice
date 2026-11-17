//! The inspector beside the Library page: what opening the selected view
//! does, when it was last saved, what its cell and its library are for and
//! where the library lives, its checks and the schematics that place it, or
//! the selected model when the built-in Verilog-A library is open.

use egui::Ui;
use rspice_app_state::veriloga_library::BuiltinVerilogAModel;
use rspice_ui_kit::panels::section_header;

use crate::state::{CellViewRef, ViewType};
use crate::workbench::app_state::DesignCheckStatus;
use crate::workbench::design_system::{property_row, property_row_path};
use crate::workbench::project::commands::{LibraryCommand, ProjectCommand, ProjectRequest};

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
    let library_path = library.and_then(|item| item.path.as_ref()).map_or_else(
        || "In this project".to_owned(),
        |path| path.display().to_string(),
    );
    let technology = library.map_or("unbound", |item| {
        if item.technology.trim().is_empty() {
            "unbound"
        } else {
            item.technology.as_str()
        }
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

    section_header(ui, "Library", None);
    if let Some(description) = library
        .map(|item| item.description())
        .filter(|description| !description.is_empty())
    {
        property_row(ui, "Description", description);
    }
    if technology != "unbound" {
        property_row(ui, "Technology", technology);
    }
    // Cut from the middle, so the folder the library is kept in stays in
    // view however narrow the panel.
    property_row_path(ui, "Location", &library_path);
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

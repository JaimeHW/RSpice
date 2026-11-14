//! New Cell — small modal on the dialog primitive: target library, name,
//! description, and the views to seed the cell with.

use egui::Context;

use crate::state::workspace::VERILOGA_LIBRARY_NAME;
use crate::ui::widgets::{CommandForm, DialogChoice};
use crate::workbench::app::dialogs::library::{LIBRARY_CATALOG_STALE_MESSAGE, validate_lcv_name};
use crate::workbench::framework::feature::Cx;
use crate::workbench::project::ProjectFeature;
use crate::workbench::project::commands::{ProjectRequest, ReservedLibraryRevision};

use super::new_view::seeded_symbol_view;

/// What the footer says: why Create is refused, or, while the form waits for
/// a name, that it does, as a plain fact.
fn new_cell_status<'a>(
    catalog_current: bool,
    any_writable_library: bool,
    name: &str,
    error: Option<&'a str>,
) -> Result<Option<&'a str>, &'a str> {
    if !catalog_current {
        return Err(LIBRARY_CATALOG_STALE_MESSAGE);
    }
    if !any_writable_library {
        return Err("There is no writable library to create the cell in.");
    }
    if let Some(error) = error {
        return Err(error);
    }
    if name.trim().is_empty() {
        return Ok(Some("Enter a name for the cell."));
    }
    Ok(None)
}

impl ProjectFeature {
    /// Draw New Cell. What a created cell leaves outside its catalog change
    /// goes to the shell, which carries it out as the dialog returns.
    pub(crate) fn process_new_cell_dialog(cx: &mut Cx<'_, Self>, ctx: &Context) {
        if !cx.state.new_cell_dialog {
            return;
        }

        let mut should_close = false;
        let mut should_create = false;

        let lib_names: Vec<String> = cx
            .model
            .library_manager
            .libraries_sorted()
            .iter()
            .filter(|lib| !lib.read_only)
            .map(|lib| lib.name.clone())
            .collect();
        if cx.state.new_cell_library.is_empty() && !lib_names.is_empty() {
            cx.state.new_cell_library = lib_names[0].clone();
        }
        let catalog_current =
            cx.model.library_manager.revision() == cx.state.new_cell_library_revision;
        if !catalog_current {
            cx.state.new_cell_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        }

        let dialogs = &mut *cx.state;
        let status = new_cell_status(
            catalog_current,
            !lib_names.is_empty(),
            &dialogs.new_cell_name,
            dialogs.new_cell_error.as_deref(),
        );
        let mut name_changed = false;
        let choice = CommandForm::new("New cell", "Create")
            .option_columns()
            .describe("Create a named cell in a writable library with the selected initial views.")
            .primary_enabled(
                catalog_current
                    && !lib_names.is_empty()
                    && !dialogs.new_cell_name.trim().is_empty(),
            )
            .status(status)
            .show(ctx, |rows| {
                if let Some(index) = rows.select(
                    "Library",
                    "new-cell-library",
                    &dialogs.new_cell_library,
                    &lib_names,
                ) {
                    dialogs.new_cell_library = lib_names[index].clone();
                }
                let name = egui::Id::new("new-cell-name");
                name_changed = rows
                    .text("Cell name", name, &mut dialogs.new_cell_name, "")
                    .changed();
                rows.text(
                    "Description",
                    egui::Id::new("new-cell-description"),
                    &mut dialogs.new_cell_description,
                    "",
                );
                rows.check_columns(
                    "Views",
                    &mut [
                        ("Schematic", &mut dialogs.new_cell_create_schematic),
                        ("Symbol", &mut dialogs.new_cell_create_symbol),
                        ("Testbench", &mut dialogs.new_cell_create_testbench),
                        ("Verilog-A", &mut dialogs.new_cell_create_veriloga),
                    ],
                );
                Some(name)
            });
        // A refusal of the create belongs to the name it refused; a new name
        // has not been refused yet.
        if name_changed && catalog_current {
            cx.state.new_cell_error = None;
        }

        match choice {
            DialogChoice::Primary => should_create = true,
            DialogChoice::Ghost | DialogChoice::Cancelled => should_close = true,
            DialogChoice::Secondary | DialogChoice::None => {}
        }

        if should_create {
            should_close |= create_new_cell(cx);
        }

        if should_close {
            cx.state.new_cell_dialog = false;
            cx.state.new_cell_name.clear();
            cx.state.new_cell_description.clear();
            cx.state.new_cell_error = None;
            cx.state.new_cell_create_schematic = true;
            cx.state.new_cell_create_symbol = false;
            cx.state.new_cell_create_testbench = false;
            cx.state.new_cell_create_veriloga = false;
        }
    }
}

/// Create the cell the dialog names, and whether it was created.
///
/// The cell and every view it starts with are made on copies of the
/// catalog, so a refusal anywhere creates nothing; the catalog then takes
/// the copies, and what the creation leaves outside it goes to the shell
/// in today's order. A refusal is stated in the dialog.
pub(crate) fn create_new_cell(cx: &mut Cx<'_, ProjectFeature>) -> bool {
    let name = cx.state.new_cell_name.trim().to_string();
    let library = cx.state.new_cell_library.clone();

    if cx.model.library_manager.revision() != cx.state.new_cell_library_revision {
        cx.state.new_cell_error = Some(LIBRARY_CATALOG_STALE_MESSAGE.to_owned());
        return false;
    }
    if let Some(error) = validate_lcv_name(&name, "Cell name") {
        cx.state.new_cell_error = Some(error);
        return false;
    }
    if library.is_empty() {
        cx.state.new_cell_error = Some("Please select a library".to_string());
        return false;
    }

    let Some(lib_ro) = cx.model.library_manager.get_library(&library) else {
        cx.state.new_cell_error = Some(format!("Library '{}' not found", library));
        return false;
    };
    if lib_ro.read_only {
        cx.state.new_cell_error = Some(format!(
            "Library '{library}' is read only; create the cell in an editable library"
        ));
        return false;
    }
    let requested_identity = crate::state::canonical_cell_view_owner_key(&library, &name, "");
    if lib_ro.cells.values().any(|cell| {
        crate::state::canonical_cell_view_owner_key(&library, &cell.name, "") == requested_identity
    }) {
        cx.state.new_cell_error = Some(format!(
            "Cell '{}' clashes with an existing cell in library '{}' (cell names ignore letter case)",
            name, library
        ));
        return false;
    }

    use crate::state::{Cell, View, ViewType};

    let mut cell = Cell::new(&name);
    cell.description = cx.state.new_cell_description.clone();

    if cx.state.new_cell_create_schematic {
        cell.add_view(View::new("schematic", ViewType::Schematic));
    }
    if cx.state.new_cell_create_symbol {
        // A cell created here starts on an empty schematic, so its symbol
        // starts empty too and grows with the interface the user draws.
        match seeded_symbol_view("symbol", &[]) {
            Ok(view) => cell.add_view(view),
            Err(error) => {
                cx.state.new_cell_error = Some(error);
                return false;
            }
        }
    }
    if cx.state.new_cell_create_testbench {
        cell.add_view(View::new("testbench", ViewType::Testbench));
    }

    // Every view is made on copies, the Verilog-A view's source and
    // compiled interface included, so a refusal anywhere creates nothing.
    let mut candidate_libraries = cx.model.library_manager.clone();
    let Some(mut candidate_library) = candidate_libraries.edit_library(&library) else {
        cx.state.new_cell_error = Some(format!("Library '{}' not found", library));
        return false;
    };
    candidate_library.add_cell(cell);
    let veriloga =
        crate::state::CellViewRef::new(&library, &name, crate::state::workspace::VERILOGA_VIEW);
    let seeded_veriloga = if cx.state.new_cell_create_veriloga {
        let mut candidate_sources = cx.model.workspace.content.project_sources.clone();
        match crate::workbench::app_state::model::seed_veriloga_view(
            cx.model,
            &mut candidate_libraries,
            &mut candidate_sources,
            &veriloga,
            &[],
        ) {
            Ok(receipt) => Some((candidate_sources, receipt)),
            Err(error) => {
                cx.state.new_cell_error = Some(error);
                return false;
            }
        }
    } else {
        None
    };

    let project_mutation = match cx.model.preflight_project_library_mutation(
        crate::state::ProjectLibraryMutation::CreateCell {
            library: library.clone(),
            cell: name.clone(),
        },
    ) {
        Ok(prepared) => prepared,
        Err(error) => {
            cx.state.new_cell_error = Some(error);
            return false;
        }
    };
    cx.model.library_manager = candidate_libraries;
    let receipt = seeded_veriloga.map(|(sources, receipt)| {
        cx.model.workspace.content.project_sources = sources;
        cx.model.workspace.content.project_sources_dirty = true;
        receipt
    });
    let schematics = [
        cx.state.new_cell_create_schematic.then_some("schematic"),
        cx.state.new_cell_create_testbench.then_some("testbench"),
    ]
    .into_iter()
    .flatten()
    .map(|view| crate::state::CellViewRef::new(&library, &name, view))
    .collect();
    // A schematic defines the cell's ports, so it opens first; a Verilog-A
    // view without one is the module to write next.
    let view_to_open = if cx.state.new_cell_create_schematic {
        Some("schematic")
    } else if receipt.is_some() {
        Some(veriloga.view.as_str())
    } else if cx.state.new_cell_create_symbol {
        Some("symbol")
    } else if cx.state.new_cell_create_testbench {
        Some("testbench")
    } else {
        None
    };
    let open = view_to_open.map(|view| crate::state::CellViewRef::new(&library, &name, view));
    cx.wb.outbox.push(ProjectRequest::CellViewsCreated {
        schematics,
        mutation: ReservedLibraryRevision::new(project_mutation),
        notice: format!("Created cell '{name}' in library '{library}'."),
        open,
        veriloga: receipt.map(|receipt| Box::new((veriloga, receipt))),
        persist_global_veriloga: library == VERILOGA_LIBRARY_NAME,
    });
    cx.state.new_cell_error = None;
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A disabled Create says why: a name it waits for as a fact, anything
    /// that refuses it as a refusal.
    #[test]
    fn the_new_cell_form_says_why_create_is_off() {
        assert_eq!(
            new_cell_status(true, true, " ", None),
            Ok(Some("Enter a name for the cell."))
        );
        assert_eq!(new_cell_status(true, true, "amp", None), Ok(None));
        assert_eq!(
            new_cell_status(false, true, "amp", None),
            Err(LIBRARY_CATALOG_STALE_MESSAGE)
        );
        assert!(new_cell_status(true, false, "amp", None).is_err());
        assert_eq!(
            new_cell_status(true, true, "amp", Some("A cell named amp exists.")),
            Err("A cell named amp exists.")
        );
    }
}

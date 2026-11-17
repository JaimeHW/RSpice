//! The navigator's library rail, on the Overview and Library pages: the
//! libraries the Library page's column lists, the project's and then the
//! built-in one, those the filter matches by library, cell or view name. The
//! project's rows are built when the catalogue or the filter changes, not on
//! every frame.

use std::sync::Arc;

use egui::Ui;
use rspice_app_state::veriloga_library::{BUILTIN_VERILOGA_LIBRARY, BUILTIN_VERILOGA_MODELS};

use crate::workbench::app_state::Model;
use crate::workbench::design_system::{WorkbenchIcon, section_header};
use crate::workbench::surfaces::project::derived::derived;
use crate::workbench::surfaces::project::library::LibraryIntent;
use crate::workbench::surfaces::project::library::builtin_models::BuiltinModelsIntent;

use super::super::nav_row;
use super::ProjectNavigator;

/// The rail's rows; a click is returned as the Library page's own intent,
/// so the rail and the page's column select the same way.
pub(super) fn project_library_navigator(
    ui: &mut Ui,
    nav: &mut ProjectNavigator<'_>,
) -> Option<LibraryIntent> {
    let query = nav.filter.trim().to_ascii_lowercase();
    let key = (nav.model.library_manager.catalog().stamp(), query.clone());
    let model: &Model = nav.model;
    let libraries = derived(ui.ctx(), "navigator.libraries", key, || {
        library_rail_rows(model, &query)
    });
    let builtin_matches = query.is_empty()
        || BUILTIN_VERILOGA_LIBRARY
            .to_ascii_lowercase()
            .contains(&query)
        || BUILTIN_VERILOGA_MODELS
            .iter()
            .any(|model| model.name.to_ascii_lowercase().contains(&query));
    let total = libraries.len() + 1;
    let visible = libraries
        .iter()
        .filter(|(_, _, _, visible)| *visible)
        .count()
        + usize::from(builtin_matches);
    // The ratio is a fact only while the filter is hiding something; with no
    // filter it reads "3 / 3" on every project.
    section_header(
        ui,
        "Libraries",
        (!query.is_empty())
            .then(|| format!("{visible} / {total}"))
            .as_deref(),
    );
    let selected = nav
        .model
        .library_manager
        .selected_library
        .as_deref()
        .filter(|_| !nav.builtin_library_open);
    let mut intent = None;
    for (name, label, read_only, visible) in libraries.iter() {
        if !visible {
            continue;
        }
        // Writable is every ordinary library and the cell count is not acted
        // on here, so the only meta a row carries is the one exception.
        if nav_row(
            ui,
            if *read_only {
                WorkbenchIcon::Folder
            } else {
                WorkbenchIcon::Project
            },
            label,
            selected == Some(name.as_str()),
            read_only.then_some("read-only"),
        ) {
            intent = Some(LibraryIntent::SelectLibrary(name.clone()));
        }
    }
    if builtin_matches
        && nav_row(
            ui,
            WorkbenchIcon::Folder,
            BUILTIN_VERILOGA_LIBRARY,
            nav.builtin_library_open,
            Some("read-only"),
        )
    {
        intent = Some(LibraryIntent::BuiltinModels(
            BuiltinModelsIntent::ShowLibrary,
        ));
    }
    intent
}

/// Each library by name and the name it is shown by, whether it is
/// read-only, and whether the filter matches it by either name or a cell's or
/// a view's.
fn library_rail_rows(model: &Model, query: &str) -> Arc<[(String, String, bool, bool)]> {
    model
        .library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| {
            let matches_query = query.is_empty()
                || library.name.to_ascii_lowercase().contains(query)
                || library.display_name().to_ascii_lowercase().contains(query)
                || library.cells_sorted().into_iter().any(|cell| {
                    cell.name.to_ascii_lowercase().contains(query)
                        || cell
                            .views_sorted()
                            .into_iter()
                            .any(|view| view.name.to_ascii_lowercase().contains(query))
                });
            (
                library.name.clone(),
                library.display_name().to_owned(),
                library.read_only,
                matches_query,
            )
        })
        .collect()
}

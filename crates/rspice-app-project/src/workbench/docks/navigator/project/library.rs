//! The navigator's library rail, on the Overview and Library pages: the
//! project's libraries, those the filter matches by library, cell or view
//! name. The rows are built when the catalogue or the filter changes, not on
//! every frame.

use std::sync::Arc;

use egui::Ui;

use crate::workbench::app_state::Model;
use crate::workbench::design_system::{WorkbenchIcon, section_header};
use crate::workbench::surfaces::project::derived::derived;

use super::super::nav_row;
use super::ProjectNavigator;

pub(super) fn project_library_navigator(ui: &mut Ui, nav: &mut ProjectNavigator<'_>) {
    let query = nav.filter.trim().to_ascii_lowercase();
    let key = (nav.model.library_manager.catalog().stamp(), query.clone());
    let model: &Model = nav.model;
    let libraries = derived(ui.ctx(), "navigator.libraries", key, || {
        library_rail_rows(model, &query)
    });
    let visible = libraries.iter().filter(|(_, _, visible)| *visible).count();
    // The ratio is a fact only while the filter is hiding something; with no
    // filter it reads "3 / 3" on every project.
    section_header(
        ui,
        "Libraries",
        (!query.is_empty())
            .then(|| format!("{visible} / {}", libraries.len()))
            .as_deref(),
    );
    for (name, read_only, visible) in libraries.iter() {
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
            name,
            nav.model.library_manager.selected_library.as_deref() == Some(name.as_str()),
            read_only.then_some("read-only"),
        ) {
            nav.model.library_manager.select_library(name);
        }
    }
}

/// Each library by name, whether it is read-only, and whether the filter
/// matches it by its own name or a cell's or a view's.
fn library_rail_rows(model: &Model, query: &str) -> Arc<[(String, bool, bool)]> {
    model
        .library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| {
            let matches_query = query.is_empty()
                || library.name.to_ascii_lowercase().contains(query)
                || library.cells_sorted().into_iter().any(|cell| {
                    cell.name.to_ascii_lowercase().contains(query)
                        || cell
                            .views_sorted()
                            .into_iter()
                            .any(|view| view.name.to_ascii_lowercase().contains(query))
                });
            (library.name.clone(), library.read_only, matches_query)
        })
        .collect()
}

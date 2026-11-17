//! The navigator's library rail, on the Overview and Library pages: the
//! project's libraries, those the filter matches by library, cell or view
//! name.

use egui::Ui;

use crate::workbench::design_system::{WorkbenchIcon, section_header};

use super::super::nav_row;
use super::ProjectNavigator;

pub(super) fn project_library_navigator(ui: &mut Ui, nav: &mut ProjectNavigator<'_>) {
    let query = nav.filter.trim().to_ascii_lowercase();
    let libraries = nav
        .model
        .library_manager
        .libraries_sorted()
        .into_iter()
        .map(|library| {
            let matches_query = query.is_empty()
                || library.name.to_ascii_lowercase().contains(&query)
                || library.cells_sorted().into_iter().any(|cell| {
                    cell.name.to_ascii_lowercase().contains(&query)
                        || cell
                            .views_sorted()
                            .into_iter()
                            .any(|view| view.name.to_ascii_lowercase().contains(&query))
                });
            (library.name.clone(), library.read_only, matches_query)
        })
        .collect::<Vec<_>>();
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
    for (name, read_only, visible) in libraries {
        if !visible {
            continue;
        }
        // Writable is every ordinary library and the cell count is not acted
        // on here, so the only meta a row carries is the one exception.
        if nav_row(
            ui,
            if read_only {
                WorkbenchIcon::Folder
            } else {
                WorkbenchIcon::Project
            },
            &name,
            nav.model.library_manager.selected_library.as_deref() == Some(name.as_str()),
            read_only.then_some("read-only"),
        ) {
            nav.model.library_manager.select_library(&name);
        }
    }
}

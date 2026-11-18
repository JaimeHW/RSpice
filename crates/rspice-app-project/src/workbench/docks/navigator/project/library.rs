//! The navigator's library rail, on the Overview and Library pages: the
//! libraries the Library page's column lists, the project's and then the
//! built-in one, those the filter matches by library, cell or view name. The
//! project's rows are built when the catalogue or the filter changes, not on
//! every frame. Each row prints only its exception, the words the Library
//! page's column prints for it.

use std::borrow::Cow;
use std::sync::Arc;

use egui::Ui;
use rspice_app_state::veriloga_library::{BUILTIN_VERILOGA_LIBRARY, BUILTIN_VERILOGA_MODELS};

use crate::workbench::app_state::Model;
use crate::workbench::design_system::{WorkbenchIcon, section_header};
use crate::workbench::project::dependency_status::library_exceptions;
use crate::workbench::surfaces::project::derived::derived;
use crate::workbench::surfaces::project::library::LibraryIntent;
use crate::workbench::surfaces::project::library::builtin_models::BuiltinModelsIntent;

use super::super::{nav_row, nav_row_response};
use super::ProjectNavigator;
use super::dependencies::{MARK_ROOM, paint_status_mark, rail_status};

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
    let exceptions = library_exceptions(ui.ctx(), model);
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
        // Writable and Found is every ordinary library and the cell count is
        // not acted on here, so the only meta a row carries is its exception.
        // A marked one keeps the mark's room in front of its words, as the
        // dependency rail does.
        let exception = exceptions.get(name);
        let problem = exception.and_then(|exception| exception.mark.problem());
        let shown = exception.map(|exception| rail_status(ui, &exception.label, problem.is_some()));
        let meta = shown.as_ref().map(|shown| match problem {
            Some(_) => Cow::Owned(format!("{MARK_ROOM}{shown}")),
            None => shown.clone(),
        });
        let response = nav_row_response(
            ui,
            if *read_only {
                WorkbenchIcon::Folder
            } else {
                WorkbenchIcon::Project
            },
            label,
            selected == Some(name.as_str()),
            meta.as_deref(),
        );
        if let (Some(problem), Some(shown)) = (problem, shown.as_deref()) {
            paint_status_mark(ui, response.rect, shown, problem);
        }
        // The detail under the pointer, or the status whole when the row
        // had room for only part of it.
        let hover = exception.and_then(|exception| {
            exception.detail.as_deref().or_else(|| {
                (shown.as_deref() != Some(exception.label.as_str()))
                    .then_some(exception.label.as_str())
            })
        });
        let response = match hover {
            Some(hover) => response.on_hover_text(hover),
            None => response,
        };
        if response.clicked() {
            intent = Some(LibraryIntent::SelectLibrary(name.clone()));
        }
    }
    if builtin_matches
        && nav_row(
            ui,
            WorkbenchIcon::Folder,
            BUILTIN_VERILOGA_LIBRARY,
            nav.builtin_library_open,
            Some("Read-only"),
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

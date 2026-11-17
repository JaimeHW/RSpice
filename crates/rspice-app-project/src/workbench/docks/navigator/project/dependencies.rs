//! The navigator's dependency rail: the design and model libraries the
//! project depends on, with a row meta only for the exception. The rows are
//! built when a catalogue or the filter changes, not on every frame.

use std::sync::Arc;

use egui::Ui;

use crate::workbench::app_state::Model;
use crate::workbench::design_system::{WorkbenchIcon, section_header};
use crate::workbench::surfaces::project::derived::derived;

use super::super::nav_row;
use super::ProjectNavigator;

/// One rail row: the library's name, its kind, the exception it carries if
/// any, and its icon.
type DependencyRailRow = (String, &'static str, Option<&'static str>, WorkbenchIcon);

pub(super) fn project_dependency_navigator(ui: &mut Ui, nav: &mut ProjectNavigator<'_>) {
    let query = nav.filter.trim().to_ascii_lowercase();
    let key = (
        nav.model.library_manager.catalog().stamp(),
        nav.model.model_library_manager.catalog_stamp(),
        query.clone(),
    );
    let model: &Model = nav.model;
    let rows = derived(ui.ctx(), "navigator.dependencies", key, || {
        dependency_rail_rows(model, &query)
    });
    if nav.dependency_selection.is_none() {
        *nav.dependency_selection = rows.first().map(|(name, _, _, _)| name.clone());
    }
    section_header(ui, "Dependencies", Some(&rows.len().to_string()));
    for (name, _, status, icon) in rows.iter() {
        if nav_row(
            ui,
            *icon,
            name,
            nav.dependency_selection.as_deref() == Some(name.as_str()),
            *status,
        ) {
            *nav.dependency_selection = Some(name.clone());
        }
    }
}

fn dependency_rail_rows(model: &Model, query: &str) -> Arc<[DependencyRailRow]> {
    let mut rows = model
        .library_manager
        .libraries_sorted()
        .into_iter()
        // A writable design library and a model library whose sources resolve
        // are the ordinary cases; only the exception is worth a row's meta.
        .map(|library| {
            (
                library.name.clone(),
                "Design library",
                library.read_only.then_some("read-only"),
                WorkbenchIcon::Project,
            )
        })
        .chain(
            model
                .model_library_manager
                .libraries_sorted()
                .into_iter()
                .map(|library| {
                    (
                        library.name.clone(),
                        "Model library",
                        (!library.source_authority.has_execution_source()
                            || library.source_closure.is_empty())
                        .then_some("no model files"),
                        WorkbenchIcon::Models,
                    )
                }),
        )
        .filter(|(name, kind, status, _)| {
            query.is_empty()
                || format!("{name} {kind} {}", status.unwrap_or_default())
                    .to_ascii_lowercase()
                    .contains(&query)
        })
        .collect::<Vec<_>>();
    rows.sort_by(|left, right| left.0.cmp(&right.0));
    rows.into()
}

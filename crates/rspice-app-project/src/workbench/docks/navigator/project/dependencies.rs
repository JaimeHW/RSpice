//! The navigator's dependency rail: the design and model libraries the
//! project depends on, with a row meta only for the exception.

use egui::Ui;

use crate::workbench::design_system::{WorkbenchIcon, section_header};

use super::super::nav_row;
use super::ProjectNavigator;

pub(super) fn project_dependency_navigator(ui: &mut Ui, nav: &mut ProjectNavigator<'_>) {
    let query = nav.filter.trim().to_ascii_lowercase();
    let mut rows = nav
        .model
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
            nav.model
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
    if nav.dependency_selection.is_none() {
        *nav.dependency_selection = rows.first().map(|(name, _, _, _)| name.clone());
    }
    section_header(ui, "Dependencies", Some(&rows.len().to_string()));
    for (name, _, status, icon) in &rows {
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

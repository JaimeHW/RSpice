//! The Project workspace's navigator: the library, configuration,
//! dependency and recovery rails.

use egui::{ScrollArea, Ui};

use crate::workbench::RSpiceApp;

use super::super::super::commands::vocabulary::Command;
use super::super::super::design_system::{WorkbenchIcon, section_header};
use super::super::super::state::ProjectPage;
use super::{muted, nav_row};

pub(in crate::workbench) fn project(ui: &mut Ui, app: &mut RSpiceApp) {
    ScrollArea::vertical()
        .auto_shrink([false, true])
        .show(ui, |ui| match app.state.project.project_page {
            ProjectPage::Overview | ProjectPage::Library => project_library_navigator(ui, app),
            ProjectPage::Configuration => project_configuration_navigator(ui, app),
            ProjectPage::Dependencies => project_dependency_navigator(ui, app),
            ProjectPage::Recovery => project_recovery_navigator(ui, app),
        });
}

fn project_library_navigator(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = app
        .state
        .workbench
        .navigator_filter()
        .trim()
        .to_ascii_lowercase();
    let libraries = app
        .state
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
            app.state.model.library_manager.selected_library.as_deref() == Some(name.as_str()),
            read_only.then_some("read-only"),
        ) {
            app.state.model.library_manager.select_library(&name);
        }
    }
}

fn project_configuration_navigator(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = app
        .state
        .workbench
        .navigator_filter()
        .trim()
        .to_ascii_lowercase();
    let active_id = app
        .state
        .model
        .workspace
        .content
        .configuration_sets
        .active_configuration_id();
    let configurations = app
        .state
        .model
        .workspace
        .content
        .configuration_sets
        .configurations()
        .iter()
        .filter(|configuration| {
            query.is_empty()
                || configuration.name().to_ascii_lowercase().contains(&query)
                || configuration
                    .root()
                    .display_path()
                    .to_ascii_lowercase()
                    .contains(&query)
        })
        .map(|configuration| (configuration.id(), configuration.name().to_owned()))
        .collect::<Vec<_>>();
    let total = app
        .state
        .model
        .workspace
        .content
        .configuration_sets
        .configurations()
        .len();
    section_header(
        ui,
        "Configuration sets",
        (!query.is_empty())
            .then(|| format!("{} / {total}", configurations.len()))
            .as_deref(),
    );
    for (id, name) in configurations {
        // The row the catalog is bound to is the one the list marks; naming
        // every other row "available" says only that it is a row.
        let active = active_id == Some(id);
        if nav_row(ui, WorkbenchIcon::Sliders, &name, active, None) {
            app.state.project.project_page = ProjectPage::Configuration;
            if !active {
                Command::ConfigurationSets.execute(app);
            }
        }
    }
    // The inspector's "Active binding" section already states the testbench,
    // the DUT and the override count, so the rail keeps only the state the
    // inspector cannot explain.
    if app
        .state
        .model
        .workspace
        .content
        .configuration_sets
        .active()
        .is_none()
    {
        muted(ui, "No project configuration is active.");
    }
}

fn project_dependency_navigator(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = app
        .state
        .workbench
        .navigator_filter()
        .trim()
        .to_ascii_lowercase();
    let mut rows = app
        .state
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
            app.state
                .model
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
    if app.state.project.project_dependency_selection.is_none() {
        app.state.project.project_dependency_selection =
            rows.first().map(|(name, _, _, _)| name.clone());
    }
    section_header(ui, "Dependencies", Some(&rows.len().to_string()));
    for (name, _, status, icon) in &rows {
        if nav_row(
            ui,
            *icon,
            name,
            app.state.project.project_dependency_selection.as_deref() == Some(name.as_str()),
            *status,
        ) {
            app.state.project.project_dependency_selection = Some(name.clone());
        }
    }
}

fn project_recovery_navigator(ui: &mut Ui, app: &mut RSpiceApp) {
    let query = app
        .state
        .workbench
        .navigator_filter()
        .trim()
        .to_ascii_lowercase();
    let checkpoints = app
        .state
        .project
        .project_checkpoint_recovery
        .checkpoints
        .iter()
        .filter(|checkpoint| {
            query.is_empty()
                || format!(
                    "{} {} {}",
                    checkpoint.reason().label(),
                    checkpoint.project_revision(),
                    checkpoint.checkpoint_id()
                )
                .to_ascii_lowercase()
                .contains(&query)
        })
        .cloned()
        .collect::<Vec<_>>();
    if app.state.project.project_checkpoint_selection.is_none() {
        app.state.project.project_checkpoint_selection = checkpoints
            .first()
            .map(|checkpoint| checkpoint.checkpoint_id().to_string());
    }
    section_header(
        ui,
        "Project checkpoints",
        Some(&checkpoints.len().to_string()),
    );
    // Every checkpoint is restorable, so the meta that said so told the
    // reader nothing and left several "Autosave" rows indistinguishable. The
    // time is what tells them apart.
    let now_unix_ms = crate::time_compat::checked_unix_time_ms().ok();
    for checkpoint in &checkpoints {
        let id = checkpoint.checkpoint_id().to_string();
        let age = checkpoint_age(checkpoint.created_unix_ms(), now_unix_ms);
        if nav_row(
            ui,
            WorkbenchIcon::History,
            checkpoint.reason().label(),
            app.state.project.project_checkpoint_selection.as_deref() == Some(id.as_str()),
            Some(&age),
        ) {
            app.state.project.project_checkpoint_selection = Some(id);
        }
    }
}

/// How long ago a checkpoint was taken, for the rail's row meta.
///
/// A checkpoint stamped after the clock's present says the clock moved, not
/// how old the checkpoint is, so that case reports nothing.
fn checkpoint_age(created_unix_ms: u64, now_unix_ms: Option<u64>) -> String {
    let Some(elapsed) = now_unix_ms
        .filter(|_| created_unix_ms != 0)
        .and_then(|now| now.checked_sub(created_unix_ms))
    else {
        return String::new();
    };
    crate::time_compat::relative_age(std::time::Duration::from_millis(elapsed))
}

// Named `tests` rather than for the rails it covers: `source_guard` locates a
// file's production half by this exact boundary, and a module under any other
// name leaves every guard that scans this file panicking instead of scanning.
#[cfg(test)]
mod tests {
    use super::super::show;
    use super::*;
    use crate::workbench::state::Workspace;

    /// A project with two libraries, an active configuration and a recovery
    /// catalog behind it, so every Project rail has rows to show.
    fn project_app(page: ProjectPage) -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        app.state = crate::workbench::examples::hierarchy_reference::build().state;
        app.state.workbench.workspace = Workspace::Project;
        app.state.project.project_page = page;
        // A checkpoint locator names native files, so the recovery catalog
        // has rows only where those exist.
        #[cfg(not(target_arch = "wasm32"))]
        seed_recovery_catalog(&mut app.state);
        app
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn seed_recovery_catalog(state: &mut crate::workbench::app_state::AppState) {
        use crate::workbench::lifecycle::project_checkpoint::{
            ProjectCheckpointReason, fixture_summary,
        };

        let now = crate::time_compat::checked_unix_time_ms().expect("clock");
        let name = state.model.workspace.content.project.name().to_owned();
        state.project.project_checkpoint_recovery.project_id =
            Some(state.model.workspace.content.project.id().to_string());
        state.project.project_checkpoint_recovery.initialized = true;
        state.project.project_checkpoint_recovery.checkpoints = [
            (ProjectCheckpointReason::Manual, 3, 6 * 60_000, 48_214),
            (
                ProjectCheckpointReason::TechnologyAttachment,
                2,
                2 * 3_600_000,
                47_902,
            ),
            (ProjectCheckpointReason::Manual, 1, 3 * 86_400_000, 31_577),
        ]
        .into_iter()
        .map(|(reason, revision, age, bytes)| {
            fixture_summary(&name, revision, reason, now - age, bytes)
        })
        .collect();
    }

    /// Painted text of one Project rail.
    fn painted(page: ProjectPage) -> String {
        let mut app = project_app(page);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(260.0, 900.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
            },
        );
        let mut text = String::new();
        fn collect(shape: &egui::epaint::Shape, out: &mut String) {
            match shape {
                egui::epaint::Shape::Text(painted) => {
                    out.push_str(&painted.galley.job.text);
                    out.push('\n');
                }
                egui::epaint::Shape::Vec(shapes) => {
                    for shape in shapes {
                        collect(shape, out);
                    }
                }
                _ => {}
            }
        }
        for shape in &output.shapes {
            collect(&shape.shape, &mut text);
        }
        text
    }

    /// The Project rails list what the reader selects from, and nothing the
    /// inspector, the toolbar or the status bar already owns.
    #[test]
    fn the_project_rails_state_no_fact_a_panel_beside_them_owns() {
        let library = painted(ProjectPage::Library);
        assert!(library.contains("Libraries"), "{library}");
        for retired in [
            // The overview inspector's rows, the toolbar's plan, a revision
            // counter, the inspector's Location and the status bar's
            // unsaved-changes marker, each restated on the rail.
            "Active simulation context",
            "Run plan",
            "Revision",
            "Project source",
            "Working tree",
            // "3 / 3" with nothing filtered, and "rw" on every row.
            " / ",
            "rw",
        ] {
            assert!(!library.contains(retired), "{retired:?} in:\n{library}");
        }

        let configuration = painted(ProjectPage::Configuration);
        assert!(
            configuration.contains("Configuration sets"),
            "{configuration}"
        );
        for retired in ["available", "current", "Active binding", "DUT", "Overrides"] {
            assert!(
                !configuration.contains(retired),
                "{retired:?} in:\n{configuration}"
            );
        }

        let dependencies = painted(ProjectPage::Dependencies);
        assert!(dependencies.contains("Dependencies"), "{dependencies}");
        for retired in [
            "Resolved closure",
            "Closure contract",
            "Missing",
            "Source advisories",
            "Manifest",
            "writable",
            "qualified",
        ] {
            assert!(
                !dependencies.contains(retired),
                "{retired:?} in:\n{dependencies}"
            );
        }

        let recovery = painted(ProjectPage::Recovery);
        assert!(recovery.contains("Project checkpoints"), "{recovery}");
        // Rows carry the time that tells two autosaves apart, not a word that
        // is true of every checkpoint.
        assert!(recovery.contains("min ago"), "{recovery}");
        for retired in [
            "restorable",
            "Recovery store",
            "Protected",
            "Quarantined",
            "Integrity",
        ] {
            assert!(!recovery.contains(retired), "{retired:?} in:\n{recovery}");
        }
    }

    #[test]
    fn a_checkpoint_with_no_usable_stamp_reports_no_age_rather_than_a_wrong_one() {
        assert_eq!(checkpoint_age(0, Some(10_000)), "");
        assert_eq!(checkpoint_age(20_000, Some(10_000)), "");
        assert_eq!(checkpoint_age(10_000, None), "");
        assert_eq!(checkpoint_age(9_000, Some(10_000)), "just now");
        assert_eq!(checkpoint_age(10_000, Some(190_000)), "3 min ago");
        assert_eq!(checkpoint_age(0, None), "");
    }
}

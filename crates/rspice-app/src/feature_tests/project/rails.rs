//! The Project rails drawn through the whole application: each rail lists what
//! the reader selects from, and nothing a panel beside it already owns.

use crate::workbench::RSpiceApp;
use crate::workbench::docks::navigator::show;
use crate::workbench::project::workspace_state::ProjectPage;
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
    assert!(configuration.contains("Configurations"), "{configuration}");
    assert!(configuration.contains("Default"), "{configuration}");
    for retired in [
        "available",
        "current",
        "Active binding",
        "DUT",
        "Overrides",
        "Configuration sets",
        "No project configuration is active.",
    ] {
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

//! The Project rails drawn through the whole application: each rail lists what
//! the reader selects from, and nothing a panel beside it already owns.

use crate::workbench::RSpiceApp;
use egui::{Event, Modifiers};
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_state::state::{Cell, Library, View, ViewType};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::docks::navigator::show;

use super::surface::painted_text;

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
    use rspice_app_services::workbench::lifecycle::project_checkpoint::{
        ProjectCheckpointReason, fixture_summary,
    };

    let now = rspice_app_state::time_compat::checked_unix_time_ms().expect("clock");
    let name = state.model.workspace.content.project.name().to_owned();
    state.project.project_checkpoint_recovery.project_id =
        Some(state.model.workspace.content.project.id().to_string());
    state.project.project_checkpoint_recovery.initialized = true;
    state.project.project_checkpoint_recovery.checkpoints = [
        (ProjectCheckpointReason::Manual, 3, 6 * 60_000, 48_214),
        (
            ProjectCheckpointReason::TechnologyChange,
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
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
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

/// One pass of the Project rail as the shell draws it.
fn rail_frame(ctx: &egui::Context, app: &mut RSpiceApp, events: Vec<Event>) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(260.0, 900.0),
            )),
            events,
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| {
                show(ui, app);
            });
        },
    )
}

/// On the Overview, a library chosen on the rail opens the Library page on
/// it, and the rail's filter says that it filters libraries.
#[test]
fn a_library_on_the_overview_rail_opens_the_library_page_on_it() {
    let mut app = RSpiceApp::test_instance();
    let mut bench = Library::new("bench");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    bench.add_cell(cell);
    app.state.model.library_manager.add_library(bench);
    app.state.workbench.workspace = Workspace::Project;
    app.state.project.project_page = ProjectPage::Overview;
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut settled = rail_frame(&ctx, &mut app, Vec::new());
    for _ in 0..2 {
        settled = rail_frame(&ctx, &mut app, Vec::new());
    }
    let painted = painted_text(&settled);
    assert!(painted.contains("Filter libraries\u{2026}"), "{painted}");
    assert!(!painted.contains("cells, views"), "{painted}");
    let bounds = settled
        .platform_output
        .accesskit_update
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .find_map(|(_, node)| (node.label()? == "bench").then_some(node.bounds()?))
        .expect("the rail lists the library");
    let at = egui::pos2(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    );
    let _ = rail_frame(
        &ctx,
        &mut app,
        vec![
            Event::PointerMoved(at),
            Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: true,
                modifiers: Modifiers::default(),
            },
            Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed: false,
                modifiers: Modifiers::default(),
            },
        ],
    );
    assert_eq!(app.state.project.project_page, ProjectPage::Library);
    assert_eq!(
        app.state.model.library_manager.selected_library.as_deref(),
        Some("bench")
    );
}

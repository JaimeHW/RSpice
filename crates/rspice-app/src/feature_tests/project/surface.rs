//! The Project workspace drawn through the whole application: the overview
//! and the History page from live state, and a press on the page carried
//! out in the frame it was clicked in.

use egui::{Modifiers, Ui, pos2};

use crate::workbench::shell_host::project::library_publication::commit_project_library_publication;
use crate::workbench::shell_host::project::library_publication::prepare_project_library_publication;
use crate::workbench::{RSpiceApp, features};
use rspice_app_project::workbench::project::ProjectFeature;
use rspice_app_project::workbench::project::workspace_state::ProjectPage;
use rspice_app_services::workbench::lifecycle::project_lifecycle::begin_project_replacement;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::framework::command_registry::ids;
use rspice_app_workbench::workbench::framework::host::{Host, Point};

/// The Project workspace as the shell draws it on the stage.
pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    app.contribute(Point::Surface(Workspace::Project), ui);
}

pub(super) fn painted_text(output: &egui::FullOutput) -> String {
    fn collect(shape: &egui::epaint::Shape, rendered: &mut String) {
        match shape {
            egui::epaint::Shape::Text(text) => {
                rendered.push_str(&text.galley.job.text);
                rendered.push('\n');
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    collect(shape, rendered);
                }
            }
            _ => {}
        }
    }
    let mut rendered = String::new();
    for shape in &output.shapes {
        collect(&shape.shape, &mut rendered);
    }
    rendered
}

fn render_project_workspace(project_open: bool) -> String {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let mut app = RSpiceApp::test_instance();
    if !project_open {
        app.state.close_project_session();
    }
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1440.0, 900.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
        },
    );
    painted_text(&output)
}

/// Closing a project, dismissing the launcher, and a live-session mirror
/// all land here with nothing open. Every section projects an open
/// project's state, so the sections must not paint the bootstrap
/// placeholder's zeros as if they described the reader's work.
#[test]
fn a_closed_project_lands_rather_than_dressing_up_the_bootstrap_placeholder() {
    let closed = render_project_workspace(false);
    assert!(closed.contains("No project open"), "{closed}");
    for page in ProjectPage::ALL {
        assert!(
            !closed.contains(page.label()),
            "the {} tab is offered with no project behind it",
            page.label()
        );
    }

    let open = render_project_workspace(true);
    assert!(!open.contains("No project open"), "{open}");
    for page in ProjectPage::ALL {
        assert!(open.contains(page.label()), "{}", page.label());
    }
}

/// One pass of the Project workspace as the shell draws it, with the
/// AccessKit tree it published.
pub(super) fn project_frame(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1440.0, 900.0),
            )),
            events,
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| show(ui, app));
        },
    )
}

/// The Project workspace on `page`, drawn until its layout settles.
fn settled_on(page: ProjectPage) -> (egui::Context, RSpiceApp, egui::FullOutput) {
    settled(RSpiceApp::test_instance(), page)
}

/// `app`'s Project workspace on `page`, drawn until its layout settles.
pub(super) fn settled(
    mut app: RSpiceApp,
    page: ProjectPage,
) -> (egui::Context, RSpiceApp, egui::FullOutput) {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    app.state.project.project_page = page;
    let mut settled = project_frame(&ctx, &mut app, Vec::new());
    for _ in 0..2 {
        settled = project_frame(&ctx, &mut app, Vec::new());
    }
    (ctx, app, settled)
}

/// Press and release the pointer in the middle of the control the
/// settled frame announced as `label`, and return the frame that took
/// the click.
pub(super) fn click_announced(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    settled: egui::FullOutput,
    label: &str,
) -> egui::FullOutput {
    let bounds = settled
        .platform_output
        .accesskit_update
        .expect("the settled frame publishes its tree")
        .nodes
        .iter()
        .find_map(|(_, node)| (node.label()? == label).then_some(node.bounds()?))
        .unwrap_or_else(|| panic!("the settled frame announces `{label}`"));
    let at = pos2(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    );
    let click = vec![
        egui::Event::PointerMoved(at),
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: Modifiers::default(),
        },
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: Modifiers::default(),
        },
    ];
    project_frame(ctx, app, click)
}

/// The project feature reads the Configurations command's availability
/// from the snapshot its context is lent with: the dispatcher's real answer
/// for the command, so it runs with an open project and not while a project
/// operation runs or with no project open.
#[test]
fn the_configurations_command_availability_reads_through_the_snapshot() {
    fn runs(app: &mut RSpiceApp) -> bool {
        features::cx_with_foreign_answers::<ProjectFeature>(app, |_| true)
            .foreign_availability(ids::CONFIGURATION_SETS)
            .is_available()
    }
    let mut app = RSpiceApp::test_instance();
    assert!(runs(&mut app), "an open project runs it");
    begin_project_replacement(&mut app.state).expect("a project replacement starts");
    assert!(!runs(&mut app), "a project operation holds it");

    let mut app = RSpiceApp::test_instance();
    app.state.close_project_session();
    assert!(!runs(&mut app), "no open project, so it cannot run");
}

/// The detail under the Library browser shows the view a click in the
/// browser selects, in the frame that took the click: what the reader
/// asked of the browser is carried out before the detail draws. The band
/// offers to open the view selected, by its kind; its path is the
/// inspector's to state.
#[test]
fn the_library_detail_shows_the_view_the_browser_selects_in_the_frame_it_was_clicked_in() {
    use rspice_app_state::state::{Cell, Library, View, ViewType};
    let (ctx, mut app, _) = settled_on(ProjectPage::Library);
    let mut library = Library::new("frame_test");
    let mut cell = Cell::new("divider");
    cell.add_view(View::new("schematic_a", ViewType::Schematic));
    cell.add_view(View::new("symbol_b", ViewType::Symbol));
    library.add_cell(cell);
    let manager = &mut app.state.model.library_manager;
    manager.add_library(library);
    manager.select_view("frame_test", "divider", "schematic_a");
    let mut settled = project_frame(&ctx, &mut app, Vec::new());
    for _ in 0..2 {
        settled = project_frame(&ctx, &mut app, Vec::new());
    }
    let (first, second) = ("Open schematic", "Open symbol");
    let painted = painted_text(&settled);
    assert!(
        painted.contains(first) && !painted.contains(second),
        "{painted}"
    );

    let clicked = click_announced(&ctx, &mut app, settled, "symbol_b, symbol");

    assert_eq!(
        app.state.model.library_manager.selected_view.as_deref(),
        Some("symbol_b")
    );
    let painted = painted_text(&clicked);
    assert!(
        painted.contains(second) && !painted.contains(first),
        "{painted}"
    );
}

/// The label of the control `output` announces whose label starts with
/// `prefix`.
fn announced_starting(output: &egui::FullOutput, prefix: &str) -> String {
    output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .find_map(|(_, node)| node.label().filter(|label| label.starts_with(prefix)))
        .unwrap_or_else(|| panic!("the frame announces a control starting `{prefix}`"))
        .to_owned()
}

/// History… on the Overview runs its command as the page returns, inside
/// the stage's step of the frame: the History page is in front in the
/// frame it was asked for in.
#[test]
fn overview_history_opens_the_history_page_in_the_frame_it_was_clicked_in() {
    let (ctx, mut app, settled) = settled_on(ProjectPage::Overview);

    click_announced(&ctx, &mut app, settled, "History\u{2026}");

    assert_eq!(app.state.project.project_page, ProjectPage::History);
}

/// The latest run on the Overview is selected and its workspace opened as
/// the page returns, inside the stage's step of the frame that took the
/// click.
#[test]
fn overview_latest_run_opens_its_workspace_in_the_frame_it_was_clicked_in() {
    let (ctx, mut app, _) = settled_on(ProjectPage::Overview);
    app.state.model.simulation.start_run();
    let mut settled = project_frame(&ctx, &mut app, Vec::new());
    for _ in 0..2 {
        settled = project_frame(&ctx, &mut app, Vec::new());
    }
    let latest_run = announced_starting(&settled, "Latest run:");

    click_announced(&ctx, &mut app, settled, &latest_run);

    assert_eq!(app.state.workbench.workspace, Workspace::Simulate);
    assert_eq!(app.state.model.simulation.view.active_run_idx, Some(0));
}

/// Open top schematic and the schematic checks row on the Overview open
/// the project's root in Design as the page returns, inside the stage's
/// step of the frame that took the click.
#[test]
fn overview_root_actions_open_the_root_in_the_frame_they_were_clicked_in() {
    for label in ["Open top schematic", "Schematic checks:"] {
        let (ctx, mut app, settled) = settled_on(ProjectPage::Overview);
        let label = announced_starting(&settled, label);
        let project = &app.state.model.workspace.content.project;
        let root = rspice_app_state::state::CellViewRef::new(
            &project.root_library,
            &project.top_cell,
            rspice_app_state::state::workspace::DEFAULT_SCHEMATIC_VIEW,
        );

        click_announced(&ctx, &mut app, settled, &label);

        assert_eq!(app.state.workbench.workspace, Workspace::Design, "{label}");
        assert_eq!(
            app.state.model.workspace.content.active_view, root,
            "{label}"
        );
    }
}

pub(super) fn populated_project_app() -> RSpiceApp {
    use rspice_app_state::state::{
        AnalysisResult, AnalysisType, SimulationRunLifecycle, SimulationRunProvenance,
    };

    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    let _ = app
        .state
        .model
        .workspace
        .content
        .project
        .rename("lna-frontend-28g");
    let now = rspice_app_state::time_compat::unix_epoch().as_secs_f64();
    {
        let run = app.state.model.simulation.start_run();
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.success = true;
        run.timestamp = now - 3.0 * 3600.0;
        run.elapsed_time = 42.6;
        run.analyses.push(
            AnalysisResult::new(1, AnalysisType::DcOp, "op")
                .with_measurements(vec![rspice_core::MeasureResult::success("idd", 1.86e-3)]),
        );
        run.analyses
            .push(AnalysisResult::new(2, AnalysisType::Ac, "ac"));
        run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .expect("seal run");
    }
    {
        let run = app.state.model.simulation.start_run();
        run.lifecycle = SimulationRunLifecycle::Failed;
        run.success = false;
        run.timestamp = now - 40.0 * 60.0;
        run.elapsed_time = 8.3;
        run.analyses
            .push(AnalysisResult::new(3, AnalysisType::Transient, "tran"));
        run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .expect("seal run");
    }
    {
        let run = app.state.model.simulation.start_run();
        run.lifecycle = SimulationRunLifecycle::Completed;
        run.success = true;
        run.timestamp = now - 6.0 * 60.0;
        run.elapsed_time = 96.1;
        run.analyses.push(
            AnalysisResult::new(4, AnalysisType::Transient, "tran").with_measurements(vec![
                rspice_core::MeasureResult::success("gain_db", 18.4),
                rspice_core::MeasureResult::success("nf_db", 2.1),
            ]),
        );
        run.analyses
            .push(AnalysisResult::new(5, AnalysisType::Ac, "ac"));
        run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
            .expect("seal run");
    }
    let candidate = prepare_project_library_publication(
        &app.state,
        "analog-core-1.2.0",
        "james@example.test",
        "org-release-authority",
        "Qualified library handoff",
    )
    .expect("publication prepares");
    commit_project_library_publication(&mut app.state, candidate).expect("publication commits");
    {
        use rspice_design::drc::{
            DrcLocation, DrcResult, DrcSeverity, DrcViolation, DrcViolationType,
        };
        app.state.model.workspace.content.active_view = rspice_app_state::state::CellViewRef::new(
            &app.state.model.workspace.content.project.root_library,
            &app.state.model.workspace.content.project.top_cell,
            rspice_app_state::state::workspace::DEFAULT_SCHEMATIC_VIEW,
        );
        let mut result = DrcResult::new();
        result.add_violation(DrcViolation {
            severity: DrcSeverity::Warning,
            ..DrcViolation::new(
                7,
                DrcViolationType::MissingGround,
                "Net vbias1 has no DC path to ground",
                DrcLocation::Global,
            )
        });
        result.add_violation(DrcViolation {
            severity: DrcSeverity::Error,
            ..DrcViolation::new(
                8,
                DrcViolationType::UnconnectedPin,
                "R3 has an unconnected pin",
                DrcLocation::Component {
                    id: 3,
                    name: "R3".to_owned(),
                },
            )
        });
        result.completed = true;
        app.state
            .publish_active_design_check_result(result)
            .expect("publish design-check receipt");
    }
    app
}

fn netlist_first_project_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.open_test_deck(
        "Front-end bias ladder\nV1 in 0 1.8\nR1 in out 4.7k\nR2 out 0 12k\n.op\n.end\n".to_owned(),
        None,
        "front_end.sp"
    ));
    app.state.project.project_page = ProjectPage::Overview;
    app
}

fn painted_project_page(size: egui::Vec2, mut pass: impl FnMut(&mut Ui)) -> String {
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| pass(ui));
        },
    );
    painted_text(&output)
}

/// The overview names the live project, its cards, its retained operations
/// and the finding that stops a run — never placeholder copy — at every
/// supported width. A table's row is read whole from what it announces,
/// since a phone's table cuts its cells to their columns.
#[test]
fn the_overview_states_the_project_from_live_state() {
    for size in [
        egui::vec2(1440.0, 900.0),
        egui::vec2(820.0, 1180.0),
        egui::vec2(390.0, 844.0),
    ] {
        let mut app = populated_project_app();
        app.state.project.project_page = ProjectPage::Overview;
        let ctx = egui::Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
            },
        );
        let text = painted_text(&output);
        for expected in [
            "lna-frontend-28g",
            "Project status",
            "Verification",
            "Model sources",
            "Dependencies",
            "Unsaved work",
            "Cells",
            "All cells",
            "Activity",
            "All runs",
            "Open top schematic",
        ] {
            assert!(
                text.contains(expected),
                "{expected:?} missing at {}x{}:\n{text}",
                size.x,
                size.y
            );
        }
        let announced = output
            .platform_output
            .accesskit_update
            .expect("the frame publishes its tree")
            .nodes
            .into_iter()
            .filter_map(|(_, node)| node.label().or(node.value()).map(str::to_owned))
            .collect::<Vec<_>>();
        for expected in [
            "Library published · analog-core-1.2.0",
            "R3 has an unconnected pin",
        ] {
            assert!(
                announced.iter().any(|label| label.contains(expected)),
                "{expected:?} not announced at {}x{}:\n{announced:#?}",
                size.x,
                size.y
            );
        }
    }
}

/// A netlist-first project's overview speaks about the deck it continues
/// from, never about the pristine bootstrap schematic.
#[test]
fn the_netlist_first_overview_names_the_deck() {
    let mut app = netlist_first_project_app();
    let text = painted_project_page(egui::vec2(1440.0, 900.0), |ui| show(ui, &mut app));
    for expected in [
        "Open SPICE deck",
        "Source deck",
        "Checked at run",
        "front_end.sp",
    ] {
        assert!(text.contains(expected), "{expected:?} missing:\n{text}");
    }
    assert!(
        !text.contains("Open top schematic"),
        "the schematic action leaked into a deck-driven project:\n{text}"
    );
}

/// The History page over a catalog this test controls, rather than
/// whatever the machine's recovery directory happens to hold.
#[cfg(not(target_arch = "wasm32"))]
fn recovery_project_app(checkpoints: usize, set_aside: bool, error: bool) -> RSpiceApp {
    use rspice_app_services::workbench::lifecycle::project_checkpoint::{
        CheckpointDamage, ProjectCheckpointReason, fixture_quarantine, fixture_summary,
    };

    let mut app = populated_project_app();
    app.state.project.project_page = ProjectPage::History;
    let now = rspice_app_state::time_compat::checked_unix_time_ms().expect("clock");
    let name = app.state.model.workspace.content.project.name().to_owned();
    let project_id = app.state.model.workspace.content.project.id().to_string();
    let catalog = [
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
    .take(checkpoints)
    .map(|(reason, revision, age, bytes)| {
        fixture_summary(&name, revision, reason, now - age, bytes)
    })
    .collect::<Vec<_>>();
    app.state.project.project_history_selection = catalog
        .first()
        .map(|checkpoint| checkpoint.checkpoint_id().to_string());
    let recovery = &mut app.state.project.project_checkpoint_recovery;
    recovery.project_id = Some(project_id);
    recovery.initialized = true;
    recovery.checkpoints = catalog;
    if set_aside {
        recovery.quarantined = vec![
            fixture_quarantine(
                "7f0c2a9e-5b41-4d0e-9a55-2c1f6e8b3d10",
                CheckpointDamage::ChangedCopy,
                "project checkpoint bytes do not match their integrity manifest",
            )
            .named_fixture("Manual checkpoint", now - 26 * 3_600_000),
            fixture_quarantine(
                "2b9d3e61-0c7a-4f58-8e1d-5a6c9f0b4e27.checkpoint.json",
                CheckpointDamage::Record,
                "checkpoint manifest is invalid: expected value at line 1 column 1",
            ),
        ];
    }
    if error {
        recovery.error = Some(
            "project recovery directory 'D:\\designs\\amp.rspice-recovery' could not be read: \
             Access is denied. (os error 5)"
                .to_owned(),
        );
    }
    app
}

/// History lists the checkpoints newest first, each by when it was taken
/// and why, and its bar counts them against the space they are kept in.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn the_history_page_lists_checkpoints_newest_first() {
    let mut app = recovery_project_app(3, false, false);
    let made = app
        .state
        .project
        .project_checkpoint_recovery
        .checkpoints
        .iter()
        .map(|checkpoint| rspice_app_state::time_compat::local_stamp(checkpoint.created_unix_ms()))
        .collect::<Vec<_>>();
    let text = painted_project_page(egui::vec2(1440.0, 900.0), |ui| show(ui, &mut app));
    for expected in [
        "3 checkpoints \u{b7} ",
        "Create checkpoint",
        "Compare\u{2026}",
        "Restore\u{2026}",
        "Manual checkpoint",
        "Before technology change",
    ] {
        assert!(text.contains(expected), "{expected:?} missing:\n{text}");
    }
    // Two checkpoints made minutes apart are told apart by the time in the
    // row; the newest is first.
    let at = made
        .iter()
        .map(|made| {
            text.find(made.as_str())
                .unwrap_or_else(|| panic!("{made:?} missing:\n{text}"))
        })
        .collect::<Vec<_>>();
    assert!(at.is_sorted(), "{made:?} are not newest first:\n{text}");
}

/// An empty history says so once; a damaged checkpoint is a row of its own
/// that says in a few words what is wrong; and a store that cannot be read
/// says so and offers Try again instead of an empty list.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn the_history_page_states_empty_damaged_and_unreadable_catalogs() {
    let size = egui::vec2(1440.0, 900.0);
    let mut empty = recovery_project_app(0, false, false);
    let text = painted_project_page(size, |ui| show(ui, &mut empty));
    for expected in ["0 checkpoints \u{b7} ", "No checkpoints or revisions yet."] {
        assert!(text.contains(expected), "{expected:?} missing:\n{text}");
    }

    let mut damaged = recovery_project_app(2, true, false);
    let text = painted_project_page(size, |ui| show(ui, &mut damaged));
    // A damaged checkpoint whose record reads is named as its row was, by
    // its name and when it was taken; only one whose record is lost is named
    // by its file.
    let taken = rspice_app_state::time_compat::local_stamp(
        damaged
            .state
            .project
            .project_checkpoint_recovery
            .quarantined[0]
            .created_unix_ms()
            .expect("its record says when it was taken"),
    );
    for expected in [
        "2 checkpoints \u{b7} 2 damaged \u{b7} ",
        "Status",
        "Copy changed",
        "Record damaged",
        "Manual checkpoint",
        taken.as_str(),
        "2b9d3e61-0c7a-4f58-8e1d-5a6c9f0b4e27.checkpoint.json",
    ] {
        assert!(text.contains(expected), "{expected:?} missing:\n{text}");
    }
    assert!(
        !text.contains("7f0c2a9e-5b41-4d0e-9a55-2c1f6e8b3d10"),
        "a checkpoint with a name is not named by its identity:\n{text}"
    );
    assert!(!text.contains("integrity manifest"), "{text}");

    // An unreadable store says so in plain words and offers to list again,
    // what to do under the pointer; what the system said is the Console's.
    let mut unreadable = recovery_project_app(0, false, true);
    let text = painted_project_page(size, |ui| show(ui, &mut unreadable));
    for expected in ["Checkpoints unreadable", "Try again"] {
        assert!(text.contains(expected), "{expected:?} missing:\n{text}");
    }
    for system in ["Access is denied", "os error", "rspice-recovery"] {
        assert!(
            !text.contains(system),
            "{system:?} is the Console's:\n{text}"
        );
    }
    assert!(!text.contains("No checkpoints or revisions yet"), "{text}");
}

/// A store that cannot be read offers no Create checkpoint: the action is
/// disabled rather than failing once pressed.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn an_unreadable_store_cannot_take_a_checkpoint() {
    let mut app = recovery_project_app(0, false, true);
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1600.0, 900.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
        },
    );
    let create = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame announces its tree")
        .nodes
        .iter()
        .find(|(_, node)| node.label() == Some("Create checkpoint"))
        .map(|(_, node)| node.is_disabled())
        .expect("the page offers Create checkpoint");
    assert!(
        create,
        "Create checkpoint is disabled over an unreadable store"
    );
}

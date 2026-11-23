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
use rspice_app_workbench::workbench::framework::feature::Feature as _;
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

/// The project feature reads the availability of every command the
/// Overview offers from the snapshot its context is lent with: the
/// dispatcher's own answer, with a project open, while a project operation
/// runs, and with none open.
#[test]
fn the_overviews_command_availability_reads_through_the_snapshot() {
    fn agrees(app: &mut RSpiceApp, when: &str) {
        for &key in ProjectFeature::FOREIGN_AVAILABILITY {
            let dispatcher = app.command_availability(key).is_available();
            let read = features::cx_with_foreign_answers::<ProjectFeature>(app, |_| true)
                .foreign_availability(key)
                .is_available();
            assert_eq!(read, dispatcher, "{key:?} {when}");
        }
    }
    let mut app = RSpiceApp::test_instance();
    agrees(&mut app, "with a project open");
    begin_project_replacement(&mut app.state).expect("a project replacement starts");
    agrees(&mut app, "while a project operation runs");

    let mut app = RSpiceApp::test_instance();
    app.state.close_project_session();
    agrees(&mut app, "with no project open");
}

/// A click on a view in the Library browser selects it, and the next frame
/// marks that row selected and no other.
#[test]
fn a_click_in_the_library_browser_selects_that_view() {
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
    let selected = |output: &egui::FullOutput, label: &str| {
        output
            .platform_output
            .accesskit_update
            .as_ref()
            .expect("the frame publishes its tree")
            .nodes
            .iter()
            .find(|(_, node)| node.label() == Some(label))
            .and_then(|(_, node)| node.is_selected())
            == Some(true)
    };
    let (first, second) = ("schematic_a, schematic", "symbol_b, symbol");
    assert!(selected(&settled, first) && !selected(&settled, second));

    click_announced(&ctx, &mut app, settled, second);

    assert_eq!(
        app.state.model.library_manager.selected_view.as_deref(),
        Some("symbol_b")
    );
    let next = project_frame(&ctx, &mut app, Vec::new());
    assert!(selected(&next, second) && !selected(&next, first));
}

/// The latest run's value opens the newest run in Results as the page
/// returns, inside the stage's step of the frame that took the click.
#[test]
fn overview_latest_run_opens_the_run_in_the_frame_it_was_clicked_in() {
    let (ctx, mut app, settled) = settled(populated_project_app(), ProjectPage::Overview);
    let latest = settled
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the settled frame publishes its tree")
        .nodes
        .iter()
        .find_map(|(_, node)| {
            let label = node.label()?;
            (node.role() == egui::accesskit::Role::Link && label.starts_with("Run "))
                .then(|| label.to_owned())
        })
        .expect("the latest run's value is a link");

    click_announced(&ctx, &mut app, settled, &latest);

    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert_eq!(app.state.model.simulation.view.active_run_idx, Some(0));
}

/// Checks never run read "Not run", and the value opens and checks the
/// project's root in Design as the page returns, inside the stage's step of
/// the frame that took the click.
#[test]
fn overview_run_checks_opens_the_root_in_the_frame_it_was_clicked_in() {
    let (ctx, mut app, settled) = settled_on(ProjectPage::Overview);
    let project = &app.state.model.workspace.content.project;
    let root = rspice_app_state::state::CellViewRef::new(
        &project.root_library,
        &project.top_cell,
        rspice_app_state::state::workspace::DEFAULT_SCHEMATIC_VIEW,
    );

    click_announced(&ctx, &mut app, settled, "Not run");

    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(app.state.model.workspace.content.active_view, root);
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

/// The Overview states the live project as its band of facts over the
/// latest run's outputs, at every desktop width: never placeholder copy,
/// no fact twice, and no finding the Problems panel lists.
#[test]
fn the_overview_states_the_project_from_live_state() {
    for size in [egui::vec2(1440.0, 900.0), egui::vec2(1000.0, 700.0)] {
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
        // Dependencies is a page tab's name too.
        for (expected, times) in [
            ("Plan", 1),
            ("Hierarchy", 1),
            ("Devices", 1),
            ("Dependencies", 2),
            ("Checks", 1),
            ("Latest run", 1),
            ("Analyses", 1),
            ("Engine", 1),
            ("1 error \u{b7} 1 warning", 1),
            ("R3 has an unconnected pin", 0),
            ("Output", 1),
            ("Value", 1),
        ] {
            assert_eq!(
                text.lines().filter(|line| *line == expected).count(),
                times,
                "{expected:?} {times} times at {}x{}:\n{text}",
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
    for expected in ["Source deck", "front_end.sp"] {
        assert!(
            text.lines().any(|line| line.starts_with(expected)),
            "{expected:?} missing:\n{text}"
        );
    }
    for absent in ["Hierarchy", "Devices", "Checks", "Not run"] {
        assert!(
            !text.lines().any(|line| line == absent),
            "{absent:?} leaked into a deck-driven project:\n{text}"
        );
    }
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

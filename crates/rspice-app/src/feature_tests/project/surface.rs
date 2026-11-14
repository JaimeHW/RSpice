//! The Project workspace drawn through the whole application: each page at
//! the widths it supports, the overview and the recovery page from live
//! state, and a press on the page carried out in the frame it was clicked in.

use egui::{Color32, Modifiers, Rect, Ui, pos2};

use crate::ui::tokens::Tokens;
use crate::workbench::framework::host::{Host, Point};
use crate::workbench::project::workspace_state::ProjectPage;
use crate::workbench::shell_host::project::library_publication::commit_project_library_publication;
use crate::workbench::shell_host::project::library_publication::prepare_project_library_publication;
use crate::workbench::state::Workspace;
use crate::workbench::surfaces::project::recovery;
use crate::workbench::{RSpiceApp, features};

/// The Project workspace as the shell draws it on the stage.
fn show(ui: &mut Ui, app: &mut RSpiceApp) {
    app.contribute(Point::Surface(Workspace::Project), ui);
}

fn painted_text(output: &egui::FullOutput) -> String {
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
    crate::ui::Theme::default().apply(&ctx);
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
fn project_frame(
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
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let mut app = RSpiceApp::test_instance();
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
fn click_announced(
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

/// The configuration chip opens the Configuration page in the frame it
/// was clicked in: the band beside it drops the Library page's action and
/// the page below it is the Configuration page.
#[test]
fn the_configuration_chip_opens_its_page_in_the_frame_it_was_clicked_in() {
    let (ctx, mut app, settled) = settled_on(ProjectPage::Library);
    assert!(painted_text(&settled).contains("Attach library\u{2026}"));

    let clicked = click_announced(
        &ctx,
        &mut app,
        settled,
        "Configuration used for runs: Default",
    );

    assert_eq!(app.state.project.project_page, ProjectPage::Configuration);
    let painted = painted_text(&clicked);
    assert!(!painted.contains("Attach library\u{2026}"), "{painted}");
    let panels = clicked
        .platform_output
        .accesskit_update
        .expect("the clicked frame publishes its tree")
        .nodes
        .into_iter()
        .filter(|(_, node)| node.role() == egui::accesskit::Role::TabPanel)
        .filter_map(|(_, node)| node.label().map(str::to_owned))
        .collect::<Vec<_>>();
    assert_eq!(panels, ["Project Configuration"]);
}

/// The detail under the Library browser shows the view a click in the
/// browser selects, in the frame that took the click: what the reader
/// asked of the browser is carried out before the detail draws.
#[test]
fn the_library_detail_shows_the_view_the_browser_selects_in_the_frame_it_was_clicked_in() {
    use crate::state::{Cell, CellViewRef, Library, View, ViewType};
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
    let first = CellViewRef::new("frame_test", "divider", "schematic_a").display_path();
    let second = CellViewRef::new("frame_test", "divider", "symbol_b").display_path();
    let painted = painted_text(&settled);
    assert!(
        painted.contains(&first) && !painted.contains(&second),
        "{painted}"
    );

    let clicked = click_announced(&ctx, &mut app, settled, "symbol_b");

    assert_eq!(
        app.state.model.library_manager.selected_view.as_deref(),
        Some("symbol_b")
    );
    let painted = painted_text(&clicked);
    assert!(painted.contains(&second), "{painted}");
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
/// the stage's step of the frame: the revision history is open before the
/// frame draws its dialogs, so it shows in the frame it was asked for in.
#[test]
fn overview_history_opens_before_the_frame_draws_its_dialogs() {
    let (ctx, mut app, settled) = settled_on(ProjectPage::Overview);
    assert!(!app.state.project.project_revision_history.open);

    click_announced(&ctx, &mut app, settled, "History\u{2026}");

    assert!(app.state.project.project_revision_history.open);
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
        let root = crate::state::CellViewRef::new(
            &project.root_library,
            &project.top_cell,
            crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
        );

        click_announced(&ctx, &mut app, settled, &label);

        assert_eq!(app.state.workbench.workspace, Workspace::Design, "{label}");
        assert_eq!(
            app.state.model.workspace.content.active_view, root,
            "{label}"
        );
    }
}

/// Model paths… on the Dependencies page is answered as the page returns,
/// inside the stage's step of the frame: PDK settings is open before the
/// frame's dialogs draw, so it shows in the frame it was asked for in.
#[test]
fn model_paths_opens_pdk_settings_before_the_frame_draws_its_dialogs() {
    let pdk_settings = crate::workbench::framework::dialogs::DialogKey("models.pdk_settings");
    let (ctx, mut app, settled) = settled_on(ProjectPage::Dependencies);
    assert!(!features::dialog_open(&app.state, pdk_settings));

    click_announced(&ctx, &mut app, settled, "Model paths\u{2026}");

    assert!(features::dialog_open(&app.state, pdk_settings));
}

/// Revision history… on the Recovery page is answered as the page
/// returns, inside the stage's step of the frame: the dialog is open
/// before the frame draws its dialogs, so it shows in the frame it was
/// asked for in.
#[test]
fn revision_history_opens_before_the_frame_draws_its_dialogs() {
    let (ctx, mut app, settled) = settled_on(ProjectPage::Recovery);
    assert!(!app.state.project.project_revision_history.open);

    click_announced(&ctx, &mut app, settled, "Revision history\u{2026}");

    assert!(app.state.project.project_revision_history.open);
}

/// The open workspace's chrome and page share the one fill the shell
/// paints behind them: a single shape fills the whole stage with the
/// application background, and no pixel of the stage shows what lies
/// beneath it.
#[test]
fn the_chrome_and_the_page_share_one_fill_across_the_stage() {
    const BENEATH: Color32 = Color32::from_rgb(255, 0, 255);
    let size = egui::vec2(1440.0, 900.0);
    let stage = Rect::from_min_size(egui::Pos2::ZERO, size);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let bg_app = Tokens::get(&ctx).color.bg_app;
    let mut app = RSpiceApp::test_instance();
    app.state.project.project_page = ProjectPage::Library;
    let mut output = None;
    for _ in 0..3 {
        output = Some(ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(stage),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE.fill(BENEATH))
                    .show(ctx, |ui| show(ui, &mut app));
            },
        ));
    }
    fn stage_fills(shape: &egui::epaint::Shape, stage: Rect, fill: Color32, found: &mut usize) {
        match shape {
            egui::epaint::Shape::Rect(rect)
                if rect.fill == fill && rect.rect.contains_rect(stage) =>
            {
                *found += 1;
            }
            egui::epaint::Shape::Vec(shapes) => {
                for shape in shapes {
                    stage_fills(shape, stage, fill, found);
                }
            }
            _ => {}
        }
    }
    let mut fills = 0;
    for clipped in &output.expect("three passes ran").shapes {
        stage_fills(&clipped.shape, stage, bg_app, &mut fills);
    }
    assert_eq!(
        fills, 1,
        "one shape fills the stage with the application background"
    );

    let canvas = crate::ui::raster::render(size, |ui, _| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(BENEATH))
            .show(ui, |ui| show(ui, &mut app));
    });
    let sampled = canvas.pixels_in(stage).collect::<Vec<_>>();
    assert_eq!(sampled.len(), canvas.width() * canvas.height());
    assert!(
        sampled.iter().all(|pixel| *pixel != BENEATH),
        "the stage shows through the fill"
    );
    assert!(
        sampled.contains(&bg_app),
        "the fill itself is seen between the panels"
    );
}

#[test]
fn every_project_page_renders_at_desktop_tablet_and_phone_widths() {
    for page in ProjectPage::ALL {
        for size in [
            egui::vec2(1440.0, 900.0),
            egui::vec2(820.0, 1180.0),
            egui::vec2(390.0, 844.0),
        ] {
            let ctx = egui::Context::default();
            crate::ui::Theme::default().apply(&ctx);
            let mut app = RSpiceApp::test_instance();
            app.state.project.project_page = page;

            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
                },
            );

            assert!(
                !output.shapes.is_empty(),
                "{} did not render at {}x{}",
                page.label(),
                size.x,
                size.y
            );
        }
    }
}

fn populated_project_app() -> RSpiceApp {
    use crate::state::{
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
    let now = crate::time_compat::unix_epoch().as_secs_f64();
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
        app.state.model.workspace.content.active_view = crate::state::CellViewRef::new(
            &app.state.model.workspace.content.project.root_library,
            &app.state.model.workspace.content.project.top_cell,
            crate::state::workspace::DEFAULT_SCHEMATIC_VIEW,
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
        result.completed = true;
        app.state
            .publish_active_design_check_result(result)
            .expect("publish design-check receipt");
    }
    app
}

fn netlist_first_project_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    assert!(app.state.apply_imported_netlist(
        "Front-end bias ladder\nV1 in 0 1.8\nR1 in out 4.7k\nR2 out 0 12k\n.op\n.end\n".to_owned(),
        None,
        "front_end.sp"
    ));
    app.state.project.project_page = ProjectPage::Overview;
    app
}

fn painted_project_page(size: egui::Vec2, mut pass: impl FnMut(&mut Ui)) -> String {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
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

/// The overview names the live project, its cards, and its retained
/// operations — never placeholder copy — at every supported width.
#[test]
fn the_overview_states_the_project_from_live_state() {
    for size in [
        egui::vec2(1440.0, 900.0),
        egui::vec2(820.0, 1180.0),
        egui::vec2(390.0, 844.0),
    ] {
        let mut app = populated_project_app();
        app.state.project.project_page = ProjectPage::Overview;
        let text = painted_project_page(size, |ui| show(ui, &mut app));
        for expected in [
            "lna-frontend-28g",
            "Project status",
            "Execution context",
            "Design",
            "Recent operations",
            "Library published · analog-core-1.2.0",
            "Open top schematic",
            "CHK-007",
            "4 retained",
        ] {
            assert!(
                text.contains(expected),
                "{expected:?} missing at {}x{}:\n{text}",
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
        "Ownership",
        "front_end.sp",
    ] {
        assert!(text.contains(expected), "{expected:?} missing:\n{text}");
    }
    assert!(
        !text.contains("Open top schematic"),
        "the schematic action leaked into a deck-driven project:\n{text}"
    );
}

/// The Recovery page over a catalog this test controls, rather than
/// whatever the machine's recovery directory happens to hold.
#[cfg(not(target_arch = "wasm32"))]
fn recovery_project_app(checkpoints: usize, set_aside: bool, error: bool) -> RSpiceApp {
    use crate::workbench::lifecycle::project_checkpoint::{
        ProjectCheckpointReason, fixture_quarantine, fixture_summary,
    };

    let mut app = populated_project_app();
    app.state.project.project_page = ProjectPage::Recovery;
    let now = crate::time_compat::checked_unix_time_ms().expect("clock");
    let name = app.state.model.workspace.content.project.name().to_owned();
    let project_id = app.state.model.workspace.content.project.id().to_string();
    let catalog = [
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
    .take(checkpoints)
    .map(|(reason, revision, age, bytes)| {
        fixture_summary(&name, revision, reason, now - age, bytes)
    })
    .collect::<Vec<_>>();
    app.state.project.project_checkpoint_selection = catalog
        .first()
        .map(|checkpoint| checkpoint.checkpoint_id().to_string());
    let recovery = &mut app.state.project.project_checkpoint_recovery;
    recovery.project_id = Some(project_id);
    recovery.initialized = true;
    recovery.checkpoints = catalog;
    if set_aside {
        recovery.quarantined = vec![fixture_quarantine(
            "7f0c2a9e-5b41-4d0e-9a55-2c1f6e8b3d10",
            "project checkpoint bytes do not match their integrity manifest",
        )];
    }
    if error {
        recovery.error = Some(
            "project recovery directory 'D:\\designs\\.rspice-recovery' could not be read: \
             Access is denied. (os error 5)"
                .to_owned(),
        );
    }
    app
}

/// Recovery states its checkpoints, where they are kept, and its two
/// actions in plain words at every supported width, and none of the
/// retired policy ledger survives.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn the_recovery_page_lists_checkpoints_and_where_they_are_kept() {
    for size in [
        egui::vec2(1440.0, 900.0),
        egui::vec2(820.0, 1180.0),
        egui::vec2(390.0, 844.0),
    ] {
        let mut app = recovery_project_app(3, false, false);
        let text = painted_project_page(size, |ui| show(ui, &mut app));
        for expected in [
            "Recovery",
            "3 checkpoints",
            "Create checkpoint",
            "Revision history\u{2026}",
            "Checkpoints",
            "3 of 8 kept",
            "Manual checkpoint",
            "Before technology attachment",
            "6 min ago \u{b7} revision 3",
            "Compare",
            "Restore\u{2026}",
            "Storage",
            "The newest 8 checkpoints",
        ] {
            assert!(
                text.contains(expected),
                "{expected:?} missing at {}x{}:\n{text}",
                size.x,
                size.y
            );
        }
        for retired in [
            "Policy & storage",
            "SHOWN",
            "Checkpoint now",
            "Quarantined payloads",
            "governed mutations",
            "never overwritten",
            "Working state",
            // A catalog that reads back whole says nothing: neither the
            // page's state beside its title nor the Storage card reports
            // that nothing has gone wrong.
            "3 verified",
            "Integrity",
        ] {
            assert!(
                !text.contains(retired),
                "{retired:?} is still painted at {}x{}:\n{text}",
                size.x,
                size.y
            );
        }
    }
}

/// Each checkpoint row starts where the one above it ends. Its buttons
/// were laid out in a scope, which pulled the list's cursor back up to the
/// buttons' bottom edge, so every row began 12 points inside the one
/// above: its highlight and rule landed on the next row's text.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn checkpoint_rows_follow_one_another_without_overlapping() {
    for size in [
        egui::vec2(1440.0, 900.0),
        egui::vec2(820.0, 1180.0),
        egui::vec2(390.0, 844.0),
    ] {
        let mut app = recovery_project_app(3, false, false);
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        ctx.enable_accesskit();
        let mut rows = Vec::new();
        for _ in 0..3 {
            let output = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                    ..Default::default()
                },
                |ctx| {
                    egui::CentralPanel::default().show(ctx, |ui| show(ui, &mut app));
                },
            );
            rows = output
                .platform_output
                .accesskit_update
                .iter()
                .flat_map(|update| update.nodes.iter())
                .filter(|(_, node)| {
                    [node.label(), node.value()]
                        .into_iter()
                        .flatten()
                        .any(|text| text.contains(" \u{b7} revision "))
                })
                .filter_map(|(_, node)| node.bounds())
                .map(|bounds| (bounds.y0, bounds.y1))
                .collect::<Vec<_>>();
        }
        rows.sort_by(|a, b| a.0.total_cmp(&b.0));
        assert_eq!(rows.len(), 3, "{rows:?} at {}x{}", size.x, size.y);
        for pair in rows.windows(2) {
            assert!(
                (pair[1].0 - pair[0].1).abs() < 0.5,
                "rows {pair:?} overlap or part at {}x{}",
                size.x,
                size.y
            );
        }
        for (top, bottom) in &rows {
            assert!(
                bottom - top >= f64::from(recovery::CHECKPOINT_ROW_HEIGHT) - 0.5,
                "a row is {} tall at {}x{}",
                bottom - top,
                size.x,
                size.y
            );
        }
    }
}

/// An empty catalog says so once, a damaged artifact gets its own card,
/// and an unreadable directory offers a retry instead of an empty list.
#[test]
#[cfg(not(target_arch = "wasm32"))]
fn the_recovery_page_states_empty_damaged_and_unreadable_catalogs() {
    let size = egui::vec2(1440.0, 900.0);
    let mut empty = recovery_project_app(0, false, false);
    let text = painted_project_page(size, |ui| show(ui, &mut empty));
    for expected in ["no checkpoints", "No checkpoints yet"] {
        assert!(text.contains(expected), "{expected:?} missing:\n{text}");
    }
    assert!(
        !text.contains("KEPT"),
        "an empty catalog counts nothing:\n{text}"
    );
    // An empty store has nothing to verify, which it says by having no
    // Integrity row rather than by printing "Nothing to check".
    assert!(!text.contains("Integrity"), "{text}");

    let mut damaged = recovery_project_app(2, true, false);
    let text = painted_project_page(size, |ui| show(ui, &mut damaged));
    for expected in [
        "1 set aside",
        "Set aside",
        "7f0c2a9e-5b41-4d0e-9a55-2c1f6e8b3d10",
        "Project checkpoint bytes do not match their integrity manifest",
        "1 damaged, set aside",
    ] {
        assert!(text.contains(expected), "{expected:?} missing:\n{text}");
    }

    let mut unreadable = recovery_project_app(0, false, true);
    let text = painted_project_page(size, |ui| show(ui, &mut unreadable));
    for expected in [
        "unreadable",
        "Checkpoints could not be read",
        "Access is denied",
        "Try again",
    ] {
        assert!(text.contains(expected), "{expected:?} missing:\n{text}");
    }
    assert!(!text.contains("No checkpoints yet"), "{text}");
}

//! A run and what follows it, through whole application frames.

use super::veriloga_flow_tests::User;
use super::*;
use crate::workbench::session::project_transactions::create_example_project;
use rspice_app_results::workbench::documents::result_document::AnalysisPresentationKey;
use rspice_app_results::workbench::documents::result_document::ResultViewer;
use rspice_app_results::workbench::documents::result_document::waveform_plotted;
use rspice_app_simulate::workbench::simulate::commands::SimulateCommand;
use rspice_app_state::workbench::examples::EXAMPLES;
use rspice_app_state::workbench::workspace::Workspace;
use rspice_app_workbench::workbench::docks::console::CONSOLE_BODY_PADDING_TOP;
use rspice_app_workbench::workbench::state::workbench_session::ConsolePage;
use rspice_results::result_presentation::SourceWaveformPresentationKey;

/// The RC low-pass example's project, on its own short transient, the
/// session settled on it.
fn example_project() -> User {
    let mut app = RSpiceApp::test_instance();
    create_example_project(&mut app.state, EXAMPLES[0]).expect("the example opens as a project");
    User::new(app)
}

fn run_has_ended(app: &RSpiceApp, before: usize) -> bool {
    let simulation = &app.state.model.simulation;
    simulation.retained.runs.len() > before
        && !simulation.execution.has_active_execution()
        && !simulation.execution.trigger_simulation
}

fn newest_run(user: &User) -> u64 {
    user.app
        .state
        .model
        .simulation
        .retained
        .runs
        .iter()
        .map(|run| run.id)
        .max()
        .expect("a run is retained")
}

fn active_run(user: &User) -> Option<u64> {
    user.app
        .state
        .model
        .simulation
        .active_run()
        .map(|run| run.id)
}

/// Press Run as the toolbar does and frame until the run has ended.
/// The status bar's run segment, by the name it is spoken by.
fn run_segment(user: &User) -> Option<String> {
    user.named
        .iter()
        .map(|(name, _)| name)
        .find(|name| name.contains("Run 1 complete \u{b7}"))
        .cloned()
}

fn run_to_its_end(user: &mut User) {
    let before = user.app.state.model.simulation.retained.runs.len();
    assert!(user.app.command_key_enabled(SimulateCommand::Run.key()));
    user.app.execute_command_key(SimulateCommand::Run.key());
    user.wait_for("the run", |app| {
        let simulation = &app.state.model.simulation;
        simulation.retained.runs.len() > before
            && !simulation.execution.has_active_execution()
            && !simulation.execution.trigger_simulation
    });
    user.settle();
}

/// The Inspector's Open results link, clicked as a hand does, while the
/// notice of the run's end is still up: the pointer arrives, rests, presses
/// and lets go, on the frames a window delivers them.
#[test]
fn open_results_on_the_inspector_opens_that_run_in_results() {
    let mut user = example_project();
    run_to_its_end(&mut user);
    run_to_its_end(&mut user);
    let simulation = &mut user.app.state.model.simulation;
    let newest = simulation
        .retained
        .runs
        .iter()
        .map(|run| run.id)
        .max()
        .expect("the runs are retained");
    assert!(simulation.select_run_by_sequence(newest - 1));
    user.app.state.workbench.activate(Workspace::Simulate);
    user.settle();
    assert!(!user.app.state.workbench.toasts.activity().is_empty());

    let link = user.expect("Open results").center();
    user.frame(vec![egui::Event::PointerMoved(link)]);
    user.settle();
    user.press(link, egui::PointerButton::Primary);
    user.settle();

    assert_eq!(user.app.state.workbench.workspace, Workspace::Results);
    assert_eq!(
        user.app
            .state
            .model
            .simulation
            .active_run()
            .map(|run| run.id),
        Some(newest)
    );
}

/// From Results, a run leaves the reader where they are while it runs, and
/// when it completes it opens on Waves, whatever viewer Results last showed.
#[test]
fn a_finished_run_opens_in_results_on_waves_from_results() {
    let mut user = example_project();
    user.app.state.workbench.activate(Workspace::Results);
    user.app.state.results.results.session.viewer = ResultViewer::Op;
    user.settle();

    user.app.execute_command_key(SimulateCommand::Run.key());
    user.wait_for("the run", |app| {
        let ended = run_has_ended(app, 0);
        if !ended {
            assert_eq!(app.state.workbench.workspace, Workspace::Results);
        }
        ended
    });
    user.settle();

    assert_eq!(user.app.state.workbench.workspace, Workspace::Results);
    assert_eq!(
        user.app.state.results.results.session.viewer,
        ResultViewer::Waves
    );
    assert_eq!(active_run(&user), Some(newest_run(&user)));
}

/// A run pressed on the netlist or the plan plots in the dock under it, as a
/// run pressed on the drawing does: the reader stays on the page.
#[test]
fn a_run_from_the_netlist_or_the_plan_plots_in_the_dock_under_it() {
    for workspace in [Workspace::Netlist, Workspace::Simulate] {
        let mut user = example_project();
        user.app.state.workbench.activate(workspace);
        user.settle();

        run_to_its_end(&mut user);

        assert_eq!(user.app.state.workbench.workspace, workspace);
        assert!(plot_pane_shown(&user), "pressed on {workspace:?}");
        assert_eq!(active_run(&user), Some(newest_run(&user)));
    }
}

/// A reader who went elsewhere while the run ran is left there. The run's
/// end is announced in the window's corner with the way to its results, and
/// the status bar's run segment states it and opens its details.
#[test]
fn a_run_that_ends_after_the_reader_moved_on_is_announced_in_the_corner() {
    let mut user = example_project();
    user.app.execute_command_key(SimulateCommand::Run.key());
    user.frame(Vec::new());
    user.app.state.workbench.activate(Workspace::Project);
    user.wait_for("the run", |app| run_has_ended(app, 0));
    user.settle();
    assert_eq!(user.app.state.workbench.workspace, Workspace::Project);

    let segment = run_segment(&user).expect("the run segment states the outcome");
    user.click(&segment);
    assert_eq!(
        user.app
            .state
            .results
            .run_details
            .as_ref()
            .map(|details| details.run_id),
        Some(1),
        "the run segment opens Run 1's details"
    );
    user.app.state.results.run_details = None;
    user.settle();

    let offer = user
        .named
        .iter()
        .find(|(name, _)| name.starts_with("Open in Results"))
        .map(|(name, _)| name.clone())
        .expect("the corner notice offers the run's results");
    user.click(&offer);
    assert_eq!(user.app.state.workbench.workspace, Workspace::Results);
    assert_eq!(
        user.app.state.results.results.session.viewer,
        ResultViewer::Waves
    );
    assert_eq!(active_run(&user), Some(1));
}

/// Whether the dock shows the run's plot under the page: its band's Open in
/// Results is on screen.
fn plot_pane_shown(user: &User) -> bool {
    user.app.state.workbench.dock_shows(ConsolePage::Plot) && user.find("Open in Results").is_some()
}

/// Where the dock draws its Plot page: the region the dock names after the
/// page, which is taller than the page's tab.
fn plot_page(user: &User) -> egui::Rect {
    user.named
        .iter()
        .filter(|(name, _)| name == "Plot")
        .map(|(_, rect)| *rect)
        .max_by(|a, b| a.height().total_cmp(&b.height()))
        .expect("the dock shows its Plot page")
}

/// Where the dock's strip draws its tab `name`, just above the Plot page.
fn dock_tab(user: &User, name: &str) -> egui::Rect {
    let page = plot_page(user);
    user.named
        .iter()
        .find(|(candidate, rect)| {
            candidate == name
                && rect.bottom() <= page.top() + 1.0
                && rect.bottom() >= page.top() - 40.0
        })
        .map(|(_, rect)| *rect)
        .unwrap_or_else(|| panic!("the dock's strip has no {name} tab"))
}

/// A run pressed on the drawing shows its plot on the dock's Plot page
/// below it and leaves the drawing the active page: the reader never leaves
/// the schematic, as Virtuoso, ADS and KiCad plot beside it.
#[test]
fn a_run_from_the_drawing_plots_below_it_and_stays_on_the_drawing() {
    let mut user = example_project();
    user.app.state.workbench.activate(Workspace::Design);
    user.app.state.results.results.session.viewer = ResultViewer::Op;
    user.settle();
    assert!(!plot_pane_shown(&user), "no plot before any run");
    assert!(user.find("Plot").is_none(), "nor a Plot tab");

    run_to_its_end(&mut user);

    assert_eq!(user.app.state.workbench.workspace, Workspace::Design);
    assert!(
        plot_pane_shown(&user),
        "the first run opens the dock on Plot"
    );
    let page = plot_page(&user);
    let canvas = user.expect("Schematic canvas");
    assert!(
        page.top() >= canvas.bottom() - 1.0 && page.width() >= canvas.width() - 1.0,
        "the plot sits below the drawing, across its width: {page:?} under {canvas:?}"
    );
    let (plot, console) = (dock_tab(&user, "Plot"), dock_tab(&user, "Console"));
    assert!(
        plot.left() < console.left(),
        "Plot is the strip's first tab: {plot:?} before {console:?}"
    );
    assert_eq!(
        user.app.state.results.results.session.viewer,
        ResultViewer::Waves
    );
    assert_eq!(active_run(&user), Some(newest_run(&user)));
    assert!(
        !user
            .named
            .iter()
            .any(|(name, _)| name.starts_with("Open in Results:") || name.starts_with("Show plot:")),
        "nothing floats over a plot that is on screen"
    );

    user.click("Open in Results");
    assert_eq!(
        user.app.state.workbench.workspace,
        Workspace::Results,
        "the page's band opens the run in Results"
    );
    assert_eq!(active_run(&user), Some(newest_run(&user)));
    assert_eq!(
        user.app.state.results.results.session.viewer,
        ResultViewer::Waves,
        "on the sheet the page showed"
    );
}

/// The Plot page is for looking at the run's waveforms under the drawing:
/// one band of tools over the plot, no row spent on Results' sheet tabs or
/// the analysis' name, and a drawing the view showed whole stays whole
/// above it.
#[test]
fn the_plot_page_is_one_band_over_its_plot_and_the_drawing_stays_in_sight() {
    let mut user = example_project();
    user.app.state.workbench.activate(Workspace::Design);
    user.settle();
    run_to_its_end(&mut user);

    let page = plot_page(&user);
    for tab in ["Waves", "FFT", "Table", "Eye", "Manifest", "TR"] {
        assert!(
            !user
                .named
                .iter()
                .any(|(name, rect)| name == tab && page.intersects(*rect)),
            "the page spends no row on {tab}"
        );
    }
    // The band reaches back over the page's top inset, to the strip's rule.
    let band = egui::Rect::from_min_size(
        page.min - egui::vec2(0.0, CONSOLE_BODY_PADDING_TOP),
        egui::vec2(page.width(), 28.0),
    );
    for control in ["A|B", "H", "Open in Results"] {
        let rect = user.expect(control);
        assert!(
            band.contains(rect.center()),
            "{control} is on the page's band: {rect:?} in {band:?}"
        );
    }
    let trace = user.expect("V(out)");
    assert!(
        page.contains(trace.center()) && trace.top() >= band.bottom() - 1.0,
        "the plot's own row names its traces under the band: {trace:?}"
    );

    let canvas = user.expect("Schematic canvas");
    let state = &user.app.state;
    let (min_x, min_y, max_x, max_y) =
        rspice_app_design::schematic::view::schematic_symbol_context(state)
            .sheet_content_bounds(state.model.schematic.document(), |_| true)
            .expect("the example draws a circuit");
    let editor = &state.model.schematic.session.editor;
    let screen = |x: i32, y: i32| {
        egui::pos2(
            canvas.left() + (editor.pan.0 + f64::from(x) * editor.zoom) as f32,
            canvas.top() + (editor.pan.1 + f64::from(y) * editor.zoom) as f32,
        )
    };
    let drawing = egui::Rect::from_two_pos(screen(min_x, min_y), screen(max_x, max_y));
    assert!(
        canvas.contains_rect(drawing),
        "the drawing stays whole above the dock: {drawing:?} in {canvas:?}"
    );
}

/// A dock the reader collapsed stays collapsed through the next run: the
/// status bar states the run, and the window's corner offers its plot, which
/// opens the dock on Plot again.
#[test]
fn a_dock_the_reader_collapsed_stays_collapsed_and_the_corner_offers_the_plot() {
    let mut user = example_project();
    user.app.state.workbench.activate(Workspace::Design);
    run_to_its_end(&mut user);
    user.click("Collapse console");
    assert!(!plot_pane_shown(&user), "the strip's chevron collapses it");

    run_to_its_end(&mut user);

    assert_eq!(user.app.state.workbench.workspace, Workspace::Design);
    assert!(
        !user.app.state.workbench.console_visible,
        "a collapsed dock stays collapsed"
    );
    assert!(
        user.named
            .iter()
            .any(|(name, _)| name.contains("Run 2 complete")),
        "the status bar states the run"
    );
    let offer = user
        .named
        .iter()
        .find(|(name, _)| name.starts_with("Show plot:"))
        .map(|(name, _)| name.clone())
        .expect("the corner offers the run's plot");
    user.click(&offer);
    assert!(
        plot_pane_shown(&user),
        "the corner's offer opens the dock on Plot"
    );
    assert_eq!(user.app.state.workbench.workspace, Workspace::Design);
    assert_eq!(active_run(&user), Some(2));
}

/// A reader who turned the dock to another page keeps it there through the
/// next run.
#[test]
fn a_dock_the_reader_turned_to_another_page_stays_on_it() {
    let mut user = example_project();
    user.app.state.workbench.activate(Workspace::Design);
    run_to_its_end(&mut user);
    let console = dock_tab(&user, "Console");
    user.press(console.center(), egui::PointerButton::Primary);
    user.settle();
    assert_eq!(user.app.state.workbench.console_page, ConsolePage::Console);

    run_to_its_end(&mut user);

    assert!(user.app.state.workbench.console_visible);
    assert_eq!(
        user.app.state.workbench.console_page,
        ConsolePage::Console,
        "the run leaves the dock on the reader's page"
    );
    assert!(!plot_pane_shown(&user));
}

/// The dock's top edge: where egui centres its splitter.
fn dock_top(user: &User) -> f32 {
    user.expect("Resize console and diagnostics").center().y
}

/// Where the dock's strip draws its tab `name`, whatever page it shows.
fn strip_tab(user: &User, name: &str) -> egui::Rect {
    let top = dock_top(user);
    user.named
        .iter()
        .find(|(candidate, rect)| {
            candidate == name && rect.top() >= top - 1.0 && rect.top() <= top + 40.0
        })
        .map(|(_, rect)| *rect)
        .unwrap_or_else(|| panic!("the dock's strip has no {name} tab"))
}

/// A run opens the Plot page tall enough to read its plot: until the reader
/// sets the dock's height on that page, it stands at two fifths of the
/// stage, never shorter than the dock was. Once they drag the splitter, the
/// page keeps their height through the next run, and every other page keeps
/// the dock's own.
#[test]
fn a_run_opens_the_plot_page_tall_until_the_reader_sizes_it() {
    let mut user = example_project();
    user.app.state.workbench.activate(Workspace::Design);
    user.settle();
    let own = user.app.state.workbench.console_height;
    run_to_its_end(&mut user);
    let grown = dock_top(&user);

    user.press(
        strip_tab(&user, "Console").center(),
        egui::PointerButton::Primary,
    );
    user.settle();
    // The dock's foot stays put; the Console stands at the dock's own height.
    let foot = dock_top(&user) + own;
    let canvas = user.expect("Schematic canvas");
    let share = (foot - grown) / (foot - canvas.top());
    assert!(
        (share - 0.4).abs() < 0.03 && foot - grown > own + 1.0,
        "the Plot page stands at two fifths of the stage: {share} of it, from y={grown}"
    );
    user.press(
        strip_tab(&user, "Plot").center(),
        egui::PointerButton::Primary,
    );
    user.settle();
    assert!((dock_top(&user) - grown).abs() <= 1.0);

    // The reader drags the splitter up.
    let grip = user.expect("Resize console and diagnostics").center();
    let to = grip - egui::vec2(0.0, 80.0);
    user.frame(vec![egui::Event::PointerMoved(grip)]);
    user.frame(vec![egui::Event::PointerButton {
        pos: grip,
        button: egui::PointerButton::Primary,
        pressed: true,
        modifiers: egui::Modifiers::NONE,
    }]);
    user.frame(vec![egui::Event::PointerMoved(
        grip - egui::vec2(0.0, 40.0),
    )]);
    user.frame(vec![egui::Event::PointerMoved(to)]);
    user.frame(vec![egui::Event::PointerButton {
        pos: to,
        button: egui::PointerButton::Primary,
        pressed: false,
        modifiers: egui::Modifiers::NONE,
    }]);
    user.settle();
    let dragged = dock_top(&user);
    assert!(
        dragged < grown - 60.0,
        "the drag sets the page's height: its top at y={dragged}, from y={grown}"
    );

    run_to_its_end(&mut user);
    assert!(plot_pane_shown(&user));
    assert!(
        (dock_top(&user) - dragged).abs() <= 1.0,
        "the next run keeps the reader's height: its top at y={}, dragged to y={dragged}",
        dock_top(&user)
    );
    user.press(
        strip_tab(&user, "Console").center(),
        egui::PointerButton::Primary,
    );
    user.settle();
    assert!(
        (dock_top(&user) + own - foot).abs() <= 1.0,
        "the Console still keeps the dock's own height"
    );
}

/// Probing a net always shows its plot, even with the dock collapsed.
#[test]
fn probing_a_net_opens_the_plot_page() {
    let mut user = example_project();
    user.app.state.workbench.activate(Workspace::Design);
    run_to_its_end(&mut user);
    user.click("Collapse console");
    assert!(!plot_pane_shown(&user));

    probe_out_wire(&mut user);

    assert!(plot_pane_shown(&user), "the probe opens the dock on Plot");
    assert!(plotted(&user, "V(out)"));
    assert_eq!(user.app.state.workbench.workspace, Workspace::Design);
}

/// A result the design moved on from says so where the reader looks for the
/// run, never in the bar over its sheet: on Design the Plot tab carries the
/// state's glyph and its sentence, and the band leads with its mark; in
/// Results the run's row in the data browser and the Details' State row
/// carry the mark, and a press on it opens what the state says.
#[test]
fn an_out_of_date_result_is_marked_on_the_plot_tab_and_the_runs_row() {
    let marks = |user: &User| -> Vec<egui::Rect> {
        user.named
            .iter()
            .filter(|(name, _)| name.starts_with("Out of date: "))
            .map(|(_, rect)| *rect)
            .collect()
    };
    let mut user = example_project();
    user.app.state.workbench.activate(Workspace::Design);
    run_to_its_end(&mut user);
    assert!(
        !user
            .named
            .iter()
            .any(|(name, _)| name.starts_with("Plot, ")),
        "a complete result leaves the tab its name alone"
    );
    assert!(marks(&user).is_empty());

    // The plan moves on as its page adds an analysis, one it can still run.
    user.app
        .state
        .model
        .add_analysis_card_to_plan(".op")
        .expect("the plan takes an operating point");
    user.settle();

    assert!(
        user.named
            .iter()
            .any(|(name, _)| name.starts_with("Plot, Out of date \u{b7} ")),
        "the Plot tab carries the state and reads its sentence"
    );
    let band = marks(&user);
    let page = plot_page(&user);
    assert_eq!(band.len(), 1, "the band leads with the mark: {band:?}");
    assert!(page.contains(band[0].center()));

    user.click("Open in Results");
    user.app.state.workbench.navigator_visible = true;
    user.app.state.workbench.inspector_visible = true;
    user.settle();
    let viewer = user
        .named
        .iter()
        .find(|(name, _)| name.ends_with("result viewer"))
        .map(|(_, rect)| *rect)
        .expect("Results shows its sheet");
    let shown = marks(&user);
    assert_eq!(
        shown.len(),
        2,
        "the run's row and the Details' State row: {shown:?}"
    );
    assert!(
        shown
            .iter()
            .all(|mark| mark.right() <= viewer.left() || mark.left() >= viewer.right()),
        "nothing over the sheet carries it: {shown:?} beside {viewer:?}"
    );
    assert!(user.find("Out of date").is_none());
    let row = shown
        .iter()
        .min_by(|a, b| a.left().total_cmp(&b.left()))
        .copied()
        .expect("the browser's mark");
    user.press(row.center(), egui::PointerButton::Primary);
    user.settle();
    assert!(
        user.find("Out of date").is_some(),
        "a press opens what the state says"
    );

    // Its Run again runs the plan, as Run plan does.
    let before = user.app.state.model.simulation.retained.runs.len();
    user.click("Run again");
    user.wait_for("the run", |app| run_has_ended(app, before));
    user.settle();
    assert_eq!(active_run(&user), Some(newest_run(&user)));
}

/// Placing a cursor on the Plot page reads it there: the dock stays on the
/// page, and a cursor placed in Results opens no dock the reader collapsed.
#[test]
fn placing_a_cursor_keeps_the_dock_where_the_reader_left_it() {
    let mut user = example_project();
    user.app.state.workbench.activate(Workspace::Design);
    run_to_its_end(&mut user);
    if !user
        .app
        .state
        .results
        .results
        .session
        .cursor_tool
        .is_armed()
    {
        user.click("A|B");
    }
    let page = plot_page(&user);
    user.press(
        egui::pos2(page.center().x, page.center().y + 20.0),
        egui::PointerButton::Primary,
    );
    user.settle();

    assert!(
        user.app.state.results.results.session.cursors.any(),
        "the click placed a cursor"
    );
    assert_eq!(user.app.state.workbench.console_page, ConsolePage::Plot);
    assert!(plot_pane_shown(&user));

    user.click("Open in Results");
    user.app.state.workbench.console_visible = false;
    user.app.state.results.results.session.clear_cursors();
    user.settle();
    let plot = user
        .named
        .iter()
        .find(|(name, _)| name.ends_with("result viewer"))
        .map(|(_, rect)| *rect)
        .expect("Results shows its sheet");
    user.press(plot.center(), egui::PointerButton::Primary);
    user.settle();

    assert!(
        user.app.state.results.results.session.cursors.any(),
        "the click placed a cursor"
    );
    assert!(
        !user.app.state.workbench.console_visible,
        "the collapsed dock stays collapsed"
    );
}

/// A re-run updates the plot in place: the signals the reader plotted stay
/// plotted, and the zoom they set stays set.
#[test]
fn a_rerun_keeps_the_plotted_signals_and_the_zoom() {
    let mut user = example_project();
    user.app.state.workbench.activate(Workspace::Design);
    run_to_its_end(&mut user);
    let name = unplotted_signal(&user);
    let (key, _) = signal(&user, &name);
    rspice_app_results::workbench::docks::navigator::results_actions::set_result_quantity_ticked(
        &mut user.app.state.results_parts(),
        &rspice_results_ui::selection::ResultBrowserSelectionKey::Waveform(key.clone()),
        true,
    );
    user.settle();
    assert!(plotted(&user, &name));
    let session = &mut user.app.state.results.results.session;
    session
        .analysis_plot_view_pane_mut(ResultViewer::Waves, key.analysis(), 0)
        .x = Some((1e-4, 2e-4));
    let zoom = session
        .analysis_plot_view_pane(ResultViewer::Waves, key.analysis(), 0)
        .x;
    assert_eq!(zoom, Some((1e-4, 2e-4)));

    run_to_its_end(&mut user);

    assert_eq!(active_run(&user), Some(2));
    assert!(plotted(&user, &name), "{name} stays plotted on the new run");
    let (key, _) = signal(&user, &name);
    assert_eq!(
        user.app
            .state
            .results
            .results
            .session
            .analysis_plot_view_pane(ResultViewer::Waves, key.analysis(), 0)
            .x,
        zoom,
        "the zoom stays as the reader set it"
    );
    assert!(plot_pane_shown(&user));
}

/// A reader who stayed on the page they pressed Run on has the results in
/// front of them, and nothing floats over them: the run segment states the
/// outcome, and Open in Results is not offered where the results are.
#[test]
fn a_run_whose_results_are_on_screen_raises_no_notice() {
    let mut user = example_project();
    run_to_its_end(&mut user);
    assert!(plot_pane_shown(&user));
    assert!(
        !user
            .named
            .iter()
            .any(|(name, _)| name.starts_with("Open in Results:")),
        "{:?}",
        user.named.iter().map(|(name, _)| name).collect::<Vec<_>>()
    );
    let segment = run_segment(&user).expect("the run segment states the outcome");
    let elapsed = rspice_app_state::quantity::format_elapsed(
        user.app.state.model.simulation.retained.runs[0].elapsed_time,
    );
    assert!(
        segment.ends_with(&format!("Run 1 complete \u{b7} 1 warning \u{b7} {elapsed}")),
        "{segment}"
    );
    // The bar states the run once: no other segment names it again.
    assert!(
        !user
            .named
            .iter()
            .any(|(name, _)| name.contains("Run 1 \u{b7}")),
        "{:?}",
        user.named.iter().map(|(name, _)| name).collect::<Vec<_>>()
    );
    let kept = user.app.state.workbench.toasts.activity();
    assert_eq!(
        kept[0].title(),
        "Run 1 complete with 1 warning",
        "the bell keeps it"
    );
    assert_eq!(kept[0].action(), None);
}

/// A run the checks refuse says why where it can be read, and the reader
/// stays on their page.
#[test]
fn a_refused_run_shows_its_problem_and_changes_no_page() {
    let mut user = example_project();
    // Two labels the deck folds into one node: an error the checks refuse.
    let schematic = &mut user.app.state.model.schematic;
    schematic.add_net_label(
        rspice_app_state::state::Point::new(600, 600),
        "sense".to_owned(),
    );
    schematic.add_net_label(
        rspice_app_state::state::Point::new(600, 700),
        "SENSE".to_owned(),
    );
    user.app.state.sync_active_schematic_to_workspace();
    user.settle();
    let workspace = user.app.state.workbench.workspace;

    user.app.execute_command_key(SimulateCommand::Run.key());
    user.wait_for("the checks to refuse the run", |app| {
        app.state
            .runs
            .preflight
            .report
            .as_ref()
            .is_some_and(|report| !report.blockers.is_empty())
    });
    user.settle();

    assert_eq!(user.app.state.workbench.workspace, workspace);
    assert!(user.app.state.workbench.console_visible);
    assert_eq!(user.app.state.workbench.console_page, ConsolePage::Problems);
    assert!(user.app.state.model.simulation.retained.runs.is_empty());
    assert_eq!(user.app.state.workbench.run_press_route, None);
}

/// A failed run brings its own account up in the Console and leaves the
/// reader on their page, with nothing offered to open.
#[test]
fn a_failed_run_shows_its_problem_and_changes_no_page() {
    use rspice_app_state::diagnostics::{LogAnchor, LogSeverity, LogSource};
    use rspice_app_state::state::SimulationRunLifecycle as Lifecycle;

    let mut user = example_project();
    let workspace = user.app.state.workbench.workspace;
    user.app.state.workbench.run_press_route = Some(user.app.state.workbench.current_route());
    let run = user.app.state.model.simulation.start_run();
    run.lifecycle = Lifecycle::Running;
    user.frame(Vec::new());
    let run = &mut user.app.state.model.simulation.retained.runs[0];
    run.success = false;
    run.finish_lifecycle(Lifecycle::Failed)
        .expect("a running run can fail");
    let line = user.app.state.model.log_buffer.log_anchored(
        LogSeverity::Error,
        LogSource::Run,
        "Run 1 failed: timestep too small",
        Some(LogAnchor::ResultRun { run_sequence: 1 }),
    );
    user.settle();

    assert_eq!(user.app.state.workbench.workspace, workspace);
    assert!(user.app.state.workbench.console_visible);
    assert_eq!(user.app.state.workbench.console_page, ConsolePage::Console);
    assert!(
        user.app
            .state
            .workbench
            .console_view
            .selection
            .is_some_and(|selection| selection.contains(line)),
        "the Console opens on the line that says what happened"
    );
    assert!(
        !user
            .named
            .iter()
            .any(|(name, _)| name.starts_with("Open in Results")),
        "nothing is offered to open"
    );
    assert!(!plot_pane_shown(&user), "a failed run opens no plot");
}

/// A run with no curve to draw opens where its result can be read.
#[test]
fn an_operating_point_run_opens_on_its_operating_point() {
    let mut user = example_project();
    let plan = user
        .app
        .state
        .model
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("a plan");
    let ids = plan
        .instances()
        .iter()
        .map(|instance| instance.id())
        .collect::<Vec<_>>();
    for id in ids {
        plan.set_enabled(id, false).expect("the analysis exists");
    }
    plan.insert(rspice_app_state::simulation::plan::AnalysisKind::OperatingPoint)
        .expect("an operating point can be planned");
    user.app
        .state
        .model
        .sim_setup
        .refresh_legacy_analysis_projections();
    user.settle();

    run_to_its_end(&mut user);

    assert_eq!(user.app.state.workbench.workspace, Workspace::Design);
    assert!(plot_pane_shown(&user));
    assert_eq!(
        user.app.state.results.results.session.viewer,
        ResultViewer::Op
    );
}

/// Every example's Run opens Results on a plot of the circuit at work: a
/// plotted trace that moves across the time or the sweep, not a flat line at
/// its bias. A swing of a millivolt and a hundredth of its level is one the
/// plot's own scale shows.
#[test]
fn every_example_opens_on_a_plot_whose_trace_moves() {
    for example in EXAMPLES {
        let mut app = RSpiceApp::test_instance();
        create_example_project(&mut app.state, example).expect("the example opens as a project");
        let mut user = User::new(app);
        run_to_its_end(&mut user);

        let viewer = user.app.state.results.results.session.viewer;
        assert!(
            rspice_results::result_presentation::viewer_uses_wave_stack(viewer),
            "{} opens on {viewer:?}",
            example.name
        );
        let run = user
            .app
            .state
            .model
            .simulation
            .active_run()
            .expect("a run is open");
        let mut plotted = Vec::new();
        for analysis in &run.analyses {
            for waveform in &analysis.waveforms {
                let key = SourceWaveformPresentationKey::new(
                    AnalysisPresentationKey::new(run.dataset_id, analysis),
                    &waveform.name,
                );
                if waveform_plotted(
                    &user.app.state.model,
                    &user.app.state.results.results,
                    &key,
                    &waveform.name,
                    waveform.visible,
                ) {
                    let samples = waveform.y.read_whole().expect("the samples read");
                    let (low, high) = samples
                        .iter()
                        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), &y| {
                            (low.min(y), high.max(y))
                        });
                    plotted.push((waveform.name.clone(), samples.len(), low, high));
                }
            }
        }
        assert!(
            plotted.iter().any(|&(_, points, low, high)| {
                points > 2 && high - low > 1e-3 && high - low > 0.01 * low.abs().max(high.abs())
            }),
            "{}: a plotted trace moves: {plotted:?}",
            example.name
        );
    }
}

/// The Results page on Waves after a run, opened from the plot pane.
fn results_after_a_run() -> User {
    let mut user = example_project();
    run_to_its_end(&mut user);
    user.app.state.workbench.activate(Workspace::Results);
    user.app.state.workbench.navigator_visible = true;
    user.app.state.workbench.inspector_visible = true;
    user.settle();
    user
}

/// The presentation key of the active run's signal `name`, and whether the
/// run drew it by default.
fn signal(user: &User, name: &str) -> (SourceWaveformPresentationKey, bool) {
    let run = user
        .app
        .state
        .model
        .simulation
        .active_run()
        .expect("a run is open");
    run.analyses
        .iter()
        .find_map(|analysis| {
            let waveform = analysis
                .waveforms
                .iter()
                .find(|waveform| waveform.name == name)?;
            Some((
                SourceWaveformPresentationKey::new(
                    AnalysisPresentationKey::new(run.dataset_id, analysis),
                    name,
                ),
                waveform.visible,
            ))
        })
        .expect("the run holds the signal")
}

/// Whether the active run's signal `name` is on the plot.
fn plotted(user: &User, name: &str) -> bool {
    let (key, default) = signal(user, name);
    waveform_plotted(
        &user.app.state.model,
        &user.app.state.results.results,
        &key,
        name,
        default,
    )
}

/// A signal of the active run that is not on the plot.
fn unplotted_signal(user: &User) -> String {
    let run = user
        .app
        .state
        .model
        .simulation
        .active_run()
        .expect("a run is open");
    let names = run
        .analyses
        .iter()
        .flat_map(|analysis| {
            analysis
                .waveforms
                .iter()
                .map(|waveform| waveform.name.clone())
        })
        .collect::<Vec<_>>();
    names
        .into_iter()
        .find(|name| !plotted(user, name))
        .expect("a signal is off the plot")
}

fn selected_signal(user: &User) -> Option<String> {
    user.app
        .state
        .results
        .results
        .valid_selected_trace(&user.app.state.model.simulation)
        .map(|selected| selected.source_name().to_owned())
}

/// A tick plots its signal at once and unticking removes it, and the row's
/// tick, the strip's legend and the Inspector say the same thing throughout.
#[test]
fn ticking_a_signal_plots_it_and_unticking_removes_it() {
    let mut user = results_after_a_run();
    let name = unplotted_signal(&user);
    let tick = format!("Plot {name}");
    let chip = format!("Toggle {name} visibility");
    assert_eq!(user.ticked(&tick), Some(false));

    user.click(&tick);
    assert!(plotted(&user, &name), "a tick plots at once");
    assert_eq!(user.ticked(&tick), Some(true));
    assert_eq!(user.ticked(&chip), Some(true), "the legend shows it");
    assert_eq!(selected_signal(&user), None, "a tick selects nothing");

    user.click(&tick);
    assert!(!plotted(&user, &name), "unticking removes it");
    assert_eq!(user.ticked(&tick), Some(false));
    assert_ne!(user.ticked(&chip), Some(true), "and the legend lets it go");
}

/// A click on a row selects it for the Inspector and plots nothing; a double
/// click plots it and shows its statistics. The Inspector's button toggles
/// the same membership the tick shows.
#[test]
fn a_click_selects_a_signal_and_a_double_click_plots_and_inspects_it() {
    let mut user = results_after_a_run();
    let name = unplotted_signal(&user);
    let tick = format!("Plot {name}");
    // On the row's name, just right of its tick: the Inspector's list of
    // saved outputs prints the same name.
    let tick_rect = user.expect(&tick);
    let row = egui::pos2(tick_rect.right() + 24.0, tick_rect.center().y);

    user.press(row, egui::PointerButton::Primary);
    user.settle();
    assert_eq!(selected_signal(&user).as_deref(), Some(name.as_str()));
    assert!(!plotted(&user, &name), "a click plots nothing");
    assert!(user.find("Show trace").is_some());

    // Past the double-click interval, so the next two presses are their own.
    for _ in 0..30 {
        user.frame(Vec::new());
    }
    user.press(row, egui::PointerButton::Primary);
    user.press(row, egui::PointerButton::Primary);
    user.settle();
    assert!(plotted(&user, &name), "a double click plots it");
    assert_eq!(selected_signal(&user).as_deref(), Some(name.as_str()));
    assert!(user.app.state.workbench.inspector_visible);
    // A stored trace's statistics are read off the frame and arrive a few frames later.
    let measured = (0..400).any(|_| {
        user.frame(Vec::new());
        let computing = user.find("Computing\u{2026}").is_some();
        if computing {
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        !computing
    });
    assert!(
        measured && user.find("Average").is_some(),
        "with its statistics"
    );
    assert_eq!(user.ticked(&tick), Some(true));

    user.click("Hide trace");
    assert!(
        !plotted(&user, &name),
        "the Inspector's button takes it off"
    );
    assert_eq!(user.ticked(&tick), Some(false), "and the tick follows");
    user.click("Show trace");
    assert!(plotted(&user, &name), "and puts it back");
    assert_eq!(user.ticked(&tick), Some(true));
}

/// The example's wire that carries `out`, and the point its label names.
fn out_wire(user: &User) -> (u64, rspice_app_state::state::Point) {
    let document = user.app.state.model.schematic.document();
    let at = document
        .net_labels
        .iter()
        .find(|label| label.name == "out")
        .expect("the example names its output")
        .pos;
    let wire = document
        .wires
        .iter()
        .find(|wire| wire.contains_point(at))
        .expect("a wire carries the label")
        .id;
    (wire, at)
}

/// The Probe row of the menu opened on the wire that carries `out`.
fn probe_out_wire(user: &mut User) {
    use rspice_app_design::schematic::view::context_menu::execute_context_action;
    use rspice_app_design::schematic::view::schematic_symbol_context;
    use rspice_app_design::workbench::app_state::interaction_state::ContextTarget;
    use rspice_schematic_editor::view::context_menu::ContextAction;

    let (wire, at) = out_wire(user);
    let state = &mut user.app.state;
    state.design.interaction.context_target = Some((ContextTarget::Wire(wire), (at.x, at.y)));
    let symbol_context = schematic_symbol_context(state);
    execute_context_action(
        ContextAction::Probe,
        &mut state.canvas_parts(),
        at,
        &symbol_context,
    );
    user.settle();
}

/// A probe of a net the last run holds plots that run's trace at once,
/// through the tick the Data Browser uses, so the row, the legend and the
/// strip all show it, and selects it. A second probe leaves it plotted.
#[test]
fn probing_a_net_the_run_holds_plots_its_trace() {
    use rspice_app_results::workbench::docks::navigator::results_actions::set_result_quantity_ticked;
    use rspice_results_ui::selection::ResultBrowserSelectionKey;

    let mut user = results_after_a_run();
    let name = "V(out)";
    let (key, _) = signal(&user, name);
    set_result_quantity_ticked(
        &mut user.app.state.results_parts(),
        &ResultBrowserSelectionKey::Waveform(key),
        false,
    );
    user.settle();
    assert!(!plotted(&user, name));

    probe_out_wire(&mut user);
    assert!(plotted(&user, name), "the probe plots the run's trace");
    assert_eq!(user.ticked(&format!("Plot {name}")), Some(true));
    assert_eq!(selected_signal(&user).as_deref(), Some(name));
    assert_eq!(
        user.app.state.prompt_text(),
        Some("V(out) added to the plot.")
    );

    probe_out_wire(&mut user);
    assert!(plotted(&user, name), "a second probe leaves it plotted");
    assert_eq!(
        user.app.state.prompt_text(),
        Some("V(out) is already in the plot.")
    );
}

/// The net inspector's Plot button says whether the net's trace is on the
/// plot, and a press takes it off or puts it back.
#[test]
fn the_net_inspector_plot_button_reads_and_toggles_the_plot() {
    let mut user = results_after_a_run();
    let name = "V(out)";
    assert!(plotted(&user, name), "a first run plots the named net");
    let (wire, _) = out_wire(&user);
    user.app.state.workbench.activate(Workspace::Design);
    user.app
        .state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_wire_segment(wire, 0);
    user.settle();

    user.click("Plotted: V(out)");
    assert!(!plotted(&user, name), "a press takes it off the plot");
    user.click("Plot V(out)");
    assert!(plotted(&user, name), "and a second puts it back");
    assert!(user.find("Plotted: V(out)").is_some());
}

/// A source and a 1 kΩ load to ground on a new project's top sheet: `kind`
/// with `params` across the load.
fn new_project_driven_by(kind: rspice_app_state::state::ComponentType, params: &str) -> User {
    use rspice_app_state::state::{ComponentType, Point};
    let (mut user, _) = super::veriloga_flow_tests::new_project();
    let plus = Point::new(200, 200);
    let source = user.place(kind, plus);
    if !params.is_empty() {
        let placed = user.component(source);
        let mut set = placed.clone();
        params.clone_into(&mut set.params);
        assert_eq!(
            user.app
                .state
                .model
                .edit_component_transaction(&placed, set, "edit source"),
            Ok(true)
        );
    }
    let minus = user.second_terminal(source);
    user.wire(&[minus, minus + Point::new(0, 20)]);
    user.place(ComponentType::Ground, minus + Point::new(0, 20));
    let out = plus + Point::new(100, 0);
    user.wire(&[plus, out]);
    let load = user.place(ComponentType::Resistor, out);
    let load_minus = user.second_terminal(load);
    user.wire(&[load_minus, load_minus + Point::new(0, 20)]);
    user.place(ComponentType::Ground, load_minus + Point::new(0, 20));
    user.settle();
    user
}

/// The enabled analyses of the active plan, as drafts.
fn planned(user: &User) -> Vec<rspice_simulation_contract::analysis_draft::AnalysisDraft> {
    user.app
        .state
        .model
        .sim_setup
        .enabled_analysis_instances()
        .map(|instance| instance.draft().clone())
        .collect()
}

/// The first Run of a new project's plan runs what its drawing asks for,
/// once: a 1 kHz sine gives a transient over five of its periods, which the
/// plan keeps and the Console states, and the next Run runs the plan as it
/// stands.
#[test]
fn a_new_projects_first_run_shows_five_periods_of_its_sine() {
    use rspice_simulation_contract::analysis_draft::AnalysisDraft;
    let mut user = new_project_driven_by(
        rspice_app_state::state::ComponentType::VoltageSourceSin,
        "freq=1k",
    );
    let untouched = |user: &User| {
        user.app
            .state
            .model
            .sim_setup
            .stable_analysis_plan()
            .is_ok_and(|plan| plan.is_untouched())
    };
    assert!(untouched(&user), "drawing edits no plan");

    run_to_its_end(&mut user);
    let analyses = planned(&user);
    let [AnalysisDraft::Transient(setup)] = analyses.as_slice() else {
        panic!("one transient: {analyses:?}");
    };
    assert_eq!(setup.stop, "5m");
    assert!(!untouched(&user));
    assert!(
        user.app.state.model.log_buffer.entries().any(|entry| entry
            .message
            .contains("Run chose Transient, 5 ms: five periods of V1")),
        "the choice is stated"
    );
    let run = &user.app.state.model.simulation.retained.runs[0];
    let end = run
        .analyses
        .iter()
        .flat_map(|analysis| &analysis.waveforms)
        .find_map(|wave| wave.x.samples(..).last())
        .expect("the run holds a waveform");
    assert!((end - 5e-3).abs() < 1e-6, "the run ends at {end} s");

    let revision = user
        .app
        .state
        .model
        .sim_setup
        .stable_analysis_plan()
        .map(|plan| plan.revision())
        .unwrap();
    run_to_its_end(&mut user);
    assert_eq!(
        user.app
            .state
            .model
            .sim_setup
            .stable_analysis_plan()
            .map(|plan| plan.revision())
            .unwrap(),
        revision,
        "a second Run leaves the plan as it stands"
    );
}

/// A drawing with only a DC source settles to an operating point, and one
/// whose source carries only an AC magnitude sweeps it.
#[test]
fn a_new_projects_first_run_settles_a_dc_drawing_and_sweeps_an_ac_one() {
    use rspice_app_state::state::ComponentType;
    use rspice_simulation_contract::analysis_draft::AnalysisDraft;
    let mut user = new_project_driven_by(ComponentType::VoltageSource, "");
    run_to_its_end(&mut user);
    assert!(
        matches!(
            planned(&user).as_slice(),
            [AnalysisDraft::OperatingPoint(_)]
        ),
        "{:?}",
        planned(&user)
    );

    // The sweep linearizes about an operating point the plan runs first.
    let mut user = new_project_driven_by(ComponentType::VoltageSourceAc, "");
    run_to_its_end(&mut user);
    assert!(
        matches!(
            planned(&user).as_slice(),
            [AnalysisDraft::OperatingPoint(_), AnalysisDraft::Ac(_)]
        ),
        "{:?}",
        planned(&user)
    );
}

/// The run chip names the analysis Run runs and changes it where the reader
/// is: a press opens its popover, AC takes the transient's place, and a stop
/// frequency typed into its row is the plan's.
#[test]
fn the_run_chip_names_the_analysis_and_changes_it_in_place() {
    use rspice_simulation_contract::analysis_draft::AnalysisDraft;
    let mut user = User::new(RSpiceApp::test_instance());
    user.settle();
    let chip = |user: &User| {
        user.named
            .iter()
            .find(|(name, _)| name.starts_with("Analysis "))
            .map(|(name, rect)| (name.clone(), *rect))
            .expect("the toolbar draws the run chip")
    };
    let (name, at) = chip(&user);
    assert!(
        name.starts_with("Analysis Transient \u{b7} 1 ms, plan "),
        "{name}"
    );

    user.press(at.center(), egui::PointerButton::Primary);
    user.settle();
    user.click("AC");
    assert!(
        matches!(
            planned(&user).as_slice(),
            [AnalysisDraft::OperatingPoint(_), AnalysisDraft::Ac(_)]
        ),
        "the sweep brings the operating point it linearizes about: {:?}",
        planned(&user)
    );
    assert!(
        chip(&user)
            .0
            .starts_with("Analysis AC \u{b7} 1 Hz to 1 GHz, plan "),
        "{}",
        chip(&user).0
    );

    user.focus_on("Stop, Hz");
    user.frame(vec![
        egui::Event::Key {
            key: egui::Key::A,
            physical_key: Some(egui::Key::A),
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::COMMAND,
        },
        egui::Event::Text("10Meg".to_owned()),
    ]);
    user.frame(vec![egui::Event::Key {
        key: egui::Key::Enter,
        physical_key: Some(egui::Key::Enter),
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::NONE,
    }]);
    user.settle();
    let analyses = planned(&user);
    let [AnalysisDraft::OperatingPoint(_), AnalysisDraft::Ac(setup)] = analyses.as_slice() else {
        panic!("one AC sweep: {analyses:?}");
    };
    assert_eq!(setup.fstop, "10Meg");
    assert!(
        chip(&user)
            .0
            .starts_with("Analysis AC \u{b7} 1 Hz to 10 MHz, plan "),
        "{}",
        chip(&user).0
    );

    // Back to a transient: the sweep's operating point goes with it.
    let (_, at) = chip(&user);
    if user.find("Transient").is_none() {
        user.press(at.center(), egui::PointerButton::Primary);
        user.settle();
    }
    user.click("Transient");
    assert!(
        matches!(planned(&user).as_slice(), [AnalysisDraft::Transient(_)]),
        "{:?}",
        planned(&user)
    );
}

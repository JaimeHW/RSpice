//! A result signal reaches its place on the schematic in one action, and a
//! probe on the schematic reaches the plot: a pin current selects its pin, a
//! noise contributor its part, a signal read two levels down opens that
//! occurrence, and a probed net is on the plan's working view.

use super::inspector::{cross_probe, result_app_with_current_out_map, status_line};
use crate::workbench::RSpiceApp;
use rspice_app_design::schematic::view::gesture_commits::click_probe;
use rspice_app_design::workbench::design::navigator::hierarchy_tree::open_occurrence;
use rspice_app_design::workbench::design::navigator::tests::{add_master, placed};
use rspice_app_results::workbench::commands::schematic_cross_probe::{
    show_noise_contributor_on_schematic, show_signal_on_schematic,
};
use rspice_app_state::state::{
    CellViewRef, Component, ComponentType, InstancePath, Point, SchematicState, Wire,
};
use rspice_app_state::workbench::workspace::Workspace;
use rspice_simulation::netlist_gen::{MasterCrossProbe, SheetNetGeometry};
use std::collections::HashMap;

/// One net drawn by one wire from `a` to `b`, as a run's map keeps it.
fn one_net(
    name: &str,
    a: Point,
    b: Point,
) -> HashMap<Option<rspice_app_state::state::SheetId>, SheetNetGeometry> {
    HashMap::from([(
        None,
        SheetNetGeometry {
            point_to_net: HashMap::from([(a, name.to_owned()), (b, name.to_owned())]),
            net_to_points: HashMap::from([(name.to_owned(), vec![a, b])]),
            net_segments: HashMap::new(),
        },
    )])
}

/// `I(M1:d)` is the current into M1's drain: cross-probing it selects M1,
/// centres the canvas on the drain pin and lights the conductor the drain
/// meets, and the status line names the pin.
#[test]
fn a_pin_current_selects_the_pin_of_its_part() {
    let mut app = result_app_with_current_out_map(false);
    // The drain of a MOS drawn at (20, 40) stands at (40, 0), on OUT's wire.
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(
            Component::new(8, ComponentType::Nmos, Point::new(20, 40))
                .with_name_value("M1", "nmos"),
        );

    cross_probe(&mut app, "I(M1:d)").expect("M1 is drawn");

    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(
        status_line(&app).as_deref(),
        Some("Selected pin D of M1 from I(M1:d).")
    );
    let editor = &app.state.model.schematic.session.editor;
    assert!(editor.selection.has_component(8), "M1 is selected");
    assert!(
        editor.selection.wires.is_empty(),
        "the pin's part, not its wire"
    );
    assert_eq!(
        editor.center_request,
        Some(Point::new(40, 0)),
        "on the drain"
    );
    assert!(
        editor.net_highlight.is_wire_highlighted(91),
        "the conductor the drain meets is lit"
    );
}

/// A noise contributor's row shows the part that makes the noise as a
/// current probe of it does: selected and centred, on Design.
#[test]
fn a_noise_contributor_selects_the_part_that_makes_the_noise() {
    let mut app = result_app_with_current_out_map(false);
    let place = Point::new(200, 0);
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .push(Component::new(7, ComponentType::Resistor, place).with_name_value("R1", "1k"));

    show_noise_contributor_on_schematic(&mut app.state.results_parts(), "R1").expect("R1 is drawn");
    app.after_contribution();

    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(
        status_line(&app).as_deref(),
        Some("Selected R1 from the noise of R1.")
    );
    let editor = &app.state.model.schematic.session.editor;
    assert!(editor.selection.has_component(7), "R1 is selected");
    assert_eq!(editor.center_request, Some(place), "and centred");
}

/// A design two levels deep: the root places `XA` of `work/child`, which
/// places `XB` of `work/leaf`, whose sheet draws the net N1 as wire 301. The
/// root draws OUT as wire 91. The run's map keeps both, each read off its
/// own drawing as it stands.
fn two_level_design() -> RSpiceApp {
    let (a, b) = (Point::new(200, 200), Point::new(240, 200));
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Results);
    let root = app.state.model.workspace.content.active_view.clone();
    let drawing = app.state.model.schematic.document_mut_for_test();
    drawing.components.push(placed(101, "XA", "work", "child"));
    drawing.wires.push(Wire::new(91, vec![a, b]));
    app.state.model.schematic.add_net_label(a, "OUT".to_owned());
    app.state.sync_active_schematic_to_workspace();
    let mut child = SchematicState::default();
    child
        .document_mut_for_test()
        .components
        .push(placed(201, "XB", "work", "leaf"));
    add_master(&mut app.state.model, "work", "child", child);
    let mut leaf = SchematicState::default();
    leaf.document_mut_for_test()
        .wires
        .push(Wire::new(301, vec![a, b]));
    leaf.add_net_label(a, "N1".to_owned());
    add_master(&mut app.state.model, "work", "leaf", leaf);

    // The leaf's drawing as the run read it: opened once at its occurrence.
    let nested = InstancePath::parse("/XA/XB").expect("a drawn occurrence");
    land_on(&mut app, &nested);
    let leaf_topology = app.state.model.schematic.topology_version();
    app.state.navigation().focus_workspace_breadcrumb(0);
    assert!(
        app.state
            .model
            .workspace
            .content
            .occurrence_path()
            .is_root()
    );

    let root_topology = app.state.model.schematic.topology_version();
    app.state.model.simulation.source.cross_probe.install(
        root,
        one_net("OUT", a, b),
        root_topology,
        vec![MasterCrossProbe {
            reference: CellViewRef::new("work", "leaf", "schematic"),
            sheets: one_net("N1", a, b),
            topology_version: leaf_topology,
        }],
    );
    app
}

fn land_on(app: &mut RSpiceApp, occurrence: &InstancePath) {
    open_occurrence(
        &mut app.state.design,
        &mut app.state.model,
        &mut app.state.workbench,
        &app.state.runs,
        occurrence,
    );
}

/// Show `signal` on the schematic as a data-browser row's "Show on
/// schematic" does, then answer what that raised, as the shell does.
fn show(app: &mut RSpiceApp, signal: &str) {
    show_signal_on_schematic(&mut app.state.results_parts(), signal).expect("it locates");
    app.after_contribution();
}

/// A net read two levels down opens the occurrence that reads it and selects
/// its conductor there, instead of telling the reader where to go; one read
/// at the root from inside a block goes back up to it.
#[test]
fn a_net_two_levels_down_opens_that_occurrence_and_selects_it() {
    let mut app = two_level_design();

    show(&mut app, "V(xa.xb.n1)");

    assert_eq!(
        app.state.model.workspace.content.occurrence_path(),
        InstancePath::parse("/XA/XB").expect("a drawn occurrence")
    );
    assert_eq!(
        app.state.model.workspace.content.active_view,
        CellViewRef::new("work", "leaf", "schematic")
    );
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(
        status_line(&app).as_deref(),
        Some("Selected conductor N1 from V(xa.xb.n1).")
    );
    let editor = &app.state.model.schematic.session.editor;
    assert!(editor.selection.wires.contains(&301));
    assert!(editor.net_highlight.is_wire_highlighted(301));

    show(&mut app, "V(out)");

    assert!(
        app.state
            .model
            .workspace
            .content
            .occurrence_path()
            .is_root()
    );
    assert_eq!(
        status_line(&app).as_deref(),
        Some("Selected conductor OUT from V(out).")
    );
    assert!(
        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .wires
            .contains(&91)
    );
}

/// A probe click on a net the last run holds puts its signal on the plan's
/// working view, which every strip of the run draws from.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_schematic_probe_puts_its_signal_on_the_working_view() {
    use super::working_view::{quick_frame, run_plan_with, show_newest, view_plots};
    use rspice_app_runs::simulation::controller::test_execution::state_with_current_clean_drc;
    use rspice_app_state::state::RetainedTrace;

    let host = state_with_current_clean_drc();
    let mut app = RSpiceApp::test_instance();
    app.state.model = host.model;
    app.state.runs = host.runs;
    app.state.workbench.activate(Workspace::Results);
    let time = vec![0.0, 0.5, 1.0];
    let mut probed = RetainedTrace::new("V(probed)", time.clone(), vec![0.0, 0.7, 0.2], "#ffffff");
    probed.visible = false;
    run_plan_with(
        &mut app.state,
        vec![
            RetainedTrace::new("V(in)", time.clone(), vec![1.0; 3], "#ffffff"),
            probed,
        ],
    );
    show_newest(&mut app.state);
    quick_frame(&mut app.state);
    assert!(
        !view_plots(&app.state).contains(&("V(probed)".to_owned(), true)),
        "the run does not plot the net before it is probed"
    );

    // The net PROBED, drawn away from everything else on the sheet.
    let (a, b) = (Point::new(2000, 2000), Point::new(2040, 2000));
    app.state.model.schematic.add_wire(vec![a, b]);
    app.state
        .model
        .schematic
        .add_net_label(a, "PROBED".to_owned());
    app.state.sync_active_schematic_to_workspace();
    app.state.workbench.activate(Workspace::Design);

    click_probe(&mut app.state.canvas_frame(), Point::new(2020, 2000));
    app.after_contribution();
    quick_frame(&mut app.state);

    assert!(
        view_plots(&app.state).contains(&("V(probed)".to_owned(), true)),
        "the probed net is on the view: {:?}",
        view_plots(&app.state)
    );
}

/// A drag with the probe tool from one net to another puts the voltage
/// between them on the plan's working view, and the strip reads the two
/// nets' difference at every sample. The run holds both nets, so the drawing
/// keeps no marker.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_two_net_probe_plots_the_voltage_between_the_nets() {
    use super::working_view::{quick_frame, run_plan_with, show_newest, view_expressions};
    use rspice_app_design::schematic::view::gesture_commits::drag_probe;
    use rspice_app_results::analysis::calculator::{CalcValue, RealValue, WaveformsContext};
    use rspice_app_runs::simulation::controller::test_execution::state_with_current_clean_drc;
    use rspice_app_state::state::RetainedTrace;
    use rspice_results::calculator::{evaluator::evaluate, spice_parser::try_parse};

    let host = state_with_current_clean_drc();
    let mut app = RSpiceApp::test_instance();
    app.state.model = host.model;
    app.state.runs = host.runs;
    app.state.workbench.activate(Workspace::Results);
    let time = vec![0.0, 0.5, 1.0, 1.5];
    let (plus, minus) = ([0.2, 1.4, 0.9, -0.3], [0.1, 0.5, 1.25, -0.8]);
    run_plan_with(
        &mut app.state,
        vec![
            RetainedTrace::new("V(pa)", time.clone(), plus.to_vec(), "#ffffff"),
            RetainedTrace::new("V(pb)", time.clone(), minus.to_vec(), "#ffffff"),
        ],
    );
    show_newest(&mut app.state);
    quick_frame(&mut app.state);

    // The nets PA and PB, drawn away from everything else on the sheet.
    for (y, name) in [(2000, "PA"), (2100, "PB")] {
        let (a, b) = (Point::new(2000, y), Point::new(2040, y));
        app.state.model.schematic.add_wire(vec![a, b]);
        app.state.model.schematic.add_net_label(a, name.to_owned());
    }
    app.state.sync_active_schematic_to_workspace();
    app.state.workbench.activate(Workspace::Design);

    drag_probe(
        &mut app.state.canvas_frame(),
        Point::new(2020, 2000),
        Point::new(2020, 2100),
    );
    app.after_contribution();
    quick_frame(&mut app.state);

    // The run names the nets `V(pa)` and `V(pb)`, and its traces read
    // `V(PA)` and `V(PB)` as the drawing writes the nets: so does the
    // difference.
    let difference = "V(PA) - V(PB)";
    assert_eq!(
        view_expressions(&app.state),
        vec![(difference.to_owned(), true)],
        "the difference is on the view; the status line says {:?}",
        status_line(&app)
    );
    assert_eq!(
        status_line(&app).as_deref(),
        Some("V(PA) - V(PB) added to the plot.")
    );
    assert!(
        difference_ends(&app).is_empty(),
        "a run that holds both nets is only looked at: no marker is kept"
    );
    let run = app.state.model.simulation.active_run().expect("the run");
    let expression = try_parse(difference).expect("the view's expression parses");
    let value = evaluate(
        &expression,
        &WaveformsContext::new(&run.analyses[0].waveforms),
    )
    .expect("the view's expression evaluates");
    let CalcValue::Real(RealValue::Waveform(x, y)) = value else {
        panic!("a waveform, not {value:?}");
    };
    assert_eq!(x, time, "on the run's own samples");
    let expected: Vec<f64> = plus.iter().zip(minus).map(|(p, m)| p - m).collect();
    assert_eq!(y, expected, "the two nets' difference at every sample");
}

/// Where the sheet's two-net probe ends stand, and which end each is.
fn difference_ends(app: &RSpiceApp) -> Vec<(Point, rspice_app_state::state::DifferenceEnd)> {
    app.state
        .model
        .schematic
        .document()
        .probes
        .iter()
        .filter_map(|probe| Some((probe.position, probe.difference_end?)))
        .collect()
}

/// A two-net probe before any run: a marker with its sign at each end
/// saves its net, and the difference plots once the next run holds both,
/// as a single net's probe plots once the run saves it.
#[test]
fn a_two_net_probe_before_any_run_plots_after_the_next_run() {
    use super::working_view::{quick_frame, run_plan_with, show_newest, view_expressions};
    use rspice_app_design::schematic::view::gesture_commits::drag_probe;
    use rspice_app_runs::simulation::controller::test_execution::state_with_current_clean_drc;
    use rspice_app_state::state::{DifferenceEnd, RetainedTrace};

    let host = state_with_current_clean_drc();
    let mut app = RSpiceApp::test_instance();
    app.state.model = host.model;
    app.state.runs = host.runs;
    for (y, name) in [(2000, "PA"), (2100, "PB")] {
        let (a, b) = (Point::new(2000, y), Point::new(2040, y));
        app.state.model.schematic.add_wire(vec![a, b]);
        app.state.model.schematic.add_net_label(a, name.to_owned());
    }
    app.state.sync_active_schematic_to_workspace();
    app.state.workbench.activate(Workspace::Design);
    assert!(app.state.model.simulation.active_run().is_none());

    drag_probe(
        &mut app.state.canvas_frame(),
        Point::new(2020, 2000),
        Point::new(2020, 2100),
    );
    app.after_contribution();
    assert_eq!(
        status_line(&app).as_deref(),
        Some("Saved V(PA) and V(PB) for the next run; V(PA) - V(PB) plots once it runs.")
    );
    assert_eq!(
        difference_ends(&app),
        vec![
            (Point::new(2020, 2000), DifferenceEnd::Plus),
            (Point::new(2020, 2100), DifferenceEnd::Minus),
        ],
        "a marker with its sign at each end"
    );
    assert!(
        app.state
            .model
            .schematic
            .document()
            .probes
            .iter()
            .all(|probe| !probe.plot_on_materialization),
        "an end saves its net for the difference, not to plot it alone"
    );

    let time = vec![0.0, 1.0];
    run_plan_with(
        &mut app.state,
        vec![
            RetainedTrace::new("V(pa)", time.clone(), vec![1.0, 2.0], "#ffffff"),
            RetainedTrace::new("V(pb)", time, vec![0.5, 0.5], "#ffffff"),
        ],
    );
    show_newest(&mut app.state);
    quick_frame(&mut app.state);
    assert_eq!(
        view_expressions(&app.state),
        vec![("V(PA) - V(PB)".to_owned(), true)],
        "the next run's view plots the difference"
    );
}

/// Run details' Analyses table on the active run, drawn a frame at a time;
/// each frame returns the route out of the dialog the reader took.
struct RunDetailsFrames {
    ctx: egui::Context,
    /// The clock, a second on each frame, so one frame's click does not
    /// join the next frame's into a double-click.
    time: f64,
}

impl RunDetailsFrames {
    fn open(app: &mut RSpiceApp) -> Self {
        use rspice_app_results::workbench::docks::inspector::run_details;
        let run_id = app.state.model.simulation.active_run().expect("the run").id;
        run_details::open(
            &mut app.state.results.run_details,
            &app.state.model.simulation,
            run_id,
        );
        let ctx = egui::Context::default();
        rspice_app_workbench::ui::Theme::default().apply(&ctx);
        let mut frames = Self { ctx, time: 0.0 };
        // The dialog takes its size over its first frames.
        for _ in 0..3 {
            frames.frame(app, Vec::new());
        }
        frames
    }

    /// One frame over `events`: what the dialog returned, and where the
    /// inventory's rows were drawn, top first.
    fn frame(
        &mut self,
        app: &mut RSpiceApp,
        events: Vec<egui::Event>,
    ) -> (
        rspice_app_results::workbench::docks::inspector::run_details::RunDetailsRoutes,
        Vec<egui::Rect>,
    ) {
        use rspice_app_results::workbench::docks::inspector::run_details;
        self.time += 1.0;
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1600.0, 900.0),
            )),
            time: Some(self.time),
            events,
            ..Default::default()
        };
        let _ = rspice_ui_kit::probe::take(&self.ctx);
        let mut routes = None;
        let _ = self.ctx.run_ui(input, |ui| {
            routes = Some(run_details::show(
                ui.ctx(),
                &mut app.state.results.run_details,
                &app.state.model,
            ));
        });
        let probes = rspice_ui_kit::probe::take(&self.ctx);
        let probe = |key: String| {
            probes
                .iter()
                .find(|(probed, _)| *probed == key)
                .map(|(_, rect)| *rect)
        };
        let lines = (0..)
            .map_while(|row| probe(format!("table.line.{row}")))
            .collect();
        (routes.expect("the frame ran"), lines)
    }

    /// A click on the middle of `line`.
    fn click(line: egui::Rect) -> Vec<egui::Event> {
        let at = line.center();
        let button = |pressed| egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        vec![egui::Event::PointerMoved(at), button(true), button(false)]
    }

    /// A press of `key`.
    fn key(key: egui::Key) -> Vec<egui::Event> {
        vec![egui::Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }]
    }
}

/// The schematic of [`result_app_with_current_out_map`] with a finished run
/// that kept `V(out)`, from before runs carried a prepared receipt, so the
/// inventory's rows are the run's own analyses.
fn finished_run_of_out() -> RSpiceApp {
    use rspice_app_state::state::{
        AnalysisResult, AnalysisType, RetainedTrace, SimulationRun, SimulationRunLifecycle,
    };
    let mut app = result_app_with_current_out_map(false);
    let mut run = SimulationRun::new(1);
    run.lifecycle = SimulationRunLifecycle::Completed;
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
            RetainedTrace::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#ffbd2e"),
        ]),
    );
    app.state.results.results.session.selected_trace = None;
    app.state.model.simulation.retained.runs = vec![run].into();
    app.state.model.simulation.view.active_run_idx = Some(0);
    app.state.model.simulation.view.active_analysis_idx = None;
    app
}

/// Run details' Analyses table leads where its rows name, from the keyboard:
/// Enter on an analysis shows it on Results, and Enter on a signal the
/// analysis kept, listed once the Right arrow opens the analysis, selects its
/// conductor on the schematic. A click puts the keys on the table.
#[test]
fn run_details_inventory_rows_reach_their_places() {
    use rspice_app_results::workbench::docks::inspector::run_details::follow_inventory_route;

    let mut app = finished_run_of_out();
    app.state.workbench.activate(Workspace::Design);
    let run_id = app.state.model.simulation.active_run().expect("the run").id;

    let mut dialog = RunDetailsFrames::open(&mut app);
    let (_, lines) = dialog.frame(&mut app, Vec::new());
    let analysis = *lines.first().expect("the analysis is listed");
    dialog.frame(&mut app, RunDetailsFrames::click(analysis));
    let (routes, ..) = dialog.frame(&mut app, RunDetailsFrames::key(egui::Key::Enter));
    assert_eq!(routes.show_analysis, Some((run_id, 0)));
    follow_inventory_route(&mut app.state.results_parts(), &routes);
    app.after_contribution();
    assert_eq!(app.state.workbench.workspace, Workspace::Results);
    assert_eq!(app.state.model.simulation.view.active_analysis_idx, Some(0));
    assert!(
        app.state.results.run_details.is_none(),
        "taking a route closes Run details"
    );

    // Open the analysis to its signals and step down to the one it kept.
    let mut dialog = RunDetailsFrames::open(&mut app);
    let (_, lines) = dialog.frame(&mut app, Vec::new());
    let analysis = *lines.first().expect("the analysis is listed");
    dialog.frame(&mut app, RunDetailsFrames::click(analysis));
    dialog.frame(&mut app, RunDetailsFrames::key(egui::Key::ArrowRight));
    let (_, lines) = dialog.frame(&mut app, RunDetailsFrames::key(egui::Key::ArrowDown));
    assert_eq!(lines.len(), 2, "V(out) is listed under its analysis");
    let (routes, ..) = dialog.frame(&mut app, RunDetailsFrames::key(egui::Key::Enter));
    assert_eq!(routes.show_signal.as_deref(), Some("V(out)"));
    follow_inventory_route(&mut app.state.results_parts(), &routes);
    app.after_contribution();
    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(
        status_line(&app).as_deref(),
        Some("Selected conductor OUT from V(out).")
    );
}

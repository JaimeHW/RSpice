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
    let (a, b) = (Point::new(0, 0), Point::new(40, 0));
    let mut app = RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Results);
    let root = app.state.model.workspace.content.active_view.clone();
    let drawing = app.state.model.schematic.document_mut_for_test();
    drawing.components.push(placed(101, "XA", "work", "child"));
    drawing.wires.push(Wire::new(91, vec![a, b]));
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

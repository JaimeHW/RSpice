//! Cross-probing a generated netlist line to the schematic, through whole
//! application frames: the button on the navigator's source mapping shows
//! the instance the line was written for, on its schematic in Design.

use crate::workbench::RSpiceApp;
use crate::workbench::app::veriloga_design_tests::{DESKTOP, frame, frame_with, themed_context};
use rspice_app_code::workbench::code::commands::CodeCommand;
use rspice_app_state::state::{ComponentType, Point};
use rspice_app_state::workbench::workspace::Workspace;

/// A press and release in the middle of the control `output` announced as
/// `label`.
fn click_on(output: &egui::FullOutput, label: &str) -> Vec<egui::Event> {
    let bounds = output
        .platform_output
        .accesskit_update
        .as_ref()
        .expect("the frame publishes its tree")
        .nodes
        .iter()
        .find_map(|(_, node)| (node.label()? == label).then_some(node.bounds()?))
        .unwrap_or_else(|| panic!("the frame announces `{label}`"));
    let at = egui::pos2(
        ((bounds.x0 + bounds.x1) / 2.0) as f32,
        ((bounds.y0 + bounds.y1) / 2.0) as f32,
    );
    vec![
        egui::Event::PointerMoved(at),
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: true,
            modifiers: egui::Modifiers::default(),
        },
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed: false,
            modifiers: egui::Modifiers::default(),
        },
    ]
}

/// The button selects the line's instance on its schematic, brings Design to
/// the front and centres the view on it. Selecting it in place left the
/// reader on the Netlist page with nothing visible changed.
#[test]
fn cross_probing_a_netlist_line_shows_its_instance_on_the_schematic() {
    let mut app = RSpiceApp::test_instance();
    app.state.workbench.workspace = Workspace::Design;
    app.state.open_project_session();
    let far = Point::new(4000, 3000);
    let resistor = app
        .state
        .model
        .schematic
        .add_component(ComponentType::Resistor, far);
    app.state.sync_active_schematic_to_workspace();
    let schematic = app.state.model.workspace.content.active_view.clone();

    // Arrive on the resistor's card the way the reader does, then leave
    // nothing selected and the view far from it.
    app.state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_component(resistor);
    app.execute_command_key(CodeCommand::ShowInNetlist.key());
    assert_eq!(app.state.workbench.workspace, Workspace::Netlist);
    app.state.model.schematic.session.editor.selection.clear();
    app.state.model.schematic.session.editor.needs_fit = false;
    app.state.model.schematic.session.editor.pan = (0.0, 0.0);
    app.state.workbench.navigator_visible = true;

    let ctx = themed_context();
    frame(&ctx, &mut app, DESKTOP);
    let output = frame(&ctx, &mut app, DESKTOP);
    let press = click_on(&output, "Cross-probe schematic component");
    frame_with(&ctx, &mut app, DESKTOP, press);
    frame(&ctx, &mut app, DESKTOP);

    assert_eq!(app.state.workbench.workspace, Workspace::Design);
    assert_eq!(app.state.model.workspace.content.active_view, schematic);
    assert_eq!(
        app.state
            .model
            .schematic
            .session
            .editor
            .selection
            .single_component(),
        Some(resistor)
    );
    // Where the instance now sits on the canvas: inside the window, which
    // the view at the origin it was left at could not show.
    let editor = &app.state.model.schematic.session.editor;
    let on_canvas = (
        editor.pan.0 + f64::from(far.x) * editor.zoom,
        editor.pan.1 + f64::from(far.y) * editor.zoom,
    );
    assert!(
        (0.0..f64::from(DESKTOP.x)).contains(&on_canvas.0)
            && (0.0..f64::from(DESKTOP.y)).contains(&on_canvas.1),
        "the instance is brought into view: {on_canvas:?}"
    );
}

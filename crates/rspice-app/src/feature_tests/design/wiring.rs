//! Wiring on the canvas driven through a whole application.

use crate::workbench::app_state::AppState;
use rspice_app_design::schematic::view::interaction::handle_tool_interactions;
use rspice_app_design::schematic::view::schematic_symbol_context;
use rspice_app_state::state::{BusDeclaration, ComponentType, Point, Tool, Wire};
use rspice_schematic_editor::view::viewport::Viewport;

/// One frame of the armed tool at `time` seconds with `events`, on a focused
/// canvas whose screen points are document points.
fn canvas_frame(ctx: &egui::Context, state: &mut AppState, time: f64, events: Vec<egui::Event>) {
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(640.0, 480.0),
            )),
            time: Some(time),
            events,
            ..Default::default()
        },
        |root| {
            egui::CentralPanel::default().show(root, |ui| {
                let bounds = ui.max_rect();
                let viewport = Viewport {
                    offset: egui::pos2(-bounds.min.x, -bounds.min.y),
                    zoom: 1.0,
                    bounds,
                };
                // accessibility-pointer-shim: test-only canvas event harness.
                let response = ui.interact(
                    bounds,
                    egui::Id::new("test-schematic-canvas"),
                    egui::Sense::click_and_drag(),
                );
                response.request_focus();
                let symbols = schematic_symbol_context(state);
                handle_tool_interactions(
                    ui,
                    &response,
                    &mut state.canvas_parts(),
                    &viewport,
                    &symbols,
                );
            });
        },
    );
}

/// A primary click at `at`: pressed at `time`, released a frame later.
fn click(ctx: &egui::Context, state: &mut AppState, time: f64, at: Point) {
    let pos = egui::pos2(at.x as f32, at.y as f32);
    for (offset, pressed) in [(0.0, true), (0.02, false)] {
        canvas_frame(
            ctx,
            state,
            time + offset,
            vec![
                egui::Event::PointerMoved(pos),
                egui::Event::PointerButton {
                    pos,
                    button: egui::PointerButton::Primary,
                    pressed,
                    modifiers: egui::Modifiers::NONE,
                },
            ],
        );
    }
}

fn backspace(ctx: &egui::Context, state: &mut AppState, time: f64) {
    canvas_frame(
        ctx,
        state,
        time,
        vec![egui::Event::Key {
            key: egui::Key::Backspace,
            physical_key: Some(egui::Key::Backspace),
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
    );
}

/// The wire tool armed on an editable canvas, one idle frame in.
fn wire_tool(ctx: &egui::Context, state: &mut AppState) {
    state.model.schematic.init_undo_history();
    state.model.schematic.arm_tool(Tool::Wire);
    canvas_frame(ctx, state, 0.0, Vec::new());
}

/// Double-clicking a wire ends the route on it. The second click of the
/// double-click does not start a new route on the wire just joined.
#[test]
fn a_double_click_on_a_wire_ends_the_route_without_starting_another() {
    let ctx = egui::Context::default();
    let mut state = AppState::default();
    state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::segment(1, Point::new(100, 200), Point::new(300, 200)));
    wire_tool(&ctx, &mut state);
    click(&ctx, &mut state, 0.1, Point::new(200, 100));
    assert!(state.model.schematic.session.editor.wire_drawing.active);

    click(&ctx, &mut state, 1.0, Point::new(200, 200));
    click(&ctx, &mut state, 1.1, Point::new(200, 200));

    let drawing = &state.model.schematic.session.editor.wire_drawing;
    assert!(!drawing.active, "a new route began at {:?}", drawing.points);
    assert!(
        state
            .model
            .schematic
            .document()
            .wires
            .iter()
            .any(|wire| wire.points == vec![Point::new(200, 100), Point::new(200, 200)]),
        "{:?}",
        state.model.schematic.document().wires
    );
}

/// A click on a pin ends the route there, as a click on a wire does.
#[test]
fn a_click_on_a_pin_ends_the_route() {
    let ctx = egui::Context::default();
    let mut state = AppState::default();
    state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(200, 200));
    let pin = state.model.schematic.document().components[0].terminal_positions()[0].1;
    let start = Point::new(pin.x, pin.y - 100);
    wire_tool(&ctx, &mut state);
    click(&ctx, &mut state, 0.1, start);

    click(&ctx, &mut state, 1.0, pin);

    let drawing = &state.model.schematic.session.editor.wire_drawing;
    assert!(!drawing.active, "the route ran on: {:?}", drawing.points);
    assert!(
        state
            .model
            .schematic
            .document()
            .wires
            .iter()
            .any(|wire| wire.points == vec![start, pin]),
        "{:?}",
        state.model.schematic.document().wires
    );
}

/// Backspace takes back the last vertex the user placed, with the corner the
/// router added to reach it. The route stays live until its first point goes,
/// and nothing is drawn.
#[test]
fn backspace_takes_back_the_last_route_vertex() {
    let ctx = egui::Context::default();
    let mut state = AppState::default();
    wire_tool(&ctx, &mut state);
    let (first, second, third) = (
        Point::new(100, 100),
        Point::new(100, 200),
        Point::new(200, 300),
    );
    click(&ctx, &mut state, 0.1, first);
    click(&ctx, &mut state, 1.0, second);
    click(&ctx, &mut state, 2.0, third);
    assert_eq!(
        state
            .model
            .schematic
            .session
            .editor
            .wire_drawing
            .points
            .len(),
        4,
        "two clicks in line and one that turns a corner"
    );

    backspace(&ctx, &mut state, 3.0);
    assert_eq!(
        state.model.schematic.session.editor.wire_drawing.points,
        vec![first, second]
    );
    backspace(&ctx, &mut state, 3.1);
    assert_eq!(
        state.model.schematic.session.editor.wire_drawing.points,
        vec![first]
    );
    assert!(state.model.schematic.session.editor.wire_drawing.active);

    backspace(&ctx, &mut state, 3.2);
    assert!(!state.model.schematic.session.editor.wire_drawing.active);
    assert!(state.model.schematic.document().wires.is_empty());
}

/// A click on a bus ends a bus route there, on the bus, as a click on a wire
/// ends a wire route; a route drawn without a name takes the name of the bus
/// it joins.
#[test]
fn a_click_on_a_bus_ends_the_bus_route_on_it() {
    let ctx = egui::Context::default();
    let mut state = AppState::default();
    state
        .model
        .schematic
        .add_bus(
            vec![Point::new(100, 200), Point::new(300, 200)],
            Some(BusDeclaration::parse("D[3:0]").unwrap()),
        )
        .unwrap();
    state.model.schematic.init_undo_history();
    state.model.schematic.arm_tool(Tool::Bus);
    canvas_frame(&ctx, &mut state, 0.0, Vec::new());
    click(&ctx, &mut state, 0.1, Point::new(200, 100));
    assert!(state.model.schematic.session.editor.bus_drawing.active);

    // A click a few points off the bus lands on it and ends the route.
    click(&ctx, &mut state, 1.0, Point::new(203, 197));

    let drawing = &state.model.schematic.session.editor.bus_drawing;
    assert!(!drawing.active, "the route went on: {:?}", drawing.points);
    let buses = &state.model.schematic.document().buses;
    assert_eq!(buses.len(), 2, "{buses:?}");
    let route = buses
        .iter()
        .find(|bus| bus.points.first() == Some(&Point::new(200, 100)))
        .expect("the routed bus");
    assert_eq!(route.points.last(), Some(&Point::new(200, 200)));
    assert_eq!(
        route
            .declaration
            .as_ref()
            .map(ToString::to_string)
            .as_deref(),
        Some("D[3:0]")
    );
}

/// The Bus strip's Name names every bus drawn while it holds one, stroke
/// after stroke, rather than the first bus alone.
#[test]
fn every_bus_drawn_takes_the_name_the_strip_holds() {
    let ctx = egui::Context::default();
    let mut state = AppState::default();
    state.model.schematic.init_undo_history();
    state.model.schematic.arm_tool(Tool::Bus);
    state.model.schematic.session.editor.bus_drawing.name = "D[3:0]".to_owned();
    canvas_frame(&ctx, &mut state, 0.0, Vec::new());
    for (stroke, y) in [100, 300].into_iter().enumerate() {
        let time = 1.0 + stroke as f64 * 2.0;
        click(&ctx, &mut state, time, Point::new(100, y));
        click(&ctx, &mut state, time + 0.5, Point::new(300, y));
        canvas_frame(
            &ctx,
            &mut state,
            time + 0.6,
            vec![egui::Event::Key {
                key: egui::Key::Enter,
                physical_key: Some(egui::Key::Enter),
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::NONE,
            }],
        );
    }
    let buses = &state.model.schematic.document().buses;
    assert_eq!(buses.len(), 2, "{buses:?}");
    for bus in buses {
        assert_eq!(
            bus.declaration.as_ref().map(ToString::to_string).as_deref(),
            Some("D[3:0]")
        );
    }
}

/// Undo while a wire is being drawn takes back its last point, as
/// Backspace does, from the Edit menu and the touch strip alike, and keeps
/// the edit made before the route. It undid that edit underneath the live
/// route instead. Redo waits for the route to end.
#[test]
fn undo_while_routing_takes_back_the_last_point_and_keeps_the_edit_before_it() {
    use rspice_app_state::workbench::workspace::Workspace;
    use rspice_app_workbench::workbench::framework::command_registry::ids;

    let mut app = crate::workbench::RSpiceApp::test_instance();
    app.state.open_project_session();
    app.state.workbench.activate(Workspace::Design);
    app.state.sync_active_schematic_to_workspace();
    app.state.model.schematic.init_undo_history();
    app.state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .clear();
    app.state.model.schematic.arm_tool(Tool::Wire);
    app.state.model.schematic.start_wire(Point::new(0, 200));
    app.state.model.schematic.extend_wire(Point::new(200, 200));
    let earlier = app
        .state
        .model
        .schematic
        .finish_wire(|_| false)
        .expect("the first wire is laid down");
    assert!(app.state.model.schematic.can_undo(), "the wire is one edit");
    app.state.model.schematic.start_wire(Point::new(0, 0));
    app.state.model.schematic.extend_wire(Point::new(100, 0));
    app.state.model.schematic.extend_wire(Point::new(100, 80));
    assert!(app.command_key_enabled(ids::UNDO));
    assert!(!app.command_key_enabled(ids::REDO));

    app.execute_command_key(ids::UNDO);

    let drawing = &app.state.model.schematic.session.editor.wire_drawing;
    assert!(drawing.active, "the route is still being drawn");
    assert_eq!(drawing.points, vec![Point::new(0, 0), Point::new(100, 0)]);
    assert!(
        app.state
            .model
            .schematic
            .document()
            .wires
            .iter()
            .any(|wire| wire.id == earlier),
        "the edit before the route stands"
    );
    assert!(app.state.model.schematic.can_undo());
}

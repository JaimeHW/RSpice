//! Wiring on the canvas driven through a whole application.

use crate::schematic::view::interaction::handle_tool_interactions;
use crate::schematic::view::schematic_symbol_context;
use crate::state::{ComponentType, Point, Tool, Wire};
use crate::workbench::app_state::AppState;
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
                    &mut crate::schematic::view::canvas_parts(state),
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

//! Documentation shapes drawn on the canvas through real egui frames: what a
//! drag lays down, where it lands, and what the shape keeps.

use super::gestures_drawn::Fixture;
use rspice_app_design::workbench::design::commands::DesignCommand;
use rspice_app_state::state::{DocumentationShapeGeometry, DocumentationShapeKind, Point, Tool};

/// Arm `kind` as the reader does, by its Place, Shape command: the tool is
/// armed at once, with nothing asked first.
fn arm(fixture: &mut Fixture, kind: DocumentationShapeKind) {
    let command = DesignCommand::ALL
        .into_iter()
        .find(|command| command.documentation_shape() == Some(kind))
        .expect("every shape has its command");
    fixture.app.execute_command_key(command.key());
    let editor = &fixture.app.state.model.schematic.session.editor;
    assert_eq!(editor.tool, Tool::DocumentationShape, "{command:?}");
    assert_eq!(
        editor
            .pending_documentation_shape
            .as_ref()
            .map(|pending| pending.kind),
        Some(kind),
        "{command:?} arms its own shape"
    );
}

fn screen(fixture: &Fixture, x: f32, y: f32) -> egui::Pos2 {
    fixture.origin + egui::vec2(x, y)
}

fn button(fixture: &Fixture, x: f32, y: f32, pressed: bool) -> egui::Event {
    egui::Event::PointerButton {
        pos: screen(fixture, x, y),
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    }
}

/// The grid point the armed tool reads under the pointer at `(x, y)`.
fn grid_point_under(fixture: &mut Fixture, x: f32, y: f32) -> Point {
    // From a point beside it, so the pointer has moved when it arrives.
    for at in [screen(fixture, x - 7.0, y - 7.0), screen(fixture, x, y)] {
        fixture.frame(vec![egui::Event::PointerMoved(at)], true);
    }
    fixture
        .app
        .state
        .model
        .schematic
        .session
        .editor
        .documentation_shape_drawing
        .keyboard_cursor
        .expect("the armed tool follows the pointer")
}

/// Press at `from`, move past the drag threshold, and release at `to`.
fn drag(fixture: &mut Fixture, from: (f32, f32), to: (f32, f32)) {
    fixture.frame(
        vec![
            egui::Event::PointerMoved(screen(fixture, from.0, from.1)),
            button(fixture, from.0, from.1, true),
        ],
        true,
    );
    let midway = ((from.0 + to.0) * 0.5, (from.1 + to.1) * 0.5);
    for (x, y) in [midway, to] {
        fixture.frame(vec![egui::Event::PointerMoved(screen(fixture, x, y))], true);
    }
    fixture.frame(vec![button(fixture, to.0, to.1, false)], true);
}

/// A rectangle or a line is drawn by one drag, as the drawing tools of
/// KiCad and Virtuoso draw them: the press is one corner or end and the
/// release the other, and the shape lands where it was dragged.
#[test]
fn one_drag_draws_a_rectangle_or_a_line_where_it_was_dragged() {
    for kind in [
        DocumentationShapeKind::Rectangle,
        DocumentationShapeKind::Line,
    ] {
        let mut fixture = Fixture::new(false);
        arm(&mut fixture, kind);
        let from = grid_point_under(&mut fixture, 260.0, 140.0);
        let to = grid_point_under(&mut fixture, 380.0, 230.0);
        assert_ne!(from, to);
        drag(&mut fixture, (260.0, 140.0), (380.0, 230.0));

        let state = &fixture.app.state;
        let shapes = &state.model.schematic.document().documentation_shapes;
        assert_eq!(shapes.len(), 1, "{kind:?}: one drag draws one shape");
        let expected = match kind {
            DocumentationShapeKind::Rectangle => DocumentationShapeGeometry::Rectangle {
                first: from,
                opposite: to,
            },
            _ => DocumentationShapeGeometry::Line {
                start: from,
                end: to,
            },
        };
        assert_eq!(shapes[0].geometry, expected);
        assert_eq!(state.model.schematic.session.editor.tool, Tool::Select);
        assert!(
            state
                .model
                .schematic
                .session
                .editor
                .selection
                .has_documentation_shape(shapes[0].id)
        );
    }
}

/// A shape of three points takes its first two from a drag and the last
/// from a click.
#[test]
fn a_drag_lays_down_a_callouts_leader_and_a_click_its_box() {
    let mut fixture = Fixture::new(false);
    arm(&mut fixture, DocumentationShapeKind::Callout);
    let tip = grid_point_under(&mut fixture, 200.0, 250.0);
    let elbow = grid_point_under(&mut fixture, 280.0, 180.0);
    let corner = grid_point_under(&mut fixture, 420.0, 130.0);
    drag(&mut fixture, (200.0, 250.0), (280.0, 180.0));
    assert!(
        fixture
            .app
            .state
            .model
            .schematic
            .document()
            .documentation_shapes
            .is_empty(),
        "a callout waits for its box"
    );
    fixture.frame(
        vec![
            egui::Event::PointerMoved(screen(&fixture, 420.0, 130.0)),
            button(&fixture, 420.0, 130.0, true),
        ],
        true,
    );
    fixture.frame(vec![button(&fixture, 420.0, 130.0, false)], true);

    let shapes = &fixture
        .app
        .state
        .model
        .schematic
        .document()
        .documentation_shapes;
    assert_eq!(shapes.len(), 1);
    assert_eq!(
        shapes[0].geometry,
        DocumentationShapeGeometry::Callout {
            tip,
            elbow,
            box_corner: corner,
        }
    );
}

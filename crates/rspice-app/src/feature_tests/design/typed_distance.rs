//! A distance typed on the canvas while Move or Stretch is armed: "dx dy" in
//! millimetres, Enter lands it, through the whole frame the app draws.

use super::gestures_drawn::Fixture;
use rspice_app_design::workbench::design::commands::DesignCommand;
use rspice_app_state::state::{Point, Tool, Wire};

/// The key press and the character a keyboard sends for one typed key.
fn typed(fixture: &mut Fixture, text: &str) {
    for character in text.chars() {
        let key = match character {
            '-' => egui::Key::Minus,
            '.' => egui::Key::Period,
            ' ' => egui::Key::Space,
            digit => egui::Key::from_name(&digit.to_string()).expect("a digit key"),
        };
        fixture.frame(
            vec![
                egui::Event::Key {
                    key,
                    physical_key: Some(key),
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers::NONE,
                },
                egui::Event::Text(character.to_string()),
            ],
            true,
        );
    }
}

fn press(fixture: &mut Fixture, key: egui::Key) {
    fixture.frame(
        vec![egui::Event::Key {
            key,
            physical_key: Some(key),
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        }],
        true,
    );
}

/// Click empty canvas so it holds the keyboard, as the reader's last click did.
fn focus_the_canvas(fixture: &mut Fixture) {
    let empty = fixture.origin + egui::vec2(400.0, 300.0);
    fixture.frame(vec![egui::Event::PointerMoved(empty)], true);
    for pressed in [true, false] {
        fixture.frame(
            vec![egui::Event::PointerButton {
                pos: empty,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: egui::Modifiers::NONE,
            }],
            true,
        );
    }
}

fn prompt(fixture: &Fixture) -> String {
    fixture
        .app
        .state
        .workbench
        .prompt
        .as_ref()
        .map(|prompt| prompt.text.clone())
        .unwrap_or_default()
}

/// M, then "-5 2.5" and Enter, moves the part 5 mm left and 2.5 mm down in
/// one undo step. The minus types a sign rather than zooming out, and Esc
/// clears a half-typed distance without dropping Move.
#[test]
fn a_typed_distance_moves_the_selection_by_that_many_millimetres() {
    let mut fixture = Fixture::new(false);
    focus_the_canvas(&mut fixture);
    let part = fixture.app.state.model.schematic.document().components[0].id;
    let start = fixture.app.state.model.schematic.document().components[0].pos;
    let editor = &mut fixture.app.state.model.schematic.session.editor;
    editor.selection.select_only_component(part);
    let zoom = editor.zoom;
    fixture
        .app
        .execute_command_key(DesignCommand::MoveSelection.key());
    assert!(fixture.app.state.design.move_selection.armed);

    typed(&mut fixture, "12");
    press(&mut fixture, egui::Key::Escape);
    assert!(
        fixture.app.state.design.move_selection.armed,
        "Esc clears the typed distance first"
    );
    assert_eq!(fixture.app.state.design.move_selection.canvas.typed, "");

    typed(&mut fixture, "-5 2.5");
    assert_eq!(
        fixture.app.state.design.move_selection.canvas.typed,
        "-5 2.5"
    );
    assert_eq!(
        fixture.app.state.model.schematic.session.editor.zoom, zoom,
        "the minus is the distance's sign, not zoom out"
    );
    press(&mut fixture, egui::Key::Enter);

    let moved = fixture.app.state.model.schematic.document().components[0].pos;
    assert_eq!(moved, Point::new(start.x - 20, start.y + 10));
    assert_eq!(
        fixture.app.state.model.schematic.session.editor.tool,
        Tool::Select
    );
    assert_eq!(
        prompt(&fixture),
        "Moved 1 object by -5, 2.5 mm in Connected move mode."
    );
    fixture.app.state.model.schematic.undo();
    assert_eq!(
        fixture.app.state.model.schematic.document().components[0].pos,
        start
    );
}

/// S on one wire, then "0 5" and Enter, stretches that segment 5 mm down.
#[test]
fn a_typed_distance_stretches_the_segment() {
    let mut fixture = Fixture::new(false);
    fixture
        .app
        .state
        .model
        .schematic
        .document_mut_for_test()
        .wires
        .push(Wire::new(9, vec![Point::new(300, 40), Point::new(400, 40)]));
    focus_the_canvas(&mut fixture);
    fixture
        .app
        .state
        .model
        .schematic
        .session
        .editor
        .selection
        .select_only_wire(9);
    fixture
        .app
        .execute_command_key(DesignCommand::StretchSelection.key());
    assert!(fixture.app.state.design.stretch_selection.armed);

    typed(&mut fixture, "0 5");
    press(&mut fixture, egui::Key::Enter);

    let wire = fixture
        .app
        .state
        .model
        .schematic
        .document()
        .wires
        .iter()
        .find(|wire| wire.id == 9)
        .expect("the stretched wire")
        .clone();
    assert!(
        wire.points
            .windows(2)
            .any(|leg| leg[0].y == 60 && leg[1].y == 60),
        "the segment stands 5 mm lower: {:?}",
        wire.points
    );
    assert_eq!(prompt(&fixture), "Stretched the selection by 0, 5 mm.");
}

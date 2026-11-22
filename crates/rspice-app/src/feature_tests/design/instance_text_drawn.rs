//! A part's name and value on the canvas, through real frames: a click on
//! either takes the part, and a drag moves the text alone, as one undo step,
//! to where the canvas then letters it.

use super::gestures_drawn::Fixture;
use egui::{Event, Modifiers, PointerButton, Pos2, vec2};
use rspice_app_state::state::Component;
use rspice_app_workbench::workbench::framework::command_registry::ids;
use rspice_design::schematic::label_layout::{self, INSTANCE_NAME_WORLD, INSTANCE_VALUE_WORLD};
use rspice_design::schematic::label_position::{InstanceText, LabelPosition};
use rspice_schematic_editor::session::drag::DragType;

fn part(fixture: &Fixture) -> Component {
    fixture.app.state.model.schematic.document().components[0].clone()
}

/// Where the world point `(x, y)` is on the fixture's screen.
fn screen(fixture: &Fixture, x: f32, y: f32) -> Pos2 {
    let editor = &fixture.app.state.model.schematic.session.editor;
    let zoom = editor.zoom as f32;
    fixture.origin
        + vec2(
            editor.pan.0 as f32 + x * zoom,
            editor.pan.1 as f32 + y * zoom,
        )
}

/// The centre of the box the part's `text` takes, in world units.
fn centre(fixture: &Fixture, text: InstanceText) -> (f32, f32) {
    let part = part(fixture);
    let (lettering, size) = match text {
        InstanceText::Name => (part.name.clone(), INSTANCE_NAME_WORLD),
        InstanceText::Value => (part.value.clone(), INSTANCE_VALUE_WORLD),
    };
    let placed = label_layout::instance_text_placement(&part, None, text);
    let rect = label_layout::text_box(placed, &lettering, size);
    (rect.center().x, rect.center().y)
}

fn button(at: Pos2, pressed: bool) -> Event {
    Event::PointerButton {
        pos: at,
        button: PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    }
}

/// Where the frame painted `text`, if it did.
fn painted(output: &egui::FullOutput, text: &str) -> Option<egui::Rect> {
    fn find(shape: &egui::Shape, text: &str) -> Option<egui::Rect> {
        match shape {
            egui::Shape::Text(run) if run.galley.text() == text => {
                Some(shape.visual_bounding_rect())
            }
            egui::Shape::Vec(shapes) => shapes.iter().find_map(|shape| find(shape, text)),
            _ => None,
        }
    }
    output
        .shapes
        .iter()
        .find_map(|clipped| find(&clipped.shape, text))
}

#[test]
fn dragging_a_parts_value_moves_the_text_alone_as_one_undo_step() {
    let mut fixture = Fixture::new(false);
    let before = part(&fixture);
    let (x, y) = centre(&fixture, InstanceText::Value);
    let from = screen(&fixture, x, y);
    let to = screen(&fixture, x, y + 40.0);

    fixture.frame(vec![Event::PointerMoved(from), button(from, true)], true);
    fixture.frame(vec![Event::PointerMoved(from + vec2(0.0, 10.0))], true);
    fixture.frame(vec![Event::PointerMoved(to)], true);
    assert_eq!(
        fixture.app.state.design.interaction.drag.drag_type,
        DragType::MoveText(before.id, InstanceText::Value),
        "the press on the value picked up the value"
    );
    fixture.frame(vec![button(to, false)], true);
    let output = fixture.frame_output(vec![], true);

    let after = part(&fixture);
    assert_eq!(after.pos, before.pos, "the part stays where it was");
    assert_eq!(after.rotation, before.rotation);
    assert_eq!(
        after.name_label_pos,
        LabelPosition::Auto,
        "the name stays where the layout puts it"
    );
    assert!(
        matches!(after.value_label_pos, LabelPosition::Placed { .. }),
        "the value keeps the place it was dragged to: {:?}",
        after.value_label_pos
    );
    let (moved_x, moved_y) = centre(&fixture, InstanceText::Value);
    assert!(
        (moved_x - x).abs() < 1.0 && (moved_y - (y + 40.0)).abs() < 1.0,
        "the value stands 40 below where it stood: ({moved_x}, {moved_y}) from ({x}, {y})"
    );
    let drawn = painted(&output, &after.value).expect("the canvas letters the value");
    assert!(
        (drawn.center().x - to.x).abs() < 3.0 && (drawn.center().y - to.y).abs() < 3.0,
        "the canvas letters the value where it was dropped: {drawn:?}, dropped at {to:?}"
    );

    let selection = &fixture.app.state.model.schematic.session.editor.selection;
    assert!(
        selection.has_component(before.id),
        "the text's part is selected"
    );
    assert_eq!(selection.components.len(), 1);

    assert!(!fixture.app.state.model.schematic.has_pending_operation());
    assert!(fixture.app.state.model.schematic.undo());
    assert_eq!(part(&fixture), before, "one undo puts the value back");
    assert!(
        !fixture.app.state.model.schematic.can_undo(),
        "the drag was one undo step"
    );
}

#[test]
fn a_click_on_a_parts_name_selects_the_part() {
    let mut fixture = Fixture::new(false);
    fixture
        .app
        .state
        .model
        .schematic
        .session
        .editor
        .selection
        .clear();
    let before = part(&fixture);
    let (x, y) = centre(&fixture, InstanceText::Name);
    let at = screen(&fixture, x, y);

    fixture.frame(vec![Event::PointerMoved(at), button(at, true)], true);
    fixture.frame(vec![button(at, false)], true);

    let selection = &fixture.app.state.model.schematic.session.editor.selection;
    assert!(
        selection.has_component(before.id),
        "the name's part is selected"
    );
    assert_eq!(selection.components.len(), 1);
    assert_eq!(part(&fixture), before, "a click moves nothing");
    assert!(!fixture.app.state.model.schematic.can_undo());
}

fn key(key: egui::Key) -> Event {
    Event::Key {
        key,
        physical_key: Some(key),
        pressed: true,
        repeat: false,
        modifiers: Modifiers::NONE,
    }
}

/// Double-click the part's `text`, then let the entry take the caret.
fn double_click(fixture: &mut Fixture, text: InstanceText) {
    let (x, y) = centre(fixture, text);
    let at = screen(fixture, x, y);
    fixture.frame(vec![Event::PointerMoved(at), button(at, true)], true);
    fixture.frame(vec![button(at, false)], true);
    fixture.frame(vec![button(at, true)], true);
    fixture.frame(vec![button(at, false)], true);
    fixture.frame(vec![], true);
}

#[test]
fn a_double_click_on_a_value_edits_it_where_it_stands_as_one_undo_step() {
    let mut fixture = Fixture::new(false);
    let before = part(&fixture);

    double_click(&mut fixture, InstanceText::Value);
    assert!(
        fixture.app.state.design.interaction.text_entry_open(),
        "the double-click opened the value's entry"
    );
    // What is typed goes to the entry, not to the canvas's keys: R would
    // turn the part.
    fixture.frame(
        vec![key(egui::Key::R), Event::Text("4.7k".to_owned())],
        true,
    );
    fixture.frame(vec![key(egui::Key::Enter)], true);

    let after = part(&fixture);
    assert_eq!(after.value, "4.7k", "Enter commits the typed value");
    assert_eq!(after.rotation, before.rotation, "typing turned nothing");
    assert_eq!(after.pos, before.pos);
    assert!(!fixture.app.state.design.interaction.text_entry_open());
    fixture.app.execute_command_key(ids::UNDO);
    assert_eq!(part(&fixture), before, "one undo puts the value back");
    assert!(!fixture.app.state.model.schematic.can_undo());
}

#[test]
fn a_double_click_on_a_name_renames_the_part_and_escape_leaves_it() {
    let mut fixture = Fixture::new(false);
    let before = part(&fixture);

    double_click(&mut fixture, InstanceText::Name);
    assert!(fixture.app.state.design.interaction.text_entry_open());
    fixture.frame(vec![Event::Text("R9".to_owned())], true);
    fixture.frame(vec![key(egui::Key::Escape)], true);
    assert!(!fixture.app.state.design.interaction.text_entry_open());
    assert_eq!(part(&fixture), before, "Escape leaves the name as it was");

    // Long enough that egui counts the next two clicks as a double-click of
    // their own, not as the third and fourth of a triple-click.
    for _ in 0..40 {
        fixture.frame(vec![], true);
    }
    double_click(&mut fixture, InstanceText::Name);
    assert!(
        fixture.app.state.design.interaction.text_entry_open(),
        "the second double-click opened the name's entry"
    );
    fixture.frame(vec![Event::Text("R9".to_owned())], true);
    fixture.frame(vec![key(egui::Key::Enter)], true);
    assert_eq!(part(&fixture).name, "R9", "Enter renames the part");
    fixture.app.execute_command_key(ids::UNDO);
    assert_eq!(
        part(&fixture).name,
        before.name,
        "one undo puts the name back"
    );
}

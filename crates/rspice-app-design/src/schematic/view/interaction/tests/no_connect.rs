//! The no-connect tool, driven by a click on the canvas.

use super::*;

/// Where pin `pin` of part `id` ends on the canvas.
fn pin_end(state: &DesignTestHost, id: u64, pin: &str) -> Point {
    let component = state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .expect("the part is placed");
    schematic_symbol_context(state)
        .named_terminal_points(component)
        .into_iter()
        .find(|(name, _)| name == pin)
        .map(|(_, end)| end)
        .expect("the part has the pin")
}

fn marked(state: &DesignTestHost, id: u64) -> Vec<String> {
    state
        .model
        .schematic
        .document()
        .components
        .iter()
        .find(|component| component.id == id)
        .map(|component| component.no_connect_pins().map(str::to_owned).collect())
        .unwrap_or_default()
}

/// A click with the no-connect tool at `at`, read as the canvas reads it.
fn click(state: &mut DesignTestHost, at: Point) -> Option<ToolPointerAction> {
    let symbols = schematic_symbol_context(state);
    let viewport = pointer_viewport();
    let action = tool_pointer_view(&canvas_parts(state)).action(
        Tool::NoConnect,
        &viewport,
        &symbols,
        viewport.schematic_to_screen(at),
    );
    if let Some(ToolPointerAction::NoConnect { position, radius }) = action {
        crate::schematic::view::no_connect::handle_no_connect_click(
            &mut canvas_parts(state),
            position,
            radius,
            &symbols,
        );
    }
    action
}

#[test]
fn a_click_on_a_pin_end_marks_it_a_second_clears_it_and_undo_takes_each_back() {
    let mut state = DesignTestHost::default();
    let id = state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(100, 100));
    let name = state.model.schematic.document().components[0].name.clone();
    let minus = pin_end(&state, id, "-");

    click(&mut state, minus + Point::new(1, 1));
    assert_eq!(marked(&state, id), ["-"]);
    assert_eq!(
        state.prompt_text(),
        Some(format!("Marked pin {name}.- no-connect.").as_str())
    );

    click(&mut state, minus);
    assert!(marked(&state, id).is_empty());
    assert_eq!(
        state.prompt_text(),
        Some(format!("Cleared the no-connect mark on pin {name}.-.").as_str())
    );

    assert!(state.model.schematic.undo());
    assert_eq!(marked(&state, id), ["-"], "undo puts the cleared mark back");
    assert!(state.model.schematic.undo());
    assert!(marked(&state, id).is_empty(), "undo takes the mark away");
    assert!(state.model.schematic.redo());
    assert_eq!(marked(&state, id), ["-"]);
}

#[test]
fn a_click_away_from_every_pin_marks_nothing_and_says_what_to_click() {
    let mut state = DesignTestHost::default();
    let id = state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(100, 100));
    click(&mut state, Point::new(300, 300));
    assert!(marked(&state, id).is_empty());
    assert_eq!(
        state.prompt_text(),
        Some("Click the end of a pin to mark it no-connect.")
    );
}

#[test]
fn a_read_only_view_refuses_the_click() {
    let mut state = DesignTestHost::default();
    let id = state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(100, 100));
    let minus = pin_end(&state, id, "-");
    state.model.schematic.session.read_only = true;
    assert!(matches!(
        click(&mut state, minus),
        Some(ToolPointerAction::ReadOnly)
    ));
    assert!(marked(&state, id).is_empty());
}

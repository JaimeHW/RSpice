use egui::Ui;

use crate::common::app::{AppState, ConsoleMessage};

pub(super) fn render_edit_menu(ui: &mut Ui, state: &mut AppState) {
    if ui.button("Undo        Ctrl+Z").clicked() {
        perform_undo(state);
        ui.close_menu();
    }
    if ui.button("Redo        Ctrl+Y").clicked() {
        perform_redo(state);
        ui.close_menu();
    }

    ui.separator();

    if ui.button("Cut         Ctrl+X").clicked() {
        state.schematic.copy_selection();
        state.schematic.delete_selection();
        ui.close_menu();
    }
    if ui.button("Copy        Ctrl+C").clicked() {
        state.schematic.copy_selection();
        ui.close_menu();
    }
    if ui.button("Paste       Ctrl+V").clicked() {
        state.schematic.paste_at(crate::state::Point::new(200, 200));
        ui.close_menu();
    }
    if ui.button("Delete      Del").clicked() {
        state.schematic.delete_selection();
        ui.close_menu();
    }

    ui.separator();

    if ui.button("Select All  Ctrl+A").clicked() {
        select_all_components_and_wires(state);
        ui.close_menu();
    }

    if ui.button("Duplicate   Ctrl+D").clicked() {
        state.schematic.copy_selection();
        state.schematic.paste_at(crate::state::Point::new(220, 220));
        ui.close_menu();
    }
}

fn perform_undo(state: &mut AppState) {
    if state.schematic.can_undo() {
        let desc = state
            .schematic
            .undo_description()
            .unwrap_or("action")
            .to_string();
        if state.schematic.undo() {
            state.push_user_message(ConsoleMessage::info(format!("Undo: {}", desc)));
        }
    } else {
        state.push_user_message(ConsoleMessage::info("Nothing to undo"));
    }
}

fn perform_redo(state: &mut AppState) {
    if state.schematic.can_redo() {
        let desc = state
            .schematic
            .redo_description()
            .unwrap_or("action")
            .to_string();
        if state.schematic.redo() {
            state.push_user_message(ConsoleMessage::info(format!("Redo: {}", desc)));
        }
    } else {
        state.push_user_message(ConsoleMessage::info("Nothing to redo"));
    }
}

fn select_all_components_and_wires(state: &mut AppState) {
    state.schematic.selection.clear();
    for comp in &state.schematic.components {
        state.schematic.selection.select_component(comp.id);
    }
    for wire in &state.schematic.wires {
        state.schematic.selection.select_wire(wire.id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{Component, ComponentType, Point, Wire};

    #[test]
    fn test_perform_undo_emits_nothing_to_undo_message() {
        let mut state = AppState::default();

        perform_undo(&mut state);

        assert!(
            state
                .console_messages
                .iter()
                .any(|msg| msg.message.contains("Nothing to undo")),
            "expected explicit empty-history undo message"
        );
    }

    #[test]
    fn test_select_all_components_and_wires_marks_full_selection() {
        let mut state = AppState::default();
        state.schematic.components.push(
            Component::new(1, ComponentType::Resistor, Point::new(0, 0))
                .with_name_value("R1", "1k"),
        );
        state.schematic.components.push(
            Component::new(2, ComponentType::Capacitor, Point::new(100, 0))
                .with_name_value("C1", "1p"),
        );
        state
            .schematic
            .wires
            .push(Wire::new(1, vec![Point::new(0, 0), Point::new(100, 0)]));

        select_all_components_and_wires(&mut state);

        assert_eq!(state.schematic.selection.components.len(), 2);
        assert_eq!(state.schematic.selection.wires.len(), 1);
        assert!(state.schematic.selection.components.contains(&1));
        assert!(state.schematic.selection.components.contains(&2));
        assert!(state.schematic.selection.wires.contains(&1));
    }
}

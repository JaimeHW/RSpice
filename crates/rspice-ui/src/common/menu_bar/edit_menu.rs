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

use egui::Context;

use super::{
    ConsoleMessage, RSpiceApp,
    app_shortcuts::{ShortcutCommand, ShortcutInputSnapshot, collect_shortcut_commands},
};

impl RSpiceApp {
    /// Handle keyboard shortcuts
    pub(super) fn handle_shortcuts(&mut self, ctx: &Context) {
        let has_focus = ctx.memory(|memory| memory.focused().is_some());
        let snapshot = ctx.input(|input| ShortcutInputSnapshot::from_input_state(input, has_focus));
        for command in collect_shortcut_commands(&snapshot) {
            self.execute_shortcut_command(command);
        }
    }

    pub(super) fn execute_shortcut_command(&mut self, command: ShortcutCommand) {
        use crate::state::{ComponentType, Tool};

        match command {
            ShortcutCommand::FileNew => self.action_file_new(),
            ShortcutCommand::FileOpen => self.action_file_open(),
            ShortcutCommand::FileSave => {
                let _ = self.action_file_save();
            }
            ShortcutCommand::EditUndo => self.action_edit_undo(),
            ShortcutCommand::EditRedo => self.action_edit_redo(),
            ShortcutCommand::EditCopy => self.action_edit_copy(),
            ShortcutCommand::EditPaste => self.action_edit_paste(),
            ShortcutCommand::EditCut => self.action_edit_cut(),
            ShortcutCommand::EditDelete => self.action_edit_delete(),
            ShortcutCommand::EditSelectAll => self.action_edit_select_all(),
            ShortcutCommand::ToggleBrowserPanel => self.toggle_panel_browser(),
            ShortcutCommand::ToggleLogPanel => self.toggle_panel_log(),
            ShortcutCommand::ShowShortcutsHelp => {
                self.state.dialogs.shortcuts_help = true;
            }
            ShortcutCommand::ToolSelect => {
                self.state.schematic.tool = Tool::Select;
            }
            ShortcutCommand::ToolWire => {
                self.state.schematic.tool = Tool::Wire;
            }
            ShortcutCommand::PlaceGround => {
                self.state.schematic.tool = Tool::Place(ComponentType::Ground);
            }
            ShortcutCommand::PlaceVoltageSource => {
                self.state.schematic.tool = Tool::Place(ComponentType::VoltageSource);
            }
            ShortcutCommand::PlaceCurrentSource => {
                self.state.schematic.tool = Tool::Place(ComponentType::CurrentSource);
            }
            ShortcutCommand::PlaceCapacitor => {
                self.state.schematic.tool = Tool::Place(ComponentType::Capacitor);
            }
            ShortcutCommand::PlaceInductor => {
                self.state.schematic.tool = Tool::Place(ComponentType::Inductor);
            }
            ShortcutCommand::PlaceDiode => {
                self.state.schematic.tool = Tool::Place(ComponentType::Diode);
            }
            ShortcutCommand::PlaceNmos => {
                self.state.schematic.tool = Tool::Place(ComponentType::Nmos);
            }
            ShortcutCommand::PlaceNpnBjt => {
                self.state.schematic.tool = Tool::Place(ComponentType::NpnBjt);
            }
            ShortcutCommand::ToolProbe => {
                self.state.schematic.tool = Tool::Probe;
            }
            ShortcutCommand::PlaceResistor => {
                self.state.schematic.tool = Tool::Place(ComponentType::Resistor);
            }
            ShortcutCommand::RotateSelectionOrPreview => {
                self.state.schematic.preview_rotation =
                    self.state.schematic.preview_rotation.rotate_cw();
                if !self.state.schematic.selection.is_empty() {
                    self.state.schematic.rotate_selection();
                }
            }
            ShortcutCommand::MirrorSelectionHorizontal => {
                if !self.state.schematic.selection.is_empty() {
                    self.state.schematic.mirror_selection_h();
                }
            }
            ShortcutCommand::MirrorSelectionVertical => {
                if !self.state.schematic.selection.is_empty() {
                    self.state.schematic.mirror_selection_v();
                }
            }
            ShortcutCommand::OpenPropertiesEditor => {
                if let Some(comp_id) = self.state.schematic.selection.single_component()
                    && let Some(comp) = self
                        .state
                        .schematic
                        .components
                        .iter()
                        .find(|c| c.id == comp_id)
                {
                    let props = crate::properties::dialog::EditedProperties {
                        name: comp.name.clone(),
                        value: comp.value.clone(),
                        model: String::new(),
                        parameters: vec![],
                    };
                    self.state.property_editor.open_for(comp_id, props);
                }
            }
            ShortcutCommand::EscapeCancel => {
                if self.state.property_editor.open {
                    self.state.property_editor.cancel();
                } else if self.state.shell.view == crate::shell::WorkspaceView::Results
                    && self.state.shell.results.cursors.any()
                {
                    self.state.shell.results.clear_cursors();
                } else {
                    self.state.schematic.tool = Tool::Select;
                    self.state.schematic.cancel_wire();
                    self.state.schematic.selection.clear();
                    self.state.schematic.selection_rect.cancel();
                }
            }
            ShortcutCommand::RunSimulation => {
                if !self.state.schematic.components.is_empty()
                    && !self.state.simulation.is_running
                {
                    self.state.simulation.trigger_simulation = true;
                    self.state.shell.view = crate::shell::WorkspaceView::Simulate;
                }
            }
            ShortcutCommand::StopSimulation => {
                if self.state.simulation.is_running {
                    self.state.simulation.trigger_abort = true;
                }
            }
            ShortcutCommand::RunChecks => {
                crate::common::menu_bar::run_design_rule_check(&mut self.state);
            }
            ShortcutCommand::NextWorkspaceTab => {
                self.state.shell.cycle_view();
            }
            ShortcutCommand::ZoomIn => {
                self.state.schematic.zoom = (self.state.schematic.zoom * 1.25).min(4.0);
            }
            ShortcutCommand::ZoomOut => {
                self.state.schematic.zoom = (self.state.schematic.zoom / 1.25).max(0.25);
            }
            ShortcutCommand::ZoomFit => {
                self.state.schematic.needs_fit = true;
            }
            ShortcutCommand::Zoom100 => {
                self.state.schematic.zoom = 1.0;
            }
            ShortcutCommand::ToolLabel => {
                self.state.schematic.tool = Tool::Label;
            }
            ShortcutCommand::FocusCellSearch => {
                self.state.shell.view = crate::shell::WorkspaceView::Schematic;
                self.state.shell.focus_cell_search = true;
            }
        }
    }

    pub(super) fn action_edit_undo(&mut self) {
        if self.state.schematic.can_undo() {
            let desc = self
                .state
                .schematic
                .undo_description()
                .map(|s| s.to_string())
                .unwrap_or_default();
            if self.state.schematic.undo() {
                self.state
                    .push_user_message(ConsoleMessage::info(format!("Undo: {}", desc)));
            }
        } else {
            self.state
                .push_user_message(ConsoleMessage::info("Nothing to undo"));
        }
    }

    pub(super) fn action_edit_redo(&mut self) {
        if self.state.schematic.can_redo() {
            let desc = self
                .state
                .schematic
                .redo_description()
                .map(|s| s.to_string())
                .unwrap_or_default();
            if self.state.schematic.redo() {
                self.state
                    .push_user_message(ConsoleMessage::info(format!("Redo: {}", desc)));
            }
        } else {
            self.state
                .push_user_message(ConsoleMessage::info("Nothing to redo"));
        }
    }

    pub(super) fn action_edit_copy(&mut self) {
        self.state.schematic.copy_selection();
    }

    pub(super) fn action_edit_paste(&mut self) {
        use crate::state::Point;
        self.state.schematic.paste_at(Point::new(200, 200));
    }

    pub(super) fn action_edit_cut(&mut self) {
        self.state.schematic.copy_selection();
        self.state.schematic.delete_selection();
    }

    pub(super) fn action_edit_delete(&mut self) {
        self.state.schematic.delete_selection();
    }

    pub(super) fn action_edit_select_all(&mut self) {
        let schematic = &mut self.state.schematic;
        schematic.selection.clear();
        schematic.selection.components = schematic.components.iter().map(|c| c.id).collect();
        schematic.selection.wires = schematic.wires.iter().map(|w| w.id).collect();
    }

    pub(super) fn toggle_panel_browser(&mut self) {
        // Toggle the contextual side panels (focus mode).
        self.state.shell.panels_hidden = !self.state.shell.panels_hidden;
    }

    pub(super) fn toggle_panel_log(&mut self) {
        // Toggle the console between expanded and collapsed.
        self.state.shell.console.collapsed = !self.state.shell.console.collapsed;
    }
}

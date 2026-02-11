use egui::Context;

use super::{
    app_shortcuts::{collect_shortcut_commands, ShortcutCommand, ShortcutInputSnapshot},
    BottomPanelTab, ConfirmationAction, ConfirmationResponse, ConsoleMessage, RSpiceApp,
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
            ShortcutCommand::ToggleLogPanel => self.toggle_panel_log_new(),
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
                if let Some(comp_id) = self.state.schematic.selection.single_component() {
                    if let Some(comp) = self
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
            }
            ShortcutCommand::EscapeCancel => {
                if self.state.property_editor.open {
                    self.state.property_editor.cancel();
                } else {
                    self.state.schematic.tool = Tool::Select;
                    self.state.schematic.cancel_wire();
                    self.state.schematic.selection.clear();
                    self.state.schematic.selection_rect.cancel();
                }
            }
        }
    }

    /// Request a new schematic (prompts to save if dirty)
    pub(super) fn action_file_new(&mut self) {
        if self.state.schematic.is_dirty {
            // Show save confirmation dialog - don't discard unsaved changes
            self.state
                .dialogs
                .confirmation_dialog
                .show(ConfirmationAction::FileNew);
        } else {
            self.do_file_new();
        }
    }

    /// Internal: Actually create a new schematic (after confirmation)
    pub(super) fn do_file_new(&mut self) {
        crate::common::file_workflow::create_new_schematic(&mut self.state);
    }

    /// Request to open a schematic (prompts to save if dirty)
    pub(super) fn action_file_open(&mut self) {
        if self.state.schematic.is_dirty {
            // Show save confirmation dialog before opening
            self.state
                .dialogs
                .confirmation_dialog
                .show(ConfirmationAction::FileOpen);
        } else {
            self.do_file_open();
        }
    }

    /// Internal: Actually open a schematic (after confirmation)
    pub(super) fn do_file_open(&mut self) {
        let (state, io) = (&mut self.state, self.file_workflow_io.as_ref());
        crate::common::file_workflow::open_schematic_from_dialog_with_io(state, io);
    }

    /// Handle user response to save confirmation dialog
    ///
    /// This is called when the user clicks Yes, No, or Cancel in the
    /// save confirmation dialog. Commercial EDA pattern:
    /// - Yes: Save first, then execute pending action
    /// - No: Discard changes and execute pending action
    /// - Cancel: Close dialog, do nothing
    pub(super) fn handle_confirmation_response(&mut self, response: ConfirmationResponse) {
        let pending = self.state.dialogs.confirmation_dialog.pending_action;
        self.state.dialogs.confirmation_dialog.close();

        match response {
            ConfirmationResponse::Cancel => {
                // User cancelled - do nothing
            }
            ConfirmationResponse::No => {
                // Discard changes and proceed
                if let Some(action) = pending {
                    self.execute_pending_action(action);
                }
            }
            ConfirmationResponse::Yes => {
                // Save first, then proceed
                if self.action_file_save() {
                    if let Some(action) = pending {
                        self.execute_pending_action(action);
                    }
                }
            }
        }
    }

    /// Execute a pending action after confirmation dialog
    pub(super) fn execute_pending_action(&mut self, action: ConfirmationAction) {
        match action {
            ConfirmationAction::FileNew => self.do_file_new(),
            ConfirmationAction::FileOpen => self.do_file_open(),
            ConfirmationAction::Exit => {
                // Signal exit request - this will be handled by the frame update
                self.state.exit_requested = true;
            }
        }
    }

    pub(super) fn action_file_save(&mut self) -> bool {
        let (state, io) = (&mut self.state, self.file_workflow_io.as_ref());
        crate::common::file_workflow::save_schematic_with_io(state, io)
    }

    pub(super) fn action_file_save_as(&mut self) -> bool {
        let (state, io) = (&mut self.state, self.file_workflow_io.as_ref());
        crate::common::file_workflow::save_schematic_as_with_io(state, io)
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
        self.state.schematic.selection.clear();
        for comp in &self.state.schematic.components {
            self.state.schematic.selection.select_component(comp.id);
        }
        for wire in &self.state.schematic.wires {
            self.state.schematic.selection.select_wire(wire.id);
        }
    }

    pub(super) fn toggle_panel_browser(&mut self) {
        // Close results browser when opening library browser (mutually exclusive)
        if !self.state.panels.project_browser {
            self.state.panels.results_browser = false;
        }
        self.state.panels.project_browser = !self.state.panels.project_browser;
    }

    pub(super) fn toggle_panel_log(&mut self) {
        // Show bottom panel and switch to Log tab
        if self.state.panels.bottom_panel
            && self.state.panels.active_bottom_tab == BottomPanelTab::Log
        {
            self.state.panels.bottom_panel = false;
        } else {
            self.state.panels.bottom_panel = true;
            self.state.panels.active_bottom_tab = BottomPanelTab::Log;
        }
    }

    pub(super) fn toggle_panel_waveform(&mut self) {
        // Show bottom panel and switch to Waveform tab
        if self.state.panels.bottom_panel
            && self.state.panels.active_bottom_tab == BottomPanelTab::Waveform
        {
            self.state.panels.bottom_panel = false;
        } else {
            self.state.panels.bottom_panel = true;
            self.state.panels.active_bottom_tab = BottomPanelTab::Waveform;
        }
    }

    pub(super) fn toggle_panel_properties(&mut self) {
        self.state.panels.properties = !self.state.panels.properties;
    }
}

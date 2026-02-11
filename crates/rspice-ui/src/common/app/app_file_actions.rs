use super::{ConfirmationAction, ConfirmationResponse, RSpiceApp};

impl RSpiceApp {
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
}

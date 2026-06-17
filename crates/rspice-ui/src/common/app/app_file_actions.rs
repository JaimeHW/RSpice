use super::{ConfirmationAction, ConfirmationResponse, RSpiceApp};

impl RSpiceApp {
    /// Request a new schematic (prompts to save if dirty)
    pub(super) fn action_file_new(&mut self) {
        if self.state.schematic.is_dirty || self.state.workspace.any_dirty() {
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
        crate::common::file_actions::action_file_new(&mut self.state);
    }

    /// Request to open a schematic (prompts to save if dirty)
    pub(super) fn action_file_open(&mut self) {
        if self.state.schematic.is_dirty || self.state.workspace.any_dirty() {
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
        crate::common::file_actions::action_file_open_with_io(state, io);
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
        let pending_path = self.state.dialogs.confirmation_dialog.pending_path.take();
        self.state.dialogs.confirmation_dialog.close();

        match response {
            ConfirmationResponse::Cancel => {
                // User cancelled - do nothing
            }
            ConfirmationResponse::No => {
                // Discard changes and proceed
                if let Some(action) = pending {
                    self.execute_pending_action(action, pending_path);
                }
            }
            ConfirmationResponse::Yes => {
                // Save first, then proceed
                let saved = match pending {
                    Some(ConfirmationAction::ProjectNew | ConfirmationAction::ProjectOpen) => {
                        crate::common::project_workflow::save_project(&mut self.state)
                    }
                    _ => self.action_file_save(),
                };
                if saved && let Some(action) = pending {
                    self.execute_pending_action(action, pending_path);
                }
            }
        }
    }

    /// Execute a pending action after confirmation dialog
    pub(super) fn execute_pending_action(
        &mut self,
        action: ConfirmationAction,
        path: Option<std::path::PathBuf>,
    ) {
        match action {
            ConfirmationAction::ProjectNew => {
                crate::common::project_workflow::create_new_project(&mut self.state);
            }
            ConfirmationAction::ProjectOpen => {
                crate::common::project_workflow::open_project(&mut self.state);
                self.restore_workspace_after_project_load();
            }
            ConfirmationAction::FileNew => self.do_file_new(),
            ConfirmationAction::FileOpen => self.do_file_open(),
            ConfirmationAction::OpenRecent => {
                if let Some(path) = path {
                    self.do_open_recent(path);
                }
            }
            ConfirmationAction::Exit => {
                // Signal exit request - this will be handled by the frame update
                self.state.exit_requested = true;
            }
        }
    }

    /// Open an entry from the recent-files list, prompting to save first when
    /// the current document has unsaved changes.
    pub(crate) fn open_recent_file(&mut self, recent: crate::common::app::RecentFile) {
        if self.state.schematic.is_dirty || self.state.workspace.any_dirty() {
            self.state
                .dialogs
                .confirmation_dialog
                .show_with_path(ConfirmationAction::OpenRecent, recent.path);
        } else {
            self.do_open_recent(recent.path);
        }
    }

    /// Internal: actually open a recent file (after any confirmation).
    /// Entries whose file vanished are dropped from the list with a console
    /// note instead of failing silently.
    fn do_open_recent(&mut self, path: std::path::PathBuf) {
        use crate::common::app::{ConsoleMessage, RecentKind};

        if !path.exists() {
            self.state.recent_files.retain(|r| r.path != path);
            self.state
                .push_user_message(ConsoleMessage::warning(format!(
                    "File no longer exists: {}",
                    path.display()
                )));
            return;
        }

        let kind = self
            .state
            .recent_files
            .iter()
            .find(|r| r.path == path)
            .map(|r| r.kind)
            .unwrap_or(RecentKind::Schematic);

        let opened = match kind {
            RecentKind::Project => {
                let opened =
                    crate::common::project_workflow::load_project_from_path(&mut self.state, &path);
                if opened {
                    self.restore_workspace_after_project_load();
                }
                opened
            }
            RecentKind::Schematic => {
                let (state, io) = (&mut self.state, self.file_workflow_io.as_ref());
                crate::common::file_workflow::load_schematic_from_path_with_io(state, &path, io)
            }
        };

        if opened {
            self.state.shell.view = crate::shell::WorkspaceView::Schematic;
        }
    }

    pub(super) fn action_file_save(&mut self) -> bool {
        if self.state.should_save_project_for_active_document() {
            return crate::common::project_workflow::save_project(&mut self.state);
        }
        let (state, io) = (&mut self.state, self.file_workflow_io.as_ref());
        crate::common::file_actions::action_file_save_with_io(state, io)
    }
}

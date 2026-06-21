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
        let pending_example = self
            .state
            .dialogs
            .confirmation_dialog
            .pending_example
            .take();
        self.state.dialogs.confirmation_dialog.close();

        match response {
            ConfirmationResponse::Cancel => {
                // User cancelled - do nothing
            }
            ConfirmationResponse::No => {
                // Discard changes and proceed
                if let Some(action) = pending {
                    self.execute_pending_action(action, pending_path, pending_example);
                }
            }
            ConfirmationResponse::Yes => {
                // Save first, then proceed
                let saved = self.action_file_save();
                if saved {
                    if self.state.schematic.is_dirty || self.state.workspace.any_dirty() {
                        self.state.push_user_message(
                            crate::common::app::ConsoleMessage::warning(
                                "Downloaded a copy, but unsaved changes remain open. Confirm the browser accepted the download, then choose Discard to continue without them."
                            ),
                        );
                        self.state.dialogs.confirmation_dialog.visible = pending.is_some();
                        self.state.dialogs.confirmation_dialog.pending_action = pending;
                        self.state.dialogs.confirmation_dialog.pending_path = pending_path;
                        self.state.dialogs.confirmation_dialog.pending_example = pending_example;
                    } else if let Some(action) = pending {
                        self.execute_pending_action(action, pending_path, pending_example);
                    }
                }
            }
        }
    }

    /// Execute a pending action after confirmation dialog
    pub(super) fn execute_pending_action(
        &mut self,
        action: ConfirmationAction,
        path: Option<std::path::PathBuf>,
        example: Option<String>,
    ) {
        self.execute_pending_action_with_project_open(
            action,
            path,
            example,
            crate::common::project_workflow::open_project,
        );
    }

    fn execute_pending_action_with_project_open(
        &mut self,
        action: ConfirmationAction,
        path: Option<std::path::PathBuf>,
        example: Option<String>,
        open_project: impl FnOnce(&mut crate::common::app::AppState) -> bool,
    ) {
        match action {
            ConfirmationAction::ProjectNew => {
                crate::common::project_workflow::create_new_project(&mut self.state);
            }
            ConfirmationAction::ProjectOpen => {
                let opened = open_project(&mut self.state);
                #[cfg(target_arch = "wasm32")]
                let _ = opened;
                #[cfg(not(target_arch = "wasm32"))]
                if opened {
                    self.restore_workspace_after_project_load();
                }
            }
            ConfirmationAction::FileNew => self.do_file_new(),
            ConfirmationAction::FileOpen => self.do_file_open(),
            ConfirmationAction::OpenRecent => {
                if let Some(path) = path {
                    self.do_open_recent(path);
                }
            }
            ConfirmationAction::OpenExample => {
                if let Some(name) = example
                    && crate::common::menu_bar::load_named_example(&mut self.state, &name)
                {
                    self.state.shell.view = crate::shell::WorkspaceView::Schematic;
                }
            }
            ConfirmationAction::ImportNetlist => {
                crate::common::netlist_workflow::import_netlist(&mut self.state);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::export_workflow::{ExportWorkflowIo, SaveDialogConfig};
    use crate::common::file_workflow::FileWorkflowIo;
    use crate::io::{SchematicIoError, WaveformDataset};
    use crate::state::{Component, ComponentType, Point, SchematicState};
    use std::cell::RefCell;
    use std::path::{Path, PathBuf};
    use std::rc::Rc;

    #[derive(Debug)]
    struct TestFileWorkflowIo {
        saved_paths: Rc<RefCell<Vec<PathBuf>>>,
        saved_paths_are_reopenable: bool,
    }

    impl FileWorkflowIo for TestFileWorkflowIo {
        #[cfg(not(target_arch = "wasm32"))]
        fn show_open_dialog(&self) -> Result<PathBuf, SchematicIoError> {
            Err(SchematicIoError::Cancelled)
        }

        fn show_save_dialog(
            &self,
            _default_name: Option<&str>,
        ) -> Result<PathBuf, SchematicIoError> {
            Err(SchematicIoError::Cancelled)
        }

        fn load_schematic(&self, _path: &Path) -> Result<SchematicState, SchematicIoError> {
            Err(SchematicIoError::Cancelled)
        }

        fn save_schematic(
            &self,
            _schematic: &SchematicState,
            path: &Path,
        ) -> Result<(), SchematicIoError> {
            self.saved_paths.borrow_mut().push(path.to_path_buf());
            Ok(())
        }

        fn saved_paths_are_reopenable(&self) -> bool {
            self.saved_paths_are_reopenable
        }
    }

    #[derive(Debug, Default)]
    struct TestExportWorkflowIo;

    impl ExportWorkflowIo for TestExportWorkflowIo {
        fn show_save_dialog(
            &self,
            _config: SaveDialogConfig<'_>,
        ) -> Result<Option<PathBuf>, String> {
            Ok(None)
        }

        fn write_text_file(&self, _path: &Path, _contents: &str) -> Result<(), String> {
            Ok(())
        }

        fn write_waveform_csv(
            &self,
            _dataset: &WaveformDataset,
            _path: &Path,
        ) -> Result<(), String> {
            Ok(())
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn test_app_with_file_io(
        state: crate::common::app::AppState,
        file_io: TestFileWorkflowIo,
    ) -> RSpiceApp {
        RSpiceApp {
            state,
            first_frame: false,
            autosave_last: None,
            applied_theme: None,
            last_window_title: String::new(),
            symbol_library: None,
            simulation_controller: crate::simulation::SimulationController::new(),
            file_workflow_io: Box::new(file_io),
            export_workflow_io: Box::new(TestExportWorkflowIo),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn unique_temp_path(prefix: &str, extension: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "{prefix}-{}-{unique}.{extension}",
            std::process::id()
        ))
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn project_confirmation_yes_saves_dirty_standalone_schematic_to_its_file() {
        let schematic_path = unique_temp_path("rspice-standalone-confirm-save", "rsch");
        let project_path = unique_temp_path("rspice-project-confirm-save", "rspiceproj");
        let saved_paths = Rc::new(RefCell::new(Vec::new()));
        let mut state = crate::common::app::AppState::default();
        state.schematic.current_file = Some(schematic_path.clone());
        state.schematic.is_dirty = true;
        state.workspace.project.set_path(project_path.clone());
        state
            .dialogs
            .confirmation_dialog
            .show(ConfirmationAction::ProjectNew);
        let file_io = TestFileWorkflowIo {
            saved_paths: Rc::clone(&saved_paths),
            saved_paths_are_reopenable: true,
        };
        let mut app = test_app_with_file_io(state, file_io);

        app.handle_confirmation_response(ConfirmationResponse::Yes);
        let _ = std::fs::remove_file(&project_path);

        assert_eq!(saved_paths.borrow().as_slice(), [schematic_path.as_path()]);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn cancelled_project_open_after_discard_confirmation_keeps_live_schematic() {
        let mut state = crate::common::app::AppState::default();
        state.schematic.components.push(Component::new(
            42,
            ComponentType::Resistor,
            Point::new(10, 20),
        ));
        state.schematic.is_dirty = true;
        let active_key = state.workspace.active_key();
        assert!(
            state
                .workspace
                .schematic_buffers
                .get(&active_key)
                .is_none_or(|schematic| schematic.components.is_empty()),
            "test requires unsynced live schematic state"
        );

        let mut app = test_app_with_file_io(
            state,
            TestFileWorkflowIo {
                saved_paths: Rc::new(RefCell::new(Vec::new())),
                saved_paths_are_reopenable: true,
            },
        );

        app.execute_pending_action_with_project_open(
            ConfirmationAction::ProjectOpen,
            None,
            None,
            |_state| false,
        );

        assert_eq!(app.state.schematic.components.len(), 1);
        assert_eq!(app.state.schematic.components[0].id, 42);
        assert!(app.state.schematic.is_dirty);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn confirmation_yes_after_download_only_save_keeps_dirty_document_and_pending_action() {
        let schematic_path = PathBuf::from("browser-copy.rsch");
        let saved_paths = Rc::new(RefCell::new(Vec::new()));
        let mut state = crate::common::app::AppState::default();
        state.schematic.current_file = Some(schematic_path.clone());
        state.schematic.components.push(Component::new(
            77,
            ComponentType::Resistor,
            Point::new(10, 20),
        ));
        state.schematic.is_dirty = true;
        state
            .dialogs
            .confirmation_dialog
            .show(ConfirmationAction::FileNew);

        let mut app = test_app_with_file_io(
            state,
            TestFileWorkflowIo {
                saved_paths: Rc::clone(&saved_paths),
                saved_paths_are_reopenable: false,
            },
        );

        app.handle_confirmation_response(ConfirmationResponse::Yes);

        assert_eq!(saved_paths.borrow().as_slice(), [schematic_path.as_path()]);
        assert_eq!(app.state.schematic.components.len(), 1);
        assert_eq!(app.state.schematic.components[0].id, 77);
        assert!(app.state.schematic.is_dirty);
        assert!(app.state.dialogs.confirmation_dialog.visible);
        assert_eq!(
            app.state.dialogs.confirmation_dialog.pending_action,
            Some(ConfirmationAction::FileNew)
        );
    }
}

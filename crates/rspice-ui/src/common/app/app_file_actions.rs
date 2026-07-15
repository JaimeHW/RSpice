use super::{ConfirmationAction, ConfirmationResponse, RSpiceApp};

impl RSpiceApp {
    /// Internal: Actually create a new schematic (after confirmation)
    pub(super) fn do_file_new(&mut self) {
        crate::common::file_actions::action_file_new(&mut self.state);
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
        let pending_recent_kind = self
            .state
            .dialogs
            .confirmation_dialog
            .pending_recent_kind
            .take();
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
                    self.execute_pending_action(
                        action,
                        pending_path,
                        pending_recent_kind,
                        pending_example,
                    );
                }
            }
            ConfirmationResponse::Yes => {
                // Save first, then proceed
                let project_action = matches!(
                    pending,
                    Some(
                        ConfirmationAction::ProjectNew
                            | ConfirmationAction::ProjectOpen
                            | ConfirmationAction::CloseProject
                            | ConfirmationAction::Exit
                    )
                ) || matches!(
                    (pending, pending_recent_kind),
                    (
                        Some(ConfirmationAction::OpenRecent),
                        Some(crate::common::app::RecentKind::Project)
                    )
                );
                let outcome = if project_action {
                    crate::common::project_workflow::save_all_for_continuation(&mut self.state)
                } else if self.state.project_lifecycle.project_open {
                    crate::common::project_workflow::save_active_for_continuation(&mut self.state)
                } else if self.action_file_save() {
                    if self.state.schematic.is_dirty {
                        crate::common::project_workflow::SaveRequestOutcome::CopyOnly
                    } else {
                        crate::common::project_workflow::SaveRequestOutcome::CanonicalComplete
                    }
                } else {
                    crate::common::project_workflow::SaveRequestOutcome::CancelledOrFailed
                };
                if outcome.authorizes_immediate_destructive_action() {
                    if let Some(action) = pending {
                        self.execute_pending_action(
                            action,
                            pending_path,
                            pending_recent_kind,
                            pending_example,
                        );
                    }
                    return;
                }
                match outcome {
                    crate::common::project_workflow::SaveRequestOutcome::CanonicalPending(
                        transaction,
                    ) => {
                        if let Some(action) = pending {
                            self.state.dialogs.confirmation_dialog.await_canonical_save(
                                transaction,
                                action,
                                pending_path,
                                pending_recent_kind,
                                pending_example,
                            );
                        }
                    }
                    crate::common::project_workflow::SaveRequestOutcome::CopyOnly => {
                        self.state.push_user_message(
                            crate::common::app::ConsoleMessage::warning(
                                "Downloaded a copy, but no canonical save completed. The pending action was not authorized and unsaved work remains open."
                            ),
                        );
                        self.state.dialogs.confirmation_dialog.visible = pending.is_some();
                        self.state.dialogs.confirmation_dialog.pending_action = pending;
                        self.state.dialogs.confirmation_dialog.pending_path = pending_path;
                        self.state.dialogs.confirmation_dialog.pending_recent_kind =
                            pending_recent_kind;
                        self.state.dialogs.confirmation_dialog.pending_example = pending_example;
                    }
                    crate::common::project_workflow::SaveRequestOutcome::CopyPending
                    | crate::common::project_workflow::SaveRequestOutcome::CancelledOrFailed => {}
                    crate::common::project_workflow::SaveRequestOutcome::CanonicalComplete => {
                        unreachable!("canonical completion returned through the authorization gate")
                    }
                }
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(super) fn handle_save_continuation_event(
        &mut self,
        event: crate::common::project_workflow::SaveContinuationEvent,
    ) {
        let Some(pending) = self
            .state
            .dialogs
            .confirmation_dialog
            .take_canonical_save(event.transaction())
        else {
            return;
        };
        if event.authorizes_destructive_action() {
            self.execute_pending_action(
                pending.action,
                pending.path,
                pending.recent_kind,
                pending.example,
            );
        } else if event.needs_another_save() {
            self.state
                .dialogs
                .confirmation_dialog
                .restore_continuation_for_review(pending);
            self.state.push_user_message(
                crate::common::app::ConsoleMessage::warning(
                    "The requested snapshot was saved, but newer edits were made while the browser file surface was pending. Review and save those edits before continuing.",
                ),
            );
        }
    }

    pub(super) fn begin_close_project_after_save(&mut self) {
        match crate::common::project_workflow::save_all_for_continuation(&mut self.state) {
            crate::common::project_workflow::SaveRequestOutcome::CanonicalComplete => {
                self.state.dialogs.project_review_dialog.close();
                crate::common::project_workflow::close_project_discard(&mut self.state);
            }
            crate::common::project_workflow::SaveRequestOutcome::CanonicalPending(transaction) => {
                self.state.dialogs.project_review_dialog.close();
                self.state.dialogs.confirmation_dialog.await_canonical_save(
                    transaction,
                    ConfirmationAction::CloseProject,
                    None,
                    None,
                    None,
                );
            }
            crate::common::project_workflow::SaveRequestOutcome::CopyOnly => {
                self.state.push_user_message(
                    crate::common::app::ConsoleMessage::warning(
                        "A project copy was downloaded, but canonical Save all did not complete. The project remains open with its working changes.",
                    ),
                );
            }
            crate::common::project_workflow::SaveRequestOutcome::CopyPending
            | crate::common::project_workflow::SaveRequestOutcome::CancelledOrFailed => {}
        }
    }

    /// Execute a pending action after confirmation dialog
    pub(super) fn execute_pending_action(
        &mut self,
        action: ConfirmationAction,
        path: Option<std::path::PathBuf>,
        recent_kind: Option<crate::common::app::RecentKind>,
        example: Option<String>,
    ) {
        self.execute_pending_action_with_project_open(
            action,
            path,
            recent_kind,
            example,
            crate::common::project_workflow::open_project,
        );
    }

    fn execute_pending_action_with_project_open(
        &mut self,
        action: ConfirmationAction,
        path: Option<std::path::PathBuf>,
        recent_kind: Option<crate::common::app::RecentKind>,
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
            ConfirmationAction::CloseProject => {
                crate::common::project_workflow::close_project_discard(&mut self.state);
            }
            ConfirmationAction::FileNew => self.do_file_new(),
            ConfirmationAction::FileOpen => self.do_file_open(),
            ConfirmationAction::OpenRecent => {
                if let (Some(path), Some(kind)) = (path, recent_kind) {
                    self.do_open_recent(path, kind);
                }
            }
            ConfirmationAction::OpenExample => {
                if let Some(name) = example
                    && crate::common::menu_bar::load_named_example(&mut self.state, &name)
                {
                    self.state
                        .workbench
                        .activate(crate::workbench::state::Workspace::Design);
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
        if crate::common::project_lifecycle::has_unsaved_changes(&self.state) {
            self.state
                .dialogs
                .confirmation_dialog
                .show_recent(recent.path, recent.kind);
        } else {
            self.do_open_recent(recent.path, recent.kind);
        }
    }

    /// Internal: actually open a recent file (after any confirmation).
    /// Entries whose file vanished are dropped from the list with a console
    /// note instead of failing silently.
    fn do_open_recent(&mut self, path: std::path::PathBuf, kind: crate::common::app::RecentKind) {
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

        let opened = match kind {
            RecentKind::Project => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let opened = crate::common::project_workflow::load_project_from_path(
                        &mut self.state,
                        &path,
                    );
                    if opened {
                        self.restore_workspace_after_project_load();
                    }
                    opened
                }
                #[cfg(target_arch = "wasm32")]
                {
                    self.state.push_user_message(ConsoleMessage::warning(
                        "Browser projects must be reopened through Open project so file permission can be verified",
                    ));
                    false
                }
            }
            RecentKind::Schematic => {
                let (state, io) = (&mut self.state, self.file_workflow_io.as_ref());
                crate::common::file_workflow::load_schematic_from_path_with_io(state, &path, io)
            }
        };

        if opened {
            self.state
                .workbench
                .activate(crate::workbench::state::Workspace::Design);
        }
    }

    pub(super) fn action_file_save(&mut self) -> bool {
        if self.state.project_lifecycle.project_open {
            return crate::common::project_workflow::save_project(&mut self.state);
        }
        let (state, io) = (&mut self.state, self.file_workflow_io.as_ref());
        crate::common::file_actions::action_file_save_with_io(state, io)
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use crate::common::export_workflow::{ExportWorkflowIo, SaveDialogConfig};
    use crate::common::file_workflow::FileWorkflowIo;
    use crate::io::{SchematicIoError, WaveformDataset};
    use crate::simulation::plan::AnalysisKind;
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

    fn insert_ac_analysis(
        state: &mut crate::common::app::AppState,
    ) -> crate::product::AnalysisInstanceId {
        state
            .sim_setup
            .analysis_plan
            .as_mut()
            .expect("current project owns a stable plan")
            .insert(AnalysisKind::Ac)
            .expect("AC analysis inserts")
            .0
    }

    fn has_ac_analysis(setup: &crate::common::app::SimSetupState) -> bool {
        setup
            .stable_analysis_plan()
            .expect("current project owns a stable plan")
            .instances()
            .iter()
            .any(|instance| instance.kind() == AnalysisKind::Ac)
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
    fn remove_project_artifacts(path: &Path) {
        let _ = std::fs::remove_file(path);
        let _ = std::fs::remove_file(path.with_extension("rspiceproj.bak"));
        let mut lock = path.as_os_str().to_os_string();
        lock.push(".rspice.lock");
        let _ = std::fs::remove_file(PathBuf::from(lock));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn project_save_never_falls_through_to_legacy_schematic_io_in_any_workspace() {
        use crate::common::project_lifecycle::{DestinationAuthority, SaveScope};
        use crate::workbench::state::Workspace;

        for workspace in [
            Workspace::Project,
            Workspace::Design,
            Workspace::Simulate,
            Workspace::Results,
            Workspace::Verify,
            Workspace::Models,
            Workspace::Netlist,
        ] {
            let project_path = unique_temp_path("rspice-project-save-routing", "rspiceproj");
            let schematic_path = unique_temp_path("rspice-legacy-save-routing", "rsch");
            let saved_paths = Rc::new(RefCell::new(Vec::new()));
            let mut state = crate::common::app::AppState::default();
            crate::common::project_lifecycle::save_native(
                &mut state,
                SaveScope::AllDocuments,
                &project_path,
                DestinationAuthority::UserSelected,
            )
            .expect("seed canonical project");
            state.workbench.activate(workspace);
            state.schematic.current_file = Some(schematic_path);
            state.schematic.is_dirty = true;
            state.workspace.set_active_dirty(true);
            let mut app = test_app_with_file_io(
                state,
                TestFileWorkflowIo {
                    saved_paths: Rc::clone(&saved_paths),
                    saved_paths_are_reopenable: true,
                },
            );

            assert!(
                app.action_file_save(),
                "lifecycle save failed in {workspace:?}"
            );
            assert!(
                saved_paths.borrow().is_empty(),
                "project Save fell through to standalone schematic I/O in {workspace:?}"
            );

            remove_project_artifacts(&project_path);
        }
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
        state.project_lifecycle.project_open = false;
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

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn recent_kind_is_preserved_and_selects_project_or_schematic_save_scope() {
        use crate::common::project_lifecycle::{DestinationAuthority, SaveScope};

        let source = unique_temp_path("rspice-recent-source", "rspiceproj");
        let target_project = unique_temp_path("rspice-recent-target", "rspiceproj");
        let target_schematic = unique_temp_path("rspice-recent-target", "rsch");

        let mut target_state = crate::common::app::AppState::default();
        target_state
            .workspace
            .project
            .rename("Recent target project")
            .expect("valid target name");
        crate::common::project_lifecycle::save_native(
            &mut target_state,
            SaveScope::AllDocuments,
            &target_project,
            DestinationAuthority::UserSelected,
        )
        .expect("create recent target project");

        let mut project_source = crate::common::app::AppState::default();
        project_source.workbench.workspace = crate::workbench::state::Workspace::Design;
        crate::common::project_lifecycle::save_native(
            &mut project_source,
            SaveScope::AllDocuments,
            &source,
            DestinationAuthority::UserSelected,
        )
        .expect("create source project");
        let active_key = project_source.workspace.active_key();
        project_source
            .schematic
            .add_component(ComponentType::Resistor, Point::new(7, 3));
        let project_ac_id = insert_ac_analysis(&mut project_source);
        project_source
            .remember_recent_file(crate::common::app::RecentKind::Project, &target_project);
        let project_recent = project_source
            .recent_files
            .iter()
            .find(|recent| recent.path == target_project)
            .cloned()
            .expect("typed project recent entry");
        let mut project_app = test_app_with_file_io(
            project_source,
            TestFileWorkflowIo {
                saved_paths: Rc::new(RefCell::new(Vec::new())),
                saved_paths_are_reopenable: true,
            },
        );

        project_app.open_recent_file(project_recent);
        assert_eq!(
            project_app
                .state
                .dialogs
                .confirmation_dialog
                .pending_recent_kind,
            Some(crate::common::app::RecentKind::Project)
        );
        project_app.handle_confirmation_response(ConfirmationResponse::Yes);

        let saved_source = crate::io::load_project_file(&source).expect("reload Save All source");
        assert_eq!(
            saved_source
                .workspace
                .schematic_buffers
                .get(&active_key)
                .expect("saved active design")
                .components
                .len(),
            1
        );
        assert_eq!(
            saved_source
                .execution_context
                .expect("saved project execution context")
                .simulation_plan
                .stable_analysis_plan()
                .expect("saved project owns a stable plan")
                .instance(project_ac_id)
                .expect("Save All retains the exact AC identity")
                .kind(),
            AnalysisKind::Ac,
            "opening a recent project must Save All before replacement"
        );
        assert_eq!(
            project_app.state.workspace.project.name(),
            "Recent target project"
        );

        std::fs::write(&target_schematic, b"placeholder for test I/O")
            .expect("create schematic recent path");
        let mut schematic_source = crate::common::app::AppState::default();
        schematic_source.workbench.workspace = crate::workbench::state::Workspace::Design;
        crate::common::project_lifecycle::save_native(
            &mut schematic_source,
            SaveScope::AllDocuments,
            &source,
            DestinationAuthority::UserSelected,
        )
        .expect("reset source baseline");
        schematic_source
            .schematic
            .add_component(ComponentType::Capacitor, Point::new(8, 4));
        let schematic_ac_id = insert_ac_analysis(&mut schematic_source);
        schematic_source
            .remember_recent_file(crate::common::app::RecentKind::Schematic, &target_schematic);
        let schematic_recent = schematic_source
            .recent_files
            .iter()
            .find(|recent| recent.path == target_schematic)
            .cloned()
            .expect("typed schematic recent entry");
        let mut schematic_app = test_app_with_file_io(
            schematic_source,
            TestFileWorkflowIo {
                saved_paths: Rc::new(RefCell::new(Vec::new())),
                saved_paths_are_reopenable: true,
            },
        );

        schematic_app.open_recent_file(schematic_recent);
        assert_eq!(
            schematic_app
                .state
                .dialogs
                .confirmation_dialog
                .pending_recent_kind,
            Some(crate::common::app::RecentKind::Schematic)
        );
        schematic_app.handle_confirmation_response(ConfirmationResponse::Yes);

        let active_only = crate::io::load_project_file(&source).expect("reload active save");
        assert_eq!(
            active_only
                .workspace
                .schematic_buffers
                .get(&schematic_app.state.workspace.active_key())
                .expect("saved active schematic")
                .components
                .len(),
            1
        );
        assert!(
            !has_ac_analysis(
                &active_only
                    .execution_context
                    .expect("active-save execution context")
                    .simulation_plan
            ),
            "opening a recent schematic saves only the active design document"
        );
        assert_eq!(
            schematic_app
                .state
                .sim_setup
                .stable_analysis_plan()
                .expect("live plan retained")
                .instance(schematic_ac_id)
                .expect("AC identity retained")
                .kind(),
            AnalysisKind::Ac
        );

        remove_project_artifacts(&source);
        remove_project_artifacts(&target_project);
        let _ = std::fs::remove_file(target_schematic);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn close_project_review_counts_documents_and_active_run_blocks_discard() {
        use crate::common::project_lifecycle::{DestinationAuthority, SaveScope};

        let path = unique_temp_path("rspice-close-review", "rspiceproj");
        let mut state = crate::common::app::AppState::default();
        crate::common::project_lifecycle::save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .expect("seed canonical project");
        assert_eq!(
            crate::common::project_lifecycle::dirty_document_count(&state),
            0
        );
        assert!(crate::common::project_workflow::request_close_project(
            &mut state
        ));
        assert!(matches!(
            state.dialogs.project_review_dialog.request.as_ref(),
            Some(crate::common::app::ProjectReviewRequest::CloseProject)
        ));
        state.dialogs.project_review_dialog.close();

        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(2, 4));
        insert_ac_analysis(&mut state);
        assert_eq!(
            crate::common::project_lifecycle::dirty_document_count(&state),
            2
        );
        assert!(crate::common::project_workflow::request_close_project(
            &mut state
        ));
        assert!(matches!(
            state.dialogs.project_review_dialog.request.as_ref(),
            Some(crate::common::app::ProjectReviewRequest::CloseProject)
        ));

        let project_id = state.workspace.project.id();
        state.simulation.is_running = true;
        assert!(!crate::common::project_workflow::close_project_discard(
            &mut state
        ));
        assert!(state.project_lifecycle.project_open);
        assert_eq!(state.workspace.project.id(), project_id);
        assert!(matches!(
            state.dialogs.project_review_dialog.request.as_ref(),
            Some(crate::common::app::ProjectReviewRequest::CloseProject)
        ));

        // Cancel closes only the review; every project-owned draft remains.
        state.dialogs.project_review_dialog.close();
        assert!(state.dialogs.project_review_dialog.request.is_none());
        assert!(state.project_lifecycle.project_open);
        assert_eq!(
            crate::common::project_lifecycle::dirty_document_count(&state),
            2
        );
        remove_project_artifacts(&path);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn save_all_and_close_publishes_every_dirty_document_before_closing() {
        use crate::common::project_lifecycle::{DestinationAuthority, SaveScope};

        let path = unique_temp_path("rspice-save-all-close", "rspiceproj");
        let saved_paths = Rc::new(RefCell::new(Vec::new()));
        let mut state = crate::common::app::AppState::default();
        crate::common::project_lifecycle::save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .expect("seed canonical project");
        let active_key = state.workspace.active_key();
        state
            .schematic
            .add_component(ComponentType::Capacitor, Point::new(3, 8));
        let close_ac_id = insert_ac_analysis(&mut state);
        crate::common::project_workflow::request_close_project(&mut state);
        let mut app = test_app_with_file_io(
            state,
            TestFileWorkflowIo {
                saved_paths: Rc::clone(&saved_paths),
                saved_paths_are_reopenable: true,
            },
        );

        app.begin_close_project_after_save();

        assert!(!app.state.project_lifecycle.project_open);
        assert!(app.state.dialogs.project_review_dialog.request.is_none());
        assert!(saved_paths.borrow().is_empty());
        let persisted = crate::io::load_project_file(&path).expect("reload closed project");
        assert_eq!(
            persisted
                .workspace
                .schematic_buffers
                .get(&active_key)
                .expect("saved active schematic")
                .components
                .len(),
            1
        );
        assert_eq!(
            persisted
                .execution_context
                .expect("saved execution context")
                .simulation_plan
                .stable_analysis_plan()
                .expect("saved project owns a stable plan")
                .instance(close_ac_id)
                .expect("save-and-close retains the exact AC identity")
                .kind(),
            AnalysisKind::Ac
        );
        remove_project_artifacts(&path);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn external_change_prevents_save_and_close_and_retains_review_and_work() {
        use crate::common::project_lifecycle::{DestinationAuthority, SaveScope};

        let path = unique_temp_path("rspice-close-external-change", "rspiceproj");
        let mut state = crate::common::app::AppState::default();
        crate::common::project_lifecycle::save_native(
            &mut state,
            SaveScope::AllDocuments,
            &path,
            DestinationAuthority::UserSelected,
        )
        .expect("seed canonical project");
        state
            .schematic
            .add_component(ComponentType::Inductor, Point::new(6, 10));
        crate::common::project_workflow::request_close_project(&mut state);
        std::fs::write(&path, b"external owner replaced this project snapshot")
            .expect("simulate external replacement");
        let mut app = test_app_with_file_io(
            state,
            TestFileWorkflowIo {
                saved_paths: Rc::new(RefCell::new(Vec::new())),
                saved_paths_are_reopenable: true,
            },
        );

        app.begin_close_project_after_save();

        assert!(app.state.project_lifecycle.project_open);
        assert_eq!(app.state.schematic.components.len(), 1);
        assert!(matches!(
            app.state.dialogs.project_review_dialog.request.as_ref(),
            Some(crate::common::app::ProjectReviewRequest::CloseProject)
        ));
        assert!(crate::common::project_lifecycle::has_unsaved_changes(
            &app.state
        ));
        remove_project_artifacts(&path);
    }
}

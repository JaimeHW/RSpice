//! File action layer: dirty-state confirmations, project/file IO dispatch,
//! exports and application exit. Rendering lives in the workbench title bar.

use crate::workbench::app::ConfirmationAction;
use crate::workbench::app_state::AppState;
use crate::workbench::workflows::export_workflow::ExportWorkflowIo;
use crate::workbench::workflows::file_workflow::FileWorkflowIo;

/// Every action reachable from the File menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FileMenuAction {
    NewProject,
    OpenProject,
    SaveProjectAs,
    SaveAll,
    RevertActiveDocument,
    CloseActiveDocument,
    CloseProject,
    Open,
    Save,
    OpenNetlist,
    ImportNetlist,
    ExportSvg,
    ExportCsvWaveforms,
    ExportPublicationSnapshot,
    ImportVerilogA,
    Exit,
}

pub(crate) fn dispatch_file_menu_action(
    state: &mut AppState,
    action: FileMenuAction,
    file_workflow_io: &(impl FileWorkflowIo + ?Sized),
    export_workflow_io: &(impl ExportWorkflowIo + ?Sized),
) {
    if !state.commit_pending_inspector_edit() {
        return;
    }
    match action {
        FileMenuAction::NewProject => {
            if require_project_save_confirmation_if_dirty(state, ConfirmationAction::ProjectNew) {
                return;
            }
            crate::workbench::app::open_new_project_dialog(state);
        }
        FileMenuAction::OpenProject => {
            if require_project_save_confirmation_if_dirty(state, ConfirmationAction::ProjectOpen) {
                return;
            }
            crate::workbench::workflows::project_workflow::open_project(state);
        }
        FileMenuAction::SaveProjectAs => {
            crate::workbench::workflows::project_workflow::save_project_as(state);
        }
        FileMenuAction::SaveAll => {
            crate::workbench::workflows::project_workflow::save_all(state);
        }
        FileMenuAction::RevertActiveDocument => {
            crate::workbench::workflows::project_workflow::request_revert_active_document(state);
        }
        FileMenuAction::CloseActiveDocument => {
            crate::workbench::workflows::project_workflow::close_active_document(state);
        }
        FileMenuAction::CloseProject => {
            crate::workbench::workflows::project_workflow::request_close_project(state);
        }
        FileMenuAction::Open => {
            if require_save_confirmation_if_dirty(state, ConfirmationAction::FileOpen) {
                return;
            }
            crate::workbench::workflows::file_actions::action_file_open_with_io(
                state,
                file_workflow_io,
            );
        }
        FileMenuAction::Save => {
            if state.project_lifecycle.project_open {
                let _ = crate::workbench::workflows::project_workflow::save_project(state);
            } else {
                let _ = crate::workbench::workflows::file_actions::action_file_save_with_io(
                    state,
                    file_workflow_io,
                );
            }
        }
        FileMenuAction::OpenNetlist => {
            if require_project_save_confirmation_if_dirty(
                state,
                ConfirmationAction::OpenNetlistProject,
            ) {
                return;
            }
            crate::workbench::workflows::netlist_workflow::open_netlist_project(state);
        }
        FileMenuAction::ImportNetlist => {
            if require_netlist_source_save_confirmation_if_dirty(
                state,
                ConfirmationAction::ImportNetlist,
            ) {
                return;
            }
            crate::workbench::workflows::netlist_workflow::import_netlist(state);
        }
        FileMenuAction::ExportSvg => {
            super::export_actions::action_export_svg_with_io(state, export_workflow_io)
        }
        FileMenuAction::ExportCsvWaveforms => {
            super::waveform_export::action_export_csv_with_io(state, export_workflow_io)
        }
        FileMenuAction::ExportPublicationSnapshot => {
            super::export_actions::action_export_publication_snapshot_with_io(
                state,
                export_workflow_io,
            )
        }
        FileMenuAction::ImportVerilogA => {
            state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
            state.ui.code_workspace.page =
                crate::workbench::documents::code_workspace::CodeWorkspacePage::VerilogA;
            state.ui.code_workspace.veriloga.import_requested = true;
        }
        FileMenuAction::Exit => request_exit(state),
    }
}

fn require_save_confirmation_if_dirty(state: &mut AppState, action: ConfirmationAction) -> bool {
    if !crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(state) {
        return false;
    }

    state.dialogs.confirmation_dialog.show(action);
    true
}

fn request_exit(state: &mut AppState) {
    if state.workbench.has_unsaved_authoring_changes() {
        state
            .dialogs
            .confirmation_dialog
            .show(ConfirmationAction::Exit);
    } else if !require_save_confirmation_if_dirty(state, ConfirmationAction::Exit) {
        state.exit_requested = true;
    }
}

fn require_netlist_source_save_confirmation_if_dirty(
    state: &mut AppState,
    action: ConfirmationAction,
) -> bool {
    if !state.workspace.netlist_source_dirty {
        return false;
    }

    state.dialogs.confirmation_dialog.show(action);
    true
}

fn require_project_save_confirmation_if_dirty(
    state: &mut AppState,
    action: ConfirmationAction,
) -> bool {
    if !crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(state)
        && !state.workbench.model_editor_has_unsaved_changes()
    {
        return false;
    }

    state.dialogs.confirmation_dialog.show(action);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn netlist_import_only_prompts_for_source_bytes_it_will_replace() {
        let mut state = AppState::default();
        state.schematic.is_dirty = true;

        assert!(!require_netlist_source_save_confirmation_if_dirty(
            &mut state,
            ConfirmationAction::ImportNetlist,
        ));
        assert!(!state.dialogs.confirmation_dialog.visible);

        state.workspace.set_netlist_source_dirty(true);
        assert!(require_netlist_source_save_confirmation_if_dirty(
            &mut state,
            ConfirmationAction::ImportNetlist,
        ));
        assert_eq!(
            state.dialogs.confirmation_dialog.pending_action,
            Some(ConfirmationAction::ImportNetlist)
        );
    }
    struct NoIo;

    impl crate::workbench::workflows::file_workflow::FileWorkflowIo for NoIo {
        #[cfg(not(target_arch = "wasm32"))]
        fn show_open_dialog(&self) -> Result<std::path::PathBuf, crate::io::SchematicIoError> {
            panic!("unexpected file I/O")
        }
        fn show_save_dialog(
            &self,
            _: Option<&str>,
        ) -> Result<std::path::PathBuf, crate::io::SchematicIoError> {
            panic!("unexpected file I/O")
        }
        fn load_schematic(
            &self,
            _: &std::path::Path,
        ) -> Result<crate::state::SchematicState, crate::io::SchematicIoError> {
            panic!("unexpected file I/O")
        }
        fn save_schematic(
            &self,
            _: &crate::state::SchematicState,
            _: &std::path::Path,
        ) -> Result<(), crate::io::SchematicIoError> {
            panic!("unexpected file I/O")
        }
    }

    impl crate::workbench::workflows::export_workflow::ExportWorkflowIo for NoIo {
        fn show_save_dialog(
            &self,
            _: crate::workbench::workflows::export_workflow::SaveDialogConfig<'_>,
        ) -> Result<Option<std::path::PathBuf>, String> {
            panic!("unexpected export I/O")
        }
        fn write_text_file(&self, _: &std::path::Path, _: &str) -> Result<(), String> {
            panic!("unexpected export I/O")
        }
        fn write_waveform_csv(
            &self,
            _: &crate::io::WaveformDataset,
            _: &std::path::Path,
        ) -> Result<(), String> {
            panic!("unexpected export I/O")
        }
    }

    #[test]
    fn file_actions_resolve_the_draft_before_saving_or_asking_about_unsaved_changes() {
        use crate::workbench::state::{InlineEditField, InlineEditSession};
        let mut state = AppState::default();
        state.schematic.components.clear();
        state.schematic.add_component(
            crate::state::ComponentType::VoltageSource,
            crate::state::Point::origin(),
        );
        state.schematic.clear_undo_history();
        state.schematic.is_dirty = false;
        let expected = state.schematic.components[0].clone();
        let mut candidate = expected.clone();
        candidate.name = "V9".to_owned();
        let authority = state.inline_edit_authority();
        state.workbench.inline_edit.begin(InlineEditSession {
            expected,
            field: InlineEditField::Instance,
            authority,
            description: "rename instance".to_owned(),
            buffer: "invalid name".to_owned(),
            candidate: None,
            error: Some("Invalid instance designator".to_owned()),
            widget: None,
        });
        // Keep a regressed dispatch on the injectable standalone-file backend;
        // a failed assertion must never open a native project-save picker.
        state.project_lifecycle.project_open = false;
        dispatch_file_menu_action(&mut state, FileMenuAction::Save, &NoIo, &NoIo);
        assert_eq!(state.schematic.components[0].name, "V1");
        assert!(
            state
                .workbench
                .inline_edit
                .session()
                .unwrap()
                .error
                .is_some()
        );
        assert!(!state.schematic.can_undo());
        assert!(state.project_undo_sequence().is_none());
        state.project_lifecycle.project_open = true;
        state
            .workbench
            .inline_edit
            .set_draft("V9".to_owned(), Ok(candidate));
        dispatch_file_menu_action(&mut state, FileMenuAction::NewProject, &NoIo, &NoIo);
        assert_eq!(state.schematic.components[0].name, "V9");
        assert!(state.workbench.inline_edit.session().is_none());
        assert_eq!(
            state.dialogs.confirmation_dialog.pending_action,
            Some(ConfirmationAction::ProjectNew)
        );
    }
}

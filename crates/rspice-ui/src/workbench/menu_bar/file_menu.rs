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
    match action {
        FileMenuAction::NewProject => {
            if require_project_save_confirmation_if_dirty(state, ConfirmationAction::ProjectNew) {
                return;
            }
            crate::workbench::workflows::project_workflow::create_new_project(state);
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
    if !require_save_confirmation_if_dirty(state, ConfirmationAction::Exit) {
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
    if !crate::workbench::lifecycle::project_lifecycle::has_unsaved_changes(state) {
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
}

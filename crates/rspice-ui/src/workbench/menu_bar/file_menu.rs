//! File action layer: dirty-state confirmations, project/file IO dispatch,
//! exports and application exit. Rendering lives in the workbench title bar.

use crate::workbench::app::{AppState, ConfirmationAction};
use crate::workbench::export_workflow::ExportWorkflowIo;
use crate::workbench::file_workflow::FileWorkflowIo;

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
    ImportNetlist,
    ExportSvg,
    ExportCsvWaveforms,
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
            crate::workbench::project_workflow::create_new_project(state);
        }
        FileMenuAction::OpenProject => {
            if require_project_save_confirmation_if_dirty(state, ConfirmationAction::ProjectOpen) {
                return;
            }
            crate::workbench::project_workflow::open_project(state);
        }
        FileMenuAction::SaveProjectAs => {
            crate::workbench::project_workflow::save_project_as(state);
        }
        FileMenuAction::SaveAll => {
            crate::workbench::project_workflow::save_all(state);
        }
        FileMenuAction::RevertActiveDocument => {
            crate::workbench::project_workflow::request_revert_active_document(state);
        }
        FileMenuAction::CloseActiveDocument => {
            crate::workbench::project_workflow::close_active_document(state);
        }
        FileMenuAction::CloseProject => {
            crate::workbench::project_workflow::request_close_project(state);
        }
        FileMenuAction::Open => {
            if require_save_confirmation_if_dirty(state, ConfirmationAction::FileOpen) {
                return;
            }
            crate::workbench::file_actions::action_file_open_with_io(state, file_workflow_io);
        }
        FileMenuAction::Save => {
            if state.project_lifecycle.project_open {
                let _ = crate::workbench::project_workflow::save_project(state);
            } else {
                let _ =
                    crate::workbench::file_actions::action_file_save_with_io(state, file_workflow_io);
            }
        }
        FileMenuAction::ImportNetlist => {
            if require_save_confirmation_if_dirty(state, ConfirmationAction::ImportNetlist) {
                return;
            }
            crate::workbench::netlist_workflow::import_netlist(state);
        }
        FileMenuAction::ExportSvg => {
            super::export_actions::action_export_svg_with_io(state, export_workflow_io)
        }
        FileMenuAction::ExportCsvWaveforms => {
            super::waveform_export::action_export_csv_with_io(state, export_workflow_io)
        }
        FileMenuAction::ImportVerilogA => {
            state.workbench.workspace = crate::workbench::state::Workspace::Netlist;
            state.ui.code_workspace.page =
                crate::workbench::code_workspace::CodeWorkspacePage::VerilogA;
            state.ui.code_workspace.veriloga.import_requested = true;
        }
        FileMenuAction::Exit => request_exit(state),
    }
}

fn require_save_confirmation_if_dirty(state: &mut AppState, action: ConfirmationAction) -> bool {
    if !crate::workbench::project_lifecycle::has_unsaved_changes(state) {
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

fn require_project_save_confirmation_if_dirty(
    state: &mut AppState,
    action: ConfirmationAction,
) -> bool {
    if !crate::workbench::project_lifecycle::has_unsaved_changes(state) {
        return false;
    }

    state.dialogs.confirmation_dialog.show(action);
    true
}

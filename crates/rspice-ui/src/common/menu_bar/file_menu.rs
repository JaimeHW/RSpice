//! File action layer: dirty-state confirmations, project/file IO dispatch,
//! exports and application exit. Rendering lives in the workbench title bar.

use crate::common::app::{AppState, ConfirmationAction};
use crate::common::export_workflow::ExportWorkflowIo;
use crate::common::file_workflow::FileWorkflowIo;

/// Every action reachable from the File menu.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FileMenuAction {
    NewProject,
    OpenProject,
    SaveProject,
    SaveProjectAs,
    SaveAll,
    RevertActiveDocument,
    CloseActiveDocument,
    CloseProject,
    New,
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
            crate::common::project_workflow::create_new_project(state);
        }
        FileMenuAction::OpenProject => {
            if require_project_save_confirmation_if_dirty(state, ConfirmationAction::ProjectOpen) {
                return;
            }
            crate::common::project_workflow::open_project(state);
        }
        FileMenuAction::SaveProject => {
            crate::common::project_workflow::save_project(state);
        }
        FileMenuAction::SaveProjectAs => {
            crate::common::project_workflow::save_project_as(state);
        }
        FileMenuAction::SaveAll => {
            crate::common::project_workflow::save_all(state);
        }
        FileMenuAction::RevertActiveDocument => {
            crate::common::project_workflow::request_revert_active_document(state);
        }
        FileMenuAction::CloseActiveDocument => {
            crate::common::project_workflow::close_active_document(state);
        }
        FileMenuAction::CloseProject => {
            crate::common::project_workflow::request_close_project(state);
        }
        FileMenuAction::New => {
            if require_save_confirmation_if_dirty(state, ConfirmationAction::FileNew) {
                return;
            }
            crate::common::file_actions::action_file_new(state);
        }
        FileMenuAction::Open => {
            if require_save_confirmation_if_dirty(state, ConfirmationAction::FileOpen) {
                return;
            }
            crate::common::file_actions::action_file_open_with_io(state, file_workflow_io);
        }
        FileMenuAction::Save => {
            if state.project_lifecycle.project_open {
                let _ = crate::common::project_workflow::save_project(state);
            } else {
                let _ =
                    crate::common::file_actions::action_file_save_with_io(state, file_workflow_io);
            }
        }
        FileMenuAction::ImportNetlist => {
            if require_save_confirmation_if_dirty(state, ConfirmationAction::ImportNetlist) {
                return;
            }
            crate::common::netlist_workflow::import_netlist(state);
        }
        FileMenuAction::ExportSvg => {
            super::export_actions::action_export_svg_with_io(state, export_workflow_io)
        }
        FileMenuAction::ExportCsvWaveforms => {
            super::waveform_export::action_export_csv_with_io(state, export_workflow_io)
        }
        FileMenuAction::ImportVerilogA => open_veriloga_import_dialog(state),
        FileMenuAction::Exit => request_exit(state),
    }
}

fn require_save_confirmation_if_dirty(state: &mut AppState, action: ConfirmationAction) -> bool {
    if !crate::common::project_lifecycle::has_unsaved_changes(state) {
        return false;
    }

    state.dialogs.confirmation_dialog.show(action);
    true
}

fn open_veriloga_import_dialog(state: &mut AppState) {
    state.dialogs.veriloga_dialog.open();
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
    if !crate::common::project_lifecycle::has_unsaved_changes(state) {
        return false;
    }

    state.dialogs.confirmation_dialog.show(action);
    true
}

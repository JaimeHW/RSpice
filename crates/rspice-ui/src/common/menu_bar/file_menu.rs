//! File action layer: dirty-state confirmations, project/file IO dispatch,
//! exports and application exit. Rendering lives in `crate::shell::menubar`.

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
    New,
    Open,
    Save,
    SaveAs,
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
            if state.should_save_project_for_active_document() {
                let _ = crate::common::project_workflow::save_project(state);
            } else {
                let _ =
                    crate::common::file_actions::action_file_save_with_io(state, file_workflow_io);
            }
        }
        FileMenuAction::SaveAs => {
            if state.should_save_project_for_active_document() {
                let _ = crate::common::project_workflow::save_project_as(state);
            } else {
                let _ = crate::common::file_actions::action_file_save_as_with_io(
                    state,
                    file_workflow_io,
                );
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
    if !state.schematic.is_dirty && !state.workspace.any_dirty() {
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
    if !state.schematic.is_dirty && !state.workspace.any_dirty() {
        return false;
    }

    state.dialogs.confirmation_dialog.show(action);
    true
}

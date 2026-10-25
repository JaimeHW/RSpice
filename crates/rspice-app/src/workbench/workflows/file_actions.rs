//! The File menu's schematic open and save actions, over an injectable
//! `FileWorkflowIo` so the tests run them without a disk or a file picker.

use crate::io::FileWorkflowIo;
use crate::workbench::app_state::AppState;

pub(in crate::workbench) fn action_file_open_with_io(
    state: &mut AppState,
    io: &(impl FileWorkflowIo + ?Sized),
) {
    crate::workbench::workflows::file_workflow::open_schematic_from_dialog_with_io(state, io);
}

pub(in crate::workbench) fn action_file_save_with_io(
    state: &mut AppState,
    io: &(impl FileWorkflowIo + ?Sized),
) -> bool {
    crate::workbench::workflows::file_workflow::save_schematic_with_io(state, io)
}

//! File path helpers shared by the workflows.
//!
//! Extension checking and normalization, kept in one place because a
//! document saved without its expected extension is a document the next
//! open will not find.

use crate::workbench::app_state::AppState;
use crate::workbench::workflows::file_workflow::FileWorkflowIo;

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

pub(crate) fn has_file_extension(path: &std::path::Path, expected_ext: &str) -> bool {
    path.extension()
        .and_then(std::ffi::OsStr::to_str)
        .is_some_and(|ext| ext.eq_ignore_ascii_case(expected_ext))
}

pub(crate) fn ensure_file_extension(path: &mut std::path::PathBuf, expected_ext: &str) {
    if !has_file_extension(path, expected_ext) {
        path.set_extension(expected_ext);
    }
}

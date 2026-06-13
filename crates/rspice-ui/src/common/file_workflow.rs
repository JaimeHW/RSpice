use std::path::{Path, PathBuf};

use crate::common::app::{AppState, ConsoleMessage};
use crate::io::SchematicIoError;
use crate::state::SchematicState;

/// IO abstraction for schematic file workflows.
///
/// This allows open/save behavior to be fully unit tested without invoking
/// native dialogs.
pub(crate) trait FileWorkflowIo {
    fn show_open_dialog(&self) -> Result<PathBuf, SchematicIoError>;
    fn show_save_dialog(&self, default_name: Option<&str>) -> Result<PathBuf, SchematicIoError>;
    fn load_schematic(&self, path: &Path) -> Result<SchematicState, SchematicIoError>;
    fn save_schematic(
        &self,
        schematic: &SchematicState,
        path: &Path,
    ) -> Result<(), SchematicIoError>;
}

/// Production IO backend using native dialogs and filesystem operations.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct NativeFileWorkflowIo;

impl FileWorkflowIo for NativeFileWorkflowIo {
    fn show_open_dialog(&self) -> Result<PathBuf, SchematicIoError> {
        crate::io::show_open_dialog()
    }

    fn show_save_dialog(&self, default_name: Option<&str>) -> Result<PathBuf, SchematicIoError> {
        crate::io::show_save_dialog(default_name)
    }

    fn load_schematic(&self, path: &Path) -> Result<SchematicState, SchematicIoError> {
        crate::io::load_schematic(path)
    }

    fn save_schematic(
        &self,
        schematic: &SchematicState,
        path: &Path,
    ) -> Result<(), SchematicIoError> {
        crate::io::save_schematic(schematic, path)
    }
}

/// Reset the current schematic to a new empty document.
pub(crate) fn create_new_schematic(state: &mut AppState) {
    state.schematic = crate::state::SchematicState::default();
    state.sync_active_schematic_to_workspace();
    state.push_user_message(ConsoleMessage::info("Created new schematic"));
}

/// Sibling checkpoint path for a schematic file: `design.rsp` →
/// `design.rsp.autosave`. Native only — the web build has no checkpoints.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn autosave_checkpoint_path(path: &Path) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(".autosave");
    PathBuf::from(name)
}

/// Whether a newer autosave checkpoint shadows this file (native only —
/// the web build has no checkpoint files).
#[cfg(not(target_arch = "wasm32"))]
fn newer_checkpoint(path: &Path) -> Option<PathBuf> {
    let checkpoint = autosave_checkpoint_path(path);
    let checkpoint_modified = std::fs::metadata(&checkpoint)
        .and_then(|meta| meta.modified())
        .ok()?;
    let file_modified = std::fs::metadata(path)
        .and_then(|meta| meta.modified())
        .ok()?;
    (checkpoint_modified > file_modified).then_some(checkpoint)
}

pub(crate) fn load_schematic_from_path_with_io(
    state: &mut AppState,
    path: &Path,
    io: &(impl FileWorkflowIo + ?Sized),
) -> bool {
    // A newer checkpoint means the last session ended without a save —
    // ask before silently opening the older file (volta-app-dialogs §02).
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(checkpoint) = newer_checkpoint(path) {
        state.dialogs.pending_autosave_restore = Some((path.to_path_buf(), checkpoint));
        return true;
    }
    load_schematic_bypassing_autosave(state, path, io)
}

/// The actual load, with no checkpoint interception — the restore
/// dialog's own paths must not re-trigger it.
pub(crate) fn load_schematic_bypassing_autosave(
    state: &mut AppState,
    path: &Path,
    io: &(impl FileWorkflowIo + ?Sized),
) -> bool {
    match io.load_schematic(path) {
        Ok(schematic) => {
            state.schematic = schematic;
            state.sync_active_schematic_to_workspace();
            state.remember_recent_file(crate::common::app::RecentKind::Schematic, path);
            state.push_user_message(ConsoleMessage::info(format!("Opened: {}", path.display())));
            true
        }
        Err(e) => {
            state.push_user_message(ConsoleMessage::error(format!("Failed to open: {}", e)));
            false
        }
    }
}

pub(crate) fn open_schematic_from_dialog_with_io(
    state: &mut AppState,
    io: &(impl FileWorkflowIo + ?Sized),
) {
    match io.show_open_dialog() {
        Ok(path) => {
            let _ = load_schematic_from_path_with_io(state, &path, io);
        }
        Err(SchematicIoError::Cancelled) => {
            // User cancelled - no message needed
        }
        Err(e) => {
            state.push_user_message(ConsoleMessage::error(format!("Open failed: {}", e)));
        }
    }
}

pub(crate) fn save_schematic_to_path_with_io(
    state: &mut AppState,
    path: &Path,
    update_current_file: bool,
    io: &(impl FileWorkflowIo + ?Sized),
) -> bool {
    match io.save_schematic(&state.schematic, path) {
        Ok(()) => {
            if update_current_file {
                state.schematic.current_file = Some(path.to_path_buf());
            }
            state.schematic.is_dirty = false;
            // A clean save is the truth again — a leftover checkpoint
            // would only shadow it with something older next open.
            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = std::fs::remove_file(autosave_checkpoint_path(path));
            }
            state.sync_active_schematic_to_workspace();
            state.remember_recent_file(crate::common::app::RecentKind::Schematic, path);
            state.push_user_message(ConsoleMessage::info(format!("Saved: {}", path.display())));
            true
        }
        Err(e) => {
            state.push_user_message(ConsoleMessage::error(format!("Save failed: {}", e)));
            false
        }
    }
}

pub(crate) fn save_schematic_with_io(
    state: &mut AppState,
    io: &(impl FileWorkflowIo + ?Sized),
) -> bool {
    if let Some(path) = state.schematic.current_file.clone() {
        save_schematic_to_path_with_io(state, &path, false, io)
    } else {
        save_schematic_as_with_io(state, io)
    }
}

pub(crate) fn save_schematic_as_with_io(
    state: &mut AppState,
    io: &(impl FileWorkflowIo + ?Sized),
) -> bool {
    let default_name = state
        .schematic
        .current_file
        .as_ref()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().to_string());

    match io.show_save_dialog(default_name.as_deref()) {
        Ok(path) => save_schematic_to_path_with_io(state, &path, true, io),
        Err(SchematicIoError::Cancelled) => {
            // User cancelled - no message needed
            false
        }
        Err(e) => {
            state.push_user_message(ConsoleMessage::error(format!("Save As failed: {}", e)));
            false
        }
    }
}

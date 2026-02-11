use std::path::Path;

use crate::common::app::{AppState, ConsoleMessage};

/// Reset the current schematic to a new empty document.
pub(crate) fn create_new_schematic(state: &mut AppState) {
    state.schematic = crate::state::SchematicState::default();
    state.push_user_message(ConsoleMessage::info("Created new schematic"));
}

/// Load a schematic from an explicit path.
///
/// Returns `true` on success and reports errors through the user console.
pub(crate) fn load_schematic_from_path(state: &mut AppState, path: &Path) -> bool {
    match crate::io::load_schematic(path) {
        Ok(schematic) => {
            state.schematic = schematic;
            state.push_user_message(ConsoleMessage::info(format!("Opened: {}", path.display())));
            true
        }
        Err(e) => {
            state.push_user_message(ConsoleMessage::error(format!("Failed to open: {}", e)));
            false
        }
    }
}

/// Prompt the user for a schematic file path and load it.
pub(crate) fn open_schematic_from_dialog(state: &mut AppState) {
    match crate::io::show_open_dialog() {
        Ok(path) => {
            let _ = load_schematic_from_path(state, &path);
        }
        Err(crate::io::SchematicIoError::Cancelled) => {
            // User cancelled - no message needed
        }
        Err(e) => {
            state.push_user_message(ConsoleMessage::error(format!("Open failed: {}", e)));
        }
    }
}

/// Save the current schematic to an explicit path.
///
/// Returns `true` on success and reports errors through the user console.
pub(crate) fn save_schematic_to_path(
    state: &mut AppState,
    path: &Path,
    update_current_file: bool,
) -> bool {
    match crate::io::save_schematic(&state.schematic, path) {
        Ok(()) => {
            if update_current_file {
                state.schematic.current_file = Some(path.to_path_buf());
            }
            state.schematic.is_dirty = false;
            state.push_user_message(ConsoleMessage::info(format!("Saved: {}", path.display())));
            true
        }
        Err(e) => {
            state.push_user_message(ConsoleMessage::error(format!("Save failed: {}", e)));
            false
        }
    }
}

/// Save the current schematic.
///
/// If no current file exists, this falls back to Save As.
pub(crate) fn save_schematic(state: &mut AppState) -> bool {
    if let Some(path) = state.schematic.current_file.clone() {
        save_schematic_to_path(state, &path, false)
    } else {
        save_schematic_as(state)
    }
}

/// Prompt for a target path and save the current schematic.
pub(crate) fn save_schematic_as(state: &mut AppState) -> bool {
    let default_name = state
        .schematic
        .current_file
        .as_ref()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().to_string());

    match crate::io::show_save_dialog(default_name.as_deref()) {
        Ok(path) => save_schematic_to_path(state, &path, true),
        Err(crate::io::SchematicIoError::Cancelled) => {
            // User cancelled - no message needed
            false
        }
        Err(e) => {
            state.push_user_message(ConsoleMessage::error(format!("Save As failed: {}", e)));
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_new_schematic_resets_state_and_logs_message() {
        use crate::state::{Component, ComponentType, Point};

        let mut state = AppState::default();
        state.schematic.components.push(Component::new(
            1,
            ComponentType::Resistor,
            Point::new(120, 80),
        ));
        state.schematic.is_dirty = true;
        state.schematic.current_file = Some(std::path::PathBuf::from("circuit.rsch"));

        create_new_schematic(&mut state);

        assert!(state.schematic.components.is_empty());
        assert!(!state.schematic.is_dirty);
        assert!(state.schematic.current_file.is_none());
        assert!(
            state
                .console_messages
                .iter()
                .any(|msg| msg.message.contains("Created new schematic"))
        );
    }

    #[test]
    fn test_load_schematic_from_path_success_replaces_state() {
        use crate::state::{ComponentType, Point};

        let temp_dir = tempfile::tempdir().expect("tempdir should create");
        let source_path = temp_dir.path().join("load-source.rsch");

        let mut source = crate::state::SchematicState::default();
        source.add_component(ComponentType::Capacitor, Point::new(42, 24));
        crate::io::save_schematic(&source, &source_path).expect("fixture schematic should save");

        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(1, 1));

        let loaded = load_schematic_from_path(&mut state, &source_path);

        assert!(loaded);
        assert_eq!(state.schematic.components.len(), 1);
        assert_eq!(state.schematic.components[0].kind, ComponentType::Capacitor);
        assert_eq!(state.schematic.current_file, Some(source_path.clone()));
        assert!(!state.schematic.is_dirty);
        assert!(
            state
                .console_messages
                .iter()
                .any(|msg| msg.message.contains("Opened:"))
        );
    }

    #[test]
    fn test_load_schematic_from_path_failure_preserves_existing_state() {
        use crate::state::{ComponentType, Point};

        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(10, 10));
        let prior_component_count = state.schematic.components.len();

        let loaded = load_schematic_from_path(
            &mut state,
            std::path::Path::new("this/path/does/not/exist.rsch"),
        );

        assert!(!loaded);
        assert_eq!(state.schematic.components.len(), prior_component_count);
        assert!(
            state
                .console_messages
                .iter()
                .any(|msg| msg.message.contains("Failed to open:"))
        );
    }

    #[test]
    fn test_save_schematic_to_path_success_updates_dirty_state() {
        use crate::state::{ComponentType, Point};

        let temp_dir = tempfile::tempdir().expect("tempdir should create");
        let save_path = temp_dir.path().join("save-target.rsch");

        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(100, 100));
        state.schematic.is_dirty = true;

        let saved = save_schematic_to_path(&mut state, &save_path, false);

        assert!(saved);
        assert!(save_path.exists());
        assert!(!state.schematic.is_dirty);
        assert!(state.schematic.current_file.is_none());
        assert!(
            state
                .console_messages
                .iter()
                .any(|msg| msg.message.contains("Saved:"))
        );
    }

    #[test]
    fn test_save_schematic_to_path_success_sets_current_file_when_requested() {
        use crate::state::{ComponentType, Point};

        let temp_dir = tempfile::tempdir().expect("tempdir should create");
        let save_path = temp_dir.path().join("save-as-target.rsch");

        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Inductor, Point::new(15, 30));
        state.schematic.is_dirty = true;

        let saved = save_schematic_to_path(&mut state, &save_path, true);

        assert!(saved);
        assert_eq!(state.schematic.current_file, Some(save_path));
        assert!(!state.schematic.is_dirty);
    }

    #[test]
    fn test_save_schematic_to_path_failure_keeps_dirty_state() {
        use crate::state::{ComponentType, Point};

        let temp_dir = tempfile::tempdir().expect("tempdir should create");
        let invalid_target = temp_dir.path().to_path_buf(); // directory path; save should fail

        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(100, 100));
        state.schematic.is_dirty = true;

        let saved = save_schematic_to_path(&mut state, &invalid_target, false);

        assert!(!saved);
        assert!(state.schematic.is_dirty);
        assert!(
            state
                .console_messages
                .iter()
                .any(|msg| msg.message.contains("Save failed:"))
        );
    }

    #[test]
    fn test_save_schematic_with_current_file_uses_existing_path() {
        use crate::state::{ComponentType, Point};

        let temp_dir = tempfile::tempdir().expect("tempdir should create");
        let save_path = temp_dir.path().join("existing-path-save.rsch");

        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(9, 9));
        state.schematic.current_file = Some(save_path.clone());
        state.schematic.is_dirty = true;

        let saved = save_schematic(&mut state);

        assert!(saved);
        assert!(save_path.exists());
        assert_eq!(state.schematic.current_file, Some(save_path));
        assert!(!state.schematic.is_dirty);
    }
}

use crate::common::app::AppState;

pub(super) fn action_file_new(state: &mut AppState) {
    if state.schematic.is_dirty {
        log::warn!("New schematic requested but current has unsaved changes");
    }
    state.schematic = crate::state::SchematicState::default();
    state.push_user_message(crate::common::app::ConsoleMessage::info(
        "Created new schematic",
    ));
}

pub(super) fn action_file_open(state: &mut AppState) {
    use crate::io::{load_schematic, show_open_dialog, SchematicIoError};

    match show_open_dialog() {
        Ok(path) => match load_schematic(&path) {
            Ok(schematic) => {
                state.schematic = schematic;
                state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                    "Opened: {}",
                    path.display()
                )));
            }
            Err(e) => {
                state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                    "Failed to open: {}",
                    e
                )));
            }
        },
        Err(SchematicIoError::Cancelled) => {
            // User cancelled - no message needed
        }
        Err(e) => {
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                "Open failed: {}",
                e
            )));
        }
    }
}

pub(super) fn action_file_save(state: &mut AppState) {
    use crate::io::save_schematic;

    // If we have a current file path, save directly.
    // Otherwise, show Save As dialog.
    if let Some(ref path) = state.schematic.current_file.clone() {
        match save_schematic(&state.schematic, path) {
            Ok(()) => {
                state.schematic.is_dirty = false;
                state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                    "Saved: {}",
                    path.display()
                )));
            }
            Err(e) => {
                state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                    "Save failed: {}",
                    e
                )));
            }
        }
    } else {
        action_file_save_as(state);
    }
}

pub(super) fn action_file_save_as(state: &mut AppState) {
    use crate::io::{save_schematic, show_save_dialog, SchematicIoError};

    let default_name = state
        .schematic
        .current_file
        .as_ref()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().to_string());

    match show_save_dialog(default_name.as_deref()) {
        Ok(path) => match save_schematic(&state.schematic, &path) {
            Ok(()) => {
                state.schematic.current_file = Some(path.clone());
                state.schematic.is_dirty = false;
                state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                    "Saved: {}",
                    path.display()
                )));
            }
            Err(e) => {
                state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                    "Save failed: {}",
                    e
                )));
            }
        },
        Err(SchematicIoError::Cancelled) => {
            // User cancelled - no message needed
        }
        Err(e) => {
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                "Save As failed: {}",
                e
            )));
        }
    }
}

pub(super) fn has_file_extension(path: &std::path::Path, expected_ext: &str) -> bool {
    path.extension()
        .and_then(std::ffi::OsStr::to_str)
        .is_some_and(|ext| ext.eq_ignore_ascii_case(expected_ext))
}

pub(super) fn ensure_file_extension(path: &mut std::path::PathBuf, expected_ext: &str) {
    if !has_file_extension(path, expected_ext) {
        path.set_extension(expected_ext);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_file_new_clears_schematic() {
        let mut state = AppState::default();

        use crate::state::{Component, ComponentType, Point};
        let comp = Component::new(1, ComponentType::Resistor, Point::new(100, 100))
            .with_name_value("R1", "1k");
        state.schematic.components.push(comp);
        assert!(!state.schematic.components.is_empty());

        action_file_new(&mut state);

        assert!(state.schematic.components.is_empty());
        assert!(!state.console_messages.is_empty());
    }

    #[test]
    fn test_has_file_extension_case_insensitive() {
        assert!(has_file_extension(
            std::path::Path::new("schematic.SVG"),
            "svg"
        ));
        assert!(!has_file_extension(
            std::path::Path::new("schematic.raw"),
            "svg"
        ));
    }

    #[test]
    fn test_ensure_file_extension_appends_missing_extension() {
        let mut path = std::path::PathBuf::from("waveforms");
        ensure_file_extension(&mut path, "csv");
        assert_eq!(path, std::path::PathBuf::from("waveforms.csv"));
    }

    #[test]
    fn test_ensure_file_extension_replaces_mismatched_extension() {
        let mut path = std::path::PathBuf::from("results.txt");
        ensure_file_extension(&mut path, "csv");
        assert_eq!(path, std::path::PathBuf::from("results.csv"));
    }

    // NOTE: action_file_open, action_file_save, and action_file_save_as
    // cannot be tested here because they open native file dialogs which
    // would block in a headless test environment. The underlying I/O
    // functions are thoroughly tested in io::schematic_io::tests.
}

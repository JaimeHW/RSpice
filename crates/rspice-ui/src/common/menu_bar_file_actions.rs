use crate::common::app::AppState;

pub(super) fn action_file_new(state: &mut AppState) {
    crate::common::file_workflow::create_new_schematic(state);
}

pub(super) fn action_file_open(state: &mut AppState) {
    crate::common::file_workflow::open_schematic_from_dialog(state);
}

pub(super) fn action_file_save(state: &mut AppState) {
    let _ = crate::common::file_workflow::save_schematic(state);
}

pub(super) fn action_file_save_as(state: &mut AppState) {
    let _ = crate::common::file_workflow::save_schematic_as(state);
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

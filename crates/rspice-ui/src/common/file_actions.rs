use crate::common::app::AppState;
use crate::common::file_workflow::FileWorkflowIo;

pub(super) fn action_file_new(state: &mut AppState) {
    crate::common::file_workflow::create_new_schematic(state);
}

pub(super) fn action_file_open_with_io(state: &mut AppState, io: &(impl FileWorkflowIo + ?Sized)) {
    crate::common::file_workflow::open_schematic_from_dialog_with_io(state, io);
}

pub(super) fn action_file_save_with_io(
    state: &mut AppState,
    io: &(impl FileWorkflowIo + ?Sized),
) -> bool {
    crate::common::file_workflow::save_schematic_with_io(state, io)
}

pub(super) fn action_file_save_as_with_io(
    state: &mut AppState,
    io: &(impl FileWorkflowIo + ?Sized),
) -> bool {
    crate::common::file_workflow::save_schematic_as_with_io(state, io)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::collections::VecDeque;
    use std::path::{Path, PathBuf};

    #[derive(Default)]
    struct MockMenuFileWorkflowIo {
        open_dialog_results: RefCell<VecDeque<Result<PathBuf, crate::io::SchematicIoError>>>,
        save_dialog_results: RefCell<VecDeque<Result<PathBuf, crate::io::SchematicIoError>>>,
        load_results:
            RefCell<VecDeque<Result<crate::state::SchematicState, crate::io::SchematicIoError>>>,
        save_results: RefCell<VecDeque<Result<(), crate::io::SchematicIoError>>>,
        open_dialog_calls: Cell<usize>,
        save_dialog_calls: Cell<usize>,
        load_calls: Cell<usize>,
        save_calls: Cell<usize>,
        last_save_default_name: RefCell<Option<Option<String>>>,
    }

    impl MockMenuFileWorkflowIo {
        fn push_open_dialog_result(&self, result: Result<PathBuf, crate::io::SchematicIoError>) {
            self.open_dialog_results.borrow_mut().push_back(result);
        }

        fn push_save_dialog_result(&self, result: Result<PathBuf, crate::io::SchematicIoError>) {
            self.save_dialog_results.borrow_mut().push_back(result);
        }

        fn push_load_result(
            &self,
            result: Result<crate::state::SchematicState, crate::io::SchematicIoError>,
        ) {
            self.load_results.borrow_mut().push_back(result);
        }

        fn push_save_result(&self, result: Result<(), crate::io::SchematicIoError>) {
            self.save_results.borrow_mut().push_back(result);
        }

        fn open_dialog_calls(&self) -> usize {
            self.open_dialog_calls.get()
        }

        fn save_dialog_calls(&self) -> usize {
            self.save_dialog_calls.get()
        }

        fn load_calls(&self) -> usize {
            self.load_calls.get()
        }

        fn save_calls(&self) -> usize {
            self.save_calls.get()
        }

        fn last_save_default_name(&self) -> Option<Option<String>> {
            self.last_save_default_name.borrow().clone()
        }
    }

    impl FileWorkflowIo for MockMenuFileWorkflowIo {
        fn show_open_dialog(&self) -> Result<PathBuf, crate::io::SchematicIoError> {
            self.open_dialog_calls
                .set(self.open_dialog_calls.get().saturating_add(1));
            self.open_dialog_results
                .borrow_mut()
                .pop_front()
                .expect("test must provide show_open_dialog result")
        }

        fn show_save_dialog(
            &self,
            default_name: Option<&str>,
        ) -> Result<PathBuf, crate::io::SchematicIoError> {
            self.save_dialog_calls
                .set(self.save_dialog_calls.get().saturating_add(1));
            *self.last_save_default_name.borrow_mut() =
                Some(default_name.map(std::string::ToString::to_string));
            self.save_dialog_results
                .borrow_mut()
                .pop_front()
                .expect("test must provide show_save_dialog result")
        }

        fn load_schematic(
            &self,
            _path: &Path,
        ) -> Result<crate::state::SchematicState, crate::io::SchematicIoError> {
            self.load_calls.set(self.load_calls.get().saturating_add(1));
            self.load_results
                .borrow_mut()
                .pop_front()
                .expect("test must provide load_schematic result")
        }

        fn save_schematic(
            &self,
            _schematic: &crate::state::SchematicState,
            _path: &Path,
        ) -> Result<(), crate::io::SchematicIoError> {
            self.save_calls.set(self.save_calls.get().saturating_add(1));
            self.save_results
                .borrow_mut()
                .pop_front()
                .expect("test must provide save_schematic result")
        }
    }

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
    fn test_action_file_open_with_io_loads_selected_schematic() {
        use crate::state::ComponentType;

        let io = MockMenuFileWorkflowIo::default();
        io.push_open_dialog_result(Ok(PathBuf::from("menu-open.rsch")));
        let mut loaded = crate::state::SchematicState::default();
        loaded.add_component(ComponentType::Capacitor, crate::state::Point::new(10, 10));
        loaded.current_file = Some(PathBuf::from("menu-open.rsch"));
        loaded.is_dirty = false;
        io.push_load_result(Ok(loaded));

        let mut state = AppState::default();
        state.schematic.is_dirty = false;

        action_file_open_with_io(&mut state, &io);

        assert_eq!(io.open_dialog_calls(), 1);
        assert_eq!(io.load_calls(), 1);
        assert_eq!(state.schematic.components.len(), 1);
        assert_eq!(state.schematic.components[0].kind, ComponentType::Capacitor);
    }

    #[test]
    fn test_action_file_open_with_io_cancelled_dialog_keeps_state() {
        use crate::state::ComponentType;

        let io = MockMenuFileWorkflowIo::default();
        io.push_open_dialog_result(Err(crate::io::SchematicIoError::Cancelled));
        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, crate::state::Point::new(5, 5));
        let initial_count = state.schematic.components.len();

        action_file_open_with_io(&mut state, &io);

        assert_eq!(io.open_dialog_calls(), 1);
        assert_eq!(io.load_calls(), 0);
        assert_eq!(state.schematic.components.len(), initial_count);
    }

    #[test]
    fn test_action_file_save_with_io_uses_existing_current_file() {
        use crate::state::ComponentType;

        let io = MockMenuFileWorkflowIo::default();
        io.push_save_result(Ok(()));
        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, crate::state::Point::new(1, 1));
        state.schematic.current_file = Some(PathBuf::from("existing-save.rsch"));
        state.schematic.is_dirty = true;

        let saved = action_file_save_with_io(&mut state, &io);

        assert!(saved);
        assert_eq!(io.save_dialog_calls(), 0);
        assert_eq!(io.save_calls(), 1);
        assert!(!state.schematic.is_dirty);
    }

    #[test]
    fn test_action_file_save_as_with_io_passes_default_filename() {
        let io = MockMenuFileWorkflowIo::default();
        io.push_save_dialog_result(Err(crate::io::SchematicIoError::Cancelled));
        let mut state = AppState::default();
        state.schematic.current_file = Some(PathBuf::from("path/my-schematic.rsch"));

        let saved = action_file_save_as_with_io(&mut state, &io);

        assert!(!saved);
        assert_eq!(io.save_dialog_calls(), 1);
        assert_eq!(
            io.last_save_default_name(),
            Some(Some("my-schematic.rsch".to_string()))
        );
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
}

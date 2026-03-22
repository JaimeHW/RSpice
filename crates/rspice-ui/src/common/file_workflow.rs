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
    state.push_user_message(ConsoleMessage::info("Created new schematic"));
}

pub(crate) fn load_schematic_from_path_with_io(
    state: &mut AppState,
    path: &Path,
    io: &(impl FileWorkflowIo + ?Sized),
) -> bool {
    match io.load_schematic(path) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::collections::VecDeque;

    #[derive(Default)]
    struct MockFileWorkflowIo {
        open_dialog_results: RefCell<VecDeque<Result<PathBuf, SchematicIoError>>>,
        save_dialog_results: RefCell<VecDeque<Result<PathBuf, SchematicIoError>>>,
        load_results: RefCell<VecDeque<Result<SchematicState, SchematicIoError>>>,
        save_results: RefCell<VecDeque<Result<(), SchematicIoError>>>,
        open_dialog_calls: Cell<usize>,
        save_dialog_calls: Cell<usize>,
        load_calls: Cell<usize>,
        save_calls: Cell<usize>,
        last_save_default_name: RefCell<Option<Option<String>>>,
        last_load_path: RefCell<Option<PathBuf>>,
        last_save_path: RefCell<Option<PathBuf>>,
    }

    impl MockFileWorkflowIo {
        fn push_open_dialog_result(&self, result: Result<PathBuf, SchematicIoError>) {
            self.open_dialog_results.borrow_mut().push_back(result);
        }

        fn push_save_dialog_result(&self, result: Result<PathBuf, SchematicIoError>) {
            self.save_dialog_results.borrow_mut().push_back(result);
        }

        fn push_load_result(&self, result: Result<SchematicState, SchematicIoError>) {
            self.load_results.borrow_mut().push_back(result);
        }

        fn push_save_result(&self, result: Result<(), SchematicIoError>) {
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

        fn last_load_path(&self) -> Option<PathBuf> {
            self.last_load_path.borrow().clone()
        }

        fn last_save_path(&self) -> Option<PathBuf> {
            self.last_save_path.borrow().clone()
        }
    }

    impl FileWorkflowIo for MockFileWorkflowIo {
        fn show_open_dialog(&self) -> Result<PathBuf, SchematicIoError> {
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
        ) -> Result<PathBuf, SchematicIoError> {
            self.save_dialog_calls
                .set(self.save_dialog_calls.get().saturating_add(1));
            *self.last_save_default_name.borrow_mut() =
                Some(default_name.map(std::string::ToString::to_string));
            self.save_dialog_results
                .borrow_mut()
                .pop_front()
                .expect("test must provide show_save_dialog result")
        }

        fn load_schematic(&self, path: &Path) -> Result<SchematicState, SchematicIoError> {
            self.load_calls.set(self.load_calls.get().saturating_add(1));
            *self.last_load_path.borrow_mut() = Some(path.to_path_buf());
            self.load_results
                .borrow_mut()
                .pop_front()
                .expect("test must provide load_schematic result")
        }

        fn save_schematic(
            &self,
            _schematic: &SchematicState,
            path: &Path,
        ) -> Result<(), SchematicIoError> {
            self.save_calls.set(self.save_calls.get().saturating_add(1));
            *self.last_save_path.borrow_mut() = Some(path.to_path_buf());
            self.save_results
                .borrow_mut()
                .pop_front()
                .expect("test must provide save_schematic result")
        }
    }

    fn schematic_with_component(
        kind: crate::state::ComponentType,
        x: i32,
        y: i32,
    ) -> SchematicState {
        let mut schematic = SchematicState::default();
        schematic.add_component(kind, crate::state::Point::new(x, y));
        schematic
    }

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
        assert!(state
            .console_messages
            .iter()
            .any(|msg| msg.message.contains("Created new schematic")));
    }

    #[test]
    fn test_load_schematic_from_path_success_replaces_state() {
        use crate::state::ComponentType;
        let io = MockFileWorkflowIo::default();
        let source_path = PathBuf::from("fixtures/load-source.rsch");
        io.push_load_result(Ok(schematic_with_component(
            ComponentType::Capacitor,
            42,
            24,
        )));

        let mut state = AppState::default();
        state.schematic = schematic_with_component(ComponentType::Resistor, 1, 1);

        let loaded = load_schematic_from_path_with_io(&mut state, &source_path, &io);

        assert!(loaded);
        assert_eq!(state.schematic.components.len(), 1);
        assert_eq!(state.schematic.components[0].kind, ComponentType::Capacitor);
        assert_eq!(io.load_calls(), 1);
        assert_eq!(io.last_load_path(), Some(source_path));
        assert!(state
            .console_messages
            .iter()
            .any(|msg| msg.message.contains("Opened:")));
    }

    #[test]
    fn test_load_schematic_from_path_failure_preserves_existing_state() {
        use crate::state::ComponentType;
        let io = MockFileWorkflowIo::default();
        io.push_load_result(Err(SchematicIoError::Io("mock-load-failure".to_string())));

        let mut state = AppState::default();
        state.schematic = schematic_with_component(ComponentType::Resistor, 10, 10);
        let prior_component_count = state.schematic.components.len();

        let loaded = load_schematic_from_path_with_io(
            &mut state,
            std::path::Path::new("this/path/does/not/exist.rsch"),
            &io,
        );

        assert!(!loaded);
        assert_eq!(io.load_calls(), 1);
        assert_eq!(state.schematic.components.len(), prior_component_count);
        assert!(state
            .console_messages
            .iter()
            .any(|msg| msg.message.contains("Failed to open:")));
    }

    #[test]
    fn test_save_schematic_to_path_success_updates_dirty_state() {
        use crate::state::{ComponentType, Point};
        let io = MockFileWorkflowIo::default();
        io.push_save_result(Ok(()));

        let save_path = PathBuf::from("save-target.rsch");

        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(100, 100));
        state.schematic.is_dirty = true;

        let saved = save_schematic_to_path_with_io(&mut state, &save_path, false, &io);

        assert!(saved);
        assert_eq!(io.save_calls(), 1);
        assert_eq!(io.last_save_path(), Some(save_path));
        assert!(!state.schematic.is_dirty);
        assert!(state.schematic.current_file.is_none());
        assert!(state
            .console_messages
            .iter()
            .any(|msg| msg.message.contains("Saved:")));
    }

    #[test]
    fn test_save_schematic_to_path_success_sets_current_file_when_requested() {
        use crate::state::{ComponentType, Point};
        let io = MockFileWorkflowIo::default();
        io.push_save_result(Ok(()));

        let save_path = PathBuf::from("save-as-target.rsch");

        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Inductor, Point::new(15, 30));
        state.schematic.is_dirty = true;

        let saved = save_schematic_to_path_with_io(&mut state, &save_path, true, &io);

        assert!(saved);
        assert_eq!(state.schematic.current_file, Some(save_path));
        assert!(!state.schematic.is_dirty);
    }

    #[test]
    fn test_save_schematic_to_path_failure_keeps_dirty_state() {
        use crate::state::{ComponentType, Point};
        let io = MockFileWorkflowIo::default();
        io.push_save_result(Err(SchematicIoError::Io("mock-save-failure".to_string())));
        let invalid_target = PathBuf::from("invalid-target.rsch");

        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(100, 100));
        state.schematic.is_dirty = true;

        let saved = save_schematic_to_path_with_io(&mut state, &invalid_target, false, &io);

        assert!(!saved);
        assert_eq!(io.save_calls(), 1);
        assert!(state.schematic.is_dirty);
        assert!(state
            .console_messages
            .iter()
            .any(|msg| msg.message.contains("Save failed:")));
    }

    #[test]
    fn test_save_schematic_with_current_file_uses_existing_path() {
        use crate::state::{ComponentType, Point};
        let io = MockFileWorkflowIo::default();
        io.push_save_result(Ok(()));

        let save_path = PathBuf::from("existing-path-save.rsch");

        let mut state = AppState::default();
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(9, 9));
        state.schematic.current_file = Some(save_path.clone());
        state.schematic.is_dirty = true;

        let saved = save_schematic_with_io(&mut state, &io);

        assert!(saved);
        assert_eq!(io.save_dialog_calls(), 0);
        assert_eq!(io.save_calls(), 1);
        assert_eq!(state.schematic.current_file, Some(save_path));
        assert!(!state.schematic.is_dirty);
    }

    #[test]
    fn test_open_schematic_from_dialog_cancelled_skips_load_and_messages() {
        let io = MockFileWorkflowIo::default();
        io.push_open_dialog_result(Err(SchematicIoError::Cancelled));
        let mut state = AppState::default();

        open_schematic_from_dialog_with_io(&mut state, &io);

        assert_eq!(io.open_dialog_calls(), 1);
        assert_eq!(io.load_calls(), 0);
        assert!(state.console_messages.is_empty());
    }

    #[test]
    fn test_open_schematic_from_dialog_failure_logs_error_without_load() {
        let io = MockFileWorkflowIo::default();
        io.push_open_dialog_result(Err(SchematicIoError::Io(
            "mock-open-dialog-failure".to_string(),
        )));
        let mut state = AppState::default();

        open_schematic_from_dialog_with_io(&mut state, &io);

        assert_eq!(io.open_dialog_calls(), 1);
        assert_eq!(io.load_calls(), 0);
        assert!(state
            .console_messages
            .iter()
            .any(|msg| msg.message.contains("Open failed:")));
    }

    #[test]
    fn test_open_schematic_from_dialog_load_failure_preserves_state() {
        use crate::state::ComponentType;
        let io = MockFileWorkflowIo::default();
        io.push_open_dialog_result(Ok(PathBuf::from("open-target.rsch")));
        io.push_load_result(Err(SchematicIoError::ParseError(
            "mock-parse-failure".to_string(),
        )));
        let mut state = AppState::default();
        state.schematic = schematic_with_component(ComponentType::Resistor, 5, 5);
        let prior_component_count = state.schematic.components.len();

        open_schematic_from_dialog_with_io(&mut state, &io);

        assert_eq!(io.open_dialog_calls(), 1);
        assert_eq!(io.load_calls(), 1);
        assert_eq!(state.schematic.components.len(), prior_component_count);
        assert!(state
            .console_messages
            .iter()
            .any(|msg| msg.message.contains("Failed to open:")));
    }

    #[test]
    fn test_save_schematic_without_current_file_uses_save_as_flow() {
        let io = MockFileWorkflowIo::default();
        io.push_save_dialog_result(Ok(PathBuf::from("new-file.rsch")));
        io.push_save_result(Ok(()));
        let mut state = AppState::default();
        state.schematic.is_dirty = true;

        let saved = save_schematic_with_io(&mut state, &io);

        assert!(saved);
        assert_eq!(io.save_dialog_calls(), 1);
        assert_eq!(io.save_calls(), 1);
        assert_eq!(
            state.schematic.current_file,
            Some(PathBuf::from("new-file.rsch"))
        );
        assert!(!state.schematic.is_dirty);
    }

    #[test]
    fn test_save_schematic_without_current_file_cancelled_returns_false_without_save() {
        let io = MockFileWorkflowIo::default();
        io.push_save_dialog_result(Err(SchematicIoError::Cancelled));
        let mut state = AppState::default();
        state.schematic.is_dirty = true;

        let saved = save_schematic_with_io(&mut state, &io);

        assert!(!saved);
        assert_eq!(io.save_dialog_calls(), 1);
        assert_eq!(io.save_calls(), 0);
        assert!(state.schematic.is_dirty);
        assert!(state.console_messages.is_empty());
    }

    #[test]
    fn test_save_schematic_as_dialog_failure_logs_and_returns_false() {
        let io = MockFileWorkflowIo::default();
        io.push_save_dialog_result(Err(SchematicIoError::Io(
            "mock-save-dialog-failure".to_string(),
        )));
        let mut state = AppState::default();
        state.schematic.is_dirty = true;

        let saved = save_schematic_as_with_io(&mut state, &io);

        assert!(!saved);
        assert_eq!(io.save_dialog_calls(), 1);
        assert_eq!(io.save_calls(), 0);
        assert!(state.schematic.is_dirty);
        assert!(state
            .console_messages
            .iter()
            .any(|msg| msg.message.contains("Save As failed:")));
    }

    #[test]
    fn test_save_schematic_as_uses_current_filename_for_dialog_default() {
        let io = MockFileWorkflowIo::default();
        io.push_save_dialog_result(Err(SchematicIoError::Cancelled));
        let mut state = AppState::default();
        state.schematic.current_file = Some(PathBuf::from("projects/current-name.rsch"));

        let saved = save_schematic_as_with_io(&mut state, &io);

        assert!(!saved);
        assert_eq!(io.save_dialog_calls(), 1);
        assert_eq!(
            io.last_save_default_name(),
            Some(Some("current-name.rsch".to_string()))
        );
    }

    #[test]
    fn test_save_schematic_as_without_current_file_uses_none_default() {
        let io = MockFileWorkflowIo::default();
        io.push_save_dialog_result(Err(SchematicIoError::Cancelled));
        let mut state = AppState::default();

        let saved = save_schematic_as_with_io(&mut state, &io);

        assert!(!saved);
        assert_eq!(io.save_dialog_calls(), 1);
        assert_eq!(io.last_save_default_name(), Some(None));
    }
}

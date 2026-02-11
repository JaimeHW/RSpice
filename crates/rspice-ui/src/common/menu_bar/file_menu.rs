use egui::Ui;

use crate::common::app::{AppState, ConfirmationAction};
use crate::common::export_workflow::ExportWorkflowIo;
use crate::common::file_workflow::FileWorkflowIo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileMenuAction {
    New,
    Open,
    Save,
    SaveAs,
    ExportSvg,
    ExportPdf,
    ExportCsvWaveforms,
    ImportVerilogA,
    OpenPreferences,
    Exit,
}

pub(super) fn render_file_menu(
    ui: &mut Ui,
    state: &mut AppState,
    file_workflow_io: &(impl FileWorkflowIo + ?Sized),
    export_workflow_io: &(impl ExportWorkflowIo + ?Sized),
) {
    if ui.button("New").clicked() {
        dispatch_file_menu_action(
            state,
            FileMenuAction::New,
            file_workflow_io,
            export_workflow_io,
        );
        ui.close_menu();
    }
    if ui.button("Open...").clicked() {
        dispatch_file_menu_action(
            state,
            FileMenuAction::Open,
            file_workflow_io,
            export_workflow_io,
        );
        ui.close_menu();
    }

    ui.separator();

    if ui.button("Save").clicked() {
        dispatch_file_menu_action(
            state,
            FileMenuAction::Save,
            file_workflow_io,
            export_workflow_io,
        );
        ui.close_menu();
    }
    if ui.button("Save As...").clicked() {
        dispatch_file_menu_action(
            state,
            FileMenuAction::SaveAs,
            file_workflow_io,
            export_workflow_io,
        );
        ui.close_menu();
    }

    ui.separator();

    ui.menu_button("Export", |ui| {
        if ui.button("SVG...").clicked() {
            dispatch_file_menu_action(
                state,
                FileMenuAction::ExportSvg,
                file_workflow_io,
                export_workflow_io,
            );
            ui.close_menu();
        }
        if ui.button("PDF...").clicked() {
            dispatch_file_menu_action(
                state,
                FileMenuAction::ExportPdf,
                file_workflow_io,
                export_workflow_io,
            );
            ui.close_menu();
        }
        if ui.button("CSV (Waveforms)...").clicked() {
            dispatch_file_menu_action(
                state,
                FileMenuAction::ExportCsvWaveforms,
                file_workflow_io,
                export_workflow_io,
            );
            ui.close_menu();
        }
    });

    ui.menu_button("Import", |ui| {
        if ui.button("Verilog-A Model...").clicked() {
            dispatch_file_menu_action(
                state,
                FileMenuAction::ImportVerilogA,
                file_workflow_io,
                export_workflow_io,
            );
            ui.close_menu();
        }
    });

    ui.separator();

    if ui.button("Preferences...").clicked() {
        dispatch_file_menu_action(
            state,
            FileMenuAction::OpenPreferences,
            file_workflow_io,
            export_workflow_io,
        );
        ui.close_menu();
    }

    ui.separator();

    if ui.button("Exit").clicked() {
        dispatch_file_menu_action(
            state,
            FileMenuAction::Exit,
            file_workflow_io,
            export_workflow_io,
        );
        ui.close_menu();
    }
}

fn dispatch_file_menu_action(
    state: &mut AppState,
    action: FileMenuAction,
    file_workflow_io: &(impl FileWorkflowIo + ?Sized),
    export_workflow_io: &(impl ExportWorkflowIo + ?Sized),
) {
    match action {
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
            let _ = crate::common::file_actions::action_file_save_with_io(state, file_workflow_io);
        }
        FileMenuAction::SaveAs => {
            let _ =
                crate::common::file_actions::action_file_save_as_with_io(state, file_workflow_io);
        }
        FileMenuAction::ExportSvg => {
            super::export_actions::action_export_svg_with_io(state, export_workflow_io)
        }
        FileMenuAction::ExportPdf => open_pdf_export_dialog(state),
        FileMenuAction::ExportCsvWaveforms => {
            super::waveform_export::action_export_csv_with_io(state, export_workflow_io)
        }
        FileMenuAction::ImportVerilogA => open_veriloga_import_dialog(state),
        FileMenuAction::OpenPreferences => open_preferences_dialog(state),
        FileMenuAction::Exit => request_exit(state),
    }
}

fn require_save_confirmation_if_dirty(state: &mut AppState, action: ConfirmationAction) -> bool {
    if !state.schematic.is_dirty {
        return false;
    }

    state.dialogs.confirmation_dialog.show(action);
    true
}

fn open_pdf_export_dialog(state: &mut AppState) {
    state.dialogs.pdf_export_dialog = true;
}

fn open_veriloga_import_dialog(state: &mut AppState) {
    state.dialogs.veriloga_dialog.open();
}

fn open_preferences_dialog(state: &mut AppState) {
    state.dialogs.preferences = true;
}

fn request_exit(state: &mut AppState) {
    if !require_save_confirmation_if_dirty(state, ConfirmationAction::Exit) {
        state.exit_requested = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::{Cell, RefCell};
    use std::collections::VecDeque;
    use std::path::{Path, PathBuf};

    #[derive(Default)]
    struct MockFileMenuWorkflowIo {
        open_dialog_results: RefCell<VecDeque<Result<PathBuf, crate::io::SchematicIoError>>>,
        save_dialog_results: RefCell<VecDeque<Result<PathBuf, crate::io::SchematicIoError>>>,
        load_results:
            RefCell<VecDeque<Result<crate::state::SchematicState, crate::io::SchematicIoError>>>,
        save_results: RefCell<VecDeque<Result<(), crate::io::SchematicIoError>>>,
        open_dialog_calls: Cell<usize>,
        save_dialog_calls: Cell<usize>,
        load_calls: Cell<usize>,
        save_calls: Cell<usize>,
    }

    impl MockFileMenuWorkflowIo {
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
    }

    impl FileWorkflowIo for MockFileMenuWorkflowIo {
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
            _default_name: Option<&str>,
        ) -> Result<PathBuf, crate::io::SchematicIoError> {
            self.save_dialog_calls
                .set(self.save_dialog_calls.get().saturating_add(1));
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

    #[derive(Default)]
    struct MockFileMenuExportWorkflowIo;

    impl ExportWorkflowIo for MockFileMenuExportWorkflowIo {
        fn show_save_dialog(
            &self,
            _config: crate::common::export_workflow::SaveDialogConfig<'_>,
        ) -> Option<PathBuf> {
            None
        }

        fn write_text_file(&self, _path: &Path, _contents: &str) -> Result<(), String> {
            Ok(())
        }

        fn write_waveform_csv(
            &self,
            _dataset: &crate::io::WaveformDataset,
            _path: &Path,
        ) -> Result<(), String> {
            Ok(())
        }
    }

    #[test]
    fn test_open_pdf_export_dialog_sets_visibility() {
        let mut state = AppState::default();
        state.dialogs.pdf_export_dialog = false;

        open_pdf_export_dialog(&mut state);

        assert!(state.dialogs.pdf_export_dialog);
    }

    #[test]
    fn test_open_veriloga_import_dialog_sets_dialog_open() {
        let mut state = AppState::default();
        state.dialogs.veriloga_dialog.close();

        open_veriloga_import_dialog(&mut state);

        assert!(state.dialogs.veriloga_dialog.open);
    }

    #[test]
    fn test_open_preferences_dialog_sets_visibility() {
        let mut state = AppState::default();
        state.dialogs.preferences = false;

        open_preferences_dialog(&mut state);

        assert!(state.dialogs.preferences);
    }

    #[test]
    fn test_request_exit_sets_managed_exit_flag() {
        let mut state = AppState::default();
        state.exit_requested = false;
        state.schematic.is_dirty = false;

        request_exit(&mut state);

        assert!(state.exit_requested);
        assert!(
            !state.dialogs.confirmation_dialog.visible,
            "clean exit path should not show save-confirmation dialog"
        );
    }

    #[test]
    fn test_request_exit_shows_confirmation_when_schematic_is_dirty() {
        let mut state = AppState::default();
        state.exit_requested = false;
        state.schematic.is_dirty = true;

        request_exit(&mut state);

        assert!(
            !state.exit_requested,
            "dirty exit path should defer shutdown until confirmation completes"
        );
        assert!(state.dialogs.confirmation_dialog.visible);
        assert_eq!(
            state.dialogs.confirmation_dialog.pending_action,
            Some(ConfirmationAction::Exit)
        );
    }

    #[test]
    fn test_require_save_confirmation_if_dirty_clean_state_returns_false() {
        let mut state = AppState::default();
        state.schematic.is_dirty = false;

        let requires_confirmation =
            require_save_confirmation_if_dirty(&mut state, ConfirmationAction::FileNew);

        assert!(!requires_confirmation);
        assert!(!state.dialogs.confirmation_dialog.visible);
        assert!(state.dialogs.confirmation_dialog.pending_action.is_none());
    }

    #[test]
    fn test_require_save_confirmation_if_dirty_sets_pending_action_for_dirty_state() {
        let mut state = AppState::default();
        state.schematic.is_dirty = true;

        let requires_confirmation =
            require_save_confirmation_if_dirty(&mut state, ConfirmationAction::FileOpen);

        assert!(requires_confirmation);
        assert!(state.dialogs.confirmation_dialog.visible);
        assert_eq!(
            state.dialogs.confirmation_dialog.pending_action,
            Some(ConfirmationAction::FileOpen)
        );
    }

    #[test]
    fn test_dispatch_file_menu_action_new_dirty_uses_confirmation_without_mutation() {
        use crate::state::{Component, ComponentType, Point};

        let mut state = AppState::default();
        state.schematic.is_dirty = true;
        state.schematic.components.push(Component::new(
            1,
            ComponentType::Resistor,
            Point::new(100, 100),
        ));
        let component_count_before = state.schematic.components.len();
        let io = MockFileMenuWorkflowIo::default();
        let export_io = MockFileMenuExportWorkflowIo::default();

        dispatch_file_menu_action(&mut state, FileMenuAction::New, &io, &export_io);

        assert_eq!(state.schematic.components.len(), component_count_before);
        assert!(state.dialogs.confirmation_dialog.visible);
        assert_eq!(
            state.dialogs.confirmation_dialog.pending_action,
            Some(ConfirmationAction::FileNew)
        );
    }

    #[test]
    fn test_dispatch_file_menu_action_new_clean_creates_new_schematic() {
        use crate::state::{Component, ComponentType, Point};

        let mut state = AppState::default();
        state.schematic.is_dirty = false;
        state.schematic.components.push(Component::new(
            1,
            ComponentType::Resistor,
            Point::new(100, 100),
        ));
        let io = MockFileMenuWorkflowIo::default();
        let export_io = MockFileMenuExportWorkflowIo::default();

        dispatch_file_menu_action(&mut state, FileMenuAction::New, &io, &export_io);

        assert!(state.schematic.components.is_empty());
        assert!(
            state
                .console_messages
                .iter()
                .any(|msg| msg.message.contains("Created new schematic")),
            "expected new-schematic confirmation message"
        );
    }

    #[test]
    fn test_dispatch_file_menu_action_open_dirty_uses_confirmation_without_mutation() {
        use crate::state::{Component, ComponentType, Point};

        let mut state = AppState::default();
        state.schematic.is_dirty = true;
        state.schematic.components.push(Component::new(
            1,
            ComponentType::Resistor,
            Point::new(100, 100),
        ));
        let component_count_before = state.schematic.components.len();
        let io = MockFileMenuWorkflowIo::default();
        let export_io = MockFileMenuExportWorkflowIo::default();

        dispatch_file_menu_action(&mut state, FileMenuAction::Open, &io, &export_io);

        assert_eq!(state.schematic.components.len(), component_count_before);
        assert!(state.dialogs.confirmation_dialog.visible);
        assert_eq!(
            state.dialogs.confirmation_dialog.pending_action,
            Some(ConfirmationAction::FileOpen)
        );
    }

    #[test]
    fn test_dispatch_file_menu_action_open_clean_executes_injected_open_flow() {
        use crate::state::{ComponentType, Point};

        let io = MockFileMenuWorkflowIo::default();
        io.push_open_dialog_result(Ok(PathBuf::from("menu-dispatch-open.rsch")));
        let mut loaded = crate::state::SchematicState::default();
        loaded.add_component(ComponentType::Capacitor, Point::new(12, 34));
        loaded.current_file = Some(PathBuf::from("menu-dispatch-open.rsch"));
        loaded.is_dirty = false;
        io.push_load_result(Ok(loaded));

        let mut state = AppState::default();
        state.schematic.is_dirty = false;
        let export_io = MockFileMenuExportWorkflowIo::default();

        dispatch_file_menu_action(&mut state, FileMenuAction::Open, &io, &export_io);

        assert_eq!(io.open_dialog_calls.get(), 1);
        assert_eq!(io.load_calls.get(), 1);
        assert_eq!(state.schematic.components.len(), 1);
        assert_eq!(state.schematic.components[0].kind, ComponentType::Capacitor);
    }

    #[test]
    fn test_dispatch_file_menu_action_save_without_current_file_uses_save_as_path() {
        use crate::state::{ComponentType, Point};

        let io = MockFileMenuWorkflowIo::default();
        io.push_save_dialog_result(Ok(PathBuf::from("menu-save-as-target.rsch")));
        io.push_save_result(Ok(()));

        let mut state = AppState::default();
        state.schematic.is_dirty = true;
        state
            .schematic
            .add_component(ComponentType::Resistor, Point::new(1, 1));
        let export_io = MockFileMenuExportWorkflowIo::default();

        dispatch_file_menu_action(&mut state, FileMenuAction::Save, &io, &export_io);

        assert_eq!(io.save_dialog_calls.get(), 1);
        assert_eq!(io.save_calls.get(), 1);
        assert_eq!(
            state.schematic.current_file,
            Some(PathBuf::from("menu-save-as-target.rsch"))
        );
        assert!(!state.schematic.is_dirty);
    }

    #[test]
    fn test_dispatch_file_menu_action_save_as_cancel_keeps_dirty_state() {
        let io = MockFileMenuWorkflowIo::default();
        io.push_save_dialog_result(Err(crate::io::SchematicIoError::Cancelled));

        let mut state = AppState::default();
        state.schematic.is_dirty = true;
        let export_io = MockFileMenuExportWorkflowIo::default();

        dispatch_file_menu_action(&mut state, FileMenuAction::SaveAs, &io, &export_io);

        assert_eq!(io.save_dialog_calls.get(), 1);
        assert_eq!(io.save_calls.get(), 0);
        assert!(state.schematic.is_dirty);
    }

    #[test]
    fn test_dispatch_file_menu_action_open_preferences_sets_dialog_visible() {
        let io = MockFileMenuWorkflowIo::default();
        let mut state = AppState::default();
        state.dialogs.preferences = false;
        let export_io = MockFileMenuExportWorkflowIo::default();

        dispatch_file_menu_action(&mut state, FileMenuAction::OpenPreferences, &io, &export_io);

        assert!(state.dialogs.preferences);
    }

    #[test]
    fn test_dispatch_file_menu_action_import_veriloga_opens_dialog() {
        let io = MockFileMenuWorkflowIo::default();
        let mut state = AppState::default();
        state.dialogs.veriloga_dialog.close();
        let export_io = MockFileMenuExportWorkflowIo::default();

        dispatch_file_menu_action(&mut state, FileMenuAction::ImportVerilogA, &io, &export_io);

        assert!(state.dialogs.veriloga_dialog.open);
    }

    #[test]
    fn test_dispatch_file_menu_action_export_pdf_sets_dialog_visible() {
        let io = MockFileMenuWorkflowIo::default();
        let mut state = AppState::default();
        state.dialogs.pdf_export_dialog = false;
        let export_io = MockFileMenuExportWorkflowIo::default();

        dispatch_file_menu_action(&mut state, FileMenuAction::ExportPdf, &io, &export_io);

        assert!(state.dialogs.pdf_export_dialog);
    }

    #[test]
    fn test_dispatch_file_menu_action_exit_clean_requests_exit_immediately() {
        let io = MockFileMenuWorkflowIo::default();
        let mut state = AppState::default();
        state.schematic.is_dirty = false;
        state.exit_requested = false;
        let export_io = MockFileMenuExportWorkflowIo::default();

        dispatch_file_menu_action(&mut state, FileMenuAction::Exit, &io, &export_io);

        assert!(state.exit_requested);
        assert!(!state.dialogs.confirmation_dialog.visible);
    }

    #[test]
    fn test_dispatch_file_menu_action_exit_dirty_defers_with_confirmation() {
        let io = MockFileMenuWorkflowIo::default();
        let mut state = AppState::default();
        state.schematic.is_dirty = true;
        state.exit_requested = false;
        let export_io = MockFileMenuExportWorkflowIo::default();

        dispatch_file_menu_action(&mut state, FileMenuAction::Exit, &io, &export_io);

        assert!(!state.exit_requested);
        assert!(state.dialogs.confirmation_dialog.visible);
        assert_eq!(
            state.dialogs.confirmation_dialog.pending_action,
            Some(ConfirmationAction::Exit)
        );
    }
}

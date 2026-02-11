use egui::Ui;

use crate::common::app::{AppState, ConfirmationAction};

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

pub(super) fn render_file_menu(ui: &mut Ui, state: &mut AppState) {
    if ui.button("New").clicked() {
        dispatch_file_menu_action(state, FileMenuAction::New);
        ui.close_menu();
    }
    if ui.button("Open...").clicked() {
        dispatch_file_menu_action(state, FileMenuAction::Open);
        ui.close_menu();
    }

    ui.separator();

    if ui.button("Save").clicked() {
        dispatch_file_menu_action(state, FileMenuAction::Save);
        ui.close_menu();
    }
    if ui.button("Save As...").clicked() {
        dispatch_file_menu_action(state, FileMenuAction::SaveAs);
        ui.close_menu();
    }

    ui.separator();

    ui.menu_button("Export", |ui| {
        if ui.button("SVG...").clicked() {
            dispatch_file_menu_action(state, FileMenuAction::ExportSvg);
            ui.close_menu();
        }
        if ui.button("PDF...").clicked() {
            dispatch_file_menu_action(state, FileMenuAction::ExportPdf);
            ui.close_menu();
        }
        if ui.button("CSV (Waveforms)...").clicked() {
            dispatch_file_menu_action(state, FileMenuAction::ExportCsvWaveforms);
            ui.close_menu();
        }
    });

    ui.menu_button("Import", |ui| {
        if ui.button("Verilog-A Model...").clicked() {
            dispatch_file_menu_action(state, FileMenuAction::ImportVerilogA);
            ui.close_menu();
        }
    });

    ui.separator();

    if ui.button("Preferences...").clicked() {
        dispatch_file_menu_action(state, FileMenuAction::OpenPreferences);
        ui.close_menu();
    }

    ui.separator();

    if ui.button("Exit").clicked() {
        dispatch_file_menu_action(state, FileMenuAction::Exit);
        ui.close_menu();
    }
}

fn dispatch_file_menu_action(state: &mut AppState, action: FileMenuAction) {
    match action {
        FileMenuAction::New => {
            if require_save_confirmation_if_dirty(state, ConfirmationAction::FileNew) {
                return;
            }
            super::menu_bar_file_actions::action_file_new(state);
        }
        FileMenuAction::Open => {
            if require_save_confirmation_if_dirty(state, ConfirmationAction::FileOpen) {
                return;
            }
            super::menu_bar_file_actions::action_file_open(state);
        }
        FileMenuAction::Save => super::menu_bar_file_actions::action_file_save(state),
        FileMenuAction::SaveAs => super::menu_bar_file_actions::action_file_save_as(state),
        FileMenuAction::ExportSvg => super::menu_bar_export_actions::action_export_svg(state),
        FileMenuAction::ExportPdf => open_pdf_export_dialog(state),
        FileMenuAction::ExportCsvWaveforms => {
            super::menu_bar_waveform_export::action_export_csv(state)
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

        dispatch_file_menu_action(&mut state, FileMenuAction::New);

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

        dispatch_file_menu_action(&mut state, FileMenuAction::New);

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

        dispatch_file_menu_action(&mut state, FileMenuAction::Open);

        assert_eq!(state.schematic.components.len(), component_count_before);
        assert!(state.dialogs.confirmation_dialog.visible);
        assert_eq!(
            state.dialogs.confirmation_dialog.pending_action,
            Some(ConfirmationAction::FileOpen)
        );
    }
}

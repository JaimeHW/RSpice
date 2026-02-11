use egui::Ui;

use crate::common::app::AppState;

pub(super) fn render_file_menu(ui: &mut Ui, state: &mut AppState) {
    if ui.button("New").clicked() {
        super::menu_bar_file_actions::action_file_new(state);
        ui.close_menu();
    }
    if ui.button("Open...").clicked() {
        super::menu_bar_file_actions::action_file_open(state);
        ui.close_menu();
    }

    ui.separator();

    if ui.button("Save").clicked() {
        super::menu_bar_file_actions::action_file_save(state);
        ui.close_menu();
    }
    if ui.button("Save As...").clicked() {
        super::menu_bar_file_actions::action_file_save_as(state);
        ui.close_menu();
    }

    ui.separator();

    ui.menu_button("Export", |ui| {
        if ui.button("SVG...").clicked() {
            super::menu_bar_export_actions::action_export_svg(state);
            ui.close_menu();
        }
        if ui.button("PDF...").clicked() {
            open_pdf_export_dialog(state);
            ui.close_menu();
        }
        if ui.button("CSV (Waveforms)...").clicked() {
            super::menu_bar_waveform_export::action_export_csv(state);
            ui.close_menu();
        }
    });

    ui.menu_button("Import", |ui| {
        if ui.button("Verilog-A Model...").clicked() {
            open_veriloga_import_dialog(state);
            ui.close_menu();
        }
    });

    ui.separator();

    if ui.button("Preferences...").clicked() {
        open_preferences_dialog(state);
        ui.close_menu();
    }

    ui.separator();

    if ui.button("Exit").clicked() {
        request_exit(state);
        ui.close_menu();
    }
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
    state.exit_requested = true;
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

        request_exit(&mut state);

        assert!(state.exit_requested);
    }
}

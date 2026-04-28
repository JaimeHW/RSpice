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


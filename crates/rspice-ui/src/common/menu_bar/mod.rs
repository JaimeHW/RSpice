//! Menu Bar for egui Application
//!
//! Provides a professional menu bar matching the Dioxus version
//! with File, Edit, View, Simulate, Tools, and Help menus.

use egui::{Ui, menu};

use crate::common::app::AppState;
use crate::common::export_workflow::ExportWorkflowIo;
use crate::common::file_workflow::FileWorkflowIo;

mod edit_menu;
mod examples_menu;
mod export_actions;
mod file_menu;
mod help_menu;
mod netlist_compat;
mod simulate_menu;
mod tools_menu;
mod veriloga_cache;
mod view_menu;
mod waveform_export;

/// Render the menu bar
pub(crate) fn render_menu_bar(
    ui: &mut Ui,
    state: &mut AppState,
    file_workflow_io: &(impl FileWorkflowIo + ?Sized),
    export_workflow_io: &(impl ExportWorkflowIo + ?Sized),
) {
    // Add spacing between menu items for a cleaner look
    ui.spacing_mut().item_spacing.x = 8.0;

    menu::bar(ui, |ui| {
        // =====================================================================
        // FILE MENU
        // =====================================================================
        ui.menu_button("File", |ui| {
            file_menu::render_file_menu(ui, state, file_workflow_io, export_workflow_io);
        });

        // =====================================================================
        // EDIT MENU
        // =====================================================================
        ui.menu_button("Edit", |ui| {
            edit_menu::render_edit_menu(ui, state);
        });
        // =====================================================================
        // VIEW MENU
        // =====================================================================
        ui.menu_button("View", |ui| {
            view_menu::render_view_menu(ui, state);
        });
        // =====================================================================
        // SIMULATE MENU
        // =====================================================================
        ui.menu_button("Simulate", |ui| {
            simulate_menu::render_simulate_menu(ui, state, export_workflow_io);
        });
        // =====================================================================
        // TOOLS MENU
        // =====================================================================
        ui.menu_button("Tools", |ui| {
            tools_menu::render_tools_menu(ui, state);
        });
        // =====================================================================
        // EXAMPLES MENU
        // =====================================================================
        ui.menu_button("Examples", |ui| {
            examples_menu::render_examples_menu(ui, state);
        });

        // =====================================================================
        // HELP MENU
        // =====================================================================
        ui.menu_button("Help", |ui| {
            help_menu::render_help_menu(ui, state);
        });
    });
}

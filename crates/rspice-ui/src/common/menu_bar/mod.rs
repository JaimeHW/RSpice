//! Menu Bar for egui Application
//!
//! Provides a professional menu bar matching the Dioxus version
//! with File, Edit, View, Simulate, Tools, and Help menus.

use egui::{menu, Ui};

use crate::common::app::AppState;
use crate::common::export_workflow::ExportWorkflowIo;
use crate::common::file_workflow::FileWorkflowIo;

mod menu_bar_edit_menu;
mod menu_bar_examples_menu;
mod menu_bar_export_actions;
mod menu_bar_file_actions;
mod menu_bar_file_menu;
mod menu_bar_help_menu;
mod menu_bar_netlist_compat;
mod menu_bar_simulate_menu;
mod menu_bar_tools_menu;
mod menu_bar_veriloga_cache;
mod menu_bar_view_menu;
mod menu_bar_waveform_export;

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
            menu_bar_file_menu::render_file_menu(ui, state, file_workflow_io, export_workflow_io);
        });

        // =====================================================================
        // EDIT MENU
        // =====================================================================
        ui.menu_button("Edit", |ui| {
            menu_bar_edit_menu::render_edit_menu(ui, state);
        });
        // =====================================================================
        // VIEW MENU
        // =====================================================================
        ui.menu_button("View", |ui| {
            menu_bar_view_menu::render_view_menu(ui, state);
        });
        // =====================================================================
        // SIMULATE MENU
        // =====================================================================
        ui.menu_button("Simulate", |ui| {
            menu_bar_simulate_menu::render_simulate_menu(ui, state, export_workflow_io);
        });
        // =====================================================================
        // TOOLS MENU
        // =====================================================================
        ui.menu_button("Tools", |ui| {
            menu_bar_tools_menu::render_tools_menu(ui, state);
        });
        // =====================================================================
        // EXAMPLES MENU
        // =====================================================================
        ui.menu_button("Examples", |ui| {
            menu_bar_examples_menu::render_examples_menu(ui, state);
        });

        // =====================================================================
        // HELP MENU
        // =====================================================================
        ui.menu_button("Help", |ui| {
            menu_bar_help_menu::render_help_menu(ui, state);
        });
    });
}

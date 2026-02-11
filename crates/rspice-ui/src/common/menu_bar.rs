//! Menu Bar for egui Application
//!
//! Provides a professional menu bar matching the Dioxus version
//! with File, Edit, View, Simulate, Tools, and Help menus.

use egui::{menu, Ui};

use crate::common::app::AppState;

#[path = "menu_bar_edit_menu.rs"]
mod menu_bar_edit_menu;
#[path = "menu_bar_examples_menu.rs"]
mod menu_bar_examples_menu;
#[path = "menu_bar_export_actions.rs"]
mod menu_bar_export_actions;
#[path = "menu_bar_file_actions.rs"]
mod menu_bar_file_actions;
#[path = "menu_bar_file_menu.rs"]
mod menu_bar_file_menu;
#[path = "menu_bar_help_menu.rs"]
mod menu_bar_help_menu;
#[path = "menu_bar_netlist_compat.rs"]
mod menu_bar_netlist_compat;
#[path = "menu_bar_simulate_menu.rs"]
mod menu_bar_simulate_menu;
#[path = "menu_bar_tools_menu.rs"]
mod menu_bar_tools_menu;
#[path = "menu_bar_veriloga_cache.rs"]
mod menu_bar_veriloga_cache;
#[path = "menu_bar_view_menu.rs"]
mod menu_bar_view_menu;
#[path = "menu_bar_waveform_export.rs"]
mod menu_bar_waveform_export;

/// Render the menu bar
pub fn render_menu_bar(ui: &mut Ui, state: &mut AppState) {
    // Add spacing between menu items for a cleaner look
    ui.spacing_mut().item_spacing.x = 8.0;

    menu::bar(ui, |ui| {
        // =====================================================================
        // FILE MENU
        // =====================================================================
        ui.menu_button("File", |ui| {
            menu_bar_file_menu::render_file_menu(ui, state);
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
            menu_bar_simulate_menu::render_simulate_menu(ui, state);
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

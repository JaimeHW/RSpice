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
            if ui.button("New").clicked() {
                menu_bar_file_actions::action_file_new(state);
                ui.close_menu();
            }
            if ui.button("Open...").clicked() {
                menu_bar_file_actions::action_file_open(state);
                ui.close_menu();
            }

            ui.separator();

            if ui.button("Save").clicked() {
                menu_bar_file_actions::action_file_save(state);
                ui.close_menu();
            }
            if ui.button("Save As...").clicked() {
                menu_bar_file_actions::action_file_save_as(state);
                ui.close_menu();
            }

            ui.separator();

            ui.menu_button("Export", |ui| {
                if ui.button("SVG...").clicked() {
                    menu_bar_export_actions::action_export_svg(state);
                    ui.close_menu();
                }
                if ui.button("PDF...").clicked() {
                    state.dialogs.pdf_export_dialog = true;
                    ui.close_menu();
                }
                if ui.button("CSV (Waveforms)...").clicked() {
                    menu_bar_waveform_export::action_export_csv(state);
                    ui.close_menu();
                }
            });

            ui.menu_button("Import", |ui| {
                if ui.button("Verilog-A Model...").clicked() {
                    state.dialogs.veriloga_dialog.open();
                    ui.close_menu();
                }
            });

            ui.separator();

            if ui.button("Preferences...").clicked() {
                state.dialogs.preferences = true;
                ui.close_menu();
            }

            ui.separator();

            if ui.button("Exit").clicked() {
                std::process::exit(0);
            }
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
            if ui.button("Keyboard Shortcuts    F1").clicked() {
                state.dialogs.shortcuts_help = true;
                ui.close_menu();
            }

            ui.separator();

            // Documentation submenu
            ui.menu_button("Documentation", |ui| {
                if ui.button("User Guide").clicked() {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(
                        "User Guide: See docs/user_guide.md",
                    ));
                    ui.close_menu();
                }
                if ui.button("SPICE Reference").clicked() {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(
                        "SPICE Reference: See docs/spice_reference.md",
                    ));
                    ui.close_menu();
                }
                if ui.button("Analysis Guide").clicked() {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(
                        "Analysis Guide: See docs/analysis_guide.md",
                    ));
                    ui.close_menu();
                }
                if ui.button("Model Library").clicked() {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(
                        "Model Library: See docs/models.md",
                    ));
                    ui.close_menu();
                }
            });

            // Examples submenu
            ui.menu_button("Examples", |ui| {
                if ui.button("RC Low-Pass Filter").clicked() {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(
                        "Loading RC filter example...",
                    ));
                    ui.close_menu();
                }
                if ui.button("Inverter Chain").clicked() {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(
                        "Loading inverter chain example...",
                    ));
                    ui.close_menu();
                }
                if ui.button("Operational Amplifier").clicked() {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(
                        "Loading op-amp example...",
                    ));
                    ui.close_menu();
                }
                if ui.button("PLL Circuit").clicked() {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(
                        "Loading PLL example...",
                    ));
                    ui.close_menu();
                }
            });

            ui.separator();

            if ui.button("Check for Updates...").clicked() {
                state.push_user_message(crate::common::app::ConsoleMessage::info(
                    "You are running the latest version",
                ));
                ui.close_menu();
            }

            if ui.button("Report Issue...").clicked() {
                state.push_user_message(crate::common::app::ConsoleMessage::info(
                    "Please report issues at: github.com/rspice/issues",
                ));
                ui.close_menu();
            }

            ui.separator();

            if ui.button("About RSpice").clicked() {
                state.dialogs.about = true;
                ui.close_menu();
            }
        });
    });
}

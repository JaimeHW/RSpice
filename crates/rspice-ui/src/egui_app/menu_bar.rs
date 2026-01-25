//! Menu Bar for egui Application
//!
//! Provides a professional menu bar matching the Dioxus version
//! with File, Edit, View, Simulate, Tools, and Help menus.

use egui::{menu, Ui};

use super::app::AppState;

/// Render the menu bar
pub fn render_menu_bar(ui: &mut Ui, state: &mut AppState) {
    menu::bar(ui, |ui| {
        // =====================================================================
        // FILE MENU
        // =====================================================================
        ui.menu_button("File", |ui| {
            if ui.button("New").clicked() {
                action_file_new(state);
                ui.close_menu();
            }
            if ui.button("Open...").clicked() {
                action_file_open(state);
                ui.close_menu();
            }

            ui.separator();

            if ui.button("Save").clicked() {
                action_file_save(state);
                ui.close_menu();
            }
            if ui.button("Save As...").clicked() {
                action_file_save_as(state);
                ui.close_menu();
            }

            ui.separator();

            ui.menu_button("Export", |ui| {
                if ui.button("SVG...").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info("SVG export: Coming soon"));
                    ui.close_menu();
                }
                if ui.button("PDF...").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info("PDF export: Coming soon"));
                    ui.close_menu();
                }
                if ui.button("CSV (Waveforms)...").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info("CSV export: Coming soon"));
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
            if ui.button("Undo        Ctrl+Z").clicked() {
                // TODO: Implement undo
                state
                    .console_messages
                    .push(super::app::ConsoleMessage::info("Undo: Coming soon"));
                ui.close_menu();
            }
            if ui.button("Redo        Ctrl+Y").clicked() {
                // TODO: Implement redo
                state
                    .console_messages
                    .push(super::app::ConsoleMessage::info("Redo: Coming soon"));
                ui.close_menu();
            }

            ui.separator();

            if ui.button("Cut         Ctrl+X").clicked() {
                state.schematic.copy_selection();
                state.schematic.delete_selection();
                ui.close_menu();
            }
            if ui.button("Copy        Ctrl+C").clicked() {
                state.schematic.copy_selection();
                ui.close_menu();
            }
            if ui.button("Paste       Ctrl+V").clicked() {
                use crate::state::Point;
                state.schematic.paste_at(Point::new(200, 200));
                ui.close_menu();
            }
            if ui.button("Delete      Del").clicked() {
                state.schematic.delete_selection();
                ui.close_menu();
            }

            ui.separator();

            if ui.button("Select All  Ctrl+A").clicked() {
                state.schematic.selection.clear();
                for comp in &state.schematic.components {
                    state.schematic.selection.select_component(comp.id);
                }
                for wire in &state.schematic.wires {
                    state.schematic.selection.select_wire(wire.id);
                }
                ui.close_menu();
            }

            if ui.button("Duplicate   Ctrl+D").clicked() {
                use crate::state::Point;
                state.schematic.copy_selection();
                state.schematic.paste_at(Point::new(220, 220));
                ui.close_menu();
            }
        });

        // =====================================================================
        // VIEW MENU
        // =====================================================================
        ui.menu_button("View", |ui| {
            if ui.button("Zoom In     Ctrl++").clicked() {
                state.schematic.zoom = (state.schematic.zoom * 1.25).min(4.0);
                ui.close_menu();
            }
            if ui.button("Zoom Out    Ctrl+-").clicked() {
                state.schematic.zoom = (state.schematic.zoom / 1.25).max(0.25);
                ui.close_menu();
            }
            if ui.button("Zoom to Fit").clicked() {
                state.schematic.zoom_to_fit(800.0, 600.0);
                ui.close_menu();
            }
            if ui.button("Zoom 100%   Ctrl+0").clicked() {
                state.schematic.zoom = 1.0;
                ui.close_menu();
            }

            ui.separator();

            // Panel toggles with checkmarks
            let browser_label = if state.panels.project_browser {
                "✓ Library Browser"
            } else {
                "  Library Browser"
            };
            if ui.button(browser_label).clicked() {
                state.panels.project_browser = !state.panels.project_browser;
                ui.close_menu();
            }

            let waveform_label = if state.panels.waveform {
                "✓ Waveform Viewer"
            } else {
                "  Waveform Viewer"
            };
            if ui.button(waveform_label).clicked() {
                state.panels.waveform = !state.panels.waveform;
                ui.close_menu();
            }

            let console_label = if state.panels.console {
                "✓ Console"
            } else {
                "  Console"
            };
            if ui.button(console_label).clicked() {
                state.panels.console = !state.panels.console;
                ui.close_menu();
            }

            let props_label = if state.panels.properties {
                "✓ Properties"
            } else {
                "  Properties"
            };
            if ui.button(props_label).clicked() {
                state.panels.properties = !state.panels.properties;
                ui.close_menu();
            }
        });

        // =====================================================================
        // SIMULATE MENU
        // =====================================================================
        ui.menu_button("Simulate", |ui| {
            if ui.button("Run Simulation").clicked() {
                if state.schematic.components.is_empty() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::warning(
                            "No circuit to simulate. Add components first.",
                        ));
                } else {
                    state.simulation.trigger_simulation = true;
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info("Simulation started..."));
                }
                ui.close_menu();
            }

            if ui.button("Stop Simulation").clicked() {
                state.simulation.is_running = false;
                state
                    .console_messages
                    .push(super::app::ConsoleMessage::warning("Simulation stopped"));
                ui.close_menu();
            }

            ui.separator();

            if ui.button("Setup...").clicked() {
                state.dialogs.simulation_dialog = true;
                ui.close_menu();
            }

            if ui.button("Options...").clicked() {
                state.dialogs.simulation_options = true;
                ui.close_menu();
            }
        });

        // =====================================================================
        // TOOLS MENU
        // =====================================================================
        ui.menu_button("Tools", |ui| {
            if ui.button("Verilog-A Compiler...").clicked() {
                state
                    .console_messages
                    .push(super::app::ConsoleMessage::info("Verilog-A: Coming soon"));
                ui.close_menu();
            }

            if ui.button("Model Browser...").clicked() {
                state
                    .console_messages
                    .push(super::app::ConsoleMessage::info(
                        "Model Browser: Coming soon",
                    ));
                ui.close_menu();
            }

            if ui.button("Design Rule Check").clicked() {
                state
                    .console_messages
                    .push(super::app::ConsoleMessage::info("DRC: Coming soon"));
                ui.close_menu();
            }
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

            if ui.button("About RSpice").clicked() {
                state.dialogs.about = true;
                ui.close_menu();
            }
        });
    });
}

// =============================================================================
// Action Handlers (for menu-specific logic)
// =============================================================================

fn action_file_new(state: &mut AppState) {
    if state.schematic.is_dirty {
        log::warn!("New schematic requested but current has unsaved changes");
    }
    state.schematic = crate::state::SchematicState::default();
    state
        .console_messages
        .push(super::app::ConsoleMessage::info("Created new schematic"));
}

fn action_file_open(state: &mut AppState) {
    state
        .console_messages
        .push(super::app::ConsoleMessage::info("Open: Coming soon"));
}

fn action_file_save(state: &mut AppState) {
    state
        .console_messages
        .push(super::app::ConsoleMessage::info("Save: Coming soon"));
}

fn action_file_save_as(state: &mut AppState) {
    state
        .console_messages
        .push(super::app::ConsoleMessage::info("Save As: Coming soon"));
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_file_new_clears_schematic() {
        let mut state = AppState::default();

        // Add a component using correct API
        use crate::state::{Component, ComponentType, Point};
        let comp = Component::new(1, ComponentType::Resistor, Point::new(100, 100))
            .with_name_value("R1", "1k");
        state.schematic.components.push(comp);
        assert!(!state.schematic.components.is_empty());

        // New schematic
        action_file_new(&mut state);

        assert!(state.schematic.components.is_empty());
        assert!(!state.console_messages.is_empty());
    }

    #[test]
    fn test_action_file_open_adds_console_message() {
        let mut state = AppState::default();
        assert!(state.console_messages.is_empty());

        action_file_open(&mut state);

        assert!(!state.console_messages.is_empty());
    }

    #[test]
    fn test_action_file_save_adds_console_message() {
        let mut state = AppState::default();
        assert!(state.console_messages.is_empty());

        action_file_save(&mut state);

        assert!(!state.console_messages.is_empty());
    }
}

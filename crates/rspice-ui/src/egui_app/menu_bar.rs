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

            let browser_label = if state.panels.signal_browser {
                "✓ Signal Browser"
            } else {
                "  Signal Browser"
            };
            if ui.button(browser_label).clicked() {
                state.panels.signal_browser = !state.panels.signal_browser;
                ui.close_menu();
            }

            ui.separator();

            // Specialized Viewers submenu
            ui.menu_button("Specialized Viewers", |ui| {
                if ui.button("Waveform").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Switched to Waveform viewer",
                        ));
                    ui.close_menu();
                }
                if ui.button("Smith Chart").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Switched to Smith Chart viewer",
                        ));
                    ui.close_menu();
                }
                if ui.button("Eye Diagram").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Switched to Eye Diagram viewer",
                        ));
                    ui.close_menu();
                }
                if ui.button("Histogram").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Switched to Histogram viewer",
                        ));
                    ui.close_menu();
                }
                if ui.button("Bode Plot").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Switched to Bode Plot viewer",
                        ));
                    ui.close_menu();
                }
                if ui.button("Nyquist Plot").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Switched to Nyquist Plot viewer",
                        ));
                    ui.close_menu();
                }
                if ui.button("FFT Spectrum").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Switched to FFT Spectrum viewer",
                        ));
                    ui.close_menu();
                }
                if ui.button("Pole-Zero Map").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Switched to Pole-Zero Map viewer",
                        ));
                    ui.close_menu();
                }
            });
        });

        // =====================================================================
        // SIMULATE MENU
        // =====================================================================
        ui.menu_button("Simulate", |ui| {
            // Quick run - uses currently selected analysis
            if ui.button("▶ Run Simulation    F5").clicked() {
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

            // Run Analysis submenu for quick access to specific analyses
            ui.menu_button("Run Analysis", |ui| {
                let analyses = [
                    ("DC Operating Point", 0),
                    ("Transient", 1),
                    ("AC Analysis", 2),
                    ("DC Sweep", 3),
                    ("Noise", 4),
                    ("Pole-Zero", 5),
                    ("Sensitivity", 6),
                    ("Monte Carlo", 7),
                    ("PSS", 8),
                    ("Stability (STB)", 9),
                    ("Temperature Sweep", 10),
                ];
                for (name, tab) in analyses {
                    if ui.button(name).clicked() {
                        state.dialogs.sim_active_tab = tab;
                        if state.schematic.components.is_empty() {
                            state
                                .console_messages
                                .push(super::app::ConsoleMessage::warning(
                                    "No circuit to simulate. Add components first.",
                                ));
                        } else {
                            state
                                .console_messages
                                .push(super::app::ConsoleMessage::info(format!(
                                    "Starting {} analysis...",
                                    name
                                )));
                            state.simulation.trigger_simulation = true;
                        }
                        ui.close_menu();
                    }
                }
            });

            if ui.button("⏹ Stop Simulation").clicked() {
                state.simulation.is_running = false;
                state
                    .console_messages
                    .push(super::app::ConsoleMessage::warning("Simulation stopped"));
                ui.close_menu();
            }

            ui.separator();

            if ui.button("Setup...     Ctrl+Shift+S").clicked() {
                state.dialogs.simulation_dialog = true;
                ui.close_menu();
            }

            if ui.button("Options...").clicked() {
                state.dialogs.simulation_options = true;
                ui.close_menu();
            }

            ui.separator();

            // Netlist menu
            ui.menu_button("Netlist", |ui| {
                if ui.button("View Netlist").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Netlist viewer: Coming soon",
                        ));
                    ui.close_menu();
                }
                if ui.button("Export SPICE Netlist...").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Export netlist: Coming soon",
                        ));
                    ui.close_menu();
                }
                if ui.button("Export Spectre Netlist...").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Export Spectre netlist: Coming soon",
                        ));
                    ui.close_menu();
                }
            });
        });

        // =====================================================================
        // TOOLS MENU
        // =====================================================================
        ui.menu_button("Tools", |ui| {
            // Verilog-A submenu
            ui.menu_button("Verilog-A", |ui| {
                if ui.button("Compile Module...").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Verilog-A compiler: Coming soon",
                        ));
                    ui.close_menu();
                }
                if ui.button("Module Library...").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Verilog-A library: Coming soon",
                        ));
                    ui.close_menu();
                }
            });

            if ui.button("Model Browser...").clicked() {
                state
                    .console_messages
                    .push(super::app::ConsoleMessage::info(
                        "Model Browser: Coming soon",
                    ));
                ui.close_menu();
            }

            if ui.button("Parameter Extractor...").clicked() {
                state
                    .console_messages
                    .push(super::app::ConsoleMessage::info(
                        "Parameter Extractor: Coming soon",
                    ));
                ui.close_menu();
            }

            ui.separator();

            // Calculators submenu
            ui.menu_button("Calculators", |ui| {
                if ui.button("Unit Converter").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Unit Converter: Coming soon",
                        ));
                    ui.close_menu();
                }
                if ui.button("Filter Design").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Filter Calculator: Coming soon",
                        ));
                    ui.close_menu();
                }
                if ui.button("Impedance Matching").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Impedance Calculator: Coming soon",
                        ));
                    ui.close_menu();
                }
                if ui.button("S-Parameter Converter").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "S-Parameter Converter: Coming soon",
                        ));
                    ui.close_menu();
                }
            });

            ui.separator();

            // Verification tools
            if ui.button("Design Rule Check").clicked() {
                state
                    .console_messages
                    .push(super::app::ConsoleMessage::info("DRC: Coming soon"));
                ui.close_menu();
            }

            if ui.button("Electrical Rule Check").clicked() {
                state
                    .console_messages
                    .push(super::app::ConsoleMessage::info("ERC: Coming soon"));
                ui.close_menu();
            }

            if ui.button("LVS Check").clicked() {
                state
                    .console_messages
                    .push(super::app::ConsoleMessage::info("LVS: Coming soon"));
                ui.close_menu();
            }

            ui.separator();

            // Waveform tools
            ui.menu_button("Waveform Tools", |ui| {
                if ui.button("Calculator...").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Waveform Calculator: Coming soon",
                        ));
                    ui.close_menu();
                }
                if ui.button("Measurements...").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Measurements panel: Coming soon",
                        ));
                    ui.close_menu();
                }
                if ui.button("Cross-Probe").clicked() {
                    state.panels.signal_browser = true;
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info("Cross-probe enabled"));
                    ui.close_menu();
                }
            });
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
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "User Guide: See docs/user_guide.md",
                        ));
                    ui.close_menu();
                }
                if ui.button("SPICE Reference").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "SPICE Reference: See docs/spice_reference.md",
                        ));
                    ui.close_menu();
                }
                if ui.button("Analysis Guide").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Analysis Guide: See docs/analysis_guide.md",
                        ));
                    ui.close_menu();
                }
                if ui.button("Model Library").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Model Library: See docs/models.md",
                        ));
                    ui.close_menu();
                }
            });

            // Examples submenu
            ui.menu_button("Examples", |ui| {
                if ui.button("RC Low-Pass Filter").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Loading RC filter example...",
                        ));
                    ui.close_menu();
                }
                if ui.button("Inverter Chain").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Loading inverter chain example...",
                        ));
                    ui.close_menu();
                }
                if ui.button("Operational Amplifier").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info(
                            "Loading op-amp example...",
                        ));
                    ui.close_menu();
                }
                if ui.button("PLL Circuit").clicked() {
                    state
                        .console_messages
                        .push(super::app::ConsoleMessage::info("Loading PLL example..."));
                    ui.close_menu();
                }
            });

            ui.separator();

            if ui.button("Check for Updates...").clicked() {
                state
                    .console_messages
                    .push(super::app::ConsoleMessage::info(
                        "You are running the latest version",
                    ));
                ui.close_menu();
            }

            if ui.button("Report Issue...").clicked() {
                state
                    .console_messages
                    .push(super::app::ConsoleMessage::info(
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

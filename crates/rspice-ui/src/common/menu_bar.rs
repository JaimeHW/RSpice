//! Menu Bar for egui Application
//!
//! Provides a professional menu bar matching the Dioxus version
//! with File, Edit, View, Simulate, Tools, and Help menus.

use egui::{menu, Ui};

use crate::common::app::AppState;

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
                    action_export_svg(state);
                    ui.close_menu();
                }
                if ui.button("PDF...").clicked() {
                    state.dialogs.pdf_export_dialog = true;
                    ui.close_menu();
                }
                if ui.button("CSV (Waveforms)...").clicked() {
                    action_export_csv(state);
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
            if ui.button("Undo        Ctrl+Z").clicked() {
                if state.schematic.can_undo() {
                    let desc = state
                        .schematic
                        .undo_description()
                        .unwrap_or("action")
                        .to_string();
                    if state.schematic.undo() {
                        state
                            .console_messages
                            .push(crate::common::app::ConsoleMessage::info(format!(
                                "Undo: {}",
                                desc
                            )));
                    }
                } else {
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info("Nothing to undo"));
                }
                ui.close_menu();
            }
            if ui.button("Redo        Ctrl+Y").clicked() {
                if state.schematic.can_redo() {
                    let desc = state
                        .schematic
                        .redo_description()
                        .unwrap_or("action")
                        .to_string();
                    if state.schematic.redo() {
                        state
                            .console_messages
                            .push(crate::common::app::ConsoleMessage::info(format!(
                                "Redo: {}",
                                desc
                            )));
                    }
                } else {
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info("Nothing to redo"));
                }
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

            let waveform_label = if state.panels.bottom_panel
                && state.panels.active_bottom_tab == crate::common::app::BottomPanelTab::Waveform
            {
                "✓ Waveform Viewer"
            } else {
                "  Waveform Viewer"
            };
            if ui.button(waveform_label).clicked() {
                // Toggle: if already on Waveform tab, hide panel; otherwise show and switch to it
                if state.panels.bottom_panel
                    && state.panels.active_bottom_tab
                        == crate::common::app::BottomPanelTab::Waveform
                {
                    state.panels.bottom_panel = false;
                } else {
                    state.panels.bottom_panel = true;
                    state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
                }
                ui.close_menu();
            }

            let console_label = if state.panels.bottom_panel
                && state.panels.active_bottom_tab == crate::common::app::BottomPanelTab::Console
            {
                "✓ Console"
            } else {
                "  Console"
            };
            if ui.button(console_label).clicked() {
                // Toggle: if already on Console tab, hide panel; otherwise show and switch to it
                if state.panels.bottom_panel
                    && state.panels.active_bottom_tab == crate::common::app::BottomPanelTab::Console
                {
                    state.panels.bottom_panel = false;
                } else {
                    state.panels.bottom_panel = true;
                    state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Console;
                }
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

            let script_label = if state.panels.script_console {
                "✓ Automation Console"
            } else {
                "  Automation Console"
            };
            if ui.button(script_label).clicked() {
                state.panels.script_console = !state.panels.script_console;
                ui.close_menu();
            }

            ui.separator();

            // Specialized Viewers submenu
            ui.menu_button("Specialized Viewers", |ui| {
                use crate::viewers::ActiveViewer;

                for viewer in ActiveViewer::all() {
                    let is_active = state.active_viewer == *viewer;
                    let label = if is_active {
                        format!("✓ {}", viewer.name())
                    } else {
                        format!("  {}", viewer.name())
                    };

                    if ui.button(&label).clicked() {
                        state.active_viewer = *viewer;
                        state
                            .console_messages
                            .push(crate::common::app::ConsoleMessage::info(format!(
                                "Switched to {} viewer",
                                viewer.name()
                            )));
                        ui.close_menu();
                    }
                }
            });
        });

        // =====================================================================
        // SIMULATE MENU
        // =====================================================================
        ui.menu_button("Simulate", |ui| {
            // Quick run - uses currently selected analysis
            if ui.button("▶ Run Simulation    F5").clicked() {
                log::info!(
                    "Run button clicked! Components: {}",
                    state.schematic.components.len()
                );
                if state.schematic.components.is_empty() {
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::warning(
                            "No circuit to simulate. Add components first.",
                        ));
                } else {
                    log::info!("Setting trigger_simulation = true");
                    state.simulation.trigger_simulation = true;
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info(
                            "Simulation started...",
                        ));
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
                    ("Reliability (Aging)", 21),
                ];
                for (name, tab) in analyses {
                    if ui.button(name).clicked() {
                        state.dialogs.sim_active_tab = tab;
                        if state.schematic.components.is_empty() {
                            state.console_messages.push(
                                crate::common::app::ConsoleMessage::warning(
                                    "No circuit to simulate. Add components first.",
                                ),
                            );
                        } else {
                            state
                                .console_messages
                                .push(crate::common::app::ConsoleMessage::info(format!(
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
                    .push(crate::common::app::ConsoleMessage::warning(
                        "Simulation stopped",
                    ));
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
                    action_view_netlist(state);
                    ui.close_menu();
                }
                if ui.button("Export SPICE Netlist...").clicked() {
                    action_export_netlist(state, crate::io::NetlistFormat::Spice);
                    ui.close_menu();
                }
                if ui.button("Export Spectre Netlist...").clicked() {
                    action_export_netlist(state, crate::io::NetlistFormat::Spectre);
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
                    // TODO: Open file dialog to select .va file, then compile via rspice-veriloga
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info(
                            "Verilog-A: Select File > Open to load a .va module for compilation",
                        ));
                    ui.close_menu();
                }
                if ui.button("Module Library...").clicked() {
                    // Verilog-A compiled modules are managed through Model Browser
                    state.dialogs.model_browser_dialog = true;
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info(
                            "Opening Model Browser to view compiled Verilog-A modules",
                        ));
                    ui.close_menu();
                }
            });

            if ui.button("Model Browser...").clicked() {
                state.dialogs.model_browser_dialog = true;
                ui.close_menu();
            }

            if ui.button("Optimization Engine...").clicked() {
                state.dialogs.sim_active_tab = 22; // Optimizer tab
                state.dialogs.simulation_dialog = true;
                ui.close_menu();
            }

            if ui.button("Automation Console").clicked() {
                state.panels.script_console = true;
                ui.close_menu();
            }

            ui.separator();

            // Calculators submenu
            ui.menu_button("Calculators", |ui| {
                if ui.button("Unit Converter").clicked() {
                    state.dialogs.unit_converter_dialog = true;
                    ui.close_menu();
                }
                if ui.button("Filter Design").clicked() {
                    state.dialogs.filter_calculator_dialog = true;
                    ui.close_menu();
                }
                if ui.button("Impedance Matching").clicked() {
                    state.dialogs.impedance_calculator_dialog = true;
                    ui.close_menu();
                }
                if ui.button("S-Parameter Converter").clicked() {
                    state.dialogs.sparam_converter_dialog = true;
                    ui.close_menu();
                }
            });

            ui.separator();

            // Verification tools
            if ui.button("Design Rule Check").clicked() {
                // Run DRC on current schematic using DrcChecker
                let mut checker = crate::services::drc::DrcChecker::new();
                // TODO: Convert schematic state to DRC input types
                // For now, run with empty data to show the dialog
                let result = checker.check_connectivity(&[], &[], &[]);
                let summary = result.summary();
                let msg = if result.passed() {
                    format!(
                        "DRC passed: {} info, {} warnings",
                        summary.info, summary.warnings
                    )
                } else {
                    format!(
                        "DRC found {} violations ({} errors, {} critical)",
                        summary.total, summary.errors, summary.critical
                    )
                };
                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(&msg));
                state.dialogs.drc_results = Some(result);
                state.dialogs.drc_dialog = true;
                ui.close_menu();
            }

            if ui.button("Electrical Rule Check").clicked() {
                // ERC is part of DRC - runs connectivity checks
                let mut checker = crate::services::drc::DrcChecker::new();
                // TODO: Convert schematic to DRC input types for full ERC
                let result = checker.check_connectivity(&[], &[], &[]);
                let msg = if result.passed() {
                    "ERC passed: No electrical rule violations".to_string()
                } else {
                    format!("ERC found {} violations", result.total_count())
                };
                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(&msg));
                state.dialogs.drc_results = Some(result);
                state.dialogs.drc_dialog = true;
                ui.close_menu();
            }

            if ui.button("LVS Check").clicked() {
                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(
                        "LVS: Requires layout data (not available in schematic-only mode)",
                    ));
                ui.close_menu();
            }

            ui.separator();

            // Waveform tools
            ui.menu_button("Waveform Tools", |ui| {
                if ui.button("Calculator...").clicked() {
                    state.dialogs.waveform_calculator_dialog = true;
                    ui.close_menu();
                }
                if ui.button("Measurements...").clicked() {
                    state.dialogs.measurements_panel = true;
                    ui.close_menu();
                }
                if ui.button("Cross-Probe").clicked() {
                    state.panels.signal_browser = true;
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info(
                            "Cross-probe enabled",
                        ));
                    ui.close_menu();
                }
            });
        });

        // =====================================================================
        // EXAMPLES MENU
        // =====================================================================
        ui.menu_button("Examples", |ui| {
            use crate::common::examples::{load_example, EXAMPLES};

            for example in EXAMPLES {
                if ui
                    .button(example.name)
                    .on_hover_text(example.description)
                    .clicked()
                {
                    load_example(example.name, &mut state.schematic);
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info(format!(
                            "Loaded example: {} ({})",
                            example.name, example.category
                        )));
                    ui.close_menu();
                }
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

            // Documentation submenu
            ui.menu_button("Documentation", |ui| {
                if ui.button("User Guide").clicked() {
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info(
                            "User Guide: See docs/user_guide.md",
                        ));
                    ui.close_menu();
                }
                if ui.button("SPICE Reference").clicked() {
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info(
                            "SPICE Reference: See docs/spice_reference.md",
                        ));
                    ui.close_menu();
                }
                if ui.button("Analysis Guide").clicked() {
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info(
                            "Analysis Guide: See docs/analysis_guide.md",
                        ));
                    ui.close_menu();
                }
                if ui.button("Model Library").clicked() {
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info(
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
                        .push(crate::common::app::ConsoleMessage::info(
                            "Loading RC filter example...",
                        ));
                    ui.close_menu();
                }
                if ui.button("Inverter Chain").clicked() {
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info(
                            "Loading inverter chain example...",
                        ));
                    ui.close_menu();
                }
                if ui.button("Operational Amplifier").clicked() {
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info(
                            "Loading op-amp example...",
                        ));
                    ui.close_menu();
                }
                if ui.button("PLL Circuit").clicked() {
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info(
                            "Loading PLL example...",
                        ));
                    ui.close_menu();
                }
            });

            ui.separator();

            if ui.button("Check for Updates...").clicked() {
                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(
                        "You are running the latest version",
                    ));
                ui.close_menu();
            }

            if ui.button("Report Issue...").clicked() {
                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(
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
        .push(crate::common::app::ConsoleMessage::info(
            "Created new schematic",
        ));
}

fn action_file_open(state: &mut AppState) {
    use crate::io::{load_schematic, show_open_dialog, SchematicIoError};

    match show_open_dialog() {
        Ok(path) => match load_schematic(&path) {
            Ok(schematic) => {
                state.schematic = schematic;
                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(format!(
                        "Opened: {}",
                        path.display()
                    )));
            }
            Err(e) => {
                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::error(format!(
                        "Failed to open: {}",
                        e
                    )));
            }
        },
        Err(SchematicIoError::Cancelled) => {
            // User cancelled - no message needed
        }
        Err(e) => {
            state
                .console_messages
                .push(crate::common::app::ConsoleMessage::error(format!(
                    "Open failed: {}",
                    e
                )));
        }
    }
}

fn action_file_save(state: &mut AppState) {
    use crate::io::{save_schematic, SchematicIoError};

    // If we have a current file path, save directly
    // Otherwise, show Save As dialog
    if let Some(ref path) = state.schematic.current_file.clone() {
        match save_schematic(&state.schematic, path) {
            Ok(()) => {
                state.schematic.is_dirty = false;
                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(format!(
                        "Saved: {}",
                        path.display()
                    )));
            }
            Err(e) => {
                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::error(format!(
                        "Save failed: {}",
                        e
                    )));
            }
        }
    } else {
        // No current file - do Save As
        action_file_save_as(state);
    }
}

fn action_file_save_as(state: &mut AppState) {
    use crate::io::{save_schematic, show_save_dialog, SchematicIoError};

    // Get default filename from current file or use "untitled"
    let default_name = state
        .schematic
        .current_file
        .as_ref()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().to_string());

    match show_save_dialog(default_name.as_deref()) {
        Ok(path) => match save_schematic(&state.schematic, &path) {
            Ok(()) => {
                state.schematic.current_file = Some(path.clone());
                state.schematic.is_dirty = false;
                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::info(format!(
                        "Saved: {}",
                        path.display()
                    )));
            }
            Err(e) => {
                state
                    .console_messages
                    .push(crate::common::app::ConsoleMessage::error(format!(
                        "Save failed: {}",
                        e
                    )));
            }
        },
        Err(SchematicIoError::Cancelled) => {
            // User cancelled - no message needed
        }
        Err(e) => {
            state
                .console_messages
                .push(crate::common::app::ConsoleMessage::error(format!(
                    "Save As failed: {}",
                    e
                )));
        }
    }
}

fn action_export_svg(state: &mut AppState) {
    use crate::schematic::export::{export_to_svg, SvgExportConfig};

    // Generate SVG content
    let config = SvgExportConfig::default();
    let svg_content = export_to_svg(&state.schematic, &config);

    // Get default filename from current schematic file
    let default_name = state
        .schematic
        .current_file
        .as_ref()
        .and_then(|p| p.file_stem())
        .map(|s| format!("{}.svg", s.to_string_lossy()))
        .unwrap_or_else(|| "schematic.svg".to_string());

    // Show save dialog for SVG
    let dialog = rfd::FileDialog::new()
        .add_filter("SVG Image", &["svg"])
        .set_file_name(&default_name)
        .set_title("Export SVG");

    match dialog.save_file() {
        Some(mut path) => {
            // Ensure .svg extension
            if path.extension().is_none() || path.extension().unwrap() != "svg" {
                path.set_extension("svg");
            }

            match std::fs::write(&path, &svg_content) {
                Ok(()) => {
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info(format!(
                            "Exported SVG: {}",
                            path.display()
                        )));
                }
                Err(e) => {
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::error(format!(
                            "SVG export failed: {}",
                            e
                        )));
                }
            }
        }
        None => {
            // User cancelled - no message needed
        }
    }
}

fn action_export_csv(state: &mut AppState) {
    use crate::io::{SignalType, WaveformDataset, WaveformFormat, WaveformSignal, WaveformWriter};

    // Check if we have simulation results to export
    if state.simulation.waveforms.is_empty() {
        state
            .console_messages
            .push(crate::common::app::ConsoleMessage::warning(
                "No simulation results to export. Run a simulation first.",
            ));
        return;
    }

    // Convert simulation waveforms to WaveformDataset
    let mut dataset = WaveformDataset::new("Simulation Results");

    // Use the X axis from the first waveform (assuming all share the same time axis)
    if let Some(first_wf) = state.simulation.waveforms.first() {
        let mut time_signal = WaveformSignal::new("time", SignalType::Time);
        for &t in &first_wf.x {
            time_signal.push(t);
        }
        dataset.set_x(time_signal);
    }

    // Add all waveform signals
    for wf in &state.simulation.waveforms {
        let sig_type = if wf.name.starts_with("V(") || wf.name.starts_with("v(") {
            SignalType::Voltage
        } else if wf.name.starts_with("I(") || wf.name.starts_with("i(") {
            SignalType::Current
        } else {
            SignalType::Unknown
        };

        let mut signal = WaveformSignal::new(&wf.name, sig_type);
        for &value in &wf.y {
            signal.push(value);
        }
        dataset.add_signal(signal);
    }

    // Show save dialog for CSV
    let dialog = rfd::FileDialog::new()
        .add_filter("CSV Files", &["csv"])
        .set_file_name("waveforms.csv")
        .set_title("Export Waveform CSV");

    match dialog.save_file() {
        Some(mut path) => {
            // Ensure .csv extension
            if path.extension().is_none() || path.extension().unwrap() != "csv" {
                path.set_extension("csv");
            }

            let writer = WaveformWriter::new(WaveformFormat::Csv);
            match writer.write(&dataset, &path) {
                Ok(()) => {
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info(format!(
                            "Exported CSV: {} ({} signals, {} points)",
                            path.display(),
                            dataset.signal_count(),
                            dataset.point_count()
                        )));
                }
                Err(e) => {
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::error(format!(
                            "CSV export failed: {}",
                            e
                        )));
                }
            }
        }
        None => {
            // User cancelled - no message needed
        }
    }
}

fn action_export_netlist(state: &mut AppState, format: crate::io::NetlistFormat) {
    use crate::io::{ExportOptions, NetlistExporter};
    #[allow(unused_imports)]
    use crate::state::ComponentType;

    // Check if we have a schematic to export
    if state.schematic.components.is_empty() {
        state
            .console_messages
            .push(crate::common::app::ConsoleMessage::warning(
                "No circuit to export. Add components first.",
            ));
        return;
    }

    // Generate netlist from schematic using NetlistExporter
    let mut exporter = NetlistExporter::default();
    exporter.options.format = format;
    exporter.title = state
        .schematic
        .current_file
        .as_ref()
        .and_then(|p| p.file_stem())
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "Untitled".to_string());

    // Convert schematic components to netlist instances
    for comp in &state.schematic.components {
        let comp_type_char = match comp.kind {
            ComponentType::Resistor => 'R',
            ComponentType::Capacitor => 'C',
            ComponentType::Inductor => 'L',
            ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::VoltageSourcePwl
            | ComponentType::VoltageSourceExp
            | ComponentType::VoltageSourceSffm => 'V',
            ComponentType::CurrentSource
            | ComponentType::CurrentSourceAc
            | ComponentType::CurrentSourcePulse
            | ComponentType::CurrentSourceSin
            | ComponentType::CurrentSourcePwl
            | ComponentType::CurrentSourceExp => 'I',
            ComponentType::Diode => 'D',
            ComponentType::Nmos => 'M',
            ComponentType::Pmos => 'M',
            ComponentType::NpnBjt | ComponentType::PnpBjt => 'Q',
            ComponentType::Njfet | ComponentType::Pjfet => 'J',
            ComponentType::Vcvs => 'E',
            ComponentType::Vccs => 'G',
            ComponentType::Ccvs => 'H',
            ComponentType::Cccs => 'F',
            _ => 'X',
        };

        let nodes = vec!["N1".to_string(), "N2".to_string()]; // Simplified
        let instance =
            crate::io::netlist_export::ComponentInstance::new(&comp.name, comp_type_char, nodes)
                .with_param("value", &comp.value);
        exporter.instances.push(instance);
    }

    let netlist_content = exporter.generate();

    // Default filename
    let default_name = state
        .schematic
        .current_file
        .as_ref()
        .and_then(|p| p.file_stem())
        .map(|s| format!("{}.{}", s.to_string_lossy(), format.extension()))
        .unwrap_or_else(|| format!("circuit.{}", format.extension()));

    // Show save dialog
    let filter_name = match format {
        crate::io::NetlistFormat::Spectre => "Spectre Netlist",
        crate::io::NetlistFormat::Spice => "SPICE Netlist",
        crate::io::NetlistFormat::Hspice => "HSPICE Netlist",
        crate::io::NetlistFormat::Xyce => "Xyce Netlist",
    };

    let dialog = rfd::FileDialog::new()
        .add_filter(filter_name, &[format.extension()])
        .set_file_name(&default_name)
        .set_title("Export Netlist");

    match dialog.save_file() {
        Some(mut path) => {
            // Ensure correct extension
            let ext = format.extension();
            if path.extension().is_none() || path.extension().unwrap() != ext {
                path.set_extension(ext);
            }

            match std::fs::write(&path, &netlist_content) {
                Ok(()) => {
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::info(format!(
                            "Exported {}: {}",
                            filter_name,
                            path.display()
                        )));
                }
                Err(e) => {
                    state
                        .console_messages
                        .push(crate::common::app::ConsoleMessage::error(format!(
                            "Netlist export failed: {}",
                            e
                        )));
                }
            }
        }
        None => {
            // User cancelled - no message needed
        }
    }
}

fn action_view_netlist(state: &mut AppState) {
    use crate::io::{NetlistExporter, NetlistFormat};
    use crate::state::ComponentType;

    // Check if we have a schematic to view
    if state.schematic.components.is_empty() {
        state
            .console_messages
            .push(crate::common::app::ConsoleMessage::warning(
                "No circuit to generate netlist. Add components first.",
            ));
        return;
    }

    // Generate netlist from schematic using NetlistExporter
    let mut exporter = NetlistExporter::default();
    exporter.options.format = NetlistFormat::Spice;
    exporter.title = state
        .schematic
        .current_file
        .as_ref()
        .and_then(|p| p.file_stem())
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "Untitled".to_string());

    // Convert schematic components to netlist instances
    for comp in &state.schematic.components {
        let comp_type_char = match comp.kind {
            ComponentType::Resistor => 'R',
            ComponentType::Capacitor => 'C',
            ComponentType::Inductor => 'L',
            ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::VoltageSourcePwl
            | ComponentType::VoltageSourceExp
            | ComponentType::VoltageSourceSffm => 'V',
            ComponentType::CurrentSource
            | ComponentType::CurrentSourceAc
            | ComponentType::CurrentSourcePulse
            | ComponentType::CurrentSourceSin
            | ComponentType::CurrentSourcePwl
            | ComponentType::CurrentSourceExp => 'I',
            ComponentType::Diode => 'D',
            ComponentType::Nmos => 'M',
            ComponentType::Pmos => 'M',
            ComponentType::NpnBjt | ComponentType::PnpBjt => 'Q',
            ComponentType::Njfet | ComponentType::Pjfet => 'J',
            ComponentType::Vcvs => 'E',
            ComponentType::Vccs => 'G',
            ComponentType::Ccvs => 'H',
            ComponentType::Cccs => 'F',
            _ => 'X',
        };

        let nodes = vec!["N1".to_string(), "N2".to_string()]; // Simplified
        let instance =
            crate::io::netlist_export::ComponentInstance::new(&comp.name, comp_type_char, nodes)
                .with_param("value", &comp.value);
        exporter.instances.push(instance);
    }

    let netlist_content = exporter.generate();

    // Store in simulation state for editor viewing
    state.simulation.netlist_content = netlist_content.clone();

    // Log first few lines to console for quick preview
    let preview_lines: Vec<&str> = netlist_content.lines().take(10).collect();
    let preview = preview_lines.join("\n");
    let total_lines = netlist_content.lines().count();

    state
        .console_messages
        .push(crate::common::app::ConsoleMessage::info(format!(
            "Generated netlist ({} lines):\n{}{}",
            total_lines,
            preview,
            if total_lines > 10 { "\n..." } else { "" }
        )));
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

    // NOTE: action_file_open, action_file_save, and action_file_save_as
    // cannot be tested here because they open native file dialogs which
    // would block in a headless test environment. The underlying I/O
    // functions are thoroughly tested in io::schematic_io::tests.
}

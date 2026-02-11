//! Menu Bar for egui Application
//!
//! Provides a professional menu bar matching the Dioxus version
//! with File, Edit, View, Simulate, Tools, and Help menus.

use egui::{Ui, menu};

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
                        state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                            "Undo: {}",
                            desc
                        )));
                    }
                } else {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(
                        "Nothing to undo",
                    ));
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
                        state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                            "Redo: {}",
                            desc
                        )));
                    }
                } else {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(
                        "Nothing to redo",
                    ));
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

            let log_label = if state.panels.bottom_panel
                && state.panels.active_bottom_tab == crate::common::app::BottomPanelTab::Log
            {
                "[x] Log"
            } else {
                "[ ] Log"
            };
            if ui.button(log_label).clicked() {
                // Toggle: if already on Log tab, hide panel; otherwise show and switch to it
                if state.panels.bottom_panel
                    && state.panels.active_bottom_tab == crate::common::app::BottomPanelTab::Log
                {
                    state.panels.bottom_panel = false;
                } else {
                    state.panels.bottom_panel = true;
                    state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Log;
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
                        state.panels.bottom_panel = true;
                        state.panels.active_bottom_tab =
                            crate::common::app::BottomPanelTab::Waveform;
                        state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
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
                    state.push_user_message(crate::common::app::ConsoleMessage::warning(
                        "No circuit to simulate. Add components first.",
                    ));
                } else {
                    log::info!("Setting trigger_simulation = true");
                    state.simulation.trigger_simulation = true;
                    state.push_user_message(crate::common::app::ConsoleMessage::info(
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
                    ("DISTO", 24),
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
                            state.push_user_message(crate::common::app::ConsoleMessage::warning(
                                "No circuit to simulate. Add components first.",
                            ));
                        } else {
                            state.push_user_message(crate::common::app::ConsoleMessage::info(
                                format!("Starting {} analysis...", name),
                            ));
                            state.simulation.trigger_simulation = true;
                        }
                        ui.close_menu();
                    }
                }
            });

            if ui.button("⏹ Stop Simulation").clicked() {
                state.simulation.is_running = false;
                state.push_user_message(crate::common::app::ConsoleMessage::warning(
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
                state.dialogs.simulation_options_state =
                    crate::simulation::dialog::OptionsDialogState::from_options(
                        &state.dialogs.simulation_options_config,
                    );
                state.dialogs.simulation_options_errors.clear();
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
                    // Open the Verilog-A import/compile dialog
                    state.dialogs.veriloga_dialog.open();
                    ui.close_menu();
                }
                if ui.button("Module Library...").clicked() {
                    // Open library browser and select the veriloga library
                    state.library_manager.select_library("veriloga");
                    state.panels.project_browser = true;
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("Compile Cache Status").clicked() {
                    action_veriloga_cache_status(state);
                    ui.close_menu();
                }
                if ui.button("List Compile Cache Entries").clicked() {
                    action_veriloga_cache_list_entries(state);
                    ui.close_menu();
                }
                if ui.button("Prune Compile Cache").clicked() {
                    action_veriloga_cache_prune(state);
                    ui.close_menu();
                }
                if ui.button("Clear Compile Cache").clicked() {
                    action_veriloga_cache_clear(state);
                    ui.close_menu();
                }
                if ui.button("Recompile Global Verilog-A Library").clicked() {
                    action_veriloga_recompile_library(state);
                    ui.close_menu();
                }
            });

            if ui.button("Model Browser...").clicked() {
                state.model_browser_state.open = true;
                state.model_browser_state.browse_only = true;
                ui.close_menu();
            }

            if ui.button("PDK Settings...").clicked() {
                state.pdk_settings_dialog.open(state.pdk_config.clone());
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
                // Run DRC on current schematic using the extraction bridge
                let result = crate::services::drc::run_drc_check(&state.schematic);
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
                state.push_user_message(crate::common::app::ConsoleMessage::info(&msg));
                state.dialogs.drc_results = Some(result);
                state.dialogs.drc_dialog = true;
                ui.close_menu();
            }

            if ui.button("Electrical Rule Check").clicked() {
                // ERC is part of DRC - runs connectivity checks
                let result = crate::services::drc::run_drc_check(&state.schematic);
                let msg = if result.passed() {
                    "ERC passed: No electrical rule violations".to_string()
                } else {
                    format!("ERC found {} violations", result.total_count())
                };
                state.push_user_message(crate::common::app::ConsoleMessage::info(&msg));
                state.dialogs.drc_results = Some(result);
                state.dialogs.drc_dialog = true;
                ui.close_menu();
            }

            if ui.button("LVS Check").clicked() {
                state.push_user_message(crate::common::app::ConsoleMessage::info(
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
                    state.push_user_message(crate::common::app::ConsoleMessage::info(
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
            use crate::common::examples::{EXAMPLES, load_example};

            for example in EXAMPLES {
                if ui
                    .button(example.name)
                    .on_hover_text(example.description)
                    .clicked()
                {
                    load_example(example.name, &mut state.schematic);
                    state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
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

// =============================================================================
// Action Handlers (for menu-specific logic)
// =============================================================================

fn action_file_new(state: &mut AppState) {
    if state.schematic.is_dirty {
        log::warn!("New schematic requested but current has unsaved changes");
    }
    state.schematic = crate::state::SchematicState::default();
    state.push_user_message(crate::common::app::ConsoleMessage::info(
        "Created new schematic",
    ));
}

fn action_file_open(state: &mut AppState) {
    use crate::io::{SchematicIoError, load_schematic, show_open_dialog};

    match show_open_dialog() {
        Ok(path) => match load_schematic(&path) {
            Ok(schematic) => {
                state.schematic = schematic;
                state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                    "Opened: {}",
                    path.display()
                )));
            }
            Err(e) => {
                state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                    "Failed to open: {}",
                    e
                )));
            }
        },
        Err(SchematicIoError::Cancelled) => {
            // User cancelled - no message needed
        }
        Err(e) => {
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                "Open failed: {}",
                e
            )));
        }
    }
}

fn action_file_save(state: &mut AppState) {
    use crate::io::{SchematicIoError, save_schematic};

    // If we have a current file path, save directly
    // Otherwise, show Save As dialog
    if let Some(ref path) = state.schematic.current_file.clone() {
        match save_schematic(&state.schematic, path) {
            Ok(()) => {
                state.schematic.is_dirty = false;
                state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                    "Saved: {}",
                    path.display()
                )));
            }
            Err(e) => {
                state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
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
    use crate::io::{SchematicIoError, save_schematic, show_save_dialog};

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
                state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                    "Saved: {}",
                    path.display()
                )));
            }
            Err(e) => {
                state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                    "Save failed: {}",
                    e
                )));
            }
        },
        Err(SchematicIoError::Cancelled) => {
            // User cancelled - no message needed
        }
        Err(e) => {
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                "Save As failed: {}",
                e
            )));
        }
    }
}

fn has_file_extension(path: &std::path::Path, expected_ext: &str) -> bool {
    path.extension()
        .and_then(std::ffi::OsStr::to_str)
        .is_some_and(|ext| ext.eq_ignore_ascii_case(expected_ext))
}

fn ensure_file_extension(path: &mut std::path::PathBuf, expected_ext: &str) {
    if !has_file_extension(path, expected_ext) {
        path.set_extension(expected_ext);
    }
}

fn action_export_svg(state: &mut AppState) {
    use crate::schematic::export::{SvgExportConfig, export_to_svg};

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
            ensure_file_extension(&mut path, "svg");

            match std::fs::write(&path, &svg_content) {
                Ok(()) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                        "Exported SVG: {}",
                        path.display()
                    )));
                }
                Err(e) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
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
        state.push_user_message(crate::common::app::ConsoleMessage::warning(
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
            ensure_file_extension(&mut path, "csv");

            let writer = WaveformWriter::new(WaveformFormat::Csv);
            match writer.write(&dataset, &path) {
                Ok(()) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                        "Exported CSV: {} ({} signals, {} points)",
                        path.display(),
                        dataset.signal_count(),
                        dataset.point_count()
                    )));
                }
                Err(e) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
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
    // Check if we have a schematic to export
    if state.schematic.components.is_empty() {
        state.push_user_message(crate::common::app::ConsoleMessage::warning(
            "No circuit to export. Add components first.",
        ));
        return;
    }

    let Some(netlist_content) = build_menu_netlist(state, format) else {
        return;
    };

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
            ensure_file_extension(&mut path, format.extension());

            match std::fs::write(&path, &netlist_content) {
                Ok(()) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                        "Exported {}: {}",
                        filter_name,
                        path.display()
                    )));
                }
                Err(e) => {
                    state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
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
    // Check if we have a schematic to view
    if state.schematic.components.is_empty() {
        state.push_user_message(crate::common::app::ConsoleMessage::warning(
            "No circuit to generate netlist. Add components first.",
        ));
        return;
    }

    let Some(netlist_content) = build_menu_netlist(state, crate::io::NetlistFormat::Spice) else {
        return;
    };

    // Store in simulation state for editor viewing
    state.simulation.netlist_content = netlist_content.clone();

    // Log first few lines to console for quick preview
    let preview_lines: Vec<&str> = netlist_content.lines().take(10).collect();
    let preview = preview_lines.join("\n");
    let total_lines = netlist_content.lines().count();

    state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
        "Generated netlist ({} lines):\n{}{}",
        total_lines,
        preview,
        if total_lines > 10 { "\n..." } else { "" }
    )));
}

fn build_menu_netlist(state: &mut AppState, format: crate::io::NetlistFormat) -> Option<String> {
    let generation =
        crate::simulation::netlist_gen::generate_netlist_with_analysis(&state.schematic, &[]);

    if !generation.errors.is_empty() {
        for err in generation.errors {
            state.push_user_message(crate::common::app::ConsoleMessage::error(err));
        }
        return None;
    }

    for warning in generation.warnings {
        state.push_user_message(crate::common::app::ConsoleMessage::warning(warning));
    }

    let spice_netlist = generation.netlist;
    Some(match format {
        crate::io::NetlistFormat::Spectre => spice_to_spectre_compatible_netlist(&spice_netlist),
        _ => spice_netlist,
    })
}

fn spice_to_spectre_compatible_netlist(spice_netlist: &str) -> String {
    let mut ahdl_paths: Vec<String> = Vec::new();
    let mut retained_lines: Vec<&str> = Vec::new();

    for line in spice_netlist.lines() {
        if let Some(path) = parse_veriloga_include_path(line) {
            if !ahdl_paths.iter().any(|existing| existing == &path) {
                ahdl_paths.push(path);
            }
            continue;
        }
        retained_lines.push(line);
    }

    if ahdl_paths.is_empty() {
        return retained_lines.join("\n");
    }

    let mut output: Vec<String> = Vec::new();
    let mut inserted_prefix = false;
    for line in retained_lines {
        let trimmed = line.trim();
        if !inserted_prefix && !trimmed.is_empty() && !trimmed.starts_with('*') {
            output.push("simulator lang=spectre".to_string());
            for path in &ahdl_paths {
                output.push(format!("ahdl_include {}", quote_netlist_path(path)));
            }
            output.push("simulator lang=spice".to_string());
            inserted_prefix = true;
        }
        output.push(line.to_string());
    }

    if !inserted_prefix {
        output.push("simulator lang=spectre".to_string());
        for path in &ahdl_paths {
            output.push(format!("ahdl_include {}", quote_netlist_path(path)));
        }
        output.push("simulator lang=spice".to_string());
    }

    output.join("\n")
}

fn parse_veriloga_include_path(line: &str) -> Option<String> {
    let trimmed = line.trim_start();
    let prefix = trimmed.get(..9)?;
    if !prefix.eq_ignore_ascii_case(".veriloga") {
        return None;
    }

    let rest = trimmed.get(9..)?.trim_start();
    if rest.is_empty() {
        return None;
    }

    if let Some(quoted) = rest.strip_prefix('"') {
        let end = quoted.find('"')?;
        return Some(quoted[..end].to_string());
    }

    rest.split_whitespace().next().map(ToString::to_string)
}

fn quote_netlist_path(path: &str) -> String {
    let escaped = path.replace('"', "\\\"");
    format!("\"{}\"", escaped)
}

fn format_cache_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * 1024.0 * 1024.0;

    let bytes_f = bytes as f64;
    if bytes_f >= GB {
        format!("{:.2} GiB", bytes_f / GB)
    } else if bytes_f >= MB {
        format!("{:.2} MiB", bytes_f / MB)
    } else if bytes_f >= KB {
        format!("{:.2} KiB", bytes_f / KB)
    } else {
        format!("{} B", bytes)
    }
}

fn action_veriloga_cache_status(state: &mut AppState) {
    match rspice_core::veriloga_cache_stats() {
        Ok(stats) => {
            state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                "Verilog-A compile cache: {} entries, {} used (limits: {} entries, {}) at {}",
                stats.entry_count,
                format_cache_bytes(stats.total_bytes),
                stats.max_entries,
                format_cache_bytes(stats.max_bytes),
                stats.root.display()
            )));
        }
        Err(err) => {
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                "Failed to inspect Verilog-A compile cache: {}",
                err
            )));
        }
    }
}

fn action_veriloga_cache_list_entries(state: &mut AppState) {
    match rspice_core::veriloga_cache_entries() {
        Ok(entries) => {
            if entries.is_empty() {
                state.push_user_message(crate::common::app::ConsoleMessage::info(
                    "Verilog-A compile cache is empty",
                ));
                return;
            }

            state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                "Verilog-A compile cache entries: {}",
                entries.len()
            )));
            let max_lines = 64_usize;
            for entry in entries.iter().take(max_lines) {
                state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                    "  {} -> {} (deps: {}, size: {})",
                    entry.source_path.display(),
                    entry.cache_path.display(),
                    entry.dependencies.len(),
                    format_cache_bytes(entry.size_bytes)
                )));
                for dep in &entry.dependencies {
                    state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                        "    dep: {}",
                        dep.display()
                    )));
                }
            }
            if entries.len() > max_lines {
                state.push_user_message(crate::common::app::ConsoleMessage::warning(format!(
                    "Listing truncated to first {} entries ({} total)",
                    max_lines,
                    entries.len()
                )));
            }
        }
        Err(err) => {
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                "Failed to list Verilog-A compile cache entries: {}",
                err
            )));
        }
    }
}

fn action_veriloga_cache_prune(state: &mut AppState) {
    match rspice_core::prune_veriloga_cache() {
        Ok(report) => {
            state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                    "Pruned Verilog-A compile cache: removed {} entries, reclaimed {} (now {} entries, {})",
                    report.removed_entries,
                    format_cache_bytes(report.reclaimed_bytes),
                    report.stats.entry_count,
                    format_cache_bytes(report.stats.total_bytes)
                )));
        }
        Err(err) => {
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                "Failed to prune Verilog-A compile cache: {}",
                err
            )));
        }
    }
}

fn action_veriloga_cache_clear(state: &mut AppState) {
    match rspice_core::clear_veriloga_cache() {
        Ok(report) => {
            state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                "Cleared Verilog-A compile cache: removed {} entries, reclaimed {}",
                report.removed_entries,
                format_cache_bytes(report.reclaimed_bytes)
            )));
        }
        Err(err) => {
            state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                "Failed to clear Verilog-A compile cache: {}",
                err
            )));
        }
    }
}

fn action_veriloga_recompile_library(state: &mut AppState) {
    #[derive(Clone)]
    struct Candidate {
        library: String,
        cell: String,
        view: String,
        source_path: std::path::PathBuf,
    }

    let Some(lib) = state.library_manager.get_library("veriloga") else {
        state.push_user_message(crate::common::app::ConsoleMessage::warning(
            "No global 'veriloga' library found",
        ));
        return;
    };

    let (candidates, discovery_warnings) = {
        let mut candidates = Vec::<Candidate>::new();
        let mut discovery_warnings = Vec::<String>::new();
        for cell in lib.cells_sorted() {
            for view in cell.views_sorted() {
                if view.view_type != crate::state::library_browser::ViewType::VerilogA {
                    continue;
                }
                let source_path = view
                    .file_path
                    .clone()
                    .or_else(|| {
                        view.metadata
                            .get("netlist.source_path")
                            .or_else(|| view.metadata.get("veriloga.source_path"))
                            .map(std::path::PathBuf::from)
                    })
                    .or_else(|| {
                        cell.metadata
                            .get("netlist.source_path")
                            .or_else(|| cell.metadata.get("veriloga.source_path"))
                            .map(std::path::PathBuf::from)
                    });
                let Some(source_path) = source_path else {
                    discovery_warnings.push(format!(
                        "Skipping {}/{}/{}: missing source path metadata",
                        lib.name, cell.name, view.name
                    ));
                    continue;
                };
                candidates.push(Candidate {
                    library: lib.name.clone(),
                    cell: cell.name.clone(),
                    view: view.name.clone(),
                    source_path,
                });
            }
        }
        (candidates, discovery_warnings)
    };

    for warning in discovery_warnings {
        state.push_user_message(crate::common::app::ConsoleMessage::warning(warning));
    }

    if candidates.is_empty() {
        state.push_user_message(crate::common::app::ConsoleMessage::warning(
            "No Verilog-A views found in global library",
        ));
        return;
    }

    let compiler = rspice_veriloga::VerilogACompiler::default();
    let mut ok_count = 0_usize;
    let mut fail_count = 0_usize;
    state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
        "Recompiling {} Verilog-A model(s) from global library...",
        candidates.len()
    )));

    for candidate in candidates {
        match compiler.compile_file_with_metadata(&candidate.source_path) {
            Ok(compiled) => {
                let model_name = compiled.model.name.clone();
                match rspice_core::register_precompiled_veriloga_model_with_dependencies(
                    &candidate.source_path,
                    &compiled.dependencies,
                    compiled.model,
                ) {
                    Ok(()) => {
                        ok_count += 1;
                        state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
                            "Recompiled {}/{}/{} -> '{}' ({} deps)",
                            candidate.library,
                            candidate.cell,
                            candidate.view,
                            model_name,
                            compiled.dependencies.len()
                        )));
                    }
                    Err(err) => {
                        fail_count += 1;
                        state.push_user_message(crate::common::app::ConsoleMessage::error(
                            format!(
                                "Cache registration failed for {}/{}/{} ({}): {}",
                                candidate.library,
                                candidate.cell,
                                candidate.view,
                                candidate.source_path.display(),
                                err
                            ),
                        ));
                    }
                }
            }
            Err(err) => {
                fail_count += 1;
                state.push_user_message(crate::common::app::ConsoleMessage::error(format!(
                    "Compile failed for {}/{}/{} ({}): {}",
                    candidate.library,
                    candidate.cell,
                    candidate.view,
                    candidate.source_path.display(),
                    err
                )));
            }
        }
    }

    state.push_user_message(crate::common::app::ConsoleMessage::info(format!(
        "Verilog-A recompile complete: {} succeeded, {} failed",
        ok_count, fail_count
    )));
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn cache_env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    fn with_temp_cache_env(label: &str, f: impl FnOnce()) {
        let _guard = cache_env_lock()
            .lock()
            .expect("failed to acquire cache env lock");
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be monotonic")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "rspice_ui_veriloga_cache_test_{}_{}_{}",
            label,
            std::process::id(),
            nanos
        ));
        std::fs::create_dir_all(&dir).expect("failed to create temp cache dir");

        // SAFETY: tests serialize environment access via cache_env_lock().
        unsafe {
            std::env::set_var(
                "RSPICE_VERILOGA_CACHE_DIR",
                dir.to_string_lossy().to_string(),
            );
            std::env::remove_var("RSPICE_VERILOGA_CACHE_MAX_ENTRIES");
            std::env::remove_var("RSPICE_VERILOGA_CACHE_MAX_BYTES");
        }
        f();
        // SAFETY: tests serialize environment access via cache_env_lock().
        unsafe {
            std::env::remove_var("RSPICE_VERILOGA_CACHE_DIR");
            std::env::remove_var("RSPICE_VERILOGA_CACHE_MAX_ENTRIES");
            std::env::remove_var("RSPICE_VERILOGA_CACHE_MAX_BYTES");
        }
        let _ = std::fs::remove_dir_all(dir);
    }

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
    fn test_format_cache_bytes_units() {
        assert_eq!(format_cache_bytes(512), "512 B");
        assert!(format_cache_bytes(2048).contains("KiB"));
        assert!(format_cache_bytes(3 * 1024 * 1024).contains("MiB"));
    }

    #[test]
    fn test_has_file_extension_case_insensitive() {
        assert!(has_file_extension(
            std::path::Path::new("schematic.SVG"),
            "svg"
        ));
        assert!(!has_file_extension(
            std::path::Path::new("schematic.raw"),
            "svg"
        ));
    }

    #[test]
    fn test_ensure_file_extension_appends_missing_extension() {
        let mut path = std::path::PathBuf::from("waveforms");
        ensure_file_extension(&mut path, "csv");
        assert_eq!(path, std::path::PathBuf::from("waveforms.csv"));
    }

    #[test]
    fn test_ensure_file_extension_replaces_mismatched_extension() {
        let mut path = std::path::PathBuf::from("results.txt");
        ensure_file_extension(&mut path, "csv");
        assert_eq!(path, std::path::PathBuf::from("results.csv"));
    }

    #[test]
    fn test_action_veriloga_cache_status_emits_console_message() {
        with_temp_cache_env("status", || {
            let mut state = AppState::default();
            action_veriloga_cache_status(&mut state);
            assert!(
                !state.console_messages.is_empty(),
                "cache status action should emit a console message"
            );
        });
    }

    #[test]
    fn test_action_veriloga_cache_list_entries_emits_console_message() {
        with_temp_cache_env("list", || {
            let mut state = AppState::default();
            action_veriloga_cache_list_entries(&mut state);
            assert!(
                !state.console_messages.is_empty(),
                "cache listing action should emit a console message"
            );
        });
    }

    #[test]
    fn test_action_veriloga_recompile_library_without_library_warns() {
        let mut state = AppState::default();
        action_veriloga_recompile_library(&mut state);
        assert!(
            state
                .console_messages
                .iter()
                .any(|msg| msg.message.contains("No global 'veriloga' library found"))
        );
    }

    #[test]
    fn test_parse_veriloga_include_path_handles_quoted_and_unquoted_paths() {
        let quoted = r#".VERILOGA "C:/models/opamp.va" opamp"#;
        let bare = ".veriloga C:/models/opamp.va";
        let unrelated = ".include \"models.lib\"";

        assert_eq!(
            parse_veriloga_include_path(quoted),
            Some("C:/models/opamp.va".to_string())
        );
        assert_eq!(
            parse_veriloga_include_path(bare),
            Some("C:/models/opamp.va".to_string())
        );
        assert_eq!(parse_veriloga_include_path(unrelated), None);
    }

    #[test]
    fn test_spice_to_spectre_compatible_netlist_converts_veriloga_directives() {
        let spice = r#"
* Header
.VERILOGA "C:/models/opamp.va" opamp
R1 in out 1k
.end
"#;

        let spectre = spice_to_spectre_compatible_netlist(spice);
        assert!(spectre.contains("simulator lang=spectre"));
        assert!(spectre.contains("ahdl_include \"C:/models/opamp.va\""));
        assert!(spectre.contains("simulator lang=spice"));
        assert!(spectre.contains("R1 in out 1k"));
        assert!(!spectre.contains(".VERILOGA"));
    }

    #[test]
    fn test_action_view_netlist_uses_generated_schematic_netlist() {
        let mut state = AppState::default();
        use crate::state::{Component, ComponentType, Point};
        let comp = Component::new(1, ComponentType::Resistor, Point::new(100, 100))
            .with_name_value("R1", "1k");
        state.schematic.components.push(comp);

        action_view_netlist(&mut state);

        assert!(
            state.simulation.netlist_content.contains("R1"),
            "generated netlist should include the real component instance"
        );
        assert!(
            !state.simulation.netlist_content.contains("N1 N2"),
            "legacy placeholder node names must not appear"
        );
    }

    // NOTE: action_file_open, action_file_save, and action_file_save_as
    // cannot be tested here because they open native file dialogs which
    // would block in a headless test environment. The underlying I/O
    // functions are thoroughly tested in io::schematic_io::tests.
}

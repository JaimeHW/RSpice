//! Toolbar Component
//!
//! Main application toolbar with file operations and simulation controls.

use dioxus::prelude::*;

use super::button::{Button, ButtonVariant};
use super::file_handlers;
use super::icons::{Icon, IconType};
use crate::state::simulation_command::SimulationConfig;
use crate::state::{
    generate_netlist, run_simulation, ConsoleMessage, SchematicState, SimulationState, WaveformData,
};
use crate::theme::Theme;

/// Default content for new files
const DEFAULT_NEW_FILE: &str = r#"* RSpice Circuit
* New Circuit

V1 1 0 DC 5
R1 1 0 1k

.OP
.END
"#;

/// Main application toolbar
#[component]
pub fn Toolbar() -> Element {
    let theme: Signal<Theme> = use_context();
    let mut sim_state: Signal<SimulationState> = use_context();
    let mut schematic: Signal<SchematicState> = use_context();
    let mut waveform_visible: Signal<crate::app::WaveformVisible> = use_context();
    let mut console_visible: Signal<crate::app::ConsoleVisible> = use_context();
    let _sim_config: Signal<SimulationConfig> = use_context();
    let mut sim_dialog_visible: Signal<bool> = use_context();
    let th = theme.read();

    let is_running = sim_state.read().is_running;

    rsx! {
        div {
            class: "toolbar",
            style: "
                display: flex;
                align-items: center;
                height: 48px;
                padding: 0 {Theme::SPACING_MD};
                background: {th.bg_secondary()};
                border-bottom: 1px solid {th.border()};
                gap: {Theme::SPACING_MD};
            ",

            // Logo / App name
            div {
                style: "
                    display: flex;
                    align-items: center;
                    gap: {Theme::SPACING_SM};
                    padding-right: {Theme::SPACING_MD};
                    border-right: 1px solid {th.border()};
                ",
                // Inline SVG logo
                svg {
                    width: "28",
                    height: "28",
                    view_box: "0 0 512 512",
                    fill: "none",

                    // Hexagon Background
                    path {
                        d: "M256 32 L464 140 V372 L256 480 L48 372 V140 Z",
                        fill: "#2a2a3a"
                    }

                    // Inner Border Highlight
                    path {
                        d: "M256 32 L464 140 V372 L256 480 L48 372 V140 Z",
                        stroke: "#FF5F15",
                        stroke_opacity: "0.3",
                        stroke_width: "12"
                    }

                    // Signal Pulse
                    path {
                        d: "M100 256 L160 256 L200 150 L256 360 L312 150 L352 256 L412 256",
                        stroke: "#FF5F15",
                        stroke_width: "28",
                        stroke_linecap: "round",
                        stroke_linejoin: "round"
                    }
                }

                span {
                    style: "
                        font-size: {Theme::FONT_SIZE_LG};
                        font-weight: 700;
                        color: {th.text_primary()};
                    ",
                    "RSpice"
                }
            }

            // File operations group
            ToolbarGroup {
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::File, size: 16 } },
                    onclick: move |_| {
                        // New file - clear editor
                        let mut state = sim_state.write();
                        state.netlist_content = DEFAULT_NEW_FILE.to_string();
                        state.current_file = None;
                        state.is_dirty = false;
                        state.waveforms.clear();
                        state.console_messages.clear();
                        state.console_messages.push(ConsoleMessage::info("New file created"));
                    },
                    "New"
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::FolderOpen, size: 16 } },
                    onclick: move |_| {
                        // Open file dialog
                        spawn(file_handlers::open_netlist(sim_state));
                    },
                    "Open"
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::Save, size: 16 } },
                    onclick: move |_| {
                        spawn(file_handlers::save_netlist(sim_state));
                    },
                    "Save"
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::FolderOpen, size: 16 } },
                    onclick: move |_| {
                        // Import LTspice .raw waveform file
                        spawn(file_handlers::import_raw(sim_state));
                    },
                    "Import"
                }
            }

            // Divider
            ToolbarDivider {}

            // Schematic file operations
            ToolbarGroup {
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::Save, size: 16 } },
                    onclick: move |_| {
                        spawn(file_handlers::save_schematic(schematic, sim_state));
                    },
                    "Save Sch"
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::FolderOpen, size: 16 } },
                    onclick: move |_| {
                        spawn(file_handlers::open_schematic(schematic, sim_state));
                    },
                    "Open Sch"
                }
            }

            // Divider
            ToolbarDivider {}

            // Edit operations
            ToolbarGroup {
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::Undo, size: 16 } },
                    ""
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::Redo, size: 16 } },
                    ""
                }
            }

            // Spacer
            div { style: "flex: 1;" }

            // Simulation setup button
            ToolbarGroup {
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::Settings, size: 16 } },
                    onclick: move |_| {
                        sim_dialog_visible.set(true);
                    },
                    "Simulate..."
                }
            }

            // Divider
            ToolbarDivider {}

            // Simulation controls
            ToolbarGroup {
                if is_running {
                    Button {
                        variant: ButtonVariant::Danger,
                        icon: rsx! { Icon { icon: IconType::Stop, size: 16, color: "#ffffff".to_string() } },
                        onclick: move |_| {
                            sim_state.write().is_running = false;
                            log::info!("Simulation stopped");
                        },
                        "Stop"
                    }
                } else {
                    Button {
                        variant: ButtonVariant::Success,
                        icon: rsx! { Icon { icon: IconType::Play, size: 16, color: "#ffffff".to_string() } },
                        onclick: move |_| {
                            // Generate netlist from schematic
                            let schematic_state = schematic.read();
                            let netlist_result = generate_netlist(&schematic_state);
                            drop(schematic_state);

                            // Store the point-to-net mapping for probe tool
                            schematic.write().net_mapping = netlist_result.point_to_net.clone();

                            // Check for errors in netlist generation
                            if !netlist_result.errors.is_empty() {
                                let mut state = sim_state.write();
                                for err in &netlist_result.errors {
                                    state.console_messages.push(ConsoleMessage::error(err.clone()));
                                }
                                return;
                            }

                            // Log warnings
                            for warn in &netlist_result.warnings {
                                sim_state.write().console_messages.push(ConsoleMessage::warning(warn.clone()));
                            }

                            // Get simulation configuration and generate SPICE commands
                            let config = _sim_config.read();
                            let sim_commands = config.to_spice_string();
                            let has_config = config.has_analysis();
                            drop(config);

                            // Build complete netlist with simulation commands
                            let mut netlist_content = netlist_result.netlist.clone();

                            // Professional behavior: if dialog has configuration, it REPLACES
                            // any existing analysis commands in the netlist (like LTspice)
                            if has_config {
                                // Strip existing analysis commands from netlist
                                netlist_content = strip_analysis_commands(&netlist_content);

                                // Insert configured commands before .END
                                if let Some(end_pos) = netlist_content.to_uppercase().rfind(".END") {
                                    netlist_content.insert_str(end_pos, &format!("{}\n\n", sim_commands));
                                } else {
                                    // No .END found, append commands at the end
                                    netlist_content.push_str(&format!("\n{}\n.END\n", sim_commands));
                                }
                            }
                            // If no dialog config, use whatever's already in the netlist

                            sim_state.write().netlist_content = netlist_content.clone();
                            sim_state.write().is_running = true;

                            // Log the generated netlist for debugging
                            log::info!("Generated netlist:\n{}", netlist_content);
                            for (net_name, _points) in &netlist_result.nets {
                                log::info!("  Net: {}", net_name);
                            }

                            sim_state.write().console_messages.push(ConsoleMessage::info(
                                format!("Generated netlist ({} nets)", netlist_result.nets.len())
                            ));

                            let result = run_simulation(&netlist_content);

                            // Update state with results
                            let mut state = sim_state.write();
                            state.is_running = false;

                            if result.success {
                                // Clear old waveforms and add new ones
                                state.waveforms.clear();

                                // Collect waveform names for ground detection
                                let mut waveform_nets: std::collections::HashSet<String> = std::collections::HashSet::new();

                                if let Some(tran) = result.transient {
                                    log::info!("Transient data: {} time points, {} voltage traces",
                                        tran.time.len(), tran.voltages.len());
                                    for (idx, (name, values)) in tran.voltages.into_iter().enumerate() {
                                        log::info!("  Adding waveform: {} with {} points", name, values.len());
                                        // Extract net name from V(N001) -> N001
                                        let net_name = name
                                            .trim_start_matches("V(")
                                            .trim_end_matches(')')
                                            .to_string();
                                        waveform_nets.insert(net_name);
                                        state.waveforms.push(WaveformData {
                                            name,
                                            x: tran.time.clone(),
                                            y: values,
                                            color: Theme::trace_color_static(idx).to_string(),
                                            visible: true,
                                        });
                                    }
                                } else {
                                    log::warn!("No transient data in simulation result");
                                }

                                // Derive ground node: the net that exists in netlist but not in waveforms
                                let all_nets: std::collections::HashSet<String> = netlist_result.nets.keys().cloned().collect();
                                let ground_candidates: Vec<String> = all_nets.difference(&waveform_nets).cloned().collect();
                                if let Some(ground) = ground_candidates.first() {
                                    state.ground_node = Some(ground.clone());
                                    log::info!("Ground reference node: {}", ground);
                                } else {
                                    state.ground_node = None;
                                }

                                log::info!("Total waveforms after simulation: {}", state.waveforms.len());

                                state.console_messages.push(ConsoleMessage::success(
                                    format!("Simulation complete! {} points, {:.1}ms",
                                        result.stats.num_points,
                                        result.stats.sim_time_ms
                                    )
                                ));
                                log::info!("Simulation completed: {} points in {:.1}ms",
                                    result.stats.num_points, result.stats.sim_time_ms);
                            } else {
                                state.console_messages.push(ConsoleMessage::error(
                                    result.error.unwrap_or_else(|| "Unknown error".to_string())
                                ));
                                log::error!("Simulation failed");
                            }
                        },
                        "Run"
                    }
                }
            }

            // Divider
            ToolbarDivider {}

            // View controls
            ToolbarGroup {
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::ZoomIn, size: 16 } },
                    ""
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::ZoomOut, size: 16 } },
                    ""
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::FitToScreen, size: 16 } },
                    ""
                }
            }

            // View toggles
            ToolbarGroup {
                // Console toggle
                Button {
                    variant: if console_visible.read().0 { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                    onclick: move |_| {
                        let current = console_visible.read().0;
                        console_visible.set(crate::app::ConsoleVisible(!current));
                    },
                    "Console"
                }
                // Waveform toggle (only enabled when waveforms exist)
                Button {
                    variant: if waveform_visible.read().0 && !sim_state.read().waveforms.is_empty() {
                        ButtonVariant::Primary
                    } else {
                        ButtonVariant::Ghost
                    },
                    disabled: sim_state.read().waveforms.is_empty(),
                    onclick: move |_| {
                        if !sim_state.read().waveforms.is_empty() {
                            let current = waveform_visible.read().0;
                            waveform_visible.set(crate::app::WaveformVisible(!current));
                        }
                    },
                    "Waveform"
                }
            }

            // Settings
            Button {
                variant: ButtonVariant::Ghost,
                icon: rsx! { Icon { icon: IconType::Settings, size: 18 } },
                ""
            }
        }
    }
}

/// Toolbar button group
#[component]
fn ToolbarGroup(children: Element) -> Element {
    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                gap: 2px;
            ",
            {children}
        }
    }
}

/// Vertical divider between toolbar sections
#[component]
fn ToolbarDivider() -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        div {
            style: "
                width: 1px;
                height: 24px;
                background: {th.border()};
                margin: 0 {Theme::SPACING_XS};
            "
        }
    }
}

/// Strip existing analysis commands from a netlist.
/// This allows dialog-configured commands to replace schematic-embedded ones.
fn strip_analysis_commands(netlist: &str) -> String {
    let analysis_prefixes = [".TRAN", ".AC", ".DC", ".OP"];

    netlist
        .lines()
        .filter(|line| {
            let trimmed = line.trim().to_uppercase();
            // Keep lines that don't start with analysis commands
            !analysis_prefixes
                .iter()
                .any(|prefix| trimmed.starts_with(prefix))
        })
        .collect::<Vec<_>>()
        .join("\n")
}

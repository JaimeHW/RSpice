//! Toolbar Component
//!
//! Main application toolbar with file operations and simulation controls.

use dioxus::prelude::*;

use super::button::{Button, ButtonVariant};
use super::confirm_modal::{SaveDialogResult, UnsavedChangesModal};
use super::file_handlers;
use super::icons::{Icon, IconType};
use crate::state::simulation_command::SimulationConfig;
use crate::state::{
    generate_netlist, run_simulation, ConsoleMessage, SchematicState, SimulationResult,
    SimulationState, WaveformData,
};
use crate::theme::Theme;

/// Pending action that requires confirmation
#[derive(Debug, Clone, Copy, PartialEq)]
enum PendingAction {
    /// Create a new schematic
    New,
    /// Open a schematic file
    Open,
}

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

    // State for unsaved changes modal
    let mut show_save_modal = use_signal(|| false);
    let mut pending_action: Signal<Option<PendingAction>> = use_signal(|| None);

    // Get current file name for modal display
    let current_filename = schematic
        .read()
        .current_file
        .as_ref()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().to_string());

    // Helper: perform the New action
    let mut do_new = move || {
        let mut state = schematic.write();
        state.components.clear();
        state.wires.clear();
        state.selection.clear();
        state.current_file = None;
        state.is_dirty = false;
        drop(state);

        sim_state.write().waveforms.clear();
        sim_state.write().console_messages.clear();
        sim_state
            .write()
            .console_messages
            .push(ConsoleMessage::info("New schematic created"));
    };

    // Helper: perform the Open action
    let do_open = move || {
        spawn(file_handlers::open_schematic(schematic, sim_state));
    };

    // Handle modal result
    let handle_modal_result = move |result: SaveDialogResult| {
        show_save_modal.set(false);
        let action = pending_action.read().clone();
        pending_action.set(None);

        match result {
            SaveDialogResult::Save => {
                // Save first, then perform action
                spawn(async move {
                    file_handlers::save_schematic(schematic, sim_state).await;
                    // After save, perform the pending action
                    if let Some(act) = action {
                        match act {
                            PendingAction::New => do_new(),
                            PendingAction::Open => do_open(),
                        }
                    }
                });
            }
            SaveDialogResult::DontSave => {
                // Perform action without saving
                if let Some(act) = action {
                    match act {
                        PendingAction::New => do_new(),
                        PendingAction::Open => do_open(),
                    }
                }
            }
            SaveDialogResult::Cancel => {
                // Do nothing - stay on current schematic
            }
        }
    };

    rsx! {
        // Unsaved changes modal
        UnsavedChangesModal {
            visible: *show_save_modal.read(),
            filename: current_filename.clone(),
            on_result: handle_modal_result,
        }

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

            // File operations (Schematic-focused)
            ToolbarGroup {
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::File, size: 16 } },
                    title: "New Schematic (Ctrl+N)",
                    onclick: move |_| {
                        // Check for unsaved changes
                        if schematic.read().is_dirty {
                            pending_action.set(Some(PendingAction::New));
                            show_save_modal.set(true);
                        } else {
                            do_new();
                        }
                    },
                    "New"
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::FolderOpen, size: 16 } },
                    title: "Open Schematic (Ctrl+O)",
                    onclick: move |_| {
                        // Check for unsaved changes
                        if schematic.read().is_dirty {
                            pending_action.set(Some(PendingAction::Open));
                            show_save_modal.set(true);
                        } else {
                            do_open();
                        }
                    },
                    "Open"
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::Save, size: 16 } },
                    title: "Save Schematic (Ctrl+S)",
                    onclick: move |_| {
                        spawn(file_handlers::save_schematic(schematic, sim_state));
                    },
                    "Save"
                }
            }

            // Divider
            ToolbarDivider {}

            // Edit operations (Undo/Redo) - placeholder, actual undo/redo uses keyboard shortcuts
            ToolbarGroup {
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::Undo, size: 16 } },
                    title: "Undo (Ctrl+Z)",
                    ""
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::Redo, size: 16 } },
                    title: "Redo (Ctrl+Y)",
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

                            // Clone data needed for async block
                            let netlist_for_sim = netlist_content.clone();
                            let nets_for_ground = netlist_result.nets.clone();

                            // Run simulation in background thread to prevent UI freeze
                            // spawn_blocking is essential for CPU-bound work in async context
                            spawn(async move {
                                // Use spawn_blocking to run CPU-intensive simulation on thread pool
                                let result = tokio::task::spawn_blocking(move || {
                                    run_simulation(&netlist_for_sim)
                                }).await.unwrap_or_else(|_| SimulationResult {
                                    success: false,
                                    transient: None,
                                    dc_op: None,
                                    error: Some("Simulation task panicked".to_string()),
                                    stats: Default::default(),
                                });

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
                                    let all_nets: std::collections::HashSet<String> = nets_for_ground.keys().cloned().collect();
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
                            });
                        },
                        "Run"
                    }
                }
            }

            // Divider
            ToolbarDivider {}

            // View toggles
            ToolbarGroup {
                // Console toggle
                Button {
                    variant: if console_visible.read().0 { ButtonVariant::Primary } else { ButtonVariant::Ghost },
                    title: "Toggle Console",
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
                    title: "Toggle Waveform Viewer",
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

//! Toolbar Component
//!
//! Main application toolbar with file operations and simulation controls.

use dioxus::prelude::*;

use super::button::{Button, ButtonVariant};
use super::icons::{Icon, IconType};
use crate::state::{run_simulation, ConsoleMessage, SimulationState, WaveformData};
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
                    font-size: {Theme::FONT_SIZE_LG};
                    font-weight: 700;
                    color: {th.accent_primary()};
                    padding-right: {Theme::SPACING_MD};
                    border-right: 1px solid {th.border()};
                ",
                "⚡ RSpice"
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
                        spawn(async move {
                            if let Some(path) = rfd::AsyncFileDialog::new()
                                .add_filter("SPICE Netlist", &["cir", "sp", "spice", "net"])
                                .add_filter("All Files", &["*"])
                                .pick_file()
                                .await
                            {
                                match std::fs::read_to_string(path.path()) {
                                    Ok(content) => {
                                        let mut state = sim_state.write();
                                        state.netlist_content = content;
                                        state.current_file = Some(path.path().to_path_buf());
                                        state.is_dirty = false;
                                        state.waveforms.clear();
                                        state.console_messages.push(ConsoleMessage::success(
                                            format!("Opened: {}", path.path().display())
                                        ));
                                    }
                                    Err(e) => {
                                        sim_state.write().console_messages.push(
                                            ConsoleMessage::error(format!("Failed to open file: {}", e))
                                        );
                                    }
                                }
                            }
                        });
                    },
                    "Open"
                }
                Button {
                    variant: ButtonVariant::Ghost,
                    icon: rsx! { Icon { icon: IconType::Save, size: 16 } },
                    onclick: move |_| {
                        let content = sim_state.read().netlist_content.clone();
                        let current_path = sim_state.read().current_file.clone();

                        spawn(async move {
                            let save_path = if let Some(path) = current_path {
                                Some(path)
                            } else {
                                // Save as dialog
                                rfd::AsyncFileDialog::new()
                                    .add_filter("SPICE Netlist", &["cir"])
                                    .set_file_name("circuit.cir")
                                    .save_file()
                                    .await
                                    .map(|f| f.path().to_path_buf())
                            };

                            if let Some(path) = save_path {
                                match std::fs::write(&path, &content) {
                                    Ok(_) => {
                                        let mut state = sim_state.write();
                                        state.current_file = Some(path.clone());
                                        state.is_dirty = false;
                                        state.console_messages.push(ConsoleMessage::success(
                                            format!("Saved: {}", path.display())
                                        ));
                                    }
                                    Err(e) => {
                                        sim_state.write().console_messages.push(
                                            ConsoleMessage::error(format!("Failed to save: {}", e))
                                        );
                                    }
                                }
                            }
                        });
                    },
                    "Save"
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
                            // Get netlist content from state
                            let netlist_content = sim_state.read().netlist_content.clone();

                            // Run simulation
                            sim_state.write().is_running = true;
                            sim_state.write().console_messages.clear();
                            sim_state.write().console_messages.push(ConsoleMessage::info("Starting simulation..."));

                            let result = run_simulation(&netlist_content);

                            // Update state with results
                            let mut state = sim_state.write();
                            state.is_running = false;

                            if result.success {
                                // Clear old waveforms and add new ones
                                state.waveforms.clear();

                                if let Some(tran) = result.transient {
                                    for (idx, (name, values)) in tran.voltages.into_iter().enumerate() {
                                        state.waveforms.push(WaveformData {
                                            name,
                                            x: tran.time.clone(),
                                            y: values,
                                            color: Theme::trace_color_static(idx).to_string(),
                                            visible: true,
                                        });
                                    }
                                }

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

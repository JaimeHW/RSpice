//! Schematic Toolbar Component
//!
//! Tool selection buttons and view options for the schematic editor.

use dioxus::prelude::*;

use crate::state::{SchematicState, Tool};
use crate::theme::Theme;

/// Schematic toolbar with tool buttons and view options
#[component]
pub fn SchematicToolbar(schematic: Signal<SchematicState>) -> Element {
    let theme: Signal<Theme> = use_context();
    let mut display_settings: Signal<crate::state::display_settings::SchematicDisplaySettings> =
        use_context();
    let th = theme.read();
    let tool = schematic.read().tool;
    let settings = display_settings.read();

    // Current pin visibility state for toggle button
    let pins_visible = matches!(
        settings.show_pin_names,
        crate::state::display_settings::PinNameVisibility::Always
    );
    let pin_btn_label = if pins_visible {
        "📍 Pins ✓"
    } else {
        "📍 Pins"
    };

    let pin_bg = if pins_visible {
        th.accent_primary()
    } else {
        th.surface()
    };
    let pin_color = if pins_visible {
        "#fff"
    } else {
        th.text_primary()
    };

    rsx! {
        div {
            style: "display: flex; align-items: center; height: 32px; padding: 0 8px; background: {th.bg_tertiary()}; border-bottom: 1px solid {th.border()}; gap: 4px; flex-wrap: nowrap; overflow: hidden; white-space: nowrap;",
            // Tool buttons
            ToolBtn { label: "↖ Select", active: matches!(tool, Tool::Select), onclick: move |_| schematic.write().tool = Tool::Select }
            ToolBtn { label: "— Wire", active: matches!(tool, Tool::Wire), onclick: move |_| schematic.write().tool = Tool::Wire }
            ToolBtn { label: "⚡ Probe", active: matches!(tool, Tool::Probe), onclick: move |_| {
                let mut s = schematic.write();
                s.selection.clear();
                s.tool = Tool::Probe;
            }}
            ToolBtn { label: "🏷 Label", active: matches!(tool, Tool::Label), onclick: move |_| {
                let mut s = schematic.write();
                s.selection.clear();
                s.tool = Tool::Label;
            }}
            div { style: "width: 1px; height: 18px; background: {th.border()}; margin: 0 4px; flex-shrink: 0;" }
            button { style: "padding: 4px 8px; background: {th.surface()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 12px; cursor: pointer; flex-shrink: 0; white-space: nowrap;", onclick: move |_| schematic.write().rotate_selection(), "⟳ Rotate" }
            button { style: "padding: 4px 8px; background: {th.surface()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 12px; cursor: pointer; flex-shrink: 0; white-space: nowrap;", onclick: move |_| schematic.write().delete_selection(), "🗑 Delete" }

            // View section divider
            div { style: "width: 1px; height: 18px; background: {th.border()}; margin: 0 8px; flex-shrink: 0;" }
            span { style: "font-size: 11px; color: {th.text_muted()}; margin-right: 4px; flex-shrink: 0; white-space: nowrap;", "View:" }

            // Zoom controls - zoom centered around content
            button {
                style: "padding: 4px 6px; background: {th.surface()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 12px; cursor: pointer; flex-shrink: 0;",
                title: "Zoom In (Ctrl++)",
                onclick: move |_| {
                    let mut s = schematic.write();
                    let factor = 1.25;
                    let new_zoom = (s.zoom * factor).min(4.0);
                    // Adjust pan to keep center fixed: pan' = pan - (center * (new_zoom - old_zoom))
                    // But since we don't have viewport center here, we center on content bounds
                    if let Some((min_x, min_y, max_x, max_y)) = s.content_bounds() {
                        let gs = s.grid_size as f64;
                        let center_x = ((min_x + max_x) as f64 / 2.0) * gs;
                        let center_y = ((min_y + max_y) as f64 / 2.0) * gs;
                        s.pan.0 -= center_x * (new_zoom - s.zoom);
                        s.pan.1 -= center_y * (new_zoom - s.zoom);
                    }
                    s.zoom = new_zoom;
                },
                "🔍+"
            }
            button {
                style: "padding: 4px 6px; background: {th.surface()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 12px; cursor: pointer; flex-shrink: 0;",
                title: "Zoom Out (Ctrl+-)",
                onclick: move |_| {
                    let mut s = schematic.write();
                    let factor = 1.25;
                    let new_zoom = (s.zoom / factor).max(0.25);
                    // Adjust pan to keep center fixed
                    if let Some((min_x, min_y, max_x, max_y)) = s.content_bounds() {
                        let gs = s.grid_size as f64;
                        let center_x = ((min_x + max_x) as f64 / 2.0) * gs;
                        let center_y = ((min_y + max_y) as f64 / 2.0) * gs;
                        s.pan.0 -= center_x * (new_zoom - s.zoom);
                        s.pan.1 -= center_y * (new_zoom - s.zoom);
                    }
                    s.zoom = new_zoom;
                },
                "🔍−"
            }
            button {
                style: "padding: 4px 6px; background: {th.surface()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 12px; cursor: pointer; flex-shrink: 0;",
                title: "Fit to Window (Ctrl+0)",
                onclick: move |_| {
                    // Set needs_fit flag - the schematic component will detect this
                    // and call zoom_to_fit with actual canvas dimensions
                    schematic.write().needs_fit = true;
                },
                "⊞"
            }
            button {
                style: "padding: 4px 6px; background: {th.surface()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 12px; cursor: pointer; flex-shrink: 0;",
                title: "Actual Size (100%)",
                onclick: move |_| {
                    schematic.write().zoom = 1.0;
                },
                "1:1"
            }
            span { style: "font-size: 11px; color: {th.text_muted()}; margin: 0 4px; flex-shrink: 0; white-space: nowrap;", {format!("{}%", (schematic.read().zoom * 100.0).round() as i32)} }

            // Grid style toggle - cycles: Lines → Dots → Hidden → Lines
            {
                use crate::state::display_settings::GridStyle;
                let grid_style = settings.grid_style;
                let grid_visible = grid_style.is_visible();
                let grid_bg = if grid_visible { th.accent_primary() } else { th.surface() };
                let grid_color = if grid_visible { "#fff".to_string() } else { th.text_primary().to_string() };
                let button_text = grid_style.button_text();
                rsx! {
                    button {
                        style: "padding: 4px 8px; background: {grid_bg}; border: 1px solid {th.border()}; border-radius: 4px; color: {grid_color}; font-size: 12px; cursor: pointer; flex-shrink: 0; white-space: nowrap;",
                        title: "Cycle Grid Style (G)",
                        onclick: move |_| {
                            let current = display_settings.read().grid_style;
                            display_settings.write().grid_style = current.cycle();
                        },
                        "{button_text}"
                    }
                }
            }

            // Pin names toggle (View option)
            button {
                style: "padding: 4px 8px; background: {pin_bg}; border: 1px solid {th.border()}; border-radius: 4px; color: {pin_color}; font-size: 12px; cursor: pointer; flex-shrink: 0; white-space: nowrap;",
                title: "Toggle terminal pin names",
                onclick: move |_| {
                    let mut ds = display_settings.write();
                    ds.show_pin_names = if matches!(ds.show_pin_names, crate::state::display_settings::PinNameVisibility::Always) {
                        crate::state::display_settings::PinNameVisibility::Hidden
                    } else {
                        crate::state::display_settings::PinNameVisibility::Always
                    };
                },
                "{pin_btn_label}"
            }

            // DC Annotation toggle
            {
                let mut sim_state: Signal<crate::state::SimulationState> = use_context();
                let mode = sim_state.read().dc_annotations.mode;
                let dc_label = match mode {
                    crate::state::dc_annotation::AnnotationMode::Hidden => "DC: Off",
                    crate::state::dc_annotation::AnnotationMode::Voltages => "DC: V",
                    crate::state::dc_annotation::AnnotationMode::Currents => "DC: I",
                    crate::state::dc_annotation::AnnotationMode::All => "DC: All",
                };
                let dc_active = !matches!(mode, crate::state::dc_annotation::AnnotationMode::Hidden);
                let dc_bg = if dc_active { th.accent_primary() } else { th.surface() };
                let dc_color = if dc_active { "#fff".to_string() } else { th.text_primary().to_string() };

                rsx! {
                    button {
                        style: "padding: 4px 8px; background: {dc_bg}; border: 1px solid {th.border()}; border-radius: 4px; color: {dc_color}; font-size: 12px; cursor: pointer; flex-shrink: 0; white-space: nowrap;",
                        title: "Cycle DC annotation mode",
                        onclick: move |_| {
                            let current_mode = sim_state.read().dc_annotations.mode;
                            let new_mode = current_mode.cycle();
                            sim_state.write().dc_annotations.mode = new_mode;
                        },
                        "{dc_label}"
                    }
                }
            }

            // Simulation divider
            div { style: "width: 1px; height: 18px; background: {th.border()}; margin: 0 8px; flex-shrink: 0;" }

            // Run/Stop simulation button - consolidated from main toolbar
            {
                let mut sim_state: Signal<crate::state::SimulationState> = use_context();
                let sim_config: Signal<crate::state::simulation_command::SimulationConfig> = use_context();
                let sim_options: Signal<crate::dialogs::SimulationOptions> = use_context();
                let is_running = sim_state.read().is_running;

                if is_running {
                    rsx! {
                        button {
                            style: "padding: 4px 12px; background: #e53935; border: none; border-radius: 4px; color: #fff; font-size: 12px; font-weight: 600; cursor: pointer; flex-shrink: 0; white-space: nowrap;",
                            title: "Stop Simulation",
                            onclick: move |_| {
                                sim_state.write().is_running = false;
                                sim_state.write().console_messages.push(
                                    crate::state::ConsoleMessage::warning("Simulation stopped".to_string())
                                );
                            },
                            "⏹ Stop"
                        }
                    }
                } else {
                    rsx! {
                        button {
                            style: "padding: 4px 12px; background: #43a047; border: none; border-radius: 4px; color: #fff; font-size: 12px; font-weight: 600; cursor: pointer; flex-shrink: 0; white-space: nowrap;",
                            title: "Run Simulation (F5)",
                            onclick: move |_| {
                                // Use the simulation_runner module for shared logic
                                crate::components::simulation_runner::run_simulation(
                                    schematic, sim_state, sim_config, sim_options
                                );
                            },
                            "▶ Run"
                        }
                    }
                }
            }

            div { style: "flex: 1;" }
            span { style: "font-size: 12px; color: {th.text_muted()}; flex-shrink: 0; white-space: nowrap;", {format!("{} components, {} wires", schematic.read().components.len(), schematic.read().wires.len())} }
        }
    }
}

/// Tool button component
#[component]
fn ToolBtn(label: &'static str, active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let (bg, col) = if active {
        (th.accent_primary(), "#fff")
    } else {
        (th.surface(), th.text_primary())
    };
    rsx! { button { style: "padding: 4px 8px; background: {bg}; border: 1px solid {th.border()}; border-radius: 4px; color: {col}; font-size: 12px; cursor: pointer; flex-shrink: 0; white-space: nowrap;", onclick: move |e| onclick.call(e), "{label}" } }
}

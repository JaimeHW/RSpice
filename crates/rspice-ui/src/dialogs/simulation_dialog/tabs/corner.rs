//! Corner Analysis Tab
//!
//! Configuration form for PVT (Process, Voltage, Temperature) corner
//! analysis to verify circuit robustness across manufacturing variations.

use dioxus::prelude::*;

use crate::dialogs::simulation_dialog::form_components::{FormInput, FormRow};
use crate::state::simulation_command::ProcessCorner;
use crate::theme::Theme;

/// Corner analysis configuration tab
#[component]
pub fn CornerTab(
    enabled: Signal<bool>,
    process_corners: Signal<Vec<ProcessCorner>>,
    voltage_corners: Signal<String>,
    temperature_corners: Signal<String>,
    full_matrix: Signal<bool>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let is_enabled = *enabled.read();
    let opacity = if is_enabled { "1" } else { "0.5" };
    let pointer = if is_enabled { "auto" } else { "none" };
    let is_full_matrix = *full_matrix.read();

    // Calculate total combinations for display
    let corners = process_corners.read();
    let v_count = voltage_corners
        .read()
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .count()
        .max(1);
    let t_count = temperature_corners
        .read()
        .split(',')
        .filter(|s| !s.trim().is_empty())
        .count()
        .max(1);
    let total = if is_full_matrix {
        corners.len() * v_count * t_count
    } else {
        corners.len().max(v_count).max(t_count)
    };

    rsx! {
        div {
            // Enable checkbox
            label {
                style: "display: flex; align-items: center; gap: 8px; margin-bottom: 20px; cursor: pointer;",
                input {
                    r#type: "checkbox",
                    checked: is_enabled,
                    onchange: move |e| enabled.set(e.checked()),
                }
                span {
                    style: "font-weight: 500; color: {th.text_primary()};",
                    "Enable Corner Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                // Description
                div {
                    style: "margin-bottom: 16px; padding: 12px; background: {th.bg_tertiary()}; border-radius: 6px; font-size: 12px; color: {th.text_secondary()};",
                    "Corner analysis sweeps across PVT (Process, Voltage, Temperature) variations to verify circuit robustness under worst-case manufacturing conditions."
                }

                // Process corners as checkboxes
                div {
                    style: "margin-bottom: 16px;",
                    div {
                        style: "font-size: 12px; font-weight: 500; color: {th.text_primary()}; margin-bottom: 8px;",
                        "Process Corners"
                    }
                    div {
                        style: "display: flex; flex-wrap: wrap; gap: 12px;",
                        for corner in ProcessCorner::ALL.iter() {
                            {
                                let corner_copy = *corner;
                                let is_selected = corners.contains(&corner_copy);
                                rsx! {
                                    label {
                                        style: "display: flex; align-items: center; gap: 6px; cursor: pointer;",
                                        input {
                                            r#type: "checkbox",
                                            checked: is_selected,
                                            onchange: move |e| {
                                                let mut current = process_corners.read().clone();
                                                if e.checked() {
                                                    if !current.contains(&corner_copy) {
                                                        current.push(corner_copy);
                                                    }
                                                } else {
                                                    current.retain(|c| *c != corner_copy);
                                                }
                                                process_corners.set(current);
                                            },
                                        }
                                        span {
                                            style: "font-size: 12px; color: {th.text_secondary()};",
                                            "{corner.display_name()}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Voltage corners
                FormRow {
                    label: "Voltage Corners",
                    help: "Comma-separated % values (e.g., 90, 100, 110)",
                    FormInput {
                        value: voltage_corners.read().clone(),
                        placeholder: "90, 100, 110",
                        suffix: "%",
                        onchange: move |v| voltage_corners.set(v),
                    }
                }

                // Temperature corners
                FormRow {
                    label: "Temperature Corners",
                    help: "Comma-separated °C values",
                    FormInput {
                        value: temperature_corners.read().clone(),
                        placeholder: "-40, 25, 125",
                        suffix: "°C",
                        onchange: move |v| temperature_corners.set(v),
                    }
                }

                // Full matrix option
                div {
                    style: "margin-top: 16px; padding-top: 16px; border-top: 1px solid {th.border()};",

                    label {
                        style: "display: flex; align-items: center; gap: 8px; cursor: pointer;",
                        input {
                            r#type: "checkbox",
                            checked: is_full_matrix,
                            onchange: move |e| full_matrix.set(e.checked()),
                        }
                        span {
                            style: "font-size: 13px; color: {th.text_secondary()};",
                            "Full matrix (all combinations)"
                        }
                    }

                    div {
                        style: "margin-top: 12px; padding: 8px 12px; background: {th.bg_tertiary()}; border-radius: 4px; font-size: 12px; color: {th.text_muted()};",
                        "Total simulations: {total}"
                    }
                }
            }
        }
    }
}

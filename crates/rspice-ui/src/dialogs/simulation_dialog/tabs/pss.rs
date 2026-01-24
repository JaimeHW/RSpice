//! PSS (Periodic Steady State) Analysis Tab
//!
//! Configuration form for periodic steady state analysis of oscillators
//! and other periodically-forced circuits.

use dioxus::prelude::*;

use crate::dialogs::simulation_dialog::form_components::{FormInput, FormRow};
use crate::theme::Theme;

/// PSS analysis configuration tab
#[component]
pub fn PssTab(
    enabled: Signal<bool>,
    fundamental_freq: Signal<String>,
    auto_detect: Signal<bool>,
    num_harmonics: Signal<String>,
    stabilization_cycles: Signal<String>,
    tolerance: Signal<String>,
    max_iterations: Signal<String>,
    floquet_analysis: Signal<bool>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let is_enabled = *enabled.read();
    let is_auto = *auto_detect.read();
    let opacity = if is_enabled { "1" } else { "0.5" };
    let pointer = if is_enabled { "auto" } else { "none" };
    let auto_opacity = if is_auto { "0.5" } else { "1" };
    let auto_pointer = if is_auto { "none" } else { "auto" };

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
                    "Enable PSS Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                // Description
                div {
                    style: "margin-bottom: 16px; padding: 12px; background: {th.bg_tertiary()}; border-radius: 6px; font-size: 12px; color: {th.text_secondary()};",
                    "PSS finds the periodic steady-state operating point for autonomous oscillators or periodically-forced circuits. Required for PAC, PNOISE, and PSTB analyses."
                }

                // Auto-detect checkbox
                label {
                    style: "display: flex; align-items: center; gap: 8px; margin-bottom: 16px; cursor: pointer;",
                    input {
                        r#type: "checkbox",
                        checked: is_auto,
                        onchange: move |e| auto_detect.set(e.checked()),
                    }
                    span {
                        style: "font-size: 13px; color: {th.text_secondary()};",
                        "Auto-detect fundamental period (for oscillators)"
                    }
                }

                // Frequency input (disabled when auto-detect is on)
                div {
                    style: "opacity: {auto_opacity}; pointer-events: {auto_pointer};",
                    FormRow {
                        label: "Fundamental Frequency",
                        help: "Expected oscillation frequency",
                        FormInput {
                            value: fundamental_freq.read().clone(),
                            placeholder: "1MEG",
                            suffix: "Hz",
                            onchange: move |v| fundamental_freq.set(v),
                        }
                    }
                }

                FormRow {
                    label: "Number of Harmonics",
                    help: "Harmonics to compute (affects accuracy)",
                    FormInput {
                        value: num_harmonics.read().clone(),
                        placeholder: "10",
                        suffix: "",
                        onchange: move |v| num_harmonics.set(v),
                    }
                }

                FormRow {
                    label: "Stabilization Cycles",
                    help: "Periods before steady-state check",
                    FormInput {
                        value: stabilization_cycles.read().clone(),
                        placeholder: "3",
                        suffix: "",
                        onchange: move |v| stabilization_cycles.set(v),
                    }
                }

                // Advanced options (collapsible in future)
                div {
                    style: "margin-top: 16px; padding-top: 16px; border-top: 1px solid {th.border()};",
                    div {
                        style: "font-size: 11px; color: {th.text_muted()}; margin-bottom: 12px; text-transform: uppercase; letter-spacing: 0.5px;",
                        "Advanced Options"
                    }

                    FormRow {
                        label: "Tolerance",
                        help: "Convergence tolerance",
                        FormInput {
                            value: tolerance.read().clone(),
                            placeholder: "1e-6",
                            suffix: "",
                            onchange: move |v| tolerance.set(v),
                        }
                    }

                    FormRow {
                        label: "Max Iterations",
                        help: "Maximum Newton iterations per period",
                        FormInput {
                            value: max_iterations.read().clone(),
                            placeholder: "50",
                            suffix: "",
                            onchange: move |v| max_iterations.set(v),
                        }
                    }

                    label {
                        style: "display: flex; align-items: center; gap: 8px; margin-top: 12px; cursor: pointer;",
                        input {
                            r#type: "checkbox",
                            checked: *floquet_analysis.read(),
                            onchange: move |e| floquet_analysis.set(e.checked()),
                        }
                        span {
                            style: "font-size: 13px; color: {th.text_secondary()};",
                            "Enable Floquet stability analysis"
                        }
                    }
                }
            }
        }
    }
}

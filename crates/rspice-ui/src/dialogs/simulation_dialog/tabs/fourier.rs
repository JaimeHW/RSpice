//! Fourier/THD Analysis Tab
//!
//! Configuration form for Fourier analysis and Total Harmonic
//! Distortion (THD) measurement of periodic waveforms.

use dioxus::prelude::*;

use crate::dialogs::simulation_dialog::form_components::{FormInput, FormRow};
use crate::theme::Theme;

/// Fourier analysis configuration tab
#[component]
pub fn FourierTab(
    enabled: Signal<bool>,
    fundamental_freq: Signal<String>,
    num_harmonics: Signal<String>,
    output_var: Signal<String>,
    calculate_thd: Signal<bool>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let is_enabled = *enabled.read();
    let opacity = if is_enabled { "1" } else { "0.5" };
    let pointer = if is_enabled { "auto" } else { "none" };

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
                    "Enable Fourier/THD Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                // Description
                div {
                    style: "margin-bottom: 16px; padding: 12px; background: {th.bg_tertiary()}; border-radius: 6px; font-size: 12px; color: {th.text_secondary()};",
                    "Fourier analysis decomposes a periodic waveform into its harmonic components and calculates Total Harmonic Distortion (THD). Requires a transient simulation to run first."
                }

                // Note about transient dependency
                div {
                    style: "margin-bottom: 16px; padding: 10px 12px; background: rgba(100, 149, 237, 0.1); border: 1px solid rgba(100, 149, 237, 0.3); border-radius: 6px; font-size: 12px; color: #6495ed;",
                    "ℹ Note: Fourier analysis is performed on transient simulation results."
                }

                FormRow {
                    label: "Fundamental Frequency",
                    help: "Base frequency for harmonic analysis",
                    FormInput {
                        value: fundamental_freq.read().clone(),
                        placeholder: "1k",
                        suffix: "Hz",
                        onchange: move |v| fundamental_freq.set(v),
                    }
                }

                FormRow {
                    label: "Number of Harmonics",
                    help: "How many harmonics to compute",
                    FormInput {
                        value: num_harmonics.read().clone(),
                        placeholder: "10",
                        suffix: "",
                        onchange: move |v| num_harmonics.set(v),
                    }
                }

                FormRow {
                    label: "Output Variable",
                    help: "Signal to analyze",
                    FormInput {
                        value: output_var.read().clone(),
                        placeholder: "V(out)",
                        suffix: "",
                        onchange: move |v| output_var.set(v),
                    }
                }

                // THD calculation option
                label {
                    style: "display: flex; align-items: center; gap: 8px; margin-top: 16px; cursor: pointer;",
                    input {
                        r#type: "checkbox",
                        checked: *calculate_thd.read(),
                        onchange: move |e| calculate_thd.set(e.checked()),
                    }
                    span {
                        style: "font-size: 13px; color: {th.text_secondary()};",
                        "Calculate Total Harmonic Distortion (THD)"
                    }
                }

                // THD formula reference
                div {
                    style: "margin-top: 16px; padding: 12px; background: {th.bg_tertiary()}; border-radius: 6px;",
                    div {
                        style: "font-size: 11px; color: {th.text_muted()}; margin-bottom: 4px;",
                        "THD Formula:"
                    }
                    div {
                        style: "font-family: 'JetBrains Mono', 'Fira Code', monospace; font-size: 12px; color: {th.text_secondary()};",
                        "THD = √(V₂² + V₃² + ... + Vₙ²) / V₁ × 100%"
                    }
                }
            }
        }
    }
}

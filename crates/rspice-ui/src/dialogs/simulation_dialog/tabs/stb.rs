//! STB (Loop Stability) Analysis Tab
//!
//! Configuration form for loop stability analysis of feedback systems.
//! Computes gain/phase margins and generates Bode/Nyquist plots.

use dioxus::prelude::*;

use crate::dialogs::simulation_dialog::form_components::{FormInput, FormRow};
use crate::theme::Theme;

/// STB analysis configuration tab
#[component]
pub fn StbTab(
    enabled: Signal<bool>,
    probe_pos: Signal<String>,
    probe_neg: Signal<String>,
    start_freq: Signal<String>,
    stop_freq: Signal<String>,
    points_per_decade: Signal<String>,
    target_phase_margin: Signal<String>,
    target_gain_margin: Signal<String>,
    show_nyquist: Signal<bool>,
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
                    "Enable STB (Stability) Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                // Description
                div {
                    style: "margin-bottom: 16px; padding: 12px; background: {th.bg_tertiary()}; border-radius: 6px; font-size: 12px; color: {th.text_secondary()};",
                    "STB analysis measures loop gain and phase for feedback systems, providing gain margin, phase margin, and crossover frequency. Essential for amplifier and regulator stability verification."
                }

                // Probe insertion point
                div {
                    style: "margin-bottom: 16px; padding: 12px; border: 1px solid {th.border()}; border-radius: 6px;",
                    div {
                        style: "font-size: 12px; font-weight: 500; color: {th.text_primary()}; margin-bottom: 12px;",
                        "Loop Break Point"
                    }
                    div {
                        style: "font-size: 11px; color: {th.text_muted()}; margin-bottom: 12px;",
                        "Insert the STB probe where the feedback loop should be broken."
                    }

                    FormRow {
                        label: "Probe Node (+)",
                        help: "Positive terminal of loop break",
                        FormInput {
                            value: probe_pos.read().clone(),
                            placeholder: "fb",
                            suffix: "",
                            onchange: move |v| probe_pos.set(v),
                        }
                    }

                    FormRow {
                        label: "Probe Node (-)",
                        help: "Negative terminal (usually ground)",
                        FormInput {
                            value: probe_neg.read().clone(),
                            placeholder: "0",
                            suffix: "",
                            onchange: move |v| probe_neg.set(v),
                        }
                    }
                }

                // Frequency range
                FormRow {
                    label: "Start Frequency",
                    help: "Beginning of frequency sweep",
                    FormInput {
                        value: start_freq.read().clone(),
                        placeholder: "1",
                        suffix: "Hz",
                        onchange: move |v| start_freq.set(v),
                    }
                }

                FormRow {
                    label: "Stop Frequency",
                    help: "End of frequency sweep",
                    FormInput {
                        value: stop_freq.read().clone(),
                        placeholder: "100MEG",
                        suffix: "Hz",
                        onchange: move |v| stop_freq.set(v),
                    }
                }

                FormRow {
                    label: "Points/Decade",
                    help: "Frequency resolution",
                    FormInput {
                        value: points_per_decade.read().clone(),
                        placeholder: "20",
                        suffix: "",
                        onchange: move |v| points_per_decade.set(v),
                    }
                }

                // Margin targets
                div {
                    style: "margin-top: 16px; padding-top: 16px; border-top: 1px solid {th.border()};",
                    div {
                        style: "font-size: 11px; color: {th.text_muted()}; margin-bottom: 12px; text-transform: uppercase; letter-spacing: 0.5px;",
                        "Stability Targets"
                    }

                    FormRow {
                        label: "Phase Margin Target",
                        help: "Minimum acceptable phase margin",
                        FormInput {
                            value: target_phase_margin.read().clone(),
                            placeholder: "45",
                            suffix: "°",
                            onchange: move |v| target_phase_margin.set(v),
                        }
                    }

                    FormRow {
                        label: "Gain Margin Target",
                        help: "Minimum acceptable gain margin",
                        FormInput {
                            value: target_gain_margin.read().clone(),
                            placeholder: "10",
                            suffix: "dB",
                            onchange: move |v| target_gain_margin.set(v),
                        }
                    }

                    label {
                        style: "display: flex; align-items: center; gap: 8px; margin-top: 12px; cursor: pointer;",
                        input {
                            r#type: "checkbox",
                            checked: *show_nyquist.read(),
                            onchange: move |e| show_nyquist.set(e.checked()),
                        }
                        span {
                            style: "font-size: 13px; color: {th.text_secondary()};",
                            "Generate Nyquist plot"
                        }
                    }
                }
            }
        }
    }
}

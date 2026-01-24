//! Harmonic Balance Analysis Tab
//!
//! Configuration form for harmonic balance analysis of RF and
//! microwave circuits with multi-tone excitation.

use dioxus::prelude::*;

use crate::dialogs::simulation_dialog::form_components::{FormInput, FormRow};
use crate::theme::Theme;

/// Harmonic Balance analysis configuration tab
#[component]
pub fn HarmonicBalanceTab(
    enabled: Signal<bool>,
    tone1_freq: Signal<String>,
    tone1_harmonics: Signal<String>,
    tone1_source: Signal<String>,
    tone2_enabled: Signal<bool>,
    tone2_freq: Signal<String>,
    tone2_harmonics: Signal<String>,
    tone2_source: Signal<String>,
    max_order: Signal<String>,
    tolerance: Signal<String>,
    max_iterations: Signal<String>,
    use_krylov: Signal<bool>,
    oversample: Signal<String>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let is_enabled = *enabled.read();
    let is_tone2 = *tone2_enabled.read();
    let opacity = if is_enabled { "1" } else { "0.5" };
    let pointer = if is_enabled { "auto" } else { "none" };
    let tone2_opacity = if is_tone2 { "1" } else { "0.5" };
    let tone2_pointer = if is_tone2 { "auto" } else { "none" };

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
                    "Enable Harmonic Balance Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                // Description
                div {
                    style: "margin-bottom: 16px; padding: 12px; background: {th.bg_tertiary()}; border-radius: 6px; font-size: 12px; color: {th.text_secondary()};",
                    "Harmonic Balance solves for the steady-state response in the frequency domain. Ideal for RF/microwave circuits like mixers, amplifiers, and oscillators with multi-tone excitation."
                }

                // Tone 1
                div {
                    style: "margin-bottom: 16px; padding: 12px; border: 1px solid {th.border()}; border-radius: 6px;",
                    div {
                        style: "font-size: 12px; font-weight: 500; color: {th.text_primary()}; margin-bottom: 12px;",
                        "Tone 1 (Primary)"
                    }

                    FormRow {
                        label: "Frequency",
                        help: "Fundamental frequency",
                        FormInput {
                            value: tone1_freq.read().clone(),
                            placeholder: "1G",
                            suffix: "Hz",
                            onchange: move |v| tone1_freq.set(v),
                        }
                    }

                    FormRow {
                        label: "Harmonics",
                        help: "Number of harmonics",
                        FormInput {
                            value: tone1_harmonics.read().clone(),
                            placeholder: "7",
                            suffix: "",
                            onchange: move |v| tone1_harmonics.set(v),
                        }
                    }

                    FormRow {
                        label: "Source",
                        help: "Associated source name",
                        FormInput {
                            value: tone1_source.read().clone(),
                            placeholder: "V1",
                            suffix: "",
                            onchange: move |v| tone1_source.set(v),
                        }
                    }
                }

                // Tone 2 (optional)
                div {
                    style: "margin-bottom: 16px; padding: 12px; border: 1px solid {th.border()}; border-radius: 6px;",
                    label {
                        style: "display: flex; align-items: center; gap: 8px; margin-bottom: 12px; cursor: pointer;",
                        input {
                            r#type: "checkbox",
                            checked: is_tone2,
                            onchange: move |e| tone2_enabled.set(e.checked()),
                        }
                        span {
                            style: "font-size: 12px; font-weight: 500; color: {th.text_primary()};",
                            "Tone 2 (Two-Tone Analysis)"
                        }
                    }

                    div {
                        style: "opacity: {tone2_opacity}; pointer-events: {tone2_pointer};",

                        FormRow {
                            label: "Frequency",
                            help: "Second tone frequency",
                            FormInput {
                                value: tone2_freq.read().clone(),
                                placeholder: "1.001G",
                                suffix: "Hz",
                                onchange: move |v| tone2_freq.set(v),
                            }
                        }

                        FormRow {
                            label: "Harmonics",
                            help: "Number of harmonics",
                            FormInput {
                                value: tone2_harmonics.read().clone(),
                                placeholder: "3",
                                suffix: "",
                                onchange: move |v| tone2_harmonics.set(v),
                            }
                        }

                        FormRow {
                            label: "Source",
                            help: "Associated source name",
                            FormInput {
                                value: tone2_source.read().clone(),
                                placeholder: "V2",
                                suffix: "",
                                onchange: move |v| tone2_source.set(v),
                            }
                        }
                    }
                }

                FormRow {
                    label: "Max IM Order",
                    help: "Maximum intermodulation order",
                    FormInput {
                        value: max_order.read().clone(),
                        placeholder: "7",
                        suffix: "",
                        onchange: move |v| max_order.set(v),
                    }
                }

                // Advanced options
                div {
                    style: "margin-top: 16px; padding-top: 16px; border-top: 1px solid {th.border()};",
                    div {
                        style: "font-size: 11px; color: {th.text_muted()}; margin-bottom: 12px; text-transform: uppercase; letter-spacing: 0.5px;",
                        "Solver Options"
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
                        help: "Maximum solver iterations",
                        FormInput {
                            value: max_iterations.read().clone(),
                            placeholder: "200",
                            suffix: "",
                            onchange: move |v| max_iterations.set(v),
                        }
                    }

                    FormRow {
                        label: "Oversample",
                        help: "Time-domain oversampling factor",
                        FormInput {
                            value: oversample.read().clone(),
                            placeholder: "4",
                            suffix: "x",
                            onchange: move |v| oversample.set(v),
                        }
                    }

                    label {
                        style: "display: flex; align-items: center; gap: 8px; margin-top: 12px; cursor: pointer;",
                        input {
                            r#type: "checkbox",
                            checked: *use_krylov.read(),
                            onchange: move |e| use_krylov.set(e.checked()),
                        }
                        span {
                            style: "font-size: 13px; color: {th.text_secondary()};",
                            "Use Krylov subspace solver (faster for large problems)"
                        }
                    }
                }
            }
        }
    }
}

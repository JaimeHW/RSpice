//! Envelope Transient Analysis Tab
//!
//! Configuration form for envelope transient analysis of modulated
//! RF signals, providing faster simulation of slowly-varying envelopes.

use dioxus::prelude::*;

use crate::dialogs::simulation_dialog::form_components::{FormInput, FormRow, FormSelect};
use crate::state::simulation_command::ModulationType;
use crate::theme::Theme;

/// Envelope transient analysis configuration tab
#[component]
pub fn EnvelopeTab(
    enabled: Signal<bool>,
    carrier_freq: Signal<String>,
    modulation_type: Signal<ModulationType>,
    envelope_bandwidth: Signal<String>,
    stop_time: Signal<String>,
    max_step: Signal<String>,
    carrier_harmonics: Signal<String>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let is_enabled = *enabled.read();
    let opacity = if is_enabled { "1" } else { "0.5" };
    let pointer = if is_enabled { "auto" } else { "none" };
    let current_mod = *modulation_type.read();

    let mod_options = vec![
        ModulationType::Am.display_name().to_string(),
        ModulationType::Fm.display_name().to_string(),
        ModulationType::Pm.display_name().to_string(),
        ModulationType::Iq.display_name().to_string(),
    ];

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
                    "Enable Envelope Transient Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                // Description
                div {
                    style: "margin-bottom: 16px; padding: 12px; background: {th.bg_tertiary()}; border-radius: 6px; font-size: 12px; color: {th.text_secondary()};",
                    "Envelope transient extracts the slowly-varying envelope of modulated RF signals, enabling efficient simulation of AM/FM/PM systems over many carrier cycles."
                }

                FormRow {
                    label: "Carrier Frequency",
                    help: "RF carrier frequency",
                    FormInput {
                        value: carrier_freq.read().clone(),
                        placeholder: "1G",
                        suffix: "Hz",
                        onchange: move |v| carrier_freq.set(v),
                    }
                }

                FormRow {
                    label: "Modulation Type",
                    help: "Type of envelope extraction",
                    FormSelect {
                        value: current_mod.display_name().to_string(),
                        options: mod_options,
                        onchange: move |v: String| {
                            let mt = match v.as_str() {
                                "AM (Amplitude)" => ModulationType::Am,
                                "FM (Frequency)" => ModulationType::Fm,
                                "PM (Phase)" => ModulationType::Pm,
                                "IQ (Complex)" => ModulationType::Iq,
                                _ => ModulationType::Am,
                            };
                            modulation_type.set(mt);
                        },
                    }
                }

                FormRow {
                    label: "Envelope Bandwidth",
                    help: "Maximum modulation frequency",
                    FormInput {
                        value: envelope_bandwidth.read().clone(),
                        placeholder: "10MEG",
                        suffix: "Hz",
                        onchange: move |v| envelope_bandwidth.set(v),
                    }
                }

                FormRow {
                    label: "Stop Time",
                    help: "Envelope simulation duration",
                    FormInput {
                        value: stop_time.read().clone(),
                        placeholder: "1u",
                        suffix: "s",
                        onchange: move |v| stop_time.set(v),
                    }
                }

                FormRow {
                    label: "Max Time Step",
                    help: "Maximum envelope step (optional)",
                    FormInput {
                        value: max_step.read().clone(),
                        placeholder: "(auto)",
                        suffix: "s",
                        onchange: move |v| max_step.set(v),
                    }
                }

                FormRow {
                    label: "Carrier Harmonics",
                    help: "Number of carrier harmonics",
                    FormInput {
                        value: carrier_harmonics.read().clone(),
                        placeholder: "3",
                        suffix: "",
                        onchange: move |v| carrier_harmonics.set(v),
                    }
                }
            }
        }
    }
}

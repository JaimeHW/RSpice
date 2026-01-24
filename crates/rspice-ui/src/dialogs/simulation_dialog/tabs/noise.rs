//! Noise Analysis Tab
//!
//! Configuration form for noise simulation.

use dioxus::prelude::*;

use crate::dialogs::simulation_dialog::form_components::{FormInput, FormRow};
use crate::theme::Theme;

/// Noise analysis configuration tab
#[component]
pub fn NoiseTab(
    enabled: Signal<bool>,
    output_node: Signal<String>,
    ref_node: Signal<String>,
    input_source: Signal<String>,
    start_freq: Signal<String>,
    stop_freq: Signal<String>,
    points: Signal<String>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let is_enabled = *enabled.read();
    let opacity = if is_enabled { "1" } else { "0.5" };
    let pointer = if is_enabled { "auto" } else { "none" };

    rsx! {
        div {
            label {
                style: "display: flex; align-items: center; gap: 8px; margin-bottom: 20px; cursor: pointer;",
                input {
                    r#type: "checkbox",
                    checked: is_enabled,
                    onchange: move |e| enabled.set(e.checked()),
                }
                span {
                    style: "font-weight: 500; color: {th.text_primary()};",
                    "Enable Noise Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                FormRow {
                    label: "Output Node",
                    help: "Node to measure output noise",
                    FormInput {
                        value: output_node.read().clone(),
                        placeholder: "out",
                        suffix: "",
                        onchange: move |v| output_node.set(v),
                    }
                }

                FormRow {
                    label: "Reference Node",
                    help: "Reference for output (usually 0)",
                    FormInput {
                        value: ref_node.read().clone(),
                        placeholder: "0",
                        suffix: "",
                        onchange: move |v| ref_node.set(v),
                    }
                }

                FormRow {
                    label: "Input Source",
                    help: "Source for input-referred noise",
                    FormInput {
                        value: input_source.read().clone(),
                        placeholder: "Vin",
                        suffix: "",
                        onchange: move |v| input_source.set(v),
                    }
                }

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
                        placeholder: "1MEG",
                        suffix: "Hz",
                        onchange: move |v| stop_freq.set(v),
                    }
                }

                FormRow {
                    label: "Points/Decade",
                    help: "Frequency resolution",
                    FormInput {
                        value: points.read().clone(),
                        placeholder: "10",
                        suffix: "",
                        onchange: move |v| points.set(v),
                    }
                }
            }
        }
    }
}

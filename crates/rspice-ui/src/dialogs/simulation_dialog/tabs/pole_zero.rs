//! Pole-Zero Analysis Tab
//!
//! Configuration form for pole-zero simulation.

use dioxus::prelude::*;

use crate::dialogs::simulation_dialog::form_components::{FormInput, FormRow};
use crate::theme::Theme;

/// Pole-Zero analysis configuration tab
#[component]
pub fn PoleZeroTab(
    enabled: Signal<bool>,
    input_pos: Signal<String>,
    input_neg: Signal<String>,
    output_pos: Signal<String>,
    output_neg: Signal<String>,
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
                    "Enable Pole-Zero Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                p {
                    style: "color: {th.text_muted()}; font-size: 12px; margin: 0 0 16px 0;",
                    "Finds poles and zeros of the transfer function for stability and frequency analysis."
                }

                FormRow {
                    label: "Input (+)",
                    help: "Positive input node",
                    FormInput {
                        value: input_pos.read().clone(),
                        placeholder: "in",
                        suffix: "",
                        onchange: move |v| input_pos.set(v),
                    }
                }

                FormRow {
                    label: "Input (−)",
                    help: "Negative input node (ref)",
                    FormInput {
                        value: input_neg.read().clone(),
                        placeholder: "0",
                        suffix: "",
                        onchange: move |v| input_neg.set(v),
                    }
                }

                FormRow {
                    label: "Output (+)",
                    help: "Positive output node",
                    FormInput {
                        value: output_pos.read().clone(),
                        placeholder: "out",
                        suffix: "",
                        onchange: move |v| output_pos.set(v),
                    }
                }

                FormRow {
                    label: "Output (−)",
                    help: "Negative output node (ref)",
                    FormInput {
                        value: output_neg.read().clone(),
                        placeholder: "0",
                        suffix: "",
                        onchange: move |v| output_neg.set(v),
                    }
                }
            }
        }
    }
}

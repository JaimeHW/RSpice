//! DC Sweep Analysis Tab
//!
//! Configuration form for DC sweep simulation.

use dioxus::prelude::*;

use crate::dialogs::simulation_dialog::form_components::{FormInput, FormRow};
use crate::theme::Theme;

/// DC sweep analysis configuration tab
#[component]
pub fn DcSweepTab(
    enabled: Signal<bool>,
    source_name: Signal<String>,
    start_value: Signal<String>,
    stop_value: Signal<String>,
    step_value: Signal<String>,
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
                    "Enable DC Sweep Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                FormRow {
                    label: "Source Name",
                    help: "Voltage or current source to sweep",
                    FormInput {
                        value: source_name.read().clone(),
                        placeholder: "V1",
                        suffix: "",
                        onchange: move |v| source_name.set(v),
                    }
                }

                FormRow {
                    label: "Start Value",
                    help: "Beginning of sweep",
                    FormInput {
                        value: start_value.read().clone(),
                        placeholder: "0",
                        suffix: "V",
                        onchange: move |v| start_value.set(v),
                    }
                }

                FormRow {
                    label: "Stop Value",
                    help: "End of sweep",
                    FormInput {
                        value: stop_value.read().clone(),
                        placeholder: "5",
                        suffix: "V",
                        onchange: move |v| stop_value.set(v),
                    }
                }

                FormRow {
                    label: "Increment",
                    help: "Step size",
                    FormInput {
                        value: step_value.read().clone(),
                        placeholder: "0.1",
                        suffix: "V",
                        onchange: move |v| step_value.set(v),
                    }
                }
            }
        }
    }
}

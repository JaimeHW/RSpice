//! Transient Analysis Tab
//!
//! Configuration form for time-domain transient simulation.

use dioxus::prelude::*;

use crate::dialogs::simulation_dialog::form_components::{FormInput, FormRow};
use crate::theme::Theme;

/// Transient analysis configuration tab
#[component]
pub fn TransientTab(
    enabled: Signal<bool>,
    stop_time: Signal<String>,
    time_step: Signal<String>,
    start_time: Signal<String>,
    max_step: Signal<String>,
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
                    "Enable Transient Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                FormRow {
                    label: "Stop Time",
                    help: "End time for simulation",
                    FormInput {
                        value: stop_time.read().clone(),
                        placeholder: "1m",
                        suffix: "s",
                        onchange: move |v| stop_time.set(v),
                    }
                }

                FormRow {
                    label: "Time Step",
                    help: "Suggested internal step",
                    FormInput {
                        value: time_step.read().clone(),
                        placeholder: "1u",
                        suffix: "s",
                        onchange: move |v| time_step.set(v),
                    }
                }

                FormRow {
                    label: "Start Saving At",
                    help: "Skip data before this time",
                    FormInput {
                        value: start_time.read().clone(),
                        placeholder: "0",
                        suffix: "s",
                        onchange: move |v| start_time.set(v),
                    }
                }

                FormRow {
                    label: "Maximum Step",
                    help: "Limit adaptive stepping (optional)",
                    FormInput {
                        value: max_step.read().clone(),
                        placeholder: "(auto)",
                        suffix: "s",
                        onchange: move |v| max_step.set(v),
                    }
                }
            }
        }
    }
}

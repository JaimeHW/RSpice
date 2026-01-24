//! Sensitivity Analysis Tab
//!
//! Configuration form for DC and AC sensitivity simulation.

use dioxus::prelude::*;

use crate::dialogs::simulation_dialog::form_components::{FormInput, FormRow};
use crate::theme::Theme;

/// Sensitivity analysis configuration tab
#[component]
pub fn SensitivityTab(
    enabled: Signal<bool>,
    output_var: Signal<String>,
    is_ac: Signal<bool>,
    frequency: Signal<String>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let is_enabled = *enabled.read();
    let is_ac_mode = *is_ac.read();
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
                    "Enable Sensitivity Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                FormRow {
                    label: "Output Variable",
                    help: "Variable to analyze sensitivity",
                    FormInput {
                        value: output_var.read().clone(),
                        placeholder: "V(out)",
                        suffix: "",
                        onchange: move |v| output_var.set(v),
                    }
                }

                FormRow {
                    label: "Analysis Type",
                    help: "DC or AC sensitivity",
                    div {
                        style: "display: flex; gap: 16px;",
                        label {
                            style: "display: flex; align-items: center; gap: 6px; cursor: pointer;",
                            input {
                                r#type: "radio",
                                name: "sens_type",
                                checked: !is_ac_mode,
                                onchange: move |_| is_ac.set(false),
                            }
                            span { style: "color: {th.text_primary()};", "DC" }
                        }
                        label {
                            style: "display: flex; align-items: center; gap: 6px; cursor: pointer;",
                            input {
                                r#type: "radio",
                                name: "sens_type",
                                checked: is_ac_mode,
                                onchange: move |_| is_ac.set(true),
                            }
                            span { style: "color: {th.text_primary()};", "AC" }
                        }
                    }
                }

                if is_ac_mode {
                    FormRow {
                        label: "Frequency",
                        help: "AC sensitivity frequency",
                        FormInput {
                            value: frequency.read().clone(),
                            placeholder: "1MEG",
                            suffix: "Hz",
                            onchange: move |v| frequency.set(v),
                        }
                    }
                }
            }
        }
    }
}

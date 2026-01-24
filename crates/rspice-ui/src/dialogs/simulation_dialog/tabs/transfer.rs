//! Transfer Function Analysis Tab
//!
//! Configuration form for DC transfer function (.TF) analysis,
//! computing small-signal input/output resistance and gain.

use dioxus::prelude::*;

use crate::dialogs::simulation_dialog::form_components::{FormInput, FormRow};
use crate::theme::Theme;

/// Transfer function analysis configuration tab
#[component]
pub fn TransferTab(
    enabled: Signal<bool>,
    output_var: Signal<String>,
    input_source: Signal<String>,
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
                    "Enable Transfer Function Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                // Description
                div {
                    style: "margin-bottom: 16px; padding: 12px; background: {th.bg_tertiary()}; border-radius: 6px; font-size: 12px; color: {th.text_secondary()};",
                    "Transfer function analysis (.TF) computes the DC small-signal transfer function, input resistance, and output resistance for a specified input-output pair."
                }

                // Results preview
                div {
                    style: "margin-bottom: 16px; padding: 12px; border: 1px solid {th.border()}; border-radius: 6px;",
                    div {
                        style: "font-size: 11px; color: {th.text_muted()}; margin-bottom: 8px;",
                        "This analysis will compute:"
                    }
                    ul {
                        style: "margin: 0; padding-left: 20px; font-size: 12px; color: {th.text_secondary()};",
                        li { "Transfer function: output/input ratio" }
                        li { "Input resistance at the input source" }
                        li { "Output resistance at the output node" }
                    }
                }

                FormRow {
                    label: "Output Variable",
                    help: "Node voltage or current to measure",
                    FormInput {
                        value: output_var.read().clone(),
                        placeholder: "V(out)",
                        suffix: "",
                        onchange: move |v| output_var.set(v),
                    }
                }

                FormRow {
                    label: "Input Source",
                    help: "Independent source for small-signal excitation",
                    FormInput {
                        value: input_source.read().clone(),
                        placeholder: "Vin",
                        suffix: "",
                        onchange: move |v| input_source.set(v),
                    }
                }
            }
        }
    }
}

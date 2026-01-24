//! Parametric Sweep Analysis Tab
//!
//! Configuration form for parametric sweep analysis (.STEP),
//! varying component or model parameters across simulations.

use dioxus::prelude::*;

use crate::dialogs::simulation_dialog::form_components::{FormInput, FormRow, FormSelect};
use crate::state::simulation_command::ParametricStepType;
use crate::theme::Theme;

/// Parametric sweep analysis configuration tab
#[component]
pub fn ParametricTab(
    enabled: Signal<bool>,
    param_name: Signal<String>,
    start_value: Signal<String>,
    stop_value: Signal<String>,
    step_type: Signal<ParametricStepType>,
    num_steps: Signal<String>,
    values_list: Signal<String>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let is_enabled = *enabled.read();
    let opacity = if is_enabled { "1" } else { "0.5" };
    let pointer = if is_enabled { "auto" } else { "none" };
    let current_type = *step_type.read();
    let is_list = current_type == ParametricStepType::List;

    let step_options = vec![
        ParametricStepType::Linear.display_name().to_string(),
        ParametricStepType::Decade.display_name().to_string(),
        ParametricStepType::Octave.display_name().to_string(),
        ParametricStepType::List.display_name().to_string(),
    ];

    // Pre-compute styles for RSX interpolation
    let range_opacity = if is_list { "0.5" } else { "1" };
    let range_pointer = if is_list { "none" } else { "auto" };
    let list_opacity = if is_list { "1" } else { "0.5" };
    let list_pointer = if is_list { "auto" } else { "none" };
    let step_label = if current_type == ParametricStepType::Linear {
        "Number of Steps"
    } else {
        "Points per Decade"
    };

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
                    "Enable Parametric Sweep"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                // Description
                div {
                    style: "margin-bottom: 16px; padding: 12px; background: {th.bg_tertiary()}; border-radius: 6px; font-size: 12px; color: {th.text_secondary()};",
                    "Parametric sweep (.STEP) runs the simulation multiple times while varying a parameter, useful for sensitivity analysis and design optimization."
                }

                FormRow {
                    label: "Parameter Name",
                    help: "Component or model parameter to sweep",
                    FormInput {
                        value: param_name.read().clone(),
                        placeholder: "R1",
                        suffix: "",
                        onchange: move |v| param_name.set(v),
                    }
                }

                FormRow {
                    label: "Step Type",
                    help: "How to vary the parameter",
                    FormSelect {
                        value: current_type.display_name().to_string(),
                        options: step_options,
                        onchange: move |v: String| {
                            let st = match v.as_str() {
                                "Linear" => ParametricStepType::Linear,
                                "Decade (Log)" => ParametricStepType::Decade,
                                "Octave" => ParametricStepType::Octave,
                                "Value List" => ParametricStepType::List,
                                _ => ParametricStepType::Linear,
                            };
                            step_type.set(st);
                        },
                    }
                }

                // Step range (for non-list types)
                div {
                    style: "opacity: {range_opacity}; pointer-events: {range_pointer};",

                    FormRow {
                        label: "Start Value",
                        help: "Beginning of sweep range",
                        FormInput {
                            value: start_value.read().clone(),
                            placeholder: "1k",
                            suffix: "",
                            onchange: move |v| start_value.set(v),
                        }
                    }

                    FormRow {
                        label: "Stop Value",
                        help: "End of sweep range",
                        FormInput {
                            value: stop_value.read().clone(),
                            placeholder: "10k",
                            suffix: "",
                            onchange: move |v| stop_value.set(v),
                        }
                    }

                    FormRow {
                        label: step_label,
                        help: "Resolution of sweep",
                        FormInput {
                            value: num_steps.read().clone(),
                            placeholder: "10",
                            suffix: "",
                            onchange: move |v| num_steps.set(v),
                        }
                    }
                }

                // Value list (for list type)
                div {
                    style: "opacity: {list_opacity}; pointer-events: {list_pointer};",

                    FormRow {
                        label: "Value List",
                        help: "Space-separated parameter values",
                        FormInput {
                            value: values_list.read().clone(),
                            placeholder: "1k 2k 5k 10k",
                            suffix: "",
                            onchange: move |v| values_list.set(v),
                        }
                    }
                }

                // Preview of generated command
                div {
                    style: "margin-top: 16px; padding: 12px; background: {th.bg_tertiary()}; border-radius: 6px;",
                    div {
                        style: "font-size: 11px; color: {th.text_muted()}; margin-bottom: 4px;",
                        "Generated Command:"
                    }
                    div {
                        style: "font-family: 'JetBrains Mono', 'Fira Code', monospace; font-size: 12px; color: {th.accent_primary()};",
                        {
                            if is_list {
                                format!(".STEP PARAM {} LIST {}", param_name.read(), values_list.read())
                            } else {
                                format!(".STEP PARAM {} {} {} {} steps",
                                    param_name.read(),
                                    start_value.read(),
                                    stop_value.read(),
                                    num_steps.read()
                                )
                            }
                        }
                    }
                }
            }
        }
    }
}

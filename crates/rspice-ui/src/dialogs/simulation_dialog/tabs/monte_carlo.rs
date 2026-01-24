//! Monte Carlo Analysis Tab
//!
//! Configuration form for Monte Carlo statistical simulation.

use dioxus::prelude::*;

use crate::dialogs::simulation_dialog::form_components::{FormInput, FormRow};
use crate::state::simulation_command::McDistribution;
use crate::theme::Theme;

/// Monte Carlo analysis configuration tab
#[component]
pub fn MonteCarloTab(
    enabled: Signal<bool>,
    num_runs: Signal<String>,
    tolerance: Signal<String>,
    output_var: Signal<String>,
    distribution: Signal<McDistribution>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let is_enabled = *enabled.read();
    let opacity = if is_enabled { "1" } else { "0.5" };
    let pointer = if is_enabled { "auto" } else { "none" };
    let current_dist = *distribution.read();

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
                    "Enable Monte Carlo Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                FormRow {
                    label: "Number of Runs",
                    help: "Simulation iterations",
                    FormInput {
                        value: num_runs.read().clone(),
                        placeholder: "100",
                        suffix: "",
                        onchange: move |v| num_runs.set(v),
                    }
                }

                FormRow {
                    label: "Tolerance",
                    help: "Component variation range",
                    FormInput {
                        value: tolerance.read().clone(),
                        placeholder: "5",
                        suffix: "%",
                        onchange: move |v| tolerance.set(v),
                    }
                }

                FormRow {
                    label: "Distribution",
                    help: "Statistical distribution type",
                    select {
                        style: "padding: 8px 10px; background: {th.bg_primary()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 13px; width: 100%;",
                        value: if current_dist == McDistribution::Uniform { "uniform" } else { "gaussian" },
                        onchange: move |e: Event<FormData>| {
                            let val = e.value();
                            distribution.set(if val == "gaussian" { McDistribution::Gaussian } else { McDistribution::Uniform });
                        },
                        option { value: "uniform", "Uniform (±%)" }
                        option { value: "gaussian", "Gaussian (σ)" }
                    }
                }

                FormRow {
                    label: "Track Output",
                    help: "Variable to measure statistics",
                    FormInput {
                        value: output_var.read().clone(),
                        placeholder: "V(out)",
                        suffix: "",
                        onchange: move |v| output_var.set(v),
                    }
                }
            }
        }
    }
}

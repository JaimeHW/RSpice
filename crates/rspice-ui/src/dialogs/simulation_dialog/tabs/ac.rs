//! AC Analysis Tab
//!
//! Configuration form for frequency-domain AC simulation.

use dioxus::prelude::*;

use crate::dialogs::simulation_dialog::form_components::{FormInput, FormRow};
use crate::state::simulation_command::AcSweepType;
use crate::theme::Theme;

/// AC analysis configuration tab
#[component]
pub fn AcTab(
    enabled: Signal<bool>,
    start_freq: Signal<String>,
    stop_freq: Signal<String>,
    num_points: Signal<String>,
    sweep_type: Signal<AcSweepType>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let is_enabled = *enabled.read();
    let opacity = if is_enabled { "1" } else { "0.5" };
    let pointer = if is_enabled { "auto" } else { "none" };
    let current_sweep = *sweep_type.read();

    let points_help = match current_sweep {
        AcSweepType::Decade => "Points per decade",
        AcSweepType::Octave => "Points per octave",
        AcSweepType::Linear => "Total number of points",
    };

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
                    "Enable AC Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                FormRow {
                    label: "Start Frequency",
                    help: "Beginning of sweep",
                    FormInput {
                        value: start_freq.read().clone(),
                        placeholder: "1",
                        suffix: "Hz",
                        onchange: move |v| start_freq.set(v),
                    }
                }

                FormRow {
                    label: "Stop Frequency",
                    help: "End of sweep",
                    FormInput {
                        value: stop_freq.read().clone(),
                        placeholder: "1MEG",
                        suffix: "Hz",
                        onchange: move |v| stop_freq.set(v),
                    }
                }

                FormRow {
                    label: "Sweep Type",
                    help: "Frequency spacing method",
                    select {
                        style: "width: 100%; padding: 8px 10px; background: {th.bg_primary()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 13px;",
                        onchange: move |e: Event<FormData>| {
                            let val = e.value();
                            let st = match val.as_str() {
                                "Octave" => AcSweepType::Octave,
                                "Linear" => AcSweepType::Linear,
                                _ => AcSweepType::Decade,
                            };
                            sweep_type.set(st);
                        },
                        option { value: "Decade", selected: current_sweep == AcSweepType::Decade, "Decade" }
                        option { value: "Octave", selected: current_sweep == AcSweepType::Octave, "Octave" }
                        option { value: "Linear", selected: current_sweep == AcSweepType::Linear, "Linear" }
                    }
                }

                FormRow {
                    label: "Points",
                    help: points_help,
                    FormInput {
                        value: num_points.read().clone(),
                        placeholder: "10",
                        suffix: "",
                        onchange: move |v| num_points.set(v),
                    }
                }
            }
        }
    }
}

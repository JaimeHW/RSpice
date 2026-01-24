//! PAC (Periodic AC) Analysis Tab
//!
//! Configuration form for small-signal analysis around a periodic
//! operating point established by PSS.

use dioxus::prelude::*;

use crate::dialogs::simulation_dialog::form_components::{FormInput, FormRow, FormSelect};
use crate::state::simulation_command::AcSweepType;
use crate::theme::Theme;

/// PAC analysis configuration tab
#[component]
pub fn PacTab(
    enabled: Signal<bool>,
    start_freq: Signal<String>,
    stop_freq: Signal<String>,
    points_per_decade: Signal<String>,
    sweep_type: Signal<AcSweepType>,
    max_sidebands: Signal<String>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let is_enabled = *enabled.read();
    let opacity = if is_enabled { "1" } else { "0.5" };
    let pointer = if is_enabled { "auto" } else { "none" };
    let current_sweep = *sweep_type.read();

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
                    "Enable PAC Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                // Description / dependency note
                div {
                    style: "margin-bottom: 16px; padding: 12px; background: {th.bg_tertiary()}; border-radius: 6px; font-size: 12px; color: {th.text_secondary()};",
                    "PAC performs small-signal AC analysis around the periodic operating point from PSS. Useful for mixer conversion gain, phase noise sidebands, and RF circuit characterization."
                }

                // Warning if PSS not enabled
                div {
                    style: "margin-bottom: 16px; padding: 10px 12px; background: rgba(255, 193, 7, 0.1); border: 1px solid rgba(255, 193, 7, 0.3); border-radius: 6px; font-size: 12px; color: #ffc107;",
                    "⚠ Note: PAC requires a valid PSS analysis to run first."
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
                        placeholder: "1G",
                        suffix: "Hz",
                        onchange: move |v| stop_freq.set(v),
                    }
                }

                FormRow {
                    label: "Points/Decade",
                    help: "Frequency resolution",
                    FormInput {
                        value: points_per_decade.read().clone(),
                        placeholder: "10",
                        suffix: "",
                        onchange: move |v| points_per_decade.set(v),
                    }
                }

                FormRow {
                    label: "Sweep Type",
                    help: "Frequency variation method",
                    FormSelect {
                        value: current_sweep.display_name().to_string(),
                        options: AcSweepType::ALL.iter().map(|s| s.display_name().to_string()).collect(),
                        onchange: move |v: String| {
                            let st = AcSweepType::ALL.iter()
                                .find(|s| s.display_name() == v)
                                .copied()
                                .unwrap_or_default();
                            sweep_type.set(st);
                        },
                    }
                }

                FormRow {
                    label: "Max Sidebands",
                    help: "Number of sidebands to include",
                    FormInput {
                        value: max_sidebands.read().clone(),
                        placeholder: "3",
                        suffix: "",
                        onchange: move |v| max_sidebands.set(v),
                    }
                }
            }
        }
    }
}

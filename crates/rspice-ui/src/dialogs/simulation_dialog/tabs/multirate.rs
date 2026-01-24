//! Multi-Rate Analysis Tab
//!
//! Configuration form for multi-rate transient analysis that partitions
//! circuits into fast and slow sections for efficient mixed-signal simulation.

use dioxus::prelude::*;

use crate::dialogs::simulation_dialog::form_components::{FormInput, FormRow};
use crate::theme::Theme;

/// Multi-rate analysis configuration tab
#[component]
pub fn MultiRateTab(
    enabled: Signal<bool>,
    auto_partition: Signal<bool>,
    fast_rate: Signal<String>,
    slow_rate: Signal<String>,
    stop_time: Signal<String>,
    latency_tolerance: Signal<String>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let is_enabled = *enabled.read();
    let is_auto = *auto_partition.read();
    let opacity = if is_enabled { "1" } else { "0.5" };
    let pointer = if is_enabled { "auto" } else { "none" };
    let auto_opacity = if is_auto { "0.5" } else { "1" };
    let auto_pointer = if is_auto { "none" } else { "auto" };

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
                    "Enable Multi-Rate Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                // Description
                div {
                    style: "margin-bottom: 16px; padding: 12px; background: {th.bg_tertiary()}; border-radius: 6px; font-size: 12px; color: {th.text_secondary()};",
                    "Multi-rate analysis partitions the circuit into fast (carrier) and slow (baseband) sections, solving each at appropriate time steps for efficient mixed-signal simulation."
                }

                // Auto-partition checkbox
                label {
                    style: "display: flex; align-items: center; gap: 8px; margin-bottom: 16px; cursor: pointer;",
                    input {
                        r#type: "checkbox",
                        checked: is_auto,
                        onchange: move |e| auto_partition.set(e.checked()),
                    }
                    span {
                        style: "font-size: 13px; color: {th.text_secondary()};",
                        "Auto-detect circuit partitioning"
                    }
                }

                // Manual rate specification
                div {
                    style: "opacity: {auto_opacity}; pointer-events: {auto_pointer};",

                    FormRow {
                        label: "Fast Rate",
                        help: "Update rate for fast partition",
                        FormInput {
                            value: fast_rate.read().clone(),
                            placeholder: "1G",
                            suffix: "Hz",
                            onchange: move |v| fast_rate.set(v),
                        }
                    }

                    FormRow {
                        label: "Slow Rate",
                        help: "Update rate for slow partition",
                        FormInput {
                            value: slow_rate.read().clone(),
                            placeholder: "1MEG",
                            suffix: "Hz",
                            onchange: move |v| slow_rate.set(v),
                        }
                    }
                }

                FormRow {
                    label: "Stop Time",
                    help: "Simulation duration",
                    FormInput {
                        value: stop_time.read().clone(),
                        placeholder: "1m",
                        suffix: "s",
                        onchange: move |v| stop_time.set(v),
                    }
                }

                // Advanced options
                div {
                    style: "margin-top: 16px; padding-top: 16px; border-top: 1px solid {th.border()};",
                    div {
                        style: "font-size: 11px; color: {th.text_muted()}; margin-bottom: 12px; text-transform: uppercase; letter-spacing: 0.5px;",
                        "Interface Options"
                    }

                    FormRow {
                        label: "Latency Tolerance",
                        help: "Partition interface synchronization tolerance",
                        FormInput {
                            value: latency_tolerance.read().clone(),
                            placeholder: "1p",
                            suffix: "s",
                            onchange: move |v| latency_tolerance.set(v),
                        }
                    }
                }
            }
        }
    }
}

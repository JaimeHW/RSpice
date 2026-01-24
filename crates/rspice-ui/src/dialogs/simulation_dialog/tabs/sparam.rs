//! S-Parameter Analysis Tab
//!
//! Configuration form for RF/microwave S-parameter simulation.

use dioxus::prelude::*;

use crate::dialogs::simulation_dialog::form_components::{FormInput, FormRow};
use crate::theme::Theme;

/// S-Parameter analysis configuration tab
#[component]
pub fn SParamTab(
    enabled: Signal<bool>,
    port1_pos: Signal<String>,
    port1_neg: Signal<String>,
    port2_pos: Signal<String>,
    port2_neg: Signal<String>,
    z0: Signal<String>,
    start_freq: Signal<String>,
    stop_freq: Signal<String>,
    points: Signal<String>,
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
                    "Enable S-Parameter Analysis"
                }
            }

            div {
                style: "opacity: {opacity}; pointer-events: {pointer};",

                p {
                    style: "color: {th.text_muted()}; font-size: 12px; margin: 0 0 16px 0;",
                    "RF/microwave scattering parameter analysis for S11, S21, S12, S22."
                }

                div {
                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 8px; margin-bottom: 14px;",
                    div {
                        div { style: "font-size: 11px; color: {th.text_muted()}; margin-bottom: 4px;", "Port 1 (+)" }
                        input {
                            r#type: "text",
                            style: "width: 100%; padding: 6px 8px; background: {th.bg_primary()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 12px;",
                            value: "{port1_pos.read()}",
                            oninput: move |e| port1_pos.set(e.value()),
                        }
                    }
                    div {
                        div { style: "font-size: 11px; color: {th.text_muted()}; margin-bottom: 4px;", "Port 1 (−)" }
                        input {
                            r#type: "text",
                            style: "width: 100%; padding: 6px 8px; background: {th.bg_primary()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 12px;",
                            value: "{port1_neg.read()}",
                            oninput: move |e| port1_neg.set(e.value()),
                        }
                    }
                    div {
                        div { style: "font-size: 11px; color: {th.text_muted()}; margin-bottom: 4px;", "Port 2 (+)" }
                        input {
                            r#type: "text",
                            style: "width: 100%; padding: 6px 8px; background: {th.bg_primary()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 12px;",
                            value: "{port2_pos.read()}",
                            oninput: move |e| port2_pos.set(e.value()),
                        }
                    }
                    div {
                        div { style: "font-size: 11px; color: {th.text_muted()}; margin-bottom: 4px;", "Port 2 (−)" }
                        input {
                            r#type: "text",
                            style: "width: 100%; padding: 6px 8px; background: {th.bg_primary()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 12px;",
                            value: "{port2_neg.read()}",
                            oninput: move |e| port2_neg.set(e.value()),
                        }
                    }
                }

                FormRow {
                    label: "Reference Z₀",
                    help: "Characteristic impedance",
                    FormInput {
                        value: z0.read().clone(),
                        placeholder: "50",
                        suffix: "Ω",
                        onchange: move |v| z0.set(v),
                    }
                }

                FormRow {
                    label: "Start Frequency",
                    help: "Beginning of sweep",
                    FormInput {
                        value: start_freq.read().clone(),
                        placeholder: "1MEG",
                        suffix: "Hz",
                        onchange: move |v| start_freq.set(v),
                    }
                }

                FormRow {
                    label: "Stop Frequency",
                    help: "End of sweep",
                    FormInput {
                        value: stop_freq.read().clone(),
                        placeholder: "10G",
                        suffix: "Hz",
                        onchange: move |v| stop_freq.set(v),
                    }
                }

                FormRow {
                    label: "Points/Decade",
                    help: "Frequency resolution",
                    FormInput {
                        value: points.read().clone(),
                        placeholder: "20",
                        suffix: "",
                        onchange: move |v| points.set(v),
                    }
                }
            }
        }
    }
}

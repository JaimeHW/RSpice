//! Operating Point Analysis Tab
//!
//! Configuration form for DC operating point simulation.

use dioxus::prelude::*;

use crate::theme::Theme;

/// Operating point analysis configuration tab
#[component]
pub fn OpTab(enabled: Signal<bool>) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let is_enabled = *enabled.read();

    rsx! {
        div {
            label {
                style: "display: flex; align-items: center; gap: 8px; margin-bottom: 16px; cursor: pointer;",
                input {
                    r#type: "checkbox",
                    checked: is_enabled,
                    onchange: move |e| enabled.set(e.checked()),
                }
                span {
                    style: "font-weight: 500; color: {th.text_primary()};",
                    "Enable Operating Point Analysis"
                }
            }

            p {
                style: "color: {th.text_muted()}; font-size: 13px; line-height: 1.5; margin: 0;",
                "Operating point analysis computes the DC bias point of the circuit. "
                "This calculates all node voltages and branch currents with capacitors "
                "open and inductors shorted."
            }
        }
    }
}

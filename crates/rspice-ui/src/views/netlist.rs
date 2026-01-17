//! Netlist Editor View
//!
//! SPICE netlist text editor with syntax highlighting.

use dioxus::prelude::*;

use crate::theme::Theme;

/// Netlist text editor
#[component]
pub fn Netlist() -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let mut content = use_signal(|| DEFAULT_NETLIST.to_string());

    rsx! {
        div {
            class: "netlist-editor",
            style: "
                display: flex;
                flex-direction: column;
                width: 100%;
                height: 100%;
                background: {th.bg_secondary()};
            ",

            // Editor header with tabs
            div {
                style: "
                    display: flex;
                    align-items: center;
                    padding: {Theme::SPACING_XS} {Theme::SPACING_SM};
                    background: {th.bg_tertiary()};
                    border-bottom: 1px solid {th.border()};
                    gap: 2px;
                ",

                // Active tab
                div {
                    style: "
                        padding: {Theme::SPACING_XS} {Theme::SPACING_MD};
                        background: {th.bg_secondary()};
                        color: {th.text_primary()};
                        font-size: {Theme::FONT_SIZE_SM};
                        border-radius: {Theme::RADIUS_SM} {Theme::RADIUS_SM} 0 0;
                        border-bottom: 2px solid {th.accent_primary()};
                    ",
                    "circuit.cir"
                }

                // New tab button
                button {
                    style: "
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        width: 24px;
                        height: 24px;
                        background: transparent;
                        border: none;
                        color: {th.text_muted()};
                        font-size: 16px;
                        cursor: pointer;
                        border-radius: {Theme::RADIUS_SM};
                    ",
                    "+"
                }
            }

            // Editor content
            div {
                style: "
                    flex: 1;
                    display: flex;
                    overflow: hidden;
                ",

                // Line numbers
                div {
                    class: "line-numbers",
                    style: "
                        padding: {Theme::SPACING_MD};
                        padding-right: {Theme::SPACING_SM};
                        background: {th.bg_tertiary()};
                        color: {th.text_muted()};
                        font-family: {Theme::FONT_MONO};
                        font-size: {Theme::FONT_SIZE_SM};
                        line-height: 1.6;
                        text-align: right;
                        user-select: none;
                        border-right: 1px solid {th.border()};
                    ",
                    {
                        let line_count = content.read().lines().count().max(1);
                        (1..=line_count)
                            .map(|n| rsx! { div { "{n}" } })
                    }
                }

                // Text area
                textarea {
                    style: "
                        flex: 1;
                        padding: {Theme::SPACING_MD};
                        background: transparent;
                        color: {th.text_primary()};
                        font-family: {Theme::FONT_MONO};
                        font-size: {Theme::FONT_SIZE_SM};
                        line-height: 1.6;
                        border: none;
                        outline: none;
                        resize: none;
                        white-space: pre;
                        overflow: auto;
                    ",
                    value: "{content}",
                    oninput: move |e| content.set(e.value()),
                    spellcheck: "false",
                }
            }
        }
    }
}

/// Default netlist content for new files
const DEFAULT_NETLIST: &str = r#"* RSpice Circuit
* Example: RC Lowpass Filter

V1 in 0 DC 0 AC 1 SIN(0 1 1k)
R1 in out 1k
C1 out 0 1u

.TRAN 1u 5m
.AC DEC 10 1 100k
.END
"#;

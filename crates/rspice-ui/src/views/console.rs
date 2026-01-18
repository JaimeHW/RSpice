//! Console View
//!
//! Output console for simulation messages and logs.

use dioxus::prelude::*;

use crate::state::{MessageSeverity, SimulationState};
use crate::theme::Theme;

/// Console output panel
#[component]
pub fn Console() -> Element {
    let theme: Signal<Theme> = use_context();
    let mut sim_state: Signal<SimulationState> = use_context();
    let mut console_visible: Signal<crate::app::ConsoleVisible> = use_context();
    let th = theme.read();

    let messages = &sim_state.read().console_messages;

    rsx! {
        div {
            class: "console",
            style: "
                display: flex;
                flex-direction: column;
                height: 100%;
                background: {th.bg_tertiary()};
                border-top: 1px solid {th.border()};
            ",

            // Console header
            div {
                style: "
                    display: flex;
                    align-items: center;
                    padding: {Theme::SPACING_XS} {Theme::SPACING_MD};
                    background: {th.bg_secondary()};
                    border-bottom: 1px solid {th.border()};
                    font-size: {Theme::FONT_SIZE_SM};
                    font-weight: 600;
                    color: {th.text_secondary()};
                ",
                "Console"

                // Spacer
                div { style: "flex: 1;" }

                // Clear button
                button {
                    style: "
                        background: transparent;
                        border: none;
                        color: {th.text_muted()};
                        font-size: {Theme::FONT_SIZE_SM};
                        cursor: pointer;
                        padding: 0 8px;
                    ",
                    onclick: move |_| {
                        sim_state.write().clear_console();
                    },
                    "Clear"
                }

                // Close button
                button {
                    style: "
                        background: transparent;
                        border: none;
                        color: {th.text_muted()};
                        font-size: 16px;
                        cursor: pointer;
                        padding: 0 4px;
                        line-height: 1;
                    ",
                    onclick: move |_| {
                        console_visible.set(crate::app::ConsoleVisible(false));
                    },
                    "×"
                }
            }

            // Console content - uses column-reverse for natural bottom-pinning
            // Messages display oldest at top, newest at bottom, auto-scrolls to new messages
            div {
                id: "console-content",
                style: "
                    flex: 1;
                    overflow-y: auto;
                    display: flex;
                    flex-direction: column-reverse;
                    padding: {Theme::SPACING_SM};
                    font-family: {Theme::FONT_MONO};
                    font-size: {Theme::FONT_SIZE_SM};
                ",

                // Inner container reversed to correct display order
                div {
                    style: "display: flex; flex-direction: column;",

                    if messages.is_empty() {
                        div {
                            style: "color: {th.text_muted()};",
                            "Ready."
                        }
                    } else {
                        for msg in messages.iter() {
                            ConsoleMessageLine { message: msg.clone() }
                        }
                    }
                }
            }
        }
    }
}

/// Individual console message line
#[component]
fn ConsoleMessageLine(message: crate::state::ConsoleMessage) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let color = match message.severity {
        MessageSeverity::Info => th.text_secondary(),
        MessageSeverity::Warning => th.accent_warning(),
        MessageSeverity::Error => th.accent_error(),
        MessageSeverity::Success => th.accent_success(),
    };

    let prefix = match message.severity {
        MessageSeverity::Info => "ℹ",
        MessageSeverity::Warning => "⚠",
        MessageSeverity::Error => "✗",
        MessageSeverity::Success => "✓",
    };

    rsx! {
        div {
            style: "
                display: flex;
                gap: {Theme::SPACING_SM};
                color: {color};
                padding: 2px 0;
            ",
            span { style: "opacity: 0.8;", "{prefix}" }
            span { "{message.message}" }
        }
    }
}

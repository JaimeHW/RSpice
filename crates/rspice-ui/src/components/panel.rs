//! Panel Component
//!
//! Collapsible panel container for sidebars and content sections.

use dioxus::prelude::*;

use crate::theme::Theme;

/// Collapsible panel container
#[component]
pub fn Panel(
    /// Panel title
    title: String,
    /// Panel width (CSS value)
    #[props(default = "auto".to_string())]
    width: String,
    /// Position: "left" or "right" - controls border side
    #[props(default = "left".to_string())]
    position: String,
    /// Panel content
    children: Element,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let border = if position == "right" {
        format!("border-left: 1px solid {};", th.border())
    } else {
        format!("border-right: 1px solid {};", th.border())
    };

    rsx! {
        div {
            class: "panel",
            style: "
                display: flex;
                flex-direction: column;
                width: {width};
                min-width: {width};
                background: {th.bg_secondary()};
                {border}
            ",

            // Panel header
            div {
                class: "panel-header",
                style: "
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    height: 32px;
                    padding: 0 {Theme::SPACING_MD};
                    background: {th.bg_tertiary()};
                    border-bottom: 1px solid {th.border()};
                    font-size: {Theme::FONT_SIZE_SM};
                    font-weight: 600;
                    color: {th.text_secondary()};
                    text-transform: uppercase;
                    letter-spacing: 0.5px;
                ",
                "{title}"
            }

            // Panel content
            div {
                class: "panel-content",
                style: "
                    flex: 1;
                    overflow-y: auto;
                ",
                {children}
            }
        }
    }
}

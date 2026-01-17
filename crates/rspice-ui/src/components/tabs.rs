//! Tab Components
//!
//! Tabbed interface components for document switching.

use dioxus::prelude::*;

use crate::theme::Theme;

/// Tab bar container
#[component]
pub fn TabBar(
    /// Tab content (should be Tab components)
    children: Element,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        div {
            class: "tab-bar",
            style: "
                display: flex;
                align-items: center;
                background: {th.bg_tertiary()};
                border-bottom: 1px solid {th.border()};
                padding: 0 {Theme::SPACING_SM};
                gap: 2px;
                height: 36px;
            ",
            {children}
        }
    }
}

/// Individual tab
#[component]
pub fn Tab(
    /// Tab label
    label: String,
    /// Whether this tab is active
    #[props(default)]
    active: bool,
    /// Click handler
    #[props(default)]
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let mut is_hovered = use_signal(|| false);
    let hovered = *is_hovered.read();

    let bg = if active {
        th.bg_secondary()
    } else if hovered {
        th.surface()
    } else {
        "transparent"
    };

    let text_color = if active {
        th.text_primary()
    } else {
        th.text_secondary()
    };

    let border_bottom = if active {
        format!("2px solid {}", th.accent_primary())
    } else {
        "2px solid transparent".to_string()
    };

    rsx! {
        div {
            class: "tab",
            style: "
                display: flex;
                align-items: center;
                gap: {Theme::SPACING_XS};
                padding: {Theme::SPACING_XS} {Theme::SPACING_MD};
                background: {bg};
                color: {text_color};
                font-size: {Theme::FONT_SIZE_SM};
                border-bottom: {border_bottom};
                border-radius: {Theme::RADIUS_SM} {Theme::RADIUS_SM} 0 0;
                cursor: pointer;
                transition: background {Theme::TRANSITION_FAST}, color {Theme::TRANSITION_FAST};
                user-select: none;
            ",
            onmouseenter: move |_| is_hovered.set(true),
            onmouseleave: move |_| is_hovered.set(false),
            onclick: move |e| onclick.call(e),

            // Tab label
            span { "{label}" }
        }
    }
}

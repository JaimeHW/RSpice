//! Button Component
//!
//! Versatile button component with multiple variants and sizes.

use dioxus::prelude::*;

use crate::theme::Theme;

/// Button visual variant
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ButtonVariant {
    /// Primary action button with accent color
    Primary,
    /// Secondary button with subtle styling
    #[default]
    Secondary,
    /// Ghost button with transparent background
    Ghost,
    /// Success button (green)
    Success,
    /// Danger button (red)
    Danger,
}

/// Button size
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum ButtonSize {
    /// Small button
    Small,
    /// Medium button (default)
    #[default]
    Medium,
    /// Large button
    Large,
}

/// Reusable button component
#[component]
pub fn Button(
    /// Button content
    children: Element,
    /// Visual variant
    #[props(default)]
    variant: ButtonVariant,
    /// Size
    #[props(default)]
    size: ButtonSize,
    /// Whether the button is disabled
    #[props(default)]
    disabled: bool,
    /// Click handler
    #[props(default)]
    onclick: EventHandler<MouseEvent>,
    /// Optional icon (renders before text)
    #[props(default)]
    icon: Option<Element>,
    /// Tooltip text
    #[props(default)]
    title: Option<&'static str>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let mut is_hovered = use_signal(|| false);

    // Compute colors based on variant
    let (bg, bg_hover, text_color) = match variant {
        ButtonVariant::Primary => (
            th.accent_primary(),
            "#2563eb", // Slightly darker blue
            "#ffffff",
        ),
        ButtonVariant::Secondary => (th.surface(), th.surface_hover(), th.text_primary()),
        ButtonVariant::Ghost => ("transparent", th.surface(), th.text_primary()),
        ButtonVariant::Success => (th.accent_success(), "#16a34a", "#ffffff"),
        ButtonVariant::Danger => (th.accent_error(), "#dc2626", "#ffffff"),
    };

    // Compute size-based padding
    let (padding, font_size) = match size {
        ButtonSize::Small => ("4px 8px", Theme::FONT_SIZE_SM),
        ButtonSize::Medium => ("8px 16px", Theme::FONT_SIZE_BASE),
        ButtonSize::Large => ("12px 24px", Theme::FONT_SIZE_LG),
    };

    let current_bg = if *is_hovered.read() && !disabled {
        bg_hover
    } else {
        bg
    };

    let opacity = if disabled { "0.5" } else { "1" };
    let cursor = if disabled { "not-allowed" } else { "pointer" };

    rsx! {
        button {
            style: "
                display: inline-flex;
                align-items: center;
                justify-content: center;
                gap: {Theme::SPACING_XS};
                padding: {padding};
                background: {current_bg};
                color: {text_color};
                border: none;
                border-radius: {Theme::RADIUS_SM};
                font-family: {Theme::FONT_FAMILY};
                font-size: {font_size};
                font-weight: 500;
                cursor: {cursor};
                opacity: {opacity};
                transition: background {Theme::TRANSITION_FAST};
                outline: none;
                white-space: nowrap;
            ",
            title: title.unwrap_or(""),
            disabled: disabled,
            onmouseenter: move |_| is_hovered.set(true),
            onmouseleave: move |_| is_hovered.set(false),
            onclick: move |e| {
                if !disabled {
                    onclick.call(e);
                }
            },

            // Optional leading icon
            if let Some(icon_element) = icon {
                {icon_element}
            }

            {children}
        }
    }
}

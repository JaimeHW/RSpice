//! Waveform viewer control components.
//!
//! Header toolbar, control buttons, and legend items for the waveform viewer.

use dioxus::html::input_data::keyboard_types::Key;
use dioxus::prelude::*;

use crate::theme::Theme;

/// Waveform header with controls and expression input.
#[component]
pub fn WaveformHeader(
    on_fit: EventHandler<MouseEvent>,
    on_fit_x: EventHandler<MouseEvent>,
    on_fit_y: EventHandler<MouseEvent>,
    on_zoom_in: EventHandler<MouseEvent>,
    on_zoom_out: EventHandler<MouseEvent>,
    on_add_trace: EventHandler<String>,
    #[props(default)] on_toggle_measurements: EventHandler<MouseEvent>,
    #[props(default)] on_toggle_fft: EventHandler<MouseEvent>,
    #[props(default)] on_toggle_sweep: EventHandler<MouseEvent>,
    #[props(default)] on_toggle_export: EventHandler<MouseEvent>,
    #[props(default)] on_close: EventHandler<MouseEvent>,
    #[props(default)] measurements_active: bool,
    #[props(default)] fft_active: bool,
    #[props(default)] sweep_active: bool,
    #[props(default)] export_active: bool,
    /// Error message to display (set by parent when expression evaluation fails)
    #[props(default)]
    error_message: Option<String>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let mut expr_input = use_signal(|| String::new());

    // Use error from prop (parent-controlled) for display
    let display_error = error_message.clone();

    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                padding: {Theme::SPACING_XS} {Theme::SPACING_MD};
                background: {th.bg_tertiary()};
                border-bottom: 1px solid {th.border()};
                gap: {Theme::SPACING_MD};
            ",

            // Title
            span {
                style: "
                    font-size: {Theme::FONT_SIZE_SM};
                    font-weight: 600;
                    color: {th.text_secondary()};
                ",
                "Waveform Viewer"
            }

            // Expression input
            div {
                style: "
                    display: flex;
                    align-items: center;
                    gap: {Theme::SPACING_XS};
                    flex: 1;
                    max-width: 300px;
                ",

                input {
                    r#type: "text",
                    placeholder: "Add trace: V(out), I(R1)*1000, db(V(out))...",
                    value: "{expr_input}",
                    style: "
                        flex: 1;
                        padding: 4px 8px;
                        background: {th.bg_primary()};
                        border: 1px solid {th.border()};
                        border-radius: {Theme::RADIUS_SM};
                        color: {th.text_primary()};
                        font-size: 11px;
                        font-family: {Theme::FONT_MONO};
                        outline: none;
                    ",
                    oninput: move |e| {
                        expr_input.set(e.value().clone());
                    },
                    onkeydown: move |e| {
                        if e.key() == Key::Enter {
                            let expr = expr_input.read().clone();
                            if !expr.trim().is_empty() {
                                on_add_trace.call(expr.clone());
                                expr_input.set(String::new());
                            }
                        }
                    },
                }

                // Add button
                button {
                    style: "
                        padding: 4px 8px;
                        background: #3b82f6;
                        border: none;
                        border-radius: {Theme::RADIUS_SM};
                        color: white;
                        font-size: 11px;
                        cursor: pointer;
                    ",
                    onclick: move |_| {
                        let expr = expr_input.read().clone();
                        if !expr.trim().is_empty() {
                            on_add_trace.call(expr.clone());
                            expr_input.set(String::new());
                        }
                    },
                    "Add"
                }
            }

            // Error message
            if let Some(ref err) = display_error {
                span {
                    style: "
                        color: #ef4444;
                        font-size: 10px;
                        background: rgba(239, 68, 68, 0.1);
                        padding: 2px 6px;
                        border-radius: 3px;
                    ",
                    "⚠ {err}"
                }
            }

            // Spacer
            div { style: "flex: 1;" }

            // View controls
            div {
                style: "
                    display: flex;
                    gap: {Theme::SPACING_XS};
                ",

                ControlButton { label: "⊕", title: "Zoom In", onclick: on_zoom_in }
                ControlButton { label: "⊖", title: "Zoom Out", onclick: on_zoom_out }
                ControlButton { label: "⊡", title: "Fit All", onclick: on_fit }
                ControlButton { label: "↔", title: "Fit X (Time)", onclick: on_fit_x }
                ControlButton { label: "↕", title: "Fit Y (Voltage)", onclick: on_fit_y }
                ControlButton { label: "│", title: "Cursor 1" }
                ControlButton { label: "┃", title: "Cursor 2" }
            }

            // Analysis toggles
            div {
                style: "
                    display: flex;
                    gap: {Theme::SPACING_XS};
                    margin-left: {Theme::SPACING_SM};
                    padding-left: {Theme::SPACING_SM};
                    border-left: 1px solid {th.border()};
                ",

                ToggleButton {
                    label: "📊",
                    title: "Measurements",
                    active: measurements_active,
                    onclick: on_toggle_measurements,
                }
                ToggleButton {
                    label: "📈",
                    title: "FFT View",
                    active: fft_active,
                    onclick: on_toggle_fft,
                }
                ToggleButton {
                    label: "🔄",
                    title: "Sweep",
                    active: sweep_active,
                    onclick: on_toggle_sweep,
                }
                ToggleButton {
                    label: "💾",
                    title: "Export",
                    active: export_active,
                    onclick: on_toggle_export,
                }
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
                    margin-left: {Theme::SPACING_SM};
                ",
                title: "Close Waveform Viewer",
                onclick: move |e| on_close.call(e),
                "×"
            }
        }
    }
}

/// Small control button.
#[component]
pub fn ControlButton(
    label: &'static str,
    title: &'static str,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let mut hovered = use_signal(|| false);

    let bg = if *hovered.read() {
        th.surface_hover()
    } else {
        th.surface()
    };

    rsx! {
        button {
            title: "{title}",
            style: "
                width: 24px;
                height: 24px;
                display: flex;
                align-items: center;
                justify-content: center;
                background: {bg};
                border: none;
                border-radius: {Theme::RADIUS_SM};
                color: {th.text_primary()};
                font-size: 12px;
                cursor: pointer;
                transition: background {Theme::TRANSITION_FAST};
            ",
            onmouseenter: move |_| hovered.set(true),
            onmouseleave: move |_| hovered.set(false),
            onclick: move |e| onclick.call(e),
            "{label}"
        }
    }
}

/// Toggle button with active state indicator.
#[component]
pub fn ToggleButton(
    label: &'static str,
    title: &'static str,
    #[props(default)] active: bool,
    #[props(default)] onclick: EventHandler<MouseEvent>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let mut hovered = use_signal(|| false);

    let bg = if active {
        th.accent_primary()
    } else if *hovered.read() {
        th.surface_hover()
    } else {
        th.surface()
    };

    let text_color = if active { "#ffffff" } else { th.text_primary() };

    rsx! {
        button {
            title: "{title}",
            style: "
                height: 24px;
                padding: 0 8px;
                display: flex;
                align-items: center;
                justify-content: center;
                gap: 4px;
                background: {bg};
                border: none;
                border-radius: {Theme::RADIUS_SM};
                color: {text_color};
                font-size: 11px;
                cursor: pointer;
                transition: all {Theme::TRANSITION_FAST};
            ",
            onmouseenter: move |_| hovered.set(true),
            onmouseleave: move |_| hovered.set(false),
            onclick: move |e| onclick.call(e),
            "{label}"
        }
    }
}

/// Legend item with visibility toggle and cross-probe support.
#[component]
pub fn LegendItem(
    name: String,
    color: String,
    visible: bool,
    #[props(default)] highlighted: bool,
    on_toggle: EventHandler<MouseEvent>,
    #[props(default)] on_crossprobe: EventHandler<String>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let mut hovered = use_signal(|| false);
    let signal_name = name.clone();

    let opacity = if visible { "1" } else { "0.4" };
    let bg = if highlighted {
        th.accent_primary().to_string() + "30" // Highlighted background
    } else if *hovered.read() {
        th.surface_hover().to_string()
    } else {
        "transparent".to_string()
    };
    let border_style = if highlighted {
        format!("2px solid {}", th.accent_primary())
    } else {
        "2px solid transparent".to_string()
    };
    let checkbox_bg = if visible { "#3b82f6" } else { "transparent" };

    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                gap: {Theme::SPACING_XS};
                padding: 4px 6px;
                margin: 0 -6px;
                font-size: {Theme::FONT_SIZE_SM};
                cursor: pointer;
                border-radius: {Theme::RADIUS_SM};
                background: {bg};
                border: {border_style};
                opacity: {opacity};
                transition: all {Theme::TRANSITION_FAST};
            ",
            onmouseenter: move |_| hovered.set(true),
            onmouseleave: move |_| hovered.set(false),
            onclick: move |e| on_toggle.call(e),
            // Right-click for cross-probing to schematic
            oncontextmenu: {
                let name_for_probe = signal_name.clone();
                move |e: Event<MouseData>| {
                    e.prevent_default();
                    e.stop_propagation();
                    on_crossprobe.call(name_for_probe.clone());
                }
            },

            // Visibility checkbox
            div {
                style: "
                    width: 14px;
                    height: 14px;
                    border: 1px solid {th.border()};
                    border-radius: 3px;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    background: {checkbox_bg};
                    transition: background {Theme::TRANSITION_FAST};
                ",
                if visible {
                    span {
                        style: "color: white; font-size: 10px; font-weight: bold;",
                        "✓"
                    }
                }
            }

            // Color swatch
            div {
                style: "
                    width: 12px;
                    height: 3px;
                    background: {color};
                    border-radius: 1px;
                "
            }

            // Name
            span {
                style: "color: {th.text_primary()}; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;",
                "{name}"
            }
        }
    }
}

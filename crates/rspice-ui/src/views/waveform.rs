//! Waveform Viewer
//!
//! Canvas-based waveform plotting with zoom, pan, and cursor support.

use dioxus::prelude::*;

use crate::state::SimulationState;
use crate::theme::Theme;

/// Waveform viewer component
#[component]
pub fn Waveform() -> Element {
    let theme: Signal<Theme> = use_context();
    let sim_state: Signal<SimulationState> = use_context();
    let th = theme.read();

    let waveforms = &sim_state.read().waveforms;

    rsx! {
        div {
            class: "waveform-viewer",
            style: "
                display: flex;
                flex-direction: column;
                width: 100%;
                height: 100%;
                background: {th.bg_secondary()};
            ",

            // Waveform header with controls
            WaveformHeader {}

            // Main waveform area
            div {
                style: "
                    flex: 1;
                    display: flex;
                    overflow: hidden;
                ",

                // Y-axis labels
                div {
                    class: "y-axis",
                    style: "
                        width: 60px;
                        display: flex;
                        flex-direction: column;
                        justify-content: space-between;
                        padding: {Theme::SPACING_SM} {Theme::SPACING_XS};
                        font-family: {Theme::FONT_MONO};
                        font-size: 10px;
                        color: {th.text_muted()};
                        text-align: right;
                        border-right: 1px solid {th.border()};
                    ",
                    span { "1.0V" }
                    span { "0.5V" }
                    span { "0.0V" }
                    span { "-0.5V" }
                    span { "-1.0V" }
                }

                // Plot area
                div {
                    class: "plot-area",
                    style: "
                        flex: 1;
                        position: relative;
                        background: {th.bg_primary()};
                        overflow: hidden;
                    ",

                    // Grid lines
                    WaveformGrid {}

                    // Waveform traces
                    if waveforms.is_empty() {
                        // Placeholder with demo waveform
                        WaveformPlaceholder {}
                    } else {
                        for wf in waveforms.iter() {
                            WaveformTrace {
                                key: "{wf.name}",
                                x: wf.x.clone(),
                                y: wf.y.clone(),
                                color: wf.color.clone(),
                            }
                        }
                    }

                    // Cursor overlay (future)
                    div {
                        class: "cursor-overlay",
                        style: "
                            position: absolute;
                            inset: 0;
                            pointer-events: none;
                        "
                    }
                }

                // Legend
                div {
                    class: "legend",
                    style: "
                        width: 120px;
                        padding: {Theme::SPACING_SM};
                        border-left: 1px solid {th.border()};
                        overflow-y: auto;
                    ",

                    div {
                        style: "
                            font-size: {Theme::FONT_SIZE_SM};
                            font-weight: 600;
                            color: {th.text_secondary()};
                            margin-bottom: {Theme::SPACING_SM};
                        ",
                        "Traces"
                    }

                    if waveforms.is_empty() {
                        // Demo legend
                        LegendItem { name: "V(out)".to_string(), color: th.trace_color(0).to_string() }
                    } else {
                        for wf in waveforms.iter() {
                            LegendItem { name: wf.name.clone(), color: wf.color.clone() }
                        }
                    }
                }
            }

            // X-axis with time labels
            div {
                class: "x-axis",
                style: "
                    display: flex;
                    justify-content: space-between;
                    padding: {Theme::SPACING_XS} {Theme::SPACING_MD};
                    padding-left: 70px;
                    padding-right: 130px;
                    font-family: {Theme::FONT_MONO};
                    font-size: 10px;
                    color: {th.text_muted()};
                    border-top: 1px solid {th.border()};
                    background: {th.bg_tertiary()};
                ",
                span { "0ms" }
                span { "1ms" }
                span { "2ms" }
                span { "3ms" }
                span { "4ms" }
                span { "5ms" }
            }
        }
    }
}

/// Waveform header with controls
#[component]
fn WaveformHeader() -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

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

            // Spacer
            div { style: "flex: 1;" }

            // View controls
            div {
                style: "
                    display: flex;
                    gap: {Theme::SPACING_XS};
                ",

                ControlButton { label: "⊕", title: "Zoom In" }
                ControlButton { label: "⊖", title: "Zoom Out" }
                ControlButton { label: "⊡", title: "Fit" }
                ControlButton { label: "│", title: "Cursor 1" }
                ControlButton { label: "┃", title: "Cursor 2" }
            }
        }
    }
}

/// Small control button
#[component]
fn ControlButton(label: &'static str, title: &'static str) -> Element {
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
            "{label}"
        }
    }
}

/// Grid lines for the waveform plot
#[component]
fn WaveformGrid() -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        svg {
            style: "
                position: absolute;
                inset: 0;
                width: 100%;
                height: 100%;
            ",

            // Horizontal grid lines
            for i in 0..5 {
                line {
                    x1: "0",
                    y1: "{i as f64 * 25.0}%",
                    x2: "100%",
                    y2: "{i as f64 * 25.0}%",
                    stroke: "{th.border_subtle()}",
                    stroke_width: "1",
                }
            }

            // Vertical grid lines
            for i in 0..7 {
                line {
                    x1: "{i as f64 * 16.67}%",
                    y1: "0",
                    x2: "{i as f64 * 16.67}%",
                    y2: "100%",
                    stroke: "{th.border_subtle()}",
                    stroke_width: "1",
                }
            }

            // Center line (0V reference)
            line {
                x1: "0",
                y1: "50%",
                x2: "100%",
                y2: "50%",
                stroke: "{th.border()}",
                stroke_width: "1",
            }
        }
    }
}

/// Placeholder demo waveform
#[component]
fn WaveformPlaceholder() -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    // Generate a demo sine wave path
    let path = generate_sine_wave_path(100);

    rsx! {
        svg {
            style: "
                position: absolute;
                inset: 0;
                width: 100%;
                height: 100%;
            ",
            view_box: "0 0 100 100",
            preserve_aspect_ratio: "none",

            path {
                d: "{path}",
                fill: "none",
                stroke: "{th.trace_color(0)}",
                stroke_width: "1",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                vector_effect: "non-scaling-stroke",
            }
        }
    }
}

/// Generate SVG path for a sine wave (coordinates in 0-100 range)
fn generate_sine_wave_path(points: usize) -> String {
    let mut path = String::with_capacity(points * 20);

    for i in 0..points {
        let x = (i as f64 / points as f64) * 100.0;
        let t = (i as f64 / points as f64) * 4.0 * std::f64::consts::PI;
        let y = 50.0 - (t.sin() * 35.0);

        if i == 0 {
            path.push_str(&format!("M {:.2} {:.2}", x, y));
        } else {
            path.push_str(&format!(" L {:.2} {:.2}", x, y));
        }
    }

    path
}

/// Actual waveform trace renderer
#[component]
fn WaveformTrace(x: Vec<f64>, y: Vec<f64>, color: String) -> Element {
    // Generate SVG path from data points
    let path = if x.is_empty() || y.is_empty() {
        String::new()
    } else {
        let x_min = x.iter().copied().fold(f64::INFINITY, f64::min);
        let x_max = x.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        let y_min = y.iter().copied().fold(f64::INFINITY, f64::min);
        let y_max = y.iter().copied().fold(f64::NEG_INFINITY, f64::max);

        let x_range = (x_max - x_min).max(1e-12);
        let y_range = (y_max - y_min).max(1e-12);

        let mut path = String::new();
        for (i, (xi, yi)) in x.iter().zip(y.iter()).enumerate() {
            let px = ((xi - x_min) / x_range) * 100.0;
            let py = 100.0 - ((yi - y_min) / y_range) * 100.0;

            if i == 0 {
                path.push_str(&format!("M {:.2} {:.2}", px, py));
            } else {
                path.push_str(&format!(" L {:.2} {:.2}", px, py));
            }
        }
        path
    };

    rsx! {
        svg {
            style: "
                position: absolute;
                inset: 0;
                width: 100%;
                height: 100%;
            ",
            view_box: "0 0 100 100",
            preserve_aspect_ratio: "none",

            path {
                d: "{path}",
                fill: "none",
                stroke: "{color}",
                stroke_width: "1",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                vector_effect: "non-scaling-stroke",
            }
        }
    }
}

/// Legend item
#[component]
fn LegendItem(name: String, color: String) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                gap: {Theme::SPACING_XS};
                padding: 4px 0;
                font-size: {Theme::FONT_SIZE_SM};
            ",

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
                style: "color: {th.text_primary()};",
                "{name}"
            }
        }
    }
}

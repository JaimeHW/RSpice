//! Waveform Viewer
//!
//! Canvas-based waveform plotting with zoom, pan, and cursor support.

use dioxus::prelude::*;

use crate::state::SimulationState;
use crate::theme::Theme;

/// View state for zoom and pan
#[derive(Debug, Clone, Copy, PartialEq)]
struct ViewState {
    /// X-axis range (time or frequency)
    x_min: f64,
    x_max: f64,
    /// Y-axis range
    y_min: f64,
    y_max: f64,
    /// Whether we're currently panning
    is_panning: bool,
    /// Last mouse position during pan
    pan_start_x: f64,
    pan_start_y: f64,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            x_min: 0.0,
            x_max: 5e-3, // 5ms default
            y_min: -1.5,
            y_max: 1.5,
            is_panning: false,
            pan_start_x: 0.0,
            pan_start_y: 0.0,
        }
    }
}

impl ViewState {
    /// Zoom around a point (mouse position as fraction 0-1)
    fn zoom(&mut self, factor: f64, mouse_x_frac: f64, mouse_y_frac: f64) {
        let x_range = self.x_max - self.x_min;
        let y_range = self.y_max - self.y_min;

        // Calculate the point under the mouse in data coordinates
        let x_point = self.x_min + mouse_x_frac * x_range;
        let y_point = self.y_max - mouse_y_frac * y_range; // Y is inverted

        // Apply zoom factor
        let new_x_range = x_range * factor;
        let new_y_range = y_range * factor;

        // Reposition so the point stays under the mouse
        self.x_min = x_point - mouse_x_frac * new_x_range;
        self.x_max = x_point + (1.0 - mouse_x_frac) * new_x_range;
        self.y_min = y_point - (1.0 - mouse_y_frac) * new_y_range;
        self.y_max = y_point + mouse_y_frac * new_y_range;
    }

    /// Pan by delta (in data units)
    fn pan(&mut self, dx: f64, dy: f64) {
        self.x_min += dx;
        self.x_max += dx;
        self.y_min += dy;
        self.y_max += dy;
    }

    /// Fit to waveform data
    fn fit_to_data(&mut self, waveforms: &[crate::state::WaveformData]) {
        if waveforms.is_empty() {
            *self = Self::default();
            return;
        }

        let mut x_min = f64::INFINITY;
        let mut x_max = f64::NEG_INFINITY;
        let mut y_min = f64::INFINITY;
        let mut y_max = f64::NEG_INFINITY;

        for wf in waveforms {
            for &x in &wf.x {
                x_min = x_min.min(x);
                x_max = x_max.max(x);
            }
            for &y in &wf.y {
                y_min = y_min.min(y);
                y_max = y_max.max(y);
            }
        }

        // Add 10% margin
        let x_margin = (x_max - x_min) * 0.05;
        let y_margin = (y_max - y_min) * 0.1;

        self.x_min = x_min - x_margin;
        self.x_max = x_max + x_margin;
        self.y_min = y_min - y_margin;
        self.y_max = y_max + y_margin;
    }
}

/// Waveform viewer component
#[component]
pub fn Waveform() -> Element {
    let theme: Signal<Theme> = use_context();
    let sim_state: Signal<SimulationState> = use_context();
    let th = theme.read();

    let waveforms = &sim_state.read().waveforms;
    let mut view_state = use_signal(ViewState::default);

    // Auto-fit view when waveforms change
    let waveform_count = waveforms.len();
    use_effect(move || {
        let waveforms = &sim_state.read().waveforms;
        if !waveforms.is_empty() {
            view_state.write().fit_to_data(waveforms);
        }
    });

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
            WaveformHeader {
                on_fit: move |_| {
                    let waveforms = &sim_state.read().waveforms;
                    view_state.write().fit_to_data(waveforms);
                },
                on_zoom_in: move |_| {
                    view_state.write().zoom(0.8, 0.5, 0.5);
                },
                on_zoom_out: move |_| {
                    view_state.write().zoom(1.25, 0.5, 0.5);
                },
            }

            // Main waveform area
            div {
                style: "
                    flex: 1;
                    display: flex;
                    overflow: hidden;
                ",

                // Y-axis labels
                YAxisLabels { view: *view_state.read() }

                // Plot area with zoom/pan
                WaveformPlotArea {
                    view_state: view_state,
                    waveforms: waveforms.clone(),
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
            XAxisLabels { view: *view_state.read() }
        }
    }
}

/// Plot area with mouse interaction for zoom/pan
#[component]
fn WaveformPlotArea(
    view_state: Signal<ViewState>,
    waveforms: Vec<crate::state::WaveformData>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let view = *view_state.read();

    rsx! {
        div {
            class: "plot-area",
            style: "
                flex: 1;
                position: relative;
                background: {th.bg_primary()};
                overflow: hidden;
                cursor: grab;
            ",

            // Mouse wheel zoom
            onwheel: move |e| {
                let delta = e.delta();
                let delta_y = match delta {
                    dioxus::html::geometry::WheelDelta::Pixels(p) => p.y,
                    dioxus::html::geometry::WheelDelta::Lines(l) => l.y * 20.0,
                    dioxus::html::geometry::WheelDelta::Pages(p) => p.y * 100.0,
                };
                let factor = if delta_y > 0.0 { 1.15 } else { 0.87 };
                // Zoom at center for now (could use mouse position with element coords)
                view_state.write().zoom(factor, 0.5, 0.5);
            },

            // Pan with mouse drag
            onmousedown: move |e| {
                view_state.write().is_panning = true;
                view_state.write().pan_start_x = e.client_coordinates().x;
                view_state.write().pan_start_y = e.client_coordinates().y;
            },

            onmouseup: move |_| {
                view_state.write().is_panning = false;
            },

            onmouseleave: move |_| {
                view_state.write().is_panning = false;
            },

            onmousemove: move |e| {
                if view_state.read().is_panning {
                    let vs = *view_state.read();
                    let dx = (e.client_coordinates().x - vs.pan_start_x) / 500.0; // Scaled sensitivity
                    let dy = (e.client_coordinates().y - vs.pan_start_y) / 500.0;

                    let x_range = vs.x_max - vs.x_min;
                    let y_range = vs.y_max - vs.y_min;

                    let mut new_vs = view_state.write();
                    new_vs.pan(-dx * x_range, dy * y_range);
                    new_vs.pan_start_x = e.client_coordinates().x;
                    new_vs.pan_start_y = e.client_coordinates().y;
                }
            },

            // Grid lines
            WaveformGrid {}

            // Waveform traces
            if waveforms.is_empty() {
                WaveformPlaceholder {}
            } else {
                for wf in waveforms.iter() {
                    WaveformTraceView {
                        key: "{wf.name}",
                        x: wf.x.clone(),
                        y: wf.y.clone(),
                        color: wf.color.clone(),
                        view: view,
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
    }
}

/// Y-axis labels with actual values
#[component]
fn YAxisLabels(view: ViewState) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let labels = generate_axis_labels(view.y_min, view.y_max, 5);

    rsx! {
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
            for label in labels.iter().rev() {
                span { "{label}" }
            }
        }
    }
}

/// X-axis labels with actual values
#[component]
fn XAxisLabels(view: ViewState) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let labels = generate_time_labels(view.x_min, view.x_max, 6);

    rsx! {
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
            for label in labels.iter() {
                span { "{label}" }
            }
        }
    }
}

/// Generate axis labels
fn generate_axis_labels(min: f64, max: f64, count: usize) -> Vec<String> {
    let step = (max - min) / (count - 1) as f64;
    (0..count)
        .map(|i| {
            let val = min + i as f64 * step;
            format!("{:.2}V", val)
        })
        .collect()
}

/// Generate time labels with appropriate SI prefix
fn generate_time_labels(min: f64, max: f64, count: usize) -> Vec<String> {
    let range = max - min;
    let (scale, suffix) = if range < 1e-6 {
        (1e9, "ns")
    } else if range < 1e-3 {
        (1e6, "µs")
    } else if range < 1.0 {
        (1e3, "ms")
    } else {
        (1.0, "s")
    };

    let step = range / (count - 1) as f64;
    (0..count)
        .map(|i| {
            let val = (min + i as f64 * step) * scale;
            format!("{:.1}{}", val, suffix)
        })
        .collect()
}

/// Waveform header with controls
#[component]
fn WaveformHeader(
    on_fit: EventHandler<MouseEvent>,
    on_zoom_in: EventHandler<MouseEvent>,
    on_zoom_out: EventHandler<MouseEvent>,
) -> Element {
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

                ControlButton { label: "⊕", title: "Zoom In", onclick: on_zoom_in }
                ControlButton { label: "⊖", title: "Zoom Out", onclick: on_zoom_out }
                ControlButton { label: "⊡", title: "Fit", onclick: on_fit }
                ControlButton { label: "│", title: "Cursor 1" }
                ControlButton { label: "┃", title: "Cursor 2" }
            }
        }
    }
}

/// Small control button
#[component]
fn ControlButton(
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

/// Waveform trace with view transform applied
#[component]
fn WaveformTraceView(x: Vec<f64>, y: Vec<f64>, color: String, view: ViewState) -> Element {
    // Generate SVG path from data points, applying view transform
    let path = if x.is_empty() || y.is_empty() {
        String::new()
    } else {
        let x_range = (view.x_max - view.x_min).max(1e-12);
        let y_range = (view.y_max - view.y_min).max(1e-12);

        let mut path = String::new();
        for (i, (xi, yi)) in x.iter().zip(y.iter()).enumerate() {
            // Transform to view coordinates (0-100 range)
            let px = ((xi - view.x_min) / x_range) * 100.0;
            let py = 100.0 - ((yi - view.y_min) / y_range) * 100.0;

            // Skip points outside view
            if px < -10.0 || px > 110.0 {
                continue;
            }

            if path.is_empty() {
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

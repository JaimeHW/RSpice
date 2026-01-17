//! Measurements Panel Component
//!
//! Displays automated measurements for selected waveform traces.
//! Shows rise/fall time, frequency, RMS, and other metrics.

use dioxus::prelude::*;

use crate::state::SimulationState;
use crate::theme::Theme;

use super::measurement_calcs::{self, Measurement};

/// Props for the measurements panel
#[derive(Props, Clone, PartialEq)]
pub struct MeasurementsPanelProps {
    /// Whether the panel is visible
    pub visible: bool,
    /// Callback to close the panel
    pub on_close: EventHandler<()>,
    /// Optional region bounds (x_min, x_max) for bounded measurements
    #[props(default)]
    pub region: Option<(f64, f64)>,
    /// Panel position (managed by parent for global drag)
    #[props(default = (250, 50))]
    pub position: (i32, i32),
    /// Callback when drag starts (passes offset from panel position)
    #[props(default)]
    pub on_drag_start: EventHandler<(i32, i32)>,
}

/// Measurements panel showing waveform statistics
#[component]
pub fn MeasurementsPanel(props: MeasurementsPanelProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let sim_state: Signal<SimulationState> = use_context();
    let th = theme.read();

    if !props.visible {
        return rsx! {};
    }

    let (x, y) = props.position;
    let waveforms = &sim_state.read().waveforms;

    rsx! {
        div {
            class: "measurements-panel",
            style: "
                position: absolute;
                left: {x}px;
                top: {y}px;
                width: 280px;
                max-height: 400px;
                background: {th.bg_secondary()};
                border: 1px solid {th.border()};
                border-radius: {Theme::RADIUS_MD};
                box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
                z-index: 100;
                overflow: hidden;
                display: flex;
                flex-direction: column;
            ",

            // Draggable Header
            div {
                style: "
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: {Theme::SPACING_SM} {Theme::SPACING_MD};
                    border-bottom: 1px solid {th.border()};
                    background: {th.surface()};
                    cursor: move;
                    user-select: none;
                ",
                onmousedown: {
                    let position = props.position;
                    move |e| {
                        let offset_x = e.client_coordinates().x as i32 - position.0;
                        let offset_y = e.client_coordinates().y as i32 - position.1;
                        props.on_drag_start.call((offset_x, offset_y));
                    }
                },

                span {
                    style: "
                        font-weight: 600;
                        font-size: {Theme::FONT_SIZE_SM};
                        color: {th.text_primary()};
                    ",
                    "Measurements"
                }

                button {
                    style: "
                        background: none;
                        border: none;
                        color: {th.text_muted()};
                        cursor: pointer;
                        padding: 4px;
                        font-size: 14px;
                        line-height: 1;
                    ",
                    onmousedown: move |e| e.stop_propagation(),
                    onclick: move |_| props.on_close.call(()),
                    "✕"
                }
            }

            // Region indicator (if bounded)
            if let Some((x_min, x_max)) = props.region {
                div {
                    style: "
                        padding: {Theme::SPACING_XS} {Theme::SPACING_MD};
                        background: {th.accent_primary()}20;
                        font-size: 11px;
                        color: {th.text_secondary()};
                        border-bottom: 1px solid {th.border()};
                    ",
                    "Region: {format_time_compact(x_min)} → {format_time_compact(x_max)}"
                }
            }

            // Measurements content
            div {
                style: "
                    flex: 1;
                    overflow-y: auto;
                    padding: {Theme::SPACING_SM};
                ",

                if waveforms.is_empty() {
                    // Demo measurements for placeholder
                    {
                        let demo_x: Vec<f64> = (0..500)
                            .map(|i| i as f64 * 5e-3 / 499.0)
                            .collect();
                        let demo_y: Vec<f64> = demo_x
                            .iter()
                            .map(|t| (2.0 * std::f64::consts::PI * 1000.0 * t).sin())
                            .collect();

                        rsx! {
                            WaveformMeasurements {
                                name: "V(out) [demo]".to_string(),
                                color: th.trace_color(0).to_string(),
                                x: demo_x,
                                y: demo_y,
                                region: props.region,
                            }
                        }
                    }
                } else {
                    for (idx, wf) in waveforms.iter().filter(|w| w.visible).enumerate() {
                        WaveformMeasurements {
                            key: "{wf.name}-{idx}",
                            name: wf.name.clone(),
                            color: wf.color.clone(),
                            x: wf.x.clone(),
                            y: wf.y.clone(),
                            region: props.region,
                        }
                    }
                }
            }
        }
    }
}

/// Props for individual waveform measurements
#[derive(Props, Clone, PartialEq)]
struct WaveformMeasurementsProps {
    name: String,
    color: String,
    x: Vec<f64>,
    y: Vec<f64>,
    #[props(default)]
    region: Option<(f64, f64)>,
}

/// Display measurements for a single waveform
#[component]
fn WaveformMeasurements(props: WaveformMeasurementsProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    // Apply region bounds if specified
    let (x_data, y_data) = if let Some((x_min, x_max)) = props.region {
        extract_region(&props.x, &props.y, x_min, x_max)
    } else {
        (props.x.clone(), props.y.clone())
    };

    // Compute all measurements
    let measurements = measurement_calcs::all_measurements(&x_data, &y_data);

    rsx! {
        div {
            style: "
                margin-bottom: {Theme::SPACING_MD};
            ",

            // Trace name with color indicator
            div {
                style: "
                    display: flex;
                    align-items: center;
                    gap: {Theme::SPACING_XS};
                    margin-bottom: {Theme::SPACING_XS};
                    padding-bottom: {Theme::SPACING_XS};
                    border-bottom: 1px solid {th.border()};
                ",

                // Color dot
                div {
                    style: "
                        width: 10px;
                        height: 10px;
                        border-radius: 50%;
                        background: {props.color};
                    "
                }

                span {
                    style: "
                        font-size: {Theme::FONT_SIZE_SM};
                        font-weight: 500;
                        color: {th.text_primary()};
                        flex: 1;
                        overflow: hidden;
                        text-overflow: ellipsis;
                        white-space: nowrap;
                    ",
                    "{props.name}"
                }
            }

            // Measurement grid
            div {
                style: "
                    display: grid;
                    grid-template-columns: repeat(2, 1fr);
                    gap: 2px {Theme::SPACING_SM};
                ",

                for m in measurements.iter() {
                    MeasurementRow {
                        key: "{m.name}",
                        measurement: m.clone(),
                    }
                }
            }
        }
    }
}

/// Props for measurement row
#[derive(Props, Clone, PartialEq)]
struct MeasurementRowProps {
    measurement: Measurement,
}

/// Single measurement row
#[component]
fn MeasurementRow(props: MeasurementRowProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let m = &props.measurement;

    rsx! {
        div {
            style: "
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 2px 4px;
                border-radius: 2px;
                background: {th.surface()};
            ",

            span {
                style: "
                    font-size: 11px;
                    color: {th.text_muted()};
                    font-weight: 500;
                ",
                "{m.name}"
            }

            span {
                style: "
                    font-size: 11px;
                    color: {th.text_primary()};
                    font-family: monospace;
                ",
                "{m.formatted_value()}"
            }
        }
    }
}

/// Extract data within a specified x region
fn extract_region(x: &[f64], y: &[f64], x_min: f64, x_max: f64) -> (Vec<f64>, Vec<f64>) {
    let mut out_x = Vec::new();
    let mut out_y = Vec::new();

    for (i, &xi) in x.iter().enumerate() {
        if xi >= x_min && xi <= x_max {
            out_x.push(xi);
            out_y.push(y[i]);
        }
    }

    (out_x, out_y)
}

/// Compact time formatting for region display
fn format_time_compact(t: f64) -> String {
    let abs = t.abs();
    if abs >= 1.0 {
        format!("{:.2}s", t)
    } else if abs >= 1e-3 {
        format!("{:.2}ms", t * 1e3)
    } else if abs >= 1e-6 {
        format!("{:.2}µs", t * 1e6)
    } else {
        format!("{:.2}ns", t * 1e9)
    }
}

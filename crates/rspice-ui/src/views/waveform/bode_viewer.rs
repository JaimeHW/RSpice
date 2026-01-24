//! Bode Plot Viewer
//!
//! Professional-grade Bode plot component for AC and STB analysis visualization.
//! Features dual-axis display (magnitude dB + phase degrees), log frequency axis,
//! and interactive cursor with frequency/magnitude/phase readout.

use crate::services::{AcData, StbData};
use dioxus::prelude::*;

/// Bode plot display mode
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum BodePlotMode {
    #[default]
    MagnitudeAndPhase,
    MagnitudeOnly,
    PhaseOnly,
}

/// Bode plot data source
#[derive(Debug, Clone)]
pub enum BodePlotData {
    Ac(AcData),
    Stb(StbData),
}

/// Bode plot component properties
#[derive(Props, Clone, PartialEq)]
pub struct BodeViewerProps {
    /// AC or STB data to display
    pub data: Signal<Option<BodePlotData>>,
    /// Selected trace index (for AC with multiple nodes)
    #[props(default = 0)]
    pub selected_trace: usize,
    /// Display mode
    #[props(default)]
    pub mode: BodePlotMode,
    /// Width in pixels
    #[props(default = 800.0)]
    pub width: f64,
    /// Height in pixels
    #[props(default = 400.0)]
    pub height: f64,
}

/// Professional Bode plot viewer component
#[component]
pub fn BodeViewer(props: BodeViewerProps) -> Element {
    let cursor_pos = use_signal(|| None::<(f64, f64)>);
    let cursor_info = use_signal(|| String::new());

    // Extract plot data based on source type
    let plot_data = use_memo(move || {
        let data = props.data.read();
        match data.as_ref() {
            Some(BodePlotData::Ac(ac)) => {
                if props.selected_trace < ac.responses.len() {
                    let freqs = ac.frequencies.clone();
                    let mag_db = ac.magnitude_db(props.selected_trace);
                    let phase_deg = ac.phase_deg(props.selected_trace);
                    let name = ac.responses[props.selected_trace].0.clone();
                    Some((freqs, mag_db, phase_deg, name, None))
                } else {
                    None
                }
            }
            Some(BodePlotData::Stb(stb)) => Some((
                stb.frequencies.clone(),
                stb.loop_gain_db.clone(),
                stb.loop_phase_deg.clone(),
                "Loop Gain".to_string(),
                Some((stb.phase_margin, stb.gain_margin, stb.unity_gain_freq)),
            )),
            None => None,
        }
    });

    // Plot dimensions
    let margin = PlotMargins {
        left: 70.0,
        right: 70.0,
        top: 40.0,
        bottom: 50.0,
    };
    let plot_width = props.width - margin.left - margin.right;
    let plot_height = props.height - margin.top - margin.bottom;

    rsx! {
        div {
            class: "bode-viewer",
            style: "background: #1a1a2e; border-radius: 8px; padding: 16px; font-family: 'Inter', sans-serif;",

            // Title bar
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;",

                h3 {
                    style: "margin: 0; color: #e0e0e0; font-size: 14px; font-weight: 600;",
                    "Bode Plot"
                }

                // Cursor readout
                div {
                    style: "color: #a0a0a0; font-size: 12px; font-family: monospace;",
                    "{cursor_info}"
                }
            }

            // SVG Plot
            svg {
                width: "{props.width}",
                height: "{props.height}",
                style: "display: block;",

                // Background
                rect {
                    x: "0",
                    y: "0",
                    width: "{props.width}",
                    height: "{props.height}",
                    fill: "#0d0d1a",
                    rx: "4",
                }

                // Plot area background
                rect {
                    x: "{margin.left}",
                    y: "{margin.top}",
                    width: "{plot_width}",
                    height: "{plot_height}",
                    fill: "#131320",
                }

                // Render plot content
                if let Some((freqs, mag_db, phase_deg, name, margins)) = plot_data() {
                    // Calculate axis ranges
                    {
                        let (freq_min, freq_max) = calculate_range(&freqs, true);
                        let (mag_min, mag_max) = calculate_range(&mag_db, false);
                        let phase_range = (-180.0, 0.0);

                        rsx! {
                            // Grid lines
                            {render_grid(margin.left, margin.top, plot_width, plot_height)}

                            // Magnitude trace (left axis)
                            if props.mode != BodePlotMode::PhaseOnly {
                                {render_trace(
                                    &freqs, &mag_db,
                                    freq_min, freq_max, mag_min, mag_max,
                                    margin.left, margin.top, plot_width, plot_height,
                                    "#4fc3f7", true
                                )}
                            }

                            // Phase trace (right axis)
                            if props.mode != BodePlotMode::MagnitudeOnly {
                                {render_trace(
                                    &freqs, &phase_deg,
                                    freq_min, freq_max, phase_range.0, phase_range.1,
                                    margin.left, margin.top, plot_width, plot_height,
                                    "#ff9800", true
                                )}
                            }

                            // Stability margins for STB
                            if let Some((pm, gm, ugf)) = margins {
                                // Unity gain frequency marker
                                if ugf > 0.0 {
                                    {render_vertical_marker(
                                        ugf, freq_min, freq_max,
                                        margin.left, margin.top, plot_width, plot_height,
                                        "#4caf50"
                                    )}
                                }

                                // 0 dB reference line
                                {render_horizontal_line(
                                    0.0, mag_min, mag_max,
                                    margin.left, margin.top, plot_width, plot_height,
                                    "#666"
                                )}

                                // -180° reference line
                                {render_horizontal_line(
                                    -180.0, phase_range.0, phase_range.1,
                                    margin.left, margin.top, plot_width, plot_height,
                                    "#664400"
                                )}

                                // Margins annotation
                                text {
                                    x: "{margin.left + 10.0}",
                                    y: "{margin.top + 20.0}",
                                    fill: "#4caf50",
                                    font_size: "11",
                                    font_family: "monospace",
                                    "PM: {pm:.1}° | GM: {gm:.1} dB"
                                }
                            }

                            // Axis labels
                            // Left Y-axis (Magnitude)
                            text {
                                x: "15",
                                y: "{margin.top + plot_height / 2.0}",
                                fill: "#4fc3f7",
                                font_size: "11",
                                text_anchor: "middle",
                                transform: "rotate(-90, 15, {margin.top + plot_height / 2.0})",
                                "Magnitude (dB)"
                            }

                            // Right Y-axis (Phase)
                            text {
                                x: "{props.width - 15.0}",
                                y: "{margin.top + plot_height / 2.0}",
                                fill: "#ff9800",
                                font_size: "11",
                                text_anchor: "middle",
                                transform: "rotate(90, {props.width - 15.0}, {margin.top + plot_height / 2.0})",
                                "Phase (°)"
                            }

                            // X-axis (Frequency)
                            text {
                                x: "{margin.left + plot_width / 2.0}",
                                y: "{props.height - 10.0}",
                                fill: "#a0a0a0",
                                font_size: "11",
                                text_anchor: "middle",
                                "Frequency (Hz)"
                            }

                            // Trace label
                            text {
                                x: "{margin.left + plot_width - 10.0}",
                                y: "{margin.top + 15.0}",
                                fill: "#4fc3f7",
                                font_size: "10",
                                text_anchor: "end",
                                "{name}"
                            }
                        }
                    }
                } else {
                    // No data placeholder
                    text {
                        x: "{props.width / 2.0}",
                        y: "{props.height / 2.0}",
                        fill: "#666",
                        font_size: "14",
                        text_anchor: "middle",
                        "No AC/STB data available"
                    }
                }
            }
        }
    }
}

// =============================================================================
// Helper Structures
// =============================================================================

struct PlotMargins {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

// =============================================================================
// Rendering Helpers
// =============================================================================

fn calculate_range(data: &[f64], log_scale: bool) -> (f64, f64) {
    if data.is_empty() {
        return (1.0, 10.0);
    }

    let min = data.iter().copied().fold(f64::INFINITY, f64::min);
    let max = data.iter().copied().fold(f64::NEG_INFINITY, f64::max);

    if log_scale {
        let min = min.max(1e-15);
        let max = max.max(min * 10.0);
        (min, max)
    } else {
        let range = max - min;
        let padding = range * 0.1;
        (min - padding, max + padding)
    }
}

fn render_grid(x: f64, y: f64, width: f64, height: f64) -> Element {
    let grid_lines = 5;

    rsx! {
        g { class: "grid",
            // Horizontal grid lines
            for i in 0..=grid_lines {
                line {
                    x1: "{x}",
                    y1: "{y + (height * i as f64 / grid_lines as f64)}",
                    x2: "{x + width}",
                    y2: "{y + (height * i as f64 / grid_lines as f64)}",
                    stroke: "#2a2a40",
                    stroke_width: "1",
                }
            }
            // Vertical grid lines (log-spaced)
            for i in 0..=grid_lines {
                line {
                    x1: "{x + (width * i as f64 / grid_lines as f64)}",
                    y1: "{y}",
                    x2: "{x + (width * i as f64 / grid_lines as f64)}",
                    y2: "{y + height}",
                    stroke: "#2a2a40",
                    stroke_width: "1",
                }
            }
        }
    }
}

fn render_trace(
    x_data: &[f64],
    y_data: &[f64],
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    plot_x: f64,
    plot_y: f64,
    plot_width: f64,
    plot_height: f64,
    color: &str,
    log_x: bool,
) -> Element {
    if x_data.is_empty() || y_data.is_empty() {
        return rsx! {};
    }

    let log_x_min = if log_x { x_min.log10() } else { x_min };
    let log_x_max = if log_x { x_max.log10() } else { x_max };

    let mut path = String::new();
    for (i, (&x, &y)) in x_data.iter().zip(y_data.iter()).enumerate() {
        let x_norm = if log_x {
            (x.max(1e-15).log10() - log_x_min) / (log_x_max - log_x_min)
        } else {
            (x - x_min) / (x_max - x_min)
        };
        let y_norm = 1.0 - (y - y_min) / (y_max - y_min);

        let px = plot_x + x_norm * plot_width;
        let py = plot_y + y_norm * plot_height;

        if i == 0 {
            path.push_str(&format!("M{:.2},{:.2}", px, py));
        } else {
            path.push_str(&format!(" L{:.2},{:.2}", px, py));
        }
    }

    rsx! {
        path {
            d: "{path}",
            stroke: "{color}",
            stroke_width: "2",
            fill: "none",
            stroke_linecap: "round",
            stroke_linejoin: "round",
        }
    }
}

fn render_vertical_marker(
    x_val: f64,
    x_min: f64,
    x_max: f64,
    plot_x: f64,
    plot_y: f64,
    plot_width: f64,
    plot_height: f64,
    color: &str,
) -> Element {
    let log_x_min = x_min.log10();
    let log_x_max = x_max.log10();
    let x_norm = (x_val.log10() - log_x_min) / (log_x_max - log_x_min);
    let px = plot_x + x_norm * plot_width;

    rsx! {
        line {
            x1: "{px}",
            y1: "{plot_y}",
            x2: "{px}",
            y2: "{plot_y + plot_height}",
            stroke: "{color}",
            stroke_width: "1",
            stroke_dasharray: "4,2",
        }
    }
}

fn render_horizontal_line(
    y_val: f64,
    y_min: f64,
    y_max: f64,
    plot_x: f64,
    plot_y: f64,
    plot_width: f64,
    plot_height: f64,
    color: &str,
) -> Element {
    let y_norm = 1.0 - (y_val - y_min) / (y_max - y_min);
    let py = plot_y + y_norm * plot_height;

    rsx! {
        line {
            x1: "{plot_x}",
            y1: "{py}",
            x2: "{plot_x + plot_width}",
            y2: "{py}",
            stroke: "{color}",
            stroke_width: "1",
            stroke_dasharray: "4,2",
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_range_linear() {
        let data = vec![0.0, 50.0, 100.0];
        let (min, max) = calculate_range(&data, false);
        assert!(min < 0.0);
        assert!(max > 100.0);
    }

    #[test]
    fn test_calculate_range_log() {
        let data = vec![1.0, 100.0, 10000.0];
        let (min, max) = calculate_range(&data, true);
        assert!((min - 1.0).abs() < 1e-10);
        assert!((max - 10000.0).abs() < 1e-10);
    }

    #[test]
    fn test_calculate_range_empty() {
        let data: Vec<f64> = vec![];
        let (min, max) = calculate_range(&data, false);
        assert_eq!(min, 1.0);
        assert_eq!(max, 10.0);
    }
}

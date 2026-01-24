//! Noise Spectral Density Viewer
//!
//! Professional-grade noise analysis viewer with log-log spectral density plot,
//! device contribution breakdown, and integrated RMS noise display.

use crate::services::NoiseData;
use dioxus::prelude::*;

/// Noise viewer component properties
#[derive(Props, Clone, PartialEq)]
pub struct NoiseViewerProps {
    /// Noise analysis data
    pub data: Signal<Option<NoiseData>>,
    /// Width in pixels
    #[props(default = 800.0)]
    pub width: f64,
    /// Height in pixels
    #[props(default = 500.0)]
    pub height: f64,
}

/// Professional noise analysis viewer component
#[component]
pub fn NoiseViewer(props: NoiseViewerProps) -> Element {
    // Plot dimensions
    let margin = PlotMargins {
        left: 70.0,
        right: 200.0,
        top: 40.0,
        bottom: 50.0,
    };
    let plot_width = props.width - margin.left - margin.right;
    let plot_height = props.height - margin.top - margin.bottom;

    rsx! {
        div {
            class: "noise-viewer",
            style: "background: #1a1a2e; border-radius: 8px; padding: 16px; font-family: 'Inter', sans-serif;",

            // Title bar
            div {
                style: "display: flex; justify-content: space-between; align-items: center; margin-bottom: 12px;",

                h3 {
                    style: "margin: 0; color: #e0e0e0; font-size: 14px; font-weight: 600;",
                    "Noise Spectral Density"
                }

                // Integrated noise display
                if let Some(data) = props.data.read().as_ref() {
                    div {
                        style: "color: #4caf50; font-size: 12px; font-family: monospace; background: #1e3a2e; padding: 4px 8px; border-radius: 4px;",
                        "Total RMS: {format_voltage(data.total_output_noise)}"
                    }
                }
            }

            div {
                style: "display: flex; gap: 16px;",

                // SVG Plot
                svg {
                    width: "{props.width - margin.right}",
                    height: "{props.height}",
                    style: "display: block; flex-shrink: 0;",

                    // Background
                    rect {
                        x: "0",
                        y: "0",
                        width: "{props.width - margin.right}",
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
                    if let Some(data) = props.data.read().as_ref() {
                        // Calculate axis ranges (log-log for noise)
                        {
                            let freqs = &data.frequencies;
                            let noise = &data.output_noise;

                            let (freq_min, freq_max) = calculate_log_range(freqs);
                            let (noise_min, noise_max) = calculate_log_range(noise);

                            rsx! {
                                // Grid lines
                                {render_log_grid(margin.left, margin.top, plot_width, plot_height)}

                                // Noise trace
                                {render_log_trace(
                                    freqs, noise,
                                    freq_min, freq_max, noise_min, noise_max,
                                    margin.left, margin.top, plot_width, plot_height,
                                    "#e91e63"
                                )}

                                // Area fill under curve
                                {render_area_fill(
                                    freqs, noise,
                                    freq_min, freq_max, noise_min, noise_max,
                                    margin.left, margin.top, plot_width, plot_height,
                                    "#e91e63"
                                )}

                                // Y-axis label
                                text {
                                    x: "15",
                                    y: "{margin.top + plot_height / 2.0}",
                                    fill: "#e91e63",
                                    font_size: "11",
                                    text_anchor: "middle",
                                    transform: "rotate(-90, 15, {margin.top + plot_height / 2.0})",
                                    "Noise Density (V²/Hz)"
                                }

                                // X-axis label
                                text {
                                    x: "{margin.left + plot_width / 2.0}",
                                    y: "{props.height - 10.0}",
                                    fill: "#a0a0a0",
                                    font_size: "11",
                                    text_anchor: "middle",
                                    "Frequency (Hz)"
                                }

                                // Axis tick labels
                                {render_log_tick_labels(
                                    freq_min, freq_max, noise_min, noise_max,
                                    margin.left, margin.top, plot_width, plot_height
                                )}
                            }
                        }
                    } else {
                        // No data placeholder
                        text {
                            x: "{(props.width - margin.right) / 2.0}",
                            y: "{props.height / 2.0}",
                            fill: "#666",
                            font_size: "14",
                            text_anchor: "middle",
                            "No noise data available"
                        }
                    }
                }

                // Noise contributions panel
                if let Some(data) = props.data.read().as_ref() {
                    div {
                        style: "width: {margin.right - 16.0}px; background: #0d0d1a; border-radius: 4px; padding: 12px;",

                        h4 {
                            style: "margin: 0 0 12px 0; color: #e0e0e0; font-size: 12px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.5px;",
                            "Contributions"
                        }

                        div {
                            style: "display: flex; flex-direction: column; gap: 8px;",

                            for (device, percentage) in data.contributions.iter() {
                                {render_contribution_bar(device, *percentage)}
                            }

                            if data.contributions.is_empty() {
                                div {
                                    style: "color: #666; font-size: 11px; font-style: italic;",
                                    "No device contributions"
                                }
                            }
                        }
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
// Formatting Helpers
// =============================================================================

fn format_voltage(v: f64) -> String {
    if v.abs() >= 1.0 {
        format!("{:.3} V", v)
    } else if v.abs() >= 1e-3 {
        format!("{:.3} mV", v * 1e3)
    } else if v.abs() >= 1e-6 {
        format!("{:.3} µV", v * 1e6)
    } else if v.abs() >= 1e-9 {
        format!("{:.3} nV", v * 1e9)
    } else {
        format!("{:.3e} V", v)
    }
}

// =============================================================================
// Rendering Helpers
// =============================================================================

fn calculate_log_range(data: &[f64]) -> (f64, f64) {
    if data.is_empty() {
        return (1.0, 1e6);
    }

    let min = data
        .iter()
        .copied()
        .filter(|&x| x > 0.0)
        .fold(f64::INFINITY, f64::min)
        .max(1e-30);
    let max = data
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, f64::max)
        .max(min * 10.0);

    // Round to nearest decade
    let log_min = min.log10().floor();
    let log_max = max.log10().ceil();

    (10f64.powf(log_min), 10f64.powf(log_max))
}

fn render_log_grid(x: f64, y: f64, width: f64, height: f64) -> Element {
    let grid_lines = 4;

    rsx! {
        g { class: "grid",
            // Horizontal grid lines (log-spaced)
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
            // Vertical grid lines (log-spaced decades)
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

fn render_log_trace(
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
) -> Element {
    if x_data.is_empty() || y_data.is_empty() {
        return rsx! {};
    }

    let log_x_min = x_min.log10();
    let log_x_max = x_max.log10();
    let log_y_min = y_min.log10();
    let log_y_max = y_max.log10();

    let mut path = String::new();
    for (i, (&x, &y)) in x_data.iter().zip(y_data.iter()).enumerate() {
        if x <= 0.0 || y <= 0.0 {
            continue;
        }

        let x_norm = (x.log10() - log_x_min) / (log_x_max - log_x_min);
        let y_norm = 1.0 - (y.log10() - log_y_min) / (log_y_max - log_y_min);

        let px = plot_x + x_norm * plot_width;
        let py = plot_y + y_norm * plot_height;

        if path.is_empty() {
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

fn render_area_fill(
    x_data: &[f64],
    y_data: &[f64],
    x_min: f64,
    x_max: f64,
    y_min: f64,
    _y_max: f64,
    plot_x: f64,
    plot_y: f64,
    plot_width: f64,
    plot_height: f64,
    color: &str,
) -> Element {
    if x_data.is_empty() || y_data.is_empty() {
        return rsx! {};
    }

    let log_x_min = x_min.log10();
    let log_x_max = x_max.log10();
    let log_y_min = y_min.log10();
    let log_y_max = _y_max.log10();

    let mut path = String::new();
    let mut first_px = 0.0;
    let mut last_px = 0.0;

    for (i, (&x, &y)) in x_data.iter().zip(y_data.iter()).enumerate() {
        if x <= 0.0 || y <= 0.0 {
            continue;
        }

        let x_norm = (x.log10() - log_x_min) / (log_x_max - log_x_min);
        let y_norm = 1.0 - (y.log10() - log_y_min) / (log_y_max - log_y_min);

        let px = plot_x + x_norm * plot_width;
        let py = plot_y + y_norm * plot_height;

        if path.is_empty() {
            first_px = px;
            path.push_str(&format!("M{:.2},{:.2}", px, plot_y + plot_height));
            path.push_str(&format!(" L{:.2},{:.2}", px, py));
        } else {
            path.push_str(&format!(" L{:.2},{:.2}", px, py));
        }
        last_px = px;
    }

    // Close the path
    path.push_str(&format!(" L{:.2},{:.2}", last_px, plot_y + plot_height));
    path.push_str(&format!(" L{:.2},{:.2} Z", first_px, plot_y + plot_height));

    rsx! {
        path {
            d: "{path}",
            fill: "{color}",
            fill_opacity: "0.15",
            stroke: "none",
        }
    }
}

fn render_log_tick_labels(
    freq_min: f64,
    freq_max: f64,
    noise_min: f64,
    noise_max: f64,
    plot_x: f64,
    plot_y: f64,
    plot_width: f64,
    plot_height: f64,
) -> Element {
    let log_freq_min = freq_min.log10() as i32;
    let log_freq_max = freq_max.log10() as i32;
    let log_noise_min = noise_min.log10() as i32;
    let log_noise_max = noise_max.log10() as i32;

    rsx! {
        g { class: "tick-labels",
            // X-axis (frequency) tick labels
            for decade in log_freq_min..=log_freq_max {
                {
                    let x_norm = (decade as f64 - log_freq_min as f64) / (log_freq_max - log_freq_min) as f64;
                    let px = plot_x + x_norm * plot_width;
                    let label = format_decade(decade);
                    rsx! {
                        text {
                            x: "{px}",
                            y: "{plot_y + plot_height + 15.0}",
                            fill: "#808080",
                            font_size: "10",
                            text_anchor: "middle",
                            "{label}"
                        }
                    }
                }
            }

            // Y-axis (noise) tick labels
            for decade in log_noise_min..=log_noise_max {
                {
                    let y_norm = 1.0 - (decade as f64 - log_noise_min as f64) / (log_noise_max - log_noise_min) as f64;
                    let py = plot_y + y_norm * plot_height;
                    let label = format!("1e{}", decade);
                    rsx! {
                        text {
                            x: "{plot_x - 5.0}",
                            y: "{py + 3.0}",
                            fill: "#808080",
                            font_size: "9",
                            text_anchor: "end",
                            "{label}"
                        }
                    }
                }
            }
        }
    }
}

fn format_decade(exp: i32) -> String {
    match exp {
        0 => "1".to_string(),
        1 => "10".to_string(),
        2 => "100".to_string(),
        3 => "1k".to_string(),
        4 => "10k".to_string(),
        5 => "100k".to_string(),
        6 => "1M".to_string(),
        7 => "10M".to_string(),
        8 => "100M".to_string(),
        9 => "1G".to_string(),
        _ => format!("1e{}", exp),
    }
}

fn render_contribution_bar(device: &str, percentage: f64) -> Element {
    let bar_width = (percentage / 100.0).min(1.0) * 100.0;
    let color = if percentage > 50.0 {
        "#e91e63"
    } else if percentage > 20.0 {
        "#ff9800"
    } else {
        "#4caf50"
    };

    rsx! {
        div {
            style: "display: flex; flex-direction: column; gap: 2px;",

            div {
                style: "display: flex; justify-content: space-between; font-size: 10px;",
                span { style: "color: #a0a0a0;", "{device}" }
                span { style: "color: {color}; font-weight: 600;", "{percentage:.1}%" }
            }

            div {
                style: "height: 4px; background: #2a2a40; border-radius: 2px; overflow: hidden;",
                div {
                    style: "width: {bar_width}%; height: 100%; background: {color}; border-radius: 2px;",
                }
            }
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
    fn test_format_voltage() {
        assert!(format_voltage(1.5).contains("1.500 V"));
        assert!(format_voltage(0.001).contains("mV"));
        assert!(format_voltage(1e-6).contains("µV"));
        assert!(format_voltage(1e-9).contains("nV"));
    }

    #[test]
    fn test_calculate_log_range() {
        let data = vec![1e-12, 1e-10, 1e-8];
        let (min, max) = calculate_log_range(&data);
        assert!(min <= 1e-12);
        assert!(max >= 1e-8);
    }

    #[test]
    fn test_format_decade() {
        assert_eq!(format_decade(0), "1");
        assert_eq!(format_decade(3), "1k");
        assert_eq!(format_decade(6), "1M");
    }
}

//! Axis label components.
//!
//! Components for rendering X and Y axis labels with automatic
//! SI prefix formatting.

use dioxus::prelude::*;

use super::axis::{calculate_nice_step_fixed_divisions, time_scale_for_range};
use super::state::ViewState;
use crate::theme::Theme;

/// Y-axis labels with actual values - positioned at grid lines.
/// Uses adaptive division count based on container height for consistent visual density.
#[component]
pub fn YAxisLabels(view: ViewState) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    // Use fixed divisions like professional simulators (5 divisions for Y axis)
    let y_range = view.y_max - view.y_min;
    let y_step = calculate_nice_step_fixed_divisions(y_range, 5);

    // Generate labels at grid line positions
    let mut labels: Vec<(f64, String)> = Vec::new();
    let y_start = (view.y_min / y_step).floor() * y_step;
    let mut y = y_start;
    while y <= view.y_max + y_step * 0.01 {
        if y >= view.y_min - y_step * 0.01 {
            let pct = (1.0 - (y - view.y_min) / y_range) * 100.0;
            if pct >= -5.0 && pct <= 105.0 {
                labels.push((pct, format!("{:.2}V", y)));
            }
        }
        y += y_step;
    }

    rsx! {
        div {
            class: "y-axis",
            style: "
                width: 60px;
                position: relative;
                font-family: {Theme::FONT_MONO};
                font-size: 10px;
                color: {th.text_muted()};
                border-right: 1px solid {th.border()};
            ",
            for (pct, label) in labels.iter() {
                span {
                    style: "
                        position: absolute;
                        right: 4px;
                        top: {pct}%;
                        transform: translateY(-50%);
                        white-space: nowrap;
                    ",
                    "{label}"
                }
            }
        }
    }
}

/// X-axis labels with actual values - positioned at grid lines.
/// Uses adaptive division count based on container width for consistent visual density.
#[component]
pub fn XAxisLabels(view: ViewState) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    // Use fixed divisions like professional simulators (6 divisions for X axis)
    let x_range = view.x_max - view.x_min;
    let x_step = calculate_nice_step_fixed_divisions(x_range, 6);

    // Determine SI prefix for time display
    let (scale, suffix) = time_scale_for_range(x_range);

    // Generate labels at grid line positions
    let mut labels: Vec<(f64, String)> = Vec::new();
    let x_start = (view.x_min / x_step).floor() * x_step;
    let mut x = x_start;
    while x <= view.x_max + x_step * 0.01 {
        if x >= view.x_min - x_step * 0.01 {
            let pct = (x - view.x_min) / x_range * 100.0;
            if pct >= -5.0 && pct <= 105.0 {
                labels.push((pct, format!("{:.1}{}", x * scale, suffix)));
            }
        }
        x += x_step;
    }

    rsx! {
        div {
            class: "x-axis",
            style: "
                position: relative;
                height: 24px;
                margin-left: 60px;
                margin-right: 120px;
                font-family: {Theme::FONT_MONO};
                font-size: 10px;
                color: {th.text_muted()};
                border-top: 1px solid {th.border()};
                background: {th.bg_tertiary()};
            ",
            for (pct, label) in labels.iter() {
                span {
                    style: "
                        position: absolute;
                        left: {pct}%;
                        top: 4px;
                        transform: translateX(-50%);
                        white-space: nowrap;
                    ",
                    "{label}"
                }
            }
        }
    }
}

/// Grid lines for the waveform plot.
/// Uses SVG rendering for consistent cross-platform display.
/// Grid division count adapts to container size for consistent visual density.
#[component]
pub fn WaveformGrid(view: ViewState) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    // Calculate grid step sizes using fixed divisions (professional approach)
    let x_range = view.x_max - view.x_min;
    let y_range = view.y_max - view.y_min;
    let x_step = calculate_nice_step_fixed_divisions(x_range, 6);
    let y_step = calculate_nice_step_fixed_divisions(y_range, 5);

    // Generate vertical grid lines (X axis)
    let mut v_lines: Vec<f64> = Vec::new();
    let x_start = (view.x_min / x_step).floor() * x_step;
    let mut x = x_start;
    while x <= view.x_max + x_step * 0.01 {
        if x >= view.x_min - x_step * 0.01 {
            // Convert to percentage position
            let pct = (x - view.x_min) / x_range * 100.0;
            if pct >= 0.0 && pct <= 100.0 {
                v_lines.push(pct);
            }
        }
        x += x_step;
    }

    // Generate horizontal grid lines (Y axis)
    let mut h_lines: Vec<f64> = Vec::new();
    let y_start = (view.y_min / y_step).floor() * y_step;
    let mut y = y_start;
    while y <= view.y_max + y_step * 0.01 {
        if y >= view.y_min - y_step * 0.01 {
            // Convert to percentage position (Y is inverted - top is 0%)
            let pct = (1.0 - (y - view.y_min) / y_range) * 100.0;
            if pct >= 0.0 && pct <= 100.0 {
                h_lines.push(pct);
            }
        }
        y += y_step;
    }

    let grid_color = th.border();

    rsx! {
        svg {
            class: "waveform-grid",
            style: "position: absolute; inset: 0; width: 100%; height: 100%; pointer-events: none;",

            // Vertical grid lines
            for pct in v_lines.iter() {
                line {
                    x1: "{pct}%",
                    y1: "0%",
                    x2: "{pct}%",
                    y2: "100%",
                    stroke: "{grid_color}",
                    stroke_width: "1",
                    stroke_opacity: "0.3",
                }
            }

            // Horizontal grid lines
            for pct in h_lines.iter() {
                line {
                    x1: "0%",
                    y1: "{pct}%",
                    x2: "100%",
                    y2: "{pct}%",
                    stroke: "{grid_color}",
                    stroke_width: "1",
                    stroke_opacity: "0.3",
                }
            }
        }
    }
}

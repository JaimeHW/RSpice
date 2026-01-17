//! Axis label components.
//!
//! Components for rendering X and Y axis labels with automatic
//! SI prefix formatting.

use dioxus::prelude::*;

use super::axis::{calculate_nice_grid_step, time_scale_for_range};
use super::state::ViewState;
use crate::theme::Theme;

/// Y-axis labels with actual values - positioned at grid lines.
#[component]
pub fn YAxisLabels(view: ViewState) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    // Use same nice step calculation as GPU gridlines
    let y_range = view.y_max - view.y_min;
    let y_step = calculate_nice_grid_step(y_range);

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
#[component]
pub fn XAxisLabels(view: ViewState) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    // Use same nice step calculation as GPU gridlines
    let x_range = view.x_max - view.x_min;
    let x_step = calculate_nice_grid_step(x_range);

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
/// Note: Gridlines are now rendered by the GPU waveform renderer.
#[component]
pub fn WaveformGrid() -> Element {
    // Gridlines are rendered by the GPU, this component is kept for compatibility
    rsx! {}
}

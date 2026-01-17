//! Cursor rendering components.
//!
//! Components for rendering cursor lines and the cursor readout overlay.

use dioxus::prelude::*;

use super::interpolation::get_cursor_values;
use super::state::{CursorState, ViewState};
use crate::state::WaveformData;
use crate::theme::Theme;
use crate::utils::formatting::{format_frequency, format_time, format_voltage};

/// Cursor lines overlay (SVG).
#[component]
pub fn CursorLines(cursors: CursorState, view: ViewState) -> Element {
    rsx! {
        svg {
            style: "
                position: absolute;
                inset: 0;
                width: 100%;
                height: 100%;
                pointer-events: none;
            ",

            // Cursor 1 (yellow)
            if let Some(c1) = cursors.cursor1 {
                {
                    let x_frac = (c1 - view.x_min) / (view.x_max - view.x_min);
                    let x_pct = format!("{}%", x_frac * 100.0);
                    rsx! {
                        line {
                            x1: "{x_pct}",
                            y1: "0",
                            x2: "{x_pct}",
                            y2: "100%",
                            stroke: "#eab308",
                            stroke_width: "1",
                            stroke_dasharray: "4",
                        }
                    }
                }
            }

            // Cursor 2 (cyan)
            if let Some(c2) = cursors.cursor2 {
                {
                    let x_frac = (c2 - view.x_min) / (view.x_max - view.x_min);
                    let x_pct = format!("{}%", x_frac * 100.0);
                    rsx! {
                        line {
                            x1: "{x_pct}",
                            y1: "0",
                            x2: "{x_pct}",
                            y2: "100%",
                            stroke: "#06b6d4",
                            stroke_width: "1",
                            stroke_dasharray: "4",
                        }
                    }
                }
            }
        }
    }
}

/// Cursor readout overlay with per-trace values.
#[component]
pub fn CursorReadout(cursors: CursorState, waveforms: Vec<WaveformData>) -> Element {
    // Compute cursor values for each cursor
    let c1_values = cursors.cursor1.map(|t| get_cursor_values(t, &waveforms));
    let c2_values = cursors.cursor2.map(|t| get_cursor_values(t, &waveforms));

    rsx! {
        div {
            style: "
                position: absolute;
                top: 8px;
                right: 8px;
                background: rgba(0, 0, 0, 0.9);
                padding: 10px 14px;
                border-radius: 8px;
                font-family: {Theme::FONT_MONO};
                font-size: 11px;
                min-width: 180px;
                max-width: 280px;
                box-shadow: 0 4px 12px rgba(0,0,0,0.4);
                border: 1px solid rgba(255,255,255,0.1);
                z-index: 30;
            ",

            // Title
            div {
                style: "color: #9ca3af; font-size: 10px; margin-bottom: 8px; text-transform: uppercase; letter-spacing: 0.5px; border-bottom: 1px solid rgba(255,255,255,0.1); padding-bottom: 4px;",
                "Cursor Readout"
            }

            // Cursor 1 section
            if let Some(ref vals) = c1_values {
                div {
                    style: "margin-bottom: 8px;",
                    div {
                        style: "color: #eab308; font-weight: 600; margin-bottom: 4px; display: flex; justify-content: space-between;",
                        span { "C1:" }
                        span { "{format_time(vals.x)}" }
                    }
                    for tv in vals.traces.iter() {
                        div {
                            style: "display: flex; justify-content: space-between; padding: 1px 0; color: {tv.color};",
                            span { style: "opacity: 0.8;", "{tv.name}:" }
                            span { style: "font-weight: 500;", "{format_voltage(tv.value)}" }
                        }
                    }
                }
            }

            // Cursor 2 section
            if let Some(ref vals) = c2_values {
                div {
                    style: "margin-bottom: 8px;",
                    div {
                        style: "color: #06b6d4; font-weight: 600; margin-bottom: 4px; display: flex; justify-content: space-between;",
                        span { "C2:" }
                        span { "{format_time(vals.x)}" }
                    }
                    for tv in vals.traces.iter() {
                        div {
                            style: "display: flex; justify-content: space-between; padding: 1px 0; color: {tv.color};",
                            span { style: "opacity: 0.8;", "{tv.name}:" }
                            span { style: "font-weight: 500;", "{format_voltage(tv.value)}" }
                        }
                    }
                }
            }

            // Delta and frequency
            if let Some(delta) = cursors.delta() {
                div {
                    style: "border-top: 1px solid rgba(255,255,255,0.1); margin-top: 4px; padding-top: 6px;",
                    div {
                        style: "color: #22c55e; margin-bottom: 2px; display: flex; justify-content: space-between;",
                        span { "Δt:" }
                        span { style: "font-weight: 500;", "{format_time(delta)}" }
                    }
                    if let Some(freq) = cursors.frequency() {
                        div {
                            style: "color: #a78bfa; display: flex; justify-content: space-between;",
                            span { "Freq:" }
                            span { style: "font-weight: 500;", "{format_frequency(freq)}" }
                        }
                    }
                }
            }
        }
    }
}

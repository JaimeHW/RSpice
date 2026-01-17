//! Waveform canvas components.
//!
//! Components for rendering waveform traces using SVG and GPU acceleration.

use dioxus::prelude::*;
use std::sync::{Arc, Mutex};

use super::state::ViewState;
use crate::state::WaveformData;
use crate::theme::Theme;
use crate::utils::formatting::parse_hex_color;
use crate::views::waveform_gpu::{
    is_gpu_available, WaveformGpuState, WaveformPainter, WaveformTrace,
};

/// GPU-accelerated waveform canvas component.
#[component]
pub fn GpuWaveformCanvas(view: ViewState, waveforms: Vec<WaveformData>) -> Element {
    // Track container size for dynamic resolution
    let mut container_size = use_signal(|| (1200u32, 400u32));

    // Build traces from waveform data
    let traces: Vec<WaveformTrace> = if waveforms.is_empty() {
        // Demo waveform - generate X values covering current view
        let n = 1000;
        let x_range = view.x_max - view.x_min;
        let step = x_range / (n - 1) as f64;
        let x: Vec<f64> = (0..n).map(|i| view.x_min + i as f64 * step).collect();
        let y: Vec<f64> = x
            .iter()
            .map(|t| (2.0 * std::f64::consts::PI * 1000.0 * t).sin())
            .collect();
        vec![WaveformTrace {
            x,
            y,
            color: [0.133, 0.773, 0.369, 1.0], // #22c55e
            name: "V(out)".to_string(),
        }]
    } else {
        waveforms
            .iter()
            .filter(|wf| wf.visible)
            .map(|wf| {
                let color = parse_hex_color(&wf.color);
                WaveformTrace {
                    x: wf.x.clone(),
                    y: wf.y.clone(),
                    color,
                    name: wf.name.clone(),
                }
            })
            .collect()
    };

    // Create GPU state and render
    let gpu_state = Arc::new(Mutex::new(WaveformGpuState {
        traces,
        x_min: view.x_min,
        x_max: view.x_max,
        y_min: view.y_min,
        y_max: view.y_max,
        dirty: true,
    }));

    let (width, height) = *container_size.read();
    // Render at 2x resolution for high-DPI displays, with minimum size
    let render_width = (width * 2).max(800);
    let render_height = (height * 2).max(300);

    let mut painter = WaveformPainter::new(gpu_state);
    let img_src = painter
        .render_to_base64(render_width, render_height)
        .unwrap_or_default();

    rsx! {
        div {
            style: "position: absolute; inset: 0;",
            onmounted: move |evt| {
                // Measure container size when mounted
                spawn(async move {
                    if let Ok(rect) = evt.get_client_rect().await {
                        let w = rect.width() as u32;
                        let h = rect.height() as u32;
                        if w > 0 && h > 0 {
                            container_size.set((w, h));
                        }
                    }
                });
            },
            if !img_src.is_empty() {
                img {
                    style: "
                        position: absolute;
                        inset: 0;
                        width: 100%;
                        height: 100%;
                        object-fit: fill;
                    ",
                    src: "{img_src}",
                }
            }
        }
    }
}

/// Waveform trace with view transform applied (SVG fallback).
#[component]
pub fn WaveformTraceView(x: Vec<f64>, y: Vec<f64>, color: String, view: ViewState) -> Element {
    // Generate SVG path from data points, applying view transform
    let path = if x.is_empty() || y.is_empty() {
        String::new()
    } else {
        let x_range = (view.x_max - view.x_min).max(1e-12);
        let y_range = (view.y_max - view.y_min).max(1e-12);

        let mut path = String::new();
        for (xi, yi) in x.iter().zip(y.iter()) {
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

/// Placeholder demo waveform.
#[component]
pub fn WaveformPlaceholder() -> Element {
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

/// Generate SVG path for a sine wave (coordinates in 0-100 range).
pub fn generate_sine_wave_path(points: usize) -> String {
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

//! Waveform canvas components.
//!
//! Components for rendering waveform traces using SVG and GPU acceleration.

use dioxus::prelude::*;
use std::sync::{Arc, Mutex};

use super::state::ViewState;
use crate::state::digital_waveform::{DigitalSignal, DigitalWaveformConfig, LogicState};
use crate::state::WaveformData;
use crate::theme::Theme;
use crate::utils::formatting::parse_hex_color;
use crate::views::waveform_gpu::{
    is_gpu_available, WaveformGpuState, WaveformPainter, WaveformTrace,
};

/// GPU-accelerated waveform canvas component.
/// On desktop: uses synchronous rendering.
/// On web (WASM): uses async rendering to avoid blocking the browser.
///
/// Accepts `view_state` as a Signal so the component reacts to view changes.
#[component]
pub fn GpuWaveformCanvas(view_state: Signal<ViewState>, waveforms: Vec<WaveformData>) -> Element {
    // Read view from signal - this creates a reactive dependency
    let view = *view_state.read();

    // Track container size for dynamic resolution
    let mut container_size = use_signal(|| (1200u32, 400u32));

    // Signal to store the rendered image (updated asynchronously on web)
    let mut rendered_image = use_signal(String::new);

    // Memoize the key inputs to detect when we need to re-render
    // Use the incoming view prop directly - this is the current state we want to render
    let visible_trace_names: String = waveforms
        .iter()
        .filter(|w| w.visible)
        .map(|w| w.name.as_str())
        .collect::<Vec<_>>()
        .join(",");

    let (width, height) = *container_size.read();
    let render_width = (width * 2).max(800);
    let render_height = (height * 2).max(300);

    // Use fresh view prop for render key - ensures effect triggers on view changes
    let _render_key = format!(
        "{:.6}_{:.6}_{:.6}_{:.6}_{}_{}x{}",
        view.x_min,
        view.x_max,
        view.y_min,
        view.y_max,
        visible_trace_names,
        render_width,
        render_height,
    );

    // Build traces from waveform data
    let build_traces = |view: &ViewState, waveforms: &[WaveformData]| -> Vec<WaveformTrace> {
        if waveforms.is_empty() {
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
        }
    };

    // Track the last render key to avoid redundant renders
    let _last_render_key = use_signal(String::new);

    // Platform-specific rendering
    #[cfg(target_arch = "wasm32")]
    {
        // Track render generation and timing
        let mut render_generation = use_signal(|| 0u64);
        let mut is_rendering = use_signal(|| false);

        // Spawn async render directly on component render (not via use_effect)
        // This ensures we react to every render caused by signal changes
        let key = _render_key.clone();
        let last_key = _last_render_key.read().clone();

        // Throttle: only start new render if not already rendering and key changed
        if last_key != key && !*is_rendering.read() {
            log::info!("[GPU Canvas] new render key detected, spawning GPU render");
            _last_render_key.set(key.clone());

            // Increment generation
            let current_gen = *render_generation.read() + 1;
            render_generation.set(current_gen);
            is_rendering.set(true);

            // Capture values for async block
            let view = view;
            let waveforms = waveforms.clone();

            spawn(async move {
                // Debounce: wait for pan/zoom to settle before GPU render
                gloo_timers::future::TimeoutFuture::new(100).await;

                // Check if this render is still current
                if *render_generation.read() != current_gen {
                    is_rendering.set(false);
                    return; // Stale, skip
                }

                log::info!("[GPU Canvas] starting GPU render gen={}", current_gen);
                let traces = build_traces(&view, &waveforms);

                let gpu_state = Arc::new(Mutex::new(WaveformGpuState {
                    traces,
                    x_min: view.x_min,
                    x_max: view.x_max,
                    y_min: view.y_min,
                    y_max: view.y_max,
                    dirty: true,
                }));

                let mut painter = WaveformPainter::new(gpu_state);
                if let Some(img) = painter
                    .render_to_base64_async(render_width, render_height)
                    .await
                {
                    log::info!("[GPU Canvas] render succeeded gen={}", current_gen);
                    rendered_image.set(img);
                } else {
                    log::warn!("[GPU Canvas] render failed gen={}", current_gen);
                }

                // Allow next render to start
                is_rendering.set(false);
            });
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        // Synchronous rendering for desktop
        let traces = build_traces(&view, &waveforms);

        let gpu_state = Arc::new(Mutex::new(WaveformGpuState {
            traces,
            x_min: view.x_min,
            x_max: view.x_max,
            y_min: view.y_min,
            y_max: view.y_max,
            dirty: true,
        }));

        let mut painter = WaveformPainter::new(gpu_state);
        if let Some(img) = painter.render_to_base64(render_width, render_height) {
            rendered_image.set(img);
        }
    }

    let img_src = rendered_image.read().clone();

    // Simple static display - waveforms update after pan/zoom settles (debounced 100ms)
    rsx! {
        div {
            style: "position: absolute; inset: 0;",
            onmounted: move |evt| {
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
                    style: "position: absolute; inset: 0; width: 100%; height: 100%; object-fit: fill;",
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

/// Digital waveform trace with step-style rendering.
/// Automatically detects digital signals and renders with:
/// - Sharp vertical transitions (no interpolation)
/// - Logic level coloring (green=high, red=low, gray=unknown)
/// - Bus value annotations for multi-bit buses
#[component]
pub fn DigitalWaveformTraceView(
    x: Vec<f64>,
    y: Vec<f64>,
    color: String,
    view: ViewState,
) -> Element {
    // Auto-detect if this is a digital signal (CMOS 3.3V thresholds)
    let config = DigitalWaveformConfig::cmos_3v3();
    let signal = DigitalSignal::from_analog("signal", &x, &y, &config);
    let segments = signal.render_segments();

    // Generate step-style SVG path with vertical transitions
    let path = if x.is_empty() || y.is_empty() {
        String::new()
    } else {
        let x_range = (view.x_max - view.x_min).max(1e-12);
        let y_range = (view.y_max - view.y_min).max(1e-12);

        let mut path = String::new();
        let mut last_px = None;
        let mut last_py = None;

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
            } else if let (Some(_lpx), Some(_lpy)) = (last_px, last_py) {
                // Step-style: first horizontal, then vertical transition
                path.push_str(&format!(" H {:.2}", px));
                path.push_str(&format!(" V {:.2}", py));
            } else {
                path.push_str(&format!(" L {:.2} {:.2}", px, py));
            }

            last_px = Some(px);
            last_py = Some(py);
        }
        path
    };

    // Color based on logic level (use theme accent for high, muted for low)
    let stroke_color = if segments.is_empty() {
        color.clone()
    } else {
        // Use first segment's level to determine color
        match segments[0].2 {
            LogicState::High => "#22c55e".to_string(), // Green for HIGH
            LogicState::Low => "#ef4444".to_string(),  // Red for LOW
            LogicState::Unknown => "#6b7280".to_string(), // Gray for unknown
            _ => color.clone(),
        }
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
                stroke: "{stroke_color}",
                stroke_width: "2",
                stroke_linecap: "square",  // Square caps for digital waveforms
                stroke_linejoin: "miter",  // Sharp corners for vertical transitions
                vector_effect: "non-scaling-stroke",
            }
        }
    }
}

/// Check if a waveform appears to be a digital signal.
/// Uses simple heuristics: mostly at extreme values, fast transitions.
pub fn is_digital_signal(y: &[f64], config: &DigitalWaveformConfig) -> bool {
    if y.is_empty() {
        return false;
    }

    // Count how many samples are at logical HIGH or LOW
    let mut digital_count = 0;
    for val in y {
        if *val >= config.v_high || *val <= config.v_low {
            digital_count += 1;
        }
    }

    // If >80% of samples are at logic levels, it's digital
    (digital_count as f64 / y.len() as f64) > 0.8
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

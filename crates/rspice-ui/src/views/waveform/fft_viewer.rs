//! FFT Viewer Component
//!
//! Displays frequency-domain analysis of waveform data using FFT.
//! Provides toggle between time-domain and frequency-domain views.

use dioxus::prelude::*;

use crate::state::fft::{compute_fft, WindowFunction};
use crate::state::SimulationState;
use crate::theme::Theme;

/// Props for FFT viewer panel
#[derive(Props, Clone, PartialEq)]
pub struct FftViewerProps {
    /// Whether the viewer is visible
    pub visible: bool,
    /// Callback to close the viewer
    pub on_close: EventHandler<()>,
    /// Panel position (managed by parent for global drag)
    #[props(default = (80, 50))]
    pub position: (i32, i32),
    /// Callback when drag starts
    #[props(default)]
    pub on_drag_start: EventHandler<(i32, i32)>,
}

/// FFT Viewer panel showing frequency-domain analysis
#[component]
pub fn FftViewer(props: FftViewerProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let sim_state: Signal<SimulationState> = use_context();
    let th = theme.read();

    let mut window_fn = use_signal(|| WindowFunction::Hanning);
    let mut log_scale = use_signal(|| true);
    let mut db_scale = use_signal(|| true);

    if !props.visible {
        return rsx! {};
    }

    // Memoize FFT computation - only recompute when waveforms or window function changes
    // This prevents expensive FFT recalculation during drag operations
    let window = *window_fn.read();
    let fft_result = use_memo(move || {
        let waveforms = &sim_state.read().waveforms;

        if waveforms.is_empty() {
            // Demo sine wave FFT
            let n = 1024;
            let sample_rate = 10000.0;
            let freq = 1000.0;
            let time: Vec<f64> = (0..n).map(|i| i as f64 / sample_rate).collect();
            let values: Vec<f64> = time
                .iter()
                .map(|t| (2.0 * std::f64::consts::PI * freq * t).sin())
                .collect();
            compute_fft(&time, &values, window)
        } else {
            let wf = waveforms.iter().find(|w| w.visible).or(waveforms.first());
            if let Some(wf) = wf {
                compute_fft(&wf.x, &wf.y, window)
            } else {
                None
            }
        }
    });

    let (x, y) = props.position;

    rsx! {
        div {
            class: "fft-viewer",
            style: "
                position: absolute;
                left: {x}px;
                top: {y}px;
                width: 500px;
                height: 350px;
                background: {th.bg_primary()};
                border: 1px solid {th.border()};
                border-radius: {Theme::RADIUS_MD};
                box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
                z-index: 95;
                display: flex;
                flex-direction: column;
                overflow: hidden;
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
                    "FFT Spectrum"
                }

                // Controls
                div {
                    style: "display: flex; gap: {Theme::SPACING_MD}; align-items: center;",

                    // Window function selector
                    WindowSelector {
                        value: *window_fn.read(),
                        on_change: move |w| window_fn.set(w),
                    }

                    // Log scale toggle
                    label {
                        style: "
                            display: flex;
                            align-items: center;
                            gap: 4px;
                            font-size: 11px;
                            color: {th.text_secondary()};
                            cursor: pointer;
                        ",
                        input {
                            r#type: "checkbox",
                            checked: *log_scale.read(),
                            onchange: move |e| log_scale.set(e.checked()),
                        }
                        "Log freq"
                    }

                    // dB scale toggle
                    label {
                        style: "
                            display: flex;
                            align-items: center;
                            gap: 4px;
                            font-size: 11px;
                            color: {th.text_secondary()};
                            cursor: pointer;
                        ",
                        input {
                            r#type: "checkbox",
                            checked: *db_scale.read(),
                            onchange: move |e| db_scale.set(e.checked()),
                        }
                        "dB scale"
                    }
                }

                button {
                    style: "
                        background: none;
                        border: none;
                        color: {th.text_muted()};
                        cursor: pointer;
                        padding: 4px;
                        font-size: 14px;
                    ",
                    onmousedown: move |e| e.stop_propagation(),
                    onclick: move |_| props.on_close.call(()),
                    "✕"
                }
            }

            // FFT Plot area
            div {
                style: "
                    flex: 1;
                    padding: {Theme::SPACING_MD};
                    position: relative;
                ",

                if let Some(result) = fft_result.read().as_ref() {
                    FftPlot {
                        frequencies: result.frequencies.clone(),
                        magnitudes: result.magnitudes.clone(),
                        log_freq: *log_scale.read(),
                        db_mag: *db_scale.read(),
                    }
                } else {
                    div {
                        style: "
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            height: 100%;
                            color: {th.text_muted()};
                            font-size: {Theme::FONT_SIZE_SM};
                        ",
                        "No data available for FFT"
                    }
                }
            }

            // Peak frequency readout
            if let Some(result) = fft_result.read().as_ref() {
                if let Some((peak_freq, peak_mag)) = result.peak_frequency() {
                    div {
                        style: "
                            padding: {Theme::SPACING_XS} {Theme::SPACING_MD};
                            border-top: 1px solid {th.border()};
                            background: {th.surface()};
                            font-size: 11px;
                            display: flex;
                            gap: {Theme::SPACING_LG};
                        ",

                        span {
                            style: "color: {th.text_secondary()};",
                            "Peak: "
                            span {
                                style: "color: {th.accent_primary()}; font-weight: 600;",
                                "{format_frequency(peak_freq)}"
                            }
                        }

                        span {
                            style: "color: {th.text_secondary()};",
                            "Magnitude: "
                            span {
                                style: "color: {th.text_primary()}; font-family: monospace;",
                                if *db_scale.read() {
                                    "{format_db(peak_mag)}"
                                } else {
                                    "{peak_mag:.4}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Window function selector dropdown
#[component]
fn WindowSelector(value: WindowFunction, on_change: EventHandler<WindowFunction>) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        select {
            style: "
                padding: 2px 8px;
                background: {th.bg_primary()};
                border: 1px solid {th.border()};
                border-radius: {Theme::RADIUS_SM};
                color: {th.text_primary()};
                font-size: 11px;
                cursor: pointer;
            ",
            value: "{value:?}",
            onchange: move |e| {
                let window = match e.value().as_str() {
                    "Rectangular" => WindowFunction::Rectangular,
                    "Hanning" => WindowFunction::Hanning,
                    "Hamming" => WindowFunction::Hamming,
                    "Blackman" => WindowFunction::Blackman,
                    _ => WindowFunction::Hanning,
                };
                on_change.call(window);
            },

            option { value: "Rectangular", "Rectangular" }
            option { value: "Hanning", selected: matches!(value, WindowFunction::Hanning), "Hanning" }
            option { value: "Hamming", "Hamming" }
            option { value: "Blackman", "Blackman" }
        }
    }
}

/// FFT plot using SVG
#[component]
fn FftPlot(frequencies: Vec<f64>, magnitudes: Vec<f64>, log_freq: bool, db_mag: bool) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    // Filter out DC and very low frequencies for log scale
    let start_idx = if log_freq { 1 } else { 0 };
    let freqs = if start_idx < frequencies.len() {
        &frequencies[start_idx..]
    } else {
        &frequencies[..]
    };
    let mags = if db_mag {
        magnitudes[start_idx..]
            .iter()
            .map(|m| if *m > 1e-15 { 20.0 * m.log10() } else { -300.0 })
            .collect::<Vec<_>>()
    } else {
        magnitudes[start_idx..].to_vec()
    };

    if freqs.is_empty() || mags.is_empty() {
        return rsx! {};
    }

    // Calculate bounds
    let f_min = if log_freq { freqs[0].max(1.0) } else { 0.0 };
    let f_max = *freqs.last().unwrap_or(&1.0);
    let m_min = if db_mag { -100.0 } else { 0.0 };
    let m_max = mags
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max)
        .max(m_min + 1.0);

    // Generate SVG path
    let width = 100.0; // percentage
    let height = 100.0;

    let points: Vec<String> = freqs
        .iter()
        .zip(mags.iter())
        .filter(|(f, _)| **f > 0.0)
        .map(|(f, m)| {
            let x = if log_freq {
                (f.log10() - f_min.log10()) / (f_max.log10() - f_min.log10()) * width
            } else {
                (f - f_min) / (f_max - f_min) * width
            };
            let y = height - ((m - m_min) / (m_max - m_min) * height);
            format!("{:.2},{:.2}", x.max(0.0).min(width), y.max(0.0).min(height))
        })
        .collect();

    if points.is_empty() {
        return rsx! {};
    }

    let path = format!("M {} L {}", points[0], points.join(" L "));

    rsx! {
        svg {
            width: "100%",
            height: "100%",
            view_box: "0 0 100 100",
            preserve_aspect_ratio: "none",
            style: "background: {th.bg_secondary()}; border-radius: {Theme::RADIUS_SM};",

            // Grid lines
            for i in 0..5 {
                line {
                    x1: "0",
                    y1: "{i as f64 * 25.0}",
                    x2: "100",
                    y2: "{i as f64 * 25.0}",
                    stroke: "{th.border()}",
                    stroke_width: "0.2",
                }
            }
            for i in 0..5 {
                line {
                    x1: "{i as f64 * 25.0}",
                    y1: "0",
                    x2: "{i as f64 * 25.0}",
                    y2: "100",
                    stroke: "{th.border()}",
                    stroke_width: "0.2",
                }
            }

            // FFT trace
            path {
                d: "{path}",
                fill: "none",
                stroke: "{th.trace_color(0)}",
                stroke_width: "0.5",
                vector_effect: "non-scaling-stroke",
            }
        }
    }
}

/// Format frequency with SI prefix
fn format_frequency(f: f64) -> String {
    if f >= 1e9 {
        format!("{:.2} GHz", f / 1e9)
    } else if f >= 1e6 {
        format!("{:.2} MHz", f / 1e6)
    } else if f >= 1e3 {
        format!("{:.2} kHz", f / 1e3)
    } else {
        format!("{:.2} Hz", f)
    }
}

/// Format magnitude in dB
fn format_db(m: f64) -> String {
    if m > 1e-15 {
        format!("{:.1} dB", 20.0 * m.log10())
    } else {
        "-∞ dB".to_string()
    }
}

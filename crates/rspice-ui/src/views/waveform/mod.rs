//! Waveform Viewer Module
//!
//! Canvas-based waveform plotting with zoom, pan, and cursor support.
//! Supports GPU-accelerated rendering when available.

pub mod axis;
mod axis_labels;
mod canvas;
mod controls;
mod cursor;
mod export_panel;
mod fft_viewer;
mod global_drag;
mod interpolation;
mod measurement_calcs;
mod measurements;
mod measurements_panel;
mod state;
mod sweep_panel;

// Re-export types that are used externally
pub use self::state::{BoxSelection, CursorState, ViewState, WaveformViewerState};

// Internal imports for components used in this file
use axis_labels::{WaveformGrid, XAxisLabels, YAxisLabels};
use canvas::{GpuWaveformCanvas, WaveformTraceView};
use controls::{LegendItem, WaveformHeader};
use cursor::{CursorLines, CursorReadout};
use export_panel::ExportPanel;
use fft_viewer::FftViewer;
use measurements_panel::MeasurementsPanel;
use sweep_panel::SweepPanel;

use dioxus::prelude::*;

use crate::state::waveform_math;
use crate::state::SimulationState;
use crate::theme::Theme;
use crate::views::waveform_gpu::is_gpu_available;

/// Waveform viewer component.
#[component]
pub fn Waveform() -> Element {
    let theme: Signal<Theme> = use_context();
    let mut sim_state: Signal<SimulationState> = use_context();
    let mut waveform_visible: Signal<crate::app::WaveformVisible> = use_context();
    let th = theme.read();

    let waveforms = &sim_state.read().waveforms;
    let mut view_state = use_signal(ViewState::default);
    let cursor_state = use_signal(CursorState::default);
    let box_selection = use_signal(BoxSelection::default);
    let _viewer_state = use_signal(WaveformViewerState::new);

    // UI state for analysis panels
    let mut show_measurements = use_signal(|| false);
    let mut show_fft = use_signal(|| false);
    let mut show_sweep = use_signal(|| false);
    let mut show_export = use_signal(|| false);

    // Global drag state for smooth panel dragging
    let mut global_drag = use_signal(global_drag::GlobalDragState::default);

    // Panel positions (managed at parent level for global mouse capture)
    let mut measurements_pos = use_signal(|| (250i32, 50i32));
    let mut fft_pos = use_signal(|| (80i32, 50i32));
    let mut sweep_pos = use_signal(|| (60i32, 50i32));
    let mut export_pos = use_signal(|| (400i32, 50i32));

    // Auto-fit view when waveforms change
    let _waveform_count = waveforms.len();
    use_effect(move || {
        let waveforms = &sim_state.read().waveforms;
        if !waveforms.is_empty() {
            view_state.write().fit_to_data(waveforms);
        }
    });

    rsx! {
        // Fullscreen drag overlay - only appears when dragging to capture all mouse events
        if global_drag.read().active_panel.is_some() {
            div {
                class: "drag-overlay",
                style: "
                    position: fixed;
                    top: 0;
                    left: 0;
                    right: 0;
                    bottom: 0;
                    z-index: 9999;
                    cursor: move;
                ",
                onmousemove: move |e| {
                    let drag = global_drag.read();
                    if let Some(ref panel_id) = drag.active_panel {
                        let new_x = (e.client_coordinates().x as i32 - drag.offset_x).max(0);
                        let new_y = (e.client_coordinates().y as i32 - drag.offset_y).max(0);
                        match panel_id.as_str() {
                            "measurements" => measurements_pos.set((new_x, new_y)),
                            "fft" => fft_pos.set((new_x, new_y)),
                            "sweep" => sweep_pos.set((new_x, new_y)),
                            "export" => export_pos.set((new_x, new_y)),
                            _ => {}
                        }
                    }
                },
                onmouseup: move |_| {
                    global_drag.write().active_panel = None;
                },
            }
        }

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
                on_add_trace: move |expr: String| {
                    // Build signal map from existing waveforms
                    let state = sim_state.read();
                    let signals: std::collections::HashMap<String, (Vec<f64>, Vec<f64>)> =
                        state.waveforms.iter()
                            .map(|wf| (wf.name.clone(), (wf.x.clone(), wf.y.clone())))
                            .collect();
                    drop(state);

                    // Try to evaluate the expression
                    match waveform_math::eval_expression(&expr, &signals) {
                        Ok((x_data, y_data)) => {
                            // Generate a color for the new trace
                            let trace_count = sim_state.read().waveforms.len();
                            let theme: Signal<Theme> = use_context();
                            let color = theme.read().trace_color(trace_count).to_string();

                            // Add the computed waveform
                            sim_state.write().waveforms.push(crate::state::WaveformData {
                                name: expr,
                                x: x_data,
                                y: y_data,
                                color,
                                visible: true,
                            });
                        }
                        Err(e) => {
                            // Log error (in a real app would show in UI)
                            eprintln!("Expression error: {}", e);
                        }
                    }
                },
                on_toggle_measurements: move |_| {
                    let current = *show_measurements.read();
                    show_measurements.set(!current);
                },
                on_toggle_fft: move |_| {
                    let current = *show_fft.read();
                    show_fft.set(!current);
                },
                on_toggle_sweep: move |_| {
                    let current = *show_sweep.read();
                    show_sweep.set(!current);
                },
                on_toggle_export: move |_| {
                    let current = *show_export.read();
                    show_export.set(!current);
                },
                on_close: move |_| {
                    waveform_visible.set(crate::app::WaveformVisible(false));
                },
                measurements_active: *show_measurements.read(),
                fft_active: *show_fft.read(),
                sweep_active: *show_sweep.read(),
                export_active: *show_export.read(),
            }

            // Measurements panel overlay
            MeasurementsPanel {
                visible: *show_measurements.read(),
                on_close: move |_| show_measurements.set(false),
                region: None,
                position: *measurements_pos.read(),
                on_drag_start: move |offset: (i32, i32)| {
                    let mut drag = global_drag.write();
                    drag.active_panel = Some("measurements".to_string());
                    drag.offset_x = offset.0;
                    drag.offset_y = offset.1;
                },
            }

            // FFT viewer overlay
            FftViewer {
                visible: *show_fft.read(),
                on_close: move |_| show_fft.set(false),
                position: *fft_pos.read(),
                on_drag_start: move |offset: (i32, i32)| {
                    let mut drag = global_drag.write();
                    drag.active_panel = Some("fft".to_string());
                    drag.offset_x = offset.0;
                    drag.offset_y = offset.1;
                },
            }

            // Sweep panel overlay
            SweepPanel {
                visible: *show_sweep.read(),
                on_close: move |_| show_sweep.set(false),
                on_run_sweep: move |config: sweep_panel::SweepConfig| {
                    // TODO: Implement actual sweep execution
                    println!("Running sweep: {} from {} to {} ({} steps)",
                        config.parameter, config.start, config.end, config.steps);
                },
                position: *sweep_pos.read(),
                on_drag_start: move |offset: (i32, i32)| {
                    let mut drag = global_drag.write();
                    drag.active_panel = Some("sweep".to_string());
                    drag.offset_x = offset.0;
                    drag.offset_y = offset.1;
                },
            }

            // Export panel overlay
            ExportPanel {
                visible: *show_export.read(),
                on_close: move |_| show_export.set(false),
                position: *export_pos.read(),
                on_drag_start: move |offset: (i32, i32)| {
                    let mut drag = global_drag.write();
                    drag.active_panel = Some("export".to_string());
                    drag.offset_x = offset.0;
                    drag.offset_y = offset.1;
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

                // Plot area with zoom/pan/cursors/box selection
                WaveformPlotArea {
                    view_state: view_state,
                    cursor_state: cursor_state,
                    box_selection: box_selection,
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
                        LegendItem {
                            name: "V(out)".to_string(),
                            color: th.trace_color(0).to_string(),
                            visible: true,
                            on_toggle: move |_| {}, // Demo, no toggle
                        }
                    } else {
                        for (idx, wf) in waveforms.iter().enumerate() {
                            LegendItem {
                                key: "{wf.name}-{idx}",
                                name: wf.name.clone(),
                                color: wf.color.clone(),
                                visible: wf.visible,
                                on_toggle: move |_| {
                                    // Toggle visibility in SimulationState
                                    if let Some(wf) = sim_state.write().waveforms.get_mut(idx) {
                                        wf.visible = !wf.visible;
                                    }
                                },
                            }
                        }
                    }
                }
            }

            // X-axis with time labels
            XAxisLabels { view: *view_state.read() }
        }
    }
}

/// Plot area with mouse interaction for zoom/pan.
#[component]
fn WaveformPlotArea(
    mut view_state: Signal<ViewState>,
    mut cursor_state: Signal<CursorState>,
    mut box_selection: Signal<BoxSelection>,
    waveforms: Vec<crate::state::WaveformData>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let view = *view_state.read();
    let cursors = *cursor_state.read();
    let box_sel = *box_selection.read();

    // Store mounted element data for dimension queries
    let mut mounted_data: Signal<Option<std::rc::Rc<dioxus::prelude::MountedData>>> =
        use_signal(|| None);

    // Determine cursor style based on pan state
    let cursor_style = if view.is_panning {
        "grabbing"
    } else {
        "crosshair"
    };

    rsx! {
        div {
            class: "plot-area",
            style: "
                flex: 1;
                position: relative;
                background: {th.bg_primary()};
                overflow: hidden;
                cursor: {cursor_style};
                user-select: none;
                -webkit-user-select: none;
            ",

            // Get actual element dimensions on mount and store ref
            onmounted: move |e| {
                let data = e.data();
                mounted_data.set(Some(data.clone()));
                spawn(async move {
                    if let Ok(rect) = data.get_client_rect().await {
                        view_state.write().plot_width = rect.width().max(100.0);
                        view_state.write().plot_height = rect.height().max(100.0);
                    }
                });
            },

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

            // Pan with mouse drag, or box zoom with shift+drag
            onmousedown: move |e| {
                // Prevent native browser drag behavior
                e.prevent_default();

                let elem = e.element_coordinates();
                let vs = *view_state.read();

                // Convert element coords to data coords for box selection
                let plot_width = vs.plot_width.max(100.0);
                let plot_height = vs.plot_height.max(100.0);
                let data_x = vs.x_min + (elem.x / plot_width) * (vs.x_max - vs.x_min);
                let data_y = vs.y_max - (elem.y / plot_height) * (vs.y_max - vs.y_min);

                if e.modifiers().shift() {
                    // Shift+drag = box zoom selection
                    box_selection.write().start(data_x, data_y, elem.x, elem.y);
                } else {
                    // Normal drag = pan
                    let mut vs = view_state.write();
                    vs.is_panning = true;
                    vs.pan_start_x = elem.x;
                    vs.pan_start_y = elem.y;
                    vs.did_drag = false; // Reset drag flag
                }
            },

            onmouseup: move |_| {
                // Finish box selection if active
                if let Some((x_min, x_max, y_min, y_max)) = box_selection.write().finish() {
                    // Zoom to selected region
                    let mut vs = view_state.write();
                    vs.x_min = x_min;
                    vs.x_max = x_max;
                    vs.y_min = y_min;
                    vs.y_max = y_max;
                }
                view_state.write().is_panning = false;
            },

            onmouseleave: move |_| {
                box_selection.write().cancel();
                view_state.write().is_panning = false;
            },

            // Click to place cursor (only if not dragging)
            onclick: move |e| {
                // Skip cursor placement if we just finished a drag
                if view_state.read().did_drag {
                    view_state.write().did_drag = false;
                    return;
                }

                let elem = e.element_coordinates();

                // Refresh dimensions from stored element ref if available
                if let Some(ref data) = *mounted_data.read() {
                    let data = data.clone();
                    spawn(async move {
                        if let Ok(rect) = data.get_client_rect().await {
                            let plot_width = rect.width().max(100.0);
                            let plot_height = rect.height().max(100.0);
                            view_state.write().plot_width = plot_width;
                            view_state.write().plot_height = plot_height;

                            let vs = *view_state.read();
                            let data_x = vs.x_min + (elem.x / plot_width) * (vs.x_max - vs.x_min);

                            if !box_selection.read().is_selecting {
                                cursor_state.write().place(data_x);
                            }
                        }
                    });
                } else {
                    // Fallback to cached dimensions
                    let vs = *view_state.read();
                    let plot_width = vs.plot_width.max(100.0);
                    let data_x = vs.x_min + (elem.x / plot_width) * (vs.x_max - vs.x_min);

                    if !box_selection.read().is_selecting {
                        cursor_state.write().place(data_x);
                    }
                }
            },

            onmousemove: move |e| {
                let elem = e.element_coordinates();
                let vs = *view_state.read();
                let plot_width = vs.plot_width.max(100.0);
                let plot_height = vs.plot_height.max(100.0);

                // Update box selection if active
                if box_selection.read().is_selecting {
                    let data_x = vs.x_min + (elem.x / plot_width) * (vs.x_max - vs.x_min);
                    let data_y = vs.y_max - (elem.y / plot_height) * (vs.y_max - vs.y_min);
                    box_selection.write().update(data_x, data_y);
                } else if vs.is_panning {
                    // Calculate pixel deltas using element-relative coordinates
                    let pixel_dx = elem.x - vs.pan_start_x;
                    let pixel_dy = elem.y - vs.pan_start_y;

                    let x_range = vs.x_max - vs.x_min;
                    let y_range = vs.y_max - vs.y_min;

                    // Convert pixel movement to data coordinate movement
                    let data_dx = -(pixel_dx / plot_width) * x_range;
                    let data_dy = (pixel_dy / plot_height) * y_range;

                    let mut new_vs = view_state.write();
                    new_vs.pan(data_dx, data_dy);
                    new_vs.pan_start_x = elem.x;
                    new_vs.pan_start_y = elem.y;
                    new_vs.did_drag = true; // Mark that a drag occurred
                }
            },

            // Grid lines
            WaveformGrid { view: view }

            // Waveform traces - GPU or SVG rendering
            if is_gpu_available() {
                // GPU-accelerated rendering - pass Signal for reactivity
                GpuWaveformCanvas {
                    view_state: view_state,
                    waveforms: waveforms.clone(),
                }
            } else {
                // SVG fallback
                if waveforms.is_empty() {
                    {
                        let n = 500;
                        let x_range = view.x_max - view.x_min;
                        let step = x_range / (n - 1) as f64;
                        let demo_x: Vec<f64> = (0..n).map(|i| view.x_min + i as f64 * step).collect();
                        let demo_y: Vec<f64> = demo_x.iter()
                            .map(|t| (2.0 * std::f64::consts::PI * 1000.0 * t).sin())
                            .collect();
                        let demo_name = "demo";
                        rsx! {
                            WaveformTraceView {
                                key: "{demo_name}",
                                x: demo_x,
                                y: demo_y,
                                color: "#22c55e".to_string(),
                                view: view,
                            }
                        }
                    }
                } else {
                    for wf in waveforms.iter().filter(|w| w.visible) {
                        WaveformTraceView {
                            key: "{wf.name}",
                            x: wf.x.clone(),
                            y: wf.y.clone(),
                            color: wf.color.clone(),
                            view: view,
                        }
                    }
                }
            }

            // Cursor lines
            CursorLines { cursors: cursors, view: view }

            // Box selection overlay (during Shift+drag)
            if box_sel.is_selecting {
                {
                    let x1_frac = ((box_sel.start_x - view.x_min) / (view.x_max - view.x_min)).clamp(0.0, 1.0);
                    let x2_frac = ((box_sel.end_x - view.x_min) / (view.x_max - view.x_min)).clamp(0.0, 1.0);
                    let y1_frac = ((view.y_max - box_sel.start_y) / (view.y_max - view.y_min)).clamp(0.0, 1.0);
                    let y2_frac = ((view.y_max - box_sel.end_y) / (view.y_max - view.y_min)).clamp(0.0, 1.0);

                    let x_pct = x1_frac.min(x2_frac) * 100.0;
                    let y_pct = y1_frac.min(y2_frac) * 100.0;
                    let w_pct = (x1_frac - x2_frac).abs() * 100.0;
                    let h_pct = (y1_frac - y2_frac).abs() * 100.0;

                    rsx! {
                        div {
                            style: "
                                position: absolute;
                                left: {x_pct}%;
                                top: {y_pct}%;
                                width: {w_pct}%;
                                height: {h_pct}%;
                                background: rgba(59, 130, 246, 0.2);
                                border: 2px solid #3b82f6;
                                pointer-events: none;
                                z-index: 20;
                            "
                        }
                    }
                }
            }

            // Cursor readout overlay with per-trace values
            if cursors.cursor1.is_some() || cursors.cursor2.is_some() {
                CursorReadout { cursors: cursors, waveforms: waveforms.clone() }
            }
        }
    }
}

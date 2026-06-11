//! EYE — density-rendered eye diagram with the compliance mask, measured
//! metrics in the right panel.
//!
//! Acquisitions are folded segments of the active transient (built by the
//! eye pipeline). They are rasterized once per data revision into an
//! alpha-accumulated density texture — the classic persistence picture —
//! so each frame costs one textured quad plus the vector mask, instead of
//! restroking a transient's worth of segments.

use egui::Ui;

use crate::analysis::eye_diagram::EyeData;
use crate::common::AppState;
use crate::ui::plot::{self, Axis, PlotSpec, XScale, fmt_si};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::section_header;

use super::strip::{self, LegendChip};
use super::well_hint;

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the eye.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let eye = &state.analysis.eye_diagram_state;
    let data = &eye.data;
    if data.traces.is_empty() {
        well_hint(ui, "No eye yet — the eye folds the active transient at the bit period");
        return;
    }

    let subtitle = format!(
        "{} acquisitions · {} UI · {}",
        data.traces.len(),
        data.ui_count,
        fmt_si(data.data_rate, "b/s", 1)
    );
    let legend = [LegendChip {
        name: "density",
        color: c.traces[0],
        on: true,
    }];
    strip::StripHeader::new("EYE", &subtitle, &legend).show(ui);

    let ui_count = f64::from(data.ui_count.max(1));
    let swing = (data.v_high - data.v_low).abs().max(1e-9);
    let y0 = data.v_low - swing * 0.18;
    let y1 = data.v_high + swing * 0.18;

    let half_ticks: Vec<f64> = (0..=(ui_count * 2.0) as i64)
        .map(|i| i as f64 * 0.5)
        .collect();
    let mut spec = PlotSpec::new(
        Axis::with_ticks(0.0, ui_count, "UI", &half_ticks),
        XScale::Linear,
        Axis::linear(y0, y1, "V"),
    );

    // Bake (or fetch) the density texture for the current plot size. The
    // bake walks every acquisition once; frames just draw the quad.
    let plot_rect = plot::plot_rect(ui, &spec);
    let tex_size = [
        (plot_rect.width().round() as usize).max(8),
        (plot_rect.height().round() as usize).max(8),
    ];
    let revision = eye.data_revision();
    let trace_color = c.traces[0];
    let needs_bake = match &state.shell.results.eye_texture {
        Some(tex) => {
            tex.revision != revision || tex.size != tex_size || tex.color != trace_color
        }
        None => true,
    };
    if needs_bake {
        let image = rasterize_density(data, ui_count, y0, y1, tex_size, trace_color);
        let handle =
            ui.ctx()
                .load_texture("volta.eye.density", image, egui::TextureOptions::LINEAR);
        state.shell.results.eye_texture = Some(super::EyeTexture {
            revision,
            size: tex_size,
            color: trace_color,
            handle,
        });
    }
    let texture_id = state
        .shell
        .results
        .eye_texture
        .as_ref()
        .map(|tex| tex.handle.id());

    // Underlay: the density quad, then the mask above it.
    let show_mask = eye.show_mask && eye.mask.enabled && !eye.mask.inner.points.is_empty();
    let mask_fill = {
        let [r, g, b, _] = c.err.to_array();
        egui::Color32::from_rgba_unmultiplied(r, g, b, 26)
    };
    // The absolute mask mapped into display coordinates (unit intervals,
    // volts): time is scaled by the UI ratio so the mask keeps its per-UI
    // geometry at any data rate, voltage lands on the axis as stored.
    let mask_points: Vec<(f64, f64)> = eye.mask.inner_in_ui_volts().points;
    let err = c.err;
    spec.underlay = Some(Box::new(move |painter, mapper| {
        if let Some(texture_id) = texture_id {
            painter.image(
                texture_id,
                mapper.rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        }
        if show_mask {
            let points: Vec<egui::Pos2> = mask_points
                .iter()
                .map(|&(t_ui, volts)| egui::pos2(mapper.x(t_ui), mapper.y(volts)))
                .collect();
            if points.len() >= 3 {
                painter.add(egui::Shape::convex_polygon(
                    points.clone(),
                    mask_fill,
                    egui::Stroke::NONE,
                ));
                let mut outline = points;
                outline.push(outline[0]);
                painter.extend(egui::Shape::dashed_line(
                    &outline,
                    egui::Stroke::new(1.0, err),
                    5.0,
                    4.0,
                ));
            }
        }
    }));

    plot::show(ui, &spec, &mut state.shell.results.cache, None, None);
}

/// Rasterize every acquisition into an alpha-accumulated density image —
/// the same visual as stroking each at ~18/255 alpha, computed once.
fn rasterize_density(
    data: &EyeData,
    ui_count: f64,
    y0: f64,
    y1: f64,
    size: [usize; 2],
    color: egui::Color32,
) -> egui::ColorImage {
    let [width, height] = size;
    let mut coverage = vec![0.0f32; width * height];
    let x_scale = width as f64 / ui_count.max(1e-12);
    let y_span = (y1 - y0).max(1e-12);
    let y_scale = height as f64 / y_span;

    for trace in &data.traces {
        let n = trace.time.len().min(trace.amplitude.len());
        for i in 1..n {
            let ax = (trace.time[i - 1] * x_scale) as f32;
            let ay = ((y1 - trace.amplitude[i - 1]) * y_scale) as f32;
            let bx = (trace.time[i] * x_scale) as f32;
            let by = ((y1 - trace.amplitude[i]) * y_scale) as f32;
            accumulate_line(&mut coverage, width, height, ax, ay, bx, by);
        }
    }

    let [r, g, b, _] = color.to_array();
    let pixels = coverage
        .iter()
        .map(|&cov| {
            let alpha = (cov * 18.0).min(255.0) as u8;
            egui::Color32::from_rgba_unmultiplied(r, g, b, alpha)
        })
        .collect();
    egui::ColorImage {
        size: [width, height],
        pixels,
    }
}

/// Anti-aliased DDA: walk the segment one pixel-step at a time, splitting
/// each step's coverage bilinearly across the four nearest pixels.
fn accumulate_line(
    coverage: &mut [f32],
    width: usize,
    height: usize,
    ax: f32,
    ay: f32,
    bx: f32,
    by: f32,
) {
    let dx = bx - ax;
    let dy = by - ay;
    let steps = dx.abs().max(dy.abs()).ceil().max(1.0) as usize;
    let inv = 1.0 / steps as f32;
    for step in 0..steps {
        let t = step as f32 * inv;
        let x = ax + dx * t;
        let y = ay + dy * t;
        let (xf, yf) = (x.floor(), y.floor());
        let (fx, fy) = (x - xf, y - yf);
        let (xi, yi) = (xf as isize, yf as isize);
        let splat = [
            (xi, yi, (1.0 - fx) * (1.0 - fy)),
            (xi + 1, yi, fx * (1.0 - fy)),
            (xi, yi + 1, (1.0 - fx) * fy),
            (xi + 1, yi + 1, fx * fy),
        ];
        for (px, py, weight) in splat {
            if px >= 0 && py >= 0 && (px as usize) < width && (py as usize) < height {
                coverage[py as usize * width + px as usize] += weight;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Eye metrics + mask verdict.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let eye = &state.analysis.eye_diagram_state;
    if eye.data.traces.is_empty() {
        section_header(ui, "Eye", None);
        super::panel_note(ui, "Metrics appear once the eye is built from the transient.");
        return;
    }
    let m = &eye.measurements;

    section_header(ui, "Eye", None);
    let rows = [
        ("Eye height", fmt_si(m.eye_height, "V", 0), true),
        ("Eye width", format!("{:.2} UI", m.eye_width), true),
        ("Jitter rms", fmt_si(m.jitter_rms, "s", 1), false),
        ("Jitter p-p", fmt_si(m.jitter_pp, "s", 1), false),
        (
            "Crossing",
            format!("{:.1} %", m.crossing_percentage * 100.0),
            false,
        ),
        ("Rise 20–80", fmt_si(m.rise_time, "s", 0), false),
        ("Fall 80–20", fmt_si(m.fall_time, "s", 0), false),
        ("Q factor", format!("{:.1}", m.q_factor), false),
    ];
    super::stat_table(ui, &rows);

    if eye.mask.enabled {
        section_header(ui, "Mask", None);
        let mask = &eye.mask;
        let rows = [
            ("Template", mask.name.clone(), false),
            (
                "Violations",
                format!("{} / {}", mask.violation_count, mask.total_samples),
                true,
            ),
            (
                "Margin",
                format!("{:.0} %", mask.get_margin() * 100.0),
                false,
            ),
        ];
        super::stat_table(ui, &rows);
        // The mask is stored in absolute units at its authoring rate; when
        // the on-screen eye runs at a different rate, the mask time is
        // scaled by the UI ratio (see `EyeMask::inner_in_ui_volts`) — say so.
        let reference = mask.reference_data_rate;
        let current = eye.data.data_rate;
        if reference > 0.0 && (reference - current).abs() > reference * 1e-6 {
            super::panel_note(
                ui,
                &format!(
                    "Mask authored at {} — time scaled to the current UI.",
                    fmt_si(reference, "b/s", 1)
                ),
            );
        }
    }
    super::panel_note(
        ui,
        "Acquisitions folded at the configured bit period; thresholds 20/50/80 %.",
    );
}

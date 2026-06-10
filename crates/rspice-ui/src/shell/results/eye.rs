//! EYE — density-rendered eye diagram with the compliance mask, measured
//! metrics in the right panel.
//!
//! Acquisitions are folded segments of the active transient (built by the
//! eye pipeline); each is stroked at low alpha so overlap accumulates into
//! the classic density picture — one batched pass, no textures.

use egui::Ui;

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
    strip::header(ui, "EYE", &subtitle, &legend, false, false);

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

    // Density underlay: every acquisition at low alpha; the mask above them.
    let show_mask = eye.show_mask && eye.mask.enabled && !eye.mask.inner.points.is_empty();
    let alpha_color = {
        let [r, g, b, _] = c.traces[0].to_array();
        egui::Color32::from_rgba_unmultiplied(r, g, b, 18)
    };
    let mask_fill = {
        let [r, g, b, _] = c.err.to_array();
        egui::Color32::from_rgba_unmultiplied(r, g, b, 26)
    };
    let stroke = egui::Stroke::new(1.4, alpha_color);
    let traces = &data.traces;
    let mask_points: &[(f64, f64)] = &eye.mask.inner.points;
    let v_cross = data.v_cross;
    let err = c.err;
    spec.underlay = Some(Box::new(move |painter, mapper| {
        for trace in traces {
            let points: Vec<egui::Pos2> = trace
                .time
                .iter()
                .zip(trace.amplitude.iter())
                .map(|(&x, &v)| egui::pos2(mapper.x(x), mapper.y(v)))
                .collect();
            if points.len() >= 2 {
                painter.add(egui::Shape::line(points, stroke));
            }
        }
        if show_mask {
            let points: Vec<egui::Pos2> = mask_points
                .iter()
                .map(|&(tn, vn)| {
                    let x = tn * ui_count;
                    let v = eye_mask_v(vn, v_cross, swing);
                    egui::pos2(mapper.x(x), mapper.y(v))
                })
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

/// Map a normalized mask voltage to volts around the crossing level.
fn eye_mask_v(v_normalized: f64, v_cross: f64, swing: f64) -> f64 {
    v_cross + v_normalized * swing
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
    }
    super::panel_note(
        ui,
        "Acquisitions folded at the configured bit period; thresholds 20/50/80 %.",
    );
}

//! SMITH — S-parameter loci on the reflection-coefficient plane with the
//! canonical resistance/reactance grid, in design-system colors.
//!
//! The chart grid is painted as an underlay in `canvas_grid`; traces come
//! from the token cycle like every other viewer.

use egui::Ui;

use crate::common::AppState;
use crate::ui::plot::{self, Axis, PlotSpec, Trace, XScale, fmt_si};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::section_header;

use super::strip::{self, LegendChip};
use super::well_hint;

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the Smith chart.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let smith = &state.analysis.smith_chart_state;
    let visible: Vec<usize> = smith
        .traces
        .iter()
        .enumerate()
        .filter(|(_, tr)| tr.visible && !tr.points.is_empty())
        .map(|(i, _)| i)
        .collect();
    if visible.is_empty() {
        well_hint(ui, "No S-parameter traces — run an SP analysis");
        return;
    }

    // Γ-plane component arrays per trace, cached per data version.
    let mut arrays = Vec::new();
    for &index in &visible {
        let points = smith.traces[index].points.clone();
        let derived = &mut state.shell.results.derived;
        let re = derived.get_or(0x501_0000 | (index as u64) << 8, || {
            points.iter().map(|p| p.s.re).collect()
        });
        let im = derived.get_or(0x501_0001 | (index as u64) << 8, || {
            points.iter().map(|p| p.s.im).collect()
        });
        arrays.push((index, re, im));
    }

    let smith = &state.analysis.smith_chart_state;
    let legend: Vec<LegendChip> = visible
        .iter()
        .enumerate()
        .map(|(slot, &index)| LegendChip {
            name: &smith.traces[index].name,
            color: c.traces[slot % c.traces.len()],
            on: true,
        })
        .collect();
    strip::header(
        ui,
        "SMITH",
        &format!("Z₀ = {} Ω", smith.z0),
        &legend,
        false,
        false,
        false,
    );

    let mut spec = PlotSpec::new(
        Axis::linear(-1.12, 1.12, "Re Γ"),
        XScale::Linear,
        Axis::linear(-1.12, 1.12, "Im Γ"),
    );
    for (slot, (index, re, im)) in arrays.iter().enumerate() {
        spec.traces.push(
            Trace::new(re, im, c.traces[slot % c.traces.len()])
                .cache_key(0x501_00F0 | *index as u64),
        );
    }

    // The canonical chart grid: constant-resistance circles and
    // constant-reactance arcs in token grid color, |Γ| = 1 boundary in
    // the strong border color.
    let grid = c.canvas_grid;
    let boundary = c.border_strong;
    spec.underlay = Some(Box::new(move |painter, mapper| {
        let center = egui::pos2(mapper.x(0.0), mapper.y(0.0));
        let unit = (mapper.x(1.0) - mapper.x(0.0)).abs();
        let stroke = egui::Stroke::new(1.0, grid);

        // Constant resistance r: center ((r/(r+1)), 0), radius 1/(r+1).
        for r in [0.2, 0.5, 1.0, 2.0, 5.0] {
            let cx = r / (r + 1.0);
            let radius = 1.0 / (r + 1.0);
            painter.circle_stroke(
                egui::pos2(mapper.x(cx), mapper.y(0.0)),
                (radius * unit as f64) as f32,
                stroke,
            );
        }
        // Constant reactance x: circles centered (1, ±1/x) with radius
        // 1/x, clipped to the unit disc. Walk the full circle and emit a
        // polyline per contiguous in-disc run.
        for x in [0.2f64, 0.5, 1.0, 2.0, 5.0] {
            for sign in [1.0f64, -1.0] {
                let cy = sign / x;
                let radius = 1.0 / x;
                let mut run: Vec<egui::Pos2> = Vec::new();
                for step in 0..=128 {
                    let theta = std::f64::consts::TAU * step as f64 / 128.0;
                    let gx = 1.0 + radius * theta.cos();
                    let gy = cy + radius * theta.sin();
                    if gx * gx + gy * gy <= 1.0001 {
                        run.push(egui::pos2(mapper.x(gx), mapper.y(gy)));
                    } else if run.len() >= 2 {
                        painter.add(egui::Shape::line(std::mem::take(&mut run), stroke));
                    } else {
                        run.clear();
                    }
                }
                if run.len() >= 2 {
                    painter.add(egui::Shape::line(run, stroke));
                }
            }
        }
        // Real axis + boundary.
        painter.hline(
            egui::Rangef::new(mapper.x(-1.0), mapper.x(1.0)),
            center.y,
            stroke,
        );
        painter.circle_stroke(center, unit, egui::Stroke::new(1.2, boundary));
    }));

    // Square chart region.
    let avail = ui.available_rect_before_wrap();
    let side = avail.width().min(avail.height());
    let rect = egui::Rect::from_center_size(avail.center(), egui::vec2(side, side));
    let mut plot_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );

    plot::show(&mut plot_ui, &spec, &mut state.shell.results.cache, None, None);
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Trace summary.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    section_header(ui, "S-parameters", None);
    let smith = &state.analysis.smith_chart_state;
    let Some(trace) = smith.traces.iter().find(|tr| tr.visible && !tr.points.is_empty())
    else {
        super::panel_note(ui, "Trace metrics appear once S-parameter data is loaded.");
        return;
    };

    let first = trace.points.first();
    let last = trace.points.last();
    let rows = [
        ("Trace", trace.name.clone(), true),
        ("Points", trace.points.len().to_string(), false),
        (
            "f range",
            match (first, last) {
                (Some(a), Some(b)) => format!(
                    "{} – {}",
                    fmt_si(a.frequency, "Hz", 1),
                    fmt_si(b.frequency, "Hz", 1)
                ),
                _ => "—".to_owned(),
            },
            false,
        ),
        ("Z₀", format!("{} Ω", smith.z0), false),
    ];
    super::stat_table(ui, &rows);
    super::panel_note(
        ui,
        "Loci on the reflection-coefficient plane; grid circles are constant R and X.",
    );
}

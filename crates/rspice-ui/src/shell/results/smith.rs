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
    strip::StripHeader::new("SMITH", &format!("Z₀ = {} Ω", smith.z0), &legend).show(ui);

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

    let response = plot::show(&mut plot_ui, &spec, &mut state.shell.results.cache, None, None);

    // Interactive readout: nearest locus point on hover, click to pin.
    // The chart space IS the Γ plane, so the conversion to impedance is
    // z = z0 (1+Γ)/(1−Γ).
    let ranges = ((-1.12, 1.12), (-1.12, 1.12));
    let mut hovered: Option<(usize, usize)> = None;
    if let Some(pointer) = response.response.hover_pos()
        && response.plot_rect.contains(pointer)
    {
        let mut best = 14.0f32 * 14.0;
        for (slot, (_, re, im)) in arrays.iter().enumerate() {
            for i in 0..re.len() {
                let pos = super::xy_screen_pos(
                    response.plot_rect,
                    (re[i], im[i]),
                    ranges.0,
                    ranges.1,
                );
                let d2 = pos.distance_sq(pointer);
                if d2 < best {
                    best = d2;
                    hovered = Some((slot, i));
                }
            }
        }
    }

    if response.response.clicked() {
        let pins = &mut state.shell.results.rf_pin;
        match hovered {
            Some(hit) if pins.get(&super::ResultViewer::Smith) == Some(&hit) => {
                pins.remove(&super::ResultViewer::Smith);
            }
            Some(hit) => {
                pins.insert(super::ResultViewer::Smith, hit);
            }
            None => {
                pins.remove(&super::ResultViewer::Smith);
            }
        }
    }

    let pinned = state
        .shell
        .results
        .rf_pin
        .get(&super::ResultViewer::Smith)
        .copied()
        .filter(|(slot, i)| {
            arrays
                .get(*slot)
                .is_some_and(|(_, re, _)| *i < re.len())
        });
    let target = hovered.or(pinned);

    if let Some((slot, i)) = target {
        let (trace_index, re, im) = &arrays[slot];
        let gamma_re = re[i];
        let gamma_im = im[i];
        let pos = super::xy_screen_pos(
            response.plot_rect,
            (gamma_re, gamma_im),
            ranges.0,
            ranges.1,
        );
        let color = c.traces[slot % c.traces.len()];
        let painter = plot_ui.painter();
        if pinned == Some((slot, i)) {
            painter.circle_stroke(pos, 6.0, egui::Stroke::new(1.8, color));
        }
        painter.circle_stroke(pos, 4.0, egui::Stroke::new(1.5, c.accent));

        let smith = &state.analysis.smith_chart_state;
        let trace = &smith.traces[*trace_index];
        let frequency = trace.points.get(i).map(|p| p.frequency).unwrap_or(0.0);
        let mag = (gamma_re * gamma_re + gamma_im * gamma_im).sqrt();
        let phase_deg = gamma_im.atan2(gamma_re).to_degrees();
        // z = z0 (1+Γ)/(1−Γ)
        let denom = (1.0 - gamma_re) * (1.0 - gamma_re) + gamma_im * gamma_im;
        let (r, x) = if denom > 1e-12 {
            (
                smith.z0 * (1.0 - gamma_re * gamma_re - gamma_im * gamma_im) / denom,
                smith.z0 * (2.0 * gamma_im) / denom,
            )
        } else {
            (f64::INFINITY, 0.0)
        };
        let vswr = if mag < 1.0 {
            format!("{:.2}", (1.0 + mag) / (1.0 - mag))
        } else {
            "∞".to_owned()
        };
        let rows = [
            ("f".to_owned(), fmt_si(frequency, "Hz", 2)),
            (
                "Γ".to_owned(),
                format!("{mag:.3} ∠ {phase_deg:.1}°"),
            ),
            (
                "Z".to_owned(),
                if r.is_finite() {
                    format!(
                        "{} {} j{}",
                        fmt_si(r, "Ω", 2),
                        if x >= 0.0 { "+" } else { "−" },
                        fmt_si(x.abs(), "Ω", 2)
                    )
                } else {
                    "open".to_owned()
                },
            ),
            ("VSWR".to_owned(), vswr),
        ];
        super::point_card(&plot_ui, response.plot_rect, pos, &trace.name, color, &rows);
    }
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

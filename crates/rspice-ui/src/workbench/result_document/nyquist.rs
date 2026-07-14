//! NYQUIST — the loop-gain locus on the complex plane, with the unit
//! circle, the −1 critical point, and stability numbers in the right panel.
//!
//! Built on the plot grammar like every other viewer: token grid and frame,
//! decimated token traces, bordered marker tags — no private palette.

use egui::Ui;

use crate::analysis::nyquist::NyquistData;
use crate::common::AppState;
use crate::ui::plot::{self, Axis, PlotSpec, Trace, XScale, fmt_si};
use crate::ui::tokens::Tokens;
use crate::ui::widgets::section_header;

use super::strip::{self, LegendChip};
use super::well_hint;

/// The active curve, if any (prefer the selected one).
fn active_curve(state: &AppState) -> Option<&NyquistData> {
    let nyquist = &state.analysis.nyquist_state;
    nyquist
        .curves
        .get(nyquist.selected)
        .or_else(|| nyquist.curves.first())
        .filter(|curve| !curve.is_empty())
}

/// Stability numbers, cached on the data version (the scans are O(points)).
#[derive(Debug, Clone, Copy)]
pub struct NyquistDerived {
    pub(crate) version: u64,
    pub(crate) encirclements: i32,
    pub(crate) min_distance: Option<f64>,
    pub(crate) gain_margin: Option<f64>,
    pub(crate) phase_margin: Option<f64>,
}

fn derived(state: &mut AppState) -> Option<NyquistDerived> {
    let version = state.simulation.data_version;
    if let Some(cached) = state.ui.results.nyquist
        && cached.version == version
    {
        return Some(cached);
    }
    let curve = active_curve(state)?;
    let computed = NyquistDerived {
        version,
        encirclements: curve.count_encirclements(),
        min_distance: curve.min_distance_from_critical(),
        gain_margin: curve.gain_margin(),
        phase_margin: curve.phase_margin(),
    };
    state.ui.results.nyquist = Some(computed);
    Some(computed)
}

// ---------------------------------------------------------------------------
// center view
// ---------------------------------------------------------------------------

/// Render the Nyquist locus.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let Some(curve) = active_curve(state) else {
        well_hint(ui, "No loop-gain data — run an AC analysis");
        return;
    };
    let name = curve.name.clone();
    let point_count = curve.len();

    // Component arrays for the plot engine, cached per data version.
    let (re, im) = {
        let points: Vec<_> = curve.points.clone();
        let derived = &mut state.ui.results.derived;
        let re = derived.get_or(0x917_0001, || {
            std::sync::Arc::new(points.iter().map(|p| p.real).collect::<Vec<_>>())
        });
        let im = derived.get_or(0x917_0002, || {
            std::sync::Arc::new(points.iter().map(|p| p.imag).collect::<Vec<_>>())
        });
        (re, im)
    };

    let stats = derived(state);

    let legend = [LegendChip {
        name: "loop gain",
        color: c.traces[0],
        on: true,
    }];
    strip::StripHeader::new("NYQ", &format!("{name} · {point_count} pts"), &legend).show(ui);

    // Equal-aspect ranges around the locus and the critical point.
    let mut extent = 1.3f64;
    for (&r, &i) in re.iter().zip(im.iter()) {
        if r.is_finite() && i.is_finite() {
            extent = extent.max(r.abs()).max(i.abs());
        }
    }
    let extent = (extent * 1.1).min(50.0);

    let mut spec = PlotSpec::new(
        Axis::linear(-extent, extent, "Re"),
        XScale::Linear,
        Axis::linear(-extent, extent, "Im"),
    )
    .accessible_name("Nyquist plot");
    spec.ref_lines.push(plot::RefLine { y: 0.0 });
    spec.traces
        .push(Trace::new(&re, &im, c.traces[0]).cache_key(0x917_00FF));

    // Critical point.
    spec.markers.push(plot::Marker {
        x: -1.0,
        y: 0.0,
        side: plot::YSide::Left,
        color: c.err,
        label: "−1 + j0".to_owned(),
        drop_line: false,
        label_dy: 0.0,
    });

    // Unit circle + vertical axis underlay.
    let grid = c.canvas_grid;
    spec.underlay = Some(Box::new(move |painter, mapper| {
        let center = egui::pos2(mapper.x(0.0), mapper.y(0.0));
        let radius = (mapper.x(1.0) - mapper.x(0.0)).abs();
        painter.circle_stroke(center, radius, egui::Stroke::new(1.0, grid));
        painter.vline(
            center.x,
            egui::Rangef::new(mapper.rect.top(), mapper.rect.bottom()),
            egui::Stroke::new(1.0, grid),
        );
    }));

    // Region sized so the INNER plot area is square: stability reading
    // needs the unit circle and the locus on one scale.
    let avail = ui.available_rect_before_wrap();
    let rect = plot::square_outer_rect(avail, &spec);
    let mut plot_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(egui::Layout::top_down(egui::Align::Min)),
    );

    let stats_for_readout = stats;
    let readout = move |_x: f64| -> Vec<(String, String)> {
        let mut rows = Vec::new();
        if let Some(s) = stats_for_readout {
            rows.push(("N".to_owned(), s.encirclements.to_string()));
            if let Some(d) = s.min_distance {
                rows.push(("min |1+L|".to_owned(), fmt_si(d, "", 2)));
            }
        }
        rows
    };

    let response = plot::show(
        &mut plot_ui,
        &spec,
        &mut state.ui.results.cache,
        None,
        Some(&readout),
    );

    // Nearest locus point on hover, click to pin — gain/phase/frequency at
    // a spot on the locus is the question a Nyquist plot exists to answer.
    let ranges = ((-extent, extent), (-extent, extent));
    let mut hovered: Option<(usize, usize)> = None;
    if let Some(pointer) = response.response.hover_pos()
        && response.plot_rect.contains(pointer)
    {
        let mut best = 14.0f32 * 14.0;
        for i in 0..re.len() {
            let pos = super::xy_screen_pos(response.plot_rect, (re[i], im[i]), ranges.0, ranges.1);
            let d2 = pos.distance_sq(pointer);
            if d2 < best {
                best = d2;
                hovered = Some((0, i));
            }
        }
    }

    if response.response.clicked() {
        let pins = &mut state.ui.results.rf_pin;
        match hovered {
            Some(hit) if pins.get(&super::ResultViewer::Nyquist) == Some(&hit) => {
                pins.remove(&super::ResultViewer::Nyquist);
            }
            Some(hit) => {
                pins.insert(super::ResultViewer::Nyquist, hit);
            }
            None => {
                pins.remove(&super::ResultViewer::Nyquist);
            }
        }
    }

    let pinned = state
        .ui
        .results
        .rf_pin
        .get(&super::ResultViewer::Nyquist)
        .copied()
        .filter(|(_, i)| *i < re.len());
    let target = hovered.or(pinned);

    if let Some((_, i)) = target {
        let pos = super::xy_screen_pos(response.plot_rect, (re[i], im[i]), ranges.0, ranges.1);
        let painter = plot_ui.painter();
        if pinned == target {
            painter.circle_stroke(pos, 6.0, egui::Stroke::new(1.8, c.traces[0]));
        }
        painter.circle_stroke(pos, 4.0, egui::Stroke::new(1.5, c.accent));

        let frequency = active_curve(state)
            .and_then(|curve| curve.points.get(i).map(|p| p.frequency))
            .unwrap_or(0.0);
        let magnitude = (re[i] * re[i] + im[i] * im[i]).sqrt();
        let phase_deg = im[i].atan2(re[i]).to_degrees();
        let rows = [
            ("f".to_owned(), fmt_si(frequency, "Hz", 2)),
            ("Re".to_owned(), fmt_si(re[i], "", 3)),
            ("Im".to_owned(), fmt_si(im[i], "", 3)),
            (
                "|L| ∠".to_owned(),
                format!("{magnitude:.3} ∠ {phase_deg:.1}°"),
            ),
        ];
        super::point_card(&plot_ui, response.plot_rect, pos, &name, c.traces[0], &rows);
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Stability readout.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    section_header(ui, "Stability", None);
    let Some(s) = derived(state) else {
        super::panel_note(
            ui,
            "Stability numbers appear once AC loop-gain data is loaded.",
        );
        return;
    };

    let verdict = if s.encirclements == 0 {
        "stable"
    } else {
        "unstable"
    };
    let fmt_opt =
        |v: Option<f64>, f: &dyn Fn(f64) -> String| -> String { v.map_or("—".to_owned(), f) };
    let rows = [
        ("Encirclements", s.encirclements.to_string(), true),
        ("Verdict", verdict.to_owned(), true),
        (
            "Min distance to −1",
            fmt_opt(s.min_distance, &|v| fmt_si(v, "", 3)),
            false,
        ),
        (
            "Gain margin",
            fmt_opt(s.gain_margin, &|v| format!("{:.1} dB", 20.0 * v.log10())),
            false,
        ),
        (
            "Phase margin",
            fmt_opt(s.phase_margin, &|v| format!("{v:.1}°")),
            false,
        ),
    ];
    super::stat_table(ui, &rows);
    super::panel_note(
        ui,
        "Counted around −1 + j0 on the mapped AC loop gain; margins read off the same locus.",
    );
}

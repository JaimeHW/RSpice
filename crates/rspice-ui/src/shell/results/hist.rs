//! HIST — Monte-Carlo distribution: accent-tinted bins, a normal-fit
//! overlay, the ±1σ band, and µ / spec-limit markers; distribution stats and
//! yield in the right panel.

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

/// Render the histogram.
pub fn show(ui: &mut Ui, state: &mut AppState) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let hist_state = &state.analysis.histogram_state;
    let Some(histogram) = hist_state.histograms.get(hist_state.selected) else {
        well_hint(ui, "No distribution yet — run a Monte Carlo analysis");
        return;
    };
    if histogram.bins.is_empty() || histogram.total_count == 0 {
        well_hint(ui, "The selected distribution is empty");
        return;
    }
    let stats = hist_state.stats.get(hist_state.selected);

    let subtitle = format!("{} · {} samples", histogram.name, histogram.total_count);
    let legend = [LegendChip {
        name: "count",
        color: c.accent,
        on: true,
    }];
    let view = state.shell.results.plot_view(super::ResultViewer::Hist, 0);
    let header = strip::StripHeader::new("MC", &subtitle, &legend)
        .zoomed(view.is_zoomed())
        .show(ui);
    if header.fit_clicked {
        state
            .shell
            .results
            .reset_plot_view(super::ResultViewer::Hist, 0);
    }

    let span = (histogram.data_max - histogram.data_min).abs().max(1e-12);
    let x0 = histogram.data_min - span * 0.06;
    let x1 = histogram.data_max + span * 0.06;
    let (x0, x1) = view.x.unwrap_or((x0, x1));
    let max_count = histogram.bins.iter().map(|b| b.count).max().unwrap_or(1) as f64;
    let y1 = (max_count * 1.18).ceil().max(4.0);
    let (y0, y1) = view.y.unwrap_or((0.0, y1));

    // Normal-fit overlay buffers — declared before the spec so the borrows
    // they hand to it outlive the plot call.
    let mut curve_x: Vec<f64> = Vec::new();
    let mut curve_y: Vec<f64> = Vec::new();

    let mut spec = PlotSpec::new(
        Axis::linear(x0, x1, ""),
        XScale::Linear,
        Axis::linear_with(y0, y1, "n", 5),
    );
    spec.left_margin = 48.0;

    // ±1σ band + µ marker + normal overlay from the fitted stats.
    if let Some(stats) = stats {
        if stats.std_dev > 0.0 {
            spec.bands.push(plot::Band {
                x0: stats.mean - stats.std_dev,
                x1: stats.mean + stats.std_dev,
            });
            // Normal pdf scaled to expected counts per bin.
            let bin_width = histogram.bins[0].width().max(1e-30);
            let scale = histogram.total_count as f64 * bin_width;
            let norm = 1.0 / (stats.std_dev * (2.0 * std::f64::consts::PI).sqrt());
            for i in 0..=120 {
                let x = x0 + (x1 - x0) * i as f64 / 120.0;
                let z = (x - stats.mean) / stats.std_dev;
                curve_x.push(x);
                curve_y.push(scale * norm * (-0.5 * z * z).exp());
            }
        }
        spec.markers.push(plot::Marker {
            x: stats.mean,
            y: y1 * 0.86,
            side: plot::YSide::Left,
            color: c.accent,
            label: format!("µ {}", fmt_si(stats.mean, "", 2)),
            drop_line: true,
            label_dy: 0.0,
        });
    }

    // Spec limits from the yield manager, when present.
    if let Some(yield_result) = state.simulation.yield_results.first() {
        if let Some(lsl) = yield_result.spec.min
            && lsl > x0
            && lsl < x1
        {
            spec.markers.push(plot::Marker {
                x: lsl,
                y: y1 * 0.72,
                side: plot::YSide::Left,
                color: c.err,
                label: format!("LSL {}", fmt_si(lsl, "", 2)),
                drop_line: true,
                label_dy: 0.0,
            });
        }
        if let Some(usl) = yield_result.spec.max
            && usl > x0
            && usl < x1
        {
            spec.markers.push(plot::Marker {
                x: usl,
                y: y1 * 0.72,
                side: plot::YSide::Left,
                color: c.err,
                label: format!("USL {}", fmt_si(usl, "", 2)),
                drop_line: true,
                label_dy: 0.0,
            });
        }
    }

    // Bars under everything else, with the out-of-spec regions washed in
    // the error tint — the fail zone itself, not the data envelope.
    let bins = &histogram.bins;
    let accent = c.accent;
    let accent_dim = c.accent_dim;
    let err = c.err;
    let spec_limits = state
        .simulation
        .yield_results
        .first()
        .map(|y| (y.spec.min, y.spec.max));
    spec.underlay = Some(Box::new(move |painter, mapper| {
        if let Some((lsl, usl)) = spec_limits {
            let wash = err.gamma_multiply(0.09);
            if let Some(lsl) = lsl
                && lsl > x0
            {
                let rect = egui::Rect::from_min_max(
                    egui::pos2(mapper.rect.left(), mapper.rect.top()),
                    egui::pos2(mapper.x(lsl.min(x1)), mapper.rect.bottom()),
                );
                painter.rect_filled(rect, 0.0, wash);
            }
            if let Some(usl) = usl
                && usl < x1
            {
                let rect = egui::Rect::from_min_max(
                    egui::pos2(mapper.x(usl.max(x0)), mapper.rect.top()),
                    egui::pos2(mapper.rect.right(), mapper.rect.bottom()),
                );
                painter.rect_filled(rect, 0.0, wash);
            }
        }
        for bin in bins {
            if bin.count == 0 {
                continue;
            }
            let left = mapper.x(bin.lower) + 1.0;
            let right = (mapper.x(bin.upper) - 1.0).max(left + 1.0);
            let top = mapper.y(bin.count as f64);
            let bottom = mapper.y(0.0);
            let rect = egui::Rect::from_min_max(egui::pos2(left, top), egui::pos2(right, bottom));
            painter.rect(
                rect,
                0.0,
                accent_dim,
                egui::Stroke::new(1.0, accent),
                egui::StrokeKind::Inside,
            );
        }
    }));

    if !curve_x.is_empty() {
        spec.traces
            .push(Trace::new(&curve_x, &curve_y, c.text_dim).thin().dashed());
    }

    let readout = |x: f64| -> Vec<(String, String)> {
        let count = bins
            .iter()
            .find(|b| x >= b.lower && x < b.upper)
            .map_or(0, |b| b.count);
        vec![
            ("x".to_owned(), fmt_si(x, "", 2)),
            ("count".to_owned(), count.to_string()),
        ]
    };

    let response = plot::show(
        ui,
        &spec,
        &mut state.shell.results.cache,
        None,
        Some(&readout),
    );
    if response.view.any() {
        state
            .shell
            .results
            .plot_view_mut(super::ResultViewer::Hist, 0)
            .apply(&response.view);
    }
}

// ---------------------------------------------------------------------------
// right panel
// ---------------------------------------------------------------------------

/// Distribution stats + spec/yield verdict.
pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let hist_state = &state.analysis.histogram_state;
    let Some(histogram) = hist_state.histograms.get(hist_state.selected) else {
        section_header(ui, "Distribution", None);
        super::panel_note(ui, "Stats appear once a Monte Carlo run is loaded.");
        return;
    };

    section_header(ui, "Distribution", None);
    if let Some(stats) = hist_state.stats.get(hist_state.selected) {
        let rows = [
            ("Samples", stats.count.to_string(), false),
            ("Mean", fmt_si(stats.mean, "", 3), true),
            ("Std dev", fmt_si(stats.std_dev, "", 3), true),
            ("Min", fmt_si(stats.min, "", 3), false),
            ("Max", fmt_si(stats.max, "", 3), false),
            ("Median", fmt_si(stats.median, "", 3), false),
            ("Skew", format!("{:+.2}", stats.skewness), false),
        ];
        super::stat_table(ui, &rows);
    } else {
        super::panel_note(ui, &format!("{} samples", histogram.total_count));
    }

    if let Some(yield_result) = state.simulation.yield_results.first() {
        section_header(ui, "Spec", None);
        let cpk = yield_result
            .stats
            .cpk
            .map_or("—".to_owned(), |v| format!("{v:.2}"));
        let rows = [
            (
                "Yield",
                format!("{:.1} %", yield_result.yield_percent),
                true,
            ),
            ("Cpk", cpk, false),
            (
                "Failures",
                format!("{} / {}", yield_result.fail_count, yield_result.total_runs),
                false,
            ),
        ];
        super::stat_table(ui, &rows);
    }
    super::panel_note(ui, "Normal fit overlaid; the shaded band spans ±1σ.");
}

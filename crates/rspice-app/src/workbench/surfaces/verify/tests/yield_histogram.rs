//! The Monte Carlo yield page: limits spelled like the values beside them,
//! statistics from the retained samples, and the histogram drawn under the
//! table.

use super::*;

fn fixture_yield_results(app: &RSpiceApp) -> Vec<rspice_results::yield_analysis::YieldResult> {
    verification_yield_results(app).to_vec()
}

/// A Spec cell and an axis tick state a quantity the way the measured values
/// beside them do: the SI prefix on the unit, one space in front of it.
///
/// The Spec column used to read `≥ 10M Hz` and `≤ 1.200m A` — the prefix split
/// off the unit it scales, and three zeros nobody authored — while the value
/// in the next cell read `11.90 MHz`.
#[test]
fn a_limit_and_a_tick_are_spelled_like_the_values_beside_them() {
    let bound = |min: Option<f64>, max: Option<f64>, unit: &str| {
        super::limit_cell(&SpecEntry {
            measurement: "m".to_owned(),
            expression: String::new(),
            min,
            max,
            unit: unit.to_owned(),
            scope: crate::state::SpecPointScope::AllPoints,
        })
        .text
    };

    assert_eq!(bound(Some(10.0e6), None, "Hz"), "\u{2265} 10 MHz");
    assert_eq!(bound(None, Some(1.2e-3), "A"), "\u{2264} 1.2 mA");
    assert_eq!(bound(Some(40.0), None, "dB"), "\u{2265} 40 dB");
    assert_eq!(bound(Some(45.0), None, "deg"), "\u{2265} 45 deg");
    assert_eq!(bound(None, Some(2.0e-6), "s"), "\u{2264} 2 \u{b5}s");
    assert_eq!(bound(Some(1.62), Some(1.98), "V"), "1.62 V \u{2026} 1.98 V");
    assert_eq!(
        bound(None, None, "V"),
        "",
        "a bound of nothing says nothing"
    );

    // One space before the unit, the same one the values carry.
    assert_eq!(super::histogram::tick_text(38.0, 0.5, "dB"), "38 dB");
    assert_eq!(super::histogram::tick_text(10.0e6, 2.0e6, "Hz"), "10 MHz");
    assert_eq!(super::format_value(38.9, "dB"), "38.90 dB");
}

/// The statistics come from the exact retained samples. The fixture's
/// summary block is all zeros, which is what the old histogram header printed
/// as "mean 0.000 dB" over data spanning 38.9 to 43.2.
#[test]
fn yield_rows_compute_their_statistics_from_the_samples() {
    let app = super::raster::monte_carlo_app();
    let results = fixture_yield_results(&app);
    assert_eq!(
        results[0].stats.mean, 0.0,
        "the fixture's own summary is empty"
    );
    let row = YieldRow::from_result(&results[0]);
    let samples = &results[0].samples;
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    let stats = row.stats.expect("forty samples have statistics");
    assert!((stats.mean - mean).abs() < 1.0e-12);
    assert_eq!(stats.min, 38.9);
    assert_eq!(
        stats.max,
        samples.iter().copied().fold(f64::NEG_INFINITY, f64::max)
    );
    assert!((stats.max - 43.2).abs() < 1.0e-9);
    assert!(stats.std_dev > 0.5 && stats.std_dev < 2.0);
    let cpk = stats.cpk.expect("a lower limit has a one-sided Cpk");
    assert!((cpk - (stats.mean - 40.0) / (3.0 * stats.std_dev)).abs() < 1.0e-9);
    assert_eq!((row.passing, row.total), (37, 40));
    assert_eq!(row.yield_percent(), Some(92.5));
    assert_eq!(row.verdict(), Verdict::Fail);
    assert_eq!(row.limit.limit_text(), "≥ 40 dB");
    assert_eq!(
        yield_summary(&results),
        Some(super::toolbar::Summary::failing("yield 92.5 % (37 / 40)"))
    );
}

#[test]
fn the_shown_histogram_is_the_readers_pick_or_the_lowest_yield() {
    let row = |output: &str, passing: usize| YieldRow {
        output: output.to_owned(),
        limit: SpecEntry {
            measurement: output.to_owned(),
            expression: String::new(),
            min: Some(0.0),
            max: None,
            unit: String::new(),
            scope: crate::state::SpecPointScope::AllPoints,
        },
        stats: None,
        passing,
        total: 10,
    };
    let rows = [row("gain", 9), row("bw", 7), row("pm", 10)];
    assert_eq!(shown_yield_row(&rows, None), Some(1));
    assert_eq!(shown_yield_row(&rows, Some("pm")), Some(2));
    assert_eq!(shown_yield_row(&rows, Some("gone")), Some(1));
    assert_eq!(shown_yield_row(&[], None), None);
}

/// The bins are laid so the limit is a bin edge: every sample in a failing
/// bar breaks the limit and every sample in a neutral bar meets it.
#[test]
fn histogram_bins_put_the_limit_on_an_edge_and_keep_every_sample() {
    let app = super::raster::monte_carlo_app();
    let results = fixture_yield_results(&app);
    let model = super::histogram::histogram_model(&results[0].samples, Some(40.0), None)
        .expect("forty samples bin");
    assert_eq!(model.counts.iter().sum::<usize>(), 40);
    assert!(
        model.edges.iter().any(|edge| (edge - 40.0).abs() < 1.0e-9),
        "the limit is an edge: {:?}",
        model.edges
    );
    let failing = model
        .counts
        .iter()
        .zip(&model.failing)
        .filter(|(_, failing)| **failing)
        .map(|(count, _)| count)
        .sum::<usize>();
    assert_eq!(failing, 3, "exactly the three samples under 40 dB");
    assert_eq!(model.limits, [40.0]);
    assert!(model.axis.0 <= model.mean - 3.0 * model.std_dev);
    assert!(model.axis.1 >= model.mean + 3.0 * model.std_dev);
    assert!((8..=48).contains(&model.counts.len()));
    assert!(model.ticks.len() >= 4);
    assert!(model.ticks.windows(2).all(|pair| pair[1] > pair[0]));
    assert_eq!(super::histogram::tick_text(39.5, 0.5, "dB"), "39.5 dB");
    assert!(super::histogram::histogram_model(&[f64::NAN], Some(1.0), None).is_none());
}

/// Bars are neutral and failing bars a muted error tone; nothing in the
/// histogram is painted in the accent.
#[test]
fn histogram_bars_are_neutral_and_failing_bars_muted_error() {
    let app = super::raster::monte_carlo_app();
    let results = fixture_yield_results(&app);
    let model = super::histogram::histogram_model(&results[0].samples, Some(40.0), None)
        .expect("forty samples bin");
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut fills = Vec::new();
    for _ in 0..2 {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(900.0, 320.0),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default().show(ctx, |ui| {
                    super::histogram::paint_histogram(ui, "gain", "dB", &model, 300.0);
                });
            },
        );
        fills = output
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                egui::Shape::Rect(rect) => Some(rect.fill),
                _ => None,
            })
            .collect::<Vec<_>>();
    }
    let t = Tokens::get(&ctx);
    assert!(!fills.contains(&t.color.accent));
    assert!(fills.contains(&t.color.err.gamma_multiply(0.45)));
    assert!(fills.contains(&t.color.text_dim.gamma_multiply(0.38)));
}

/// The selected row's histogram sits in the pane under the table, not stacked
/// above it, and the table keeps the page's grammar.
#[test]
fn the_yield_page_draws_the_selected_histogram_under_the_table() {
    let geometry =
        super::raster::page_geometry(super::raster::monte_carlo_app(), VerificationPage::Yield);
    let rows = geometry.all("table.row");
    assert_eq!(rows.len(), 1);
    let split = geometry
        .one("split")
        .expect("the detail pane is split off the table");
    assert_eq!(split.top(), rows[0].bottom());
    let histogram = geometry
        .one("histogram")
        .expect("the selected row's histogram");
    assert_eq!(histogram.top(), split.bottom());
    assert!(
        (220.0..=340.0).contains(&histogram.height()),
        "{histogram:?}"
    );
}

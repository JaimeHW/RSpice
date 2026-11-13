//! A worst-sample row keeps its margin column clear at drawer width, and the
//! yield interval stays inside a probability.

use super::*;
use rspice_ui_kit::panels::PANEL_INSET;

#[test]
fn failure_row_reserves_a_non_overlapping_margin_column_at_drawer_width() {
    let row_width = 228.0;
    let columns = failure_row_columns(row_width, 48.0, PANEL_INSET);
    let target_right = PANEL_INSET + columns.target_width;
    let margin_left = row_width - PANEL_INSET - columns.margin_width;

    assert!(target_right + FAILURE_ROW_COLUMN_GAP <= margin_left + f32::EPSILON);
    assert_eq!(columns.margin_width, 48.0);
}

#[test]
fn failure_row_height_grows_from_wrapped_target_copy_at_drawer_width() {
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let input = egui::RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, Vec2::new(228.0, 400.0))),
        ..egui::RawInput::default()
    };
    let failure = FailedSample {
        target: "an intentionally long production specification target that must wrap".to_owned(),
        unit: "V".to_owned(),
        sample_index: 16,
        value: 0.912_345,
        normalized_margin: -0.1834,
    };
    let mut row_height = 0.0;

    let _ = ctx.run_ui(input, |ctx| {
        egui::CentralPanel::default()
            .frame(egui::Frame::new())
            .show(ctx, |ui| {
                row_height = failure_row(ui, &failure).rect.height();
            });
    });

    assert!(row_height > 58.0, "wrapped row height was {row_height}");
}

#[test]
fn wilson_interval_is_bounded_and_contains_the_observed_rate() {
    let (low, high) = wilson_interval_95(986, 1_000).expect("valid population");

    assert!((0.0..=1.0).contains(&low));
    assert!((0.0..=1.0).contains(&high));
    assert!(low < 0.986 && high > 0.986);
    assert!(wilson_interval_95(0, 0).is_none());
    assert!(wilson_interval_95(2, 1).is_none());
}

/// A trial that did not converge has a place in the trail and no sample, so
/// after it the two no longer line up. The worst samples are the values
/// outside their limit, not the values at a failing trail position.
#[test]
fn worst_samples_are_the_values_outside_their_limit() {
    let result = rspice_results::yield_analysis::YieldResult {
        spec: rspice_results::yield_analysis::YieldSpec::lower("gain", 40.0, "dB"),
        total_runs: 4,
        pass_count: 2,
        fail_count: 2,
        yield_percent: 50.0,
        stats: Default::default(),
        // The second trial did not converge; the fourth measured 39.5 dB.
        trail: vec![true, false, true, false],
        samples: vec![41.0, 42.0, 39.5],
    };
    let worst = worst_samples(&[result]);
    assert_eq!(
        worst
            .iter()
            .map(|sample| (sample.sample_index, sample.value))
            .collect::<Vec<_>>(),
        [(2, 39.5)]
    );
}

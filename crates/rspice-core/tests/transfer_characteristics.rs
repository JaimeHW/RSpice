use rspice_core::Complex64;
use rspice_core::analysis::transfer::{AcTransferPoint, AcTransferResult};

fn point(frequency: f64, db: f64, degrees: f64) -> AcTransferPoint {
    AcTransferPoint::new(
        frequency,
        Complex64::from_polar(10f64.powf(db / 20.0), degrees.to_radians()),
    )
}

#[test]
fn transfer_bandpass_finds_both_cutoffs_adjacent_to_the_peak() {
    let mut result = AcTransferResult::new("out", "in");
    result.points = vec![
        point(1.0, -10.0, 0.0),
        point(10.0, 0.0, 0.0),
        point(100.0, -10.0, 0.0),
    ];
    result.compute_characteristics();
    let low = 10f64.powf(0.7);
    let high = 10f64.powf(1.3);
    assert!((result.cutoff_low.unwrap() - low).abs() < 1e-12);
    assert!((result.cutoff_high.unwrap() - high).abs() < 1e-12);
    assert!((result.bandwidth.unwrap() - (high - low)).abs() < 1e-12);
    assert_eq!(result.dc_gain, None);
    result.points.clear();
    result.compute_characteristics();
    assert_eq!(result.bandwidth, None);
    assert_eq!(result.peak_frequency, None);
    assert_eq!(result.q_factor, None);
}

#[test]
fn transfer_phase_margin_uses_continuous_phase() {
    let mut result = AcTransferResult::new("out", "in");
    result.points = vec![point(1.0, 6.0, -170.0), point(100.0, -6.0, 170.0)];
    result.compute_characteristics();
    assert!(result.phase_margin.unwrap().abs() < 1e-10);
}

#[test]
fn transfer_group_delay_handles_large_phases_and_rejects_repeated_frequencies() {
    let first = point(1.0, 0.0, 0.0);
    let mut next = point(2.0, 0.0, 0.0);
    next.phase_rad = 1e20;
    assert!(first.group_delay(&next).is_finite());
    next.frequency = 1.0;
    assert!(first.group_delay(&next).is_nan());
}

use rspice_core::Complex64;
use rspice_core::analysis::stb::{StbAnalysisError, StbAnalyzer, StbConfig};

fn gain(db: f64, degrees: f64) -> Complex64 {
    Complex64::from_polar(10f64.powf(db / 20.0), degrees.to_radians())
}

#[test]
fn stability_interpolates_across_the_phase_branch_cut() {
    let result = StbAnalyzer::new(StbConfig::default())
        .analyze(&[1.0, 100.0], &[gain(6.0, -170.0), gain(-6.0, 170.0)])
        .unwrap();
    assert!(result.margins.phase_margin_deg.abs() < 1e-10);
    assert!((result.margins.phase_margin_freq - 10.0).abs() < 1e-10);
    assert!(result.margins.gain_margin_db.abs() < 1e-10);
}

#[test]
fn stability_does_not_invent_a_negative_real_crossing_near_zero_phase() {
    let result = StbAnalyzer::new(StbConfig::default())
        .analyze(&[1.0, 100.0], &[gain(-6.0, -10.0), gain(-6.0, 10.0)])
        .unwrap();
    assert_eq!(result.margins.gain_margin_db, f64::INFINITY);
    assert_eq!(result.margins.gain_margin_freq, 0.0);
}

#[test]
fn stability_rejects_invalid_samples_instead_of_certifying_them() {
    let analyzer = StbAnalyzer::new(StbConfig::default());
    for bad in [
        Complex64::new(f64::NAN, 0.0),
        Complex64::new(f64::INFINITY, 0.0),
        Complex64::new(0.0, 0.0),
    ] {
        assert!(matches!(
            analyzer.analyze(&[1.0, 100.0], &[gain(6.0, 0.0), bad]),
            Err(StbAnalysisError::InvalidSample { index: 1, .. })
        ));
    }
    for frequencies in [[1.0, 1.0], [2.0, 1.0], [0.0, 1.0], [1.0, f64::NAN]] {
        assert!(
            analyzer
                .analyze(&frequencies, &[gain(6.0, 0.0); 2])
                .is_err()
        );
    }
    let empty = analyzer.analyze(&[], &[]).unwrap();
    assert!(!empty.is_stable());
    assert_eq!(empty.assessment(), "ANALYSIS FAILED");
}

#[test]
fn sampled_unity_crossovers_are_counted_once_including_sweep_endpoints() {
    for gains in [[0.0, -6.0, -12.0], [6.0, 0.0, -6.0], [12.0, 6.0, 0.0]] {
        let result = StbAnalyzer::new(StbConfig::default())
            .analyze(&[1.0, 10.0, 100.0], &gains.map(|db| gain(db, -90.0)))
            .unwrap();
        assert_eq!(result.margins.num_crossovers, 1);
        assert!((result.margins.phase_margin_deg - 90.0).abs() < 1e-10);
    }
}

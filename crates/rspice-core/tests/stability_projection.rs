use rspice_core::Complex64;
use rspice_core::analysis::stb::{StbAnalysisError, StbAnalyzer, StbConfig};
use rspice_core::execution::result_document::ScalarValue;
use rspice_core::execution::{AnalysisInstanceId, AnalysisKind, AnalysisResultDocument};

fn gain(db: f64, degrees: f64) -> Complex64 {
    Complex64::from_polar(10f64.powf(db / 20.0), degrees.to_radians())
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn stability_interpolates_across_the_phase_branch_cut() {
    let result = StbAnalyzer::new(StbConfig::default())
        .analyze(&[1.0, 100.0], &[gain(6.0, -170.0), gain(-6.0, 170.0)])
        .unwrap();
    assert!(result.margins.phase_margin_deg.abs() < 1e-10);
    assert!((result.margins.phase_margin_freq - 10.0).abs() < 1e-10);
    assert!(result.margins.gain_margin_db.abs() < 1e-10);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn stability_does_not_invent_a_negative_real_crossing_near_zero_phase() {
    let result = StbAnalyzer::new(StbConfig::default())
        .analyze(&[1.0, 100.0], &[gain(-6.0, -10.0), gain(-6.0, 10.0)])
        .unwrap();
    assert_eq!(result.margins.gain_margin_db, f64::INFINITY);
    assert_eq!(result.margins.gain_margin_freq, 0.0);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
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

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn sampled_unity_crossovers_are_counted_once_including_sweep_endpoints() {
    for gains in [[0.0, -6.0, -12.0], [6.0, 0.0, -6.0], [12.0, 6.0, 0.0]] {
        let result = StbAnalyzer::new(StbConfig::default())
            .analyze(&[1.0, 10.0, 100.0], &gains.map(|db| gain(db, -90.0)))
            .unwrap();
        assert_eq!(result.margins.num_crossovers, 1);
        assert!((result.margins.phase_margin_deg - 90.0).abs() < 1e-10);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn unity_and_negative_real_plateaus_keep_the_smallest_margin() {
    let analyzer = StbAnalyzer::new(StbConfig::default());
    let unity = analyzer
        .analyze(
            &[1.0, 10.0, 100.0],
            &[
                Complex64::new(1.0, 0.0),
                Complex64::new(0.0, -1.0),
                Complex64::new(-1.0, 0.0),
            ],
        )
        .unwrap();
    assert_eq!(unity.margins.num_crossovers, 1);
    assert_eq!(unity.margins.phase_margin_deg, 0.0);
    assert_eq!(unity.margins.phase_margin_freq, 100.0);

    // Every frequency in this segment is on the negative real axis.
    // Its closest gain margin is inside the segment, at L=-1.
    let inversion = analyzer
        .analyze(
            &[1.0, 100.0],
            &[Complex64::new(-2.0, 0.0), Complex64::new(-0.5, 0.0)],
        )
        .unwrap();
    assert_eq!(inversion.margins.num_crossovers, 1);
    assert_eq!(inversion.margins.gain_margin_db, 0.0);
    assert_eq!(inversion.margins.phase_margin_deg, 0.0);
    assert_eq!(inversion.margins.gain_margin_freq, 10.0);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn phase_margin_uses_the_closest_crossover_and_retains_its_sign() {
    // Each decade crosses unity halfway in log frequency. The three phase
    // margins are 100, 50 and -10 degrees, so the last crossing binds.
    let samples = [
        gain(6.0, -60.0),
        gain(-6.0, -100.0),
        gain(6.0, -160.0),
        gain(-6.0, -220.0),
    ];
    let result = StbAnalyzer::new(StbConfig::default())
        .analyze(&[1.0, 10.0, 100.0, 1000.0], &samples)
        .unwrap();
    assert_eq!(result.margins.num_crossovers, 3);
    assert!((result.margins.phase_margin_deg + 10.0).abs() < 1e-10);
    assert!((result.margins.phase_margin_freq / 1000f64.sqrt() / 10.0 - 1.0).abs() < 1e-12);
    assert_eq!(
        result.margins.unity_gain_bandwidth,
        result.margins.phase_margin_freq
    );

    let document = AnalysisResultDocument::from_stability(
        AnalysisInstanceId::new(AnalysisKind::Stb, 0),
        &result,
    )
    .unwrap()
    .build()
    .unwrap();
    let decoded = AnalysisResultDocument::from_json(&document.to_json().unwrap()).unwrap();
    assert_eq!(decoded, document);
    for (name, expected) in [
        ("phase_margin_degrees", -10.0),
        ("phase_margin_frequency", 100_000f64.sqrt()),
        ("unity_gain_bandwidth", 100_000f64.sqrt()),
    ] {
        let scalar = decoded
            .scalars()
            .iter()
            .find(|scalar| scalar.name() == name)
            .unwrap();
        let ScalarValue::Real {
            value: Some(actual),
        } = scalar.value()
        else {
            panic!("{name} lost its measured value");
        };
        assert!((actual - expected).abs() < 1e-9, "{name}");
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gain_margin_searches_all_phase_crossings_even_without_unity_gain() {
    for sign in [-1.0, 1.0] {
        // Both crossings are at -180 degrees; their gain margins are
        // -sign*12 and -sign*7 dB. Neither sweep crosses unity.
        let result = StbAnalyzer::new(StbConfig::default())
            .analyze(
                &[1.0, 10.0, 100.0],
                &[
                    gain(sign * 12.0, -160.0),
                    gain(sign * 12.0, -200.0),
                    gain(sign * 2.0, -160.0),
                ],
            )
            .unwrap();
        assert_eq!(result.margins.num_crossovers, 0);
        assert!((result.margins.gain_margin_db + sign * 7.0).abs() < 1e-10);
        assert!((result.margins.gain_margin_freq / 1000f64.sqrt() - 1.0).abs() < 1e-12);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn margins_use_each_odd_half_turn_without_losing_phase_winding() {
    let result = StbAnalyzer::new(StbConfig::default())
        .analyze(
            &[1.0, 10.0, 100.0, 1000.0, 1e4, 1e5],
            &[
                gain(12.0, -60.0),
                gain(10.0, -180.0),
                gain(8.0, -300.0),
                gain(6.0, -420.0),
                gain(4.0, -540.0),
                gain(-4.0, -600.0),
            ],
        )
        .unwrap();
    assert!((result.margins.phase_margin_deg + 30.0).abs() < 1e-10);
    assert!((result.margins.gain_margin_db + 4.0).abs() < 1e-10);
    assert!((result.margins.gain_margin_freq / 1e4 - 1.0).abs() < 1e-12);
    assert!((result.bode_points.last().unwrap().phase_deg + 600.0).abs() < 1e-10);

    // A sweep starting on the positive branch must find the same critical
    // negative-real axis as one starting on the negative branch.
    for start in [-200.0, 160.0] {
        let result = StbAnalyzer::new(StbConfig::default())
            .analyze(&[1.0, 100.0], &[gain(6.0, start), gain(-6.0, start + 40.0)])
            .unwrap();
        assert!(result.margins.phase_margin_deg.abs() < 1e-10);
    }
}

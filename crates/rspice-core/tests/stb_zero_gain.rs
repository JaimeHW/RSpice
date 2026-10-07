use rspice_core::analysis::stb::{StbAnalyzer, StbConfig, StbResult, StbSweepType};
use rspice_core::execution::{AnalysisInstanceId, AnalysisKind, AnalysisResultDocument};
use rspice_core::{Complex64, Engine, Netlist};

fn roundtrip(result: &StbResult) -> serde_json::Value {
    result
        .validate_with_abort(
            &rspice_core::ResourceLimits::default(),
            &rspice_core::NoAbort,
        )
        .unwrap();
    let document = AnalysisResultDocument::from_stability(
        AnalysisInstanceId::new(AnalysisKind::Stb, 0),
        result,
    )
    .unwrap()
    .build()
    .unwrap();
    let json = document.to_json().unwrap();
    assert_eq!(AnalysisResultDocument::from_json(&json).unwrap(), document);
    serde_json::from_str(&json).unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn retained_stb_rejects_fabricated_ordinates_and_mismatched_evidence() {
    let result = StbAnalyzer::new(StbConfig::new())
        .analyze(
            &[1.0, 100.0, 10000.0],
            &[
                Complex64::new(0.0, 2.0),
                Complex64::new(0.0, 0.0),
                Complex64::new(0.0, -2.0),
            ],
        )
        .unwrap();
    for corrupt in 0..5 {
        let mut bad = result.clone();
        match corrupt {
            0 => bad.bode_points[1].phase_deg = Some(0.0),
            1 => bad.bode_points[1].magnitude_db = Some(-6000.0),
            2 => bad.bode_points[2].phase_deg = Some(270.0),
            3 => bad.nyquist_points[0].real = 1.0,
            _ => bad.margins.num_crossovers = 0,
        }
        assert!(
            bad.validate_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort
            )
            .is_err()
        );
    }
    assert!(
        result
            .validate_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::abort_signal::ImmediateAbort
            )
            .is_err()
    );
    let mut limits = rspice_core::ResourceLimits::default();
    limits.max_result_values = 26;
    assert!(
        result
            .validate_with_abort(&limits, &rspice_core::NoAbort)
            .is_err()
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn zero_loop_gain_retains_raw_samples_and_explicit_derived_absence() {
    let netlist = Netlist::parse("* zero loop\nE1 eo 0 ctrl 0 0\nVP eo x 0\nR1 x ctrl 1k\nC1 ctrl 0 159.154943091895n\n.end\n").unwrap();
    for points in [1, 3] {
        let result = Engine::default()
            .run_stb(
                &netlist,
                StbConfig::new()
                    .with_probe("VP")
                    .with_sweep_type(StbSweepType::Linear)
                    .with_sweep(10.0, 1000.0, points),
            )
            .unwrap();
        assert!(result.result.success);
        assert_eq!(result.loop_gains, vec![Complex64::new(0.0, 0.0); points]);
        assert_eq!(result.result.nyquist_points.len(), points);
        for point in &result.result.bode_points {
            assert_eq!(point.magnitude, Some(0.0));
            assert_eq!(point.magnitude_db, None);
            assert_eq!(point.phase_deg, None);
        }
        assert_eq!(result.result.margins.gain_margin, None);
        assert_eq!(result.result.margins.phase_margin, None);
        let json = roundtrip(&result.result);
        for name in ["loop_gain_db", "loop_gain_phase"] {
            let signal = json["signals"]
                .as_array()
                .unwrap()
                .iter()
                .find(|s| s["descriptor"]["canonicalName"] == name)
                .unwrap();
            assert_eq!(
                signal["values"]["samples"],
                serde_json::json!(vec![None::<f64>; points])
            );
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn zero_endpoints_use_complex_interpolation_without_bridging_undefined_phase() {
    for gains in [
        [
            Complex64::new(-2.0, 0.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(-2.0, 0.0),
        ],
        [
            Complex64::new(0.0, 2.0),
            Complex64::new(0.0, 0.0),
            Complex64::new(0.0, -2.0),
        ],
    ] {
        let result = StbAnalyzer::new(StbConfig::new())
            .analyze(&[1.0, 100.0, 10_000.0], &gains)
            .unwrap();
        assert_eq!(result.margins.num_crossovers, 2);
        let phase = result.margins.phase_margin.unwrap();
        assert!((phase.frequency - 10.0).abs() < 1e-12);
        assert_eq!(result.bode_points[1].phase_deg, None);
        if gains[0].im == 0.0 {
            assert_eq!(phase.value, 0.0);
            let gain = result.margins.gain_margin.unwrap();
            assert_eq!(gain.value, 0.0);
            assert!((gain.frequency - 10.0).abs() < 1e-12);
        } else {
            assert_eq!(result.margins.gain_margin, None);
            assert_eq!(phase.value, -90.0);
            assert_eq!(result.bode_points[0].phase_deg, Some(90.0));
            assert_eq!(result.bode_points[2].phase_deg, Some(-90.0));
        }
        roundtrip(&result);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn finite_components_keep_log_magnitude_when_linear_magnitude_overflows() {
    let result = StbAnalyzer::new(StbConfig::new())
        .analyze(&[1.0], &[Complex64::new(f64::MAX, f64::MAX)])
        .unwrap();
    let point = &result.bode_points[0];
    assert_eq!(point.magnitude, None);
    assert!(
        (point.magnitude_db.unwrap() - (20.0 * f64::MAX.log10() + 10.0 * 2f64.log10())).abs()
            < 1e-10
    );
    assert_eq!(point.phase_deg, Some(45.0));
    roundtrip(&result);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn retained_validation_preserves_subnormal_gain_and_tiny_crossover_scales() {
    let analyzer = StbAnalyzer::new(StbConfig::new());
    let mut small_gain = analyzer
        .analyze(&[1.0], &[Complex64::new(1e-310, 0.0)])
        .unwrap();
    roundtrip(&small_gain);
    small_gain.bode_points[0].magnitude = Some(0.0);
    assert!(
        small_gain
            .validate_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort
            )
            .is_err()
    );
    let mut tiny_frequency = analyzer
        .analyze(
            &[1e-200, 1e-198],
            &[Complex64::new(0.0, -2.0), Complex64::new(0.0, -0.5)],
        )
        .unwrap();
    roundtrip(&tiny_frequency);
    tiny_frequency
        .margins
        .phase_margin
        .as_mut()
        .unwrap()
        .frequency = 1e-100;
    assert!(
        tiny_frequency
            .validate_with_abort(
                &rspice_core::ResourceLimits::default(),
                &rspice_core::NoAbort
            )
            .is_err()
    );
}

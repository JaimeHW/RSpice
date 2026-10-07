use rspice_core::analysis::stb::{StbAnalyzer, StbConfig, StbResult, StbSweepType};
use rspice_core::execution::{AnalysisInstanceId, AnalysisKind, AnalysisResultDocument};
use rspice_core::{Complex64, Engine, Netlist};

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn single_point_sweeps_retain_the_authored_raw_results() {
    for (gain, points) in [(0.5, 1), (1000.0, 1)] {
        let netlist = Netlist::parse(&format!(
            "* analytic single-pole loop\nE1 eo 0 ctrl 0 -{gain}\nVP eo x 0\n\
             R1 x ctrl 1k\nC1 ctrl 0 159.154943091895n\n.end\n"
        ))
        .unwrap();
        let result = Engine::default()
            .run_stb(
                &netlist,
                StbConfig::default()
                    .with_probe("VP")
                    .with_sweep_type(StbSweepType::Linear)
                    .with_sweep(10.0, 1000.0, points),
            )
            .expect("valid single-point sweeps must retain their measurements");
        assert!(result.result.success);
        assert_eq!(result.frequencies.len(), points);
        assert_eq!(result.result.bode_points.len(), points);
        assert_eq!(result.result.nyquist_points.len(), points);
        assert_eq!(result.result.margins.gain_margin, None);
        assert_eq!(result.result.margins.phase_margin, None);
        assert_eq!(result.result.margins.unity_gain_bandwidth(), None);
        roundtrip(&result.result);
        for (&frequency, &actual) in result.frequencies.iter().zip(&result.loop_gains) {
            let expected = gain / Complex64::new(1.0, frequency / 1000.0);
            assert!((actual - expected).norm() <= expected.norm() * 1e-10);
        }
    }
}

fn roundtrip(result: &StbResult) -> AnalysisResultDocument {
    let document = AnalysisResultDocument::from_stability(
        AnalysisInstanceId::new(AnalysisKind::Stb, 0),
        result,
    )
    .unwrap()
    .build()
    .unwrap();
    assert_eq!(
        AnalysisResultDocument::from_json(&document.to_json().unwrap()).unwrap(),
        document
    );
    document
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn sampled_critical_points_have_real_zero_margins_even_on_a_single_point_grid() {
    let result = StbAnalyzer::new(StbConfig::default())
        .analyze(&[100.0], &[Complex64::new(-1.0, 0.0)])
        .unwrap();
    assert_eq!(result.margins.num_crossovers, 1);
    for margin in [result.margins.gain_margin, result.margins.phase_margin] {
        let margin = margin.unwrap();
        assert_eq!(margin.value, 0.0);
        assert_eq!(margin.frequency, 100.0);
    }
    roundtrip(&result);
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn a_finite_band_cannot_establish_infinite_margins_or_stability() {
    // Stable closed-loop pole -1001*2*pi*1000 and unstable pole +500,
    // respectively. Both sweeps omit the crossover evidence margins need.
    let frequencies = [10.0, 100.0, 1000.0];
    for unstable in [false, true] {
        let samples = frequencies.map(|frequency| {
            if unstable {
                0.0005 / Complex64::new(-0.001, std::f64::consts::TAU * frequency * 1e-6)
            } else {
                1000.0 / Complex64::new(1.0, frequency / 1000.0)
            }
        });
        let result = StbAnalyzer::new(StbConfig::default())
            .analyze(&frequencies, &samples)
            .unwrap();
        assert_eq!(result.margins.gain_margin, None);
        assert_eq!(result.margins.phase_margin, None);
        assert_eq!(result.margin_assessment(), "NO MARGINS MEASURED");
        roundtrip(&result);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn interpolated_crossover_frequencies_stay_inside_extreme_finite_endpoints() {
    for frequencies in [
        [f64::from_bits(1), f64::from_bits(2)],
        [f64::from_bits(f64::MAX.to_bits() - 1), f64::MAX],
    ] {
        let result = StbAnalyzer::new(StbConfig::default())
            .analyze(
                &frequencies,
                &[Complex64::new(0.0, -2.0), Complex64::new(0.0, -0.5)],
            )
            .unwrap();
        let margin = result.margins.phase_margin.unwrap();
        assert!((frequencies[0]..=frequencies[1]).contains(&margin.frequency));
        assert_eq!(margin.value, 90.0);
        roundtrip(&result);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn legacy_margin_sentinels_are_migrated_and_current_pairs_are_validated() {
    let result = StbAnalyzer::new(StbConfig::default())
        .analyze(&[1.0, 2.0], &[Complex64::new(0.5, 0.0); 2])
        .unwrap();
    let original: serde_json::Value =
        serde_json::from_str(&roundtrip(&result).to_json().unwrap()).unwrap();
    for version in 1..=14 {
        let mut legacy = original.clone();
        legacy["schemaVersion"] = version.into();
        let scalars = legacy["scalars"].as_array_mut().unwrap();
        if version < 14 {
            scalars.retain(|s| s["name"] != "dc_loop_gain");
        }
        for scalar in scalars {
            match scalar["name"].as_str().unwrap() {
                "gain_margin_db" => scalar["value"]["reason"] = "positive_infinity".into(),
                "phase_margin_degrees" => scalar["value"]["reason"] = "negative_infinity".into(),
                "multiple_unity_gain_crossovers" => scalar["name"] = "conditionally_stable".into(),
                _ => {}
            }
        }
        let decoded = AnalysisResultDocument::from_json(&legacy.to_string()).unwrap();
        let normalized: serde_json::Value =
            serde_json::from_str(&decoded.to_json().unwrap()).unwrap();
        let scalars = normalized["scalars"].as_array().unwrap();
        for name in ["gain_margin_db", "phase_margin_degrees"] {
            assert_eq!(
                scalars.iter().find(|s| s["name"] == name).unwrap()["value"]["reason"],
                "no_crossover"
            );
        }
        assert!(!scalars.iter().any(|s| s["name"] == "conditionally_stable"));
        assert_eq!(
            AnalysisResultDocument::from_json(&decoded.to_json().unwrap()).unwrap(),
            decoded
        );
    }
    for (name, value) in [
        (
            "gain_margin_db",
            serde_json::json!({"representation":"real","value":1.0}),
        ),
        (
            "gain_margin_frequency",
            serde_json::json!({"representation":"real","value":0.0}),
        ),
        (
            "phase_margin_degrees",
            serde_json::json!({"representation":"unavailable","reason":"negative_infinity"}),
        ),
        (
            "unity_gain_bandwidth",
            serde_json::json!({"representation":"real","value":100.0}),
        ),
        (
            "multiple_unity_gain_crossovers",
            serde_json::json!({"representation":"boolean","value":true}),
        ),
        (
            "unity_gain_crossovers",
            serde_json::json!({"representation":"count","value":1}),
        ),
    ] {
        let mut invalid = original.clone();
        let scalar = invalid["scalars"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|s| s["name"] == name)
            .unwrap();
        scalar["value"] = value;
        assert!(
            AnalysisResultDocument::from_json(&invalid.to_string()).is_err(),
            "{name}"
        );
    }
}

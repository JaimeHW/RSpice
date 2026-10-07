use rspice_core::analysis::stb::{StbAnalyzer, StbConfig, StbResult, StbSweepType};
use rspice_core::execution::{AnalysisInstanceId, AnalysisKind, AnalysisResultDocument};
use rspice_core::{Complex64, Engine, Netlist};

const SINGLE_POLE: &str = "* T(s)=1000/(1+s/(2*pi*1000))
E1 eo 0 ctrl 0 -1000
VP eo x 0
R1 x ctrl 1k
C1 ctrl 0 159.154943091895n
.end
";

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn dc_loop_gain_is_measured_independently_of_the_sweep_start() {
    let netlist = Netlist::parse(SINGLE_POLE).unwrap();
    for start in [10.0, 1e5] {
        let result = Engine::default()
            .run_stb(
                &netlist,
                StbConfig::new()
                    .with_probe("VP")
                    .with_sweep_type(StbSweepType::Linear)
                    .with_sweep(start, 1e7, 3),
            )
            .unwrap();
        assert!((result.result.margins.dc_gain_db().unwrap() - 60.0).abs() < 1e-10);
        assert_eq!(result.frequencies.len(), 3);
        assert_eq!(result.frequencies[0], start);
        assert!((result.result.margins.dc_loop_gain.unwrap().re - 1000.0).abs() < 1e-8);
    }
}

fn document(result: &StbResult) -> AnalysisResultDocument {
    AnalysisResultDocument::from_stability(AnalysisInstanceId::new(AnalysisKind::Stb, 0), result)
        .unwrap()
        .build()
        .unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn zero_and_undefined_dc_gain_preserve_valid_ac_sweeps() {
    let cases = [
        // High-pass loop: T(s)=1000*s*RC/(1+s*RC), exactly zero at DC.
        (
            "E1 eo 0 ctrl 0 -1000\nVP eo x 0\nC1 x ctrl 1u\nR1 ctrl 0 1k\n",
            true,
        ),
        // Integrator loop: T(s)=gm/(s*C), undefined at DC even though its
        // closed-loop bias and positive-frequency response are well defined.
        (
            "G1 eo 0 ctrl 0 1m\nC1 eo 0 1u\nVP eo x 0\nR1 x ctrl 1k\n",
            false,
        ),
    ];
    for (circuit, zero) in cases {
        let netlist = Netlist::parse(&format!("* DC availability\n{circuit}.end\n")).unwrap();
        let result = Engine::default()
            .run_stb(
                &netlist,
                StbConfig::new()
                    .with_probe("VP")
                    .with_sweep_type(StbSweepType::Linear)
                    .with_sweep(10.0, 1000.0, 3),
            )
            .unwrap();
        for (&frequency, &actual) in result.frequencies.iter().zip(&result.loop_gains) {
            let jw = Complex64::new(0.0, std::f64::consts::TAU * frequency);
            let expected = if zero {
                1000.0 * jw * 1e-3 / (1.0 + jw * 1e-3)
            } else {
                1000.0 / jw
            };
            assert!((actual - expected).norm() <= expected.norm() * 1e-10);
        }
        if zero {
            assert_eq!(
                result.result.margins.dc_loop_gain,
                Some(Complex64::new(0.0, 0.0))
            );
            assert_eq!(result.result.margins.dc_gain_db(), Some(f64::NEG_INFINITY));
        } else {
            assert_eq!(result.result.margins.dc_loop_gain, None);
            assert_eq!(result.result.margins.dc_gain_db(), None);
            assert!(
                result
                    .result
                    .warnings
                    .iter()
                    .any(|warning| warning.contains("DC loop gain"))
            );
        }
        let original = document(&result.result);
        let decoded = AnalysisResultDocument::from_json(&original.to_json().unwrap()).unwrap();
        assert_eq!(original, decoded);
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn positive_frequency_samples_do_not_claim_a_dc_measurement() {
    let result = StbAnalyzer::new(StbConfig::default())
        .analyze(
            &[1e5, 1e6],
            &[Complex64::new(10.0, -1.0), Complex64::new(1.0, -1.0)],
        )
        .unwrap();
    assert_eq!(result.margins.dc_loop_gain, None);
    assert_eq!(result.margins.dc_gain_db(), None);
    let doc = document(&result);
    assert_eq!(
        AnalysisResultDocument::from_json(&doc.to_json().unwrap()).unwrap(),
        doc
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn saved_dc_evidence_is_validated_and_legacy_first_samples_are_relabelled() {
    let netlist = Netlist::parse(SINGLE_POLE).unwrap();
    let result = Engine::default()
        .run_stb(&netlist, StbConfig::new().with_probe("VP"))
        .unwrap();
    let json = document(&result.result).to_json().unwrap();
    let original: serde_json::Value = serde_json::from_str(&json).unwrap();
    for version in 1..=13 {
        let mut legacy = original.clone();
        legacy["schemaVersion"] = version.into();
        let scalars = legacy["scalars"].as_array_mut().unwrap();
        scalars.retain(|scalar| scalar["name"] != "dc_loop_gain");
        let db = scalars
            .iter_mut()
            .find(|scalar| scalar["name"] == "dc_loop_gain_db")
            .unwrap();
        db["value"]["value"] = 20.0.into(); // historical first AC sample
        let decoded = AnalysisResultDocument::from_json(&legacy.to_string()).unwrap();
        let normalized: serde_json::Value =
            serde_json::from_str(&decoded.to_json().unwrap()).unwrap();
        let scalars = normalized["scalars"].as_array().unwrap();
        assert!(
            !scalars
                .iter()
                .any(|scalar| scalar["name"] == "dc_loop_gain_db")
        );
        assert_eq!(
            scalars
                .iter()
                .find(|scalar| scalar["name"] == "sweep_start_gain_db")
                .unwrap()["value"]["value"],
            20.0
        );
        assert_eq!(
            AnalysisResultDocument::from_json(&decoded.to_json().unwrap()).unwrap(),
            decoded
        );
    }
    for change in 0..3 {
        let mut invalid = original.clone();
        let scalars = invalid["scalars"].as_array_mut().unwrap();
        match change {
            0 => scalars.retain(|scalar| scalar["name"] != "dc_loop_gain"),
            1 => {
                scalars
                    .iter_mut()
                    .find(|scalar| scalar["name"] == "dc_loop_gain")
                    .unwrap()["value"]["value"]["real"] = 2.0.into()
            }
            _ => {
                scalars
                    .iter_mut()
                    .find(|scalar| scalar["name"] == "dc_loop_gain_db")
                    .unwrap()["unit"] = serde_json::json!({"unit":"volt"})
            }
        }
        assert!(AnalysisResultDocument::from_json(&invalid.to_string()).is_err());
    }
}

//! A stability document must retain one coherent measured response.
use super::*;
use serde_json::json;

#[test]
fn stability_import_rejects_inconsistent_bode_nyquist_and_margin_evidence() {
    let valid = document_for(AnalysisResultKind::Stability);
    let source: serde_json::Value = serde_json::from_str(&valid.to_json().unwrap()).unwrap();
    for case in [
        "negative_frequency",
        "duplicate_frequency",
        "wrong_axis_unit",
        "wrong_magnitude_unit",
        "nyquist_coordinate",
        "nyquist_value",
        "duplicate_nyquist",
        "magnitude",
        "db",
        "phase",
        "phase_wrap",
        "gain_margin",
        "phase_margin",
        "crossovers",
    ] {
        let mut document = source.clone();
        match case {
            "negative_frequency" => document["axes"][0]["values"]["values"][0] = json!(-1),
            "duplicate_frequency" => document["axes"][0]["values"]["values"][1] = json!(1000),
            "wrong_axis_unit" => document["axes"][0]["unit"] = json!({"unit":"volt"}),
            "wrong_magnitude_unit" => {
                document["signals"][1]["descriptor"]["unit"] = json!({"unit":"volt"})
            }
            "nyquist_coordinate" => document["payload"]["nyquist"][0]["frequency"] = json!(2000),
            "nyquist_value" => document["payload"]["nyquist"][0]["real"] = json!(-10),
            "duplicate_nyquist" => {
                let sample = document["payload"]["nyquist"][0].clone();
                document["payload"]["nyquist"]
                    .as_array_mut()
                    .unwrap()
                    .push(sample);
            }
            "magnitude" => document["signals"][1]["values"]["samples"][0] = json!(100),
            "db" => document["signals"][2]["values"]["samples"][0] = json!(300),
            "phase" => document["signals"][3]["values"]["samples"][0] = json!(30),
            "phase_wrap" => document["signals"][3]["values"]["samples"][2] = json!(180),
            "gain_margin" | "phase_margin" => {
                let name = if case == "gain_margin" {
                    "gain_margin_db"
                } else {
                    "phase_margin_degrees"
                };
                let scalar = document["scalars"]
                    .as_array_mut()
                    .unwrap()
                    .iter_mut()
                    .find(|scalar| scalar["name"] == name)
                    .unwrap();
                scalar["value"]["value"] = json!(123);
            }
            "crossovers" => {
                for scalar in document["scalars"].as_array_mut().unwrap() {
                    match scalar["name"].as_str().unwrap() {
                        "unity_gain_crossovers" => scalar["value"]["value"] = json!(2),
                        "multiple_unity_gain_crossovers" => scalar["value"]["value"] = json!(true),
                        _ => {}
                    }
                }
            }
            _ => panic!("unknown case"),
        }
        assert!(
            AnalysisResultDocument::from_json(&document.to_string()).is_err(),
            "accepted {case}"
        );
    }
}

#[test]
fn stability_projection_preserves_missing_samples_without_inventing_an_unwrap_offset() {
    let mut document = document_for(AnalysisResultKind::Stability);
    let SeriesValues::Complex { samples } = &mut document.signals[0].values else {
        panic!("complex gain")
    };
    samples[1] = None;
    // The final phase is unwrapped using an omitted source sample. Its direction
    // remains verifiable, while the full margin calculation is no longer held.
    let json = document.to_json().unwrap();
    assert_eq!(AnalysisResultDocument::from_json(&json).unwrap(), document);
    let SeriesValues::Real { samples } = &mut document.signals[3].values else {
        panic!("phase")
    };
    samples[2] = Some(90.0);
    assert!(document.validate().is_err());
}

#[test]
fn retained_nyquist_samples_qualify_margins_when_the_complex_series_is_omitted() {
    let mut result = stability_result();
    result.nyquist_points = result
        .bode_points
        .iter()
        .map(|point| NyquistPoint::from_loop_gain(point.loop_gain, point.frequency))
        .collect();
    let mut document = AnalysisResultDocument::from_stability(instance(AnalysisKind::Stb), &result)
        .unwrap()
        .build()
        .unwrap();
    document.signals.clear();
    document.validate().unwrap();
    let margin = document
        .scalars
        .iter_mut()
        .find(|scalar| scalar.name == "phase_margin_degrees")
        .unwrap();
    margin.value = ScalarValue::Real { value: Some(123.0) };
    assert!(document.validate().is_err());
}

#[test]
fn zero_gain_cannot_publish_a_finite_decibel_magnitude_or_phase() {
    let result = crate::analysis::stb::StbAnalyzer::new(Default::default())
        .analyze(&[1.0, 2.0, 3.0], &[Complex64::new(0.0, 0.0); 3])
        .unwrap();
    let valid = AnalysisResultDocument::from_stability(instance(AnalysisKind::Stb), &result)
        .unwrap()
        .build()
        .unwrap();
    for name in ["loop_gain_db", "loop_gain_phase"] {
        let mut document = valid.clone();
        let signal = document
            .signals
            .iter_mut()
            .find(|signal| signal.descriptor.canonical_name() == name)
            .unwrap();
        let SeriesValues::Real { samples } = &mut signal.values else {
            panic!("real projection")
        };
        samples[0] = Some(0.0);
        assert!(document.validate().is_err(), "accepted a fabricated {name}");
    }
}

#[test]
fn stability_curve_validation_observes_cancellation() {
    let frequencies: Vec<_> = (1..=ABORT_POLL_STRIDE * 8)
        .map(|index| index as f64)
        .collect();
    let result = crate::analysis::stb::StbAnalyzer::new(Default::default())
        .analyze(
            &frequencies,
            &vec![Complex64::new(0.5, 0.0); frequencies.len()],
        )
        .unwrap();
    let document = AnalysisResultDocument::from_stability(instance(AnalysisKind::Stb), &result)
        .unwrap()
        .build()
        .unwrap();
    for polls in [10, 20, 30] {
        let abort = CountingAbort::new(polls);
        assert!(matches!(
            super::super::stability::validate(&document, &abort),
            Err(ResultDocumentError::Aborted)
        ));
        assert_eq!(abort.polls_after_abort(), 0);
    }
}

#[test]
fn stability_curve_validation_preserves_finite_components_across_binary64_scales() {
    let result = crate::analysis::stb::StbAnalyzer::new(Default::default())
        .analyze(
            &[1.0, 2.0, 3.0],
            &[
                Complex64::new(f64::from_bits(1), 0.0),
                Complex64::new(f64::MAX, f64::MAX),
                Complex64::new(0.0, 0.0),
            ],
        )
        .unwrap();
    let document = AnalysisResultDocument::from_stability(instance(AnalysisKind::Stb), &result)
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(
        AnalysisResultDocument::from_json(&document.to_json().unwrap()).unwrap(),
        document
    );
}

use super::*;
use serde_json::{Value, json};

fn wire() -> Value {
    serde_json::from_str(
        &document_for(AnalysisResultKind::Distortion)
            .to_json()
            .unwrap(),
    )
    .unwrap()
}

#[test]
fn distortion_import_refuses_second_tone_underflow() {
    let mut document = wire();
    document["axes"][0]["values"]["values"] = json!([1e-200]);
    document["payload"]["f2OverF1"] = json!(1e-200);
    assert!(AnalysisResultDocument::from_json(&document.to_string()).is_err());
}

#[test]
fn distortion_import_refuses_invalid_fundamental_geometry() {
    let original = wire();
    for ratio in [-0.8, 0.0, 1.0, 2.0] {
        let mut document = original.clone();
        document["payload"]["f2OverF1"] = json!(ratio);
        assert!(
            AnalysisResultDocument::from_json(&document.to_string()).is_err(),
            "ratio {ratio}"
        );
    }
    for defect in [
        "negative", "zero", "overflow", "unit", "kind", "axis", "integer",
    ] {
        let mut document = original.clone();
        match defect {
            "negative" => document["axes"][0]["values"]["values"] = json!([-1000.0]),
            "zero" => document["axes"][0]["values"]["values"] = json!([0.0]),
            "overflow" => document["axes"][0]["values"]["values"] = json!([f64::MAX]),
            "unit" => document["axes"][0]["unit"] = json!({"unit": "dimensionless"}),
            "kind" => document["axes"][0]["kind"] = json!("index"),
            "axis" => document["axes"] = json!([]),
            "integer" => {
                document["axes"][0]["values"] =
                    json!({"representation": "integer", "values": [1000]})
            }
            _ => panic!("unknown frequency defect: {defect}"),
        }
        assert!(
            AnalysisResultDocument::from_json(&document.to_string()).is_err(),
            "{defect}"
        );
    }
}

#[test]
fn distortion_import_refuses_inconsistent_product_metadata() {
    let original = wire();
    for defect in [
        "frequency",
        "negative",
        "length",
        "duplicate",
        "mode",
        "qualifier",
        "order",
        "unsupported",
    ] {
        let mut document = original.clone();
        match defect {
            "frequency" => document["payload"]["products"][0]["frequencies"] = json!([1234.0]),
            "negative" => document["payload"]["products"][0]["frequencies"] = json!([-1100.0]),
            "length" => document["payload"]["products"][0]["frequencies"] = json!([]),
            "duplicate" => {
                let products = document["payload"]["products"].as_array_mut().unwrap();
                products.push(products[0].clone());
            }
            "mode" => document["payload"]["f2OverF1"] = Value::Null,
            "qualifier" => {
                document["signals"][0]["qualifier"] =
                    json!({"kind": "distortion-product", "product": "sum"});
            }
            "order" => document["payload"]["products"][0]["order"] = json!(2),
            "unsupported" => {
                document["payload"]["products"][0]["product"] = json!("third-harmonic")
            }
            _ => panic!("unknown product defect: {defect}"),
        }
        assert!(
            AnalysisResultDocument::from_json(&document.to_string()).is_err(),
            "{defect}"
        );
    }
}

#[test]
fn distortion_validation_uses_fixed_second_tone_throughout_sweep() {
    let mut source = distortion_result();
    let mut second = source.points[0].clone();
    second.fundamental_f1.frequency = 2000.0;
    second.products[0].response.frequency = 3100.0;
    source.points.push(second);
    let document =
        AnalysisResultDocument::from_distortion(instance(AnalysisKind::Distortion), &source)
            .unwrap()
            .build()
            .unwrap();
    assert_eq!(
        AnalysisResultDocument::from_json(&document.to_json().unwrap()).unwrap(),
        document
    );

    source.points[1].fundamental_f2.as_mut().unwrap().frequency = 1800.0;
    assert!(
        AnalysisResultDocument::from_distortion(instance(AnalysisKind::Distortion), &source)
            .is_err()
    );
    source.points[1].fundamental_f2.as_mut().unwrap().frequency = 900.0;
    source.points[1].fundamental_f1.frequency = 800.0;
    source.points[1].products[0].response.frequency = 700.0;
    assert!(
        AnalysisResultDocument::from_distortion(instance(AnalysisKind::Distortion), &source)
            .unwrap()
            .build()
            .is_err()
    );
}

#[test]
fn distortion_validation_cancels_within_product_frequency_scan() {
    let mut document = document_for(AnalysisResultKind::Distortion);
    let points = ABORT_POLL_STRIDE * 8;
    document.point_count = points;
    document.axes[0].values = AxisValues::Real {
        values: vec![1000.0; points],
    };
    document.signals.clear();
    let ResultPayload::Distortion(payload) = &mut document.payload else {
        panic!("expected a distortion payload")
    };
    payload.products[0].frequencies = vec![1100.0; points];
    // Entry, eight F1 polls, product entry, then the first two product rows.
    let abort = CountingAbort::new(11);
    assert!(matches!(
        super::super::distortion::validate(&document, &abort),
        Err(ResultDocumentError::Aborted)
    ));
    assert_eq!(abort.observed_at(), Some(12));
    assert_eq!(abort.polls_after_abort(), 0);
    super::super::distortion::validate(&document, &NoAbort).unwrap();
}

#[test]
fn distortion_import_preserves_subnormal_tones_and_real_dc_products() {
    for (f1, ratio, product, frequency) in [
        (1e-323, 0.5, "third-order-difference", 1.5e-323),
        (1000.0, 0.5, "third-order-difference-f2", 0.0),
        (1000.0, 0.25, "third-order-difference-f2", 500.0),
    ] {
        let mut document = wire();
        document["axes"][0]["values"]["values"] = json!([f1]);
        document["payload"]["f2OverF1"] = json!(ratio);
        document["payload"]["products"][0]["product"] = json!(product);
        document["payload"]["products"][0]["frequencies"] = json!([frequency]);
        for signal in document["signals"].as_array_mut().unwrap() {
            if signal["qualifier"]["kind"] == "distortion-product" {
                signal["qualifier"]["product"] = json!(product);
            }
        }
        let decoded = AnalysisResultDocument::from_json(&document.to_string()).unwrap();
        let ResultPayload::Distortion(payload) = decoded.payload() else {
            panic!("expected a distortion payload")
        };
        assert_eq!(payload.products[0].frequencies, [frequency]);
        assert_eq!(
            AnalysisResultDocument::from_json(&decoded.to_json().unwrap()).unwrap(),
            decoded
        );
    }
}

#[test]
fn distortion_projection_refuses_to_discard_conflicting_second_tone_evidence() {
    for defect in ["frequency", "missing", "mode"] {
        let mut source = distortion_result();
        match defect {
            "frequency" => source.points[0].fundamental_f2.as_mut().unwrap().frequency = 901.0,
            "missing" => source.points[0].fundamental_f2 = None,
            "mode" => source.f2_over_f1 = None,
            _ => panic!("unknown source defect: {defect}"),
        }
        let result =
            AnalysisResultDocument::from_distortion(instance(AnalysisKind::Distortion), &source)
                .and_then(AnalysisResultDocumentBuilder::build);
        assert!(result.is_err(), "{defect}");
    }
}

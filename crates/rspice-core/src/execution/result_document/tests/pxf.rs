use super::*;
use serde_json::{Value, json};

fn wire() -> Value {
    serde_json::from_str(&document_for(AnalysisResultKind::Pxf).to_json().unwrap()).unwrap()
}

#[test]
fn pxf_import_rejects_inconsistent_conversion_geometry() {
    let original = wire();
    for defect in [
        "carrier",
        "zero_carrier",
        "output",
        "qualifier",
        "foreign_qualifier",
        "minimum_sideband",
        "negative_depth",
        "offset",
        "zero_offset",
        "order",
        "duplicate_offset",
        "axis",
        "kind",
        "unit",
        "integer",
        "output_unit",
        "delay_frequency",
        "delay_order",
        "delay_duplicate",
        "overflow",
        "empty",
    ] {
        let mut value = original.clone();
        match defect {
            "carrier" => value["payload"]["fundamentalFrequency"] = json!(-1e6),
            "zero_carrier" => value["payload"]["fundamentalFrequency"] = json!(0.0),
            "output" => value["signals"][1]["values"]["samples"][0] = json!(1234.0),
            "qualifier" => value["signals"][0]["qualifier"]["input"] = json!(0),
            "foreign_qualifier" => {
                value["signals"][0]["qualifier"] = json!({"kind": "pac-sideband", "sideband": 1})
            }
            "minimum_sideband" => {
                value["payload"]["inputSideband"] = json!(i32::MIN);
                value["payload"]["maxSideband"] = json!(i32::MAX);
                value["signals"][0]["qualifier"]["input"] = json!(i32::MIN);
            }
            "negative_depth" => value["payload"]["maxSideband"] = json!(-1),
            "offset" => value["axes"][0]["values"]["values"][0] = json!(-1.0),
            "zero_offset" => value["axes"][0]["values"]["values"][0] = json!(0.0),
            "order" => value["axes"][0]["values"]["values"]
                .as_array_mut()
                .unwrap()
                .swap(0, 1),
            "duplicate_offset" => value["axes"][0]["values"]["values"][1] = json!(1000.0),
            "axis" => value["axes"] = json!([]),
            "kind" => value["axes"][0]["kind"] = json!("frequency"),
            "unit" => value["axes"][0]["unit"] = json!({"unit": "dimensionless"}),
            "integer" => {
                value["axes"][0]["values"] =
                    json!({"representation": "integer", "values": [1000,2000,3000]})
            }
            "output_unit" => {
                value["signals"][1]["descriptor"]["unit"] = json!({"unit": "dimensionless"})
            }
            "delay_frequency" => value["payload"]["groupDelay"][0]["frequency"] = json!(1234.0),
            "delay_order" => value["payload"]["groupDelay"]
                .as_array_mut()
                .unwrap()
                .swap(0, 1),
            "delay_duplicate" => {
                let delays = value["payload"]["groupDelay"].as_array_mut().unwrap();
                delays.push(delays[1].clone());
            }
            "overflow" => {
                value["payload"]["inputSideband"] = json!(2);
                value["payload"]["maxSideband"] = json!(2);
                value["payload"]["fundamentalFrequency"] = json!(f64::MAX);
                value["signals"][0]["qualifier"]["input"] = json!(2);
            }
            "empty" => {
                value["pointCount"] = json!(0);
                value["axes"][0]["values"]["values"] = json!([]);
                value["signals"] = json!([]);
                value["payload"]["groupDelay"] = json!([]);
            }
            _ => panic!("unknown defect: {defect}"),
        }
        assert!(
            AnalysisResultDocument::from_json(&value.to_string()).is_err(),
            "{defect}"
        );
    }
}

#[test]
fn pxf_projection_refuses_to_relabel_source_transfer_points() {
    for input in [false, true] {
        let (card, mut source) = pxf_measurement();
        if input {
            source.points[1].sideband_in = 0;
        } else {
            source.points[1].sideband_out = 1;
        }
        let error = AnalysisResultDocument::from_pxf(instance(AnalysisKind::Pxf), &card, &source)
            .unwrap_err();
        assert!(error.to_string().contains("transfer point 1"), "{error}");
    }
}

#[test]
fn pxf_projection_preserves_signed_dc_and_fused_conversion_frequencies() {
    for (carrier, output, offsets) in [
        (1000.0, -1, [100.0, 1000.0, 1100.0]),
        (0.1, -3, [0.1, 0.3, 0.5]),
    ] {
        let (mut card, mut source) = pxf_measurement();
        card.output_sideband = output;
        card.max_sideband = 3;
        source.output_sideband = output;
        source.fundamental_freq = carrier;
        for (point, offset) in source.points.iter_mut().zip(offsets) {
            point.freq_in = offset;
            point.freq_out = f64::from(output).mul_add(carrier, offset);
            point.sideband_out = output;
        }
        source.compute_metrics();
        let document =
            AnalysisResultDocument::from_pxf(instance(AnalysisKind::Pxf), &card, &source)
                .unwrap()
                .build()
                .unwrap();
        let decoded = AnalysisResultDocument::from_json(&document.to_json().unwrap()).unwrap();
        assert_eq!(decoded, document);
        assert!(source.points[0].freq_out < 0.0);
        if output == -1 {
            assert_eq!(source.points[1].freq_out, 0.0);
        } else {
            assert_ne!(
                source.points[1].freq_out,
                f64::from(output) * carrier + offsets[1]
            );
        }
    }
}

#[test]
fn pxf_import_preserves_projected_missing_samples_and_midpoints() {
    let mut value = wire();
    value["signals"][0]["values"]["samples"][0] = Value::Null;
    value["signals"][1]["values"]["samples"][1] = Value::Null;
    value["payload"]["groupDelay"]
        .as_array_mut()
        .unwrap()
        .remove(0);
    let document = AnalysisResultDocument::from_json(&value.to_string()).unwrap();
    let encoded: Value = serde_json::from_str(&document.to_json().unwrap()).unwrap();
    assert_eq!(encoded, value);
    value["signals"] = json!([]);
    value["payload"]["groupDelay"] = json!([]);
    AnalysisResultDocument::from_json(&value.to_string()).unwrap();
}

#[test]
fn pxf_validation_cancels_during_group_delay_scan() {
    let mut document = document_for(AnalysisResultKind::Pxf);
    let offsets = (1..=ABORT_POLL_STRIDE * 8)
        .map(|i| i as f64)
        .collect::<Vec<_>>();
    document.point_count = offsets.len();
    document.signals.clear();
    let ResultPayload::Pxf(payload) = &mut document.payload else {
        panic!("expected PXF")
    };
    payload.group_delay = offsets
        .windows(2)
        .map(|pair| PxfGroupDelaySample {
            frequency: pair[0].midpoint(pair[1]),
            delay: 0.0,
        })
        .collect();
    document.axes[0].values = AxisValues::Real { values: offsets };
    super::super::pxf::validate(&document, &NoAbort).unwrap();
    // One initial poll and eight offset blocks precede the midpoint scan.
    let abort = CountingAbort::new(10);
    assert!(matches!(
        super::super::pxf::validate(&document, &abort),
        Err(ResultDocumentError::Aborted)
    ));
    assert_eq!(abort.observed_at(), Some(11));
    assert_eq!(abort.polls_after_abort(), 0);
}

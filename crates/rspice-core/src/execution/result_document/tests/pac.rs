use super::*;
use serde_json::{Value, json};

fn wire() -> Value {
    serde_json::from_str(&document_for(AnalysisResultKind::Pac).to_json().unwrap()).unwrap()
}

#[test]
fn pac_import_rejects_inconsistent_frequency_geometry() {
    let original = wire();
    for defect in [
        "absolute",
        "offset",
        "length",
        "duplicate",
        "order",
        "carrier",
        "zero_carrier",
        "negative_offset",
        "unit",
        "kind",
        "integer",
        "axis",
        "empty",
        "qualifier",
        "overflow",
        "residual",
    ] {
        let mut document = original.clone();
        match defect {
            "absolute" => {
                document["payload"]["sidebands"][0]["absoluteFrequencies"][0] = json!(1234.0)
            }
            "offset" => document["payload"]["sidebands"][0]["frequencyOffsets"][0] = json!(1234.0),
            "length" => {
                document["payload"]["sidebands"][0]["frequencyOffsets"]
                    .as_array_mut()
                    .unwrap()
                    .pop();
                document["payload"]["sidebands"][0]["absoluteFrequencies"]
                    .as_array_mut()
                    .unwrap()
                    .pop();
            }
            "duplicate" => {
                let bands = document["payload"]["sidebands"].as_array_mut().unwrap();
                bands.push(bands[0].clone());
            }
            "order" => document["payload"]["sidebands"]
                .as_array_mut()
                .unwrap()
                .swap(0, 1),
            "carrier" => document["payload"]["fundamentalFrequency"] = json!(-1e6),
            "zero_carrier" => document["payload"]["fundamentalFrequency"] = json!(0.0),
            "negative_offset" => document["axes"][0]["values"]["values"][0] = json!(-1.0),
            "unit" => document["axes"][0]["unit"] = json!({"unit": "dimensionless"}),
            "kind" => document["axes"][0]["kind"] = json!("frequency"),
            "integer" => {
                document["axes"][0]["values"] =
                    json!({"representation": "integer", "values": [1000, 10000]})
            }
            "axis" => document["axes"] = json!([]),
            "empty" => {
                document["payload"]["sidebands"] = json!([]);
                document["signals"] = json!([]);
            }
            "qualifier" => {
                document["signals"][0]["qualifier"] =
                    json!({"kind": "pac-sideband", "sideband": 99})
            }
            "overflow" => {
                document["payload"]["sidebandMaximum"] = json!(i32::MAX);
                document["payload"]["fundamentalFrequency"] = json!(f64::MAX);
            }
            "residual" => document["payload"]["residual"] = json!(-1.0),
            _ => panic!("unknown defect: {defect}"),
        }
        assert!(
            AnalysisResultDocument::from_json(&document.to_string()).is_err(),
            "{defect}"
        );
    }
}

#[test]
fn pac_import_rejects_ambiguous_or_out_of_range_conversion_entries() {
    for defect in ["duplicate", "frequency", "input", "output"] {
        let mut document = wire();
        let entries = document["payload"]["conversionMatrix"]["entries"]
            .as_array_mut()
            .unwrap();
        match defect {
            "duplicate" => entries.push(entries[0].clone()),
            "frequency" => entries[0]["frequencyIndex"] = json!(2),
            "input" => entries[0]["inputSideband"] = json!(-2),
            "output" => entries[0]["outputSideband"] = json!(2),
            _ => panic!("unknown defect: {defect}"),
        }
        assert!(
            AnalysisResultDocument::from_json(&document.to_string()).is_err(),
            "{defect}"
        );
    }
}

#[test]
fn pac_projection_refuses_to_discard_source_coordinate_evidence() {
    use crate::analysis::pac::ConversionMatrix;

    for defect in [
        "identity",
        "matrix_carrier",
        "matrix_offsets",
        "matrix_range",
        "unbounded_range",
    ] {
        let mut source = pac_result();
        match defect {
            "identity" => source.get_sideband_data_mut(0, -1).unwrap().sideband = 0,
            "matrix_carrier" => {
                source.conversion_matrix =
                    ConversionMatrix::new(2e6, -1, 1, source.frequencies.clone()).unwrap()
            }
            "matrix_offsets" => {
                source.conversion_matrix =
                    ConversionMatrix::new(1e6, -1, 1, vec![2e3, 1e4]).unwrap()
            }
            "matrix_range" => {
                source.conversion_matrix =
                    ConversionMatrix::new(1e6, 0, 0, source.frequencies.clone()).unwrap()
            }
            "unbounded_range" => {
                source.sideband_min = i32::MIN;
                source.sideband_max = i32::MAX;
            }
            _ => panic!("unknown defect: {defect}"),
        }
        let result = AnalysisResultDocument::from_pac(instance(AnalysisKind::Pac), &source)
            .and_then(AnalysisResultDocumentBuilder::build);
        assert!(result.is_err(), "{defect}");
    }
}

#[test]
fn pac_validation_cancels_during_sparse_matrix_scan() {
    let mut document = document_for(AnalysisResultKind::Pac);
    let points = ABORT_POLL_STRIDE * 8;
    document.point_count = points;
    document.axes[0].values = AxisValues::Real {
        values: vec![1000.0; points],
    };
    document.signals.clear();
    let ResultPayload::Pac(payload) = &mut document.payload else {
        panic!("expected PAC")
    };
    payload.sidebands.truncate(1);
    payload.sidebands[0].frequency_offsets = vec![1000.0; points];
    payload.sidebands[0].absolute_frequencies = vec![-999000.0; points];
    payload.conversion_matrix = Some(PacConversionMatrixDocument {
        entries: (0..points)
            .map(|frequency_index| PacConversionEntry {
                frequency_index,
                input_sideband: -1,
                output_sideband: -1,
                value: ComplexSample::new(1.0, 0.0),
            })
            .collect(),
    });
    super::super::pac::validate(&document, &NoAbort).unwrap();
    // Past the carrier and sideband scans, interrupt the matrix's second block.
    let abort = CountingAbort::new(20);
    assert!(matches!(
        super::super::pac::validate(&document, &abort),
        Err(ResultDocumentError::Aborted)
    ));
    assert_eq!(abort.observed_at(), Some(21));
    assert_eq!(abort.polls_after_abort(), 0);
}

#[test]
fn pac_validation_refuses_nonfinite_conversion_values() {
    for imaginary in [false, true] {
        let mut document = document_for(AnalysisResultKind::Pac);
        let ResultPayload::Pac(payload) = &mut document.payload else {
            panic!("expected PAC")
        };
        let value = &mut payload.conversion_matrix.as_mut().unwrap().entries[0].value;
        if imaginary {
            value.imaginary = f64::INFINITY;
        } else {
            value.real = f64::NAN;
        }
        assert!(document.validate().is_err());
    }
}

#[test]
fn pac_import_preserves_signed_dc_and_fused_sideband_frequencies() {
    for (carrier, offsets, minimum) in [(1000.0, vec![0.0, 1000.0], -1), (0.1, vec![0.3, 0.0], -3)]
    {
        let source = crate::analysis::PacResult::new(
            carrier,
            offsets,
            minimum,
            1,
            vec!["out".into()],
            vec![],
        )
        .unwrap();
        let document = AnalysisResultDocument::from_pac(instance(AnalysisKind::Pac), &source)
            .unwrap()
            .build()
            .unwrap();
        let decoded = AnalysisResultDocument::from_json(&document.to_json().unwrap()).unwrap();
        assert_eq!(decoded, document);
        let ResultPayload::Pac(payload) = decoded.payload() else {
            panic!("expected PAC")
        };
        assert!(payload.sidebands[0].absolute_frequencies[0] < 0.0);
        if carrier == 1000.0 {
            assert_eq!(payload.sidebands[0].absolute_frequencies[1], 0.0);
        } else {
            let expected = (-3.0_f64).mul_add(carrier, 0.3);
            assert_eq!(payload.sidebands[0].absolute_frequencies[0], expected);
            assert_ne!(expected, -3.0 * carrier + 0.3);
        }
    }
}

#[test]
fn pac_import_keeps_sparse_conversion_paths_when_zero_sideband_is_not_reported() {
    let mut source = pac_result();
    source.include_dc = false;
    let document = AnalysisResultDocument::from_pac(instance(AnalysisKind::Pac), &source)
        .unwrap()
        .build()
        .unwrap();
    let mut value: Value = serde_json::from_str(&document.to_json().unwrap()).unwrap();
    value["payload"]["conversionMatrix"]["entries"]
        .as_array_mut()
        .unwrap()
        .retain(|entry| entry["frequencyIndex"] == 0);
    let decoded = AnalysisResultDocument::from_json(&value.to_string()).unwrap();
    let ResultPayload::Pac(payload) = decoded.payload() else {
        panic!("expected PAC")
    };
    assert!(!payload.sidebands.iter().any(|band| band.sideband == 0));
    assert!(
        payload
            .conversion_matrix
            .as_ref()
            .unwrap()
            .entries
            .iter()
            .any(|entry| entry.output_sideband == 0)
    );
    assert_eq!(
        AnalysisResultDocument::from_json(&decoded.to_json().unwrap()).unwrap(),
        decoded
    );
}

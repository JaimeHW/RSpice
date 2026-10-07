//! Table documents preserve physical row identity through serialization/windows.
use std::sync::atomic::{AtomicUsize, Ordering};

use rspice_core::analysis::{AcResult, NoiseResult};
use rspice_core::engine::{FrequencyDataResult, FrequencyDataTarget};
use rspice_core::execution::result_document::{
    AnalysisResultDocument, AxisValues, FrequencyTableMetadata, ResultDocumentError, ResultPayload,
    SeriesValues,
};
use rspice_core::execution::{AnalysisInstanceId, AnalysisKind, SignalUnit};
use rspice_core::{
    AbortSignal, Engine, ModelFinish, ModelFinishPoint, Netlist, NoAbort, ResourceKind,
    ResourceLimits, SimulationConfig,
};

const DECK: &str = "Table documents\n.param resistance=1k\nV1 in 0 AC 1\nR1 in out {resistance} tc1=.01 tnom=27\nC1 out 0 1u\n.data Mixed resistance HERTZ TEMP C1:CAP\n1k 100 27 1u\n2k 10 127 2u\n3k 100 77 3u\n.enddata\n.end\n";

fn results() -> (
    FrequencyDataResult<AcResult>,
    FrequencyDataResult<NoiseResult>,
) {
    let netlist = Netlist::parse(DECK).unwrap();
    let engine = Engine::new(SimulationConfig::default());
    (
        engine.run_ac_table(&netlist, "Mixed").unwrap(),
        engine
            .run_noise_table_named_with_input_source(&netlist, "out", None, "V1", "Mixed", 300.15)
            .unwrap(),
    )
}

fn identity(kind: AnalysisKind) -> AnalysisInstanceId {
    AnalysisInstanceId::new(kind, 0)
}

fn metadata(document: &AnalysisResultDocument) -> &FrequencyTableMetadata {
    document.frequency_table().unwrap()
}

fn close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < expected.abs() * 1e-9 + 1e-30,
        "{actual:e} != {expected:e}"
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn physical_coordinates_complex_signals_and_noise_round_trip_with_windows() {
    let (ac, noise) = results();
    let ac_doc = AnalysisResultDocument::from_ac_table(identity(AnalysisKind::Ac), &ac)
        .unwrap()
        .build()
        .unwrap();
    let noise_doc = AnalysisResultDocument::from_noise_table(identity(AnalysisKind::Noise), &noise)
        .unwrap()
        .build()
        .unwrap();
    for document in [&ac_doc, &noise_doc] {
        assert_eq!(document.schema_version(), 11);
        let table = metadata(document);
        assert_eq!(table.table_name, "Mixed");
        assert_eq!(table.requested_rows, 3);
        assert_eq!(table.finish, None);
        assert_eq!(
            table
                .columns
                .iter()
                .map(|column| column.name.as_str())
                .collect::<Vec<_>>(),
            ["resistance", "HERTZ", "TEMP", "C1:CAP"]
        );
        assert_eq!(
            table.columns[0].target,
            FrequencyDataTarget::Parameter("RESISTANCE".into())
        );
        assert_eq!(
            table.columns[3].target,
            FrequencyDataTarget::DeviceParameter {
                device_name: "C1".into(),
                parameter_name: "C".into()
            }
        );
        assert_eq!(document.axes().len(), 4);
        let window = document.window(1, 2).unwrap();
        for (column, source) in table.columns.iter().zip(&ac.columns) {
            let axis = document
                .axes()
                .iter()
                .find(|axis| axis.name() == column.axis)
                .unwrap();
            if column.target != FrequencyDataTarget::Frequency {
                assert_eq!(axis.display_name(), column.name);
            }
            assert_eq!(
                axis.values(),
                &AxisValues::Real {
                    values: source.values.clone()
                }
            );
            let expected_unit = match column.target {
                FrequencyDataTarget::Frequency => SignalUnit::Hertz,
                FrequencyDataTarget::Parameter(ref name) if name == "TEMP" => {
                    SignalUnit::Custom("degC".into())
                }
                _ => SignalUnit::Unspecified,
            };
            assert_eq!(axis.unit(), &expected_unit);
            let sliced = window
                .axes
                .iter()
                .find(|axis| axis.name == column.axis)
                .unwrap();
            assert_eq!(
                sliced.values,
                AxisValues::Real {
                    values: source.values[1..].to_vec()
                }
            );
        }
        let encoded = document.to_json().unwrap();
        assert_eq!(
            AnalysisResultDocument::from_json(&encoded).unwrap(),
            *document
        );
    }
    let out = ac_doc
        .signals()
        .iter()
        .find(|signal| {
            signal
                .descriptor()
                .canonical_name()
                .eq_ignore_ascii_case("v(out)")
        })
        .unwrap();
    let SeriesValues::Complex { samples } = out.values() else {
        panic!("complex output")
    };
    let density = noise_doc
        .signals()
        .iter()
        .find(|signal| signal.descriptor().canonical_name() == "inoise_spectrum")
        .unwrap();
    let SeriesValues::Real { samples: densities } = density.values() else {
        panic!("noise density")
    };
    for row in 0..3 {
        let temperature = ac.columns[2].values[row];
        let resistance = ac.columns[0].values[row] * (1.0 + 0.01 * (temperature - 27.0));
        let wrc = std::f64::consts::TAU
            * ac.columns[1].values[row]
            * resistance
            * ac.columns[3].values[row];
        let sample = samples[row].as_ref().unwrap();
        close(sample.real, 1.0 / (1.0 + wrc * wrc));
        close(sample.imaginary, -wrc / (1.0 + wrc * wrc));
        close(
            densities[row].unwrap(),
            4.0 * 1.380649e-23 * (temperature + 273.15) * resistance,
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn old_documents_have_no_invented_table_and_cannot_claim_new_evidence() {
    let (ac, noise) = results();
    for document in [
        AnalysisResultDocument::from_ac(identity(AnalysisKind::Ac), &ac.points)
            .unwrap()
            .build()
            .unwrap(),
        AnalysisResultDocument::from_noise(identity(AnalysisKind::Noise), &noise.points)
            .unwrap()
            .build()
            .unwrap(),
    ] {
        let mut wire = serde_json::to_value(&document).unwrap();
        assert!(wire.get("frequencyTable").is_none());
        for version in 1..=10 {
            wire["schemaVersion"] = version.into();
            let restored = AnalysisResultDocument::from_json(&wire.to_string()).unwrap();
            match restored.payload() {
                ResultPayload::Ac(_) => assert!(restored.frequency_table().is_none()),
                ResultPayload::Noise(_) => assert!(restored.frequency_table().is_none()),
                _ => unreachable!(),
            }
        }
    }
    let document = AnalysisResultDocument::from_ac_table(identity(AnalysisKind::Ac), &ac)
        .unwrap()
        .build()
        .unwrap();
    let mut wire = serde_json::to_value(&document).unwrap();
    for version in 1..=10 {
        wire["schemaVersion"] = version.into();
        assert!(
            AnalysisResultDocument::from_json(&wire.to_string())
                .unwrap_err()
                .to_string()
                .contains("version 11")
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn malformed_table_metadata_and_axis_links_are_rejected() {
    let (ac, _) = results();
    let document = AnalysisResultDocument::from_ac_table(identity(AnalysisKind::Ac), &ac)
        .unwrap()
        .build()
        .unwrap();
    let original = serde_json::to_value(&document).unwrap();
    for (pointer, value) in [
        ("/frequencyTable/requestedRows", serde_json::json!(4)),
        ("/frequencyTable/tableName", serde_json::json!("")),
        (
            "/frequencyTable/columns/0/axis",
            serde_json::json!("missing"),
        ),
        ("/frequencyTable/columns/0/name", serde_json::json!("TEMP")),
        (
            "/frequencyTable/columns/0/target/target",
            serde_json::json!("resistance"),
        ),
        (
            "/frequencyTable/columns/2/target",
            serde_json::json!({"kind":"parameter","target":"RESISTANCE"}),
        ),
        ("/axes/0/unit", serde_json::json!({"unit":"second"})),
        (
            "/axes/0/values",
            serde_json::json!({"representation":"integer","values":[100,10,100]}),
        ),
        ("/axes/0/values/values/0", serde_json::json!(-1)),
    ] {
        let mut wire = original.clone();
        *wire.pointer_mut(pointer).unwrap() = value;
        assert!(
            AnalysisResultDocument::from_json(&wire.to_string()).is_err(),
            "accepted {pointer}"
        );
    }
    let mut missing_column = original.clone();
    missing_column["frequencyTable"]["columns"]
        .as_array_mut()
        .unwrap()
        .pop();
    assert!(AnalysisResultDocument::from_json(&missing_column.to_string()).is_err());
    for pointer in [
        "/frequencyTable",
        "/frequencyTable/columns/0",
        "/frequencyTable/columns/0/target",
        "/frequencyTable/columns/3/target/target",
    ] {
        let mut wire = original.clone();
        wire.pointer_mut(pointer)
            .unwrap()
            .as_object_mut()
            .unwrap()
            .insert("unexpected".into(), serde_json::json!(true));
        assert!(
            AnalysisResultDocument::from_json(&wire.to_string()).is_err(),
            "ignored unknown {pointer}"
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn model_finish_prefixes_keep_requested_rows_and_strict_finish_metadata() {
    let (mut ac, _) = results();
    ac.points.truncate(2);
    for column in &mut ac.columns {
        column.values.truncate(2);
    }
    assert!(AnalysisResultDocument::from_ac_table(identity(AnalysisKind::Ac), &ac).is_err());
    for point in [
        ModelFinishPoint::Initialization,
        ModelFinishPoint::OperatingPoint,
        ModelFinishPoint::Frequency { frequency: 100.0 },
    ] {
        ac.finish = Some(ModelFinish {
            instance: "X1".into(),
            model: "finish_model".into(),
            site: 2,
            point,
            diagnostic_level: 1,
        });
        let document = AnalysisResultDocument::from_ac_table(identity(AnalysisKind::Ac), &ac)
            .unwrap()
            .build()
            .unwrap();
        assert_eq!(metadata(&document).finish, ac.finish);
        assert_eq!(document.point_count(), 2);
        assert_eq!(metadata(&document).requested_rows, 3);
        assert_eq!(
            AnalysisResultDocument::from_json(&document.to_json().unwrap()).unwrap(),
            document
        );
        for pointer in ["/frequencyTable/finish", "/frequencyTable/finish/point"] {
            let mut wire = serde_json::to_value(&document).unwrap();
            wire.pointer_mut(pointer)
                .unwrap()
                .as_object_mut()
                .unwrap()
                .insert("unexpected".into(), true.into());
            assert!(AnalysisResultDocument::from_json(&wire.to_string()).is_err());
        }
    }
    ac.finish.as_mut().unwrap().point = ModelFinishPoint::Transient { time: 1.0 };
    assert!(AnalysisResultDocument::from_ac_table(identity(AnalysisKind::Ac), &ac).is_err());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn malformed_source_coordinates_and_signal_shapes_cannot_be_projected() {
    let (ac, _) = results();
    for mutation in 0..7 {
        let mut changed = ac.clone();
        match mutation {
            0 => changed.columns[0].values.pop().map(|_| ()).unwrap(),
            1 => changed.columns[1].values[0] += 1.0,
            2 => changed.columns[0].values[0] = f64::NAN,
            3 => changed.columns[0].target = FrequencyDataTarget::Frequency,
            4 => changed.points[0]
                .voltages
                .push(num_complex::Complex64::new(1.0, 0.0)),
            5 => changed.points[1].node_names[0] = "different".into(),
            _ => changed.columns.clear(),
        }
        assert!(
            AnalysisResultDocument::from_ac_table(identity(AnalysisKind::Ac), &changed).is_err(),
            "mutation {mutation}"
        );
    }
}

struct CancelAfter {
    polls: AtomicUsize,
    limit: usize,
}
impl AbortSignal for CancelAfter {
    fn is_aborted(&self) -> bool {
        self.polls.fetch_add(1, Ordering::Relaxed) >= self.limit
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn projection_decode_and_validation_charge_coordinates_and_metadata_exactly() {
    let (ac, noise) = results();
    for is_noise in [false, true] {
        let project = |limits: &ResourceLimits, abort: &dyn AbortSignal| {
            if is_noise {
                AnalysisResultDocument::from_noise_table_with_limits_and_abort(
                    identity(AnalysisKind::Noise),
                    &noise,
                    limits,
                    abort,
                )
            } else {
                AnalysisResultDocument::from_ac_table_with_limits_and_abort(
                    identity(AnalysisKind::Ac),
                    &ac,
                    limits,
                    abort,
                )
            }
        };
        let document = project(&ResourceLimits::default(), &NoAbort)
            .unwrap()
            .build()
            .unwrap();
        let count = document.total_value_count();
        let mut limits = ResourceLimits::default();
        limits.max_result_values = count;
        assert_eq!(
            project(&limits, &NoAbort)
                .unwrap()
                .build_with_limits_and_abort(&limits, &NoAbort)
                .unwrap(),
            document
        );
        let json = document.to_json().unwrap();
        assert!(
            AnalysisResultDocument::from_json_with_limits_and_abort(
                &json,
                &limits,
                &NoAbort,
                u64::MAX
            )
            .is_ok()
        );
        limits.max_result_values -= 1;
        for result in [
            project(&limits, &NoAbort).map(|_| ()),
            document.validate_with_limits_and_abort(&limits, &NoAbort),
            AnalysisResultDocument::from_json_with_limits_and_abort(
                &json,
                &limits,
                &NoAbort,
                u64::MAX,
            )
            .map(|_| ()),
        ] {
            let ResultDocumentError::ResourceLimit(error) = result.unwrap_err() else {
                panic!("resource failure")
            };
            assert_eq!(error.resource, ResourceKind::ResultValues);
            assert_eq!(error.requested, count);
        }
        let counted = CancelAfter {
            polls: AtomicUsize::new(0),
            limit: usize::MAX,
        };
        project(&ResourceLimits::default(), &counted).unwrap();
        let polls = counted.polls.load(Ordering::Relaxed);
        for limit in [0, 1, polls / 2, polls - 1] {
            let abort = CancelAfter {
                polls: AtomicUsize::new(0),
                limit,
            };
            assert!(matches!(
                project(&ResourceLimits::default(), &abort),
                Err(ResultDocumentError::Aborted)
            ));
        }
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn zero_ac_frequency_is_retained_and_zero_noise_frequency_is_rejected() {
    let (mut ac, mut noise) = results();
    ac.points[0].frequency = 0.0;
    ac.columns[1].values[0] = 0.0;
    let document = AnalysisResultDocument::from_ac_table(identity(AnalysisKind::Ac), &ac)
        .unwrap()
        .build()
        .unwrap();
    assert_eq!(
        AnalysisResultDocument::from_json(&document.to_json().unwrap()).unwrap(),
        document
    );
    noise.points[0].frequency = 0.0;
    noise.columns[1].values[0] = 0.0;
    assert!(
        AnalysisResultDocument::from_noise_table(identity(AnalysisKind::Noise), &noise).is_err()
    );
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn current_referred_noise_keeps_density_units_and_aligned_contributions() {
    let netlist = Netlist::parse("Current noise\nI1 0 out AC 1\nR1 out 0 1k\n.data rows FREQ R1\n10 1k\n100 2k\n.enddata\n.end\n").unwrap();
    let noise = Engine::default()
        .run_noise_table_named_with_input_source(&netlist, "out", None, "I1", "rows", 300.15)
        .unwrap();
    let document = AnalysisResultDocument::from_noise_table(identity(AnalysisKind::Noise), &noise)
        .unwrap()
        .build()
        .unwrap();
    let input = document
        .signals()
        .iter()
        .find(|signal| signal.descriptor().canonical_name() == "inoise_spectrum")
        .unwrap();
    assert_eq!(input.descriptor().unit().symbol(), "A^2/Hz");
    let SeriesValues::Real { samples } = input.values() else {
        panic!("density")
    };
    for (value, resistance) in samples.iter().zip([1000.0, 2000.0]) {
        close(value.unwrap(), 4.0 * 1.380649e-23 * 300.15 / resistance);
    }
    let mut wire = serde_json::to_value(&document).unwrap();
    wire["payload"]["contributions"][0]["outputContribution"]
        .as_array_mut()
        .unwrap()
        .pop();
    assert!(AnalysisResultDocument::from_json(&wire.to_string()).is_err());
}

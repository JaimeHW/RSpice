//! Physical row coordinates remain attached to table results at the wire boundary.
use rspice_core::engine::{FrequencyDataResult, FrequencyDataTarget};
use rspice_core::execution::result_document::AxisValues;
use rspice_core::execution::{AnalysisInstanceId, AnalysisKind, AnalysisResultDocument};
use rspice_core::{Engine, Netlist};

const DECK: &str = "Table document\n.param resistance=1k\nV1 in 0 AC 1\nR1 in out {resistance}\nC1 out 0 1u\n.data points HERTZ resistance C1:C\n100 1k 1u\n10 2k 2u\n100 3k 3u\n.enddata\n.end\n";

fn ac() -> FrequencyDataResult<rspice_core::analysis::AcResult> {
    Engine::default()
        .run_ac_table(&Netlist::parse(DECK).unwrap(), "points")
        .unwrap()
}

fn ac_document() -> AnalysisResultDocument {
    AnalysisResultDocument::from_ac_table(AnalysisInstanceId::new(AnalysisKind::Ac, 0), &ac())
        .unwrap()
        .build()
        .unwrap()
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn ac_and_noise_documents_preserve_order_targets_and_row_coordinates() {
    let engine = Engine::default();
    let noise = engine
        .run_noise_table_named_with_input_source(
            &Netlist::parse(DECK).unwrap(),
            "out",
            None,
            "V1",
            "points",
            300.15,
        )
        .unwrap();
    let noise = AnalysisResultDocument::from_noise_table(
        AnalysisInstanceId::new(AnalysisKind::Noise, 0),
        &noise,
    )
    .unwrap()
    .build()
    .unwrap();
    for document in [ac_document(), noise] {
        let metadata = document.frequency_table().unwrap();
        assert_eq!(metadata.table_name.to_ascii_lowercase(), "points");
        assert_eq!(metadata.requested_rows, 3);
        assert_eq!(metadata.columns.len(), 3);
        assert!(metadata.finish.is_none());
        assert_eq!(metadata.columns[0].target, FrequencyDataTarget::Frequency);
        assert_eq!(
            metadata.columns[1].target,
            FrequencyDataTarget::Parameter("RESISTANCE".into())
        );
        assert_eq!(
            metadata.columns[2].target,
            FrequencyDataTarget::DeviceParameter {
                device_name: "C1".into(),
                parameter_name: "C".into(),
            }
        );
        for (axis, expected) in document.axes().iter().zip([
            vec![100.0, 10.0, 100.0],
            vec![1000.0, 2000.0, 3000.0],
            vec![1e-6, 2e-6, 3e-6],
        ]) {
            assert_eq!(axis.values(), &AxisValues::Real { values: expected });
        }
        let json = document.to_json().unwrap();
        assert_eq!(AnalysisResultDocument::from_json(&json).unwrap(), document);
        let window = document.window(1, 1).unwrap();
        assert_eq!(window.axes.len(), 3);
        assert_eq!(
            window.axes[1].values,
            AxisValues::Real {
                values: vec![2000.0]
            }
        );
        let mut limits = rspice_core::ResourceLimits::default();
        limits.max_result_values = document.total_value_count() - 1;
        assert!(
            document
                .validate_with_limits_and_abort(&limits, &rspice_core::NoAbort)
                .is_err()
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn early_model_finish_is_explicit_and_an_unexplained_prefix_is_rejected() {
    let mut result = ac();
    result.points.truncate(2);
    for column in &mut result.columns {
        column.values.truncate(2);
    }
    let build = |result: &FrequencyDataResult<_>| {
        AnalysisResultDocument::from_ac_table(AnalysisInstanceId::new(AnalysisKind::Ac, 0), result)
            .unwrap()
            .build()
    };
    assert!(build(&result).is_err());
    result.finish = Some(rspice_core::ModelFinish {
        model: "resistor".into(),
        instance: "r1".into(),
        site: 3,
        diagnostic_level: 1,
        point: rspice_core::ModelFinishPoint::Frequency { frequency: 100.0 },
    });
    let document = build(&result).unwrap();
    assert_eq!(document.point_count(), 2);
    assert_eq!(document.frequency_table().unwrap().requested_rows, 3);
    assert_eq!(document.frequency_table().unwrap().finish, result.finish);
    assert_eq!(
        AnalysisResultDocument::from_json(&document.to_json().unwrap()).unwrap(),
        document
    );
    let mut wire: serde_json::Value = serde_json::from_str(&document.to_json().unwrap()).unwrap();
    assert_eq!(wire["frequencyTable"]["finish"]["diagnosticLevel"], 1);
    assert_eq!(
        wire["frequencyTable"]["finish"]["point"]["kind"],
        "frequency"
    );
    wire["frequencyTable"]["finish"]["unexpected"] = true.into();
    assert!(AnalysisResultDocument::from_json(&wire.to_string()).is_err());
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn malformed_or_downgraded_table_provenance_is_not_trusted() {
    let original: serde_json::Value =
        serde_json::from_str(&ac_document().to_json().unwrap()).unwrap();
    for change in 0..8 {
        let mut wire = original.clone();
        match change {
            0 => wire["schemaVersion"] = 10.into(),
            1 => wire["frequencyTable"]["requestedRows"] = 4.into(),
            2 => wire["frequencyTable"]["columns"][1]["axis"] = "absent".into(),
            3 => {
                wire["frequencyTable"]["columns"][1]["target"] =
                    wire["frequencyTable"]["columns"][0]["target"].clone()
            }
            4 => {
                wire["frequencyTable"]["columns"][1]["name"] =
                    wire["frequencyTable"]["columns"][0]["name"].clone()
            }
            5 => wire["axes"][0]["values"]["values"][0] = (-1).into(),
            6 => wire["frequencyTable"]["columns"]
                .as_array_mut()
                .unwrap()
                .truncate(2),
            _ => wire["frequencyTable"]["columns"][1]["target"]["target"] = "resistance".into(),
        }
        assert!(
            AnalysisResultDocument::from_json(&wire.to_string()).is_err(),
            "mutation {change}"
        );
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn constructors_refuse_coordinates_that_disagree_with_solved_points() {
    let mut result = ac();
    result.columns[0].values[0] = 101.0;
    assert!(
        AnalysisResultDocument::from_ac_table(
            AnalysisInstanceId::new(AnalysisKind::Ac, 0),
            &result
        )
        .is_err()
    );
}

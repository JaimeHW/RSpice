use super::*;
use arrow_array::{ArrayRef, Float64Array, RecordBatch};
use arrow_schema::{DataType, Field, Schema};

fn field(name: &str, unit: Option<&str>) -> Field {
    Field::new(name, DataType::Float64, false).with_metadata(
        unit.map(|unit| ("unit".into(), unit.into()))
            .into_iter()
            .collect(),
    )
}

fn read_formats(
    coordinate_unit: Option<&str>,
    real_unit: Option<&str>,
    imag_unit: Option<&str>,
    analysis: &str,
) -> Vec<Result<crate::numeric::DecodedNumericDataset, ColumnarReadError>> {
    // The selected coordinate is deliberately not the first column; unit
    // metadata must follow the field when complex components are combined.
    let schema = Arc::new(Schema::new_with_metadata(
        vec![
            field("Im(gain)", imag_unit),
            field("x", coordinate_unit),
            field("Re(gain)", real_unit),
            field("current", Some("uA")),
        ],
        HashMap::from([
            ("rspice.coordinate".into(), "x".into()),
            ("rspice.analysis".into(), analysis.into()),
        ]),
    ));
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(Float64Array::from(vec![-0.0, -2.0])) as ArrayRef,
            Arc::new(Float64Array::from(vec![1.0, 2.0])),
            Arc::new(Float64Array::from(vec![3.0, 4.0])),
            Arc::new(Float64Array::from(vec![5.0, 6.0])),
        ],
    )
    .unwrap();
    let mut file = Vec::new();
    let mut writer = arrow_ipc::writer::FileWriter::try_new(&mut file, &schema).unwrap();
    writer.write(&batch).unwrap();
    writer.finish().unwrap();
    drop(writer);
    let mut stream = Vec::new();
    let mut writer = arrow_ipc::writer::StreamWriter::try_new(&mut stream, &schema).unwrap();
    writer.write(&batch).unwrap();
    writer.finish().unwrap();
    drop(writer);
    let mut parquet = Vec::new();
    let mut writer = parquet::arrow::ArrowWriter::try_new(&mut parquet, schema, None).unwrap();
    writer.write(&batch).unwrap();
    writer.close().unwrap();
    let limits = ColumnarLimits {
        max_columns: 4,
        max_rows: 2,
        max_values: 8,
    };
    vec![
        decode_arrow_ipc(&file, limits, "Arrow file"),
        decode_arrow_ipc(&stream, limits, "Arrow stream"),
        decode_parquet(&parquet, limits, "Parquet"),
    ]
}

#[test]
fn field_units_survive_columnar_import_and_coordinates_are_normalized() {
    for (analysis, unit, canonical, expected) in [
        ("tran", "ms", "s", [0.001, 0.002]),
        ("ac", "kHz", "Hz", [1000.0, 2000.0]),
    ] {
        for decoded in read_formats(Some(unit), Some("mV"), Some("mV"), analysis) {
            let decoded = decoded.unwrap();
            assert_eq!(decoded.coordinate, expected);
            assert_eq!(decoded.coordinate_unit.as_deref(), Some(canonical));
            assert_eq!(decoded.signals[0].name, "current");
            assert_eq!(decoded.signals[0].unit.as_deref(), Some("uA"));
            assert_eq!(decoded.signals[0].real, [5.0, 6.0]);
            assert_eq!(decoded.signals[1].name, "gain");
            assert_eq!(decoded.signals[1].unit.as_deref(), Some("mV"));
            assert_eq!(decoded.signals[1].real, [3.0, 4.0]);
            assert_eq!(
                decoded.signals[1].imag.as_ref().unwrap()[0].to_bits(),
                (-0.0_f64).to_bits()
            );
        }
    }
}

#[test]
fn invalid_coordinate_units_and_conflicting_complex_units_are_refused() {
    for unit in ["V", "fortnight", "\ns"] {
        for decoded in read_formats(Some(unit), Some("V"), Some("V"), "tran") {
            let error = decoded.unwrap_err();
            assert!(
                matches!(
                    error.reason,
                    ColumnarReadFailure::Coordinate(_) | ColumnarReadFailure::InvalidUnit { .. }
                ),
                "{error}"
            );
        }
    }
    for (real, imag) in [
        (Some("V"), Some("mV")),
        (Some("V"), None),
        (None, Some("V")),
    ] {
        for decoded in read_formats(Some("s"), real, imag, "tran") {
            let error = decoded.unwrap_err();
            assert!(
                matches!(error.reason, ColumnarReadFailure::ComplexColumns(crate::numeric::ComplexColumnError::InconsistentUnits { ref name, .. }) if name == "gain"),
                "{error}"
            );
        }
    }
}

#[test]
fn missing_and_legacy_empty_unit_metadata_remain_unstated() {
    for unit in [None, Some("")] {
        for decoded in read_formats(unit, unit, unit, "tran") {
            let decoded = decoded.unwrap();
            assert_eq!(decoded.coordinate, [1.0, 2.0]);
            assert_eq!(decoded.coordinate_unit, None);
            assert_eq!(decoded.signals[1].unit, None);
        }
    }
}

use super::*;

#[test]
fn native_import_budgets_include_nulls_and_retained_complex_magnitudes() {
    // Reordered fields and escaped keys must have identical accounting.
    let source = r#"{"signals":[{"name":"real","values":[null,1]},{"name":"complex","imag":[null,4],"real":[null,3]}],"coordinate":{"values":[1,2],"name":"frequency"},"analysis":"ac","schema":"rspice-waveform-dataset/3"}"#;
    for source in [
        source.to_owned(),
        source
            .replace("\"values\"", "\"val\\u0075es\"")
            .replace("\"signals\"", "\"sig\\u006eals\""),
    ] {
        for kind in [NativeBundleKind::Result, NativeBundleKind::Dataset] {
            let bytes = pack_source(&source, kind);
            // Two coordinates, two real samples, and six retained complex values.
            let exact = NativeBundleReadLimits {
                max_rows: 2,
                max_columns: 3,
                max_numeric_values: 10,
                ..limits()
            };
            let decoded = decode_native_bundle(&bytes, kind, exact).unwrap();
            assert_eq!(decoded.coordinate, [1.0, 2.0]);
            assert!(decoded.signals[0].real[0].is_nan());
            assert!(decoded.signals[1].real[0].is_nan());
            assert!(decoded.signals[1].imag.as_ref().unwrap()[0].is_nan());
            assert_eq!(decoded.signals[1].real[1], 3.0);
            for (bounds, expected) in [
                (
                    NativeBundleReadLimits {
                        max_rows: 1,
                        ..exact
                    },
                    "2 samples per column; the limit is 1",
                ),
                (
                    NativeBundleReadLimits {
                        max_columns: 2,
                        ..exact
                    },
                    "3 columns; the limit is 2",
                ),
                (
                    NativeBundleReadLimits {
                        max_numeric_values: 9,
                        ..exact
                    },
                    "10 retained numeric values; the limit is 9",
                ),
            ] {
                let error = decode_native_bundle(&bytes, kind, bounds).unwrap_err();
                assert!(
                    matches!(error, NativeBundleError::InvalidData(_)),
                    "{error}"
                );
                assert!(error.to_string().contains(expected), "{error}");
            }
        }
    }
}

#[test]
fn native_import_stops_at_the_limit_before_parsing_an_invalid_tail() {
    let cases = [
        (
            r#"{"coordinate":{"values":[0,0,0,0,INVALID]}}"#,
            NativeBundleReadLimits {
                max_numeric_values: 3,
                ..limits()
            },
            "4 retained numeric values; the limit is 3",
        ),
        (
            r#"{"signals":[{"values":[null,null,null,null,INVALID]}]}"#,
            NativeBundleReadLimits {
                max_numeric_values: 3,
                ..limits()
            },
            "4 retained numeric values; the limit is 3",
        ),
        (
            r#"{"signals":[{"real":[null,null,null,INVALID]}]}"#,
            NativeBundleReadLimits {
                max_numeric_values: 3,
                ..limits()
            },
            "4 retained numeric values; the limit is 3",
        ),
        (
            r#"{"signals":[{"imag":[null,null,null,null,INVALID]}]}"#,
            NativeBundleReadLimits {
                max_numeric_values: 3,
                ..limits()
            },
            "4 retained numeric values; the limit is 3",
        ),
        (
            r#"{"coordinate":{"values":[0,0,0,INVALID]}}"#,
            NativeBundleReadLimits {
                max_rows: 2,
                ..limits()
            },
            "3 samples per column; the limit is 2",
        ),
        (
            r#"{"signals":[{"real":[0,0,0,INVALID]}]}"#,
            NativeBundleReadLimits {
                max_rows: 2,
                ..limits()
            },
            "3 samples per column; the limit is 2",
        ),
        (
            r#"{"signals":[{}, {}, INVALID]}"#,
            NativeBundleReadLimits {
                max_columns: 2,
                ..limits()
            },
            "3 columns; the limit is 2",
        ),
    ];
    for (source, bounds, expected) in cases {
        for kind in [NativeBundleKind::Result, NativeBundleKind::Dataset] {
            let error = decode_native_bundle(&pack_source(source, kind), kind, bounds).unwrap_err();
            assert!(
                matches!(error, NativeBundleError::InvalidData(_)),
                "{error}"
            );
            assert!(error.to_string().contains(expected), "{error}");
        }
    }
}

#[test]
fn native_import_counts_actual_component_lengths_before_shape_validation() {
    let source = r#"{"schema":"rspice-waveform-dataset/1","analysis":"transient","coordinate":{"name":"time","values":[0,1]},"signals":[{"name":"out","values":[0,0,0,0,0,0,0,0,0]}]}"#;
    let error = decode_native_bundle(
        &pack_source(source, NativeBundleKind::Result),
        NativeBundleKind::Result,
        NativeBundleReadLimits {
            max_numeric_values: 8,
            ..limits()
        },
    )
    .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("9 retained numeric values; the limit is 8"),
        "{error}"
    );
}

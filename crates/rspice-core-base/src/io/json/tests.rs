#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(tag = "family")]
enum BufferedPayload {
    Samples {
        values: Vec<Option<f64>>,
        counter: u64,
    },
    Map {
        values: std::collections::BTreeMap<String, f64>,
        counter: u64,
    },
}

#[test]
fn precision_checks_follow_buffered_enums_and_do_not_round_native_integer_metadata() {
    use super::from_str;
    let value: BufferedPayload = from_str(
        r#"{"counter":18446744073709551615,"values":[null,-0.0,5e-324],"family":"Samples"}"#,
        &crate::NoAbort,
    )
    .unwrap();
    let BufferedPayload::Samples { values, counter } = value else {
        panic!("wrong variant")
    };
    assert_eq!(counter, u64::MAX);
    assert_eq!(values[1].unwrap().to_bits(), (-0.0_f64).to_bits());
    assert_eq!(values[2].unwrap().to_bits(), 1);

    for (source, path) in [
        (
            r#"{"counter":18446744073709551615,"values":[null,9007199254740993],"family":"Samples"}"#,
            "/values/1",
        ),
        (
            r#"{"values":{"a/b~c":9007199254740993},"family":"Map","counter":0}"#,
            "/values/a~1b~0c",
        ),
    ] {
        let error = from_str::<BufferedPayload>(source, &crate::NoAbort).unwrap_err();
        assert!(
            error.to_string().contains("cannot be represented exactly"),
            "{error}"
        );
        assert!(error.to_string().contains(path), "{error}");
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
enum ExternalPayload {
    Newtype(f64),
    Tuple(u64, f64),
    Struct { counter: u64, value: f64 },
}

#[test]
fn precision_checks_cover_external_variants_and_numeric_map_keys() {
    use super::from_str;
    for (source, path) in [
        (r#"{"Newtype":9007199254740993}"#, "/Newtype"),
        (
            r#"{"Tuple":[18446744073709551615,9007199254740993]}"#,
            "/Tuple/1",
        ),
        (
            r#"{"Struct":{"counter":18446744073709551615,"value":9007199254740993}}"#,
            "/Struct/value",
        ),
    ] {
        let error = from_str::<ExternalPayload>(source, &crate::NoAbort).unwrap_err();
        assert!(error.to_string().contains(path), "{error}");
        assert!(
            error.to_string().contains("cannot be represented exactly"),
            "{error}"
        );
    }
    let error = from_str::<std::collections::BTreeMap<u128, f64>>(
        r#"{"18446744073709551617":9007199254740993}"#,
        &crate::NoAbort,
    )
    .unwrap_err();
    assert!(
        error.to_string().contains("/18446744073709551617"),
        "{error}"
    );
    let native: (i64, u64, u128) = from_str(
        "[-9223372036854775807,18446744073709551615,340282366920938463463374607431768211455]",
        &crate::NoAbort,
    )
    .unwrap();
    assert_eq!(native, (i64::MIN + 1, u64::MAX, u128::MAX));
}

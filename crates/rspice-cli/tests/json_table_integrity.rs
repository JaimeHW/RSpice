//! Numeric representations must be unambiguous before any selection or output.
mod common;

use common::{read_json, test_dir};
use std::process::{Command, Output};

fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--quiet")
        .args(args)
        .output()
        .unwrap()
}

fn source() -> serde_json::Value {
    serde_json::json!({
        "analysis": "converted", "plot_name": "Converted Data",
        "scale": {"name": "time", "type": "time", "values": [0.0, 1.0]},
        "signals": [
            {"name": "D(clk)", "type": "digital", "values": [0.0, 1.0]},
            {"name": "V(out)", "type": "voltage", "real": [2.0, 3.0], "imag": [4.0, 5.0]},
        ],
    })
}

#[test]
fn ambiguous_numeric_representations_preserve_outputs_even_when_unselected() {
    let directory = test_dir("ambiguous_json_components");
    let input = directory.join("input.json");
    let golden = directory.join("golden.json");
    let missing = directory.join("missing.json");
    let original = serde_json::to_vec(&source()).unwrap();
    std::fs::write(&golden, &original).unwrap();
    for (target, fields, expected) in [
        (
            "signal",
            serde_json::json!({"values": [2.0, 3.0], "real": [7.0, 8.0]}),
            "conflicting numeric representations",
        ),
        (
            "signal",
            serde_json::json!({"values": [2.0, 3.0], "imag": [4.0, 5.0]}),
            "conflicting numeric representations",
        ),
        (
            "signal",
            serde_json::json!({"values": [2.0, 3.0], "real": [7.0, 8.0], "imag": [4.0, 5.0]}),
            "conflicting numeric representations",
        ),
        (
            "signal",
            serde_json::json!({"values": [2.0, 3.0], "imag": null}),
            "conflicting numeric representations",
        ),
        (
            "scale",
            serde_json::json!({"imag": [4.0, 5.0]}),
            "coordinate fields are not supported",
        ),
        (
            "scale",
            serde_json::json!({"real": [7.0, 8.0]}),
            "coordinate fields are not supported",
        ),
    ] {
        let mut data = source();
        let object = if target == "signal" {
            data["signals"][1] = serde_json::json!({"name": "V(out)", "type": "voltage"});
            data["signals"][1].as_object_mut().unwrap()
        } else {
            data["scale"].as_object_mut().unwrap()
        };
        object.extend(fields.as_object().unwrap().clone());
        let invalid = serde_json::to_vec(&data).unwrap();
        std::fs::write(&input, &invalid).unwrap();
        let check = |output: Output| {
            assert_eq!(output.status.code(), Some(1), "{target}: {output:?}");
            assert!(
                String::from_utf8_lossy(&output.stderr).contains(expected),
                "{output:?}"
            );
        };
        for format in ["json", "csv", "tsv", "raw", "ascii", "hdf5", "vcd"] {
            let output = directory.join(format!("protected.{format}"));
            std::fs::write(&output, "predecessor").unwrap();
            check(cli(&[
                "convert",
                input.to_str().unwrap(),
                output.to_str().unwrap(),
                "--to",
                format,
                "--variables",
                "D(clk)",
            ]));
            assert_eq!(std::fs::read_to_string(output).unwrap(), "predecessor");
        }
        check(cli(&[
            "convert",
            input.to_str().unwrap(),
            input.to_str().unwrap(),
            "--to",
            "json",
        ]));
        assert_eq!(std::fs::read(&input).unwrap(), invalid);
        for bless in [false, true] {
            let mut args = vec![
                "compare",
                input.to_str().unwrap(),
                golden.to_str().unwrap(),
                "--variables",
                "D(clk)",
            ];
            if bless {
                args.push("--bless");
            }
            check(cli(&args));
            assert_eq!(std::fs::read(&golden).unwrap(), original);
        }
        check(cli(&[
            "compare",
            input.to_str().unwrap(),
            missing.to_str().unwrap(),
            "--bless",
        ]));
        assert!(!missing.exists());
    }
}

#[test]
fn separate_real_and_complex_json_signals_round_trip_in_every_table_format() {
    let directory = test_dir("json_numeric_representations");
    let input = directory.join("source.json");
    let expected = source();
    std::fs::write(&input, serde_json::to_vec(&expected).unwrap()).unwrap();
    for format in ["json", "csv", "tsv", "raw", "ascii", "hdf5"] {
        let encoded = directory.join(format!("encoded.{format}"));
        let recovered = directory.join("recovered.json");
        for (source, destination, format) in
            [(&input, &encoded, format), (&encoded, &recovered, "json")]
        {
            let output = cli(&[
                "convert",
                source.to_str().unwrap(),
                destination.to_str().unwrap(),
                "--to",
                format,
            ]);
            assert!(output.status.success(), "{output:?}");
        }
        let actual = read_json(&recovered);
        assert_eq!(actual["scale"], expected["scale"], "{format}");
        assert_eq!(actual["signals"], expected["signals"], "{format}");
    }
}

#[test]
fn repeated_json_fields_cannot_discard_samples_or_change_signal_identity() {
    let directory = test_dir("duplicate_json_fields");
    let input = directory.join("input.json");
    let golden = directory.join("golden.json");
    let missing = directory.join("missing.json");
    let original = serde_json::to_string(&source()).unwrap();
    std::fs::write(&golden, &original).unwrap();
    for (needle, preceding, key) in [
        (r#""values":[0.0,1.0]"#, r#""values":[99,99],"#, "values"),
        (r#""real":[2.0,3.0]"#, r#""real":[99,99],"#, "real"),
        (r#""imag":[4.0,5.0]"#, r#""imag":null,"#, "imag"),
        (r#""name":"V(out)""#, r#""name":"I(out)","#, "name"),
        (r#""type":"voltage""#, r#""type":"current","#, "type"),
        (
            r#""analysis":"converted""#,
            r#""analysis":"fft","#,
            "analysis",
        ),
        (r#""scale":{"#, r#""scale":{},"#, "scale"),
        (r#""signals":["#, r#""signals":[],"#, "signals"),
        // Keys are compared after JSON escaping, and identical values are
        // still duplicates rather than an implicit agreement between writers.
        (r#""real":[2.0,3.0]"#, r#""\u0072eal":[2.0,3.0],"#, "real"),
    ] {
        assert!(original.contains(needle));
        let invalid = original.replacen(needle, &format!("{preceding}{needle}"), 1);
        std::fs::write(&input, &invalid).unwrap();
        let check = |output: Output| {
            assert_eq!(output.status.code(), Some(1), "{key}: {output:?}");
            let message = String::from_utf8_lossy(&output.stderr);
            assert!(
                message.contains("duplicate JSON field") && message.contains(key),
                "{message}"
            );
            assert!(
                message.contains("line") && message.contains("column"),
                "{message}"
            );
        };
        for format in ["json", "csv", "tsv", "raw", "ascii", "hdf5", "vcd"] {
            let output = directory.join(format!("protected.{format}"));
            std::fs::write(&output, "predecessor").unwrap();
            check(cli(&[
                "convert",
                input.to_str().unwrap(),
                output.to_str().unwrap(),
                "--to",
                format,
                "--variables",
                "D(clk)",
            ]));
            assert_eq!(std::fs::read_to_string(output).unwrap(), "predecessor");
        }
        check(cli(&[
            "convert",
            input.to_str().unwrap(),
            input.to_str().unwrap(),
            "--to",
            "json",
        ]));
        assert_eq!(std::fs::read_to_string(&input).unwrap(), invalid);
        for bless in [false, true] {
            let mut args = vec![
                "compare",
                input.to_str().unwrap(),
                golden.to_str().unwrap(),
                "--variables",
                "D(clk)",
            ];
            if bless {
                args.push("--bless");
            }
            check(cli(&args));
            assert_eq!(std::fs::read_to_string(&golden).unwrap(), original);
        }
        check(cli(&[
            "compare",
            input.to_str().unwrap(),
            missing.to_str().unwrap(),
            "--bless",
        ]));
        assert!(!missing.exists());
    }
}

#[test]
fn malformed_optional_json_metadata_cannot_disable_quantity_checks() {
    let directory = test_dir("malformed_json_metadata");
    let input = directory.join("input.json");
    let golden = directory.join("golden.json");
    let original = serde_json::to_vec(&source()).unwrap();
    std::fs::write(&golden, &original).unwrap();
    for (pointer, field) in [
        ("/analysis", "analysis"),
        ("/plot_name", "plot_name"),
        ("/scale/name", "scale.name"),
        ("/scale/type", "scale.type"),
        ("/signals/1/type", "signal.type"),
    ] {
        for invalid in [
            serde_json::json!(42),
            serde_json::json!(true),
            serde_json::json!(["current"]),
            serde_json::json!({"type": "current"}),
        ] {
            let mut value = source();
            *value.pointer_mut(pointer).unwrap() = invalid;
            std::fs::write(&input, serde_json::to_vec(&value).unwrap()).unwrap();
            let check = |output: Output| {
                assert_eq!(output.status.code(), Some(1), "{field}: {output:?}");
                let message = String::from_utf8_lossy(&output.stderr);
                assert!(
                    message.contains(field) && message.contains("must be a string"),
                    "{message}"
                );
            };
            for format in ["json", "csv", "tsv", "raw", "ascii", "hdf5", "vcd"] {
                let output = directory.join(format!("protected.{format}"));
                std::fs::write(&output, "predecessor").unwrap();
                check(cli(&[
                    "convert",
                    input.to_str().unwrap(),
                    output.to_str().unwrap(),
                    "--to",
                    format,
                    "--variables",
                    "D(clk)",
                ]));
                assert_eq!(std::fs::read_to_string(output).unwrap(), "predecessor");
            }
            for bless in [false, true] {
                let mut args = vec!["compare", input.to_str().unwrap(), golden.to_str().unwrap()];
                if bless {
                    args.push("--bless");
                }
                check(cli(&args));
                assert_eq!(std::fs::read(&golden).unwrap(), original);
            }
            let missing = directory.join("missing.json");
            check(cli(&[
                "compare",
                input.to_str().unwrap(),
                missing.to_str().unwrap(),
                "--bless",
            ]));
            assert!(!missing.exists());
        }
    }
}

#[test]
fn unstated_legacy_json_metadata_keeps_existing_defaults_and_type_inference() {
    let directory = test_dir("unstated_json_metadata");
    let input = directory.join("input.json");
    let output = directory.join("output.json");
    for null in [false, true] {
        let mut value = serde_json::json!({
            "scale": {"values": [0, 1]},
            "signals": [{"name": "I(V1)", "values": [-1, 2]}],
        });
        if null {
            value["analysis"] = serde_json::Value::Null;
            value["plot_name"] = serde_json::Value::Null;
            value["scale"]["name"] = serde_json::Value::Null;
            value["scale"]["type"] = serde_json::Value::Null;
            value["signals"][0]["type"] = serde_json::Value::Null;
        }
        std::fs::write(&input, serde_json::to_vec(&value).unwrap()).unwrap();
        let result = cli(&[
            "convert",
            input.to_str().unwrap(),
            output.to_str().unwrap(),
            "--to",
            "json",
        ]);
        assert!(result.status.success(), "{result:?}");
        let actual = read_json(&output);
        assert_eq!(actual["analysis"], "converted");
        assert_eq!(actual["plot_name"], "Converted Data");
        assert_eq!(actual["scale"]["name"], "scale");
        assert_eq!(actual["scale"]["type"], "value");
        assert_eq!(actual["signals"][0]["type"], "current");
        assert_eq!(
            actual["signals"][0]["values"],
            serde_json::json!([-1.0, 2.0])
        );
    }
}

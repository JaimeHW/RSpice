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

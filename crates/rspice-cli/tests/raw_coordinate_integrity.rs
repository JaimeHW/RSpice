//! A RAW coordinate cannot silently lose its imaginary component at admission.
mod common;

use common::{read_json, test_dir};
use std::path::Path;
use std::process::{Command, Output};

fn cli(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--quiet")
        .args(args)
        .output()
        .unwrap()
}

fn raw(plot: &str, imaginary: [f64; 2], binary: bool) -> Vec<u8> {
    let mut bytes = format!("Title: coordinate\nPlotname: {plot}\nFlags: complex double\nNo. Variables: 2\nNo. Points: 2\nVariables:\n0 frequency frequency\n1 V(out) voltage\n").into_bytes();
    bytes.extend_from_slice(if binary { b"Binary:\n" } else { b"Values:\n" });
    for (index, imaginary) in imaginary.into_iter().enumerate() {
        let values = [index as f64 + 1.0, imaginary, index as f64 + 2.0, 3.0];
        if binary {
            for value in values {
                bytes.extend_from_slice(&value.to_le_bytes());
            }
        } else {
            bytes.extend_from_slice(
                format!(
                    "{index} {},{} {},{}\n",
                    values[0], values[1], values[2], values[3]
                )
                .as_bytes(),
            );
        }
    }
    bytes
}

fn convert(input: &Path, output: &Path, format: &str, extra: &[&str]) -> Output {
    let mut args = vec![
        "convert",
        input.to_str().unwrap(),
        output.to_str().unwrap(),
        "--to",
        format,
    ];
    args.extend(extra);
    cli(&args)
}

fn coordinate_error(output: &Output) {
    assert_eq!(output.status.code(), Some(1), "{output:?}");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("nonzero imaginary component"),
        "{output:?}"
    );
}

#[test]
fn complex_raw_coordinates_cannot_be_converted_compared_or_blessed() {
    let directory = test_dir("raw_complex_coordinate");
    let input = directory.join("input.raw");
    let golden = directory.join("golden.raw");
    let missing = directory.join("missing.raw");
    for binary in [false, true] {
        let original = raw("AC Analysis", [0.0, 0.0], binary);
        std::fs::write(&golden, &original).unwrap();
        for imaginary in [[7.0, 9.0], [0.0, f64::from_bits(1)]] {
            std::fs::write(&input, raw("AC Analysis", imaginary, binary)).unwrap();
            for format in ["json", "csv", "tsv", "raw", "ascii", "hdf5", "vcd"] {
                let output = directory.join(format!("protected.{format}"));
                std::fs::write(&output, "predecessor").unwrap();
                coordinate_error(&convert(&input, &output, format, &[]));
                assert_eq!(std::fs::read_to_string(&output).unwrap(), "predecessor");
            }
            for bless in [false, true] {
                let mut args = vec!["compare", input.to_str().unwrap(), golden.to_str().unwrap()];
                if bless {
                    args.push("--bless");
                }
                coordinate_error(&cli(&args));
                assert_eq!(std::fs::read(&golden).unwrap(), original);
            }
            coordinate_error(&cli(&[
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
fn real_complex_encoded_axes_and_complex_operating_points_remain_lossless() {
    let directory = test_dir("raw_coordinate_kinds");
    let input = directory.join("input.raw");
    let output = directory.join("result.json");
    for binary in [false, true] {
        std::fs::write(&input, raw("AC Analysis", [0.0, -0.0], binary)).unwrap();
        let result = convert(&input, &output, "json", &[]);
        assert!(result.status.success(), "{result:?}");
        let data = read_json(&output);
        assert_eq!(data["scale"]["values"], serde_json::json!([1.0, 2.0]));
        assert_eq!(data["signals"][0]["imag"], serde_json::json!([3.0, 3.0]));

        for title in ["DC OP", "Operating Point", "DC Operating Point"] {
            std::fs::write(&input, raw(title, [7.0, 9.0], binary)).unwrap();
            let result = convert(&input, &output, "json", &[]);
            assert!(result.status.success(), "{title}: {result:?}");
            let data = read_json(&output);
            assert_eq!(data["scale"]["name"], "point");
            assert_eq!(data["scale"]["values"], serde_json::json!([0.0, 1.0]));
            assert_eq!(data["signals"].as_array().unwrap().len(), 2);
            assert_eq!(data["signals"][0]["imag"], serde_json::json!([7.0, 9.0]));
            let encoded = directory.join("cycle.raw");
            let result = convert(&output, &encoded, "raw", &[]);
            assert!(result.status.success(), "{result:?}");
            let result = convert(&encoded, &output, "json", &[]);
            assert!(result.status.success(), "{result:?}");
            let recovered = read_json(&output);
            assert_eq!(recovered["scale"], data["scale"]);
            assert_eq!(recovered["signals"], data["signals"]);
        }
    }
}

#[test]
fn unselected_raw_coordinates_are_checked_before_table_or_event_projection() {
    let directory = test_dir("raw_companion_coordinate");
    let input = directory.join("container.raw");
    for binary in [false, true] {
        let mut bytes = raw("AC Analysis", [0.0, 7.0], binary);
        bytes.extend_from_slice(b"Title: Events\nPlotname: Digital Events (rspice-digital-events/1)\nFlags: real double\nNo. Variables: 2\nNo. Points: 2\nVariables:\n0 time time\n1 D(clk) digital\nValues:\n0 0 0\n1 1 1\n");
        std::fs::write(&input, bytes).unwrap();
        for (format, extra) in [
            ("json", &["--section", "2"][..]),
            ("vcd", &[][..]),
            ("vcd", &["--section", "2"][..]),
        ] {
            let output = directory.join(format!("protected.{format}"));
            std::fs::write(&output, "predecessor").unwrap();
            coordinate_error(&convert(&input, &output, format, extra));
            assert_eq!(std::fs::read_to_string(&output).unwrap(), "predecessor");
        }
    }
}

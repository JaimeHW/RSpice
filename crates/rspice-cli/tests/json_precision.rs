//! Refuse numeric information loss before comparison or publication.
mod common;

use std::process::Command;

fn waveform(coordinate: &str, real: &str, imag: &str) -> String {
    format!(
        r#"{{"metadata":{{"123":"escaped \\\"1e-999\\\" and \\123", "nested":[42,-3.5,null]}},"scale":{{"name":"time","values":[{coordinate},1]}},"signals":[{{"name":"V(out)","real":[{real},2],"imag":[{imag},3]}},{{"name":"D(clk)","values":[0,1]}}]}}"#
    )
}

fn refuses_without_publication(literal: &str, expected: &str) {
    let dir = common::test_dir("json_precision_refusal");
    let input = dir.join("source.json");
    let output = dir.join("output.json");
    let golden = dir.join("golden.json");
    let missing = dir.join("missing.json");
    let baseline = waveform("0", "0", "0");
    std::fs::write(&golden, &baseline).unwrap();
    for source in [
        waveform("0", literal, "0"),
        waveform("0", "0", literal),
        waveform(literal, "0", "0"),
        format!(
            r#"{{"scale":{{"name":"time","values":[0,1]}},"signals":[{{"name":"V(out)","values":[{literal},2]}},{{"name":"D(clk)","values":[0,1]}}]}}"#
        ),
    ] {
        std::fs::write(&input, &source).unwrap();
        std::fs::write(&output, "preserve existing output").unwrap();
        for (operation, destination) in [
            ("convert", &output),
            ("convert", &input),
            ("compare", &golden),
            ("bless", &golden),
            ("bless", &missing),
        ] {
            let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
            command.args([
                "--quiet",
                if operation == "bless" {
                    "compare"
                } else {
                    operation
                },
            ]);
            command.arg(&input).arg(destination);
            if operation == "convert" {
                command.args(["--to", "json"]);
            } else if operation == "bless" {
                command.arg("--bless");
            }
            // An unselected bad signal must not disappear behind filtering.
            command.args(["--variables", "D(clk)"]);
            let result = command.output().unwrap();
            assert_eq!(
                result.status.code(),
                Some(1),
                "{literal}/{operation}: {result:?}"
            );
            let error = String::from_utf8_lossy(&result.stderr);
            assert!(error.contains(expected), "{literal}: {error}");
            assert_eq!(
                std::fs::read_to_string(&output).unwrap(),
                "preserve existing output"
            );
            assert_eq!(std::fs::read_to_string(&golden).unwrap(), baseline);
            assert_eq!(std::fs::read_to_string(&input).unwrap(), source);
            assert!(!missing.exists());
        }
    }
}

#[test]
fn json_decimal_underflow_cannot_be_converted_compared_or_blessed_as_zero() {
    for literal in ["1e-999", "-1e-999", "2e-324", "-2e-324"] {
        refuses_without_publication(literal, "underflow");
    }
}

#[test]
fn json_integer_samples_cannot_be_silently_rounded() {
    for literal in [
        "9007199254740993",
        "-9007199254740993",
        "9223372036854775807",
        "-9223372036854775809",
        "18446744073709551615",
        "18446744073709551617",
        "-18446744073709551617",
        "1234567890123456789012345678901234567890",
    ] {
        refuses_without_publication(literal, "cannot be represented exactly");
    }
}

#[test]
fn json_representable_boundary_values_preserve_every_bit() {
    let dir = common::test_dir("json_precision_boundaries");
    let input = dir.join("source.json");
    let output = dir.join("output.json");
    for (literal, expected) in [
        ("0e999", 0.0_f64),
        ("-0e-999", -0.0_f64),
        ("-0", -0.0_f64),
        ("5e-324", 5e-324_f64),
        ("-5e-324", -5e-324_f64),
        ("9007199254740992", 9007199254740992.0_f64),
        ("9007199254740994", 9007199254740994.0_f64),
        ("-9007199254740994", -9007199254740994.0_f64),
        ("-9223372036854775808", -9223372036854775808.0_f64),
        ("18446744073709551616", 18446744073709551616.0_f64),
        ("-18446744073709551616", -18446744073709551616.0_f64),
        ("1.0000000000000002", f64::from_bits(1.0_f64.to_bits() + 1)),
    ] {
        std::fs::write(&input, waveform("0", literal, literal)).unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "convert"])
            .arg(&input)
            .arg(&output)
            .args(["--to", "json"])
            .output()
            .unwrap();
        assert!(result.status.success(), "{literal}: {result:?}");
        let decoded = common::read_json(&output);
        for component in ["real", "imag"] {
            assert_eq!(
                decoded["signals"][0][component][0]
                    .as_f64()
                    .unwrap()
                    .to_bits(),
                expected.to_bits()
            );
        }
    }
}

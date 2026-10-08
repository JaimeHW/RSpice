//! Text samples must not disappear before conversion, comparison or blessing.
mod common;

use std::process::Command;

#[test]
fn decimal_underflow_refuses_conversion_comparison_and_blessing_without_publication() {
    let dir = common::test_dir("decimal_underflow_refusal");
    for (extension, separator) in [("csv", ','), ("tsv", '\t')] {
        for (name, source) in [
            ("coordinate", "time,v\n1e-999,1\n1,2\n"),
            ("real", "time,v\n0,1e-999\n1,2\n"),
            ("negative", "time,v\n0,-1e-999\n1,2\n"),
            ("complex_real", "time,Re(v),Im(v)\n0,1e-999,0\n1,2,3\n"),
            ("complex_imag", "time,Re(v),Im(v)\n0,0,1e-999\n1,2,3\n"),
            ("operating_point", "Signal,Value\nV(out),1e-999\n"),
        ] {
            let source = source.replace(',', &separator.to_string());
            let input = dir.join(format!("{name}.{extension}"));
            let golden = dir.join(format!("{name}.golden.{extension}"));
            let converted = dir.join(format!("{name}.json"));
            let baseline = source.replace("1e-999", "0");
            std::fs::write(&input, source).unwrap();
            std::fs::write(&golden, &baseline).unwrap();
            std::fs::write(&converted, "preserve existing output").unwrap();
            for operation in ["convert", "compare", "bless"] {
                let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
                command.arg("--quiet");
                if operation == "convert" {
                    command
                        .args(["convert"])
                        .arg(&input)
                        .arg(&converted)
                        .args(["--to", "json"]);
                } else {
                    command.arg("compare").arg(&input).arg(&golden);
                    if operation == "bless" {
                        command.arg("--bless");
                    }
                }
                let result = command.output().unwrap();
                assert!(
                    !result.status.success(),
                    "{extension}/{name}/{operation}: {result:?}"
                );
                let error = String::from_utf8_lossy(&result.stderr);
                assert!(
                    error.contains("underflow") && error.contains("row 2"),
                    "{error}"
                );
                assert_eq!(
                    std::fs::read_to_string(&converted).unwrap(),
                    "preserve existing output"
                );
                assert_eq!(std::fs::read_to_string(&golden).unwrap(), baseline);
            }
        }
    }
}

#[test]
fn authored_zero_and_representable_subnormals_remain_exact_in_text_conversion() {
    let dir = common::test_dir("decimal_underflow_boundaries");
    for (extension, separator) in [("csv", ','), ("tsv", '\t')] {
        let input = dir.join(format!("source.{extension}"));
        let output = dir.join("output.json");
        for (literal, expected) in [
            ("0e999", 0.0_f64),
            ("-0e-999", -0.0_f64),
            ("+0.000e-45", 0.0_f64),
            ("5e-324", 5e-324_f64),
            ("-5e-324", -5e-324_f64),
        ] {
            std::fs::write(&input, format!("time{separator}v\n0{separator}{literal}\n")).unwrap();
            let result = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "convert"])
                .arg(&input)
                .arg(&output)
                .args(["--to", "json"])
                .output()
                .unwrap();
            assert!(result.status.success(), "{literal}: {result:?}");
            let decoded = common::read_json(&output);
            assert_eq!(
                decoded["signals"][0]["values"][0]
                    .as_f64()
                    .unwrap()
                    .to_bits(),
                expected.to_bits()
            );
        }
    }
}

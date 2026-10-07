//! Numeric admission must precede selection and stop at the first excess value.
mod common;

use common::{read_json, test_dir};
use std::process::{Command, Output};

fn cli(args: &[&str], resource: &str, limit: usize) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json"])
        .args(args)
        .env_remove("RSPICE_MAX_EXTERNAL_DATA_VALUES")
        .env_remove("RSPICE_MAX_RESULT_VALUES")
        .env(resource, limit.to_string())
        .output()
        .unwrap()
}

#[test]
fn json_value_budgets_cover_every_component_before_selection_and_publication() {
    let directory = test_dir("json_streaming_admission");
    let input = directory.join("input.json");
    let golden = directory.join("golden.json");
    let missing = directory.join("missing.json");
    let source = serde_json::json!({
        // Unrelated metadata is not a table sample. Nested objects with the
        // same field names cannot accidentally become waveform coordinates.
        "schema": {"vendor_metadata": [1,2,3]},
        "metadata": {"scale": {"values": [0,1,2,3,4,5,6,7,8,9]}},
        "scale": {"name": "time", "values": [0.0,1.0]},
        "signals": [
            {"name": "D(clk)", "values": [0.0,1.0]},
            {"name": "V(out)", "real": [2.0,3.0], "imag": [4.0,5.0]},
        ],
    });
    // Escaped key spellings participate in admission after JSON decoding.
    let original = serde_json::to_string(&source)
        .unwrap()
        .replace("\"values\"", "\"\\u0076alues\"");
    std::fs::write(&input, &original).unwrap();
    std::fs::write(&golden, &original).unwrap();
    for (variable, resource) in [
        ("RSPICE_MAX_EXTERNAL_DATA_VALUES", "external_data_values"),
        ("RSPICE_MAX_RESULT_VALUES", "result_values"),
    ] {
        for limit in [0, 1, 3, 7] {
            let check = |output: Output| {
                assert_eq!(output.status.code(), Some(75), "{output:?}");
                let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
                assert_eq!(error["error"]["resource"], resource);
                assert_eq!(error["error"]["requested"], limit + 1);
                assert_eq!(error["error"]["limit"], limit);
            };
            for format in ["json", "csv", "tsv", "raw", "ascii", "hdf5", "vcd"] {
                let output = directory.join(format!("protected.{format}"));
                std::fs::write(&output, "predecessor").unwrap();
                check(cli(
                    &[
                        "convert",
                        input.to_str().unwrap(),
                        output.to_str().unwrap(),
                        "--to",
                        format,
                        "--variables",
                        "D(clk)",
                    ],
                    variable,
                    limit,
                ));
                assert_eq!(std::fs::read_to_string(&output).unwrap(), "predecessor");
            }
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
                check(cli(&args, variable, limit));
                assert_eq!(std::fs::read_to_string(&golden).unwrap(), original);
            }
            check(cli(
                &[
                    "compare",
                    input.to_str().unwrap(),
                    missing.to_str().unwrap(),
                    "--bless",
                ],
                variable,
                limit,
            ));
            assert!(!missing.exists());
        }
        // Exactly eight retained values, including the unselected complex
        // signal, are valid under either policy.
        let output = directory.join("admitted.json");
        let result = cli(
            &[
                "convert",
                input.to_str().unwrap(),
                output.to_str().unwrap(),
                "--to",
                "json",
            ],
            variable,
            8,
        );
        assert!(result.status.success(), "{result:?}");
        let actual = read_json(&output);
        assert_eq!(actual["scale"]["values"], source["scale"]["values"]);
        assert_eq!(actual["signals"][1]["real"], source["signals"][1]["real"]);
        assert_eq!(actual["signals"][1]["imag"], source["signals"][1]["imag"]);
    }
}

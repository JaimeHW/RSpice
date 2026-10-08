//! Early RAW admission must reach every file consumer and preserve destinations.
mod common;

use common::test_dir;
use std::process::Command;

#[test]
fn inferred_ascii_limits_precede_tail_decoding_and_publication() {
    let directory = test_dir("raw_inferred_admission");
    let input = directory.join("input.raw");
    let golden = directory.join("golden.raw");
    let missing = directory.join("missing.raw");
    let mut source = b"Title: admission\nPlotname: Transient Analysis\nFlags: real\nNo. Variables: 2\nNo. Points: 0\nVariables:\n0 time time\n1 V(out) voltage\nValues:\n0 0 1\n1 1 2\n".to_vec();
    source.extend([0xff, b'\n']);
    std::fs::write(&input, &source).unwrap();
    std::fs::write(&golden, "predecessor").unwrap();

    for (variable, resource, limit) in [
        ("RSPICE_MAX_EXTERNAL_DATA_VALUES", "external_data_values", 2),
        ("RSPICE_MAX_RESULT_VALUES", "result_values", 4),
    ] {
        let check = |args: &[&str]| {
            let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "--error-format", "json"])
                .args(args)
                .env_remove("RSPICE_MAX_EXTERNAL_DATA_VALUES")
                .env_remove("RSPICE_MAX_RESULT_VALUES")
                .env(variable, limit.to_string())
                .output()
                .unwrap();
            assert_eq!(output.status.code(), Some(75), "{output:?}");
            let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
            assert_eq!(error["error"]["resource"], resource);
            assert_eq!(error["error"]["requested"], 2 * limit);
            assert_eq!(error["error"]["limit"], limit);
        };
        for format in ["json", "csv", "tsv", "raw", "ascii", "hdf5", "vcd"] {
            let output = directory.join(format!("protected.{format}"));
            std::fs::write(&output, "predecessor").unwrap();
            check(&[
                "convert",
                input.to_str().unwrap(),
                output.to_str().unwrap(),
                "--to",
                format,
            ]);
            assert_eq!(std::fs::read_to_string(output).unwrap(), "predecessor");
        }
        check(&[
            "convert",
            input.to_str().unwrap(),
            input.to_str().unwrap(),
            "--to",
            "raw",
        ]);
        assert_eq!(std::fs::read(&input).unwrap(), source);
        for bless in [false, true] {
            let mut args = vec!["compare", input.to_str().unwrap(), golden.to_str().unwrap()];
            if bless {
                args.push("--bless");
            }
            check(&args);
            assert_eq!(std::fs::read_to_string(&golden).unwrap(), "predecessor");
        }
        check(&[
            "compare",
            input.to_str().unwrap(),
            missing.to_str().unwrap(),
            "--bless",
        ]);
        assert!(!missing.exists());
    }
}

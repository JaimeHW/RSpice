//! Result admission failures retain their category across CLI readers.
mod common;

use common::test_dir;
use std::process::Command;

#[test]
fn result_readers_preserve_limit_diagnostics_and_destinations() {
    let dir = test_dir("result_read_limits");
    for (format, content) in [
        (
            "vcd",
            "$timescale 1 ns $end\n$scope module test $end\n$var wire 2 ! bus [1:0] $end\n$upscope $end\n$enddefinitions $end\n#0\nb01 !\n#10\nb10 !\n",
        ),
        (
            "raw",
            "Title: digital grid\nDate: test\nPlotname: Transient Analysis\nFlags: real\nNo. Variables: 2\nNo. Points: 2\nVariables:\n0 time time\n1 D(clk) digital\nValues:\n0 0 0\n1 1e-9 1\n",
        ),
    ] {
        let source = dir.join(format!("source.{format}"));
        std::fs::write(&source, content).unwrap();
        let valid = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "convert"])
            .arg(&source)
            .arg(dir.join("valid.vcd"))
            .args(["--to", "vcd"])
            .output()
            .unwrap();
        assert!(valid.status.success(), "{format}: {valid:?}");
        for resource in [
            "max_external_data_bytes",
            "max_external_data_values",
            "max_result_values",
        ] {
            let config = dir.join(format!("{resource}.toml"));
            std::fs::write(&config, format!("[resources]\n{resource}=1\n")).unwrap();
            for operation in ["compare", "bless", "table", "vcd"] {
                let extension = match operation {
                    "table" => "csv",
                    "vcd" => "vcd",
                    _ => format,
                };
                let destination = dir.join(format!("{operation}-{resource}.{extension}"));
                let original = b"preserve destination";
                std::fs::write(&destination, original).unwrap();
                let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
                command
                    .arg("--config")
                    .arg(&config)
                    .args(["--quiet", "--error-format", "json"]);
                if matches!(operation, "compare" | "bless") {
                    command.arg("compare").arg(&source).arg(&destination);
                    if operation == "bless" {
                        command.arg("--bless");
                    }
                } else {
                    command
                        .arg("convert")
                        .arg(&source)
                        .arg(&destination)
                        .args(["--to", extension]);
                }
                let output = command.output().unwrap();
                assert_eq!(
                    output.status.code(),
                    Some(75),
                    "{format}, {operation}, {resource}: {output:?}"
                );
                let report: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
                assert_eq!(report["error"]["code"], "resource_limit", "{report}");
                assert_eq!(report["error"]["limit"], 1);
                assert!(report["error"]["requested"].as_u64().unwrap() > 1);
                assert_eq!(report["error"]["path"], source.to_str().unwrap());
                assert_eq!(std::fs::read(&destination).unwrap(), original);
            }
        }
    }
}

#[test]
fn result_io_failures_keep_the_source_path_and_io_exit_status() {
    let dir = test_dir("result_input_io");
    for format in ["raw", "vcd", "csv", "tsv", "json", "h5", "s1p"] {
        let source = dir.join(format!("directory.{format}"));
        std::fs::create_dir(&source).unwrap();
        for operation in ["compare", "bless", "table", "vcd"] {
            let extension = if operation == "table" { "csv" } else { format };
            let destination = dir.join(format!("{operation}.{extension}"));
            let original = b"preserve destination";
            std::fs::write(&destination, original).unwrap();
            let mut command = Command::new(env!("CARGO_BIN_EXE_rspice"));
            command.args(["--quiet", "--error-format", "json"]);
            if matches!(operation, "compare" | "bless") {
                command.arg("compare").arg(&source).arg(&destination);
                if operation == "bless" {
                    command.arg("--bless");
                }
            } else {
                command
                    .arg("convert")
                    .arg(&source)
                    .arg(&destination)
                    .args(["--to", if operation == "vcd" { "vcd" } else { "csv" }]);
            }
            let output = command.output().unwrap();
            assert_eq!(
                output.status.code(),
                Some(74),
                "{format}, {operation}: {output:?}"
            );
            let report: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
            assert_eq!(report["error"]["code"], "input_read_error", "{report}");
            assert_eq!(report["error"]["path"], source.to_str().unwrap());
            assert_eq!(std::fs::read(&destination).unwrap(), original);
        }
    }
}

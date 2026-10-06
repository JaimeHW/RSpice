//! VCD admission failures retain resource-limit status across CLI readers.
mod common;

use common::test_dir;
use std::process::Command;

#[test]
fn vcd_readers_preserve_limit_diagnostics_and_destinations() {
    let dir = test_dir("vcd_read_limits");
    let source = dir.join("source.vcd");
    std::fs::write(&source, "$timescale 1 ns $end\n$scope module test $end\n$var wire 2 ! bus [1:0] $end\n$upscope $end\n$enddefinitions $end\n#0\nb01 !\n#10\nb10 !\n").unwrap();
    for resource in [
        "max_external_data_bytes",
        "max_external_data_values",
        "max_result_values",
    ] {
        let config = dir.join(format!("{resource}.toml"));
        std::fs::write(&config, format!("[resources]\n{resource}=1\n")).unwrap();
        for operation in ["compare", "bless", "table", "vcd"] {
            let extension = if operation == "table" { "csv" } else { "vcd" };
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
                "{operation}, {resource}: {output:?}"
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

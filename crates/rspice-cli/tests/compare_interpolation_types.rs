//! Either input can carry the metadata lost by an untyped CSV/TSV export.
mod common;

use serde_json::json;
use std::path::Path;
use std::process::Command;

fn converted(source: &Path, destination: &Path, format: &str) {
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(source)
        .arg(destination)
        .args(["--to", format])
        .output()
        .unwrap();
    assert!(output.status.success(), "{format}: {output:?}");
}

#[test]
fn interpolation_uses_logic_metadata_from_either_input_in_every_typed_format() {
    let directory = common::test_dir("comparison_interpolation_types");
    for metadata in [
        json!({"type":"logic"}),
        json!({"type":"DIGITAL"}),
        json!({"unit":"logic"}),
    ] {
        for (format, extension) in [
            ("json", "json"),
            ("raw", "raw"),
            ("ascii", "ascii.raw"),
            ("hdf5", "h5"),
        ] {
            for typed_result in [false, true] {
                for ramped in [false, true] {
                    let times = if typed_result {
                        vec![0, 2, 4]
                    } else {
                        vec![0, 1, 2, 3, 4]
                    };
                    let values = if typed_result {
                        vec![0.0, 1.0, 0.0]
                    } else if ramped {
                        vec![0.0, 0.5, 1.0, 0.5, 0.0]
                    } else {
                        vec![0.0, 0.0, 1.0, 1.0, 0.0]
                    };
                    let mut signal = metadata.clone();
                    signal["name"] = json!("clock");
                    signal["values"] = json!(values);
                    let source = directory.join("source.json");
                    std::fs::write(
                        &source,
                        json!({
                            "scale":{"name":"time", "values":times},
                            "signals":[signal]
                        })
                        .to_string(),
                    )
                    .unwrap();
                    let typed = directory.join(format!("typed.{extension}"));
                    converted(&source, &typed, format);
                    let untyped = directory.join("untyped.csv");
                    std::fs::write(
                        &untyped,
                        if !typed_result {
                            "time,clock\n0,0\n2,1\n4,0\n"
                        } else if ramped {
                            "time,clock\n0,0\n1,0.5\n2,1\n3,0.5\n4,0\n"
                        } else {
                            "time,clock\n0,0\n1,0\n2,1\n3,1\n4,0\n"
                        },
                    )
                    .unwrap();
                    let (result, golden) = if typed_result {
                        (&typed, &untyped)
                    } else {
                        (&untyped, &typed)
                    };
                    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                        .args(["--quiet", "compare"])
                        .arg(result)
                        .arg(golden)
                        .args([
                            "--interpolate",
                            "--json",
                            "--variables",
                            "clock",
                            "--abstol",
                            "0",
                            "--reltol",
                            "0",
                        ])
                        .output()
                        .unwrap();
                    assert_eq!(
                        output.status.code(),
                        Some(if ramped { 3 } else { 0 }),
                        "metadata={metadata}, format={format}, typed_result={typed_result}, ramped={ramped}: {output:?}"
                    );
                    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
                    assert_eq!(report["num_differences"], if ramped { 2 } else { 0 });
                    assert_eq!(report["num_points"], 5);
                    assert_eq!(report["problems"], json!([]));
                }
            }
        }
    }
}

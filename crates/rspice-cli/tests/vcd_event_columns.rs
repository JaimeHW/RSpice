//! Named event columns must not disappear from an otherwise successful dump.
mod common;

use serde_json::json;
use std::path::Path;
use std::process::{Command, Output};

fn convert(input: &Path, output: &Path, format: &str) -> Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "--error-format", "json", "convert"])
        .arg(input)
        .arg(output)
        .args(["--to", format])
        .output()
        .unwrap()
}

#[test]
fn unsupported_event_columns_cannot_publish_partial_dumps() {
    let directory = common::test_dir("vcd_event_columns");
    for name in ["D(data)", "E(sample)"] {
        for (payload, cause) in [
            (json!({"values":[1,null]}), "undefined"),
            (json!({"real":[0,1],"imag":[1,2]}), "complex"),
            (json!({"real":[null,1],"imag":[null,2]}), "complex"),
        ] {
            let mut event = payload;
            event["name"] = json!(name);
            let source = directory.join("source.json");
            std::fs::write(
                &source,
                json!({
                    "scale":{"name":"time","values":[0,1e-9]},
                    "signals":[{"name":"D(clk)","values":[0,1]},event]
                })
                .to_string(),
            )
            .unwrap();
            for (format, extension) in [
                ("json", "json"),
                ("csv", "csv"),
                ("tsv", "tsv"),
                ("raw", "raw"),
                ("ascii", "ascii.raw"),
                ("hdf5", "h5"),
            ] {
                let input = directory.join(format!("input.{extension}"));
                let encoded = convert(&source, &input, format);
                assert!(encoded.status.success(), "{format}: {encoded:?}");
                for existing in [false, true] {
                    let output = directory.join(format!("events-{existing}.vcd"));
                    if existing {
                        std::fs::write(&output, "existing dump").unwrap();
                    }
                    let converted = convert(&input, &output, "vcd");
                    assert_eq!(
                        converted.status.code(),
                        Some(1),
                        "{name}/{cause}/{format}: {converted:?}"
                    );
                    let diagnostic: serde_json::Value =
                        serde_json::from_slice(&converted.stderr).unwrap();
                    let message = diagnostic["error"]["message"].as_str().unwrap();
                    assert!(message.contains(name), "{diagnostic}");
                    assert!(message.contains(cause), "{diagnostic}");
                    if existing {
                        assert_eq!(std::fs::read_to_string(&output).unwrap(), "existing dump");
                    } else {
                        assert!(!output.exists());
                    }
                }
                let original = std::fs::read(&input).unwrap();
                let converted = convert(&input, &input, "vcd");
                assert_eq!(converted.status.code(), Some(1), "{converted:?}");
                assert_eq!(std::fs::read(&input).unwrap(), original);
            }
        }
    }
}

#[test]
fn unrelated_analog_columns_do_not_hide_or_prevent_event_exports() {
    let directory = common::test_dir("vcd_analog_columns");
    let source = directory.join("source.json");
    let output = directory.join("events.vcd");
    std::fs::write(
        &source,
        json!({
            "scale":{"name":"time","values":[0,1e-9]},
            "signals":[
                {"name":"undefined_analog","values":[null,1]},
                {"name":"D(clk)","values":[0,1]},
                {"name":"complex_analog","real":[1,2],"imag":[3,4]},
                {"name":"E(sample)","values":[2,3]}
            ]
        })
        .to_string(),
    )
    .unwrap();
    let converted = convert(&source, &output, "vcd");
    assert!(converted.status.success(), "{converted:?}");
    let document = rspice_core::io::parse_vcd_file(&output).unwrap();
    assert_eq!(document.signals.len(), 2);
    for (signal, name) in document.signals.iter().zip(["clk", "sample"]) {
        assert_eq!(signal.variables[0].name, name);
        assert_eq!(signal.changes.len(), 2);
    }
}

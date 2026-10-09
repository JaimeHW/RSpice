//! Reject contradictory retained STB evidence before replacing any output.
mod common;

use serde_json::json;
use std::process::Command;

#[test]
fn malformed_stability_documents_cannot_be_exported_or_blessed() {
    let directory = common::test_dir("stb_result_validation");
    let deck = directory.join("loop.sp");
    let source = directory.join("loop.json");
    std::fs::write(&deck, "* loop\nE1 EO 0 CTRL 0 -1000\nVP EO X 0\nR1 X CTRL 1k\nC1 CTRL 0 159.154943091895n\n.stb dec 10 10 10meg probe=VP\n.end\n").unwrap();
    let run = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(deck)
        .arg("-o")
        .arg(&source)
        .args(["-f", "json"])
        .output()
        .unwrap();
    assert!(run.status.success(), "{run:?}");
    let original = common::read_json(&source);
    let altered = directory.join("altered.json");
    for case in [
        "db",
        "phase",
        "nyquist_value",
        "nyquist_coordinate",
        "negative_frequency",
        "wrong_unit",
        "margin",
    ] {
        let mut document = original.clone();
        match case {
            "db" | "phase" => {
                let name = if case == "db" {
                    "loop_gain_db"
                } else {
                    "loop_gain_phase"
                };
                let signal = document["signals"]
                    .as_array_mut()
                    .unwrap()
                    .iter_mut()
                    .find(|signal| signal["descriptor"]["canonicalName"] == name)
                    .unwrap();
                signal["values"]["samples"][0] = json!(300.0);
            }
            "nyquist_value" => document["payload"]["nyquist"][0]["real"] = json!(-10),
            "nyquist_coordinate" => document["payload"]["nyquist"][0]["frequency"] = json!(11),
            "negative_frequency" => document["axes"][0]["values"]["values"][0] = json!(-10),
            "wrong_unit" => document["axes"][0]["unit"] = json!({"unit":"volt"}),
            "margin" => {
                let scalar = document["scalars"]
                    .as_array_mut()
                    .unwrap()
                    .iter_mut()
                    .find(|scalar| scalar["name"] == "phase_margin_degrees")
                    .unwrap();
                scalar["value"]["value"] = json!(123);
            }
            _ => panic!("unknown case"),
        }
        std::fs::write(&altered, document.to_string()).unwrap();
        for (format, extension) in [
            ("json", "json"),
            ("csv", "csv"),
            ("tsv", "tsv"),
            ("raw", "raw"),
            ("ascii", "ascii.raw"),
            ("hdf5", "h5"),
        ] {
            let destination = directory.join(format!("protected.{extension}"));
            std::fs::write(&destination, "predecessor").unwrap();
            let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
                .args(["--quiet", "--error-format", "json", "convert"])
                .arg(&altered)
                .arg(&destination)
                .args(["--to", format])
                .output()
                .unwrap();
            assert_eq!(output.status.code(), Some(1), "{case}/{format}: {output:?}");
            let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
            assert_eq!(error["error"]["code"], "conversion_error");
            assert_eq!(std::fs::read_to_string(destination).unwrap(), "predecessor");
        }
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "compare"])
            .arg(&altered)
            .arg(&source)
            .arg("--bless")
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(1), "{case}: {output:?}");
        assert_eq!(common::read_json(&source), original);
    }
}

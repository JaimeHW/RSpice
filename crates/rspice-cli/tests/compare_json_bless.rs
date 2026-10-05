//! Integration tests for `rspice compare --json --bless`.

mod common;

use common::test_dir;

use std::process::Command;

#[test]
fn json_bless_reports_accepted_mismatch_consistently() {
    let dir = test_dir("mismatch");
    let result = dir.join("result.csv");
    let golden = dir.join("golden.csv");
    std::fs::write(&result, "time,V(OUT)\n0,1.0\n1e-6,2.5\n").unwrap();
    std::fs::write(&golden, "time,V(OUT)\n0,1.0\n1e-6,2.0\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            result.to_str().unwrap(),
            golden.to_str().unwrap(),
            "--json",
            "--bless",
        ])
        .output()
        .expect("run rspice");

    assert_eq!(
        output.status.code(),
        Some(0),
        "successful bless should exit 0; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|err| {
        panic!(
            "compare --json --bless should emit valid JSON: {err}; stdout: {}",
            String::from_utf8_lossy(&output.stdout)
        )
    });
    assert_eq!(
        json["passed"].as_bool(),
        Some(true),
        "JSON passed must match the successful accepted outcome: {json}"
    );
    assert_eq!(
        json["comparison_passed"].as_bool(),
        Some(false),
        "raw comparison result should still be visible: {json}"
    );
    assert_eq!(
        json["accepted"].as_bool(),
        Some(true),
        "blessed mismatch should be marked accepted: {json}"
    );
    assert_eq!(
        json["blessed"].as_bool(),
        Some(true),
        "JSON should state that the mismatch was blessed: {json}"
    );
    assert_eq!(
        json["num_differences"].as_u64(),
        Some(1),
        "mismatch details should be preserved: {json}"
    );
    assert_eq!(
        std::fs::read_to_string(&golden).unwrap(),
        std::fs::read_to_string(&result).unwrap(),
        "successful bless should update the golden file"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn bootstrap_bless_reports_a_single_json_document_without_quiet() {
    let dir = test_dir("bootstrap_json");
    let result = dir.join("result.csv");
    let golden = dir.join("golden.csv");
    std::fs::write(&result, "time,V(out)\n0,1\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("compare")
        .arg(&result)
        .arg(&golden)
        .args(["--json", "--bless"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["accepted"], true);
    assert_eq!(json["blessed"], true);
    assert_eq!(json["comparison_passed"], false);
    assert_eq!(
        std::fs::read(result).unwrap(),
        std::fs::read(golden).unwrap()
    );
}

#[test]
fn cross_format_bless_never_replaces_or_creates_a_mislabelled_golden() {
    let dir = test_dir("cross_format_bless");
    let csv = dir.join("source.csv");
    let raw = dir.join("source.raw");
    std::fs::write(&csv, "time,V(out)\n0,1\n").unwrap();
    let converted = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "convert"])
        .arg(&csv)
        .arg(&raw)
        .args(["--to", "raw"])
        .output()
        .unwrap();
    assert!(converted.status.success(), "{converted:?}");
    for existing in [false, true] {
        let golden = dir.join(format!("golden-{existing}.csv"));
        let original = b"time,V(out)\n0,2\n";
        if existing {
            std::fs::write(&golden, original).unwrap();
        }
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "compare"])
            .arg(&raw)
            .arg(&golden)
            .arg("--bless")
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        if existing {
            assert_eq!(std::fs::read(&golden).unwrap(), original);
        } else {
            assert!(!golden.exists());
        }
    }
}

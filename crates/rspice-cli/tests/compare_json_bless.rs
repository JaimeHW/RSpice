//! Integration tests for `rspice compare --json --bless`.

use std::path::PathBuf;
use std::process::Command;

fn test_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "rspice_compare_json_bless_{}_{}",
        std::process::id(),
        tag
    ));
    std::fs::create_dir_all(&dir).expect("create test dir");
    dir
}

#[test]
fn json_bless_reports_accepted_mismatch_consistently() {
    let dir = test_dir("mismatch");
    let result = dir.join("result.csv");
    let golden = dir.join("golden.csv");
    std::fs::write(&result, "time,V(OUT)\n0,1.0\n1e-6,2.5\n").unwrap();
    std::fs::write(&golden, "time,V(OUT)\n0,1.0\n1e-6,2.0\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
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

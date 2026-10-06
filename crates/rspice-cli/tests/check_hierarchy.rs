//! Topology checking uses the same hierarchy and resource policy as execution.
mod common;
use common::test_dir;
use std::process::Command;

#[test]
fn strict_check_rejects_a_voltage_loop_inside_a_subcircuit_as_json() {
    let dir = test_dir("hierarchical_loop");
    let deck = dir.join("loop.cir");
    std::fs::write(&deck, "Nested loop\n.subckt bad p n\nV1 p n 1\nV2 p n 2\n.ends\nX1 in 0 bad\nR1 in 0 1k\n.op\n.end\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("check")
        .arg(deck)
        .args(["--strict", "--connectivity", "--json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(65), "{output:?}");
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["valid"], false);
    assert!(
        json["errors"]
            .as_array()
            .unwrap()
            .iter()
            .any(|error| error["message"].as_str().unwrap().contains("closes a loop"))
    );
}

#[test]
fn check_does_not_ignore_hierarchy_admission_failures() {
    let dir = test_dir("hierarchy_limit");
    let config = dir.join("limits.toml");
    std::fs::write(&config, "[resources]\nmax_flattened_elements=1\n").unwrap();
    let deck = dir.join("nested.cir");
    std::fs::write(
        &deck,
        "Limited hierarchy\n.subckt load p n\nR1 p n 1k\nR2 p n 2k\n.ends\nX1 in 0 load\n.end\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--config")
        .arg(config)
        .args(["--error-format", "json", "check"])
        .arg(deck)
        .args(["--connectivity", "--json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(75), "{output:?}");
    let json: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["valid"], false);
}

#[test]
fn validation_errors_take_precedence_over_strict_warnings() {
    let dir = test_dir("strict_error_precedence");
    let deck = dir.join("deck.sp");
    std::fs::write(
        &deck,
        "* error and warning\n.option foobar=1\nV1 in 0 1\nV2 in 0 2\n.op\n.end\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "check"])
        .arg(deck)
        .args(["--strict", "--json"])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(65), "{output:?}");
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(!report["warnings"].as_array().unwrap().is_empty());
    assert!(!report["errors"].as_array().unwrap().is_empty());
}

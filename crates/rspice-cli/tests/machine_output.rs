mod common;
use std::process::Command;

#[test]
fn usage_errors_honor_json_and_help_remains_help() {
    for args in [
        vec!["--error-format", "json", "run"],
        vec!["run", "-", "--error-format=json", "--nonesuch"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(args)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2));
        let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
        assert_eq!(error["error"]["exit_code"], 2);
    }
    let help = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--error-format", "json", "--help"])
        .output()
        .unwrap();
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("Usage:"));
}

#[test]
fn stdout_summary_is_json_without_quiet_and_tf_obeys_quiet() {
    let dir = common::test_dir("machine_summary");
    let deck = dir.join("tf.sp");
    std::fs::write(
        &deck,
        "* TF\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.tf V(out) V1\n.end\n",
    )
    .unwrap();
    let summary = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--verbose")
        .arg("run")
        .arg(&deck)
        .args(["--summary", "-"])
        .output()
        .unwrap();
    assert!(summary.status.success(), "{summary:?}");
    let _: serde_json::Value = serde_json::from_slice(&summary.stdout).unwrap();
    let quiet = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(deck)
        .output()
        .unwrap();
    assert!(quiet.status.success(), "{quiet:?}");
    assert!(quiet.stdout.is_empty(), "{quiet:?}");
}

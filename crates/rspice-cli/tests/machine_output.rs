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

#[test]
fn stdout_summary_preserves_control_print_as_a_listed_json_artifact() {
    let dir = common::test_dir("control_summary");
    let deck = dir.join("op.sp");
    std::fs::write(
        &deck,
        "* OP\nV1 in 0 3\nR1 in 0 1k\n.control\nop\nprint V(in)\n.endc\n.end\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("run")
        .arg(&deck)
        .args(["--summary", "-"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let summary: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let artifact = summary["outputs"]
        .as_array()
        .unwrap()
        .iter()
        .find_map(|path| {
            path.as_str()
                .filter(|path| path.ends_with("control-001.json"))
        })
        .unwrap();
    let print: serde_json::Value =
        serde_json::from_slice(&std::fs::read(artifact).unwrap()).unwrap();
    assert_eq!(print["command"], "print");
    assert_eq!(print["traces"][0]["y"]["samples"][0][0], 3.0);
}

#[test]
fn operating_point_console_preserves_small_signal_magnitudes() {
    let dir = common::test_dir("op_console_precision");
    let deck = dir.join("deck.sp");
    std::fs::write(&deck, "* small OP\nV1 in 0 1p\nR1 in 0 1meg\n.op\n.end\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("run")
        .arg(deck)
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("1.000000000e-12 V"), "{text}");
    let current = text.lines().find(|line| line.contains("I(V1) =")).unwrap();
    let value: f64 = current
        .split('=')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .next()
        .unwrap()
        .parse()
        .unwrap();
    assert!((value + 1e-18).abs() < 1e-26, "{text}");
    assert!(current.contains("e-18 A"), "{text}");
}

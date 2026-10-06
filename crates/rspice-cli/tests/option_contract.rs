mod common;
use std::process::Command;
#[test]
fn options_cannot_be_silently_ignored() {
    let dir = common::test_dir("option_contract");
    let deck = dir.join("op.sp");
    std::fs::write(&deck, "* OP\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n").unwrap();
    for options in [
        ["--checkpoint", "ignored.checkpoint"],
        ["--resume", "missing.checkpoint"],
        ["--tran-stop", "1m"],
        ["--sens-output", "in"],
        ["--report-format", "junit"],
        ["--meas-format", "csv"],
    ] {
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "--error-format", "json", "run"])
            .arg(&deck)
            .args(options)
            .current_dir(&dir)
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{options:?}: {output:?}");
        let _: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    }
    assert!(!dir.join("ignored.checkpoint").exists());
    for temperature in ["-40", "-40m"] {
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["--temp", temperature])
            .output()
            .unwrap();
        assert!(output.status.success(), "{output:?}");
    }
}

#[test]
fn alternative_analysis_modes_cannot_silently_override_each_other() {
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--error-format",
            "json",
            "run",
            "-",
            "--corners",
            "tt",
            "--hb-freq",
            "1meg",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    let error: serde_json::Value = serde_json::from_slice(&output.stderr).unwrap();
    assert!(
        error["error"]["message"]
            .as_str()
            .unwrap()
            .contains("cannot be used with")
    );
}

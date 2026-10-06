mod common;
use std::process::Command;

#[test]
fn overlapping_result_checkpoint_and_report_paths_are_refused_without_overwriting() {
    let dir = common::test_dir("destination_collision");
    let deck = dir.join("deck.sp");
    std::fs::write(
        &deck,
        "* collision\nV1 in 0 1\nR1 in 0 1k\n.tran 1n 2n\n.end\n",
    )
    .unwrap();
    for flags in [
        vec!["--meas-file"],
        vec!["--summary"],
        vec!["--report-format", "junit", "--report-file"],
        vec!["--checkpoint"],
    ] {
        let destination = dir.join("result.csv");
        std::fs::write(&destination, "original bytes").unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["-f", "csv", "-o"])
            .arg(&destination)
            .args(flags)
            .arg(dir.join("./result.csv"))
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{output:?}");
        assert_eq!(
            std::fs::read_to_string(destination).unwrap(),
            "original bytes"
        );
    }
}

#[test]
fn derived_corner_paths_cannot_overwrite_reports_even_on_workers() {
    let dir = common::test_dir("derived_destination_collision");
    let deck = dir.join("deck.sp");
    std::fs::write(&deck, "* collision\nV1 in 0 1\nR1 in 0 1k\n.op\n.end\n").unwrap();
    let report = dir.join("result.ss.csv");
    std::fs::write(&report, "original bytes").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .args(["--corners", "tt,ss", "-j", "2", "-f", "csv", "-o"])
        .arg(dir.join("result.csv"))
        .arg("--meas-file")
        .arg(&report)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stderr).contains("collides"));
    assert_eq!(std::fs::read_to_string(report).unwrap(), "original bytes");
}

#[test]
fn reports_cannot_share_a_destination_with_each_other() {
    let dir = common::test_dir("report_destination_collision");
    let deck = dir.join("deck.sp");
    std::fs::write(&deck, "* OP\nV1 in 0 1\nR1 in 0 1k\n.end\n").unwrap();
    let report = dir.join("report.json");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(deck)
        .arg("--summary")
        .arg(&report)
        .arg("--meas-file")
        .arg(&report)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2), "{output:?}");
    assert!(!report.exists());
}

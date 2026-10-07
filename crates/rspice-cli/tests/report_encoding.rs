//! CI reports must keep user-authored text and full-precision measurements.
mod common;

use common::test_dir;
use std::process::Command;

#[test]
fn failed_runs_with_tap_directive_text_in_the_filename_stay_failures() {
    let directory = test_dir("tap_directive_filename");
    let deck = directory.join("check # TODO ignored.cir");
    std::fs::write(
        &deck,
        "Report encoding\nV1 n 0 1\nR1 n 0 1k\n.tran 1n 2n\n.meas tran missing WHEN V(n)=2\n.end\n",
    )
    .unwrap();
    let report = directory.join("report.tap");
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(["--quiet", "run"])
        .arg(&deck)
        .args(["--report-format", "tap", "--report-file"])
        .arg(&report)
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(3), "{output:?}");
    let text = std::fs::read_to_string(report).unwrap();
    let results: Vec<_> = text
        .lines()
        .filter(|line| line.starts_with("not ok "))
        .collect();
    assert_eq!(results.len(), 2, "{text}");
    assert!(results.iter().all(|line| !line.contains('#')), "{text}");
    for scalar in text
        .lines()
        .filter_map(|line| line.strip_prefix("  message: "))
    {
        assert!(!serde_json::from_str::<String>(scalar).unwrap().is_empty());
    }
}

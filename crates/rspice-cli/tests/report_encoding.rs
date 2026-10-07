//! CI reports must keep user-authored text and full-precision measurements.
mod common;

use common::test_dir;
use std::process::Command;

#[test]
fn measurement_csv_and_json_agree_at_adjacent_floating_point_values() {
    let directory = test_dir("measurement_csv_precision");
    let deck = directory.join("precision.cir");
    std::fs::write(&deck, "Report precision\nV1 n 0 1\nR1 n 0 1k\n.dc V1 1 2 1\n.meas dc value param='1.0000000000000002'\n.end\n").unwrap();
    let csv = directory.join("measurements.csv");
    let json = directory.join("measurements.json");
    for (format, path) in [("csv", &csv), ("json", &json)] {
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args(["--quiet", "run"])
            .arg(&deck)
            .args(["--meas-format", format, "--meas-file"])
            .arg(path)
            .output()
            .unwrap();
        assert!(output.status.success(), "{output:?}");
    }
    let csv = std::fs::read_to_string(csv).unwrap();
    let (header, row) = csv.trim_end().split_once('\n').unwrap();
    let fields: std::collections::HashMap<_, _> = header.split(',').zip(row.split(',')).collect();
    let json = common::read_json(&json);
    for field in ["value", "raw_value"] {
        let expected = json["measurements"][0][field].as_f64().unwrap();
        assert_eq!(expected, 1.0_f64.next_up());
        assert_eq!(
            fields[field].parse::<f64>().unwrap().to_bits(),
            expected.to_bits(),
            "{csv}"
        );
    }
}

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

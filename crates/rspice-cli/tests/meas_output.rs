//! Integration tests for `.MEAS` result display: the `--meas` stdout summary
//! must keep SPICE-scale magnitudes visible (a ~220ns risetime used to render
//! as `0.000000`), and the `--meas-file` writers must keep full precision.

use std::path::PathBuf;
use std::process::Command;

/// RC low-pass with tau = 100ns: risetime (10%..90% of a 5V step) is
/// ln(9)*tau ~= 219.7ns, far below fixed-point display resolution.
const SUB_MICROSECOND_DECK: &str = "* RC low-pass\n\
     V1 in 0 PULSE(0 5 0 1n 1n 1u 2u)\n\
     R1 in out 1k\n\
     C1 out 0 100p\n\
     .TRAN 1n 2u\n\
     .MEAS TRAN risetime TRIG V(out) VAL=0.5 RISE=1 TARG V(out) VAL=4.5 RISE=1\n\
     .END\n";

const EXPECTED_RISETIME: f64 = 2.197e-7;

fn test_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rspice_meas_out_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&dir).expect("create test dir");
    dir
}

fn run_rspice(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(args)
        .output()
        .expect("run rspice")
}

#[test]
fn meas_summary_displays_sub_microsecond_values() {
    let dir = test_dir("stdout");
    let deck = dir.join("rc.sp");
    std::fs::write(&deck, SUB_MICROSECOND_DECK).expect("write deck");

    let output = run_rspice(&["run", deck.to_str().unwrap(), "--meas"]);
    assert!(
        output.status.success(),
        "run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);

    let line = stdout
        .lines()
        .find(|line| line.trim_start().starts_with("RISETIME ="))
        .unwrap_or_else(|| panic!("no RISETIME line in output:\n{stdout}"));
    assert!(
        !line.contains("0.000000"),
        "sub-microsecond risetime must not display as zero: {line}"
    );

    let value: f64 = line
        .split('=')
        .nth(1)
        .expect("value field")
        .trim()
        .parse()
        .unwrap_or_else(|e| panic!("unparseable value in '{line}': {e}"));
    assert!(
        (value - EXPECTED_RISETIME).abs() < 5e-9,
        "displayed risetime should be ~219.7ns, got {value}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn meas_file_writers_keep_full_precision() {
    let dir = test_dir("files");
    let deck = dir.join("rc.sp");
    std::fs::write(&deck, SUB_MICROSECOND_DECK).expect("write deck");

    // JSON (default --meas-file format) carries the raw f64
    let json_path = dir.join("meas.json");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--meas-file",
        json_path.to_str().unwrap(),
    ]);
    assert!(
        output.status.success(),
        "json run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&json_path).expect("read json"))
            .expect("parse meas json");
    let value = json["measurements"][0]["value"]
        .as_f64()
        .expect("numeric measurement value");
    assert!(
        (value - EXPECTED_RISETIME).abs() < 5e-9,
        "JSON risetime should be ~219.7ns, got {value}"
    );

    // CSV uses scientific notation, so the magnitude survives round-trip
    let csv_path = dir.join("meas.csv");
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--meas-file",
        csv_path.to_str().unwrap(),
        "--meas-format",
        "csv",
    ]);
    assert!(
        output.status.success(),
        "csv run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let csv = std::fs::read_to_string(&csv_path).expect("read csv");
    let row = csv
        .lines()
        .find(|line| line.to_ascii_uppercase().contains("RISETIME"))
        .unwrap_or_else(|| panic!("no RISETIME row in csv:\n{csv}"));
    let value: f64 = row
        .split(',')
        .nth(2)
        .expect("value column")
        .parse()
        .unwrap_or_else(|e| panic!("unparseable value in '{row}': {e}"));
    assert!(
        (value - EXPECTED_RISETIME).abs() < 5e-9,
        "CSV risetime should be ~219.7ns, got {value}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

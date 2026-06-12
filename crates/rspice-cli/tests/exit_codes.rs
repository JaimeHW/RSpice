//! The exit-status contract automation relies on:
//! 0 = success, 1 = simulation error, 2 = usage error, 3 = verification
//! failure (failed .MEAS), 65 = parse error, 66 = missing input.

use std::path::PathBuf;
use std::process::Command;

fn test_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("rspice_exit_{}_{}", std::process::id(), tag));
    std::fs::create_dir_all(&dir).expect("create test dir");
    dir
}

fn run_rspice(args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(args)
        .output()
        .expect("run rspice")
}

const PASSING_DECK: &str = "* RC with a measurement that evaluates\n\
     V1 in 0 PULSE(0 5 0 1n 1n 1u 2u)\n\
     R1 in out 1k\n\
     C1 out 0 100p\n\
     .TRAN 1n 2u\n\
     .MEAS TRAN vmax MAX V(out)\n\
     .END\n";

const FAILING_MEAS_DECK: &str = "* RC with a measurement that cannot trigger\n\
     V1 in 0 PULSE(0 5 0 1n 1n 1u 2u)\n\
     R1 in out 1k\n\
     C1 out 0 100p\n\
     .TRAN 1n 2u\n\
     .MEAS TRAN never TRIG V(out) VAL=0.5 RISE=1 TARG V(out) VAL=9.9 RISE=1\n\
     .END\n";

#[test]
fn passing_measurements_exit_zero() {
    let dir = test_dir("pass");
    let deck = dir.join("pass.sp");
    std::fs::write(&deck, PASSING_DECK).expect("write deck");

    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn failed_measurement_exits_three() {
    let dir = test_dir("meas_fail");
    let deck = dir.join("fail.sp");
    std::fs::write(&deck, FAILING_MEAS_DECK).expect("write deck");

    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);
    assert_eq!(
        output.status.code(),
        Some(3),
        "failed .MEAS must exit 3; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("NEVER"),
        "stderr should name the failed measurement: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn allow_failed_meas_restores_exit_zero() {
    let dir = test_dir("meas_allow");
    let deck = dir.join("fail.sp");
    std::fs::write(&deck, FAILING_MEAS_DECK).expect("write deck");

    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--allow-failed-meas",
    ]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// Measurements for an analysis that never ran are failures, not skips.
#[test]
fn unevaluated_measurement_exits_three() {
    let dir = test_dir("unevaluated");
    let deck = dir.join("ac_meas.sp");
    std::fs::write(
        &deck,
        "* tran-only deck with an AC measurement\n\
         V1 in 0 PULSE(0 5 0 1n 1n 1u 2u)\n\
         R1 in out 1k\n\
         C1 out 0 100p\n\
         .TRAN 1n 2u\n\
         .MEAS AC gain MAX V(out)\n\
         .END\n",
    )
    .expect("write deck");

    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);
    assert_eq!(
        output.status.code(),
        Some(3),
        "unevaluated .MEAS must exit 3; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// Conflicting parallel voltage sources produce a singular system; the
/// NaN/Inf solution must be an error, not a quiet success.
#[test]
fn nonfinite_result_is_a_simulation_error() {
    let dir = test_dir("nonfinite");
    let deck = dir.join("vloop.sp");
    std::fs::write(
        &deck,
        "* conflicting parallel voltage sources\n\
         V1 a 0 5\n\
         V2 a 0 3\n\
         .op\n\
         .end\n",
    )
    .expect("write deck");

    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);
    assert_eq!(
        output.status.code(),
        Some(1),
        "non-finite OP must exit 1; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("non-finite"),
        "stderr should explain the non-finite solution: {stderr}"
    );

    // The escape hatch restores the old behavior for debugging.
    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--allow-nonfinite",
    ]);
    assert_eq!(output.status.code(), Some(0));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn parse_error_exits_sixty_five() {
    let dir = test_dir("parse");
    let deck = dir.join("broken.sp");
    std::fs::write(&deck, "* broken\nR1 in out\n.tran 1u 1m\n.end\n").expect("write deck");

    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);
    assert_eq!(output.status.code(), Some(65));

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn missing_input_exits_sixty_six() {
    let output = run_rspice(&["--quiet", "run", "definitely_missing_deck.sp"]);
    assert_eq!(output.status.code(), Some(66));
}

/// --timeout stops a long transient at the next safe point and exits with
/// the GNU timeout convention (124).
#[test]
fn timeout_exits_one_twenty_four() {
    let dir = test_dir("timeout");
    let deck = dir.join("slow.sp");
    // 100 simulated seconds of a 1kHz sine at 1ns steps: far longer than
    // the 1-second budget on any machine.
    std::fs::write(
        &deck,
        "* long transient\n\
         V1 in 0 SIN(0 1 1k)\n\
         R1 in out 1k\n\
         C1 out 0 100n\n\
         .tran 1n 100\n\
         .end\n",
    )
    .expect("write deck");

    let start = std::time::Instant::now();
    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap(), "--timeout", "1"]);
    assert_eq!(
        output.status.code(),
        Some(124),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        start.elapsed() < std::time::Duration::from_secs(30),
        "timeout must stop the run promptly"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// Failed measurements still land in report files alongside the bad exit
/// status, so CI dashboards and shell checks agree.
#[test]
fn failed_measurement_recorded_in_junit_report() {
    let dir = test_dir("junit");
    let deck = dir.join("fail.sp");
    std::fs::write(&deck, FAILING_MEAS_DECK).expect("write deck");
    let report = dir.join("report.xml");

    let output = run_rspice(&[
        "--quiet",
        "run",
        deck.to_str().unwrap(),
        "--report-format",
        "junit",
        "--report-file",
        report.to_str().unwrap(),
    ]);
    assert_eq!(output.status.code(), Some(3));

    let xml = std::fs::read_to_string(&report).expect("read junit report");
    assert!(
        xml.contains("failures=\"1\""),
        "JUnit report should count the failed measurement: {xml}"
    );
    assert!(
        xml.contains("Measurement failed"),
        "JUnit report should carry the failure element: {xml}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

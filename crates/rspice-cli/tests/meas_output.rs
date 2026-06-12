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

/// Transient measurements can address the time axis (`FIND TIME WHEN`) and
/// branch currents (`I(v1)`), and DC measurements evaluate against the sweep.
#[test]
fn time_current_and_dc_measurements_evaluate() {
    let dir = test_dir("signals");
    let deck = dir.join("rc_signals.sp");
    // tau = 100ns; V(out) crosses 2.5V (half of 5V) at ln(2)*tau ~= 69.3ns.
    std::fs::write(
        &deck,
        "* RC low-pass, time and current measurements\n\
         V1 in 0 PULSE(0 5 0 1n 1n 1u 2u)\n\
         R1 in out 1k\n\
         C1 out 0 100p\n\
         .TRAN 1n 2u\n\
         .MEAS TRAN tcross FIND TIME WHEN V(out)=2.5\n\
         .MEAS TRAN ipeak MIN I(v1)\n\
         .END\n",
    )
    .expect("write deck");

    let output = run_rspice(&["run", deck.to_str().unwrap(), "--meas"]);
    assert!(
        output.status.success(),
        "run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);

    let tcross_line = stdout
        .lines()
        .find(|line| line.trim_start().starts_with("TCROSS ="))
        .unwrap_or_else(|| panic!("no TCROSS line in output:\n{stdout}"));
    assert!(
        !tcross_line.contains("FAILED"),
        "TIME-based measurement must evaluate: {tcross_line}"
    );
    let tcross: f64 = tcross_line
        .split('=')
        .nth(1)
        .expect("value field")
        .trim()
        .parse()
        .expect("numeric tcross");
    assert!(
        (tcross - 6.93e-8).abs() < 2e-9,
        "half-rail crossing should be ~ln(2)*tau = 69.3ns, got {tcross}"
    );

    // Charging current peaks at -5V/1k = -5mA (out of the source).
    let ipeak_line = stdout
        .lines()
        .find(|line| line.trim_start().starts_with("IPEAK ="))
        .unwrap_or_else(|| panic!("no IPEAK line in output:\n{stdout}"));
    assert!(
        !ipeak_line.contains("FAILED"),
        "current-based measurement must evaluate: {ipeak_line}"
    );
    let ipeak: f64 = ipeak_line
        .split('=')
        .nth(1)
        .expect("value field")
        .trim()
        .parse()
        .expect("numeric ipeak");
    assert!(
        (ipeak + 5e-3).abs() < 5e-4,
        "peak charging current should be ~-5mA, got {ipeak}"
    );

    // DC sweep measurement: divider halves the swept source.
    let dc_deck = dir.join("divider.sp");
    std::fs::write(
        &dc_deck,
        "* divider\n\
         V1 in 0 5\n\
         R1 in out 1k\n\
         R2 out 0 1k\n\
         .DC V1 0 5 0.5\n\
         .MEAS DC vhalf FIND V(out) AT=2.5\n\
         .END\n",
    )
    .expect("write dc deck");
    let output = run_rspice(&["run", dc_deck.to_str().unwrap(), "--meas"]);
    assert!(
        output.status.success(),
        "dc run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    let vhalf_line = stdout
        .lines()
        .find(|line| line.trim_start().starts_with("VHALF ="))
        .unwrap_or_else(|| panic!("no VHALF line in output:\n{stdout}"));
    let vhalf: f64 = vhalf_line
        .split('=')
        .nth(1)
        .expect("value field")
        .trim()
        .parse()
        .expect("numeric vhalf");
    assert!(
        (vhalf - 1.25).abs() < 1e-6,
        "V(out) at V1=2.5 through the divider should be 1.25V, got {vhalf}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// AC measurements evaluate against the derived real series: magnitude,
/// decibels, and phase in degrees, with the frequency axis addressable.
#[test]
fn ac_measurements_evaluate() {
    let dir = test_dir("ac");
    let deck = dir.join("rc_ac.sp");
    // -3dB corner of the RC low-pass: 1/(2*pi*1k*100n) = 1591.55 Hz,
    // where the phase is exactly -45 degrees.
    std::fs::write(
        &deck,
        "* RC low-pass AC\n\
         V1 in 0 AC 1\n\
         R1 in out 1k\n\
         C1 out 0 100n\n\
         .AC DEC 50 10 1Meg\n\
         .MEAS AC f3db FIND FREQUENCY WHEN VDB(out)=-3.0103\n\
         .MEAS AC corner_phase FIND VP(out) WHEN VDB(out)=-3.0103\n\
         .END\n",
    )
    .expect("write deck");

    let output = run_rspice(&["run", deck.to_str().unwrap(), "--meas"]);
    assert!(
        output.status.success(),
        "run failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);

    let value_of = |name: &str| -> f64 {
        let line = stdout
            .lines()
            .find(|line| line.trim_start().starts_with(name))
            .unwrap_or_else(|| panic!("no {name} line in output:\n{stdout}"));
        assert!(!line.contains("FAILED"), "{name} must evaluate: {line}");
        line.split('=').nth(1).expect("value").trim().parse().expect("numeric")
    };

    let f3db = value_of("F3DB =");
    assert!(
        (f3db - 1591.55).abs() < 25.0,
        "-3dB corner should be ~1591.55 Hz, got {f3db}"
    );
    let phase = value_of("CORNER_PHASE =");
    assert!(
        (phase + 45.0).abs() < 1.0,
        "phase at the corner should be ~-45 degrees, got {phase}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// NOISE measurements address the spectral densities; GOAL/TOL turns a
/// measurement into a pass-fail check that gates the exit code.
#[test]
fn noise_measurements_and_goal_checks() {
    let dir = test_dir("noise_goal");
    let deck = dir.join("rc_noise.sp");
    // 1k resistor at 300.15K: sqrt(4kTR) = 4.071e-9 V/sqrt(Hz), flat well
    // below the 1591 Hz corner.
    std::fs::write(
        &deck,
        "* RC noise\n\
         V1 in 0 AC 1\n\
         R1 in out 1k\n\
         C1 out 0 100n\n\
         .NOISE V(out) V1 DEC 20 10 100k\n\
         .MEAS NOISE spot FIND ONOISE AT=10 GOAL=4.07e-9 TOL=1e-11\n\
         .END\n",
    )
    .expect("write deck");

    let output = run_rspice(&["--quiet", "run", deck.to_str().unwrap()]);
    assert_eq!(
        output.status.code(),
        Some(0),
        "in-tolerance GOAL must pass: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // The same measurement with an impossible goal gates the exit code.
    let bad = dir.join("rc_noise_bad.sp");
    std::fs::write(
        &bad,
        "* RC noise, wrong goal\n\
         V1 in 0 AC 1\n\
         R1 in out 1k\n\
         C1 out 0 100n\n\
         .NOISE V(out) V1 DEC 20 10 100k\n\
         .MEAS NOISE spot FIND ONOISE AT=10 GOAL=9e-9 TOL=1e-11\n\
         .END\n",
    )
    .expect("write deck");
    let output = run_rspice(&["--quiet", "run", bad.to_str().unwrap()]);
    assert_eq!(
        output.status.code(),
        Some(3),
        "missed GOAL must exit 3: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("SPOT"), "failure names the measurement: {stderr}");

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

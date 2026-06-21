//! Integration tests for `rspice compare`: cross-format golden comparison
//! and tolerance behavior.

use std::path::{Path, PathBuf};
use std::process::Command;

const TRAN_DECK: &str = "* transient compare test
v1 in 0 sin(0 1 1k)
r1 in out 1k
c1 out 0 1u
.tran 10u 100u
.end
";

fn test_dir(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "rspice_compare_formats_{}_{}",
        std::process::id(),
        tag
    ));
    std::fs::create_dir_all(&dir).expect("create test dir");
    dir
}

fn rspice_ok(args: &[&str]) {
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .arg("--quiet")
        .args(args)
        .output()
        .expect("run rspice");
    assert!(
        output.status.success(),
        "rspice {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
}

fn simulate(dir: &Path, format: &str, out_name: &str) -> PathBuf {
    let deck_path = dir.join("deck.sp");
    std::fs::write(&deck_path, TRAN_DECK).expect("write deck");
    let out = dir.join(out_name);
    rspice_ok(&[
        "run",
        deck_path.to_str().unwrap(),
        "-o",
        out.to_str().unwrap(),
        "-f",
        format,
    ]);
    out
}

#[test]
fn raw_result_compares_against_csv_golden() {
    let dir = test_dir("raw_vs_csv");
    let raw = simulate(&dir, "raw", "result.raw");
    let csv = simulate(&dir, "csv", "golden.csv");

    rspice_ok(&[
        "compare",
        raw.to_str().unwrap(),
        csv.to_str().unwrap(),
        "--abstol",
        "1e-9",
    ]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn hdf5_result_compares_against_raw_golden() {
    let dir = test_dir("h5_vs_raw");
    let h5 = simulate(&dir, "hdf5", "result.h5");
    let raw = simulate(&dir, "raw", "golden.raw");

    rspice_ok(&[
        "compare",
        h5.to_str().unwrap(),
        raw.to_str().unwrap(),
        "--abstol",
        "1e-9",
    ]);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn differences_fail_with_verification_exit_code() {
    let dir = test_dir("diff");
    std::fs::write(dir.join("a.csv"), "time,V(OUT)\n0,1.0\n1e-6,2.0\n").unwrap();
    std::fs::write(dir.join("b.csv"), "time,V(OUT)\n0,1.0\n1e-6,2.5\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            dir.join("a.csv").to_str().unwrap(),
            dir.join("b.csv").to_str().unwrap(),
        ])
        .output()
        .expect("run rspice");
    assert!(!output.status.success());
    assert_eq!(
        output.status.code(),
        Some(3),
        "comparison mismatches exit with the verification-failure code"
    );
    let _ = std::fs::remove_dir_all(&dir);
}

/// A result file missing trailing points must FAIL against a longer golden
/// file — a simulation that died mid-run cannot pass regression by matching
/// on the overlap it did write.
#[test]
fn truncated_result_fails_against_longer_golden() {
    let dir = test_dir("trunc");
    std::fs::write(
        dir.join("golden.csv"),
        "time,V(OUT)\n0,1.0\n1e-6,2.0\n2e-6,3.0\n3e-6,4.0\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("result.csv"),
        "time,V(OUT)\n0,1.0\n1e-6,2.0\n2e-6,3.0\n",
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            dir.join("result.csv").to_str().unwrap(),
            dir.join("golden.csv").to_str().unwrap(),
        ])
        .output()
        .expect("run rspice");
    assert_eq!(
        output.status.code(),
        Some(3),
        "truncated result must fail; stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("points"),
        "output should explain the point-count mismatch: {stdout}"
    );

    // Explicit opt-out compares the overlap only.
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            dir.join("result.csv").to_str().unwrap(),
            dir.join("golden.csv").to_str().unwrap(),
            "--allow-truncated",
        ])
        .output()
        .expect("run rspice");
    assert_eq!(output.status.code(), Some(0));

    let _ = std::fs::remove_dir_all(&dir);
}

/// Golden variables absent from the result are missing coverage, not a
/// silent skip.
#[test]
fn missing_golden_variable_fails() {
    let dir = test_dir("missing_var");
    std::fs::write(
        dir.join("golden.csv"),
        "time,V(OUT),V(MID)\n0,1.0,0.5\n1e-6,2.0,1.0\n",
    )
    .unwrap();
    std::fs::write(dir.join("result.csv"), "time,V(OUT)\n0,1.0\n1e-6,2.0\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            dir.join("result.csv").to_str().unwrap(),
            dir.join("golden.csv").to_str().unwrap(),
        ])
        .output()
        .expect("run rspice");
    assert_eq!(output.status.code(), Some(3));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("V(MID)"),
        "output should name the missing variable: {stdout}"
    );

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            dir.join("result.csv").to_str().unwrap(),
            dir.join("golden.csv").to_str().unwrap(),
            "--ignore-missing",
        ])
        .output()
        .expect("run rspice");
    assert_eq!(output.status.code(), Some(0));

    let _ = std::fs::remove_dir_all(&dir);
}

/// SPICE signal names are case-insensitive. `--ignore-missing` must not
/// turn a case-only spelling mismatch into a pass that compares only time.
#[test]
fn compare_matches_variables_case_insensitively_before_ignoring_missing() {
    let dir = test_dir("case_var");
    std::fs::write(dir.join("golden.csv"), "time,v(out)\n0,1.0\n1e-6,2.0\n").unwrap();
    std::fs::write(dir.join("result.csv"), "time,V(OUT)\n0,1.0\n1e-6,9.0\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            dir.join("result.csv").to_str().unwrap(),
            dir.join("golden.csv").to_str().unwrap(),
            "--ignore-missing",
        ])
        .output()
        .expect("run rspice");

    assert_eq!(
        output.status.code(),
        Some(3),
        "case-only variable differences must still compare the signal; stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("V(OUT)") || stdout.contains("v(out)"),
        "output should identify the compared signal: {stdout}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn compare_variable_filter_accepts_bare_voltage_node_name() {
    let dir = test_dir("bare_var");
    std::fs::write(
        dir.join("golden.csv"),
        "time,V(OUT),V(IN)\n0,1.0,5.0\n1e-6,2.0,5.0\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("result.csv"),
        "time,V(OUT),V(IN)\n0,1.0,5.0\n1e-6,9.0,5.0\n",
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            dir.join("result.csv").to_str().unwrap(),
            dir.join("golden.csv").to_str().unwrap(),
            "--variables",
            "out",
        ])
        .output()
        .expect("run rspice");

    assert_eq!(
        output.status.code(),
        Some(3),
        "bare node filter should select V(OUT) and catch the drift; stdout: {}; stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("V(OUT)"),
        "output should name the selected voltage signal: {stdout}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// --bless bootstraps a missing golden and accepts drift on demand.
#[test]
fn bless_updates_the_golden_file() {
    let dir = test_dir("bless");
    let result = dir.join("result.csv");
    let golden = dir.join("golden.csv");
    std::fs::write(&result, "time,V(OUT)\n0,1.0\n1e-6,2.0\n").unwrap();

    // Bootstrap: golden does not exist yet.
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            result.to_str().unwrap(),
            golden.to_str().unwrap(),
            "--bless",
        ])
        .output()
        .expect("run rspice");
    assert_eq!(output.status.code(), Some(0));
    assert!(golden.exists(), "--bless must create the golden file");

    // Drift: result changes, bless accepts it, plain compare then passes.
    std::fs::write(&result, "time,V(OUT)\n0,1.0\n1e-6,2.5\n").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            result.to_str().unwrap(),
            golden.to_str().unwrap(),
            "--bless",
        ])
        .output()
        .expect("run rspice");
    assert_eq!(output.status.code(), Some(0));

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            result.to_str().unwrap(),
            golden.to_str().unwrap(),
        ])
        .output()
        .expect("run rspice");
    assert_eq!(
        output.status.code(),
        Some(0),
        "blessed golden must now match"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// --interpolate resamples the result onto the golden grid, so a finer
/// (or shifted) time base compares point-for-point instead of failing on
/// the point count.
#[test]
fn interpolate_compares_across_different_grids() {
    let dir = test_dir("interp");
    // Result: ramp sampled at 0.5us; golden: the same ramp at 1us.
    std::fs::write(
        dir.join("result.csv"),
        "time,V(OUT)\n0,1.0\n5e-7,1.5\n1e-6,2.0\n1.5e-6,2.5\n2e-6,3.0\n",
    )
    .unwrap();
    std::fs::write(
        dir.join("golden.csv"),
        "time,V(OUT)\n0,1.0\n1e-6,2.0\n2e-6,3.0\n",
    )
    .unwrap();

    // Without interpolation the differing grids are a structural failure.
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            dir.join("result.csv").to_str().unwrap(),
            dir.join("golden.csv").to_str().unwrap(),
        ])
        .output()
        .expect("run rspice");
    assert_eq!(output.status.code(), Some(3));

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            dir.join("result.csv").to_str().unwrap(),
            dir.join("golden.csv").to_str().unwrap(),
            "--interpolate",
        ])
        .output()
        .expect("run rspice");
    assert_eq!(
        output.status.code(),
        Some(0),
        "stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );

    // A golden grid extending beyond the result must refuse to extrapolate.
    std::fs::write(
        dir.join("long_golden.csv"),
        "time,V(OUT)\n0,1.0\n1e-6,2.0\n5e-6,6.0\n",
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            dir.join("result.csv").to_str().unwrap(),
            dir.join("long_golden.csv").to_str().unwrap(),
            "--interpolate",
        ])
        .output()
        .expect("run rspice");
    assert_eq!(output.status.code(), Some(3));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("extrapolate"),
        "refusal should explain why: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn compare_variable_filter_matches_complex_signal_aliases() {
    let dir = test_dir("complex_filter_aliases");
    let result = dir.join("result.csv");
    let golden = dir.join("golden.csv");
    let csv = "frequency,Re(V(out)),Im(V(out)),Re(I(V1)),Im(I(V1))\n\
               1.0e3,1.0,0.25,-0.01,0.02\n\
               1.0e4,0.5,-0.75,-0.02,0.03\n";
    std::fs::write(&result, csv).unwrap();
    std::fs::write(&golden, csv).unwrap();

    for requested in ["V(out)", "v(OUT)", "out", "Re(V(out))"] {
        let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
            .args([
                "compare",
                result.to_str().unwrap(),
                golden.to_str().unwrap(),
                "--variables",
                requested,
            ])
            .output()
            .expect("run rspice");
        assert_eq!(
            output.status.code(),
            Some(0),
            "{requested} should select V(out) real/imag parts; stdout: {}; stderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn compare_explicit_missing_variable_respects_ignore_missing() {
    let dir = test_dir("explicit_ignore_missing");
    std::fs::write(dir.join("result.csv"), "time,V(OUT)\n0,1.0\n1e-6,2.0\n").unwrap();
    std::fs::write(dir.join("golden.csv"), "time,V(OUT)\n0,1.0\n1e-6,2.0\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            dir.join("result.csv").to_str().unwrap(),
            dir.join("golden.csv").to_str().unwrap(),
            "--variables",
            "V(MISSING)",
            "--ignore-missing",
        ])
        .output()
        .expect("run rspice");
    assert_eq!(
        output.status.code(),
        Some(0),
        "explicit missing variables should be skippable with --ignore-missing; stdout: {}; stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let _ = std::fs::remove_dir_all(&dir);
}

/// NaN is malformed waveform input, not a value that can compare equal.
#[test]
fn nonfinite_values_are_rejected_before_comparison() {
    let dir = test_dir("nan");
    std::fs::write(dir.join("golden.csv"), "time,V(OUT)\n0,1.0\n1e-6,2.0\n").unwrap();
    std::fs::write(dir.join("result.csv"), "time,V(OUT)\n0,1.0\n1e-6,NaN\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            dir.join("result.csv").to_str().unwrap(),
            dir.join("golden.csv").to_str().unwrap(),
        ])
        .output()
        .expect("run rspice");
    assert!(
        !output.status.success(),
        "NaN in the result must fail comparison"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("non-finite value"),
        "stderr should explain the malformed value: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn negative_compare_tolerances_are_usage_errors() {
    let dir = test_dir("negative_tolerances");
    let result = dir.join("result.csv");
    let golden = dir.join("golden.csv");
    let csv = "time,V(OUT)\n0,1.0\n1e-6,2.0\n";
    std::fs::write(&result, csv).unwrap();
    std::fs::write(&golden, csv).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            result.to_str().unwrap(),
            golden.to_str().unwrap(),
            "--abstol=-1",
            "--reltol=-1",
        ])
        .output()
        .expect("run rspice");
    assert_eq!(
        output.status.code(),
        Some(2),
        "invalid compare tolerances are usage errors"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("--abstol") || stderr.contains("--reltol"),
        "stderr should identify the invalid tolerance: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn ragged_delimited_rows_are_rejected_before_comparison() {
    let dir = test_dir("ragged_csv");
    std::fs::write(dir.join("golden.csv"), "time,V(OUT)\n0,1.0\n1e-6,2.0\n").unwrap();
    std::fs::write(
        dir.join("result.csv"),
        "time,V(OUT)\n0,1.0\n1e-6\n2e-6,3.0,extra\n",
    )
    .unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            dir.join("result.csv").to_str().unwrap(),
            dir.join("golden.csv").to_str().unwrap(),
        ])
        .output()
        .expect("run rspice");
    assert!(
        !output.status.success(),
        "ragged delimited input must not silently compare"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("row 3") && stderr.contains("expected 2 columns"),
        "stderr should name the ragged row and expected width: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn bless_missing_golden_validates_result_before_copying() {
    let dir = test_dir("bless_missing_ragged_csv");
    let result = dir.join("result.csv");
    let golden = dir.join("golden.csv");
    std::fs::write(&result, "time,V(OUT)\n0,1.0\n1e-6\n2e-6,3.0,extra\n").unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "compare",
            result.to_str().unwrap(),
            golden.to_str().unwrap(),
            "--bless",
        ])
        .output()
        .expect("run rspice");
    assert!(
        !output.status.success(),
        "malformed result must not be blessed into a missing golden"
    );
    assert!(
        !golden.exists(),
        "missing golden must not be created from malformed result"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("row 3") && stderr.contains("expected 2 columns"),
        "stderr should name the malformed result: {stderr}"
    );

    let _ = std::fs::remove_dir_all(&dir);
}

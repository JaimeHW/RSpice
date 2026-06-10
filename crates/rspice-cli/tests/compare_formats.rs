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
fn differences_fail_with_exit_code_1() {
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
    assert_eq!(output.status.code(), Some(1), "comparison failures exit 1");
    let _ = std::fs::remove_dir_all(&dir);
}

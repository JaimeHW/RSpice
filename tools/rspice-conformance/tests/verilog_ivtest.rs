//! The ivtest selection against `tests/verilog/ivtest-manifest.tsv`.
//!
//! Nightly only (`.github/workflows/nightly.yml`, job `ivtest`), where the
//! workflow builds Icarus Verilog v13_0 from its source tag and points
//! `RSPICE_IVTEST_DIR` at that checkout's `ivtest` directory. Without the
//! variable the test reports the oracle absent and returns, unless
//! `RSPICE_VERILOG_ORACLES_REQUIRED=1` makes absence a failure.
//!
//! Every selected test runs through `rspice-ivtest`, concurrently, and any
//! outcome that regressed, or improved without being re-recorded, fails the
//! test. `RSPICE_IVTEST_JOBS=N` overrides the default job count.

use std::process::Command;
use std::time::Instant;

use rspice_conformance::suites::verilog::ivtest::ivtest_dir;
use rspice_conformance::suites::verilog::oracle::{ORACLES_REQUIRED_ENV, oracles_required};

#[test]
fn every_ivtest_outcome_matches_the_manifest() {
    if let Err(absent) = ivtest_dir() {
        assert!(
            !oracles_required(),
            "{absent}; {ORACLES_REQUIRED_ENV} makes that a failure"
        );
        println!("{absent}");
        return;
    }
    let mut command = Command::new(env!("CARGO_BIN_EXE_rspice-ivtest"));
    command.arg("run");
    if let Ok(jobs) = std::env::var("RSPICE_IVTEST_JOBS") {
        command.arg("--jobs").arg(jobs);
    }
    let started = Instant::now();
    let output = command.output().expect("spawn rspice-ivtest");
    // The per-test lines, then the summary and any drift.
    println!("{}", String::from_utf8_lossy(&output.stderr));
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!("ivtest: {:.1} s wall", started.elapsed().as_secs_f64());
    assert!(
        output.status.success(),
        "ivtest drifted from the manifest ({}); the outcomes and the drift are printed above",
        output.status
    );
}

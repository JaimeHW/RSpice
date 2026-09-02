//! Cancellation regressions for basic-analysis dispatch.
//!
//! The public CLI normalizes `.TEMP` into the canonical run-axis planner, so
//! the process tests below cover that reachable surface. The source guard is
//! what protects the retained legacy `run_temp` adapter until it is removed.

use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

struct TestDirectory(PathBuf);

impl TestDirectory {
    fn new(tag: &str) -> Self {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let serial = NEXT.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "rspice_basic_cancel_{}_{}_{}",
            std::process::id(),
            tag,
            serial
        ));
        std::fs::create_dir(&path).expect("create basic-cancellation test directory");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[test]
fn legacy_temperature_runner_has_no_non_abort_dc_wrapper() {
    let source = include_str!("../src/commands/run/basic.rs");
    let run_temp = source
        .split_once("pub(super) fn run_temp")
        .expect("legacy temperature runner remains present")
        .1
        .split_once("fn resolve_transient_max_step")
        .expect("temperature runner has a stable end marker")
        .0;

    assert!(
        run_temp.contains(".run_dc_op_with_abort("),
        "legacy temperature execution must pass ProcessAbort into the DC solve"
    );
    assert!(
        run_temp.contains("&crate::abort::ProcessAbort"),
        "legacy temperature execution must use the process-wide abort source"
    );
    assert!(
        run_temp.contains("Err(rspice_core::SimulationError::Aborted)"),
        "legacy temperature execution must distinguish cancellation from non-convergence"
    );
    assert!(
        !run_temp.contains(".run_dc_op("),
        "legacy temperature execution regressed to the non-cancellable DC wrapper"
    );
}

#[test]
fn canonical_temperature_axis_still_completes_and_publishes_each_coordinate() {
    let directory = TestDirectory::new("success");
    let deck = directory.path().join("temperature_success.sp");
    let requested = directory.path().join("temperature.json");
    std::fs::write(
        &deck,
        "* reachable canonical temperature-axis success\n\
         V1 in 0 10\n\
         R1 in out 1k\n\
         R2 out 0 1k\n\
         .temp 25 75\n\
         .op\n\
         .end\n",
    )
    .expect("write successful temperature fixture");

    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().expect("UTF-8 deck path"),
            "--output",
            requested.to_str().expect("UTF-8 output path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run successful temperature fixture");
    assert!(
        output.status.success(),
        "temperature-axis run failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !requested.exists(),
        "the canonical multi-run axis must not overwrite the unqualified base path"
    );

    for ordinal in 1..=2 {
        let artifact = directory
            .path()
            .join(format!("temperature.step_{ordinal:06}.json"));
        let document: Value = serde_json::from_slice(
            &std::fs::read(&artifact)
                .unwrap_or_else(|error| panic!("read {}: {error}", artifact.display())),
        )
        .unwrap_or_else(|error| panic!("parse {}: {error}", artifact.display()));
        assert_eq!(document["analysis"], "dc_op", "{}", artifact.display());
    }
}

#[test]
fn reachable_temperature_axis_timeout_is_typed_and_publishes_no_artifact() {
    let directory = TestDirectory::new("timeout");
    let deck = directory.path().join("temperature_timeout.sp");
    let requested = directory.path().join("cancelled.json");

    // A large but policy-compliant analog chain makes the real CLI path stay
    // active long enough for the cooperative timer to fire before any
    // coordinate can publish. This exercises the canonical `.TEMP` surface,
    // not the legacy adapter protected by the source guard above.
    let mut source = String::from("* reachable cancellable temperature axis\nV1 n0 0 1\n");
    for index in 0..20_000 {
        source.push_str(&format!("R{index} n{index} n{} 1k\n", index + 1));
    }
    source.push_str("RTAIL n20000 0 1k\n.temp 25 75\n.op\n.end\n");
    std::fs::write(&deck, source).expect("write cancellable temperature fixture");

    let started = std::time::Instant::now();
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args([
            "--quiet",
            "run",
            deck.to_str().expect("UTF-8 deck path"),
            "--timeout",
            "0.001",
            "--output",
            requested.to_str().expect("UTF-8 output path"),
            "--format",
            "json",
        ])
        .output()
        .expect("run cancellable temperature fixture");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(
        output.status.code(),
        Some(124),
        "temperature timeout must use the stable timeout exit code\nstdout:\n{stdout}\nstderr:\n{stderr}"
    );
    assert!(
        stderr.contains("timed out") || stderr.contains("Timeout"),
        "timeout diagnostic must remain explicit:\n{stderr}"
    );
    assert!(
        started.elapsed() < std::time::Duration::from_secs(20),
        "temperature cancellation must stop promptly"
    );

    let leaked = std::fs::read_dir(directory.path())
        .expect("list timeout directory")
        .map(|entry| entry.expect("directory entry").path())
        .filter(|path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name.starts_with("cancelled"))
        })
        .collect::<Vec<_>>();
    assert!(
        leaked.is_empty(),
        "a cancelled temperature run published artifacts: {leaked:?}"
    );
}

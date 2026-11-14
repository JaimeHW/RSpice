//! The mixed-signal reference corpus, every design, against `outcomes.tsv`.
//!
//! Nightly only (`.github/workflows/nightly.yml`, job `mixed-corpus`), once per
//! route: the route is the build, so the job sets `RSPICE_MIXED_ROUTE` to the
//! route it compiled and this test refuses a mismatch rather than recording
//! one route's outcomes under the other's name. `RSPICE_MIXED_EXTENDED=1`
//! includes the extended analyses. Any cell that regressed, or improved
//! without being re-recorded, fails the test.
//!
//! The runner runs the cells concurrently at its default `--jobs`;
//! `RSPICE_MIXED_JOBS=N` overrides it.
//!
//! `RSPICE_MIXED_ROUTE=worker` runs the cells in the browser worker under
//! Node instead, from the directory `RSPICE_MIXED_WORKER` names (default
//! `crates/rspice-app/web`, where the browser build generates the worker
//! image's `pkg/` beside `simulation-worker.js`); either build serves it.

use std::process::Command;
use std::time::Instant;

use rspice_conformance::suites::mixed::Route;

#[test]
fn every_mixed_corpus_cell_matches_its_recorded_outcome() {
    let compiled = Route::compiled();
    let expected = std::env::var("RSPICE_MIXED_ROUTE").ok();
    let worker = expected.as_deref() == Some(Route::Worker.token());
    if let Some(expected) = expected.as_deref().filter(|_| !worker) {
        assert_eq!(
            expected,
            compiled.token(),
            "RSPICE_MIXED_ROUTE={expected} but this test binary was built as the {compiled} route"
        );
    }
    let mut command = Command::new(env!("CARGO_BIN_EXE_rspice-mixed-corpus"));
    command.arg("run").arg("--expect-route");
    if worker {
        let web = std::env::var_os("RSPICE_MIXED_WORKER").map_or_else(
            || std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/rspice-app/web"),
            std::path::PathBuf::from,
        );
        command.arg(Route::Worker.token()).arg("--worker").arg(web);
    } else {
        command.arg(compiled.token());
    }
    if std::env::var("RSPICE_MIXED_EXTENDED").is_ok_and(|value| value == "1") {
        command.arg("--extended");
    }
    if let Ok(jobs) = std::env::var("RSPICE_MIXED_JOBS") {
        command.arg("--jobs").arg(jobs);
    }
    let started = Instant::now();
    let output = command.output().expect("spawn rspice-mixed-corpus");
    // The per-cell lines (with each cell's time), then the summary.
    println!("{}", String::from_utf8_lossy(&output.stderr));
    println!("{}", String::from_utf8_lossy(&output.stdout));
    println!(
        "mixed corpus: {:.1} s wall",
        started.elapsed().as_secs_f64()
    );
    assert!(
        output.status.success(),
        "mixed corpus drifted from outcomes.tsv ({}); the cells and the drift are printed above",
        output.status
    );
}

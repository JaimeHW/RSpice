//! The mixed-signal reference corpus, every design, against `outcomes.tsv`.
//!
//! One route in one command. The route is the build, so `RSPICE_MIXED_ROUTE`
//! names the route this binary was compiled as and the test refuses a
//! mismatch rather than recording one route's outcomes under the other's
//! name. `RSPICE_MIXED_EXTENDED=1` includes the extended analyses. Any cell
//! that regressed, or improved without being re-recorded, fails the test.
//! The nightly (`.github/workflows/nightly.yml`, jobs `mixed-corpus-build`
//! and `mixed-corpus`) runs the same `rspice-mixed-corpus run`, each route
//! built once and its designs split over shards (`--shard K/N`).
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
        "{} ({}); the cells are printed above",
        if rspice_conformance::suites::perturb::active().is_some()
            || rspice_conformance::suites::perturb::active_time().is_some()
        {
            "perturbed mixed corpus has blind cells, listed with their allowances"
        } else {
            "mixed corpus drifted from outcomes.tsv, with the drift"
        },
        output.status
    );
}

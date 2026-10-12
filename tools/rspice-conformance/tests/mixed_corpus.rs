//! The mixed-signal reference corpus, every design, against `outcomes.tsv`.
//!
//! Nightly only (`.github/workflows/nightly.yml`, job `mixed-corpus`), once per
//! route: the route is the build, so the job sets `RSPICE_MIXED_ROUTE` to the
//! route it compiled and this test refuses a mismatch rather than recording
//! one route's outcomes under the other's name. `RSPICE_MIXED_EXTENDED=1`
//! includes the extended analyses. Any cell that regressed, or improved
//! without being re-recorded, fails the test.

use std::process::Command;

use rspice_conformance::suites::mixed::Route;

#[test]
fn every_mixed_corpus_cell_matches_its_recorded_outcome() {
    let compiled = Route::compiled();
    if let Ok(expected) = std::env::var("RSPICE_MIXED_ROUTE") {
        assert_eq!(
            expected,
            compiled.token(),
            "RSPICE_MIXED_ROUTE={expected} but this test binary was built as the {compiled} route"
        );
    }
    let mut command = Command::new(env!("CARGO_BIN_EXE_rspice-mixed-corpus"));
    command
        .arg("run")
        .arg("--expect-route")
        .arg(compiled.token());
    if std::env::var("RSPICE_MIXED_EXTENDED").is_ok_and(|value| value == "1") {
        command.arg("--extended");
    }
    let output = command.output().expect("spawn rspice-mixed-corpus");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("{stdout}");
    assert!(
        output.status.success(),
        "mixed corpus drifted from outcomes.tsv ({}):\n{stdout}\n{stderr}",
        output.status
    );
}

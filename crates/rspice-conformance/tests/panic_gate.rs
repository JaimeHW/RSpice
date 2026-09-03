//! The corpus-wide panic gate.
//!
//! One test, one job: sweep every vendored deck through ingestion, planning,
//! materialization and output resolution, and fail the run listing every deck
//! that panicked instead of refusing.
//!
//! It prints its counts unconditionally so a CI log shows how much corpus the
//! gate actually swept. A gate that silently discovers zero decks — a moved
//! corpus, a checkout without the vendored trees — would otherwise pass and
//! mean nothing, so the deck count is asserted too.

use std::path::PathBuf;

use rspice_conformance::suites::panic_gate::{PANIC_GATE_CORPORA, run_panic_gate};

/// The vendored corpora live at the workspace root, not inside this crate.
fn tests_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("rspice-conformance is a workspace crate under crates/")
        .join("tests")
}

/// Below this, the corpora are not present and the gate is not a gate.
///
/// The vendored trees hold thousands of decks; the floor is deliberately far
/// under that so re-vendoring does not have to touch this number, while a
/// checkout that lost the corpora still fails loudly.
const MINIMUM_DECKS: usize = 1_000;

#[test]
fn no_vendored_deck_panics_during_ingestion_or_planning() {
    let tests_dir = tests_dir();
    assert!(
        tests_dir.is_dir(),
        "the vendored corpora are missing from {}",
        tests_dir.display()
    );

    let started = std::time::Instant::now();
    let report = run_panic_gate(&tests_dir);
    // Printed rather than logged: these lines are the gate's CI-visible
    // output, and a gate that swept nothing must be visibly different from one
    // that swept everything.
    println!("{}", report.summary());
    println!(
        "panic-gate: elapsed_ms={} corpora={}",
        started.elapsed().as_millis(),
        PANIC_GATE_CORPORA
            .iter()
            .map(|corpus| corpus.label)
            .collect::<Vec<_>>()
            .join(", ")
    );

    assert!(
        report.decks >= MINIMUM_DECKS,
        "the panic gate found only {} decks under {}; the vendored corpora look absent",
        report.decks,
        tests_dir.display()
    );
    assert_eq!(
        report.loaded + report.refused + report.panics.len(),
        report.decks,
        "every swept deck must be accounted for"
    );
    assert!(
        report.panics.is_empty(),
        "{} deck(s) panicked instead of refusing:\n{}",
        report.panics.len(),
        report
            .panics
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n")
    );
}

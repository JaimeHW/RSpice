//! No authored deck panics: parse, plan, materialize, and one cheap solve.
//!
//! Rule 2 of the engineering contract says input-derived indices, names,
//! schemas and shapes use checked access, so an authored deck produces a typed
//! refusal and never a panic. The conformance crate holds the full-corpus
//! version of this gate; this one lives in `rspice-core` so the same claim is
//! checked by the crate's own test job, on a deterministic subset small enough
//! to run per commit.
//!
//! Two corpora feed it:
//!
//! * a stable one-in-`SELECTION_MODULUS` slice of `tests/xyce/Netlists`,
//!   chosen by hashing each deck's corpus-relative name so the selection does
//!   not shift when a deck is added or removed elsewhere, and
//! * every fixture under `crates/rspice-cli/tests/fixtures/`, which are the
//!   decks written specifically to reach output-projection and analysis
//!   corners.
//!
//! Each deck goes through include expansion, parsing, deck planning,
//! coordinate materialization, one materialized run, authored-output
//! resolution, and a bounded operating-point solve — bounded because this gate
//! measures panics, not convergence, so a deck that would take a minute is
//! cancelled rather than skipped. Every stage runs inside `catch_unwind`, and
//! the test fails listing every deck that escaped.

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use rspice_core::ResourceLimits;
use rspice_core::abort_signal::{CountingAbort, NoAbort};
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::execution::{DeckPlan, SignalProjection};
use rspice_core::netlist::{Netlist, validate_output_symbols};

/// One deck in every `SELECTION_MODULUS` is swept from the Xyce corpus.
///
/// Chosen so the slice stays in the low hundreds of decks: enough that a
/// regression in a shared ingestion path is caught here rather than only in
/// the nightly full-corpus gate, few enough to stay inside a per-commit job.
const SELECTION_MODULUS: u64 = 48;

/// Stack the sweep runs on.
///
/// The test harness gives each test thread 2 MiB, which is smaller than the
/// stack a shipped binary runs on. Ingestion is recursive over hierarchy and
/// expression depth and is bounded by [`ResourceLimits`], not by the stack, so
/// a 2 MiB harness thread would report a limit the product does not have — and
/// a stack overflow is an abort no `catch_unwind` can turn into a finding.
const SWEEP_STACK_BYTES: usize = 32 * 1024 * 1024;

/// Polls a bounded solve is allowed before it is cancelled.
///
/// The solve exists to reach the device-evaluation and matrix-assembly code
/// with a real deck, not to converge. A deck that needs more than this is
/// cancelled, which is a typed outcome and not a skip.
const SOLVE_POLL_BUDGET: usize = 20_000;

/// Below this the corpus is not present and the gate is not a gate.
const MINIMUM_DECKS: usize = 50;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("rspice-core is a workspace crate under crates/")
        .to_path_buf()
}

/// FNV-1a over the deck's corpus-relative name.
///
/// A content-independent, position-independent selector: the same deck is
/// always either in or out of the slice, whatever else the corpus gains or
/// loses, so a failure here names a deck that will be selected again.
fn selection_hash(key: &str) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in key.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

fn collect(root: &Path, dir: &Path, extensions: &[&str], out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(root, &path, extensions, out);
            continue;
        }
        let is_deck = path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| {
                extensions
                    .iter()
                    .any(|wanted| extension.eq_ignore_ascii_case(wanted))
            });
        if is_deck && let Ok(relative) = path.strip_prefix(root) {
            out.push(relative.to_string_lossy().replace('\\', "/"));
        }
    }
}

fn selected_decks() -> Vec<(String, PathBuf)> {
    let root = workspace_root();
    let mut decks = Vec::new();

    let xyce = root.join("tests").join("xyce").join("Netlists");
    let mut keys = Vec::new();
    collect(&xyce, &xyce, &["cir"], &mut keys);
    keys.sort();
    for key in keys {
        if selection_hash(&key).is_multiple_of(SELECTION_MODULUS) {
            decks.push((format!("xyce/Netlists/{key}"), xyce.join(&key)));
        }
    }

    let fixtures = root
        .join("crates")
        .join("rspice-cli")
        .join("tests")
        .join("fixtures");
    let mut keys = Vec::new();
    collect(&fixtures, &fixtures, &["cir", "sp", "net"], &mut keys);
    keys.sort();
    for key in keys {
        decks.push((format!("cli/fixtures/{key}"), fixtures.join(&key)));
    }

    decks
}

//=============================================================================
// Panic capture
//=============================================================================

static LAST_PANIC: Mutex<Option<String>> = Mutex::new(None);

/// Replace the default hook so a caught panic does not print a backtrace for
/// every deck; the message is kept and reported once, with the deck name.
fn install_panic_recorder() {
    std::panic::set_hook(Box::new(|info| {
        let location = info
            .location()
            .map_or_else(|| "unknown location".to_owned(), ToString::to_string);
        let message = info.payload_as_str().unwrap_or("<non-string payload>");
        if let Ok(mut last) = LAST_PANIC.lock() {
            *last = Some(format!("{message} (at {location})"));
        }
    }));
}

fn take_panic_message() -> String {
    LAST_PANIC
        .lock()
        .ok()
        .and_then(|mut last| last.take())
        .unwrap_or_else(|| "<panic message unavailable>".to_owned())
}

//=============================================================================
// The sweep
//=============================================================================

#[derive(Debug, Default)]
struct Report {
    decks: usize,
    completed: usize,
    refused: usize,
    panics: Vec<String>,
}

/// Run `body` under `catch_unwind`, recording a panic against `deck`/`stage`.
fn stage<T>(
    deck: &str,
    stage: &'static str,
    report: &mut Report,
    body: impl FnOnce() -> Result<T, ()>,
) -> Option<Result<T, ()>> {
    match catch_unwind(AssertUnwindSafe(body)) {
        Ok(value) => Some(value),
        Err(_) => {
            report.panics.push(format!(
                "{deck} panicked during {stage}: {}",
                take_panic_message()
            ));
            None
        }
    }
}

macro_rules! step {
    ($deck:expr, $report:expr, $stage:literal, $body:expr) => {
        match stage($deck, $stage, $report, || $body) {
            Some(Ok(value)) => value,
            Some(Err(())) => {
                $report.refused += 1;
                return;
            }
            None => return,
        }
    };
}

fn sweep_deck(deck: &str, path: &Path, report: &mut Report) {
    let source = step!(
        deck,
        report,
        "read",
        std::fs::read_to_string(path).map_err(drop)
    );
    let expanded = step!(
        deck,
        report,
        "expand-includes",
        Netlist::preprocess_includes(&source, path).map_err(drop)
    );
    let netlist = step!(
        deck,
        report,
        "parse",
        Netlist::parse_with_path(&expanded, path).map_err(drop)
    );
    let limits = ResourceLimits::default();
    let plan = step!(
        deck,
        report,
        "plan",
        DeckPlan::from_netlist(&netlist, &limits).map_err(drop)
    );
    let coordinates = step!(
        deck,
        report,
        "coordinates",
        plan.coordinates_with_abort(&limits, &NoAbort).map_err(drop)
    );
    if coordinates.is_empty() {
        report.refused += 1;
        return;
    }
    let engine = Engine::new(SimulationConfig::default());
    let materialize_abort = CountingAbort::new(SOLVE_POLL_BUDGET);
    step!(deck, report, "materialize", {
        engine
            .prepare_deck_plan_materializer_with_abort(&netlist, &plan, &materialize_abort)
            .and_then(|materializer| materializer.materialize_run_with_abort(0, &materialize_abort))
            .map(drop)
            .map_err(drop)
    });
    step!(deck, report, "projection", {
        let _ = validate_output_symbols(&netlist);
        SignalProjection::from_netlist(&netlist).map_err(drop)
    });
    // The cheap analysis. A refusal, a non-convergence, or a cancellation are
    // all acceptable outcomes; only a panic is not.
    let solve_abort = CountingAbort::new(SOLVE_POLL_BUDGET);
    step!(deck, report, "operating-point", {
        let _ = engine.run_dc_op_with_abort(&netlist, &solve_abort);
        Ok::<(), ()>(())
    });
    report.completed += 1;
}

fn sweep() -> Report {
    let decks = selected_decks();
    install_panic_recorder();
    let mut report = Report {
        decks: decks.len(),
        ..Report::default()
    };
    for (deck, path) in &decks {
        sweep_deck(deck, path, &mut report);
    }
    report
}

#[test]
fn no_bundled_deck_panics_through_parse_plan_materialize_and_a_cheap_solve() {
    let report = std::thread::Builder::new()
        .stack_size(SWEEP_STACK_BYTES)
        .spawn(sweep)
        .expect("spawn the sweep thread")
        .join()
        .expect("the sweep thread itself must not die");

    // Printed so a CI log shows the corpus this gate actually swept.
    println!(
        "no-panic-corpus: decks={} completed={} refused={} panicked={}",
        report.decks,
        report.completed,
        report.refused,
        report.panics.len()
    );

    assert!(
        report.decks >= MINIMUM_DECKS,
        "the no-panic gate selected only {} decks; the bundled corpora look absent",
        report.decks
    );
    assert_eq!(
        report.completed + report.refused + report.panics.len(),
        report.decks,
        "every swept deck must be accounted for"
    );
    assert!(
        report.panics.is_empty(),
        "{} deck(s) panicked instead of refusing:\n{}",
        report.panics.len(),
        report.panics.join("\n")
    );
}

//! What the Console keeps about a plan command, and what the notice centre
//! does not.
//!
//! A plan command's outcome has two readers with different jobs. The lifecycle
//! status states the latest outcome, and the Console is the session's log of
//! what was commanded — where an operator reconstructs an hour of plan edits
//! afterwards, the way a Spectre session log records every command with its
//! result. Before the funnel on [`crate::workbench::AppState`] only the first
//! reader existed: a refusal survived as a toast and a committed receipt was
//! dropped, so a successful plan edit left no durable record anywhere.
//!
//! These tests pin the three rules that make the second reader worth having:
//! every announcing site logs through the one funnel, an outcome is logged
//! once per event rather than once per frame, and a `PLAN` entry is a record
//! rather than a notice.
//!
//! The mirror half of that last rule is asserted in `workbench::frame`'s own
//! tests, beside the mirror it is about. The activity mirror is private to the
//! frame loop, and widening it so this file could call it would be a worse
//! trade than stating the claim where the code is.

use super::*;

use std::path::{Path, PathBuf};

use crate::diagnostics::{LogSeverity, LogSource};

/// Every `PLAN` entry the Console holds, oldest first.
fn plan_entries(app: &RSpiceApp) -> Vec<(LogSeverity, String)> {
    app.state
        .log_buffer
        .entries()
        .filter(|entry| entry.source == LogSource::Plan)
        .map(|entry| (entry.severity, entry.message.clone()))
        .collect()
}

/// The first analysis instance of the fixture's plan.
fn first_instance(app: &RSpiceApp) -> AnalysisInstanceId {
    app.state
        .sim_setup
        .stable_analysis_plan()
        .expect("the fixture has a stable plan")
        .instances()
        .first()
        .expect("the fixture plan owns an instance")
        .id()
}

/// A committed command is recorded at `Info`, in the receipt's own words.
///
/// The Console page shows every severity, so this is where a receipt is
/// readable; the Problems page shows warnings and errors, so a receipt does
/// not appear there as a problem.
#[test]
fn a_committed_plan_command_leaves_one_plan_receipt_in_the_console() {
    let mut app = RSpiceApp::test_instance();
    let id = first_instance(&app);

    apply_analysis_action(&mut app, id, AnalysisAction::SetEnabled(false));

    let entries = plan_entries(&app);
    assert_eq!(entries.len(), 1, "one command is one record: {entries:?}");
    assert_eq!(entries[0].0, LogSeverity::Info);
    assert_eq!(
        entries[0].1,
        app.state.workbench.analysis_lifecycle_status.message(),
        "the Console records the receipt itself, not a paraphrase of it"
    );
    assert!(
        entries[0].1.starts_with("Receipt #"),
        "the record is the receipt, sequence and all: {}",
        entries[0].1
    );
    assert!(
        app.state.ui.toasts.activity().is_empty(),
        "a committed edit is not an event the reader has to be chased with"
    );
}

/// A refused command is recorded at `Warning`, and noticed exactly once.
///
/// The one notice is the drain's titled toast, which is what carries a refusal
/// to a reader who is not on the page that raised it. The Console record is
/// the other half of the pair and must not become a second notice for the same
/// refusal; that the mirror never lifts it is asserted in `workbench::frame`.
#[test]
fn a_refused_plan_command_is_recorded_once_and_noticed_once() {
    let ctx = egui::Context::default();
    let mut app = RSpiceApp::test_instance();

    apply_analysis_action(
        &mut app,
        AnalysisInstanceId::new(),
        AnalysisAction::SetEnabled(true),
    );
    drain_lifecycle_refusal(&ctx, &mut app.state);

    let entries = plan_entries(&app);
    assert_eq!(entries.len(), 1, "one refusal is one record: {entries:?}");
    assert_eq!(
        entries[0].0,
        LogSeverity::Warning,
        "a refusal is a problem, and the Problems page lists warnings and errors"
    );
    assert_eq!(
        entries[0].1,
        app.state.workbench.analysis_lifecycle_status.message()
    );
    assert_eq!(
        app.state.ui.toasts.activity().len(),
        1,
        "the drain's toast is the one notice this refusal produces"
    );
}

/// An outcome that is merely still true is not a second event.
///
/// Three announcing sites sit on the render path and restate a standing
/// refusal every frame. The funnel logs only when the outcome changes, and the
/// price of not writing that refusal sixty times a second is that two
/// consecutive identical receipts are one record.
#[test]
fn a_restated_outcome_is_recorded_once_and_a_changed_one_again() {
    let mut app = RSpiceApp::test_instance();
    for _ in 0..3 {
        app.state
            .record_plan_refusal("Remove rejected fail-closed: another analysis is bound to it.");
    }
    assert_eq!(plan_entries(&app).len(), 1);

    app.state
        .record_plan_receipt("Receipt #2 \u{00b7} Edit committed for instance 1.");
    let entries = plan_entries(&app);
    assert_eq!(entries.len(), 2, "{entries:?}");
    assert_eq!(entries[1].0, LogSeverity::Info);
}

/// Every `.rs` file this crate ships, read at test time.
///
/// Walked rather than listed, so a file added to the crate is scanned the day
/// it lands rather than the day someone remembers to add it here. Test-only
/// files, and the `#[cfg(test)]` items inside the remaining ones, are removed
/// through `crate::source_guard`: a test is free to announce an outcome
/// however it needs to, and the shipped lines keep their line numbers so a
/// failure names the line a match sits on.
fn shipped_sources() -> Vec<(PathBuf, String)> {
    fn walk(directory: &Path, out: &mut Vec<PathBuf>) {
        let entries = std::fs::read_dir(directory)
            .unwrap_or_else(|error| panic!("read {}: {error}", directory.display()));
        for entry in entries {
            let path = entry.expect("directory entry").path();
            if path.is_dir() {
                walk(&path, out);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                out.push(path);
            }
        }
    }

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut paths = Vec::new();
    walk(&root, &mut paths);
    paths.sort();
    assert!(
        paths.len() > 100,
        "the crate-wide scan found only {} files under {}; a scan that reaches nothing \
         passes forever",
        paths.len(),
        root.display()
    );
    let sources: Vec<(PathBuf, String)> = paths
        .into_iter()
        .map(|path| {
            let source = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
            (path, source)
        })
        .collect();
    let roots: Vec<PathBuf> = sources
        .iter()
        .flat_map(|(path, source)| crate::source_guard::test_only_roots(path, source))
        .collect();
    sources
        .iter()
        .filter(|(path, _)| crate::source_guard::ships(path, &roots))
        .map(|(path, source)| {
            (
                path.clone(),
                crate::source_guard::without_test_items(source),
            )
        })
        .collect()
}

/// One funnel writes the lifecycle outcome, so the rules above are rules
/// rather than the behaviour of whichever site happened to raise the outcome.
///
/// Twenty-odd sites used to write the status field directly, which is why a
/// receipt reached no log at all: there was no single place to log from. The
/// funnel on `AppState` is now the only writer, and this is what keeps it
/// that way — a new site that reaches for the field fails here, naming its own
/// line.
///
/// Prose is scanned like code on purpose. A comment that spells the direct
/// call is a comment describing a shape the crate no longer has, and the
/// funnel's own doc is where that shape is named.
#[test]
fn only_the_funnel_writes_the_lifecycle_outcome() {
    const FIELD: &str = "analysis_lifecycle_status";

    let funnel = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("workbench")
        .join("app_state.rs");
    let mut found = Vec::new();
    let mut scanned = 0usize;
    for (path, source) in shipped_sources() {
        if path == funnel {
            continue;
        }
        scanned += 1;
        for (index, _) in source.match_indices(FIELD) {
            if !source[index + FIELD.len()..]
                .trim_start()
                .starts_with(".record_")
            {
                continue;
            }
            found.push(format!(
                "{}:{}",
                path.display(),
                source[..index].matches('\n').count() + 1
            ));
        }
    }

    assert!(
        scanned > 100,
        "the scan covered only {scanned} shipped files; a scan that reaches nothing \
         passes forever"
    );
    assert!(
        found.is_empty(),
        "these announce a plan outcome without logging it \u{2014} call \
         `AppState::record_plan_receipt` or `record_plan_refusal` instead, which \
         write the status line and the Console record together:\n  {}",
        found.join("\n  ")
    );
}

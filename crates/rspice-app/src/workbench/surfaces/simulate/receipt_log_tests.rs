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
//! These tests pin the rules that make the second reader worth having: an
//! announced outcome reaches the Console, it is logged once per event rather
//! than once per frame, and a `PLAN` entry is a record rather than a notice.
//!
//! The mirror half of that last rule is asserted in `workbench::frame`'s own
//! tests, beside the mirror it is about. The activity mirror is private to the
//! frame loop, and widening it so this file could call it would be a worse
//! trade than stating the claim where the code is.

use super::*;

use crate::diagnostics::{LogSeverity, LogSource};

/// Every `PLAN` entry the Console holds, oldest first.
fn plan_entries(app: &RSpiceApp) -> Vec<(LogSeverity, String)> {
    app.state
        .model
        .log_buffer
        .entries()
        .filter(|entry| entry.source == LogSource::Plan)
        .map(|entry| (entry.severity, entry.message.clone()))
        .collect()
}

/// The first analysis instance of the fixture's plan.
fn first_instance(app: &RSpiceApp) -> AnalysisInstanceId {
    app.state
        .model
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
    app.state.drain_notices(&egui::Context::default());

    let entries = plan_entries(&app);
    assert_eq!(entries.len(), 1, "one command is one record: {entries:?}");
    assert_eq!(entries[0].0, LogSeverity::Info);
    assert_eq!(
        entries[0].1,
        app.state.simulate.analysis_lifecycle_status.message(),
        "the Console records the receipt itself, not a paraphrase of it"
    );
    assert!(
        entries[0].1.starts_with("Receipt #"),
        "the record is the receipt, sequence and all: {}",
        entries[0].1
    );
    assert!(
        app.state.workbench.toasts.activity().is_empty(),
        "a committed edit is not an event the reader has to be chased with"
    );
}

/// A refused command leaves exactly one record, at `Warning`, one prompt and
/// no toast.
///
/// The prompt answers the gesture wherever the user is, the Analyses
/// lifecycle line states the refusal on that route, and the Console keeps the
/// record; a toast would be one more statement of one refusal.
#[test]
fn a_refused_plan_command_is_recorded_once_and_raises_no_toast() {
    let mut app = RSpiceApp::test_instance();

    apply_analysis_action(
        &mut app,
        AnalysisInstanceId::new(),
        AnalysisAction::SetEnabled(true),
    );
    app.state.drain_notices(&egui::Context::default());

    let entries = plan_entries(&app);
    assert_eq!(entries.len(), 1, "one refusal is one record: {entries:?}");
    assert_eq!(entries[0].0, LogSeverity::Warning);
    assert_eq!(
        entries[0].1,
        app.state.simulate.analysis_lifecycle_status.message()
    );
    assert_eq!(
        app.state
            .workbench
            .prompt
            .as_ref()
            .map(|prompt| prompt.text.as_str()),
        Some(entries[0].1.as_str()),
        "the refusal answers the gesture on the status bar, wherever the user is"
    );
    assert!(
        app.state.workbench.toasts.activity().is_empty(),
        "a refusal raises no toast and no notice"
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

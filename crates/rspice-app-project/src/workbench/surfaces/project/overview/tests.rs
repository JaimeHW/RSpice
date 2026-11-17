//! The overview reports what is current, never a retained copy of what was.
//!
//! These pin the ways a stale value could be read as project status: a
//! completed failure rendered in the shape of a success, and a time the
//! reader cannot place.

use super::activity::{run_title, unix_seconds_to_millis};
use super::header::{header_facts, import_deck_intent};
use super::*;
use crate::state::SimulationRunLifecycle;

#[test]
fn completed_failure_is_not_rendered_as_success() {
    assert_eq!(
        lifecycle_label(SimulationRunLifecycle::Completed, false),
        "completed with failures"
    );
    assert_eq!(
        lifecycle_tone(SimulationRunLifecycle::Completed, false),
        Tone::Error
    );
}

#[test]
fn run_title_does_not_duplicate_retained_sequence_prefix() {
    assert_eq!(
        run_title(12, "Run 12 (10:31:02 AM)"),
        "Run 12 (10:31:02 AM)"
    );
    assert_eq!(run_title(12, "corner sweep"), "Run 12 · corner sweep");
}

fn operation_at(timestamp_ms: Option<u64>) -> OperationSnapshot {
    let (timestamp_ms, stamp) = OperationSnapshot::dated(timestamp_ms);
    OperationSnapshot {
        timestamp_ms,
        stamp,
        event: String::new(),
        elapsed: None,
        detail: String::new(),
        tone: Tone::Neutral,
        intent: None,
    }
}

/// Inside a day the Activity table says how long ago; past one, or for a
/// time ahead of the clock, the date and time, which the pointer reads in
/// full on every row.
#[test]
fn an_operation_says_how_long_ago_inside_a_day_and_its_date_past_one() {
    assert_eq!(unix_seconds_to_millis(f64::NAN), None);
    assert_eq!(unix_seconds_to_millis(-1.0), None);
    assert_eq!(unix_seconds_to_millis(1.25), Some(1_250));

    let at = 1_700_000_000_000;
    let full = "2023-11-14 22:13 UTC";
    let operation = operation_at(Some(at));
    let when = |now: u64| {
        operation
            .when(now)
            .map(|(short, full)| (short, full.to_owned()))
    };
    assert_eq!(
        when(at + 30_000),
        Some(("just now".to_owned(), full.to_owned()))
    );
    assert_eq!(
        when(at + 2 * 3_600_000),
        Some(("2 h ago".to_owned(), full.to_owned()))
    );
    assert_eq!(
        when(at + 25 * 3_600_000),
        Some((full.to_owned(), full.to_owned()))
    );
    assert_eq!(when(at - 60_000), Some((full.to_owned(), full.to_owned())));
    assert_eq!(operation_at(None).when(at), None);
}

#[test]
fn run_destination_only_leaves_simulate_for_schematic_projects() {
    assert_eq!(run_destination(true, false), Workspace::Results);
    assert_eq!(run_destination(true, true), Workspace::Results);
    assert_eq!(run_destination(false, false), Workspace::Simulate);
    assert_eq!(run_destination(false, true), Workspace::Netlist);
}

#[test]
fn overview_import_deck_uses_the_independent_netlist_project_workflow() {
    assert_eq!(
        import_deck_intent(),
        OverviewIntent::Command(ids::OPEN_NETLIST)
    );
}

/// The line under the name says the revision, and where the file is only
/// once there is one: "Not yet saved" there was the Unsaved work row's fact a
/// second time.
#[test]
fn the_header_states_the_revision_and_the_file_once_there_is_one() {
    assert_eq!(header_facts(1, None), "Revision 1");
    assert_eq!(
        header_facts(4, Some("C:/work/amp.rspiceproj")),
        "Revision 4 \u{b7} C:/work/amp.rspiceproj"
    );
}

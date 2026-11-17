//! The overview reports what is current, never a retained copy of what was.
//!
//! These pin the ways a stale value could be read as project status: a
//! completed failure rendered in the shape of a success, and a display limit
//! reported as though it were the length of the history.

use super::activity::{operations_meta, relative_time, run_title, unix_seconds_to_millis};
use super::header::{header_facts, import_deck_intent};
use super::*;
use crate::state::SimulationRunLifecycle;

#[test]
fn operations_meta_never_reports_the_display_limit_as_the_history() {
    assert_eq!(operations_meta(5, 23), "5 of 23");
    assert_eq!(operations_meta(3, 3), "");
    assert_eq!(operations_meta(0, 0), "");
}

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

#[test]
fn recent_operation_timestamps_fail_closed_and_format_relative_age() {
    assert_eq!(unix_seconds_to_millis(f64::NAN), None);
    assert_eq!(unix_seconds_to_millis(-1.0), None);
    assert_eq!(unix_seconds_to_millis(1.25), Some(1_250));
    assert_eq!(relative_time(1_000, 999), "time n/a");
    assert_eq!(relative_time(1_000, 1_030), "now");
    assert_eq!(relative_time(1_000, 61_000), "1 min");
    assert_eq!(relative_time(1_000, 3_601_000), "1 h");
    assert_eq!(relative_time(1_000, 86_401_000), "1 d");
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

//! The overview reports what is current, never a retained copy of what was.
//!
//! These pin the ways a stale value could be read as project status: a
//! completed failure rendered in the shape of a success, and a time the
//! reader cannot place.

use super::*;
use crate::state::SimulationRunLifecycle;
use crate::workbench::project::health::{
    lifecycle_label, lifecycle_tone, run_title, told, unix_seconds_to_millis,
};

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
    // A label that only says when the run started reads as the run: the
    // row states the time.
    assert_eq!(run_title(12, "Run 12 (10:31:02 AM)"), "Run 12");
    assert_eq!(run_title(12, "Run 12 (21 Nov 2026, 1:04:01 PM)"), "Run 12");
    assert_eq!(
        run_title(12, "fast corner (21 Nov 2026, 1:04:01 PM)"),
        "Run 12 \u{b7} fast corner"
    );
    assert_eq!(
        run_title(12, "Run 12 (fast corner)"),
        "Run 12 (fast corner)"
    );
    assert_eq!(run_title(12, "corner sweep"), "Run 12 · corner sweep");
}

/// Inside a day the Latest run fact says how long ago; past one, or for a
/// time ahead of the clock, the date and time.
#[test]
fn a_run_says_how_long_ago_inside_a_day_and_its_date_past_one() {
    assert_eq!(unix_seconds_to_millis(f64::NAN), None);
    assert_eq!(unix_seconds_to_millis(-1.0), None);
    assert_eq!(unix_seconds_to_millis(1.25), Some(1_250));

    let at = 1_700_000_000_000;
    let full = crate::time_compat::local_stamp(at);
    assert_eq!(told(at, at + 30_000), "just now");
    assert_eq!(told(at, at + 2 * 3_600_000), "2 h ago");
    assert_eq!(told(at, at + 25 * 3_600_000), full);
    // A run started this instant can be stamped a millisecond past the
    // clock read after it.
    assert_eq!(told(at, at - 1), "just now");
    assert_eq!(told(at, at - 60_000), full);
}

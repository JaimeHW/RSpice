//! The overview reports what is current, never a retained copy of what was.
//!
//! These pin the three ways a stale value could be read as project status: a
//! check result kept past the edit that invalidated it, a completed failure
//! rendered in the shape of a success, and a display limit reported as though
//! it were the length of the history.

use super::*;

#[test]
fn stale_checks_never_reuse_retained_result_copy() {
    let copy = drc_copy(
        false,
        Some(DrcCounts {
            critical: 0,
            errors: 0,
            warnings: 0,
            info: 0,
        }),
    );
    assert_eq!(copy.state, "Not current");
    assert_eq!(copy.tone, Tone::Warn);
}

#[test]
fn current_check_blockers_are_counted_without_losing_advisories() {
    let copy = drc_copy(
        true,
        Some(DrcCounts {
            critical: 1,
            errors: 2,
            warnings: 3,
            info: 1,
        }),
    );
    assert_eq!(copy.state, "3 blocking findings");
    assert_eq!(copy.detail, "1 critical · 2 errors · 4 advisories");
    assert_eq!(copy.tone, Tone::Error);
}

#[test]
fn project_status_counts_areas_and_never_borrows_the_advisory_word() {
    let status = |tone| StatusSnapshot {
        area: "Area",
        state: String::new(),
        detail: String::new(),
        action: "",
        tone,
        intent: OverviewIntent::OpenProjectRoot,
        enabled: true,
        disabled_reason: None,
    };
    let statuses = [
        status(Tone::Neutral),
        status(Tone::Warn),
        status(Tone::Warn),
        status(Tone::Error),
    ];

    assert_eq!(
        project_status_summary(&statuses, Tone::Error),
        "blocked · 3 of 4 areas"
    );
    assert_eq!(
        project_status_summary(&statuses[..3], Tone::Warn),
        "review · 2 of 3 areas"
    );
    // A project with nothing to act on says nothing beside its name: the
    // Project status card and its register are the owners of that fact.
    assert_eq!(
        project_status_summary(&statuses[..1], Tone::Neutral),
        String::new()
    );
    // "advisory" belongs to finding counts, which this summary never counts.
    for tone in [Tone::Error, Tone::Warn, Tone::Neutral] {
        assert!(!project_status_summary(&statuses, tone).contains("advisor"));
    }
}

#[test]
fn operations_meta_never_reports_the_display_limit_as_the_history() {
    assert_eq!(operations_meta(5, 23), "5 of 23 retained");
    assert_eq!(operations_meta(3, 3), "3 retained");
    assert_eq!(operations_meta(0, 0), "0 retained");
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

//! What a sign-off report says: its verdict against waived and unwaived
//! errors, the waivers that cover nothing, and its CSV escaping.

use super::*;
use crate::workbench::app_state::diagnostic_store::DiagnosticInputs;
use crate::workbench::app_state::waivers::WaiverAction;
use rspice_design::drc::{DrcResult, DrcViolationType};

const DATE: &str = "2026-10-17T09:00:00Z";

fn open_state() -> AppState {
    let mut state = AppState::default();
    state.project_lifecycle.authority.open_session();
    state
}

/// An open pin, stated as an error, as Check Design's finding about the
/// configured root.
fn publish_open_pin(state: &mut AppState, message: &str) {
    let mut finding = DrcViolation::new(
        1,
        DrcViolationType::UnconnectedPin,
        message,
        DrcLocation::Terminal {
            component_id: 1,
            component_name: "R1".to_owned(),
            pin_name: "1".to_owned(),
            point: crate::state::Point::origin(),
        },
    );
    finding.severity = DrcSeverity::Error;
    let mut result = DrcResult::new();
    result.add_violation(finding);
    result.completed = true;
    state
        .publish_configured_root_design_check_result(result)
        .expect("the root's findings publish");
}

fn publish_clean(state: &mut AppState) {
    let mut result = DrcResult::new();
    result.completed = true;
    state
        .publish_configured_root_design_check_result(result)
        .expect("the root's findings publish");
}

#[test]
fn an_unwaived_error_fails_sign_off_and_a_waived_one_does_not() {
    let mut state = open_state();
    publish_open_pin(&mut state, "Pin R1.1 is not connected");

    let failing = state.signoff_report(DATE).expect("the report builds");
    assert!(!failing.passed);
    assert!(
        failing
            .summary
            .ends_with("RESULT: FAIL (1 unwaived error)\n"),
        "{}",
        failing.summary
    );
    assert!(
        failing
            .summary
            .contains(&format!("Date              {DATE}"))
    );

    let root = state.workspace.content.simulation_root_reference();
    let target = state
        .diagnostics
        .batch(&DiagnosticInputs::DesignCheck { subject: root })
        .expect("the finding is on the page")
        .records
        .iter()
        .find_map(|record| match &record.waiver_action {
            Some(WaiverAction::Waive(target)) => Some(target.clone()),
            _ => None,
        })
        .expect("an open pin may be waived");
    state
        .waive_finding(&target, "left open, bonded out at package level")
        .expect("the waiver is recorded");

    let passing = state.signoff_report(DATE).expect("the report builds");
    assert!(passing.passed);
    assert!(
        passing.summary.ends_with("RESULT: PASS\n"),
        "{}",
        passing.summary
    );
    let row = passing
        .detail
        .lines()
        .nth(1)
        .expect("the finding has a row");
    assert!(
        row.starts_with("DRC-UNCONNECTED-PIN,Error,")
            && row.contains(",yes,\"left open, bonded out at package level\","),
        "{row}"
    );
    assert!(passing.summary.contains("Unmatched waivers\n  none\n"));

    // The pin is wired: the waiver now covers nothing, and says so.
    publish_clean(&mut state);
    let wired = state.signoff_report(DATE).expect("the report builds");
    assert!(wired.passed);
    assert!(
        wired.summary.contains(
            "DRC-UNCONNECTED-PIN  terminal:1:1  \"left open, bonded out at package level\""
        ),
        "{}",
        wired.summary
    );
}

#[test]
fn a_finding_s_fields_are_escaped_as_rfc_4180_writes_them() {
    assert_eq!(csv_field("plain"), "plain");
    assert_eq!(csv_field("a, b"), "\"a, b\"");
    assert_eq!(csv_field("say \"hi\""), "\"say \"\"hi\"\"\"");
    assert_eq!(csv_field("two\nlines"), "\"two\nlines\"");
    assert_eq!(csv_field("carriage\rreturn"), "\"carriage\rreturn\"");

    let mut state = open_state();
    publish_open_pin(&mut state, "Pin \"R1.1\", left open\nby the author");
    let report = state.signoff_report(DATE).expect("the report builds");
    assert!(report.detail.starts_with(
        "rule,severity,cellview,occurrence,object,message,waived,waiver_reason,\
         waiver_author,waiver_date\r\n"
    ));
    assert!(
        report
            .detail
            .contains(",\"Pin \"\"R1.1\"\", left open\nby the author\",no,,,\r\n"),
        "{}",
        report.detail
    );
}

/// A report is built from a check of the design as it stands: an edit after
/// one report is in the next, which runs Check Design again.
#[test]
fn a_report_is_never_built_from_a_check_the_design_moved_away_from() {
    let mut state = open_state();
    let before = state.signoff_report(DATE).expect("the report builds");
    state.schematic.add_component(
        crate::state::ComponentType::Resistor,
        crate::state::Point::new(400, 400),
    );
    let after = state.signoff_report(DATE).expect("the report builds");
    let digest = |report: &SignoffReport| {
        report
            .summary
            .lines()
            .find(|line| line.starts_with("Input digest"))
            .map(str::to_owned)
    };
    assert_ne!(digest(&before), digest(&after));
    assert!(after.detail.contains("R1"), "{}", after.detail);
}

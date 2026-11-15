//! The design check run through the whole application: a check that finds errors warns
//! in the notices and opens the Problems page.

use crate::workbench::app_state::AppState;
use rspice_app_design::workbench::app::actions::workspace::design_checks::*;
use rspice_app_state::diagnostics::LogSeverity;
use rspice_design::drc::DrcResult;
use rspice_design::drc::{DrcLocation, DrcSeverity, DrcViolation, DrcViolationType};

fn finding(severity: DrcSeverity) -> DrcResult {
    let mut violation = DrcViolation::new(
        1,
        DrcViolationType::UnconnectedPin,
        "R1 pin 2 is not connected",
        DrcLocation::Point { x: 0.0, y: 0.0 },
    );
    violation.severity = severity;
    let mut result = DrcResult::new();
    result.add_violation(violation);
    result
}

/// A check that found errors says so at Warning and takes the reader to
/// the Problems page; a clean one is a record. Neither is toasted.
#[test]
fn a_check_that_finds_errors_warns_and_opens_problems() {
    let ctx = egui::Context::default();
    let mut state = AppState::default();
    report(
        &mut state.model,
        &mut state.workbench,
        "Design checks",
        Ok(finding(DrcSeverity::Error)),
    );
    state.drain_notices(&ctx);
    let line = state
        .model
        .log_buffer
        .entries()
        .last()
        .expect("the check is recorded");
    assert_eq!(line.severity, LogSeverity::Warning);
    assert!(
        line.message
            .starts_with("Design checks: 1 error, 0 advisories.")
    );
    assert!(state.workbench.console_visible);
    assert_eq!(
        state.workbench.console_page,
        crate::workbench::state::ConsolePage::Problems
    );
    assert!(state.workbench.toasts.activity().is_empty());

    let mut state = AppState::default();
    let page = state.workbench.console_page;
    report(
        &mut state.model,
        &mut state.workbench,
        "Design checks",
        Ok(finding(DrcSeverity::Warning)),
    );
    state.drain_notices(&ctx);
    let line = state
        .model
        .log_buffer
        .entries()
        .last()
        .expect("the check is recorded");
    assert_eq!(line.severity, LogSeverity::Info);
    assert_eq!(state.workbench.console_page, page);
    assert!(state.workbench.toasts.activity().is_empty());
}

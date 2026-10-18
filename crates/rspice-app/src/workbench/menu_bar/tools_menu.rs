//! Design-check actions (Design menu). Rendering lives in
//! `crate::workbench::chrome::title_bar`.

use crate::workbench::app_state::AppState;
use rspice_design::drc::DrcResult;

/// Check the open cellview's own findings.
///
/// The findings themselves live in the check receipt, which is what the
/// Problems page, the canvas badges and the docbar pill all read. This writes
/// one Console line, because a check having run is a session event; it does
/// not copy the findings into the Console as well. Copying them produced two
/// rows per finding on the Problems page — one from the receipt and one from
/// the log — and a count that said twice the truth.
pub(crate) fn check_cellview(state: &mut AppState) {
    let checked = state.check_active_cellview();
    report(state, "Cellview checks", checked);
}

/// Check the whole design from the simulation root, under its configuration.
pub(crate) fn check_design(state: &mut AppState) {
    let checked = state.check_design();
    report(state, "Design checks", checked);
}

fn report(state: &mut AppState, check: &str, checked: Result<DrcResult, String>) {
    let result = match checked {
        Ok(result) => result,
        Err(error) => {
            state.report_failure(format!("{check} could not run"), error);
            return;
        }
    };
    let summary = result.summary();
    let errors = summary.critical + summary.errors;
    let advisories = summary.warnings + summary.info;
    state.record_check_outcome(
        &result,
        format!(
            "{check}: {errors} error{}, {advisories} advisor{}. See Problems.",
            if errors == 1 { "" } else { "s" },
            if advisories == 1 { "y" } else { "ies" },
        ),
    );
    // A check that found errors takes the reader to them, the way a preflight
    // that blocks a run does. Nothing is toasted: the reader asked, and the
    // Problems page is the answer.
    if !result.passed() {
        state.show_problem_findings();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::LogSeverity;
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
        let mut state = AppState::default();
        report(&mut state, "Design checks", Ok(finding(DrcSeverity::Error)));
        let line = state
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
        assert!(state.ui.toasts.activity().is_empty());

        let mut state = AppState::default();
        let page = state.workbench.console_page;
        report(
            &mut state,
            "Design checks",
            Ok(finding(DrcSeverity::Warning)),
        );
        let line = state
            .log_buffer
            .entries()
            .last()
            .expect("the check is recorded");
        assert_eq!(line.severity, LogSeverity::Info);
        assert_eq!(state.workbench.console_page, page);
        assert!(state.ui.toasts.activity().is_empty());
    }
}

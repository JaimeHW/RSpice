//! Design-check actions (Design menu). Rendering lives in
//! `crate::workbench::chrome::title_bar`.

use crate::diagnostics::{LogSeverity, LogSource};
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
            state.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!("{check} could not run: {error}"),
            );
            return;
        }
    };
    let summary = result.summary();
    let errors = summary.critical + summary.errors;
    let advisories = summary.warnings + summary.info;
    state.record(
        LogSeverity::Info,
        LogSource::Studio,
        format!(
            "{check}: {errors} error{}, {advisories} advisor{}. See Problems.",
            if errors == 1 { "" } else { "s" },
            if advisories == 1 { "y" } else { "ies" },
        ),
    );
}

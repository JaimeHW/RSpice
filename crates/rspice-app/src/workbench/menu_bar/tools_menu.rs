//! Design-check actions (Check menu). Rendering lives in
//! `crate::workbench::chrome::title_bar`.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app_state::AppState;

/// Run the design-rule / electrical-rule checks on the active schematic.
///
/// The findings themselves live in the check receipt, which is what the
/// Problems page, the canvas badges and the docbar pill all read. This writes
/// one Console line, because a check having run is a session event; it does
/// not copy the findings into the Console as well. Copying them produced two
/// rows per finding on the Problems page — one from the receipt and one from
/// the log — and a count that said twice the truth.
pub(crate) fn run_design_rule_check(state: &mut AppState) {
    let result = match state.run_active_design_checks() {
        Ok(result) => result,
        Err(error) => {
            state.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!("Design checks could not run: {error}"),
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
            "Check: {errors} error{}, {advisories} advisor{}. See Problems.",
            if errors == 1 { "" } else { "s" },
            if advisories == 1 { "y" } else { "ies" },
        ),
    );
}

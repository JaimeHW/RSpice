//! Design-check actions (Check menu). Rendering lives in
//! `crate::workbench::chrome::title_bar`.

use crate::diagnostics::ConsoleMessage;
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
            state.push_user_message(ConsoleMessage::error(format!(
                "Design checks could not create current evidence: {error}"
            )));
            return;
        }
    };
    let summary = result.summary();
    let errors = summary.critical + summary.errors;
    let advisories = summary.warnings + summary.info;
    state.push_user_message(ConsoleMessage::info(format!(
        "Schematic checks · {errors} error{} · {advisories} advisor{}",
        if errors == 1 { "" } else { "s" },
        if advisories == 1 { "y" } else { "ies" },
    )));
}

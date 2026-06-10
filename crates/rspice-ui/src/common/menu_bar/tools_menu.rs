//! Design-check actions (Check menu). Rendering lives in
//! `crate::shell::menubar`.

use crate::common::app::{AppState, ConsoleMessage};

/// Run the design-rule / electrical-rule checks on the active schematic.
/// Results surface as the ERC pill in the schematic docbar and a console
/// summary line.
pub(crate) fn run_design_rule_check(state: &mut AppState) {
    let result = crate::services::drc::run_drc_check(&state.schematic);
    let summary = result.summary();
    let msg = if result.passed() {
        format!(
            "DRC passed: {} info, {} warnings",
            summary.info, summary.warnings
        )
    } else {
        format!(
            "DRC found {} violations ({} errors, {} critical)",
            summary.total, summary.errors, summary.critical
        )
    };
    state.push_user_message(ConsoleMessage::info(msg));
    state.dialogs.drc_results = Some(result);
}

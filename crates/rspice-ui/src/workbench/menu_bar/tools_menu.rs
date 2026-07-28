//! Design-check actions (Check menu). Rendering lives in
//! `crate::workbench::chrome::title_bar`.

use crate::diagnostics::{ConsoleMessage, LogSeverity, LogSource};
use crate::services::drc::{DrcConfig, DrcSeverity};
use crate::state::CellViewRef;
use crate::workbench::app::AppState;

/// Per-run cap on per-finding console rows; the rest stay reachable via
/// the canvas badges and typed finding-navigation commands.
const MAX_FINDING_ROWS: usize = 50;

/// Run the design-rule / electrical-rule checks on the active schematic.
/// Results surface as the ERC pill in the schematic docbar, canvas badges,
/// and console rows — one summary line plus a clickable row per finding
/// that jumps to its anchor.
pub(crate) fn run_design_rule_check(state: &mut AppState) {
    let hierarchy = crate::simulation::netlist_gen::HierarchySource::from_workspace(
        &state.library_manager,
        &state.workspace.schematic_buffers,
    );
    let root_schematic = CellViewRef::new(
        &state.workspace.project.root_library,
        &state.workspace.project.top_cell,
        "schematic",
    );
    let result = crate::services::drc::run_drc_check_with_hierarchy_and_config(
        &state.schematic,
        &hierarchy,
        DrcConfig {
            // Ground is a top-level circuit contract. Requiring every reusable
            // child cell to contain node 0 produces false blockers.
            check_missing_ground: state.workspace.active_schematic_reference() == root_schematic,
            ..DrcConfig::default()
        },
    );
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

    for violation in result.violations().iter().take(MAX_FINDING_ROWS) {
        let severity = match violation.severity {
            DrcSeverity::Critical | DrcSeverity::Error => LogSeverity::Error,
            DrcSeverity::Warning => LogSeverity::Warning,
            DrcSeverity::Info => LogSeverity::Info,
        };
        let anchor = crate::schematic::view::violations::finding_anchor(state, violation);
        state.log_buffer.log_anchored(
            severity,
            LogSource::Drc,
            violation.message.clone(),
            None,
            anchor,
        );
    }
    let hidden = result.violations().len().saturating_sub(MAX_FINDING_ROWS);
    if hidden > 0 {
        state.log_buffer.log(
            LogSeverity::Info,
            LogSource::Drc,
            format!("+{hidden} more findings — use the finding navigation commands"),
            None,
        );
    }

    state.dialogs.drc_results = Some(result);
    state.dialogs.drc_checked_version = state.schematic.topology_version();
}

//! Checking the design from the workbench.
//!
//! The model checks the design as it is configured. Where the drawing's
//! annotation could not be restored, the model syncs the open drawing into
//! the workspace first, so the check reads what the reader sees, and the
//! sync's prompt is stated here.

use rspice_design::drc::DrcResult;

use crate::state::{CellViewRef, ViewType};
use crate::workbench::app_state::design_checks::CheckScope;
use crate::workbench::app_state::{Model, SlotFacts};
use crate::workbench::state::WorkbenchSession;

/// Check Cellview, and say what it found.
///
/// The findings themselves live in the check receipt, which is what the
/// Problems page, the canvas badges and the docbar pill all read. This writes
/// one Console line, because a check having run is a session event; it does
/// not copy the findings into the Console as well. Copying them produced two
/// rows per finding on the Problems page — one from the receipt and one from
/// the log — and a count that said twice the truth.
pub(crate) fn check_cellview(model: &mut Model, wb: &mut WorkbenchSession, facts: SlotFacts) {
    let checked = check_active_cellview(model, wb, facts);
    report(model, wb, "Cellview checks", checked);
}

/// Check Design: the configured design, walked from the simulation root,
/// with every schematic of the project's own libraries it never reaches;
/// and say what it found.
pub(crate) fn check_design(model: &mut Model, wb: &mut WorkbenchSession, facts: SlotFacts) {
    let root = model.workspace.content.simulation_root_reference();
    let checked = run_design_check(model, wb, root, CheckScope::Design, facts);
    report(model, wb, "Design checks", checked);
}

/// The open cellview's own findings.
///
/// The hierarchy below is walked only for what no single cellview can
/// answer — whether the design reaches ground and whether a cycle runs
/// through it — so it answers in the time the one drawing takes.
pub(crate) fn check_active_cellview(
    model: &mut Model,
    wb: &mut WorkbenchSession,
    facts: SlotFacts,
) -> Result<DrcResult, String> {
    if !matches!(
        model.workspace.content.active_view_type(),
        Some(ViewType::Schematic | ViewType::Testbench)
    ) {
        return Err("only a schematic or a testbench has design rules to check".to_owned());
    }
    let subject = model.workspace.content.active_schematic_reference();
    run_design_check(model, wb, subject, CheckScope::Cellview, facts)
}

/// Check `subject` as far as `scope` reaches with the slots' facts, stating
/// what syncing the open drawing into the workspace had to say.
fn run_design_check(
    model: &mut Model,
    wb: &mut WorkbenchSession,
    subject: CellViewRef,
    scope: CheckScope,
    facts: SlotFacts,
) -> Result<DrcResult, String> {
    let (outcome, prompt) = model.run_synced_design_check(subject, scope, facts);
    if let Some(prompt) = prompt {
        wb.prompt(prompt);
    }
    outcome
}

/// Record what `check` found, and take the reader to the findings when it
/// found errors, the way a preflight that blocks a run does. Nothing is
/// toasted: the reader asked, and the Problems page is the answer.
pub(crate) fn report(
    model: &mut Model,
    wb: &mut WorkbenchSession,
    check: &str,
    checked: Result<DrcResult, String>,
) {
    if rspice_app_runs::workbench::preflight::record_design_check(model, check, checked) {
        wb.show_problem_findings();
    }
}

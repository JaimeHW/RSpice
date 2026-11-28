//! What every workflow shares: one plan-change transaction, the draft trait a
//! dialog's choice is finished through, and each draft's dialog.

use super::*;

/// Adopt one plan-data mutation as a single configuration transaction.
///
/// The workspace and the setup are mutated on clones and adopted only once the
/// change, the configuration validation, and the receipt have all succeeded, so
/// a rejected edit leaves no partial state behind and reports why. Every
/// registry edit routes through here, which is what makes "an edit produces a
/// receipt and invalidates preflight" one rule rather than a convention each
/// page repeats. Returns whether the transaction was adopted, which lets a
/// caller that renamed a record carry the selection onto its new name.
///
/// Capture-group membership is re-evaluated here for the same reason. A rule
/// group's membership is a function of the registry, so *every* registry edit
/// can move outputs between groups — adding an output inside `/X1`, deleting
/// one, or widening a rule all do. Re-resolving at each edit site would be a
/// rule each page had to remember; re-resolving here means an output cannot
/// change hands without the receipt for that edit saying so.
pub(crate) fn commit_plan_change(
    cx: &mut Cx<'_, SimulateFeature>,
    plan_id: crate::product::SimulationPlanId,
    detail: &str,
    change: impl FnOnce(
        &mut crate::state::ProjectWorkspace,
        crate::product::SimulationPlanId,
    ) -> Result<(), String>,
) -> bool {
    let step = cx.model.setup_snapshot(SetupParts::PLAN_DATA);
    let mut workspace = cx.model.workspace.clone();
    let mut setup = cx.model.sim_setup.clone();
    let before = plan_capture_state(&cx.model.workspace, plan_id);
    let outcome = change(&mut workspace, plan_id)
        .and_then(|()| {
            workspace
                .content
                .validate_simulation_configuration()
                .map_err(|error| error.to_string())
        })
        .and_then(|()| {
            let after = plan_capture_state(&workspace, plan_id);
            setup
                .commit_active_plan_configuration_change(membership_detail(detail, &before, &after))
                .map_err(|error| error.to_string())
        });
    match outcome {
        Ok(receipt) => {
            cx.model.workspace = workspace;
            cx.model.sim_setup = setup;
            // One step of the setup's history, named for the page that took
            // it.
            cx.model
                .record_setup_change(cx.state.simulation_page.label(), step);
            crate::workbench::simulate::invalidate_preflight(cx);
            cx.model.record_plan_receipt(receipt.status_line());
            true
        }
        Err(error) => {
            crate::workbench::simulate::record_plan_refusal(cx, error);
            false
        }
    }
}

/// One plan's outputs and the groups that own them, as of one instant.
///
/// Held together because a membership answer is only meaningful beside the
/// output list it was resolved from: the diff matches outputs by identity, and
/// the identities live here.
pub(super) struct PlanCaptureState {
    groups: Vec<crate::state::CaptureGroup>,
    outputs: Vec<SavedOutput>,
    membership: crate::state::CaptureGroupMembership,
}

/// Resolve one plan's membership from the authored registry.
///
/// The *authored* registry, not the effective set: probes and automatic
/// fallback outputs are not project records, and a receipt reporting that a
/// synthesized output changed groups would name something the reader cannot
/// find in their plan.
fn plan_capture_state(
    workspace: &crate::state::ProjectWorkspace,
    plan_id: crate::product::SimulationPlanId,
) -> PlanCaptureState {
    let payload = workspace.content.plan_data(plan_id);
    let groups = payload
        .map(|payload| payload.capture_groups.clone())
        .unwrap_or_default();
    let outputs = payload
        .map(|payload| payload.saved_outputs.clone())
        .unwrap_or_default();
    let membership = crate::state::CaptureGroupMembership::resolve(&groups, &outputs);
    PlanCaptureState {
        groups,
        outputs,
        membership,
    }
}

/// How many moves one receipt names before it counts the rest.
///
/// A receipt detail is bounded at 512 characters, and a rule widened across a
/// large plan can move dozens of outputs. Naming the first few and counting the
/// remainder keeps the common case — one or two outputs changing hands —
/// completely legible, and never truncates a name mid-word to make room.
const MEMBERSHIP_MOVES_NAMED: usize = 3;

/// The edit's own detail, plus what the edit moved between capture groups.
///
/// Appended to the detail rather than written as a second receipt because it is
/// not a second change: the membership moved *because* of this edit, and two
/// receipts would let a reader adopt one and not the other.
fn membership_detail(detail: &str, before: &PlanCaptureState, after: &PlanCaptureState) -> String {
    let namer = crate::state::group_namer(&after.groups);
    let moves = crate::state::CaptureGroupMembership::diff(
        (&before.membership, &before.outputs),
        (&after.membership, &after.outputs),
        &|id| {
            // A group the edit deleted is not in the new set, so it is named
            // from the old one; falling through to "Ungrouped" would report a
            // removal as a move to somewhere the output never went.
            before
                .groups
                .iter()
                .find(|group| group.id == id)
                .map_or_else(|| namer(id), |group| group.name.clone())
        },
    );
    if moves.is_empty() {
        return detail.to_owned();
    }
    let named = moves
        .iter()
        .take(MEMBERSHIP_MOVES_NAMED)
        .map(crate::state::MembershipMove::summary)
        .collect::<Vec<_>>()
        .join("; ");
    let remainder = moves.len().saturating_sub(MEMBERSHIP_MOVES_NAMED);
    let summary = if remainder == 0 {
        format!("{detail} Capture membership: {named}.")
    } else {
        format!("{detail} Capture membership: {named}; and {remainder} more.")
    };
    // The plan refuses a detail over 512 characters, and losing the edit's own
    // description to a long membership list would be the wrong half to drop.
    if summary.chars().count() > 512 {
        format!(
            "{detail} Capture membership: {} outputs moved.",
            moves.len()
        )
    } else {
        summary
    }
}

pub fn finish_workflow_choice<D>(
    cx: &mut Cx<'_, SimulateFeature>,
    choice: DialogChoice,
    mut draft: D,
    commit: fn(&mut Cx<'_, SimulateFeature>, &D) -> Result<String, String>,
) where
    D: Clone + Into<SimulationWorkflowDialog> + WorkflowDraft,
{
    match choice {
        DialogChoice::Primary => match commit(cx, &draft) {
            Ok(message) => {
                cx.state.simulation_workflow = None;
                cx.wb.prompt(message);
            }
            Err(error) => {
                set_workflow_error(&mut draft, error);
                cx.state.simulation_workflow = Some(draft.into());
            }
        },
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            cx.state.simulation_workflow = None;
        }
        DialogChoice::None | DialogChoice::Secondary => {
            cx.state.simulation_workflow = Some(draft.into());
        }
    }
}

pub trait WorkflowDraft {
    fn set_error(&mut self, error: String);
}

impl WorkflowDraft for ClonePlanDraft {
    fn set_error(&mut self, error: String) {
        self.validation_error = Some(error);
    }
}

impl WorkflowDraft for DesignVariableDraft {
    fn set_error(&mut self, error: String) {
        self.validation_error = Some(error);
    }
}

impl WorkflowDraft for SavedOutputDraft {
    fn set_error(&mut self, error: String) {
        self.validation_error = Some(error);
    }
}

impl WorkflowDraft for RenameAnalysisDraft {
    fn set_error(&mut self, error: String) {
        self.validation_error = Some(error);
    }
}

impl WorkflowDraft for CaptureGroupDraft {
    fn set_error(&mut self, error: String) {
        self.validation_error = Some(error);
    }
}

impl WorkflowDraft for AnalysisRunPointsDraft {
    fn set_error(&mut self, error: String) {
        self.validation_error = Some(error);
    }
}

impl WorkflowDraft for crate::workbench::state::DesignVariableImportDraft {
    fn set_error(&mut self, error: String) {
        self.validation_error = Some(error);
    }
}

pub(super) fn set_workflow_error(draft: &mut impl WorkflowDraft, error: String) {
    draft.set_error(error);
}

impl From<ClonePlanDraft> for SimulationWorkflowDialog {
    fn from(draft: ClonePlanDraft) -> Self {
        Self::ClonePlan(draft)
    }
}

impl From<DesignVariableDraft> for SimulationWorkflowDialog {
    fn from(draft: DesignVariableDraft) -> Self {
        Self::DesignVariable(draft)
    }
}

impl From<AnalysisRunPointsDraft> for SimulationWorkflowDialog {
    fn from(draft: AnalysisRunPointsDraft) -> Self {
        Self::AnalysisRunPoints(draft)
    }
}

impl From<crate::workbench::state::DesignVariableImportDraft> for SimulationWorkflowDialog {
    fn from(draft: crate::workbench::state::DesignVariableImportDraft) -> Self {
        Self::DesignVariableImport(draft)
    }
}

impl From<SavedOutputDraft> for SimulationWorkflowDialog {
    fn from(draft: SavedOutputDraft) -> Self {
        Self::SavedOutput(draft)
    }
}

impl From<RenameAnalysisDraft> for SimulationWorkflowDialog {
    fn from(draft: RenameAnalysisDraft) -> Self {
        Self::RenameAnalysis(draft)
    }
}

impl From<CaptureGroupDraft> for SimulationWorkflowDialog {
    fn from(draft: CaptureGroupDraft) -> Self {
        Self::CaptureGroup(draft)
    }
}

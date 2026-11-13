//! What the Simulation Studio's cases seed and open: the active plan's
//! identity, a capture group, and the variable import dialog opened on a
//! sheet. The studio's own cases and the simulate feature's whole-application
//! tests share them.

use crate::product::SimulationPlanId;
use crate::workbench::app_state::Model;
use crate::workbench::slots::SimulateState;
use crate::workbench::state::{DesignVariableImportDraft, SimulationWorkflowDialog};

use super::variable_import::import_draft_for_sheet;

/// The active plan's identity, for tests that seed plan-owned records.
pub(crate) fn plan_id(model: &Model) -> crate::product::SimulationPlanId {
    model
        .sim_setup
        .stable_analysis_plan()
        .expect("the test instance has a stable plan")
        .id()
}

/// Seed one plan with two outputs in different scopes and a group over the
/// first, and hand back the group's identity.
pub(crate) fn seed_capture_group(model: &mut Model) -> crate::product::CaptureGroupId {
    use crate::state::{
        CaptureGroup, CaptureGroupRule, InstancePath, SavedOutput, SavedOutputCompatibility,
        SavedOutputKind, SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming,
    };

    let id = plan_id(model);
    for (name, expression) in [("core_n", "V(x1.n)"), ("edge_n", "V(x2.n)")] {
        let output = SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            name,
            expression,
            SavedOutputCompatibility::AllCompatibleAnalyses,
            SavedOutputPolicy::SelectedAndFinalPoints,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("valid saved output");
        model
            .workspace
            .content
            .add_saved_output(id, output)
            .expect("the plan accepts the output");
    }
    let mut group = CaptureGroup::new("Core rails").expect("group name");
    group.rules.push(CaptureGroupRule::for_scope(
        InstancePath::parse_legacy("/x1").expect("scope"),
    ));
    group.points = Some(SavedOutputPolicy::EveryAcceptedPoint);
    model
        .workspace
        .content
        .add_capture_group(id, group)
        .expect("the plan accepts the group")
}

pub(crate) fn plan(model: &Model) -> SimulationPlanId {
    model
        .sim_setup
        .stable_analysis_plan()
        .expect("the test instance owns a stable plan")
        .id()
}

/// Drive the real entry point and hand back the draft it opened on.
pub(crate) fn open(
    model: &Model,
    simulate: &mut SimulateState,
    plan_id: SimulationPlanId,
    sheet: &str,
) -> DesignVariableImportDraft {
    let draft = import_draft_for_sheet(model, simulate, plan_id, "spec.csv", sheet);
    simulate.simulation_workflow = Some(SimulationWorkflowDialog::DesignVariableImport(draft));
    match simulate.simulation_workflow.clone() {
        Some(SimulationWorkflowDialog::DesignVariableImport(draft)) => draft,
        other => panic!("the import dialog must open, got {other:?}"),
    }
}

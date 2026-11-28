//! Committing an edited draft, or one analysis's solver overrides, to the plan.

use super::*;

pub(crate) fn commit_draft(
    cx: &mut Cx<'_, SimulateFeature>,
    id: AnalysisInstanceId,
    draft: AnalysisDraft,
) {
    let step = cx.model.setup_snapshot(SetupParts::ANALYSES);
    let result = match cx.model.sim_setup.stable_analysis_plan_mut() {
        Ok(plan) => plan
            .edit(id, |target| target.clone_from(&draft))
            .map_err(|error| error.to_string()),
        Err(error) => Err(error),
    };
    cx.model
        .record_setup_change(SimulationPage::Analyses.label(), step);
    match result {
        Ok(((), receipt)) => {
            refresh_analysis_projections(cx);
            record_receipt(cx, &receipt);
        }
        Err(error) => record_failure(cx, "Edit", &error),
    }
}

/// Commit one analysis's departures from the plan's solver policy.
///
/// Deliberately the same shape as [`commit_draft`]: a numerical override is a
/// plan mutation like any other, so it produces a receipt, refreshes the
/// projections, and invalidates preflight through the one path every editor
/// uses. A refusal leaves the plan untouched and is reported verbatim, because
/// the reason names which owner already holds the value.
pub(crate) fn commit_numeric_override(
    cx: &mut Cx<'_, SimulateFeature>,
    id: AnalysisInstanceId,
    record: Option<crate::simulation::plan::AnalysisNumericOverride>,
) -> Result<(), String> {
    let step = cx.model.setup_snapshot(SetupParts::ANALYSES);
    let result = match cx.model.sim_setup.stable_analysis_plan_mut() {
        Ok(plan) => plan
            .set_numeric_override(id, record)
            .map_err(|error| error.to_string()),
        Err(error) => Err(error),
    };
    cx.model
        .record_setup_change(SimulationPage::Analyses.label(), step);
    match result {
        Ok(receipt) => {
            refresh_analysis_projections(cx);
            record_receipt(cx, &receipt);
            Ok(())
        }
        Err(error) => {
            record_failure(cx, "Edit", &error);
            Err(error)
        }
    }
}

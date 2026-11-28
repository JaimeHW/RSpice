//! Committing an edited draft, or one analysis's solver overrides, to the plan.

use super::*;

/// Commit `draft` as the settings of the analysis `id`: one step of the
/// setup's history, and one plain line in the Console naming what changed.
/// `field` is the label of the field the value was typed in, when it was.
pub(crate) fn commit_draft(
    cx: &mut Cx<'_, SimulateFeature>,
    id: AnalysisInstanceId,
    draft: AnalysisDraft,
    field: Option<&str>,
) {
    let before = committed(cx.model, id, |instance| instance.draft().clone());
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
            let line = before.map_or_else(
                || receipt_line(&receipt),
                |(name, before)| edit_line(&name, field, &before, &draft),
            );
            cx.model.record_plan_receipt(line);
        }
        Err(error) => record_failure(cx, "Edit", &error),
    }
}

/// Commit one analysis's departures from the plan's solver policy.
///
/// Deliberately the same shape as [`commit_draft`]: a numerical override is a
/// plan mutation like any other, so it is one step of the setup's history,
/// refreshes the projections, and invalidates preflight through the one path
/// every editor uses. A refusal leaves the plan untouched and is reported
/// verbatim, because the reason names which owner already holds the value.
pub(crate) fn commit_numeric_override(
    cx: &mut Cx<'_, SimulateFeature>,
    id: AnalysisInstanceId,
    record: Option<crate::simulation::plan::AnalysisNumericOverride>,
) -> Result<(), String> {
    let before = committed(cx.model, id, |instance| {
        instance.numeric_override().cloned()
    });
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
            let after = committed(cx.model, id, |instance| {
                instance.numeric_override().cloned()
            });
            let line = match (before, after) {
                (Some((name, before)), Some((_, after))) => {
                    options_line(&name, before.as_ref(), after.as_ref())
                }
                _ => receipt_line(&receipt),
            };
            cx.model.record_plan_receipt(line);
            Ok(())
        }
        Err(error) => {
            record_failure(cx, "Edit", &error);
            Err(error)
        }
    }
}

/// What `read` takes from the analysis `id` as the plan holds it, beside the
/// name it shows under.
fn committed<T>(
    model: &Model,
    id: AnalysisInstanceId,
    read: impl FnOnce(&crate::simulation::plan::AnalysisInstance) -> T,
) -> Option<(String, T)> {
    let plan = model.sim_setup.stable_analysis_plan().ok()?;
    let instance = plan.instance(id)?;
    Some((instance.display_name().to_owned(), read(instance)))
}

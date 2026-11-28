//! Adding an analysis to the plan: the catalogue and the Run Set page both add here.

use super::*;

pub(super) type InsertAnalysisResult = Result<
    (
        AnalysisInstanceId,
        AnalysisLifecycleReceipt,
        Option<Result<AnalysisLifecycleReceipt, String>>,
    ),
    String,
>;

/// Add one instance of `kind` to the stable plan, and name it if it committed.
///
/// The identity comes back because the caller decides what happens next, and
/// the two callers want different things: the run-set page adds the Monte
/// Carlo controller its declaration needs and stays where it is, while the
/// catalogue carries the reader to the route that edits what was just added.
/// Both selections are the same one this function already wrote — the return
/// is what lets the second one act on it without re-reading the plan.
///
/// `None` is every refusal, each of which has already stated itself: a plan
/// that does not resolve, an insert the plan rejected.
pub fn insert_analysis_instance(
    cx: &mut Cx<'_, SimulateFeature>,
    kind: AnalysisKind,
) -> Option<AnalysisInstanceId> {
    cx.model.sim_setup.session.palette_open = false;
    cx.model.sim_setup.session.palette_query.clear();
    cx.model.sim_setup.session.palette_active = 0;
    cx.model.sim_setup.session.palette_scroll_to_active = false;
    let repair_context = build_envelope_source_catalog(cx.model).dependency_repair_context();
    let step = cx.model.setup_snapshot(SetupParts::ANALYSES);
    let result: InsertAnalysisResult = match cx.model.sim_setup.stable_analysis_plan_mut() {
        Ok(plan) => match plan.insert(kind) {
            Ok((id, insert_receipt)) => {
                let has_prerequisites = !plan.required_prerequisite_roles(id).is_empty();
                let bind_receipt = has_prerequisites.then(|| {
                    plan.auto_bind_dependencies_with_context(id, &repair_context)
                        .map_err(|error| error.to_string())
                });
                Ok((id, insert_receipt, bind_receipt))
            }
            Err(error) => Err(error.to_string()),
        },
        Err(error) => Err(error),
    };
    cx.model
        .record_setup_change(SimulationPage::Analyses.label(), step);
    match result {
        Ok((id, insert_receipt, bind_receipt)) => {
            refresh_analysis_projections(cx);
            cx.state.active_analysis_instance = Some(id);
            match bind_receipt {
                None => cx.model.record_plan_receipt(receipt_line(&insert_receipt)),
                Some(Ok(bind_receipt)) => {
                    let issue = cx
                        .model
                        .sim_setup
                        .stable_analysis_plan()
                        .ok()
                        .and_then(|plan| {
                            plan.validation_issues()
                                .iter()
                                .find(|issue| issue_applies_to(issue, id))
                                .map(format_plan_issue)
                        });
                    let readiness = issue.map_or_else(
                        || "Dependencies are explicitly bound.".to_owned(),
                        |issue| format!("Preflight remains blocked: {issue}"),
                    );
                    cx.model.record_plan_receipt(format!(
                        "Receipts #{} and #{} committed for instance {id}. {} {} {readiness}",
                        insert_receipt.sequence(),
                        bind_receipt.sequence(),
                        insert_receipt.detail(),
                        bind_receipt.detail(),
                    ));
                }
                Some(Err(error)) => {
                    crate::workbench::simulate::record_plan_refusal(
                        cx,
                        format!(
                            "Receipt #{} committed for instance {id}. {} Automatic dependency binding was rejected fail-closed: {error}. The inserted instance remains selected and preflight blocked.",
                            insert_receipt.sequence(),
                            insert_receipt.detail(),
                        ),
                    );
                }
            }
            Some(id)
        }
        Err(error) => {
            crate::workbench::simulate::record_plan_refusal(
                cx,
                failure_line("Add analysis", &error),
            );
            None
        }
    }
}

//! Adding an analysis to the plan: the catalogue and the Run Set page both add here.

use super::*;

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
    let result = match cx.model.sim_setup.stable_analysis_plan_mut() {
        Ok(plan) => {
            let position = plan.instances().len();
            plan.insert_with_prerequisites_with_context(
                AnalysisDraft::for_kind(kind),
                position,
                &repair_context,
            )
            .map_err(|error| error.to_string())
        }
        Err(error) => Err(error),
    };
    cx.model
        .record_setup_change(SimulationPage::Analyses.label(), step);
    match result {
        Ok((id, unprepared)) => {
            refresh_analysis_projections(cx);
            cx.state.active_analysis_instance = Some(id);
            let line = cx
                .model
                .sim_setup
                .stable_analysis_plan()
                .ok()
                .map(|plan| added_line(plan, id))
                .unwrap_or_default();
            match unprepared {
                None => cx.model.record_plan_receipt(line),
                Some(error) => crate::workbench::simulate::record_plan_refusal(
                    cx,
                    format!("{line} Choose what it uses on its form: {error}."),
                ),
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

/// What adding `id` did, by name: "Added Periodic AC 1, using PSS 1." or,
/// with nothing to read, "Added AC 1."
fn added_line(plan: &crate::simulation::plan::SimulationPlan, id: AnalysisInstanceId) -> String {
    let Some(instance) = plan.instance(id) else {
        return String::new();
    };
    let used = instance
        .dependencies()
        .iter()
        .filter_map(|dependency| plan.instance(dependency.target()))
        .map(|target| target.display_name().to_owned())
        .collect::<Vec<_>>();
    let missing = plan
        .required_prerequisite_roles(id)
        .into_iter()
        .filter(|role| {
            !instance
                .dependencies()
                .iter()
                .any(|dependency| dependency.prerequisite() == *role)
        })
        .map(prerequisite_need)
        .collect::<Vec<_>>();
    let name = instance.display_name();
    match (used.is_empty(), missing.is_empty()) {
        (_, false) => format!(
            "Added {name}. It needs {} before it.",
            missing.join(" and ")
        ),
        (false, true) => format!("Added {name}, using {}.", used.join(" and ")),
        (true, true) => format!("Added {name}."),
    }
}

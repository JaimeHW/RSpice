//! The one sentence a plan diagnostic is stated in.
//!
//! A graph diagnostic is a shape — a missing prerequisite, a cycle, a
//! dependency bound to the wrong kind — and a reader needs it as a sentence
//! that names the analysis and the fix. The sentences themselves live with the
//! plan (`plan_dependency::issue_sentence`), so the Check states a finding
//! exactly as the form does; this is where the form reaches them, with the
//! repair action an option is labelled with, the line a repair leaves, and the
//! dependency closure that decides which instance an issue belongs to.
//!
//! None of it edits anything and none of it paints a surface. That is the seam:
//! a route resolves its plan, asks here for the words, and lays them out
//! itself, so two surfaces reporting one diagnostic cannot word it differently.

use super::*;

pub(super) use rspice_simulation_contract::plan_dependency::{
    analysis_title, issue_is_damage, issue_reason, prerequisite_need,
};

#[cfg(test)]
pub(super) fn dependency_repair_cta(
    plan: &crate::simulation::plan::SimulationPlan,
    issues: &[AnalysisPlanIssue],
    dependencies: &[AnalysisDependency],
) -> Option<String> {
    let context = AnalysisDependencyRepairContext::exact_periodic_sources(
        "periodic fixture\nVIN_DIFF in 0 SIN(0 1 1k)\nR1 in 0 1k\n.end\n",
    )
    .expect("test periodic-source fixture is exact");
    dependency_repair_cta_with_context(plan, issues, dependencies, &context)
}

/// What `id` reads once its prerequisites are repaired, by name: "FFT
/// spectrum now uses Transient."
pub(super) fn now_uses_line(
    plan: &crate::simulation::plan::SimulationPlan,
    id: AnalysisInstanceId,
) -> String {
    let used = plan
        .instance(id)
        .map(|instance| {
            instance
                .dependencies()
                .iter()
                .filter_map(|dependency| plan.instance(dependency.target()))
                .map(|target| target.display_name().to_owned())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    if used.is_empty() {
        format!("{} needs nothing before it.", analysis_title(plan, id))
    } else {
        format!(
            "{} now uses {}.",
            analysis_title(plan, id),
            used.join(" and ")
        )
    }
}

pub(super) fn compatible_dependency_repair_label(
    plan: &crate::simulation::plan::SimulationPlan,
    dependent: AnalysisInstanceId,
    prerequisite: AnalysisKind,
    repair_context: &AnalysisDependencyRepairContext,
) -> String {
    let qualifier = if prerequisite == AnalysisKind::Transient
        && plan.instance(dependent).is_some_and(|instance| {
            matches!(instance.kind(), AnalysisKind::Fourier | AnalysisKind::Fft)
        }) {
        "compatible "
    } else {
        ""
    };
    let Some(position) = plan
        .instances()
        .iter()
        .position(|instance| instance.id() == dependent)
    else {
        return format!("Repair {} prerequisite", prerequisite.label());
    };
    let before = &plan.instances()[..position];
    let after = &plan.instances()[position + 1..];
    if before.iter().rev().any(|candidate| {
        candidate.enabled()
            && plan.dependency_candidate_is_compatible_with_context(
                dependent,
                prerequisite,
                candidate.id(),
                repair_context,
            )
    }) {
        format!("Bind {qualifier}{}", prerequisite.label())
    } else if before.iter().rev().any(|candidate| {
        plan.dependency_candidate_is_compatible_with_context(
            dependent,
            prerequisite,
            candidate.id(),
            repair_context,
        )
    }) {
        format!("Enable {qualifier}{}", prerequisite.label())
    } else if after.iter().any(|candidate| {
        candidate.enabled()
            && plan.dependency_candidate_is_compatible_with_context(
                dependent,
                prerequisite,
                candidate.id(),
                repair_context,
            )
    }) {
        format!("Move {qualifier}{} earlier", prerequisite.label())
    } else if after.iter().any(|candidate| {
        plan.dependency_candidate_is_compatible_with_context(
            dependent,
            prerequisite,
            candidate.id(),
            repair_context,
        )
    }) {
        format!(
            "Enable and move {qualifier}{} earlier",
            prerequisite.label()
        )
    } else {
        format!("Add {qualifier}{}", prerequisite.label())
    }
}

pub(super) fn dependency_closure_ids(
    plan: &crate::simulation::plan::SimulationPlan,
    root: AnalysisInstanceId,
) -> HashSet<AnalysisInstanceId> {
    let mut closure = HashSet::new();
    if plan
        .instance(root)
        .is_some_and(|instance| !instance.enabled())
    {
        closure.insert(root);
        return closure;
    }
    let mut pending = vec![root];
    while let Some(id) = pending.pop() {
        if !closure.insert(id) {
            continue;
        }
        if let Some(instance) = plan.instance(id) {
            let required_roles = plan.required_prerequisite_roles(id);
            pending.extend(instance.dependencies().iter().filter_map(|dependency| {
                let target = dependency.target();
                let role_is_unique = instance
                    .dependencies()
                    .iter()
                    .filter(|candidate| candidate.prerequisite() == dependency.prerequisite())
                    .count()
                    == 1;
                if target == id
                    || !role_is_unique
                    || !required_roles.contains(&dependency.prerequisite())
                {
                    return None;
                }
                plan.instance(target)
                    .filter(|candidate| candidate.kind() == dependency.prerequisite())
                    .map(|candidate| candidate.id())
            }));
        }
    }
    closure
}

pub(super) fn format_plan_issue(
    plan: &crate::simulation::plan::SimulationPlan,
    issue: &AnalysisPlanIssue,
) -> String {
    rspice_simulation_contract::plan_dependency::issue_sentence(plan, issue)
}

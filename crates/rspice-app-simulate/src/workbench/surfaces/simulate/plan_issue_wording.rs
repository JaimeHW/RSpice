//! The one sentence a plan diagnostic is stated in.
//!
//! A graph diagnostic is a shape — a missing prerequisite, a cycle, a
//! dependency bound to the wrong kind — and a reader needs it as a sentence
//! that names the fix. This is where that translation lives: the issue wording
//! every surface prints, the repair action the guided button is labelled with,
//! and the dependency closure that decides which instance an issue belongs to.
//!
//! None of it edits anything and none of it paints a surface. That is the seam:
//! a route resolves its plan, asks here for the words, and lays them out
//! itself, so two surfaces reporting one diagnostic cannot word it differently.

use super::*;

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

pub(super) fn format_plan_issue(issue: &AnalysisPlanIssue) -> String {
    match issue {
        AnalysisPlanIssue::NoEnabledInstances => {
            "The simulation plan has no enabled analysis instances.".to_owned()
        }
        AnalysisPlanIssue::DuplicateInstanceId { id } => {
            format!("Stable analysis identity {id} is duplicated.")
        }
        AnalysisPlanIssue::DuplicateTombstoneId { id } => {
            format!("Retired analysis identity {id} has duplicate tombstones.")
        }
        AnalysisPlanIssue::ReusedTombstonedId { id } => {
            format!("Retired analysis identity {id} was reused by an active instance.")
        }
        AnalysisPlanIssue::KindDraftMismatch {
            id,
            expected,
            actual,
        } => format!(
            "Analysis {id} requires a {} draft, but contains {}.",
            expected.label(),
            actual.label()
        ),
        AnalysisPlanIssue::InvalidInstanceRevision { id } => {
            format!("Analysis {id} has an invalid revision range.")
        }
        AnalysisPlanIssue::InvalidInstanceName { id } => {
            format!("Analysis {id} carries a name this plan cannot show.")
        }
        AnalysisPlanIssue::DuplicateInstanceName { id, name } => {
            format!("Analysis {id} shares the name \"{name}\" with another analysis.")
        }
        AnalysisPlanIssue::InvalidLifecycle { id, state, enabled } => {
            format!("Analysis {id} lifecycle {state} conflicts with enabled state {enabled}.")
        }
        AnalysisPlanIssue::MissingPrerequisite {
            dependent,
            prerequisite,
        } => format!(
            "Analysis {dependent} requires an earlier enabled {} instance.",
            prerequisite.label()
        ),
        AnalysisPlanIssue::UnexpectedDependencyRole {
            dependent,
            prerequisite,
        } => format!(
            "Analysis {dependent} does not accept {} as a prerequisite.",
            prerequisite.label()
        ),
        AnalysisPlanIssue::DuplicateDependencyRole {
            dependent,
            prerequisite,
        } => format!(
            "Analysis {dependent} binds {} more than once.",
            prerequisite.label()
        ),
        AnalysisPlanIssue::SelfDependency { dependent } => {
            format!("Analysis {dependent} cannot depend on itself.")
        }
        AnalysisPlanIssue::DanglingDependency { dependent, target } => {
            format!("Analysis {dependent} references missing prerequisite instance {target}.")
        }
        AnalysisPlanIssue::WrongDependencyKind {
            dependent,
            prerequisite,
            target,
            actual,
        } => format!(
            "Analysis {dependent} requires {} at {target}, but that instance is {}.",
            prerequisite.label(),
            actual.label()
        ),
        AnalysisPlanIssue::DisabledDependency { dependent, target } => {
            format!("Analysis {dependent} prerequisite instance {target} is disabled.")
        }
        AnalysisPlanIssue::DependencyNotEarlier { dependent, target } => {
            format!("Analysis {dependent} prerequisite instance {target} must appear earlier.")
        }
        AnalysisPlanIssue::IncompatibleDependencyConfiguration {
            dependent,
            prerequisite,
            target,
            detail,
        } => format!(
            "Analysis {dependent} cannot use {} instance {target}: {detail}.",
            prerequisite.label()
        ),
        AnalysisPlanIssue::DependencyCycle { members } => format!(
            "Analysis dependency cycle contains {} instance{}.",
            members.len(),
            if members.len() == 1 { "" } else { "s" }
        ),
        AnalysisPlanIssue::InvalidTombstoneRevision { id } => {
            format!("Retired analysis identity {id} has an invalid revision range.")
        }
        AnalysisPlanIssue::InvalidReceiptSequence { sequence } => {
            format!("Lifecycle receipt sequence {sequence} is not contiguous.")
        }
        AnalysisPlanIssue::InvalidReceiptRevision { sequence } => {
            format!("Lifecycle receipt {sequence} has an invalid revision transition.")
        }
        AnalysisPlanIssue::DanglingReceiptInstance { sequence, id } => {
            format!("Lifecycle receipt {sequence} references unknown analysis instance {id}.")
        }
        AnalysisPlanIssue::ReceiptKindMismatch {
            sequence,
            expected,
            actual,
        } => format!(
            "Lifecycle receipt {sequence} identifies {}, but its retained analysis is {}.",
            actual.label(),
            expected.label()
        ),
        AnalysisPlanIssue::EmptyReceiptDetail { sequence } => {
            format!("Lifecycle receipt {sequence} has no status detail.")
        }
        AnalysisPlanIssue::InvalidNextReceiptSequence { expected, actual } => {
            format!("Next lifecycle receipt sequence is {actual}; expected {expected}.")
        }
    }
}

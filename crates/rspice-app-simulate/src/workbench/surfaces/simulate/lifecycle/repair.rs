//! Prerequisite repair: which issues a repair can fix, and the label of the control
//! that offers it.

use super::*;

pub(super) const fn dependency_issue_is_repairable(issue: &AnalysisPlanIssue) -> bool {
    matches!(
        issue,
        AnalysisPlanIssue::MissingPrerequisite { .. }
            | AnalysisPlanIssue::UnexpectedDependencyRole { .. }
            | AnalysisPlanIssue::DuplicateDependencyRole { .. }
            | AnalysisPlanIssue::SelfDependency { .. }
            | AnalysisPlanIssue::DanglingDependency { .. }
            | AnalysisPlanIssue::WrongDependencyKind { .. }
            | AnalysisPlanIssue::DisabledDependency { .. }
            | AnalysisPlanIssue::DependencyNotEarlier { .. }
            | AnalysisPlanIssue::IncompatibleDependencyConfiguration { .. }
    )
}

pub(crate) fn dependency_repair_cta_with_context(
    plan: &crate::simulation::plan::SimulationPlan,
    issues: &[AnalysisPlanIssue],
    dependencies: &[AnalysisDependency],
    repair_context: &AnalysisDependencyRepairContext,
) -> Option<String> {
    let prerequisite_for_target = |dependent, target| {
        plan.instance(dependent)
            .and_then(|instance| {
                instance
                    .dependencies()
                    .iter()
                    .find(|dependency| dependency.target() == target)
                    .map(|dependency| dependency.prerequisite())
            })
            .or_else(|| {
                // Unit fixtures and legacy callers may supply the exact edge
                // separately. The owning instance remains authoritative when
                // it is available, avoiding ambiguity across a transitive
                // closure that happens to repeat a corrupt target identity.
                dependencies
                    .iter()
                    .find(|dependency| dependency.target() == target)
                    .map(|dependency| dependency.prerequisite())
            })
    };
    let repair_role = |issue: &AnalysisPlanIssue| match issue {
        AnalysisPlanIssue::MissingPrerequisite {
            dependent,
            prerequisite,
        }
        | AnalysisPlanIssue::UnexpectedDependencyRole {
            dependent,
            prerequisite,
        }
        | AnalysisPlanIssue::DuplicateDependencyRole {
            dependent,
            prerequisite,
        }
        | AnalysisPlanIssue::WrongDependencyKind {
            dependent,
            prerequisite,
            ..
        }
        | AnalysisPlanIssue::IncompatibleDependencyConfiguration {
            dependent,
            prerequisite,
            ..
        } => Some((*dependent, *prerequisite)),
        AnalysisPlanIssue::SelfDependency { dependent } => {
            prerequisite_for_target(*dependent, *dependent)
                .map(|prerequisite| (*dependent, prerequisite))
        }
        AnalysisPlanIssue::DanglingDependency { dependent, target }
        | AnalysisPlanIssue::DisabledDependency { dependent, target }
        | AnalysisPlanIssue::DependencyNotEarlier { dependent, target } => {
            prerequisite_for_target(*dependent, *target)
                .map(|prerequisite| (*dependent, prerequisite))
        }
        _ => None,
    };
    let complete_closure_is_repairable = |dependent| {
        plan.instance(dependent).is_some()
            && plan
                .required_prerequisite_roles(dependent)
                .iter()
                .all(|prerequisite| {
                    plan.dependency_prerequisite_is_repairable_with_context(
                        dependent,
                        *prerequisite,
                        repair_context,
                    )
                })
    };
    // Corrupt legacy state can contain an undeclared edge before a genuinely
    // repairable missing role. Do not let that unrelated issue suppress the
    // contextual quick action for the declared prerequisite.
    let issue = issues.iter().find(|issue| {
        dependency_issue_is_repairable(issue)
            && match issue {
                AnalysisPlanIssue::UnexpectedDependencyRole { dependent, .. } => {
                    complete_closure_is_repairable(*dependent)
                }
                _ => repair_role(issue).is_some_and(|(dependent, prerequisite)| {
                    complete_closure_is_repairable(dependent)
                        && plan.dependency_prerequisite_is_repairable_with_context(
                            dependent,
                            prerequisite,
                            repair_context,
                        )
                }),
            }
    })?;
    Some(match issue {
        AnalysisPlanIssue::MissingPrerequisite {
            dependent,
            prerequisite,
        } => compatible_dependency_repair_label(plan, *dependent, *prerequisite, repair_context),
        AnalysisPlanIssue::UnexpectedDependencyRole { prerequisite, .. } => {
            format!("Remove unexpected {} binding", prerequisite.label())
        }
        AnalysisPlanIssue::DuplicateDependencyRole { prerequisite, .. } => {
            format!("Resolve duplicate {} binding", prerequisite.label())
        }
        AnalysisPlanIssue::SelfDependency { dependent } => prerequisite_for_target(
            *dependent, *dependent,
        )
        .map_or_else(
            || "Replace self prerequisite".to_owned(),
            |prerequisite| {
                compatible_dependency_repair_label(plan, *dependent, prerequisite, repair_context)
            },
        ),
        AnalysisPlanIssue::DanglingDependency { dependent, target } => prerequisite_for_target(
            *dependent, *target,
        )
        .map_or_else(
            || "Replace missing prerequisite".to_owned(),
            |prerequisite| {
                compatible_dependency_repair_label(plan, *dependent, prerequisite, repair_context)
            },
        ),
        AnalysisPlanIssue::WrongDependencyKind {
            dependent,
            prerequisite,
            ..
        } => compatible_dependency_repair_label(plan, *dependent, *prerequisite, repair_context),
        AnalysisPlanIssue::DisabledDependency { dependent, target } => {
            let also_needs_move = issues.iter().any(|candidate| {
                matches!(
                    candidate,
                    AnalysisPlanIssue::DependencyNotEarlier {
                        dependent: candidate_dependent,
                        target: candidate_target,
                    } if candidate_dependent == dependent && candidate_target == target
                )
            });
            prerequisite_for_target(*dependent, *target).map_or_else(
                || {
                    if also_needs_move {
                        "Enable and move prerequisite earlier".to_owned()
                    } else {
                        "Enable prerequisite".to_owned()
                    }
                },
                |prerequisite| {
                    if also_needs_move {
                        format!("Enable and move {} earlier", prerequisite.label())
                    } else {
                        format!("Enable {}", prerequisite.label())
                    }
                },
            )
        }
        AnalysisPlanIssue::DependencyNotEarlier { dependent, target } => {
            prerequisite_for_target(*dependent, *target).map_or_else(
                || "Move prerequisite earlier".to_owned(),
                |prerequisite| format!("Move {} earlier", prerequisite.label()),
            )
        }
        AnalysisPlanIssue::IncompatibleDependencyConfiguration {
            dependent,
            prerequisite,
            ..
        } => compatible_dependency_repair_label(plan, *dependent, *prerequisite, repair_context),
        _ => unreachable!("repairable dependency issue must have a contextual action"),
    })
}

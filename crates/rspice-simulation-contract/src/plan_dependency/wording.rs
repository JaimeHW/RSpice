//! The plain sentences a test's plan findings are stated in.
//!
//! Every surface that reports a finding about the analyses of a test (the
//! analysis form, the Check, the line an edit leaves) states it here, by the
//! analysis's name and kind: "Periodic AC needs a periodic steady state
//! before it." A finding that only a damaged file can produce is stated as
//! damage and nothing more; its exact shape, identities included, is
//! [`damage_details`], for the log.

use rspice_vocabulary::product::AnalysisInstanceId;

use crate::analysis_kind::AnalysisKind;
use crate::plan_diagnostics::AnalysisPlanIssue;
use crate::plan_model::SimulationPlan;

/// An analysis as a sentence names it: its name, followed by its kind when
/// the name does not already say it ("Mixer gain (Periodic AC)", "Periodic
/// AC").
#[must_use]
pub fn analysis_title(plan: &SimulationPlan, id: AnalysisInstanceId) -> String {
    plan.instance(id).map_or_else(
        || "An analysis no longer in this test".to_owned(),
        |instance| {
            let name = instance.display_name();
            let kind = instance.kind().label();
            if name == kind {
                name.to_owned()
            } else {
                format!("{name} ({kind})")
            }
        },
    )
}

/// What an analysis of `kind` hands the analyses built on it, as a reason
/// names it: "needs a periodic steady state before it".
#[must_use]
pub const fn prerequisite_need(kind: AnalysisKind) -> &'static str {
    match kind {
        AnalysisKind::Pss => "a periodic steady state",
        AnalysisKind::HarmonicBalance => "a harmonic-balance steady state",
        AnalysisKind::Qpss => "a quasi-periodic steady state",
        AnalysisKind::Transient => "a transient",
        AnalysisKind::OperatingPoint => "an operating point",
        _ => "another analysis",
    }
}

/// Whether `issue` is one only a damaged file or a fault can produce, rather
/// than one an edit of the test leads to.
#[must_use]
pub const fn issue_is_damage(issue: &AnalysisPlanIssue) -> bool {
    !matches!(
        issue,
        AnalysisPlanIssue::NoEnabledInstances
            | AnalysisPlanIssue::DuplicateInstanceName { .. }
            | AnalysisPlanIssue::MissingPrerequisite { .. }
            | AnalysisPlanIssue::DisabledDependency { .. }
            | AnalysisPlanIssue::DependencyNotEarlier { .. }
            | AnalysisPlanIssue::NearerCarrier { .. }
            | AnalysisPlanIssue::IncompatibleDependencyConfiguration { .. }
    )
}

/// The sentence `issue` is shown as.
#[must_use]
pub fn issue_sentence(plan: &SimulationPlan, issue: &AnalysisPlanIssue) -> String {
    let title = |id| analysis_title(plan, id);
    match issue {
        AnalysisPlanIssue::NoEnabledInstances => {
            "No analysis in this test is switched on.".to_owned()
        }
        AnalysisPlanIssue::DuplicateInstanceName { name, .. } => {
            format!("Two analyses are called \"{name}\"; rename one.")
        }
        AnalysisPlanIssue::MissingPrerequisite {
            dependent,
            prerequisite,
        } => format!(
            "{} needs {} before it.",
            title(*dependent),
            prerequisite_need(*prerequisite)
        ),
        AnalysisPlanIssue::DisabledDependency { dependent, target } => format!(
            "{} uses {}, which is switched off.",
            title(*dependent),
            title(*target)
        ),
        AnalysisPlanIssue::DependencyNotEarlier { dependent, target } => format!(
            "{} uses {}, which runs after it; move {} up the list.",
            title(*dependent),
            title(*target),
            plan.instance(*target)
                .map_or("it", |instance| instance.display_name())
        ),
        AnalysisPlanIssue::NearerCarrier {
            dependent,
            target,
            nearer,
        } => {
            let name = |id| {
                plan.instance(id)
                    .map_or("it", |instance| instance.display_name())
            };
            format!(
                "{} uses {}, but {} is nearer above it, and a netlist of this test hands it the \
                 nearer one; use {} or move {} below it.",
                title(*dependent),
                name(*target),
                name(*nearer),
                name(*nearer),
                name(*target)
            )
        }
        AnalysisPlanIssue::IncompatibleDependencyConfiguration {
            dependent,
            target,
            detail,
            ..
        } => format!(
            "{} cannot use {}: {detail}.",
            title(*dependent),
            title(*target)
        ),
        // What it used was deleted: it needs one again, as if never bound.
        AnalysisPlanIssue::DanglingDependency { dependent, target } => format!(
            "{} needs {} before it.",
            title(*dependent),
            plan.instance(*dependent)
                .and_then(|instance| {
                    instance
                        .dependencies()
                        .iter()
                        .find(|dependency| dependency.target() == *target)
                })
                .map_or("another analysis", |dependency| {
                    prerequisite_need(dependency.prerequisite())
                })
        ),
        AnalysisPlanIssue::UnexpectedDependencyRole { dependent, .. }
        | AnalysisPlanIssue::DuplicateDependencyRole { dependent, .. }
        | AnalysisPlanIssue::SelfDependency { dependent }
        | AnalysisPlanIssue::WrongDependencyKind { dependent, .. } => format!(
            "{} is bound to an analysis it cannot use; choose what it uses again.",
            title(*dependent)
        ),
        AnalysisPlanIssue::DependencyCycle { members } => members.first().map_or_else(
            || "Analyses in this test wait on each other; choose what they use again.".to_owned(),
            |first| {
                format!(
                    "{} and the analyses it uses wait on each other; choose what they use again.",
                    title(*first)
                )
            },
        ),
        AnalysisPlanIssue::DuplicateInstanceId { id }
        | AnalysisPlanIssue::ReusedTombstonedId { id }
        | AnalysisPlanIssue::KindDraftMismatch { id, .. }
        | AnalysisPlanIssue::InvalidInstanceRevision { id }
        | AnalysisPlanIssue::InvalidInstanceName { id }
        | AnalysisPlanIssue::InvalidLifecycle { id, .. } => format!(
            "{} is damaged in the saved test; remove it and add it again.",
            title(*id)
        ),
        AnalysisPlanIssue::DuplicateTombstoneId { .. }
        | AnalysisPlanIssue::InvalidTombstoneRevision { .. } => {
            "This test's saved analysis list is damaged.".to_owned()
        }
    }
}

/// What `issue` says about the analysis it concerns, in the few words its
/// row in the list has room for, without naming that analysis: "needs a
/// periodic steady state", "uses Carrier, which is switched off".
#[must_use]
pub fn issue_reason(plan: &SimulationPlan, issue: &AnalysisPlanIssue) -> String {
    let name = |id| {
        plan.instance(id).map_or_else(
            || "an analysis no longer in this test".to_owned(),
            |instance| instance.display_name().to_owned(),
        )
    };
    match issue {
        AnalysisPlanIssue::NoEnabledInstances => "nothing is switched on".to_owned(),
        AnalysisPlanIssue::DuplicateInstanceName { .. } => "its name is used twice".to_owned(),
        AnalysisPlanIssue::MissingPrerequisite { prerequisite, .. } => {
            format!("needs {}", prerequisite_need(*prerequisite))
        }
        AnalysisPlanIssue::DisabledDependency { target, .. } => {
            format!("uses {}, which is switched off", name(*target))
        }
        AnalysisPlanIssue::DependencyNotEarlier { target, .. } => {
            format!("uses {}, which runs after it", name(*target))
        }
        AnalysisPlanIssue::NearerCarrier { target, nearer, .. } => {
            format!("uses {}, but {} is nearer", name(*target), name(*nearer))
        }
        AnalysisPlanIssue::IncompatibleDependencyConfiguration { target, .. } => {
            format!("cannot use {}", name(*target))
        }
        AnalysisPlanIssue::DanglingDependency { dependent, target } => format!(
            "needs {}",
            plan.instance(*dependent)
                .and_then(|instance| {
                    instance
                        .dependencies()
                        .iter()
                        .find(|dependency| dependency.target() == *target)
                })
                .map_or("another analysis", |dependency| {
                    prerequisite_need(dependency.prerequisite())
                })
        ),
        AnalysisPlanIssue::DependencyCycle { .. } => "waits on itself".to_owned(),
        _ => "damaged in the saved test".to_owned(),
    }
}

/// The exact shape of every damage finding in `plan`, identities included:
/// what the log records where the interface says only that something is
/// damaged.
#[must_use]
pub fn damage_details(plan: &SimulationPlan) -> Vec<String> {
    plan.validation_issues()
        .iter()
        .filter(|issue| issue_is_damage(issue))
        .map(ToString::to_string)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn every_issue(
        dependent: AnalysisInstanceId,
        target: AnalysisInstanceId,
    ) -> Vec<AnalysisPlanIssue> {
        use AnalysisKind::{Pac, Pss};
        vec![
            AnalysisPlanIssue::NoEnabledInstances,
            AnalysisPlanIssue::DuplicateInstanceId { id: dependent },
            AnalysisPlanIssue::DuplicateTombstoneId { id: target },
            AnalysisPlanIssue::ReusedTombstonedId { id: dependent },
            AnalysisPlanIssue::KindDraftMismatch {
                id: dependent,
                expected: Pac,
                actual: Pss,
            },
            AnalysisPlanIssue::InvalidInstanceRevision { id: dependent },
            AnalysisPlanIssue::InvalidInstanceName { id: dependent },
            AnalysisPlanIssue::DuplicateInstanceName {
                id: dependent,
                name: "Mixer gain".to_owned(),
            },
            AnalysisPlanIssue::InvalidLifecycle {
                id: dependent,
                state: crate::analysis_lifecycle::AnalysisLifecycleState::Draft,
                enabled: false,
            },
            AnalysisPlanIssue::MissingPrerequisite {
                dependent,
                prerequisite: Pss,
            },
            AnalysisPlanIssue::UnexpectedDependencyRole {
                dependent,
                prerequisite: Pss,
            },
            AnalysisPlanIssue::DuplicateDependencyRole {
                dependent,
                prerequisite: Pss,
            },
            AnalysisPlanIssue::SelfDependency { dependent },
            AnalysisPlanIssue::DanglingDependency { dependent, target },
            AnalysisPlanIssue::WrongDependencyKind {
                dependent,
                prerequisite: Pss,
                target,
                actual: Pac,
            },
            AnalysisPlanIssue::DisabledDependency { dependent, target },
            AnalysisPlanIssue::DependencyNotEarlier { dependent, target },
            AnalysisPlanIssue::NearerCarrier {
                dependent,
                target,
                nearer: target,
            },
            AnalysisPlanIssue::IncompatibleDependencyConfiguration {
                dependent,
                prerequisite: Pss,
                target,
                detail: "its tones differ".to_owned(),
            },
            AnalysisPlanIssue::DependencyCycle {
                members: vec![dependent, target],
            },
            AnalysisPlanIssue::InvalidTombstoneRevision { id: target },
        ]
    }

    #[test]
    fn a_reason_names_the_analysis_by_name_and_kind() {
        let mut plan = SimulationPlan::empty();
        let (pss, _) = plan.insert(AnalysisKind::Pss).unwrap();
        let (pac, _) = plan.insert(AnalysisKind::Pac).unwrap();
        let missing = AnalysisPlanIssue::MissingPrerequisite {
            dependent: pac,
            prerequisite: AnalysisKind::Pss,
        };
        assert_eq!(
            issue_sentence(&plan, &missing),
            "Periodic AC needs a periodic steady state before it."
        );

        plan.set_instance_name(pac, "Mixer gain").unwrap();
        plan.set_instance_name(pss, "Carrier").unwrap();
        assert_eq!(
            issue_sentence(&plan, &missing),
            "Mixer gain (Periodic AC) needs a periodic steady state before it."
        );
        assert_eq!(
            issue_reason(&plan, &missing),
            "needs a periodic steady state"
        );
        assert_eq!(
            issue_sentence(
                &plan,
                &AnalysisPlanIssue::DisabledDependency {
                    dependent: pac,
                    target: pss,
                }
            ),
            "Mixer gain (Periodic AC) uses Carrier (Periodic steady state), which is switched off."
        );
    }

    #[test]
    fn no_reason_shows_an_identity_and_damage_stays_in_the_log() {
        let mut plan = SimulationPlan::empty();
        let (pss, _) = plan.insert(AnalysisKind::Pss).unwrap();
        let (pac, _) = plan.insert(AnalysisKind::Pac).unwrap();
        let removed = AnalysisInstanceId::new();
        for (dependent, target) in [(pac, pss), (pac, removed), (removed, removed)] {
            for issue in every_issue(dependent, target) {
                let sentence = issue_sentence(&plan, &issue);
                for id in [dependent, target] {
                    assert!(
                        !sentence.contains(&id.to_string()),
                        "{issue:?} reads {sentence:?}"
                    );
                }
                assert!(
                    !sentence.to_lowercase().contains("prerequisite")
                        && !sentence.to_lowercase().contains("instance")
                        && !sentence.to_lowercase().contains("closure"),
                    "{sentence:?}"
                );
                assert!(sentence.ends_with('.'), "{sentence:?}");
                let reason = issue_reason(&plan, &issue);
                assert!(
                    !reason.ends_with('.')
                        && !reason.to_lowercase().contains("prerequisite")
                        && [dependent, target]
                            .iter()
                            .all(|id| !reason.contains(&id.to_string())),
                    "{issue:?} reads {reason:?}"
                );
            }
        }

        assert!(damage_details(&plan).is_empty());
        let (fft, _) = plan.insert(AnalysisKind::Fft).unwrap();
        assert!(
            damage_details(&plan).is_empty(),
            "a missing transient is an edit to make, not damage"
        );
        assert!(!issue_is_damage(&AnalysisPlanIssue::MissingPrerequisite {
            dependent: fft,
            prerequisite: AnalysisKind::Transient,
        }));
        assert!(issue_is_damage(&AnalysisPlanIssue::DanglingDependency {
            dependent: fft,
            target: removed,
        }));
    }
}

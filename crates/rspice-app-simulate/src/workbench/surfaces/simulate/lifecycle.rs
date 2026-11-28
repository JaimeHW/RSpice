//! Mutating the analysis stack: insert, commit, validate, remove, repair.
//!
//! Every mutation goes through this module and leaves one plain line in the
//! Console, so what the surface shows about an analysis is always what
//! actually happened to it. A draft is validated before it is committed and a failed action
//! records the failure rather than the intent, which is why a stale or
//! rejected edit can never appear as a configured analysis.

mod commit;
mod insert;
mod messages;
mod repair;

pub(crate) use commit::*;
pub use insert::*;
pub(crate) use messages::*;
pub(crate) use repair::*;

use super::*;

use crate::simulation::plan::AnalysisPlanError;
use crate::workbench::app::{PlanRemovalConsequence, PlanRemovalTarget, PlanRemovalTone};
use crate::workbench::app_state::design_history::SetupParts;
use crate::workbench::framework::feature::Cx;
use crate::workbench::simulate::{SimulateFeature, record_plan_refusal};

pub(super) fn resolve_active_analysis_instance(
    model: &Model,
    state: &mut SimulateState,
) -> Result<(), String> {
    let current = state.active_analysis_instance;
    let index = state.active_analysis;
    let resolved = {
        let plan = model.sim_setup.stable_analysis_plan()?;
        match current {
            Some(id) if plan.instance(id).is_some() => Some(id),
            Some(_) => plan.instances().first().map(|instance| instance.id()),
            None => AnalysisKind::from_singleton_index(index)
                .and_then(|kind| {
                    plan.instances()
                        .iter()
                        .find(|instance| instance.kind() == kind)
                })
                .or_else(|| plan.instances().first())
                .map(|instance| instance.id()),
        }
    };
    state.active_analysis_instance = resolved;
    Ok(())
}

pub(super) fn selected_analysis(
    model: &Model,
    state: &SimulateState,
    dependency_sources: &EnvelopeSourceCatalog,
) -> Result<Option<SelectedAnalysis>, String> {
    let Some(id) = state.active_analysis_instance else {
        return Ok(None);
    };
    let plan = model.sim_setup.stable_analysis_plan()?;
    let Some((position, instance)) = plan
        .instances()
        .iter()
        .enumerate()
        .find(|(_, instance)| instance.id() == id)
    else {
        return Err("The selected analysis is no longer in this test.".to_owned());
    };
    let closure_ids = dependency_closure_ids(plan, id);
    let issues = plan
        .validation_issues()
        .into_iter()
        .filter(|issue| {
            closure_ids
                .iter()
                .any(|closure_id| issue_applies_to(issue, *closure_id))
        })
        .collect::<Vec<_>>();
    let dependency_closure: Vec<AnalysisDependency> = closure_ids
        .iter()
        .filter_map(|closure_id| plan.instance(*closure_id))
        .flat_map(|instance| instance.dependencies().iter().copied())
        .collect();
    let contextual_dependency_error =
        contextual_dependency_error(plan, id, &closure_ids, dependency_sources);
    let repair_context = dependency_sources.dependency_repair_context();
    let prerequisite_roles = plan.required_prerequisite_roles(id);
    let mut candidate_roles = prerequisite_roles.clone();
    for optional in instance.kind().optional_prerequisites() {
        if !candidate_roles.contains(optional) {
            candidate_roles.push(*optional);
        }
    }
    let prerequisite_candidates = candidate_roles
        .iter()
        .map(|prerequisite| {
            let candidates = plan.instances()[..position]
                .iter()
                .enumerate()
                .filter(|(_, candidate)| {
                    candidate.enabled()
                        && plan.dependency_candidate_is_compatible_with_context(
                            id,
                            *prerequisite,
                            candidate.id(),
                            &repair_context,
                        )
                })
                .map(|(_, candidate)| DependencyCandidate {
                    id: candidate.id(),
                    // What it is called, as the list beside it names it.
                    label: candidate.display_name().to_owned(),
                })
                .collect();
            (*prerequisite, candidates)
        })
        .collect();
    let disabled_prerequisite_label = (!instance.enabled())
        .then(|| {
            prerequisite_roles.first().map(|prerequisite| {
                compatible_dependency_repair_label(plan, id, *prerequisite, &repair_context)
            })
        })
        .flatten();
    let proposed_repair_label =
        dependency_repair_cta_with_context(plan, &issues, &dependency_closure, &repair_context)
            .or_else(|| {
                contextual_dependency_error
                    .is_some()
                    .then(|| "Repair PSS prerequisite".to_owned())
            })
            .or(disabled_prerequisite_label);
    // The visible quick action must be executable for the selected root, not
    // merely for the first transitive issue used to describe it. Preview the
    // exact transactional repair against the same circuit context and omit an
    // action that would fail closed.
    let repair_label = proposed_repair_label.filter(|_| {
        let mut preview = plan.clone();
        preview
            .repair_dependencies_with_context(id, &repair_context)
            .is_ok()
    });
    let phase_noise_requires_user_pss = matches!(
        instance.draft(),
        AnalysisDraft::Pnoise(draft)
            if draft
                .to_config()
                .is_ok_and(|config| config.noise_ref == NoiseReferenceType::Phase)
    );
    let prepared_pss = prepared_autonomous_pss_id(plan, id);
    let configure_pss_label =
        (phase_noise_requires_user_pss && repair_label.is_none()).then(|| {
            if prepared_pss.is_some() {
                "Configure existing autonomous PSS".to_owned()
            } else {
                "Add and configure autonomous PSS".to_owned()
            }
        });
    // The reason the form gives: the first finding about the analysis or what
    // it uses, else the circuit's objection to the PSS it uses.
    let reason = issues
        .first()
        .map(|issue| format_plan_issue(plan, issue))
        .or_else(|| contextual_dependency_error.clone());
    Ok(Some(SelectedAnalysis {
        id,
        kind: instance.kind(),
        name: instance.display_name().to_owned(),
        draft: instance.draft().clone(),
        dependencies: instance.dependencies().to_vec(),
        dependency_names: instance
            .dependencies()
            .iter()
            .filter_map(|dependency| {
                plan.instance(dependency.target())
                    .map(|target| (target.id(), target.display_name().to_owned()))
            })
            .collect(),
        prerequisite_roles,
        prerequisite_candidates,
        lifecycle: instance.lifecycle(),
        position,
        plan_length: plan.instances().len(),
        issues,
        contextual_dependency_error: reason,
        repair_label,
        configure_pss_label,
        enabled: instance.enabled(),
    }))
}

pub(super) fn prepared_autonomous_pss_id(
    plan: &crate::simulation::plan::SimulationPlan,
    dependent: AnalysisInstanceId,
) -> Option<AnalysisInstanceId> {
    let dependent_index = plan
        .instances()
        .iter()
        .position(|instance| instance.id() == dependent)?;
    plan.instances()[..dependent_index]
        .iter()
        .rev()
        .find(|candidate| matches!(candidate.draft(), AnalysisDraft::Pss(pss) if pss.osc_mode))
        .map(|candidate| candidate.id())
}

pub(super) fn contextual_dependency_error(
    plan: &crate::simulation::plan::SimulationPlan,
    root: AnalysisInstanceId,
    closure_ids: &HashSet<AnalysisInstanceId>,
    dependency_sources: &EnvelopeSourceCatalog,
) -> Option<String> {
    plan.instances()
        .iter()
        .filter(|instance| instance.id() != root && closure_ids.contains(&instance.id()))
        .find_map(|instance| {
            let AnalysisDraft::Pss(draft) = instance.draft() else {
                return None;
            };
            dependency_sources
                .dependency_repair_context()
                .validate_pss_sources(draft)
                .err()
                .map(|error| {
                    format!(
                        "{} does not match the circuit: {error}",
                        analysis_title(plan, instance.id())
                    )
                })
        })
}

pub fn apply_analysis_action(
    cx: &mut Cx<'_, SimulateFeature>,
    id: AnalysisInstanceId,
    action: AnalysisAction,
) {
    match action {
        AnalysisAction::Clone => {
            let step = cx.model.setup_snapshot(SetupParts::ANALYSES);
            let result = match cx.model.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan.clone_instance(id).map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            cx.model
                .record_setup_change(SimulationPage::Analyses.label(), step);
            match result {
                Ok((clone_id, receipt)) => {
                    refresh_analysis_projections(cx);
                    cx.state.active_analysis_instance = Some(clone_id);
                    record_receipt(cx, &receipt);
                }
                Err(error) => record_failure(cx, "Clone", &error),
            }
        }
        AnalysisAction::Earlier(position) | AnalysisAction::Later(position) => {
            let step = cx.model.setup_snapshot(SetupParts::ANALYSES);
            let result = match cx.model.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan
                    .reorder(id, position)
                    .map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            cx.model
                .record_setup_change(SimulationPage::Analyses.label(), step);
            match result {
                Ok(receipt) => {
                    refresh_analysis_projections(cx);
                    record_receipt(cx, &receipt);
                }
                Err(error) => record_failure(cx, "Reorder", &error),
            }
        }
        AnalysisAction::BindDependencies => {
            let repair_context =
                build_envelope_source_catalog(cx.model).dependency_repair_context();
            let step = cx.model.setup_snapshot(SetupParts::ANALYSES);
            let result = match cx.model.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan
                    .auto_bind_dependencies_with_context(id, &repair_context)
                    .map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            cx.model
                .record_setup_change(SimulationPage::Analyses.label(), step);
            match result {
                Ok(receipt) => {
                    refresh_analysis_projections(cx);
                    record_receipt(cx, &receipt);
                }
                Err(error) => record_failure(cx, "Bind dependencies", &error),
            }
        }
        AnalysisAction::BindDependency {
            prerequisite,
            target,
        } => {
            let step = cx.model.setup_snapshot(SetupParts::ANALYSES);
            let result = match cx.model.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan
                    .bind_dependency(id, prerequisite, target)
                    .map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            cx.model
                .record_setup_change(SimulationPage::Analyses.label(), step);
            match result {
                Ok(receipt) => {
                    refresh_analysis_projections(cx);
                    record_receipt(cx, &receipt);
                }
                Err(error) => record_failure(cx, "Bind prerequisite", &error),
            }
        }
        AnalysisAction::StartFromOwnSolve { prerequisite } => {
            let step = cx.model.setup_snapshot(SetupParts::ANALYSES);
            let result = match cx.model.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan
                    .unbind_optional_dependency(id, prerequisite)
                    .map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            cx.model
                .record_setup_change(SimulationPage::Analyses.label(), step);
            match result {
                Ok(()) => refresh_analysis_projections(cx),
                Err(error) => record_failure(cx, "Start from its own DC solve", &error),
            }
        }
        AnalysisAction::RepairDependencies => {
            let repair_context =
                build_envelope_source_catalog(cx.model).dependency_repair_context();
            let step = cx.model.setup_snapshot(SetupParts::ANALYSES);
            let result = match cx.model.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan
                    .repair_dependencies_with_context(id, &repair_context)
                    .map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            cx.model
                .record_setup_change(SimulationPage::Analyses.label(), step);
            match result {
                Ok(_) => {
                    refresh_analysis_projections(cx);
                    let line = cx
                        .model
                        .sim_setup
                        .stable_analysis_plan()
                        .map(|plan| now_uses_line(plan, id))
                        .unwrap_or_default();
                    cx.model.record_plan_receipt(line);
                }
                Err(error) => record_failure(cx, "Repair prerequisites", &error),
            }
        }
        AnalysisAction::PrepareAutonomousPss => {
            let existing = cx
                .model
                .sim_setup
                .stable_analysis_plan()
                .ok()
                .and_then(|plan| prepared_autonomous_pss_id(plan, id));
            if let Some(pss_id) = existing {
                cx.state.active_analysis_instance = Some(pss_id);
                let line = cx
                    .model
                    .sim_setup
                    .stable_analysis_plan()
                    .map(|plan| {
                        format!(
                            "Opened {}, the autonomous PSS {} can use. Enter its oscillator node and enable it.",
                            analysis_title(plan, pss_id),
                            analysis_title(plan, id)
                        )
                    })
                    .unwrap_or_default();
                cx.model.record_plan_receipt(line);
                return;
            }
            let pss = PssDialogState {
                osc_mode: true,
                osc_node: String::new(),
                tone_sources: String::new(),
                ..Default::default()
            };
            let repair_context =
                build_envelope_source_catalog(cx.model).dependency_repair_context();
            let step = cx.model.setup_snapshot(SetupParts::ANALYSES);
            let result = match cx.model.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan
                    .prepare_prerequisite_for_configuration(
                        id,
                        AnalysisKind::Pss,
                        AnalysisDraft::Pss(pss),
                        &repair_context,
                    )
                    .map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            cx.model
                .record_setup_change(SimulationPage::Analyses.label(), step);
            match result {
                Ok((pss_id, _)) => {
                    refresh_analysis_projections(cx);
                    cx.state.active_analysis_instance = Some(pss_id);
                    let line = cx
                        .model
                        .sim_setup
                        .stable_analysis_plan()
                        .map(|plan| {
                            format!(
                                "Added an autonomous PSS ahead of {}. Enter its oscillator node and enable it, then choose it in the Uses row of {}.",
                                analysis_title(plan, id),
                                analysis_title(plan, id)
                            )
                        })
                        .unwrap_or_default();
                    cx.model.record_plan_receipt(line);
                }
                Err(error) => record_failure(cx, "Configure PSS prerequisite", &error),
            }
        }
        AnalysisAction::Validate => validate_analysis_instance(cx, id),
        AnalysisAction::Remove => remove_analysis_instance(cx, id),
        AnalysisAction::SetEnabled(enabled) => {
            let step = cx.model.setup_snapshot(SetupParts::ANALYSES);
            let result = match cx.model.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan
                    .set_enabled(id, enabled)
                    .map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            cx.model
                .record_setup_change(SimulationPage::Analyses.label(), step);
            match result {
                Ok(receipt) => {
                    refresh_analysis_projections(cx);
                    record_receipt(cx, &receipt);
                }
                Err(error) => record_failure(cx, "Enable state", &error),
            }
        }
    }
}

pub(super) fn validate_analysis_instance(cx: &mut Cx<'_, SimulateFeature>, id: AnalysisInstanceId) {
    let envelope_sources = build_envelope_source_catalog(cx.model);
    // What only a damaged file produces is stated plainly on the form and in
    // full, identities included, in the log.
    let mut damage = None;
    let result = (|| {
        let plan = cx.model.sim_setup.stable_analysis_plan()?;
        if plan.instance(id).is_none() {
            return Err("The analysis is no longer in this test.".to_owned());
        }
        let closure_ids = dependency_closure_ids(plan, id);
        if let Some(issue) = plan.validation_issues().iter().find(|issue| {
            closure_ids
                .iter()
                .any(|closure_id| issue_applies_to(issue, *closure_id))
        }) {
            if issue_is_damage(issue) {
                damage = Some(issue.to_string());
            }
            return Err(format_plan_issue(plan, issue));
        }
        for closure_instance in plan
            .instances()
            .iter()
            .filter(|candidate| closure_ids.contains(&candidate.id()))
        {
            if let Some(error) = analysis_validation_error(
                cx.model,
                closure_instance.draft(),
                Some(&envelope_sources),
            ) {
                return Err(format!(
                    "{}: {error}",
                    analysis_title(plan, closure_instance.id())
                ));
            }
        }
        Ok(analysis_title(plan, id))
    })();

    if let Some(detail) = damage {
        cx.model.record(
            crate::diagnostics::LogSeverity::Warning,
            crate::diagnostics::LogSource::Plan,
            detail,
        );
    }
    match result {
        Ok(title) => cx
            .model
            .record_plan_receipt(format!("{title} is ready to run.")),
        Err(error) => record_plan_refusal(cx, error),
    }
}

/// Retained runs holding a result attributed to this analysis.
fn retained_runs_for_analysis(model: &Model, id: AnalysisInstanceId) -> usize {
    model
        .simulation
        .retained
        .runs
        .iter()
        .filter(|run| run.find_analysis_by_source_instance(id).is_some())
        .count()
}

/// The analyses bound to this one as a prerequisite, in plan order.
///
/// Both halves are carried. The name is what a reader knows the analysis by —
/// a notice listing "Transient, Transient" tells them nothing about what is in
/// the way — and the id is what a hop to it needs.
fn analyses_depending_on(
    model: &Model,
    id: AnalysisInstanceId,
) -> Vec<(AnalysisInstanceId, String)> {
    model
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| {
            plan.instances()
                .iter()
                .filter(|instance| instance.id() != id)
                .filter(|instance| {
                    instance
                        .dependencies()
                        .iter()
                        .any(|dependency| dependency.target() == id)
                })
                .map(|instance| (instance.id(), instance.display_name().to_owned()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

/// What this analysis shows under, or a phrase that stands in for it.
fn analysis_display_name(model: &Model, id: AnalysisInstanceId) -> String {
    model
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| {
            plan.instances()
                .iter()
                .find(|instance| instance.id() == id)
                .map(|instance| instance.display_name().to_owned())
        })
        .unwrap_or_else(|| "this analysis".to_owned())
}

/// Remove one analysis: refuse it, review it, or do it.
///
/// Three outcomes, decided before anything opens.
///
/// An analysis another one is bound to is *refused*. The plan's removal
/// transaction refuses that case on exactly this predicate, so the destructive
/// review this used to stage was a confirmation with only one possible ending:
/// the reader authorised the removal, nothing was removed, and the notice they
/// got afterwards named the blocking analyses by instance id. A refusal is not
/// a question, so it is not asked as one.
///
/// An analysis with retained results and nothing bound to it is *reviewed*.
/// That decision is genuinely the reader's: the removal will succeed, and what
/// it costs is the attribution of results that stay in the project either way.
///
/// Anything else is removed at once, because a confirmation with nothing behind
/// it only teaches the reader to click through the ones that matter.
pub fn remove_analysis_instance(cx: &mut Cx<'_, SimulateFeature>, id: AnalysisInstanceId) {
    let dependents = analyses_depending_on(cx.model, id);
    if !dependents.is_empty() {
        let label = analysis_display_name(cx.model, id);
        cx.state.plan_removal_refusal.open(id, label, dependents);
        return;
    }

    let retained_runs = retained_runs_for_analysis(cx.model, id);
    if retained_runs == 0 {
        commit_analysis_removal(cx, id);
        return;
    }

    let label = analysis_display_name(cx.model, id);
    // Rows in the order the review states them; each paragraph is carried by
    // the row it explains, and the dialog puts the warnings before the asides.
    let retained = PlanRemovalConsequence::stated(
        "Retained runs holding its results",
        retained_runs.to_string(),
    )
    .explained(PlanRemovalTone::Aside, "Results of earlier runs are kept.");
    // Always "none" — a dependent sends the request to the refusal above and
    // never reaches this review. It is still stated, because a reader deciding
    // what retained results cost needs to know that nothing else is at stake.
    let dependents = PlanRemovalConsequence::stated("Analyses bound to it", "none");
    cx.state.plan_removal_review.open(
        PlanRemovalTarget::Analysis(id),
        label,
        vec![retained, dependents],
    );
}

/// Apply a removal the reader confirmed in the destructive review.
///
/// Called once per frame from the surface, so the modal records an answer
/// rather than reaching into the plan itself.
pub fn apply_confirmed_analysis_removal(cx: &mut Cx<'_, SimulateFeature>) {
    if let Some(id) = cx.state.plan_removal_review.take_confirmed_analysis() {
        commit_analysis_removal(cx, id);
    }
}

/// Show the analysis a refused removal named as the one in the way.
///
/// The same handshake the confirmed removal uses, for the same reason: the
/// notice records what the reader asked for and the surface that owns the plan
/// is what acts on it, so no modal navigates another surface itself.
///
/// Takes the state rather than the application. Answering a notice moves what
/// is on screen and touches nothing else, and a handler holding the whole
/// application could mutate every subsystem to do it.
pub fn apply_requested_blocker_reveal(state: &mut SimulateState) {
    if let Some(blocker) = state.plan_removal_refusal.take_reveal_target() {
        super::advanced_options::reveal_in_analyses(state, blocker);
    }
}

/// Perform the removal itself, after any review has been answered.
fn commit_analysis_removal(cx: &mut Cx<'_, SimulateFeature>, id: AnalysisInstanceId) {
    let prior_run_ids = cx
        .model
        .simulation
        .retained
        .runs
        .iter()
        .filter(|run| run.find_analysis_by_source_instance(id).is_some())
        .map(|run| run.run_id)
        .collect();
    let position = cx
        .model
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| {
            plan.instances()
                .iter()
                .position(|instance| instance.id() == id)
        })
        .unwrap_or(0);
    // The typed refusal is carried out of the borrow rather than stringified
    // inside it: the plan speaks in instance ids and is right to, and the names
    // that make a refusal readable are only resolvable out here, against the
    // plan a refused transaction left standing.
    let step = cx.model.setup_snapshot(SetupParts::ANALYSES);
    let result = match cx.model.sim_setup.stable_analysis_plan_mut() {
        Ok(plan) => plan
            .remove(id, prior_run_ids)
            .map_err(RemovalFailure::Refused),
        Err(error) => Err(RemovalFailure::Unavailable(error)),
    };
    cx.model
        .record_setup_change(SimulationPage::Analyses.label(), step);
    match result {
        Ok(receipt) => {
            refresh_analysis_projections(cx);
            let next = cx
                .model
                .sim_setup
                .stable_analysis_plan()
                .ok()
                .and_then(|plan| {
                    let next_position = position.min(plan.instances().len().saturating_sub(1));
                    plan.instances()
                        .get(next_position)
                        .map(|instance| instance.id())
                });
            cx.state.active_analysis_instance = next;
            record_receipt(cx, &receipt);
        }
        Err(RemovalFailure::Unavailable(error)) => {
            record_failure(cx, "Remove", &error);
        }
        Err(RemovalFailure::Refused(error)) => {
            let message = removal_refusal_naming_instances(cx.model, &error);
            record_failure(cx, "Remove", &message);
        }
    }
}

/// Why a removal did not commit.
///
/// Two different things, and the difference is what the reader is told. A
/// refusal is the plan holding its own rule and can be restated in the names
/// the reader knows; an unreachable plan is already a sentence and has no
/// instances to name.
enum RemovalFailure {
    Refused(AnalysisPlanError),
    Unavailable(String),
}

/// Refresh the plan's legacy projections and drop what preflight authorised,
/// after any edit to the analysis stack.
pub(super) fn refresh_analysis_projections(cx: &mut Cx<'_, SimulateFeature>) {
    cx.model.sim_setup.refresh_legacy_analysis_projections();
    crate::workbench::simulate::invalidate_preflight(cx);
}

pub(super) const fn availability_label(kind: AnalysisKind) -> &'static str {
    kind.availability().label()
}

pub const fn analysis_catalog_group(kind: AnalysisKind) -> &'static str {
    kind.category().label
}

pub(super) fn issue_applies_to(issue: &AnalysisPlanIssue, id: AnalysisInstanceId) -> bool {
    match issue {
        AnalysisPlanIssue::NoEnabledInstances => true,
        AnalysisPlanIssue::DuplicateInstanceId { id: issue_id }
        | AnalysisPlanIssue::ReusedTombstonedId { id: issue_id }
        | AnalysisPlanIssue::KindDraftMismatch { id: issue_id, .. }
        | AnalysisPlanIssue::InvalidInstanceRevision { id: issue_id }
        | AnalysisPlanIssue::InvalidInstanceName { id: issue_id }
        | AnalysisPlanIssue::DuplicateInstanceName { id: issue_id, .. }
        | AnalysisPlanIssue::InvalidLifecycle { id: issue_id, .. } => *issue_id == id,
        AnalysisPlanIssue::MissingPrerequisite { dependent, .. }
        | AnalysisPlanIssue::UnexpectedDependencyRole { dependent, .. }
        | AnalysisPlanIssue::DuplicateDependencyRole { dependent, .. }
        | AnalysisPlanIssue::SelfDependency { dependent }
        | AnalysisPlanIssue::DanglingDependency { dependent, .. }
        | AnalysisPlanIssue::WrongDependencyKind { dependent, .. }
        | AnalysisPlanIssue::DisabledDependency { dependent, .. }
        | AnalysisPlanIssue::DependencyNotEarlier { dependent, .. }
        | AnalysisPlanIssue::IncompatibleDependencyConfiguration { dependent, .. } => {
            *dependent == id
        }
        AnalysisPlanIssue::DependencyCycle { members } => members.contains(&id),
        AnalysisPlanIssue::DuplicateTombstoneId { .. }
        | AnalysisPlanIssue::InvalidTombstoneRevision { .. } => false,
    }
}

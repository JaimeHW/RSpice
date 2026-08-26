//! Mutating the analysis stack: insert, commit, validate, remove, repair.
//!
//! Every mutation goes through this module and leaves a lifecycle receipt, so
//! what the surface shows about an analysis is always what actually happened
//! to it. A draft is validated before it is committed and a failed action
//! records the failure rather than the intent, which is why a stale or
//! rejected edit can never appear as a configured analysis.

use super::*;

pub(super) fn resolve_active_analysis_instance(app: &mut AppState) -> Result<(), String> {
    let current = app.workbench.active_analysis_instance;
    let legacy_index = app.workbench.active_analysis;
    let resolved = {
        let plan = app.sim_setup.stable_analysis_plan()?;
        match current {
            Some(id) if plan.instance(id).is_some() => Some(id),
            Some(_) => plan.instances().first().map(|instance| instance.id()),
            None => AnalysisKind::from_legacy_index(legacy_index)
                .and_then(|kind| {
                    plan.instances()
                        .iter()
                        .find(|instance| instance.kind() == kind)
                })
                .or_else(|| plan.instances().first())
                .map(|instance| instance.id()),
        }
    };
    app.workbench.active_analysis_instance = resolved;
    Ok(())
}

pub(super) fn selected_analysis(
    app: &RSpiceApp,
    dependency_sources: &EnvelopeSourceCatalog,
) -> Result<Option<SelectedAnalysis>, String> {
    let Some(id) = app.state.workbench.active_analysis_instance else {
        return Ok(None);
    };
    let plan = app.state.sim_setup.stable_analysis_plan()?;
    let Some((position, instance)) = plan
        .instances()
        .iter()
        .enumerate()
        .find(|(_, instance)| instance.id() == id)
    else {
        return Err(format!(
            "selected analysis instance {id} is not present in the stable plan"
        ));
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
    let prerequisite_roles = instance.prerequisite_roles().to_vec();
    let prerequisite_candidates = prerequisite_roles
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
                .map(|(candidate_position, candidate)| DependencyCandidate {
                    id: candidate.id(),
                    // Position, kind, then what it is called. Two candidates
                    // of one kind were previously told apart only by the
                    // UUID they were labelled with.
                    label: format!(
                        "{:02} · {} · {}",
                        candidate_position + 1,
                        candidate.kind().stable_id().to_uppercase(),
                        candidate.display_name()
                    ),
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
    Ok(Some(SelectedAnalysis {
        id,
        kind: instance.kind(),
        name: instance.display_name().to_owned(),
        draft: instance.draft().clone(),
        dependencies: instance.dependencies().to_vec(),
        prerequisite_roles,
        prerequisite_candidates,
        lifecycle: instance.lifecycle(),
        position,
        plan_length: plan.instances().len(),
        issues,
        contextual_dependency_error,
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
                        "PSS prerequisite instance {} does not match the active circuit: {error}",
                        instance.id()
                    )
                })
        })
}

pub(super) fn commit_draft(app: &mut RSpiceApp, id: AnalysisInstanceId, draft: AnalysisDraft) {
    let result = match app.state.sim_setup.stable_analysis_plan_mut() {
        Ok(plan) => plan
            .edit(id, |target| target.clone_from(&draft))
            .map_err(|error| error.to_string()),
        Err(error) => Err(error),
    };
    match result {
        Ok(((), receipt)) => {
            refresh_analysis_projections(app);
            record_receipt(&mut app.state, &receipt);
        }
        Err(error) => record_failure(&mut app.state, "Edit", &error),
    }
}

/// Commit one analysis's departures from the plan's solver policy.
///
/// Deliberately the same shape as [`commit_draft`]: a numerical override is a
/// plan mutation like any other, so it produces a receipt, refreshes the
/// projections, and invalidates preflight through the one path every editor
/// uses. A refusal leaves the plan untouched and is reported verbatim, because
/// the reason names which owner already holds the value.
pub(super) fn commit_numeric_override(
    app: &mut RSpiceApp,
    id: AnalysisInstanceId,
    record: Option<crate::simulation::plan::AnalysisNumericOverride>,
) -> Result<(), String> {
    let result = match app.state.sim_setup.stable_analysis_plan_mut() {
        Ok(plan) => plan
            .set_numeric_override(id, record)
            .map_err(|error| error.to_string()),
        Err(error) => Err(error),
    };
    match result {
        Ok(receipt) => {
            refresh_analysis_projections(app);
            record_receipt(&mut app.state, &receipt);
            Ok(())
        }
        Err(error) => {
            record_failure(&mut app.state, "Edit", &error);
            Err(error)
        }
    }
}

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
/// `None` is every refusal, each of which has already stated itself: a blocked
/// kind, a plan that does not resolve, an insert the plan rejected.
pub(super) fn insert_analysis_instance(
    app: &mut RSpiceApp,
    kind: AnalysisKind,
) -> Option<AnalysisInstanceId> {
    app.state.sim_setup.palette_open = false;
    app.state.sim_setup.palette_query.clear();
    app.state.sim_setup.palette_active = 0;
    app.state.sim_setup.palette_scroll_to_active = false;
    if let Some(reason) = kind.execution_blocker() {
        record_failure(&mut app.state, "Insert", reason);
        return None;
    }
    let repair_context = build_envelope_source_catalog(&app.state).dependency_repair_context();
    let result: InsertAnalysisResult = match app.state.sim_setup.stable_analysis_plan_mut() {
        Ok(plan) => match plan.insert(kind) {
            Ok((id, insert_receipt)) => {
                let has_prerequisites = plan
                    .instance(id)
                    .is_some_and(|instance| !instance.prerequisite_roles().is_empty());
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
    match result {
        Ok((id, insert_receipt, bind_receipt)) => {
            refresh_analysis_projections(app);
            app.state.workbench.active_analysis_instance = Some(id);
            match bind_receipt {
                None => record_receipt(&mut app.state, &insert_receipt),
                Some(Ok(bind_receipt)) => {
                    let issue = app
                        .state
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
                    app.state.workbench.analysis_lifecycle_status.record_receipt(format!(
                        "Receipts #{} and #{} committed for instance {id}. {} {} {readiness} Prior datasets remain immutable.",
                        insert_receipt.sequence(),
                        bind_receipt.sequence(),
                        insert_receipt.detail(),
                        bind_receipt.detail(),
                    ));
                }
                Some(Err(error)) => {
                    app.state.workbench.analysis_lifecycle_status.record_refusal(format!(
                        "Receipt #{} committed for instance {id}. {} Automatic dependency binding was rejected fail-closed: {error}. The inserted instance remains selected and preflight blocked; prior datasets remain immutable.",
                        insert_receipt.sequence(),
                        insert_receipt.detail(),
                    ));
                }
            }
            Some(id)
        }
        Err(error) => {
            record_failure(&mut app.state, "Add analysis", &error);
            None
        }
    }
}

pub(super) fn apply_analysis_action(
    app: &mut RSpiceApp,
    id: AnalysisInstanceId,
    action: AnalysisAction,
) {
    match action {
        AnalysisAction::Clone => {
            let result = match app.state.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan.clone_instance(id).map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            match result {
                Ok((clone_id, receipt)) => {
                    refresh_analysis_projections(app);
                    app.state.workbench.active_analysis_instance = Some(clone_id);
                    record_receipt(&mut app.state, &receipt);
                }
                Err(error) => record_failure(&mut app.state, "Clone", &error),
            }
        }
        AnalysisAction::Earlier(position) | AnalysisAction::Later(position) => {
            let result = match app.state.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan
                    .reorder(id, position)
                    .map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            match result {
                Ok(receipt) => {
                    refresh_analysis_projections(app);
                    record_receipt(&mut app.state, &receipt);
                }
                Err(error) => record_failure(&mut app.state, "Reorder", &error),
            }
        }
        AnalysisAction::BindDependencies => {
            let repair_context =
                build_envelope_source_catalog(&app.state).dependency_repair_context();
            let result = match app.state.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan
                    .auto_bind_dependencies_with_context(id, &repair_context)
                    .map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            match result {
                Ok(receipt) => {
                    refresh_analysis_projections(app);
                    record_receipt(&mut app.state, &receipt);
                }
                Err(error) => record_failure(&mut app.state, "Bind dependencies", &error),
            }
        }
        AnalysisAction::BindDependency {
            prerequisite,
            target,
        } => {
            let result = match app.state.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan
                    .bind_dependency(id, prerequisite, target)
                    .map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            match result {
                Ok(receipt) => {
                    refresh_analysis_projections(app);
                    record_receipt(&mut app.state, &receipt);
                }
                Err(error) => record_failure(&mut app.state, "Bind prerequisite", &error),
            }
        }
        AnalysisAction::RepairDependencies => {
            let repair_context =
                build_envelope_source_catalog(&app.state).dependency_repair_context();
            let result = match app.state.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan
                    .repair_dependencies_with_context(id, &repair_context)
                    .map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            match result {
                Ok((repair, receipt)) => {
                    refresh_analysis_projections(app);
                    app.state.workbench.analysis_lifecycle_status.record_receipt(format!(
                        "Receipt #{} committed for instance {id}. Prerequisite repair completed atomically: {} added, {} enabled, {} moved earlier, {} exact bindings updated, and {} invalid bindings removed. Prior datasets remain immutable.",
                        receipt.sequence(),
                        repair.inserted().len(),
                        repair.enabled().len(),
                        repair.moved().len(),
                        repair.bound().len(),
                        repair.removed().len(),
                    ));
                }
                Err(error) => record_failure(&mut app.state, "Repair prerequisites", &error),
            }
        }
        AnalysisAction::PrepareAutonomousPss => {
            let existing = app
                .state
                .sim_setup
                .stable_analysis_plan()
                .ok()
                .and_then(|plan| prepared_autonomous_pss_id(plan, id));
            if let Some(pss_id) = existing {
                app.state.workbench.active_analysis_instance = Some(pss_id);
                app.state.workbench.analysis_lifecycle_status.record_receipt(format!(
                    "Existing autonomous PSS prerequisite {pss_id} was selected for phase-noise analysis {id}. Complete its oscillator node and remaining controls, enable it, then run dependency repair to bind the exact prerequisite without creating a duplicate instance."
                ));
                return;
            }
            let pss = PssDialogState {
                osc_mode: true,
                osc_node: String::new(),
                tone_sources: String::new(),
                ..Default::default()
            };
            let repair_context =
                build_envelope_source_catalog(&app.state).dependency_repair_context();
            let result = match app.state.sim_setup.stable_analysis_plan_mut() {
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
            match result {
                Ok((pss_id, receipt)) => {
                    refresh_analysis_projections(app);
                    app.state.workbench.active_analysis_instance = Some(pss_id);
                    app.state.workbench.analysis_lifecycle_status.record_receipt(format!(
                        "Receipt #{} committed. Autonomous PSS prerequisite {pss_id} and its inferable dependency chain are prepared before phase-noise analysis {id}. Enter the exact oscillator node, review the remaining PSS controls, then enable it; dependency repair will reuse and bind it while preflight remains fail-closed until configuration is complete.",
                        receipt.sequence(),
                    ));
                }
                Err(error) => record_failure(&mut app.state, "Configure PSS prerequisite", &error),
            }
        }
        AnalysisAction::Validate => validate_analysis_instance(&mut app.state, id),
        AnalysisAction::Remove => remove_analysis_instance(app, id),
        AnalysisAction::SetEnabled(enabled) => {
            let result = match app.state.sim_setup.stable_analysis_plan_mut() {
                Ok(plan) => plan
                    .set_enabled(id, enabled)
                    .map_err(|error| error.to_string()),
                Err(error) => Err(error),
            };
            match result {
                Ok(receipt) => {
                    refresh_analysis_projections(app);
                    record_receipt(&mut app.state, &receipt);
                }
                Err(error) => record_failure(&mut app.state, "Enable state", &error),
            }
        }
    }
}

pub(super) fn validate_analysis_instance(app: &mut AppState, id: AnalysisInstanceId) {
    let envelope_sources = build_envelope_source_catalog(app);
    let result = (|| {
        let plan = app.sim_setup.stable_analysis_plan()?;
        let instance = plan
            .instance(id)
            .ok_or_else(|| format!("analysis instance {id} no longer exists"))?;
        let closure_ids = dependency_closure_ids(plan, id);
        if let Some(issue) = plan.validation_issues().iter().find(|issue| {
            closure_ids
                .iter()
                .any(|closure_id| issue_applies_to(issue, *closure_id))
        }) {
            return Err(format_plan_issue(issue));
        }
        for closure_instance in plan
            .instances()
            .iter()
            .filter(|candidate| closure_ids.contains(&candidate.id()))
        {
            if let Some(error) =
                analysis_validation_error(app, closure_instance.draft(), Some(&envelope_sources))
            {
                return Err(format!(
                    "{} ({}): {error}",
                    closure_instance.display_name(),
                    closure_instance.id()
                ));
            }
        }
        Ok(instance.display_name().to_owned())
    })();

    match result {
        Ok(name) => {
            app.workbench.analysis_lifecycle_status.record_receipt(format!(
                "Validation passed for {name} ({id}). Dependency identity, order, and enabled state are valid for this instance."
            ));
        }
        Err(error) => record_failure(app, "Validate", &error),
    }
}

/// Retained runs holding a result attributed to this analysis.
fn retained_runs_for_analysis(app: &RSpiceApp, id: AnalysisInstanceId) -> usize {
    app.state
        .simulation
        .runs
        .iter()
        .filter(|run| run.find_analysis_by_source_instance(id).is_some())
        .count()
}

/// Names of the analyses bound to this one as a prerequisite.
///
/// By name, because a removal review listing "Transient, Transient" tells the
/// reader nothing about what they are about to break.
fn analyses_depending_on(app: &RSpiceApp, id: AnalysisInstanceId) -> Vec<String> {
    app.state
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
                .map(|instance| instance.display_name().to_owned())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

/// Remove one analysis, stopping first if removal costs something.
///
/// The check is deliberately narrow: retained results attributed to this
/// analysis, or other analyses bound to it. Anything else is removed at once,
/// because a confirmation with nothing behind it only teaches the reader to
/// click through the ones that matter.
pub(super) fn remove_analysis_instance(app: &mut RSpiceApp, id: AnalysisInstanceId) {
    let retained_runs = retained_runs_for_analysis(app, id);
    let dependent_analyses = analyses_depending_on(app, id);
    if retained_runs == 0 && dependent_analyses.is_empty() {
        commit_analysis_removal(app, id);
        return;
    }

    let label = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| {
            plan.instances()
                .iter()
                .find(|instance| instance.id() == id)
                .map(|instance| instance.display_name().to_owned())
        })
        .unwrap_or_else(|| "this analysis".to_owned());
    app.state
        .dialogs
        .analysis_removal_review
        .open(id, label, retained_runs, dependent_analyses);
}

/// Apply a removal the reader confirmed in the destructive review.
///
/// Called once per frame from the surface, so the modal records an answer
/// rather than reaching into the plan itself.
pub(super) fn apply_confirmed_analysis_removal(app: &mut RSpiceApp) {
    if let Some(id) = app.state.dialogs.analysis_removal_review.take_confirmed() {
        commit_analysis_removal(app, id);
    }
}

/// Perform the removal itself, after any review has been answered.
fn commit_analysis_removal(app: &mut RSpiceApp, id: AnalysisInstanceId) {
    let prior_run_ids = app
        .state
        .simulation
        .runs
        .iter()
        .filter(|run| run.find_analysis_by_source_instance(id).is_some())
        .map(|run| run.run_id)
        .collect();
    let position = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .and_then(|plan| {
            plan.instances()
                .iter()
                .position(|instance| instance.id() == id)
        })
        .unwrap_or(0);
    let result = match app.state.sim_setup.stable_analysis_plan_mut() {
        Ok(plan) => plan
            .remove(id, prior_run_ids)
            .map_err(|error| error.to_string()),
        Err(error) => Err(error),
    };
    match result {
        Ok(receipt) => {
            refresh_analysis_projections(app);
            let next = app
                .state
                .sim_setup
                .stable_analysis_plan()
                .ok()
                .and_then(|plan| {
                    let next_position = position.min(plan.instances().len().saturating_sub(1));
                    plan.instances()
                        .get(next_position)
                        .map(|instance| instance.id())
                });
            app.state.workbench.active_analysis_instance = next;
            record_receipt(&mut app.state, &receipt);
        }
        Err(error) => record_failure(&mut app.state, "Remove", &error),
    }
}

pub(super) fn refresh_analysis_projections(app: &mut RSpiceApp) {
    app.state.sim_setup.refresh_legacy_analysis_projections();
    app.invalidate_simulation_preflight();
}

/// Both of these write one field — the lifecycle status strip — so they take
/// it rather than the application. A handler that took the whole app could
/// mutate every subsystem to announce an outcome.
pub(super) fn record_receipt(state: &mut AppState, receipt: &AnalysisLifecycleReceipt) {
    let related = receipt
        .related_instance_id()
        .map_or_else(String::new, |id| format!(" · related instance {id}"));
    state.workbench.analysis_lifecycle_status.record_receipt(format!(
        "Receipt #{} · {} committed for instance {}{related} · revision {} to {} · outcome {}. {} Prior datasets remain immutable.",
        receipt.sequence(),
        lifecycle_command_label(receipt.command()),
        receipt.instance_id(),
        receipt.source_revision().get(),
        receipt.committed_revision().get(),
        receipt.outcome(),
        receipt.detail(),
    ));
}

pub(super) fn record_failure(state: &mut AppState, action: &str, error: &str) {
    state.workbench.analysis_lifecycle_status.record_refusal(format!(
        "{action} rejected fail-closed: {error}. The stable plan is unchanged and prior datasets remain immutable."
    ));
}

pub(super) const fn lifecycle_command_label(command: AnalysisLifecycleCommand) -> &'static str {
    match command {
        AnalysisLifecycleCommand::Insert => "Insert",
        AnalysisLifecycleCommand::Edit => "Edit",
        AnalysisLifecycleCommand::Rename => "Rename",
        AnalysisLifecycleCommand::Clone => "Clone",
        AnalysisLifecycleCommand::Enable => "Enable",
        AnalysisLifecycleCommand::Disable => "Disable",
        AnalysisLifecycleCommand::Reorder => "Reorder",
        AnalysisLifecycleCommand::Dependency => "Bind dependencies",
        AnalysisLifecycleCommand::Validate => "Validate",
        AnalysisLifecycleCommand::Preflight => "Preflight",
        AnalysisLifecycleCommand::Execute => "Execute",
        AnalysisLifecycleCommand::Remove => "Remove",
    }
}

pub(super) const fn availability_label(kind: AnalysisKind) -> &'static str {
    kind.availability().label()
}

pub(super) const fn analysis_catalog_group(kind: AnalysisKind) -> &'static str {
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
        | AnalysisPlanIssue::InvalidTombstoneRevision { .. }
        | AnalysisPlanIssue::InvalidReceiptSequence { .. }
        | AnalysisPlanIssue::InvalidReceiptRevision { .. }
        | AnalysisPlanIssue::DanglingReceiptInstance { .. }
        | AnalysisPlanIssue::ReceiptKindMismatch { .. }
        | AnalysisPlanIssue::EmptyReceiptDetail { .. }
        | AnalysisPlanIssue::InvalidNextReceiptSequence { .. } => false,
    }
}

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

pub(super) fn dependency_repair_cta_with_context(
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
        plan.instance(dependent).is_some_and(|instance| {
            instance.prerequisite_roles().iter().all(|prerequisite| {
                plan.dependency_prerequisite_is_repairable_with_context(
                    dependent,
                    *prerequisite,
                    repair_context,
                )
            })
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

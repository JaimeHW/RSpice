//! Revision-bound simulation-preflight workflow.
//!
//! Validation is executed once and captured as an immutable report for the
//! exact project, topology and plan revision it inspected. The report is the
//! authority a run is queued against; nothing here draws it.
//!
//! It used to draw it, in a modal over the design. That meant the one list of
//! reasons a run would not start was readable only where the schematic was
//! not, and acting on any of them meant dismissing the list first. The
//! findings are now records in the canonical diagnostic store, where they sit
//! beside every other condition that stands against the design, and where
//! each one's own destination is what following the row does. What this module
//! still owns is the pass itself: run the checks, prepare the immutable
//! snapshot, and queue only what that snapshot authorizes.
//!
//! The local checks run in the frame that takes the press. What follows them
//! does not: the run is prepared from inputs frozen at the press, and then
//! the engine builds every deck it dispatches, without solving, and what it
//! refuses there blocks the run. Both run on one worker; the pass finishes
//! when the frame loop finds its verdict, and an edit made meanwhile leaves
//! the verdict stale rather than racing it.

mod engine_refusals;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::simulation::SimulationController;
use crate::simulation::controller::{PreflightOutcome, PreflightVerdict};
use crate::simulation::execution::PreparationError;
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::design_checks::CheckScope;

use super::state::{
    ConsolePage, ModelsPage, PreflightAdvisory, PreflightCheck, PreflightIssue,
    PreflightRemediation, PreflightReport, PreparedPreflightContract, Workspace,
};

/// Run every local preflight check and the engine's build pass, retain the
/// exact report for the current project revision, and publish what it found.
///
/// The design rules are decided from Check Design's receipt for the
/// configured design, which is run first only where none describes the
/// design as it stands; its findings are the ones the page lists. The report
/// carries one blocker that stands for them when any counts as an error. The
/// open view is not checked: it is not what the run executes.
///
/// A blocked pass leaves the reader on the Problems page with the blockers
/// listed; a clean one leaves them where they were. Either way the outcome is
/// a Console line the notice centre lifts into a notice, which is where the
/// reader is told of a finished job. It used to be a plan receipt, which no
/// chrome draws and which the notice centre deliberately never lifts, so a
/// clean pass with the Console collapsed said nothing anywhere.
///
/// When the checks before it pass, the pass ends on a worker, which prepares
/// the run and has the engine build its decks; [`poll`] finishes it.
pub(crate) fn run(app: &mut RSpiceApp) {
    start(&mut app.state, &mut app.simulation_controller, false);
}

/// Start a pass. `queue` asks for the run to be queued once the pass
/// authorizes it.
fn start(state: &mut AppState, controller: &mut SimulationController, queue: bool) {
    if let Some(check) = state.workbench.preflight.checking.as_mut() {
        // The decks being built are the ones this press would build. A Run
        // press during a check alone asks for the run it authorizes.
        check.queue |= queue;
        return;
    }
    state.sync_active_schematic_to_workspace();
    // In the frame that takes the press, before the report is collected from
    // the receipt it leaves current: the worker reads a frozen copy of the
    // inputs, and the store is the session's.
    if let Err(error) = ensure_configured_design_check(state) {
        state.record(
            LogSeverity::Error,
            LogSource::Studio,
            format!("Design checks could not run: {error}"),
        );
    }
    let mut report = collect(state);
    if report.blockers.is_empty() {
        // The run is prepared, and the engine builds the decks it dispatches,
        // on a worker over a copy of the inputs taken here. Only a prepared
        // run has decks to build, so what the engine refuses is decided
        // against exactly those decks, and it keeps the run from being
        // authorized until the build says otherwise.
        match controller.start_preflight(state) {
            Ok(()) => {
                state.workbench.preflight.checking = Some(PreflightCheck {
                    report,
                    queue,
                    inputs_at_press: state.dispatch_inputs_at_press().ok(),
                });
                return;
            }
            Err(error) => report.blockers.push(preparation_issue(&error)),
        }
    }
    conclude(state, controller, report, None, queue);
}

/// The blocker a preparation failure stands as.
fn preparation_issue(error: &PreparationError) -> PreflightIssue {
    PreflightIssue {
        check: preparation_check_label(error.stage()).to_owned(),
        observed: error.message().to_owned(),
        required: preparation_requirement(error.stage()).to_owned(),
        remediation: preparation_remediation(error.stage(), error.line()),
    }
}

/// What a prepared run's report authorizes, if the engine refuses nothing.
fn contract_of(
    metadata: crate::simulation::execution::PreparedRunMetadata,
) -> PreparedPreflightContract {
    PreparedPreflightContract {
        snapshot_digest: metadata.snapshot_digest,
        source_digest: metadata.source_digest,
        receipt_digest: metadata.receipt_digest,
        receipt_label: metadata.receipt_label,
        analysis_ids: metadata.analysis_ids,
        task_count: metadata.task_count,
        saved_output_contract_count: metadata.saved_output_contract_count,
        pvt_point_count: metadata.pvt_point_count,
        target: metadata.target,
        save_policy: metadata.save_policy,
        model_identity_count: metadata.model_identity_count,
    }
}

/// Finish the pass whose run the worker was preparing and checking, once it
/// has a verdict. Called once per frame; does nothing while the worker runs.
///
/// A stop the reader asked for since the last frame is applied first, so a
/// verdict that lands in between authorizes nothing.
///
/// A pass whose run is no longer wanted — an edit since the press dropped the
/// retained run, or another preparation replaced it — authorizes nothing. Its
/// findings are still published, against the inputs as they stood at the
/// press, so they read as stale rather than as current.
pub(crate) fn poll(state: &mut AppState, controller: &mut SimulationController) {
    if state.workbench.preflight.take_check_cancel_request() {
        cancel(state, controller);
        return;
    }
    let Some(check) = state.workbench.preflight.checking.as_ref() else {
        // A pass whose state was dropped — a replaced session — has nothing
        // left to finish.
        controller.cancel_preflight();
        return;
    };
    if controller.preflight_stage().is_none() {
        // The controller dropped the worker: the design was replaced under it.
        let revision = check.report.project_revision;
        state.workbench.preflight.checking = None;
        state.record(
            LogSeverity::Warning,
            LogSource::Studio,
            format!(
                "Preflight stopped · the design was replaced while revision {revision} was \
                 checked · nothing was queued"
            ),
        );
        return;
    }
    let Some(PreflightOutcome {
        verdict,
        current: retained,
    }) = controller.poll_preflight()
    else {
        return;
    };
    let Some(PreflightCheck {
        mut report,
        queue,
        inputs_at_press,
    }) = state.workbench.preflight.checking.take()
    else {
        return;
    };
    match verdict {
        PreflightVerdict::NotPrepared(error) => report.blockers.push(preparation_issue(&error)),
        PreflightVerdict::Checked {
            mut metadata,
            refusals,
        } => {
            report.advisories.extend(
                std::mem::take(&mut metadata.advisories)
                    .into_iter()
                    .map(PreflightAdvisory::from),
            );
            let refused = engine_refusals::engine_refusal_issues(state, &refusals);
            if refused.is_empty() {
                report.prepared = Some(contract_of(*metadata));
            } else {
                report.blockers.extend(refused);
            }
        }
        PreflightVerdict::NoVerdict => report.blockers.push(engine_refusals::no_verdict_issue()),
    }
    let (topology_root, topology_revision, topology_closure) = state.configured_topology_revision();
    let unedited = report.is_current_for(
        state.workspace.content.project.revision().get(),
        &topology_root,
        topology_revision,
        &topology_closure,
        state.active_plan_revision(),
    );
    if retained && unedited {
        conclude(state, controller, report, inputs_at_press, queue);
    } else {
        if retained {
            // The run was retained as the one checked, but the design it was
            // prepared from has moved: it authorizes nothing now.
            controller.clear_prepared_run();
        }
        conclude_stale(state, report, inputs_at_press);
    }
}

/// Stop the pass whose run the worker is preparing or checking. Nothing it
/// found is published, and the run it was checking is not authorized.
fn cancel(state: &mut AppState, controller: &mut SimulationController) {
    let Some(check) = state.workbench.preflight.checking.take() else {
        return;
    };
    controller.cancel_preflight();
    controller.clear_prepared_run();
    state.record(
        LogSeverity::Warning,
        LogSource::Studio,
        format!(
            "Preflight cancelled · design revision {} was not checked · nothing was queued",
            check.report.project_revision
        ),
    );
}

/// A pass whose run is no longer the one retained: its findings are
/// published against the inputs it checked, and nothing is authorized or
/// queued. The report is not retained — whatever moved the run has already
/// said what the session's report is.
fn conclude_stale(
    state: &mut AppState,
    mut report: PreflightReport,
    inputs_at_press: Option<crate::product::ContentDigest>,
) {
    report.prepared = None;
    if let Some(inputs_at_press) = inputs_at_press {
        state.publish_dispatch_diagnostics_against(&report, inputs_at_press);
    }
    state.record(
        LogSeverity::Warning,
        LogSource::Studio,
        format!(
            "Preflight stale · the design changed while revision {} was checked · {} finding{} \
             marked stale · nothing was queued",
            report.project_revision,
            report.blockers.len(),
            if report.blockers.len() == 1 { "" } else { "s" }
        ),
    );
}

/// Retain a finished pass's report, publish what it found, say what it
/// decided, and queue the run when the press asked for one and the report
/// authorizes it.
fn conclude(
    state: &mut AppState,
    controller: &mut SimulationController,
    report: PreflightReport,
    inputs_at_press: Option<crate::product::ContentDigest>,
    queue: bool,
) {
    if report.prepared.is_none() {
        controller.clear_prepared_run();
    }
    let current_plan = state.active_plan_revision();
    let (topology_root, topology_revision, topology_closure) = state.configured_topology_revision();
    let blocked = !report.is_runnable_for(
        state.workspace.content.project.revision().get(),
        &topology_root,
        topology_revision,
        &topology_closure,
        current_plan,
    );
    let message = if blocked {
        format!(
            "Preflight blocked · {} blocking issue{} · design revision {} was not queued",
            report.blockers.len(),
            if report.blockers.len() == 1 { "" } else { "s" },
            report.project_revision
        )
    } else {
        let task_count = report
            .prepared
            .as_ref()
            .map_or(0, |prepared| prepared.task_count);
        format!(
            "Preflight complete · 0 blocking errors · {} advisor{} · design revision {} · {} task{}",
            report.advisories.len(),
            if report.advisories.len() == 1 {
                "y"
            } else {
                "ies"
            },
            report.project_revision,
            task_count,
            if task_count == 1 { "" } else { "s" }
        )
    };

    // Retained before its findings are published: the findings stand only
    // while the report they came from does, and that is part of what their
    // digest is taken over.
    state.workbench.preflight.report = Some(report.clone());
    match inputs_at_press {
        Some(inputs_at_press) => {
            state.publish_dispatch_diagnostics_against(&report, inputs_at_press)
        }
        None => state.publish_dispatch_diagnostics(&report),
    }
    // An event, at the severity of the pass rather than of its findings: the
    // findings are records on the Problems page, and this says the pass ran
    // and what it decided.
    let severity = if blocked {
        LogSeverity::Warning
    } else {
        LogSeverity::Info
    };
    state.record(severity, LogSource::Studio, message);

    if blocked {
        show_blockers(state);
    } else if queue {
        queue_retained_run(state);
    }
}

/// Bring the reader to the blockers a pass just filed: the Problems page,
/// listing them. A filter left from earlier reading could hide the very
/// findings that stopped the run, so the filters are cleared.
fn show_blockers(state: &mut AppState) {
    state.workbench.console_visible = true;
    state.workbench.console_page = ConsolePage::Problems;
    state.problems_view.clear_filters();
}

/// Run the same governed preflight used by the explicit Preflight command and
/// queue only the exact immutable snapshot it authorizes. A retained current
/// report is reusable; when it consumes the one-shot permit, the controller
/// still refuses the snapshot if anything it was prepared from has moved.
///
/// While the engine builds the decks, the run is queued when [`poll`] finds
/// that they authorize it, and not before.
pub(crate) fn run_and_queue(app: &mut RSpiceApp) {
    if app.state.simulation.execution.has_active_execution()
        || app.state.simulation.execution.trigger_simulation
    {
        app.state
            .prompt("A simulation is already running or waiting to start.");
        return;
    }

    // A report retained while a check is building decks is the one that
    // check will replace, so it authorizes nothing.
    if app.state.workbench.preflight.checking.is_none() && queue_retained_run(&mut app.state) {
        return;
    }

    start(&mut app.state, &mut app.simulation_controller, true);
}

/// Queue a retained authorized report only while every revision it names is
/// still current.
fn queue_retained_run(state: &mut AppState) -> bool {
    let project_revision = state.workspace.content.project.revision().get();
    let (topology_root, topology_revision, topology_closure) = state.configured_topology_revision();
    let current_plan = state.active_plan_revision();
    let runnable = state
        .workbench
        .preflight
        .report
        .as_ref()
        .is_some_and(|report| {
            report.is_runnable_for(
                project_revision,
                &topology_root,
                topology_revision,
                &topology_closure,
                current_plan,
            )
        });
    if !runnable {
        return false;
    }

    state.request_run_set_simulation();
    state.workbench.activate(Workspace::Simulate);
    true
}

#[cfg(test)]
impl RSpiceApp {
    /// Block until the pass in flight has a verdict, and finish it, as the
    /// frame loop would on the frame that finds it.
    pub(crate) fn settle_preflight(&mut self) {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(300);
        while self.state.workbench.preflight.checking.is_some() {
            assert!(
                std::time::Instant::now() < deadline,
                "the engine pass did not finish"
            );
            poll(&mut self.state, &mut self.simulation_controller);
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
    }
}

/// Check Design over the configured design, unless a check of it still
/// describes the design as it stands. Its receipt is what the pass decides
/// the design rules from, and the page lists what it published, so the two
/// cannot disagree; a design checked since its last edit is not checked
/// again.
fn ensure_configured_design_check(state: &mut AppState) -> Result<(), String> {
    let root = state.workspace.content.simulation_root_reference();
    state.ensure_design_check(root, CheckScope::Design)
}

/// The report a pass over `state` collects, the configured design checked
/// first as the pass checks it.
#[cfg(test)]
fn collect_report(state: &AppState) -> PreflightReport {
    let mut state = state.clone();
    let _ = ensure_configured_design_check(&mut state);
    collect(&state)
}

/// The one blocker that stands for the configured root's design-rule errors.
///
/// One, not one per finding: the findings themselves are published by the
/// design-rule producer, under the identities the Check command gives them,
/// and listing them again as blockers is what made the page state each one
/// twice. This says the run cannot start because of them, and its route
/// leads to them.
fn design_rule_blocker(
    root: &crate::state::CellViewRef,
    result: &rspice_design::drc::DrcResult,
) -> Option<PreflightIssue> {
    let summary = result.summary();
    let errors = summary.critical + summary.errors;
    (errors > 0).then(|| PreflightIssue {
        check: "Schematic validation".to_owned(),
        observed: format!(
            "{errors} design-rule error{} in {}",
            if errors == 1 { "" } else { "s" },
            root.display_path()
        ),
        required: "No error-level design-rule findings in the configured design".to_owned(),
        remediation: PreflightRemediation::DesignChecks,
    })
}

fn collect(state: &AppState) -> PreflightReport {
    let mut blockers = Vec::new();
    let mut advisories = Vec::new();
    let root_reference = state.workspace.content.simulation_root_reference();
    let inspection = state.workspace.inspect_design_projection(
        &state.library_manager,
        &state.workspace.content.active_view,
        &state.schematic,
    );
    let execution_projection = inspection
        .clone()
        .and_then(|projection| projection.into_execution());

    match &execution_projection {
        Err(error) => blockers.push(PreflightIssue {
            check: "Design topology".to_owned(),
            observed: error.to_string(),
            required: "An existing, fully resolved executable configuration root".to_owned(),
            remediation: PreflightRemediation::DesignChecks,
        }),
        Ok(projection)
            if projection
                .root_schematic()
                .is_some_and(|root| root.document().components.is_empty()) =>
        {
            blockers.push(PreflightIssue {
                check: "Design topology".to_owned(),
                observed: format!(
                    "Configured simulation root '{}' contains no components.",
                    root_reference.display_path()
                ),
                required: "A non-empty circuit topology".to_owned(),
                remediation: PreflightRemediation::DesignChecks,
            });
        }
        Ok(_) => {}
    }

    let hierarchy_bindings = inspection.as_ref().map_or(&[][..], |projection| {
        projection.hierarchy_resolution().bindings.as_slice()
    });
    for binding in hierarchy_bindings
        .iter()
        .filter(|binding| !binding.status.is_resolved())
    {
        let instance_scope = if binding.instance_count == 1 {
            String::new()
        } else {
            format!(" · {} instances", binding.instance_count)
        };
        let diagnostic = binding.diagnostic.clone().unwrap_or_else(|| {
            format!(
                "{}/{} is {}",
                binding.reference.library,
                binding.reference.cell,
                binding.status.label()
            )
        });
        blockers.push(PreflightIssue {
            check: "Hierarchy binding".to_owned(),
            observed: format!("{diagnostic}{instance_scope}"),
            required: "A finite executable master for every hierarchical instance".to_owned(),
            remediation: PreflightRemediation::DesignChecks,
        });
    }
    for binding in hierarchy_bindings
        .iter()
        .filter(|binding| binding.used_review_fallback)
    {
        advisories.push(PreflightAdvisory::about(
            "configuration-fallback",
            binding.reference.key(),
            format!(
                "Configuration fallback reviewed for {} at {}",
                binding.reference.display_path(),
                binding.instance_paths.join(", ")
            ),
        ));
    }
    // A warning is a resolved binding the configuration honours differently
    // than it reads. It blocks nothing — the run will proceed — but starting
    // one without saying so leaves the author reading a policy the executed
    // hierarchy does not follow.
    for binding in hierarchy_bindings {
        for warning in &binding.warnings {
            advisories.push(PreflightAdvisory::about(
                "configuration-warning",
                format!("{}:{warning}", binding.reference.key()),
                format!(
                    "Configuration warning for {}: {warning}",
                    binding.reference.display_path()
                ),
            ));
        }
    }

    if execution_projection.is_ok() {
        // Check Design's own receipt for the configured design: the
        // configuration and the waivers the page states its findings under.
        if let Some(result) = state.current_design_check(&root_reference, CheckScope::Design) {
            blockers.extend(design_rule_blocker(&root_reference, result));
        } else {
            blockers.push(PreflightIssue {
                check: "Schematic validation".to_owned(),
                observed: "The configured-root schematic validator did not complete.".to_owned(),
                required: "A completed validation result for this topology revision".to_owned(),
                remediation: PreflightRemediation::DesignChecks,
            });
        }
    }

    match state.sim_setup.stable_analysis_plan() {
        Err(error) => blockers.push(PreflightIssue {
            check: "Analysis-instance plan".to_owned(),
            observed: error,
            required: "A migrated, structurally valid simulation plan".to_owned(),
            remediation: PreflightRemediation::SimulationPlan { analysis: None },
        }),
        Ok(plan) => {
            for issue in plan.validation_issues() {
                blockers.push(PreflightIssue {
                    check: "Analysis-instance graph".to_owned(),
                    observed: issue.to_string(),
                    required: "Stable enabled identities with resolved, ordered prerequisites"
                        .to_owned(),
                    remediation: PreflightRemediation::SimulationPlan {
                        analysis: issue_analysis(&issue),
                    },
                });
            }
            for instance in plan
                .instances()
                .iter()
                .filter(|instance| instance.enabled())
            {
                if let Some(error) = state
                    .sim_setup
                    .analysis_draft_validation_error(instance.draft())
                {
                    blockers.push(PreflightIssue {
                        check: format!("{} configuration", instance.kind().label()),
                        observed: format!("{}: {error}", instance.id()),
                        required: "A complete, numerically valid analysis configuration".to_owned(),
                        remediation: PreflightRemediation::SimulationPlan {
                            analysis: Some(instance.id()),
                        },
                    });
                }
            }
        }
    }

    // A project need not have a technology; if it has one it must be valid,
    // and if the plan needs one it must have one. Each demanding entity is
    // reported as its own row so the operator sees what to change. A plan
    // that needs none, on a project that has none, is not a finding at all:
    // reporting it made every clean pass without a technology a warning.
    let demand = state.technology_demand();
    match state.workspace.content.project.technology_binding() {
        None if demand.is_empty() => {}
        None => blockers.extend(demand.reasons().iter().map(|reason| PreflightIssue {
            check: "Project technology contract".to_owned(),
            observed: reason.observed(),
            required: reason.required(),
            remediation: PreflightRemediation::ProjectTechnology,
        })),
        Some(binding) => {
            if state
                .workspace
                .content
                .project
                .technology_change_audit()
                .is_empty()
            {
                blockers.push(PreflightIssue {
                    check: "Project technology contract".to_owned(),
                    observed:
                        "The attached technology binding predates checkpoint-backed authority receipts"
                            .to_owned(),
                    required: "Reattach the technology to record an audited change receipt"
                        .to_owned(),
                    remediation: PreflightRemediation::ProjectTechnology,
                });
            }
            if let Err(error) = state
                .model_library_manager
                .catalog()
                .validate_attached_technology(Some(binding))
            {
                blockers.push(PreflightIssue {
                    check: "Project model technology".to_owned(),
                    observed: error,
                    required:
                        "The exact attached model-source closure must resolve without catalog drift"
                            .to_owned(),
                    remediation: PreflightRemediation::ProjectTechnology,
                });
            }
            if let Err(error) = state
                .pdk_config
                .technology_registry
                .validate_project_binding(binding)
            {
                blockers.push(PreflightIssue {
                    check: "Project signed PDK".to_owned(),
                    observed: error.to_string(),
                    required:
                        "The exact attached package revision must remain signature-trusted and compatible with this runtime"
                            .to_owned(),
                    remediation: PreflightRemediation::ProjectTechnology,
                });
            }
        }
    }

    // An unmet demand already owns the missing reference section and carries the
    // only remediation that resolves it; a second row would contradict it.
    if (state.project_technology_in_effect() || demand.is_empty())
        && let Err(error) = state
            .model_library_manager
            .reference_process_model_cards(state.sim_setup.reference_pvt.process)
    {
        let process = state.sim_setup.reference_pvt.process;
        // The check is asked of the whole closure and its answer is a sentence
        // about the closure, so the destination is resolved from what the
        // failure *names* — the process — rather than parsed back out of that
        // sentence. A process declared by exactly one library has an
        // unambiguous home and the route lands on it with the corner matrix
        // inspected; anything else keeps the page destination and says which
        // library the reader has to pick, because Corners & sections renders
        // from a selection and landing there with none is landing nowhere.
        let providers = state
            .model_library_manager
            .libraries_declaring_process(process);
        let (required, remediation) = match providers.as_slice() {
            [library] => (
                format!("A resolved {} model section", process.short_name()),
                PreflightRemediation::model_corner(*library, None),
            ),
            _ => (
                format!(
                    "A resolved {} model section \u{00b7} select the library that declares {} on \
                     Corners & sections",
                    process.short_name(),
                    process.short_name()
                ),
                PreflightRemediation::models_page(ModelsPage::Corners),
            ),
        };
        blockers.push(PreflightIssue {
            check: "Reference model binding".to_owned(),
            observed: error,
            required,
            remediation,
        });
    }

    advisories.extend(temperature_validity_advisories(state));

    let simulation_plan = state.active_plan_revision();
    let (topology_root, topology_revision, topology_closure) = state.configured_topology_revision();
    PreflightReport {
        project_revision: state.workspace.content.project.revision().get(),
        topology_root,
        topology_revision,
        topology_closure,
        simulation_plan_id: simulation_plan.map(|(id, _)| id),
        simulation_plan_revision: simulation_plan.map(|(_, revision)| revision),
        blockers,
        advisories,
        prepared: None,
    }
}

/// Corners the run set asks to run outside the range the PDK qualified them for.
///
/// An advisory rather than a blocker, deliberately. A run at 150 °C against a
/// corner qualified to 125 °C is a run the foundry does not vouch for — it is
/// not a run the tool can prove wrong, and refusing it would be the tool
/// substituting its judgement for the engineer's. What the tool owes is the
/// sentence, once, before the run rather than after it.
///
/// Only the corner each library has *active* is checked here: that is the one
/// a run expands. Every corner the project is authoring is checked on the
/// Corners page, where authoring one is what a reader is doing.
fn temperature_validity_advisories(state: &AppState) -> Vec<PreflightAdvisory> {
    let requested = state.sim_setup.requested_temperatures_celsius();
    state
        .model_library_manager
        .libraries_sorted()
        .into_iter()
        .filter_map(|library| {
            let active = library.selected_corner.as_deref()?;
            let corner = library
                .corners
                .values()
                .find(|corner| corner.name.eq_ignore_ascii_case(active))?;
            let outside = corner.temperatures_outside_qualified_range(&requested);
            (!outside.is_empty()).then(|| PreflightAdvisory {
                message: format!(
                    "'{}' is active in '{}' and qualified {}; this run set requests {}",
                    corner.name,
                    library.name,
                    corner.qualified_range_label(),
                    crate::state::model_library::stated_temperatures(&outside)
                ),
                // The two identities the sentence above is built from travel
                // with the destination, so Corners & sections opens on this
                // library's matrix with this corner inspected.
                remediation: Some(PreflightRemediation::model_corner(
                    library.name.clone(),
                    Some(corner.name.clone()),
                )),
                about: (
                    "corner-qualification",
                    format!("{}:{}", library.name, corner.name),
                ),
            })
        })
        .collect()
}

fn preparation_check_label(stage: crate::simulation::execution::PreparationStage) -> &'static str {
    use crate::simulation::execution::PreparationStage;
    match stage {
        PreparationStage::DesignChecks => "Schematic validation",
        PreparationStage::SourceChecks => "Source closure",
        PreparationStage::AnalysisPlan => "Run set",
        PreparationStage::ModelBindings => "Model bindings",
        PreparationStage::Netlist => "Executable netlist",
        PreparationStage::Authorization => "Dispatch authorization",
    }
}

fn preparation_requirement(stage: crate::simulation::execution::PreparationStage) -> &'static str {
    use crate::simulation::execution::PreparationStage;
    match stage {
        PreparationStage::DesignChecks => "A current completed source-check receipt",
        PreparationStage::SourceChecks => "A closed, parseable source dependency set",
        PreparationStage::AnalysisPlan => "A complete executable analysis configuration",
        PreparationStage::ModelBindings => "Authenticated materialized model sources",
        PreparationStage::Netlist => "A deterministic self-contained executable netlist",
        PreparationStage::Authorization => "A one-use generation-safe dispatch authorization",
    }
}

/// Where a preparation failure is repaired, and — for the netlist stage —
/// exactly where in the deck.
///
/// `line` comes from the failure itself rather than from its wording. A
/// stage that reports no line yields a destination with none, so the row
/// that offers it can tell a located finding from an unlocated one instead
/// of promising a jump it cannot make.
fn preparation_remediation(
    stage: crate::simulation::execution::PreparationStage,
    line: Option<usize>,
) -> PreflightRemediation {
    use crate::simulation::execution::PreparationStage;
    match stage {
        // A preparation failure names a stage, not an analysis, so the page
        // opens without moving the selection.
        PreparationStage::AnalysisPlan | PreparationStage::Authorization => {
            PreflightRemediation::SimulationPlan { analysis: None }
        }
        // A model-binding failure is repaired where the bindings live, not by
        // re-running the source checks that reported it.
        PreparationStage::ModelBindings => PreflightRemediation::models_page(ModelsPage::Corners),
        // A netlist-stage failure is a defect in the deck the run would
        // execute, so it opens the deck. Running the design checks again is
        // what this used to do, and those checks had already passed — that is
        // how the stage got as far as generating a netlist.
        PreparationStage::Netlist => PreflightRemediation::NetlistSource { line },
        PreparationStage::DesignChecks | PreparationStage::SourceChecks => {
            PreflightRemediation::DesignChecks
        }
    }
}

/// The analysis a plan finding is about, where it is about one the page can
/// show.
///
/// The same reading of each finding the Analyses page uses to decide which
/// analysis a finding is shown against. A cycle is shown on each of its
/// members, so it opens on the first. A finding about the plan as a whole, a
/// tombstone or a run receipt names no analysis the page lists.
fn issue_analysis(
    issue: &crate::simulation::plan::AnalysisPlanIssue,
) -> Option<crate::product::AnalysisInstanceId> {
    use crate::simulation::plan::AnalysisPlanIssue;
    match issue {
        AnalysisPlanIssue::DuplicateInstanceId { id }
        | AnalysisPlanIssue::ReusedTombstonedId { id }
        | AnalysisPlanIssue::KindDraftMismatch { id, .. }
        | AnalysisPlanIssue::InvalidInstanceRevision { id }
        | AnalysisPlanIssue::InvalidInstanceName { id }
        | AnalysisPlanIssue::DuplicateInstanceName { id, .. }
        | AnalysisPlanIssue::InvalidLifecycle { id, .. } => Some(*id),
        AnalysisPlanIssue::MissingPrerequisite { dependent, .. }
        | AnalysisPlanIssue::UnexpectedDependencyRole { dependent, .. }
        | AnalysisPlanIssue::DuplicateDependencyRole { dependent, .. }
        | AnalysisPlanIssue::SelfDependency { dependent }
        | AnalysisPlanIssue::DanglingDependency { dependent, .. }
        | AnalysisPlanIssue::WrongDependencyKind { dependent, .. }
        | AnalysisPlanIssue::DisabledDependency { dependent, .. }
        | AnalysisPlanIssue::DependencyNotEarlier { dependent, .. }
        | AnalysisPlanIssue::IncompatibleDependencyConfiguration { dependent, .. } => {
            Some(*dependent)
        }
        AnalysisPlanIssue::DependencyCycle { members } => members.first().copied(),
        AnalysisPlanIssue::NoEnabledInstances
        | AnalysisPlanIssue::DuplicateTombstoneId { .. }
        | AnalysisPlanIssue::InvalidTombstoneRevision { .. }
        | AnalysisPlanIssue::InvalidReceiptSequence { .. }
        | AnalysisPlanIssue::InvalidReceiptRevision { .. }
        | AnalysisPlanIssue::DanglingReceiptInstance { .. }
        | AnalysisPlanIssue::ReceiptKindMismatch { .. }
        | AnalysisPlanIssue::EmptyReceiptDetail { .. }
        | AnalysisPlanIssue::InvalidNextReceiptSequence { .. } => None,
    }
}

#[cfg(test)]
mod tests;

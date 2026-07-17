//! Governed execution bridge for the strict project Automation workflow.

use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::automation_workflow::{
    ArtifactKind, AutomationPlan, CheckEvidence, CheckOutcome, CompletedEvidence, DiagnosticSet,
    compare_governed_waveforms, compile_workflow, render_requested_artifacts,
};
use crate::common::export_workflow::SaveDialogConfig;
use crate::common::{ConsoleMessage, RSpiceApp};
use crate::product::{ContentDigest, RunId, SimulationPlanId};
use crate::state::{AnalysisResultSourceDomain, ProjectSourceLanguage, SimulationRun, SpecEntry};

use super::{
    AutomationDispatchSnapshot, AutomationExecutionState, AutomationValidationReceipt,
    CodeEditorDiagnostic, CodeEditorSeverity, SourceOperationToken,
};

/// Validate, bind, and dispatch the exact current workflow against the active
/// project plan. Every prerequisite is checked before the ordinary prepared
/// simulation controller receives a run request.
pub fn start_automation_workflow(app: &mut RSpiceApp) {
    if app.state.ui.code_workspace.automation.execution.is_active() {
        return;
    }
    let Some(document) = app
        .state
        .workspace
        .project_sources
        .get(ProjectSourceLanguage::RSpiceAutomation)
        .cloned()
    else {
        fail(app, "This project has no Automation source document.");
        return;
    };
    let token = SourceOperationToken {
        project_id: app.state.workspace.project.id(),
        revision: document.revision().get(),
        content_digest: document.content_digest(),
    };
    let plan = match compile_workflow(document.content()) {
        Ok(plan) => plan,
        Err(diagnostics) => {
            app.state.ui.code_workspace.automation.diagnostics = editor_diagnostics(&diagnostics);
            app.state.ui.code_workspace.automation.diagnostic_token = Some(token);
            fail(app, &diagnostics.to_string());
            return;
        }
    };
    app.state.ui.code_workspace.automation.diagnostics.clear();
    app.state.ui.code_workspace.automation.diagnostic_token = None;

    let active_name = app.state.sim_setup.active_plan_name().as_str().to_owned();
    if plan.project_name() != active_name {
        fail(
            app,
            &format!(
                "Workflow plan {:?} does not match the active project plan {:?}.",
                plan.project_name(),
                active_name
            ),
        );
        return;
    }
    let plan_id = match active_plan_id(app) {
        Ok(plan_id) => plan_id,
        Err(error) => {
            fail(app, &error);
            return;
        }
    };
    let Some(plan_payload) = app.state.workspace.active_plan_data(plan_id).cloned() else {
        fail(
            app,
            "The active simulation plan has no governed configuration payload.",
        );
        return;
    };
    if plan_payload.specs.is_empty() {
        fail(
            app,
            "The workflow requires release specifications, but the active plan has none configured.",
        );
        return;
    }
    if let Err(error) = validate_regression_policy(&plan_payload.regression_tolerances) {
        fail(app, &format!("The regression policy is invalid: {error}"));
        return;
    }
    if let Err(error) = require_corner_matrix(app) {
        fail(app, &error);
        return;
    }
    let baseline_run = match baseline_run(app, plan_id) {
        Ok(run) => run,
        Err(error) => {
            fail(app, &error);
            return;
        }
    };
    if let Some(reason) = app.state.simulation_run_block_reason() {
        fail(
            app,
            &format!("Run preflight rejected the workflow: {reason}"),
        );
        return;
    }
    if let Err(error) = app
        .state
        .workspace
        .mark_project_source_validated(ProjectSourceLanguage::RSpiceAutomation)
    {
        fail(
            app,
            &format!("Could not retain the workflow validation receipt: {error}"),
        );
        return;
    }

    let plan_revision = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(crate::simulation::plan::SimulationPlan::revision)
        .expect("active plan identity was checked above");
    let baseline_digest = match simulation_run_digest(&baseline_run) {
        Ok(digest) => digest,
        Err(error) => {
            fail(
                app,
                &format!("Could not seal the governed baseline snapshot: {error}"),
            );
            return;
        }
    };
    // Automation owns a one-shot prepared permit for precisely this dispatch.
    // Clear any unrelated retained preflight first, then seal the complete
    // current execution contract immediately before publishing the request.
    app.simulation_controller.clear_prepared_run();
    let prepared = match app
        .simulation_controller
        .prepare_run_set_for_preflight(&app.state)
    {
        Ok(prepared) => prepared,
        Err(error) => {
            fail(
                app,
                &format!("Could not authorize the Automation run snapshot: {error}"),
            );
            return;
        }
    };
    let snapshot = AutomationDispatchSnapshot {
        project_id: app.state.workspace.project.id(),
        project_revision: app.state.workspace.project.revision(),
        plan_id,
        plan_revision,
        prepared_snapshot_digest: prepared.snapshot_digest,
        source_content_digest: prepared.source_digest,
        plan_name: active_name,
        plan_payload,
        project_sources: app.state.workspace.project_sources.clone(),
        baseline_run: Arc::new(baseline_run),
        baseline_digest,
    };
    let prior_run_ids = app
        .state
        .simulation
        .runs
        .iter()
        .map(|run| run.run_id)
        .collect();
    app.state.ui.code_workspace.automation.receipt = Some(AutomationValidationReceipt {
        token,
        plan: plan.clone(),
    });
    app.state.ui.code_workspace.automation.artifacts.clear();
    app.state.ui.code_workspace.automation.last_error = None;
    app.state.ui.code_workspace.automation.execution = AutomationExecutionState::AwaitingDispatch {
        token,
        plan,
        prior_run_ids,
        snapshot,
    };
    app.state.request_run_set_simulation();
    app.state.push_user_message(ConsoleMessage::info(
        "Automation workflow passed static validation and entered prepared-run preflight.",
    ));
}

/// Follow the exact run created by the prepared controller and publish
/// artifacts only after that immutable run, its release specs, and its named
/// baseline comparison all complete.
pub fn poll_automation_workflow(app: &mut RSpiceApp) {
    let execution = app.state.ui.code_workspace.automation.execution.clone();
    match execution {
        AutomationExecutionState::AwaitingDispatch {
            token,
            plan,
            prior_run_ids,
            snapshot,
        } => {
            if let Err(error) = dispatch_snapshot_is_current(app, &snapshot) {
                fail(app, &error);
                return;
            }
            let matching_run_ids = app
                .state
                .simulation
                .runs
                .iter()
                .filter(|run| {
                    !prior_run_ids.contains(&run.run_id) && run_matches_dispatch(run, &snapshot)
                })
                .map(|run| run.run_id)
                .collect::<Vec<_>>();
            if matching_run_ids.len() > 1 {
                fail(
                    app,
                    "Prepared execution produced multiple runs matching the same Automation dispatch contract; exact run correlation is ambiguous.",
                );
                return;
            }
            if let Some(run_id) = matching_run_ids.first().copied() {
                if app.state.simulation.is_running {
                    app.state.ui.code_workspace.automation.execution =
                        AutomationExecutionState::Running {
                            token,
                            plan,
                            run_id,
                            snapshot,
                        };
                } else {
                    finish_run(app, token, &plan, run_id, &snapshot);
                }
            } else if !app.state.simulation.trigger_simulation && !app.state.simulation.is_running {
                fail(
                    app,
                    "Prepared-run preflight ended without creating a unique authenticated run matching the immutable Automation dispatch contract.",
                );
            }
        }
        AutomationExecutionState::Running {
            token,
            plan,
            run_id,
            snapshot,
        } if !app.state.simulation.is_running => {
            finish_run(app, token, &plan, run_id, &snapshot);
        }
        AutomationExecutionState::Idle
        | AutomationExecutionState::Running { .. }
        | AutomationExecutionState::Complete { .. }
        | AutomationExecutionState::Failed => {}
    }
}

/// Save one completed artifact through the same native/browser publication
/// contract used by the rest of the application.
pub fn export_automation_artifact(app: &mut RSpiceApp, kind: ArtifactKind) {
    let Some(artifact) = app
        .state
        .ui
        .code_workspace
        .automation
        .artifacts
        .iter()
        .find(|artifact| artifact.kind() == kind)
        .cloned()
    else {
        fail(
            app,
            "That workflow artifact is not available for the completed run.",
        );
        return;
    };
    let (filter_name, extension): (&str, &[&str]) = match kind {
        ArtifactKind::JunitXml => ("JUnit XML", &["xml"]),
        ArtifactKind::SummaryJson => ("JSON", &["json"]),
        ArtifactKind::VerificationPdf => ("PDF", &["pdf"]),
    };
    let result = (|| {
        let path = app
            .export_workflow_io
            .show_save_dialog(SaveDialogConfig {
                title: "Export automation artifact",
                default_name: artifact.file_name(),
                filter_name,
                filter_extensions: extension,
            })?
            .ok_or_else(|| "export cancelled".to_owned())?;
        let destination = app.export_workflow_io.observe_destination(&path)?;
        app.export_workflow_io.write_bytes_file_observed(
            &destination,
            artifact.bytes(),
            artifact.media_type(),
        )?;
        Ok::<_, String>(path)
    })();
    match result {
        Ok(path) => app.state.push_user_message(ConsoleMessage::info(format!(
            "Exported workflow artifact to {}.",
            path.display()
        ))),
        Err(error) if error == "export cancelled" => {}
        Err(error) => fail(app, &format!("Could not export workflow artifact: {error}")),
    }
}

fn finish_run(
    app: &mut RSpiceApp,
    token: SourceOperationToken,
    plan: &AutomationPlan,
    run_id: RunId,
    snapshot: &AutomationDispatchSnapshot,
) {
    if !source_token_is_current(app, token) {
        fail(
            app,
            "The Automation source changed while the workflow was running; completed evidence was not published.",
        );
        return;
    }
    if let Err(error) = dispatch_snapshot_is_current(app, snapshot) {
        fail(app, &error);
        return;
    }
    let Some(candidate) = app.state.simulation.run_by_stable_id(run_id).cloned() else {
        fail(
            app,
            "The workflow run is no longer retained in project history.",
        );
        return;
    };
    if !run_matches_dispatch(&candidate, snapshot) {
        fail(
            app,
            "The selected run no longer matches the immutable Automation dispatch contract.",
        );
        return;
    }
    if let Err(error) = validate_candidate_run(&candidate, snapshot) {
        fail(
            app,
            &format!("Workflow run evidence is incomplete: {error}"),
        );
        return;
    }
    if let Err(error) = validate_baseline_run(&snapshot.baseline_run, snapshot.plan_id) {
        fail(app, &format!("Baseline evidence is not eligible: {error}"));
        return;
    }

    let checks = match build_spec_evidence(&snapshot.plan_payload.specs, &candidate) {
        Ok(checks) => checks,
        Err(error) => {
            fail(
                app,
                &format!("Could not seal specification evidence: {error}"),
            );
            return;
        }
    };
    let comparison = match compare_governed_waveforms(
        plan.baseline(),
        &snapshot.baseline_run,
        &candidate,
        &snapshot.plan_payload.regression_tolerances,
    ) {
        Ok(comparison) => comparison,
        Err(error) => {
            fail(app, &error);
            return;
        }
    };
    let evidence = match CompletedEvidence::try_new(
        plan.source_digest(),
        snapshot.project_revision.get().to_string(),
        run_id.to_string(),
        checks,
        comparison,
    ) {
        Ok(evidence) => evidence,
        Err(error) => {
            fail(app, &format!("Could not seal workflow evidence: {error}"));
            return;
        }
    };
    let passed = evidence.passed();
    let artifacts = match render_requested_artifacts(plan, &evidence) {
        Ok(artifacts) => artifacts,
        Err(error) => {
            fail(
                app,
                &format!("Could not render workflow artifacts: {error}"),
            );
            return;
        }
    };
    app.state.ui.code_workspace.automation.artifacts = artifacts;
    app.state.ui.code_workspace.automation.last_error = None;
    app.state.ui.code_workspace.automation.execution =
        AutomationExecutionState::Complete { run_id, passed };
    let message = if passed {
        "Automation workflow completed; every release gate passed."
    } else {
        "Automation workflow completed with failing release evidence."
    };
    app.state.push_user_message(if passed {
        ConsoleMessage::info(message)
    } else {
        ConsoleMessage::warning(message)
    });
}

fn active_plan_id(app: &RSpiceApp) -> Result<SimulationPlanId, String> {
    app.state
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(crate::simulation::plan::SimulationPlan::id)
        .ok_or_else(|| "The active simulation plan has not been migrated.".to_owned())
}

fn require_corner_matrix(app: &RSpiceApp) -> Result<(), String> {
    use crate::simulation::plan::AnalysisKind;
    let stable = app.state.sim_setup.stable_analysis_plan()?;
    if !stable
        .instances()
        .iter()
        .any(|instance| instance.enabled() && instance.kind() == AnalysisKind::Corner)
    {
        return Err(
            "with_corners(\"all\") requires an enabled Process corners analysis in the active plan."
                .to_owned(),
        );
    }
    app.state
        .sim_setup
        .corner
        .to_config()
        .map(|_| ())
        .map_err(|error| format!("The configured PVT matrix is invalid: {error}"))
}

fn baseline_run(app: &RSpiceApp, plan_id: SimulationPlanId) -> Result<SimulationRun, String> {
    let run_id = app
        .state
        .workspace
        .active_plan_data(plan_id)
        .and_then(|payload| payload.regression_baseline_run)
        .ok_or_else(|| {
            "baseline=\"main\" requires a retained governed baseline for the active plan."
                .to_owned()
        })?;
    let run = app
        .state
        .simulation
        .run_by_stable_id(run_id)
        .ok_or_else(|| "The active plan's baseline run is no longer retained.".to_owned())?;
    validate_baseline_run(run, plan_id)?;
    Ok(run.clone())
}

fn validate_run_shape(run: &SimulationRun) -> Result<(), String> {
    let receipt = run
        .prepared_receipt()
        .ok_or_else(|| "run has no prepared-run authority".to_owned())?;
    run.validate_provenance()?;
    if run.analyses.len() != receipt.tasks().len() {
        return Err(format!(
            "{} of {} authenticated tasks produced results",
            run.analyses.len(),
            receipt.tasks().len()
        ));
    }
    if let Some(analysis) = run.analyses.iter().find(|analysis| !analysis.success) {
        return Err(analysis.error_message.clone().unwrap_or_else(|| {
            format!("analysis {} failed without retained detail", analysis.label)
        }));
    }
    if !run.success {
        return Err("run-level success status is false".to_owned());
    }
    Ok(())
}

fn validate_baseline_run(run: &SimulationRun, plan_id: SimulationPlanId) -> Result<(), String> {
    validate_run_shape(run)?;
    let receipt = run
        .prepared_receipt()
        .expect("run shape requires a prepared receipt");
    if receipt.source_domain() != AnalysisResultSourceDomain::SimulationPlan
        || receipt.simulation_plan_id() != Some(plan_id)
    {
        return Err("baseline was not produced by the active simulation plan identity".to_owned());
    }
    Ok(())
}

fn validate_candidate_run(
    run: &SimulationRun,
    snapshot: &AutomationDispatchSnapshot,
) -> Result<(), String> {
    validate_run_shape(run)?;
    if !run_matches_dispatch(run, snapshot) {
        return Err("run receipt does not match the dispatched project/plan revision".to_owned());
    }
    Ok(())
}

fn run_matches_dispatch(run: &SimulationRun, snapshot: &AutomationDispatchSnapshot) -> bool {
    run.prepared_receipt().is_some_and(|receipt| {
        receipt.source_domain() == AnalysisResultSourceDomain::SimulationPlan
            && receipt.simulation_plan_id() == Some(snapshot.plan_id)
            && receipt.project_revision() == snapshot.project_revision
            && receipt.prepared_snapshot_digest() == snapshot.prepared_snapshot_digest
            && receipt.source_content_digest() == snapshot.source_content_digest
            && !receipt.tasks().is_empty()
            && receipt
                .tasks()
                .iter()
                .all(|task| task.source_revision() == snapshot.plan_revision)
    })
}

fn dispatch_snapshot_is_current(
    app: &RSpiceApp,
    snapshot: &AutomationDispatchSnapshot,
) -> Result<(), String> {
    if app.state.workspace.project.id() != snapshot.project_id
        || app.state.workspace.project.revision() != snapshot.project_revision
    {
        return Err(
            "The project identity or revision changed while Automation execution was in flight; evidence publication was refused."
                .to_owned(),
        );
    }
    let current_plan = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .ok_or_else(|| "The dispatched simulation plan is no longer active.".to_owned())?;
    if current_plan.id() != snapshot.plan_id || current_plan.revision() != snapshot.plan_revision {
        return Err(
            "The simulation plan identity or revision changed after Automation dispatch; evidence publication was refused."
                .to_owned(),
        );
    }
    if app.state.sim_setup.active_plan_name().as_str() != snapshot.plan_name {
        return Err(
            "The active simulation plan name changed after Automation dispatch; evidence publication was refused."
                .to_owned(),
        );
    }
    if app.state.workspace.active_plan_data(snapshot.plan_id) != Some(&snapshot.plan_payload) {
        return Err(
            "The governed plan payload, specifications, baseline selection, or tolerance policy changed after Automation dispatch; evidence publication was refused."
                .to_owned(),
        );
    }
    if app.state.workspace.project_sources != snapshot.project_sources {
        return Err(
            "A project-owned source document or validation identity changed after Automation dispatch; evidence publication was refused."
                .to_owned(),
        );
    }
    app.simulation_controller
        .ensure_run_set_snapshot_current(
            &app.state,
            snapshot.prepared_snapshot_digest,
            snapshot.source_content_digest,
        )
        .map_err(|error| {
            format!(
                "The prepared simulation contract changed after Automation dispatch; evidence publication was refused: {error}"
            )
        })?;
    let current_baseline = app
        .state
        .simulation
        .run_by_stable_id(snapshot.baseline_run.run_id)
        .ok_or_else(|| "The dispatched baseline run is no longer retained.".to_owned())?;
    let current_digest = simulation_run_digest(current_baseline)?;
    if current_digest != snapshot.baseline_digest {
        return Err(
            "The retained baseline dataset changed after Automation dispatch; evidence publication was refused."
                .to_owned(),
        );
    }
    Ok(())
}

fn validate_regression_policy(
    rules: &[crate::state::RegressionToleranceRule],
) -> Result<(), String> {
    for rule in rules {
        rule.validate()?;
    }
    let waveform_rules = rules
        .iter()
        .filter(|rule| rule.target.kind == crate::state::RegressionTargetKind::Waveform)
        .collect::<Vec<_>>();
    if waveform_rules.is_empty() {
        return Err(
            "waveform comparison requires at least one persisted waveform tolerance rule"
                .to_owned(),
        );
    }
    for (index, rule) in waveform_rules.iter().enumerate() {
        if waveform_rules[..index].iter().any(|prior| {
            prior.target.source_domain == rule.target.source_domain
                && prior.target.source_instance_id == rule.target.source_instance_id
                && prior.target.kind == rule.target.kind
                && prior.target.name.eq_ignore_ascii_case(&rule.target.name)
                && prior.target.occurrence == rule.target.occurrence
        }) {
            return Err(format!(
                "duplicate persisted waveform tolerance target {:?}[{}] for analysis {}",
                rule.target.name, rule.target.occurrence, rule.target.source_instance_id
            ));
        }
    }
    Ok(())
}

fn build_spec_evidence(
    specs: &[SpecEntry],
    run: &SimulationRun,
) -> Result<Vec<CheckEvidence>, String> {
    let required = specs
        .len()
        .checked_mul(run.analyses.len())
        .ok_or_else(|| "specification coverage count overflowed".to_owned())?;
    if required == 0 {
        return Err("specification coverage requires at least one task and one spec".to_owned());
    }
    let mut checks = Vec::with_capacity(required);
    for (task_index, analysis) in run.analyses.iter().enumerate() {
        let provenance = analysis.provenance.as_ref().ok_or_else(|| {
            format!(
                "analysis {:?} has no authenticated source identity",
                analysis.label
            )
        })?;
        for (spec_index, spec) in specs.iter().enumerate() {
            let matches = analysis
                .measurements
                .iter()
                .filter(|measurement| measurement.name.eq_ignore_ascii_case(&spec.measurement))
                .collect::<Vec<_>>();
            let (passed, detail) = match matches.as_slice() {
                [] => (
                    false,
                    format!(
                        "Task {} ({}) retained no measurement named {:?}; required {}.",
                        task_index + 1,
                        provenance.source_instance_id(),
                        spec.measurement,
                        specification_limits(spec)
                    ),
                ),
                [measurement] => {
                    let value = measurement.value.filter(|value| value.is_finite());
                    let passed = analysis.success
                        && measurement.passed
                        && measurement.error.is_none()
                        && value.is_some_and(|value| spec.passes(value));
                    let detail = value.map_or_else(
                        || {
                            format!(
                                "Task {} ({}) retained no verified finite value for {:?}: {}",
                                task_index + 1,
                                provenance.source_instance_id(),
                                spec.measurement,
                                measurement
                                    .error
                                    .as_deref()
                                    .unwrap_or("measurement result was incomplete")
                            )
                        },
                        |value| {
                            let status = measurement.error.as_deref().map_or_else(
                                || {
                                    if measurement.passed {
                                        "measurement evaluation passed".to_owned()
                                    } else {
                                        "measurement evaluation failed its declared goal".to_owned()
                                    }
                                },
                                |error| format!("measurement error: {error}"),
                            );
                            format!(
                                "Task {} ({}) measured {value:.12e}; required {}; {status}.",
                                task_index + 1,
                                provenance.source_instance_id(),
                                specification_limits(spec)
                            )
                        },
                    );
                    (passed, detail)
                }
                duplicates => (
                    false,
                    format!(
                        "Task {} ({}) retained {} ambiguous measurements named {:?}.",
                        task_index + 1,
                        provenance.source_instance_id(),
                        duplicates.len(),
                        spec.measurement
                    ),
                ),
            };
            checks.push(
                CheckEvidence::try_new(
                    format!(
                        "release-spec-{:05}-task-{}",
                        spec_index + 1,
                        provenance.source_instance_id()
                    ),
                    format!("{} / {}", analysis.label, spec.measurement),
                    if passed {
                        CheckOutcome::Passed
                    } else {
                        CheckOutcome::Failed
                    },
                    0,
                    detail,
                )
                .map_err(|error| format!("invalid task/spec evidence: {error}"))?,
            );
        }
    }
    Ok(checks)
}

fn specification_limits(spec: &SpecEntry) -> String {
    match (spec.min, spec.max) {
        (Some(min), Some(max)) => format!("{min:.12e} to {max:.12e} {}", spec.unit),
        (Some(min), None) => format!(">= {min:.12e} {}", spec.unit),
        (None, Some(max)) => format!("<= {max:.12e} {}", spec.unit),
        (None, None) => "a tracked finite value".to_owned(),
    }
}

fn simulation_run_digest(run: &SimulationRun) -> Result<ContentDigest, String> {
    validate_run_shape(run)?;
    let receipt = run
        .prepared_receipt()
        .expect("run shape requires a prepared receipt");
    let mut digest = Sha256::new();
    digest.update(b"rspice-automation-run-evidence-v1\0");
    digest.update(run.run_id.as_uuid().as_bytes());
    digest.update(run.dataset_id.as_uuid().as_bytes());
    hash_u64(&mut digest, run.id);
    hash_f64(&mut digest, run.timestamp);
    hash_f64(&mut digest, run.elapsed_time);
    digest.update([u8::from(run.success)]);
    digest.update([match receipt.source_domain() {
        AnalysisResultSourceDomain::SimulationPlan => 0,
        AnalysisResultSourceDomain::ManualDeck => 1,
        AnalysisResultSourceDomain::LegacyUnclassified => 2,
    }]);
    match receipt.simulation_plan_id() {
        Some(plan_id) => {
            digest.update([1]);
            digest.update(plan_id.as_uuid().as_bytes());
        }
        None => digest.update([0]),
    }
    hash_u64(&mut digest, receipt.project_revision().get());
    digest.update(receipt.prepared_snapshot_digest().as_bytes());
    digest.update(receipt.source_content_digest().as_bytes());
    digest.update(receipt.source_check_receipt().digest().as_bytes());
    hash_u64(&mut digest, receipt.tasks().len() as u64);
    for task in receipt.tasks() {
        digest.update(task.instance_id().as_uuid().as_bytes());
        hash_u64(&mut digest, task.source_revision().get());
        digest.update([task.analysis_kind_tag()]);
        digest.update(task.config_digest().as_bytes());
        hash_u64(&mut digest, task.dependencies().len() as u64);
        for dependency in task.dependencies() {
            digest.update(dependency.as_uuid().as_bytes());
        }
    }
    hash_u64(&mut digest, run.analyses.len() as u64);
    for analysis in &run.analyses {
        hash_u64(&mut digest, analysis.id);
        hash_text(&mut digest, &analysis.label);
        hash_f64(&mut digest, analysis.timestamp);
        digest.update([u8::from(analysis.success)]);
        hash_optional_text(&mut digest, analysis.error_message.as_deref());
        let provenance = analysis
            .provenance
            .as_ref()
            .ok_or_else(|| format!("analysis {:?} has no provenance", analysis.label))?;
        digest.update(provenance.source_instance_id().as_uuid().as_bytes());
        hash_u64(&mut digest, provenance.source_revision().get());
        digest.update(provenance.prepared_snapshot_digest().as_bytes());
        hash_u64(&mut digest, analysis.measurements.len() as u64);
        for measurement in &analysis.measurements {
            hash_text(&mut digest, &measurement.name);
            hash_optional_f64(&mut digest, measurement.value);
            hash_optional_text(&mut digest, measurement.error.as_deref());
            digest.update([u8::from(measurement.passed)]);
            hash_optional_f64(&mut digest, measurement.expected);
            hash_optional_f64(&mut digest, measurement.tolerance);
        }
        hash_u64(&mut digest, analysis.waveforms.len() as u64);
        for waveform in &analysis.waveforms {
            hash_text(&mut digest, &waveform.name);
            hash_float_slice(&mut digest, &waveform.x);
            hash_float_slice(&mut digest, &waveform.y);
            match &waveform.complex {
                Some(complex) => {
                    digest.update([1]);
                    hash_text(&mut digest, &complex.source_name);
                    hash_float_slice(&mut digest, &complex.real);
                    hash_float_slice(&mut digest, &complex.imag);
                }
                None => digest.update([0]),
            }
        }
    }
    Ok(ContentDigest::from_bytes(digest.finalize().into()))
}

fn hash_u64(digest: &mut Sha256, value: u64) {
    digest.update(value.to_le_bytes());
}

fn hash_f64(digest: &mut Sha256, value: f64) {
    digest.update(value.to_bits().to_le_bytes());
}

fn hash_text(digest: &mut Sha256, value: &str) {
    hash_u64(digest, value.len() as u64);
    digest.update(value.as_bytes());
}

fn hash_optional_text(digest: &mut Sha256, value: Option<&str>) {
    match value {
        Some(value) => {
            digest.update([1]);
            hash_text(digest, value);
        }
        None => digest.update([0]),
    }
}

fn hash_optional_f64(digest: &mut Sha256, value: Option<f64>) {
    match value {
        Some(value) => {
            digest.update([1]);
            hash_f64(digest, value);
        }
        None => digest.update([0]),
    }
}

fn hash_float_slice(digest: &mut Sha256, values: &[f64]) {
    hash_u64(digest, values.len() as u64);
    for value in values {
        hash_f64(digest, *value);
    }
}

fn source_token_is_current(app: &RSpiceApp, token: SourceOperationToken) -> bool {
    app.state
        .workspace
        .project_sources
        .get(ProjectSourceLanguage::RSpiceAutomation)
        .is_some_and(|document| {
            app.state.workspace.project.id() == token.project_id
                && document.revision().get() == token.revision
                && document.content_digest() == token.content_digest
        })
}

fn editor_diagnostics(diagnostics: &DiagnosticSet) -> Vec<CodeEditorDiagnostic> {
    diagnostics
        .as_slice()
        .iter()
        .map(|diagnostic| CodeEditorDiagnostic {
            severity: CodeEditorSeverity::Error,
            message: diagnostic.message.clone(),
            detail: diagnostic.code.as_str().to_owned(),
            byte_range: Some(diagnostic.span.start..diagnostic.span.end),
            line: Some(diagnostic.start.line),
            column: Some(diagnostic.start.column),
        })
        .collect()
}

fn fail(app: &mut RSpiceApp, message: &str) {
    app.state.ui.code_workspace.automation.execution = AutomationExecutionState::Failed;
    app.state.ui.code_workspace.automation.artifacts.clear();
    app.state.ui.code_workspace.automation.last_error = Some(message.to_owned());
    app.state
        .push_user_message(ConsoleMessage::error(message.to_owned()));
}

#[cfg(test)]
mod tests {
    use crate::product::{AnalysisInstanceId, ObjectRevision, ProjectId};
    use crate::state::{
        AnalysisResult, AnalysisResultProvenance, AnalysisType, PreparedRunReceipt,
        PreparedRunTaskReceipt, PreparedSourceCheckReceipt, ProjectSourceRegistry,
        SimulationPlanPayload, WaveformData,
    };

    use super::*;

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }

    fn result(
        instance_id: AnalysisInstanceId,
        measurement: Option<rspice_core::MeasureResult>,
    ) -> AnalysisResult {
        let provenance = AnalysisResultProvenance::new(
            instance_id,
            ObjectRevision::INITIAL,
            digest(1),
            Vec::new(),
        )
        .expect("valid provenance");
        AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_provenance(provenance)
            .with_measurements(measurement.into_iter().collect())
    }

    fn spec(name: &str) -> SpecEntry {
        SpecEntry {
            measurement: name.to_owned(),
            min: Some(0.5),
            max: Some(1.5),
            unit: "V".to_owned(),
        }
    }

    fn prepared_run(
        plan_id: SimulationPlanId,
        project_revision: ObjectRevision,
        plan_revision: ObjectRevision,
    ) -> SimulationRun {
        let instance_id = AnalysisInstanceId::new();
        let snapshot_digest = digest(0x31);
        let task =
            PreparedRunTaskReceipt::new(instance_id, plan_revision, Vec::new(), 5, digest(0x32))
                .expect("valid task");
        let receipt = PreparedRunReceipt::new(
            AnalysisResultSourceDomain::SimulationPlan,
            Some(plan_id),
            project_revision,
            snapshot_digest,
            digest(0x33),
            PreparedSourceCheckReceipt::SchematicDrc(digest(0x34)),
            vec![task],
        )
        .expect("valid receipt");
        let provenance =
            AnalysisResultProvenance::new(instance_id, plan_revision, snapshot_digest, Vec::new())
                .expect("valid provenance");
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "Transient")
            .with_provenance(provenance)
            .with_measurements(vec![rspice_core::MeasureResult::success("gain", 1.0)])
            .with_waveforms(vec![WaveformData::new(
                "V(out)",
                vec![0.0, 1.0],
                vec![1.0, 2.0],
                "#ffffff",
            )]);
        let mut run = SimulationRun::new_prepared(1, receipt);
        run.add_analysis(analysis);
        run
    }

    #[test]
    fn specification_evidence_covers_every_task_and_spec() {
        let mut run = SimulationRun::new(1);
        run.add_analysis(result(
            AnalysisInstanceId::new(),
            Some(rspice_core::MeasureResult::success("gain", 1.0)),
        ));
        run.add_analysis(result(
            AnalysisInstanceId::new(),
            Some(rspice_core::MeasureResult::success("gain", 1.2)),
        ));

        let checks = build_spec_evidence(&[spec("gain")], &run).expect("spec evidence");
        assert_eq!(checks.len(), 2);
        assert!(
            checks
                .iter()
                .all(|check| check.outcome() == CheckOutcome::Passed)
        );
        assert_ne!(checks[0].id(), checks[1].id());
    }

    #[test]
    fn missing_or_duplicate_task_measurements_are_explicit_failures() {
        let id = AnalysisInstanceId::new();
        let mut missing = SimulationRun::new(1);
        missing.add_analysis(result(id, None));
        let checks = build_spec_evidence(&[spec("gain")], &missing).expect("failure evidence");
        assert_eq!(checks[0].outcome(), CheckOutcome::Failed);
        assert!(checks[0].detail().contains("retained no measurement"));

        let mut duplicate = result(id, Some(rspice_core::MeasureResult::success("gain", 1.0)));
        duplicate
            .measurements
            .push(rspice_core::MeasureResult::success("GAIN", 1.0));
        let mut duplicate_run = SimulationRun::new(2);
        duplicate_run.add_analysis(duplicate);
        let checks =
            build_spec_evidence(&[spec("gain")], &duplicate_run).expect("failure evidence");
        assert_eq!(checks[0].outcome(), CheckOutcome::Failed);
        assert!(checks[0].detail().contains("ambiguous measurements"));
    }

    #[test]
    fn dispatch_matching_requires_exact_project_and_plan_revisions() {
        let plan_id = SimulationPlanId::new();
        let project_revision = ObjectRevision::INITIAL;
        let plan_revision = ObjectRevision::INITIAL;
        let run = prepared_run(plan_id, project_revision, plan_revision);
        let baseline_digest = simulation_run_digest(&run).expect("baseline digest");
        let snapshot = AutomationDispatchSnapshot {
            project_id: ProjectId::new(),
            project_revision,
            plan_id,
            plan_revision,
            prepared_snapshot_digest: run
                .prepared_receipt()
                .expect("prepared run")
                .prepared_snapshot_digest(),
            source_content_digest: run
                .prepared_receipt()
                .expect("prepared run")
                .source_content_digest(),
            plan_name: "Plan".to_owned(),
            plan_payload: SimulationPlanPayload::default(),
            project_sources: ProjectSourceRegistry::default(),
            baseline_run: Arc::new(run.clone()),
            baseline_digest,
        };
        assert!(run_matches_dispatch(&run, &snapshot));

        let mut changed = snapshot.clone();
        changed.plan_revision = ObjectRevision::new(2).expect("revision two");
        assert!(!run_matches_dispatch(&run, &changed));

        let mut changed = snapshot.clone();
        changed.prepared_snapshot_digest = digest(0x91);
        assert!(!run_matches_dispatch(&run, &changed));

        let mut changed = snapshot;
        changed.source_content_digest = digest(0x92);
        assert!(!run_matches_dispatch(&run, &changed));
    }

    #[test]
    fn baseline_digest_changes_when_retained_evidence_changes() {
        let plan_id = SimulationPlanId::new();
        let mut baseline = prepared_run(plan_id, ObjectRevision::INITIAL, ObjectRevision::INITIAL);
        let original = simulation_run_digest(&baseline).expect("baseline digest");
        baseline.analyses[0].waveforms[0].y = Arc::new(vec![1.0, 2.001]);
        let changed = simulation_run_digest(&baseline).expect("changed digest");
        assert_ne!(original, changed);
    }
}

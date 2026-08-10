//! Governed execution bridge for the strict project Automation workflow.

use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::automation_workflow::{
    ArtifactKind, AutomationPlan, AutomationRoleBinding, AutomationSourceDocument,
    AutomationSourceRole, AutomationWorkspaceManifest, CheckEvidence, CheckOutcome,
    CompletedEvidence, DiagnosticSet, compare_governed_waveforms, compile_automation_documents,
    compile_workflow, render_requested_artifacts,
};
use crate::diagnostics::ConsoleMessage;
use crate::product::{ContentDigest, RunId, SimulationPlanId};
use crate::state::{
    AnalysisResultSourceDomain, ProjectSourceBundle, ProjectSourceLanguage, ProjectSourceOwner,
    ProjectSourceRole, SimulationRun, SpecEntry,
};
use crate::workbench::RSpiceApp;
use crate::workbench::workflows::export_workflow::SaveDialogConfig;

use super::page::{AutomationDebugFrame, AutomationDebugValue, AutomationDebugVariableScope};
use super::{
    AutomationArtifactStore, AutomationDebugPhase, AutomationDispatchSnapshot,
    AutomationExecutionState, AutomationRuntimeLaunchMode, AutomationValidationReceipt,
    CodeDiagnosticCollection, CodeEditorDiagnostic, CodeEditorSeverity, SourceOperationToken,
};

pub(crate) const MAX_AUTOMATION_TASKS: usize = 10_000_000;

/// Validate, bind, and dispatch the exact current workflow against the active
/// project plan. Every prerequisite is checked before the ordinary prepared
/// simulation controller receives a run request.
pub fn start_automation_workflow(app: &mut RSpiceApp) {
    if python_automation_requires_managed_worker(app) {
        start_managed_python_workflow(app, AutomationRuntimeLaunchMode::Run);
        return;
    }
    prepare_automation_workflow(app, true);
}

/// Exercise the complete governed preflight without creating a simulation
/// request or consuming execution capacity.
pub fn dry_run_automation_workflow(app: &mut RSpiceApp) {
    if python_automation_requires_managed_worker(app) {
        start_managed_python_workflow(app, AutomationRuntimeLaunchMode::DryRun);
        return;
    }
    prepare_automation_workflow(app, false);
}

fn python_automation_requires_managed_worker(app: &RSpiceApp) -> bool {
    app.state
        .workspace
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::code_workspace(
            ProjectSourceLanguage::RSpiceAutomation,
        ))
        .is_some_and(|bundle| !bundle.root().logical_path().ends_with(".rspice"))
}

/// Invalidate every receipt and live worker session derived from an older
/// Automation closure while retaining only user-authored editor preferences.
pub(crate) fn invalidate_automation_evidence(app: &mut RSpiceApp) {
    let _ = app.automation_runtime.terminate();
    let selected_file = app.state.ui.code_workspace.automation.selected_file.clone();
    let breakpoints = app
        .state
        .ui
        .code_workspace
        .automation
        .debug
        .breakpoints
        .clone();
    app.state.ui.code_workspace.automation = Default::default();
    app.state.ui.code_workspace.automation.selected_file = selected_file;
    app.state.ui.code_workspace.automation.debug.breakpoints = breakpoints;
}

fn start_managed_python_workflow(app: &mut RSpiceApp, mode: AutomationRuntimeLaunchMode) {
    let runtime = &app.state.ui.code_workspace.automation;
    if runtime.runtime_execution.is_some()
        || runtime.pending_runtime_validation.is_some()
        || runtime.execution.is_active()
    {
        return;
    }
    let owner = ProjectSourceOwner::code_workspace(ProjectSourceLanguage::RSpiceAutomation);
    let Some(bundle) = app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&owner)
        .cloned()
    else {
        fail(app, "This project has no Automation source workspace.");
        return;
    };
    let token = SourceOperationToken {
        project_id: app.state.workspace.project.id(),
        revision: bundle.revision().get(),
        content_digest: bundle.closure_digest(),
    };
    let validated = app
        .state
        .ui
        .code_workspace
        .automation
        .receipt
        .as_ref()
        .is_some_and(|receipt| receipt.token == token)
        && app
            .state
            .ui
            .code_workspace
            .automation
            .runtime_snapshot
            .as_ref()
            .is_some_and(|snapshot| runtime_snapshot_matches_token(snapshot, token));
    if !validated {
        app.state
            .ui
            .code_workspace
            .automation
            .deferred_runtime_launch = Some(mode);
        validate_automation_workspace(app);
        return;
    }
    begin_managed_python_execution(app, bundle.id(), token, mode);
}

/// Cancel the exact Automation dispatch owned by this workspace. A queued
/// request is withdrawn before controller pickup; an active simulation uses
/// its identity-bound abort contract.
pub fn cancel_automation_workflow(app: &mut RSpiceApp) {
    if let Some(pending) = app
        .state
        .ui
        .code_workspace
        .automation
        .runtime_execution
        .clone()
    {
        if app.state.simulation.is_running || app.state.simulation.trigger_simulation {
            if let Some(session_id) = pending.session_id
                && let Err(error) = app
                    .automation_runtime
                    .send_request(rspice_automation_protocol::RuntimeRequest::Cancel { session_id })
            {
                runtime_protocol_failure(
                    app,
                    &format!("Could not cancel the managed Python session: {error}"),
                );
                return;
            }
        } else {
            let _ = app.automation_runtime.terminate();
            let runtime = &mut app.state.ui.code_workspace.automation;
            runtime.runtime_execution = None;
            runtime.execution = AutomationExecutionState::Cancelled;
            runtime.cancel_requested = false;
            runtime.artifacts.clear();
            runtime.last_error = None;
            app.state.push_user_message(ConsoleMessage::info(
                "Managed Python execution was cancelled and its isolated worker was terminated.",
            ));
            return;
        }
    }
    match app.state.ui.code_workspace.automation.execution.clone() {
        AutomationExecutionState::AwaitingDispatch { .. }
            if app.state.simulation.trigger_simulation && !app.state.simulation.is_running =>
        {
            app.state.simulation.trigger_simulation = false;
            app.invalidate_simulation_preflight();
            let runtime = &mut app.state.ui.code_workspace.automation;
            runtime.execution = AutomationExecutionState::Cancelled;
            runtime.cancel_requested = false;
            runtime.artifacts.clear();
            runtime.last_error = None;
            app.state.push_user_message(ConsoleMessage::info(
                "Automation dispatch was cancelled before execution started.",
            ));
        }
        AutomationExecutionState::AwaitingDispatch { .. }
        | AutomationExecutionState::Running { .. } => {
            match app.state.simulation.request_abort_active_run() {
                Ok(_) => {
                    app.state.ui.code_workspace.automation.cancel_requested = true;
                    app.state.push_user_message(ConsoleMessage::info(
                        "Cancellation requested for the exact active Automation run.",
                    ));
                }
                Err(error) => fail(
                    app,
                    &format!("Could not cancel the Automation run: {error}"),
                ),
            }
        }
        _ => {}
    }
}

/// Validate the exact configured Automation closure. This is intentionally
/// independent of simulation readiness so authors can resolve source,
/// environment, and permission diagnostics before a design plan is configured.
pub fn validate_automation_workspace(app: &mut RSpiceApp) {
    if app.state.ui.code_workspace.automation.execution.is_active() {
        return;
    }
    let owner = ProjectSourceOwner::code_workspace(ProjectSourceLanguage::RSpiceAutomation);
    let Some(bundle) = app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&owner)
        .cloned()
    else {
        fail(app, "This project has no Automation source workspace.");
        return;
    };
    let token = SourceOperationToken {
        project_id: app.state.workspace.project.id(),
        revision: bundle.revision().get(),
        content_digest: bundle.closure_digest(),
    };
    match compile_automation_bundle(&bundle) {
        Ok((_plan, manifest)) => {
            let runtime_snapshot = match manifest.as_ref() {
                Some(manifest) => {
                    let capabilities = match automation_capabilities(app, manifest) {
                        Ok(capabilities) => capabilities,
                        Err(error) => {
                            fail(
                                app,
                                &format!("Could not seal the Automation capability scope: {error}"),
                            );
                            return;
                        }
                    };
                    match crate::automation_workflow::build_automation_runtime_snapshot(
                        app.state.workspace.project.id(),
                        &bundle,
                        manifest,
                        capabilities,
                    ) {
                        Ok(snapshot) => Some(snapshot),
                        Err(error) => {
                            fail(
                                app,
                                &format!("Could not seal the Automation worker snapshot: {error}"),
                            );
                            return;
                        }
                    }
                }
                None => None,
            };
            let runtime = &mut app.state.ui.code_workspace.automation;
            runtime.receipt = None;
            runtime.manifest = manifest;
            runtime.runtime_snapshot = runtime_snapshot;
            runtime.pending_runtime_validation = None;
            Arc::make_mut(&mut runtime.diagnostics).clear();
            runtime.diagnostic_token = Some(token);
            runtime.last_error = None;
            runtime.execution = AutomationExecutionState::Idle;

            if app.state.ui.code_workspace.automation.manifest.is_some() {
                begin_managed_python_validation(app, bundle.id(), token);
            } else {
                complete_automation_validation(app, bundle.id(), token);
            }
        }
        Err(diagnostics) => publish_compile_failure(app, token, diagnostics),
    }
}

pub fn poll_managed_automation_runtime(app: &mut RSpiceApp) {
    use rspice_automation_protocol::{RuntimeEvent, RuntimeState};

    let events = app.automation_runtime.poll_events();
    for event in events {
        let event = match event {
            Ok(event) => event,
            Err(error) => {
                runtime_protocol_failure(
                    app,
                    &format!("Managed Python worker authentication or transport failed: {error}"),
                );
                continue;
            }
        };
        let validation = app
            .state
            .ui
            .code_workspace
            .automation
            .pending_runtime_validation
            .clone()
            .filter(|pending| event.request_id == Some(pending.request_id));
        let execution = app
            .state
            .ui
            .code_workspace
            .automation
            .runtime_execution
            .clone()
            .filter(|pending| {
                event.request_id == Some(pending.request_id)
                    || (pending.mode.is_debug()
                        && pending.session_id.is_some()
                        && pending.session_id == event.session_id)
            });
        if event.request_id.is_some() && validation.is_none() && execution.is_none() {
            continue;
        }
        if let Some(execution) = execution.as_ref()
            && let Some(session_id) = event.session_id
        {
            let current = app
                .state
                .ui
                .code_workspace
                .automation
                .runtime_execution
                .as_ref()
                .and_then(|pending| pending.session_id);
            if current.is_some_and(|current| current != session_id) {
                runtime_protocol_failure(
                    app,
                    "Managed Python worker changed session identity during one execution request.",
                );
                continue;
            }
            if current.is_none()
                && let Some(pending) = app
                    .state
                    .ui
                    .code_workspace
                    .automation
                    .runtime_execution
                    .as_mut()
                && pending.request_id == execution.request_id
            {
                pending.session_id = Some(session_id);
            }
        }
        let request_id = event.request_id;
        let session_id = event.session_id;
        match event.event {
            RuntimeEvent::Hello { identity } => {
                app.state.push_user_message(ConsoleMessage::info(format!(
                    "Authenticated managed Python {} / RSpice API {}.",
                    identity.python_version, identity.rspice_api_version
                )));
            }
            RuntimeEvent::State { state, detail } => match (state, validation, execution) {
                (RuntimeState::Completed, Some(pending), _) => {
                    // Native resource limits are process-lifetime state, so
                    // always launch execution in a fresh verified process
                    // after validation. Browser Pyodide may reuse this exact
                    // authenticated worker because validation already owns
                    // the same lifetime-stable 2 GiB boundary as execution.
                    #[cfg(not(target_arch = "wasm32"))]
                    let _ = app.automation_runtime.terminate();
                    #[cfg(target_arch = "wasm32")]
                    if app
                        .state
                        .ui
                        .code_workspace
                        .automation
                        .deferred_runtime_launch
                        .is_none()
                    {
                        let _ = app.automation_runtime.terminate();
                    }
                    complete_automation_validation(app, pending.bundle_id, pending.token)
                }
                (RuntimeState::Completed, _, Some(pending)) => {
                    if pending.pending_execute_call_id.is_some() {
                        runtime_protocol_failure(
                            app,
                            "Managed Python completed while a simulator host call was still awaiting its authenticated result.",
                        );
                        continue;
                    }
                    if !source_token_is_current(app, pending.token) {
                        runtime_protocol_failure(
                            app,
                            "Managed Python completed for a stale Automation source revision.",
                        );
                        continue;
                    }
                    let _ = app.automation_runtime.terminate();
                    let runtime = &mut app.state.ui.code_workspace.automation;
                    runtime.runtime_execution = None;
                    runtime.cancel_requested = false;
                    if pending.mode.is_debug() {
                        runtime.execution = AutomationExecutionState::Idle;
                        runtime.debug.phase = AutomationDebugPhase::Stopped;
                        runtime.debug.pause_requested = false;
                        runtime.debug.current_line = None;
                        runtime.debug.selected_frame_id = None;
                        runtime.debug.pending_stack_request_id = None;
                        runtime.debug.pending_variable_requests.clear();
                        runtime.debug.pending_evaluation_requests.clear();
                        let _ = push_debug_output(
                            &mut runtime.debug,
                            "info",
                            "session-completed",
                            "Debuggee completed normally",
                        );
                    } else if matches!(
                        runtime.execution,
                        AutomationExecutionState::PythonBroker { .. }
                    ) {
                        runtime.execution = AutomationExecutionState::PythonComplete {
                            dry_run: pending.mode.is_dry_run(),
                        };
                    }
                    runtime.last_error = None;
                    app.state.push_user_message(ConsoleMessage::info(
                        if pending.mode.is_dry_run() {
                            "Managed Python dry run completed without dispatching a simulation."
                        } else {
                            "Managed Python execution completed."
                        },
                    ));
                }
                (RuntimeState::Cancelled, _, Some(pending)) => {
                    let _ = app.automation_runtime.terminate();
                    let runtime = &mut app.state.ui.code_workspace.automation;
                    runtime.runtime_execution = None;
                    runtime.execution = if pending.mode.is_debug() {
                        AutomationExecutionState::Idle
                    } else {
                        AutomationExecutionState::Cancelled
                    };
                    runtime.cancel_requested = false;
                    runtime.artifacts.clear();
                    runtime.last_error = None;
                    if pending.mode.is_debug() {
                        runtime.debug.phase = AutomationDebugPhase::Stopped;
                        runtime.debug.pause_requested = false;
                        runtime.debug.current_line = None;
                        runtime.debug.selected_frame_id = None;
                        runtime.debug.call_stack.clear();
                        runtime.debug.locals.clear();
                        runtime.debug.globals.clear();
                    }
                }
                (RuntimeState::Failed | RuntimeState::Terminated, Some(_), _) => {
                    let _ = app.automation_runtime.terminate();
                    app.state
                        .ui
                        .code_workspace
                        .automation
                        .pending_runtime_validation = None;
                    fail(app, &format!("Managed Python validation failed: {detail}"));
                }
                (RuntimeState::Failed | RuntimeState::Terminated, _, Some(pending)) => {
                    let _ = app.automation_runtime.terminate();
                    app.state.ui.code_workspace.automation.runtime_execution = None;
                    if pending.mode.is_debug() {
                        debug_fail(app, &format!("Managed Python debugger failed: {detail}"));
                    } else {
                        fail(app, &format!("Managed Python execution failed: {detail}"));
                    }
                }
                (RuntimeState::Running, _, Some(pending)) if pending.mode.is_debug() => {
                    let debug = &mut app.state.ui.code_workspace.automation.debug;
                    debug.phase = AutomationDebugPhase::Running;
                    debug.pause_requested = false;
                }
                (RuntimeState::Paused, _, Some(pending)) if pending.mode.is_debug() => {
                    app.state.ui.code_workspace.automation.debug.phase =
                        AutomationDebugPhase::Paused;
                }
                _ => {}
            },
            RuntimeEvent::Diagnostic { diagnostic } => {
                let editor_diagnostic = runtime_editor_diagnostic(app, &diagnostic);
                let capacity_error =
                    Arc::make_mut(&mut app.state.ui.code_workspace.automation.diagnostics)
                        .try_push(editor_diagnostic)
                        .err();
                if let Some(error) = capacity_error {
                    let _ = app.automation_runtime.terminate();
                    app.state.ui.code_workspace.automation.runtime_execution = None;
                    app.state
                        .ui
                        .code_workspace
                        .automation
                        .pending_runtime_validation = None;
                    fail(app, &error);
                }
            }
            RuntimeEvent::Output {
                channel,
                category,
                text,
            } => {
                let capacity_error = {
                    let debug = &mut app.state.ui.code_workspace.automation.debug;
                    push_debug_output(debug, &channel, &category, &text).err()
                };
                if let Some(error) = capacity_error {
                    let _ = app.automation_runtime.terminate();
                    runtime_protocol_failure(app, &error);
                }
            }
            RuntimeEvent::WorkerFailed {
                code,
                message,
                recoverable: _,
            } => {
                let _ = app.automation_runtime.terminate();
                if validation.is_some() {
                    app.state
                        .ui
                        .code_workspace
                        .automation
                        .pending_runtime_validation = None;
                }
                if execution.is_some() {
                    app.state.ui.code_workspace.automation.runtime_execution = None;
                }
                let rendered =
                    format!("Managed Python worker rejected the operation: {message} ({code})");
                if execution
                    .as_ref()
                    .is_some_and(|pending| pending.mode.is_debug())
                {
                    debug_fail(app, &rendered);
                } else {
                    fail(app, &rendered);
                }
            }
            RuntimeEvent::HostCall { call } => {
                let (Some(request_id), Some(session_id), Some(_)) =
                    (request_id, session_id, execution)
                else {
                    runtime_protocol_failure(
                        app,
                        "Managed Python emitted a host call without an active authenticated execution session.",
                    );
                    continue;
                };
                handle_runtime_host_call(app, request_id, session_id, call);
            }
            RuntimeEvent::Stopped {
                reason,
                description,
                frame_id,
            } => {
                let Some(pending) = execution.filter(|pending| pending.mode.is_debug()) else {
                    runtime_protocol_failure(
                        app,
                        "Managed Python emitted a debugger stop outside an authenticated debug session.",
                    );
                    continue;
                };
                let debug = &mut app.state.ui.code_workspace.automation.debug;
                debug.phase = AutomationDebugPhase::Paused;
                debug.pause_requested = false;
                debug.selected_frame_id = frame_id;
                debug.current_line = None;
                debug.call_stack.clear();
                debug.locals.clear();
                debug.globals.clear();
                let _ = push_debug_output(
                    debug,
                    "info",
                    "stopped",
                    &format!("{reason:?}: {description}"),
                );
                let Some(debug_session_id) = pending.session_id else {
                    runtime_protocol_failure(
                        app,
                        "Managed Python paused without an authenticated debugger session identity.",
                    );
                    continue;
                };
                request_debug_stack(app, debug_session_id);
            }
            RuntimeEvent::Stack { frames, total: _ } => {
                apply_debug_stack(app, request_id, frames);
            }
            RuntimeEvent::Variables { values, total: _ } => {
                apply_debug_variables(app, request_id, values);
            }
            RuntimeEvent::Evaluated { expression, result } => {
                apply_debug_evaluation(app, request_id, &expression, result);
            }
            RuntimeEvent::Terminated { reason, .. }
                if validation.is_some() || execution.is_some() =>
            {
                let runtime = &mut app.state.ui.code_workspace.automation;
                runtime.pending_runtime_validation = None;
                runtime.runtime_execution = None;
                fail(
                    app,
                    &format!("Managed Python worker terminated during an operation: {reason}"),
                );
            }
            _ => {}
        }
    }
}

fn handle_runtime_host_call(
    app: &mut RSpiceApp,
    request_id: u64,
    session_id: uuid::Uuid,
    call: rspice_automation_protocol::HostCall,
) {
    use rspice_automation_protocol::HostOperation;
    use uuid::Uuid;

    let Some(mut pending) = app
        .state
        .ui
        .code_workspace
        .automation
        .runtime_execution
        .clone()
        .filter(|pending| pending.request_id == request_id)
    else {
        runtime_protocol_failure(
            app,
            "A managed Python host call has no owning request ledger.",
        );
        return;
    };
    if pending.session_id != Some(session_id) {
        runtime_protocol_failure(
            app,
            "A managed Python host call has the wrong session identity.",
        );
        return;
    }
    if call.call_id != pending.last_call_id.saturating_add(1) {
        runtime_protocol_failure(
            app,
            "Managed Python host-call identities are not a strict monotonic sequence.",
        );
        return;
    }
    pending.last_call_id = call.call_id;
    let exact_bundle_is_current = app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::code_workspace(
            ProjectSourceLanguage::RSpiceAutomation,
        ))
        .is_some_and(|bundle| bundle.id() == pending.bundle_id);
    if !exact_bundle_is_current || !source_token_is_current(app, pending.token) {
        reject_runtime_host_call(
            app,
            session_id,
            call.call_id,
            "STALE-SOURCE",
            "The project Automation source changed after this Python session was authorized.",
            false,
        );
        return;
    }
    let Some(snapshot) = app
        .state
        .ui
        .code_workspace
        .automation
        .runtime_snapshot
        .clone()
        .filter(|snapshot| runtime_snapshot_matches_token(snapshot, pending.token))
    else {
        runtime_protocol_failure(
            app,
            "The authenticated Python source snapshot is no longer retained.",
        );
        return;
    };
    let Some(grant) = snapshot
        .capabilities
        .iter()
        .find(|grant| grant.capability == call.capability && grant.token == call.capability_token)
    else {
        reject_runtime_host_call(
            app,
            session_id,
            call.call_id,
            "CAPABILITY-DENIED",
            "The host call did not present a capability token from this exact source snapshot.",
            true,
        );
        return;
    };
    let scope_allowed = match (&call.operation, call.capability) {
        (
            HostOperation::OpenProject { .. }
            | HostOperation::LoadRunPlan { .. }
            | HostOperation::ValidateRunPlan { .. }
            | HostOperation::ExecuteRunPlan { .. },
            rspice_automation_protocol::CapabilityKind::ProjectRead
            | rspice_automation_protocol::CapabilityKind::SimulationExecute,
        ) => grant.scope == pending.token.project_id.to_string(),
        (
            HostOperation::EvaluateRequirements { .. } | HostOperation::CompareRun { .. },
            rspice_automation_protocol::CapabilityKind::ResultRead,
        ) => grant.scope == "current-project-results",
        (
            HostOperation::ExportRun { .. },
            rspice_automation_protocol::CapabilityKind::ArtifactWrite,
        ) => grant.scope == "current-project-artifacts",
        (
            HostOperation::ReadEnvironment { name },
            rspice_automation_protocol::CapabilityKind::EnvironmentRead,
        ) => serde_json::from_str::<Vec<String>>(&grant.scope)
            .is_ok_and(|names| names.iter().any(|authorized| authorized == name)),
        _ => false,
    };
    if !scope_allowed {
        reject_runtime_host_call(
            app,
            session_id,
            call.call_id,
            "CAPABILITY-SCOPE-DENIED",
            "The capability token is not scoped to this host operation.",
            true,
        );
        return;
    }

    let manifest = app.state.ui.code_workspace.automation.manifest.clone();
    let mut response = None;
    match call.operation {
        HostOperation::OpenProject { selector } => {
            let project_id = pending.token.project_id.to_string();
            if selector != "." && selector != project_id {
                response = Some(host_failure(
                    "PROJECT-NOT-IN-SNAPSHOT",
                    "Project.open() may address only the current project sealed into this execution snapshot.",
                ));
            } else {
                let handle = pending.project_handle.unwrap_or_else(Uuid::new_v4);
                pending.project_handle = Some(handle);
                response = Some(host_success(Some(handle), "current project opened"));
            }
        }
        HostOperation::LoadRunPlan {
            project_handle,
            document_id,
        } => {
            if pending.project_handle != Some(project_handle) {
                response = Some(host_failure(
                    "INVALID-PROJECT-HANDLE",
                    "The run plan was requested through a project handle not issued by this session.",
                ));
            } else if snapshot.selected_run_plan_document_id != Some(document_id) {
                response = Some(host_failure(
                    "RUN-PLAN-NOT-IN-SNAPSHOT",
                    "The requested document is not the run plan selected in this execution snapshot.",
                ));
            } else {
                let handle = pending.plan_handle.unwrap_or_else(Uuid::new_v4);
                pending.plan_handle = Some(handle);
                response = Some(host_success(Some(handle), "run plan loaded"));
            }
        }
        HostOperation::ValidateRunPlan {
            plan_handle,
            target,
            fail_closed,
        } => {
            let Some(manifest) = manifest.as_ref() else {
                retain_runtime_execution(app, pending);
                send_runtime_host_response(
                    app,
                    session_id,
                    call.call_id,
                    host_failure(
                        "MISSING-WORKSPACE-MANIFEST",
                        "The Python workspace manifest is no longer available.",
                    ),
                );
                return;
            };
            if pending.plan_handle != Some(plan_handle) {
                response = Some(host_failure(
                    "INVALID-RUN-PLAN-HANDLE",
                    "The validation request used a run-plan handle not issued by this session.",
                ));
            } else if target != manifest.target {
                response = Some(host_failure(
                    "TARGET-MISMATCH",
                    "The requested target differs from the target sealed into the selected run plan.",
                ));
            } else if !fail_closed {
                response = Some(host_failure(
                    "FAIL-CLOSED-REQUIRED",
                    "Commercial Automation execution requires fail_closed=True.",
                ));
            } else {
                app.state.ui.code_workspace.automation.last_error = None;
                prepare_automation_workflow(app, false);
                let accepted = app
                    .state
                    .ui
                    .code_workspace
                    .automation
                    .receipt
                    .as_ref()
                    .is_some_and(|receipt| receipt.token == pending.token)
                    && app.state.ui.code_workspace.automation.last_error.is_none();
                if accepted {
                    let handle = pending.preview_handle.unwrap_or_else(Uuid::new_v4);
                    pending.preview_handle = Some(handle);
                    response = Some(host_success(Some(handle), "run-plan preflight passed"));
                } else {
                    let message = app
                        .state
                        .ui
                        .code_workspace
                        .automation
                        .last_error
                        .clone()
                        .unwrap_or_else(|| {
                            "Run-plan preflight did not produce an exact receipt.".to_owned()
                        });
                    response = Some(host_failure("PREFLIGHT-FAILED", message));
                }
            }
        }
        HostOperation::ExecuteRunPlan { preview_handle } => {
            if pending.preview_handle != Some(preview_handle) {
                response = Some(host_failure(
                    "INVALID-PREVIEW-HANDLE",
                    "The execution request used a preview handle not issued by this session.",
                ));
            } else if pending.run_handle.is_some() || pending.pending_execute_call_id.is_some() {
                response = Some(host_failure(
                    "RUN-ALREADY-DISPATCHED",
                    "One Python execution session may dispatch its sealed run plan only once.",
                ));
            } else {
                let run_handle = Uuid::new_v4();
                pending.run_handle = Some(run_handle);
                if pending.mode.is_dry_run() {
                    response = Some(host_success(
                        Some(run_handle),
                        "dry-run execution call validated; no simulation was dispatched",
                    ));
                } else {
                    app.state.ui.code_workspace.automation.last_error = None;
                    prepare_automation_workflow(app, true);
                    if matches!(
                        app.state.ui.code_workspace.automation.execution,
                        AutomationExecutionState::AwaitingDispatch { .. }
                    ) {
                        pending.pending_execute_call_id = Some(call.call_id);
                    } else {
                        let message = app
                            .state
                            .ui
                            .code_workspace
                            .automation
                            .last_error
                            .clone()
                            .unwrap_or_else(|| {
                                "The simulator did not accept the prepared run.".to_owned()
                            });
                        pending.run_handle = None;
                        response = Some(host_failure("DISPATCH-FAILED", message));
                    }
                }
            }
        }
        HostOperation::EvaluateRequirements {
            run_handle,
            profile,
        } => {
            if pending.run_handle != Some(run_handle) {
                response = Some(host_failure(
                    "INVALID-RUN-HANDLE",
                    "Requirement evaluation used a run handle not issued by this session.",
                ));
            } else if manifest
                .as_ref()
                .is_none_or(|manifest| profile != manifest.requirements)
            {
                response = Some(host_failure(
                    "REQUIREMENT-PROFILE-MISMATCH",
                    "The requested requirement profile differs from the selected run plan.",
                ));
            } else if !pending.mode.is_dry_run() && !completed_run_matches_pending(app, &pending) {
                response = Some(host_failure(
                    "RUN-NOT-COMPLETE",
                    "Requirement evidence is unavailable until the exact authenticated simulation completes.",
                ));
            } else {
                pending.requirements_evaluated = true;
                response = Some(host_success(
                    None,
                    if pending.mode.is_dry_run() {
                        "requirement evaluation call validated; no evidence was fabricated"
                    } else {
                        "requirement evidence evaluated"
                    },
                ));
            }
        }
        HostOperation::CompareRun {
            run_handle,
            baseline,
            waveforms,
        } => {
            let comparison_matches = manifest.as_ref().is_some_and(|manifest| {
                baseline == manifest.baseline && waveforms == manifest.compare_waveforms
            });
            if pending.run_handle != Some(run_handle) {
                response = Some(host_failure(
                    "INVALID-RUN-HANDLE",
                    "Baseline comparison used a run handle not issued by this session.",
                ));
            } else if !comparison_matches {
                response = Some(host_failure(
                    "BASELINE-MISMATCH",
                    "The requested comparison differs from the baseline policy sealed into the run plan.",
                ));
            } else if !pending.mode.is_dry_run() && !completed_run_matches_pending(app, &pending) {
                response = Some(host_failure(
                    "RUN-NOT-COMPLETE",
                    "Comparison evidence is unavailable until the exact authenticated simulation completes.",
                ));
            } else {
                pending.comparison_completed = true;
                response = Some(host_success(
                    None,
                    if pending.mode.is_dry_run() {
                        "comparison call validated; no evidence was fabricated"
                    } else {
                        "governed baseline comparison completed"
                    },
                ));
            }
        }
        HostOperation::ExportRun {
            run_handle,
            formats,
        } => {
            let requested = formats
                .iter()
                .map(|format| automation_api_artifact_kind(format))
                .collect::<Option<Vec<_>>>();
            let configured = manifest.as_ref().map(|manifest| {
                manifest
                    .artifacts
                    .iter()
                    .copied()
                    .collect::<std::collections::BTreeSet<_>>()
            });
            if pending.run_handle != Some(run_handle) {
                response = Some(host_failure(
                    "INVALID-RUN-HANDLE",
                    "Artifact publication used a run handle not issued by this session.",
                ));
            } else if requested.as_ref().zip(configured.as_ref()).is_none_or(
                |(requested, configured)| requested.iter().any(|kind| !configured.contains(kind)),
            ) {
                response = Some(host_failure(
                    "ARTIFACT-POLICY-MISMATCH",
                    "One or more requested artifact formats are not authorized by the selected run plan.",
                ));
            } else if !pending.mode.is_dry_run() && !completed_run_matches_pending(app, &pending) {
                response = Some(host_failure(
                    "RUN-NOT-COMPLETE",
                    "Artifacts cannot be published until the exact authenticated simulation completes.",
                ));
            } else if !pending.mode.is_dry_run()
                && requested.as_ref().is_some_and(|requested| {
                    requested.iter().any(|kind| {
                        !app.state
                            .ui
                            .code_workspace
                            .automation
                            .artifacts
                            .iter()
                            .any(|artifact| artifact.kind() == *kind)
                    })
                })
            {
                response = Some(host_failure(
                    "ARTIFACT-NOT-AVAILABLE",
                    "The completed run did not produce every requested governed artifact.",
                ));
            } else {
                pending.artifacts_exported = true;
                response = Some(host_success(
                    None,
                    if pending.mode.is_dry_run() {
                        "artifact publication call validated; no artifact was fabricated or written"
                    } else {
                        "requested governed artifacts are available for publication"
                    },
                ));
            }
        }
        HostOperation::ReadEnvironment { name } => {
            #[cfg(not(target_arch = "wasm32"))]
            {
                response = Some(host_success_value(
                    None,
                    format!("authorized environment variable {name} read"),
                    std::env::var_os(&name).map(|value| value.to_string_lossy().into_owned()),
                ));
            }
            #[cfg(target_arch = "wasm32")]
            {
                let _ = name;
                response = Some(host_failure(
                    "ENVIRONMENT-UNAVAILABLE",
                    "Browser Automation has no ambient operating-system environment. Configure project-owned secrets through a browser-capable credential provider.",
                ));
            }
        }
    }
    retain_runtime_execution(app, pending);
    if let Some(response) = response {
        send_runtime_host_response(app, session_id, call.call_id, response);
    }
}

fn retain_runtime_execution(
    app: &mut RSpiceApp,
    pending: super::page::PendingAutomationRuntimeExecution,
) {
    if app
        .state
        .ui
        .code_workspace
        .automation
        .runtime_execution
        .as_ref()
        .is_some_and(|current| current.request_id == pending.request_id)
    {
        app.state.ui.code_workspace.automation.runtime_execution = Some(pending);
    }
}

fn host_success(
    handle: Option<uuid::Uuid>,
    detail: impl Into<String>,
) -> rspice_automation_protocol::HostResponse {
    rspice_automation_protocol::HostResponse::Success {
        handle,
        detail: detail.into(),
        value: None,
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn host_success_value(
    handle: Option<uuid::Uuid>,
    detail: impl Into<String>,
    value: Option<String>,
) -> rspice_automation_protocol::HostResponse {
    rspice_automation_protocol::HostResponse::Success {
        handle,
        detail: detail.into(),
        value,
    }
}

fn host_failure(
    code: impl Into<String>,
    message: impl Into<String>,
) -> rspice_automation_protocol::HostResponse {
    rspice_automation_protocol::HostResponse::Failure {
        code: code.into(),
        message: message.into(),
        permission_denied: false,
    }
}

fn reject_runtime_host_call(
    app: &mut RSpiceApp,
    session_id: uuid::Uuid,
    call_id: u64,
    code: &str,
    message: &str,
    permission_denied: bool,
) {
    if let Some(pending) = app
        .state
        .ui
        .code_workspace
        .automation
        .runtime_execution
        .as_mut()
    {
        pending.last_call_id = call_id;
    }
    send_runtime_host_response(
        app,
        session_id,
        call_id,
        rspice_automation_protocol::HostResponse::Failure {
            code: code.to_owned(),
            message: message.to_owned(),
            permission_denied,
        },
    );
}

fn send_runtime_host_response(
    app: &mut RSpiceApp,
    session_id: uuid::Uuid,
    call_id: u64,
    response: rspice_automation_protocol::HostResponse,
) {
    if let Err(error) = app.automation_runtime.send_request(
        rspice_automation_protocol::RuntimeRequest::HostResponse {
            session_id,
            call_id,
            response,
        },
    ) {
        runtime_protocol_failure(
            app,
            &format!("Could not return a simulator host-call result to managed Python: {error}"),
        );
    }
}

fn runtime_protocol_failure(app: &mut RSpiceApp, message: &str) {
    let debugging = app
        .state
        .ui
        .code_workspace
        .automation
        .runtime_execution
        .as_ref()
        .is_some_and(|pending| pending.mode.is_debug());
    let _ = app.automation_runtime.terminate();
    let runtime = &mut app.state.ui.code_workspace.automation;
    runtime.pending_runtime_validation = None;
    runtime.deferred_runtime_launch = None;
    runtime.runtime_execution = None;
    if debugging {
        debug_fail(app, message);
    } else {
        fail(app, message);
    }
}

fn completed_run_matches_pending(
    app: &RSpiceApp,
    pending: &super::page::PendingAutomationRuntimeExecution,
) -> bool {
    matches!(
        app.state.ui.code_workspace.automation.execution,
        AutomationExecutionState::Complete { run_id, .. }
            if pending.simulation_run_id == Some(run_id)
    )
}

fn automation_api_artifact_kind(value: &str) -> Option<ArtifactKind> {
    match value {
        "junit" => Some(ArtifactKind::JunitXml),
        "summary-json" => Some(ArtifactKind::SummaryJson),
        "verification-pdf" => Some(ArtifactKind::VerificationPdf),
        _ => None,
    }
}

fn runtime_editor_diagnostic(
    app: &RSpiceApp,
    diagnostic: &rspice_automation_protocol::RuntimeDiagnostic,
) -> CodeEditorDiagnostic {
    let snapshot = app
        .state
        .ui
        .code_workspace
        .automation
        .runtime_snapshot
        .as_ref();
    let document = diagnostic.document_id.and_then(|document_id| {
        snapshot?
            .documents
            .iter()
            .find(|document| document.document_id == document_id)
    });
    let byte_range = diagnostic.range.and_then(|range| {
        let start = usize::try_from(range.start.byte_offset).ok()?;
        let end = usize::try_from(range.end.byte_offset).ok()?;
        let source_len = document?.source.len();
        (start <= end && end <= source_len).then_some(start..end)
    });
    let rendered = CodeEditorDiagnostic::current(
        diagnostic.source.clone(),
        diagnostic.code.clone(),
        match diagnostic.severity {
            rspice_automation_protocol::DiagnosticSeverity::Error => CodeEditorSeverity::Error,
            rspice_automation_protocol::DiagnosticSeverity::Warning => CodeEditorSeverity::Warning,
            rspice_automation_protocol::DiagnosticSeverity::Information => CodeEditorSeverity::Info,
            rspice_automation_protocol::DiagnosticSeverity::Hint => CodeEditorSeverity::Hint,
        },
        diagnostic.message.clone(),
        format!("{} / {}", diagnostic.source, diagnostic.code),
        document.map(|document| document.logical_path.clone()),
        document.map(|document| document.source.clone()),
        byte_range,
        diagnostic
            .range
            .and_then(|range| usize::try_from(range.start.line).ok()),
        diagnostic
            .range
            .and_then(|range| usize::try_from(range.start.column).ok()),
    );
    if let Some(snapshot) = snapshot {
        rendered.bind_validation(
            diagnostic
                .document_id
                .unwrap_or(snapshot.workspace_id)
                .to_string(),
            snapshot.workspace_revision,
            protocol_digest_id(snapshot.closure_digest),
        )
    } else {
        rendered
    }
}

fn protocol_digest_id(digest: rspice_automation_protocol::Digest) -> String {
    use std::fmt::Write as _;
    let mut encoded = String::with_capacity(71);
    encoded.push_str("sha256:");
    for byte in digest.0 {
        let _ = write!(encoded, "{byte:02x}");
    }
    encoded
}

fn complete_automation_validation(
    app: &mut RSpiceApp,
    bundle_id: crate::state::ProjectSourceId,
    token: SourceOperationToken,
) {
    if !source_token_is_current(app, token) {
        fail(
            app,
            "Automation validation completed for a stale source revision; validate the current files again.",
        );
        return;
    }
    if let Err(error) = app
        .state
        .workspace
        .mark_project_source_bundle_validated(bundle_id)
    {
        fail(
            app,
            &format!("Could not retain the workflow validation receipt: {error}"),
        );
        return;
    }
    let runtime = &mut app.state.ui.code_workspace.automation;
    runtime.receipt = Some(AutomationValidationReceipt { token });
    runtime.pending_runtime_validation = None;
    runtime.diagnostic_token = None;
    runtime.last_error = None;
    runtime.execution = AutomationExecutionState::Idle;
    app.state.push_user_message(ConsoleMessage::info(
        "Automation workspace validation passed for the exact current source closure.",
    ));
    let deferred = app
        .state
        .ui
        .code_workspace
        .automation
        .deferred_runtime_launch
        .take();
    if let Some(mode) = deferred {
        begin_managed_python_execution(app, bundle_id, token, mode);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn begin_managed_python_execution(
    app: &mut RSpiceApp,
    bundle_id: crate::state::ProjectSourceId,
    token: SourceOperationToken,
    mode: AutomationRuntimeLaunchMode,
) {
    use rspice_automation_protocol::{ExceptionPolicy, LaunchMode, ResourceLimits, RuntimeRequest};

    if !source_token_is_current(app, token) {
        fail(
            app,
            "The Automation source changed before managed Python could start; validate the current files again.",
        );
        return;
    }
    let Some(snapshot) = app
        .state
        .ui
        .code_workspace
        .automation
        .runtime_snapshot
        .clone()
        .filter(|snapshot| runtime_snapshot_matches_token(snapshot, token))
    else {
        fail(
            app,
            "Managed Python execution has no exact validated source snapshot.",
        );
        return;
    };
    let scratch = match dirs::cache_dir() {
        Some(root) => root
            .join("rspice")
            .join("automation")
            .join("sessions")
            .join(snapshot.workspace_id.to_string()),
        None => {
            fail(
                app,
                "The operating system did not provide an application cache directory for the managed Python worker.",
            );
            return;
        }
    };
    if let Err(error) = app.automation_runtime.ensure_worker(scratch) {
        fail(
            app,
            &format!(
                "Could not start the RSpice-managed Python worker: {error}. RSpice will not use a system Python fallback."
            ),
        );
        return;
    }
    let breakpoints = if mode.is_debug() {
        runtime_breakpoints(app, &snapshot)
    } else {
        Vec::new()
    };
    let exception_policy = if mode.is_debug() {
        runtime_exception_policy(app)
    } else {
        ExceptionPolicy::Uncaught
    };
    let request = RuntimeRequest::Launch {
        mode: match mode {
            AutomationRuntimeLaunchMode::Run => LaunchMode::Run,
            AutomationRuntimeLaunchMode::DryRun => LaunchMode::DryRun,
            AutomationRuntimeLaunchMode::Debug => LaunchMode::Debug,
        },
        snapshot: Box::new(snapshot),
        limits: ResourceLimits {
            wall_time_ms: 24 * 60 * 60 * 1_000,
            cpu_time_ms: 24 * 60 * 60 * 1_000,
            memory_bytes: 2 * 1024 * 1024 * 1024,
            output_bytes: 16 * 1024 * 1024,
            artifact_bytes: 512 * 1024 * 1024,
            max_tasks: 1,
            max_stack_depth: 4_000,
        },
        breakpoints,
        exception_policy,
    };
    match app.automation_runtime.send_request(request) {
        Ok(request_id) => {
            let runtime = &mut app.state.ui.code_workspace.automation;
            runtime.runtime_execution = Some(super::page::PendingAutomationRuntimeExecution {
                request_id,
                bundle_id,
                token,
                mode,
                session_id: None,
                last_call_id: 0,
                project_handle: None,
                plan_handle: None,
                preview_handle: None,
                run_handle: None,
                simulation_run_id: None,
                pending_execute_call_id: None,
                requirements_evaluated: false,
                comparison_completed: false,
                artifacts_exported: false,
            });
            runtime.execution = AutomationExecutionState::PythonBroker {
                token,
                dry_run: mode.is_dry_run(),
            };
            runtime.cancel_requested = false;
            runtime.artifacts.clear();
            runtime.last_error = None;
            if mode.is_debug() {
                let debug = &mut app.state.ui.code_workspace.automation.debug;
                debug.phase = AutomationDebugPhase::Running;
                debug.pause_requested = false;
                debug.current_line = None;
                debug.selected_frame_id = None;
                debug.call_stack.clear();
                debug.locals.clear();
                debug.globals.clear();
                debug.pending_stack_request_id = None;
                debug.pending_variable_requests.clear();
                debug.pending_evaluation_requests.clear();
                debug.last_error = None;
            }
            app.state
                .push_user_message(ConsoleMessage::info(if mode.is_debug() {
                    "Managed Python debugger started against the exact validated source closure."
                } else if mode.is_dry_run() {
                    "Managed Python dry run started against the exact validated source closure."
                } else {
                    "Managed Python workflow started against the exact validated source closure."
                }));
        }
        Err(error) => fail(
            app,
            &format!("Could not submit managed Python execution: {error}"),
        ),
    }
}

#[cfg(target_arch = "wasm32")]
fn begin_managed_python_execution(
    app: &mut RSpiceApp,
    bundle_id: crate::state::ProjectSourceId,
    token: SourceOperationToken,
    mode: AutomationRuntimeLaunchMode,
) {
    use rspice_automation_protocol::{ExceptionPolicy, LaunchMode, ResourceLimits, RuntimeRequest};

    if !source_token_is_current(app, token) {
        fail(
            app,
            "The Automation source changed before managed Python could start; validate the current files again.",
        );
        return;
    }
    let Some(snapshot) = app
        .state
        .ui
        .code_workspace
        .automation
        .runtime_snapshot
        .clone()
        .filter(|snapshot| runtime_snapshot_matches_token(snapshot, token))
    else {
        fail(
            app,
            "Managed Python execution has no exact validated source snapshot.",
        );
        return;
    };
    if let Err(error) = app.automation_runtime.ensure_worker() {
        fail(
            app,
            &format!(
                "Could not start the pinned RSpice Python/WASM worker: {error}. RSpice will not use a browser CDN or system Python fallback."
            ),
        );
        return;
    }
    let breakpoints = if mode.is_debug() {
        runtime_breakpoints(app, &snapshot)
    } else {
        Vec::new()
    };
    let exception_policy = if mode.is_debug() {
        runtime_exception_policy(app)
    } else {
        ExceptionPolicy::Uncaught
    };
    let request = RuntimeRequest::Launch {
        mode: match mode {
            AutomationRuntimeLaunchMode::Run => LaunchMode::Run,
            AutomationRuntimeLaunchMode::DryRun => LaunchMode::DryRun,
            AutomationRuntimeLaunchMode::Debug => LaunchMode::Debug,
        },
        snapshot: Box::new(snapshot),
        limits: ResourceLimits {
            wall_time_ms: 24 * 60 * 60 * 1_000,
            cpu_time_ms: 24 * 60 * 60 * 1_000,
            memory_bytes: 2 * 1024 * 1024 * 1024,
            output_bytes: 16 * 1024 * 1024,
            artifact_bytes: 512 * 1024 * 1024,
            max_tasks: 1,
            max_stack_depth: 4_000,
        },
        breakpoints,
        exception_policy,
    };
    match app.automation_runtime.send_request(request) {
        Ok(request_id) => {
            let runtime = &mut app.state.ui.code_workspace.automation;
            runtime.runtime_execution = Some(super::page::PendingAutomationRuntimeExecution {
                request_id,
                bundle_id,
                token,
                mode,
                session_id: None,
                last_call_id: 0,
                project_handle: None,
                plan_handle: None,
                preview_handle: None,
                run_handle: None,
                simulation_run_id: None,
                pending_execute_call_id: None,
                requirements_evaluated: false,
                comparison_completed: false,
                artifacts_exported: false,
            });
            runtime.execution = AutomationExecutionState::PythonBroker {
                token,
                dry_run: mode.is_dry_run(),
            };
            runtime.cancel_requested = false;
            runtime.artifacts.clear();
            runtime.last_error = None;
            if mode.is_debug() {
                let debug = &mut app.state.ui.code_workspace.automation.debug;
                debug.phase = AutomationDebugPhase::Running;
                debug.pause_requested = false;
                debug.current_line = None;
                debug.selected_frame_id = None;
                debug.call_stack.clear();
                debug.locals.clear();
                debug.globals.clear();
                debug.pending_stack_request_id = None;
                debug.pending_variable_requests.clear();
                debug.pending_evaluation_requests.clear();
                debug.last_error = None;
            }
            app.state.push_user_message(ConsoleMessage::info(if mode.is_debug() {
                "Managed Python/WASM debugger started against the exact validated source closure."
            } else if mode.is_dry_run() {
                "Managed Python/WASM dry run started against the exact validated source closure."
            } else {
                "Managed Python/WASM workflow started against the exact validated source closure."
            }));
        }
        Err(error) => fail(
            app,
            &format!("Could not submit managed Python/WASM execution: {error}"),
        ),
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn begin_managed_python_validation(
    app: &mut RSpiceApp,
    bundle_id: crate::state::ProjectSourceId,
    token: SourceOperationToken,
) {
    use rspice_automation_protocol::{ExceptionPolicy, LaunchMode, ResourceLimits, RuntimeRequest};

    let Some(snapshot) = app
        .state
        .ui
        .code_workspace
        .automation
        .runtime_snapshot
        .clone()
    else {
        fail(
            app,
            "Automation worker validation has no sealed source snapshot.",
        );
        return;
    };
    let scratch = match dirs::cache_dir() {
        Some(root) => root
            .join("rspice")
            .join("automation")
            .join("sessions")
            .join(snapshot.workspace_id.to_string()),
        None => {
            fail(
                app,
                "The operating system did not provide an application cache directory for the managed Python worker.",
            );
            return;
        }
    };
    if let Err(error) = app.automation_runtime.ensure_worker(scratch) {
        fail(
            app,
            &format!("Could not start authoritative managed Python validation: {error}"),
        );
        return;
    }
    let request = RuntimeRequest::Launch {
        mode: LaunchMode::Validate,
        snapshot: Box::new(snapshot),
        limits: ResourceLimits {
            wall_time_ms: 30_000,
            cpu_time_ms: 20_000,
            memory_bytes: 512 * 1024 * 1024,
            output_bytes: 4 * 1024 * 1024,
            artifact_bytes: 1024 * 1024,
            max_tasks: 1,
            max_stack_depth: 2_000,
        },
        breakpoints: Vec::new(),
        exception_policy: ExceptionPolicy::Uncaught,
    };
    match app.automation_runtime.send_request(request) {
        Ok(request_id) => {
            app.state
                .ui
                .code_workspace
                .automation
                .pending_runtime_validation =
                Some(super::page::PendingAutomationRuntimeValidation {
                    request_id,
                    bundle_id,
                    token,
                });
            app.state.push_user_message(ConsoleMessage::info(
                "Managed CPython is validating the exact Automation source closure.",
            ));
        }
        Err(error) => fail(
            app,
            &format!("Could not submit managed Python validation: {error}"),
        ),
    }
}

#[cfg(target_arch = "wasm32")]
fn begin_managed_python_validation(
    app: &mut RSpiceApp,
    bundle_id: crate::state::ProjectSourceId,
    token: SourceOperationToken,
) {
    use rspice_automation_protocol::{ExceptionPolicy, LaunchMode, ResourceLimits, RuntimeRequest};

    let Some(snapshot) = app
        .state
        .ui
        .code_workspace
        .automation
        .runtime_snapshot
        .clone()
    else {
        fail(
            app,
            "Automation worker validation has no sealed source snapshot.",
        );
        return;
    };
    if let Err(error) = app.automation_runtime.ensure_worker() {
        fail(
            app,
            &format!("Could not start authoritative managed Python/WASM validation: {error}"),
        );
        return;
    }
    let request = RuntimeRequest::Launch {
        mode: LaunchMode::Validate,
        snapshot: Box::new(snapshot),
        limits: ResourceLimits {
            wall_time_ms: 30_000,
            cpu_time_ms: 30_000,
            memory_bytes: 2 * 1024 * 1024 * 1024,
            output_bytes: 4 * 1024 * 1024,
            artifact_bytes: 1024 * 1024,
            max_tasks: 1,
            max_stack_depth: 2_000,
        },
        breakpoints: Vec::new(),
        exception_policy: ExceptionPolicy::Uncaught,
    };
    match app.automation_runtime.send_request(request) {
        Ok(request_id) => {
            app.state
                .ui
                .code_workspace
                .automation
                .pending_runtime_validation =
                Some(super::page::PendingAutomationRuntimeValidation {
                    request_id,
                    bundle_id,
                    token,
                });
            app.state.push_user_message(ConsoleMessage::info(
                "Managed Python/WASM is validating the exact Automation source closure.",
            ));
        }
        Err(error) => fail(
            app,
            &format!("Could not submit managed Python/WASM validation: {error}"),
        ),
    }
}

/// Start the governed Python debug adapter and run source setup statements to
/// the first enabled breakpoint. The adapter exposes only values produced by
/// the versioned RSpice API; secret-bearing environment values are never
/// materialized in the debugger model.
pub fn start_automation_debugger(app: &mut RSpiceApp) {
    if app.state.ui.code_workspace.automation.execution.is_active() {
        return;
    }
    if !python_automation_requires_managed_worker(app) {
        debug_fail(
            app,
            "Automation debugging is available for governed Python workspaces.",
        );
        return;
    }
    start_managed_python_workflow(app, AutomationRuntimeLaunchMode::Debug);
}

pub fn continue_automation_debugger(app: &mut RSpiceApp) {
    send_debug_control(app, rspice_automation_protocol::DebugControl::Continue);
}

pub fn step_over_automation_debugger(app: &mut RSpiceApp) {
    send_debug_control(app, rspice_automation_protocol::DebugControl::StepOver);
}

pub fn step_into_automation_debugger(app: &mut RSpiceApp) {
    send_debug_control(app, rspice_automation_protocol::DebugControl::StepIn);
}

pub fn step_out_automation_debugger(app: &mut RSpiceApp) {
    send_debug_control(app, rspice_automation_protocol::DebugControl::StepOut);
}

pub fn restart_automation_debugger(app: &mut RSpiceApp) {
    if active_debug_session(app).is_some() {
        send_debug_control(app, rspice_automation_protocol::DebugControl::Restart);
    } else {
        start_automation_debugger(app);
    }
}

/// Request a debugger pause. Governed simulation dispatch is an atomic host
/// call, so the pause is honored at its next safe completion boundary.
pub fn pause_automation_debugger(app: &mut RSpiceApp) {
    if send_debug_control(app, rspice_automation_protocol::DebugControl::Pause) {
        app.state.ui.code_workspace.automation.debug.pause_requested = true;
    }
}

pub fn stop_automation_debugger(app: &mut RSpiceApp) {
    if active_debug_session(app).is_some() {
        let _ = send_debug_control(app, rspice_automation_protocol::DebugControl::Stop);
        return;
    }
    let debug = &mut app.state.ui.code_workspace.automation.debug;
    debug.phase = AutomationDebugPhase::Stopped;
    debug.current_line = None;
    debug.call_stack.clear();
    debug.locals.clear();
    let _ = push_debug_output(
        debug,
        "info",
        "session-stopped",
        "Debug session stopped by user",
    );
}

pub fn toggle_automation_breakpoint(app: &mut RSpiceApp, line: usize) {
    let Some((document_id, source_path)) = active_automation_python_document(app) else {
        debug_fail(
            app,
            "A breakpoint requires an active project-owned Python source document.",
        );
        return;
    };
    let breakpoints = &mut app.state.ui.code_workspace.automation.debug.breakpoints;
    if let Some(index) = breakpoints
        .iter()
        .position(|breakpoint| breakpoint.document_id == document_id && breakpoint.line == line)
    {
        breakpoints.remove(index);
    } else if line > 0 {
        breakpoints.push(super::AutomationBreakpoint::stop(
            document_id,
            source_path,
            line,
        ));
        breakpoints.sort_by(|left, right| {
            left.source_path
                .cmp(&right.source_path)
                .then(left.line.cmp(&right.line))
        });
    }
    synchronize_runtime_breakpoints(app);
}

pub fn remove_automation_breakpoint(app: &mut RSpiceApp, document_id: uuid::Uuid, line: usize) {
    app.state
        .ui
        .code_workspace
        .automation
        .debug
        .breakpoints
        .retain(|breakpoint| breakpoint.document_id != document_id || breakpoint.line != line);
    synchronize_runtime_breakpoints(app);
}

pub fn select_automation_debug_frame(app: &mut RSpiceApp, frame_id: u64) {
    if app.state.ui.code_workspace.automation.debug.phase != AutomationDebugPhase::Paused {
        return;
    }
    let frame = app
        .state
        .ui
        .code_workspace
        .automation
        .debug
        .call_stack
        .iter()
        .find(|frame| frame.frame_id == frame_id)
        .cloned();
    let Some(frame) = frame else {
        return;
    };
    {
        let debug = &mut app.state.ui.code_workspace.automation.debug;
        debug.selected_frame_id = Some(frame.frame_id);
        debug.current_line = Some(frame.line);
        debug.locals.clear();
        debug.globals.clear();
    }
    request_debug_variables(
        app,
        frame.locals_reference,
        AutomationDebugVariableScope::Locals,
    );
    request_debug_variables(
        app,
        frame.globals_reference,
        AutomationDebugVariableScope::Globals,
    );
    refresh_automation_debugger(app);
}

pub fn refresh_automation_debugger(app: &mut RSpiceApp) {
    if app.state.ui.code_workspace.automation.debug.phase != AutomationDebugPhase::Paused {
        return;
    }
    let Some((session_id, _)) = active_debug_session(app) else {
        debug_fail(
            app,
            "Cannot evaluate watches without an authenticated debug session.",
        );
        return;
    };
    let Some(frame_id) = app
        .state
        .ui
        .code_workspace
        .automation
        .debug
        .selected_frame_id
    else {
        return;
    };
    let expressions = app
        .state
        .ui
        .code_workspace
        .automation
        .debug
        .watches
        .iter()
        .map(|watch| watch.expression.clone())
        .collect::<Vec<_>>();
    for expression in expressions {
        match app.automation_runtime.send_request(
            rspice_automation_protocol::RuntimeRequest::Evaluate {
                session_id,
                frame_id,
                expression: expression.clone(),
            },
        ) {
            Ok(request_id) => {
                app.state
                    .ui
                    .code_workspace
                    .automation
                    .debug
                    .pending_evaluation_requests
                    .insert(request_id, expression);
            }
            Err(error) => {
                debug_fail(
                    app,
                    &format!("Could not evaluate Automation watch: {error}"),
                );
                return;
            }
        }
    }
}

pub fn sync_automation_debugger(app: &mut RSpiceApp) {
    synchronize_runtime_breakpoints(app);
}

fn active_debug_session(app: &RSpiceApp) -> Option<(uuid::Uuid, u64)> {
    app.state
        .ui
        .code_workspace
        .automation
        .runtime_execution
        .as_ref()
        .filter(|pending| pending.mode.is_debug())
        .and_then(|pending| {
            pending
                .session_id
                .map(|session| (session, pending.request_id))
        })
}

fn send_debug_control(
    app: &mut RSpiceApp,
    control: rspice_automation_protocol::DebugControl,
) -> bool {
    let Some((session_id, _)) = active_debug_session(app) else {
        debug_fail(
            app,
            "No authenticated RSpice-managed Python debugger session is connected.",
        );
        return false;
    };
    let label = format!("{control:?}");
    match app.automation_runtime.send_request(
        rspice_automation_protocol::RuntimeRequest::DebugControl {
            session_id,
            control,
        },
    ) {
        Ok(_) => {
            let debug = &mut app.state.ui.code_workspace.automation.debug;
            if !matches!(control, rspice_automation_protocol::DebugControl::Pause) {
                debug.phase = AutomationDebugPhase::Running;
                debug.pause_requested = false;
                debug.current_line = None;
                debug.call_stack.clear();
                debug.locals.clear();
                debug.globals.clear();
                debug.pending_stack_request_id = None;
                debug.pending_variable_requests.clear();
                debug.pending_evaluation_requests.clear();
            }
            let _ = push_debug_output(debug, "info", "debug-control", &label);
            true
        }
        Err(error) => {
            debug_fail(
                app,
                &format!("Could not send debugger control {label}: {error}"),
            );
            false
        }
    }
}

fn active_automation_python_document(app: &RSpiceApp) -> Option<(uuid::Uuid, String)> {
    let bundle = app.state.workspace.project_sources.bundle_for_owner(
        &ProjectSourceOwner::code_workspace(ProjectSourceLanguage::RSpiceAutomation),
    )?;
    let path = app
        .state
        .ui
        .code_workspace
        .automation
        .selected_file
        .as_deref()
        .unwrap_or_else(|| bundle.root().logical_path());
    if !path.to_ascii_lowercase().ends_with(".py") {
        return None;
    }
    let document_id = bundle.document_id(path)?.as_uuid();
    Some((document_id, path.to_owned()))
}

fn runtime_breakpoints(
    app: &RSpiceApp,
    snapshot: &rspice_automation_protocol::SourceSnapshot,
) -> Vec<rspice_automation_protocol::Breakpoint> {
    use rspice_automation_protocol::{Breakpoint, BreakpointKind};

    app.state
        .ui
        .code_workspace
        .automation
        .debug
        .breakpoints
        .iter()
        .filter(|breakpoint| {
            snapshot.documents.iter().any(|document| {
                document.document_id == breakpoint.document_id
                    && document.logical_path == breakpoint.source_path
                    && matches!(
                        document.role,
                        rspice_automation_protocol::DocumentRole::PythonEntry
                            | rspice_automation_protocol::DocumentRole::PythonModule
                    )
            })
        })
        .map(|breakpoint| Breakpoint {
            breakpoint_id: breakpoint.breakpoint_id,
            document_id: breakpoint.document_id,
            line: breakpoint.line as u64,
            column: 1,
            enabled: breakpoint.enabled,
            kind: match breakpoint.kind {
                super::AutomationBreakpointKind::Stop => BreakpointKind::Stop,
                super::AutomationBreakpointKind::Conditional => BreakpointKind::Conditional {
                    expression: breakpoint.expression.clone(),
                },
                super::AutomationBreakpointKind::Logpoint => BreakpointKind::Logpoint {
                    template: breakpoint.expression.clone(),
                },
                super::AutomationBreakpointKind::HitCount => BreakpointKind::HitCount {
                    count: breakpoint.hit_count,
                    condition: (!breakpoint.expression.trim().is_empty())
                        .then(|| breakpoint.expression.clone()),
                },
            },
        })
        .collect()
}

fn runtime_exception_policy(app: &RSpiceApp) -> rspice_automation_protocol::ExceptionPolicy {
    match app
        .state
        .ui
        .code_workspace
        .automation
        .debug
        .exception_policy
    {
        super::AutomationExceptionPolicy::All => rspice_automation_protocol::ExceptionPolicy::All,
        super::AutomationExceptionPolicy::Uncaught => {
            rspice_automation_protocol::ExceptionPolicy::Uncaught
        }
        super::AutomationExceptionPolicy::Never => {
            rspice_automation_protocol::ExceptionPolicy::Never
        }
    }
}

fn synchronize_runtime_breakpoints(app: &mut RSpiceApp) {
    let Some((session_id, _)) = active_debug_session(app) else {
        return;
    };
    let Some(snapshot) = app
        .state
        .ui
        .code_workspace
        .automation
        .runtime_snapshot
        .clone()
    else {
        debug_fail(app, "The debugger source snapshot is no longer available.");
        return;
    };
    let breakpoints = runtime_breakpoints(app, &snapshot);
    if let Err(error) = app.automation_runtime.send_request(
        rspice_automation_protocol::RuntimeRequest::SetBreakpoints {
            session_id,
            breakpoints,
        },
    ) {
        debug_fail(
            app,
            &format!("Could not update debugger breakpoints: {error}"),
        );
    }
}

fn request_debug_stack(app: &mut RSpiceApp, session_id: uuid::Uuid) {
    match app.automation_runtime.send_request(
        rspice_automation_protocol::RuntimeRequest::StackTrace {
            session_id,
            start: 0,
            count: 1_024,
        },
    ) {
        Ok(request_id) => {
            app.state
                .ui
                .code_workspace
                .automation
                .debug
                .pending_stack_request_id = Some(request_id);
        }
        Err(error) => debug_fail(app, &format!("Could not request debugger stack: {error}")),
    }
}

fn apply_debug_stack(
    app: &mut RSpiceApp,
    request_id: Option<u64>,
    frames: Vec<rspice_automation_protocol::StackFrame>,
) {
    let Some(request_id) = request_id else {
        runtime_protocol_failure(app, "Debugger stack response has no request identity.");
        return;
    };
    if app
        .state
        .ui
        .code_workspace
        .automation
        .debug
        .pending_stack_request_id
        != Some(request_id)
    {
        runtime_protocol_failure(app, "Debugger stack response does not match its request.");
        return;
    }
    let paths = app
        .state
        .ui
        .code_workspace
        .automation
        .runtime_snapshot
        .as_ref()
        .map(|snapshot| {
            snapshot
                .documents
                .iter()
                .map(|document| (document.document_id, document.logical_path.clone()))
                .collect::<std::collections::BTreeMap<_, _>>()
        })
        .unwrap_or_default();
    let mapped = frames
        .into_iter()
        .filter_map(|frame| {
            Some(AutomationDebugFrame {
                frame_id: frame.frame_id,
                document_id: frame.document_id,
                name: frame.name,
                source_path: paths.get(&frame.document_id)?.clone(),
                line: usize::try_from(frame.range.start.line).ok()?,
                locals_reference: frame.locals_reference,
                globals_reference: frame.globals_reference,
            })
        })
        .collect::<Vec<_>>();
    let selected = app
        .state
        .ui
        .code_workspace
        .automation
        .debug
        .selected_frame_id
        .and_then(|selected| mapped.iter().find(|frame| frame.frame_id == selected))
        .or_else(|| mapped.first())
        .cloned();
    {
        let debug = &mut app.state.ui.code_workspace.automation.debug;
        debug.pending_stack_request_id = None;
        debug.call_stack = mapped;
        debug.selected_frame_id = selected.as_ref().map(|frame| frame.frame_id);
        debug.current_line = selected.as_ref().map(|frame| frame.line);
    }
    if let Some(frame) = selected {
        request_debug_variables(
            app,
            frame.locals_reference,
            AutomationDebugVariableScope::Locals,
        );
        request_debug_variables(
            app,
            frame.globals_reference,
            AutomationDebugVariableScope::Globals,
        );
        refresh_automation_debugger(app);
    }
}

fn request_debug_variables(
    app: &mut RSpiceApp,
    reference: u64,
    scope: AutomationDebugVariableScope,
) {
    let Some((session_id, _)) = active_debug_session(app) else {
        return;
    };
    match app.automation_runtime.send_request(
        rspice_automation_protocol::RuntimeRequest::Variables {
            session_id,
            reference,
            start: 0,
            count: 10_000,
        },
    ) {
        Ok(request_id) => {
            app.state
                .ui
                .code_workspace
                .automation
                .debug
                .pending_variable_requests
                .insert(request_id, scope);
        }
        Err(error) => debug_fail(
            app,
            &format!("Could not request debugger variables: {error}"),
        ),
    }
}

fn apply_debug_variables(
    app: &mut RSpiceApp,
    request_id: Option<u64>,
    values: Vec<rspice_automation_protocol::Variable>,
) {
    let Some(request_id) = request_id else {
        runtime_protocol_failure(app, "Debugger variable response has no request identity.");
        return;
    };
    let Some(scope) = app
        .state
        .ui
        .code_workspace
        .automation
        .debug
        .pending_variable_requests
        .remove(&request_id)
    else {
        runtime_protocol_failure(
            app,
            "Debugger variable response does not match its request.",
        );
        return;
    };
    let mapped = values
        .into_iter()
        .map(|value| AutomationDebugValue {
            name: value.name,
            type_name: value.type_name,
            display_value: value.display_value,
            redacted: value.redacted,
            variables_reference: value.variables_reference,
        })
        .collect();
    let debug = &mut app.state.ui.code_workspace.automation.debug;
    match scope {
        AutomationDebugVariableScope::Locals => debug.locals = mapped,
        AutomationDebugVariableScope::Globals => debug.globals = mapped,
    }
}

fn apply_debug_evaluation(
    app: &mut RSpiceApp,
    request_id: Option<u64>,
    expression: &str,
    result: rspice_automation_protocol::Variable,
) {
    let Some(request_id) = request_id else {
        runtime_protocol_failure(app, "Debugger evaluation response has no request identity.");
        return;
    };
    let expected = app
        .state
        .ui
        .code_workspace
        .automation
        .debug
        .pending_evaluation_requests
        .remove(&request_id);
    if expected.as_deref() != Some(expression) {
        runtime_protocol_failure(
            app,
            "Debugger evaluation response does not match its request.",
        );
        return;
    }
    if let Some(watch) = app
        .state
        .ui
        .code_workspace
        .automation
        .debug
        .watches
        .iter_mut()
        .find(|watch| watch.expression == expression)
    {
        if result.type_name == "error" {
            watch.value.clear();
            watch.error = Some(result.display_value);
        } else {
            watch.value = if result.redacted {
                "<redacted>".to_owned()
            } else {
                result.display_value
            };
            watch.error = None;
        }
    }
}

fn debug_fail(app: &mut RSpiceApp, message: &str) {
    let debug = &mut app.state.ui.code_workspace.automation.debug;
    debug.phase = AutomationDebugPhase::Failed;
    debug.last_error = Some(message.to_owned());
    let _ = push_debug_output(debug, "error", "adapter-failed", message);
    app.state
        .push_user_message(ConsoleMessage::error(message.to_owned()));
}

fn push_debug_output(
    debug: &mut super::AutomationDebugState,
    level: &str,
    event: &str,
    detail: &str,
) -> Result<u64, String> {
    debug.output.try_push(level, event, detail)
}

fn prepare_automation_workflow(app: &mut RSpiceApp, dispatch: bool) {
    let automation = &app.state.ui.code_workspace.automation;
    let broker_owned = matches!(
        (&automation.runtime_execution, &automation.execution),
        (
            Some(pending),
            AutomationExecutionState::PythonBroker { token, .. }
        ) if pending.token == *token
    );
    if app.state.ui.code_workspace.automation.execution.is_active() && !broker_owned {
        return;
    }
    let Some(bundle) = app
        .state
        .workspace
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::code_workspace(
            ProjectSourceLanguage::RSpiceAutomation,
        ))
        .cloned()
    else {
        fail(app, "This project has no Automation source workspace.");
        return;
    };
    let token = SourceOperationToken {
        project_id: app.state.workspace.project.id(),
        revision: bundle.revision().get(),
        content_digest: bundle.closure_digest(),
    };
    let (plan, manifest) = match compile_automation_bundle(&bundle) {
        Ok(compiled) => compiled,
        Err(diagnostics) => {
            publish_compile_failure(app, token, diagnostics);
            return;
        }
    };
    if let Err(error) = automation_task_count(app) {
        fail(app, &error);
        return;
    }
    let runtime_snapshot = match manifest.as_ref() {
        Some(manifest) => match app
            .state
            .ui
            .code_workspace
            .automation
            .runtime_snapshot
            .clone()
            .filter(|snapshot| runtime_snapshot_matches_token(snapshot, token))
        {
            Some(snapshot) => Some(snapshot),
            None => {
                let capabilities = match automation_capabilities(app, manifest) {
                    Ok(capabilities) => capabilities,
                    Err(error) => {
                        fail(
                            app,
                            &format!("Could not seal the Automation capability scope: {error}"),
                        );
                        return;
                    }
                };
                match crate::automation_workflow::build_automation_runtime_snapshot(
                    app.state.workspace.project.id(),
                    &bundle,
                    manifest,
                    capabilities,
                ) {
                    Ok(snapshot) => Some(snapshot),
                    Err(error) => {
                        fail(
                            app,
                            &format!("Could not seal the Automation worker snapshot: {error}"),
                        );
                        return;
                    }
                }
            }
        },
        None => None,
    };
    let manifest_for_preflight = manifest.clone();
    app.state.ui.code_workspace.automation.manifest = manifest;
    app.state.ui.code_workspace.automation.runtime_snapshot = runtime_snapshot;
    Arc::make_mut(&mut app.state.ui.code_workspace.automation.diagnostics).clear();
    app.state.ui.code_workspace.automation.diagnostic_token = None;

    let active_name = app.state.sim_setup.active_plan_name().as_str().to_owned();
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
    if let Some(manifest) = manifest_for_preflight.as_ref()
        && let Err(error) = validate_requested_analyses(app, &manifest.analyses)
    {
        fail(app, &error);
        return;
    }
    let requirements_enabled = manifest_for_preflight
        .as_ref()
        .is_none_or(|manifest| !manifest.requirements.eq_ignore_ascii_case("none"));
    if requirements_enabled && plan_payload.specs.is_empty() {
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
    let corners_enabled = manifest_for_preflight.as_ref().is_none_or(|manifest| {
        !matches!(
            manifest.corners.to_ascii_lowercase().as_str(),
            "none" | "nominal"
        )
    });
    if corners_enabled && let Err(error) = require_corner_matrix(app) {
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
        .mark_project_source_bundle_validated(bundle.id())
    {
        fail(
            app,
            &format!("Could not retain the workflow validation receipt: {error}"),
        );
        return;
    }

    let Some(plan_revision) = app
        .state
        .sim_setup
        .analysis_plan
        .as_ref()
        .map(crate::simulation::plan::SimulationPlan::revision)
    else {
        fail(
            app,
            "The active simulation plan disappeared before Automation could seal its dispatch revision.",
        );
        return;
    };
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
    if !dispatch {
        let runtime = &mut app.state.ui.code_workspace.automation;
        runtime.receipt = Some(AutomationValidationReceipt { token });
        runtime.artifacts.clear();
        runtime.last_error = None;
        runtime.execution = if broker_owned {
            AutomationExecutionState::PythonBroker {
                token,
                dry_run: runtime
                    .runtime_execution
                    .as_ref()
                    .is_some_and(|execution| execution.mode.is_dry_run()),
            }
        } else {
            AutomationExecutionState::Idle
        };
        app.state.push_user_message(ConsoleMessage::info(
            "Automation dry run passed source, permission, plan, corner, release-gate, and baseline preflight without dispatching a simulation.",
        ));
        return;
    }
    // Automation owns a one-shot prepared permit for precisely this dispatch.
    // Clear any unrelated retained preflight first, then seal the complete
    // current execution contract immediately before publishing the request.
    app.invalidate_simulation_preflight();
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
    app.state.ui.code_workspace.automation.receipt = Some(AutomationValidationReceipt { token });
    app.state.ui.code_workspace.automation.artifacts.clear();
    app.state.ui.code_workspace.automation.cancel_requested = false;
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

pub(crate) fn automation_task_count(app: &RSpiceApp) -> Result<usize, String> {
    use crate::simulation::plan::AnalysisKind;

    let plan = app.state.sim_setup.stable_analysis_plan()?;
    let enabled = plan
        .instances()
        .iter()
        .filter(|instance| instance.enabled())
        .count();
    let corner_enabled = plan
        .instances()
        .iter()
        .any(|instance| instance.enabled() && instance.kind() == AnalysisKind::Corner);
    let pvt_points = if corner_enabled {
        app.state
            .sim_setup
            .corner
            .to_config()
            .map_or(0, |config| config.num_corners())
    } else {
        1
    };
    let tasks = enabled.checked_mul(pvt_points).ok_or_else(|| {
        "Automation task expansion overflowed the platform task counter.".to_owned()
    })?;
    if tasks > MAX_AUTOMATION_TASKS {
        return Err(format!(
            "Automation expands to {tasks} tasks; the supported maximum is {MAX_AUTOMATION_TASKS}."
        ));
    }
    Ok(tasks)
}

fn automation_capabilities(
    app: &RSpiceApp,
    manifest: &AutomationWorkspaceManifest,
) -> Result<Vec<rspice_automation_protocol::CapabilityGrant>, String> {
    use rspice_automation_protocol::{CapabilityGrant, CapabilityKind};
    use uuid::Uuid;

    let project_scope = app.state.workspace.project.id().to_string();
    let mut grants = Vec::new();
    if manifest.project_files.eq_ignore_ascii_case("read") {
        grants.push(CapabilityGrant {
            capability: CapabilityKind::ProjectRead,
            scope: project_scope.clone(),
            token: Uuid::new_v4(),
        });
    }
    grants.push(CapabilityGrant {
        capability: CapabilityKind::SimulationExecute,
        scope: project_scope,
        token: Uuid::new_v4(),
    });
    grants.push(CapabilityGrant {
        capability: CapabilityKind::ResultRead,
        scope: "current-project-results".to_owned(),
        token: Uuid::new_v4(),
    });
    if manifest.artifact_directory.eq_ignore_ascii_case("write") {
        grants.push(CapabilityGrant {
            capability: CapabilityKind::ArtifactWrite,
            scope: "current-project-artifacts".to_owned(),
            token: Uuid::new_v4(),
        });
    }
    if !manifest.environment.is_empty() {
        let scope = serde_json::to_string(&manifest.environment).map_err(|error| {
            format!("could not seal the governed environment-variable scope: {error}")
        })?;
        grants.push(CapabilityGrant {
            capability: CapabilityKind::EnvironmentRead,
            scope,
            token: Uuid::new_v4(),
        });
    }
    Ok(grants)
}

/// Compile the complete persisted Automation closure. Newly created projects
/// use the versioned Python/YAML/TOML contract. A single legacy `.rspice`
/// document remains loadable so existing projects can be migrated without
/// silently rewriting user source.
fn compile_automation_bundle(
    bundle: &ProjectSourceBundle,
) -> Result<(AutomationPlan, Option<AutomationWorkspaceManifest>), Vec<CodeEditorDiagnostic>> {
    if bundle.root().logical_path().ends_with(".rspice") {
        return compile_workflow(bundle.root().content())
            .map(|plan| (plan, None))
            .map_err(|diagnostics| editor_diagnostics(&diagnostics));
    }

    let mut documents = vec![AutomationSourceDocument {
        path: bundle.root().logical_path(),
        source: bundle.root().content(),
    }];
    documents.extend(bundle.files().iter().map(|file| AutomationSourceDocument {
        path: file.logical_path(),
        source: file.content(),
    }));
    let roles = bundle
        .roles()
        .iter()
        .filter_map(|binding| {
            let role = match binding.role() {
                ProjectSourceRole::VerilogABuildProfile | ProjectSourceRole::AutomationEntry => {
                    return None;
                }
                ProjectSourceRole::AutomationRunPlan => AutomationSourceRole::RunPlan,
                ProjectSourceRole::AutomationEnvironmentLock => {
                    AutomationSourceRole::EnvironmentLock
                }
                ProjectSourceRole::AutomationPermissionManifest => {
                    AutomationSourceRole::PermissionManifest
                }
            };
            Some(AutomationRoleBinding {
                path: binding.logical_path(),
                role,
            })
        })
        .collect::<Vec<_>>();
    let (plan, manifest) =
        compile_automation_documents(bundle.root().logical_path(), &documents, &roles).map_err(
            |diagnostics| {
                diagnostics
                    .into_iter()
                    .map(|diagnostic| {
                        CodeEditorDiagnostic::current(
                            "rspice.automation.workspace",
                            diagnostic.code,
                            CodeEditorSeverity::Error,
                            diagnostic.message,
                            "Automation workspace validation",
                            Some(diagnostic.path.clone()),
                            bundle.file_content(&diagnostic.path).map(str::to_owned),
                            None,
                            Some(diagnostic.line),
                            Some(diagnostic.column),
                        )
                    })
                    .collect::<Vec<_>>()
            },
        )?;
    let permission_diagnostics = fail_closed_permission_diagnostics(bundle, &manifest);
    if permission_diagnostics.is_empty() {
        Ok((plan, Some(manifest)))
    } else {
        Err(permission_diagnostics)
    }
}

fn fail_closed_permission_diagnostics(
    bundle: &ProjectSourceBundle,
    manifest: &AutomationWorkspaceManifest,
) -> Vec<CodeEditorDiagnostic> {
    let source = bundle.file_content(&manifest.permissions_path);
    [
        (
            "network",
            &manifest.network,
            "RSPAUTO-PERM-NETWORK-SCOPE",
            "Broad network access is not a valid RSpice capability scope. Set network = \"deny\"; network access remains denied at the host boundary.",
        ),
        (
            "process_spawn",
            &manifest.process_spawn,
            "RSPAUTO-PERM-PROCESS-SCOPE",
            "Broad process spawning is not a valid RSpice capability scope. Set process_spawn = \"deny\"; process creation remains denied at the host boundary.",
        ),
    ]
    .into_iter()
    .filter(|(_, disposition, _, _)| disposition.eq_ignore_ascii_case("allow"))
    .map(|(key, _, code, message)| {
        let line = source.and_then(|source| {
            source
                .lines()
                .position(|line| line.trim_start().starts_with(key))
                .map(|line| line + 1)
        });
        CodeEditorDiagnostic::current(
            "rspice.automation.permissions",
            code,
            CodeEditorSeverity::Error,
            message,
            "Automation permissions fail closed",
            Some(manifest.permissions_path.clone()),
            source.map(str::to_owned),
            None,
            line,
            Some(1),
        )
    })
    .collect()
}

fn publish_compile_failure(
    app: &mut RSpiceApp,
    token: SourceOperationToken,
    diagnostics: Vec<CodeEditorDiagnostic>,
) {
    let diagnostics = diagnostics
        .into_iter()
        .map(|diagnostic| {
            let document_id = diagnostic
                .source_path
                .clone()
                .unwrap_or_else(|| token.project_id.to_string().into());
            diagnostic.bind_validation(
                document_id,
                token.revision,
                token.content_digest.to_string(),
            )
        })
        .collect::<Vec<_>>();
    let total = diagnostics.len();
    let mut summary = diagnostics
        .iter()
        .take(20)
        .map(|diagnostic| {
            format!(
                "{}:{}:{}: {} ({})",
                diagnostic.source_path.as_deref().unwrap_or("Automation"),
                diagnostic.line.unwrap_or(1),
                diagnostic.column.unwrap_or(1),
                diagnostic.message,
                diagnostic.detail
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    if total > 20 {
        summary.push_str(&format!("\n... and {} more diagnostics.", total - 20));
    }
    let diagnostics = CodeDiagnosticCollection::try_new(diagnostics).unwrap_or_else(|error| {
        CodeDiagnosticCollection::try_new(vec![
            CodeEditorDiagnostic::current(
                "rspice.diagnostics",
                "DIAGNOSTIC-CAPACITY",
                CodeEditorSeverity::Error,
                "Diagnostic collection exceeded the supported maximum",
                error,
                None,
                None,
                None,
                None,
                None,
            )
            .bind_validation(
                token.project_id.to_string(),
                token.revision,
                token.content_digest.to_string(),
            ),
        ])
        .unwrap_or_default()
    });
    app.state.ui.code_workspace.automation.manifest = None;
    app.state.ui.code_workspace.automation.runtime_snapshot = None;
    app.state
        .ui
        .code_workspace
        .automation
        .pending_runtime_validation = None;
    app.state.ui.code_workspace.automation.diagnostics = Arc::new(diagnostics);
    app.state.ui.code_workspace.automation.diagnostic_token = Some(token);
    fail(app, &summary);
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
            if app.state.ui.code_workspace.automation.cancel_requested {
                let runtime = &mut app.state.ui.code_workspace.automation;
                runtime.execution = AutomationExecutionState::Cancelled;
                runtime.cancel_requested = false;
                runtime.artifacts.clear();
                runtime.last_error = None;
                app.state.push_user_message(ConsoleMessage::info(
                    "Automation workflow cancellation completed; no release artifacts were published.",
                ));
            } else {
                finish_run(app, token, &plan, run_id, &snapshot);
            }
        }
        AutomationExecutionState::Idle
        | AutomationExecutionState::PythonBroker { .. }
        | AutomationExecutionState::PythonComplete { .. }
        | AutomationExecutionState::Running { .. }
        | AutomationExecutionState::Complete { .. }
        | AutomationExecutionState::Cancelled
        | AutomationExecutionState::Failed => {}
    }
}

/// Complete the one host call that is intentionally held while the ordinary
/// simulator controller owns an authenticated run. Python receives a run
/// handle only after exact run correlation and evidence publication succeed.
pub fn settle_pending_python_execute(app: &mut RSpiceApp) {
    let Some(pending) = app
        .state
        .ui
        .code_workspace
        .automation
        .runtime_execution
        .clone()
        .filter(|pending| pending.pending_execute_call_id.is_some())
    else {
        return;
    };
    let (Some(session_id), Some(call_id), Some(run_handle)) = (
        pending.session_id,
        pending.pending_execute_call_id,
        pending.run_handle,
    ) else {
        runtime_protocol_failure(
            app,
            "The pending simulator host call lost its session, call, or run-handle identity.",
        );
        return;
    };
    let response = match app.state.ui.code_workspace.automation.execution {
        AutomationExecutionState::Complete { run_id, .. } => {
            if let Some(current) = app
                .state
                .ui
                .code_workspace
                .automation
                .runtime_execution
                .as_mut()
            {
                current.pending_execute_call_id = None;
                current.simulation_run_id = Some(run_id);
            }
            host_success(
                Some(run_handle),
                "authenticated simulation and evidence completed",
            )
        }
        AutomationExecutionState::Cancelled => {
            if let Some(current) = app
                .state
                .ui
                .code_workspace
                .automation
                .runtime_execution
                .as_mut()
            {
                current.pending_execute_call_id = None;
            }
            host_failure(
                "RUN-CANCELLED",
                "The authenticated simulation was cancelled.",
            )
        }
        AutomationExecutionState::Failed => {
            if let Some(current) = app
                .state
                .ui
                .code_workspace
                .automation
                .runtime_execution
                .as_mut()
            {
                current.pending_execute_call_id = None;
            }
            host_failure(
                "RUN-FAILED",
                app.state
                    .ui
                    .code_workspace
                    .automation
                    .last_error
                    .clone()
                    .unwrap_or_else(|| "The authenticated simulation failed.".to_owned()),
            )
        }
        _ => return,
    };
    send_runtime_host_response(app, session_id, call_id, response);
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
        .get(kind)
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
    app.state.ui.code_workspace.automation.artifacts =
        match AutomationArtifactStore::try_new(artifacts) {
            Ok(artifacts) => artifacts,
            Err(error) => {
                fail(app, &error);
                return;
            }
        };
    app.state.ui.code_workspace.automation.last_error = None;
    app.state.ui.code_workspace.automation.execution =
        AutomationExecutionState::Complete { passed, run_id };
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

fn validate_requested_analyses(app: &RSpiceApp, requested: &[String]) -> Result<(), String> {
    let stable = app.state.sim_setup.stable_analysis_plan()?;
    let enabled = stable
        .instances()
        .iter()
        .filter(|instance| instance.enabled())
        .map(|instance| instance.kind().stable_id())
        .collect::<std::collections::BTreeSet<_>>();
    let missing = requested
        .iter()
        .filter(|analysis| !enabled.contains(analysis.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "The Automation run plan requests analyses that are not enabled in the active immutable simulation plan: {}.",
            missing.join(", ")
        ))
    }
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
        .ok_or_else(|| "baseline run lost its prepared-run authority".to_owned())?;
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
        .ok_or_else(|| "run lost its prepared-run authority before evidence sealing".to_owned())?;
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
        .bundle_for_owner(&ProjectSourceOwner::code_workspace(
            ProjectSourceLanguage::RSpiceAutomation,
        ))
        .is_some_and(|bundle| {
            app.state.workspace.project.id() == token.project_id
                && bundle.revision().get() == token.revision
                && bundle.closure_digest() == token.content_digest
        })
}

fn runtime_snapshot_matches_token(
    snapshot: &rspice_automation_protocol::SourceSnapshot,
    token: SourceOperationToken,
) -> bool {
    snapshot.project_id == token.project_id.as_uuid()
        && snapshot.workspace_revision == token.revision
        && snapshot.closure_digest.0 == *token.content_digest.as_bytes()
}

fn editor_diagnostics(diagnostics: &DiagnosticSet) -> Vec<CodeEditorDiagnostic> {
    diagnostics
        .as_slice()
        .iter()
        .map(|diagnostic| {
            CodeEditorDiagnostic::current(
                "rspice.automation.language-service",
                diagnostic.code.as_str(),
                CodeEditorSeverity::Error,
                diagnostic.message.clone(),
                "Automation source validation",
                None,
                None,
                Some(diagnostic.span.start..diagnostic.span.end),
                Some(diagnostic.start.line),
                Some(diagnostic.start.column),
            )
        })
        .collect()
}

fn fail(app: &mut RSpiceApp, message: &str) {
    app.state.ui.code_workspace.automation.execution = AutomationExecutionState::Failed;
    app.state.ui.code_workspace.automation.cancel_requested = false;
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

    #[test]
    fn source_workspace_runtime_transitions_have_no_production_panic_shortcuts() {
        for (name, source) in [
            ("Automation runtime", include_str!("automation.rs")),
            ("Code workspace state", include_str!("page.rs")),
            ("Verilog-A runtime", include_str!("veriloga.rs")),
            (
                "Netlist document runtime",
                include_str!("../netlist_document.rs"),
            ),
            ("Netlist surface", include_str!("../../surfaces/netlist.rs")),
        ] {
            let production = source
                .split("\n#[cfg(test)]\nmod tests {")
                .next()
                .expect("the production source precedes its test module");
            for forbidden in [
                ".expect(",
                ".unwrap(",
                "panic!(",
                "unreachable!(",
                "todo!(",
                "unimplemented!(",
            ] {
                assert!(
                    !production.contains(forbidden),
                    "{name} contains production panic shortcut {forbidden}"
                );
            }
        }
    }

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
            expression: String::new(),
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

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn environment_authority_is_one_exact_multi_name_scope() {
        let app = RSpiceApp::test_instance();
        let documents = [
            AutomationSourceDocument {
                path: "characterize.py",
                source: crate::automation_workflow::DEFAULT_AUTOMATION_PYTHON,
            },
            AutomationSourceDocument {
                path: "runplan.rspice.yaml",
                source: crate::automation_workflow::DEFAULT_AUTOMATION_RUN_PLAN,
            },
            AutomationSourceDocument {
                path: "requirements.lock",
                source: crate::automation_workflow::DEFAULT_ENVIRONMENT_LOCK,
            },
            AutomationSourceDocument {
                path: "permissions.toml",
                source: crate::automation_workflow::DEFAULT_AUTOMATION_PERMISSIONS,
            },
        ];
        let roles = [
            AutomationRoleBinding {
                path: "runplan.rspice.yaml",
                role: AutomationSourceRole::RunPlan,
            },
            AutomationRoleBinding {
                path: "requirements.lock",
                role: AutomationSourceRole::EnvironmentLock,
            },
            AutomationRoleBinding {
                path: "permissions.toml",
                role: AutomationSourceRole::PermissionManifest,
            },
        ];
        let (_, mut manifest) = compile_automation_documents("characterize.py", &documents, &roles)
            .expect("valid workspace");
        manifest.environment = vec!["RSPICE_ALPHA".to_owned(), "RSPICE_BETA".to_owned()];

        let grants = automation_capabilities(&app, &manifest).expect("capability scope");
        let environment = grants
            .iter()
            .filter(|grant| {
                grant.capability == rspice_automation_protocol::CapabilityKind::EnvironmentRead
            })
            .collect::<Vec<_>>();
        assert_eq!(environment.len(), 1);
        assert_eq!(
            serde_json::from_str::<Vec<String>>(&environment[0].scope).unwrap(),
            manifest.environment
        );
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn unscoped_network_and_process_permissions_fail_closed() {
        let mut app = RSpiceApp::test_instance();
        let owner = ProjectSourceOwner::code_workspace(ProjectSourceLanguage::RSpiceAutomation);
        let bundle = app
            .state
            .workspace
            .project_sources
            .bundle_for_owner(&owner)
            .unwrap();
        let bundle_id = bundle.id();
        let permissions_path = bundle
            .paths_for_role(ProjectSourceRole::AutomationPermissionManifest)
            .next()
            .unwrap()
            .to_owned();
        let permissions = bundle
            .file_content(&permissions_path)
            .unwrap()
            .replace("network = \"deny\"", "network = \"allow\"")
            .replace("process_spawn = \"deny\"", "process_spawn = \"allow\"");
        app.state
            .workspace
            .replace_project_source_bundle_file(bundle_id, &permissions_path, permissions)
            .unwrap();

        let bundle = app
            .state
            .workspace
            .project_sources
            .get_bundle(bundle_id)
            .unwrap();
        let diagnostics = compile_automation_bundle(bundle).unwrap_err();
        assert_eq!(diagnostics.len(), 2);
        assert!(
            diagnostics.iter().any(
                |diagnostic| diagnostic.canonical.code.as_ref() == "RSPAUTO-PERM-NETWORK-SCOPE"
            )
        );
        assert!(
            diagnostics.iter().any(
                |diagnostic| diagnostic.canonical.code.as_ref() == "RSPAUTO-PERM-PROCESS-SCOPE"
            )
        );

        let documents = [
            AutomationSourceDocument {
                path: "characterize.py",
                source: crate::automation_workflow::DEFAULT_AUTOMATION_PYTHON,
            },
            AutomationSourceDocument {
                path: "runplan.rspice.yaml",
                source: crate::automation_workflow::DEFAULT_AUTOMATION_RUN_PLAN,
            },
            AutomationSourceDocument {
                path: "requirements.lock",
                source: crate::automation_workflow::DEFAULT_ENVIRONMENT_LOCK,
            },
            AutomationSourceDocument {
                path: "permissions.toml",
                source: crate::automation_workflow::DEFAULT_AUTOMATION_PERMISSIONS,
            },
        ];
        let roles = [
            AutomationRoleBinding {
                path: "runplan.rspice.yaml",
                role: AutomationSourceRole::RunPlan,
            },
            AutomationRoleBinding {
                path: "requirements.lock",
                role: AutomationSourceRole::EnvironmentLock,
            },
            AutomationRoleBinding {
                path: "permissions.toml",
                role: AutomationSourceRole::PermissionManifest,
            },
        ];
        let (_, mut manifest) = compile_automation_documents("characterize.py", &documents, &roles)
            .expect("valid baseline workspace");
        manifest.network = "allow".to_owned();
        manifest.process_spawn = "allow".to_owned();
        let grants = automation_capabilities(&app, &manifest).expect("capability scope");
        assert!(grants.iter().all(|grant| {
            !matches!(
                grant.capability,
                rspice_automation_protocol::CapabilityKind::Network
                    | rspice_automation_protocol::CapabilityKind::ProcessSpawn
            )
        }));
    }

    #[test]
    fn ambient_external_write_and_clipboard_authorities_are_never_granted() {
        let app = RSpiceApp::test_instance();
        let documents = [
            AutomationSourceDocument {
                path: "characterize.py",
                source: crate::automation_workflow::DEFAULT_AUTOMATION_PYTHON,
            },
            AutomationSourceDocument {
                path: "runplan.rspice.yaml",
                source: crate::automation_workflow::DEFAULT_AUTOMATION_RUN_PLAN,
            },
            AutomationSourceDocument {
                path: "requirements.lock",
                source: crate::automation_workflow::DEFAULT_ENVIRONMENT_LOCK,
            },
            AutomationSourceDocument {
                path: "permissions.toml",
                source: crate::automation_workflow::DEFAULT_AUTOMATION_PERMISSIONS,
            },
        ];
        let roles = [
            AutomationRoleBinding {
                path: "runplan.rspice.yaml",
                role: AutomationSourceRole::RunPlan,
            },
            AutomationRoleBinding {
                path: "requirements.lock",
                role: AutomationSourceRole::EnvironmentLock,
            },
            AutomationRoleBinding {
                path: "permissions.toml",
                role: AutomationSourceRole::PermissionManifest,
            },
        ];
        let (_, manifest) = compile_automation_documents("characterize.py", &documents, &roles)
            .expect("valid governed workspace");
        let grants = automation_capabilities(&app, &manifest).expect("capability scope");

        assert!(grants.iter().all(|grant| {
            !matches!(
                grant.capability,
                rspice_automation_protocol::CapabilityKind::ProjectWrite
                    | rspice_automation_protocol::CapabilityKind::ExternalFileRead
                    | rspice_automation_protocol::CapabilityKind::ExternalFileWrite
                    | rspice_automation_protocol::CapabilityKind::Network
                    | rspice_automation_protocol::CapabilityKind::ProcessSpawn
                    | rspice_automation_protocol::CapabilityKind::ClipboardRead
                    | rspice_automation_protocol::CapabilityKind::ClipboardWrite
            )
        }));
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

//! The managed-runtime host-call protocol.
//!
//! Every handle the script receives is host-issued and request-scoped: the
//! workspace never infers a project, plan, or run identity from a value the
//! script sent back. A call that names a handle this session did not issue is
//! rejected rather than resolved.

use super::debugger::debug_fail;
use super::*;

pub(super) fn handle_runtime_host_call(
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

pub(super) fn retain_runtime_execution(
    app: &mut RSpiceApp,
    pending: page::PendingAutomationRuntimeExecution,
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

pub(super) fn host_success(
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
pub(super) fn host_success_value(
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

pub(super) fn host_failure(
    code: impl Into<String>,
    message: impl Into<String>,
) -> rspice_automation_protocol::HostResponse {
    rspice_automation_protocol::HostResponse::Failure {
        code: code.into(),
        message: message.into(),
        permission_denied: false,
    }
}

pub(super) fn reject_runtime_host_call(
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

pub(super) fn send_runtime_host_response(
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

pub(super) fn runtime_protocol_failure(app: &mut RSpiceApp, message: &str) {
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

pub(super) fn completed_run_matches_pending(
    app: &RSpiceApp,
    pending: &page::PendingAutomationRuntimeExecution,
) -> bool {
    matches!(
        app.state.ui.code_workspace.automation.execution,
        AutomationExecutionState::Complete { run_id, .. }
            if pending.simulation_run_id == Some(run_id)
    )
}

pub(super) fn automation_api_artifact_kind(value: &str) -> Option<ArtifactKind> {
    match value {
        "junit" => Some(ArtifactKind::JunitXml),
        "summary-json" => Some(ArtifactKind::SummaryJson),
        "verification-pdf" => Some(ArtifactKind::VerificationPdf),
        _ => None,
    }
}

pub(super) fn runtime_editor_diagnostic(
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

pub(super) fn protocol_digest_id(digest: rspice_automation_protocol::Digest) -> String {
    use std::fmt::Write as _;
    let mut encoded = String::with_capacity(71);
    encoded.push_str("sha256:");
    for byte in digest.0 {
        let _ = write!(encoded, "{byte:02x}");
    }
    encoded
}

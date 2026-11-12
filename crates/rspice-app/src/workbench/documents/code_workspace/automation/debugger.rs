//! The governed Python debug adapter.
//!
//! The debugger exposes only values produced by the versioned RSpice API;
//! secret-bearing environment values are never materialized in its model.
//! Breakpoints, frames, and evaluations are all request-correlated, so a
//! response that arrives after its session ended is discarded, not applied.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use super::host_calls::runtime_protocol_failure;
use super::*;

/// Start the governed Python debug adapter and run source setup statements to
/// the first enabled breakpoint. The adapter exposes only values produced by
/// the versioned RSpice API; secret-bearing environment values are never
/// materialized in the debugger model.
pub fn start_automation_debugger(parts: &mut AutomationParts<'_>) {
    if parts.code.automation.execution.is_active() {
        return;
    }
    if !python_automation_requires_managed_worker(parts) {
        debug_fail(
            parts,
            "Automation debugging is available for governed Python workspaces.",
        );
        return;
    }
    start_managed_python_workflow(parts, AutomationRuntimeLaunchMode::Debug);
}

pub fn continue_automation_debugger(parts: &mut AutomationParts<'_>) {
    send_debug_control(parts, rspice_automation_protocol::DebugControl::Continue);
}

pub fn step_over_automation_debugger(parts: &mut AutomationParts<'_>) {
    send_debug_control(parts, rspice_automation_protocol::DebugControl::StepOver);
}

pub fn step_into_automation_debugger(parts: &mut AutomationParts<'_>) {
    send_debug_control(parts, rspice_automation_protocol::DebugControl::StepIn);
}

pub fn step_out_automation_debugger(parts: &mut AutomationParts<'_>) {
    send_debug_control(parts, rspice_automation_protocol::DebugControl::StepOut);
}

pub fn restart_automation_debugger(parts: &mut AutomationParts<'_>) {
    if active_debug_session(parts).is_some() {
        send_debug_control(parts, rspice_automation_protocol::DebugControl::Restart);
    } else {
        start_automation_debugger(parts);
    }
}

/// Request a debugger pause. Governed simulation dispatch is an atomic host
/// call, so the pause is honored at its next safe completion boundary.
pub fn pause_automation_debugger(parts: &mut AutomationParts<'_>) {
    if send_debug_control(parts, rspice_automation_protocol::DebugControl::Pause) {
        parts.code.automation.debug.pause_requested = true;
    }
}

pub fn stop_automation_debugger(parts: &mut AutomationParts<'_>) {
    if active_debug_session(parts).is_some() {
        let _ = send_debug_control(parts, rspice_automation_protocol::DebugControl::Stop);
        return;
    }
    let debug = &mut parts.code.automation.debug;
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

pub fn toggle_automation_breakpoint(parts: &mut AutomationParts<'_>, line: usize) {
    let Some((document_id, source_path)) = active_automation_python_document(parts) else {
        debug_fail(
            parts,
            "A breakpoint requires an active project-owned Python source document.",
        );
        return;
    };
    let breakpoints = &mut parts.code.automation.debug.breakpoints;
    if let Some(index) = breakpoints
        .iter()
        .position(|breakpoint| breakpoint.document_id == document_id && breakpoint.line == line)
    {
        breakpoints.remove(index);
    } else if line > 0 {
        breakpoints.push(AutomationBreakpoint::stop(document_id, source_path, line));
        breakpoints.sort_by(|left, right| {
            left.source_path
                .cmp(&right.source_path)
                .then(left.line.cmp(&right.line))
        });
    }
    synchronize_runtime_breakpoints(parts);
}

pub fn remove_automation_breakpoint(
    parts: &mut AutomationParts<'_>,
    document_id: uuid::Uuid,
    line: usize,
) {
    parts
        .code
        .automation
        .debug
        .breakpoints
        .retain(|breakpoint| breakpoint.document_id != document_id || breakpoint.line != line);
    synchronize_runtime_breakpoints(parts);
}

pub fn select_automation_debug_frame(parts: &mut AutomationParts<'_>, frame_id: u64) {
    if parts.code.automation.debug.phase != AutomationDebugPhase::Paused {
        return;
    }
    let frame = parts
        .code
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
        let debug = &mut parts.code.automation.debug;
        debug.selected_frame_id = Some(frame.frame_id);
        debug.current_line = Some(frame.line);
        debug.locals.clear();
        debug.globals.clear();
    }
    request_debug_variables(
        parts,
        frame.locals_reference,
        AutomationDebugVariableScope::Locals,
    );
    request_debug_variables(
        parts,
        frame.globals_reference,
        AutomationDebugVariableScope::Globals,
    );
    refresh_automation_debugger(parts);
}

pub fn refresh_automation_debugger(parts: &mut AutomationParts<'_>) {
    if parts.code.automation.debug.phase != AutomationDebugPhase::Paused {
        return;
    }
    let Some((session_id, _)) = active_debug_session(parts) else {
        debug_fail(parts, "Cannot evaluate watches without a debug session.");
        return;
    };
    let Some(frame_id) = parts.code.automation.debug.selected_frame_id else {
        return;
    };
    let expressions = parts
        .code
        .automation
        .debug
        .watches
        .iter()
        .map(|watch| watch.expression.clone())
        .collect::<Vec<_>>();
    for expression in expressions {
        match parts.handles.automation_runtime.send_request(
            rspice_automation_protocol::RuntimeRequest::Evaluate {
                session_id,
                frame_id,
                expression: expression.clone(),
            },
        ) {
            Ok(request_id) => {
                parts
                    .code
                    .automation
                    .debug
                    .pending_evaluation_requests
                    .insert(request_id, expression);
            }
            Err(error) => {
                debug_fail(
                    parts,
                    &format!("Could not evaluate Automation watch: {error}"),
                );
                return;
            }
        }
    }
}

pub fn sync_automation_debugger(parts: &mut AutomationParts<'_>) {
    synchronize_runtime_breakpoints(parts);
}

pub(super) fn active_debug_session(parts: &AutomationParts<'_>) -> Option<(uuid::Uuid, u64)> {
    parts
        .code
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

pub(super) fn send_debug_control(
    parts: &mut AutomationParts<'_>,
    control: rspice_automation_protocol::DebugControl,
) -> bool {
    let Some((session_id, _)) = active_debug_session(parts) else {
        debug_fail(
            parts,
            "No RSpice-managed Python debugger session is connected.",
        );
        return false;
    };
    let label = match control {
        rspice_automation_protocol::DebugControl::Continue => "Continue",
        rspice_automation_protocol::DebugControl::Pause => "Pause",
        rspice_automation_protocol::DebugControl::StepIn => "Step into",
        rspice_automation_protocol::DebugControl::StepOver => "Step over",
        rspice_automation_protocol::DebugControl::StepOut => "Step out",
        rspice_automation_protocol::DebugControl::Restart => "Restart",
        rspice_automation_protocol::DebugControl::Stop => "Stop",
    };
    match parts.handles.automation_runtime.send_request(
        rspice_automation_protocol::RuntimeRequest::DebugControl {
            session_id,
            control,
        },
    ) {
        Ok(_) => {
            let debug = &mut parts.code.automation.debug;
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
            let _ = push_debug_output(debug, "info", "debug-control", label);
            true
        }
        Err(error) => {
            debug_fail(
                parts,
                &format!("Could not send debugger control {label}: {error}"),
            );
            false
        }
    }
}

pub(super) fn active_automation_python_document(
    parts: &AutomationParts<'_>,
) -> Option<(uuid::Uuid, String)> {
    let bundle = parts
        .model
        .workspace
        .content
        .project_sources
        .bundle_for_owner(&ProjectSourceOwner::code_workspace(
            ProjectSourceLanguage::RSpiceAutomation,
        ))?;
    let path = parts
        .code
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

pub(super) fn runtime_breakpoints(
    parts: &AutomationParts<'_>,
    snapshot: &rspice_automation_protocol::SourceSnapshot,
) -> Vec<rspice_automation_protocol::Breakpoint> {
    use rspice_automation_protocol::{Breakpoint, BreakpointKind};

    parts
        .code
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
                AutomationBreakpointKind::Stop => BreakpointKind::Stop,
                AutomationBreakpointKind::Conditional => BreakpointKind::Conditional {
                    expression: breakpoint.expression.clone(),
                },
                AutomationBreakpointKind::Logpoint => BreakpointKind::Logpoint {
                    template: breakpoint.expression.clone(),
                },
                AutomationBreakpointKind::HitCount => BreakpointKind::HitCount {
                    count: breakpoint.hit_count,
                    condition: (!breakpoint.expression.trim().is_empty())
                        .then(|| breakpoint.expression.clone()),
                },
            },
        })
        .collect()
}

pub(super) fn runtime_exception_policy(
    parts: &AutomationParts<'_>,
) -> rspice_automation_protocol::ExceptionPolicy {
    match parts.code.automation.debug.exception_policy {
        AutomationExceptionPolicy::All => rspice_automation_protocol::ExceptionPolicy::All,
        AutomationExceptionPolicy::Uncaught => {
            rspice_automation_protocol::ExceptionPolicy::Uncaught
        }
        AutomationExceptionPolicy::Never => rspice_automation_protocol::ExceptionPolicy::Never,
    }
}

pub(super) fn synchronize_runtime_breakpoints(parts: &mut AutomationParts<'_>) {
    let Some((session_id, _)) = active_debug_session(parts) else {
        return;
    };
    let Some(snapshot) = parts.code.automation.runtime_snapshot.clone() else {
        debug_fail(
            parts,
            "The debugger source snapshot is no longer available.",
        );
        return;
    };
    let breakpoints = runtime_breakpoints(parts, &snapshot);
    if let Err(error) = parts.handles.automation_runtime.send_request(
        rspice_automation_protocol::RuntimeRequest::SetBreakpoints {
            session_id,
            breakpoints,
        },
    ) {
        debug_fail(
            parts,
            &format!("Could not update debugger breakpoints: {error}"),
        );
    }
}

pub(super) fn request_debug_stack(parts: &mut AutomationParts<'_>, session_id: uuid::Uuid) {
    match parts.handles.automation_runtime.send_request(
        rspice_automation_protocol::RuntimeRequest::StackTrace {
            session_id,
            start: 0,
            count: 1_024,
        },
    ) {
        Ok(request_id) => {
            parts.code.automation.debug.pending_stack_request_id = Some(request_id);
        }
        Err(error) => debug_fail(parts, &format!("Could not request debugger stack: {error}")),
    }
}

pub(super) fn apply_debug_stack(
    parts: &mut AutomationParts<'_>,
    request_id: Option<u64>,
    frames: Vec<rspice_automation_protocol::StackFrame>,
) {
    let Some(request_id) = request_id else {
        runtime_protocol_failure(parts, "Debugger stack response has no request identity.");
        return;
    };
    if parts.code.automation.debug.pending_stack_request_id != Some(request_id) {
        runtime_protocol_failure(parts, "Debugger stack response does not match its request.");
        return;
    }
    let paths = parts
        .code
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
    let selected = parts
        .code
        .automation
        .debug
        .selected_frame_id
        .and_then(|selected| mapped.iter().find(|frame| frame.frame_id == selected))
        .or_else(|| mapped.first())
        .cloned();
    {
        let debug = &mut parts.code.automation.debug;
        debug.pending_stack_request_id = None;
        debug.call_stack = mapped;
        debug.selected_frame_id = selected.as_ref().map(|frame| frame.frame_id);
        debug.current_line = selected.as_ref().map(|frame| frame.line);
    }
    if let Some(frame) = selected {
        request_debug_variables(
            parts,
            frame.locals_reference,
            AutomationDebugVariableScope::Locals,
        );
        request_debug_variables(
            parts,
            frame.globals_reference,
            AutomationDebugVariableScope::Globals,
        );
        refresh_automation_debugger(parts);
    }
}

pub(super) fn request_debug_variables(
    parts: &mut AutomationParts<'_>,
    reference: u64,
    scope: AutomationDebugVariableScope,
) {
    let Some((session_id, _)) = active_debug_session(parts) else {
        return;
    };
    match parts.handles.automation_runtime.send_request(
        rspice_automation_protocol::RuntimeRequest::Variables {
            session_id,
            reference,
            start: 0,
            count: 10_000,
        },
    ) {
        Ok(request_id) => {
            parts
                .code
                .automation
                .debug
                .pending_variable_requests
                .insert(request_id, scope);
        }
        Err(error) => debug_fail(
            parts,
            &format!("Could not request debugger variables: {error}"),
        ),
    }
}

pub(super) fn apply_debug_variables(
    parts: &mut AutomationParts<'_>,
    request_id: Option<u64>,
    values: Vec<rspice_automation_protocol::Variable>,
) {
    let Some(request_id) = request_id else {
        runtime_protocol_failure(parts, "Debugger variable response has no request identity.");
        return;
    };
    let Some(scope) = parts
        .code
        .automation
        .debug
        .pending_variable_requests
        .remove(&request_id)
    else {
        runtime_protocol_failure(
            parts,
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
    let debug = &mut parts.code.automation.debug;
    match scope {
        AutomationDebugVariableScope::Locals => debug.locals = mapped,
        AutomationDebugVariableScope::Globals => debug.globals = mapped,
    }
}

pub(super) fn apply_debug_evaluation(
    parts: &mut AutomationParts<'_>,
    request_id: Option<u64>,
    expression: &str,
    result: rspice_automation_protocol::Variable,
) {
    let Some(request_id) = request_id else {
        runtime_protocol_failure(
            parts,
            "Debugger evaluation response has no request identity.",
        );
        return;
    };
    let expected = parts
        .code
        .automation
        .debug
        .pending_evaluation_requests
        .remove(&request_id);
    if expected.as_deref() != Some(expression) {
        runtime_protocol_failure(
            parts,
            "Debugger evaluation response does not match its request.",
        );
        return;
    }
    if let Some(watch) = parts
        .code
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

pub(super) fn debug_fail(parts: &mut AutomationParts<'_>, message: &str) {
    let debug = &mut parts.code.automation.debug;
    debug.phase = AutomationDebugPhase::Failed;
    debug.last_error = Some(message.to_owned());
    let _ = push_debug_output(debug, "error", "adapter-failed", message);
    parts
        .model
        .record(LogSeverity::Error, LogSource::Studio, message);
}

pub(super) fn push_debug_output(
    debug: &mut AutomationDebugState,
    level: &str,
    event: &str,
    detail: &str,
) -> Result<u64, String> {
    debug.output.try_push(level, event, detail)
}

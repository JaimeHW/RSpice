//! The check a Verilog-A source gets while it is being written.
//!
//! Once the writer leaves the source alone for half a second it is read
//! through semantic analysis, off the editor's thread and short of
//! generating code, and what that finds is the source's findings until it
//! changes again or is compiled. The browser build compiles in a worker of
//! its own and has no checker beside it, so there the source is checked by
//! Compile alone.

use crate::product::ProjectId;

use super::SelectedVerilogASource;
use super::page::VerilogAWorkbenchState;

/// How long the source has to stay as it is before it is checked.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const LIVE_CHECK_PAUSE: std::time::Duration = std::time::Duration::from_millis(500);

/// Drive the check of `selected`, the source the editor shows, on by one
/// frame: take a finished check's findings, note a change, and start a check
/// once the source has stood for [`LIVE_CHECK_PAUSE`]. Nothing is checked
/// while a compile is on its way, or once a compile or a check has spoken
/// for the source as it stands.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn drive_veriloga_live_check(
    ctx: &egui::Context,
    state: &mut VerilogAWorkbenchState,
    project_id: ProjectId,
    selected: &SelectedVerilogASource,
) {
    use std::sync::{Arc, Mutex, mpsc};

    use super::page::RunningVerilogACheck;
    use crate::state::code_diagnostics::CodeDiagnosticCollection;

    if let Some(running) = state.live_check.running.take() {
        let received = running
            .receiver
            .lock()
            .map_or(Err(mpsc::TryRecvError::Disconnected), |receiver| {
                receiver.try_recv()
            });
        match received {
            // Kept with the source it was of, which may have moved on.
            Ok(findings) => {
                let findings = CodeDiagnosticCollection::try_new(findings).unwrap_or_default();
                state.live_check.findings = Some((running.token, Arc::new(findings)));
            }
            Err(mpsc::TryRecvError::Empty) => state.live_check.running = Some(running),
            Err(mpsc::TryRecvError::Disconnected) => {}
        }
    }
    let token = selected.token(project_id);
    if state.pending.is_some()
        || super::veriloga_current_diagnostics(state, project_id, selected).is_some()
    {
        state.live_check.edited = None;
        return;
    }
    let now = ctx.input(|input| input.time);
    let live = &mut state.live_check;
    let since = match live.edited {
        Some((edited, since)) if edited == token => since,
        _ => {
            live.edited = Some((token, now));
            now
        }
    };
    let pause = LIVE_CHECK_PAUSE.as_secs_f64();
    if now - since < pause {
        ctx.request_repaint_after(std::time::Duration::from_secs_f64(pause - (now - since)));
        return;
    }
    // One check at a time: a check of an earlier text answers first, and
    // its answer wakes the next frame.
    if live.running.is_some() {
        return;
    }
    let (sender, receiver) = mpsc::channel();
    let wake = ctx.clone();
    rspice_simulation::project_veriloga::live_check::start_check(
        selected.bundle().clone(),
        selected.selected_module().map(str::to_owned),
        sender,
        move || wake.request_repaint(),
    );
    live.running = Some(RunningVerilogACheck {
        token,
        receiver: Arc::new(Mutex::new(receiver)),
    });
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn drive_veriloga_live_check(
    _ctx: &egui::Context,
    _state: &mut VerilogAWorkbenchState,
    _project_id: ProjectId,
    _selected: &SelectedVerilogASource,
) {
}

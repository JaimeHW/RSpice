//! Process-wide cooperative abort.
//!
//! Ctrl-C and `--timeout` share one flag that the engine polls during long
//! analyses (transient and DC sweeps check it via `run_*_with_abort`). The
//! first interrupt requests a clean stop at the next safe point; a second
//! interrupt force-quits. The recorded reason maps to the conventional exit
//! codes: 130 for an interrupt, 124 for a timeout.
//!
//! The same flag is the completion latch. A run that ends under its own power
//! claims the flag as [`COMPLETE`], which is what stops the detached timeout
//! thread from announcing a cancellation for a run that had already finished:
//! the timer only speaks when its own claim wins, and a claim can only win
//! while a run is still in progress.

use rspice_core::abort_signal::AbortSignal;
use std::sync::atomic::{AtomicU8, Ordering};

const NONE: u8 = 0;
const INTERRUPT: u8 = 1;
const TIMEOUT: u8 = 2;
/// The cancellable region ended without being cancelled. Terminal, like the
/// two abort reasons: nothing may claim the flag afterwards.
const COMPLETE: u8 = 3;

static STATE: AtomicU8 = AtomicU8::new(NONE);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbortReason {
    Interrupt,
    Timeout,
}

/// Claim the shared flag for `value`, reporting whether this caller won.
///
/// The flag is single-assignment: the first claim wins and every later one is
/// refused, so a timeout firing after Ctrl-C cannot change the exit code and
/// neither can announce anything about a run that already completed.
fn claim(state: &AtomicU8, value: u8) -> bool {
    state
        .compare_exchange(NONE, value, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
}

/// Record an abort request. The first reason wins; later requests are
/// ignored so a timeout firing after Ctrl-C cannot change the exit code.
pub fn request(reason: AbortReason) {
    let value = match reason {
        AbortReason::Interrupt => INTERRUPT,
        AbortReason::Timeout => TIMEOUT,
    };
    claim(&STATE, value);
}

/// The recorded abort reason, if any.
pub fn reason() -> Option<AbortReason> {
    match STATE.load(Ordering::SeqCst) {
        INTERRUPT => Some(AbortReason::Interrupt),
        TIMEOUT => Some(AbortReason::Timeout),
        _ => None,
    }
}

/// Abort signal handed to the engine: aborted once a reason is recorded.
///
/// A completed run is not an aborted one, so [`COMPLETE`] reads as "keep
/// going" — anything still running after the latch closes (final artifact
/// bookkeeping, for instance) must not be told to stop.
#[derive(Debug, Clone, Copy, Default)]
pub struct ProcessAbort;

impl AbortSignal for ProcessAbort {
    #[inline]
    fn is_aborted(&self) -> bool {
        matches!(STATE.load(Ordering::Relaxed), INTERRUPT | TIMEOUT)
    }

    fn abort_reason(&self) -> rspice_core::AbortReason {
        process_abort_reason()
    }
}

/// The engine's view of why this process is stopping.
///
/// The engine's inner loops raise a reason-free stop because only the owner of
/// the flag knows what set it. This is that owner, so the reason it records is
/// what re-labels a propagated stop as a cancellation or an expired budget.
/// No recorded reason means nothing asked for a stop, and a stop that arrives
/// anyway is treated as a cancellation.
pub fn process_abort_reason() -> rspice_core::AbortReason {
    match reason() {
        Some(AbortReason::Timeout) => rspice_core::AbortReason::TimeLimit,
        Some(AbortReason::Interrupt) | None => rspice_core::AbortReason::Cancelled,
    }
}

/// Abort signal that also drives a progress bar from the engine's
/// completed-fraction reports.
pub struct ProgressAbort<'a> {
    bar: &'a indicatif::ProgressBar,
}

impl<'a> ProgressAbort<'a> {
    /// `bar` should have a length of [`Self::SCALE`].
    pub const SCALE: u64 = 1000;

    pub fn new(bar: &'a indicatif::ProgressBar) -> Self {
        Self { bar }
    }
}

impl AbortSignal for ProgressAbort<'_> {
    #[inline]
    fn is_aborted(&self) -> bool {
        matches!(STATE.load(Ordering::Relaxed), INTERRUPT | TIMEOUT)
    }

    fn observe_progress(&self, fraction: f64) {
        self.bar
            .set_position((fraction * Self::SCALE as f64) as u64);
    }
}

/// Install the Ctrl-C handler. Safe to call once per process; errors are
/// ignored (the default handler then terminates the process, which is the
/// pre-existing behavior).
#[cfg(not(windows))]
pub fn install_interrupt_handler() {
    let _ = ctrlc::set_handler(|| {
        if reason().is_some() {
            // Second interrupt: the user wants out now.
            std::process::exit(130);
        }
        eprintln!("\nInterrupted — stopping at the next safe point (Ctrl-C again to force quit)");
        request(AbortReason::Interrupt);
    });
}

/// Windows already dispatches console-control callbacks on a system-managed
/// thread. Register directly instead of asking `ctrlc` to allocate a
/// semaphore and permanently spawn another waiter thread for every short CLI
/// invocation.
#[cfg(windows)]
pub fn install_interrupt_handler() {
    use windows_sys::Win32::System::Console::SetConsoleCtrlHandler;

    // SAFETY: `windows_console_control_handler` has the required static
    // `PHANDLER_ROUTINE` ABI and accesses only process-lifetime atomics.
    unsafe {
        let _ = SetConsoleCtrlHandler(Some(windows_console_control_handler), 1);
    }
}

#[cfg(windows)]
unsafe extern "system" fn windows_console_control_handler(control_type: u32) -> i32 {
    use windows_sys::Win32::System::Console::{CTRL_BREAK_EVENT, CTRL_C_EVENT};

    if !matches!(control_type, CTRL_C_EVENT | CTRL_BREAK_EVENT) {
        return 0;
    }
    if reason().is_some() {
        std::process::exit(130);
    }
    request(AbortReason::Interrupt);
    1
}

/// Closes the completion latch when the cancellable region ends.
///
/// Hold one for exactly as long as `--timeout` is allowed to cancel work.
/// Dropping it retires the timer: a run that finishes on its own claims the
/// shared flag, and the detached thread that wakes afterwards finds the claim
/// refused and stays silent.
#[must_use = "the timeout stays armed only while its guard is alive"]
pub struct TimeoutGuard;

impl Drop for TimeoutGuard {
    fn drop(&mut self) {
        claim(&STATE, COMPLETE);
    }
}

/// Arm the run timeout: after `seconds`, long-running analyses stop at the
/// next abort check and the process exits 124.
///
/// The timer thread announces the timeout only when its own claim on the
/// shared flag wins, which can happen only while the returned guard is alive
/// — that is, only when the deadline genuinely interrupted a run in progress.
/// A run that finished first, or one already stopping for Ctrl-C, loses the
/// race deliberately and produces no diagnostic.
pub fn arm_timeout(seconds: f64) -> TimeoutGuard {
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs_f64(seconds));
        if claim(&STATE, TIMEOUT) {
            eprintln!("Timeout: simulation exceeded {seconds}s — stopping at the next safe point");
        }
    });
    TimeoutGuard
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_completed_run_refuses_a_later_timeout_claim() {
        let state = AtomicU8::new(NONE);
        assert!(claim(&state, COMPLETE));
        assert!(
            !claim(&state, TIMEOUT),
            "a timeout firing after the run completed must not be recorded, so the \
             detached timer prints nothing"
        );
        assert_eq!(state.load(Ordering::SeqCst), COMPLETE);
    }

    #[test]
    fn a_timeout_that_wins_refuses_the_completion_latch() {
        let state = AtomicU8::new(NONE);
        assert!(claim(&state, TIMEOUT));
        assert!(
            !claim(&state, COMPLETE),
            "a run cancelled by its deadline must not overwrite the recorded reason"
        );
        assert_eq!(state.load(Ordering::SeqCst), TIMEOUT);
    }

    #[test]
    fn the_first_abort_reason_wins_over_every_later_claim() {
        let state = AtomicU8::new(NONE);
        assert!(claim(&state, INTERRUPT));
        assert!(!claim(&state, TIMEOUT));
        assert!(!claim(&state, COMPLETE));
        assert_eq!(state.load(Ordering::SeqCst), INTERRUPT);
    }
}

//! Process-wide cooperative abort.
//!
//! Ctrl-C and `--timeout` share one flag that the engine polls during long
//! analyses (transient and DC sweeps check it via `run_*_with_abort`). The
//! first interrupt requests a clean stop at the next safe point; a second
//! interrupt force-quits. The recorded reason maps to the conventional exit
//! codes: 130 for an interrupt, 124 for a timeout.

use rspice_core::abort_signal::AbortSignal;
use std::sync::atomic::{AtomicU8, Ordering};

const NONE: u8 = 0;
const INTERRUPT: u8 = 1;
const TIMEOUT: u8 = 2;

static STATE: AtomicU8 = AtomicU8::new(NONE);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AbortReason {
    Interrupt,
    Timeout,
}

/// Record an abort request. The first reason wins; later requests are
/// ignored so a timeout firing after Ctrl-C cannot change the exit code.
pub fn request(reason: AbortReason) {
    let value = match reason {
        AbortReason::Interrupt => INTERRUPT,
        AbortReason::Timeout => TIMEOUT,
    };
    let _ = STATE.compare_exchange(NONE, value, Ordering::SeqCst, Ordering::SeqCst);
}

/// The recorded abort reason, if any.
pub fn reason() -> Option<AbortReason> {
    match STATE.load(Ordering::SeqCst) {
        INTERRUPT => Some(AbortReason::Interrupt),
        TIMEOUT => Some(AbortReason::Timeout),
        _ => None,
    }
}

/// Abort signal handed to the engine: aborted as soon as any reason is set.
#[derive(Debug, Clone, Copy, Default)]
pub struct ProcessAbort;

impl AbortSignal for ProcessAbort {
    #[inline]
    fn is_aborted(&self) -> bool {
        STATE.load(Ordering::Relaxed) != NONE
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
        STATE.load(Ordering::Relaxed) != NONE
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

/// Arm the run timeout: after `seconds`, long-running analyses stop at the
/// next abort check and the process exits 124.
pub fn arm_timeout(seconds: f64) {
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_secs_f64(seconds));
        if reason().is_none() {
            eprintln!("Timeout: simulation exceeded {seconds}s — stopping at the next safe point");
        }
        request(AbortReason::Timeout);
    });
}

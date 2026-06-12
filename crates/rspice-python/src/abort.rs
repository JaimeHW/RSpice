//! Cooperative cancellation for long-running simulations.
//!
//! Core exposes `run_*_with_abort` entry points that poll an [`AbortSignal`].
//! Here we run the simulation on a scoped worker thread while the calling
//! thread keeps servicing Python signal handlers, so Ctrl-C interrupts a
//! 10-minute transient instead of being delivered after it completes.

use pyo3::prelude::*;
use rspice_core::AtomicAbort;
use rspice_core::engine::SimulationError;
use std::time::Duration;

/// Longest interval between signal polls; bounds Ctrl-C latency.
const MAX_POLL_INTERVAL: Duration = Duration::from_millis(50);

/// Run a simulation closure on a worker thread, polling Python signals.
///
/// The closure receives an abort handle it must thread into a core
/// `run_*_with_abort` call. When a Python signal handler raises (typically
/// `KeyboardInterrupt`), the abort flag is set, the core run unwinds with
/// `SimulationError::Aborted`, and the signal exception is raised in Python.
///
/// Polling uses exponential backoff from 50 µs to 50 ms so sub-millisecond
/// solves pay ~0.1 ms of overhead while long runs poll cheaply.
pub(crate) fn run_interruptible<T, F>(py: Python<'_>, f: F) -> PyResult<T>
where
    T: Send,
    F: FnOnce(&AtomicAbort) -> Result<T, SimulationError> + Send,
{
    let abort = AtomicAbort::new();
    let (outcome, signal_error) = std::thread::scope(|scope| {
        let abort_ref = &abort;
        let worker = scope.spawn(move || f(abort_ref));

        let mut signal_error: Option<PyErr> = None;
        let mut wait = Duration::from_micros(50);
        while !worker.is_finished() {
            if signal_error.is_none()
                && let Err(err) = py.check_signals()
            {
                signal_error = Some(err);
                abort_ref.set();
            }
            py.allow_threads(|| std::thread::sleep(wait));
            wait = (wait * 2).min(MAX_POLL_INTERVAL);
        }

        let outcome = worker.join().unwrap_or_else(|_| {
            Err(SimulationError::Circuit(
                "simulation worker thread panicked".to_string(),
            ))
        });
        (outcome, signal_error)
    });

    if let Some(err) = signal_error {
        return Err(err);
    }
    outcome.map_err(crate::errors::simulation_error_to_pyerr)
}

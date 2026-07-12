//! Cooperative cancellation for long-running simulations.
//!
//! Core exposes `run_*_with_abort` entry points that poll an [`AbortSignal`].
//! Here we run the simulation on a scoped worker thread while the calling
//! thread keeps servicing Python signal handlers, so Ctrl-C interrupts a
//! 10-minute transient instead of being delivered after it completes.

use pyo3::prelude::*;
use rspice_core::engine::SimulationError;
use rspice_core::{AbortSignal, AtomicAbort};
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

/// Longest interval between signal polls; bounds Ctrl-C latency.
const MAX_POLL_INTERVAL: Duration = Duration::from_millis(50);

/// Thread-safe registry of simulations currently running through one Engine.
#[derive(Default)]
pub(crate) struct ActiveRuns {
    next_id: AtomicU64,
    runs: Mutex<HashMap<u64, Arc<RunControl>>>,
}

impl ActiveRuns {
    fn lock(&self) -> MutexGuard<'_, HashMap<u64, Arc<RunControl>>> {
        self.runs
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn register(&self) -> ActiveRun<'_> {
        let control = Arc::new(RunControl::default());
        let mut runs = self.lock();
        let id = loop {
            let candidate = self.next_id.fetch_add(1, Ordering::Relaxed);
            if let std::collections::hash_map::Entry::Vacant(entry) = runs.entry(candidate) {
                entry.insert(Arc::clone(&control));
                break candidate;
            }
        };
        drop(runs);
        ActiveRun {
            registry: self,
            id,
            control,
        }
    }

    /// Signal every call that was active when this method acquired the lock.
    pub(crate) fn cancel_all(&self) -> usize {
        let runs = self.lock();
        for control in runs.values() {
            control.cancel();
        }
        runs.len()
    }

    pub(crate) fn count(&self) -> usize {
        self.lock().len()
    }

    /// Progress for a single active call, if that analysis reports it.
    /// Multiple simultaneous calls are intentionally ambiguous.
    pub(crate) fn progress(&self) -> Option<f64> {
        let runs = self.lock();
        if runs.len() != 1 {
            return None;
        }
        runs.values().next().and_then(|control| control.progress())
    }
}

struct ActiveRun<'a> {
    registry: &'a ActiveRuns,
    id: u64,
    control: Arc<RunControl>,
}

impl Drop for ActiveRun<'_> {
    fn drop(&mut self) {
        self.registry.lock().remove(&self.id);
    }
}

pub(crate) struct RunControl {
    abort: AtomicAbort,
    progress_bits: AtomicU64,
}

impl Default for RunControl {
    fn default() -> Self {
        Self {
            abort: AtomicAbort::new(),
            progress_bits: AtomicU64::new(f64::NAN.to_bits()),
        }
    }
}

impl RunControl {
    fn cancel(&self) {
        self.abort.set();
    }

    fn progress(&self) -> Option<f64> {
        let value = f64::from_bits(self.progress_bits.load(Ordering::Relaxed));
        value.is_finite().then_some(value)
    }
}

impl AbortSignal for RunControl {
    fn is_aborted(&self) -> bool {
        self.abort.is_aborted()
    }

    fn observe_progress(&self, fraction: f64) {
        if fraction.is_finite() {
            self.progress_bits
                .store(fraction.clamp(0.0, 1.0).to_bits(), Ordering::Relaxed);
        }
    }
}

/// Run a simulation closure on a worker thread, polling Python signals.
///
/// The closure receives an abort handle it must thread into a core
/// `run_*_with_abort` call. When a Python signal handler raises (typically
/// `KeyboardInterrupt`), the abort flag is set, the core run unwinds with
/// `SimulationError::Aborted`, and the signal exception is raised in Python.
///
/// Polling uses exponential backoff from 50 µs to 50 ms so sub-millisecond
/// solves pay ~0.1 ms of overhead while long runs poll cheaply.
pub(crate) fn run_interruptible<T, F>(py: Python<'_>, active_runs: &ActiveRuns, f: F) -> PyResult<T>
where
    T: Send,
    F: FnOnce(&RunControl) -> Result<T, SimulationError> + Send,
{
    let active_run = active_runs.register();
    let control = Arc::clone(&active_run.control);
    let (outcome, signal_error) = std::thread::scope(|scope| {
        let worker_control = Arc::clone(&control);
        let worker = scope.spawn(move || f(worker_control.as_ref()));

        let mut signal_error: Option<PyErr> = None;
        let mut wait = Duration::from_micros(50);
        while !worker.is_finished() {
            if signal_error.is_none()
                && let Err(err) = py.check_signals()
            {
                signal_error = Some(err);
                control.cancel();
            }
            py.detach(|| std::thread::sleep(wait));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_run_registration_cancellation_and_cleanup_are_scoped() {
        let runs = ActiveRuns::default();
        assert_eq!(runs.count(), 0);
        assert_eq!(runs.cancel_all(), 0);

        let first = runs.register();
        let second = runs.register();
        assert_eq!(runs.count(), 2);
        assert_eq!(runs.cancel_all(), 2);
        assert!(first.control.is_aborted());
        assert!(second.control.is_aborted());

        drop(first);
        assert_eq!(runs.count(), 1);
        drop(second);
        assert_eq!(runs.count(), 0);

        let later = runs.register();
        assert!(!later.control.is_aborted());
    }

    #[test]
    fn progress_is_exposed_only_for_one_reporting_run() {
        let runs = ActiveRuns::default();
        assert_eq!(runs.progress(), None);

        let first = runs.register();
        assert_eq!(runs.progress(), None);
        first.control.observe_progress(0.25);
        assert_eq!(runs.progress(), Some(0.25));

        let second = runs.register();
        second.control.observe_progress(0.75);
        assert_eq!(runs.progress(), None);

        drop(second);
        assert_eq!(runs.progress(), Some(0.25));
    }
}

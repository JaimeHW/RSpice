//! An abort signal that trips when a run reports complete progress.

use rspice_core::AbortSignal;
use std::sync::atomic::{AtomicBool, Ordering};

/// Reports aborted from the first check after the run published a progress
/// fraction of 1.0, so cancellation lands after the last solve.
pub struct StopAtCompletion(pub AtomicBool);

impl AbortSignal for StopAtCompletion {
    fn is_aborted(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
    fn observe_progress(&self, fraction: f64) {
        if fraction == 1.0 {
            self.0.store(true, Ordering::Relaxed);
        }
    }
}

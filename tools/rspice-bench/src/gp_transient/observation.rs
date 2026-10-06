//! Accepted-sample cancellation, without a scheduler-dependent trigger thread.

use rspice_core::abort_signal::{AbortSignal, TransientSample};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

pub(super) struct Observation {
    cancel_at: f64,
    pub requested: OnceLock<(Instant, f64)>,
    pub samples: AtomicUsize,
    pub samples_after_request: AtomicUsize,
    pub polls: AtomicUsize,
    pub polls_after_request: AtomicUsize,
}

impl Observation {
    pub fn new(cancel_at: f64) -> Self {
        Self {
            cancel_at,
            requested: OnceLock::new(),
            samples: AtomicUsize::new(0),
            samples_after_request: AtomicUsize::new(0),
            polls: AtomicUsize::new(0),
            polls_after_request: AtomicUsize::new(0),
        }
    }
}

impl AbortSignal for Observation {
    fn is_aborted(&self) -> bool {
        self.polls.fetch_add(1, Ordering::Relaxed);
        if self.requested.get().is_some() {
            self.polls_after_request.fetch_add(1, Ordering::Relaxed);
            true
        } else {
            false
        }
    }

    fn observe_transient_sample(&self, sample: TransientSample<'_>) {
        if self.requested.get().is_some() {
            self.samples_after_request.fetch_add(1, Ordering::Relaxed);
        }
        self.samples.fetch_add(1, Ordering::Relaxed);
        if let Some(&time) = sample.time.last()
            && time >= self.cancel_at
            && self.requested.get().is_none()
        {
            let _ = self.requested.set((Instant::now(), time));
        }
    }
}

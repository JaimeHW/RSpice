//! The engine's build pass over a retained run's decks, off the UI thread.
//!
//! Preflight builds every deck a prepared run dispatches, without solving, to
//! learn what the engine refuses there. With a PDK and a multi-point Run Set
//! that is many parses and builds, and doing them inside the frame that took
//! the press froze the window for as long as they took. The pass now runs on
//! a worker over its own copy of the retained snapshot, so the decks it
//! builds are the ones frozen at the press; the frame loop polls it, the
//! status line states its progress, and Stop cancels it.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, mpsc};

use super::*;
use crate::simulation::execution::{DeckBuildProgress, EngineRefusal};

/// A pass in flight.
pub(super) struct PendingEngineCheck {
    /// The snapshot the pass builds, which is the one it may authorize.
    snapshot: crate::product::ContentDigest,
    receiver: mpsc::Receiver<Vec<EngineRefusal>>,
    cancel: Arc<AtomicBool>,
    built: Arc<AtomicUsize>,
    total: Arc<AtomicUsize>,
}

/// How a pass ended.
#[derive(Debug)]
pub(crate) struct EngineCheckOutcome {
    /// What the engine refused, or `None` when the worker ended without a
    /// verdict.
    pub(crate) refusals: Option<Vec<EngineRefusal>>,
    /// Whether the run the pass checked is still the one retained for
    /// dispatch. A run prepared or dropped since the press means the
    /// refusals describe decks nobody will dispatch.
    pub(crate) current: bool,
}

impl SimulationController {
    /// Start building the retained run's decks on a worker, replacing any
    /// pass still in flight. `false` when no run is retained.
    pub(crate) fn start_engine_check(&mut self) -> bool {
        self.cancel_engine_check();
        let Some(snapshot) = self.run_authorization.retained_snapshot().cloned() else {
            return false;
        };
        let (sender, receiver) = mpsc::channel();
        let cancel = Arc::new(AtomicBool::new(false));
        let built = Arc::new(AtomicUsize::new(0));
        let total = Arc::new(AtomicUsize::new(0));
        let pending = PendingEngineCheck {
            snapshot: snapshot.digest(),
            receiver,
            cancel: Arc::clone(&cancel),
            built: Arc::clone(&built),
            total: Arc::clone(&total),
        };
        spawn_or_inline(move || {
            let refusals = snapshot.engine_refusals_reporting(&cancel, &|progress| {
                total.store(progress.total, Ordering::Relaxed);
                built.store(progress.built, Ordering::Relaxed);
            });
            if !cancel.load(Ordering::Relaxed) {
                let _ = sender.send(refusals);
            }
        });
        self.engine_check = Some(pending);
        true
    }

    /// How far the pass in flight has got, or `None` when none is.
    pub(crate) fn engine_check_progress(&self) -> Option<DeckBuildProgress> {
        self.engine_check.as_ref().map(|pending| DeckBuildProgress {
            built: pending.built.load(Ordering::Relaxed),
            total: pending.total.load(Ordering::Relaxed),
        })
    }

    /// Stop the pass in flight. Its worker stops at the next deck it would
    /// build or the next abort point inside a build, and its verdict is
    /// dropped. Returns whether a pass was in flight.
    pub(crate) fn cancel_engine_check(&mut self) -> bool {
        let Some(pending) = self.engine_check.take() else {
            return false;
        };
        pending.cancel.store(true, Ordering::Relaxed);
        true
    }

    /// The pass's verdict, once it has one. `None` while it runs or when no
    /// pass is in flight.
    pub(crate) fn poll_engine_check(&mut self) -> Option<EngineCheckOutcome> {
        let pending = self.engine_check.as_ref()?;
        let refusals = match pending.receiver.try_recv() {
            Ok(refusals) => Some(refusals),
            Err(mpsc::TryRecvError::Empty) => return None,
            Err(mpsc::TryRecvError::Disconnected) => None,
        };
        let current = self
            .run_authorization
            .retained_snapshot()
            .is_some_and(|snapshot| snapshot.digest() == pending.snapshot);
        self.engine_check = None;
        Some(EngineCheckOutcome { refusals, current })
    }
}

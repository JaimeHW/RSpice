//! Viewer projections that outlive the frame that built them.
//!
//! A Results viewer is a pure function of retained evidence and the reader's
//! controls, which is what makes immediate mode the right shape for it — and
//! also what makes it easy to write one that filters, groups, sorts and scans
//! a complete dataset on every frame for a picture that has not changed.
//!
//! Each plan here is that work, done once and keyed by everything it read.
//! The key is checked rather than the plan being cleared from outside: a memo
//! that depends on someone else remembering to invalidate it is one refactor
//! away from serving last run's answer. Handles are `Arc` so a viewer can
//! hold its plan while still writing the reader's controls back to state.

use std::sync::Arc;

/// Every memoized viewer projection the Results workspace holds.
#[derive(Debug, Clone, Default)]
pub(super) struct ViewPlans {
    /// Safe-operating-area per-rule stress facts; see [`super::soa`].
    pub(super) soa: Option<Arc<super::soa::SoaPlan>>,
}

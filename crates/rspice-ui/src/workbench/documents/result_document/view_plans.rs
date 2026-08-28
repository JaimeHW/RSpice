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

use std::cell::RefCell;
use std::sync::Arc;

/// Every memoized viewer projection the Results workspace holds.
#[derive(Debug, Clone, Default)]
pub(super) struct ViewPlans {
    /// The dataset manifest projection; see [`super::manifest`].
    pub(super) manifest: Option<Arc<super::manifest::ManifestPlan>>,
    /// Serialized exact evidence for the selected typed artifact; see
    /// [`super::table`].
    pub(super) artifact: Option<Arc<super::table::ArtifactTextPlan>>,
    /// Family envelopes for the drawn wave panes, one per pane; see
    /// [`super::waves::FamilyEnvelopeCache`].
    pub(super) envelopes: super::waves::FamilyEnvelopeCache,
    /// Descriptive statistics for the drawn distribution; see
    /// [`super::hist`].
    pub(super) hist: Option<Arc<super::hist::HistPlan>>,
    /// The operating-point sheet's row plan; see [`super::op_inspector`].
    pub(super) op: Option<Arc<super::op_inspector::OpPlan>>,
    /// Where the retained optimizer history is; see
    /// [`super::optimization`]. Behind a cell because the tab strip's
    /// availability gate holds only `&AppState` and asks every frame.
    pub(super) optimization: RefCell<Option<Arc<super::optimization::OptimizationPlan>>>,
    /// The ranked order of the retained sensitivity result; see
    /// [`super::sensitivity`].
    pub(super) sensitivity: Option<Arc<super::sensitivity::SensitivityPlan>>,
    /// Safe-operating-area per-rule stress facts; see [`super::soa`].
    pub(super) soa: Option<Arc<super::soa::SoaPlan>>,
}

#[cfg(test)]
impl ViewPlans {
    /// Whether every memoized projection has been released.
    ///
    /// Spelled over the whole struct rather than over the slot a test happens
    /// to have filled: a plan added later and forgotten by
    /// [`super::ResultsState::retain_datasets`] holds a discarded dataset's
    /// projection — `ArtifactTextPlan` holds a complete serialized artifact —
    /// for the life of the session.
    pub(super) fn is_empty(&self) -> bool {
        let Self {
            manifest,
            artifact,
            envelopes,
            hist,
            op,
            optimization,
            sensitivity,
            soa,
        } = self;
        manifest.is_none()
            && artifact.is_none()
            && envelopes.is_empty()
            && hist.is_none()
            && op.is_none()
            && optimization.borrow().is_none()
            && sensitivity.is_none()
            && soa.is_none()
    }
}

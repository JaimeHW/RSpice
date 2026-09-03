//! Typed result of one envelope-following run.
//!
//! Envelope execution is two engine steps: solve an authenticated carrier
//! periodic state with the selected slow sources frozen, then reactivate those
//! sources and continue transient integration from slow-time origin zero.
//! Before this type the two halves were only ever returned as loose tuples, so
//! no surface could name an envelope result, and the capability registry had a
//! family with no result type behind it.
//!
//! [`EnvelopeResult`] captures exactly what those two calls compute and nothing
//! else. In particular it does not synthesize harmonic envelope series: the
//! production engine does not compute them, and inventing them here would
//! publish physics the solver never solved.

use super::{HbEnvelopeContinuationState, HbEnvelopeStateGuarantee};
use crate::Value;
use crate::analysis::HbResult;
use crate::engine::{TransientCheckpoint, TransientResult};

/// One completed envelope run: its carrier state, the authenticated
/// continuation artifact, and the continued transient.
#[derive(Debug, Clone)]
pub struct EnvelopeResult {
    carrier: HbResult,
    state: HbEnvelopeContinuationState,
    continued_transient: TransientResult,
    final_checkpoint: TransientCheckpoint,
    slow_time_duration: Value,
    slow_time_max_step: Value,
}

impl EnvelopeResult {
    pub(crate) const fn new(
        carrier: HbResult,
        state: HbEnvelopeContinuationState,
        continued_transient: TransientResult,
        final_checkpoint: TransientCheckpoint,
        slow_time_duration: Value,
        slow_time_max_step: Value,
    ) -> Self {
        Self {
            carrier,
            state,
            continued_transient,
            final_checkpoint,
            slow_time_duration,
            slow_time_max_step,
        }
    }

    /// The converged carrier periodic state the envelope started from.
    pub const fn carrier(&self) -> &HbResult {
        &self.carrier
    }

    /// The authenticated continuation artifact binding carrier and transient.
    pub const fn state(&self) -> &HbEnvelopeContinuationState {
        &self.state
    }

    /// Completeness contract of the carrier-to-transient projection.
    pub fn guarantee(&self) -> HbEnvelopeStateGuarantee {
        self.state.guarantee()
    }

    /// The transient continued from the carrier state.
    pub const fn continued_transient(&self) -> &TransientResult {
        &self.continued_transient
    }

    /// Checkpoint at the end of the continued transient.
    pub const fn final_checkpoint(&self) -> &TransientCheckpoint {
        &self.final_checkpoint
    }

    /// Slow-time origin the continuation restarted from, in seconds.
    pub fn time_origin(&self) -> Value {
        self.state.time_origin()
    }

    /// Slow-time interval the continuation integrated, in seconds.
    pub const fn slow_time_duration(&self) -> Value {
        self.slow_time_duration
    }

    /// Maximum slow-time step the continuation was allowed, in seconds.
    pub const fn slow_time_max_step(&self) -> Value {
        self.slow_time_max_step
    }
}

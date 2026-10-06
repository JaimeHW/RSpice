//! Transport record accounting across live state and checkpoint ownership.

use super::*;

impl BjtTransientHistory {
    pub(in crate::engine) fn transport_allocated_bytes(&self) -> usize {
        self.phase
            .iter()
            .flatten()
            .map(|phase| phase.allocated_bytes())
            .fold(0usize, usize::saturating_add)
    }

    pub(in crate::engine) fn transport_copy_bytes(&self) -> usize {
        self.phase
            .iter()
            .flatten()
            .map(|phase| phase.copy_allocation_bytes())
            .fold(0usize, usize::saturating_add)
    }
}

impl Engine {
    pub(in crate::engine) fn ensure_transport_history_bytes(
        &self,
        requested: usize,
    ) -> Result<(), SimulationError> {
        crate::resource::ResourceLimitError::ensure(
            crate::ResourceKind::TransportHistoryBytes,
            requested,
            self.config.resource_limits.max_transport_history_bytes,
        )?;
        Ok(())
    }

    /// The source remains live during copying; previously retained output
    /// checkpoints remain owned by the analysis as well. Check this peak before
    /// the first record copy, rather than after materializing the checkpoint.
    pub(in crate::engine) fn ensure_transport_history_copy(
        &self,
        history: &BjtTransientHistory,
        other_retained_bytes: usize,
    ) -> Result<(), SimulationError> {
        self.ensure_transport_history_bytes(
            history
                .transport_allocated_bytes()
                .saturating_add(history.transport_copy_bytes())
                .saturating_add(other_retained_bytes),
        )
    }
}

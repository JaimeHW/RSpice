use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

use crate::product::ContentDigest;

/// Controller-owned issuer for single-use execution permits.
///
/// Even values are idle generations and odd values are outstanding permits.
/// Issuing a replacement first invalidates any prior odd generation. A permit
/// can therefore be consumed exactly once, including across accidental owner
/// clones that share this authority.
#[derive(Debug, Clone, Default)]
pub(in crate::simulation) struct ExecutionPermitIssuer {
    generation: Arc<AtomicU64>,
}

/// A non-cloneable authorization for exactly one prepared snapshot digest.
#[derive(Debug)]
pub(in crate::simulation) struct ExecutionPermit {
    generation: Arc<AtomicU64>,
    expected_generation: u64,
    snapshot_digest: ContentDigest,
}

/// Opaque proof emitted only after the generation CAS succeeds.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::simulation) struct ConsumedExecutionPermit {
    snapshot_digest: ContentDigest,
}

impl ConsumedExecutionPermit {
    pub(super) const fn snapshot_digest(&self) -> ContentDigest {
        self.snapshot_digest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::simulation) enum PermitError {
    GenerationExhausted,
    SnapshotMismatch,
    AlreadyConsumedOrInvalidated,
}

impl std::fmt::Display for PermitError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GenerationExhausted => {
                formatter.write_str("execution authorization generation exhausted")
            }
            Self::SnapshotMismatch => {
                formatter.write_str("execution authorization does not match the prepared input")
            }
            Self::AlreadyConsumedOrInvalidated => {
                formatter.write_str("execution authorization was already consumed or invalidated")
            }
        }
    }
}

impl ExecutionPermitIssuer {
    pub(in crate::simulation) fn issue(
        &self,
        snapshot_digest: ContentDigest,
    ) -> Result<ExecutionPermit, PermitError> {
        loop {
            let observed = self.generation.load(Ordering::Acquire);
            let idle = if observed & 1 == 0 {
                observed
            } else {
                observed
                    .checked_add(1)
                    .ok_or(PermitError::GenerationExhausted)?
            };
            let reserved = idle
                .checked_add(1)
                .ok_or(PermitError::GenerationExhausted)?;
            if self
                .generation
                .compare_exchange(observed, reserved, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return Ok(ExecutionPermit {
                    generation: Arc::clone(&self.generation),
                    expected_generation: reserved,
                    snapshot_digest,
                });
            }
        }
    }

    pub(in crate::simulation) fn invalidate(&self) -> Result<(), PermitError> {
        loop {
            let observed = self.generation.load(Ordering::Acquire);
            if observed & 1 == 0 {
                return Ok(());
            }
            let invalidated = observed
                .checked_add(1)
                .ok_or(PermitError::GenerationExhausted)?;
            if self
                .generation
                .compare_exchange(observed, invalidated, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return Ok(());
            }
        }
    }
}

impl ExecutionPermit {
    /// Consume this permit only when both the retained and freshly prepared
    /// snapshots have the exact digest authorized at issuance.
    pub(in crate::simulation) fn consume(
        self,
        retained_digest: ContentDigest,
        current_digest: ContentDigest,
    ) -> Result<ConsumedExecutionPermit, PermitError> {
        if retained_digest != self.snapshot_digest || current_digest != self.snapshot_digest {
            return Err(PermitError::SnapshotMismatch);
        }
        let consumed = self
            .expected_generation
            .checked_add(1)
            .ok_or(PermitError::GenerationExhausted)?;
        self.generation
            .compare_exchange(
                self.expected_generation,
                consumed,
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .map(|_| ConsumedExecutionPermit {
                snapshot_digest: self.snapshot_digest,
            })
            .map_err(|_| PermitError::AlreadyConsumedOrInvalidated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn digest(byte: u8) -> ContentDigest {
        ContentDigest::from_bytes([byte; 32])
    }

    #[test]
    fn permit_is_single_use_even_if_the_private_generation_is_replayed() {
        let issuer = ExecutionPermitIssuer::default();
        let permit = issuer.issue(digest(1)).expect("issue permit");
        let replay = ExecutionPermit {
            generation: Arc::clone(&permit.generation),
            expected_generation: permit.expected_generation,
            snapshot_digest: permit.snapshot_digest,
        };

        permit.consume(digest(1), digest(1)).expect("first consume");
        assert_eq!(
            replay.consume(digest(1), digest(1)),
            Err(PermitError::AlreadyConsumedOrInvalidated)
        );
    }

    #[test]
    fn replacement_issue_invalidates_the_previous_generation() {
        let issuer = ExecutionPermitIssuer::default();
        let stale = issuer.issue(digest(1)).expect("stale permit");
        let current = issuer.issue(digest(2)).expect("replacement permit");

        assert_eq!(
            stale.consume(digest(1), digest(1)),
            Err(PermitError::AlreadyConsumedOrInvalidated)
        );
        current
            .consume(digest(2), digest(2))
            .expect("replacement remains valid");
    }

    #[test]
    fn permit_fails_closed_when_live_input_digest_changed() {
        let issuer = ExecutionPermitIssuer::default();
        let permit = issuer.issue(digest(1)).expect("issue permit");
        assert_eq!(
            permit.consume(digest(1), digest(2)),
            Err(PermitError::SnapshotMismatch)
        );
    }
}

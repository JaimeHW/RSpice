//! Working memory a periodic solve confirms with the allocator before it
//! starts filling it.

use crate::Value;
use std::collections::TryReserveError;

/// The allocator's refusal of a solve's working memory, naming its size.
#[derive(Debug)]
pub(crate) struct WorkingMemoryRefusal {
    pub(crate) object: &'static str,
    pub(crate) bytes: usize,
    pub(crate) source: TryReserveError,
}

/// Confirm the allocator can back `values` binary64 values of `object`.
///
/// A periodic solve's working set is spread over per-row vectors that it
/// fills as it runs, so no single allocation names its size. Reserving the
/// whole set as one block first lets the allocator refuse a set this machine
/// cannot hold as a typed failure before any row exists, instead of an abort
/// partway through the solve. The block is released at once; its size, not
/// its storage, is what is being asked about. Nothing caps `values`: it comes
/// from the circuit and the analysis grid.
pub(crate) fn confirm_working_values(
    object: &'static str,
    values: usize,
) -> Result<(), WorkingMemoryRefusal> {
    let mut block: Vec<Value> = Vec::new();
    block
        .try_reserve_exact(values)
        .map_err(|source| WorkingMemoryRefusal {
            object,
            bytes: values.saturating_mul(std::mem::size_of::<Value>()),
            source,
        })
}

impl From<WorkingMemoryRefusal> for crate::analysis::quasi_periodic::QuasiPeriodicError {
    fn from(refusal: WorkingMemoryRefusal) -> Self {
        Self::Allocation {
            object: refusal.object,
            bytes: refusal.bytes,
            source: refusal.source,
        }
    }
}

impl From<WorkingMemoryRefusal> for crate::analysis::harmonic_balance::HbError {
    fn from(refusal: WorkingMemoryRefusal) -> Self {
        Self::Allocation {
            object: refusal.object,
            bytes: refusal.bytes,
            source: refusal.source,
        }
    }
}

impl From<WorkingMemoryRefusal> for crate::SimulationError {
    fn from(refusal: WorkingMemoryRefusal) -> Self {
        Self::Allocation {
            object: refusal.object,
            source: refusal.source,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_set_beyond_the_address_space_is_refused_with_its_size() {
        let values = usize::MAX / 4;
        let refusal = confirm_working_values("test workspace", values)
            .expect_err("no allocator backs 2^62 binary64 values");
        assert_eq!(refusal.object, "test workspace");
        assert_eq!(refusal.bytes, values.saturating_mul(8));
    }

    #[test]
    fn a_set_this_machine_holds_is_confirmed() {
        confirm_working_values("test workspace", 1 << 20).expect("8 MiB is held");
    }
}

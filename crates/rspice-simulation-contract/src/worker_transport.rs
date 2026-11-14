//! Portable numeric-series representation for the worker protocol.
//!
//! Every numeric array crosses the worker boundary as its own transferable
//! `Float64Array`. A response costs one copy out of the worker's linear
//! memory and one copy into the receiver's; `postMessage` moves the buffers
//! between the threads without copying them. Neither side caps how many
//! values or buffers cross: a run needs the memory its result needs, and an
//! allocation the memory cannot give is refused as that allocation, with the
//! size it asked for ([`try_f64_buffer`]).

use serde::{Deserialize, Serialize};

use crate::resource_failure::ResourceFailure;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkerF64Series {
    Inline(Vec<f64>),
    Buffer { buffer: usize, len: usize },
}

/// A zeroed buffer of `len` values for `object`, or the allocation failure
/// naming the bytes it needed.
///
/// A receiver copies each transferred array into its own memory with this,
/// so a result larger than the memory left to it is refused by name instead
/// of aborting the program. In 32-bit WebAssembly no allocation can exceed
/// `isize::MAX` bytes (2 GiB) and the whole memory is at most 4 GiB; both
/// arrive here as the same refusal.
pub fn try_f64_buffer(len: usize, object: &str) -> Result<Vec<f64>, ResourceFailure> {
    let mut values = Vec::new();
    values
        .try_reserve_exact(len)
        .map_err(|error| f64_allocation_failure(object, len, &error.to_string()))?;
    values.resize(len, 0.0);
    Ok(values)
}

/// The refusal for `len` values of `object` that could not be allocated.
pub fn f64_allocation_failure(object: &str, len: usize, reason: &str) -> ResourceFailure {
    ResourceFailure::Allocation {
        object: object.to_owned(),
        detail: format!(
            "{len} values ({} bytes) were needed: {reason}",
            len.saturating_mul(std::mem::size_of::<f64>())
        ),
    }
}

/// The total value count of a request's transfer buffers, or the overflow
/// that makes it unrepresentable on this platform.
pub fn checked_worker_request_numeric_total(
    current_total: usize,
    buffer_len: usize,
) -> Result<usize, String> {
    current_total
        .checked_add(buffer_len)
        .ok_or_else(|| "worker request numeric size overflows this platform".to_owned())
}

impl WorkerF64Series {
    pub fn into_convergence_values(self, buffers: &[Vec<f64>]) -> Result<Vec<f64>, String> {
        if matches!(self, Self::Inline(_)) {
            return Err("Convergence arrays must use dedicated transfer buffers".to_owned());
        }
        self.into_vec(buffers)
    }
    pub fn from_vec(values: Vec<f64>, buffers: &mut Vec<Vec<f64>>) -> Self {
        let len = values.len();
        let buffer = buffers.len();
        buffers.push(values);
        Self::Buffer { buffer, len }
    }

    pub fn into_vec(self, buffers: &[Vec<f64>]) -> Result<Vec<f64>, String> {
        match self {
            Self::Inline(values) => Ok(values),
            Self::Buffer { buffer, len } => {
                let values = buffers
                    .get(buffer)
                    .ok_or_else(|| format!("missing transferable buffer {buffer}"))?;
                if values.len() != len {
                    return Err(format!(
                        "transferable buffer {buffer} has length {}, expected {len}",
                        values.len()
                    ));
                }
                Ok(values.clone())
            }
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Inline(values) => values.len(),
            Self::Buffer { len, .. } => *len,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_buffer_the_memory_cannot_hold_is_refused_with_the_size_it_needed() {
        assert_eq!(
            try_f64_buffer(3, "x").expect("three values fit"),
            vec![0.0; 3]
        );
        let len = usize::MAX / 4;
        let failure = try_f64_buffer(len, "transfer buffer 7").expect_err("cannot be had");
        let ResourceFailure::Allocation { object, detail } = &failure else {
            panic!("an allocation failure, not {failure:?}")
        };
        assert_eq!(object, "transfer buffer 7");
        assert!(
            detail.starts_with(&format!(
                "{len} values ({} bytes) were needed: ",
                usize::MAX
            )),
            "{detail}"
        );
        assert_eq!(failure.code(), "allocation_failed");
    }

    #[test]
    fn a_request_total_only_refuses_what_this_platform_cannot_count() {
        assert_eq!(checked_worker_request_numeric_total(10, 20), Ok(30));
        assert!(checked_worker_request_numeric_total(usize::MAX, 1).is_err());
    }
}

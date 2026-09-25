//! Portable numeric-series representation and transfer limits for the worker protocol.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WorkerF64Series {
    Inline(Vec<f64>),
    Buffer { buffer: usize, len: usize },
}

pub const MAX_WORKER_F64_VALUES: usize = 16_777_216;
pub const MAX_WORKER_TRANSFER_BUFFERS: usize = 65_536;

pub fn checked_worker_request_numeric_total(
    current_total: usize,
    buffer_index: usize,
    buffer_len: usize,
) -> Result<usize, String> {
    if buffer_len > MAX_WORKER_F64_VALUES {
        return Err(format!(
            "worker request transport buffer {buffer_index} contains {buffer_len} values, exceeding the {MAX_WORKER_F64_VALUES}-value limit"
        ));
    }
    let total = current_total
        .checked_add(buffer_len)
        .ok_or_else(|| "worker request numeric size overflows this platform".to_owned())?;
    if total > MAX_WORKER_F64_VALUES {
        return Err(format!(
            "worker request contains more than {MAX_WORKER_F64_VALUES} numerical values"
        ));
    }
    Ok(total)
}

pub fn validate_worker_request_transfer_buffers(buffers: &[Vec<f64>]) -> Result<(), String> {
    validate_worker_request_transfer_buffer_lengths(buffers.iter().map(Vec::len))
}

pub fn validate_worker_request_transfer_buffer_lengths(
    lengths: impl IntoIterator<Item = usize>,
) -> Result<(), String> {
    let lengths = lengths.into_iter();
    let (lower_bound, upper_bound) = lengths.size_hint();
    let declared_count = upper_bound.unwrap_or(lower_bound);
    if declared_count > MAX_WORKER_TRANSFER_BUFFERS {
        return Err(format!(
            "worker request contains {declared_count} transfer buffers, exceeding the {MAX_WORKER_TRANSFER_BUFFERS}-buffer limit"
        ));
    }
    let mut total = 0usize;
    for (index, len) in lengths.enumerate() {
        if index >= MAX_WORKER_TRANSFER_BUFFERS {
            return Err(format!(
                "worker request contains more than {MAX_WORKER_TRANSFER_BUFFERS} transfer buffers"
            ));
        }
        total = checked_worker_request_numeric_total(total, index, len)?;
    }
    Ok(())
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
            Self::Inline(values) => {
                if values.len() > MAX_WORKER_F64_VALUES {
                    return Err(format!(
                        "inline worker series contains {} values, exceeding the {MAX_WORKER_F64_VALUES}-value limit",
                        values.len()
                    ));
                }
                Ok(values)
            }
            Self::Buffer { buffer, len } => {
                if len > MAX_WORKER_F64_VALUES {
                    return Err(format!(
                        "transferable buffer {buffer} declares {len} values, exceeding the {MAX_WORKER_F64_VALUES}-value limit"
                    ));
                }
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

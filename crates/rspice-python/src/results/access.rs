//! Result access failures and node addressing.
//!
//! Every accessor in this module tree raises `IndexError` for an out-of-range
//! index and `KeyError` for an unknown node or branch name. Nothing fabricates
//! a silent zero: a caller who mistypes a node name learns immediately rather
//! than plotting a flat trace.
//!
//! [`ResultAccessError`] is the internal, testable form of that contract.
//! Converting it to a `PyErr` only at the boundary keeps the checks usable
//! from Rust unit tests that have no interpreter.

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ResultAccessError {
    InvalidNodeIndex {
        node: usize,
        available_nodes: usize,
    },
    InvalidTimeIndex {
        time_index: usize,
        available_points: usize,
    },
    InvalidSweepIndex {
        index: usize,
        available_points: usize,
    },
    InvalidFreqIndex {
        index: usize,
        available_points: usize,
    },
    UnknownNodeName {
        name: String,
    },
    UnknownBranchName {
        name: String,
    },
}

impl From<ResultAccessError> for PyErr {
    fn from(error: ResultAccessError) -> Self {
        match error {
            ResultAccessError::InvalidNodeIndex {
                node,
                available_nodes,
            } => crate::errors::index_error(format!(
                "node index {node} is out of range for result with {available_nodes} nodes"
            )),
            ResultAccessError::InvalidTimeIndex {
                time_index,
                available_points,
            } => crate::errors::index_error(format!(
                "time index {time_index} is out of range for result with {available_points} points"
            )),
            ResultAccessError::InvalidSweepIndex {
                index,
                available_points,
            } => crate::errors::index_error(format!(
                "sweep index {index} is out of range for result with {available_points} points"
            )),
            ResultAccessError::InvalidFreqIndex {
                index,
                available_points,
            } => crate::errors::index_error(format!(
                "frequency index {index} is out of range for result with {available_points} points"
            )),
            ResultAccessError::UnknownNodeName { name } => {
                crate::errors::key_error(format!("unknown node '{name}'"))
            }
            ResultAccessError::UnknownBranchName { name } => {
                crate::errors::key_error(format!("unknown branch '{name}'"))
            }
        }
    }
}

pub(super) type AccessResult<T> = Result<T, ResultAccessError>;

pub(super) fn invalid_node_index_error(node: usize, available_nodes: usize) -> ResultAccessError {
    ResultAccessError::InvalidNodeIndex {
        node,
        available_nodes,
    }
}

pub(super) fn invalid_time_index_error(
    time_index: usize,
    available_points: usize,
) -> ResultAccessError {
    ResultAccessError::InvalidTimeIndex {
        time_index,
        available_points,
    }
}

pub(super) fn invalid_sweep_index_error(
    index: usize,
    available_points: usize,
) -> ResultAccessError {
    ResultAccessError::InvalidSweepIndex {
        index,
        available_points,
    }
}

pub(super) fn invalid_freq_index_error(index: usize, available_points: usize) -> ResultAccessError {
    ResultAccessError::InvalidFreqIndex {
        index,
        available_points,
    }
}

pub(super) fn unknown_node_name_error(name: &str) -> ResultAccessError {
    ResultAccessError::UnknownNodeName {
        name: name.to_string(),
    }
}

pub(super) fn unknown_branch_name_error(name: &str) -> ResultAccessError {
    ResultAccessError::UnknownBranchName {
        name: name.to_string(),
    }
}

pub(crate) fn is_ground_name(name: &str) -> bool {
    matches!(name, "0") || name.eq_ignore_ascii_case("gnd")
}

pub(super) fn checked_simulation_voltage(
    result: &SimulationResult,
    node: usize,
) -> AccessResult<f64> {
    result
        .try_voltage(node)
        .ok_or_else(|| invalid_node_index_error(node, result.node_voltages.len().saturating_sub(1)))
}

pub(super) fn checked_simulation_voltage_named(
    result: &SimulationResult,
    name: &str,
) -> AccessResult<f64> {
    result
        .try_voltage_named(name)
        .ok_or_else(|| unknown_node_name_error(name))
}

/// Helper enum for node identification (by index or name)
#[derive(FromPyObject, Debug, Clone)]
pub enum NodeIdentifier {
    #[pyo3(transparent)]
    Index(usize),
    #[pyo3(transparent)]
    Name(String),
}

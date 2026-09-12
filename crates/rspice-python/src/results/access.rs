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
    /// A named net the result carries as events, asked for as a voltage.
    ///
    /// Distinct from [`Self::UnknownNodeName`] because the net is not unknown:
    /// it exists, it was recorded, and it was recorded in the only domain that
    /// resolves it. Telling a caller "unknown node" would send them looking
    /// for a typo. The kind travels with the name because the refusal names
    /// the carrier, and the two event domains publish under different
    /// spellings; the surface travels with it because the two result classes
    /// do too — `TransientResult.digital_events` against
    /// `CompressedTransientResult.digital_trace` — and naming one class's
    /// method on the other is the same defect as naming no method at all.
    EventOnlyNode {
        name: String,
        kind: rspice_core::analysis::transient::EventOnlyNetKind,
        surface: rspice_core::analysis::transient::EventTraceSurface,
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
            ResultAccessError::EventOnlyNode {
                name,
                kind,
                surface,
            } => crate::errors::key_error(
                rspice_core::analysis::transient::event_only_voltage_refusal(&name, kind, surface),
            ),
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

pub(super) fn event_only_node_error(
    name: &str,
    kind: rspice_core::analysis::transient::EventOnlyNetKind,
    surface: rspice_core::analysis::transient::EventTraceSurface,
) -> ResultAccessError {
    ResultAccessError::EventOnlyNode {
        name: name.to_string(),
        kind,
        surface,
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

/// The refusal a DC node addressed as a voltage gets when it carries events.
///
/// `try_voltage` answers `None` for both "no such node" and "that node has no
/// voltage", and the two need different sentences: one sends the caller
/// looking for a typo, the other names the carrier that does exist. The kind
/// decides which carrier, and the surface says whose accessor it is — a
/// solved DC point publishes no event trace of its own.
fn simulation_event_only_error(
    result: &SimulationResult,
    node: usize,
    name: &str,
) -> Option<ResultAccessError> {
    let kind = result.event_only_node_kind(node)?;
    Some(event_only_node_error(
        name,
        kind,
        rspice_core::analysis::transient::EventTraceSurface::SolvedPoint,
    ))
}

pub(super) fn checked_simulation_voltage(
    result: &SimulationResult,
    node: usize,
) -> AccessResult<f64> {
    result.try_voltage(node).ok_or_else(|| {
        let name = result
            .node_names
            .get(node)
            .cloned()
            .unwrap_or_else(|| node.to_string());
        simulation_event_only_error(result, node, &name).unwrap_or_else(|| {
            invalid_node_index_error(node, result.node_voltages.len().saturating_sub(1))
        })
    })
}

pub(super) fn checked_simulation_voltage_named(
    result: &SimulationResult,
    name: &str,
) -> AccessResult<f64> {
    result.try_voltage_named(name).ok_or_else(|| {
        result
            .node_index_named(name)
            .and_then(|node| simulation_event_only_error(result, node, name))
            .unwrap_or_else(|| unknown_node_name_error(name))
    })
}

/// Helper enum for node identification (by index or name)
#[derive(FromPyObject, Debug, Clone)]
pub enum NodeIdentifier {
    #[pyo3(transparent)]
    Index(usize),
    #[pyo3(transparent)]
    Name(String),
}

//! What a run of the reader's own design may hold on the machine it runs on.
//!
//! The engine ships fixed counts for what one run may materialize. They guard
//! a service against input it did not write, and stay there. A run of the
//! reader's own design on their own machine is bounded by that machine
//! instead: its counts are sized to the memory installed, and a design that
//! needs more than the machine has free is refused before it starts, in words
//! that say what it needs and what there is.

use crate::preparation::{PreparationError, PreparationStage};
use rspice_core::{ResourceKind, ResourceLimits};

/// The memory of the machine a run is prepared on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineMemory {
    /// The memory installed, which bounds the largest design the machine
    /// can run. A run's counts are sized to it, so they do not move from
    /// one preparation to the next.
    pub installed_bytes: u64,
    /// The memory free as the run is prepared, which is what this run can
    /// have.
    pub free_bytes: u64,
}

/// What one device of the flattened deck costs a run at its peak, from the
/// deck's flattening through the solved circuit and its first results.
///
/// Measured on the desktop app running a design of 32^4 resistors placed
/// through four levels of hierarchy: 1,082,401 devices peaked at 5.0 GB.
pub(crate) const RUN_BYTES_PER_DEVICE: u64 = 5 * 1024;

/// What one circuit node, or one unknown of the system the run solves, costs
/// a run at its peak: its name and index, its row of the matrix and its
/// entries in every solution and history vector.
pub(crate) const RUN_BYTES_PER_UNKNOWN: u64 = 4 * 1024;

/// What one retained result value costs: the sample, its place in a column
/// and the column's share of what indexes it.
const RUN_BYTES_PER_RESULT_VALUE: u64 = 16;

/// `limits` with the counts a run materializes sized to `installed` bytes of
/// memory. A count never falls below the one the engine ships: a design
/// those admit is refused, if at all, by [`check_fits`], which says why.
pub(crate) fn sized_to(mut limits: ResourceLimits, installed: u64) -> ResourceLimits {
    let fits = |each: u64| usize::try_from(installed / each).unwrap_or(usize::MAX);
    limits.max_flattened_elements = limits
        .max_flattened_elements
        .max(fits(RUN_BYTES_PER_DEVICE));
    limits.max_circuit_nodes = limits.max_circuit_nodes.max(fits(RUN_BYTES_PER_UNKNOWN));
    limits.max_matrix_unknowns = limits.max_matrix_unknowns.max(fits(RUN_BYTES_PER_UNKNOWN));
    limits.max_result_values = limits
        .max_result_values
        .max(fits(RUN_BYTES_PER_RESULT_VALUE));
    limits
}

/// Refuse a run whose `devices` need more memory than `memory` has free.
pub(crate) fn check_fits(devices: usize, memory: MachineMemory) -> Result<(), PreparationError> {
    let needed = (devices as u64).saturating_mul(RUN_BYTES_PER_DEVICE);
    if needed <= memory.free_bytes {
        return Ok(());
    }
    let message = if needed > memory.installed_bytes {
        format!(
            "This design has {} devices. Running it needs about {} of memory, more than the {} \
             this computer has. Run it on a computer with more memory.",
            grouped(devices),
            bytes(needed),
            bytes(memory.installed_bytes)
        )
    } else {
        format!(
            "This design has {} devices. Running it needs about {} of memory, and this computer \
             has {} free. Close other programs to free memory, or run it on a computer with more.",
            grouped(devices),
            bytes(needed),
            bytes(memory.free_bytes)
        )
    };
    Err(PreparationError::new(
        PreparationStage::SourceChecks,
        message,
    ))
}

/// `error` in the reader's words when the deck outgrew the counts
/// [`sized_to`] gave it for the memory installed: the flattening stops at the
/// count, so how far past it the design goes is not known.
pub(crate) fn too_big_for(error: PreparationError, memory: MachineMemory) -> PreparationError {
    let Some(limit) = error.resource_limit().copied() else {
        return error;
    };
    let what = match limit.resource {
        ResourceKind::FlattenedElements => "devices",
        ResourceKind::CircuitNodes => "nodes",
        ResourceKind::MatrixUnknowns => "unknowns to solve for",
        _ => return error,
    };
    error.said_as(format!(
        "This design has more than {} {what}. Running it needs more than the {} of memory this \
         computer has. Run it on a computer with more memory.",
        grouped(limit.limit),
        bytes(memory.installed_bytes)
    ))
}

/// `count` with its thousands grouped, `1,048,577`.
fn grouped(count: usize) -> String {
    let digits = count.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (at, digit) in digits.chars().enumerate() {
        if at > 0 && (digits.len() - at) % 3 == 0 {
            out.push(',');
        }
        out.push(digit);
    }
    out
}

/// `count` bytes as the operating system states memory, `3.1 GB`.
fn bytes(count: u64) -> String {
    const MB: f64 = 1024.0 * 1024.0;
    const GB: f64 = 1024.0 * MB;
    let count = count as f64;
    if count >= GB {
        format!("{:.1} GB", count / GB)
    } else {
        format!("{:.0} MB", (count / MB).max(1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GB: u64 = 1024 * 1024 * 1024;

    /// A machine with memory to spare sizes every count past the shipped
    /// one, by what each costs.
    #[test]
    fn the_counts_follow_the_memory_installed() {
        let shipped = ResourceLimits::default();
        let sized = sized_to(shipped, 16 * GB);
        assert_eq!(
            sized.max_flattened_elements as u64,
            16 * GB / RUN_BYTES_PER_DEVICE
        );
        assert_eq!(
            sized.max_circuit_nodes as u64,
            16 * GB / RUN_BYTES_PER_UNKNOWN
        );
        assert_eq!(
            sized.max_matrix_unknowns as u64,
            16 * GB / RUN_BYTES_PER_UNKNOWN
        );
        assert!(sized.max_flattened_elements > shipped.max_flattened_elements);
        // What a run reads from files and how deep it nests stay as shipped.
        assert_eq!(sized.max_netlist_bytes, shipped.max_netlist_bytes);
        assert_eq!(sized.max_hierarchy_depth, shipped.max_hierarchy_depth);
        assert_eq!(
            sized.max_external_data_bytes,
            shipped.max_external_data_bytes
        );
    }

    /// A small machine never narrows a count below the shipped one: a run
    /// it cannot hold is refused in words, by what it needs, instead.
    #[test]
    fn a_small_machine_keeps_the_shipped_counts() {
        assert_eq!(
            sized_to(ResourceLimits::default(), GB / 4),
            ResourceLimits::default()
        );
    }

    #[test]
    fn a_design_too_big_for_the_memory_free_says_what_it_needs_and_what_there_is() {
        let memory = MachineMemory {
            installed_bytes: 16 * GB,
            free_bytes: 3 * GB,
        };
        assert!(check_fits(1000, memory).is_ok());
        let refused =
            check_fits(1_048_577, memory).expect_err("five gigabytes do not fit in three");
        assert_eq!(
            refused.message(),
            "This design has 1,048,577 devices. Running it needs about 5.0 GB of memory, and \
             this computer has 3.0 GB free. Close other programs to free memory, or run it on a \
             computer with more."
        );
        let small = MachineMemory {
            installed_bytes: GB,
            free_bytes: GB / 2,
        };
        assert_eq!(
            check_fits(250_000, small)
                .expect_err("1.2 GB does not fit in 1 GB")
                .message(),
            "This design has 250,000 devices. Running it needs about 1.2 GB of memory, more than \
             the 1.0 GB this computer has. Run it on a computer with more memory."
        );
    }

    #[test]
    fn a_deck_that_outgrew_its_count_says_the_machine_cannot_hold_it() {
        let error = PreparationError::from_parse(
            PreparationStage::ModelBindings,
            "Executable hierarchy cannot authenticate project model use",
            rspice_core::netlist::ParseWithAbortError::Parse(
                rspice_core::netlist::ParseError::ResourceLimit(rspice_core::ResourceLimitError {
                    resource: ResourceKind::FlattenedElements,
                    requested: 3_355_444,
                    limit: 3_355_443,
                }),
            ),
        );
        let memory = MachineMemory {
            installed_bytes: 16 * GB,
            free_bytes: 3 * GB,
        };
        let refused = too_big_for(error, memory);
        assert_eq!(
            refused.message(),
            "This design has more than 3,355,443 devices. Running it needs more than the 16.0 GB \
             of memory this computer has. Run it on a computer with more memory."
        );
        assert!(
            refused.resource_limit().is_some(),
            "it stays a resource limit"
        );
    }
}

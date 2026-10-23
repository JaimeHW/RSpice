//! # RSpice Core Base
//!
//! The vocabulary every crate of the RSpice engine reads, and nothing in it
//! reads back up: constants and naming rules, configuration, numerics, the
//! sparse solver, the expression VM, the waveform readers and writers, and
//! the notices, diagnostics and limits a run reports through.
//!
//! `rspice-core` re-exports each module at the path it has always had, so a
//! frontend names these through `rspice_core` and never through this crate.
//! A module that `rspice-core` keeps out of its public API is
//! `#[doc(hidden)]` here: it is public only so that the crates of the engine
//! family above can reach it.

/// Abort signal for cancelling long-running simulations
pub mod abort_signal;
/// Starter model cards embedded in the binary; a leaf with no dependencies so
/// that library, netlist and engine can all share one copy.
#[doc(hidden)]
pub mod builtin_lib;
/// The typed refusal of a well-formed construct this build does not execute.
/// Below configuration and XSPICE, which raise one; published through
/// `analysis::error` and the crate root of `rspice-core`.
#[doc(hidden)]
pub mod capability;
/// Names of the XSPICE code models compiled in; a leaf so that both the parser
/// that classifies a name and the subsystem that instantiates it read down.
#[doc(hidden)]
pub mod codemodels;
/// Simulation configuration. Below everything that reads it, so a tolerance
/// or dialect flag is a downward reference rather than a reach into `engine`.
pub mod config;
pub mod constants;
pub mod diagnostics;
/// The names an event-only net is refused and published under.
#[doc(hidden)]
pub mod event_net;
pub mod expr;
#[doc(hidden)]
pub mod family_features;
/// Stable analysis and run-coordinate identities, named by both the engine's
/// error taxonomy and the deck planner. A leaf, so neither has to reach for
/// the other to say which analysis or sweep point a value belongs to.
pub mod identity;
pub mod io;
/// Which run a log record belongs to, carried onto the engine's worker threads.
/// A leaf: the analyses that hand work to another thread read down into it.
pub mod log_scope;
/// SPICE naming rules: what a name means in a deck, independent of dialect.
/// A leaf — everything else depends on it, so it depends on nothing.
#[doc(hidden)]
pub mod naming;
/// What a run raised and went on to complete with, in a shape a reader can
/// act on: the advisories the log states as prose.
pub mod notice;
pub mod numerics;
/// The vocabulary of labels a device operating-point report may carry. A leaf:
/// the device families that emit one and the circuit store that assembles the
/// report both read down into it.
pub mod op_label;
pub mod resource;
/// Typed per-analysis scalars, carried by analysis results, control-script
/// PRINT and result documents alike. Published under
/// `execution::result_document` of `rspice-core`.
#[doc(hidden)]
pub mod result_scalar;
/// Physical units carried by result schemas and control-expression vectors.
#[doc(hidden)]
pub mod signal_unit;
/// SIMD-accelerated operations (optional, requires `simd` feature)
#[cfg(feature = "simd")]
pub mod simd;
pub mod solver;
/// Where a line of deck text was written. A leaf, so a refusal raised below
/// the parser can cite one; published through `netlist` of `rspice-core`.
#[doc(hidden)]
pub mod source_location;
#[doc(hidden)]
pub mod spice_number;
pub mod time_compat;
#[doc(hidden)]
pub mod transient_observation;

family_features!();

// The names these modules read at the crate root, as they did inside
// `rspice-core`. Crate-private: `rspice-core` states its own.
pub(crate) use abort_signal::{AbortReason, AbortSignal};
#[cfg(test)]
pub(crate) use abort_signal::{AtomicAbort, NoAbort};
pub(crate) use capability::UnsupportedCapabilityError;
pub(crate) use num_complex::Complex64;
#[cfg(test)]
pub(crate) use resource::ResourceLimits;
pub(crate) use transient_observation::{CurrentImpulseTrace, VoltageImpulseTrace};

/// Simulation value type (using f64 for high precision)
pub(crate) type Value = f64;

/// Node identifier (0 = ground, always).
pub(crate) type NodeId = usize;

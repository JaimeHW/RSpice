//! # RSpice Core Netlist
//!
//! The engine's reading of a deck: the lexer, parser and AST, subcircuit
//! flattening, `.include` and `.lib` resolution, parameter scoping, model
//! libraries and Verilog-A pack discovery, and the vocabulary the
//! control-session machine shares with its engine host. It reads only the
//! base vocabulary below it, so a crate that reads decks and model
//! libraries does not wait for the device models or the engine.
//!
//! `rspice-core` re-exports each module at the path it has always had, so a
//! frontend names these through `rspice_core`. A module that `rspice-core`
//! keeps out of its public API is `#[doc(hidden)]` here: it is public only
//! so that the crates of the engine family above can reach it.

/// Shared vocabulary between the control-session machine and its engine host.
#[doc(hidden)]
pub mod control_protocol;
pub mod library;
pub mod netlist;

rspice_core_base::family_features!(rspice_core_base);

// The base modules these read at the crate root, as they did inside
// `rspice-core`. Crate-private: `rspice-core` states its own.
use rspice_core_base::{
    abort_signal, builtin_lib, codemodels, config, constants, expr, io, naming, numerics, op_label,
    resource, solver, source_location, spice_number,
};

// The names these modules read at the crate root, as they did inside
// `rspice-core`. Crate-private: `rspice-core` states its own.
#[cfg(test)]
pub(crate) use netlist::Netlist;
pub(crate) use rspice_core_base::abort_signal::NoAbort;
pub(crate) use rspice_core_base::resource::{ResourceKind, ResourceLimitError, ResourceLimits};

/// Simulation value type (using f64 for high precision)
pub(crate) type Value = f64;

/// Complex value type for AC analysis
pub(crate) type ComplexValue = num_complex::Complex64;

//! # RSpice Core Devices
//!
//! The engine's device models: passives, sources, diodes and BJTs, the
//! MOSFET and FET family, transmission lines, memristors, behavioral
//! sources, and the runtime adapter of the generated Verilog-A built-ins.
//! They read the base vocabulary and the parsed netlist below them and
//! nothing of the engine above, so they compile beside the Verilog-A
//! compiler instead of after it.
//!
//! `rspice-core` re-exports `device` at the path it has always had, with
//! the compiled Verilog-A device beside these models: that one needs the
//! compiler, so it stays in `rspice-core`.

pub mod device;

rspice_core_base::family_features!(rspice_core_base, rspice_core_netlist);

// The modules these read at the crate root, as they did inside
// `rspice-core`. Crate-private: `rspice-core` states its own.
use rspice_core_base::{
    abort_signal, config, constants, expr, log_scope, naming, notice, numerics, op_label, resource,
    solver,
};
use rspice_core_netlist::netlist;

pub(crate) use num_complex::Complex64;

/// Simulation value type (using f64 for high precision)
pub(crate) type Value = f64;

/// Node identifier (0 = ground, always).
pub(crate) type NodeId = usize;

/// Complex value type for AC analysis
#[cfg(test)]
pub(crate) type ComplexValue = Complex64;

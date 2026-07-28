//! Process-isolated conformance harnesses for RSpice.
//!
//! The simulator remains in [`rspice_core`]; this crate owns the regression
//! runners and their result protocol so production builds do not carry test
//! harness implementation details.

pub use rspice_core::{
    Complex64, ComplexValue, Engine, Netlist, SimulationResult, Value, abort_signal, analysis,
    circuit, engine, netlist, solver,
};

pub mod testing;

//! # RSpice Core
//!
//! High-performance SPICE circuit simulation engine.
//!
//! ## Architecture
//!
//! The simulation engine is organized into the following modules:
//!
//! - [`netlist`] - Parsing SPICE netlist format
//! - [`device`] - Device models (resistors, capacitors, transistors, etc.)
//! - [`circuit`] - Circuit representation with SoA storage
//! - [`solver`] - Sparse LU solver and Newton-Raphson iteration
//! - [`analysis`] - DC, AC, and Transient analysis engines
//! - [`engine`] - Main simulation pipeline
//!
//! ## Example
//!
//! ```rust,ignore
//! use rspice_core::{Netlist, Engine};
//!
//! let netlist = Netlist::parse("V1 1 0 10\nR1 1 0 1k\n.end")?;
//! let engine = Engine::default();
//! let result = engine.run_dc_op(&netlist)?;
//! println!("V(1) = {}", result.voltage(1));
//! ```

pub mod analysis;
pub mod circuit;
pub mod compat;
pub mod device;
pub mod engine;
pub mod expr;
pub mod library;
pub mod netlist;
pub mod solver;
pub mod testing;

/// SIMD-accelerated operations (optional, requires `simd` feature)
#[cfg(feature = "simd")]
pub mod simd;

// Re-export primary types for convenience
pub use analysis::{AcAnalysis, DcAnalysis, MeasureEngine, TransientAnalysis};
pub use circuit::{Circuit, CircuitData, Node, NodeId};
pub use device::{Device, DeviceModel};
pub use engine::{Engine, SimulationConfig, SimulationError};
pub use netlist::Netlist;
pub use solver::{SimulationResult, Simulator, SparseLuSolver, StaticMatrix, TripletMatrix};

/// Error types for the simulation engine
pub mod error {
    pub use crate::circuit::CircuitError;
    pub use crate::netlist::ParseError;
    pub use crate::solver::SolverError;
}

/// Simulation value type (using f64 for high precision)
pub type Value = f64;

/// Complex value type for AC analysis
/// Re-export from num_complex for convenience
pub use num_complex::Complex64;

/// Type alias for complex values used in AC analysis
pub type ComplexValue = Complex64;

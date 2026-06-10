//! # RSpice Python Bindings
//!
//! Commercial-grade Python bindings for the RSpice circuit simulation engine,
//! providing automation, scripting, and integration with the Python scientific ecosystem.
//!
//! ## Architecture
//!
//! The bindings are organized into focused modules:
//!
//! - [`netlist`] - Netlist parsing and manipulation
//! - [`engine`] - Simulation engine and analysis runners
//! - [`config`] - Simulation and convergence configuration
//! - [`results`] - Simulation results with NumPy array support
//! - [`errors`] - Python exception types
//!
//! ## Example
//!
//! ```python
//! import rspice
//!
//! netlist = rspice.Netlist.parse("V1 1 0 10\nR1 1 0 1k\n.end")
//! engine = rspice.Engine()
//! result = engine.run_dc_op(netlist)
//! print(f"V(1) = {result.voltage(1)} V")
//! ```

mod config;
mod engine;
mod errors;
mod netlist;
mod results;

use pyo3::prelude::*;

/// RSpice Python module - circuit simulation engine
///
/// This module provides Python bindings for the RSpice SPICE-compatible
/// circuit simulation engine, supporting DC, AC, and transient analysis.
#[pymodule]
fn rspice(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Version information
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", "RSpice Contributors")?;

    // Core classes
    m.add_class::<netlist::PyNetlist>()?;
    m.add_class::<engine::PyEngine>()?;

    // Configuration classes
    m.add_class::<config::PySimulationConfig>()?;
    m.add_class::<config::PyConvergenceConfig>()?;
    m.add_class::<config::PyBypassConfig>()?;
    m.add_class::<config::PyDampingStrategy>()?;

    // Result classes
    m.add_class::<results::PySimulationResult>()?;
    m.add_class::<results::PyTransientResult>()?;
    m.add_class::<results::PyAcResult>()?;
    m.add_class::<results::PyDcSweepResult>()?;
    m.add_class::<results::PyNoiseResult>()?;
    m.add_class::<results::PyNoiseContribution>()?;
    m.add_class::<results::PyMonteCarloResult>()?;
    m.add_class::<results::PyVariableStatistics>()?;
    m.add_class::<results::PyPoleZeroResult>()?;
    m.add_class::<results::PyComplexValue>()?;

    // Exception types
    m.add("RSpiceError", m.py().get_type::<errors::RSpiceError>())?;
    m.add("ParseError", m.py().get_type::<errors::ParseError>())?;
    m.add(
        "SimulationError",
        m.py().get_type::<errors::SimulationError>(),
    )?;
    m.add(
        "ConvergenceError",
        m.py().get_type::<errors::ConvergenceError>(),
    )?;

    m.add(
        "__all__",
        [
            "Netlist",
            "Engine",
            "SimulationConfig",
            "ConvergenceConfig",
            "BypassConfig",
            "DampingStrategy",
            "SimulationResult",
            "TransientResult",
            "AcResult",
            "DcSweepResult",
            "NoiseResult",
            "NoiseContribution",
            "MonteCarloResult",
            "VariableStatistics",
            "PoleZeroResult",
            "ComplexValue",
            "RSpiceError",
            "ParseError",
            "SimulationError",
            "ConvergenceError",
        ],
    )?;

    Ok(())
}

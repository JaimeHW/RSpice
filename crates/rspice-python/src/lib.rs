//! # RSpice Python Bindings
//!
//! Python bindings for the RSpice circuit simulation engine, built for
//! automation, scripting, and automated circuit verification (analog CI).
//!
//! ## Architecture
//!
//! The bindings are organized into focused modules:
//!
//! - [`netlist`] - Netlist parsing and introspection
//! - [`engine`] - Simulation engine and analysis runners
//! - [`config`] - Simulation and convergence configuration
//! - [`results`] - Simulation results with NumPy array support
//! - [`measure`] - .MEAS evaluation against simulation results
//! - [`abort`] - Ctrl-C cancellation plumbing
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

mod abort;
mod config;
mod engine;
mod errors;
mod measure;
mod netlist;
mod results;

use numpy::{PyArray1, ToPyArray};
use pyo3::prelude::*;

/// Generate AC sweep frequencies without running an analysis
///
/// Mirrors the `.AC DEC|OCT|LIN` frequency grids exactly, so Python-side
/// sweeps match netlist-directive sweeps point for point.
///
/// Args:
///     variation: "dec", "oct", or "lin"
///     points: Points per decade/octave, or total points for "lin"
///     start_freq: Sweep start frequency in Hz
///     stop_freq: Sweep stop frequency in Hz
///
/// Returns:
///     numpy.ndarray: Frequency points in Hz
///
/// Example:
///     >>> freqs = rspice.ac_frequencies("dec", 20, 1.0, 1e6)
#[pyfunction]
fn ac_frequencies<'py>(
    py: Python<'py>,
    variation: &str,
    points: usize,
    start_freq: f64,
    stop_freq: f64,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let variation = match variation.to_ascii_lowercase().as_str() {
        "dec" | "decade" => rspice_core::netlist::FreqVariation::Dec,
        "oct" | "octave" => rspice_core::netlist::FreqVariation::Oct,
        "lin" | "linear" => rspice_core::netlist::FreqVariation::Lin,
        other => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "variation must be 'dec', 'oct', or 'lin', got '{other}'"
            )));
        }
    };
    let frequencies =
        rspice_core::analysis::ac::ac_sweep_frequencies(variation, points, start_freq, stop_freq);
    if frequencies.is_empty() {
        return Err(pyo3::exceptions::PyValueError::new_err(format!(
            "invalid frequency sweep: {points} points from {start_freq} to {stop_freq} Hz"
        )));
    }
    Ok(frequencies.to_pyarray(py))
}

/// RSpice Python module - circuit simulation engine
///
/// This module provides Python bindings for the RSpice SPICE-compatible
/// circuit simulation engine, supporting DC, AC, transient, noise,
/// pole-zero, Monte Carlo, sensitivity, transfer-function, and Fourier
/// analyses, plus .MEAS-based automated verification.
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
    m.add_class::<config::PyIntegrationMethod>()?;

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
    m.add_class::<results::PyFourierResult>()?;
    m.add_class::<results::PyHarmonic>()?;
    m.add_class::<results::PyTransferFunctionResult>()?;
    m.add_class::<results::PyMeasurement>()?;
    m.add_class::<results::PyAnalysisRecord>()?;
    m.add_class::<results::PyRunReport>()?;

    // Module-level functions
    m.add_function(wrap_pyfunction!(ac_frequencies, m)?)?;

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
        "MeasurementError",
        m.py().get_type::<errors::MeasurementError>(),
    )?;

    // __all__ drives the package __init__'s star-import: dunders must be
    // listed explicitly or maturin's wrapper hides them.
    m.add(
        "__all__",
        [
            "__version__",
            "__author__",
            "Netlist",
            "Engine",
            "SimulationConfig",
            "ConvergenceConfig",
            "BypassConfig",
            "DampingStrategy",
            "IntegrationMethod",
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
            "FourierResult",
            "Harmonic",
            "TransferFunctionResult",
            "Measurement",
            "AnalysisRecord",
            "RunReport",
            "ac_frequencies",
            "RSpiceError",
            "ParseError",
            "SimulationError",
            "ConvergenceError",
            "MeasurementError",
        ],
    )?;

    Ok(())
}

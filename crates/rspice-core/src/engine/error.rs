//! Engine error types.

use thiserror::Error;
/// Simulation errors
#[derive(Debug, Error)]
pub enum SimulationError {
    #[error("Invalid simulation configuration: {0}")]
    Configuration(#[from] super::SimulationConfigError),

    #[error("Circuit error: {0}")]
    Circuit(String),

    #[error("Solver error: {0}")]
    Solver(#[from] crate::solver::SolverError),

    #[error("Netlist error: {0}")]
    Netlist(String),

    #[error("Convergence failed after {0} iterations")]
    ConvergenceFailed(usize),

    #[error("Simulation aborted by user")]
    Aborted,
}

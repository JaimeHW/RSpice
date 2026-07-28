//! Translating engine errors into user-facing ones.
//!
//! Engine diagnostics name internal nodes and matrix rows. This maps them
//! back onto the design objects a user can act on.

use super::EngineBridge;
use crate::simulation::runner::SimulationError;

impl EngineBridge {
    /// Translate core engine error to UI error.
    pub(super) fn translate_error(&self, err: rspice_core::SimulationError) -> SimulationError {
        match err {
            rspice_core::SimulationError::Configuration(
                rspice_core::SimulationConfigError::ResourceLimit(error),
            )
            | rspice_core::SimulationError::ResourceLimit(error) => {
                SimulationError::ResourceLimit {
                    resource: error.resource.as_str().to_string(),
                    requested: error.requested,
                    limit: error.limit,
                }
            }
            rspice_core::SimulationError::Configuration(error) => {
                SimulationError::InvalidConfig(error.to_string())
            }
            rspice_core::SimulationError::Circuit(msg) => SimulationError::CircuitError(msg),
            rspice_core::SimulationError::Solver(solver_err) => {
                SimulationError::SolverError(solver_err.to_string())
            }
            rspice_core::SimulationError::Netlist(msg) => SimulationError::ParseError(msg),
            rspice_core::SimulationError::ConvergenceFailed(iterations) => {
                SimulationError::ConvergenceFailed {
                    iterations,
                    message: "Newton-Raphson iteration limit exceeded".to_string(),
                }
            }
            rspice_core::SimulationError::Aborted => SimulationError::Aborted,
        }
    }
}

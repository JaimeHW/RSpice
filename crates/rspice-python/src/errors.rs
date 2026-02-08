//! Python exception types for RSpice errors
//!
//! Provides properly typed Python exceptions for different error conditions:
//! - `RSpiceError` - Base exception for all RSpice errors
//! - `ParseError` - Netlist parsing failures
//! - `SimulationError` - General simulation failures
//! - `ConvergenceError` - Newton-Raphson convergence failures

use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

// Base exception for all RSpice errors
create_exception!(
    rspice,
    RSpiceError,
    PyException,
    "Base exception for all RSpice errors."
);

// Netlist parsing errors
create_exception!(
    rspice,
    ParseError,
    RSpiceError,
    "Raised when netlist parsing fails due to syntax or semantic errors."
);

// General simulation errors
create_exception!(
    rspice,
    SimulationError,
    RSpiceError,
    "Raised when simulation fails due to circuit or solver errors."
);

// Convergence failures
create_exception!(
    rspice,
    ConvergenceError,
    SimulationError,
    "Raised when Newton-Raphson iteration fails to converge."
);

/// Convert a parse error to PyErr
pub fn parse_error_to_pyerr(err: rspice_core::netlist::ParseError) -> PyErr {
    ParseError::new_err(err.to_string())
}

/// Convert a simulation error to PyErr
pub fn simulation_error_to_pyerr(err: rspice_core::engine::SimulationError) -> PyErr {
    match &err {
        rspice_core::engine::SimulationError::ConvergenceFailed(_) => {
            ConvergenceError::new_err(err.to_string())
        }
        _ => SimulationError::new_err(err.to_string()),
    }
}

/// Convert a solver error to PyErr
pub fn solver_error_to_pyerr(err: rspice_core::solver::SolverError) -> PyErr {
    match &err {
        rspice_core::solver::SolverError::ConvergenceFailed(_) => {
            ConvergenceError::new_err(err.to_string())
        }
        _ => SimulationError::new_err(err.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use rspice_core::engine::SimulationError;
    use rspice_core::netlist::ParseError;
    use rspice_core::solver::SolverError;

    // Note: PyErr creation requires a Python GIL, so we test the underlying errors only

    #[test]
    fn test_parse_error_display() {
        let err = ParseError::Syntax {
            line: 1,
            message: "Invalid syntax".to_string(),
        };
        let msg = err.to_string();
        assert!(msg.contains("Invalid syntax") || msg.contains("line 1"));
    }

    #[test]
    fn test_simulation_error_display() {
        let err = SimulationError::Circuit("No nodes in circuit".to_string());
        let msg = err.to_string();
        assert!(msg.contains("No nodes") || msg.contains("circuit"));
    }

    #[test]
    fn test_convergence_error_display() {
        let err = SimulationError::ConvergenceFailed(50);
        let msg = err.to_string();
        assert!(msg.contains("50") || msg.contains("converg"));
    }

    #[test]
    fn test_solver_convergence_error_display() {
        let err = SolverError::ConvergenceFailed(100);
        let msg = err.to_string();
        assert!(msg.contains("100") || msg.contains("converg"));
    }

    #[test]
    fn test_solver_singular_matrix_display() {
        let err = SolverError::SingularMatrix;
        let msg = err.to_string().to_lowercase();
        assert!(msg.contains("singular") || msg.contains("matrix"));
    }
}

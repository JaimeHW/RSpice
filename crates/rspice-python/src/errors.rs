//! Python exception types for RSpice errors
//!
//! Provides properly typed Python exceptions for different error conditions:
//! - `RSpiceError` - Base exception for all RSpice errors
//! - `ParseError` - Netlist parsing failures
//! - `SimulationError` - General simulation failures
//! - `ConvergenceError` - Newton-Raphson convergence failures
//! - `CancelledError` - Programmatic simulation cancellation
//! - `MeasurementError` - Failed .MEAS verification (raised by
//!   `RunReport.assert_passed`)

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

// Programmatic cancellation (Ctrl-C remains KeyboardInterrupt).
create_exception!(
    rspice,
    CancelledError,
    SimulationError,
    "Raised in a simulation's calling thread after Engine.cancel()."
);

// Measurement verification failures
create_exception!(
    rspice,
    MeasurementError,
    RSpiceError,
    "Raised when .MEAS verification fails (see RunReport.assert_passed)."
);

/// Convert a parse error to PyErr
pub fn parse_error_to_pyerr(err: rspice_core::netlist::ParseError) -> PyErr {
    use rspice_core::netlist::{MissingSubcircuitEndsBoundary, ParseError as CoreParseError};

    let message = err.to_string();
    let (
        kind,
        line,
        detail,
        source,
        detected_line,
        detected_source,
        boundary,
        authored_name,
        canonical_name,
        qualified_name,
    ) = match &err {
        CoreParseError::Syntax { line, message } => (
            "syntax",
            Some(*line),
            Some(message.clone()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        CoreParseError::UnknownDevice(value) => (
            "unknown_device",
            None,
            Some(value.clone()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        CoreParseError::InvalidNode(value) => (
            "invalid_node",
            None,
            Some(value.clone()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        CoreParseError::DuplicateName {
            canonical_name,
            duplicate_line,
            ..
        } => (
            "duplicate_name",
            Some(*duplicate_line),
            Some(canonical_name.clone()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        CoreParseError::MissingSubcircuitEnds {
            authored_name,
            canonical_name,
            qualified_name,
            opened_at,
            detected_at,
            boundary,
        } => (
            "missing_subcircuit_ends",
            Some(opened_at.line),
            Some(canonical_name.clone()),
            opened_at
                .path
                .as_ref()
                .map(|path| path.to_string_lossy().into_owned()),
            Some(detected_at.line),
            detected_at
                .path
                .as_ref()
                .map(|path| path.to_string_lossy().into_owned()),
            Some(
                match boundary {
                    MissingSubcircuitEndsBoundary::EndCard => "end_card",
                    MissingSubcircuitEndsBoundary::AlterCard => "alter_card",
                    MissingSubcircuitEndsBoundary::EndOfSource => "end_of_source",
                }
                .to_string(),
            ),
            Some(authored_name.clone()),
            Some(canonical_name.clone()),
            Some(qualified_name.clone()),
        ),
        CoreParseError::MissingParameter(value) => (
            "missing_parameter",
            None,
            Some(value.clone()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        CoreParseError::UndefinedParameter(value) => (
            "undefined_parameter",
            None,
            Some(value.clone()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        CoreParseError::InvalidValue(value) => (
            "invalid_value",
            None,
            Some(value.clone()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        ),
        CoreParseError::Io(_) => ("io", None, None, None, None, None, None, None, None, None),
    };
    let error = ParseError::new_err(message);
    let _attribute_result = Python::attach(|py| {
        let value = error.value(py);
        value.setattr("kind", kind)?;
        value.setattr("line", line)?;
        value.setattr("detail", detail)?;
        value.setattr("source", source)?;
        value.setattr("detected_line", detected_line)?;
        value.setattr("detected_source", detected_source)?;
        value.setattr("boundary", boundary)?;
        value.setattr("authored_name", authored_name)?;
        value.setattr("canonical_name", canonical_name)?;
        value.setattr("qualified_name", qualified_name)?;
        Ok::<_, PyErr>(())
    });
    error
}

/// Convert a simulation error to PyErr
pub fn simulation_error_to_pyerr(err: rspice_core::engine::SimulationError) -> PyErr {
    use rspice_core::engine::SimulationError as CoreSimulationError;

    let (kind, iterations) = match &err {
        CoreSimulationError::Circuit(_) => ("circuit", None),
        CoreSimulationError::Solver(_) => ("solver", None),
        CoreSimulationError::Netlist(_) => ("netlist", None),
        CoreSimulationError::ConvergenceFailed(iterations) => ("convergence", Some(*iterations)),
        CoreSimulationError::Aborted => ("aborted", None),
    };
    let error = match &err {
        CoreSimulationError::ConvergenceFailed(_) => ConvergenceError::new_err(err.to_string()),
        CoreSimulationError::Aborted => CancelledError::new_err(err.to_string()),
        _ => SimulationError::new_err(err.to_string()),
    };
    let _attribute_result = Python::attach(|py| {
        let value = error.value(py);
        value.setattr("kind", kind)?;
        value.setattr("iterations", iterations)?;
        Ok::<_, PyErr>(())
    });
    error
}

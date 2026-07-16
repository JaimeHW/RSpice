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

#[derive(Default)]
struct ParseErrorAttributes {
    kind: &'static str,
    category: Option<&'static str>,
    line: Option<usize>,
    detail: Option<String>,
    source: Option<String>,
    primary_line: Option<usize>,
    primary_source: Option<String>,
    related_line: Option<usize>,
    related_source: Option<String>,
    detected_line: Option<usize>,
    detected_source: Option<String>,
    boundary: Option<String>,
    authored_name: Option<String>,
    canonical_name: Option<String>,
    qualified_name: Option<String>,
    device: Option<String>,
    requested_path: Option<String>,
    value_index: Option<usize>,
    value: Option<f64>,
    expected: Option<String>,
    actual: Option<usize>,
    device_type: Option<String>,
}

impl ParseErrorAttributes {
    fn new(kind: &'static str) -> Self {
        Self {
            kind,
            ..Self::default()
        }
    }

    fn set_primary(&mut self, location: &rspice_core::netlist::NetlistSourceLocation) {
        self.line = Some(location.line);
        self.source = location
            .path
            .as_ref()
            .map(|path| path.to_string_lossy().into_owned());
        self.primary_line = self.line;
        self.primary_source = self.source.clone();
    }

    fn set_related(&mut self, location: &rspice_core::netlist::NetlistSourceLocation) {
        self.related_line = Some(location.line);
        self.related_source = location
            .path
            .as_ref()
            .map(|path| path.to_string_lossy().into_owned());
    }
}

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
    use rspice_core::netlist::{
        DeviceInitialConditionError, MissingSubcircuitEndsBoundary, ParseError as CoreParseError,
    };

    let message = err.to_string();
    let mut attributes = match &err {
        CoreParseError::Syntax { line, message } => {
            let mut attributes = ParseErrorAttributes::new("syntax");
            attributes.line = Some(*line);
            attributes.primary_line = Some(*line);
            attributes.detail = Some(message.clone());
            attributes
        }
        CoreParseError::UnknownDevice(value) => {
            let mut attributes = ParseErrorAttributes::new("unknown_device");
            attributes.detail = Some(value.clone());
            attributes
        }
        CoreParseError::InvalidNode(value) => {
            let mut attributes = ParseErrorAttributes::new("invalid_node");
            attributes.detail = Some(value.clone());
            attributes
        }
        CoreParseError::DuplicateName {
            canonical_name,
            duplicate_line,
            ..
        } => {
            let mut attributes = ParseErrorAttributes::new("duplicate_name");
            attributes.line = Some(*duplicate_line);
            attributes.primary_line = Some(*duplicate_line);
            attributes.detail = Some(canonical_name.clone());
            attributes
        }
        CoreParseError::MissingSubcircuitEnds(error) => {
            let mut attributes = ParseErrorAttributes::new("missing_subcircuit_ends");
            attributes.set_primary(&error.opened_at);
            attributes.set_related(&error.detected_at);
            attributes.detail = Some(error.canonical_name.clone());
            attributes.detected_line = Some(error.detected_at.line);
            attributes.detected_source = error
                .detected_at
                .path
                .as_ref()
                .map(|path| path.to_string_lossy().into_owned());
            attributes.boundary = Some(
                match error.boundary {
                    MissingSubcircuitEndsBoundary::EndCard => "end_card",
                    MissingSubcircuitEndsBoundary::AlterCard => "alter_card",
                    MissingSubcircuitEndsBoundary::EndOfSource => "end_of_source",
                }
                .to_string(),
            );
            attributes.authored_name = Some(error.authored_name.clone());
            attributes.canonical_name = Some(error.canonical_name.clone());
            attributes.qualified_name = Some(error.qualified_name.clone());
            attributes
        }
        CoreParseError::DeviceInitialCondition(error) => {
            let mut attributes = ParseErrorAttributes::new("device_initial_condition");
            attributes.category = Some("device_initial_condition");
            match error.as_ref() {
                DeviceInitialConditionError::DuplicateDirective { first, duplicate } => {
                    attributes.kind = "device_initial_condition_duplicate_directive";
                    attributes.set_primary(duplicate);
                    attributes.set_related(first);
                }
                DeviceInitialConditionError::MissingInformation { origin } => {
                    attributes.kind = "device_initial_condition_missing_information";
                    attributes.set_primary(origin);
                }
                DeviceInitialConditionError::MalformedDirective { origin, detail } => {
                    attributes.kind = "device_initial_condition_malformed_directive";
                    attributes.set_primary(origin);
                    attributes.detail = Some(detail.clone());
                }
                DeviceInitialConditionError::SourceUnavailable {
                    origin,
                    requested_path,
                } => {
                    attributes.kind = "device_initial_condition_source_unavailable";
                    attributes.set_primary(origin);
                    attributes.requested_path = Some(requested_path.clone());
                }
                DeviceInitialConditionError::MalformedSource {
                    origin,
                    requested_path,
                    record_origin,
                    detail,
                } => {
                    attributes.kind = "device_initial_condition_malformed_source";
                    attributes.set_primary(record_origin);
                    attributes.set_related(origin);
                    attributes.requested_path = Some(requested_path.clone());
                    attributes.detail = Some(detail.clone());
                }
                DeviceInitialConditionError::NonFiniteValue {
                    origin,
                    device,
                    value_index,
                    value,
                } => {
                    attributes.kind = "device_initial_condition_nonfinite_value";
                    attributes.set_primary(origin);
                    attributes.device = Some(device.clone());
                    attributes.value_index = Some(*value_index);
                    attributes.value = Some(*value);
                }
                DeviceInitialConditionError::UnresolvedSource {
                    origin,
                    requested_path,
                } => {
                    attributes.kind = "device_initial_condition_unresolved_source";
                    attributes.set_primary(origin);
                    attributes.requested_path = Some(requested_path.clone());
                }
                DeviceInitialConditionError::InvalidArity {
                    origin,
                    device,
                    expected,
                    actual,
                } => {
                    attributes.kind = "device_initial_condition_invalid_arity";
                    attributes.set_primary(origin);
                    attributes.device = Some(device.clone());
                    attributes.expected = Some(expected.clone());
                    attributes.actual = Some(*actual);
                }
                DeviceInitialConditionError::UnsupportedTarget {
                    origin,
                    device,
                    device_type,
                } => {
                    attributes.kind = "device_initial_condition_unsupported_target";
                    attributes.set_primary(origin);
                    attributes.device = Some(device.clone());
                    attributes.device_type = Some(device_type.clone());
                }
            }
            attributes
        }
        CoreParseError::MissingParameter(value) => {
            let mut attributes = ParseErrorAttributes::new("missing_parameter");
            attributes.detail = Some(value.clone());
            attributes
        }
        CoreParseError::UndefinedParameter(value) => {
            let mut attributes = ParseErrorAttributes::new("undefined_parameter");
            attributes.detail = Some(value.clone());
            attributes
        }
        CoreParseError::InvalidValue(value) => {
            let mut attributes = ParseErrorAttributes::new("invalid_value");
            attributes.detail = Some(value.clone());
            attributes
        }
        CoreParseError::Io(_) => ParseErrorAttributes::new("io"),
    };
    if attributes.primary_line.is_none() {
        attributes.primary_line = attributes.line;
        attributes.primary_source = attributes.source.clone();
    }
    let error = ParseError::new_err(message);
    let _attribute_result = Python::attach(|py| {
        let value = error.value(py);
        value.setattr("kind", attributes.kind)?;
        value.setattr("category", attributes.category)?;
        value.setattr("line", attributes.line)?;
        value.setattr("detail", attributes.detail)?;
        value.setattr("source", attributes.source)?;
        value.setattr("primary_line", attributes.primary_line)?;
        value.setattr("primary_source", attributes.primary_source)?;
        value.setattr("related_line", attributes.related_line)?;
        value.setattr("related_source", attributes.related_source)?;
        value.setattr("detected_line", attributes.detected_line)?;
        value.setattr("detected_source", attributes.detected_source)?;
        value.setattr("boundary", attributes.boundary)?;
        value.setattr("authored_name", attributes.authored_name)?;
        value.setattr("canonical_name", attributes.canonical_name)?;
        value.setattr("qualified_name", attributes.qualified_name)?;
        value.setattr("device", attributes.device)?;
        value.setattr("requested_path", attributes.requested_path)?;
        value.setattr("value_index", attributes.value_index)?;
        value.setattr("value", attributes.value)?;
        value.setattr("expected", attributes.expected)?;
        value.setattr("actual", attributes.actual)?;
        value.setattr("device_type", attributes.device_type)?;
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

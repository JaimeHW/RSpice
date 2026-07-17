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

/// One unresolved circuit symbol retained by an aggregate output-validation
/// error. The string-valued tags are deliberately stable across core enum
/// evolution so Python automation does not need to parse display messages.
#[pyclass(
    name = "UnresolvedOutputSymbol",
    module = "rspice",
    frozen,
    from_py_object
)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyUnresolvedOutputSymbol {
    #[pyo3(get)]
    pub directive: String,
    #[pyo3(get)]
    pub operator: String,
    #[pyo3(get)]
    pub symbol: String,
    #[pyo3(get)]
    pub kind: String,
    #[pyo3(get)]
    pub line: usize,
    #[pyo3(get)]
    pub source: Option<String>,
}

impl From<&rspice_core::netlist::UnresolvedOutputSymbol> for PyUnresolvedOutputSymbol {
    fn from(item: &rspice_core::netlist::UnresolvedOutputSymbol) -> Self {
        use rspice_core::netlist::{OutputDirectiveKind, OutputSymbolKind};

        let directive = match item.directive {
            OutputDirectiveKind::Save => "save",
            OutputDirectiveKind::Probe => "probe",
            OutputDirectiveKind::Print => "print",
            OutputDirectiveKind::Plot => "plot",
            OutputDirectiveKind::Measure => "measure",
            OutputDirectiveKind::Four => "four",
        };
        let kind = match item.kind {
            OutputSymbolKind::Node => "node",
            OutputSymbolKind::Device => "device",
        };
        Self {
            directive: directive.to_string(),
            operator: item.operator.clone(),
            symbol: item.symbol.clone(),
            kind: kind.to_string(),
            line: item.origin.line,
            source: item
                .origin
                .path
                .as_ref()
                .map(|path| path.to_string_lossy().into_owned()),
        }
    }
}

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
    subcircuit_name: Option<String>,
    canonical_subcircuit_name: Option<String>,
    instance_name: Option<String>,
    canonical_instance_name: Option<String>,
    qualified_instance_name: Option<String>,
    formal_port: Option<String>,
    first_position: Option<usize>,
    conflicting_position: Option<usize>,
    first_actual_node: Option<String>,
    conflicting_actual_node: Option<String>,
    position: Option<usize>,
    actual_node: Option<String>,
    authored_coupling_name: Option<String>,
    canonical_coupling_name: Option<String>,
    qualified_coupling_name: Option<String>,
    authored_inductor_name: Option<String>,
    canonical_inductor_name: Option<String>,
    qualified_inductor_name: Option<String>,
    scope_name: Option<String>,
    reference_position: Option<usize>,
    device: Option<String>,
    requested_path: Option<String>,
    value_index: Option<usize>,
    value: Option<f64>,
    expected: Option<String>,
    actual: Option<usize>,
    device_type: Option<String>,
    unresolved_output_symbols: Option<Vec<PyUnresolvedOutputSymbol>>,
    first_startup_kind: Option<String>,
    conflicting_startup_kind: Option<String>,
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

fn duplicate_subcircuit_binding_attributes(
    error: &rspice_core::netlist::DuplicateSubcircuitPortBindingError,
) -> ParseErrorAttributes {
    let mut attributes = ParseErrorAttributes::new("duplicate_subcircuit_port_binding");
    attributes.category = Some("subcircuit_binding");
    attributes.detail = Some(error.formal_port.clone());
    attributes.authored_name = Some(error.instance_name.clone());
    attributes.canonical_name = Some(error.canonical_instance_name.clone());
    attributes.qualified_name = Some(error.qualified_instance_name.clone());
    attributes.subcircuit_name = Some(error.subcircuit_name.clone());
    attributes.canonical_subcircuit_name = Some(error.canonical_subcircuit_name.clone());
    attributes.instance_name = Some(error.instance_name.clone());
    attributes.canonical_instance_name = Some(error.canonical_instance_name.clone());
    attributes.qualified_instance_name = Some(error.qualified_instance_name.clone());
    attributes.formal_port = Some(error.formal_port.clone());
    attributes.first_position = Some(error.first_position);
    attributes.conflicting_position = Some(error.conflicting_position);
    attributes.first_actual_node = Some(error.first_actual_node.clone());
    attributes.conflicting_actual_node = Some(error.conflicting_actual_node.clone());
    attributes
}

fn global_subcircuit_binding_attributes(
    error: &rspice_core::netlist::GlobalSubcircuitPortBindingError,
) -> ParseErrorAttributes {
    let mut attributes = ParseErrorAttributes::new("global_subcircuit_port_binding");
    attributes.category = Some("subcircuit_binding");
    attributes.detail = Some(error.formal_port.clone());
    attributes.authored_name = Some(error.instance_name.clone());
    attributes.canonical_name = Some(error.canonical_instance_name.clone());
    attributes.qualified_name = Some(error.qualified_instance_name.clone());
    attributes.subcircuit_name = Some(error.subcircuit_name.clone());
    attributes.canonical_subcircuit_name = Some(error.canonical_subcircuit_name.clone());
    attributes.instance_name = Some(error.instance_name.clone());
    attributes.canonical_instance_name = Some(error.canonical_instance_name.clone());
    attributes.qualified_instance_name = Some(error.qualified_instance_name.clone());
    attributes.formal_port = Some(error.formal_port.clone());
    attributes.position = Some(error.position);
    attributes.actual_node = Some(error.actual_node.clone());
    attributes
}

fn undefined_mutual_inductor_reference_attributes(
    error: &rspice_core::netlist::UndefinedMutualInductorReferenceError,
) -> ParseErrorAttributes {
    let mut attributes = ParseErrorAttributes::new("undefined_mutual_inductor_reference");
    attributes.category = Some("mutual_inductor_reference");
    attributes.set_primary(&error.origin);
    attributes.detail = Some(error.authored_inductor_name.clone());
    attributes.authored_name = Some(error.authored_coupling_name.clone());
    attributes.canonical_name = Some(error.canonical_coupling_name.clone());
    attributes.qualified_name = Some(error.qualified_coupling_name.clone());
    attributes.authored_coupling_name = Some(error.authored_coupling_name.clone());
    attributes.canonical_coupling_name = Some(error.canonical_coupling_name.clone());
    attributes.qualified_coupling_name = Some(error.qualified_coupling_name.clone());
    attributes.authored_inductor_name = Some(error.authored_inductor_name.clone());
    attributes.canonical_inductor_name = Some(error.canonical_inductor_name.clone());
    attributes.qualified_inductor_name = Some(error.qualified_inductor_name.clone());
    attributes.scope_name = error.scope_name.clone();
    attributes.reference_position = Some(error.reference_position);
    attributes
}

fn output_symbol_validation_attributes(
    error: &rspice_core::netlist::OutputSymbolValidationError,
) -> ParseErrorAttributes {
    let mut attributes = ParseErrorAttributes::new("undefined_output_symbols");
    attributes.category = Some("output_symbol_validation");
    let count = error.unresolved.len();
    attributes.detail = Some(format!(
        "{count} unresolved output symbol{}",
        if count == 1 { "" } else { "s" }
    ));
    attributes.unresolved_output_symbols = Some(error.unresolved.iter().map(Into::into).collect());
    if let Some(first) = error.unresolved.first() {
        attributes.set_primary(&first.origin);
    }
    attributes
}

fn startup_directive_kind_name(kind: rspice_core::netlist::StartupDirectiveKind) -> &'static str {
    match kind {
        rspice_core::netlist::StartupDirectiveKind::Ic => "ic",
        rspice_core::netlist::StartupDirectiveKind::NodeSet => "nodeset",
    }
}

fn startup_directive_conflict_attributes(
    error: &rspice_core::netlist::StartupDirectiveConflictError,
) -> ParseErrorAttributes {
    let mut attributes = ParseErrorAttributes::new("conflicting_startup_directives");
    attributes.category = Some("startup_directive_validation");
    attributes.set_primary(&error.first);
    attributes.set_related(&error.conflicting);
    attributes.first_startup_kind = Some(startup_directive_kind_name(error.first_kind).to_string());
    attributes.conflicting_startup_kind =
        Some(startup_directive_kind_name(error.conflicting_kind).to_string());
    attributes
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
        CoreParseError::DuplicateSubcircuitPortBinding(error) => {
            duplicate_subcircuit_binding_attributes(error)
        }
        CoreParseError::GlobalSubcircuitPortBinding(error) => {
            global_subcircuit_binding_attributes(error)
        }
        CoreParseError::UndefinedMutualInductorReference(error) => {
            undefined_mutual_inductor_reference_attributes(error)
        }
        CoreParseError::OutputSymbolValidation(error) => output_symbol_validation_attributes(error),
        CoreParseError::StartupDirectiveConflict(error) => {
            startup_directive_conflict_attributes(error)
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
        value.setattr("subcircuit_name", attributes.subcircuit_name)?;
        value.setattr(
            "canonical_subcircuit_name",
            attributes.canonical_subcircuit_name,
        )?;
        value.setattr("instance_name", attributes.instance_name)?;
        value.setattr(
            "canonical_instance_name",
            attributes.canonical_instance_name,
        )?;
        value.setattr(
            "qualified_instance_name",
            attributes.qualified_instance_name,
        )?;
        value.setattr("formal_port", attributes.formal_port)?;
        value.setattr("first_position", attributes.first_position)?;
        value.setattr("conflicting_position", attributes.conflicting_position)?;
        value.setattr("first_actual_node", attributes.first_actual_node)?;
        value.setattr(
            "conflicting_actual_node",
            attributes.conflicting_actual_node,
        )?;
        value.setattr("position", attributes.position)?;
        value.setattr("actual_node", attributes.actual_node)?;
        value.setattr("authored_coupling_name", attributes.authored_coupling_name)?;
        value.setattr(
            "canonical_coupling_name",
            attributes.canonical_coupling_name,
        )?;
        value.setattr(
            "qualified_coupling_name",
            attributes.qualified_coupling_name,
        )?;
        value.setattr("authored_inductor_name", attributes.authored_inductor_name)?;
        value.setattr(
            "canonical_inductor_name",
            attributes.canonical_inductor_name,
        )?;
        value.setattr(
            "qualified_inductor_name",
            attributes.qualified_inductor_name,
        )?;
        value.setattr("scope_name", attributes.scope_name)?;
        value.setattr("reference_position", attributes.reference_position)?;
        value.setattr("device", attributes.device)?;
        value.setattr("requested_path", attributes.requested_path)?;
        value.setattr("value_index", attributes.value_index)?;
        value.setattr("value", attributes.value)?;
        value.setattr("expected", attributes.expected)?;
        value.setattr("actual", attributes.actual)?;
        value.setattr("device_type", attributes.device_type)?;
        value.setattr(
            "unresolved_output_symbols",
            attributes.unresolved_output_symbols,
        )?;
        value.setattr("first_startup_kind", attributes.first_startup_kind)?;
        value.setattr(
            "conflicting_startup_kind",
            attributes.conflicting_startup_kind,
        )?;
        Ok::<_, PyErr>(())
    });
    error
}

/// Convert a simulation error to PyErr
pub fn simulation_error_to_pyerr(err: rspice_core::engine::SimulationError) -> PyErr {
    use rspice_core::engine::SimulationError as CoreSimulationError;

    let (kind, iterations) = match &err {
        CoreSimulationError::Configuration(_) => ("configuration", None),
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

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::netlist::{
        DuplicateSubcircuitPortBindingError, GlobalSubcircuitPortBindingError,
        NetlistSourceLocation, OutputDirectiveKind, OutputSymbolKind, OutputSymbolValidationError,
        StartupDirectiveConflictError, StartupDirectiveKind, UndefinedMutualInductorReferenceError,
        UnresolvedOutputSymbol,
    };

    #[test]
    fn duplicate_subcircuit_binding_exposes_structured_python_attributes() {
        let error = DuplicateSubcircuitPortBindingError {
            subcircuit_name: "inv1".into(),
            canonical_subcircuit_name: "INV1".into(),
            instance_name: "Xinv1".into(),
            canonical_instance_name: "XINV1".into(),
            qualified_instance_name: "TOP.Xinv1".into(),
            formal_port: "GND".into(),
            first_position: 4,
            conflicting_position: 8,
            first_actual_node: "0".into(),
            conflicting_actual_node: "VDD".into(),
        };
        let attributes = duplicate_subcircuit_binding_attributes(&error);

        assert_eq!(attributes.kind, "duplicate_subcircuit_port_binding");
        assert_eq!(attributes.category, Some("subcircuit_binding"));
        assert_eq!(attributes.subcircuit_name.as_deref(), Some("inv1"));
        assert_eq!(
            attributes.canonical_subcircuit_name.as_deref(),
            Some("INV1")
        );
        assert_eq!(attributes.canonical_instance_name.as_deref(), Some("XINV1"));
        assert_eq!(attributes.first_position, Some(4));
        assert_eq!(attributes.conflicting_actual_node.as_deref(), Some("VDD"));
    }

    #[test]
    fn global_subcircuit_binding_exposes_structured_python_attributes() {
        let error = GlobalSubcircuitPortBindingError {
            subcircuit_name: "cell".into(),
            canonical_subcircuit_name: "CELL".into(),
            instance_name: "X1".into(),
            canonical_instance_name: "X1".into(),
            qualified_instance_name: "TOP.X1".into(),
            formal_port: "$G_SHARED".into(),
            position: 1,
            actual_node: "LOCAL".into(),
        };
        let attributes = global_subcircuit_binding_attributes(&error);

        assert_eq!(attributes.kind, "global_subcircuit_port_binding");
        assert_eq!(attributes.subcircuit_name.as_deref(), Some("cell"));
        assert_eq!(
            attributes.canonical_subcircuit_name.as_deref(),
            Some("CELL")
        );
        assert_eq!(attributes.formal_port.as_deref(), Some("$G_SHARED"));
        assert_eq!(attributes.actual_node.as_deref(), Some("LOCAL"));
    }

    #[test]
    fn undefined_mutual_inductor_exposes_structured_python_attributes() {
        let error = UndefinedMutualInductorReferenceError {
            origin: NetlistSourceLocation::in_file("bug75.cir", 12),
            authored_coupling_name: "K3".into(),
            canonical_coupling_name: "K3".into(),
            qualified_coupling_name: "K3".into(),
            authored_inductor_name: "L2".into(),
            canonical_inductor_name: "L2".into(),
            qualified_inductor_name: "L2".into(),
            scope_name: None,
            reference_position: 2,
        };
        let attributes = undefined_mutual_inductor_reference_attributes(&error);

        assert_eq!(attributes.kind, "undefined_mutual_inductor_reference");
        assert_eq!(attributes.category, Some("mutual_inductor_reference"));
        assert_eq!(attributes.line, Some(12));
        assert_eq!(attributes.source.as_deref(), Some("bug75.cir"));
        assert_eq!(attributes.authored_coupling_name.as_deref(), Some("K3"));
        assert_eq!(attributes.authored_inductor_name.as_deref(), Some("L2"));
        assert_eq!(attributes.reference_position, Some(2));
    }

    #[test]
    fn output_symbol_validation_preserves_order_repetitions_and_provenance() {
        let item = |operator: &str, symbol: &str, kind| UnresolvedOutputSymbol {
            directive: OutputDirectiveKind::Print,
            origin: NetlistSourceLocation::in_file("invalid.cir", 17),
            operator: operator.into(),
            symbol: symbol.into(),
            kind,
        };
        let error = OutputSymbolValidationError {
            unresolved: vec![
                item("I", "RBogo", OutputSymbolKind::Device),
                item("VP", "bogo9", OutputSymbolKind::Node),
                item("VM", "bogo9", OutputSymbolKind::Node),
            ],
        };
        let attributes = output_symbol_validation_attributes(&error);

        assert_eq!(attributes.kind, "undefined_output_symbols");
        assert_eq!(attributes.category, Some("output_symbol_validation"));
        assert_eq!(attributes.line, Some(17));
        assert_eq!(attributes.source.as_deref(), Some("invalid.cir"));
        assert_eq!(
            attributes.detail.as_deref(),
            Some("3 unresolved output symbols")
        );
        let unresolved = attributes
            .unresolved_output_symbols
            .expect("aggregate items are exposed");
        assert_eq!(
            unresolved
                .iter()
                .map(|item| {
                    (
                        item.directive.as_str(),
                        item.operator.as_str(),
                        item.symbol.as_str(),
                        item.kind.as_str(),
                        item.line,
                        item.source.as_deref(),
                    )
                })
                .collect::<Vec<_>>(),
            vec![
                ("print", "I", "RBogo", "device", 17, Some("invalid.cir")),
                ("print", "VP", "bogo9", "node", 17, Some("invalid.cir")),
                ("print", "VM", "bogo9", "node", 17, Some("invalid.cir")),
            ]
        );
    }

    #[test]
    fn startup_conflict_preserves_both_typed_origins_and_modes() {
        let attributes = startup_directive_conflict_attributes(&StartupDirectiveConflictError {
            first_kind: StartupDirectiveKind::Ic,
            first: NetlistSourceLocation::in_file("deck.cir", 8),
            conflicting_kind: StartupDirectiveKind::NodeSet,
            conflicting: NetlistSourceLocation::in_file("included.cir", 12),
        });

        assert_eq!(attributes.kind, "conflicting_startup_directives");
        assert_eq!(attributes.category, Some("startup_directive_validation"));
        assert_eq!(attributes.primary_line, Some(8));
        assert_eq!(attributes.primary_source.as_deref(), Some("deck.cir"));
        assert_eq!(attributes.related_line, Some(12));
        assert_eq!(attributes.related_source.as_deref(), Some("included.cir"));
        assert_eq!(attributes.first_startup_kind.as_deref(), Some("ic"));
        assert_eq!(
            attributes.conflicting_startup_kind.as_deref(),
            Some("nodeset")
        );
    }
}

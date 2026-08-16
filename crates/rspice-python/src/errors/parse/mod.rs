//! Mapping `ParseError` onto typed Python exceptions.
//!
//! A `ParseError` carries structured detail -- the offending symbol, the
//! subcircuit involved, the conflicting directive -- and that detail is
//! attached to the raised exception as attributes rather than being flattened
//! into the message. A caller writing a linter can then branch on the cause
//! instead of matching substrings of prose that is free to change.
//!
//! `ParseErrorAttributes` below is the single flat projection every variant
//! fills in, and the sibling modules own one diagnostic family each -- the
//! translation from a core error's own shape into that projection. Only the
//! dispatch and the Python hand-off stay here.

use super::*;

mod device;
mod directives;
mod output;
mod subcircuit;
mod symbols;

pub use symbols::PyUnresolvedOutputSymbol;

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
    parameter_name: Option<String>,
    canonical_parameter_name: Option<String>,
    expression: Option<String>,
    output_directive: Option<String>,
    operator_name: Option<String>,
    function_name: Option<String>,
    identifier_name: Option<String>,
    missing_dependency: Option<String>,
    reason: Option<String>,
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
    resource: Option<&'static str>,
    requested: Option<usize>,
    limit: Option<usize>,
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
        self.source = location.path.as_ref().map(|path| public_path_string(path));
        self.primary_line = self.line;
        self.primary_source = self.source.clone();
    }

    fn set_related(&mut self, location: &rspice_core::netlist::NetlistSourceLocation) {
        self.related_line = Some(location.line);
        self.related_source = location.path.as_ref().map(|path| public_path_string(path));
    }

    /// Attach every field to the raised exception. Absent detail is set to
    /// `None` rather than left off, so a caller can read any attribute on any
    /// `ParseError` without guarding each access with `hasattr`.
    fn attach(self, value: &Bound<'_, PyAny>) -> PyResult<()> {
        value.setattr("kind", self.kind)?;
        value.setattr("category", self.category)?;
        value.setattr("line", self.line)?;
        value.setattr("detail", self.detail)?;
        value.setattr("source", self.source)?;
        value.setattr("primary_line", self.primary_line)?;
        value.setattr("primary_source", self.primary_source)?;
        value.setattr("related_line", self.related_line)?;
        value.setattr("related_source", self.related_source)?;
        value.setattr("detected_line", self.detected_line)?;
        value.setattr("detected_source", self.detected_source)?;
        value.setattr("boundary", self.boundary)?;
        value.setattr("authored_name", self.authored_name)?;
        value.setattr("canonical_name", self.canonical_name)?;
        value.setattr("qualified_name", self.qualified_name)?;
        value.setattr("subcircuit_name", self.subcircuit_name)?;
        value.setattr("canonical_subcircuit_name", self.canonical_subcircuit_name)?;
        value.setattr("instance_name", self.instance_name)?;
        value.setattr("canonical_instance_name", self.canonical_instance_name)?;
        value.setattr("qualified_instance_name", self.qualified_instance_name)?;
        value.setattr("parameter_name", self.parameter_name)?;
        value.setattr("canonical_parameter_name", self.canonical_parameter_name)?;
        value.setattr("expression", self.expression)?;
        value.setattr("output_directive", self.output_directive)?;
        value.setattr("operator_name", self.operator_name)?;
        value.setattr("function_name", self.function_name)?;
        value.setattr("identifier_name", self.identifier_name)?;
        value.setattr("missing_dependency", self.missing_dependency)?;
        value.setattr("reason", self.reason)?;
        value.setattr("formal_port", self.formal_port)?;
        value.setattr("first_position", self.first_position)?;
        value.setattr("conflicting_position", self.conflicting_position)?;
        value.setattr("first_actual_node", self.first_actual_node)?;
        value.setattr("conflicting_actual_node", self.conflicting_actual_node)?;
        value.setattr("position", self.position)?;
        value.setattr("actual_node", self.actual_node)?;
        value.setattr("authored_coupling_name", self.authored_coupling_name)?;
        value.setattr("canonical_coupling_name", self.canonical_coupling_name)?;
        value.setattr("qualified_coupling_name", self.qualified_coupling_name)?;
        value.setattr("authored_inductor_name", self.authored_inductor_name)?;
        value.setattr("canonical_inductor_name", self.canonical_inductor_name)?;
        value.setattr("qualified_inductor_name", self.qualified_inductor_name)?;
        value.setattr("scope_name", self.scope_name)?;
        value.setattr("reference_position", self.reference_position)?;
        value.setattr("device", self.device)?;
        value.setattr("requested_path", self.requested_path)?;
        value.setattr("value_index", self.value_index)?;
        value.setattr("value", self.value)?;
        value.setattr("expected", self.expected)?;
        value.setattr("actual", self.actual)?;
        value.setattr("device_type", self.device_type)?;
        value.setattr("unresolved_output_symbols", self.unresolved_output_symbols)?;
        value.setattr("first_startup_kind", self.first_startup_kind)?;
        value.setattr("conflicting_startup_kind", self.conflicting_startup_kind)?;
        value.setattr("resource", self.resource)?;
        value.setattr("requested", self.requested)?;
        value.setattr("limit", self.limit)?;
        Ok(())
    }
}

/// Convert a parse error to PyErr
pub fn parse_error_to_pyerr(err: rspice_core::netlist::ParseError) -> PyErr {
    use rspice_core::netlist::{MissingSubcircuitEndsBoundary, ParseError as CoreParseError};

    let message = err.to_string();
    let mut attributes = match &err {
        CoreParseError::ResourceLimit(error) => {
            let mut attributes = ParseErrorAttributes::new("resource_limit");
            attributes.category = Some("resource_limit");
            attributes.resource = Some(error.resource.as_str());
            attributes.requested = Some(error.requested);
            attributes.limit = Some(error.limit);
            attributes
        }
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
        CoreParseError::ParameterRedefinition(error) => {
            directives::parameter_redefinition_attributes(error)
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
                .map(|path| public_path_string(path));
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
            subcircuit::duplicate_subcircuit_binding_attributes(error)
        }
        CoreParseError::GlobalSubcircuitPortBinding(error) => {
            subcircuit::global_subcircuit_binding_attributes(error)
        }
        CoreParseError::UndefinedSubcircuit(error) => {
            subcircuit::undefined_subcircuit_attributes(error)
        }
        CoreParseError::MissingDeviceModel(error) => device::missing_device_model_attributes(error),
        CoreParseError::UnresolvedSubcircuitParameter(error) => {
            subcircuit::unresolved_subcircuit_parameter_attributes(error)
        }
        CoreParseError::UndefinedMutualInductorReference(error) => {
            device::undefined_mutual_inductor_reference_attributes(error)
        }
        CoreParseError::OutputSymbolValidation(error) => {
            output::output_symbol_validation_attributes(error)
        }
        CoreParseError::OutputExpressionValidation(error) => {
            output::output_expression_validation_attributes(error)
        }
        CoreParseError::StartupDirectiveConflict(error) => {
            directives::startup_directive_conflict_attributes(error)
        }
        CoreParseError::DeviceInitialCondition(error) => {
            device::device_initial_condition_attributes(error.as_ref())
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
        attributes.attach(value.as_any())
    });
    error
}

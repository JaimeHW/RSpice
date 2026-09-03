//! The structured browser error and its JavaScript projection.
//!
//! `code`, `category`, and `retryable` come from the core error descriptor, so
//! a new core failure taxonomy flows through without a per-error edit here.

use rspice_core::{ResourceKind, ResourceLimitError};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::dto::{
    WasmDiagnostic, WasmSourceLocation, WasmStartupDiagnostic, WasmStartupDirectiveScope,
};

/// Stable structured error exposed by the browser bindings.
///
/// The legacy human-readable message remains available verbatim. Consumers
/// that need reliable diagnostics should branch on `kind` and `category`
/// instead of parsing that message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmError {
    pub message: String,
    /// Cross-interface stable error code. `kind` remains as a compatibility
    /// alias for existing browser consumers.
    pub code: String,
    pub kind: String,
    pub category: String,
    pub retryable: bool,
    pub primary_source: Option<String>,
    pub primary_line: Option<usize>,
    #[serde(default)]
    pub related_source: Option<String>,
    #[serde(default)]
    pub related_line: Option<usize>,
    #[serde(default)]
    pub first_startup_kind: Option<String>,
    #[serde(default)]
    pub conflicting_startup_kind: Option<String>,
    #[serde(default)]
    pub iterations: Option<usize>,
    #[serde(default)]
    pub resource: Option<String>,
    #[serde(default)]
    pub requested: Option<usize>,
    #[serde(default)]
    pub limit: Option<usize>,
    #[serde(default)]
    pub subcircuit_name: Option<String>,
    #[serde(default)]
    pub canonical_subcircuit_name: Option<String>,
    #[serde(default)]
    pub instance_name: Option<String>,
    #[serde(default)]
    pub canonical_instance_name: Option<String>,
    #[serde(default)]
    pub qualified_instance_name: Option<String>,
    #[serde(default)]
    pub parameter_name: Option<String>,
    #[serde(default)]
    pub canonical_parameter_name: Option<String>,
    #[serde(default)]
    pub expression: Option<String>,
    #[serde(default)]
    pub output_directive: Option<String>,
    #[serde(default)]
    pub operator_name: Option<String>,
    #[serde(default)]
    pub function_name: Option<String>,
    #[serde(default)]
    pub identifier_name: Option<String>,
    #[serde(default)]
    pub missing_dependency: Option<String>,
    #[serde(default)]
    pub reason: Option<String>,
    pub unresolved_output_symbols: Vec<WasmUnresolvedOutputSymbol>,
}

/// One unresolved output symbol, preserved in the core validator's exact
/// diagnostic order.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WasmUnresolvedOutputSymbol {
    pub directive: String,
    pub source: Option<String>,
    pub line: usize,
    pub operator: String,
    pub symbol: String,
    pub symbol_kind: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct JsWasmErrorDetails<'a> {
    message: &'a str,
    code: &'a str,
    kind: &'a str,
    category: &'a str,
    retryable: bool,
    primary_source: Option<&'a str>,
    primary_line: Option<usize>,
    related_source: Option<&'a str>,
    related_line: Option<usize>,
    first_startup_kind: Option<&'a str>,
    conflicting_startup_kind: Option<&'a str>,
    iterations: Option<usize>,
    resource: Option<&'a str>,
    requested: Option<usize>,
    limit: Option<usize>,
    subcircuit_name: Option<&'a str>,
    canonical_subcircuit_name: Option<&'a str>,
    instance_name: Option<&'a str>,
    canonical_instance_name: Option<&'a str>,
    qualified_instance_name: Option<&'a str>,
    parameter_name: Option<&'a str>,
    canonical_parameter_name: Option<&'a str>,
    expression: Option<&'a str>,
    output_directive: Option<&'a str>,
    operator_name: Option<&'a str>,
    function_name: Option<&'a str>,
    identifier_name: Option<&'a str>,
    missing_dependency: Option<&'a str>,
    reason: Option<&'a str>,
    unresolved_output_symbols: Vec<JsUnresolvedOutputSymbol<'a>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct JsUnresolvedOutputSymbol<'a> {
    directive: &'a str,
    source: Option<&'a str>,
    line: usize,
    operator: &'a str,
    symbol: &'a str,
    symbol_kind: &'a str,
}

impl WasmError {
    pub(crate) fn new(message: String, kind: &str, category: &str) -> Self {
        Self {
            message,
            code: kind.to_string(),
            kind: kind.to_string(),
            category: category.to_string(),
            retryable: false,
            primary_source: None,
            primary_line: None,
            related_source: None,
            related_line: None,
            first_startup_kind: None,
            conflicting_startup_kind: None,
            iterations: None,
            resource: None,
            requested: None,
            limit: None,
            subcircuit_name: None,
            canonical_subcircuit_name: None,
            instance_name: None,
            canonical_instance_name: None,
            qualified_instance_name: None,
            parameter_name: None,
            canonical_parameter_name: None,
            expression: None,
            output_directive: None,
            operator_name: None,
            function_name: None,
            identifier_name: None,
            missing_dependency: None,
            reason: None,
            unresolved_output_symbols: Vec::new(),
        }
    }

    pub(crate) fn invalid_argument(message: String) -> Self {
        Self::new(message, "invalid_argument", "input_validation")
    }

    pub(crate) fn unsupported_cancellation(mechanism: String) -> Self {
        let mut error = Self::new(
            format!("unsupported cancellation mechanism '{mechanism}'; expected 'sharedInt32'"),
            "unsupported_cancellation",
            "cancellation",
        );
        error.reason = Some(mechanism);
        error
    }

    pub(crate) fn resource_limit(message: String, error: ResourceLimitError) -> Self {
        let mut structured = Self::new(message, "resource_limit", "resource_limit");
        structured.resource = Some(error.resource.as_str().to_string());
        structured.requested = Some(error.requested);
        structured.limit = Some(error.limit);
        structured
    }

    pub(crate) fn from_simulation_error(error: rspice_core::engine::SimulationError) -> Self {
        let descriptor = error.descriptor();
        let message = error.to_string();
        let mut structured = if let Some(resource) = descriptor.resource_limit {
            Self::resource_limit(message, resource)
        } else {
            Self::new(
                message,
                descriptor.code.as_str(),
                descriptor.category.as_str(),
            )
        };
        structured.code = descriptor.code.as_str().to_string();
        structured.kind = structured.code.clone();
        structured.category = descriptor.category.as_str().to_string();
        structured.retryable = descriptor.retryable;
        structured.iterations = descriptor.iterations;
        if let rspice_core::engine::SimulationError::BehavioralReference(error) = &error {
            structured.instance_name = Some(error.owner_name.clone());
            structured.canonical_instance_name = Some(error.canonical_owner_name.clone());
            structured.missing_dependency = Some(error.canonical_dependency_name.clone());
            structured.reason = Some(error.reason.as_str().to_string());
        }
        structured
    }

    pub(crate) fn from_parse_error(error: rspice_core::netlist::ParseError) -> Self {
        let message = error.to_string();
        match error {
            rspice_core::netlist::ParseError::ResourceLimit(error) => {
                Self::resource_limit(message, error)
            }
            rspice_core::netlist::ParseError::OutputSymbolValidation(error) => {
                let unresolved_output_symbols = error
                    .unresolved
                    .iter()
                    .map(|item| WasmUnresolvedOutputSymbol {
                        directive: output_directive_name(item.directive).to_string(),
                        source: source_path(&item.origin),
                        line: item.origin.line,
                        operator: item.operator.clone(),
                        symbol: item.symbol.clone(),
                        symbol_kind: output_symbol_kind_name(item.kind).to_string(),
                    })
                    .collect::<Vec<_>>();
                let primary = error.unresolved.first().map(|item| &item.origin);

                Self {
                    message,
                    code: "undefined_output_symbols".to_string(),
                    kind: "undefined_output_symbols".to_string(),
                    category: "output_symbol_validation".to_string(),
                    retryable: false,
                    primary_source: primary.and_then(source_path),
                    primary_line: primary.map(|origin| origin.line),
                    related_source: None,
                    related_line: None,
                    first_startup_kind: None,
                    conflicting_startup_kind: None,
                    iterations: None,
                    resource: None,
                    requested: None,
                    limit: None,
                    subcircuit_name: None,
                    canonical_subcircuit_name: None,
                    instance_name: None,
                    canonical_instance_name: None,
                    qualified_instance_name: None,
                    parameter_name: None,
                    canonical_parameter_name: None,
                    expression: None,
                    output_directive: None,
                    operator_name: None,
                    function_name: None,
                    identifier_name: None,
                    missing_dependency: None,
                    reason: None,
                    unresolved_output_symbols,
                }
            }
            rspice_core::netlist::ParseError::OutputExpressionValidation(error) => {
                use rspice_core::netlist::OutputExpressionIssue;

                let (kind, operator_name, function_name, identifier_name) = match &error.issue {
                    OutputExpressionIssue::UnknownFunction { function } => (
                        "unknown_output_function",
                        None,
                        Some(function.clone()),
                        None,
                    ),
                    OutputExpressionIssue::UnresolvedIdentifier { identifier } => (
                        "unresolved_output_identifier",
                        None,
                        None,
                        Some(identifier.clone()),
                    ),
                    OutputExpressionIssue::InvalidAccessor { operator, .. } => (
                        "invalid_output_accessor",
                        Some(operator.clone()),
                        None,
                        None,
                    ),
                    OutputExpressionIssue::UnresolvedDeviceParameter { .. } => {
                        ("unresolved_output_device_parameter", None, None, None)
                    }
                    OutputExpressionIssue::Syntax { .. } => {
                        ("invalid_output_expression_syntax", None, None, None)
                    }
                };
                let mut structured = Self::new(message, kind, "output_expression_validation");
                structured.primary_source = source_path(&error.origin);
                structured.primary_line = Some(error.origin.line);
                structured.expression = Some(error.expression);
                structured.output_directive = Some(error.directive.to_string());
                structured.operator_name = operator_name;
                structured.function_name = function_name;
                structured.identifier_name = identifier_name;
                if let OutputExpressionIssue::UnresolvedDeviceParameter { device, parameter } =
                    &error.issue
                {
                    structured.instance_name = Some(device.clone());
                    structured.parameter_name = Some(parameter.clone());
                }
                structured.reason = Some(error.issue.reason());
                structured
            }
            rspice_core::netlist::ParseError::StartupDirectiveConflict(error) => Self {
                message,
                code: "conflicting_startup_directives".to_string(),
                kind: "conflicting_startup_directives".to_string(),
                category: "startup_directive_validation".to_string(),
                retryable: false,
                primary_source: source_path(&error.first),
                primary_line: Some(error.first.line),
                related_source: source_path(&error.conflicting),
                related_line: Some(error.conflicting.line),
                first_startup_kind: Some(startup_directive_kind_name(error.first_kind).to_string()),
                conflicting_startup_kind: Some(
                    startup_directive_kind_name(error.conflicting_kind).to_string(),
                ),
                iterations: None,
                resource: None,
                requested: None,
                limit: None,
                subcircuit_name: None,
                canonical_subcircuit_name: None,
                instance_name: None,
                canonical_instance_name: None,
                qualified_instance_name: None,
                parameter_name: None,
                canonical_parameter_name: None,
                expression: None,
                output_directive: None,
                operator_name: None,
                function_name: None,
                identifier_name: None,
                missing_dependency: None,
                reason: None,
                unresolved_output_symbols: Vec::new(),
            },
            rspice_core::netlist::ParseError::UnresolvedSubcircuitParameter(error) => {
                let mut structured = Self::new(
                    message,
                    "unresolved_subcircuit_parameter",
                    "subcircuit_parameter_resolution",
                );
                structured.subcircuit_name = Some(error.subcircuit_name);
                structured.canonical_subcircuit_name = Some(error.canonical_subcircuit_name);
                structured.instance_name = Some(error.instance_name);
                structured.canonical_instance_name = Some(error.canonical_instance_name);
                structured.qualified_instance_name = Some(error.qualified_instance_name);
                structured.parameter_name = Some(error.parameter_name);
                structured.canonical_parameter_name = Some(error.canonical_parameter_name);
                structured.expression = Some(error.expression);
                structured.missing_dependency = error.missing_dependency;
                structured.reason = Some(error.reason);
                structured
            }
            rspice_core::netlist::ParseError::UndefinedSubcircuit(error) => {
                let mut structured =
                    Self::new(message, "undefined_subcircuit", "subcircuit_resolution");
                structured.subcircuit_name = Some(error.subcircuit_name);
                structured.canonical_subcircuit_name = Some(error.canonical_subcircuit_name);
                structured.instance_name = Some(error.instance_name);
                structured.canonical_instance_name = Some(error.canonical_instance_name);
                structured.qualified_instance_name = Some(error.qualified_instance_name);
                structured
            }
            rspice_core::netlist::ParseError::MissingDeviceModel(error) => {
                let mut structured =
                    Self::new(message, "missing_device_model", "device_model_resolution");
                structured.primary_line = Some(error.line);
                structured.instance_name = Some(error.device_name);
                structured.canonical_instance_name = Some(error.canonical_device_name);
                structured.reason = Some(error.device_type);
                structured
            }
            _ => Self::new(message, "parse_error", "netlist_parse"),
        }
    }
}

pub(crate) fn source_path(
    location: &rspice_core::netlist::NetlistSourceLocation,
) -> Option<String> {
    location
        .path
        .as_ref()
        .map(|path| path.to_string_lossy().into_owned())
}

pub(crate) fn startup_directive_kind_name(
    kind: rspice_core::netlist::StartupDirectiveKind,
) -> &'static str {
    match kind {
        rspice_core::netlist::StartupDirectiveKind::Ic => "ic",
        rspice_core::netlist::StartupDirectiveKind::NodeSet => "nodeset",
    }
}

pub(crate) fn output_directive_name(
    kind: rspice_core::netlist::OutputDirectiveKind,
) -> &'static str {
    use rspice_core::netlist::OutputDirectiveKind;
    match kind {
        OutputDirectiveKind::Save => "save",
        OutputDirectiveKind::Probe => "probe",
        OutputDirectiveKind::Print => "print",
        OutputDirectiveKind::Plot => "plot",
        OutputDirectiveKind::Measure => "measure",
        OutputDirectiveKind::Four => "four",
        OutputDirectiveKind::Fft => "fft",
    }
}

pub(crate) fn output_symbol_kind_name(
    kind: rspice_core::netlist::OutputSymbolKind,
) -> &'static str {
    match kind {
        rspice_core::netlist::OutputSymbolKind::Node => "node",
        rspice_core::netlist::OutputSymbolKind::Device => "device",
    }
}

pub(crate) fn resource_limit_error(
    resource: ResourceKind,
    requested: usize,
    limit: usize,
) -> Box<WasmError> {
    let error = ResourceLimitError {
        resource,
        requested,
        limit,
    };
    Box::new(WasmError::resource_limit(error.to_string(), error))
}

pub(crate) fn wasm_error_to_js(error: WasmError) -> JsValue {
    let js_error = js_sys::Error::new(&error.message);
    js_error.set_name("RSpiceError");
    let object: &JsValue = js_error.as_ref();

    let details = JsWasmErrorDetails {
        message: &error.message,
        code: &error.code,
        kind: &error.kind,
        category: &error.category,
        retryable: error.retryable,
        primary_source: error.primary_source.as_deref(),
        primary_line: error.primary_line,
        related_source: error.related_source.as_deref(),
        related_line: error.related_line,
        first_startup_kind: error.first_startup_kind.as_deref(),
        conflicting_startup_kind: error.conflicting_startup_kind.as_deref(),
        iterations: error.iterations,
        resource: error.resource.as_deref(),
        requested: error.requested,
        limit: error.limit,
        subcircuit_name: error.subcircuit_name.as_deref(),
        canonical_subcircuit_name: error.canonical_subcircuit_name.as_deref(),
        instance_name: error.instance_name.as_deref(),
        canonical_instance_name: error.canonical_instance_name.as_deref(),
        qualified_instance_name: error.qualified_instance_name.as_deref(),
        parameter_name: error.parameter_name.as_deref(),
        canonical_parameter_name: error.canonical_parameter_name.as_deref(),
        expression: error.expression.as_deref(),
        output_directive: error.output_directive.as_deref(),
        operator_name: error.operator_name.as_deref(),
        function_name: error.function_name.as_deref(),
        identifier_name: error.identifier_name.as_deref(),
        missing_dependency: error.missing_dependency.as_deref(),
        reason: error.reason.as_deref(),
        unresolved_output_symbols: error
            .unresolved_output_symbols
            .iter()
            .map(|item| JsUnresolvedOutputSymbol {
                directive: &item.directive,
                source: item.source.as_deref(),
                line: item.line,
                operator: &item.operator,
                symbol: &item.symbol,
                symbol_kind: &item.symbol_kind,
            })
            .collect(),
    };
    if let Ok(details) = serde_wasm_bindgen::to_value(&details) {
        for field in [
            "code",
            "kind",
            "category",
            "retryable",
            "primarySource",
            "primaryLine",
            "relatedSource",
            "relatedLine",
            "firstStartupKind",
            "conflictingStartupKind",
            "iterations",
            "resource",
            "requested",
            "limit",
            "subcircuitName",
            "canonicalSubcircuitName",
            "instanceName",
            "canonicalInstanceName",
            "qualifiedInstanceName",
            "parameterName",
            "canonicalParameterName",
            "expression",
            "outputDirective",
            "operatorName",
            "functionName",
            "identifierName",
            "missingDependency",
            "reason",
            "unresolvedOutputSymbols",
        ] {
            let key = JsValue::from_str(field);
            if let Ok(value) = js_sys::Reflect::get(&details, &key) {
                let _ = js_sys::Reflect::set(object, &key, &value);
            }
        }
        let _ = js_sys::Reflect::set(object, &JsValue::from_str("details"), &details);
    }

    js_error.into()
}

pub(crate) fn diagnostic_summary(
    diagnostic: &rspice_core::netlist::ParseDiagnostic,
) -> WasmDiagnostic {
    WasmDiagnostic {
        line: diagnostic.line,
        severity: match diagnostic.severity {
            rspice_core::netlist::DiagnosticSeverity::Warning => "warning".to_string(),
        },
        code: diagnostic.code.clone(),
        message: diagnostic.message.clone(),
    }
}

pub(crate) fn startup_diagnostic_summary(
    diagnostic: &rspice_core::netlist::StartupDiagnostic,
) -> WasmStartupDiagnostic {
    use rspice_core::netlist::{StartupDiagnosticStage, StartupDirectiveScope};

    WasmStartupDiagnostic {
        code: diagnostic.code.as_str().to_string(),
        stage: match diagnostic.stage {
            StartupDiagnosticStage::Parse => "parse",
            StartupDiagnosticStage::StartupTopology => "startup_topology",
        }
        .to_string(),
        directive: startup_directive_kind_name(diagnostic.kind).to_string(),
        origins: diagnostic
            .origins
            .iter()
            .map(|origin| WasmSourceLocation {
                source: source_path(origin),
                line: origin.line,
            })
            .collect(),
        scopes: diagnostic
            .scopes
            .iter()
            .map(|scope| match scope {
                StartupDirectiveScope::TopLevel => WasmStartupDirectiveScope {
                    kind: "top_level".to_string(),
                    qualified_definition: None,
                    qualified_instances: Vec::new(),
                },
                StartupDirectiveScope::Subcircuit {
                    qualified_definition,
                    qualified_instances,
                } => WasmStartupDirectiveScope {
                    kind: "subcircuit".to_string(),
                    qualified_definition: Some(qualified_definition.clone()),
                    qualified_instances: qualified_instances.clone(),
                },
            })
            .collect(),
        canonical_nodes: diagnostic.canonical_nodes.clone(),
    }
}

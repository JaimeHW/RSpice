//! Typed output-request provenance and pre-construction symbol validation.
//!
//! Output directives are intentionally retained in their existing execution
//! forms (`SaveSet`, `MeasureStatement`, and `AnalysisCommand::Four`).  This
//! module is the semantic sidecar: it records where each request came from,
//! extracts every circuit-symbol dependency in authored order, and validates
//! those dependencies against the flattened circuit namespace before any
//! topology reduction or device stamping occurs.

use super::{
    AnalysisCommand, Element, ElementKind, Flattener, FlattenerConfig, MeasureStatement, Netlist,
    NetlistSourceLocation, ParseError, ParseWithAbortError, ensure_parse_not_aborted,
    poll_parse_abort, poll_parse_text,
};
use crate::abort_signal::AbortSignal;
use std::collections::{HashMap, HashSet};

/// The directive family that owns an output request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputDirectiveKind {
    Save,
    Probe,
    Print,
    Plot,
    Measure,
    Four,
}

impl OutputDirectiveKind {
    fn is_direct_output(self) -> bool {
        matches!(self, Self::Save | Self::Probe | Self::Print | Self::Plot)
    }
}

impl std::fmt::Display for OutputDirectiveKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Save => ".SAVE",
            Self::Probe => ".PROBE",
            Self::Print => ".PRINT",
            Self::Plot => ".PLOT",
            Self::Measure => ".MEASURE",
            Self::Four => ".FOUR",
        })
    }
}

/// Analysis domain selected by a direct output request.
///
/// `.PRINT` and `.PLOT` carry this qualifier explicitly. Measurement
/// requests retain it from their parsed statement, while analysis-agnostic
/// `.SAVE` and `.PROBE` requests leave it unset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputAnalysisKind {
    /// Transient analysis.
    Tran,
    /// Small-signal AC analysis.
    Ac,
    /// DC sweep analysis.
    Dc,
    /// Noise spectral-density analysis.
    Noise,
    /// Small-signal distortion analysis.
    Disto,
    /// DC operating-point analysis.
    Op,
    /// Small-signal transfer-function analysis.
    Tf,
    /// Scattering-parameter analysis.
    Sp,
    /// Periodic steady-state analysis.
    Pss,
}

impl OutputAnalysisKind {
    pub(crate) fn from_keyword(keyword: &str) -> Option<Self> {
        match keyword.trim().to_ascii_uppercase().as_str() {
            "TR" | "TRAN" | "TRAN_CONT" => Some(Self::Tran),
            "AC" | "AC_CONT" => Some(Self::Ac),
            "DC" | "DC_CONT" => Some(Self::Dc),
            "NOISE" | "NOISE_CONT" => Some(Self::Noise),
            "DISTO" => Some(Self::Disto),
            "OP" => Some(Self::Op),
            "TF" => Some(Self::Tf),
            "SP" => Some(Self::Sp),
            "PSS" => Some(Self::Pss),
            _ => None,
        }
    }
}

/// Namespace searched by one output dependency.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OutputSymbolKind {
    Node,
    Device,
}

/// Field delimiter selected by a source-authored Xyce `.PRINT` card.
///
/// Xyce treats an absent or invalid `DELIMITER=` value as its standard
/// whitespace table layout.  Quoted custom delimiters are retained without
/// their quotes so every frontend can render the same typed request.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum PrintDelimiter {
    #[default]
    Whitespace,
    Tab,
    Comma,
    Colon,
    Semicolon,
    Custom(String),
}

impl PrintDelimiter {
    /// The exact separator written between unpadded table fields.
    pub fn separator(&self) -> &str {
        match self {
            Self::Whitespace => " ",
            Self::Tab => "\t",
            Self::Comma => ",",
            Self::Colon => ":",
            Self::Semicolon => ";",
            Self::Custom(value) => value,
        }
    }
}

impl std::fmt::Display for OutputSymbolKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Node => "node",
            Self::Device => "device",
        })
    }
}

/// One circuit-symbol occurrence referenced by an output request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputSymbolDependency {
    /// Authored accessor (`V`, `VM`, `I`, `@`, ...), retained for adapters.
    pub operator: String,
    /// Authored symbol spelling, including hierarchy separators or wildcards.
    pub symbol: String,
    pub kind: OutputSymbolKind,
    /// Whether this accessor occurred inside a braced/quoted expression.
    /// Direct-output expression occurrences retain duplicates exactly.
    pub expression: bool,
}

/// One source-authored operand on an ordered output card.
///
/// `authored` is the exact slice of the parser's logical card. The typed kind
/// is the execution contract, so exporters never need to reconstruct display
/// names or guess whether a token was a direct getter or an expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OutputOperand {
    pub(crate) authored: String,
    pub(crate) kind: OutputOperandKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OutputOperandKind {
    Probe(super::SaveSignal),
    Expression { body: String },
}

/// Provenance sidecar for one source-level output request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputRequest {
    pub directive: OutputDirectiveKind,
    pub origin: NetlistSourceLocation,
    /// Explicit analysis qualifier, when the directive carries one.
    pub analysis: Option<OutputAnalysisKind>,
    /// Optional semantic name used to replace redefined measurements.
    pub name: Option<String>,
    /// Effective Xyce `.PRINT` delimiter. This is `Some(Whitespace)` for a
    /// `.PRINT` card with no delimiter or an invalid delimiter, and `None`
    /// for directives that do not own Xyce print formatting.
    pub print_delimiter: Option<PrintDelimiter>,
    /// Complete source-authored output operands in card order.
    ///
    /// Unlike `dependencies`, this retains one entry per rendered column, so
    /// expressions remain interleaved with direct probes and duplicates stay
    /// meaningful at the export boundary.
    pub operands: Vec<String>,
    /// Typed semantics aligned one-to-one with `operands`.
    pub(crate) operand_kinds: Vec<OutputOperandKind>,
    /// Authored braced or quoted expression bodies in source order.
    ///
    /// Keeping these separately from circuit dependencies lets semantic
    /// validation reject unknown functions and unresolved scalar identifiers
    /// before an analysis starts, while preserving duplicate expressions and
    /// their request-level source provenance.
    pub expressions: Vec<String>,
    /// Dependencies in their original occurrence order. Repetitions are
    /// meaningful and are therefore not collapsed here.
    pub dependencies: Vec<OutputSymbolDependency>,
}

impl OutputRequest {
    /// Whether this request needs any transient device-current operand.
    pub(crate) fn requires_transient_device_current_operand(&self) -> bool {
        self.analysis
            .is_none_or(|analysis| analysis == OutputAnalysisKind::Tran)
            && self.dependencies.iter().any(|dependency| {
                dependency.kind == OutputSymbolKind::Device
                    && is_transient_device_current_operator(&dependency.operator)
            })
    }

    /// Whether this request needs a derived transient current for `device`.
    ///
    /// Direct `I(device)` probes already appear in [`SaveSet`](super::SaveSet),
    /// but current and power accessors nested inside output expressions are
    /// represented only by this typed dependency sidecar. Result retention
    /// must honor both representations before integration starts.
    pub(crate) fn selects_transient_device_current(&self, device: &str) -> bool {
        if !self.requires_transient_device_current_operand() {
            return false;
        }
        let device = canonical_symbol(device);
        self.dependencies.iter().any(|dependency| {
            dependency.kind == OutputSymbolKind::Device
                && is_transient_device_current_operator(&dependency.operator)
                && hierarchy_pattern_matches(&canonical_symbol(&dependency.symbol), &device)
        })
    }

    /// Whether this request retains a transient node-voltage operand for the
    /// named node.  Node dependencies are recorded for direct voltage probes
    /// and for voltage operands nested inside expressions; device-current
    /// dependencies are intentionally excluded.
    pub(crate) fn selects_transient_node_voltage(&self, node: &str) -> bool {
        if self
            .analysis
            .is_some_and(|analysis| analysis != OutputAnalysisKind::Tran)
        {
            return false;
        }
        let node = canonical_symbol(node);
        self.dependencies.iter().any(|dependency| {
            dependency.kind == OutputSymbolKind::Node
                && hierarchy_pattern_matches(&canonical_symbol(&dependency.symbol), &node)
        })
    }

    pub(crate) fn from_source(
        directive: OutputDirectiveKind,
        origin: NetlistSourceLocation,
        source: &str,
        expressions: Vec<String>,
    ) -> Self {
        let analysis = if matches!(
            directive,
            OutputDirectiveKind::Print | OutputDirectiveKind::Plot
        ) {
            source
                .split_whitespace()
                .next()
                .and_then(OutputAnalysisKind::from_keyword)
        } else {
            None
        };
        Self {
            directive,
            origin,
            analysis,
            name: None,
            print_delimiter: matches!(directive, OutputDirectiveKind::Print)
                .then_some(PrintDelimiter::Whitespace),
            operands: Vec::new(),
            operand_kinds: Vec::new(),
            expressions,
            dependencies: extract_output_dependencies(source),
        }
    }

    /// Build the semantic request corresponding to one frontend output
    /// override, such as a command-line `--save` value.
    ///
    /// The same accessor extractor used for source-authored output cards owns
    /// dependency recognition here. Bare vector names follow
    /// [`parse_save_probe`](super::parse_save_probe) semantics and are treated
    /// as node-voltage shorthand. Device-parameter probes remain outside
    /// symbol-existence validation because their validity belongs to device
    /// metadata validation.
    pub fn from_save_override(origin: NetlistSourceLocation, source: &str) -> Self {
        let mut dependencies = extract_output_dependencies(source);
        if dependencies.is_empty()
            && !source.trim_start().starts_with('@')
            && let Some(super::SaveSignal::Raw(node)) = super::parse_save_probe(source)
            && !node.eq_ignore_ascii_case("all")
        {
            dependencies = extract_output_dependencies(&format!("V({})", source.trim()));
        }
        Self {
            directive: OutputDirectiveKind::Save,
            origin,
            analysis: None,
            name: None,
            print_delimiter: None,
            operands: Vec::new(),
            operand_kinds: Vec::new(),
            expressions: extract_output_expressions(source),
            dependencies,
        }
    }

    pub(crate) fn from_measure(
        statement: &MeasureStatement,
        origin: NetlistSourceLocation,
        authored_source: &str,
    ) -> Self {
        let mut sources = Vec::new();
        collect_measure_sources(statement, &mut sources);
        let mut dependencies = Vec::new();
        for source in sources {
            let (source, expression_context) = match source {
                MeasureDependencySource::Direct(source) => (source, false),
                MeasureDependencySource::Expression(source) => (source, true),
            };
            dependencies.extend(extract_output_dependencies_with_context(
                source,
                expression_context,
            ));
        }
        let dependencies = retain_authored_dependency_spelling(
            dependencies,
            extract_output_dependencies(authored_source),
        );
        Self {
            directive: OutputDirectiveKind::Measure,
            origin,
            analysis: OutputAnalysisKind::from_keyword(&statement.analysis),
            name: Some(statement.name.clone()),
            print_delimiter: None,
            operands: Vec::new(),
            operand_kinds: Vec::new(),
            expressions: Vec::new(),
            dependencies,
        }
    }

    pub(crate) fn from_four(
        outputs: &[String],
        origin: NetlistSourceLocation,
        authored_source: &str,
    ) -> Self {
        let dependencies = outputs
            .iter()
            .flat_map(|output| extract_output_dependencies(output))
            .collect();
        let dependencies = retain_authored_dependency_spelling(
            dependencies,
            extract_output_dependencies(authored_source),
        );
        Self {
            directive: OutputDirectiveKind::Four,
            origin,
            analysis: None,
            name: None,
            print_delimiter: None,
            operands: Vec::new(),
            operand_kinds: Vec::new(),
            expressions: outputs
                .iter()
                .flat_map(|output| extract_output_expressions(output))
                .collect(),
            dependencies,
        }
    }

    pub(crate) fn with_print_delimiter(mut self, delimiter: PrintDelimiter) -> Self {
        if self.directive == OutputDirectiveKind::Print {
            self.print_delimiter = Some(delimiter);
        }
        self
    }

    pub(crate) fn from_ordered_operands(
        directive: OutputDirectiveKind,
        origin: NetlistSourceLocation,
        analysis: Option<OutputAnalysisKind>,
        operands: Vec<OutputOperand>,
    ) -> Self {
        let source = operands
            .iter()
            .map(|operand| operand.authored.as_str())
            .collect::<Vec<_>>()
            .join(" ");
        let expressions = operands
            .iter()
            .filter_map(|operand| match &operand.kind {
                OutputOperandKind::Expression { body } => Some(body.clone()),
                OutputOperandKind::Probe(_) => None,
            })
            .collect();
        let (operands, operand_kinds) = operands
            .into_iter()
            .map(|operand| (operand.authored, operand.kind))
            .unzip();
        Self {
            directive,
            origin,
            analysis,
            name: None,
            print_delimiter: matches!(directive, OutputDirectiveKind::Print)
                .then_some(PrintDelimiter::Whitespace),
            operands,
            operand_kinds,
            expressions,
            dependencies: extract_output_dependencies(&source),
        }
    }
}

/// Canonical current-projection output operators supported by Xyce.
pub(crate) fn is_current_projection_accessor(operator: &str) -> bool {
    matches!(operator, "I" | "IR" | "II" | "IM" | "IP" | "IDB")
}

/// Xyce's device-lead syntax is `I?`, where `?` is an arbitrary one-byte
/// terminal designator (`ID`, `IT`, `I1`, and so on). The caller must already
/// have canonicalized the operator.
pub(crate) fn is_device_lead_current_accessor(operator: &str) -> bool {
    operator.len() == 2 && operator.starts_with('I')
}

/// Direct/raw Xyce current-output grammar. Expression callers must reserve
/// builtin function names such as `IF` using their expression context.
pub(crate) fn is_current_output_accessor(operator: &str) -> bool {
    is_current_projection_accessor(operator) || is_device_lead_current_accessor(operator)
}

fn is_transient_device_current_operator(operator: &str) -> bool {
    matches!(operator, "P" | "W") || is_current_output_accessor(operator)
}

fn retain_authored_dependency_spelling(
    semantic: Vec<OutputSymbolDependency>,
    authored: Vec<OutputSymbolDependency>,
) -> Vec<OutputSymbolDependency> {
    let mut authored_index = 0;
    semantic
        .into_iter()
        .map(|dependency| {
            let matched = authored[authored_index..]
                .iter()
                .position(|candidate| {
                    candidate.kind == dependency.kind
                        && candidate
                            .operator
                            .eq_ignore_ascii_case(&dependency.operator)
                        && canonical_symbol(&candidate.symbol)
                            == canonical_symbol(&dependency.symbol)
                })
                .map(|offset| authored_index + offset);
            let Some(index) = matched else {
                return dependency;
            };
            authored_index = index + 1;
            let authored = &authored[index];
            OutputSymbolDependency {
                operator: authored.operator.clone(),
                symbol: authored.symbol.clone(),
                kind: dependency.kind,
                expression: dependency.expression,
            }
        })
        .collect()
}

/// One unresolved occurrence in a validated output request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnresolvedOutputSymbol {
    pub directive: OutputDirectiveKind,
    pub origin: NetlistSourceLocation,
    pub operator: String,
    pub symbol: String,
    pub kind: OutputSymbolKind,
}

/// Typed aggregate returned when output requests reference absent symbols.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputSymbolValidationError {
    pub unresolved: Vec<UnresolvedOutputSymbol>,
}

impl std::fmt::Display for OutputSymbolValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("undefined symbol(s) in output request(s): ")?;
        for (index, item) in self.unresolved.iter().enumerate() {
            if index != 0 {
                formatter.write_str(", ")?;
            }
            write!(
                formatter,
                "{} {} '{}' via {} at {}",
                item.directive, item.kind, item.symbol, item.operator, item.origin
            )?;
        }
        Ok(())
    }
}

impl std::error::Error for OutputSymbolValidationError {}

/// Semantic reason that an authored output expression cannot be evaluated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputExpressionIssue {
    /// An ordinary scalar name remained after parameter and function
    /// expansion. Runtime quantities and circuit probes are excluded.
    UnresolvedIdentifier { identifier: String },
    /// A call did not resolve to a built-in or authored `.FUNC` definition.
    UnknownFunction { function: String },
    /// A recognized output accessor has an invalid operand contract.
    InvalidAccessor { operator: String, detail: String },
    /// A device-parameter operand names an absent device or unsupported
    /// parameter in a device family whose metadata is known at elaboration.
    UnresolvedDeviceParameter { device: String, parameter: String },
    /// The expression could not be parsed or violated a built-in contract.
    Syntax { detail: String },
}

impl OutputExpressionIssue {
    pub fn unresolved_identifier(&self) -> Option<&str> {
        match self {
            Self::UnresolvedIdentifier { identifier } => Some(identifier),
            Self::UnknownFunction { .. }
            | Self::InvalidAccessor { .. }
            | Self::UnresolvedDeviceParameter { .. }
            | Self::Syntax { .. } => None,
        }
    }

    pub fn reason(&self) -> String {
        match self {
            Self::UnresolvedIdentifier { identifier } => {
                format!("unresolved identifier '{identifier}'")
            }
            Self::UnknownFunction { function } => format!("unknown function '{function}'"),
            Self::InvalidAccessor { operator, detail } => {
                format!("invalid {operator} output accessor: {detail}")
            }
            Self::UnresolvedDeviceParameter { device, parameter } => {
                format!("unresolved device parameter '{device}:{parameter}'")
            }
            Self::Syntax { detail } => detail.clone(),
        }
    }
}

impl std::fmt::Display for OutputExpressionIssue {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.reason())
    }
}

/// Typed failure for the first invalid authored output expression.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputExpressionValidationError {
    pub directive: OutputDirectiveKind,
    pub origin: NetlistSourceLocation,
    pub expression: String,
    pub issue: OutputExpressionIssue,
}

impl std::fmt::Display for OutputExpressionValidationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "invalid {} expression '{{{}}}' at {}: {}",
            self.directive, self.expression, self.origin, self.issue
        )
    }
}

impl std::error::Error for OutputExpressionValidationError {}

/// Validate authored output expressions after the complete parameter/function
/// scope has been parsed, but before topology construction or analysis.
pub fn validate_output_expressions_with_abort(
    netlist: &Netlist,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    ensure_parse_not_aborted(abort)?;
    if !netlist
        .output_requests
        .iter()
        .any(|request| !request.expressions.is_empty())
    {
        return Ok(());
    }
    let mut flattener = Flattener::with_models_config(
        &netlist.subcircuits,
        &netlist.models,
        FlattenerConfig::debug(),
    );
    let flattened_elements = match flattener.flatten_with_abort(netlist, abort) {
        Ok(elements) => Some(elements),
        Err(ParseWithAbortError::Aborted) => return Err(ParseWithAbortError::Aborted),
        Err(ParseWithAbortError::Parse(_)) => None,
    };
    let xyce_dialect =
        netlist.params.expression_dialect() == crate::config::ExpressionDialect::Xyce;
    for (request_index, request) in netlist.output_requests.iter().enumerate() {
        poll_parse_abort(abort, request_index)?;
        for (expression_index, expression) in request.expressions.iter().enumerate() {
            poll_parse_abort(abort, expression_index)?;
            poll_parse_text(abort, expression)?;
            let runtime_scalar_identifiers = output_runtime_scalar_identifiers(
                netlist,
                flattened_elements.as_deref(),
                request.analysis,
                Some(expression),
                abort,
            )?;
            let authored_expression = expression;
            let expanded_expression = super::expr::expand_output_user_functions_with_abort(
                authored_expression,
                &netlist.params,
                &runtime_scalar_identifiers,
                abort,
            )
            .map_err(|error| match error {
                super::expr::BehavioralPreparationError::Aborted => ParseWithAbortError::Aborted,
                super::expr::BehavioralPreparationError::Semantic(detail) => {
                    output_expression_error(
                        request,
                        authored_expression,
                        OutputExpressionIssue::Syntax { detail },
                    )
                }
            })?;
            if xyce_dialect && let Some(issue) = first_xyce_ddx_issue(&expanded_expression, abort)?
            {
                return Err(output_expression_error(request, authored_expression, issue));
            }
            let protected = protect_xyce_output_operands(
                &expanded_expression,
                request.analysis,
                flattened_elements.as_deref(),
                &runtime_scalar_identifiers,
                abort,
            )
            .map_err(|error| match error {
                OutputOperandProtectionError::Aborted => ParseWithAbortError::Aborted,
                OutputOperandProtectionError::Invalid(issue) => {
                    output_expression_error(request, authored_expression, issue)
                }
            })?;
            let prepared = super::expr::prepare_behavioral_expression_with_abort(
                &protected,
                &netlist.params,
                abort,
            )
            .map_err(|error| match error {
                super::expr::BehavioralPreparationError::Aborted => ParseWithAbortError::Aborted,
                super::expr::BehavioralPreparationError::Semantic(reason) => {
                    output_expression_error(
                        request,
                        authored_expression,
                        OutputExpressionIssue::Syntax { detail: reason },
                    )
                }
            })?;
            // User-defined functions and symbolic parameters can expand to
            // output-only operands, so protect once more after expansion.
            ensure_parse_not_aborted(abort)?;
            let prepared = protect_xyce_output_operands(
                &prepared,
                request.analysis,
                flattened_elements.as_deref(),
                &runtime_scalar_identifiers,
                abort,
            )
            .map_err(|error| match error {
                OutputOperandProtectionError::Aborted => ParseWithAbortError::Aborted,
                OutputOperandProtectionError::Invalid(issue) => {
                    output_expression_error(request, authored_expression, issue)
                }
            })?;
            let parsed = match super::expr::parse_expression_with_abort(&prepared, abort) {
                Ok(parsed) => parsed,
                Err(super::expr::ParseExpressionWithAbortError::Aborted) => {
                    return Err(ParseWithAbortError::Aborted);
                }
                Err(super::expr::ParseExpressionWithAbortError::Parse(error)) => {
                    return Err(ParseError::OutputExpressionValidation(Box::new(
                        OutputExpressionValidationError {
                            directive: request.directive,
                            origin: request.origin.clone(),
                            expression: authored_expression.clone(),
                            issue: OutputExpressionIssue::Syntax {
                                detail: error.to_string(),
                            },
                        },
                    ))
                    .into());
                }
            };
            ensure_parse_not_aborted(abort)?;
            let issue =
                if let Some(issue) = first_output_function_issue(&parsed, xyce_dialect, abort)? {
                    issue
                } else if let Some(identifier) =
                    first_unresolved_output_identifier(&parsed, &runtime_scalar_identifiers, abort)?
                {
                    OutputExpressionIssue::UnresolvedIdentifier {
                        identifier: identifier.to_ascii_uppercase(),
                    }
                } else if let Err(error) = crate::expr::parse_expression_strict_with_abort(
                    &strict_output_validation_expression(
                        &parsed,
                        &runtime_scalar_identifiers,
                        xyce_dialect,
                        abort,
                    )?,
                    abort,
                ) {
                    match error {
                        crate::expr::ParseExpressionWithAbortError::Aborted => {
                            return Err(ParseWithAbortError::Aborted);
                        }
                        crate::expr::ParseExpressionWithAbortError::Parse(error) => {
                            OutputExpressionIssue::Syntax {
                                detail: error.to_string(),
                            }
                        }
                    }
                } else {
                    continue;
                };
            return Err(ParseError::OutputExpressionValidation(Box::new(
                OutputExpressionValidationError {
                    directive: request.directive,
                    origin: request.origin.clone(),
                    expression: authored_expression.clone(),
                    issue,
                },
            ))
            .into());
        }
    }
    ensure_parse_not_aborted(abort)
}

fn output_expression_error(
    request: &OutputRequest,
    expression: &str,
    issue: OutputExpressionIssue,
) -> ParseWithAbortError {
    ParseError::OutputExpressionValidation(Box::new(OutputExpressionValidationError {
        directive: request.directive,
        origin: request.origin.clone(),
        expression: expression.to_string(),
        issue,
    }))
    .into()
}

fn first_output_function_issue(
    expression: &super::expr::Expr,
    xyce_dialect: bool,
    abort: &dyn AbortSignal,
) -> Result<Option<OutputExpressionIssue>, ParseWithAbortError> {
    use super::expr::Expr;
    let mut pending = vec![expression];
    let mut visited = 0usize;
    while let Some(expression) = pending.pop() {
        poll_parse_abort(abort, visited)?;
        visited = visited.saturating_add(1);
        match expression {
            Expr::FnCall { name, args } => {
                let upper = name.to_ascii_uppercase();
                if let Some((minimum, maximum)) = output_only_function_arity(&upper, xyce_dialect) {
                    if !(minimum..=maximum).contains(&args.len()) {
                        return Ok(Some(OutputExpressionIssue::Syntax {
                            detail: format!(
                                "function {upper} expects {} argument{} but got {}",
                                if minimum == maximum {
                                    minimum.to_string()
                                } else {
                                    format!("{minimum}..={maximum}")
                                },
                                if minimum == maximum && minimum == 1 {
                                    ""
                                } else {
                                    "s"
                                },
                                args.len()
                            ),
                        }));
                    }
                } else if crate::expr::Function::from_name(&upper).is_none() {
                    return Ok(Some(OutputExpressionIssue::UnknownFunction {
                        function: upper,
                    }));
                }
                pending.extend(args.iter().rev());
            }
            Expr::UnaryOp { operand, .. } => pending.push(operand),
            Expr::BinOp { left, right, .. } => {
                pending.push(right);
                pending.push(left);
            }
            Expr::Number(_) | Expr::ComplexNumber(_) | Expr::StringLiteral(_) | Expr::Param(_) => {}
        }
    }
    Ok(None)
}

fn output_only_function_arity(name: &str, xyce_dialect: bool) -> Option<(usize, usize)> {
    match name {
        "R" | "RE" | "REAL" | "IMG" | "IMAG" | "PH" | "PHASE" | "DB" => Some((1, 1)),
        "DDX" if xyce_dialect => Some((2, 2)),
        "RAND" | "RANDOM" => Some((0, 0)),
        "UNIF" | "AUNIF" => Some((2, 2)),
        "GAUSS" | "AGAUSS" => Some((2, 3)),
        _ => None,
    }
}

fn first_xyce_ddx_issue(
    source: &str,
    abort: &dyn AbortSignal,
) -> Result<Option<OutputExpressionIssue>, ParseWithAbortError> {
    first_xyce_ddx_issue_in_source(source, abort)
}

fn first_xyce_ddx_issue_in_source(
    source: &str,
    abort: &dyn AbortSignal,
) -> Result<Option<OutputExpressionIssue>, ParseWithAbortError> {
    let bytes = source.as_bytes();
    let mut index = 0usize;
    while index < bytes.len() {
        poll_parse_abort(abort, index)?;
        if bytes[index] == b'"' {
            let delimiter = bytes[index];
            index += 1;
            while index < bytes.len()
                && (bytes[index] != delimiter || is_backslash_escaped(bytes, index))
            {
                poll_parse_abort(abort, index)?;
                index += 1;
            }
            index = index.saturating_add(1);
            continue;
        }
        if !(bytes[index] as char).is_ascii_alphabetic() {
            index += 1;
            continue;
        }
        let start = index;
        index += 1;
        while index < bytes.len()
            && ((bytes[index] as char).is_ascii_alphanumeric() || bytes[index] == b'_')
        {
            poll_parse_abort(abort, index)?;
            index += 1;
        }
        if !source[start..index].eq_ignore_ascii_case("DDX") {
            continue;
        }
        let mut open = index;
        while open < bytes.len() && (bytes[open] as char).is_ascii_whitespace() {
            poll_parse_abort(abort, open)?;
            open += 1;
        }
        if open == bytes.len() || bytes[open] != b'(' {
            continue;
        }
        let Some(close) = matching_output_parenthesis_with_abort(bytes, open, abort)? else {
            return Ok(Some(OutputExpressionIssue::Syntax {
                detail: "DDX is missing its closing ')'".to_string(),
            }));
        };
        let call = &source[start..=close];
        let parsed = match super::expr::parse_expression_with_abort(call, abort) {
            Ok(parsed) => parsed,
            Err(super::expr::ParseExpressionWithAbortError::Aborted) => {
                return Err(ParseWithAbortError::Aborted);
            }
            Err(super::expr::ParseExpressionWithAbortError::Parse(error)) => {
                return Ok(Some(OutputExpressionIssue::Syntax {
                    detail: format!("invalid DDX expression: {error}"),
                }));
            }
        };
        if let Some(issue) = validate_xyce_ddx_expression_tree(&parsed, abort)? {
            return Ok(Some(issue));
        }
        index = close + 1;
    }
    ensure_parse_not_aborted(abort)?;
    Ok(None)
}

fn validate_xyce_ddx_expression_tree(
    expression: &super::expr::Expr,
    abort: &dyn AbortSignal,
) -> Result<Option<OutputExpressionIssue>, ParseWithAbortError> {
    use super::expr::Expr;
    let mut pending = vec![expression];
    let mut visited = 0usize;
    while let Some(expression) = pending.pop() {
        poll_parse_abort(abort, visited)?;
        visited = visited.saturating_add(1);
        match expression {
            Expr::FnCall { name, args } if name.eq_ignore_ascii_case("DDX") => {
                if let Some(issue) = validate_one_xyce_ddx_expression(expression, abort)? {
                    return Ok(Some(issue));
                }
                pending.extend(args.iter().rev());
            }
            Expr::FnCall { args, .. } => pending.extend(args.iter().rev()),
            Expr::UnaryOp { operand, .. } => pending.push(operand),
            Expr::BinOp { left, right, .. } => {
                pending.push(right);
                pending.push(left);
            }
            Expr::Number(_) | Expr::ComplexNumber(_) | Expr::StringLiteral(_) | Expr::Param(_) => {}
        }
    }
    Ok(None)
}

fn validate_one_xyce_ddx_expression(
    expression: &super::expr::Expr,
    abort: &dyn AbortSignal,
) -> Result<Option<OutputExpressionIssue>, ParseWithAbortError> {
    use super::expr::Expr;
    let Expr::FnCall { name, args } = expression else {
        return Ok(Some(OutputExpressionIssue::Syntax {
            detail: "DDX did not parse as a function call".to_string(),
        }));
    };
    if !name.eq_ignore_ascii_case("DDX") {
        return Ok(Some(OutputExpressionIssue::Syntax {
            detail: "DDX did not parse as a DDX operator".to_string(),
        }));
    }
    let [left, right] = args.as_slice() else {
        return Ok(Some(OutputExpressionIssue::Syntax {
            detail: format!("DDX expects exactly 2 arguments but got {}", args.len()),
        }));
    };
    let Some(target) = xyce_ddx_target(right) else {
        return Ok(Some(OutputExpressionIssue::Syntax {
            detail: "DDX differentiation target must be a parameter, V(node), or I(device)"
                .to_string(),
        }));
    };
    if !xyce_ddx_left_contains_target(left, &target, abort)? {
        return Ok(Some(OutputExpressionIssue::Syntax {
            detail: format!(
                "DDX differentiation target {} is not present in its first argument",
                target.label()
            ),
        }));
    }
    Ok(None)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum XyceDdxTarget {
    Parameter(String),
    Voltage(String),
    Current(String),
}

impl XyceDdxTarget {
    fn label(&self) -> String {
        match self {
            Self::Parameter(name) => name.clone(),
            Self::Voltage(node) => format!("V({node})"),
            Self::Current(device) => format!("I({device})"),
        }
    }
}

fn xyce_ddx_target(expression: &super::expr::Expr) -> Option<XyceDdxTarget> {
    use super::expr::Expr;
    match expression {
        Expr::Param(name) if super::expr::runtime_special_quantity(name).is_none() => {
            Some(XyceDdxTarget::Parameter(name.trim().to_ascii_uppercase()))
        }
        Expr::FnCall { name, args } if name.eq_ignore_ascii_case("V") => {
            let [argument] = args.as_slice() else {
                return None;
            };
            xyce_ddx_atomic_reference(argument).map(XyceDdxTarget::Voltage)
        }
        Expr::FnCall { name, args } if name.eq_ignore_ascii_case("I") => {
            let [Expr::Param(device)] = args.as_slice() else {
                return None;
            };
            Some(XyceDdxTarget::Current(canonical_symbol(device)))
        }
        _ => None,
    }
}

fn xyce_ddx_atomic_reference(expression: &super::expr::Expr) -> Option<String> {
    match expression {
        super::expr::Expr::Param(name) => Some(canonical_symbol(name)),
        super::expr::Expr::Number(value) if value.is_finite() => Some(value.to_string()),
        _ => None,
    }
}

fn xyce_ddx_left_contains_target(
    expression: &super::expr::Expr,
    target: &XyceDdxTarget,
    abort: &dyn AbortSignal,
) -> Result<bool, ParseWithAbortError> {
    use super::expr::Expr;
    let mut pending = vec![expression];
    let mut visited = 0usize;
    while let Some(expression) = pending.pop() {
        poll_parse_abort(abort, visited)?;
        visited = visited.saturating_add(1);
        if xyce_ddx_target(expression).as_ref() == Some(target) {
            return Ok(true);
        }
        match expression {
            Expr::UnaryOp { operand, .. } => pending.push(operand),
            Expr::BinOp { left, right, .. } => {
                pending.push(right);
                pending.push(left);
            }
            Expr::FnCall { name, .. }
                if name.eq_ignore_ascii_case("V") || name.eq_ignore_ascii_case("I") => {}
            Expr::FnCall { args, .. } => pending.extend(args.iter().rev()),
            Expr::Number(_) | Expr::ComplexNumber(_) | Expr::StringLiteral(_) | Expr::Param(_) => {}
        }
    }
    Ok(false)
}

fn first_unresolved_output_identifier<'a>(
    expression: &'a super::expr::Expr,
    runtime_scalar_identifiers: &HashSet<String>,
    abort: &dyn AbortSignal,
) -> Result<Option<&'a str>, ParseWithAbortError> {
    use super::expr::Expr;
    let mut pending = vec![expression];
    let mut visited = 0usize;
    while let Some(expression) = pending.pop() {
        poll_parse_abort(abort, visited)?;
        visited = visited.saturating_add(1);
        match expression {
            Expr::Param(name) if super::expr::runtime_special_quantity(name).is_some() => {}
            Expr::Param(name) if runtime_scalar_identifiers.contains(&canonical_symbol(name)) => {}
            Expr::Param(name) => return Ok(Some(name)),
            Expr::UnaryOp { operand, .. } => pending.push(operand),
            Expr::BinOp { left, right, .. } => {
                pending.push(right);
                pending.push(left);
            }
            Expr::FnCall { args, .. } => pending.extend(args.iter().rev()),
            Expr::Number(_) | Expr::ComplexNumber(_) | Expr::StringLiteral(_) => {}
        }
    }
    Ok(None)
}

fn output_runtime_scalar_identifiers(
    netlist: &Netlist,
    flattened_elements: Option<&[Element]>,
    analysis: Option<OutputAnalysisKind>,
    atomic_device_expression: Option<&str>,
    abort: &dyn AbortSignal,
) -> Result<HashSet<String>, ParseWithAbortError> {
    let mut names = HashSet::new();
    for (index, element) in netlist.elements.iter().enumerate() {
        poll_parse_abort(abort, index)?;
        if element_is_independent_source(&element.kind) {
            names.insert(canonical_symbol(&element.name));
        }
    }
    if let Some(authored) = atomic_device_expression {
        let authored = canonical_symbol(authored.trim());
        for (index, element) in flattened_elements
            .unwrap_or(&netlist.elements)
            .iter()
            .enumerate()
        {
            poll_parse_abort(abort, index)?;
            if element_supports_bare_output_scalar(&element.kind)
                && canonical_symbol(&element.name) == authored
            {
                names.insert(authored.clone());
                break;
            }
        }
    }
    let offset = names.len();
    if analysis == Some(OutputAnalysisKind::Tran) {
        for (index, measurement) in netlist.measurements.iter().enumerate() {
            poll_parse_abort(abort, offset.saturating_add(index))?;
            if OutputAnalysisKind::from_keyword(&measurement.analysis)
                == Some(OutputAnalysisKind::Tran)
                && matches!(
                    measurement.measure_type,
                    super::measure::MeasureType::Equation { .. }
                )
            {
                names.insert(canonical_symbol(&measurement.name));
            }
        }
    }
    ensure_parse_not_aborted(abort)?;
    Ok(names)
}

/// Validate authored output expressions without cancellation.
pub fn validate_output_expressions(netlist: &Netlist) -> Result<(), ParseError> {
    super::finish_non_aborting_parse(validate_output_expressions_with_abort(
        netlist,
        &crate::abort_signal::NoAbort,
    ))
}

/// Validate the complete authored output contract in deterministic order.
pub fn validate_output_requests_with_abort(
    netlist: &Netlist,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    validate_output_expressions_with_abort(netlist, abort)?;
    validate_output_symbols_with_abort(netlist, abort)
}

/// Validate the complete authored output contract without cancellation.
pub fn validate_output_requests(netlist: &Netlist) -> Result<(), ParseError> {
    super::finish_non_aborting_parse(validate_output_requests_with_abort(
        netlist,
        &crate::abort_signal::NoAbort,
    ))
}

/// Validate every typed output dependency against the flattened namespace.
///
/// The operation is transactional: all dependencies are scanned into a local
/// error vector, and the netlist is never mutated. Cancellation is polled while
/// traversing requests and flattened symbols. If flattening itself is invalid,
/// that existing error remains owned by the ordinary elaboration stage rather
/// than being reordered behind output validation.
pub fn validate_output_symbols_with_abort(
    netlist: &Netlist,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    if netlist.output_requests.is_empty() {
        return Ok(());
    }
    ensure_parse_not_aborted(abort)?;

    let mut flattener = Flattener::with_models_config(
        &netlist.subcircuits,
        &netlist.models,
        FlattenerConfig::debug(),
    );
    let elements = match flattener.flatten_with_abort(netlist, abort) {
        Ok(elements) => elements,
        Err(ParseWithAbortError::Aborted) => return Err(ParseWithAbortError::Aborted),
        Err(ParseWithAbortError::Parse(_)) => {
            // Do not change established flattening-error precedence. The same
            // elaboration will report its typed failure before topology/stamping.
            return Ok(());
        }
    };

    let mut nodes = HashSet::new();
    let mut devices = HashSet::new();
    nodes.insert("0".to_string());
    for (index, element) in elements.iter().enumerate() {
        poll_parse_abort(abort, index)?;
        devices.insert(canonical_symbol(&element.name));
        if let ElementKind::Coupling {
            model: Some(_),
            inductors,
            ..
        } = &element.kind
            && !inductors.is_empty()
        {
            // Xyce's nonlinear Core device owns internal vectors under the
            // generated YMIN!KNAME namespace (for example
            // N(YMIN!KTRANS1_H)).
            devices.insert(canonical_symbol(&format!("YMIN!{}", element.name)));
        }
        for node in &element.nodes {
            nodes.insert(canonical_symbol(node));
        }
        collect_embedded_element_nodes(&element.kind, &mut nodes);
    }
    let node_aliases = collect_interface_node_aliases_with_abort(netlist, abort)?;
    ensure_parse_not_aborted(abort)?;

    let mut unresolved = Vec::new();
    for (request_index, request) in netlist.output_requests.iter().enumerate() {
        poll_parse_abort(abort, request_index)?;
        let mut expanded_dependencies = Vec::new();
        for expression in &request.expressions {
            let runtime_scalar_identifiers = output_runtime_scalar_identifiers(
                netlist,
                Some(&elements),
                request.analysis,
                Some(expression),
                abort,
            )?;
            let expanded = super::expr::expand_output_user_functions_with_abort(
                expression,
                &netlist.params,
                &runtime_scalar_identifiers,
                abort,
            )
            .map_err(|error| match error {
                super::expr::BehavioralPreparationError::Aborted => ParseWithAbortError::Aborted,
                super::expr::BehavioralPreparationError::Semantic(detail) => {
                    output_expression_error(
                        request,
                        expression,
                        OutputExpressionIssue::Syntax { detail },
                    )
                }
            })?;
            expanded_dependencies.extend(extract_output_dependencies_with_context(&expanded, true));
        }
        let mut ordered = validation_order(request);
        ordered.extend(expanded_dependencies.iter());
        let mut seen = HashSet::new();
        for dependency in ordered {
            poll_parse_text(abort, &dependency.symbol)?;
            let canonical = canonical_dependency_symbol(
                &dependency.symbol,
                dependency.kind,
                netlist.ground_policy(),
            );
            let dedup_key = (dependency.kind, canonical.clone());
            if request.directive.is_direct_output()
                && !dependency.expression
                && !seen.insert(dedup_key)
            {
                continue;
            }
            let matched =
                if analysis_owned_output_vector_exists(netlist, request, dependency, &canonical) {
                    true
                } else {
                    let namespace = match dependency.kind {
                        OutputSymbolKind::Node => &nodes,
                        OutputSymbolKind::Device => &devices,
                    };
                    if dependency.kind == OutputSymbolKind::Node {
                        let node_match =
                            namespace_matches_with_aliases(namespace, &node_aliases, &canonical);
                        if node_match {
                            true
                        } else if is_internal_node_accessor(
                            &dependency.operator.to_ascii_uppercase(),
                        ) {
                            n_operator_device_vector_exists(&devices, &canonical)
                        } else {
                            false
                        }
                    } else {
                        namespace_matches(namespace, &canonical)
                    }
                };
            if !matched {
                unresolved.push(UnresolvedOutputSymbol {
                    directive: request.directive,
                    origin: request.origin.clone(),
                    operator: dependency.operator.clone(),
                    symbol: dependency.symbol.clone(),
                    kind: dependency.kind,
                });
            }
        }
    }
    ensure_parse_not_aborted(abort)?;
    if unresolved.is_empty() {
        Ok(())
    } else {
        Err(
            ParseError::OutputSymbolValidation(Box::new(OutputSymbolValidationError {
                unresolved,
            }))
            .into(),
        )
    }
}

fn analysis_owned_output_vector_exists(
    netlist: &Netlist,
    request: &OutputRequest,
    dependency: &OutputSymbolDependency,
    canonical: &str,
) -> bool {
    if dependency.kind != OutputSymbolKind::Node
        || !matches!(
            canonical,
            "INOISE_SPECTRUM" | "ONOISE_SPECTRUM" | "INOISE" | "ONOISE"
        )
        || !netlist.analyses.iter().any(|analysis| {
            matches!(
                analysis,
                AnalysisCommand::Noise { .. } | AnalysisCommand::NoiseData { .. }
            )
        })
    {
        return false;
    }

    match request.directive {
        OutputDirectiveKind::Print | OutputDirectiveKind::Plot | OutputDirectiveKind::Measure => {
            request.analysis == Some(OutputAnalysisKind::Noise)
        }
        OutputDirectiveKind::Save | OutputDirectiveKind::Probe => true,
        OutputDirectiveKind::Four => false,
    }
}

/// Build the complete flattened node namespace used by semantic validators.
///
/// This includes element terminals, control/embedded nodes, and one-hop
/// hierarchy-interface aliases. `None` preserves ordinary elaboration-error
/// precedence when a deck cannot yet be flattened.
pub(crate) fn collect_output_node_namespace_from_elements_with_abort(
    netlist: &Netlist,
    elements: &[Element],
    abort: &dyn AbortSignal,
) -> Result<HashSet<String>, ParseWithAbortError> {
    ensure_parse_not_aborted(abort)?;
    let mut nodes = HashSet::new();
    nodes.insert("0".to_string());
    for (index, element) in elements.iter().enumerate() {
        poll_parse_abort(abort, index)?;
        for node in &element.nodes {
            nodes.insert(canonical_symbol(node));
        }
        collect_embedded_element_nodes(&element.kind, &mut nodes);
    }
    let aliases = collect_interface_node_aliases_with_abort(netlist, abort)?;
    for (alias, target) in aliases.iter() {
        if nodes.contains(target) {
            nodes.insert(alias.to_string());
        }
    }
    ensure_parse_not_aborted(abort)?;
    Ok(nodes)
}

fn n_operator_device_vector_exists(devices: &HashSet<String>, canonical: &str) -> bool {
    if canonical
        .rsplit_once('.')
        .is_some_and(|(device, _)| namespace_matches(devices, device))
    {
        return true;
    }
    // Xyce exposes model-owned internal-node and branch solution vectors as
    // N(<device>_<vector>). Device names may themselves contain underscores,
    // so test every separator and accept only a prefix in the actual flattened
    // device namespace; the generated model remains the authority for whether
    // that vector is present at execution time.
    canonical
        .match_indices('_')
        .any(|(separator, _)| separator != 0 && namespace_matches(devices, &canonical[..separator]))
}

/// Validate output dependencies without cancellation.
///
/// Simulation engines invoke this transactionally before circuit
/// construction. Tooling may call it earlier to provide editor diagnostics
/// without making syntactic parsing depend on a fully constructed circuit.
pub fn validate_output_symbols(netlist: &Netlist) -> Result<(), ParseError> {
    super::finish_non_aborting_parse(validate_output_symbols_with_abort(
        netlist,
        &crate::abort_signal::NoAbort,
    ))
}

fn validation_order(request: &OutputRequest) -> Vec<&OutputSymbolDependency> {
    if !request.directive.is_direct_output() {
        return request
            .dependencies
            .iter()
            .filter(|dependency| request.expressions.is_empty() || !dependency.expression)
            .collect();
    }
    // Xyce creates direct lead-current operators before solution-vector node
    // operators. Keep lexical order within each namespace.
    let mut devices = request
        .dependencies
        .iter()
        .filter(|dependency| !dependency.expression && dependency.kind == OutputSymbolKind::Device)
        .collect::<Vec<_>>();
    let mut nodes = request
        .dependencies
        .iter()
        .filter(|dependency| !dependency.expression && dependency.kind == OutputSymbolKind::Node)
        .collect::<Vec<_>>();
    devices.sort_by_key(|dependency| canonical_symbol(&dependency.symbol));
    nodes.sort_by_key(|dependency| canonical_symbol(&dependency.symbol));
    devices.extend(nodes);
    devices
}

fn collect_embedded_element_nodes(kind: &ElementKind, nodes: &mut HashSet<String>) {
    let mut insert = |node: &str| {
        nodes.insert(canonical_symbol(node));
    };
    match kind {
        ElementKind::Vcvs { control_nodes, .. } | ElementKind::Vccs { control_nodes, .. } => {
            insert(&control_nodes.0);
            insert(&control_nodes.1);
        }
        ElementKind::VSwitch {
            control_pos,
            control_neg,
            ..
        } => {
            insert(control_pos);
            insert(control_neg);
        }
        ElementKind::Xspice { ports, .. } => {
            for port in ports {
                use super::XspicePort;
                match port {
                    XspicePort::Analog(node)
                    | XspicePort::Digital(node)
                    | XspicePort::ExplicitDigital(node)
                    | XspicePort::DigitalInverted(node)
                    | XspicePort::Conductance(node)
                    | XspicePort::Current(node)
                    | XspicePort::VoltageName(node)
                    | XspicePort::Hybrid(node) => insert(node),
                    XspicePort::AnalogVector(nodes) | XspicePort::DigitalVector(nodes) => {
                        for node in nodes {
                            insert(node);
                        }
                    }
                    XspicePort::DigitalVectorMixed(nodes) => {
                        for node in nodes {
                            insert(&node.name);
                        }
                    }
                    XspicePort::DifferentialVoltage { pos, neg }
                    | XspicePort::DifferentialCurrent { pos, neg }
                    | XspicePort::DifferentialConductance { pos, neg }
                    | XspicePort::DifferentialHybrid { pos, neg } => {
                        insert(pos);
                        insert(neg);
                    }
                    XspicePort::Null => {}
                }
            }
        }
        _ => {}
    }
}

fn canonical_dependency_symbol(
    symbol: &str,
    kind: OutputSymbolKind,
    ground_policy: super::GroundPolicy,
) -> String {
    let canonical = canonical_symbol(symbol);
    if kind == OutputSymbolKind::Node && ground_policy.is_ground(&canonical) {
        "0".to_string()
    } else {
        canonical
    }
}

pub(crate) fn canonical_symbol(symbol: &str) -> String {
    symbol
        .trim()
        .chars()
        .map(|ch| {
            if ch == ':' {
                '.'
            } else {
                ch.to_ascii_uppercase()
            }
        })
        .collect()
}

/// Canonical map from subcircuit interface names to flattened solution nodes.
///
/// The representation stays opaque so validation and runtime consumers share
/// the same case and hierarchy-separator rules. Targets remain one-hop: a
/// target can itself be a physical node whose spelling also happens to be an
/// interface alias, and that physical node must win.
#[derive(Debug, Clone, Default)]
pub(crate) struct InterfaceNodeAliases {
    targets: HashMap<String, String>,
}

impl InterfaceNodeAliases {
    /// Resolve an authored alias spelling to its direct flattened target.
    pub(crate) fn resolve(&self, authored: &str) -> Option<&str> {
        let canonical = canonical_symbol(authored);
        self.targets.get(&canonical).map(String::as_str)
    }

    pub(crate) fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.targets
            .iter()
            .map(|(alias, target)| (alias.as_str(), target.as_str()))
    }

    fn keys(&self) -> impl Iterator<Item = &String> {
        self.targets.keys()
    }
}

/// Collect interface aliases for non-abortable analysis post-processing.
#[cfg(test)]
pub(crate) fn collect_interface_node_aliases(
    netlist: &Netlist,
) -> Result<InterfaceNodeAliases, ParseError> {
    super::finish_non_aborting_parse(collect_interface_node_aliases_with_abort(
        netlist,
        &crate::abort_signal::NoAbort,
    ))
}

/// Collect only interface aliases referenced by one analysis adapter.
///
/// Only hierarchy paths that can own a requested alias are traversed, and only
/// requested formal ports are retained. Exhaustive output-symbol validation
/// remains owned by the full collector. This projection matches Xyce's
/// requested-alias materialization policy without scaling measurement setup
/// with unrelated hierarchy.
#[cfg(test)]
fn collect_requested_interface_node_aliases(
    netlist: &Netlist,
    requested: &HashSet<String>,
) -> Result<InterfaceNodeAliases, ParseError> {
    super::finish_non_aborting_parse(collect_requested_interface_node_aliases_with_abort(
        netlist,
        requested,
        &crate::abort_signal::NoAbort,
    ))
}

pub(crate) fn collect_requested_interface_node_aliases_with_abort(
    netlist: &Netlist,
    requested: &HashSet<String>,
    abort: &dyn AbortSignal,
) -> Result<InterfaceNodeAliases, ParseWithAbortError> {
    super::ensure_parse_not_aborted(abort)?;
    let mut canonical_requested = HashSet::with_capacity(requested.len());
    for (index, alias) in requested.iter().enumerate() {
        super::poll_parse_abort(abort, index)?;
        canonical_requested.insert(canonical_symbol(alias));
    }
    let requested = canonical_requested;
    if requested.is_empty() {
        return Ok(InterfaceNodeAliases::default());
    }
    collect_interface_node_aliases_impl(netlist, Some(&requested), abort)
}

fn collect_interface_node_aliases_with_abort(
    netlist: &Netlist,
    abort: &dyn AbortSignal,
) -> Result<InterfaceNodeAliases, ParseWithAbortError> {
    collect_interface_node_aliases_impl(netlist, None, abort)
}

fn collect_interface_node_aliases_impl(
    netlist: &Netlist,
    requested_aliases: Option<&HashSet<String>>,
    abort: &dyn AbortSignal,
) -> Result<InterfaceNodeAliases, ParseWithAbortError> {
    let config = FlattenerConfig::default();
    collect_interface_node_aliases_impl_with_limits(
        netlist,
        requested_aliases,
        abort,
        config.max_depth,
        config.max_elements,
    )
}

fn collect_interface_node_aliases_impl_with_limits(
    netlist: &Netlist,
    requested_aliases: Option<&HashSet<String>>,
    abort: &dyn AbortSignal,
    max_depth: usize,
    max_elements: usize,
) -> Result<InterfaceNodeAliases, ParseWithAbortError> {
    fn requested_instance_paths(
        requested_aliases: Option<&HashSet<String>>,
    ) -> (Option<HashSet<String>>, Option<HashSet<String>>) {
        let Some(requested_aliases) = requested_aliases else {
            return (None, None);
        };
        let mut visit = HashSet::new();
        let mut descend = HashSet::new();
        for alias in requested_aliases {
            let Some((deepest_instance, _)) = alias.rsplit_once('.') else {
                continue;
            };
            visit.insert(deepest_instance.to_string());
            let mut child = deepest_instance;
            while let Some((parent, _)) = child.rsplit_once('.') {
                visit.insert(parent.to_string());
                descend.insert(parent.to_string());
                child = parent;
            }
        }
        (Some(visit), Some(descend))
    }

    struct AliasCollector<'a> {
        definitions: HashMap<String, &'a super::SubcircuitDef>,
        external_subcircuits: HashSet<String>,
        globals: &'a HashSet<String>,
        ground_policy: super::GroundPolicy,
        aliases: HashMap<String, String>,
        requested_aliases: Option<&'a HashSet<String>>,
        instances_to_visit: Option<HashSet<String>>,
        instances_to_descend: Option<HashSet<String>>,
        max_depth: usize,
        max_elements: usize,
        emitted_elements: usize,
        traversal_steps: usize,
    }

    impl<'a> AliasCollector<'a> {
        fn charge_emitted_element(&mut self) -> Result<(), ParseWithAbortError> {
            let requested = self.emitted_elements.saturating_add(1);
            crate::resource::ResourceLimitError::ensure(
                crate::resource::ResourceKind::FlattenedElements,
                requested,
                self.max_elements,
            )
            .map_err(ParseError::from)?;
            self.emitted_elements = requested;
            Ok(())
        }

        fn remap_node(&self, node: &str, prefix: &str, ports: &HashMap<String, String>) -> String {
            let canonical =
                canonical_dependency_symbol(node, OutputSymbolKind::Node, self.ground_policy);
            if canonical == "0" || self.globals.contains(&canonical) || canonical.starts_with("$G")
            {
                return canonical;
            }
            if let Some(mapped) = ports.get(&canonical) {
                return mapped.clone();
            }
            if prefix.is_empty() {
                canonical
            } else {
                format!("{prefix}.{canonical}")
            }
        }

        fn collect(
            &mut self,
            root_elements: &'a [super::Element],
            abort: &dyn AbortSignal,
        ) -> Result<(), ParseWithAbortError> {
            struct Frame<'a> {
                elements: &'a [super::Element],
                next_index: usize,
                prefix: String,
                parent_ports: HashMap<String, String>,
                active_definitions: Vec<String>,
                depth: usize,
            }

            let mut frames = vec![Frame {
                elements: root_elements,
                next_index: 0,
                prefix: String::new(),
                parent_ports: HashMap::new(),
                active_definitions: Vec::new(),
                depth: 0,
            }];
            while !frames.is_empty() {
                let frame_index = frames.len() - 1;
                if frames[frame_index].next_index >= frames[frame_index].elements.len() {
                    frames.pop();
                    continue;
                }
                let element_index = frames[frame_index].next_index;
                frames[frame_index].next_index += 1;
                poll_parse_abort(abort, self.traversal_steps)?;
                self.traversal_steps = self.traversal_steps.saturating_add(1);
                let frame = &frames[frame_index];
                let element = &frame.elements[element_index];
                crate::resource::ResourceLimitError::ensure(
                    crate::resource::ResourceKind::HierarchyDepth,
                    frame.depth,
                    self.max_depth,
                )
                .map_err(ParseError::from)?;
                let ElementKind::Subcircuit { subckt_name, .. } = &element.kind else {
                    if self.requested_aliases.is_none() {
                        self.charge_emitted_element()?;
                    }
                    continue;
                };
                let instance = if frame.prefix.is_empty() {
                    canonical_symbol(&element.name)
                } else {
                    format!("{}.{}", frame.prefix, canonical_symbol(&element.name))
                };
                if self
                    .instances_to_visit
                    .as_ref()
                    .is_some_and(|instances| !instances.contains(&instance))
                {
                    continue;
                }
                let canonical_subcircuit = subckt_name.to_ascii_uppercase();
                let Some(definition) = self.definitions.get(&canonical_subcircuit).copied() else {
                    if self.external_subcircuits.contains(&canonical_subcircuit) {
                        self.charge_emitted_element()?;
                        continue;
                    }
                    return Err(ParseError::Syntax {
                        line: 0,
                        message: format!("Undefined subcircuit: {subckt_name}"),
                    }
                    .into());
                };
                let mapped_ports = definition
                    .ports
                    .iter()
                    .zip(&element.nodes)
                    .map(|(formal, actual)| {
                        (
                            formal,
                            self.remap_node(actual, &frame.prefix, &frame.parent_ports),
                        )
                    })
                    .collect::<Vec<_>>();
                super::flattener::validate_subcircuit_port_bindings(
                    definition,
                    subckt_name,
                    &element.name,
                    &instance,
                    element.nodes.len(),
                    &mapped_ports,
                    self.globals,
                    abort,
                )?;
                if let Some(start) = frame
                    .active_definitions
                    .iter()
                    .position(|name| name == &canonical_subcircuit)
                {
                    let mut chain = frame.active_definitions[start..].to_vec();
                    chain.push(canonical_subcircuit);
                    return Err(ParseError::Syntax {
                        line: 0,
                        message: format!(
                            "Recursive subcircuit instantiation at '{instance}': {}",
                            chain.join(" -> ")
                        ),
                    }
                    .into());
                }
                let mut child_ports = HashMap::new();
                for (formal, target) in mapped_ports {
                    let formal = canonical_symbol(formal);
                    let alias = format!("{instance}.{formal}");
                    if self
                        .requested_aliases
                        .is_none_or(|requested| requested.contains(&alias))
                    {
                        if let Some(existing) = self.aliases.get(&alias) {
                            if existing != &target {
                                return Err(ParseError::Syntax {
                                    line: 0,
                                    message: format!(
                                        "Interface alias '{alias}' resolves ambiguously to '{existing}' and '{target}'"
                                    ),
                                }
                                .into());
                            }
                        } else {
                            self.aliases.insert(alias, target.clone());
                        }
                    }
                    child_ports.entry(formal).or_insert(target);
                }
                if self
                    .instances_to_descend
                    .as_ref()
                    .is_some_and(|instances| !instances.contains(&instance))
                {
                    continue;
                }
                let mut active_definitions = frame.active_definitions.clone();
                active_definitions.push(canonical_subcircuit);
                frames.push(Frame {
                    elements: &definition.elements,
                    next_index: 0,
                    prefix: instance,
                    parent_ports: child_ports,
                    active_definitions,
                    depth: frame.depth.saturating_add(1),
                });
            }
            Ok(())
        }
    }

    let definitions = netlist
        .subcircuits
        .iter()
        .map(|definition| (definition.name.to_ascii_uppercase(), definition))
        .collect();
    let globals = netlist
        .global_nodes
        .iter()
        .map(|node| canonical_symbol(node))
        .collect::<HashSet<_>>();
    let (instances_to_visit, instances_to_descend) = requested_instance_paths(requested_aliases);
    let mut collector = AliasCollector {
        definitions,
        external_subcircuits: Flattener::collect_external_subckts(netlist),
        globals: &globals,
        ground_policy: netlist.ground_policy(),
        aliases: HashMap::new(),
        requested_aliases,
        instances_to_visit,
        instances_to_descend,
        max_depth,
        max_elements,
        emitted_elements: 0,
        traversal_steps: 0,
    };
    collector.collect(&netlist.elements, abort)?;
    Ok(InterfaceNodeAliases {
        targets: collector.aliases,
    })
}

fn namespace_matches(namespace: &HashSet<String>, pattern: &str) -> bool {
    if pattern == "*" {
        return true;
    }
    if !pattern.contains('*') && !pattern.contains('?') {
        return namespace.contains(pattern);
    }
    namespace
        .iter()
        .any(|candidate| hierarchy_pattern_matches(pattern, candidate))
}

fn namespace_matches_with_aliases(
    namespace: &HashSet<String>,
    aliases: &InterfaceNodeAliases,
    pattern: &str,
) -> bool {
    if pattern == "*" {
        return true;
    }
    if pattern.contains('*') || pattern.contains('?') {
        return namespace
            .iter()
            .any(|candidate| hierarchy_pattern_matches(pattern, candidate))
            || aliases.keys().any(|alias| {
                hierarchy_pattern_matches(pattern, alias)
                    && resolved_alias_exists(namespace, aliases, alias)
            });
    }
    namespace.contains(pattern) || resolved_alias_exists(namespace, aliases, pattern)
}

fn resolved_alias_exists(
    namespace: &HashSet<String>,
    aliases: &InterfaceNodeAliases,
    alias: &str,
) -> bool {
    aliases
        .resolve(alias)
        .is_some_and(|target| namespace.contains(target))
}

fn hierarchy_pattern_matches(pattern: &str, candidate: &str) -> bool {
    let pattern = pattern.chars().collect::<Vec<_>>();
    let candidate = candidate.chars().collect::<Vec<_>>();
    let mut reachable = vec![false; candidate.len() + 1];
    reachable[0] = true;
    for token in pattern {
        let mut next = vec![false; candidate.len() + 1];
        if token == '*' {
            let mut active = false;
            for index in 0..=candidate.len() {
                active |= reachable[index];
                next[index] = active;
            }
        } else {
            for index in 0..candidate.len() {
                if reachable[index] && (token == '?' || token == candidate[index]) {
                    next[index + 1] = true;
                }
            }
        }
        reachable = next;
    }
    reachable[candidate.len()]
}

/// Extract recognized circuit accessors without interpreting arithmetic.
/// Unknown operators are deliberately excluded: operator support and symbol
/// existence are independent diagnostics.
pub(crate) fn extract_output_dependencies(source: &str) -> Vec<OutputSymbolDependency> {
    extract_output_dependencies_with_context(source, false)
}

/// Extract top-level braced and quoted output-expression bodies in authored
/// order. Quoted literals nested within a braced expression stay inside that
/// expression and are not emitted a second time.
fn extract_output_expressions(source: &str) -> Vec<String> {
    let bytes = source.as_bytes();
    let mut expressions = Vec::new();
    let mut index = 0usize;
    while index < bytes.len() {
        let delimiter = bytes[index];
        if delimiter == b'{' {
            let start = index + 1;
            let mut depth = 1usize;
            let mut quote = None;
            index += 1;
            while index < bytes.len() {
                let byte = bytes[index];
                if let Some(active) = quote {
                    if byte == active && !is_backslash_escaped(bytes, index) {
                        quote = None;
                    }
                } else {
                    match byte {
                        b'\'' | b'"' => quote = Some(byte),
                        b'{' => depth += 1,
                        b'}' => {
                            depth -= 1;
                            if depth == 0 {
                                expressions.push(source[start..index].trim().to_string());
                                index += 1;
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                index += 1;
            }
            if depth != 0 {
                expressions.push(source[start..].trim().to_string());
            }
            continue;
        }
        if matches!(delimiter, b'\'' | b'"') {
            let start = index + 1;
            index += 1;
            while index < bytes.len()
                && (bytes[index] != delimiter || is_backslash_escaped(bytes, index))
            {
                index += 1;
            }
            expressions.push(source[start..index.min(bytes.len())].trim().to_string());
            index = index.saturating_add(1);
            continue;
        }
        index += 1;
    }
    expressions
}

/// Replace Xyce output-only terminal operands with neutral scalar leaves so
/// the shared strict arithmetic/function parser can validate the surrounding
/// expression. Circuit existence remains owned by the typed dependency pass.
///
/// Xyce's expression lexer recognizes a deliberately narrower lead-current
/// set than direct `.PRINT` syntax. In particular `IV(device)` is not a lead
/// accessor and must remain visible to strict validation as an unknown
/// function.
enum OutputOperandProtectionError {
    Aborted,
    Invalid(OutputExpressionIssue),
}

impl From<OutputExpressionIssue> for OutputOperandProtectionError {
    fn from(issue: OutputExpressionIssue) -> Self {
        Self::Invalid(issue)
    }
}

fn protect_xyce_output_operands(
    source: &str,
    analysis: Option<OutputAnalysisKind>,
    flattened_elements: Option<&[Element]>,
    runtime_scalar_identifiers: &HashSet<String>,
    abort: &dyn AbortSignal,
) -> Result<String, OutputOperandProtectionError> {
    let bytes = source.as_bytes();
    let mut protected = String::with_capacity(source.len());
    let mut copied_through = 0usize;
    let mut index = 0usize;
    while index < bytes.len() {
        if index % 64 == 0 && abort.is_aborted() {
            return Err(OutputOperandProtectionError::Aborted);
        }
        if matches!(bytes[index], b'\'' | b'"') {
            let delimiter = bytes[index];
            index += 1;
            while index < bytes.len()
                && (bytes[index] != delimiter || is_backslash_escaped(bytes, index))
            {
                if index % 64 == 0 && abort.is_aborted() {
                    return Err(OutputOperandProtectionError::Aborted);
                }
                index += 1;
            }
            index = index.saturating_add(1);
            continue;
        }
        if !(bytes[index] as char).is_ascii_alphabetic() {
            index += 1;
            continue;
        }
        let start = index;
        index += 1;
        while index < bytes.len()
            && ((bytes[index] as char).is_ascii_alphanumeric() || bytes[index] == b'_')
        {
            index += 1;
        }
        let operator = source[start..index].to_ascii_uppercase();
        let mut open = index;
        while open < bytes.len() && (bytes[open] as char).is_ascii_whitespace() {
            open += 1;
        }
        if open < bytes.len() && bytes[open] == b'(' && xyce_output_operand_operator(&operator) {
            let close = match matching_output_parenthesis_with_abort(bytes, open, abort) {
                Ok(Some(close)) => close,
                Ok(None) => {
                    return Err(OutputExpressionIssue::InvalidAccessor {
                        operator,
                        detail: "missing closing ')'".to_string(),
                    }
                    .into());
                }
                Err(ParseWithAbortError::Aborted) => {
                    return Err(OutputOperandProtectionError::Aborted);
                }
                Err(ParseWithAbortError::Parse(_)) => unreachable!("parenthesis scan cannot parse"),
            };
            validate_xyce_output_accessor(&operator, &source[open + 1..close], analysis)?;
            protected.push_str(&source[copied_through..start]);
            protected.push('0');
            copied_through = close + 1;
            index = copied_through;
            continue;
        }
        if matches!(operator.as_str(), "ONOISE" | "INOISE")
            && analysis == Some(OutputAnalysisKind::Noise)
        {
            protected.push_str(&source[copied_through..start]);
            protected.push('0');
            copied_through = index;
            continue;
        }
        if index < bytes.len() && bytes[index] == b':' {
            let mut end = index + 1;
            while end < bytes.len() && is_output_parameter_char(bytes[end] as char) {
                end += 1;
            }
            let token = &source[start..end];
            if token.split(':').all(|segment| !segment.is_empty()) {
                if !runtime_scalar_identifiers.contains(&canonical_symbol(token)) {
                    validate_output_device_parameter(token, flattened_elements)?;
                }
                protected.push_str(&source[copied_through..start]);
                protected.push('0');
                copied_through = end;
                index = end;
            }
        }
    }
    protected.push_str(&source[copied_through..]);
    if abort.is_aborted() {
        return Err(OutputOperandProtectionError::Aborted);
    }
    Ok(protected)
}

fn element_supports_bare_output_scalar(kind: &ElementKind) -> bool {
    matches!(
        kind,
        ElementKind::VoltageSource(_)
            | ElementKind::CurrentSource(_)
            | ElementKind::Resistor { .. }
            | ElementKind::Capacitor { .. }
            | ElementKind::Inductor { .. }
            | ElementKind::JilesAthertonInductor { .. }
    )
}

fn element_is_independent_source(kind: &ElementKind) -> bool {
    matches!(
        kind,
        ElementKind::VoltageSource(_) | ElementKind::CurrentSource(_)
    )
}

fn validate_xyce_output_accessor(
    operator: &str,
    authored_arguments: &str,
    analysis: Option<OutputAnalysisKind>,
) -> Result<(), OutputExpressionIssue> {
    if matches!(operator, "DNO" | "DNI") && analysis != Some(OutputAnalysisKind::Noise) {
        return Err(OutputExpressionIssue::InvalidAccessor {
            operator: operator.to_string(),
            detail: "noise-contribution accessors are valid only for NOISE output requests"
                .to_string(),
        });
    }
    let arguments = split_top_level_args(authored_arguments);
    let (minimum, maximum) = match operator {
        "V" | "VR" | "VI" | "VM" | "VP" | "VDB" => (1, 2),
        "DNO" | "DNI" => (1, 2),
        "S" | "SR" | "SI" | "SM" | "SP" | "SDB" | "Y" | "YR" | "YI" | "YM" | "YP" | "YDB" | "Z"
        | "ZR" | "ZI" | "ZM" | "ZP" | "ZDB" => (2, 2),
        _ => (1, 1),
    };
    if !(minimum..=maximum).contains(&arguments.len())
        || arguments.iter().any(|argument| argument.trim().is_empty())
    {
        return Err(OutputExpressionIssue::InvalidAccessor {
            operator: operator.to_string(),
            detail: format!(
                "expected {} operand{} but got {}",
                if minimum == maximum {
                    minimum.to_string()
                } else {
                    format!("{minimum}..={maximum}")
                },
                if maximum == 1 { "" } else { "s" },
                arguments.len()
            ),
        });
    }
    let network_parameter = matches!(
        operator,
        "S" | "SR"
            | "SI"
            | "SM"
            | "SP"
            | "SDB"
            | "Y"
            | "YR"
            | "YI"
            | "YM"
            | "YP"
            | "YDB"
            | "Z"
            | "ZR"
            | "ZI"
            | "ZM"
            | "ZP"
            | "ZDB"
    );
    for argument in &arguments {
        let argument = argument.trim();
        let valid = if network_parameter {
            argument.parse::<usize>().is_ok_and(|index| index != 0)
        } else {
            valid_atomic_output_operand(argument)
        };
        if !valid {
            return Err(OutputExpressionIssue::InvalidAccessor {
                operator: operator.to_string(),
                detail: format!("invalid operand '{argument}'"),
            });
        }
    }
    // N(...) is intentionally node-first. The same colon spelling can name a
    // hierarchy-interface node, a model-owned internal vector, or a device
    // parameter; the existing typed symbol pass resolves that ambiguity
    // against the complete node/device namespace after this grammar check.
    Ok(())
}

fn valid_atomic_output_operand(argument: &str) -> bool {
    !argument.is_empty()
        && !argument
            .chars()
            .any(|character| matches!(character, '(' | ')' | ',' | '{' | '}' | '\'' | '"'))
}

fn validate_output_device_parameter(
    authored: &str,
    flattened_elements: Option<&[Element]>,
) -> Result<(), OutputExpressionIssue> {
    let Some((device, parameter)) = authored.rsplit_once(':') else {
        return Ok(());
    };
    let device = device.trim();
    let parameter = parameter.trim();
    if device.is_empty() || parameter.is_empty() {
        return Err(OutputExpressionIssue::UnresolvedDeviceParameter {
            device: device.to_ascii_uppercase(),
            parameter: parameter.to_ascii_uppercase(),
        });
    }
    let Some(elements) = flattened_elements else {
        return Ok(());
    };
    let Some(element) = elements
        .iter()
        .find(|element| canonical_symbol(&element.name) == canonical_symbol(device))
    else {
        return Err(OutputExpressionIssue::UnresolvedDeviceParameter {
            device: device.to_ascii_uppercase(),
            parameter: parameter.to_ascii_uppercase(),
        });
    };
    if known_output_parameter_for_element(&element.kind, parameter) == Some(false) {
        return Err(OutputExpressionIssue::UnresolvedDeviceParameter {
            device: device.to_ascii_uppercase(),
            parameter: parameter.to_ascii_uppercase(),
        });
    }
    Ok(())
}

/// Return `Some` only for device families whose scalar output metadata is
/// intrinsic to the authored element. Compact/generated-model namespaces are
/// model-owned and remain deferred to their canonical runtime metadata.
fn known_output_parameter_for_element(kind: &ElementKind, parameter: &str) -> Option<bool> {
    let parameter = parameter.trim().to_ascii_uppercase();
    let matches = |names: &[&str]| names.iter().any(|name| *name == parameter);
    match kind {
        ElementKind::Resistor {
            instance_params, ..
        } => Some(
            matches(&[
                "R",
                "RESISTANCE",
                "VALUE",
                "L",
                "LENGTH",
                "W",
                "WIDTH",
                "A",
                "AREA",
                "M",
                "MULT",
                "NRS",
                "NRSQ",
                "NSQ",
                "SQUARES",
                "TC",
                "TC1",
                "TC2",
                "TEMP",
                "DTEMP",
            ]) || instance_params
                .iter()
                .any(|(name, _)| name.eq_ignore_ascii_case(&parameter)),
        ),
        ElementKind::Capacitor {
            instance_params, ..
        } => Some(
            matches(&[
                "C",
                "CAP",
                "CAPACITANCE",
                "VALUE",
                "IC",
                "L",
                "LENGTH",
                "W",
                "WIDTH",
                "M",
                "MULT",
                "SCALE",
                "TC",
                "TC1",
                "TC2",
                "TEMP",
                "DTEMP",
            ]) || instance_params
                .iter()
                .any(|(name, _)| name.eq_ignore_ascii_case(&parameter)),
        ),
        ElementKind::Inductor {
            instance_params, ..
        } => Some(
            matches(&[
                "L",
                "IND",
                "INDUCTANCE",
                "VALUE",
                "IC",
                "M",
                "MULT",
                "SCALE",
                "TC",
                "TC1",
                "TC2",
                "TEMP",
                "DTEMP",
            ]) || instance_params
                .iter()
                .any(|(name, _)| name.eq_ignore_ascii_case(&parameter)),
        ),
        ElementKind::JilesAthertonInductor { .. } => {
            Some(matches(&["L", "INDUCTANCE", "VALUE", "M", "H", "B"]))
        }
        _ => None,
    }
}

/// Serialize the validated permissive AST into the strict arithmetic grammar,
/// neutralizing only typed runtime scalar leaves and output-only functions.
/// Working from the AST avoids mistaking numeric exponents or engineering
/// suffixes for identifiers with the same spelling.
fn strict_output_validation_expression(
    expression: &super::expr::Expr,
    runtime_scalar_identifiers: &HashSet<String>,
    xyce_dialect: bool,
    abort: &dyn AbortSignal,
) -> Result<String, ParseWithAbortError> {
    use super::expr::{BinOpKind, Expr, UnaryOpKind};
    enum Task<'a> {
        Expression(&'a Expr),
        Static(&'static str),
        Owned(String),
    }
    let mut output = String::new();
    let mut tasks = vec![Task::Expression(expression)];
    let mut visited = 0usize;
    while let Some(task) = tasks.pop() {
        poll_parse_abort(abort, visited)?;
        visited = visited.saturating_add(1);
        match task {
            Task::Static(text) => output.push_str(text),
            Task::Owned(text) => {
                for (index, character) in text.chars().enumerate() {
                    poll_parse_abort(abort, index)?;
                    output.push(character);
                }
            }
            Task::Expression(Expr::Number(value)) => output.push_str(&value.to_string()),
            Task::Expression(Expr::ComplexNumber(value)) => {
                output.push_str(&value.re.to_string());
            }
            Task::Expression(Expr::StringLiteral(value)) => {
                output.push('"');
                for (index, character) in value.chars().enumerate() {
                    poll_parse_abort(abort, index)?;
                    match character {
                        '\\' => output.push_str("\\\\"),
                        '"' => output.push_str("\\\""),
                        _ => output.push(character),
                    }
                }
                output.push('"');
            }
            Task::Expression(Expr::Param(name)) => {
                if runtime_scalar_identifiers.contains(&canonical_symbol(name)) {
                    output.push('0');
                } else {
                    output.push_str(name);
                }
            }
            Task::Expression(Expr::UnaryOp { op, operand }) => {
                tasks.push(Task::Static(")"));
                tasks.push(Task::Expression(operand));
                tasks.push(Task::Static(match op {
                    UnaryOpKind::Neg => "(-",
                    UnaryOpKind::Pos => "(+",
                    UnaryOpKind::Not => "(!",
                }));
            }
            Task::Expression(Expr::BinOp { op, left, right }) => {
                let operator = match op {
                    BinOpKind::Add => "+",
                    BinOpKind::Sub => "-",
                    BinOpKind::Mul => "*",
                    BinOpKind::Div => "/",
                    BinOpKind::Mod => "%",
                    BinOpKind::Pow => "^",
                    BinOpKind::Gt => ">",
                    BinOpKind::Lt => "<",
                    BinOpKind::Ge => ">=",
                    BinOpKind::Le => "<=",
                    BinOpKind::Eq => "==",
                    BinOpKind::Ne => "!=",
                    BinOpKind::And => "&&",
                    BinOpKind::Or => "||",
                };
                tasks.push(Task::Static(")"));
                tasks.push(Task::Expression(right));
                tasks.push(Task::Static(operator));
                tasks.push(Task::Expression(left));
                tasks.push(Task::Static("("));
            }
            Task::Expression(Expr::FnCall { name, args }) => {
                if output_only_function_arity(name, xyce_dialect).is_some() {
                    output.push('0');
                    continue;
                }
                tasks.push(Task::Static(")"));
                for index in (0..args.len()).rev() {
                    tasks.push(Task::Expression(&args[index]));
                    if index != 0 {
                        tasks.push(Task::Static(","));
                    }
                }
                tasks.push(Task::Static("("));
                tasks.push(Task::Owned(name.to_ascii_uppercase()));
            }
        }
    }
    ensure_parse_not_aborted(abort)?;
    Ok(output)
}

fn xyce_output_operand_operator(operator: &str) -> bool {
    is_internal_node_accessor(operator)
        || matches!(
            operator,
            "V" | "VR"
                | "VI"
                | "VM"
                | "VP"
                | "VDB"
                | "I"
                | "IR"
                | "II"
                | "IM"
                | "IP"
                | "IDB"
                | "IS"
                | "ID"
                | "IG"
                | "IB"
                | "IE"
                | "IC"
                | "I1"
                | "I2"
                | "I3"
                | "I4"
                | "I5"
                | "I6"
                | "I7"
                | "I8"
                | "I9"
                | "S"
                | "SR"
                | "SI"
                | "SM"
                | "SP"
                | "SDB"
                | "Y"
                | "YR"
                | "YI"
                | "YM"
                | "YP"
                | "YDB"
                | "Z"
                | "ZR"
                | "ZI"
                | "ZM"
                | "ZP"
                | "ZDB"
                | "DNO"
                | "DNI"
                | "P"
                | "W"
        )
}

fn is_internal_node_accessor(operator: &str) -> bool {
    matches!(operator, "N" | "NR" | "NI" | "NM" | "NP" | "NDB")
}

fn matching_output_parenthesis_with_abort(
    bytes: &[u8],
    open: usize,
    abort: &dyn AbortSignal,
) -> Result<Option<usize>, ParseWithAbortError> {
    let mut depth = 0usize;
    let mut quote = None;
    for (index, byte) in bytes.iter().copied().enumerate().skip(open) {
        poll_parse_abort(abort, index)?;
        if let Some(active) = quote {
            if byte == active && !is_backslash_escaped(bytes, index) {
                quote = None;
            }
            continue;
        }
        match byte {
            b'\'' | b'"' => quote = Some(byte),
            b'(' => depth += 1,
            b')' => {
                let Some(next_depth) = depth.checked_sub(1) else {
                    return Ok(None);
                };
                depth = next_depth;
                if depth == 0 {
                    return Ok(Some(index));
                }
            }
            _ => {}
        }
    }
    ensure_parse_not_aborted(abort)?;
    Ok(None)
}

fn is_output_parameter_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '_' | '$' | '.' | ':' | '!' | '#' | '[' | ']')
}

fn is_backslash_escaped(bytes: &[u8], index: usize) -> bool {
    let mut preceding = 0usize;
    let mut cursor = index;
    while cursor != 0 && bytes[cursor - 1] == b'\\' {
        preceding += 1;
        cursor -= 1;
    }
    preceding % 2 != 0
}

fn extract_output_dependencies_with_context(
    source: &str,
    inherited_expression_context: bool,
) -> Vec<OutputSymbolDependency> {
    let bytes = source.as_bytes();
    // Determine expression membership once for the whole source. The previous
    // implementation rescanned `source[..operator_start]` for every accessor,
    // making a long `.PRINT` list quadratic. Real Xyce certification decks can
    // request tens of thousands of node voltages and device currents on one
    // line, so that repeated prefix scan looked like an indefinite hang.
    let expression_context = output_expression_context_by_byte(source);
    let mut dependencies = Vec::new();
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] == b'@' {
            let start = index + 1;
            let mut end = start;
            while end < bytes.len() && is_symbol_char(bytes[end] as char) {
                end += 1;
            }
            // Device-parameter probes belong to parameter metadata
            // validation, not circuit symbol-existence validation.
            index = end.max(index + 1);
            continue;
        }
        if !(bytes[index] as char).is_ascii_alphabetic() {
            index += 1;
            continue;
        }
        let operator_start = index;
        index += 1;
        while index < bytes.len()
            && ((bytes[index] as char).is_ascii_alphanumeric() || bytes[index] == b'_')
        {
            index += 1;
        }
        let operator = source[operator_start..index].to_ascii_uppercase();
        let mut open = index;
        while open < bytes.len() && (bytes[open] as char).is_ascii_whitespace() {
            open += 1;
        }
        if open >= bytes.len() || bytes[open] != b'(' {
            continue;
        }
        let Some(close) = matching_parenthesis(bytes, open) else {
            continue;
        };
        let args = split_top_level_args(&source[open + 1..close]);
        let first_new_dependency = dependencies.len();
        let expression = inherited_expression_context || expression_context[operator_start];
        // `IF(device)` is a valid direct Xyce lead-current request, while
        // `IF(condition, then, else)` inside an authored expression is the
        // expression builtin and must be traversed recursively.
        let current_accessor =
            is_current_output_accessor(&operator) && !(expression && operator.as_str() == "IF");
        match operator.as_str() {
            "V" | "VR" | "VI" | "VM" | "VP" | "VDB" => {
                for arg in args.into_iter().take(2) {
                    push_dependency(&mut dependencies, &operator, arg, OutputSymbolKind::Node);
                }
            }
            _ if is_internal_node_accessor(&operator) => {
                if let Some(arg) = args.first() {
                    push_dependency(&mut dependencies, &operator, arg, OutputSymbolKind::Node);
                }
            }
            "DNO" | "DNI" | "P" | "W" => {
                if let Some(arg) = args.first() {
                    push_dependency(&mut dependencies, &operator, arg, OutputSymbolKind::Device);
                }
            }
            _ if current_accessor => {
                if let Some(arg) = args.first() {
                    push_dependency(&mut dependencies, &operator, arg, OutputSymbolKind::Device);
                }
            }
            _ => {}
        }
        for dependency in &mut dependencies[first_new_dependency..] {
            dependency.expression = expression;
        }
        // Continue inside the argument list as well, so expressions nested in
        // ordinary arithmetic functions retain their circuit dependencies.
        if !is_internal_node_accessor(&operator)
            && !matches!(
                operator.as_str(),
                "V" | "VR" | "VI" | "VM" | "VP" | "VDB" | "DNO" | "DNI" | "P" | "W"
            )
            && !current_accessor
        {
            dependencies.extend(extract_output_dependencies_with_context(
                &source[open + 1..close],
                expression,
            ));
        }
        index = close + 1;
    }
    dependencies
}

fn push_dependency(
    dependencies: &mut Vec<OutputSymbolDependency>,
    operator: &str,
    symbol: &str,
    kind: OutputSymbolKind,
) {
    // Whitespace around hierarchy separators and wildcards is lexical, not
    // part of a circuit symbol. Xyce's legacy `I(YDEVICE BRANCH)` spelling is
    // the sole exception: its two tokens form one logical branch-current name.
    let authored = symbol.trim();
    let mut parts = authored.split_whitespace();
    let first = parts.next();
    let second = parts.next();
    let preserve_legacy_branch = operator == "I"
        && first.is_some()
        && second.is_some_and(|part| part.eq_ignore_ascii_case("BRANCH"))
        && parts.next().is_none();
    let symbol = if preserve_legacy_branch {
        format!(
            "{} {}",
            first.expect("checked above"),
            second.expect("checked above")
        )
    } else {
        authored.split_whitespace().collect::<String>()
    };
    if !symbol.is_empty() {
        dependencies.push(OutputSymbolDependency {
            operator: operator.to_string(),
            symbol,
            kind,
            expression: false,
        });
    }
}

fn output_expression_context_by_byte(source: &str) -> Vec<bool> {
    let mut context = vec![false; source.len() + 1];
    let mut braces = 0usize;
    let mut single_quote = false;
    let mut double_quote = false;
    for (index, byte) in source.bytes().enumerate() {
        context[index] = braces != 0 || single_quote || double_quote;
        match byte {
            b'{' if !single_quote && !double_quote => braces += 1,
            b'}' if !single_quote && !double_quote => braces = braces.saturating_sub(1),
            b'\'' if !double_quote => single_quote = !single_quote,
            b'"' if !single_quote => double_quote = !double_quote,
            _ => {}
        }
    }
    context[source.len()] = braces != 0 || single_quote || double_quote;
    context
}

fn matching_parenthesis(bytes: &[u8], open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (index, byte) in bytes.iter().enumerate().skip(open) {
        match byte {
            b'(' => depth += 1,
            b')' => {
                depth = depth.checked_sub(1)?;
                if depth == 0 {
                    return Some(index);
                }
            }
            _ => {}
        }
    }
    None
}

fn split_top_level_args(source: &str) -> Vec<&str> {
    let bytes = source.as_bytes();
    let mut depth = 0usize;
    let mut start = 0usize;
    let mut args = Vec::new();
    for (index, byte) in bytes.iter().enumerate() {
        match byte {
            b'(' => depth += 1,
            b')' => depth = depth.saturating_sub(1),
            b',' if depth == 0 => {
                args.push(source[start..index].trim());
                start = index + 1;
            }
            _ => {}
        }
    }
    args.push(source[start..].trim());
    args
}

fn is_symbol_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || matches!(ch, '_' | '$' | '.' | ':' | '!' | '*' | '?')
}

#[derive(Clone, Copy)]
enum MeasureDependencySource<'a> {
    Direct(&'a str),
    Expression(&'a str),
}

fn collect_measure_sources<'a>(
    statement: &'a MeasureStatement,
    output: &mut Vec<MeasureDependencySource<'a>>,
) {
    use crate::netlist::measure::{MeasureOperand, MeasureType, TriggerEvent};

    fn condition<'a>(
        condition: &'a crate::netlist::measure::WhenCondition,
        output: &mut Vec<MeasureDependencySource<'a>>,
    ) {
        output.push(MeasureDependencySource::Direct(condition.left.as_str()));
        if let MeasureOperand::Waveform(source) = &condition.right {
            output.push(MeasureDependencySource::Direct(source.as_str()));
        }
    }
    fn trigger<'a>(
        trigger: &'a crate::netlist::measure::TrigSpec,
        output: &mut Vec<MeasureDependencySource<'a>>,
    ) {
        if let TriggerEvent::When(when) = &trigger.event {
            condition(when, output);
        }
    }
    match &statement.measure_type {
        MeasureType::Delay { trig, targ, .. } => {
            trigger(trig, output);
            trigger(targ, output);
        }
        MeasureType::Find { signal, when, .. } | MeasureType::Derivative { signal, when, .. } => {
            output.push(MeasureDependencySource::Direct(signal));
            if let Some(when) = when {
                condition(when, output);
            }
        }
        MeasureType::When {
            condition: when, ..
        } => condition(when, output),
        MeasureType::Param { expression } | MeasureType::Equation { expression, .. } => output
            .push(match expression.kind {
                crate::netlist::measure::MeasureExpressionKind::Expression => {
                    MeasureDependencySource::Expression(&expression.text)
                }
                crate::netlist::measure::MeasureExpressionKind::RawReference
                | crate::netlist::measure::MeasureExpressionKind::RawOutputOperator => {
                    MeasureDependencySource::Direct(&expression.text)
                }
            }),
        MeasureType::ErrorFunction {
            measured,
            comparison,
            ..
        } => {
            output.push(MeasureDependencySource::Direct(measured));
            output.push(MeasureDependencySource::Direct(comparison));
        }
        MeasureType::FileError { signal, .. }
        | MeasureType::Min { signal, .. }
        | MeasureType::Max { signal, .. }
        | MeasureType::PeakToPeak { signal, .. }
        | MeasureType::Avg { signal, .. }
        | MeasureType::Rms { signal, .. }
        | MeasureType::RiseTime { signal, .. }
        | MeasureType::FallTime { signal, .. }
        | MeasureType::Integ { signal, .. } => output.push(MeasureDependencySource::Direct(signal)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn compact_expression(expression: &str) -> String {
        expression.split_whitespace().collect()
    }

    #[test]
    fn output_expression_extraction_preserves_authored_order_and_nested_quotes() {
        assert_eq!(
            extract_output_expressions(
                "TRAN { FABS(V(1)) } '{V(2)+1}' \"V(3)\" {TABLE(\"wave.dat\",TIME)}"
            ),
            vec!["FABS(V(1))", "{V(2)+1}", "V(3)", "TABLE(\"wave.dat\",TIME)",]
        );
    }

    #[test]
    fn direct_output_expressions_reject_unknown_functions_and_identifiers() {
        let options = super::super::NetlistParseOptions {
            expression_dialect: super::super::ExpressionDialect::Xyce,
            ..super::super::NetlistParseOptions::default()
        };
        for (expression, expected) in [("FABS(V(1))", "FABS"), ("IV(RB)", "IV"), ("BAR", "BAR")] {
            let source = format!(
                "typed output expression\nVA 0 1 5\nRB 1 2 100\nRC 2 0 100\n.TRAN 0 1\n.PRINT TRAN {{{expression}}}\n.END\n"
            );
            let error = Netlist::parse_validated_with_options(&source, options)
                .expect_err("invalid output expression must fail before execution");
            let ParseError::OutputExpressionValidation(error) = error else {
                panic!("wrong typed output-expression error: {error:?}");
            };
            assert_eq!(error.directive, OutputDirectiveKind::Print);
            assert_eq!(error.origin.line, 6);
            assert_eq!(compact_expression(&error.expression), expression);
            match (&error.issue, expression) {
                (OutputExpressionIssue::UnresolvedIdentifier { identifier }, "BAR") => {
                    assert_eq!(identifier, expected)
                }
                (OutputExpressionIssue::UnknownFunction { function }, _) => {
                    assert_eq!(function, expected);
                }
                (issue, _) => panic!("wrong issue for {expression}: {issue:?}"),
            }
        }

        let valid = "valid output expression\nVA 0 1 5\nRB 1 2 100\nRC 2 0 100\n.TRAN 0 1\n.PRINT TRAN {ABS(V(1))}\n.END\n";
        Netlist::parse_validated_with_options(valid, options)
            .expect("a supported function and defined probe remain valid");
    }

    #[test]
    fn xyce_nested_grouping_ddx_and_runtime_scalar_names_validate() {
        let options = super::super::NetlistParseOptions {
            expression_dialect: super::super::ExpressionDialect::Xyce,
            ..super::super::NetlistParseOptions::default()
        };
        let source = "valid Xyce runtime output names
VC 1 0 5
R1 1 0 100
.SUBCKT CELL A
R2 A 0 200
.ENDS
X1 1 CELL
.TRAN 0 1
.PRINT TRAN {V(1)-{(V(1)-V(0))}} {DDX(V(1)*V(1),V(1))}
.PRINT TRAN {VC+VV1-VV2}
.MEASURE TRAN VV1 PARAM='V(1)+1'
.MEASURE TRAN VV2 EQN {V(1)+2}
.END
";
        Netlist::parse_validated_with_options(source, options)
            .expect("nested grouping, DDX, and live measures validate");

        let atomic_devices = "atomic Xyce device outputs
VC 1 0 5
R1 1 0 100
.SUBCKT CELL A
R2 A 0 200
.ENDS
X1 1 CELL
.DC VC 5 5 1
.PRINT DC {VC} {R1} {X1:R2}
.END
";
        Netlist::parse_validated_with_options(atomic_devices, options)
            .expect("atomic source, passive, and flattened device values validate");
        assert!(matches!(
            Netlist::parse_validated_with_options(
                &atomic_devices.replace("{R1}", "{R1+1}"),
                options,
            ),
            Err(ParseError::OutputExpressionValidation(error))
                if matches!(error.issue, OutputExpressionIssue::UnresolvedIdentifier { ref identifier } if identifier == "R1")
        ));

        for invalid_measure in [
            source.replace(
                ".MEASURE TRAN VV1 PARAM='V(1)+1'",
                ".MEASURE AC VV1 PARAM='V(1)+1'",
            ),
            source.replace(
                ".MEASURE TRAN VV1 PARAM='V(1)+1'",
                ".MEASURE TRAN VV1 AVG V(1)",
            ),
        ] {
            assert!(
                matches!(
                    Netlist::parse_validated_with_options(&invalid_measure, options),
                    Err(ParseError::OutputExpressionValidation(error))
                        if matches!(error.issue, OutputExpressionIssue::UnresolvedIdentifier { ref identifier } if identifier == "VV1")
                ),
                "only same-analysis live equation measurements may be referenced by output expressions"
            );
        }

        let error = Netlist::parse_validated_with_options(
            &source.replace("DDX(V(1)*V(1),V(1))", "DDX(V(1))"),
            options,
        )
        .expect_err("DDX retains its exact two-argument contract");
        assert!(matches!(
            error,
            ParseError::OutputExpressionValidation(error)
                if matches!(error.issue, OutputExpressionIssue::Syntax { ref detail } if detail.contains("DDX") && detail.contains("2"))
        ));

        for invalid in [
            "DDX(V(1),1)",
            "DDX(V(1),VR(1))",
            "DDX(V(1),V(1,0))",
            "DDX(V(1),V(2))",
            "DDX(V(1),I(VC))",
            "'DDX(V(1),V(2))'",
        ] {
            let invalid_source = source.replace("DDX(V(1)*V(1),V(1))", invalid);
            assert!(
                matches!(
                    Netlist::parse_validated_with_options(&invalid_source, options),
                    Err(ParseError::OutputExpressionValidation(error))
                        if matches!(error.issue, OutputExpressionIssue::Syntax { .. })
                ),
                "invalid derivative contract {invalid} was accepted"
            );
        }

        let numeric_collision = "runtime identifier numeric boundaries
V1 1 0 1
.TRAN 0 1
.PRINT TRAN {1e3+1e-3+1meg+E3}
.MEASURE TRAN E3 PARAM='V(1)'
.END
";
        Netlist::parse_validated_with_options(numeric_collision, options)
            .expect("runtime names do not rewrite exponent or engineering suffix tokens");

        let function_derivative = "DDX function formal
V1 1 0 1
.FUNC DERIV(X) {DDX(X*X,X)}
.TRAN 0 1
.PRINT TRAN {DERIV(V(1))}
.END
";
        Netlist::parse_validated_with_options(function_derivative, options)
            .expect("DDX accepts a referenced user-function formal target");
        assert!(matches!(
            Netlist::parse_validated_with_options(
                &function_derivative.replace("DDX(X*X,X)", "DDX(X*X,1)"),
                options,
            ),
            Err(ParseError::OutputExpressionValidation(error))
                if matches!(error.issue, OutputExpressionIssue::Syntax { .. })
        ));

        for missing in ["V(MISSING)", "I(MISSING)"] {
            let missing_dependency = format!(
                "function dependency\nV1 1 0 1\n.FUNC OBSERVE(X) {{{missing}+X}}\n.TRAN 0 1\n.PRINT TRAN {{OBSERVE(1)}}\n.END\n"
            );
            let observed = Netlist::parse_validated_with_options(&missing_dependency, options);
            assert!(
                matches!(
                    &observed,
                    Err(ParseError::OutputSymbolValidation(error))
                        if error.unresolved.len() == 1
                            && error.unresolved[0].symbol.eq_ignore_ascii_case("MISSING")
                ),
                "expanded dependency {missing} was not validated exactly once: {observed:?}"
            );
        }

        let ngspice_user_ddx = "ngspice user DDX function
V1 1 0 1
.FUNC DDX(A,B) {A+B}
.TRAN 0 1
.PRINT TRAN {DDX(V(1),2)}
.END
";
        Netlist::parse_validated(ngspice_user_ddx)
            .expect("non-Xyce user functions named DDX retain ordinary function semantics");
    }

    #[test]
    fn xyce_output_only_expression_operands_remain_valid() {
        let options = super::super::NetlistParseOptions {
            expression_dialect: super::super::ExpressionDialect::Xyce,
            ..super::super::NetlistParseOptions::default()
        };
        let source = "valid Xyce output expression operands
VA 0 1 5
RB 1 2 100
RC 2 0 100
.TRAN 1 1
.PRINT TRAN {VR(1)+VI(1)+VM(1)+VP(1)+VDB(1)}
.PRINT TRAN {I(RB)+IR(RB)+II(RB)+IM(RB)+IP(RB)+IDB(RB)}
.PRINT TRAN {IS(RB)+ID(RB)+IG(RB)+IB(RB)+IE(RB)+IC(RB)+I1(RB)}
.PRINT TRAN {N(RB:R)+P(RB)+W(RB)}
.PRINT TRAN {RB:R}
.END
";
        Netlist::parse_validated_with_options(source, options)
            .expect("Xyce output-domain operands validate before execution");

        let noise = "valid Xyce noise expression operands
V1 in 0 AC 1
R1 in out 1k
R2 out 0 1k
.NOISE V(out) V1 DEC 1 1 10
.PRINT NOISE {LOG(ONOISE)+LOG(INOISE)+DNO(R1,thermal)+DNI(R1,thermal)}
.END
";
        Netlist::parse_validated_with_options(noise, options)
            .expect("noise-owned expression operands validate in the NOISE domain");

        let wrong_domain = source.replace(
            ".PRINT TRAN {RB:R}",
            ".PRINT TRAN {LOG(ONOISE)+LOG(INOISE)}",
        );
        let error = Netlist::parse_validated_with_options(&wrong_domain, options)
            .expect_err("noise vectors remain analysis-owned");
        assert!(matches!(
            error,
            ParseError::OutputExpressionValidation(error)
                if matches!(
                    error.issue,
                    OutputExpressionIssue::UnresolvedIdentifier { ref identifier }
                        if identifier == "ONOISE"
                )
        ));
    }

    #[test]
    fn xyce_output_projection_and_statistical_functions_remain_valid() {
        let options = super::super::NetlistParseOptions {
            expression_dialect: super::super::ExpressionDialect::Xyce,
            ..super::super::NetlistParseOptions::default()
        };
        let source = "valid Xyce output functions
V1 1 0 1
.TRAN 1 1
.PRINT TRAN {R(V(1))+RE(V(1))+REAL(V(1))+IMG(V(1))+IMAG(V(1))+PH(V(1))+PHASE(V(1))+DB(V(1))}
.PRINT TRAN {RAND()+RANDOM()+UNIF(1,.1)+AUNIF(1,.1)+GAUSS(1,.1)+AGAUSS(1,.1,3)}
.END
";
        Netlist::parse_validated_with_options(source, options)
            .expect("Xyce output projections and statistical functions are valid");

        for expression in ["R()", "UNIF(1)", "GAUSS(1)", "RAND(1)"] {
            let invalid = source.replace(
                "{RAND()+RANDOM()+UNIF(1,.1)+AUNIF(1,.1)+GAUSS(1,.1)+AGAUSS(1,.1,3)}",
                &format!("{{{expression}}}"),
            );
            assert!(matches!(
                Netlist::parse_validated_with_options(&invalid, options),
                Err(ParseError::OutputExpressionValidation(error))
                    if matches!(error.issue, OutputExpressionIssue::Syntax { .. })
            ));
        }
    }

    #[test]
    fn xyce_output_accessors_enforce_arity_and_atomic_operands() {
        let options = super::super::NetlistParseOptions {
            expression_dialect: super::super::ExpressionDialect::Xyce,
            ..super::super::NetlistParseOptions::default()
        };
        for expression in ["V()", "V(1,2,3)", "N()", "ID()", "S(1)", "S(0,1)"] {
            let source = format!(
                "invalid accessor\nV1 1 0 1\nR1 1 0 1k\n.TRAN 1 1\n.PRINT TRAN {{{expression}}}\n.END\n"
            );
            let error = Netlist::parse_validated_with_options(&source, options)
                .expect_err("invalid accessor must fail semantic validation");
            assert!(matches!(
                error,
                ParseError::OutputExpressionValidation(error)
                    if matches!(error.issue, OutputExpressionIssue::InvalidAccessor { .. })
            ));
        }
    }

    #[test]
    fn xyce_internal_node_projections_all_validate_their_dependencies() {
        let options = super::super::NetlistParseOptions {
            expression_dialect: super::super::ExpressionDialect::Xyce,
            ..super::super::NetlistParseOptions::default()
        };
        for accessor in ["N", "NR", "NI", "NM", "NP", "NDB"] {
            let valid = format!(
                "internal node projection\nV1 1 0 1\n.TRAN 1 1\n.PRINT TRAN {{{accessor}(1)}}\n.END\n"
            );
            Netlist::parse_validated_with_options(&valid, options)
                .unwrap_or_else(|error| panic!("{accessor} rejected an existing node: {error}"));

            let missing = valid.replace(&format!("{accessor}(1)"), &format!("{accessor}(MISSING)"));
            assert!(matches!(
                Netlist::parse_validated_with_options(&missing, options),
                Err(ParseError::OutputSymbolValidation(error))
                    if error.unresolved.len() == 1
                        && error.unresolved[0].operator == accessor
                        && error.unresolved[0].symbol == "MISSING"
            ));
        }
    }

    #[test]
    fn xyce_noise_contribution_accessors_are_noise_domain_only() {
        let options = super::super::NetlistParseOptions {
            expression_dialect: super::super::ExpressionDialect::Xyce,
            ..super::super::NetlistParseOptions::default()
        };
        let noise = "noise contribution domain\nV1 in 0 AC 1\nR1 in out 1k\nR2 out 0 1k\n.NOISE V(out) V1 DEC 1 1 10\n.PRINT NOISE {DNO(R1)+DNI(R1,thermal)}\n.END\n";
        Netlist::parse_validated_with_options(noise, options)
            .expect("DNO and DNI are valid in a NOISE output expression");

        for (analysis, command) in [("TRAN", ".TRAN 1 1"), ("AC", ".AC LIN 1 1 1")] {
            for accessor in ["DNO", "DNI"] {
                let source = format!(
                    "wrong noise domain\nV1 1 0 1 AC 1\nR1 1 0 1k\n{command}\n.PRINT {analysis} {{{accessor}(R1)}}\n.END\n"
                );
                assert!(matches!(
                    Netlist::parse_validated_with_options(&source, options),
                    Err(ParseError::OutputExpressionValidation(error))
                        if matches!(
                            error.issue,
                            OutputExpressionIssue::InvalidAccessor { ref operator, .. }
                                if operator == accessor
                        )
                ));
            }
        }
    }

    #[test]
    fn passive_device_parameters_resolve_exact_flattened_metadata() {
        let options = super::super::NetlistParseOptions {
            expression_dialect: super::super::ExpressionDialect::Xyce,
            ..super::super::NetlistParseOptions::default()
        };
        let valid = "device parameter metadata
V1 1 0 1
RB 1 0 1k
X1 1 CELL
.SUBCKT CELL A
RSUB A 0 2k
.ENDS CELL
.TRAN 1 1
.PRINT TRAN {RB:R+X1:RSUB:R}
.END
";
        Netlist::parse_validated_with_options(valid, options)
            .expect("top-level and hierarchical passive parameters resolve");

        for authored in ["MISSING:R", "RB:BOGUS", "X1:RSUB:BOGUS"] {
            let source = valid.replace("RB:R+X1:RSUB:R", authored);
            let error = Netlist::parse_validated_with_options(&source, options)
                .expect_err("absent device parameters must fail closed");
            assert!(matches!(
                error,
                ParseError::OutputExpressionValidation(error)
                    if matches!(
                        error.issue,
                        OutputExpressionIssue::UnresolvedDeviceParameter { .. }
                    )
            ));
        }
    }

    #[test]
    fn print_string_options_are_not_misclassified_as_expressions() {
        let source = "print option
V1 1 0 1
.TRAN 1 1
.PRINT TRAN FILE=\"not_an_expression.prn\" FORMAT=CSV V(1)
.END
";
        let netlist = Netlist::parse_validated(source)
            .expect("quoted FILE option is metadata, not an output expression");
        let request = netlist
            .output_requests
            .iter()
            .find(|request| request.directive == OutputDirectiveKind::Print)
            .expect("PRINT request retained");
        assert!(request.expressions.is_empty());
    }

    #[test]
    fn print_operands_preserve_authored_column_order_and_duplicates() {
        let source = "ordered print operands\n\
V1 1 0 1\n\
R1 1 0 1k\n\
.DC V1 0 1 1\n\
.PRINT DC FORMAT=STD I(V1) V(1) R1:R {V(1)/I(V1)} V(1)\n\
.END\n";
        let netlist = Netlist::parse_validated(source).expect("ordered PRINT validates");
        let request = netlist
            .output_requests
            .iter()
            .find(|request| request.directive == OutputDirectiveKind::Print)
            .expect("PRINT request retained");
        assert_eq!(
            request.operands,
            ["I(V1)", "V(1)", "R1:R", "{V(1)/I(V1)}", "V(1)"]
        );
        assert!(matches!(
            request.operand_kinds[0],
            OutputOperandKind::Probe(crate::netlist::SaveSignal::Current(_))
        ));
        assert!(matches!(
            request.operand_kinds[3],
            OutputOperandKind::Expression { .. }
        ));
    }

    #[test]
    fn print_operand_parser_owns_one_typed_ordered_grammar() {
        let source = "typed print operand grammar\n\
V1 out 0 1\n\
VREF ref 0 0\n\
R1 out ref 1k\n\
.DC V1 0 1 1\n\
.PRINT DC FILE=\"V(MISSING)\" FORMAT=CSV NOINDEX I ( V1 ) V ( out , ref ) R1:R @R1[r] {V(out) + I(V1)} 'I(V1)*2' \"V(out)/2\" V(out) V(out)\n\
.END\n";
        let netlist = Netlist::parse(source).expect("mixed PRINT card parses");
        let request = netlist
            .output_requests
            .iter()
            .find(|request| request.directive == OutputDirectiveKind::Print)
            .expect("PRINT request retained");
        assert_eq!(request.analysis, Some(OutputAnalysisKind::Dc));
        assert_eq!(
            request.operands,
            [
                "I ( V1 )",
                "V ( out , ref )",
                "R1:R",
                "@R1[r]",
                "{V(out) + I(V1)}",
                "'I(V1)*2'",
                "\"V(out)/2\"",
                "V(out)",
                "V(out)",
            ]
        );
        assert!(matches!(
            request.operand_kinds[0],
            OutputOperandKind::Probe(crate::netlist::SaveSignal::Current(_))
        ));
        assert!(matches!(
            request.operand_kinds[1],
            OutputOperandKind::Probe(crate::netlist::SaveSignal::VoltageDiff(_, _))
        ));
        assert!(matches!(
            request.operand_kinds[2],
            OutputOperandKind::Probe(crate::netlist::SaveSignal::Raw(_))
        ));
        assert!(matches!(
            request.operand_kinds[3],
            OutputOperandKind::Probe(crate::netlist::SaveSignal::DeviceParam { .. })
        ));
        assert!(
            request.operand_kinds[4..7]
                .iter()
                .all(|kind| matches!(kind, OutputOperandKind::Expression { .. }))
        );
        assert_eq!(
            request.expressions,
            ["V(out) + I(V1)", "I(V1)*2", "V(out)/2"]
        );
        assert!(
            !request
                .dependencies
                .iter()
                .any(|dependency| dependency.symbol.eq_ignore_ascii_case("MISSING"))
        );
    }

    #[test]
    fn malformed_print_probe_delimiters_fail_closed() {
        for malformed in [
            "bad PRINT paren\nV1 1 0 1\n.DC V1 0 1 1\n.PRINT DC V(1\n.END\n",
            "bad PRINT bracket\nV1 1 0 1\n.DC V1 0 1 1\n.PRINT DC @V1[current\n.END\n",
        ] {
            let error = Netlist::parse(malformed).expect_err("malformed PRINT must fail");
            assert!(matches!(error, crate::netlist::ParseError::Syntax { .. }));
        }
    }

    #[test]
    fn syntactic_parse_remains_permissive_while_strict_phase_is_typed() {
        let source = "semantic phase\nV1 1 0 1\n.PRINT OP V(MISSING)\n.OP\n.END\n";
        let netlist = Netlist::parse(source).expect("syntactic parse accepts unresolved output");
        assert!(matches!(
            validate_output_symbols(&netlist),
            Err(ParseError::OutputSymbolValidation(_))
        ));
        assert!(matches!(
            Netlist::parse_validated(source),
            Err(ParseError::OutputSymbolValidation(_))
        ));
    }

    #[test]
    fn noise_analysis_owned_vectors_validate_only_in_the_noise_domain() {
        let source = "noise-owned output vectors\n\
                      V1 in 0 AC 1\n\
                      R1 in out 1k\n\
                      R2 out 0 1k\n\
                      .NOISE V(out) V1 DEC 1 1 10\n\
                      .PRINT NOISE V(inoise_spectrum) V(onoise)\n\
                      .SAVE V(inoise) V(onoise_spectrum)\n\
                      .END\n";
        let netlist = Netlist::parse_validated(source)
            .expect("noise-generated vectors are valid noise outputs");
        let print = netlist
            .output_requests
            .iter()
            .find(|request| request.directive == OutputDirectiveKind::Print)
            .expect("typed PRINT provenance exists");
        assert_eq!(print.analysis, Some(OutputAnalysisKind::Noise));

        let wrong_domain = source.replace(
            ".PRINT NOISE V(inoise_spectrum) V(onoise)",
            ".PRINT OP V(inoise_spectrum)",
        );
        let error = Netlist::parse_validated(&wrong_domain)
            .expect_err("noise-generated vectors are not OP topology nodes");
        assert!(matches!(error, ParseError::OutputSymbolValidation(_)));

        let wrong_namespace = source.replace(
            ".PRINT NOISE V(inoise_spectrum) V(onoise)",
            ".PRINT NOISE I(inoise)",
        );
        let error = Netlist::parse_validated(&wrong_namespace)
            .expect_err("noise-generated vectors are not circuit devices");
        assert!(matches!(error, ParseError::OutputSymbolValidation(_)));

        let no_noise_analysis = "missing noise producer\n\
                                 V1 in 0 1\n\
                                 R1 in 0 1k\n\
                                 .PRINT NOISE V(inoise_spectrum)\n\
                                 .END\n";
        let error = Netlist::parse_validated(no_noise_analysis)
            .expect_err("a qualifier alone does not create noise vectors");
        assert!(matches!(error, ParseError::OutputSymbolValidation(_)));
    }

    #[test]
    fn dependency_extraction_preserves_repetitions_and_skips_unknown_operators() {
        let dependencies = extract_output_dependencies(
            "{V(bogo1)} VP(bogo9) VM(bogo9) VQ(a,b) @x1:m1[id] N(x2:m2:id)",
        );
        assert_eq!(
            dependencies
                .iter()
                .map(|dependency| (
                    dependency.operator.as_str(),
                    dependency.symbol.as_str(),
                    dependency.kind,
                ))
                .collect::<Vec<_>>(),
            vec![
                ("V", "bogo1", OutputSymbolKind::Node),
                ("VP", "bogo9", OutputSymbolKind::Node),
                ("VM", "bogo9", OutputSymbolKind::Node),
                ("N", "x2:m2:id", OutputSymbolKind::Node),
            ]
        );
    }

    #[test]
    fn continuous_measurements_retain_their_base_analysis_domain() {
        for (keyword, expected) in [
            ("TR", OutputAnalysisKind::Tran),
            ("TRAN_CONT", OutputAnalysisKind::Tran),
            ("DC_CONT", OutputAnalysisKind::Dc),
            ("AC_CONT", OutputAnalysisKind::Ac),
            ("NOISE_CONT", OutputAnalysisKind::Noise),
        ] {
            assert_eq!(OutputAnalysisKind::from_keyword(keyword), Some(expected));
        }
    }

    #[test]
    fn derived_current_and_noise_probes_retain_device_dependencies() {
        let dependencies = extract_output_dependencies(
            "IM(X1:R1) IR(X2:R2) II(X3:R3) IP(X4:R4) IDB(X5:R5) \
             DNO(X6:R6,thermal) DNI(X7:R7)",
        );
        assert_eq!(
            dependencies
                .iter()
                .map(|dependency| (
                    dependency.operator.as_str(),
                    dependency.symbol.as_str(),
                    dependency.kind,
                ))
                .collect::<Vec<_>>(),
            vec![
                ("IM", "X1:R1", OutputSymbolKind::Device),
                ("IR", "X2:R2", OutputSymbolKind::Device),
                ("II", "X3:R3", OutputSymbolKind::Device),
                ("IP", "X4:R4", OutputSymbolKind::Device),
                ("IDB", "X5:R5", OutputSymbolKind::Device),
                ("DNO", "X6:R6", OutputSymbolKind::Device),
                ("DNI", "X7:R7", OutputSymbolKind::Device),
            ]
        );
    }

    #[test]
    fn direct_current_accessors_accept_arbitrary_leads_without_capturing_expression_builtins() {
        for operator in [
            "I", "IR", "II", "IM", "IP", "IDB", "ID", "IG", "IS", "IB", "IC", "IE", "IA", "IT",
            "I1", "IF",
        ] {
            assert!(is_current_output_accessor(operator), "{operator}");
        }
        for function in ["IMG", "INT"] {
            assert!(!is_current_output_accessor(function), "{function}");
        }

        let dependencies =
            extract_output_dependencies("IA(XA) I1(X1) IF(XF) {IF(V(A), IMG(V(B)), INT(V(C)))}");
        assert_eq!(
            dependencies
                .iter()
                .map(|dependency| (
                    dependency.operator.as_str(),
                    dependency.symbol.as_str(),
                    dependency.kind,
                ))
                .collect::<Vec<_>>(),
            vec![
                ("IA", "XA", OutputSymbolKind::Device),
                ("I1", "X1", OutputSymbolKind::Device),
                ("IF", "XF", OutputSymbolKind::Device),
                ("V", "A", OutputSymbolKind::Node),
                ("V", "B", OutputSymbolKind::Node),
                ("V", "C", OutputSymbolKind::Node),
            ]
        );
    }

    #[test]
    fn parsed_measure_expressions_preserve_builtin_context_in_output_requests() {
        let netlist = Netlist::parse_with_options(
            "typed expression dependencies\n\
             V1 out 0 0\n\
             .TRAN 1 1\n\
             .MEASURE TRAN collision EQN {IF(V(out)>0,INT(V(out)),IMG(V(out)))}\n\
             .END\n",
            crate::netlist::NetlistParseOptions {
                expression_dialect: crate::config::ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("typed expression measure parses");
        let request = netlist
            .output_requests
            .iter()
            .find(|request| {
                request
                    .name
                    .as_deref()
                    .is_some_and(|name| name.eq_ignore_ascii_case("collision"))
            })
            .expect("measure output request retained");
        assert!(request.dependencies.iter().any(|dependency| {
            dependency.operator == "V"
                && dependency.symbol.eq_ignore_ascii_case("out")
                && dependency.kind == OutputSymbolKind::Node
        }));
        assert!(request.dependencies.iter().all(|dependency| {
            !(dependency.operator == "IF" && dependency.kind == OutputSymbolKind::Device)
        }));
    }

    #[test]
    fn dependency_symbols_compact_lexical_whitespace_except_legacy_branch_names() {
        let dependencies = extract_output_dependencies("V(XTOP. *) I(XTOP:R ?) I(YDEVICE BRANCH)");
        assert_eq!(
            dependencies
                .iter()
                .map(|dependency| dependency.symbol.as_str())
                .collect::<Vec<_>>(),
            vec!["XTOP.*", "XTOP:R?", "YDEVICE BRANCH"]
        );
    }

    #[test]
    fn transient_capture_predicate_covers_current_projections_only() {
        for operator in ["I", "IR", "II", "IM", "IP", "IDB", "P", "W"] {
            let source = format!(
                "transient current selection\nR1 1 0 1\n.TRAN 1 1\n.PRINT TRAN {operator}(R1)\n.END\n"
            );
            let netlist = Netlist::parse(&source).expect("transient current request parses");
            let request = netlist
                .output_requests
                .last()
                .expect("PRINT request retained");
            assert!(
                request.requires_transient_device_current_operand(),
                "{operator}"
            );
            assert!(request.selects_transient_device_current("r1"), "{operator}");
        }

        for source in [
            "AC current is not transient\nR1 1 0 1\n.AC LIN 1 1 1\n.PRINT AC IR(R1)\n.END\n",
            "noise contribution is not a branch current\nR1 1 0 1\n.NOISE V(1) V1 LIN 1 1 1\n.PRINT NOISE DNO(R1,thermal)\n.END\n",
        ] {
            let netlist = Netlist::parse(source).expect("non-transient request parses");
            let request = netlist
                .output_requests
                .last()
                .expect("output request retained");
            assert!(!request.requires_transient_device_current_operand());
            assert!(!request.selects_transient_device_current("R1"));
        }
    }

    #[test]
    fn dependency_extraction_scales_to_certification_sized_probe_lists() {
        let source = (0..20_000)
            .map(|index| {
                if index % 2 == 0 {
                    format!("V(n{index})")
                } else {
                    format!("I(d{index})")
                }
            })
            .collect::<Vec<_>>()
            .join(" ");

        let dependencies = extract_output_dependencies(&source);

        assert_eq!(dependencies.len(), 20_000);
        assert!(dependencies.iter().all(|dependency| !dependency.expression));
        assert_eq!(dependencies[0].symbol, "n0");
        assert_eq!(dependencies[19_999].symbol, "d19999");
    }

    #[test]
    fn expression_context_is_preserved_by_linear_dependency_scan() {
        let dependencies =
            extract_output_dependencies("V(out) {V(expr)} 'I(quoted)' \"V(double_quoted)\"");

        assert_eq!(
            dependencies
                .iter()
                .map(|dependency| (dependency.symbol.as_str(), dependency.expression))
                .collect::<Vec<_>>(),
            vec![
                ("out", false),
                ("expr", true),
                ("quoted", true),
                ("double_quoted", true),
            ]
        );
    }

    #[test]
    fn measure_and_four_requests_retain_authored_dependency_spelling() {
        let netlist = Netlist::parse(
            "authored output spelling\n\
             V1 1 0 1\n\
             .TRAN 0.1 1\n\
             .MEASURE TRAN mixedCase MAX V(bogoNode)\n\
             .FOUR 1k I(BogoDevice1) V(MixedNode)\n\
             .END\n",
        )
        .expect("syntactic parse accepts unresolved output dependencies");
        let authored = netlist
            .output_requests
            .iter()
            .flat_map(|request| request.dependencies.iter())
            .map(|dependency| (dependency.operator.as_str(), dependency.symbol.as_str()))
            .collect::<Vec<_>>();
        assert_eq!(
            authored,
            vec![("V", "bogoNode"), ("I", "BogoDevice1"), ("V", "MixedNode")]
        );
    }

    #[test]
    fn save_override_requests_reuse_accessor_and_bare_vector_semantics() {
        let origin = NetlistSourceLocation::in_file("<command line --save>", 2);
        let current = OutputRequest::from_save_override(origin.clone(), "I(MissingDevice)");
        assert_eq!(current.directive, OutputDirectiveKind::Save);
        assert_eq!(current.origin, origin);
        assert_eq!(current.dependencies.len(), 1);
        assert_eq!(current.dependencies[0].operator, "I");
        assert_eq!(current.dependencies[0].symbol, "MissingDevice");
        assert_eq!(current.dependencies[0].kind, OutputSymbolKind::Device);

        let bare = OutputRequest::from_save_override(
            NetlistSourceLocation::in_file("<command line --save>", 3),
            "MissingNode",
        );
        assert_eq!(bare.dependencies.len(), 1);
        assert_eq!(bare.dependencies[0].operator, "V");
        assert_eq!(bare.dependencies[0].symbol, "MissingNode");
        assert_eq!(bare.dependencies[0].kind, OutputSymbolKind::Node);

        let parameter = OutputRequest::from_save_override(
            NetlistSourceLocation::in_file("<command line --save>", 4),
            "@m1[id]",
        );
        assert!(parameter.dependencies.is_empty());
        assert!(matches!(
            super::super::parse_save_probe("ALL"),
            Some(super::super::SaveSignal::All)
        ));
    }

    #[test]
    fn xyce_wildcards_cross_hierarchy_and_question_matches_one_character() {
        assert!(hierarchy_pattern_matches("X1.*", "X1.N1"));
        assert!(hierarchy_pattern_matches("X1.*", "X1.X2.N1"));
        assert!(hierarchy_pattern_matches("X1.*.*", "X1.X2.N1"));
        assert!(hierarchy_pattern_matches("X?.N1", "X1.N1"));
        assert!(!hierarchy_pattern_matches("X?.N1", "X12.N1"));
    }

    #[test]
    fn bug718_voltage_difference_and_ground_replacement_validate() {
        let netlist = Netlist::parse_validated(
            "BUG718 positive\n\
             .PREPROCESS REPLACEGROUND TRUE\n\
             .TRAN 0.1 1\n\
             .PRINT TRAN V(1,0) V(GND,1) {V(GROUND,1)} N(GND)\n\
             V1 1 0 1\n\
             R1 1 0 1\n\
             .END\n",
        )
        .expect("all Xyce ground synonyms resolve under REPLACEGROUND");
        assert_eq!(netlist.options.replace_ground, Some(true));
        assert!(netlist.saves.signals.iter().any(
            |signal| matches!(signal, super::super::SaveSignal::VoltageDiff(pos, _) if pos == "0")
        ));
    }

    #[test]
    fn expression_occurrences_remain_ordered_and_repeated() {
        let error = Netlist::parse_validated(
            "ordered output failures\n\
             V1 1 0 1\n\
             .AC DEC 1 1 10\n\
             .PRINT AC {VP(BOGO9)} {VM(BOGO9)}\n\
             .END\n",
        )
        .expect_err("both authored expression occurrences are invalid");
        let ParseError::OutputSymbolValidation(error) = error else {
            panic!("unexpected error: {error}");
        };
        assert_eq!(
            error
                .unresolved
                .iter()
                .map(|item| (item.operator.as_str(), item.symbol.as_str()))
                .collect::<Vec<_>>(),
            vec![("VP", "BOGO9"), ("VM", "BOGO9")]
        );
    }

    #[test]
    fn direct_requests_sort_devices_then_nodes_and_deduplicate() {
        let error = Netlist::parse_validated(
            "direct output failures\n\
             V1 1 0 1\n\
             .TRAN 0.1 1\n\
             .PRINT TRAN V(D) V(C) V(D) I(RBOGO) I(ABOGO)\n\
             .END\n",
        )
        .expect_err("direct symbols are absent");
        let ParseError::OutputSymbolValidation(error) = error else {
            panic!("unexpected error: {error}");
        };
        assert_eq!(
            error
                .unresolved
                .iter()
                .map(|item| item.symbol.as_str())
                .collect::<Vec<_>>(),
            vec!["ABOGO", "RBOGO", "C", "D"]
        );
    }

    #[test]
    fn qualified_formal_aliases_and_cross_hierarchy_wildcards_resolve() {
        Netlist::parse_validated(
            "alias outputs\n\
             V1 1 0 1\n\
             XTOP 1 0 DIV\n\
             .SUBCKT DIV A B\n\
             R1 A MID 1\n\
             R2 MID B 1\n\
             .ENDS\n\
             .PRINT DC V(XTOP:A) V(XTOP.*) I(XTOP:R?)\n\
             .DC V1 0 1 1\n\
             .END\n",
        )
        .expect("formal aliases, wildcard nodes, and wildcard devices resolve");
    }

    #[test]
    fn interface_aliases_resolve_nested_ports_case_separators_and_ground() {
        let netlist = Netlist::parse(
            "nested interface aliases\n\
             V1 1 0 1\n\
             XTOP 1 0 OUTER\n\
             .SUBCKT INNER P N\n\
             R1 P N 1\n\
             .ENDS\n\
             .SUBCKT OUTER A G\n\
             XINNER A G INNER\n\
             .ENDS\n\
             .END\n",
        )
        .expect("nested alias deck parses");

        let aliases = collect_interface_node_aliases(&netlist).expect("aliases collect");
        assert_eq!(aliases.resolve("xtop:a"), Some("1"));
        assert_eq!(aliases.resolve("XTOP.A"), Some("1"));
        assert_eq!(aliases.resolve("XTOP:XINNER:P"), Some("1"));
        assert_eq!(aliases.resolve("xtop.xinner.n"), Some("0"));
        assert_eq!(aliases.resolve("XTOP:UNKNOWN"), None);
    }

    #[test]
    fn interface_alias_resolution_is_one_hop() {
        let aliases = InterfaceNodeAliases {
            targets: HashMap::from([
                ("A".to_string(), "B".to_string()),
                ("B".to_string(), "C".to_string()),
            ]),
        };
        assert_eq!(aliases.resolve("a"), Some("B"));
        assert_eq!(aliases.resolve("b"), Some("C"));

        let physical_name_collision = InterfaceNodeAliases {
            targets: HashMap::from([("A".to_string(), "A".to_string())]),
        };
        assert_eq!(physical_name_collision.resolve("A"), Some("A"));
    }

    #[test]
    fn interface_alias_collection_rejects_recursive_hierarchy() {
        let netlist = Netlist::parse(
            "recursive aliases\n\
             X1 1 0 LOOP\n\
             .SUBCKT LOOP A B\n\
             XSELF A B LOOP\n\
             .ENDS\n\
             .END\n",
        )
        .expect("syntactic parser retains recursive hierarchy");

        let error = collect_interface_node_aliases(&netlist)
            .expect_err("runtime alias collection must reject recursion");
        assert!(
            matches!(&error, ParseError::Syntax { message, .. } if message.contains("Recursive subcircuit instantiation")),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn requested_interface_alias_collection_omits_unreferenced_ports_and_instances() {
        let netlist = Netlist::parse(
            "selected aliases\n\
             X1 1 0 CELL\n\
             X2 2 0 CELL\n\
             .SUBCKT CELL A B\n\
             R1 A B 1\n\
             .ENDS\n\
             .END\n",
        )
        .expect("selection deck parses");
        let requested = HashSet::from(["X1:A".to_string()]);

        let aliases = collect_requested_interface_node_aliases(&netlist, &requested)
            .expect("selected aliases collect");

        assert_eq!(aliases.iter().count(), 1);
        assert_eq!(aliases.resolve("X1.A"), Some("1"));
        assert_eq!(aliases.resolve("X1.B"), None);
        assert_eq!(aliases.resolve("X2.A"), None);
    }

    #[test]
    fn requested_interface_alias_collection_prunes_empty_and_unrelated_paths() {
        let recursive = Netlist::parse(
            "unrequested recursive hierarchy\n\
             X1 1 LOOP\n\
             .SUBCKT LOOP A\n\
             XSELF A LOOP\n\
             .ENDS\n\
             .END\n",
        )
        .expect("recursive hierarchy parses syntactically");
        let empty = collect_requested_interface_node_aliases(&recursive, &HashSet::new())
            .expect("empty projection does not elaborate hierarchy");
        assert_eq!(empty.iter().count(), 0);

        let netlist = Netlist::parse(
            "prefix-exact selected aliases\n\
             X1 1 MISSING\n\
             X10 10 CELL\n\
             .SUBCKT CELL A\n\
             R1 A 0 1\n\
             .ENDS\n\
             .END\n",
        )
        .expect("selected hierarchy parses syntactically");
        let selected = HashSet::from(["X10:A".to_string()]);
        let aliases = collect_requested_interface_node_aliases(&netlist, &selected)
            .expect("X10 selection must not traverse X1");
        assert_eq!(aliases.resolve("X10.A"), Some("10"));

        let invalid = HashSet::from(["X1:A".to_string()]);
        assert!(matches!(
            collect_requested_interface_node_aliases(&netlist, &invalid),
            Err(ParseError::Syntax { message, .. }) if message.contains("Undefined subcircuit")
        ));
    }

    #[test]
    fn requested_interface_aliases_share_typed_port_binding_validation() {
        let netlist = Netlist::parse(
            "duplicate formal binding\n\
             X1 1 2 CELL\n\
             .SUBCKT CELL A A\n\
             R1 A 0 1\n\
             .ENDS\n\
             .END\n",
        )
        .expect("duplicate formal hierarchy parses syntactically");
        let requested = HashSet::from(["X1:A".to_string()]);
        assert!(matches!(
            collect_requested_interface_node_aliases(&netlist, &requested),
            Err(ParseError::DuplicateSubcircuitPortBinding(_))
        ));
    }

    #[test]
    fn exhaustive_alias_resource_accounting_matches_flattened_leaf_count() {
        let one_leaf = Netlist::parse(
            "one flattened leaf\n\
             X1 1 OUTER\n\
             .SUBCKT INNER A\n\
             R1 A 0 1\n\
             .ENDS\n\
             .SUBCKT OUTER A\n\
             X2 A INNER\n\
             .ENDS\n\
             .END\n",
        )
        .expect("one-leaf hierarchy parses");
        crate::netlist::finish_non_aborting_parse(collect_interface_node_aliases_impl_with_limits(
            &one_leaf,
            None,
            &crate::abort_signal::NoAbort,
            100,
            1,
        ))
        .expect("two X containers plus one resistor emit one leaf");

        let two_leaves = Netlist::parse(
            "two flattened leaves\n\
             X1 1 CELL\n\
             .SUBCKT CELL A\n\
             R1 A 0 1\n\
             R2 A 0 2\n\
             .ENDS\n\
             .END\n",
        )
        .expect("two-leaf hierarchy parses");
        let error = crate::netlist::finish_non_aborting_parse(
            collect_interface_node_aliases_impl_with_limits(
                &two_leaves,
                None,
                &crate::abort_signal::NoAbort,
                100,
                1,
            ),
        )
        .expect_err("second emitted resistor exceeds the one-leaf limit");
        assert!(matches!(
            error,
            ParseError::ResourceLimit(crate::resource::ResourceLimitError {
                resource: crate::resource::ResourceKind::FlattenedElements,
                requested: 2,
                limit: 1,
            })
        ));
    }

    #[test]
    fn interface_alias_collection_accepts_the_exact_hierarchy_depth_limit() {
        let mut source = String::from("allowed alias depth\nXROOT 1 0 S0\n");
        for depth in 0..=99 {
            source.push_str(&format!(".SUBCKT S{depth} A B\n"));
            if depth < 99 {
                source.push_str(&format!("XNEXT A B S{}\n", depth + 1));
            } else {
                source.push_str("R1 A B 1\n");
            }
            source.push_str(".ENDS\n");
        }
        source.push_str(".END\n");
        let netlist = Netlist::parse(&source).expect("boundary hierarchy parses");

        collect_interface_node_aliases(&netlist)
            .expect("an element at hierarchy depth 100 is permitted");
    }

    #[test]
    fn interface_alias_collection_enforces_hierarchy_depth_limit() {
        let mut source = String::from("deep aliases\nXROOT 1 0 S0\n");
        for depth in 0..=100 {
            source.push_str(&format!(".SUBCKT S{depth} A B\n"));
            if depth < 100 {
                source.push_str(&format!("XNEXT A B S{}\n", depth + 1));
            } else {
                source.push_str("R1 A B 1\n");
            }
            source.push_str(".ENDS\n");
        }
        source.push_str(".END\n");
        let netlist = Netlist::parse(&source).expect("deep hierarchy parses syntactically");

        let error = collect_interface_node_aliases(&netlist)
            .expect_err("runtime alias collection must enforce depth limits");
        assert!(
            matches!(
                &error,
                ParseError::ResourceLimit(crate::resource::ResourceLimitError {
                    resource: crate::resource::ResourceKind::HierarchyDepth,
                    requested: 101,
                    limit: 100,
                })
            ),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn n_operator_resolves_interface_aliases_before_excluding_device_parameters() {
        Netlist::parse_validated(
            "N operator ambiguity\n\
             V1 1 0 1\n\
             X1 1 0 CELL\n\
             .SUBCKT CELL A B\n\
             M1 A A B B NM\n\
             .ENDS\n\
             .MODEL NM NMOS LEVEL=1\n\
             .PRINT OP N(X1:A) N(X1:M1:id)\n\
             .OP\n\
             .END\n",
        )
        .expect("formal node aliases resolve while hierarchical device parameters are excluded");
    }

    #[test]
    fn n_operator_defers_existing_device_internal_and_branch_vectors_to_execution() {
        Netlist::parse_validated(
            "device-owned vectors\n\
             VSRC_1 1 0 1\n\
             M1 1 1 0 0 NM\n\
             .MODEL NM NMOS LEVEL=1\n\
             .PRINT OP N(M1_t) N(VSRC_1_BRANCH)\n\
             .OP\n\
             .END\n",
        )
        .expect("generated internal-node and branch-vector metadata remains execution-owned");

        let error = Netlist::parse_validated(
            "unknown device vector\n\
             V1 1 0 1\n\
             .PRINT OP N(BOGO_BRANCH)\n\
             .OP\n\
             .END\n",
        )
        .expect_err("unknown device prefixes remain unresolved");
        assert!(matches!(error, ParseError::OutputSymbolValidation(_)));
    }

    #[test]
    fn n_operator_does_not_exclude_parameters_for_unknown_hierarchical_devices() {
        let error = Netlist::parse_validated(
            "N operator unknown device\n\
             V1 1 0 1\n\
             X1 1 0 CELL\n\
             .SUBCKT CELL A B\n\
             M1 A A B B NM\n\
             .ENDS\n\
             .MODEL NM NMOS LEVEL=1\n\
             .PRINT OP N(X1:BOGO:id)\n\
             .OP\n\
             .END\n",
        )
        .expect_err("an unknown device prefix remains an unresolved node dependency");
        let ParseError::OutputSymbolValidation(error) = error else {
            panic!("unexpected error: {error}");
        };
        assert_eq!(error.unresolved.len(), 1);
        assert_eq!(error.unresolved[0].operator, "N");
        assert_eq!(error.unresolved[0].symbol, "X1:BOGO:id");
        assert_eq!(error.unresolved[0].kind, OutputSymbolKind::Node);
    }

    #[test]
    fn gnd_is_an_ordinary_node_without_replaceground() {
        let options = super::super::NetlistParseOptions {
            expression_dialect: super::super::ExpressionDialect::Xyce,
            ..Default::default()
        };
        Netlist::parse_validated_with_options(
            "defined GND\nV1 GND 0 1\n.PRINT OP V(GND)\n.OP\n.END\n",
            options,
        )
        .expect("an authored GND node is valid without replacement");
        let error = Netlist::parse_validated_with_options(
            "undefined GND\nV1 1 0 1\n.PRINT OP V(GND)\n.OP\n.END\n",
            options,
        )
        .expect_err("GND is not implicit without replacement");
        assert!(matches!(error, ParseError::OutputSymbolValidation(_)));
    }

    #[test]
    fn ngspice_default_aliases_only_exact_gnd() {
        Netlist::parse_validated("ngspice GND\nV1 1 0 1\n.PRINT OP V(GND)\n.OP\n.END\n")
            .expect("ngspice's exact GND alias resolves to zero");
        for alias in ["GND!", "GROUND"] {
            let source =
                format!("ngspice ordinary alias\nV1 1 0 1\n.PRINT OP V({alias})\n.OP\n.END\n");
            assert!(
                matches!(
                    Netlist::parse_validated(&source),
                    Err(ParseError::OutputSymbolValidation(_))
                ),
                "{alias} must remain an ordinary undefined node in ngspice mode"
            );
        }
    }

    #[test]
    fn replaceground_preserves_output_provenance_and_normalizes_typed_execution_fields() {
        let source = "execution output ground aliases\n\
                      V1 out 0 1\n\
                      .PRINT DC {V(GROUND)+V(GND!)}\n\
                      .MEAS DC M PARAM='{V(GROUND)+V(GND!)}'\n\
                      .FOUR 1k V(GROUND) V(GND!)\n\
                      .PREPROCESS REPLACEGROUND TRUE\n\
                      .END\n";
        let netlist =
            Netlist::parse_validated(source).expect("all ground-alias output forms parse");

        assert!(
            netlist
                .source_text
                .as_deref()
                .is_some_and(|text| { text.contains("V(GROUND)") && text.contains("V(GND!)") })
        );
        assert!(netlist.output_requests.iter().any(|request| {
            request
                .dependencies
                .iter()
                .any(|dependency| dependency.symbol.eq_ignore_ascii_case("GROUND"))
        }));
        assert!(netlist.output_requests.iter().any(|request| {
            request
                .dependencies
                .iter()
                .any(|dependency| dependency.symbol.eq_ignore_ascii_case("GND!"))
        }));
        let crate::netlist::measure::MeasureType::Param { expression } =
            &netlist.measurements[0].measure_type
        else {
            panic!("expected PARAM measurement");
        };
        assert_eq!(expression.text, "{V(0)+V(0)}");
        let outputs = netlist
            .analyses
            .iter()
            .find_map(|analysis| match analysis {
                super::super::AnalysisCommand::Four { outputs, .. } => Some(outputs),
                _ => None,
            })
            .expect("FOUR analysis exists");
        assert_eq!(outputs, &["V(0)", "V(0)"]);
    }

    #[test]
    fn late_root_replaceground_applies_to_xline_actuals_before_flattening() {
        let options = super::super::NetlistParseOptions {
            expression_dialect: super::super::ExpressionDialect::Xyce,
            ..Default::default()
        };
        let netlist = Netlist::parse_validated_with_options(
            "late replacement\n\
             X1 1 GND DIV\n\
             .SUBCKT DIV A B\n\
             R1 A B 1\n\
             .ENDS\n\
             .END\n\
             .PREPROCESS REPLACEGROUND TRUE\n",
            options,
        )
        .expect("root prescan observes a late card after END");
        let flattened = super::super::flatten_netlist(&netlist).expect("subcircuit flattens");
        assert_eq!(flattened[0].nodes, vec!["1", "0"]);
    }

    #[test]
    fn explicit_false_keeps_xline_gnd_as_an_ordinary_node() {
        let options = super::super::NetlistParseOptions {
            expression_dialect: super::super::ExpressionDialect::Xyce,
            ..Default::default()
        };
        let netlist = Netlist::parse_validated_with_options(
            "false replacement\n\
             .PREPROCESS REPLACEGROUND FALSE\n\
             X1 1 GND DIV\n\
             .SUBCKT DIV A B\n\
             R1 A B 1\n\
             .ENDS\n\
             .END\n",
            options,
        )
        .expect("FALSE is accepted");
        assert_eq!(netlist.options.replace_ground, None);
        let flattened = super::super::flatten_netlist(&netlist).expect("subcircuit flattens");
        assert_eq!(flattened[0].nodes, vec!["1", "GND"]);
    }
}

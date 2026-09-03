//! Netlist parsing module
//!
//! Parses SPICE-compatible netlist files into an AST representation.
//! Uses a robust nom-based lexer for proper tokenization.
//!
//! Supports:
//! - Standard SPICE elements (R, L, C, V, I, D, Q, M, X)
//! - Advanced elements (K, S, W, T)
//! - Controlled sources (E, F, G, H, B)
//! - XSPICE code models (A) with bracket port syntax
//! - Analysis commands (.OP, .DC, .AC, .DISTO, .TRAN, .NOISE, .PZ, .SENS, .FOUR, .STEP, .MC, .TEMP)
//! - File inclusion (.INCLUDE, .LIB)
//! - Subcircuits with parameter passing
use crate::config::ExpressionDialect;

mod add_resistors;
mod ast;
mod data_table;
pub mod expr;
mod flattener;
pub mod hierarchy_path;
pub mod include;
mod initcond;
pub mod lexer;
pub mod measure;
mod model_resolution;
pub mod multi_run;
mod mutual_inductor;
mod output_symbols;
pub mod param_scope;
mod parser;
mod remove_unused;
pub mod source_map;
pub(crate) mod spectre_adapter;
mod spectre_statistics;
pub mod spef;
mod startup;
mod topology;
mod xspice_parser;

pub use add_resistors::*;
pub use ast::*;
pub use data_table::{FrequencyDataPoint, FrequencyDataTableError};
pub use expr::{
    ParamContext, ParameterRedefinitionDiagnosticPolicy, ParameterRedefinitionPolicy, RandomState,
    StatisticalParamMode,
};
pub(crate) use flattener::flatten_netlist_with_models_config_with_abort;
pub use flattener::{
    FlattenedNetlist, Flattener, FlattenerConfig, InstanceMetadata, XspiceAutoBridgeNodeHint,
    flatten_netlist, flatten_netlist_with_models, flatten_netlist_with_models_with_abort,
};
pub use hierarchy_path::{HierarchyPath, HierarchyPathConfig};
pub use include::source_path_literal_to_host_path;
pub use include::{
    IncludeProcessor, IncludeResolution, IncludeSearchCandidate, IncludeSearchStage,
    ResolvedIncludeDependency, SealedSourceBundle, SealedSourceEdge, normalize_source_path_literal,
    parse_include_directive, parse_lib_directive,
};
pub use initcond::{
    DeviceInitialConditionSourceProvider, DeviceInitialConditionSourceText,
    MAX_DEVICE_INITIAL_CONDITION_SOURCE_BYTES,
};
pub use model_resolution::{UnresolvedDeviceModelReference, unresolved_device_model_references};
pub use mutual_inductor::validate_mutual_inductor_references;
pub(crate) use output_symbols::{
    InterfaceNodeAliases, OutputNodeNamespace, OutputOperand, OutputOperandKind, canonical_symbol,
    collect_output_node_namespace_from_elements_with_abort,
    collect_output_node_namespace_with_limits_and_abort,
    collect_requested_interface_node_aliases_with_abort, is_current_output_accessor,
    is_current_projection_accessor, is_device_lead_current_accessor, measure_output_dependencies,
};
pub use output_symbols::{
    OutputAnalysisKind, OutputDirectiveKind, OutputExpressionIssue,
    OutputExpressionValidationError, OutputRequest, OutputSymbolDependency, OutputSymbolKind,
    OutputSymbolValidationError, PrintDelimiter, UnresolvedOutputSymbol,
    validate_output_expressions, validate_output_expressions_with_abort, validate_output_requests,
    validate_output_requests_with_abort, validate_output_symbols,
    validate_output_symbols_with_abort,
};
pub use param_scope::{ParamResolver, ParamScope, ScopedParam};
pub use parser::*;
pub use source_map::*;
pub use spectre_statistics::*;
pub use startup::{validate_startup_directives, validate_startup_directives_with_abort};
pub(crate) use topology::analyze_dc_ground_paths_with_capacitor_ic_mode;
pub use topology::{
    CapacitorIcDcMode, ConnectivityAnalysisError, ConnectivityDiagnostics, DcGroundPathDiagnostics,
    DcGroundPathNodeDiagnostic, DcGroundPathSeverity, TopologyReduction,
    XYCE_DEFAULT_ZERO_RESISTANCE_TOL, analyze_dc_ground_paths, analyze_xyce_connectivity,
    reduce_supernode_topology,
};
pub(crate) use xspice_parser::{
    DeferredXspiceStringVectorEntry, encode_deferred_xspice_complex,
    encode_deferred_xspice_complex_vector, parse_deferred_xspice_complex,
    parse_deferred_xspice_complex_vector, parse_xspice_string_vector_literal,
    xspice_model_param_accepts_bare_string, xspice_param_prefers_string_vector,
    xspice_param_preserves_numeric_string,
};

impl crate::io::xyce_prn::XycePrnDelimiterSource for PrintDelimiter {
    fn xyce_prn_delimiter(&self) -> crate::io::xyce_prn::XycePrnDelimiter<'_> {
        match self {
            Self::Whitespace => crate::io::xyce_prn::XycePrnDelimiter::Whitespace,
            _ => crate::io::xyce_prn::XycePrnDelimiter::Separated(self.separator()),
        }
    }
}

impl crate::io::xyce_prn::XycePrnRequest for OutputRequest {
    fn xyce_prn_is_print_request(&self) -> bool {
        self.directive == OutputDirectiveKind::Print
    }

    fn xyce_prn_delimiter(&self) -> Option<crate::io::xyce_prn::XycePrnDelimiter<'_>> {
        self.print_delimiter.as_ref().map(|delimiter| {
            crate::io::xyce_prn::XycePrnDelimiterSource::xyce_prn_delimiter(delimiter)
        })
    }

    fn xyce_prn_precision(&self) -> Option<i32> {
        self.print_precision
    }

    fn xyce_prn_width(&self) -> Option<i32> {
        self.print_width
    }
}

impl crate::io::xyce_prn::XycePrnOutputOptions for SimulationOptions {
    fn xyce_prn_print_header(&self) -> Option<bool> {
        self.output_print_header
    }

    fn xyce_prn_print_footer(&self) -> Option<bool> {
        self.output_print_footer
    }
}

use thiserror::Error;

use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};

/// Subcircuits supplied by the embedded RSpice foundation library.
///
/// `builtin_lib` owns the source text and nothing else: parsing it is this
/// layer's work, so the leaf that carries the bytes never has to reach up
/// into the parser to interpret them. Parsed once and cached, because both
/// the flattener and the source-map linter ask for it per netlist.
pub(crate) fn foundation_subcircuits() -> &'static [SubcircuitDef] {
    static SUBCIRCUITS: std::sync::OnceLock<Vec<SubcircuitDef>> = std::sync::OnceLock::new();
    SUBCIRCUITS.get_or_init(|| match parse_netlist(crate::builtin_lib::FOUNDATION_LIB) {
        Ok(netlist) => netlist.subcircuits,
        Err(error) => {
            log::error!("embedded RSpice foundation library did not parse: {error}");
            Vec::new()
        }
    })
}

/// Exact physical location in one netlist source.
///
/// In-memory parses have no path. File-backed parses retain the top-level or
/// included source path so diagnostics do not collapse onto expanded-text line
/// numbers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetlistSourceLocation {
    pub path: Option<std::path::PathBuf>,
    pub line: usize,
}

impl NetlistSourceLocation {
    pub fn in_memory(line: usize) -> Self {
        Self { path: None, line }
    }

    pub fn in_file(path: impl Into<std::path::PathBuf>, line: usize) -> Self {
        Self {
            path: Some(path.into()),
            line,
        }
    }
}

impl std::fmt::Display for NetlistSourceLocation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.path {
            Some(path) => write!(formatter, "{}:{}", path.display(), self.line),
            None => write!(formatter, "line {}", self.line),
        }
    }
}

/// Boundary that exposed an unterminated `.SUBCKT` definition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissingSubcircuitEndsBoundary {
    EndCard,
    AlterCard,
    EndOfSource,
}

impl std::fmt::Display for MissingSubcircuitEndsBoundary {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::EndCard => ".END",
            Self::AlterCard => ".ALTER",
            Self::EndOfSource => "end of source",
        })
    }
}

/// Structured details for an unterminated `.SUBCKT` definition.
///
/// The payload is boxed by [`ParseError`] so ordinary parse results remain
/// compact while callers retain direct, typed access to every diagnostic
/// field.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error(
    "Subcircuit {canonical_name} missing .ENDS (opened as '{authored_name}' in scope '{qualified_name}' at {opened_at}; reached {boundary} at {detected_at})"
)]
pub struct MissingSubcircuitEndsError {
    pub authored_name: String,
    pub canonical_name: String,
    pub qualified_name: String,
    pub opened_at: NetlistSourceLocation,
    pub detected_at: NetlistSourceLocation,
    pub boundary: MissingSubcircuitEndsBoundary,
}

/// Conflicting actual-node bindings for a repeated formal `.SUBCKT` port.
///
/// Xyce permits duplicate formal ports only when every occurrence maps to the
/// same effective node at a particular X-line invocation.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error(
    "Duplicate nodes in .subckt {canonical_subcircuit_name} point to different nodes in X line invocation: formal '{formal_port}' at position {first_position} maps to '{first_actual_node}', but position {conflicting_position} maps to '{conflicting_actual_node}'; Error invoking subcircuit {canonical_subcircuit_name} instance {canonical_instance_name}"
)]
pub struct DuplicateSubcircuitPortBindingError {
    pub subcircuit_name: String,
    pub canonical_subcircuit_name: String,
    pub instance_name: String,
    pub canonical_instance_name: String,
    pub qualified_instance_name: String,
    pub formal_port: String,
    pub first_position: usize,
    pub conflicting_position: usize,
    pub first_actual_node: String,
    pub conflicting_actual_node: String,
}

/// A formal global subcircuit port was connected to a differently named node.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error(
    "Global node in subcircuit invocation must match same name in .subckt: formal '{formal_port}' at position {position} maps to '{actual_node}'; Error invoking subcircuit {canonical_subcircuit_name} instance {canonical_instance_name}"
)]
pub struct GlobalSubcircuitPortBindingError {
    pub subcircuit_name: String,
    pub canonical_subcircuit_name: String,
    pub instance_name: String,
    pub canonical_instance_name: String,
    pub qualified_instance_name: String,
    pub formal_port: String,
    pub position: usize,
    pub actual_node: String,
}

/// Kind of parameter declaration involved in a same-scope redefinition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParameterDefinitionKind {
    Parameter,
    GlobalParameter,
    ParameterFunction,
}

impl std::fmt::Display for ParameterDefinitionKind {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::Parameter => ".PARAM",
            Self::GlobalParameter => ".GLOBAL_PARAM",
            Self::ParameterFunction => ".PARAM function",
        })
    }
}

/// Structured failure for a parameter name defined more than once in one
/// lexical scope while duplicate definitions are configured as errors.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error(
    "Parameter {canonical_name} defined more than once in {kind} scope (first at {first_origin}; redefined at {duplicate_origin})"
)]
pub struct ParameterRedefinitionError {
    pub duplicate_name: String,
    pub canonical_name: String,
    pub kind: ParameterDefinitionKind,
    pub first_origin: NetlistSourceLocation,
    pub duplicate_origin: NetlistSourceLocation,
}

/// A device parameter is authored more than once on one `.MODEL` card.
///
/// Parameter names are case-insensitive. `LEVEL` is intentionally excluded:
/// Xyce treats it as a model selector rather than a device parameter and
/// permits repeated selectors on one logical card.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error(
    "{model_origin}: Device model {canonical_model_name}: Duplicate specification of parameter {canonical_parameter_name}"
)]
pub struct DuplicateModelParameterError {
    pub model_name: String,
    pub canonical_model_name: String,
    pub parameter_name: String,
    pub canonical_parameter_name: String,
    pub model_origin: NetlistSourceLocation,
}

/// A subcircuit instance names a definition that is absent from its visible
/// lexical scope.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error(
    "Subcircuit {canonical_subcircuit_name} has not been defined for instance {canonical_instance_name}"
)]
pub struct UndefinedSubcircuitError {
    pub subcircuit_name: String,
    pub canonical_subcircuit_name: String,
    pub instance_name: String,
    pub canonical_instance_name: String,
    pub qualified_instance_name: String,
}

/// A model-backed device instance ended before its required model name.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error("Model is required for device {canonical_device_name} and no valid model card found")]
pub struct MissingDeviceModelError {
    pub line: usize,
    pub device_name: String,
    pub canonical_device_name: String,
    pub device_type: String,
}

/// A retained subcircuit-local `.PARAM` definition could not be resolved.
///
/// The canonical definition name and the missing dependency are distinct
/// identities. Keeping both typed prevents hierarchy diagnostics from
/// collapsing a failure such as `FOO=(MEH != 1)` into the less useful bare
/// `Undefined parameter: MEH` message.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error(
    "Unable to resolve parameter {canonical_parameter_name} found in .PARAM statement '{parameter_name}={expression}' in subcircuit {canonical_subcircuit_name} instance {qualified_instance_name}: {reason}"
)]
pub struct UnresolvedSubcircuitParameterError {
    pub subcircuit_name: String,
    pub canonical_subcircuit_name: String,
    pub instance_name: String,
    pub canonical_instance_name: String,
    pub qualified_instance_name: String,
    pub parameter_name: String,
    pub canonical_parameter_name: String,
    pub expression: String,
    pub missing_dependency: Option<String>,
    pub reason: String,
}

/// A mutual-inductor card references an inductor that is not defined in the
/// same netlist scope.
///
/// SPICE inductor names are scope-local and case-insensitive. The authored,
/// canonical, and qualified spellings are retained independently so public
/// adapters can render concise compatibility messages without discarding the
/// identity needed by IDEs and automation.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error(
    "Undefined inductor {canonical_inductor_name} in mutual inductor {canonical_coupling_name} definition."
)]
pub struct UndefinedMutualInductorReferenceError {
    pub origin: NetlistSourceLocation,
    pub authored_coupling_name: String,
    pub canonical_coupling_name: String,
    pub qualified_coupling_name: String,
    pub authored_inductor_name: String,
    pub canonical_inductor_name: String,
    pub qualified_inductor_name: String,
    pub scope_name: Option<String>,
    pub reference_position: usize,
}

/// Xyce does not permit `.IC` and `.NODESET` startup modes in one deck.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[error(
    "Cannot set both .IC and .NODESET simultaneously (first {first_kind:?} card at {first}; conflicting {conflicting_kind:?} card at {conflicting})"
)]
pub struct StartupDirectiveConflictError {
    pub first_kind: StartupDirectiveKind,
    pub first: NetlistSourceLocation,
    pub conflicting_kind: StartupDirectiveKind,
    pub conflicting: NetlistSourceLocation,
}

/// Two effective startup voltage constraints prescribe incompatible
/// potentials for the same connected constraint component.
#[derive(Debug, Clone, PartialEq, Error)]
#[error(
    "{conflicting}: inconsistent {kind:?} startup constraint V({positive},{negative})={actual:.17e}; the constraint graph established {expected:.17e} from {established}"
)]
pub struct StartupConstraintConflictError {
    pub kind: StartupDirectiveKind,
    pub established: NetlistSourceLocation,
    pub conflicting: NetlistSourceLocation,
    pub positive: String,
    pub negative: String,
    pub expected: Value,
    pub actual: Value,
}

/// Structured `.INITCOND` failures retained across parser, source-provider,
/// hierarchy, and public adapter boundaries.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum DeviceInitialConditionError {
    #[error(".INITCOND line may appear only once. First card at {first}; duplicate at {duplicate}")]
    DuplicateDirective {
        first: NetlistSourceLocation,
        duplicate: NetlistSourceLocation,
    },

    #[error("{origin}: .INITCOND line is missing information")]
    MissingInformation { origin: NetlistSourceLocation },

    #[error("{origin}: .INITCOND line is not formatted properly: {detail}")]
    MalformedDirective {
        origin: NetlistSourceLocation,
        detail: String,
    },

    #[error("Could not open the .INITCOND file {requested_path} (referenced at {origin})")]
    SourceUnavailable {
        origin: NetlistSourceLocation,
        requested_path: String,
    },

    #[error(
        ".INITCOND file '{requested_path}' is not formatted properly at {record_origin}: {detail}"
    )]
    MalformedSource {
        origin: NetlistSourceLocation,
        requested_path: String,
        record_origin: NetlistSourceLocation,
        detail: String,
    },

    #[error(
        "{origin}: .INITCOND value {value_index} for device '{device}' must be finite, found {value}"
    )]
    NonFiniteValue {
        origin: NetlistSourceLocation,
        device: String,
        value_index: usize,
        value: Value,
    },

    #[error(
        "{origin}: .INITCOND FILE '{requested_path}' has not been resolved through a source provider"
    )]
    UnresolvedSource {
        origin: NetlistSourceLocation,
        requested_path: String,
    },

    #[error(
        "{origin}: .INITCOND for device '{device}' requires {expected}, found {actual} value(s)"
    )]
    InvalidArity {
        origin: NetlistSourceLocation,
        device: String,
        expected: String,
        actual: usize,
    },

    #[error(
        "{origin}: .INITCOND target '{device}' is a matched {device_type}, whose IC grammar and startup physics are not supported"
    )]
    UnsupportedTarget {
        origin: NetlistSourceLocation,
        device: String,
        device_type: String,
    },
}

/// Errors that can occur during netlist parsing
#[derive(Debug, Error)]
pub enum ParseError {
    #[error(transparent)]
    ResourceLimit(#[from] crate::resource::ResourceLimitError),

    #[error("Syntax error at line {line}: {message}")]
    Syntax { line: usize, message: String },

    /// A construct the grammar recognizes and this build declines to lower.
    ///
    /// Separate from [`Self::Syntax`] because the deck is not wrong: an
    /// unsupported Xyce Y-device family or Spectre construct is a capability
    /// boundary, and a frontend reports it as a gap rather than as an
    /// authoring mistake. `capability` is the same stable dotted token the
    /// engine's [`crate::UnsupportedCapabilityError`] publishes.
    #[error("{origin}: unsupported capability [{capability}]: {detail}")]
    UnsupportedCapability {
        origin: NetlistSourceLocation,
        capability: &'static str,
        detail: String,
    },

    #[error("Unknown device type: {0}")]
    UnknownDevice(String),

    #[error("Invalid node reference: {0}")]
    InvalidNode(String),

    #[error(
        "Duplicate element name '{duplicate_name}' (canonical '{canonical_name}') in scope '{scope}' at line {duplicate_line}; first declared as '{first_name}' at line {first_line}"
    )]
    DuplicateName {
        canonical_name: String,
        first_name: String,
        duplicate_name: String,
        scope: String,
        first_line: usize,
        duplicate_line: usize,
    },

    #[error(transparent)]
    ParameterRedefinition(Box<ParameterRedefinitionError>),

    #[error(transparent)]
    DuplicateModelParameter(Box<DuplicateModelParameterError>),

    #[error(transparent)]
    MissingSubcircuitEnds(Box<MissingSubcircuitEndsError>),

    #[error(transparent)]
    DuplicateSubcircuitPortBinding(Box<DuplicateSubcircuitPortBindingError>),

    #[error(transparent)]
    GlobalSubcircuitPortBinding(Box<GlobalSubcircuitPortBindingError>),

    #[error(transparent)]
    UndefinedSubcircuit(Box<UndefinedSubcircuitError>),

    #[error(transparent)]
    MissingDeviceModel(Box<MissingDeviceModelError>),

    #[error(transparent)]
    UnresolvedSubcircuitParameter(Box<UnresolvedSubcircuitParameterError>),

    #[error(transparent)]
    UndefinedMutualInductorReference(Box<UndefinedMutualInductorReferenceError>),

    #[error(transparent)]
    OutputSymbolValidation(Box<OutputSymbolValidationError>),

    #[error(transparent)]
    OutputExpressionValidation(Box<OutputExpressionValidationError>),

    #[error(transparent)]
    StartupDirectiveConflict(Box<StartupDirectiveConflictError>),

    #[error(transparent)]
    StartupConstraintConflict(Box<StartupConstraintConflictError>),

    #[error(transparent)]
    DeviceInitialCondition(Box<DeviceInitialConditionError>),

    #[error("Missing required parameter: {0}")]
    MissingParameter(String),

    #[error("Undefined parameter: {0}")]
    UndefinedParameter(String),

    #[error("Invalid value: {0}")]
    InvalidValue(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

impl ParseError {
    /// Where in the source this failure was raised, when one location says it.
    ///
    /// Frontends print a file and line beside the message rather than leaving
    /// the operator to find "line N" inside prose. The match is exhaustive so
    /// a new variant has to declare whether it has a location; `None` means
    /// the failure genuinely has no single point in the source — a resolution
    /// failure discovered after flattening, or a conflict that spans two
    /// declarations, whose message names both.
    pub fn source_location(&self) -> Option<NetlistSourceLocation> {
        match self {
            Self::Syntax { line, .. } => Some(NetlistSourceLocation::in_memory(*line)),
            Self::UnsupportedCapability { origin, .. } => Some(origin.clone()),
            Self::DuplicateName { duplicate_line, .. } => {
                Some(NetlistSourceLocation::in_memory(*duplicate_line))
            }
            Self::ParameterRedefinition(error) => Some(error.duplicate_origin.clone()),
            Self::DuplicateModelParameter(error) => Some(error.model_origin.clone()),
            Self::MissingSubcircuitEnds(error) => Some(error.detected_at.clone()),
            Self::MissingDeviceModel(error) => Some(NetlistSourceLocation::in_memory(error.line)),
            Self::UndefinedMutualInductorReference(error) => Some(error.origin.clone()),
            Self::ResourceLimit(_)
            | Self::UnknownDevice(_)
            | Self::InvalidNode(_)
            | Self::DuplicateSubcircuitPortBinding(_)
            | Self::GlobalSubcircuitPortBinding(_)
            | Self::UndefinedSubcircuit(_)
            | Self::UnresolvedSubcircuitParameter(_)
            | Self::OutputSymbolValidation(_)
            | Self::OutputExpressionValidation(_)
            | Self::StartupDirectiveConflict(_)
            | Self::StartupConstraintConflict(_)
            | Self::DeviceInitialCondition(_)
            | Self::MissingParameter(_)
            | Self::UndefinedParameter(_)
            | Self::InvalidValue(_)
            | Self::Io(_) => None,
        }
    }
}

/// Error returned by cooperative, abort-aware netlist parsing APIs.
///
/// Keeping cancellation separate from [`ParseError`] preserves the existing
/// parser-error contract for legacy callers while allowing interactive
/// execution to distinguish a user-requested stop from invalid input.
#[derive(Debug, Error)]
pub enum ParseWithAbortError {
    /// Cooperative cancellation was requested.
    #[error("Netlist parsing aborted")]
    Aborted,
    /// The source was invalid or could not be read.
    #[error(transparent)]
    Parse(#[from] ParseError),
}

impl ParseWithAbortError {
    /// Whether this error represents cooperative cancellation.
    pub fn is_aborted(&self) -> bool {
        matches!(self, Self::Aborted)
    }
}

#[inline]
pub(crate) fn ensure_parse_not_aborted(abort: &dyn AbortSignal) -> Result<(), ParseWithAbortError> {
    if abort.is_aborted() {
        Err(ParseWithAbortError::Aborted)
    } else {
        Ok(())
    }
}

#[inline]
pub(crate) fn poll_parse_abort(
    abort: &dyn AbortSignal,
    index: usize,
) -> Result<(), ParseWithAbortError> {
    const POLL_STRIDE: usize = 64;
    if index.is_multiple_of(POLL_STRIDE) {
        ensure_parse_not_aborted(abort)?;
    }
    Ok(())
}

/// Poll while traversing a potentially very large single logical line.
///
/// Outer line-level checks alone are insufficient for generated `.DATA`
/// rows, expressions, or extraction records that can contain megabytes of
/// tokens without a newline.
pub(crate) fn poll_parse_text(
    abort: &dyn AbortSignal,
    text: &str,
) -> Result<(), ParseWithAbortError> {
    const TEXT_CHUNK_BYTES: usize = 4096;
    for _ in text.as_bytes().chunks(TEXT_CHUNK_BYTES) {
        ensure_parse_not_aborted(abort)?;
    }
    Ok(())
}

pub(crate) fn finish_non_aborting_parse<T>(
    result: Result<T, ParseWithAbortError>,
) -> Result<T, ParseError> {
    match result {
        Ok(value) => Ok(value),
        Err(ParseWithAbortError::Parse(error)) => Err(error),
        Err(ParseWithAbortError::Aborted) => {
            unreachable!("NoAbort cannot cancel netlist parsing")
        }
    }
}

pub(crate) fn map_abort_parse_error(
    error: ParseWithAbortError,
    map: impl FnOnce(ParseError) -> ParseError,
) -> ParseWithAbortError {
    match error {
        ParseWithAbortError::Aborted => ParseWithAbortError::Aborted,
        ParseWithAbortError::Parse(error) => ParseWithAbortError::Parse(map(error)),
    }
}

use measure::MeasureStatement;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};

/// The source resolver that produced a parsed netlist.
///
/// Derived analyses reparse the root when parameter values change. Retaining
/// the resolver contract prevents an in-memory/sealed project from silently
/// acquiring filesystem access and preserves the original desktop include
/// search semantics.
#[derive(Debug, Clone)]
pub(crate) enum NetlistReplayContext {
    InMemory,
    PathWithExecutionDir(PathBuf),
    SearchPaths {
        paths: Vec<PathBuf>,
        execution_dir: PathBuf,
    },
    Sealed(SealedSourceBundle),
}

/// Canonical AST overrides that are not represented by the authored source.
///
/// Device DATA/STEP overrides are applied after parameter-driven reparsing.
/// Keeping the overlay on the netlist makes a later reparse compositional: no
/// previously applied electrical change can disappear behind stale source.
#[derive(Debug, Clone, Default)]
pub(crate) struct NetlistAstOverlay {
    pub(crate) device_parameters: BTreeMap<(String, String), crate::Value>,
}

/// Effective dialect-specific node-zero alias policy after parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroundPolicy {
    /// Only the canonical node name `0` is ground.
    OnlyZero,
    /// ngspice's default automatic `GND` alias.
    NgspiceGnd,
    /// Xyce `.PREPROCESS REPLACEGROUND TRUE` aliases.
    XyceReplace,
}

impl GroundPolicy {
    /// Return the canonical execution node name for this dialect policy.
    /// Non-ground names retain their authored spelling.
    pub fn canonical_node<'a>(self, node: &'a str) -> &'a str {
        let canonical = node.trim().to_ascii_uppercase();
        let aliases_ground = match self {
            Self::OnlyZero => false,
            Self::NgspiceGnd => canonical == "GND",
            Self::XyceReplace => matches!(canonical.as_str(), "GND" | "GND!" | "GROUND"),
        };
        if canonical == "0" || aliases_ground {
            "0"
        } else {
            node
        }
    }

    pub fn is_ground(self, node: &str) -> bool {
        self.canonical_node(node) == "0"
    }
}

/// Rewrite node-zero aliases only where they are used as atomic arguments to
/// node-probe accessors. The source AST remains available verbatim through
/// `Netlist::source_text`; this transformation is for execution-facing typed
/// fields, including braced and quoted expressions.
pub(crate) fn apply_ground_policy_to_probe_references(input: &str, policy: GroundPolicy) -> String {
    fn is_identifier_start(character: char) -> bool {
        character.is_ascii_alphabetic() || character == '_'
    }

    fn is_identifier_continue(character: char) -> bool {
        character.is_ascii_alphanumeric() || character == '_'
    }

    fn is_node_probe(operator: &str) -> bool {
        matches!(
            operator.to_ascii_uppercase().as_str(),
            "V" | "VR" | "VI" | "VM" | "VP" | "VDB" | "N"
        )
    }

    fn matching_parenthesis(input: &str, open: usize) -> Option<usize> {
        let mut depth = 0usize;
        let mut single_quote = false;
        let mut double_quote = false;
        for (offset, character) in input[open..].char_indices() {
            match character {
                '\'' if !double_quote => single_quote = !single_quote,
                '"' if !single_quote => double_quote = !double_quote,
                '(' if !single_quote && !double_quote => depth += 1,
                ')' if !single_quote && !double_quote => {
                    depth = depth.checked_sub(1)?;
                    if depth == 0 {
                        return Some(open + offset);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn split_arguments(input: &str) -> Vec<&str> {
        let mut arguments = Vec::new();
        let mut start = 0usize;
        let mut parentheses = 0usize;
        let mut braces = 0usize;
        let mut single_quote = false;
        let mut double_quote = false;
        for (index, character) in input.char_indices() {
            match character {
                '\'' if !double_quote => single_quote = !single_quote,
                '"' if !single_quote => double_quote = !double_quote,
                '(' if !single_quote && !double_quote => parentheses += 1,
                ')' if !single_quote && !double_quote => {
                    parentheses = parentheses.saturating_sub(1);
                }
                '{' if !single_quote && !double_quote => braces += 1,
                '}' if !single_quote && !double_quote => braces = braces.saturating_sub(1),
                ',' if parentheses == 0 && braces == 0 && !single_quote && !double_quote => {
                    arguments.push(&input[start..index]);
                    start = index + character.len_utf8();
                }
                _ => {}
            }
        }
        arguments.push(&input[start..]);
        arguments
    }

    fn replace_atomic_argument(argument: &str, policy: GroundPolicy) -> String {
        let trimmed = argument.trim();
        if !policy.is_ground(trimmed) || trimmed == "0" {
            return argument.to_string();
        }
        let leading = argument.len() - argument.trim_start().len();
        let trailing = argument.len() - argument.trim_end().len();
        format!(
            "{}0{}",
            &argument[..leading],
            &argument[argument.len() - trailing..]
        )
    }

    fn rewrite(input: &str, policy: GroundPolicy) -> String {
        let mut output = String::with_capacity(input.len());
        let mut cursor = 0usize;
        while cursor < input.len() {
            let character = input[cursor..]
                .chars()
                .next()
                .expect("cursor remains on a character boundary");
            if !is_identifier_start(character) {
                output.push(character);
                cursor += character.len_utf8();
                continue;
            }

            let identifier_start = cursor;
            cursor += character.len_utf8();
            while cursor < input.len() {
                let next = input[cursor..]
                    .chars()
                    .next()
                    .expect("cursor remains on a character boundary");
                if !is_identifier_continue(next) {
                    break;
                }
                cursor += next.len_utf8();
            }
            let identifier = &input[identifier_start..cursor];
            let mut open = cursor;
            while open < input.len() {
                let whitespace = input[open..]
                    .chars()
                    .next()
                    .expect("cursor remains on a character boundary");
                if !whitespace.is_whitespace() {
                    break;
                }
                open += whitespace.len_utf8();
            }
            if !is_node_probe(identifier)
                || !input[open..].starts_with('(')
                || matching_parenthesis(input, open).is_none()
            {
                output.push_str(&input[identifier_start..cursor]);
                continue;
            }

            let close =
                matching_parenthesis(input, open).expect("matching parenthesis was checked above");
            output.push_str(&input[identifier_start..=open]);
            let rewritten_inner = rewrite(&input[open + 1..close], policy);
            let arguments = split_arguments(&rewritten_inner);
            for (index, argument) in arguments.iter().enumerate() {
                if index != 0 {
                    output.push(',');
                }
                output.push_str(&replace_atomic_argument(argument, policy));
            }
            output.push(')');
            cursor = close + 1;
        }
        output
    }

    rewrite(input, policy)
}

/// Represents a parsed netlist ready for circuit construction
#[derive(Debug, Clone)]
pub struct Netlist {
    /// Circuit title (first line of netlist)
    pub title: String,
    /// All circuit elements
    pub elements: Vec<Element>,
    /// Analysis commands
    pub analyses: Vec<AnalysisCommand>,
    /// Optional `.LIN` directive semantics.  Only `SPARCALC=0` is currently
    /// executable as an ordinary AC analysis; full Touchstone `.LIN` output
    /// remains an explicit runner contract until its multi-port solver is
    /// wired through.
    pub lin_analysis: Option<LinAnalysis>,
    /// Typed `.FFT` post-processing requests. These remain inert unless the
    /// selected primary analysis is transient.
    pub fft_analyses: Vec<FftAnalysis>,
    /// Named `.DATA` tables retained for table-driven analyses such as
    /// `.STEP DATA=<name>`.
    pub data_tables: Vec<DataTable>,
    /// Model definitions
    pub models: Vec<ModelDef>,
    /// Subcircuit definitions
    pub subcircuits: Vec<SubcircuitDef>,
    /// Parameter definitions from .PARAM statements
    pub params: ParamContext,
    /// Native Spectre process/mismatch distributions retained as an
    /// executable, validated statistical plan.
    pub spectre_statistics: SpectreStatisticsPlan,
    /// Statistical run coordinate a Spectre `statistics` block draws from.
    ///
    /// Monte Carlo drivers set this per trial. Every other coordinate is
    /// stamped by the deck-plan materializer from its own axes and
    /// temperature. A build that finds it absent performs no statistical
    /// sampling at all, so a deck with variations must never reach the
    /// builder without one.
    pub spectre_statistical_coordinate: Option<SpectreStatisticalCoordinate>,
    /// Initial conditions from .IC statements
    pub initial_conditions: Vec<InitialCondition>,
    /// Netlist-wide device `IC=` overrides from Xyce's `.INITCOND` directive.
    pub device_initial_conditions: Option<DeviceInitialConditionDirective>,
    /// Operating-point node voltage hints from .NODESET statements
    pub node_sets: Vec<NodeSet>,
    /// Card-level startup diagnostic provenance. This sidecar is read-only
    /// metadata and is intentionally excluded from checkpoint identity.
    pub(crate) startup_directives: Vec<StartupDirectiveRecord>,
    /// Global nodes from .GLOBAL (not renamed in subcircuits)
    pub global_nodes: HashSet<String>,
    /// Measurement statements from .MEAS commands
    pub measurements: Vec<MeasureStatement>,
    /// Output selection from .SAVE/.PROBE/.PRINT/.PLOT commands
    pub saves: SaveSet,
    /// Typed source/provenance sidecar for every output-producing directive.
    /// Execution continues to use `saves`, `measurements`, and `.FOUR`
    /// analysis commands; this ordered sidecar owns semantic validation.
    pub output_requests: Vec<OutputRequest>,
    /// Simulation options from .OPTIONS commands
    pub options: SimulationOptions,
    /// Verilog-A model includes from .VERILOGA statements
    pub veriloga_includes: Vec<VerilogAInclude>,
    /// SPEF parasitic files from `.spef_include` (or `.include *.spef`),
    /// back-annotated onto the parsed deck by the path-aware parse entry
    /// points (`netlist::spef`).
    pub spef_includes: Vec<String>,
    /// Non-fatal parser diagnostics for constructs that were accepted but not
    /// fully acted on. Callers should surface these to users before simulation.
    pub diagnostics: Vec<ParseDiagnostic>,
    /// What became of every command inside this deck's `.control` regions, one
    /// ordered entry per non-blank, non-comment body line. Editors read it to
    /// state the disposition on the line that authored it instead of leaving
    /// an imported ngspice block to disappear behind a single blanket warning.
    pub control_dispositions: Vec<ControlCommandRecord>,
    /// Authored PSpice E/G CHEBYSHEV card count retained independently from
    /// the variable-size synthesized element realization.
    pub(crate) pspice_chebyshev_source_count: usize,
    /// Optional original netlist text used to build this AST.
    /// Stored to support parameter re-application workflows (e.g., sensitivity).
    pub source_text: Option<String>,
    /// Optional source path for the netlist used to resolve relative includes
    /// and model-file references during reparsing workflows.
    pub source_path: Option<PathBuf>,
    /// Resolver provenance for safe, behaviorally identical source replay.
    pub(crate) replay_context: Option<NetlistReplayContext>,
    /// Canonical electrical overrides layered over the parsed source AST.
    pub(crate) ast_overlay: NetlistAstOverlay,
}

impl Netlist {
    /// Return the number of authored PSpice E/G CHEBYSHEV source cards that
    /// were accepted while parsing this netlist and its expanded includes.
    ///
    /// Each card counts once even when its exact filter realization requires
    /// several dynamic helper elements.
    pub fn pspice_chebyshev_source_count(&self) -> usize {
        self.pspice_chebyshev_source_count
    }

    /// Resolve a named `.DATA` table into validated frequency-axis rows.
    ///
    /// AC and noise table-driven analyses share this semantic contract.  The
    /// table lookup is case-insensitive, while the authored column spelling
    /// and row order are retained for parameter override application.
    pub fn frequency_data_table_points(
        &self,
        table_name: &str,
    ) -> Result<Vec<FrequencyDataPoint>, FrequencyDataTableError> {
        let table = self
            .data_tables
            .iter()
            .find(|table| table.name.eq_ignore_ascii_case(table_name))
            .ok_or_else(|| FrequencyDataTableError::UnknownTable {
                table_name: table_name.to_string(),
            })?;
        table.frequency_points()
    }

    fn enforce_root_source_limits_with_abort(
        input: &str,
        limits: crate::resource::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        crate::resource::ResourceLimitError::ensure(
            crate::resource::ResourceKind::NetlistBytes,
            input.len(),
            limits.max_netlist_bytes,
        )
        .map_err(ParseError::from)?;
        for (line_index, _) in input.lines().enumerate() {
            poll_parse_abort(abort, line_index)?;
            crate::resource::ResourceLimitError::ensure(
                crate::resource::ResourceKind::NetlistLines,
                line_index.saturating_add(1),
                limits.max_netlist_lines,
            )
            .map_err(ParseError::from)?;
        }
        ensure_parse_not_aborted(abort)
    }

    /// Effective ground policy shared by elaboration, validation, and output
    /// execution. This is semantic state, independent of source spelling.
    pub fn ground_policy(&self) -> GroundPolicy {
        if self.options.replace_ground.unwrap_or(false) {
            GroundPolicy::XyceReplace
        } else if self.params.expression_dialect() == ExpressionDialect::Xyce {
            GroundPolicy::OnlyZero
        } else {
            GroundPolicy::NgspiceGnd
        }
    }
}

/// Severity for parser diagnostics that do not abort parsing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    /// The deck parsed, but the simulator ignored or downgraded a construct.
    Warning,
}

/// Structured parser diagnostic suitable for CLI, UI, Python, and WASM callers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseDiagnostic {
    /// 1-based input line number. `0` is reserved for diagnostics that cannot be
    /// tied to one source line.
    pub line: usize,
    /// Exact physical source location when the parser has source-map context.
    ///
    /// Callers that construct diagnostics for an in-memory deck may leave this
    /// unset and continue to use [`Self::line`]. File-backed parsing populates
    /// it for root and included sources so downstream tools do not project an
    /// included warning onto the root deck.
    pub origin: Option<NetlistSourceLocation>,
    /// Stable machine-readable diagnostic code.
    pub code: String,
    /// Human-readable diagnostic message.
    pub message: String,
    /// Diagnostic severity.
    pub severity: DiagnosticSeverity,
}

impl ParseDiagnostic {
    /// Create a warning diagnostic.
    pub fn warning(line: usize, code: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            line,
            origin: None,
            code: code.into(),
            message: message.into(),
            severity: DiagnosticSeverity::Warning,
        }
    }

    /// Create a warning tied to an exact physical source location.
    pub fn warning_at(
        origin: NetlistSourceLocation,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            line: origin.line,
            origin: Some(origin),
            code: code.into(),
            message: message.into(),
            severity: DiagnosticSeverity::Warning,
        }
    }

    /// Render the two historical Xyce warning lines for diagnostics whose
    /// compatibility contract requires byte-stable wrapper predicates.
    pub fn xyce_legacy_warning_lines(&self) -> Option<[String; 2]> {
        if self.severity != DiagnosticSeverity::Warning
            || self.code != "xyce-unknown-diode-model-parameter"
        {
            return None;
        }
        let origin = self.origin.as_ref()?;
        let filename = origin.path.as_ref()?.file_name()?.to_str()?;
        Some([
            format!(
                "Netlist warning in file {filename} at or near line {}",
                origin.line
            ),
            self.message.clone(),
        ])
    }
}

/// What the parser did with one command inside a `.control` … `.endc` region.
///
/// RSpice has no `.control` interpreter: the region is scripting, and the
/// parser either lifts a command out as a declarative directive or ignores it.
/// Which of the two happened is a per-command fact, so it is recorded per
/// command rather than summarized once for the whole region.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ControlCommandDisposition {
    /// The command was rewritten as the listed declarative directives, which
    /// the parsed deck carries as if they had been authored outside the block.
    ///
    /// Almost every promotion produces exactly one spelling; the list exists
    /// because a single `set` line may carry two independently promoted
    /// settings.
    Promoted {
        /// Exact directive spellings appended to the deck, in emission order.
        directives: Vec<String>,
    },
    /// A scalar `let` assignment whose value was substituted into at least one
    /// promoted directive, directly or through another consumed assignment.
    ConsumedByPromotion {
        /// Assigned name, upper-cased the way the substitution table keys it.
        name: String,
    },
    /// Interactive scripting the parser read and ignored.
    Dropped,
}

/// One `.control` body command and what the parser did with it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlCommandRecord {
    /// 1-based line number in the source that authored the command.
    pub line: usize,
    /// Exact physical source location when the parser has source-map context,
    /// so an included block's command is not projected onto the root deck.
    pub origin: Option<NetlistSourceLocation>,
    /// Command word as written, without a leading `.` and without arguments.
    pub command: String,
    /// What became of the command.
    pub disposition: ControlCommandDisposition,
}

/// Verilog-A model include directive
///
/// References an external Verilog-A file to be compiled and used as a model.
/// Usage in netlist: `.VERILOGA filename.va [MODELNAME]`
#[derive(Debug, Clone)]
pub struct VerilogAInclude {
    /// Path to the Verilog-A source file
    pub file_path: std::path::PathBuf,
    /// Optional model name override (defaults to module name in VA file)
    pub model_name: Option<String>,
}

impl Netlist {
    /// Parse a netlist from a string.
    ///
    /// Follows the SPICE convention that the **first line is the title** and
    /// is never interpreted as an element. A deck whose first line is a real
    /// element silently loses it:
    ///
    /// ```text
    /// V1 1 0 10     <- consumed as the title, not a 10 V source
    /// R1 1 0 1k
    /// .end
    /// ```
    ///
    /// Prepend a title line (blank is fine) when building decks
    /// programmatically. [`Self::title`] reports what was consumed.
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        finish_non_aborting_parse(Self::parse_with_abort(input, &NoAbort))
    }

    /// Parse and immediately validate every authored output expression and
    /// output-symbol dependency.
    ///
    /// Ordinary [`Self::parse`] intentionally supports incomplete ASTs used by
    /// editors and synthetic-result evaluators. Strict execution frontends can
    /// use this convenience entry point to receive a typed semantic
    /// [`ParseError`] before circuit construction.
    pub fn parse_validated(input: &str) -> Result<Self, ParseError> {
        finish_non_aborting_parse(Self::parse_validated_with_abort(input, &NoAbort))
    }

    /// Parse and validate output requests with cooperative cancellation.
    pub fn parse_validated_with_abort(
        input: &str,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        Self::parse_validated_with_options_and_abort(input, NetlistParseOptions::default(), abort)
    }

    /// Parse a netlist from a string with cooperative cancellation.
    pub fn parse_with_abort(
        input: &str,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        Self::parse_with_options_and_abort(input, NetlistParseOptions::default(), abort)
    }

    /// Parse a netlist from a string with explicit parser options.
    pub fn parse_with_options(
        input: &str,
        options: NetlistParseOptions,
    ) -> Result<Self, ParseError> {
        finish_non_aborting_parse(Self::parse_with_options_and_abort(input, options, &NoAbort))
    }

    /// Parse with explicit options and validate output requests.
    pub fn parse_validated_with_options(
        input: &str,
        options: NetlistParseOptions,
    ) -> Result<Self, ParseError> {
        finish_non_aborting_parse(Self::parse_validated_with_options_and_abort(
            input, options, &NoAbort,
        ))
    }

    /// Parse with explicit options, validate output requests, and cooperatively
    /// observe cancellation throughout both phases.
    pub fn parse_validated_with_options_and_abort(
        input: &str,
        options: NetlistParseOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        let netlist = Self::parse_with_options_and_abort(input, options, abort)?;
        validate_output_requests_with_abort(&netlist, abort)?;
        Ok(netlist)
    }

    /// Parse a netlist from a string with explicit options and cooperative
    /// cancellation.
    pub fn parse_with_options_and_abort(
        input: &str,
        options: NetlistParseOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        Self::enforce_root_source_limits_with_abort(input, options.resource_limits, abort)?;
        let (sanitized, mut diagnostics, dispositions) =
            Self::sanitize_control_regions_with_abort(input, abort)?;
        let mut netlist = parser::parse_netlist_with_options_and_abort(&sanitized, options, abort)?;
        diagnostics.extend(netlist.diagnostics);
        netlist.diagnostics = diagnostics;
        netlist.control_dispositions = dispositions;
        if !netlist.spef_includes.is_empty() {
            return Err(ParseError::Syntax {
                line: 0,
                message: ".spef_include requires path-backed parsing so the annotation can be resolved and applied"
                    .to_owned(),
            }
            .into());
        }
        ensure_parse_not_aborted(abort)?;
        netlist.source_text = Some(input.to_string());
        netlist.source_path = None;
        netlist.replay_context = Some(NetlistReplayContext::InMemory);
        ensure_parse_not_aborted(abort)?;
        Ok(netlist)
    }

    /// Parse a netlist from a string with include resolution
    ///
    /// This method preprocesses .include and .lib directives using the specified
    /// file path to resolve relative paths. The process working directory is
    /// captured once as Xyce's final execution-directory fallback and is
    /// retained for deterministic source replay.
    pub fn parse_with_path(input: &str, file_path: &std::path::Path) -> Result<Self, ParseError> {
        finish_non_aborting_parse(Self::parse_with_path_and_abort(input, file_path, &NoAbort))
    }

    /// Parse with include resolution, validate output requests, and observe
    /// cooperative cancellation.
    pub fn parse_validated_with_path_and_abort(
        input: &str,
        file_path: &std::path::Path,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        Self::parse_validated_with_path_and_options_and_abort(
            input,
            file_path,
            NetlistParseOptions::default(),
            abort,
        )
    }

    /// Parse a netlist with include resolution and cooperative cancellation.
    pub fn parse_with_path_and_abort(
        input: &str,
        file_path: &std::path::Path,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        Self::parse_with_path_and_options_and_abort(
            input,
            file_path,
            NetlistParseOptions::default(),
            abort,
        )
    }

    /// Parse a netlist from a string with include resolution and parser options.
    pub fn parse_with_path_and_options(
        input: &str,
        file_path: &std::path::Path,
        options: NetlistParseOptions,
    ) -> Result<Self, ParseError> {
        finish_non_aborting_parse(Self::parse_with_path_and_options_and_abort(
            input, file_path, options, &NoAbort,
        ))
    }

    /// Parse with include resolution and explicit options, then validate
    /// output requests.
    pub fn parse_validated_with_path_and_options(
        input: &str,
        file_path: &std::path::Path,
        options: NetlistParseOptions,
    ) -> Result<Self, ParseError> {
        finish_non_aborting_parse(Self::parse_validated_with_path_and_options_and_abort(
            input, file_path, options, &NoAbort,
        ))
    }

    /// Parse with include resolution and explicit options, then validate
    /// output requests with cooperative cancellation.
    pub fn parse_validated_with_path_and_options_and_abort(
        input: &str,
        file_path: &std::path::Path,
        options: NetlistParseOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        let netlist =
            Self::parse_with_path_and_options_and_abort(input, file_path, options, abort)?;
        validate_output_requests_with_abort(&netlist, abort)?;
        Ok(netlist)
    }

    /// Parse a netlist with include resolution, explicit parser options, and
    /// cooperative cancellation.
    pub fn parse_with_path_and_options_and_abort(
        input: &str,
        file_path: &std::path::Path,
        options: NetlistParseOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        Self::parse_with_path_execution_dir_options_and_abort(
            input, file_path, None, options, abort,
        )
    }

    /// Parse a netlist using only an authenticated in-memory source bundle for
    /// every `.include`, `.inc`, and external `.lib` lookup.
    ///
    /// This is the browser-safe counterpart to
    /// [`Self::parse_with_path_and_options_and_abort`]. The supplied bundle
    /// must contain the root path and an authenticated edge for every external
    /// source directive; filesystem fallback is never attempted.
    pub fn parse_with_path_and_sealed_sources_and_options_and_abort(
        input: &str,
        file_path: &std::path::Path,
        sources: SealedSourceBundle,
        options: NetlistParseOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        let replay_sources = sources.clone();
        let include_processor = IncludeProcessor::new_sealed(file_path, sources.clone())
            .with_resource_limits(options.resource_limits);
        let mut initcond_resource_limits = options.resource_limits;
        initcond_resource_limits.max_dependency_source_bytes = initcond_resource_limits
            .max_dependency_source_bytes
            .min(MAX_DEVICE_INITIAL_CONDITION_SOURCE_BYTES);
        let initcond_source_provider = IncludeProcessor::new_sealed(file_path, sources)
            .with_resource_limits(initcond_resource_limits);
        Self::parse_with_source_providers_and_abort(
            input,
            file_path,
            options,
            abort,
            include_processor,
            initcond_source_provider,
            false,
            NetlistReplayContext::Sealed(replay_sources),
        )
    }

    /// Parse a netlist from source text with include resolution and an explicit
    /// execution directory.
    ///
    /// Xyce falls back to the execution directory after checking the including
    /// file and top-level netlist directories for nested includes. This entry
    /// point preserves the ordinary top-level path while allowing wrappers that
    /// execute a deck from another directory to model that search rule.
    pub fn parse_with_path_and_execution_dir(
        input: &str,
        file_path: &std::path::Path,
        execution_dir: &std::path::Path,
        options: NetlistParseOptions,
    ) -> Result<Self, ParseError> {
        finish_non_aborting_parse(Self::parse_with_path_and_execution_dir_and_abort(
            input,
            file_path,
            execution_dir,
            options,
            &NoAbort,
        ))
    }

    /// Parse a netlist using an explicit include execution directory and
    /// cooperative cancellation.
    pub fn parse_with_path_and_execution_dir_and_abort(
        input: &str,
        file_path: &std::path::Path,
        execution_dir: &std::path::Path,
        options: NetlistParseOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        Self::parse_with_path_execution_dir_options_and_abort(
            input,
            file_path,
            Some(execution_dir),
            options,
            abort,
        )
    }

    fn parse_with_path_execution_dir_options_and_abort(
        input: &str,
        file_path: &std::path::Path,
        execution_dir: Option<&std::path::Path>,
        options: NetlistParseOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        let default_execution_dir = if execution_dir.is_none() {
            Some(std::env::current_dir().map_err(ParseError::Io)?)
        } else {
            None
        };
        let execution_dir = execution_dir
            .or(default_execution_dir.as_deref())
            .expect("explicit or process execution directory is available");
        let include_processor =
            IncludeProcessor::new_with_execution_dir(file_path, Some(execution_dir))
                .with_resource_limits(options.resource_limits);
        let mut initcond_resource_limits = options.resource_limits;
        initcond_resource_limits.max_dependency_source_bytes = initcond_resource_limits
            .max_dependency_source_bytes
            .min(MAX_DEVICE_INITIAL_CONDITION_SOURCE_BYTES);
        let initcond_source_provider =
            IncludeProcessor::new_with_execution_dir(file_path, Some(execution_dir))
                .with_resource_limits(initcond_resource_limits);
        Self::parse_with_source_providers_and_abort(
            input,
            file_path,
            options,
            abort,
            include_processor,
            initcond_source_provider,
            true,
            NetlistReplayContext::PathWithExecutionDir(execution_dir.to_path_buf()),
        )
    }

    fn parse_with_source_providers_and_abort(
        input: &str,
        file_path: &std::path::Path,
        options: NetlistParseOptions,
        abort: &dyn AbortSignal,
        mut include_processor: IncludeProcessor,
        initcond_source_provider: IncludeProcessor,
        allow_filesystem_spef: bool,
        replay_context: NetlistReplayContext,
    ) -> Result<Self, ParseWithAbortError> {
        Self::enforce_root_source_limits_with_abort(input, options.resource_limits, abort)?;
        let expanded =
            include_processor.expand_content_mapped_with_abort(input, file_path, abort)?;
        let (sanitized, mut diagnostics, dispositions) =
            Self::sanitize_expanded_source_with_abort(expanded, abort)?;
        let mut netlist =
            parser::parse_expanded_netlist_with_options_and_abort(&sanitized, options, abort)?;
        diagnostics.extend(netlist.diagnostics);
        netlist.diagnostics = diagnostics;
        netlist.control_dispositions = dispositions;
        Self::normalize_model_string_paths_with_abort(&mut netlist, file_path, abort)?;
        Self::normalize_source_file_paths_with_abort(&mut netlist, file_path, abort)?;
        Self::normalize_measure_file_paths_with_abort(&mut netlist, file_path, abort)?;
        if !allow_filesystem_spef && !netlist.spef_includes.is_empty() {
            return Err(ParseError::Syntax {
                line: 0,
                message: "Authenticated sealed-source parsing cannot resolve .spef_include from the filesystem"
                    .to_owned(),
            }
            .into());
        }
        Self::apply_spef_includes_with_abort(
            &mut netlist,
            file_path,
            options.resource_limits,
            abort,
        )?;
        netlist.source_path = Some(file_path.to_path_buf());
        netlist
            .resolve_device_initial_condition_source_with_abort(&initcond_source_provider, abort)?;
        ensure_parse_not_aborted(abort)?;
        netlist.source_text = Some(input.to_string());
        netlist.replay_context = Some(replay_context);
        ensure_parse_not_aborted(abort)?;
        Ok(netlist)
    }

    /// Reparse a rewritten root source through the same resolver contract that
    /// produced this netlist.
    ///
    /// This is intentionally crate-private: callers must layer any AST-only
    /// overrides back onto the returned netlist before exposing it as a
    /// materialized analysis row.
    pub(crate) fn replay_root_source_with_options_and_abort(
        &self,
        input: &str,
        options: NetlistParseOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        match (&self.replay_context, self.source_path.as_deref()) {
            (Some(NetlistReplayContext::InMemory), _) | (None, None) => {
                Self::parse_with_options_and_abort(input, options, abort)
            }
            (Some(NetlistReplayContext::Sealed(sources)), Some(path)) => {
                Self::parse_with_path_and_sealed_sources_and_options_and_abort(
                    input,
                    path,
                    sources.clone(),
                    options,
                    abort,
                )
            }
            (Some(NetlistReplayContext::PathWithExecutionDir(execution_dir)), Some(path)) => {
                Self::parse_with_path_and_execution_dir_and_abort(
                    input,
                    path,
                    execution_dir,
                    options,
                    abort,
                )
            }
            (
                Some(NetlistReplayContext::SearchPaths {
                    paths: search_paths,
                    execution_dir,
                }),
                Some(path),
            ) => Self::parse_with_search_paths_execution_dir_options_and_abort(
                input,
                path,
                search_paths,
                execution_dir,
                options,
                abort,
            ),
            (None, Some(path)) => {
                Self::parse_with_path_and_options_and_abort(input, path, options, abort)
            }
            (Some(context), None) => Err(ParseError::Syntax {
                line: 0,
                message: format!(
                    "netlist replay context {context:?} requires a retained root source path"
                ),
            }
            .into()),
        }
    }

    /// Back-annotate every `.spef_include` referenced by the deck
    /// (paths resolve relative to the deck file).
    fn apply_spef_includes_with_abort(
        netlist: &mut Netlist,
        deck_path: &std::path::Path,
        resource_limits: crate::resource::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        if netlist.spef_includes.is_empty() {
            return Ok(());
        }
        if deck_path.as_os_str().is_empty() {
            return Err(ParseError::Syntax {
                line: 0,
                message: ".spef_include requires a path-backed root netlist; relative SPEF dependencies cannot be resolved from pathless source"
                    .to_owned(),
            }
            .into());
        }
        let base = deck_path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| std::path::Path::new("."));
        let mut retained_source_bytes = 0usize;
        for (index, entry) in netlist.spef_includes.clone().into_iter().enumerate() {
            poll_parse_abort(abort, index)?;
            let candidate = std::path::Path::new(&entry);
            let path = if candidate.is_absolute() {
                candidate.to_path_buf()
            } else {
                base.join(candidate)
            };
            let (content, source_bytes) = read_file_with_encoding_limited_with_abort(
                &path,
                crate::resource::ResourceKind::DependencySourceBytes,
                retained_source_bytes,
                resource_limits.max_dependency_source_bytes,
                abort,
            )
            .map_err(|error| {
                map_abort_parse_error(error, |error| match error {
                    error @ ParseError::ResourceLimit(_) => error,
                    error => ParseError::Syntax {
                        line: 0,
                        message: format!("failed to read SPEF file `{}`: {error}", path.display()),
                    },
                })
            })?;
            retained_source_bytes = retained_source_bytes.saturating_add(source_bytes);
            let contextualize = |error: ParseWithAbortError| match error {
                ParseWithAbortError::Aborted => ParseWithAbortError::Aborted,
                ParseWithAbortError::Parse(ParseError::ResourceLimit(error)) => {
                    ParseError::ResourceLimit(error).into()
                }
                ParseWithAbortError::Parse(ParseError::Syntax { line, message }) => {
                    ParseError::Syntax {
                        line,
                        message: format!("SPEF `{}`: {message}", path.display()),
                    }
                    .into()
                }
                ParseWithAbortError::Parse(error) => ParseError::Syntax {
                    line: 0,
                    message: format!("SPEF `{}`: {error}", path.display()),
                }
                .into(),
            };
            let parasitics =
                spef::SpefFile::parse_with_abort(&content, abort).map_err(&contextualize)?;
            let report = parasitics
                .apply_path_backed_with_abort(netlist, abort)
                .map_err(contextualize)?;
            log::info!(
                "SPEF `{}`: {} net(s), {} pin(s) rewired ({} skipped), {} R + {} L + {} C added",
                path.display(),
                report.nets,
                report.rewired_pins,
                report.skipped_pins,
                report.resistors,
                report.inductors,
                report.capacitors
            );
        }
        ensure_parse_not_aborted(abort)?;
        Ok(())
    }

    /// Parse a netlist from a file with include expansion
    pub fn parse_file(path: &std::path::Path) -> Result<Self, ParseError> {
        Self::parse_file_with_options(path, NetlistParseOptions::default())
    }

    /// Parse a netlist file with an explicit parsing and resource policy.
    pub fn parse_file_with_options(
        path: &std::path::Path,
        options: NetlistParseOptions,
    ) -> Result<Self, ParseError> {
        finish_non_aborting_parse(Self::parse_file_with_options_and_abort(
            path, options, &NoAbort,
        ))
    }

    /// Parse a netlist file with explicit options and cooperative cancellation.
    pub fn parse_file_with_options_and_abort(
        path: &std::path::Path,
        options: NetlistParseOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        let (content, _) = read_file_with_encoding_limited_with_abort(
            path,
            crate::resource::ResourceKind::NetlistBytes,
            0,
            options.resource_limits.max_netlist_bytes,
            abort,
        )?;
        Self::parse_with_path_and_options_and_abort(&content, path, options, abort)
    }

    /// Read a deck file with the same encoding handling `parse_file` uses
    /// (UTF-8 with fallbacks), without parsing — for callers that
    /// preprocess the text first (multi-run expansion).
    pub fn read_source(path: &std::path::Path) -> Result<String, ParseError> {
        Self::read_source_with_options(path, NetlistParseOptions::default())
    }

    /// Read and decode a root source file under an explicit byte policy.
    pub fn read_source_with_options(
        path: &std::path::Path,
        options: NetlistParseOptions,
    ) -> Result<String, ParseError> {
        finish_non_aborting_parse(Self::read_source_with_options_and_abort(
            path, options, &NoAbort,
        ))
    }

    /// Read and decode a root source file under an explicit byte policy while
    /// observing cooperative cancellation between bounded read chunks.
    pub fn read_source_with_options_and_abort(
        path: &std::path::Path,
        options: NetlistParseOptions,
        abort: &dyn AbortSignal,
    ) -> Result<String, ParseWithAbortError> {
        read_file_with_encoding_limited_with_abort(
            path,
            crate::resource::ResourceKind::NetlistBytes,
            0,
            options.resource_limits.max_netlist_bytes,
            abort,
        )
        .map(|(source, _)| source)
    }

    /// Parse a netlist from a file with additional include search directories
    ///
    /// Like [`Netlist::parse_file`], but `.include`/`.lib` references that do
    /// not resolve relative to the including file are also searched in
    /// `search_paths`, in order.
    pub fn parse_file_with_search_paths(
        path: &std::path::Path,
        search_paths: &[std::path::PathBuf],
    ) -> Result<Self, ParseError> {
        Self::parse_file_with_search_paths_and_options(
            path,
            search_paths,
            NetlistParseOptions::default(),
        )
    }

    /// Parse a file with include search paths and an explicit resource policy.
    pub fn parse_file_with_search_paths_and_options(
        path: &std::path::Path,
        search_paths: &[std::path::PathBuf],
        options: NetlistParseOptions,
    ) -> Result<Self, ParseError> {
        finish_non_aborting_parse(Self::parse_file_with_search_paths_and_options_and_abort(
            path,
            search_paths,
            options,
            &NoAbort,
        ))
    }

    /// Parse a file with search paths, explicit options, and cancellation.
    pub fn parse_file_with_search_paths_and_options_and_abort(
        path: &std::path::Path,
        search_paths: &[std::path::PathBuf],
        options: NetlistParseOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        let (content, _) = read_file_with_encoding_limited_with_abort(
            path,
            crate::resource::ResourceKind::NetlistBytes,
            0,
            options.resource_limits.max_netlist_bytes,
            abort,
        )?;
        Self::parse_with_search_paths_and_options_and_abort(
            &content,
            path,
            search_paths,
            options,
            abort,
        )
    }

    /// Parse source text with search paths and explicit parsing options.
    ///
    /// The process working directory is captured once as Xyce's execution-
    /// directory fallback and retained alongside the search paths for replay.
    pub fn parse_with_search_paths_and_options(
        input: &str,
        path: &std::path::Path,
        search_paths: &[std::path::PathBuf],
        options: NetlistParseOptions,
    ) -> Result<Self, ParseError> {
        finish_non_aborting_parse(Self::parse_with_search_paths_and_options_and_abort(
            input,
            path,
            search_paths,
            options,
            &NoAbort,
        ))
    }

    /// Parse source text with search paths, explicit options, and cancellation.
    pub fn parse_with_search_paths_and_options_and_abort(
        input: &str,
        path: &std::path::Path,
        search_paths: &[std::path::PathBuf],
        options: NetlistParseOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        let execution_dir = std::env::current_dir().map_err(ParseError::Io)?;
        Self::parse_with_search_paths_execution_dir_options_and_abort(
            input,
            path,
            search_paths,
            &execution_dir,
            options,
            abort,
        )
    }

    fn parse_with_search_paths_execution_dir_options_and_abort(
        input: &str,
        path: &std::path::Path,
        search_paths: &[std::path::PathBuf],
        execution_dir: &std::path::Path,
        options: NetlistParseOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        Self::enforce_root_source_limits_with_abort(input, options.resource_limits, abort)?;
        let mut processor = IncludeProcessor::new_with_execution_dir(path, Some(execution_dir))
            .with_resource_limits(options.resource_limits);
        for (index, dir) in search_paths.iter().enumerate() {
            poll_parse_abort(abort, index)?;
            processor.add_lib_path(dir.clone());
        }
        let expanded = processor.expand_content_mapped_with_abort(input, path, abort)?;
        let (sanitized, mut diagnostics, dispositions) =
            Self::sanitize_expanded_source_with_abort(expanded, abort)?;
        let mut netlist =
            parser::parse_expanded_netlist_with_options_and_abort(&sanitized, options, abort)?;
        diagnostics.extend(netlist.diagnostics);
        netlist.diagnostics = diagnostics;
        netlist.control_dispositions = dispositions;
        Self::normalize_model_string_paths_with_abort(&mut netlist, path, abort)?;
        Self::normalize_source_file_paths_with_abort(&mut netlist, path, abort)?;
        Self::normalize_measure_file_paths_with_abort(&mut netlist, path, abort)?;
        Self::apply_spef_includes_with_abort(&mut netlist, path, options.resource_limits, abort)?;
        netlist.source_path = Some(path.to_path_buf());
        let mut initcond_resource_limits = options.resource_limits;
        initcond_resource_limits.max_dependency_source_bytes = initcond_resource_limits
            .max_dependency_source_bytes
            .min(MAX_DEVICE_INITIAL_CONDITION_SOURCE_BYTES);
        let initcond_source_provider =
            IncludeProcessor::new_with_execution_dir(path, Some(execution_dir))
                .with_resource_limits(initcond_resource_limits);
        netlist
            .resolve_device_initial_condition_source_with_abort(&initcond_source_provider, abort)?;
        ensure_parse_not_aborted(abort)?;
        netlist.source_text = Some(input.to_string());
        netlist.replay_context = Some(NetlistReplayContext::SearchPaths {
            paths: search_paths.to_vec(),
            execution_dir: execution_dir.to_path_buf(),
        });
        ensure_parse_not_aborted(abort)?;
        Ok(netlist)
    }

    /// Preprocess netlist content to expand .include and .lib directives
    ///
    /// This method expands all .include and .lib directives in the content,
    /// resolving paths relative to the given file path and falling back to the
    /// process working directory after the including-file and top-level paths.
    pub fn preprocess_includes(
        content: &str,
        file_path: &std::path::Path,
    ) -> Result<String, ParseError> {
        finish_non_aborting_parse(Self::preprocess_includes_with_abort(
            content, file_path, &NoAbort,
        ))
    }

    /// Expand `.include` and `.lib` directives with cooperative cancellation.
    pub fn preprocess_includes_with_abort(
        content: &str,
        file_path: &std::path::Path,
        abort: &dyn AbortSignal,
    ) -> Result<String, ParseWithAbortError> {
        Self::preprocess_includes_with_execution_dir_and_abort(content, file_path, None, abort)
    }

    fn preprocess_includes_with_execution_dir_and_abort(
        content: &str,
        file_path: &std::path::Path,
        execution_dir: Option<&std::path::Path>,
        abort: &dyn AbortSignal,
    ) -> Result<String, ParseWithAbortError> {
        Self::preprocess_includes_mapped_with_execution_dir_and_abort(
            content,
            file_path,
            execution_dir,
            abort,
        )
        .map(|expanded| expanded.render())
    }

    fn preprocess_includes_mapped_with_execution_dir_and_abort(
        content: &str,
        file_path: &std::path::Path,
        execution_dir: Option<&std::path::Path>,
        abort: &dyn AbortSignal,
    ) -> Result<include::ExpandedSource, ParseWithAbortError> {
        let default_execution_dir = if execution_dir.is_none() {
            Some(std::env::current_dir().map_err(ParseError::Io)?)
        } else {
            None
        };
        let execution_dir = execution_dir
            .or(default_execution_dir.as_deref())
            .expect("explicit or process execution directory is available");
        let mut processor =
            IncludeProcessor::new_with_execution_dir(file_path, Some(execution_dir));
        processor.expand_content_mapped_with_abort(content, file_path, abort)
    }

    /// Strip .control/.endc blocks from netlist
    ///
    /// Ngspice uses .control blocks for scripting (variable assignment, loops,
    /// conditionals). These contain operators like '>' that break the netlist
    /// parser. We strip them since RSpice runs the circuit directly.
    ///
    /// This entry point sanitizes only. The parse pipeline uses
    /// [`Self::sanitize_control_regions_with_abort`], which also promotes the
    /// commands the engine can honour and records what became of each one.
    pub fn strip_control_blocks(input: &str) -> Result<String, ParseError> {
        finish_non_aborting_parse(Self::emit_sanitized_control_source(input, &[], &NoAbort))
    }

    /// Read every `.control` region once, then emit the deck the parser sees.
    ///
    /// The promotions, the per-command dispositions, and the sanitized source
    /// all come out of the same interpretation of the region, so the deck that
    /// runs and the disposition the editor reports cannot disagree about what
    /// happened to a given line.
    fn sanitize_control_regions_with_abort(
        input: &str,
        abort: &dyn AbortSignal,
    ) -> Result<(String, Vec<ParseDiagnostic>, Vec<ControlCommandRecord>), ParseWithAbortError>
    {
        let mut walk = ControlRegionWalk::default();
        for (line_index, line) in input.lines().enumerate() {
            poll_parse_abort(abort, line_index)?;
            walk.observe(line, line_index + 1, None);
        }
        let (promoted, dispositions) = walk.finish();
        let promoted = promoted
            .into_iter()
            .map(|command| command.text)
            .collect::<Vec<_>>();
        let sanitized = Self::emit_sanitized_control_source(input, &promoted, abort)?;
        let diagnostics = control_disposition_diagnostics(&dispositions);
        ensure_parse_not_aborted(abort)?;
        Ok((sanitized, diagnostics, dispositions))
    }

    /// Comment out every control-region line and splice the promoted
    /// directives in ahead of `.end`, exactly where an author would have
    /// written them.
    fn emit_sanitized_control_source(
        input: &str,
        promoted: &[String],
        abort: &dyn AbortSignal,
    ) -> Result<String, ParseWithAbortError> {
        let mut result = String::with_capacity(
            input.len()
                + promoted
                    .iter()
                    .map(|command| command.len() + 1)
                    .sum::<usize>(),
        );
        let mut in_control = false;
        let mut opened_at_line = None;
        let mut inserted = false;

        for (line_index, line) in input.lines().enumerate() {
            poll_parse_abort(abort, line_index)?;
            let line_num = line_index + 1;
            let trimmed = line.trim();
            let head = trimmed.split_whitespace().next().unwrap_or("");

            if head.eq_ignore_ascii_case(".control") {
                in_control = true;
                opened_at_line = Some(line_num);
            } else if head.eq_ignore_ascii_case(".endc") {
                if !in_control {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: ".ENDC without matching .CONTROL".to_string(),
                    }
                    .into());
                }
                in_control = false;
                opened_at_line = None;
            } else if !in_control {
                if !inserted && head.eq_ignore_ascii_case(".end") {
                    for command in promoted {
                        result.push_str(command);
                        result.push('\n');
                    }
                    inserted = true;
                }
                result.push_str(line);
                result.push('\n');
                continue;
            }

            // Boundaries and body alike survive as comments so the sanitized
            // deck keeps the same line count as the source it came from.
            result.push_str("* ");
            result.push_str(line);
            result.push('\n');
        }

        if let Some(line) = opened_at_line {
            return Err(ParseError::Syntax {
                line,
                message: ".CONTROL without a matching .ENDC".to_string(),
            }
            .into());
        }

        if !inserted {
            for (index, command) in promoted.iter().enumerate() {
                poll_parse_abort(abort, index)?;
                result.push_str(command);
                result.push('\n');
            }
        }

        ensure_parse_not_aborted(abort)?;
        Ok(result)
    }

    fn sanitize_expanded_source_with_abort(
        expanded: include::ExpandedSource,
        abort: &dyn AbortSignal,
    ) -> Result<
        (
            include::ExpandedSource,
            Vec<ParseDiagnostic>,
            Vec<ControlCommandRecord>,
        ),
        ParseWithAbortError,
    > {
        let implicit_title = expanded.implicit_title().map(str::to_owned);
        let mut walk = ControlRegionWalk::default();
        for (index, item) in expanded.items.iter().enumerate() {
            poll_parse_abort(abort, index)?;
            let include::ExpandedSourceItem::Line { text, origin } = item else {
                continue;
            };
            poll_parse_text(abort, text)?;
            walk.observe(text, origin.line, Some(origin));
        }
        let (promoted, dispositions) = walk.finish();
        let promoted = promoted
            .into_iter()
            .map(|command| {
                let origin = command
                    .origin
                    .expect("expanded control commands carry a source origin");
                (command.text, origin)
            })
            .collect::<Vec<_>>();

        let mut output = include::ExpandedSource::default();
        if let Some(title) = implicit_title {
            output.set_implicit_title(title);
        }
        let diagnostics = control_disposition_diagnostics(&dispositions);
        let mut in_control = false;
        let mut opened_at = None;
        let mut inserted = false;
        for (index, item) in expanded.items.into_iter().enumerate() {
            poll_parse_abort(abort, index)?;
            match item {
                include::ExpandedSourceItem::Line { text, origin } => {
                    poll_parse_text(abort, &text)?;
                    let trimmed = text.trim();
                    let head = trimmed.split_whitespace().next().unwrap_or("");
                    if head.eq_ignore_ascii_case(".control") {
                        in_control = true;
                        opened_at = Some(origin.clone());
                        output.items.push(include::ExpandedSourceItem::Line {
                            text: format!("* {text}"),
                            origin,
                        });
                        continue;
                    }
                    if head.eq_ignore_ascii_case(".endc") {
                        if !in_control {
                            return Err(ParseError::Syntax {
                                line: origin.line,
                                message: ".ENDC without matching .CONTROL".to_string(),
                            }
                            .into());
                        }
                        in_control = false;
                        opened_at = None;
                        output.items.push(include::ExpandedSourceItem::Line {
                            text: format!("* {text}"),
                            origin,
                        });
                        continue;
                    }
                    if in_control {
                        output.items.push(include::ExpandedSourceItem::Line {
                            text: format!("* {text}"),
                            origin,
                        });
                        continue;
                    }
                    if !inserted && head.eq_ignore_ascii_case(".end") {
                        for (command, command_origin) in &promoted {
                            output.items.push(include::ExpandedSourceItem::Line {
                                text: command.clone(),
                                origin: command_origin.clone(),
                            });
                        }
                        inserted = true;
                    }
                    output
                        .items
                        .push(include::ExpandedSourceItem::Line { text, origin });
                }
                event => output.items.push(event),
            }
        }

        if let Some(origin) = opened_at {
            return Err(ParseError::Syntax {
                line: origin.line,
                message: ".CONTROL without a matching .ENDC".to_string(),
            }
            .into());
        }

        if !inserted && !promoted.is_empty() {
            let insertion = output
                .items
                .iter()
                .rposition(|item| matches!(item, include::ExpandedSourceItem::ExitSource { .. }))
                .unwrap_or(output.items.len());
            output.items.splice(
                insertion..insertion,
                promoted
                    .into_iter()
                    .map(|(text, origin)| include::ExpandedSourceItem::Line { text, origin }),
            );
        }
        ensure_parse_not_aborted(abort)?;
        Ok((output, diagnostics, dispositions))
    }

    fn promoted_control_commands_for_line(
        line: &str,
        scalar_lets: &mut ControlScalarLets,
    ) -> Vec<String> {
        let mut promoted = Vec::new();
        if let Some(command) = Self::promote_control_netlist_command(line, scalar_lets) {
            promoted.push(command);
        }
        if let Some(command) = Self::promote_control_esave_command(line) {
            promoted.push(command);
        }
        if let Some(command) = Self::promote_control_codemodel_command(line) {
            promoted.push(command);
        }
        if let Some(command) = Self::promote_control_set_command(line) {
            promoted.push(command);
        }
        if let Some(command) = Self::promote_control_option_command(line) {
            promoted.push(command);
        }
        if let Some(command) = Self::promote_control_auto_bridge_set_command(line) {
            promoted.push(command);
        }
        if let Some(command) = Self::promote_control_auto_bridge_param_set_command(line) {
            promoted.push(command);
        }
        if let Some(command) = Self::promote_control_no_auto_bridge_family_set_command(line) {
            promoted.push(command);
        }
        promoted
    }

    fn promote_control_netlist_command(
        line: &str,
        scalar_lets: &mut ControlScalarLets,
    ) -> Option<String> {
        let body = strip_control_inline_comment(line).trim();
        if body.is_empty() || body.starts_with('*') {
            return None;
        }

        let body = body.strip_prefix('.').unwrap_or(body);
        let mut parts = body.split_whitespace();
        let command = parts.next()?;
        let args: Vec<&str> = parts.collect();
        let promoted_command = match command.to_ascii_lowercase().as_str() {
            "op" => ".op",
            "dc" => ".dc",
            "ac" => ".ac",
            "sp" => ".sp",
            "tran" => ".tran",
            "save" => ".save",
            "meas" => {
                return Self::promote_control_measure_command(".meas", &args, scalar_lets);
            }
            "measure" => {
                return Self::promote_control_measure_command(".measure", &args, scalar_lets);
            }
            _ => return None,
        };

        let mut promoted = String::from(promoted_command);
        for part in args {
            promoted.push(' ');
            promoted.push_str(&normalize_control_analysis_token(part, scalar_lets));
        }
        Some(promoted)
    }

    fn promote_control_measure_command(
        command: &str,
        args: &[&str],
        scalar_lets: &mut ControlScalarLets,
    ) -> Option<String> {
        if args.len() < 4 {
            return None;
        }

        let measure_type = args[2].to_ascii_lowercase();
        if !matches!(
            measure_type.as_str(),
            "avg" | "max" | "min" | "pp" | "rms" | "integ"
        ) {
            return None;
        }

        let mut promoted = String::from(command);
        for part in args {
            promoted.push(' ');
            promoted.push_str(&normalize_control_measure_token(part, scalar_lets));
        }
        Some(promoted)
    }

    fn promote_control_esave_command(line: &str) -> Option<String> {
        let body = strip_control_inline_comment(line).trim();
        if body.is_empty() || body.starts_with('*') {
            return None;
        }

        let body = body.strip_prefix('.').unwrap_or(body);
        let mut parts = body.split_whitespace();
        let command = parts.next()?;
        if !command.eq_ignore_ascii_case("esave") {
            return None;
        }

        match parts.next()?.to_ascii_lowercase().as_str() {
            "none" => Some(".options xspice_esave=0".to_string()),
            "all" => Some(".options xspice_esave=1".to_string()),
            _ => None,
        }
    }

    fn promote_control_codemodel_command(line: &str) -> Option<String> {
        let body = strip_control_inline_comment(line).trim();
        if body.is_empty() || body.starts_with('*') {
            return None;
        }

        let body = body.strip_prefix('.').unwrap_or(body);
        let mut parts = body.splitn(2, char::is_whitespace);
        let command = parts.next()?;
        if !command.eq_ignore_ascii_case("codemodel") {
            return None;
        }

        let args = parts.next().unwrap_or("").trim();
        Some(if args.is_empty() {
            ".CODEMODEL".to_string()
        } else {
            format!(".CODEMODEL {args}")
        })
    }

    fn promote_control_set_command(line: &str) -> Option<String> {
        let body = strip_control_inline_comment(line).trim();
        if body.is_empty() || body.starts_with('*') {
            return None;
        }

        let body = body.strip_prefix('.').unwrap_or(body);
        let mut parts = body.splitn(2, char::is_whitespace);
        let command = parts.next()?;
        if !command.eq_ignore_ascii_case("set") {
            return None;
        }

        let assignments = parts.next().unwrap_or("");
        if let Some(value) = control_set_value(assignments, "digital_delay_type") {
            return Some(format!(".options digital_delay_type={value}"));
        }
        if let Some(value) = control_set_value(assignments, "xtrtol") {
            return Some(format!(".options trtol={value}"));
        }
        None
    }

    fn promote_control_option_command(line: &str) -> Option<String> {
        let body = strip_control_inline_comment(line).trim();
        if body.is_empty() || body.starts_with('*') {
            return None;
        }

        let body = body.strip_prefix('.').unwrap_or(body);
        let mut parts = body.splitn(2, char::is_whitespace);
        let command = parts.next()?;
        if !matches!(command.to_ascii_lowercase().as_str(), "option" | "options") {
            return None;
        }

        let assignments = parts.next().unwrap_or("");
        let mut promoted = Vec::new();
        if let Some(value) = control_set_value(assignments, "trtol") {
            promoted.push(format!("trtol={value}"));
        }
        if let Some(value) = control_set_value(assignments, "xmu") {
            promoted.push(format!("xmu={value}"));
        }
        (!promoted.is_empty()).then(|| format!(".options {}", promoted.join(" ")))
    }

    fn promote_control_auto_bridge_set_command(line: &str) -> Option<String> {
        let body = strip_control_inline_comment(line).trim();
        if body.is_empty() || body.starts_with('*') {
            return None;
        }

        let body = body.strip_prefix('.').unwrap_or(body);
        let mut parts = body.splitn(2, char::is_whitespace);
        let command = parts.next()?;
        if !command.eq_ignore_ascii_case("set") {
            return None;
        }

        let (key, setup_card, device_card, max_nodes) =
            control_auto_bridge_template_assignment(parts.next().unwrap_or(""))?;
        let max_nodes = max_nodes.unwrap_or(0);
        Some(format!(
            ".RSPICE_AUTO_BRIDGE_TEMPLATE {} {} {} {}",
            control_hex_encode(&key),
            control_hex_encode(&setup_card),
            control_hex_encode(&device_card),
            max_nodes
        ))
    }

    fn promote_control_auto_bridge_param_set_command(line: &str) -> Option<String> {
        let body = strip_control_inline_comment(line).trim();
        if body.is_empty() || body.starts_with('*') {
            return None;
        }

        let body = body.strip_prefix('.').unwrap_or(body);
        let mut parts = body.splitn(2, char::is_whitespace);
        let command = parts.next()?;
        if !command.eq_ignore_ascii_case("set") {
            return None;
        }

        let (node_type, param_name) =
            control_auto_bridge_param_assignment(parts.next().unwrap_or(""))?;
        Some(format!(
            ".RSPICE_AUTO_BRIDGE_PARAM {} {}",
            control_hex_encode(&node_type),
            control_hex_encode(&param_name)
        ))
    }

    fn promote_control_no_auto_bridge_family_set_command(line: &str) -> Option<String> {
        let body = strip_control_inline_comment(line).trim();
        if body.is_empty() || body.starts_with('*') {
            return None;
        }

        let body = body.strip_prefix('.').unwrap_or(body);
        let mut parts = body.splitn(2, char::is_whitespace);
        let command = parts.next()?;
        if !command.eq_ignore_ascii_case("set") {
            return None;
        }

        let no_family = control_no_auto_bridge_family_setting(parts.next().unwrap_or(""))?;
        Some(format!(
            ".RSPICE_AUTO_BRIDGE_FAMILY {}",
            usize::from(!no_family)
        ))
    }

    fn normalize_model_string_paths_with_abort(
        &mut self,
        file_path: &std::path::Path,
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        let Some(base_dir) = file_path.parent() else {
            return Ok(());
        };

        for (model_index, model) in self.models.iter_mut().enumerate() {
            poll_parse_abort(abort, model_index)?;
            for (param_index, (name, value)) in model.string_params.iter_mut().enumerate() {
                poll_parse_abort(abort, param_index)?;
                *value = normalize_model_string_path_value(name, value, Some(base_dir));
            }
        }
        ensure_parse_not_aborted(abort)
    }

    fn normalize_source_file_paths_with_abort(
        &mut self,
        file_path: &std::path::Path,
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        let Some(base_dir) = file_path.parent() else {
            return Ok(());
        };

        for (index, element) in self.elements.iter_mut().enumerate() {
            poll_parse_abort(abort, index)?;
            match &mut element.kind {
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                    normalize_source_spec_file_paths(spec, base_dir);
                }
                _ => {}
            }
        }
        ensure_parse_not_aborted(abort)
    }

    fn normalize_measure_file_paths_with_abort(
        &mut self,
        file_path: &std::path::Path,
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        let Some(base_dir) = file_path.parent() else {
            return Ok(());
        };
        for (index, measurement) in self.measurements.iter_mut().enumerate() {
            poll_parse_abort(abort, index)?;
            let crate::netlist::measure::MeasureType::FileError { file, .. } =
                &mut measurement.measure_type
            else {
                continue;
            };
            if file.contains("://") {
                continue;
            }
            let candidate = Path::new(file);
            if !candidate.is_absolute() {
                *file = base_dir.join(candidate).to_string_lossy().into_owned();
            }
        }
        ensure_parse_not_aborted(abort)
    }

    /// Check if a node is global
    pub fn is_global(&self, node: &str) -> bool {
        self.global_nodes.contains(&node.to_uppercase())
    }
}

fn normalize_source_spec_file_paths(spec: &mut SourceSpec, source_base_dir: &Path) {
    match spec {
        SourceSpec::RfPort { inner, .. } => {
            normalize_source_spec_file_paths(inner, source_base_dir);
        }
        SourceSpec::PwlFile { path, .. } => {
            let candidate = Path::new(path);
            if !candidate.is_absolute() {
                *path = source_base_dir
                    .join(candidate)
                    .to_string_lossy()
                    .into_owned();
            }
        }
        SourceSpec::DcTransient { transient, .. } | SourceSpec::DcAcTransient { transient, .. } => {
            normalize_source_spec_file_paths(transient, source_base_dir);
        }
        _ => {}
    }
}

pub(crate) fn normalize_model_string_path_value(
    name: &str,
    value: &str,
    source_base_dir: Option<&Path>,
) -> String {
    let Some(base_dir) = source_base_dir else {
        return value.to_string();
    };
    if !model_string_param_resolves_relative(name, value) {
        return value.to_string();
    }

    let (path_value, suffix) = split_process_file_suffix(name, value);
    if path_value.trim().is_empty() || path_value.contains("://") {
        return value.to_string();
    }

    let candidate = Path::new(path_value);
    if candidate.is_absolute() {
        return value.to_string();
    }

    let mut resolved = base_dir.join(candidate).to_string_lossy().into_owned();
    resolved.push_str(suffix);
    resolved
}

fn model_string_param_resolves_relative(name: &str, value: &str) -> bool {
    let normalized = name.trim().to_ascii_lowercase();
    if normalized == "simulation" {
        return model_string_value_looks_path_like(value);
    }
    normalized.ends_with("file")
        || normalized.ends_with("_file")
        || normalized.ends_with("path")
        || matches!(normalized.as_str(), "fxpdata" | "fxmdata")
}

fn model_string_value_looks_path_like(value: &str) -> bool {
    let trimmed = value.trim();
    let lowered = trimmed.to_ascii_lowercase();
    trimmed.starts_with('.')
        || trimmed.contains('/')
        || trimmed.contains('\\')
        || lowered.ends_with(".dll")
        || lowered.ends_with(".so")
        || lowered.ends_with(".dylib")
}

fn split_process_file_suffix<'a>(name: &str, value: &'a str) -> (&'a str, &'a str) {
    if !name.eq_ignore_ascii_case("process_file") {
        return (value, "");
    }
    if let Some(base) = value.strip_suffix("||") {
        (base, "||")
    } else if let Some(base) = value.strip_suffix('|') {
        (base, "|")
    } else {
        (value, "")
    }
}

/// One directive lifted out of a `.control` region, with the location of the
/// command that produced it.
struct PromotedControlCommand {
    text: String,
    origin: Option<NetlistSourceLocation>,
}

/// Scalar `let` assignments a control region declared, and which of them
/// actually reached a promoted directive.
///
/// A `let` that nothing promotes is dropped like any other scripting line, so
/// "the parser understood this assignment" is not the same claim as "this
/// assignment survived into the deck". Only the second is worth telling a user,
/// which is why the values track their own use.
#[derive(Default)]
struct ControlScalarLets {
    /// Assigned name (upper-cased) to its fully expanded value.
    values: HashMap<String, String>,
    /// Names each definition inlined, so using one closes over its sources.
    references: HashMap<String, Vec<String>>,
    /// Names read while building a promoted directive spelling.
    used: HashSet<String>,
}

impl ControlScalarLets {
    fn define(&mut self, name: String, expression: &str) {
        let mut referenced = Vec::new();
        let expanded = expand_control_scalar_expression(expression, &self.values, &mut referenced);
        self.references.insert(name.clone(), referenced);
        self.values.insert(name, expanded);
    }

    /// Read a value for substitution into a promoted directive, recording the
    /// read.
    fn consume(&mut self, name: &str) -> Option<String> {
        let value = self.values.get(name)?.clone();
        self.used.insert(name.to_owned());
        Some(value)
    }

    /// Every name whose value reached a promoted directive, directly or
    /// through another assignment that did.
    fn consumed_names(&self) -> HashSet<String> {
        let mut pending = self.used.iter().cloned().collect::<Vec<_>>();
        let mut closed = HashSet::new();
        while let Some(name) = pending.pop() {
            if !closed.insert(name.clone()) {
                continue;
            }
            if let Some(referenced) = self.references.get(&name) {
                pending.extend(referenced.iter().cloned());
            }
        }
        closed
    }
}

/// One interpretation of every `.control` region in a deck.
///
/// Both parse pipelines — the in-memory one and the include-expanded one —
/// drive this walk, so the promoted directives and the per-command
/// dispositions come from a single reading of the region rather than from two
/// traversals that could drift apart.
#[derive(Default)]
struct ControlRegionWalk {
    in_control: bool,
    lets: ControlScalarLets,
    promoted: Vec<PromotedControlCommand>,
    dispositions: Vec<ControlCommandRecord>,
    /// Indices into `dispositions` of scalar `let` lines, with the name each
    /// one assigned. Whether the assignment was consumed is only known once
    /// the whole region has been read.
    pending_lets: Vec<(usize, String)>,
}

impl ControlRegionWalk {
    fn observe(&mut self, text: &str, line: usize, origin: Option<&NetlistSourceLocation>) {
        let head = text.split_whitespace().next().unwrap_or("");
        if head.eq_ignore_ascii_case(".control") {
            self.in_control = true;
            return;
        }
        if head.eq_ignore_ascii_case(".endc") {
            self.in_control = false;
            return;
        }
        if !self.in_control {
            return;
        }

        let body = strip_control_inline_comment(text).trim();
        // `*`, `;` and `$` all open a comment in an ngspice control block, and
        // a comment is not a command that could have had a disposition.
        if body.is_empty() || body.starts_with('*') || body.starts_with('$') {
            return;
        }
        let command = body
            .strip_prefix('.')
            .unwrap_or(body)
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_owned();

        let assignment = control_scalar_let_assignment(text);
        if let Some((name, expression)) = &assignment {
            self.lets.define(name.clone(), expression);
        }
        let directives = Netlist::promoted_control_commands_for_line(text, &mut self.lets);

        let disposition = if directives.is_empty() {
            // Resolved in `finish` once the region says whether anything read
            // the assignment.
            ControlCommandDisposition::Dropped
        } else {
            ControlCommandDisposition::Promoted {
                directives: directives.clone(),
            }
        };
        if directives.is_empty()
            && let Some((name, _)) = assignment
        {
            self.pending_lets.push((self.dispositions.len(), name));
        }
        self.promoted
            .extend(directives.into_iter().map(|text| PromotedControlCommand {
                text,
                origin: origin.cloned(),
            }));
        self.dispositions.push(ControlCommandRecord {
            line,
            origin: origin.cloned(),
            command,
            disposition,
        });
    }

    fn finish(mut self) -> (Vec<PromotedControlCommand>, Vec<ControlCommandRecord>) {
        let consumed = self.lets.consumed_names();
        for (index, name) in self.pending_lets {
            if consumed.contains(&name) {
                self.dispositions[index].disposition =
                    ControlCommandDisposition::ConsumedByPromotion { name };
            }
        }
        (self.promoted, self.dispositions)
    }
}

/// One warning per control command the parser ignored.
///
/// A promotion is not a loss, so it stays out of the diagnostic stream and is
/// reported only through [`Netlist::control_dispositions`];
/// [`DiagnosticSeverity`] has no informational level to demote it to, and
/// widening that enum for one producer would recolour every existing consumer.
fn control_disposition_diagnostics(records: &[ControlCommandRecord]) -> Vec<ParseDiagnostic> {
    records
        .iter()
        .filter(|record| matches!(record.disposition, ControlCommandDisposition::Dropped))
        .map(|record| {
            let message = format!(
                "control command '{}' ignored; .control scripting is not executed, so whatever \
                 it requests (output, plotting, control flow) will not run",
                record.command
            );
            match &record.origin {
                Some(origin) => {
                    ParseDiagnostic::warning_at(origin.clone(), "control-command-dropped", message)
                }
                None => ParseDiagnostic::warning(record.line, "control-command-dropped", message),
            }
        })
        .collect()
}

fn strip_control_inline_comment(line: &str) -> &str {
    line.split_once(';').map_or(line, |(body, _)| body)
}

fn control_scalar_let_assignment(line: &str) -> Option<(String, String)> {
    let body = strip_control_inline_comment(line).trim();
    let (command, rest) = body.split_once(char::is_whitespace)?;
    if !command.eq_ignore_ascii_case("let") {
        return None;
    }
    let (name, expression) = rest.trim().split_once('=')?;
    let name = name.trim();
    let expression = expression.trim();
    if name.is_empty()
        || expression.is_empty()
        || !name
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_')
        || !expression.chars().all(|character| {
            character.is_ascii_alphanumeric()
                || character.is_ascii_whitespace()
                || matches!(
                    character,
                    '_' | '.' | '+' | '-' | '*' | '/' | '^' | '{' | '}' | '\''
                )
        })
    {
        return None;
    }
    let arithmetic = expression
        .chars()
        .any(|character| matches!(character, '+' | '-' | '*' | '/' | '^'));
    if !arithmetic && crate::netlist::lexer::parse_spice_value(expression).is_err() {
        return None;
    }
    let expression = expression
        .strip_prefix('\'')
        .and_then(|value| value.strip_suffix('\''))
        .or_else(|| {
            expression
                .strip_prefix('{')
                .and_then(|value| value.strip_suffix('}'))
        })
        .unwrap_or(expression);
    Some((name.to_ascii_uppercase(), expression.to_string()))
}

fn expand_control_scalar_expression(
    expression: &str,
    scalar_lets: &HashMap<String, String>,
    referenced: &mut Vec<String>,
) -> String {
    let mut output = String::with_capacity(expression.len());
    let mut identifier = String::new();
    for character in expression.chars() {
        if character.is_ascii_alphanumeric() || character == '_' {
            identifier.push(character);
            continue;
        }
        if !identifier.is_empty() {
            append_control_scalar_identifier(&mut output, &identifier, scalar_lets, referenced);
            identifier.clear();
        }
        output.push(character);
    }
    if !identifier.is_empty() {
        append_control_scalar_identifier(&mut output, &identifier, scalar_lets, referenced);
    }
    output
}

fn append_control_scalar_identifier(
    output: &mut String,
    identifier: &str,
    scalar_lets: &HashMap<String, String>,
    referenced: &mut Vec<String>,
) {
    let key = identifier.to_ascii_uppercase();
    if let Some(expression) = scalar_lets.get(&key) {
        output.push('(');
        output.push_str(expression);
        output.push(')');
        referenced.push(key);
    } else {
        output.push_str(identifier);
    }
}

fn control_set_value(assignments: &str, name: &str) -> Option<String> {
    let normalized = assignments.replace('=', " = ");
    let tokens = normalized.split_whitespace().collect::<Vec<_>>();
    let mut index = 0usize;
    while index < tokens.len() {
        if tokens[index].eq_ignore_ascii_case(name)
            && tokens.get(index + 1).is_some_and(|token| *token == "=")
            && let Some(value) = tokens.get(index + 2)
        {
            return Some((*value).to_string());
        }
        index += 1;
    }
    None
}

fn control_auto_bridge_template_assignment(
    assignments: &str,
) -> Option<(String, String, String, Option<usize>)> {
    let bytes = assignments.as_bytes();
    let mut index = 0usize;

    while index < bytes.len() {
        skip_control_ws(bytes, &mut index);
        let key_start = index;
        while bytes
            .get(index)
            .is_some_and(|byte| control_variable_name_byte(*byte))
        {
            index += 1;
        }
        if key_start == index {
            index += 1;
            continue;
        }

        let key = assignments[key_start..index].to_string();
        skip_control_ws(bytes, &mut index);
        if bytes.get(index) != Some(&b'=') {
            continue;
        }
        index += 1;
        skip_control_ws(bytes, &mut index);

        if !control_auto_bridge_template_key(&key) {
            index = skip_control_assignment_value(assignments, index);
            continue;
        }

        let values = parse_control_bridge_template_list(assignments, &mut index)?;
        if values.len() < 2 {
            return None;
        }
        let max_nodes = values
            .get(2)
            .and_then(|value| value.trim().parse::<usize>().ok())
            .filter(|value| *value > 0);
        return Some((key, values[0].clone(), values[1].clone(), max_nodes));
    }

    None
}

fn control_auto_bridge_template_key(key: &str) -> bool {
    let lower = key.to_ascii_lowercase();
    lower.starts_with("auto_bridge_") && !lower.starts_with("auto_bridge_parm_")
}

fn control_auto_bridge_param_assignment(assignments: &str) -> Option<(String, String)> {
    let bytes = assignments.as_bytes();
    let mut index = 0usize;

    while index < bytes.len() {
        skip_control_ws(bytes, &mut index);
        let key_start = index;
        while bytes
            .get(index)
            .is_some_and(|byte| control_variable_name_byte(*byte))
        {
            index += 1;
        }
        if key_start == index {
            index += 1;
            continue;
        }

        let key = &assignments[key_start..index];
        skip_control_ws(bytes, &mut index);
        if bytes.get(index) != Some(&b'=') {
            continue;
        }
        index += 1;
        skip_control_ws(bytes, &mut index);

        const PREFIX: &str = "auto_bridge_parm_";
        let lower_key = key.to_ascii_lowercase();
        if !lower_key.starts_with(PREFIX) {
            index = skip_control_assignment_value(assignments, index);
            continue;
        }
        let node_type = &key[PREFIX.len()..];
        if node_type.is_empty() {
            return None;
        }

        let param_name = if bytes.get(index) == Some(&b'"') {
            parse_control_quoted_string(assignments, &mut index)?
        } else {
            parse_control_unquoted_list_value(assignments, &mut index)?
        };
        let param_name = param_name.trim();
        if param_name.is_empty() {
            return None;
        }
        return Some((node_type.to_string(), param_name.to_string()));
    }

    None
}

fn control_no_auto_bridge_family_setting(assignments: &str) -> Option<bool> {
    let bytes = assignments.as_bytes();
    let mut index = 0usize;

    while index < bytes.len() {
        skip_control_ws(bytes, &mut index);
        let key_start = index;
        while bytes
            .get(index)
            .is_some_and(|byte| control_variable_name_byte(*byte))
        {
            index += 1;
        }
        if key_start == index {
            index += 1;
            continue;
        }

        let key = &assignments[key_start..index];
        skip_control_ws(bytes, &mut index);
        if !key.eq_ignore_ascii_case("no_auto_bridge_family") {
            if bytes.get(index) == Some(&b'=') {
                index += 1;
                skip_control_ws(bytes, &mut index);
                index = skip_control_assignment_value(assignments, index);
            }
            continue;
        }

        if bytes.get(index) != Some(&b'=') {
            return Some(true);
        }
        index += 1;
        skip_control_ws(bytes, &mut index);
        let value = if bytes.get(index) == Some(&b'"') {
            parse_control_quoted_string(assignments, &mut index)?
        } else {
            parse_control_unquoted_list_value(assignments, &mut index)?
        };
        return control_bool_value(&value);
    }

    None
}

fn control_bool_value(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

fn control_variable_name_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.')
}

fn skip_control_ws(bytes: &[u8], index: &mut usize) {
    while bytes
        .get(*index)
        .is_some_and(|byte| byte.is_ascii_whitespace())
    {
        *index += 1;
    }
}

fn skip_control_assignment_value(input: &str, mut index: usize) -> usize {
    let bytes = input.as_bytes();
    let mut quote = false;
    let mut depth = 0usize;
    while index < bytes.len() {
        match bytes[index] {
            b'"' => quote = !quote,
            b'(' if !quote => depth += 1,
            b')' if !quote => depth = depth.saturating_sub(1),
            byte if !quote && depth == 0 && byte.is_ascii_whitespace() => break,
            _ => {}
        }
        index += 1;
    }
    index
}

fn parse_control_bridge_template_list(input: &str, index: &mut usize) -> Option<Vec<String>> {
    let bytes = input.as_bytes();
    if bytes.get(*index) != Some(&b'(') {
        return None;
    }
    *index += 1;

    let mut values = Vec::new();
    loop {
        skip_control_ws(bytes, index);
        match bytes.get(*index) {
            Some(b')') => {
                *index += 1;
                return Some(values);
            }
            Some(b'+') => {
                *index += 1;
                continue;
            }
            Some(b'"') => values.push(parse_control_quoted_string(input, index)?),
            Some(_) => values.push(parse_control_unquoted_list_value(input, index)?),
            None => return None,
        }
    }
}

fn parse_control_quoted_string(input: &str, index: &mut usize) -> Option<String> {
    let bytes = input.as_bytes();
    if bytes.get(*index) != Some(&b'"') {
        return None;
    }
    *index += 1;
    let mut value = String::new();
    while *index < bytes.len() {
        let byte = bytes[*index];
        *index += 1;
        if byte == b'"' {
            return Some(value);
        }
        if byte == b'\\'
            && let Some(next) = bytes.get(*index)
        {
            value.push(*next as char);
            *index += 1;
            continue;
        }
        value.push(byte as char);
    }
    None
}

fn parse_control_unquoted_list_value(input: &str, index: &mut usize) -> Option<String> {
    let bytes = input.as_bytes();
    let start = *index;
    while bytes
        .get(*index)
        .is_some_and(|byte| !byte.is_ascii_whitespace() && *byte != b')' && *byte != b'+')
    {
        *index += 1;
    }
    (*index > start).then(|| input[start..*index].to_string())
}

fn control_hex_encode(value: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(4 + value.len() * 2);
    encoded.push_str("HEX_");
    for byte in value.bytes() {
        encoded.push(HEX[(byte >> 4) as usize] as char);
        encoded.push(HEX[(byte & 0x0f) as usize] as char);
    }
    encoded
}

fn normalize_control_analysis_token(token: &str, scalar_lets: &mut ControlScalarLets) -> String {
    let Some(name) = token
        .strip_prefix("$&")
        .filter(|name| is_control_parameter_name(name))
    else {
        return token.to_string();
    };
    scalar_lets.consume(&name.to_ascii_uppercase()).map_or_else(
        || name.to_string(),
        |expression| format!("{{{expression}}}"),
    )
}

fn normalize_control_measure_token(token: &str, scalar_lets: &mut ControlScalarLets) -> String {
    let normalized = normalize_control_analysis_token(token, scalar_lets);
    if normalized != token {
        return normalized;
    }
    if let Some(expression) = scalar_lets.consume(&token.to_ascii_uppercase()) {
        return format!("{{{expression}}}");
    }
    let Some((key, value)) = token.split_once('=') else {
        return token.to_string();
    };
    scalar_lets
        .consume(&value.to_ascii_uppercase())
        .map_or_else(
            || token.to_string(),
            |expression| format!("{key}={{{expression}}}"),
        )
}

fn is_control_parameter_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '.')
}

impl Default for Netlist {
    fn default() -> Self {
        Self {
            title: String::new(),
            elements: Vec::new(),
            analyses: Vec::new(),
            lin_analysis: None,
            fft_analyses: Vec::new(),
            data_tables: Vec::new(),
            models: Vec::new(),
            subcircuits: Vec::new(),
            params: ParamContext::new(),
            spectre_statistics: SpectreStatisticsPlan::default(),
            spectre_statistical_coordinate: None,
            initial_conditions: Vec::new(),
            device_initial_conditions: None,
            node_sets: Vec::new(),
            startup_directives: Vec::new(),
            global_nodes: HashSet::new(),
            measurements: Vec::new(),
            saves: SaveSet::default(),
            output_requests: Vec::new(),
            options: SimulationOptions::default(),
            veriloga_includes: Vec::new(),
            spef_includes: Vec::new(),
            diagnostics: Vec::new(),
            control_dispositions: Vec::new(),
            pspice_chebyshev_source_count: 0,
            source_text: None,
            source_path: None,
            replay_context: None,
            ast_overlay: NetlistAstOverlay::default(),
        }
    }
}

/// Read and decode source text while polling between bounded I/O chunks.
#[cfg(test)]
fn read_file_with_encoding_with_abort(
    path: &std::path::Path,
    abort: &dyn AbortSignal,
) -> Result<String, ParseWithAbortError> {
    read_file_with_encoding_limited_with_abort(
        path,
        crate::resource::ResourceKind::NetlistBytes,
        0,
        usize::MAX,
        abort,
    )
    .map(|(source, _)| source)
}

fn read_file_with_encoding_limited_with_abort(
    path: &std::path::Path,
    resource: crate::resource::ResourceKind,
    retained_bytes: usize,
    limit: usize,
    abort: &dyn AbortSignal,
) -> Result<(String, usize), ParseWithAbortError> {
    use std::io::Read;

    const READ_CHUNK_BYTES: usize = 64 * 1024;

    ensure_parse_not_aborted(abort)?;
    let mut file = std::fs::File::open(path).map_err(ParseError::Io)?;
    let metadata_bytes = file
        .metadata()
        .ok()
        .and_then(|metadata| usize::try_from(metadata.len()).ok());
    if let Some(file_bytes) = metadata_bytes {
        crate::resource::ResourceLimitError::ensure(
            resource,
            retained_bytes.saturating_add(file_bytes),
            limit,
        )
        .map_err(ParseError::from)?;
    }
    let mut bytes = Vec::with_capacity(metadata_bytes.unwrap_or(0));
    let mut chunk = [0_u8; READ_CHUNK_BYTES];
    loop {
        ensure_parse_not_aborted(abort)?;
        let count = file.read(&mut chunk).map_err(ParseError::Io)?;
        if count == 0 {
            break;
        }
        crate::resource::ResourceLimitError::ensure(
            resource,
            retained_bytes
                .saturating_add(bytes.len())
                .saturating_add(count),
            limit,
        )
        .map_err(ParseError::from)?;
        bytes.extend_from_slice(&chunk[..count]);
    }
    ensure_parse_not_aborted(abort)?;
    let source_bytes = bytes.len();
    let source = decode_source_bytes_with_abort(bytes, abort)?;
    let retained_source_bytes = source_bytes.max(source.len());
    crate::resource::ResourceLimitError::ensure(
        resource,
        retained_bytes.saturating_add(retained_source_bytes),
        limit,
    )
    .map_err(ParseError::from)?;
    Ok((source, retained_source_bytes))
}

fn decode_source_bytes_with_abort(
    bytes: Vec<u8>,
    abort: &dyn AbortSignal,
) -> Result<String, ParseWithAbortError> {
    ensure_parse_not_aborted(abort)?;

    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        let source = String::from_utf8(bytes[3..].to_vec()).map_err(|error| {
            ParseError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, error))
        })?;
        ensure_parse_not_aborted(abort)?;
        return Ok(source);
    }

    if bytes.starts_with(&[0xFF, 0xFE]) || bytes.starts_with(&[0xFE, 0xFF]) {
        let little_endian = bytes.starts_with(&[0xFF, 0xFE]);
        let body = &bytes[2..];
        if !body.len().is_multiple_of(2) {
            return Err(ParseError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "UTF-16 data has odd number of bytes",
            ))
            .into());
        }
        let mut utf16 = Vec::with_capacity(body.len() / 2);
        for (index, pair) in body.chunks_exact(2).enumerate() {
            poll_parse_abort(abort, index)?;
            let pair = [pair[0], pair[1]];
            utf16.push(if little_endian {
                u16::from_le_bytes(pair)
            } else {
                u16::from_be_bytes(pair)
            });
        }
        ensure_parse_not_aborted(abort)?;
        let source = String::from_utf16(&utf16).map_err(|error| {
            ParseError::Io(std::io::Error::new(std::io::ErrorKind::InvalidData, error))
        })?;
        ensure_parse_not_aborted(abort)?;
        return Ok(source);
    }

    match String::from_utf8(bytes) {
        Ok(source) => {
            ensure_parse_not_aborted(abort)?;
            Ok(source)
        }
        Err(error) => {
            let bytes = error.into_bytes();
            let mut source = String::with_capacity(bytes.len());
            for (index, chunk) in bytes.chunks(4096).enumerate() {
                poll_parse_abort(abort, index)?;
                source.extend(chunk.iter().map(|byte| char::from(*byte)));
            }
            ensure_parse_not_aborted(abort)?;
            Ok(source)
        }
    }
}

/// Decode exact source bytes using RSpice's supported netlist/model encoding
/// policy. Callers that authenticate raw bytes can decode those same bytes
/// without reopening the source file.
pub fn decode_source_bytes(bytes: &[u8]) -> Result<String, std::io::Error> {
    // Check for BOM and decode accordingly
    if bytes.starts_with(&[0xEF, 0xBB, 0xBF]) {
        // UTF-8 with BOM - skip BOM bytes
        String::from_utf8(bytes[3..].to_vec())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    } else if bytes.starts_with(&[0xFF, 0xFE]) {
        // UTF-16 LE BOM
        decode_utf16_le(&bytes[2..])
    } else if bytes.starts_with(&[0xFE, 0xFF]) {
        // UTF-16 BE BOM
        decode_utf16_be(&bytes[2..])
    } else {
        // Try UTF-8 first, fall back to Latin-1
        match String::from_utf8(bytes.to_vec()) {
            Ok(s) => Ok(s),
            Err(_) => {
                // Latin-1 fallback (each byte is a valid codepoint)
                Ok(bytes.iter().map(|&b| b as char).collect())
            }
        }
    }
}

/// Decode UTF-16 LE bytes to String
fn decode_utf16_le(bytes: &[u8]) -> Result<String, std::io::Error> {
    if !bytes.len().is_multiple_of(2) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "UTF-16 data has odd number of bytes",
        ));
    }

    let utf16: Vec<u16> = bytes
        .chunks(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    String::from_utf16(&utf16).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

/// Decode UTF-16 BE bytes to String  
fn decode_utf16_be(bytes: &[u8]) -> Result<String, std::io::Error> {
    if !bytes.len().is_multiple_of(2) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "UTF-16 data has odd number of bytes",
        ));
    }

    let utf16: Vec<u16> = bytes
        .chunks(2)
        .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
        .collect();

    String::from_utf16(&utf16).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cancellation_fixture_path(name: &str) -> PathBuf {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time follows Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "rspice-netlist-{name}-{}-{nonce}.tmp",
            std::process::id()
        ))
    }

    struct IncludeFixtureCleanup {
        root: PathBuf,
        execution_files: Vec<PathBuf>,
    }

    impl Drop for IncludeFixtureCleanup {
        fn drop(&mut self) {
            for path in &self.execution_files {
                let _ = std::fs::remove_file(path);
            }
            let _ = std::fs::remove_dir_all(&self.root);
        }
    }

    fn resistor_value(netlist: &Netlist, name: &str) -> Value {
        passive_test_state(&netlist.elements, name).0
    }

    fn passive_test_state<'a>(
        elements: &'a [Element],
        name: &str,
    ) -> (Value, &'a [(String, Value)], &'a [(String, String)]) {
        let element = elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("passive element {name} exists"));
        match &element.kind {
            ElementKind::Resistor {
                value,
                instance_params,
                deferred_params,
                ..
            }
            | ElementKind::Capacitor {
                value,
                instance_params,
                deferred_params,
                ..
            }
            | ElementKind::Inductor {
                value,
                instance_params,
                deferred_params,
                ..
            } => (*value, instance_params, deferred_params),
            _ => panic!("{name} is not an R/C/L passive"),
        }
    }

    fn assert_unique_passive_param(params: &[(String, Value)], name: &str, expected: Value) {
        let matches = params
            .iter()
            .filter(|(candidate, _)| candidate.eq_ignore_ascii_case(name))
            .collect::<Vec<_>>();
        assert_eq!(matches.len(), 1, "expected one {name} in {params:?}");
        let tolerance = expected.abs().max(1.0) * 1.0e-12;
        assert!(
            (matches[0].1 - expected).abs() <= tolerance,
            "{name}={}, expected {expected}; params={params:?}",
            matches[0].1
        );
    }

    #[test]
    fn parsed_xyce_print_contract_drives_prn_adapter_end_to_end() {
        use crate::io::xyce_prn::{
            XycePrnFooter, XycePrnLimits, XycePrnTable, serialize_xyce_prn_sequence,
        };

        let netlist = Netlist::parse(
            "typed Xyce PRN adapter\n\
             V1 out 0 1\n\
             .OPTIONS OUTPUT PRINTHEADER=off PRINTFOOTER=off\n\
             .PRINT TRAN DELIMITER=COMMA PRECISION=12 WIDTH=21 V(out)\n\
             .TRAN 1n 1n\n\
             .END\n",
        )
        .expect("typed print contract parses");
        let [request] = netlist.output_requests.as_slice() else {
            panic!("expected one typed output request");
        };
        let table = XycePrnTable {
            columns: vec!["Index".into(), "TIME".into(), "V(OUT)".into()],
            rows: vec![vec![0.0, 1.0e-9, 1.0]],
        };

        let text = serialize_xyce_prn_sequence(
            &[table],
            request,
            &netlist.options,
            XycePrnFooter::Simulation,
            XycePrnLimits::new(1, 1_000),
        )
        .expect("parsed request serializes through its lower-layer adapter");

        assert_eq!(text, "0,1.000000000000e-09,1.000000000000e+00\n");
    }

    #[test]
    fn ordinary_file_parse_uses_xyce_include_precedence_and_retains_execution_dir_for_replay() {
        let root = cancellation_fixture_path("xyce-include-precedence");
        let top_level = root.join("top-level");
        let including_dir = top_level.join("sub");
        std::fs::create_dir_all(&including_dir).expect("create include precedence fixture");
        let execution_dir = std::env::current_dir().expect("read process execution directory");
        let token = root
            .file_name()
            .expect("fixture has a name")
            .to_string_lossy();
        let local_name = format!("{token}-local.inc");
        let top_name = format!("{token}-top.inc");
        let execution_name = format!("{token}-execution.inc");
        let execution_files =
            [&local_name, &top_name, &execution_name].map(|name| execution_dir.join(name));
        let _cleanup = IncludeFixtureCleanup {
            root: root.clone(),
            execution_files: execution_files.to_vec(),
        };

        std::fs::write(&execution_files[0], "RLOCAL 1 0 101\n")
            .expect("write shadowed execution-directory local include");
        std::fs::write(&execution_files[1], "RTOP 1 0 202\n")
            .expect("write shadowed execution-directory top-level include");
        std::fs::write(&execution_files[2], "REXEC 1 0 3\n")
            .expect("write execution-directory fallback include");
        std::fs::write(top_level.join(&local_name), "RLOCAL 1 0 11\n")
            .expect("write shadowed top-level local include");
        std::fs::write(top_level.join(&top_name), "RTOP 1 0 2\n")
            .expect("write top-level fallback include");
        std::fs::write(including_dir.join(&local_name), "RLOCAL 1 0 1\n")
            .expect("write including-file-local include");
        let nested_source = format!(
            ".include \"{local_name}\"\n.include \"{top_name}\"\n.include \"{execution_name}\"\n"
        );
        std::fs::write(including_dir.join("nested.inc"), nested_source)
            .expect("write nested include owner");
        let source = "Xyce include precedence\n.include sub/nested.inc\n.end\n";
        let deck_path = top_level.join("deck.cir");
        std::fs::write(&deck_path, source).expect("write top-level deck");

        let parsed = Netlist::parse_file(&deck_path)
            .expect("ordinary file parse resolves every Xyce include stage");
        assert_eq!(resistor_value(&parsed, "RLOCAL"), 1.0);
        assert_eq!(resistor_value(&parsed, "RTOP"), 2.0);
        assert_eq!(resistor_value(&parsed, "REXEC"), 3.0);
        match parsed.replay_context.as_ref() {
            Some(NetlistReplayContext::PathWithExecutionDir(captured)) => {
                assert_eq!(captured, &execution_dir)
            }
            other => {
                panic!("ordinary file parse did not retain its execution directory: {other:?}")
            }
        }

        let replayed = parsed
            .replay_root_source_with_options_and_abort(
                "Xyce include precedence replay\n.include sub/nested.inc\nRROOT 2 0 4\n.end\n",
                NetlistParseOptions::default(),
                &NoAbort,
            )
            .expect("replay uses the captured include resolver contract");
        assert_eq!(resistor_value(&replayed, "RLOCAL"), 1.0);
        assert_eq!(resistor_value(&replayed, "RTOP"), 2.0);
        assert_eq!(resistor_value(&replayed, "REXEC"), 3.0);
        assert_eq!(resistor_value(&replayed, "RROOT"), 4.0);
    }

    #[test]
    fn ordinary_search_path_parse_retains_execution_dir_for_replay() {
        let root = cancellation_fixture_path("xyce-search-path-replay");
        let top_level = root.join("top-level");
        let search_dir = root.join("configured-search");
        std::fs::create_dir_all(&top_level).expect("create top-level fixture directory");
        std::fs::create_dir_all(&search_dir).expect("create configured search directory");
        let execution_dir = std::env::current_dir().expect("read process execution directory");
        let token = root
            .file_name()
            .expect("fixture has a name")
            .to_string_lossy();
        let execution_name = format!("{token}-execution.inc");
        let search_name = format!("{token}-search.inc");
        let execution_file = execution_dir.join(&execution_name);
        let _cleanup = IncludeFixtureCleanup {
            root: root.clone(),
            execution_files: vec![execution_file.clone()],
        };
        std::fs::write(&execution_file, "REXEC 1 0 3\n")
            .expect("write execution-directory include");
        std::fs::write(search_dir.join(&search_name), "RSEARCH 2 0 5\n")
            .expect("write configured-search include");
        let source = format!(
            "search-path replay\n.include \"{execution_name}\"\n.include \"{search_name}\"\n.end\n"
        );
        let deck_path = top_level.join("deck.cir");
        std::fs::write(&deck_path, &source).expect("write search-path deck");
        let search_paths = vec![search_dir];

        let parsed = Netlist::parse_with_search_paths_and_options(
            &source,
            &deck_path,
            &search_paths,
            NetlistParseOptions::default(),
        )
        .expect("search-path parse resolves execution and configured fallbacks");
        assert_eq!(resistor_value(&parsed, "REXEC"), 3.0);
        assert_eq!(resistor_value(&parsed, "RSEARCH"), 5.0);
        match parsed.replay_context.as_ref() {
            Some(NetlistReplayContext::SearchPaths {
                paths,
                execution_dir: captured,
            }) => {
                assert_eq!(paths, &search_paths);
                assert_eq!(captured, &execution_dir);
            }
            other => panic!("search-path parse did not retain its resolver context: {other:?}"),
        }

        let replayed = parsed
            .replay_root_source_with_options_and_abort(
                &source,
                NetlistParseOptions::default(),
                &NoAbort,
            )
            .expect("search-path replay uses the captured resolver contract");
        assert_eq!(resistor_value(&replayed, "REXEC"), 3.0);
        assert_eq!(resistor_value(&replayed, "RSEARCH"), 5.0);
    }

    #[test]
    fn ordinary_include_preprocessing_uses_process_execution_dir_fallback() {
        let root = cancellation_fixture_path("xyce-preprocess-execution-dir");
        std::fs::create_dir_all(&root).expect("create preprocessing fixture");
        let execution_dir = std::env::current_dir().expect("read process execution directory");
        let token = root
            .file_name()
            .expect("fixture has a name")
            .to_string_lossy();
        let include_name = format!("{token}-preprocess.inc");
        let execution_file = execution_dir.join(&include_name);
        let _cleanup = IncludeFixtureCleanup {
            root: root.clone(),
            execution_files: vec![execution_file.clone()],
        };
        std::fs::write(&execution_file, "RPRE 1 0 7\n")
            .expect("write execution-directory preprocessing include");
        let source = format!("preprocess execution fallback\n.include \"{include_name}\"\n.end\n");
        let deck_path = root.join("deck.cir");
        std::fs::write(&deck_path, &source).expect("write preprocessing deck");

        let expanded = Netlist::preprocess_includes(&source, &deck_path)
            .expect("ordinary preprocessing resolves against process execution directory");
        assert!(expanded.contains("RPRE 1 0 7"), "{expanded}");
    }

    #[test]
    fn source_file_ingestion_aborts_between_read_chunks() {
        let path = cancellation_fixture_path("chunked-read");
        std::fs::write(&path, vec![b' '; 1024 * 1024]).expect("write source fixture");
        let abort = crate::abort_signal::CountingAbort::new(3);

        let result = read_file_with_encoding_with_abort(&path, &abort);
        let _ = std::fs::remove_file(&path);

        assert!(matches!(result, Err(ParseWithAbortError::Aborted)));
        assert!(abort.count() > 3, "source reads must poll between chunks");
    }

    #[test]
    fn model_path_normalization_aborts_after_multiple_internal_polls() {
        let mut netlist = Netlist::default();
        for index in 0..1_024 {
            netlist.models.push(ModelDef {
                name: format!("M{index}"),
                model_type: "D".to_string(),
                params: Vec::new(),
                expr_params: Vec::new(),
                string_params: vec![("file".to_string(), format!("models/m{index}.dat"))],
                string_vector_params: Vec::new(),
                real_vector_params: Vec::new(),
                real_vector_expr_params: Vec::new(),
                integer_vector_params: Vec::new(),
            });
        }
        let abort = crate::abort_signal::CountingAbort::new(8);

        let result = Netlist::normalize_model_string_paths_with_abort(
            &mut netlist,
            Path::new("project/deck.cir"),
            &abort,
        );

        assert!(matches!(result, Err(ParseWithAbortError::Aborted)));
        assert!(
            abort.count() > 8,
            "path normalization must poll during model traversal"
        );
    }

    fn first_mosfet(netlist: &Netlist) -> &ElementKind {
        netlist
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Mosfet { .. } => Some(&element.kind),
                _ => None,
            })
            .expect("MOSFET exists")
    }

    fn first_jfet(netlist: &Netlist) -> &ElementKind {
        netlist
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Jfet { .. } => Some(&element.kind),
                _ => None,
            })
            .expect("JFET exists")
    }

    fn first_mesfet(netlist: &Netlist) -> &ElementKind {
        netlist
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Mesfet { .. } => Some(&element.kind),
                _ => None,
            })
            .expect("MESFET exists")
    }

    fn first_diode(netlist: &Netlist) -> &ElementKind {
        netlist
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Diode { .. } => Some(&element.kind),
                _ => None,
            })
            .expect("diode exists")
    }

    fn scoped_model_param(models: &[ModelDef], model_name: &str, param_name: &str) -> Option<f64> {
        models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case(model_name))?
            .params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(param_name))
            .map(|(_, value)| *value)
    }

    fn first_bjt(netlist: &Netlist) -> &ElementKind {
        netlist
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Bjt { .. } => Some(&element.kind),
                _ => None,
            })
            .expect("BJT exists")
    }

    #[test]
    fn aggregate_measure_preserves_goal_and_tolerance() {
        for title in ["measure goal", "* dc measurement with failing goal"] {
            let netlist = Netlist::parse(&format!(
                "{title}\n\
                 V1 in 0 10\n\
                 R1 in out 1k\n\
                 R2 out 0 1k\n\
                 .dc V1 0 10 1\n\
                 .meas dc vout MAX V(out) GOAL=4 TOL=0.1\n\
                 .end\n"
            ))
            .expect("aggregate .MEAS with GOAL/TOL parses");

            assert_eq!(netlist.measurements.len(), 1);
            let measurement = &netlist.measurements[0];
            assert_eq!(measurement.name, "VOUT");
            assert_eq!(measurement.goal, Some(4.0), "title={title}");
            assert_eq!(measurement.tolerance, Some(0.1), "title={title}");
            match &measurement.measure_type {
                crate::netlist::measure::MeasureType::Max {
                    signal,
                    from,
                    to,
                    output,
                } => {
                    assert_eq!(signal, "V(OUT)");
                    assert_eq!(*from, None);
                    assert_eq!(*to, None);
                    assert_eq!(*output, crate::netlist::measure::ExtremaOutput::Value);
                }
                other => panic!("expected MAX measurement, got {other:?}"),
            }
        }
    }

    #[test]
    fn measure_integral_alias_preserves_range() {
        let netlist = Netlist::parse(
            "integral measurement alias\n\
             V1 out 0 AC 1\n\
             .ac dec 5 1 1k\n\
             .measure ac area integral vm(out) from=10 to=100\n\
             .end\n",
        )
        .expect("INTEGRAL alias parses");

        assert_eq!(netlist.measurements.len(), 1);
        match &netlist.measurements[0].measure_type {
            crate::netlist::measure::MeasureType::Integ { signal, from, to } => {
                assert_eq!(signal, "VM(OUT)");
                assert_eq!(*from, Some(10.0));
                assert_eq!(*to, Some(100.0));
            }
            other => panic!("expected INTEG measurement, got {other:?}"),
        }
    }

    #[test]
    fn measure_output_lexemes_follow_the_selected_expression_dialect() {
        let source = "dialect-specific measurement output spelling\n\
                      V1 out 0 AC 1\n\
                      .ac dec 5 1 1k\n\
                      .measure ac area integral vm(out) from=10 to=100\n\
                      .end\n";
        let generic = Netlist::parse(source).expect("generic measurement parses");
        let xyce = Netlist::parse_with_options(
            source,
            NetlistParseOptions {
                expression_dialect: ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("Xyce measurement parses");

        let crate::netlist::measure::MeasureType::Integ {
            signal: generic_signal,
            ..
        } = &generic.measurements[0].measure_type
        else {
            panic!("expected generic INTEG measurement");
        };
        let crate::netlist::measure::MeasureType::Integ {
            signal: xyce_signal,
            ..
        } = &xyce.measurements[0].measure_type
        else {
            panic!("expected Xyce INTEG measurement");
        };
        assert_eq!(generic_signal, "VM(OUT)");
        assert_eq!(xyce_signal, "VM(out)");
    }

    #[test]
    fn measurement_names_preserve_one_punctuated_source_field() {
        let netlist = Netlist::parse(
            "punctuated measurement names\n\
             V1 out 0 0\n\
             .dc V1 0 1 1\n\
             .measure dc constant-at deriv V(out) at=0.5\n\
             .measure dc ratio/output max V(out)\n\
             .end\n",
        )
        .expect("punctuated measurement names parse");

        assert_eq!(netlist.measurements[0].name, "CONSTANT-AT");
        assert_eq!(netlist.measurements[1].name, "RATIO/OUTPUT");
    }

    #[test]
    fn derivative_measurements_preserve_waveform_conditions_and_windows() {
        let netlist = Netlist::parse(
            "typed derivative condition\n\
             V1 one 0 0\n\
             V2 two 0 0\n\
             .dc V1 5 1 -1\n\
             .measure dc slope deriv {V(one)-V(two)} when V(two)={2*V(one)} from=4 to=2\n\
             .end\n",
        )
        .expect("waveform-valued DERIV condition parses");

        match &netlist.measurements[0].measure_type {
            crate::netlist::measure::MeasureType::Derivative {
                signal,
                at,
                when,
                from,
                to,
                ..
            } => {
                assert_eq!(signal, "{V(one)-V(two)}");
                assert_eq!(*at, None);
                assert_eq!(*from, Some(4.0));
                assert_eq!(*to, Some(2.0));
                let when = when.as_ref().expect("WHEN condition retained");
                assert_eq!(when.left, "V(TWO)");
                assert_eq!(
                    when.right,
                    crate::netlist::measure::MeasureOperand::Waveform("{2*V(one)}".to_string())
                );
            }
            other => panic!("expected DERIV measurement, got {other:?}"),
        }
    }

    #[test]
    fn find_at_accepts_xyce_optional_equals_separator() {
        let netlist = Netlist::parse(
            "Xyce FIND-AT separator forms\n\
             V1 out 0 AC 1\n\
             .ac dec 5 100 1e6\n\
             .measure ac explicit FIND VI(out) AT=1e2\n\
             .measure ac whitespace FIND VI(out) AT 1e4\n\
             .measure ac expression FIND VI(out) AT {5e4}\n\
             .end\n",
        )
        .expect("Xyce FIND-AT accepts both explicit and omitted equals separators");

        let expected = [100.0, 10_000.0, 50_000.0];
        assert_eq!(netlist.measurements.len(), expected.len());
        for (statement, expected_at) in netlist.measurements.iter().zip(expected) {
            let crate::netlist::measure::MeasureType::Find { at, .. } = statement.measure_type
            else {
                panic!(
                    "expected FIND measurement, got {:?}",
                    statement.measure_type
                );
            };
            assert_eq!(at, Some(expected_at));
        }
    }

    #[test]
    fn standalone_when_measurements_preserve_typed_operands_and_windows() {
        let netlist = Netlist::parse(
            "typed standalone WHEN conditions\n\
             V1 one 0 0\n\
             V2 two 0 0\n\
             .dc V1 5 1 -1\n\
             .measure dc waveform WHEN V(one)={2*V(two)} FROM=4 TO=2 CROSS=2\n\
             .measure dc constant WHEN V(one)=2.5\n\
             .end\n",
        )
        .expect("standalone WHEN conditions parse");

        match &netlist.measurements[0].measure_type {
            crate::netlist::measure::MeasureType::When {
                condition,
                from,
                to,
                ..
            } => {
                assert_eq!(condition.left, "V(ONE)");
                assert_eq!(
                    condition.right,
                    crate::netlist::measure::MeasureOperand::Waveform("{2*V(two)}".to_string())
                );
                assert_eq!(*from, Some(4.0));
                assert_eq!(*to, Some(2.0));
                assert_eq!(
                    condition.occurrence.edge,
                    crate::netlist::measure::EdgeType::Cross
                );
                assert_eq!(condition.occurrence.number, 2);
            }
            other => panic!("expected standalone WHEN measurement, got {other:?}"),
        }
        match &netlist.measurements[1].measure_type {
            crate::netlist::measure::MeasureType::When { condition, .. } => {
                assert_eq!(condition.left, "V(ONE)");
                assert_eq!(
                    condition.right,
                    crate::netlist::measure::MeasureOperand::Constant(2.5)
                );
            }
            other => panic!("expected standalone WHEN measurement, got {other:?}"),
        }
    }

    #[test]
    fn point_event_occurrences_are_typed_and_conflicts_fail_loudly() {
        let netlist = Netlist::parse(
            "typed point-event occurrence\n\
             V1 one 0 0\n\
             .dc V1 0 4 1\n\
             .measure dc selected FIND V(one) WHEN V(one)=2 RISE=2\n\
             .end\n",
        )
        .expect("point-event occurrence parses");
        match &netlist.measurements[0].measure_type {
            crate::netlist::measure::MeasureType::Find {
                when: Some(condition),
                ..
            } => {
                assert_eq!(
                    condition.occurrence.edge,
                    crate::netlist::measure::EdgeType::Rise
                );
                assert_eq!(condition.occurrence.number, 2);
            }
            other => panic!("expected FIND-WHEN measurement, got {other:?}"),
        }

        let error = Netlist::parse(
            "conflicting point-event occurrence\n\
             V1 one 0 0\n\
             .dc V1 0 4 1\n\
             .measure dc invalid WHEN V(one)=2 CROSS=1 FALL=2\n\
             .end\n",
        )
        .expect_err("conflicting occurrence qualifiers must not be ignored");
        assert!(error.to_string().contains("Only one RISE, FALL, or CROSS"));
    }

    #[test]
    fn trigger_target_clauses_preserve_at_moving_rhs_last_and_td() {
        let netlist = Netlist::parse(
            "typed trigger target clauses\n\
             V1 one 0 0\n\
             V2 two 0 0\n\
             .dc V1 0 4 1\n\
             .measure dc delay TRIG AT=1.5 TD=2 TARG {V(one)}={V(two)+1} FALL=LAST\n\
             .end\n",
        )
        .expect("typed trigger/target clauses parse");

        let crate::netlist::measure::MeasureType::Delay { trig, targ, .. } =
            &netlist.measurements[0].measure_type
        else {
            panic!("expected trigger/target delay measurement");
        };
        assert_eq!(trig.event, crate::netlist::measure::TriggerEvent::At(1.5));
        assert_eq!(trig.td, Some(2.0));
        match &targ.event {
            crate::netlist::measure::TriggerEvent::When(condition) => {
                assert_eq!(condition.left, "{V(one)}");
                assert_eq!(
                    condition.right,
                    crate::netlist::measure::MeasureOperand::Waveform("{V(two)+1}".to_string())
                );
                assert_eq!(
                    condition.occurrence.edge,
                    crate::netlist::measure::EdgeType::Fall
                );
                assert_eq!(condition.occurrence.number, -1);
            }
            other => panic!("expected conditional target, got {other:?}"),
        }
        assert_eq!(targ.td, None);
    }

    #[test]
    fn legacy_delay_parses_bare_targets_and_structural_global_windows() {
        let netlist = Netlist::parse(
            "legacy trigger target syntax\n\
             V1 FROM 0 0\n\
             V2 TO 0 0\n\
             .tran 1 10\n\
             .measure tran delay TRIG V(FROM) 0.1 FROM=2 RISE=1 TARG V(TO) {0.2} TO=9 FALL=2\n\
             .end\n",
        )
        .expect("legacy delay syntax parses without confusing node names for qualifiers");

        let crate::netlist::measure::MeasureType::Delay {
            trig,
            targ,
            from,
            to,
            ..
        } = &netlist.measurements[0].measure_type
        else {
            panic!("expected delay measurement");
        };
        assert_eq!((*from, *to), (Some(2.0), Some(9.0)));
        let crate::netlist::measure::TriggerEvent::When(trig) = &trig.event else {
            panic!("expected conditional trigger");
        };
        assert_eq!(trig.left, "V(FROM)");
        assert_eq!(
            trig.right,
            crate::netlist::measure::MeasureOperand::Constant(0.1)
        );
        let crate::netlist::measure::TriggerEvent::When(targ) = &targ.event else {
            panic!("expected conditional target");
        };
        assert_eq!(targ.left, "V(TO)");
        assert_eq!(
            targ.right,
            crate::netlist::measure::MeasureOperand::Waveform("{0.2}".to_string())
        );
    }

    #[test]
    fn measure_operands_share_expression_output_and_lexeme_parsing() {
        let netlist = Netlist::parse(
            "shared measurement operands\n\
             V1 1 0 0\n\
             .tran 1 2\n\
             .measure tran par_find FIND PAR('V(1)+1') WHEN V(1)=('0.5')\n\
             .measure tran par_delay TRIG PAR('V(1)-0.1') VAL=PAR('0.1') TARG ('V(1)-0.5') VAL=('0.3')\n\
             .measure tran spaced MAX I(YPDE BRANCH)\n\
             .measure tran scientific MAX V(2e3)\n\
             .measure tran numeric_when WHEN V(2e3)=0\n\
             .measure tran quoted MAX 'V(1)+2'\n\
             .measure tran spaced_find FIND I(YPDE BRANCH) AT=1\n\
             .measure tran spaced_when WHEN I(YPDE BRANCH)=0\n\
             .measure tran spaced_delay TRIG I(YPDE BRANCH)=0 TARG I(YPDE BRANCH)=1\n\
             .measure tran spaced_err ERR I(YPDE BRANCH) I(YPDE BRANCH)\n\
             .measure tran spaced_file ERROR I(YPDE BRANCH) FILE=reference.prn DEPVARCOL=1\n\
             .end\n",
        )
        .expect("shared measurement operand spellings parse");

        let crate::netlist::measure::MeasureType::Find {
            signal,
            when: Some(when),
            ..
        } = &netlist.measurements[0].measure_type
        else {
            panic!("expected FIND-WHEN");
        };
        assert_eq!(signal, "{V(1)+1}");
        assert_eq!(
            when.right,
            crate::netlist::measure::MeasureOperand::Waveform("{0.5}".to_string())
        );
        let crate::netlist::measure::MeasureType::Delay { trig, targ, .. } =
            &netlist.measurements[1].measure_type
        else {
            panic!("expected delay");
        };
        let crate::netlist::measure::TriggerEvent::When(trig) = &trig.event else {
            panic!("expected conditional trigger");
        };
        let crate::netlist::measure::TriggerEvent::When(targ) = &targ.event else {
            panic!("expected conditional target");
        };
        assert_eq!(trig.left, "{V(1)-0.1}");
        assert_eq!(targ.left, "{V(1)-0.5}");
        assert_eq!(
            trig.right,
            crate::netlist::measure::MeasureOperand::Waveform("{0.1}".to_string())
        );
        assert_eq!(
            targ.right,
            crate::netlist::measure::MeasureOperand::Waveform("{0.3}".to_string())
        );
        let crate::netlist::measure::MeasureType::Max { signal, .. } =
            &netlist.measurements[2].measure_type
        else {
            panic!("expected MAX");
        };
        assert_eq!(signal, "I(YPDE BRANCH)");
        let crate::netlist::measure::MeasureType::Max { signal, .. } =
            &netlist.measurements[3].measure_type
        else {
            panic!("expected MAX");
        };
        assert_eq!(signal, "V(2e3)");
        let crate::netlist::measure::MeasureType::When { condition, .. } =
            &netlist.measurements[4].measure_type
        else {
            panic!("expected WHEN");
        };
        assert_eq!(condition.left, "V(2e3)");
        let crate::netlist::measure::MeasureType::Max { signal, .. } =
            &netlist.measurements[5].measure_type
        else {
            panic!("expected quoted MAX expression");
        };
        assert_eq!(signal, "{V(1)+2}");
        let crate::netlist::measure::MeasureType::Find { signal, .. } =
            &netlist.measurements[6].measure_type
        else {
            panic!("expected spaced FIND");
        };
        assert_eq!(signal, "I(YPDE BRANCH)");
        let crate::netlist::measure::MeasureType::When { condition, .. } =
            &netlist.measurements[7].measure_type
        else {
            panic!("expected spaced WHEN");
        };
        assert_eq!(condition.left, "I(YPDE BRANCH)");
        let crate::netlist::measure::MeasureType::Delay { trig, targ, .. } =
            &netlist.measurements[8].measure_type
        else {
            panic!("expected spaced delay");
        };
        for clause in [trig, targ] {
            let crate::netlist::measure::TriggerEvent::When(condition) = &clause.event else {
                panic!("expected conditional spaced-current clause");
            };
            assert_eq!(condition.left, "I(YPDE BRANCH)");
        }
        let crate::netlist::measure::MeasureType::ErrorFunction {
            measured,
            comparison,
            ..
        } = &netlist.measurements[9].measure_type
        else {
            panic!("expected spaced ERR");
        };
        assert_eq!(measured, "I(YPDE BRANCH)");
        assert_eq!(comparison, "I(YPDE BRANCH)");
        let crate::netlist::measure::MeasureType::FileError { signal, .. } =
            &netlist.measurements[10].measure_type
        else {
            panic!("expected spaced ERROR");
        };
        assert_eq!(signal, "I(YPDE BRANCH)");
    }

    #[test]
    fn measure_occurrences_accept_zero_and_truncate_finite_values_toward_zero() {
        let netlist = Netlist::parse(
            "measurement occurrence conversion\n\
             V1 one 0 0\n\
             .tran 1 4\n\
             .measure tran found FIND V(one) WHEN V(one)=0 CROSS=0.9\n\
             .measure tran event WHEN V(one)=0 RISE=-2.9\n\
             .measure tran slope DERIV V(one) WHEN V(one)=0 FALL=3.8\n\
             .measure tran delay TRIG V(one)=0 CROSS=0 TARG V(one)=0 FALL=-3.8\n\
             .end\n",
        )
        .expect("finite fractional occurrence values parse");

        let occurrence = |index: usize| match &netlist.measurements[index].measure_type {
            crate::netlist::measure::MeasureType::Find {
                when: Some(condition),
                ..
            }
            | crate::netlist::measure::MeasureType::Derivative {
                when: Some(condition),
                ..
            }
            | crate::netlist::measure::MeasureType::When { condition, .. } => {
                condition.occurrence.number
            }
            other => panic!("expected point occurrence, got {other:?}"),
        };
        assert_eq!(occurrence(0), 0);
        assert_eq!(occurrence(1), -2);
        assert_eq!(occurrence(2), 3);
        let crate::netlist::measure::MeasureType::Delay { trig, targ, .. } =
            &netlist.measurements[3].measure_type
        else {
            panic!("expected delay occurrence");
        };
        let clause_number = |clause: &crate::netlist::measure::TrigSpec| {
            let crate::netlist::measure::TriggerEvent::When(condition) = &clause.event else {
                panic!("expected conditional delay clause");
            };
            condition.occurrence.number
        };
        assert_eq!(clause_number(trig), 0);
        assert_eq!(clause_number(targ), -3);

        for value in ["1e999", "1e30", "-1e30"] {
            let error = Netlist::parse(&format!(
                "invalid measurement occurrence\n\
                 V1 one 0 0\n\
                 .tran 1 2\n\
                 .measure tran invalid WHEN V(one)=0 CROSS={value}\n\
                 .end\n"
            ))
            .expect_err("non-finite or out-of-range occurrence must fail");
            assert!(
                error.to_string().contains("finite in-range occurrence"),
                "{value}: {error}"
            );
        }
    }

    #[test]
    fn conditional_measurements_preserve_statement_wide_minval() {
        let netlist = Netlist::parse(
            "statement-wide conditional minval\n\
             V1 one 0 0\n\
             V2 two 0 0\n\
             .dc V1 0 4 1\n\
             .measure dc found FIND V(one) WHEN V(one)=1 MINVAL=1e-15 RISE=1 MINVAL=2e-15\n\
             .measure dc event WHEN V(one)=2 MINVAL=3e-15 CROSS=1\n\
             .measure dc delay TRIG V(one)=1 CROSS=1 TARG V(two)=2 CROSS=1 MINVAL=4e-15\n\
             .end\n",
        )
        .expect("conditional MINVAL options parse");

        let crate::netlist::measure::MeasureType::Find {
            when: Some(_),
            minval,
            ..
        } = &netlist.measurements[0].measure_type
        else {
            panic!("expected FIND-WHEN measurement");
        };
        assert_eq!(*minval, 2.0e-15);

        let crate::netlist::measure::MeasureType::When { minval, .. } =
            &netlist.measurements[1].measure_type
        else {
            panic!("expected WHEN measurement");
        };
        assert_eq!(*minval, 3.0e-15);

        let crate::netlist::measure::MeasureType::Delay {
            trig, targ, minval, ..
        } = &netlist.measurements[2].measure_type
        else {
            panic!("expected TRIG/TARG measurement");
        };
        assert_eq!(*minval, 4.0e-15);
        for clause in [trig, targ] {
            let crate::netlist::measure::TriggerEvent::When(_) = &clause.event else {
                panic!("expected conditional TRIG/TARG clause");
            };
        }
    }

    #[test]
    fn conditional_measurement_minval_must_be_finite_and_non_negative() {
        for minval in ["-1e-15", "1e999"] {
            let source = format!(
                "invalid conditional minval\nV1 one 0 0\n.dc V1 0 1 1\n.measure dc event WHEN V(one)=0 MINVAL={minval}\n.end\n"
            );
            let error = Netlist::parse(&source).expect_err("invalid MINVAL must fail closed");
            assert!(
                error
                    .to_string()
                    .contains("MINVAL must be finite and non-negative"),
                "unexpected error for {minval}: {error}"
            );
        }
    }

    #[test]
    fn point_measurements_preserve_last_finite_td_with_optional_separator() {
        let netlist = Netlist::parse(
            "point measurement delay windows\n\
             V1 one 0 0\n\
             .tran 1n 10n\n\
             .measure tran found FIND V(one) WHEN V(one)=1 TD=-1n TD 2n\n\
             .measure tran event WHEN V(one)=2 TD 3n\n\
             .measure tran slope DERIV V(one) WHEN V(one)=3 TD=4n\n\
             .end\n",
        )
        .expect("point measurement TD options parse");

        let crate::netlist::measure::MeasureType::Find { td, .. } =
            &netlist.measurements[0].measure_type
        else {
            panic!("expected FIND measurement");
        };
        assert!(td.is_some_and(|td| (td - 2.0e-9).abs() < 1.0e-24));
        let crate::netlist::measure::MeasureType::When { td, .. } =
            &netlist.measurements[1].measure_type
        else {
            panic!("expected WHEN measurement");
        };
        assert!(td.is_some_and(|td| (td - 3.0e-9).abs() < 1.0e-24));
        let crate::netlist::measure::MeasureType::Derivative { td, .. } =
            &netlist.measurements[2].measure_type
        else {
            panic!("expected DERIV measurement");
        };
        assert!(td.is_some_and(|td| (td - 4.0e-9).abs() < 1.0e-24));
    }

    #[test]
    fn point_measurement_td_must_be_finite() {
        let error = Netlist::parse(
            "invalid point measurement delay\n\
             V1 one 0 0\n\
             .tran 1n 10n\n\
             .measure tran event WHEN V(one)=1 TD=1e999\n\
             .end\n",
        )
        .expect_err("non-finite TD must fail closed");
        assert!(error.to_string().contains(".MEAS TD must be finite"));
    }

    #[test]
    fn error_function_measurements_preserve_operands_norms_and_filters() {
        let netlist = Netlist::parse(
            "typed error functions\n\
             .global_param p1=2.5\n\
             V1 one 0 0\n\
             .dc V1 -5 5 1\n\
             .measure dc rms_error ERR1 PAR('V(one)*V(one)') V(one) FROM=4 TO=0 MINVAL=1.5 YMIN=2.5 YMAX=3.5 WEIGHT=2\n\
             .measure dc mean_error ERR2 V(one) {P1} IGNORE=2.5\n\
             .end\n",
        )
        .expect("ERR-family measurements parse");

        match &netlist.measurements[0].measure_type {
            crate::netlist::measure::MeasureType::ErrorFunction {
                measured,
                comparison,
                norm,
                from,
                to,
                minval,
                ymin,
                ymax,
                weight,
            } => {
                assert_eq!(measured, "{V(one)*V(one)}");
                assert_eq!(comparison, "V(ONE)");
                assert_eq!(
                    *norm,
                    crate::netlist::measure::ErrorFunctionNorm::RootMeanSquare
                );
                assert_eq!((*from, *to), (Some(4.0), Some(0.0)));
                assert_eq!((*minval, *ymin, *ymax), (1.5, 2.5, 3.5));
                assert_eq!(*weight, Some(2.0));
            }
            other => panic!("expected ERR1 measurement, got {other:?}"),
        }
        match &netlist.measurements[1].measure_type {
            crate::netlist::measure::MeasureType::ErrorFunction { norm, ymin, .. } => {
                assert_eq!(
                    *norm,
                    crate::netlist::measure::ErrorFunctionNorm::MeanAbsolute
                );
                assert_eq!(*ymin, 2.5);
            }
            other => panic!("expected ERR2 measurement, got {other:?}"),
        }
    }

    #[test]
    fn file_error_measurements_preserve_typed_options_and_deck_relative_paths() {
        let deck_path = std::env::temp_dir()
            .join("rspice-measure-file-error")
            .join("deck.cir");
        let netlist = Netlist::parse_with_path(
            "typed file error\n\
             V1 one 0 0\n\
             .dc V1 0 1 1\n\
             .measure dc fit ERROR V(one) FILE=Reference.MixedCase.prn COMP_FUNCTION INFNORM INDEPVARCOL -1 DEPVARCOL 2\n\
             .end\n",
            &deck_path,
        )
        .expect("file-backed ERROR measurement parses");

        match &netlist.measurements[0].measure_type {
            crate::netlist::measure::MeasureType::FileError {
                signal,
                file,
                norm,
                independent_column,
                dependent_column,
            } => {
                assert_eq!(signal, "V(ONE)");
                assert_eq!(
                    file,
                    &deck_path
                        .parent()
                        .expect("deck has parent")
                        .join("Reference.MixedCase.prn")
                        .to_string_lossy()
                );
                assert_eq!(*norm, crate::netlist::measure::FileErrorNorm::Infinity);
                assert_eq!(*independent_column, Some(-1));
                assert_eq!(*dependent_column, 2);
            }
            other => panic!("expected file-backed ERROR measurement, got {other:?}"),
        }
    }

    #[test]
    fn extrema_output_frequency_alias_selects_independent_axis() {
        let netlist = Netlist::parse(
            "extrema output frequency\n\
             V1 out 0 AC 1\n\
             .ac dec 5 1 1k\n\
             .measure ac peak_frequency max vm(out) output freq\n\
             .end\n",
        )
        .expect("OUTPUT FREQ parses without an equals sign");

        match &netlist.measurements[0].measure_type {
            crate::netlist::measure::MeasureType::Max { output, .. } => {
                assert_eq!(
                    *output,
                    crate::netlist::measure::ExtremaOutput::IndependentAxis
                )
            }
            other => panic!("expected MAX measurement, got {other:?}"),
        }
    }

    #[test]
    fn ngspice_extrema_at_aliases_select_the_independent_axis() {
        let netlist = Netlist::parse(
            "extrema at aliases\n\
             V1 out 0 PULSE(0 1 0 1n 1n 1u 2u)\n\
             .tran 1n 4u\n\
             .measure tran time_of_max MAX_AT V(out) FROM=1u TO=3u\n\
             .measure tran time_of_min MIN_AT V(out) FROM=1u TO=3u\n\
             .end\n",
        )
        .expect("ngspice MAX_AT and MIN_AT aliases parse");

        for measurement in &netlist.measurements {
            let output = match &measurement.measure_type {
                crate::netlist::measure::MeasureType::Max { output, .. }
                | crate::netlist::measure::MeasureType::Min { output, .. } => output,
                other => panic!("expected extrema measurement, got {other:?}"),
            };
            assert_eq!(
                *output,
                crate::netlist::measure::ExtremaOutput::IndependentAxis
            );
        }
    }

    #[test]
    fn fft_directive_is_typed_but_not_a_primary_analysis() {
        let netlist = Netlist::parse(
            "inactive fft under ac\n\
             V1 out 0 AC 1\n\
             .ac dec 5 1 1k\n\
             .fft v(out) np=8 window=hann format=unorm\n\
             .end\n",
        )
        .expect("valid .FFT parses under AC");

        assert_eq!(netlist.analyses.len(), 1);
        assert_eq!(netlist.fft_analyses.len(), 1);
        let fft = &netlist.fft_analyses[0];
        assert_eq!(fft.output, FftOutput::Probe("V(OUT)".to_string()));
        assert_eq!(fft.points, 8);
        assert_eq!(fft.window, FftWindow::Hann);
        assert_eq!(fft.window_name, "HANN");
        assert_eq!(fft.format, Some(FftFormat::Unnormalized));
        assert!(
            netlist
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code != "unsupported-dot-command")
        );
    }

    #[test]
    fn fft_aliases_duplicates_and_normalization_match_xyce() {
        let netlist = Netlist::parse(
            "fft qualifier semantics\n\
             V1 out 0 0\n\
             .fft {v(out)} np=11 window=black from=1 start=-2 to=3 stop=4 alfa=25 freq=5 fmin=1 fmax=10\n\
             .end\n",
        )
        .expect("Xyce-compatible .FFT aliases parse");

        let fft = &netlist.fft_analyses[0];
        assert_eq!(fft.output, FftOutput::Expression("v(out)".to_string()));
        assert_eq!(fft.points, 8);
        assert_eq!(fft.window, FftWindow::Blackman67Db);
        assert_eq!(fft.window_name, "BLACK");
        assert_eq!(fft.start, Some(0.0));
        assert_eq!(fft.stop, Some(4.0));
        assert_eq!(fft.alpha, 20.0);
        assert_eq!(fft.fundamental_frequency, Some(5.0));
        assert_eq!(fft.minimum_frequency, Some(1.0));
        assert_eq!(fft.maximum_frequency, Some(10.0));
        assert!(
            netlist
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "fft-points-normalized")
        );
        assert!(
            netlist
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "fft-start-clamped")
        );
    }

    #[test]
    fn xyce_fft_options_are_typed_without_unknown_option_diagnostics() {
        let netlist = Netlist::parse(
            "typed fft options\n\
             V1 out 0 0\n\
             .options fft fft_mode=1 fft_accurate=0 fftout=1\n\
             .fft v(out) np=8\n\
             .end\n",
        )
        .expect("Xyce FFT option package parses");

        assert_eq!(
            netlist.options.fft_mode,
            Some(XyceFftMode::SpectreCompatible)
        );
        assert_eq!(netlist.options.fft_accurate, Some(false));
        assert_eq!(netlist.options.fft_output_metrics, Some(true));
        assert!(
            netlist
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code != "unknown-option")
        );

        let error = Netlist::parse("invalid fft mode\nV1 out 0 0\n.options fft fft_mode=2\n.end\n")
            .expect_err("FFT_MODE outside 0/1 fails closed");
        assert!(
            error
                .to_string()
                .contains("FFT.FFT_MODE must be either 0 or 1")
        );

        let error = Netlist::parse("invalid fft flag\nV1 out 0 0\n.options fft fftout=2\n.end\n")
            .expect_err("FFTOUT outside 0/1 fails closed");
        assert!(
            error
                .to_string()
                .contains("FFT.FFTOUT must be the integer 0 or 1")
        );
    }

    #[test]
    fn malformed_fft_directives_fail_closed() {
        for (line, expected) in [
            (".fft", "requires one output"),
            (".fft np=64 v(out)", "parenthesized probe"),
            (".fft v(out) np 8", "requires '='"),
            (".fft v(out) np=", "missing its value"),
            (".fft v(out) bogo=2", "Unknown .FFT qualifier"),
            (".fft v(out) format=bogo", "Invalid FORMAT"),
            (".fft v(out) window=gauss", "Invalid WINDOW"),
            (".fft v(out) np=0", ".FFT NP must be positive"),
            (".fft v(out) v(alt)", "requires '='"),
            (".fft {}", "expression must not be empty"),
        ] {
            let error = Netlist::parse(&format!(
                "invalid fft\nV1 out 0 0\nV2 alt 0 0\n{line}\n.end\n"
            ))
            .expect_err("malformed .FFT must fail");
            assert!(
                error.to_string().contains(expected),
                "line={line}, expected={expected}, error={error}"
            );
        }
    }

    #[test]
    fn dc_sweep_rejects_omitted_step_for_equal_bounds() {
        let error = Netlist::parse(
            "single point dc\n\
             VIN 1 0 DC 5\n\
             R1 1 0 1k\n\
             .dc VIN 5 5\n\
             .end\n",
        )
        .expect_err("equal-bound .DC sweep still requires an explicit step value");

        assert!(
            error
                .to_string()
                .contains(".DC linear sweep requires a step value"),
            "unexpected parse error: {error}"
        );
    }

    #[test]
    fn dc_sweep_rejects_omitted_step_for_distinct_bounds() {
        let err = Netlist::parse(
            "invalid dc\n\
             VIN 1 0 DC 5\n\
             R1 1 0 1k\n\
             .dc VIN 0 5\n\
             .end\n",
        )
        .expect_err("distinct-bound .DC sweep still requires a step value");

        assert!(
            err.to_string()
                .contains(".DC linear sweep requires a step value"),
            "unexpected parse error: {err}"
        );
    }

    #[test]
    fn dc_sweep_rejects_omitted_step_for_explicit_linear_modes() {
        for directive in [".dc LIN VIN 5 5", ".dc VIN LIN 5 5"] {
            let error = Netlist::parse(&format!(
                "invalid explicit linear dc\n\
                 VIN 1 0 DC 5\n\
                 R1 1 0 1k\n\
                 {directive}\n\
                 .end\n"
            ))
            .expect_err("explicit LIN .DC sweep requires a step value");

            assert!(
                error
                    .to_string()
                    .contains(".DC linear sweep requires a step value"),
                "directive={directive}, unexpected parse error: {error}"
            );
        }
    }

    fn assert_parameter_probe_error(directive: &str, expected_line: usize, expected_message: &str) {
        let error = Netlist::parse(&format!(
            "invalid parameter probe\n\
             {directive}\n\
             V1 1 0 1\n\
             .end\n"
        ))
        .expect_err("circuit probes in parameter expressions must be rejected");
        match error {
            ParseError::Syntax { line, message } => {
                assert_eq!(line, expected_line);
                assert_eq!(message, expected_message);
            }
            other => panic!("expected structured parameter-probe syntax error, got {other:?}"),
        }
    }

    #[test]
    fn parameter_directives_reject_xyce_circuit_probe_classes() {
        for (directive, expected_message) in [
            (
                ".param RVAL={76K+v(3)}",
                "Node Voltage may not be used in parameter expression (RVAL): V(3)",
            ),
            (
                ".csparam rval={76K+i(v2)}",
                "Device Current may not be used in parameter expression (RVAL): I(V2)",
            ),
            (
                ".global_param Rval={76K+i(c2)}",
                "Lead Current may not be used in parameter expression (RVAL): I(C2)",
            ),
        ] {
            assert_parameter_probe_error(directive, 2, expected_message);
        }
    }

    #[test]
    fn malformed_parameter_probe_calls_preserve_ordinary_error_ordering() {
        for expression in [
            "V()", "V(a,b,c)", "V(1+2)", "I()", "I(v1,v2)", "I(3)", "I(v1+v2)",
        ] {
            let error = Netlist::parse(&format!(
                "malformed parameter call\n\
                 .param RVAL={{{expression}}}\n\
                 V1 1 0 1\n\
                 .end\n"
            ))
            .expect_err("malformed V/I call must fail through ordinary expression validation");
            assert!(
                matches!(
                    error,
                    ParseError::Syntax { .. }
                        | ParseError::InvalidValue(_)
                        | ParseError::UndefinedParameter(_)
                ),
                "unexpected malformed-call error for {expression}: {error:?}"
            );
            assert!(
                !error
                    .to_string()
                    .contains("may not be used in parameter expression"),
                "malformed {expression} must not be reclassified as a valid circuit probe: {error}"
            );
        }
    }

    #[test]
    fn parameter_function_definitions_may_retain_probes_for_behavioral_use() {
        Netlist::parse(
            "valid parameter function probe\n\
             .param F(x)={x+v(1)}\n\
             V1 1 0 1\n\
             B1 2 0 V={F(2)}\n\
             .end\n",
        )
        .expect("Xyce permits probes in a .PARAM function body outside scalar assignment");
    }

    #[test]
    fn parameter_assignments_reject_probes_expanded_from_func_bodies() {
        let error = Netlist::parse(
            "invalid nested function probe\n\
             .func f(x)={x+i(c2)}\n\
             .param RVAL={f(1)}\n\
             V1 1 0 1\n\
             .end\n",
        )
        .expect_err("a circuit probe expanded from .FUNC must be rejected in .PARAM");
        match error {
            ParseError::Syntax { line, message } => {
                assert_eq!(line, 3);
                assert_eq!(
                    message,
                    "Lead Current may not be used in parameter expression (RVAL): I(C2)"
                );
            }
            other => panic!("expected structured nested parameter-probe error, got {other:?}"),
        }
    }

    #[test]
    fn parameter_probe_validation_preserves_runtime_symbols() {
        let netlist = Netlist::parse(
            "valid runtime global parameter\n\
             .global_param time_only={time}\n\
             .global_param freq_only={freq}\n\
             .global_param combined={time+freq+temper}\n\
             V1 1 0 1\n\
             .end\n",
        )
        .expect("TIME, FREQ, and TEMPER are not circuit probes");
        assert_eq!(
            netlist.params.get_global_expression("TIME_ONLY"),
            Some("time")
        );
        assert_eq!(
            netlist.params.get_global_expression("FREQ_ONLY"),
            Some("freq")
        );
        assert_eq!(
            netlist.params.get_global_expression("COMBINED"),
            Some("time+freq+temper")
        );
    }

    fn assert_duplicate_element_error(
        source: &str,
        expected_canonical: &str,
        expected_first: &str,
        expected_duplicate: &str,
        expected_scope: &str,
        expected_first_line: usize,
        expected_duplicate_line: usize,
    ) {
        let error = Netlist::parse(source).expect_err("duplicate element name must be rejected");
        match error {
            ParseError::DuplicateName {
                canonical_name,
                first_name,
                duplicate_name,
                scope,
                first_line,
                duplicate_line,
            } => {
                assert_eq!(canonical_name, expected_canonical);
                assert_eq!(first_name, expected_first);
                assert_eq!(duplicate_name, expected_duplicate);
                assert_eq!(scope, expected_scope);
                assert_eq!(first_line, expected_first_line);
                assert_eq!(duplicate_line, expected_duplicate_line);
            }
            other => panic!("expected structured duplicate-name error, got {other:?}"),
        }
    }

    #[test]
    fn duplicate_element_names_are_rejected_case_insensitively_at_top_level() {
        for duplicate in ["V1", "v1"] {
            assert_duplicate_element_error(
                &format!(
                    "top-level duplicate\n\
                     V1 1 0 1\n\
                     {duplicate} 2 0 2\n\
                     .end\n"
                ),
                "V1",
                "V1",
                duplicate,
                "TOP_LEVEL",
                2,
                3,
            );
        }
    }

    #[test]
    fn duplicate_element_names_are_rejected_case_insensitively_within_subcircuits() {
        for duplicate in ["R1", "r1"] {
            assert_duplicate_element_error(
                &format!(
                    "subcircuit duplicate\n\
                     .subckt cell a b\n\
                     R1 a b 1\n\
                     {duplicate} b 0 2\n\
                     .ends\n\
                     .end\n"
                ),
                "R1",
                "R1",
                duplicate,
                "SUBCIRCUIT:CELL",
                3,
                4,
            );
        }
    }

    #[test]
    fn duplicate_element_error_preserves_logical_statement_line_provenance() {
        assert_duplicate_element_error(
            "duplicate provenance\n\
             V1 1 0 1\n\
             \n\
             * intervening comment\n\
             v1 2 0\n\
             + DC 2\n\
            .end\n",
            "V1",
            "V1",
            "v1",
            "TOP_LEVEL",
            2,
            5,
        );
    }

    #[test]
    fn element_names_may_be_reused_across_independent_scopes_and_instances() {
        let netlist = Netlist::parse(
            "legal scoped reuse\n\
             R1 top 0 1\n\
             .subckt left a b\n\
             R1 a b 2\n\
             .ends\n\
             .subckt right a b\n\
             r1 a b 3\n\
             .ends\n\
             X1 top 0 left\n\
             X2 top 0 left\n\
             X3 top 0 right\n\
             .end\n",
        )
        .expect("element names may repeat in distinct lexical and instance scopes");

        let flattened = flatten_netlist_with_models(&netlist).expect("scoped reuse flattens");
        let names = flattened
            .elements
            .iter()
            .map(|element| element.name.to_ascii_uppercase())
            .collect::<HashSet<_>>();
        for expected in ["R1", "X1.R1", "X2.R1", "X3.R1"] {
            assert!(
                names.contains(expected),
                "flattened scoped element {expected} is preserved"
            );
        }
    }

    #[test]
    fn element_names_may_be_reused_between_parent_and_nested_subcircuits() {
        Netlist::parse(
            "legal nested reuse\n\
             .subckt outer a b\n\
             R1 a b 1\n\
             .subckt inner c d\n\
             r1 c d 2\n\
             .ends inner\n\
             XINNER a b inner\n\
             .ends outer\n\
             XOUT 1 0 outer\n\
             .end\n",
        )
        .expect("parent and nested subcircuits own independent element-name scopes");
    }

    #[test]
    fn parses_bare_model_flags_as_enabled_parameters() {
        let netlist = Netlist::parse(
            "flag model\n\
             o1 1 0 2 0 lline\n\
             .model lline ltra rel=1 r=12.45 g=0 l=8.972e-9 c=0.468e-12\n\
             + len=16 steplimit compactrel=1.0e-3 compactabs=1.0e-14\n\
             .tran 0.2n 1n\n\
             .end\n",
        )
        .expect("netlist parses");

        let model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("lline"))
            .expect("model exists");

        assert!(model.params.iter().any(|(name, value)| {
            name.eq_ignore_ascii_case("steplimit") && (*value - 1.0).abs() < f64::EPSILON
        }));
    }

    #[test]
    fn transient_command_accepts_bare_seconds_units() {
        let netlist = Netlist::parse(
            "bare seconds transient\n\
             v1 in 0 dc 1\n\
             r1 in 0 1k\n\
             .tran .1s 10s\n\
             .end\n",
        )
        .expect("bare seconds units parse in .TRAN");

        let tran = netlist
            .analyses
            .iter()
            .find_map(|analysis| match analysis {
                AnalysisCommand::Tran { step, stop, .. } => Some((*step, *stop)),
                _ => None,
            })
            .expect(".TRAN exists");

        assert_eq!(tran, (0.1, 10.0));
    }

    #[test]
    fn control_block_tran_is_promoted_with_uic_and_csparam_substitution() {
        let netlist = Netlist::parse(
            "control tran promotion\n\
             v1 in 0 dc 1\n\
             r1 in 0 1k\n\
             .csparam simtime=25u\n\
             .control\n\
             save in\n\
             tran 0.1n $&simtime uic\n\
             .endc\n\
             .end\n",
        )
        .expect("control-block transient analysis parses");

        let tran = netlist
            .analyses
            .iter()
            .find_map(|analysis| match analysis {
                AnalysisCommand::Tran {
                    step,
                    stop,
                    start,
                    max_step,
                    uic,
                } => Some((*step, *stop, *start, *max_step, *uic)),
                _ => None,
            })
            .expect("promoted .TRAN exists");

        assert!((tran.0 - 0.1e-9).abs() <= 1.0e-21);
        assert!((tran.1 - 25.0e-6).abs() <= 1.0e-18);
        assert_eq!(tran.2, None);
        assert_eq!(tran.3, None);
        assert!(tran.4);
        assert_eq!(
            netlist.saves.signals,
            vec![SaveSignal::Raw("in".to_string())]
        );
    }

    #[test]
    fn control_scalar_let_can_supply_promoted_analysis_bounds() {
        let netlist = Netlist::parse(
            "control let promotion\n\
             .param stime=10n\n\
             v1 in 0 1\n\
             r1 in 0 1k\n\
             .control\n\
             let deltime = stime/100\n\
             let waveform = v(in)\n\
             tran $&deltime $&stime uic\n\
             .endc\n\
             .end\n",
        )
        .expect("scalar let supplies the promoted transient command");

        let (step, stop) = netlist
            .analyses
            .iter()
            .find_map(|analysis| match analysis {
                AnalysisCommand::Tran { step, stop, .. } => Some((*step, *stop)),
                _ => None,
            })
            .expect("promoted .TRAN exists");
        assert!((step - 0.1e-9).abs() <= 1.0e-21);
        assert!((stop - 10.0e-9).abs() <= 1.0e-21);
        assert_eq!(netlist.params.get("WAVEFORM"), None);
    }

    #[test]
    fn control_scalar_let_promotion_excludes_mutable_and_vector_dependent_values() {
        let netlist = Netlist::parse(
            "control mutable let filtering\n\
             v1 in 0 1\n\
             r1 in 0 1k\n\
             .control\n\
             let start_r = 1k\n\
             let r_act = start_r\n\
             let spectrum = fft(v(in))\n\
             let rms_spectrum = sqrt(mean(spectrum*spectrum))\n\
             let percent = 100*rms_spectrum\n\
             let r_act = r_act + start_r\n\
             dc v1 0 1 .1\n\
             .endc\n\
             .end\n",
        )
        .expect("mutable and vector-valued control lets stay in the ignored script");

        assert_eq!(netlist.params.get("START_R"), None);
        assert_eq!(netlist.params.get("R_ACT"), None);
        assert_eq!(netlist.params.get("RMS_SPECTRUM"), None);
        assert_eq!(netlist.params.get("PERCENT"), None);
    }

    #[test]
    fn reassigned_control_scalars_are_inlined_at_each_analysis() {
        let netlist = Netlist::parse(
            "control scalar snapshots\n\
             .param stime=10n\n\
             v1 in 0 1\n\
             r1 in 0 1k\n\
             .control\n\
             let deltime = stime/100\n\
             tran $&deltime $&stime\n\
             let newstime = stime/2\n\
             let deltime = newstime/100\n\
             tran $&deltime $&newstime\n\
             .endc\n\
             .end\n",
        )
        .expect("each promoted analysis receives its current scalar values");

        let transients = netlist
            .analyses
            .iter()
            .filter_map(|analysis| match analysis {
                AnalysisCommand::Tran { step, stop, .. } => Some((*step, *stop)),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(transients.len(), 2);
        assert!((transients[0].0 - 0.1e-9).abs() <= 1.0e-21);
        assert!((transients[0].1 - 10.0e-9).abs() <= 1.0e-21);
        assert!((transients[1].0 - 0.05e-9).abs() <= 1.0e-21);
        assert!((transients[1].1 - 5.0e-9).abs() <= 1.0e-21);
    }

    #[test]
    fn control_measure_inlines_bare_scalar_let_bounds() {
        let netlist = Netlist::parse(
            "control measure scalar snapshots\n\
             v0 in 0 1\n\
             r1 in 0 1k\n\
             .tran 10p 2n\n\
             .control\n\
             let dfall = 100p\n\
             let period = 1n\n\
             let delta = dfall+period\n\
             meas tran v0_min min i(v0) from=dfall to=delta\n\
             .endc\n\
             .end\n",
        )
        .expect("control measurement receives current scalar let values");

        assert_eq!(netlist.measurements.len(), 1);
        assert_eq!(netlist.params.get("DFALL"), None);
        assert_eq!(netlist.params.get("DELTA"), None);
    }

    #[test]
    fn behavioral_passive_value_retains_expanded_netlist_parameters() {
        let netlist = Netlist::parse(
            "behavioral passive parameter scope\n\
             .param cn=16n\n\
             vctrl ctrl 0 0\n\
             c1 out 0 c='cn + 0.033*cn*v(ctrl)'\n\
             r1 out 0 1k\n\
             .tran 1n 10n\n\
             .end\n",
        )
        .expect("parameterized behavioral capacitor parses");

        let expression = netlist
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Capacitor {
                    value_expr: Some(expression),
                    ..
                } => Some(expression),
                _ => None,
            })
            .expect("solution-dependent capacitor expression is retained");
        assert!(expression.to_ascii_lowercase().contains("v(ctrl)"));
        assert!(
            !expression.to_ascii_lowercase().contains("cn"),
            "parser-only parameter leaked into runtime expression: {expression}"
        );
    }

    #[test]
    fn control_block_promotes_core_analyses_and_measurements() {
        let netlist = Netlist::parse(
            "control analysis promotion\n\
             v1 in 0 dc 1 ac 1\n\
             r1 in out 1k\n\
             r2 out 0 1k\n\
             .csparam stopv=1\n\
             .control\n\
             op\n\
             dc v1 0 $&stopv 0.5\n\
             ac dec 3 1 1k\n\
             sp lin 4 10meg 20meg 1\n\
             meas ac gainmax max v(out) from=1 to=1k\n\
             .endc\n\
             .end\n",
        )
        .expect("control-block core analyses and measurement parse");

        assert!(
            netlist
                .analyses
                .iter()
                .any(|analysis| matches!(analysis, AnalysisCommand::Op))
        );
        assert!(netlist.analyses.iter().any(|analysis| {
            matches!(
                analysis,
                AnalysisCommand::Dc {
                    source,
                    start,
                    stop,
                    step,
                    ..
                } if source.eq_ignore_ascii_case("v1")
                    && *start == 0.0
                    && (*stop - 1.0).abs() <= 1.0e-12
                    && (*step - 0.5).abs() <= 1.0e-12
            )
        }));
        assert!(netlist.analyses.iter().any(|analysis| {
            matches!(
                analysis,
                AnalysisCommand::Ac {
                    points,
                    start_freq,
                    stop_freq,
                    ..
                } if *points == 3
                    && (*start_freq - 1.0).abs() <= 1.0e-12
                    && (*stop_freq - 1.0e3).abs() <= 1.0e-9
            )
        }));
        assert!(netlist.analyses.iter().any(|analysis| {
            matches!(
                analysis,
                AnalysisCommand::Sp {
                    points,
                    start_freq,
                    stop_freq,
                    do_noise,
                    ..
                } if *points == 4
                    && (*start_freq - 10.0e6).abs() <= 1.0e-6
                    && (*stop_freq - 20.0e6).abs() <= 1.0e-6
                    && *do_noise
            )
        }));
        assert_eq!(netlist.measurements.len(), 1);
        assert_eq!(netlist.measurements[0].analysis, "AC");
        assert_eq!(netlist.measurements[0].name, "GAINMAX");
    }

    #[test]
    fn control_block_digital_delay_type_set_promotes_option() {
        let netlist = Netlist::parse(
            "control xspice digital delay policy\n\
             v1 in 0 dc 1\n\
             r1 in 0 1k\n\
             .control\n\
             set noaskquit digital_delay_type = 3\n\
             .endc\n\
             .end\n",
        )
        .expect("control set digital_delay_type promotes to .options");

        assert_eq!(netlist.options.digital_delay_type, Some(3));
    }

    #[test]
    fn control_block_invalid_digital_delay_type_fails_closed() {
        let err = Netlist::parse(
            "invalid control xspice digital delay policy\n\
             v1 in 0 dc 1\n\
             r1 in 0 1k\n\
             .control\n\
             set digital_delay_type=4\n\
             .endc\n\
             .end\n",
        )
        .expect_err("invalid promoted digital_delay_type must fail parsing");

        assert!(
            err.to_string().contains("DIGITAL_DELAY_TYPE"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn control_block_xtrtol_promotes_transient_tolerance_option() {
        let set_netlist = Netlist::parse(
            "control xspice trtol set\n\
             v1 in 0 dc 1\n\
             r1 in 0 1k\n\
             .control\n\
             set xtrtol=2\n\
             .endc\n\
             .end\n",
        )
        .expect("control set xtrtol promotes to .options trtol");
        assert_eq!(set_netlist.options.trtol, Some(2.0));

        let option_netlist = Netlist::parse(
            "control option trtol\n\
             v1 in 0 dc 1\n\
             r1 in 0 1k\n\
             .control\n\
             option trtol=1\n\
             .endc\n\
             .end\n",
        )
        .expect("control option trtol promotes to .options trtol");
        assert_eq!(option_netlist.options.trtol, Some(1.0));
    }

    #[test]
    fn control_block_option_promotes_ngspice_xmu() {
        let netlist = Netlist::parse(
            "control modified trapezoidal damping\n\
             v1 in 0 dc 1\n\
             r1 in 0 1k\n\
             .control\n\
             option reltol=1e-4 xmu = .49 trtol=2\n\
             .endc\n\
             .end\n",
        )
        .expect("control option XMU promotes to an authored simulation option");

        assert_eq!(netlist.options.xmu, Some(0.49));
        assert_eq!(netlist.options.trtol, Some(2.0));
        assert!(netlist.options.reltol.is_none());
    }

    #[test]
    fn control_block_codemodel_fails_closed() {
        let err = Netlist::parse(
            "control xspice codemodel loader\n\
             v1 in 0 dc 1\n\
             r1 in 0 1k\n\
             .control\n\
             codemodel ./custom.cm\n\
             .endc\n\
             .end\n",
        )
        .expect_err("dynamic XSPICE codemodel loading must fail closed");

        let message = err.to_string();
        assert!(message.contains(".CODEMODEL"), "unexpected error: {err}");
        assert!(message.contains("custom.cm"), "unexpected error: {err}");
        assert!(message.contains("does not yet load arbitrary .cm/MIF libraries"));
    }

    #[test]
    fn dot_codemodel_fails_closed() {
        let err = Netlist::parse(
            "dot xspice codemodel loader\n\
             .codemodel ./custom.cm\n\
             v1 in 0 dc 1\n\
             r1 in 0 1k\n\
             .end\n",
        )
        .expect_err("explicit dynamic XSPICE codemodel loading must fail closed");

        let message = err.to_string();
        assert!(message.contains(".CODEMODEL"), "unexpected error: {err}");
        assert!(message.contains("custom.cm"), "unexpected error: {err}");
    }

    #[test]
    fn dot_codemodel_builtin_bundle_is_compatibility_noop() {
        Netlist::parse(
            "dot xspice builtin codemodel loader\n\
             .codemodel /usr/lib/ngspice/analog.cm digital.cm\n\
             v1 in 0 dc 1\n\
             r1 in 0 1k\n\
             .end\n",
        )
        .expect("standard ngspice built-in .cm bundle directives are no-ops");
    }

    #[test]
    fn control_block_codemodel_builtin_bundle_is_compatibility_noop() {
        Netlist::parse(
            "control xspice builtin codemodel loader\n\
             v1 in 0 dc 1\n\
             r1 in 0 1k\n\
             .control\n\
             codemodel \"C:\\ngspice\\lib\\xtradev.cm\" ./xtraevt.cm\n\
             .endc\n\
             .end\n",
        )
        .expect("control codemodel accepts standard built-in .cm bundles");
    }

    #[test]
    fn control_block_auto_bridge_template_set_promotes_option() {
        let netlist = Netlist::parse(
            "control xspice auto bridge template\n\
             v1 in 0 dc 1\n\
             r1 in 0 1k\n\
             .control\n\
             set auto_bridge_d_out = ( \".model auto_dac dac_bridge(out_low = 0 out_high = %g)\" \"auto_dac%d [ %s ] [ %s ] auto_dac\" 1 )\n\
             .endc\n\
             .end\n",
        )
        .expect("control set auto_bridge_d_out promotes to a structured template");

        let template = netlist
            .options
            .auto_bridge_templates
            .iter()
            .find(|template| template.key.eq_ignore_ascii_case("auto_bridge_d_out"))
            .expect("promoted auto_bridge_d_out template exists");

        assert_eq!(
            template.setup_card,
            ".model auto_dac dac_bridge(out_low = 0 out_high = %g)"
        );
        assert_eq!(template.device_card, "auto_dac%d [ %s ] [ %s ] auto_dac");
        assert_eq!(template.max_nodes, Some(1));
    }

    #[test]
    fn control_block_auto_bridge_param_set_promotes_option() {
        let netlist = Netlist::parse(
            "control xspice auto bridge parameter selector\n\
             v1 in 0 dc 1\n\
             r1 in 0 1k\n\
             .control\n\
             set auto_bridge_parm_d = vdd\n\
             .endc\n\
             .end\n",
        )
        .expect("control set auto_bridge_parm_d promotes to a structured selector");

        assert_eq!(netlist.options.auto_bridge_param_name("d"), Some("vdd"));
    }

    #[test]
    fn control_block_no_auto_bridge_family_set_promotes_option() {
        let netlist = Netlist::parse(
            "control xspice auto bridge family disable\n\
             v1 in 0 dc 1\n\
             r1 in 0 1k\n\
             .control\n\
             set no_auto_bridge_family\n\
             .endc\n\
             .end\n",
        )
        .expect("control set no_auto_bridge_family promotes to a structured option");

        assert_eq!(netlist.options.auto_bridge_family, Some(false));
    }

    #[test]
    fn param_statements_accept_unbraced_expression_rhs() {
        let netlist = Netlist::parse(
            "unbraced param expression\n\
             .param fact = 0.05\n\
             .param tgain = 1. + (TEMPER / 27. - 1.) * {fact} next=3\n\
             .end\n",
        )
        .expect("ngspice-style unbraced .param arithmetic should parse");

        let tgain = netlist
            .params
            .get("tgain")
            .expect("tgain parameter should be set");
        let next = netlist
            .params
            .get("next")
            .expect("following parameter should not be consumed by tgain");

        assert!((tgain - 1.0).abs() < f64::EPSILON);
        assert!((next - 3.0).abs() < f64::EPSILON);
    }

    #[test]
    fn param_statements_preserve_naked_if_comparison_operators() {
        let netlist = Netlist::parse(
            "naked Xyce IF parameter expressions\n\
             .param A = 1.0\n\
             .param B = 2.0\n\
             .param C = 3.0\n\
             .param D = 4.0\n\
             .param eq = if(A==B,C,D)\n\
             .param ge = if(A>=B,C,D)\n\
             .param le = if(A<=B,C,D)\n\
             .param ne = if(A!=B,C,D)\n\
             .end\n",
        )
        .expect("naked IF comparison operators should parse");

        assert_eq!(netlist.params.get("eq"), Some(4.0));
        assert_eq!(netlist.params.get("ge"), Some(4.0));
        assert_eq!(netlist.params.get("le"), Some(3.0));
        assert_eq!(netlist.params.get("ne"), Some(3.0));
    }

    #[test]
    fn param_statements_preserve_naked_ternary_operators() {
        let netlist = Netlist::parse(
            "naked Xyce ternary parameter expressions\n\
             .param A = 4.0\n\
             .param B = 3.0\n\
             .param C = 2.0\n\
             .param D = 1.0\n\
             .param gt = (A>B)?(C):D\n\
             .param ge = (A>=B)?(C):D\n\
             .param le = (A<=B)?(C):D\n\
             .param ne = (A!=B)?(C):D\n\
             .end\n",
        )
        .expect("naked ternary comparison operators should parse");

        assert_eq!(netlist.params.get("gt"), Some(2.0));
        assert_eq!(netlist.params.get("ge"), Some(2.0));
        assert_eq!(netlist.params.get("le"), Some(1.0));
        assert_eq!(netlist.params.get("ne"), Some(2.0));
    }

    #[test]
    fn model_param_rhs_identifier_is_not_reinterpreted_as_bare_flag() {
        let err = Netlist::parse(
            "bad model rhs\n\
             .model dmod D(IS=missing N=1)\n\
             .end\n",
        )
        .expect_err("unresolved model parameter RHS must be rejected");

        let message = err.to_string();
        let lowered = message.to_ascii_lowercase();
        assert!(
            lowered.contains("model parameter 'is'") && lowered.contains("missing"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn model_param_rhs_error_reports_deck_line() {
        let err = Netlist::parse(
            "bad model rhs\n\
             R1 a b 1k\n\
             C1 b 0 1p\n\
             .model dmod D(IS=missing N=1)\n\
             .end\n",
        )
        .expect_err("bad model RHS must report source line");

        let message = err.to_string();
        assert!(
            message.contains("line 4"),
            "expected deck line 4 in error, got: {message}"
        );
    }

    #[test]
    fn model_version_accepts_x_y_z_string_values() {
        let netlist = Netlist::parse(
            "dotted model version\n\
             M1 d g 0 0 n9 W=1u L=180n\n\
             .model n9 nmos level=9 version=3.2.2 tox=4.1n\n\
             .end\n",
        )
        .expect("dotted VERSION values are legal BSIM/Xyce model metadata");

        let model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("n9"))
            .expect("model exists");
        assert!(model.params.iter().any(|(name, value)| {
            name.eq_ignore_ascii_case("level") && (*value - 9.0).abs() < f64::EPSILON
        }));
        assert!(
            model
                .string_params
                .iter()
                .any(|(name, value)| { name.eq_ignore_ascii_case("version") && value == "3.2.2" })
        );
    }

    #[test]
    fn non_version_model_params_reject_dotted_numeric_tails() {
        let err = Netlist::parse(
            "bad dotted model param\n\
             D1 out 0 dmod\n\
             .model dmod D(IS=1.2.3 N=1)\n\
             .op\n\
             .end\n",
        )
        .expect_err("only VERSION accepts multi-dot metadata values");

        let message = err.to_string();
        assert!(
            message.contains(".MODEL") || message.contains("model parameter"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn model_vector_params_parse_decimal_vectors() {
        let netlist = Netlist::parse(
            "xspice vector model params\n\
             .model lut pwl (x_array=[-1 0 0.5 2] y_array=[0 -2 4 8])\n\
             .end\n",
        )
        .expect("model vector parameters parse");

        let model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("lut"))
            .expect("model exists");
        let x_array = model
            .real_vector_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("x_array"))
            .map(|(_, values)| values.as_slice())
            .expect("x_array exists");
        let y_array = model
            .real_vector_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("y_array"))
            .map(|(_, values)| values.as_slice())
            .expect("y_array exists");

        assert_eq!(x_array, &[-1.0, 0.0, 0.5, 2.0]);
        assert_eq!(y_array, &[0.0, -2.0, 4.0, 8.0]);
    }

    #[test]
    fn model_vector_params_store_integer_literals_as_numeric_vectors() {
        let netlist = Netlist::parse(
            "xspice integer-looking vector model params\n\
             .model lut d_lut (table_values=[0 1 1 0])\n\
             .end\n",
        )
        .expect("integer-looking vector parameters parse as numeric vectors");

        let model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("lut"))
            .expect("model exists");
        let table_values = model
            .real_vector_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("table_values"))
            .map(|(_, values)| values.as_slice())
            .expect("table_values exists");

        assert_eq!(table_values, &[0.0, 1.0, 1.0, 0.0]);
    }

    #[test]
    fn model_vector_params_accept_commas_signed_values_and_suffixes() {
        let netlist = Netlist::parse(
            "xspice vector model params with punctuation\n\
             .param scale=2\n\
             .model lut pwl (points=[-.14, 1u, -2, scale])\n\
             .end\n",
        )
        .expect("punctuated vector parameters parse");

        let model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("lut"))
            .expect("model exists");
        let points = model
            .real_vector_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("points"))
            .map(|(_, values)| values.as_slice())
            .expect("points exists");

        assert_eq!(points, &[-0.14, 1e-6, -2.0, 2.0]);
    }

    #[test]
    fn model_vector_params_accept_suffix_and_param_as_first_numeric_elements() {
        let netlist = Netlist::parse(
            "xspice vector model params starting with numeric-like idents\n\
             .param scale=0.5\n\
             .model os oneshot (pw_array=[1n 2n] cntl_array=[scale 1])\n\
             .end\n",
        )
        .expect("numeric-like leading vector elements parse as real vectors");

        let model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("os"))
            .expect("model exists");
        let pw_array = model
            .real_vector_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("pw_array"))
            .map(|(_, values)| values.as_slice())
            .expect("pw_array exists");
        let cntl_array = model
            .real_vector_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("cntl_array"))
            .map(|(_, values)| values.as_slice())
            .expect("cntl_array exists");

        assert_eq!(pw_array, &[1.0e-9, 2.0e-9]);
        assert_eq!(cntl_array, &[0.5, 1.0]);
    }

    #[test]
    fn model_params_accept_unparenthesized_trailing_close() {
        let netlist = Netlist::parse(
            "xspice unparenthesized model close\n\
             .model fil1 s_xfer gain=1000 int_ic=[0 0]\n\
             + num_coeff=[1.0 0]\n\
             + den_coeff=[1.0 1e3 1e7]\n\
             + )\n\
             .end\n",
        )
        .expect("ngspice accepts a trailing ')' after unparenthesized model params");

        let model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("fil1"))
            .expect("model exists");

        assert!(model.params.iter().any(|(name, value)| {
            name.eq_ignore_ascii_case("gain") && (*value - 1000.0).abs() < f64::EPSILON
        }));
        assert!(model.real_vector_params.iter().any(|(name, values)| {
            name.eq_ignore_ascii_case("den_coeff") && values == &[1.0, 1.0e3, 1.0e7]
        }));
    }

    #[test]
    fn xspice_model_params_accept_missing_close_at_line_end() {
        let netlist = Netlist::parse(
            "xspice missing model close\n\
             .model dac1 dac_bridge(out_low = -1 out_high = 1 out_undef = 0\n\
             + input_load = 5.0e-12\n\
             .end\n",
        )
        .expect("ngspice accepts unterminated parenthesized XSPICE model params");

        let model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("dac1"))
            .expect("model exists");

        assert!(model.params.iter().any(|(name, value)| {
            name.eq_ignore_ascii_case("out_high") && (*value - 1.0).abs() < f64::EPSILON
        }));
        assert!(model.params.iter().any(|(name, value)| {
            name.eq_ignore_ascii_case("input_load") && (*value - 5.0e-12).abs() < 1.0e-24
        }));
    }

    #[test]
    fn xspice_string_vector_params_preserve_unquoted_argv_tokens() {
        let netlist = Netlist::parse(
            "xspice string-vector argv model params\n\
             .model co d_cosim simulation=\"ivlng\" sim_args=[1e3 deck --payload -gTarget=4500 +define=1 ./dut]\n\
             .end\n",
        )
        .expect("bare d_cosim string-vector parameters parse");

        let model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("co"))
            .expect("model exists");
        let sim_args = model
            .string_vector_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("sim_args"))
            .map(|(_, values)| values.as_slice())
            .expect("sim_args exists");

        assert_eq!(
            sim_args,
            &[
                "1e3",
                "deck",
                "--payload",
                "-gTarget=4500",
                "+define=1",
                "./dut"
            ]
        );
    }

    #[test]
    fn xspice_model_params_accept_ngspice_complex_literals() {
        let netlist = Netlist::parse(
            "xspice complex model params\n\
             .model mod print_param_types (complex=<4.0 5.0>\n\
             + string=six\n\
             + real_array=[9.0 10.0]\n\
             + complex_array=[< 11.0 12.0 > < 13.0 14.0 >]\n\
             + string_array=[fifteen sixteen])\n\
             .end\n",
        )
        .expect("official ngspice complex model params parse");

        let model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("mod"))
            .expect("model exists");
        let complex = model
            .string_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("complex"))
            .map(|(_, value)| value.as_str())
            .expect("complex exists");
        let string = model
            .string_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("string"))
            .map(|(_, value)| value.as_str())
            .expect("string exists");
        let real_array = model
            .real_vector_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("real_array"))
            .map(|(_, values)| values.as_slice())
            .expect("real_array exists");
        let complex_array = model
            .string_vector_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("complex_array"))
            .map(|(_, values)| values.as_slice())
            .expect("complex_array exists");
        let string_array = model
            .string_vector_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("string_array"))
            .map(|(_, values)| values.as_slice())
            .expect("string_array exists");

        assert_eq!(complex, "<4 5>");
        assert_eq!(string, "six");
        assert_eq!(real_array, &[9.0, 10.0]);
        assert_eq!(complex_array, &["<11 12>", "<13 14>"]);
        assert_eq!(string_array, &["fifteen", "sixteen"]);
        assert!(
            model
                .params
                .iter()
                .all(|(name, _)| !name.eq_ignore_ascii_case("complex")),
            "complex literal must not also be a numeric scalar"
        );
    }

    #[test]
    fn xspice_model_params_accept_known_bare_string_literals() {
        let netlist = Netlist::parse(
            "xspice bare string model params\n\
             .model lut d_lut (table_values=0001 family=ttl)\n\
             .end\n",
        )
        .expect("known bare XSPICE string model params parse as strings");

        let model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("lut"))
            .expect("model exists");
        let table_values = model
            .string_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("table_values"))
            .map(|(_, value)| value.as_str())
            .expect("table_values exists");
        let family = model
            .string_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("family"))
            .map(|(_, value)| value.as_str())
            .expect("family exists");

        assert_eq!(table_values, "0001");
        assert_eq!(family, "ttl");
        assert!(
            model.params.iter().all(|(name, _)| {
                !name.eq_ignore_ascii_case("table_values") && !name.eq_ignore_ascii_case("family")
            }),
            "bare string params must not also be numeric params"
        );
    }

    #[test]
    fn xspice_model_params_accept_ngspice_spaced_string_literals() {
        let netlist = Netlist::parse(
            "xspice spaced string model params\n\
             .model lut d_lut (rise_delay=50n fall_delay=50n input_load=1.0p\n\
             + table_values \"0001\")\n\
             .model gen d_genlut (rise_delay=[50n 50n] fall_delay=[50n 50n]\n\
             + input_load=[1.0p 1.0p] input_delay=[2n 2n] table_values \"01100001\")\n\
             .end\n",
        )
        .expect("official ngspice d_lut/d_genlut spaced string params parse");

        for (model_name, expected) in [("lut", "0001"), ("gen", "01100001")] {
            let model = netlist
                .models
                .iter()
                .find(|model| model.name.eq_ignore_ascii_case(model_name))
                .unwrap_or_else(|| panic!("model {model_name} exists"));
            let table_values = model
                .string_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("table_values"))
                .map(|(_, value)| value.as_str())
                .expect("table_values exists");

            assert_eq!(table_values, expected);
            assert!(
                model
                    .params
                    .iter()
                    .all(|(name, _)| !name.eq_ignore_ascii_case("table_values")),
                "spaced string param must not also be a numeric flag"
            );
        }
    }

    #[test]
    fn xspice_model_string_params_preserve_unquoted_path_tokens() {
        let netlist = Netlist::parse(
            "xspice unquoted scalar string model params\n\
             .model co d_cosim simulation=./pwm\n\
             .model proc d_process (process_file=worker|)\n\
             .model table table2d (file=table-2d.tbl)\n\
             .end\n",
        )
        .expect("unquoted XSPICE string params with punctuation parse as strings");

        let co = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("co"))
            .expect("d_cosim model exists");
        let simulation = co
            .string_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("simulation"))
            .map(|(_, value)| value.as_str())
            .expect("simulation string exists");
        assert_eq!(simulation, "./pwm");

        let proc_model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("proc"))
            .expect("d_process model exists");
        let process_file = proc_model
            .string_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("process_file"))
            .map(|(_, value)| value.as_str())
            .expect("process_file string exists");
        assert_eq!(process_file, "worker|");

        let table = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("table"))
            .expect("table model exists");
        let file = table
            .string_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("file"))
            .map(|(_, value)| value.as_str())
            .expect("file string exists");
        assert_eq!(file, "table-2d.tbl");
    }

    #[test]
    fn xspice_contextual_model_params_accept_bare_string_selectors() {
        let netlist = Netlist::parse(
            "xspice contextual model string params\n\
             .model gate multi_input_pwl (x=[0 1] y=[0 1] model=or)\n\
             .model line mlin (l=1 model=1)\n\
             .end\n",
        )
        .expect("contextual XSPICE model selector params parse");

        let gate = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("gate"))
            .expect("multi_input_pwl model exists");
        let gate_selector = gate
            .string_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("model"))
            .map(|(_, value)| value.as_str())
            .expect("multi_input_pwl model selector exists");
        assert_eq!(gate_selector, "or");

        let line = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("line"))
            .expect("mlin model exists");
        assert!(
            line.string_params
                .iter()
                .all(|(name, _)| !name.eq_ignore_ascii_case("model")),
            "tline model selector must not be reclassified as a string"
        );
        assert!(line.params.iter().any(|(name, value)| {
            name.eq_ignore_ascii_case("model") && (*value - 1.0).abs() < f64::EPSILON
        }));
    }

    #[test]
    fn xspice_ako_contextual_model_params_accept_bare_string_selectors() {
        let netlist = Netlist::parse(
            "xspice AKO contextual model string params\n\
             .model base multi_input_pwl (x=[0 1] y=[0 1] model=and)\n\
             .model derived ako:base (model=or)\n\
             .end\n",
        )
        .expect("AKO contextual XSPICE model selector params parse");

        let derived = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("derived"))
            .expect("derived AKO model exists");
        let selector = derived
            .string_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("model"))
            .map(|(_, value)| value.as_str())
            .expect("derived model selector exists");

        assert_eq!(selector, "or");
        assert!(
            derived
                .params
                .iter()
                .all(|(name, _)| { !name.eq_ignore_ascii_case("model") }),
            "AKO string model override must not also be numeric"
        );
    }

    #[test]
    fn model_params_accept_spice_boolean_literals() {
        let netlist = Netlist::parse(
            "xspice boolean model params\n\
             .model sw aswitch (limit=true log=FALSE)\n\
             .end\n",
        )
        .expect("boolean model parameters parse");

        let model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("sw"))
            .expect("model exists");
        let param = |name: &str| {
            model
                .params
                .iter()
                .find(|(param_name, _)| param_name.eq_ignore_ascii_case(name))
                .map(|(_, value)| *value)
                .unwrap_or_else(|| panic!("{name} exists"))
        };

        assert_eq!(param("limit"), 1.0);
        assert_eq!(param("log"), 0.0);
    }

    #[test]
    fn model_vector_params_reject_missing_closing_bracket() {
        let err = Netlist::parse(
            "unterminated xspice vector model param\n\
             .model lut pwl (points=[1 2 3)\n\
             .end\n",
        )
        .expect_err("unterminated vector must be rejected");

        let message = err.to_string();
        let lowered = message.to_ascii_lowercase();
        assert!(
            lowered.contains("points") && message.contains("]'"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn ako_model_vector_params_inherit_and_override_by_name() {
        let netlist = Netlist::parse(
            "ako vector inheritance\n\
             .model base pwl (x_array=[0 1 2] y_array=[0 10 20])\n\
             .model child ako:base pwl (y_array=[0 5 15])\n\
             .end\n",
        )
        .expect("AKO vector inheritance parses");

        let child = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("child"))
            .expect("child model exists");
        let x_array = child
            .real_vector_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("x_array"))
            .map(|(_, values)| values.as_slice())
            .expect("x_array inherited");
        let y_array = child
            .real_vector_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("y_array"))
            .map(|(_, values)| values.as_slice())
            .expect("y_array overridden");

        assert_eq!(x_array, &[0.0, 1.0, 2.0]);
        assert_eq!(y_array, &[0.0, 5.0, 15.0]);
    }

    #[test]
    fn mosfet_off_flag_stays_instance_parameter() {
        let netlist = Netlist::parse(
            "mos off\n\
             M1 d g s b nch OFF W=1u L=50n\n\
             .model nch nmos\n\
             .end\n",
        )
        .expect("MOSFET OFF flag parses");

        match first_mosfet(&netlist) {
            ElementKind::Mosfet {
                model,
                instance_params,
                ..
            } => {
                assert!(model.eq_ignore_ascii_case("nch"));
                assert!(
                    instance_params
                        .iter()
                        .any(|(name, value)| name == "OFF" && (*value - 1.0).abs() < f64::EPSILON)
                );
                assert!(instance_params.iter().any(|(name, _)| name == "W"));
                assert!(instance_params.iter().any(|(name, _)| name == "L"));
            }
            _ => unreachable!("first_mosfet only returns MOSFETs"),
        }
    }

    #[test]
    fn mosfet_explicit_bulk_allows_off_as_model_name() {
        let netlist = Netlist::parse(
            "mos model named off\n\
             M1 d g s b OFF W=1u L=50n\n\
             .model OFF nmos level=18\n\
             .end\n",
        )
        .expect("explicit bulk MOS with model named OFF parses");

        match first_mosfet(&netlist) {
            ElementKind::Mosfet {
                model,
                instance_params,
                ..
            } => {
                assert!(model.eq_ignore_ascii_case("OFF"));
                assert!(
                    !instance_params
                        .iter()
                        .any(|(name, _)| name.eq_ignore_ascii_case("OFF")),
                    "OFF should remain the model name, not become an instance flag"
                );
                assert!(instance_params.iter().any(|(name, _)| name == "W"));
                assert!(instance_params.iter().any(|(name, _)| name == "L"));
            }
            _ => unreachable!("first_mosfet only returns MOSFETs"),
        }
    }

    #[test]
    fn mosfet_ic_vector_stays_instance_parameters() {
        let netlist = Netlist::parse(
            "mos ic vector\n\
             M1 d g s b nch IC=1.2,0.7,-0.1 W=1u L=50n\n\
             .model nch nmos\n\
             .end\n",
        )
        .expect("MOSFET IC vector parses");

        match first_mosfet(&netlist) {
            ElementKind::Mosfet {
                model,
                instance_params,
                ..
            } => {
                assert!(model.eq_ignore_ascii_case("nch"));
                for (name, expected) in [("IC_VDS", 1.2), ("IC_VGS", 0.7), ("IC_VBS", -0.1)] {
                    assert!(
                        instance_params.iter().any(|(param, value)| param == name
                            && (*value - expected).abs() < f64::EPSILON),
                        "missing {name}={expected:?} in {instance_params:?}"
                    );
                }
                assert!(instance_params.iter().any(|(name, _)| name == "W"));
                assert!(instance_params.iter().any(|(name, _)| name == "L"));
            }
            _ => unreachable!("first_mosfet only returns MOSFETs"),
        }
    }

    #[test]
    fn bjt_ic_vector_never_becomes_a_positional_area() {
        // `IC=VBE,VCE` is one vector parameter (`bjt/bjt.c:24`,
        // `N_DEV_BJT.C:114`). Taking only its first component left `,VCE` in the
        // tail, where the positional-AREA arm swallowed it and silently built a
        // larger device: ngspice-46 reports V(B) = 7.520859e-01 for
        // `Q1 c b 0 qnpn ic=0.7,3` and 7.238418e-01 for `Q1 c b 0 qnpn 3`, and
        // RSpice used to report the second answer for the first deck.
        let netlist = Netlist::parse(
            "bjt ic vector\n\
             Q1 c b 0 qnpn IC=0.7,3\n\
             .model qnpn npn\n\
             .end\n",
        )
        .expect("BJT IC vector parses");

        let ElementKind::Bjt {
            instance_params, ..
        } = &netlist.elements[0].kind
        else {
            unreachable!("Q1 is a BJT");
        };
        assert_eq!(
            instance_params,
            &vec![("IC_VBE".to_string(), 0.7), ("IC_VCE".to_string(), 3.0)],
            "no component of the vector may reach a positional arm"
        );
    }

    #[test]
    fn bjt_positional_area_survives_alongside_an_ic_vector() {
        // ngspice-46 reports V(B) = 7.342659e-01 for `Q1 c b 0 qnpn 2 ic=0.7,3`,
        // the AREA=2 answer. RSpice used to refuse the line outright with
        // "Duplicate positional AREA for BJT instance".
        let netlist = Netlist::parse(
            "bjt area then ic\n\
             Q1 c b 0 qnpn 2 ic=0.7,3\n\
             .model qnpn npn\n\
             .end\n",
        )
        .expect("a positional AREA and an IC vector coexist");

        let ElementKind::Bjt {
            instance_params, ..
        } = &netlist.elements[0].kind
        else {
            unreachable!("Q1 is a BJT");
        };
        assert_eq!(
            instance_params,
            &vec![
                ("AREA".to_string(), 2.0),
                ("IC_VBE".to_string(), 0.7),
                ("IC_VCE".to_string(), 3.0)
            ]
        );
    }

    #[test]
    fn bjt_ic_vector_longer_than_the_reference_accepts_is_refused() {
        // `bjt/bjtparam.c:68-81` returns E_BADPARM past two components.
        let err = Netlist::parse(
            "bjt ic overflow\n\
             Q1 c b 0 qnpn IC=0.7,3,1\n\
             .model qnpn npn\n\
             .end\n",
        )
        .expect_err("a three-component BJT IC vector must fail");
        assert!(
            err.to_string().contains("BJT IC vector accepts at most 2"),
            "unexpected message: {err}"
        );
    }

    #[test]
    fn jfet_and_mesfet_ic_vectors_never_become_a_positional_area() {
        // `IC=VDS,VGS` (`jfet/jfet.c:17`, `mes/mes.c:16`).
        for (line, model) in [
            ("J1 d g s njfmod IC=1,-2\n", ".model njfmod njf\n"),
            ("Z1 d g s nmfmod IC=1,-2\n", ".model nmfmod nmf\n"),
        ] {
            let netlist = Netlist::parse(&format!("fet ic vector\n{line}{model}.end\n"))
                .unwrap_or_else(|error| panic!("{line} parses: {error}"));
            let instance_params = match &netlist.elements[0].kind {
                ElementKind::Jfet {
                    instance_params, ..
                }
                | ElementKind::Mesfet {
                    instance_params, ..
                } => instance_params,
                other => unreachable!("{line} built {other:?}"),
            };
            assert_eq!(
                instance_params,
                &vec![("IC_VDS".to_string(), 1.0), ("IC_VGS".to_string(), -2.0)],
                "{line}"
            );
        }
    }

    #[test]
    fn malformed_mosfet_assignment_tail_is_rejected() {
        let err = Netlist::parse(
            "mos malformed\n\
             M1 d g s b nch W 1u L=50n\n\
             .model nch nmos\n\
             .end\n",
        )
        .expect_err("missing '=' in MOSFET W parameter must fail");

        let message = err.to_string();
        assert!(
            message.contains("MOSFET parameter 'W'") && message.contains("expected '='"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn unsupported_mosfet_instance_token_is_rejected() {
        let err = Netlist::parse(
            "mos malformed\n\
             M1 d g s b nch, = W=1u\n\
             .model nch nmos\n\
             .end\n",
        )
        .expect_err("unsupported MOSFET tail token must fail");

        let message = err.to_string();
        assert!(
            message.contains("Unsupported MOSFET instance token '='"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn non_xspice_other_punctuation_still_fails_closed() {
        let err = Netlist::parse(
            "resistor malformed punctuation\n\
             R1 in out 1k!\n\
             .end\n",
        )
        .expect_err("ordinary element punctuation must not parse as valid syntax");

        let message = err.to_string();
        assert!(
            message.contains("Unexpected trailing token in resistor specification: !"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn unsupported_xspice_instance_token_is_rejected() {
        let err = Netlist::parse(
            "xspice malformed\n\
             A1 ] in out gain gain=2\n\
             .end\n",
        )
        .expect_err("unsupported XSPICE instance token must fail");

        let message = err.to_string();
        assert!(
            message.contains("Unsupported XSPICE instance token ']'"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn unsupported_xspice_bracket_token_is_rejected() {
        let err = Netlist::parse(
            "xspice malformed bracket\n\
             A1 [in < out] gain gain=2\n\
             .end\n",
        )
        .expect_err("unsupported XSPICE bracket token must fail");

        let message = err.to_string();
        assert!(
            message.contains("XSPICE digital port requires a node name, found '<'"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn xspice_angle_delimiters_are_not_node_name_punctuation() {
        let err = Netlist::parse(
            "xspice malformed angle delimiter\n\
             A1 net<0> out model\n\
             .end\n",
        )
        .expect_err("ngspice MIF tokenization splits '<' from node identifiers");

        let message = err.to_string();
        assert!(
            message.contains("XSPICE port requires a node name, found '<'"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn xspice_accepts_commas_and_equals_as_loose_port_separators() {
        let netlist = Netlist::parse(
            "xspice comma separators\n\
             A1 = [in, out = mid], out, gain gain=2\n\
             .end\n",
        )
        .expect("commas and equals are accepted as XSPICE port separators");

        match &netlist.elements[0].kind {
            ElementKind::Xspice {
                model,
                ports,
                params,
                ..
            } => {
                assert_eq!(model, "GAIN");
                assert_eq!(
                    ports,
                    &vec![
                        XspicePort::DigitalVector(vec![
                            "IN".to_string(),
                            "OUT".to_string(),
                            "MID".to_string(),
                        ]),
                        XspicePort::Analog("OUT".to_string()),
                    ]
                );
                assert_eq!(params, &vec![("GAIN".to_string(), 2.0)]);
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_accepts_parentheses_as_loose_mif_token_separators() {
        let netlist = Netlist::parse(
            "xspice parenthesis separators\n\
             A1 (in) ([din dout]) (%v out) gain\n\
             .end\n",
        )
        .expect("ngspice MIF tokenization treats parentheses as XSPICE separators");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "GAIN");
                assert_eq!(
                    ports,
                    &vec![
                        XspicePort::Analog("IN".to_string()),
                        XspicePort::DigitalVector(vec!["DIN".to_string(), "DOUT".to_string()]),
                        XspicePort::Analog("OUT".to_string()),
                    ]
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_quoted_mif_string_tokens_parse_as_ports_and_model() {
        let netlist = Netlist::parse(
            "xspice quoted string tokens\n\
             A1 \"in node\" [\"dig a\" ~\"dig b\"] %vd(\"sig p\" \"sig n\") out \"gain\"\n\
             .end\n",
        )
        .expect("ngspice MIF tokenization strips quotes from XSPICE string tokens");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "GAIN");
                assert_eq!(ports[0], XspicePort::Analog("IN NODE".to_string()));
                assert_eq!(
                    ports[1],
                    XspicePort::DigitalVectorMixed(vec![
                        XspiceDigitalNode::new("DIG A", false),
                        XspiceDigitalNode::new("DIG B", true),
                    ])
                );
                assert!(matches!(
                    &ports[2],
                    XspicePort::DifferentialVoltage { pos, neg }
                        if pos == "SIG P" && neg == "SIG N"
                ));
                assert_eq!(ports[3], XspicePort::Analog("OUT".to_string()));
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_quoted_mif_tokens_do_not_concatenate_with_adjacent_tokens() {
        let netlist = Netlist::parse(
            "xspice adjacent quoted token\n\
             A1 \"in\"out gain\n\
             .end\n",
        )
        .expect("ngspice MIF tokenization treats quoted strings as complete tokens");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "GAIN");
                assert_eq!(
                    ports,
                    &vec![
                        XspicePort::Analog("IN".to_string()),
                        XspicePort::Analog("OUT".to_string()),
                    ]
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_typed_null_connections_parse_like_ngspice_mif_null_tokens() {
        let netlist = Netlist::parse(
            "xspice typed null tokens\n\
             A1 %v null %gd(null) out model\n\
             .end\n",
        )
        .expect("ngspice MIF port parsing treats typed null as a null connection");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "MODEL");
                assert_eq!(
                    ports,
                    &vec![
                        XspicePort::Null,
                        XspicePort::Null,
                        XspicePort::Analog("OUT".to_string()),
                    ]
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_explicit_digital_typed_ports_parse_like_ngspice_mif_ports() {
        let netlist = Netlist::parse(
            "xspice explicit digital typed ports\n\
             A1 %d in %d([bus0 bus1]) out model\n\
             .end\n",
        )
        .expect("ngspice %d typed XSPICE ports should parse");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "MODEL");
                assert_eq!(
                    ports,
                    &vec![
                        XspicePort::ExplicitDigital("IN".to_string()),
                        XspicePort::ExplicitDigital("BUS0".to_string()),
                        XspicePort::ExplicitDigital("BUS1".to_string()),
                        XspicePort::Analog("OUT".to_string()),
                    ]
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_quoted_null_mif_token_parses_as_null_connection() {
        let netlist = Netlist::parse(
            "xspice quoted null token\n\
             A1 \"null\" out model\n\
             .end\n",
        )
        .expect("ngspice MIF tokenization treats quoted null as a null token");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "MODEL");
                assert_eq!(
                    ports,
                    &vec![XspicePort::Null, XspicePort::Analog("OUT".to_string()),]
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_bracketed_null_entry_is_rejected_like_ngspice_array_null() {
        let err = Netlist::parse(
            "xspice bracketed null token\n\
             A1 [in null] out model\n\
             .end\n",
        )
        .expect_err("ngspice rejects null entries inside XSPICE arrays");

        let message = err.to_string();
        assert!(
            message.contains("NULL connection found where not allowed in XSPICE array"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn xspice_compact_typed_vector_null_entry_is_rejected_like_ngspice_array_null() {
        let err = Netlist::parse(
            "xspice compact vector null token\n\
             A1 %v([in null]) out model\n\
             .end\n",
        )
        .expect_err("ngspice rejects null entries inside compact typed XSPICE vectors");

        let message = err.to_string();
        assert!(
            message
                .contains("NULL connection found where not allowed in compact XSPICE port vector"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn xspice_digital_vector_ports_parse_ngspice_inverted_node_syntax() {
        let netlist = Netlist::parse(
            "xspice inverted digital vector\n\
             A1 [o1 ~o2 o3] out d_and\n\
             .end\n",
        )
        .expect("ngspice inverted digital vector syntax should parse");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "D_AND");
                assert_eq!(
                    ports,
                    &vec![
                        XspicePort::DigitalVectorMixed(vec![
                            XspiceDigitalNode::new("O1", false),
                            XspiceDigitalNode::new("O2", true),
                            XspiceDigitalNode::new("O3", false),
                        ]),
                        XspicePort::Analog("OUT".to_string()),
                    ]
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_top_level_inverted_digital_ports_parse_like_ngspice_mif_ports() {
        let netlist = Netlist::parse(
            "xspice bare inverted digital ports\n\
             A1 a ~b ~\"c node\" out d_and\n\
             .end\n",
        )
        .expect("ngspice allows leading tilde on digital/user-defined XSPICE ports");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "D_AND");
                assert_eq!(
                    ports,
                    &vec![
                        XspicePort::Analog("A".to_string()),
                        XspicePort::DigitalInverted("B".to_string()),
                        XspicePort::DigitalInverted("C NODE".to_string()),
                        XspicePort::Analog("OUT".to_string()),
                    ]
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_numeric_like_port_names_preserve_lexeme_text() {
        let netlist = Netlist::parse(
            "xspice numeric-looking node names\n\
             A1 1e3 [03 ~2e3] %vd([4e-6 0 5e2 0]) out model\n\
             .end\n",
        )
        .expect("numeric-looking XSPICE port names parse");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "MODEL");
                assert_eq!(ports[0], XspicePort::Analog("1e3".to_string()));
                assert_eq!(
                    ports[1],
                    XspicePort::DigitalVectorMixed(vec![
                        XspiceDigitalNode::new("03", false),
                        XspiceDigitalNode::new("2e3", true),
                    ])
                );
                assert!(matches!(
                    &ports[2],
                    XspicePort::DifferentialVoltage { pos, neg }
                        if pos == "4e-6" && neg == "0"
                ));
                assert!(matches!(
                    &ports[3],
                    XspicePort::DifferentialVoltage { pos, neg }
                        if pos == "5e2" && neg == "0"
                ));
                assert_eq!(ports[4], XspicePort::Analog("OUT".to_string()));
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_signed_net_names_parse_like_ngspice_net_tokens() {
        let netlist = Netlist::parse(
            "xspice signed net names\n\
             A1 +vcc -vee [in- ~+rst -clk] %vd(+in -in) %gd[+gate -gate] out model\n\
             .end\n",
        )
        .expect("ngspice-style signed XSPICE net names parse");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "MODEL");
                assert_eq!(ports[0], XspicePort::Analog("+VCC".to_string()));
                assert_eq!(ports[1], XspicePort::Analog("-VEE".to_string()));
                assert_eq!(
                    ports[2],
                    XspicePort::DigitalVectorMixed(vec![
                        XspiceDigitalNode::new("IN-", false),
                        XspiceDigitalNode::new("+RST", true),
                        XspiceDigitalNode::new("-CLK", false),
                    ])
                );
                assert!(matches!(
                    &ports[3],
                    XspicePort::DifferentialVoltage { pos, neg }
                        if pos == "+IN" && neg == "-IN"
                ));
                assert!(matches!(
                    &ports[4],
                    XspicePort::DifferentialConductance { pos, neg }
                        if pos == "+GATE" && neg == "-GATE"
                ));
                assert_eq!(ports[5], XspicePort::Analog("OUT".to_string()));
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_complex_net_names_parse_like_ngspice_net_tokens() {
        let netlist = Netlist::parse(
            "xspice complex net names\n\
             A1 net/a bus*1 @sense [sig/a ~+rst data-7] %vd(path/in path/out) %gd[gate*1 return/path] out model\n\
             .end\n",
        )
        .expect("complex XSPICE net names parse");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "MODEL");
                assert_eq!(ports[0], XspicePort::Analog("NET/A".to_string()));
                assert_eq!(ports[1], XspicePort::Analog("BUS*1".to_string()));
                assert_eq!(ports[2], XspicePort::Analog("@SENSE".to_string()));
                assert_eq!(
                    ports[3],
                    XspicePort::DigitalVectorMixed(vec![
                        XspiceDigitalNode::new("SIG/A", false),
                        XspiceDigitalNode::new("+RST", true),
                        XspiceDigitalNode::new("DATA-7", false),
                    ])
                );
                assert!(matches!(
                    &ports[4],
                    XspicePort::DifferentialVoltage { pos, neg }
                        if pos == "PATH/IN" && neg == "PATH/OUT"
                ));
                assert!(matches!(
                    &ports[5],
                    XspicePort::DifferentialConductance { pos, neg }
                        if pos == "GATE*1" && neg == "RETURN/PATH"
                ));
                assert_eq!(ports[6], XspicePort::Analog("OUT".to_string()));
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_hyphenated_instance_names_do_not_become_ports() {
        let netlist = Netlist::parse(
            "xspice hyphenated instance name\n\
             Abridge-fit [dout] [aout] dac1\n\
             .end\n",
        )
        .expect("ngspice-style hyphenated XSPICE instance names parse");

        assert_eq!(netlist.elements[0].name, "ABRIDGE-FIT");
        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "DAC1");
                assert_eq!(
                    ports,
                    &vec![
                        XspicePort::Digital("DOUT".to_string()),
                        XspicePort::Digital("AOUT".to_string()),
                    ]
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_other_punctuation_net_names_parse_like_ngspice_net_tokens() {
        let netlist = Netlist::parse(
            "xspice punctuation net names\n\
             A1 !bias^1 bus|2 [ctrl?0 ~!rst] %vd(sig!p sig^n) %v(net|out) out model\n\
             .end\n",
        )
        .expect("ngspice-style punctuation XSPICE net names parse");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "MODEL");
                assert_eq!(ports[0], XspicePort::Analog("!BIAS^1".to_string()));
                assert_eq!(ports[1], XspicePort::Analog("BUS|2".to_string()));
                assert_eq!(
                    ports[2],
                    XspicePort::DigitalVectorMixed(vec![
                        XspiceDigitalNode::new("CTRL?0", false),
                        XspiceDigitalNode::new("!RST", true),
                    ])
                );
                assert!(matches!(
                    &ports[3],
                    XspicePort::DifferentialVoltage { pos, neg }
                        if pos == "SIG!P" && neg == "SIG^N"
                ));
                assert_eq!(ports[4], XspicePort::Analog("NET|OUT".to_string()));
                assert_eq!(ports[5], XspicePort::Analog("OUT".to_string()));
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_instance_params_accept_spice_unit_suffixes() {
        let netlist = Netlist::parse(
            "xspice instance parameter suffixes\n\
             A1 in out gain gain=2 rise_delay=10n cap=1u limit=1meg\n\
             .end\n",
        )
        .expect("XSPICE instance params accept SPICE suffixes");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { params, .. } => {
                assert_eq!(params.len(), 4);
                assert!((params[0].1 - 2.0).abs() < f64::EPSILON);
                assert!((params[1].1 - 10.0e-9).abs() < 1.0e-21);
                assert!((params[2].1 - 1.0e-6).abs() < 1.0e-18);
                assert!((params[3].1 - 1.0e6).abs() < f64::EPSILON);
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_instance_params_accept_sign_separated_decimal_values() {
        let netlist = Netlist::parse(
            "xspice instance signed decimals\n\
             A1 in out gain gain=-.5 offset=+.25 tiny=-1p\n\
             .end\n",
        )
        .expect("XSPICE instance params accept sign-separated decimal values");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { params, .. } => {
                assert_eq!(params.len(), 3);
                assert!((params[0].1 + 0.5).abs() < f64::EPSILON);
                assert!((params[1].1 - 0.25).abs() < f64::EPSILON);
                assert!((params[2].1 + 1.0e-12).abs() < 1.0e-24);
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_instance_params_accept_top_level_brace_expressions() {
        let netlist = Netlist::parse(
            "xspice instance expression params\n\
             .param g=3\n\
             A1 in out gain gain={g*2} offset=-{g}\n\
             .end\n",
        )
        .expect("XSPICE instance params accept brace expressions");

        match &netlist.elements[0].kind {
            ElementKind::Xspice {
                params,
                expr_params,
                ..
            } => {
                assert!(expr_params.is_empty());
                assert_eq!(params.len(), 2);
                assert!((params[0].1 - 6.0).abs() < f64::EPSILON);
                assert!((params[1].1 + 3.0).abs() < f64::EPSILON);
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_instance_params_accept_string_literals() {
        let netlist = Netlist::parse(
            "xspice instance string params\n\
             A1 in out file_probe file=\"custom.tbl\" family=ttl\n\
             .end\n",
        )
        .expect("XSPICE instance params accept string literals");

        match &netlist.elements[0].kind {
            ElementKind::Xspice {
                string_params,
                string_expr_params,
                ..
            } => {
                assert!(string_expr_params.is_empty());
                assert_eq!(
                    string_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("file"))
                        .map(|(_, value)| value.as_str()),
                    Some("custom.tbl")
                );
                assert_eq!(
                    string_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("family"))
                        .map(|(_, value)| value.as_str()),
                    Some("ttl")
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_instance_string_params_preserve_unquoted_path_tokens() {
        let netlist = Netlist::parse(
            "xspice instance unquoted scalar string params\n\
             A1 in out file_probe file=table-2d.tbl simulation=./pwm process_file=worker| table_values=0001\n\
             .end\n",
        )
        .expect("XSPICE instance string params with punctuation parse");

        match &netlist.elements[0].kind {
            ElementKind::Xspice {
                params,
                string_params,
                string_expr_params,
                ..
            } => {
                assert!(string_expr_params.is_empty());
                for (name, expected) in [
                    ("file", "table-2d.tbl"),
                    ("simulation", "./pwm"),
                    ("process_file", "worker|"),
                    ("table_values", "0001"),
                ] {
                    assert_eq!(
                        string_params
                            .iter()
                            .find(|(param, _)| param.eq_ignore_ascii_case(name))
                            .map(|(_, value)| value.as_str()),
                        Some(expected),
                        "unexpected value for {name}"
                    );
                    assert!(
                        params
                            .iter()
                            .all(|(param, _)| !param.eq_ignore_ascii_case(name)),
                        "{name} must not also be parsed as a numeric parameter"
                    );
                }
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn pem_table_model_params_keep_directory_names_that_look_like_signed_numbers() {
        // A checkout under a directory such as `worktrees\cool-kilby-4be5a2`
        // puts `-4be5a2` inside the absolute table path a test writes onto the
        // model card. The tokenizer used to stall on that fragment and grow
        // its token vector until the process died, so the whole deck — not
        // just this parameter — depended on where the repository lived.
        let netlist = Netlist::parse_validated(
            "PEM table path with a signed-number directory\n\
             .model mrm1 memristor level=4\n\
             + fxpdata=C:\\models\\pem-4be5a2\\fxp_table.csv\n\
             + fxmdata=C:\\models\\pem-7268ec\\fxm_table.csv\n\
             V1 in 0 DC 0.2\n\
             .end\n",
        )
        .expect("signed-number path components parse");

        let model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("mrm1"))
            .expect("memristor model exists");
        for (name, expected) in [
            ("fxpdata", "C:\\models\\pem-4be5a2\\fxp_table.csv"),
            ("fxmdata", "C:\\models\\pem-7268ec\\fxm_table.csv"),
        ] {
            assert_eq!(
                model
                    .string_params
                    .iter()
                    .find(|(param, _)| param.eq_ignore_ascii_case(name))
                    .map(|(_, value)| value.as_str()),
                Some(expected),
                "unexpected value for {name}"
            );
        }
    }

    #[test]
    fn pem_table_model_params_resolve_relative_to_their_source_file() {
        let base = Path::new("models/memristor");
        assert_eq!(
            normalize_model_string_path_value("FXPDATA", "positive.csv", Some(base)),
            base.join("positive.csv").to_string_lossy()
        );
        assert_eq!(
            normalize_model_string_path_value("fxmdata", "virtual://pem/negative", Some(base)),
            "virtual://pem/negative"
        );
        assert_eq!(
            normalize_model_string_path_value("metadata", "not-a-path", Some(base)),
            "not-a-path"
        );
    }

    #[test]
    fn xspice_instance_params_accept_legacy_mif_params_marker() {
        let netlist = Netlist::parse(
            "xspice instance legacy MIF params marker\n\
             A1 [a b] y d_and PARAMS: rise_delay=10n fall_delay=20n family=ttl\n\
             .end\n",
        )
        .expect("XSPICE instance params accept legacy PARAMS marker");

        match &netlist.elements[0].kind {
            ElementKind::Xspice {
                model,
                ports,
                params,
                string_params,
                ..
            } => {
                assert_eq!(model, "D_AND");
                assert_eq!(
                    ports,
                    &vec![
                        XspicePort::DigitalVector(vec!["A".to_string(), "B".to_string()]),
                        XspicePort::Analog("Y".to_string()),
                    ]
                );
                assert_eq!(params.len(), 2);
                assert!(params.iter().any(|(name, value)| {
                    name.eq_ignore_ascii_case("rise_delay") && (*value - 10.0e-9).abs() < 1.0e-21
                }));
                assert!(params.iter().any(|(name, value)| {
                    name.eq_ignore_ascii_case("fall_delay") && (*value - 20.0e-9).abs() < 1.0e-21
                }));
                assert_eq!(
                    string_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("family"))
                        .map(|(_, value)| value.as_str()),
                    Some("ttl")
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_instance_contextual_model_param_keeps_numeric_selectors_numeric() {
        let netlist = Netlist::parse(
            "xspice instance contextual model param\n\
             A1 in out mlin model=1\n\
             A2 in out multi_input_pwl model=or\n\
             .end\n",
        )
        .expect("XSPICE instance model params parse by value shape");

        match &netlist.elements[0].kind {
            ElementKind::Xspice {
                params,
                string_params,
                ..
            } => {
                assert!(
                    string_params
                        .iter()
                        .all(|(name, _)| !name.eq_ignore_ascii_case("model")),
                    "numeric model selector must not be reclassified as a string"
                );
                assert!(params.iter().any(|(name, value)| {
                    name.eq_ignore_ascii_case("model") && (*value - 1.0).abs() < f64::EPSILON
                }));
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }

        match &netlist.elements[1].kind {
            ElementKind::Xspice { string_params, .. } => {
                assert_eq!(
                    string_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("model"))
                        .map(|(_, value)| value.as_str()),
                    Some("or")
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_instance_params_accept_vector_literals() {
        let netlist = Netlist::parse(
            "xspice instance vector params\n\
             A1 in out vector_probe table=[0 1.5 2k] process_params=[\"--mode\" \"fast\"]\n\
             .end\n",
        )
        .expect("XSPICE instance params accept vector literals");

        match &netlist.elements[0].kind {
            ElementKind::Xspice {
                real_vector_params,
                real_vector_expr_params,
                string_vector_params,
                string_vector_expr_params,
                ..
            } => {
                assert!(real_vector_expr_params.is_empty());
                assert!(string_vector_expr_params.is_empty());
                assert_eq!(
                    real_vector_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("table"))
                        .map(|(_, values)| values.as_slice()),
                    Some(&[0.0, 1.5, 2000.0][..])
                );
                assert_eq!(
                    string_vector_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("process_params"))
                        .map(|(_, values)| values.as_slice()),
                    Some(&["--mode".to_string(), "fast".to_string()][..])
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_instance_params_accept_ngspice_complex_literals() {
        let netlist = Netlist::parse(
            "xspice instance complex params\n\
             A1 in out print_param_types complex=<4.0 5.0> complex_array=[<11.0t 12.0g> <13.0m 14.0>]\n\
             .end\n",
        )
        .expect("XSPICE instance params accept ngspice complex literals");

        match &netlist.elements[0].kind {
            ElementKind::Xspice {
                string_params,
                string_vector_params,
                ..
            } => {
                assert_eq!(
                    string_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("complex"))
                        .map(|(_, value)| value.as_str()),
                    Some("<4 5>")
                );
                assert_eq!(
                    string_vector_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("complex_array"))
                        .map(|(_, values)| values.as_slice()),
                    Some(
                        &[
                            "<11000000000000 12000000000>".to_string(),
                            "<0.013 14>".to_string()
                        ][..]
                    )
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_instance_string_vector_scalar_params_preserve_unquoted_argv_tokens() {
        let netlist = Netlist::parse(
            "xspice instance scalar string-vector params\n\
             A1 in out process_probe process_params=--payload lib_args=+define=1 sim_args=\"-O2\"\n\
             .end\n",
        )
        .expect("XSPICE scalar string-vector argv params parse");

        match &netlist.elements[0].kind {
            ElementKind::Xspice {
                string_vector_params,
                string_vector_expr_params,
                ..
            } => {
                assert!(string_vector_expr_params.is_empty());
                for (name, expected) in [
                    ("process_params", "--payload"),
                    ("lib_args", "+define=1"),
                    ("sim_args", "-O2"),
                ] {
                    assert_eq!(
                        string_vector_params
                            .iter()
                            .find(|(param, _)| param.eq_ignore_ascii_case(name))
                            .map(|(_, values)| values.as_slice()),
                        Some(&[expected.to_string()][..]),
                        "unexpected vector value for {name}"
                    );
                }
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_instance_string_vector_params_preserve_unquoted_argv_tokens() {
        let netlist = Netlist::parse(
            "xspice instance string-vector argv params\n\
             A1 in out process_probe process_params=[1e3 deck --payload -gTarget=4500 +define=1 ./dut]\n\
             .end\n",
        )
        .expect("XSPICE instance string-vector argv params parse");

        match &netlist.elements[0].kind {
            ElementKind::Xspice {
                string_vector_params,
                string_vector_expr_params,
                ..
            } => {
                assert!(string_vector_expr_params.is_empty());
                assert_eq!(
                    string_vector_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("process_params"))
                        .map(|(_, values)| values.as_slice()),
                    Some(
                        &[
                            "1e3".to_string(),
                            "deck".to_string(),
                            "--payload".to_string(),
                            "-gTarget=4500".to_string(),
                            "+define=1".to_string(),
                            "./dut".to_string(),
                        ][..]
                    )
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_subckt_instance_params_resolve_brace_expressions_during_flattening() {
        let netlist = Netlist::parse(
            "xspice subckt instance expression params\n\
             .subckt xgain in out g=2 scale=3\n\
             A1 in out gain gain=g*scale offset=-{g}*scale\n\
             .ends xgain\n\
             XU a b xgain g=5 scale=4\n\
             .end\n",
        )
        .expect("XSPICE subcircuit expression-param deck parses");

        let flattened = flatten_netlist_with_models(&netlist)
            .expect("XSPICE subcircuit expression-param deck flattens");
        let element = flattened
            .elements
            .iter()
            .find(|element| element.name == "XU.A1")
            .expect("flattened XSPICE element exists");

        match &element.kind {
            ElementKind::Xspice {
                params,
                expr_params,
                ..
            } => {
                assert!(expr_params.is_empty());
                assert_eq!(params.len(), 2);
                assert_eq!(
                    params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("gain"))
                        .map(|(_, value)| *value),
                    Some(20.0)
                );
                assert_eq!(
                    params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("offset"))
                        .map(|(_, value)| *value),
                    Some(-20.0)
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_subckt_deferred_scalar_params_override_case_insensitively() {
        let mut netlist = Netlist::default();
        netlist.subcircuits.push(SubcircuitDef {
            name: "xgain".to_string(),
            ports: vec!["in".to_string(), "out".to_string()],
            elements: vec![Element {
                name: "A1".to_string(),
                kind: ElementKind::Xspice {
                    model: "gain".to_string(),
                    pspice_u_timing: None,
                    ports: vec![
                        XspicePort::Analog("in".to_string()),
                        XspicePort::Analog("out".to_string()),
                    ],
                    params: vec![("Gain".to_string(), 1.0)],
                    expr_params: vec![("gain".to_string(), "g".to_string())],
                    string_params: Vec::new(),
                    string_expr_params: Vec::new(),
                    string_vector_params: Vec::new(),
                    string_vector_expr_params: Vec::new(),
                    real_vector_params: Vec::new(),
                    real_vector_expr_params: Vec::new(),
                },
                nodes: Vec::new(),
                provenance: crate::netlist::ElementProvenance::Authored,
            }],
            initial_conditions: Vec::new(),
            node_sets: Vec::new(),
            params: vec![("g".to_string(), 2.0)],
            expr_params: Vec::new(),
            string_params: Vec::new(),
            body_params: Vec::new(),
            body_expr_params: Vec::new(),
            body_string_params: Vec::new(),
            body_functions: Vec::new(),
            local_options: std::collections::HashMap::new(),
            library_ref: None,
            nested_subcircuits: Vec::new(),
        });
        netlist.elements.push(Element {
            name: "XU".to_string(),
            kind: ElementKind::Subcircuit {
                subckt_name: "xgain".to_string(),
                params: vec![("g".to_string(), ParametricValue::Resolved(5.0))],
            },
            nodes: vec!["a".to_string(), "b".to_string()],
            provenance: crate::netlist::ElementProvenance::Authored,
        });

        let flattened = flatten_netlist_with_models(&netlist)
            .expect("programmatic XSPICE subcircuit AST flattens");
        let element = flattened
            .elements
            .iter()
            .find(|element| element.name == "XU.A1")
            .expect("flattened XSPICE element exists");

        match &element.kind {
            ElementKind::Xspice {
                params,
                expr_params,
                ..
            } => {
                assert!(expr_params.is_empty());
                assert_eq!(
                    params
                        .iter()
                        .filter(|(name, _)| name.eq_ignore_ascii_case("gain"))
                        .count(),
                    1
                );
                assert_eq!(
                    params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("gain"))
                        .map(|(_, value)| *value),
                    Some(5.0)
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_subckt_instance_string_params_resolve_during_flattening() {
        let netlist = Netlist::parse(
            "xspice subckt instance string params\n\
             .param actual_file=\"actual.tbl\"\n\
             .subckt xsrc out fname=\"default.tbl\"\n\
             A1 %v(out) filesrc file={fname}\n\
             .ends xsrc\n\
             XU out xsrc fname={actual_file}\n\
             .end\n",
        )
        .expect("XSPICE subcircuit string-param deck parses");

        let flattened = flatten_netlist_with_models(&netlist)
            .expect("XSPICE subcircuit string-param deck flattens");
        let element = flattened
            .elements
            .iter()
            .find(|element| element.name == "XU.A1")
            .expect("flattened XSPICE element exists");

        match &element.kind {
            ElementKind::Xspice {
                string_params,
                string_expr_params,
                ..
            } => {
                assert!(string_expr_params.is_empty());
                assert_eq!(
                    string_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("file"))
                        .map(|(_, value)| value.as_str()),
                    Some("actual.tbl")
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_subckt_instance_vector_params_resolve_during_flattening() {
        let netlist = Netlist::parse(
            "xspice subckt instance vector params\n\
             .param actual_args=\"[1e3 --mode -gTarget=4500]\"\n\
             .subckt xvec in out scale=2 args=\"[--default]\"\n\
             A1 in out vector_probe table=[0 {scale} {scale*2}] process_params={args}\n\
             .ends xvec\n\
             XU a b xvec scale=3 args={actual_args}\n\
             .end\n",
        )
        .expect("XSPICE subcircuit vector-param deck parses");

        let flattened = flatten_netlist_with_models(&netlist)
            .expect("XSPICE subcircuit vector-param deck flattens");
        let element = flattened
            .elements
            .iter()
            .find(|element| element.name == "XU.A1")
            .expect("flattened XSPICE element exists");

        match &element.kind {
            ElementKind::Xspice {
                real_vector_params,
                real_vector_expr_params,
                string_vector_params,
                string_vector_expr_params,
                ..
            } => {
                assert!(real_vector_expr_params.is_empty());
                assert!(string_vector_expr_params.is_empty());
                assert_eq!(
                    real_vector_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("table"))
                        .map(|(_, values)| values.as_slice()),
                    Some(&[0.0, 3.0, 6.0][..])
                );
                assert_eq!(
                    string_vector_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("process_params"))
                        .map(|(_, values)| values.as_slice()),
                    Some(
                        &[
                            "1e3".to_string(),
                            "--mode".to_string(),
                            "-gTarget=4500".to_string(),
                        ][..]
                    )
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_subckt_instance_vector_params_accept_leading_bare_param_refs() {
        let netlist = Netlist::parse(
            "xspice subckt instance vector leading bare params\n\
             .subckt xvec in out start=2 step=3\n\
             A1 in out vector_probe table=[start start+step]\n\
             .ends xvec\n\
             XU a b xvec start=4 step=5\n\
             .end\n",
        )
        .expect("XSPICE subcircuit vector-param deck parses");

        let flattened = flatten_netlist_with_models(&netlist)
            .expect("XSPICE subcircuit vector-param deck flattens");
        let element = flattened
            .elements
            .iter()
            .find(|element| element.name == "XU.A1")
            .expect("flattened XSPICE element exists");

        match &element.kind {
            ElementKind::Xspice {
                real_vector_params,
                real_vector_expr_params,
                string_vector_params,
                ..
            } => {
                assert!(real_vector_expr_params.is_empty());
                assert!(string_vector_params.is_empty());
                assert_eq!(
                    real_vector_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("table"))
                        .map(|(_, values)| values.as_slice()),
                    Some(&[4.0, 9.0][..])
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_subckt_instance_complex_params_resolve_during_flattening() {
        let netlist = Netlist::parse(
            "xspice subckt instance complex params\n\
             .subckt xcmp in out r=2 i=3 ar=4 ai=5\n\
             A1 in out print_param_types complex=<r i> complex_array=[<ar ai> <r*2 {i+1}>]\n\
             .ends xcmp\n\
             XU a b xcmp r=6 i=7 ar=8 ai=9\n\
             .end\n",
        )
        .expect("XSPICE subcircuit complex-param deck parses");

        let flattened = flatten_netlist_with_models(&netlist)
            .expect("XSPICE subcircuit complex-param deck flattens");
        let element = flattened
            .elements
            .iter()
            .find(|element| element.name == "XU.A1")
            .expect("flattened XSPICE element exists");

        match &element.kind {
            ElementKind::Xspice {
                string_params,
                string_expr_params,
                string_vector_params,
                string_vector_expr_params,
                ..
            } => {
                assert!(string_expr_params.is_empty());
                assert!(string_vector_expr_params.is_empty());
                assert_eq!(
                    string_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("complex"))
                        .map(|(_, value)| value.as_str()),
                    Some("<6 7>")
                );
                assert_eq!(
                    string_vector_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("complex_array"))
                        .map(|(_, values)| values.as_slice()),
                    Some(&["<8 9>".to_string(), "<12 8>".to_string()][..])
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_differential_ports_parse_documented_percent_syntax() {
        let netlist = Netlist::parse(
            "xspice differential\n\
             A1 %vd[n+ n-] out gain gain=2\n\
             .end\n",
        )
        .expect("documented XSPICE differential port syntax parses");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { ports, .. } => {
                assert!(matches!(
                    &ports[0],
                    XspicePort::DifferentialVoltage { pos, neg }
                        if pos == "N+" && neg == "N-"
                ));
                assert_eq!(ports[1], XspicePort::Analog("OUT".to_string()));
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_bracketed_typed_vector_ports_parse_ngspice_array_syntax() {
        let netlist = Netlist::parse(
            "xspice typed vector array\n\
             A1 ct mon [%id(vdd vbiasp) %id(vdd vop)] seemod2\n\
             .end\n",
        )
        .expect("ngspice bracketed typed vector syntax should parse");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "SEEMOD2");
                assert_eq!(ports.len(), 4);
                assert_eq!(ports[0], XspicePort::Analog("CT".to_string()));
                assert_eq!(ports[1], XspicePort::Analog("MON".to_string()));
                assert!(matches!(
                    &ports[2],
                    XspicePort::DifferentialCurrent { pos, neg }
                        if pos == "VDD" && neg == "VBIASP"
                ));
                assert!(matches!(
                    &ports[3],
                    XspicePort::DifferentialCurrent { pos, neg }
                        if pos == "VDD" && neg == "VOP"
                ));
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_voltage_current_ports_parse_official_spaced_percent_syntax() {
        let netlist = Netlist::parse(
            "xspice spaced voltage/current ports\n\
             A1 %vd in 0 %id sense 0 out gain\n\
             .end\n",
        )
        .expect("ngspice accepts spaced %vd/%id analog port syntax");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { ports, .. } => {
                assert_eq!(ports.len(), 3);
                assert!(matches!(
                    &ports[0],
                    XspicePort::DifferentialVoltage { pos, neg }
                        if pos == "IN" && neg == "0"
                ));
                assert!(matches!(
                    &ports[1],
                    XspicePort::DifferentialCurrent { pos, neg }
                        if pos == "SENSE" && neg == "0"
                ));
                assert_eq!(ports[2], XspicePort::Analog("OUT".to_string()));
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_split_percent_port_type_tokens_parse_like_ngspice_mif_tokens() {
        let netlist = Netlist::parse(
            "xspice split percent tokens\n\
             A1 % v in % vd p n % \"g\" gate % hd hp hn [ % id(src 0) % v(out)] model\n\
             .end\n",
        )
        .expect("ngspice MIF tokenizer accepts '%' as a separate port-type token");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "MODEL");
                assert_eq!(ports[0], XspicePort::Analog("IN".to_string()));
                assert!(matches!(
                    &ports[1],
                    XspicePort::DifferentialVoltage { pos, neg }
                        if pos == "P" && neg == "N"
                ));
                assert_eq!(ports[2], XspicePort::Conductance("GATE".to_string()));
                assert!(matches!(
                    &ports[3],
                    XspicePort::DifferentialHybrid { pos, neg }
                        if pos == "HP" && neg == "HN"
                ));
                assert!(matches!(
                    &ports[4],
                    XspicePort::DifferentialCurrent { pos, neg }
                        if pos == "SRC" && neg == "0"
                ));
                assert_eq!(ports[5], XspicePort::Analog("OUT".to_string()));
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_scalar_voltage_current_ports_parse_official_percent_syntax() {
        let netlist = Netlist::parse(
            "xspice scalar voltage/current ports\n\
             A1 %v in %i vsen out gain\n\
             .end\n",
        )
        .expect("ngspice accepts scalar %v/%i analog port syntax");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { ports, .. } => {
                assert_eq!(ports.len(), 3);
                assert_eq!(ports[0], XspicePort::Analog("IN".to_string()));
                assert_eq!(ports[1], XspicePort::Current("VSEN".to_string()));
                assert_eq!(ports[2], XspicePort::Analog("OUT".to_string()));
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_compact_differential_vector_ports_parse_ngspice_filesource_syntax() {
        let netlist = Netlist::parse(
            "xspice compact differential vector\n\
             A1 %vd([out1 0 out2 0]) filesrc\n\
             .end\n",
        )
        .expect("ngspice accepts compact %vd([p n ...]) vector syntax");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "FILESRC");
                assert_eq!(ports.len(), 2);
                assert!(matches!(
                    &ports[0],
                    XspicePort::DifferentialVoltage { pos, neg }
                        if pos == "OUT1" && neg == "0"
                ));
                assert!(matches!(
                    &ports[1],
                    XspicePort::DifferentialVoltage { pos, neg }
                        if pos == "OUT2" && neg == "0"
                ));
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_compact_scalar_vector_ports_parse_ngspice_filesource_syntax() {
        let netlist = Netlist::parse(
            "xspice compact scalar vector\n\
             A1 %v([out6 out7]) filesrc\n\
             .end\n",
        )
        .expect("ngspice accepts compact %v([n ...]) vector syntax");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "FILESRC");
                assert_eq!(
                    ports,
                    &vec![
                        XspicePort::Analog("OUT6".to_string()),
                        XspicePort::Analog("OUT7".to_string())
                    ]
                );
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_subckt_vector_param_override_creates_scoped_model() {
        let netlist = Netlist::parse(
            "xspice subckt vector param\n\
             .param default_vec=\"[1e-12 2e-12]\"\n\
             .subckt testcir in0 in1 outlut testpar = {default_vec}\n\
             A_genlut [in0 in1] [outlut] genlut\n\
             .model genlut d_genlut (\n\
             + input_delay = {testpar}\n\
             + table_values = \"0001\")\n\
             .ends testcir\n\
             .param actual_vec=\"[1.3e-3 2e-3]\"\n\
             X_subckt no1 dss node3 testcir testpar={actual_vec}\n\
             .end\n",
        )
        .expect("ngspice vector-valued subckt parameter deck parses");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("vector-valued subckt parameter flattens");
        let model_name = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Xspice { model, .. } => Some(model.as_str()),
                _ => None,
            })
            .expect("flattened XSPICE element exists");
        assert_ne!(model_name, "testcir::genlut");

        let scoped_model = flattened
            .scoped_models
            .iter()
            .find(|model| model.name == model_name)
            .expect("flattening creates a private scoped model");
        assert!(scoped_model.model_type.eq_ignore_ascii_case("d_genlut"));
        assert!(
            scoped_model.expr_params.is_empty(),
            "scoped XSPICE model expressions should resolve during flattening"
        );
        assert_eq!(
            scoped_model
                .real_vector_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("input_delay"))
                .map(|(_, values)| values.as_slice()),
            Some(&[1.3e-3, 2.0e-3][..])
        );
        assert_eq!(
            scoped_model
                .string_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("table_values"))
                .map(|(_, value)| value.as_str()),
            Some("0001")
        );
    }

    #[test]
    fn xspice_subckt_model_scalar_expression_resolves_per_instance() {
        let netlist = Netlist::parse(
            "xspice subckt inline model scalar expression\n\
             .subckt gaincell in out base=1 scale=2\n\
             A1 in out gainmodel\n\
             .model gainmodel gain (gain=base*scale in_offset={base}*scale out_offset=0)\n\
             .ends gaincell\n\
             X1 a b gaincell base=4 scale=5\n\
             X2 c d gaincell base=6 scale=7\n\
             .end\n",
        )
        .expect("subckt-local XSPICE scalar model expressions parse");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("scoped model scalar expressions flatten");
        let model_for = |element_name: &str| -> &str {
            flattened
                .elements
                .iter()
                .find_map(|element| match &element.kind {
                    ElementKind::Xspice { model, .. } if element.name == element_name => {
                        Some(model.as_str())
                    }
                    _ => None,
                })
                .unwrap_or_else(|| panic!("flattened XSPICE element {element_name} exists"))
        };

        let param_for = |model_name: &str, param_name: &str| -> f64 {
            flattened
                .scoped_models
                .iter()
                .find(|model| model.name == model_name)
                .and_then(|model| {
                    assert!(
                        model.expr_params.is_empty(),
                        "scoped model scalar expressions should resolve during flattening"
                    );
                    model
                        .params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case(param_name))
                        .map(|(_, value)| *value)
                })
                .unwrap_or_else(|| panic!("scoped model {model_name} has {param_name}"))
        };

        let x1_model = model_for("X1.A1");
        let x2_model = model_for("X2.A1");
        assert_ne!(x1_model, x2_model);
        assert_eq!(param_for(x1_model, "gain"), 20.0);
        assert_eq!(param_for(x1_model, "in_offset"), 20.0);
        assert_eq!(param_for(x2_model, "gain"), 42.0);
        assert_eq!(param_for(x2_model, "in_offset"), 42.0);
    }

    #[test]
    fn xspice_subckt_model_vector_entries_resolve_per_instance() {
        let netlist = Netlist::parse(
            "xspice subckt inline model vector entries\n\
             .subckt clocker out base=1e3 scale=2\n\
             Aclk null [out] oscmodel\n\
             .model oscmodel d_osc(cntl_array=[-1 1] freq_array=[base base*scale])\n\
             .ends clocker\n\
             X1 one clocker base=10 scale=3\n\
             X2 two clocker base=20 scale=4\n\
             .end\n",
        )
        .expect("subckt-local XSPICE vector model expressions parse");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("scoped model vector entries flatten");
        let model_for = |element_name: &str| -> &str {
            flattened
                .elements
                .iter()
                .find_map(|element| match &element.kind {
                    ElementKind::Xspice { model, .. } if element.name == element_name => {
                        Some(model.as_str())
                    }
                    _ => None,
                })
                .unwrap_or_else(|| panic!("flattened XSPICE element {element_name} exists"))
        };

        let vector_for = |model_name: &str| -> Vec<f64> {
            flattened
                .scoped_models
                .iter()
                .find(|model| model.name == model_name)
                .and_then(|model| {
                    model
                        .real_vector_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("freq_array"))
                        .map(|(_, values)| values.clone())
                })
                .unwrap_or_else(|| panic!("scoped model {model_name} has freq_array"))
        };

        assert_eq!(vector_for(model_for("X1.ACLK")), vec![10.0, 30.0]);
        assert_eq!(vector_for(model_for("X2.ACLK")), vec![20.0, 80.0]);
        assert!(
            flattened
                .scoped_models
                .iter()
                .all(|model| model.real_vector_expr_params.is_empty()),
            "scoped XSPICE model vector expressions should resolve during flattening"
        );
    }

    #[test]
    fn xspice_subckt_model_complex_params_resolve_per_instance() {
        let netlist = Netlist::parse(
            "xspice subckt inline model complex params\n\
             .subckt xcmp in out r=1 i=2 ar=3 ai=4\n\
             A1 in out cmpmodel\n\
             .model cmpmodel print_param_types (complex=<r i> complex_array=[<ar ai> <r*2 {i+1}>])\n\
             .ends xcmp\n\
             X1 a b xcmp r=5 i=6 ar=7 ai=8\n\
             X2 c d xcmp r=9 i=10 ar=11 ai=12\n\
             .end\n",
        )
        .expect("subckt-local XSPICE complex model params parse");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("scoped model complex params flatten");
        let model_for = |element_name: &str| -> &str {
            flattened
                .elements
                .iter()
                .find_map(|element| match &element.kind {
                    ElementKind::Xspice { model, .. } if element.name == element_name => {
                        Some(model.as_str())
                    }
                    _ => None,
                })
                .unwrap_or_else(|| panic!("flattened XSPICE element {element_name} exists"))
        };

        let complex_for = |model_name: &str| -> (&str, Vec<String>) {
            let model = flattened
                .scoped_models
                .iter()
                .find(|model| model.name == model_name)
                .unwrap_or_else(|| panic!("scoped model {model_name} exists"));
            assert!(
                model.expr_params.is_empty(),
                "scoped XSPICE complex model expressions should resolve during flattening"
            );
            let complex = model
                .string_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("complex"))
                .map(|(_, value)| value.as_str())
                .unwrap_or_else(|| panic!("scoped model {model_name} has complex"));
            let complex_array = model
                .string_vector_params
                .iter()
                .find(|(name, _)| name.eq_ignore_ascii_case("complex_array"))
                .map(|(_, values)| values.clone())
                .unwrap_or_else(|| panic!("scoped model {model_name} has complex_array"));
            (complex, complex_array)
        };

        let (x1_complex, x1_array) = complex_for(model_for("X1.A1"));
        let (x2_complex, x2_array) = complex_for(model_for("X2.A1"));
        assert_eq!(x1_complex, "<5 6>");
        assert_eq!(x1_array, vec!["<7 8>".to_string(), "<10 7>".to_string()]);
        assert_eq!(x2_complex, "<9 10>");
        assert_eq!(x2_array, vec!["<11 12>".to_string(), "<18 11>".to_string()]);
    }

    #[test]
    fn xspice_subckt_flattening_remaps_xspice_port_ast_nodes() {
        let netlist = Netlist::parse(
            "xspice subckt ports remap\n\
             .subckt xcell rin din pin nin out\n\
             vsen sense 0 0\n\
             areal rin mid rg\n\
             adig [din ~din_int] [dout] dg\n\
             atyp %vd(pin nin) %v(out) %i(vsen) %vnam(vsen) out2 typ\n\
             .model rg real_gain\n\
             .model dg d_and\n\
             .model typ gain\n\
             .ends xcell\n\
             X1 top_r top_d top_p top_n top_out xcell\n\
             .end\n",
        )
        .expect("XSPICE subcircuit deck parses");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("XSPICE subcircuit deck flattens");
        let ports_for = |name: &str| -> &[XspicePort] {
            flattened
                .elements
                .iter()
                .find_map(|element| match &element.kind {
                    ElementKind::Xspice { ports, .. } if element.name == name => {
                        Some(ports.as_slice())
                    }
                    _ => None,
                })
                .unwrap_or_else(|| panic!("flattened XSPICE element {name} exists"))
        };

        assert_eq!(
            ports_for("X1.AREAL"),
            &[
                XspicePort::Analog("TOP_R".to_string()),
                XspicePort::Analog("X1.MID".to_string())
            ]
        );
        assert_eq!(
            ports_for("X1.ADIG"),
            &[
                XspicePort::DigitalVectorMixed(vec![
                    XspiceDigitalNode::new("TOP_D", false),
                    XspiceDigitalNode::new("X1.DIN_INT", true),
                ]),
                XspicePort::Digital("X1.DOUT".to_string())
            ]
        );
        assert_eq!(
            ports_for("X1.ATYP"),
            &[
                XspicePort::DifferentialVoltage {
                    pos: "TOP_P".to_string(),
                    neg: "TOP_N".to_string()
                },
                XspicePort::Analog("TOP_OUT".to_string()),
                XspicePort::Current("X1.VSEN".to_string()),
                XspicePort::VoltageName("X1.VSEN".to_string()),
                XspicePort::Analog("X1.OUT2".to_string())
            ]
        );
    }

    #[test]
    fn xspice_scoped_file_param_resolves_relative_to_deck_path() {
        let deck_path = std::env::temp_dir()
            .join("rspice-xspice-scoped-file-param")
            .join("deck.cir");
        let deck_dir = deck_path.parent().expect("temp deck has parent");
        std::fs::create_dir_all(deck_dir).expect("create temp deck dir");
        let netlist = Netlist::parse_with_path(
            "xspice scoped file param\n\
             .subckt source out stim = \"stim.stim\"\n\
             A_src [out] src_model\n\
             .model src_model d_source (input_file={stim})\n\
             .ends source\n\
             X1 out source\n\
             .end\n",
            &deck_path,
        )
        .expect("deck parses with path");

        let flattened = flatten_netlist_with_models(&netlist)
            .expect("scoped XSPICE file parameter deck flattens");
        let scoped_model = flattened
            .scoped_models
            .iter()
            .find(|model| model.model_type.eq_ignore_ascii_case("d_source"))
            .expect("scoped d_source model exists");
        let input_file = scoped_model
            .string_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("input_file"))
            .map(|(_, value)| value.as_str())
            .expect("input_file string param resolved");

        assert_eq!(
            std::path::Path::new(input_file),
            deck_dir.join("stim.stim").as_path()
        );
    }

    #[test]
    fn xspice_top_level_external_paths_resolve_relative_without_rewriting_provider_ids() {
        let deck_path = std::env::temp_dir()
            .join("rspice-xspice-top-level-external-paths")
            .join("deck.cir");
        let deck_dir = deck_path.parent().expect("temp deck has parent");
        std::fs::create_dir_all(deck_dir).expect("create temp deck dir");
        let netlist = Netlist::parse_with_path(
            "xspice top-level external path params\n\
             .model cosim_path d_cosim (simulation=\"./pwm\")\n\
             .model cosim_provider d_cosim (simulation=\"ivlng\")\n\
             .model proc d_process (process_file=\"worker|\")\n\
             .model src d_source (input_file=\"virtual://xspice/stim\")\n\
             .end\n",
            &deck_path,
        )
        .expect("deck parses with path");

        let string_param = |model_name: &str, param_name: &str| -> &str {
            netlist
                .models
                .iter()
                .find(|model| model.name.eq_ignore_ascii_case(model_name))
                .and_then(|model| {
                    model
                        .string_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case(param_name))
                })
                .map(|(_, value)| value.as_str())
                .expect("string model param exists")
        };

        assert_eq!(
            std::path::Path::new(string_param("cosim_path", "simulation")),
            deck_dir.join("pwm").as_path()
        );
        assert_eq!(string_param("cosim_provider", "simulation"), "ivlng");
        assert_eq!(
            string_param("proc", "process_file"),
            format!("{}|", deck_dir.join("worker").to_string_lossy())
        );
        assert_eq!(string_param("src", "input_file"), "virtual://xspice/stim");
    }

    #[test]
    fn xspice_instance_external_paths_resolve_relative_during_flattening() {
        let deck_path = std::env::temp_dir()
            .join("rspice-xspice-instance-external-paths")
            .join("deck.cir");
        let deck_dir = deck_path.parent().expect("temp deck has parent");
        std::fs::create_dir_all(deck_dir).expect("create temp deck dir");
        let netlist = Netlist::parse_with_path(
            "xspice instance external path params\n\
             Apath [d] src input_file=stim-dir/source.txt\n\
             Aco_path [din] [dout] null co_path simulation=./pwm\n\
             Aco_provider [din] [dout] null co_provider simulation=ivlng\n\
             Aproc [din] [dout] proc process_file=worker|\n\
             Avirt [d] virt input_file=virtual://xspice/stim\n\
             .end\n",
            &deck_path,
        )
        .expect("deck parses with path");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("XSPICE instance path deck flattens");
        let string_param = |model_name: &str, param_name: &str| -> &str {
            flattened
                .elements
                .iter()
                .find_map(|element| match &element.kind {
                    ElementKind::Xspice {
                        model,
                        string_params,
                        ..
                    } if model.eq_ignore_ascii_case(model_name) => string_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case(param_name))
                        .map(|(_, value)| value.as_str()),
                    _ => None,
                })
                .expect("string instance param exists")
        };

        assert_eq!(
            std::path::Path::new(string_param("src", "input_file")),
            deck_dir.join("stim-dir").join("source.txt").as_path()
        );
        assert_eq!(
            std::path::Path::new(string_param("co_path", "simulation")),
            deck_dir.join("pwm").as_path()
        );
        assert_eq!(string_param("co_provider", "simulation"), "ivlng");
        assert_eq!(
            string_param("proc", "process_file"),
            format!("{}|", deck_dir.join("worker").to_string_lossy())
        );
        assert_eq!(string_param("virt", "input_file"), "virtual://xspice/stim");
    }

    #[test]
    fn xspice_scoped_simulation_path_resolves_relative_but_provider_name_stays_symbolic() {
        let deck_path = std::env::temp_dir()
            .join("rspice-xspice-scoped-simulation-paths")
            .join("deck.cir");
        let deck_dir = deck_path.parent().expect("temp deck has parent");
        std::fs::create_dir_all(deck_dir).expect("create temp deck dir");
        let netlist = Netlist::parse_with_path(
            "xspice scoped d_cosim simulation params\n\
             .subckt cosim din dout sim=\"./pwm\"\n\
             Aco [din] [dout] null co\n\
             .model co d_cosim (simulation={sim})\n\
             .ends cosim\n\
             Xpath in1 out1 cosim sim=\"./pwm\"\n\
             Xprovider in2 out2 cosim sim=\"ivlng\"\n\
             .end\n",
            &deck_path,
        )
        .expect("deck parses with path");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("scoped d_cosim deck flattens");
        let simulations = flattened
            .scoped_models
            .iter()
            .filter(|model| model.model_type.eq_ignore_ascii_case("d_cosim"))
            .filter_map(|model| {
                model
                    .string_params
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case("simulation"))
                    .map(|(_, value)| value.as_str())
            })
            .collect::<Vec<_>>();

        assert_eq!(simulations.len(), 2, "expected two scoped d_cosim models");
        assert!(
            simulations
                .iter()
                .any(|value| std::path::Path::new(value) == deck_dir.join("pwm").as_path()),
            "path-like simulation should resolve relative to deck dir: {simulations:?}"
        );
        assert!(
            simulations.iter().any(|value| *value == "ivlng"),
            "provider-style simulation id should remain symbolic: {simulations:?}"
        );
    }

    #[test]
    fn xspice_bare_file_param_identifier_defers_to_subckt_string_override() {
        let deck_path = std::env::temp_dir()
            .join("rspice-xspice-bare-file-param")
            .join("deck.cir");
        let deck_dir = deck_path.parent().expect("temp deck has parent");
        std::fs::create_dir_all(deck_dir).expect("create temp deck dir");
        let netlist = Netlist::parse_with_path(
            "xspice bare file param\n\
             .subckt subtest in1 in2 infile=\"whatever\"\n\
             Afs %vd([in1 0 in2 0]) filesrc\n\
             .model filesrc filesource (file=infile amploffset=[0 0] amplscale=[1 1]\n\
             + timeoffset=0 timescale=1 timerelative=false amplstep=false)\n\
             .ends subtest\n\
             X1 in1 in2 subtest infile=\"my-source.txt\"\n\
             .end\n",
            &deck_path,
        )
        .expect("ngspice bare file=infile subckt deck parses");

        let flattened = flatten_netlist_with_models(&netlist)
            .expect("bare file identifier subckt deck flattens");
        let scoped_model = flattened
            .scoped_models
            .iter()
            .find(|model| model.model_type.eq_ignore_ascii_case("filesource"))
            .expect("scoped filesource model exists");
        let file = scoped_model
            .string_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("file"))
            .map(|(_, value)| value.as_str())
            .expect("file string param resolved");

        assert_eq!(
            std::path::Path::new(file),
            deck_dir.join("my-source.txt").as_path()
        );
    }

    #[test]
    fn xspice_rejects_unknown_percent_port_type_suffixes_like_ngspice() {
        let err = Netlist::parse(
            "xspice invalid percent port type\n\
             A1 %vdc in 0 out gain\n\
             .end\n",
        )
        .expect_err("ngspice rejects unknown typed port %vdc");

        let message = err.to_string();
        assert!(
            message.contains("Unknown differential port type"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn xspice_conductance_ports_parse_official_percent_gd_syntax() {
        let netlist = Netlist::parse(
            "xspice differential conductance\n\
             A1 %gd[p n] out model\n\
             .end\n",
        )
        .expect("official XSPICE %gd conductance port syntax parses");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { ports, .. } => {
                assert!(matches!(
                    &ports[0],
                    XspicePort::DifferentialConductance { pos, neg }
                        if pos == "P" && neg == "N"
                ));
                assert_eq!(ports[1], XspicePort::Analog("OUT".to_string()));
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_conductance_ports_parse_official_percent_g_syntax() {
        let netlist = Netlist::parse(
            "xspice scalar conductance\n\
             A1 %g in out model\n\
             .end\n",
        )
        .expect("official XSPICE %g conductance port syntax parses");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { ports, .. } => {
                assert_eq!(ports[0], XspicePort::Conductance("IN".to_string()));
                assert_eq!(ports[1], XspicePort::Analog("OUT".to_string()));
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn xspice_conductance_ports_parse_official_spaced_percent_gd_syntax() {
        let netlist = Netlist::parse(
            "xspice spaced differential conductance\n\
             A1 %gd in 0 %gd out 0 model\n\
             .end\n",
        )
        .expect("official XSPICE spaced %gd conductance port syntax parses");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { ports, .. } => {
                assert_eq!(ports.len(), 2);
                assert!(matches!(
                    &ports[0],
                    XspicePort::DifferentialConductance { pos, neg }
                        if pos == "IN" && neg == "0"
                ));
                assert!(matches!(
                    &ports[1],
                    XspicePort::DifferentialConductance { pos, neg }
                        if pos == "OUT" && neg == "0"
                ));
            }
            other => panic!("expected XSPICE element, got {other:?}"),
        }
    }

    #[test]
    fn unclosed_xspice_differential_port_is_rejected() {
        let err = Netlist::parse(
            "xspice malformed differential\n\
             A1 %vd[n+ n-\n\
             .end\n",
        )
        .expect_err("unclosed XSPICE differential port must fail");

        let message = err.to_string();
        assert!(
            message.contains("Unclosed differential port"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn controlled_sources_accept_behavioral_output_assignments() {
        let netlist = Netlist::parse(
            "controlled behavioral aliases\n\
             Gtop vtop vout cur='loadcur*v(u1)'\n\
             Etop vout 0 vol='2*v(in)'\n\
             .end\n",
        )
        .expect("G cur= and E vol= behavioral aliases should parse");

        assert!(matches!(
            &netlist.elements[0].kind,
            ElementKind::BehavioralCurrent { expression, .. }
                if expression == "loadcur*v(u1)"
        ));
        assert!(matches!(
            &netlist.elements[1].kind,
            ElementKind::BehavioralVoltage { expression, .. }
                if expression == "2*v(in)"
        ));
    }

    #[test]
    fn behavioral_source_preserves_logical_operators() {
        let netlist = Netlist::parse(
            "behavioral logical source\n\
             Bcross cross 0 V=(V(live) > -2 && V(live) < 2) ? 5 : 0\n\
             .end\n",
        )
        .expect("behavioral logical expression should parse");

        let ElementKind::BehavioralVoltage { expression, .. } = &netlist.elements[0].kind else {
            panic!("expected behavioral voltage source");
        };
        assert!(
            expression.contains("&&"),
            "logical and operator was not preserved: {expression}"
        );
        assert!(
            !expression.contains("& &"),
            "logical and operator was split: {expression}"
        );
    }

    #[test]
    fn behavioral_source_preserves_unbraced_string_literals() {
        let netlist = Netlist::parse(
            "behavioral table source\n\
             B1 1 0 V=table(\"sinewave2-1.dat\")\n\
             .end\n",
        )
        .expect("behavioral expression with string literal should parse");

        let ElementKind::BehavioralVoltage { expression, .. } = &netlist.elements[0].kind else {
            panic!("expected behavioral voltage source");
        };
        assert!(
            expression.contains("\"sinewave2-1.dat\""),
            "behavioral expression must preserve string literal quotes, got {expression}"
        );
    }

    #[test]
    fn behavioral_source_lowers_xyce_braced_table_form() {
        let netlist = Netlist::parse(
            "behavioral Xyce table source\n\
             B1 out 0 V={TABLE { V(in) + 1 } (0, 0) (1, 2) (2, 3)}\n\
             .end\n",
        )
        .expect("Xyce braced TABLE behavioral source should parse");

        let ElementKind::BehavioralVoltage { expression, .. } = &netlist.elements[0].kind else {
            panic!("expected behavioral voltage source");
        };
        assert_eq!(
            expression,
            "table(limit((V(in) + 1), 0, 2), 0, 0, 1, 2, 2, 3)"
        );
    }

    #[test]
    fn multi_input_vcvs_gate_lowers_to_xspice_pwl() {
        let netlist = Netlist::parse(
            "multi-input VCVS gate\n\
             E1 out 0 nand(2) in1 0 in2 0 ({vcc / 3}, 0) ({2 * vcc / 3}, {vcc})\n\
             .end\n",
        )
        .expect("ngspice multi-input VCVS gate syntax should parse");

        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == "E1__MULTI_INPUT")
            .expect("lowered XSPICE element exists");

        match &element.kind {
            ElementKind::Xspice {
                model,
                ports,
                string_params,
                real_vector_expr_params,
                ..
            } => {
                assert_eq!(model, "multi_input_pwl");
                assert_eq!(
                    string_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("model"))
                        .map(|(_, value)| value.as_str()),
                    Some("nand")
                );
                assert_eq!(ports.len(), 3);
                assert!(matches!(
                    &ports[0],
                    XspicePort::DifferentialVoltage { pos, neg }
                        if pos.eq_ignore_ascii_case("in1") && neg == "0"
                ));
                assert!(matches!(
                    &ports[2],
                    XspicePort::DifferentialVoltage { pos, neg }
                        if pos.eq_ignore_ascii_case("out") && neg == "0"
                ));
                assert_eq!(
                    real_vector_expr_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("x"))
                        .map(|(_, values)| values.as_slice()),
                    Some(&["vcc / 3".to_string(), "2 * vcc / 3".to_string()][..])
                );
                assert_eq!(
                    real_vector_expr_params
                        .iter()
                        .find(|(name, _)| name.eq_ignore_ascii_case("y"))
                        .map(|(_, values)| values.as_slice()),
                    Some(&["0".to_string(), "vcc".to_string()][..])
                );
            }
            other => panic!("expected lowered XSPICE multi_input_pwl, got {other:?}"),
        }
    }

    #[test]
    fn linear_controlled_sources_reject_unconsumed_trailing_tokens() {
        for line in [
            "E1 out 0 in 0 2 garbage",
            "G1 out 0 in 0 2m garbage",
            "F1 out 0 Vctrl 2 garbage",
            "H1 out 0 Vctrl 2 garbage",
        ] {
            let err = Netlist::parse(&format!(
                "bad controlled source tail\n\
                 Vctrl ctrl 0 DC 1\n\
                 Vin in 0 DC 1\n\
                 {line}\n\
                 .op\n\
                 .end\n"
            ))
            .expect_err("linear controlled sources must reject trailing tokens");

            let message = err.to_string();
            assert!(
                message.contains("garbage") || message.contains("GARBAGE"),
                "unexpected error for {line}: {message}"
            );
        }
    }

    #[test]
    fn extended_controlled_source_numeric_tails_reject_non_numeric_tokens() {
        for line in [
            "E1 out 0 POLY(1) in 0 1 garbage 2",
            "G1 out 0 TABLE {V(in)} = (0 0) garbage (1 1)",
            "F1 out 0 POLY(1) Vctrl 1 garbage 2",
        ] {
            let err = Netlist::parse(&format!(
                "bad controlled source numeric tail\n\
                 Vctrl ctrl 0 DC 1\n\
                 Vin in 0 DC 1\n\
                 {line}\n\
                 .op\n\
                 .end\n"
            ))
            .expect_err("extended controlled-source numeric tails must reject junk tokens");

            let message = err.to_string();
            assert!(
                message.contains("garbage") || message.contains("GARBAGE"),
                "unexpected error for {line}: {message}"
            );
        }
    }

    #[test]
    fn transmission_switch_and_coupling_tails_reject_unconsumed_tokens() {
        for line in [
            "K1 L1 L2 0.9 garbage",
            "S1 out 0 ctrl 0 sw ON garbage",
            "W1 out 0 Vctrl sw OFF garbage",
            "T1 a 0 b 0 Z0=50 TD=1n garbage=99",
            "O1 a 0 b 0 omod garbage",
            "Y1 a 0 b 0 ymod garbage",
        ] {
            let err = Netlist::parse(&format!(
                "bad transmission/switch/coupling tail\n\
                 Vctrl ctrl 0 DC 1\n\
                 L1 n1 0 1u\n\
                 L2 n2 0 1u\n\
                 {line}\n\
                 .op\n\
                 .end\n"
            ))
            .expect_err("transmission, switch, and coupling cards must reject trailing tokens");

            let message = err.to_string();
            assert!(
                message.contains("garbage") || message.contains("GARBAGE"),
                "unexpected error for {line}: {message}"
            );
        }
    }

    /// Grounded (numeric) extra terminals are everyday SPICE: a BJT with
    /// its substrate and thermal nodes tied to ground, and an SOI MOSFET
    /// with a grounded body contact in the 5th slot.
    #[test]
    fn bjt_and_mosfet_accept_grounded_extra_terminal_nodes() {
        let netlist = Netlist::parse(
            "grounded extra terminals\n\
             Q1 c b e 0 0 qmod\n\
             M1 d g s e 0 mmod\n\
             .model qmod NPN (IS=1e-16)\n\
             .model mmod NMOS (LEVEL=57)\n\
             .op\n\
             .end\n",
        )
        .expect("numeric substrate/thermal and SOI tail nodes must parse");

        let bjt = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("Q1"))
            .expect("bjt element");
        assert_eq!(bjt.nodes, ["C", "B", "E", "0", "0"]);

        let mosfet = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case("M1"))
            .expect("mosfet element");
        assert_eq!(mosfet.nodes, ["D", "G", "S", "E", "0"]);
    }

    /// A trailing numeric token can never silently become the MOS model.
    #[test]
    fn mosfet_trailing_numeric_token_is_still_rejected() {
        let err = Netlist::parse(
            "bad mosfet tail\n\
             M1 d g s b mmod 2\n\
             .model mmod NMOS (LEVEL=1)\n\
             .op\n\
             .end\n",
        )
        .expect_err("positional numeric MOS values must be rejected");
        assert!(
            err.to_string()
                .contains("Unsupported MOSFET instance token"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn coupling_coefficient_outside_physical_range_is_rejected() {
        for coefficient in ["-0.5", "1.2"] {
            let err = Netlist::parse(&format!(
                "bad coupling coefficient\n\
                 L1 a 0 1u\n\
                 L2 b 0 1u\n\
                 K1 L1 L2 {coefficient}\n\
                 .op\n\
                 .end\n"
            ))
            .expect_err("invalid coupling coefficient must fail instead of being clamped");

            let message = err.to_string();
            assert!(
                message.contains("coupling") && message.contains(coefficient),
                "unexpected error for {coefficient}: {message}"
            );
        }
    }

    #[test]
    fn dangling_data_terminator_is_rejected() {
        let err = Netlist::parse(
            "dangling data terminator\n\
             V1 out 0 1\n\
             R1 out 0 1k\n\
             .enddata\n\
             .op\n\
             .end\n",
        )
        .expect_err("unmatched .ENDDATA must fail instead of being ignored");

        let message = err.to_string();
        assert!(
            message.contains(".ENDDATA") && message.contains(".DATA"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn unterminated_data_block_is_rejected() {
        let err = Netlist::parse(
            "unterminated data block\n\
             V1 out 0 1\n\
             .data sweep vin\n\
             0\n\
             1\n\
             .op\n\
             .end\n",
        )
        .expect_err("unterminated .DATA must fail instead of discarding the rest of the deck");

        let message = err.to_string();
        assert!(
            message.contains(".DATA") && message.contains(".ENDDATA"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn data_rows_accept_leading_decimal_values() {
        let netlist = Netlist::parse(
            "leading decimal data\n\
             .data sweep vin\n\
             .5\n\
             .enddata\n\
             .step data=sweep\n\
             .op\n\
             .end\n",
        )
        .expect("leading-decimal .DATA value should parse");

        assert_eq!(netlist.data_tables.len(), 1);
        assert_eq!(netlist.data_tables[0].rows, vec![vec![0.5]]);
    }

    #[test]
    fn step_data_table_is_retained_and_referenced() {
        let netlist = Netlist::parse(
            "step data table\n\
             .param base=2 rval=1k\n\
             R1 1 0 {rval}\n\
             V1 1 0 1\n\
             .dc V1 1 1 1\n\
             .data sweep\n\
             + rval scale\n\
             + 1k {base*3}\n\
             + 2k 8\n\
             .enddata\n\
             .step data=sweep\n\
             .end\n",
        )
        .expect(".DATA table and .STEP DATA should parse");

        let table = netlist
            .data_tables
            .iter()
            .find(|table| table.name.eq_ignore_ascii_case("sweep"))
            .expect(".DATA table retained");
        assert_eq!(table.params, vec!["rval", "scale"]);
        assert_eq!(table.rows, vec![vec![1000.0, 6.0], vec![2000.0, 8.0]]);

        let step = netlist
            .analyses
            .iter()
            .find_map(|analysis| match analysis {
                AnalysisCommand::Step(step) => Some(step),
                _ => None,
            })
            .expect(".STEP retained");
        match &step.sweep {
            StepSweep::Data { table_name } => {
                assert!(table_name.eq_ignore_ascii_case("sweep"))
            }
            other => panic!("expected .STEP DATA sweep, got {other:?}"),
        }
    }

    #[test]
    fn ac_data_table_analysis_is_retained() {
        let netlist = Netlist::parse(
            "ac data table\n\
             I1 1 0 AC 1\n\
             R1 1 0 1k\n\
             .AC DATA=pts\n\
             .DATA pts\n\
             + FREQ\n\
             + 1\n\
             + 10\n\
             .ENDDATA\n\
             .PRINT AC V(1)\n\
             .END\n",
        )
        .expect(".AC DATA table should parse");

        let table = netlist
            .data_tables
            .iter()
            .find(|table| table.name.eq_ignore_ascii_case("pts"))
            .expect(".DATA table retained");
        assert_eq!(table.params, vec!["FREQ"]);
        assert_eq!(table.rows, vec![vec![1.0], vec![10.0]]);

        assert!(netlist.analyses.iter().any(|analysis| matches!(
            analysis,
            AnalysisCommand::AcData { table_name } if table_name.eq_ignore_ascii_case("pts")
        )));
    }

    #[test]
    fn noise_data_table_analysis_is_retained() {
        let netlist = Netlist::parse(
            "noise data table\n\
             .GLOBAL_PARAM mag=1 phase=0\n\
             V1 in 0 AC {mag} {phase}\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .NOISE V(out) V1 DATA=pts 5\n\
             .DATA pts\n\
             + mag phase HERTZ\n\
             + 2 0.2 10\n\
             + 1 0.1 1\n\
             .ENDDATA\n\
             .PRINT NOISE V(out) INOISE ONOISE\n\
             .END\n",
        )
        .expect(".NOISE DATA table should parse");

        assert!(netlist.analyses.iter().any(|analysis| matches!(
            analysis,
            AnalysisCommand::NoiseData {
                output_node,
                reference_node: None,
                input_source,
                table_name,
            } if output_node.eq_ignore_ascii_case("out")
                && input_source.eq_ignore_ascii_case("V1")
                && table_name.eq_ignore_ascii_case("pts")
        )));
        let table = netlist
            .data_tables
            .iter()
            .find(|table| table.name.eq_ignore_ascii_case("pts"))
            .expect(".DATA table retained");
        assert_eq!(table.params, vec!["mag", "phase", "HERTZ"]);
        assert_eq!(table.rows, vec![vec![2.0, 0.2, 10.0], vec![1.0, 0.1, 1.0]]);
    }

    #[test]
    fn noise_data_requires_equals_and_a_table_name() {
        for source in [
            "bad noise data\nV1 in 0 AC 1\nR1 in 0 1k\n.NOISE V(in) V1 DATA pts\n.END\n",
            "bad noise data\nV1 in 0 AC 1\nR1 in 0 1k\n.NOISE V(in) V1 DATA=\n.END\n",
        ] {
            assert!(Netlist::parse(source).is_err());
        }
    }

    #[test]
    fn step_linear_source_target_without_type_keyword_parses() {
        let netlist = Netlist::parse(
            "xyce source step\n\
             vd drain 0 dc 0\n\
             vg gate 0 dc 1\n\
             .dc vd 0 1.2 0.01\n\
             .step lin vg 0.2 1.2 0.1\n\
             .end\n",
        )
        .expect("Xyce-style .STEP LIN source target should parse");

        let step = netlist
            .analyses
            .iter()
            .find_map(|analysis| match analysis {
                AnalysisCommand::Step(step) => Some(step),
                _ => None,
            })
            .expect(".STEP retained");

        assert_eq!(step.target, StepTarget::Device);
        assert!(step.name.eq_ignore_ascii_case("vg"));
        assert!(step.param_name.is_none());
        match step.sweep {
            StepSweep::Linear { start, stop, step } => {
                assert_eq!((start, stop, step), (0.2, 1.2, 0.1));
            }
            ref other => panic!("expected linear source sweep, got {other:?}"),
        }
    }

    #[test]
    fn step_linear_known_param_without_type_keyword_parses_as_param() {
        let netlist = Netlist::parse(
            "xyce parameter step\n\
             .param rval=1k\n\
             v1 out 0 dc 1\n\
             r1 out 0 {rval}\n\
             .dc v1 0 1 1\n\
             .step lin rval 1k 2k 500\n\
             .end\n",
        )
        .expect("Xyce-style .STEP LIN parameter target should parse");

        let step = netlist
            .analyses
            .iter()
            .find_map(|analysis| match analysis {
                AnalysisCommand::Step(step) => Some(step),
                _ => None,
            })
            .expect(".STEP retained");

        assert_eq!(step.target, StepTarget::Param);
        assert!(step.name.eq_ignore_ascii_case("rval"));
        match step.sweep {
            StepSweep::Linear { start, stop, step } => {
                assert_eq!((start, stop, step), (1000.0, 2000.0, 500.0));
            }
            ref other => panic!("expected linear parameter sweep, got {other:?}"),
        }
    }

    #[test]
    fn step_logarithmic_integer_point_counts_parse_for_supported_target_forms() {
        let cases = [
            (
                ".step dec param rval 1 100 5",
                StepTarget::Param,
                "rval",
                5,
                true,
            ),
            (".step oct rval 1 8 3", StepTarget::Param, "rval", 3, false),
        ];

        for (command, expected_target, expected_name, expected_points, is_decade) in cases {
            let deck = format!(
                "logarithmic parameter step\n\
                 .param rval=1\n\
                 {command}\n\
                 .end\n"
            );
            let netlist = Netlist::parse(&deck)
                .unwrap_or_else(|error| panic!("{command} should parse: {error}"));
            let step = netlist
                .analyses
                .iter()
                .find_map(|analysis| match analysis {
                    AnalysisCommand::Step(step) => Some(step),
                    _ => None,
                })
                .unwrap_or_else(|| panic!("{command} should be retained"));

            assert_eq!(step.target, expected_target, "{command}");
            assert!(step.name.eq_ignore_ascii_case(expected_name), "{command}");
            match (&step.sweep, is_decade) {
                (
                    StepSweep::Decade {
                        points_per_decade, ..
                    },
                    true,
                ) => {
                    assert_eq!(*points_per_decade, expected_points, "{command}");
                }
                (
                    StepSweep::Octave {
                        points_per_octave, ..
                    },
                    false,
                ) => {
                    assert_eq!(*points_per_octave, expected_points, "{command}");
                }
                (other, _) => panic!("unexpected sweep for {command}: {other:?}"),
            }
        }
    }

    #[test]
    fn step_logarithmic_invalid_point_counts_are_rejected() {
        let cases = [
            ("DEC", "PARAM rval", "5.9"),
            ("OCT", "rval", "0"),
            ("DEC", "PARAM rval", "-1"),
            ("OCT", "rval", "1e309"),
            ("DEC", "PARAM rval", "1e100"),
        ];

        for (sweep_type, target, points) in cases {
            let deck = format!(
                "invalid logarithmic step\n\
                 .param rval=1\n\
                 .step {sweep_type} {target} 1 100 {points}\n\
                 .end\n"
            );
            let error = Netlist::parse(&deck).unwrap_err();
            let message = error.to_string();
            assert!(
                message.contains(&format!(".STEP {sweep_type}"))
                    && message.contains("positive integer representable as usize"),
                "unexpected error for {sweep_type} points={points}: {message}"
            );
        }
    }

    #[test]
    fn unterminated_control_block_is_rejected() {
        let err = Netlist::parse(
            "unterminated control block\n\
             V1 out 0 1\n\
             .control\n\
             print v(out)\n\
             .op\n\
             .end\n",
        )
        .expect_err(
            "unterminated .control must fail instead of commenting out the rest of the deck",
        );

        let message = err.to_string().to_ascii_lowercase();
        assert!(
            message.contains(".control") && message.contains(".endc"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn top_level_ends_is_rejected() {
        let err = Netlist::parse(
            "top level ends\n\
             R1 out 0 1k\n\
             .ends\n\
             .op\n\
             .end\n",
        )
        .expect_err("top-level .ENDS must fail instead of being ignored");

        let message = err.to_string();
        assert!(
            message.contains(".ENDS") && message.contains(".SUBCKT"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn mismatched_subckt_end_name_closes_the_current_subcircuit() {
        let netlist = Netlist::parse(
            "mismatched subckt end\n\
             .subckt AMP in out\n\
             R1 in out 1k\n\
             .ends FILTER\n\
             X1 a b AMP\n\
             .end\n",
        )
        .expect("ngspice treats the optional .ENDS label as documentary");

        assert!(
            netlist
                .subcircuits
                .iter()
                .any(|subckt| subckt.name.eq_ignore_ascii_case("AMP"))
        );
    }

    #[test]
    fn split_subckt_end_name_can_match_open_subckt_name() {
        let netlist = Netlist::parse(
            "split subckt end name\n\
             .subckt count10 in out\n\
             R1 in out 1k\n\
             .ends count 10\n\
             X1 a b count10\n\
             .end\n",
        )
        .expect("ngspice accepts whitespace-split .ENDS names in example decks");

        assert!(
            netlist
                .subcircuits
                .iter()
                .any(|subckt| subckt.name.eq_ignore_ascii_case("COUNT10"))
        );
    }

    #[test]
    fn slash_inline_comments_do_not_extend_subckt_end_names() {
        let netlist = Netlist::parse(
            "slash comment after ends\n\
             .subckt sar_adc in out\n\
             R1 in out 1k\n\
             .ends // SUBCKT sar_adc\n\
             Rtop out 0 1k\n\
             .end\n",
        )
        .expect("ngspice-style // inline comment after .ENDS should parse");

        assert!(
            netlist
                .subcircuits
                .iter()
                .any(|subckt| subckt.name.eq_ignore_ascii_case("SAR_ADC"))
        );
    }

    fn assert_missing_subcircuit_ends(
        error: ParseError,
        authored_name: &str,
        canonical_name: &str,
        qualified_name: &str,
        opened_path: Option<&Path>,
        opened_line: usize,
        detected_path: Option<&Path>,
        detected_line: usize,
        boundary: MissingSubcircuitEndsBoundary,
    ) {
        match error {
            ParseError::MissingSubcircuitEnds(error) => {
                let MissingSubcircuitEndsError {
                    authored_name: actual_authored,
                    canonical_name: actual_canonical,
                    qualified_name: actual_qualified,
                    opened_at,
                    detected_at,
                    boundary: actual_boundary,
                } = *error;
                assert_eq!(actual_authored, authored_name);
                assert_eq!(actual_canonical, canonical_name);
                assert_eq!(actual_qualified, qualified_name);
                assert_eq!(opened_at.path.as_deref(), opened_path);
                assert_eq!(opened_at.line, opened_line);
                assert_eq!(detected_at.path.as_deref(), detected_path);
                assert_eq!(detected_at.line, detected_line);
                assert_eq!(actual_boundary, boundary);
            }
            other => panic!("expected MissingSubcircuitEnds, got {other:?}"),
        }
    }

    #[test]
    fn missing_subcircuit_ends_is_typed_at_end_alter_and_eof_boundaries() {
        for (source, detected_line, boundary) in [
            (
                "missing at END\n.subckt Cell a b\nR1 a b 1\n.end\n",
                4,
                MissingSubcircuitEndsBoundary::EndCard,
            ),
            (
                "missing at ALTER\n.subckt Cell a b\nR1 a b 1\n.alter\n",
                4,
                MissingSubcircuitEndsBoundary::AlterCard,
            ),
            (
                "missing at EOF\n.subckt Cell a b\nR1 a b 1\n",
                4,
                MissingSubcircuitEndsBoundary::EndOfSource,
            ),
        ] {
            let error = Netlist::parse(source).expect_err("missing .ENDS must be rejected");
            assert_missing_subcircuit_ends(
                error,
                "Cell",
                "CELL",
                "CELL",
                None,
                2,
                None,
                detected_line,
                boundary,
            );
        }
    }

    #[test]
    fn nested_missing_subcircuit_reports_innermost_qualified_scope() {
        let error = Netlist::parse(
            "nested missing ends\n\
             .subckt Outer a b\n\
             .subckt Inner x y\n\
             R1 x y 1\n",
        )
        .expect_err("innermost missing .ENDS must be rejected");
        assert_missing_subcircuit_ends(
            error,
            "Inner",
            "INNER",
            "OUTER.INNER",
            None,
            3,
            None,
            5,
            MissingSubcircuitEndsBoundary::EndOfSource,
        );
    }

    #[test]
    fn included_missing_subcircuit_retains_child_source_provenance() {
        let dir = cancellation_fixture_path("missing-ends-include");
        std::fs::create_dir_all(&dir).expect("create include provenance fixture");
        let deck = dir.join("deck.cir");
        let child = dir.join("missing.ends");
        std::fs::write(&deck, "include missing ends\n.include missing.ends\n.end\n")
            .expect("write include owner");
        std::fs::write(&child, ".subckt testsub a b\nR1 a b 1\nR2 b 0 1\n")
            .expect("write missing child");

        let error = Netlist::parse_file(&deck).expect_err("included missing .ENDS must fail");
        let child = child.canonicalize().expect("canonical child");
        assert_missing_subcircuit_ends(
            error,
            "testsub",
            "TESTSUB",
            "TESTSUB",
            Some(&child),
            1,
            Some(&child),
            4,
            MissingSubcircuitEndsBoundary::EndOfSource,
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn included_end_card_is_a_source_owned_missing_ends_boundary() {
        let dir = cancellation_fixture_path("missing-ends-child-end");
        std::fs::create_dir_all(&dir).expect("create included END fixture");
        let deck = dir.join("deck.cir");
        let child = dir.join("child.inc");
        std::fs::write(&deck, "included END\n.include child.inc\n.end\n")
            .expect("write include owner");
        std::fs::write(
            &child,
            ".subckt child a b\nR1 a b 1\n.end\nRignored 9 0 9\n",
        )
        .expect("write child END fixture");

        let error = Netlist::parse_file(&deck).expect_err("child .END cannot replace .ENDS");
        let child = child.canonicalize().expect("canonical child");
        assert_missing_subcircuit_ends(
            error,
            "child",
            "CHILD",
            "CHILD",
            Some(&child),
            1,
            Some(&child),
            3,
            MissingSubcircuitEndsBoundary::EndCard,
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn mapped_control_transformations_preserve_missing_ends_origins() {
        let dir = cancellation_fixture_path("missing-ends-control-origin");
        std::fs::create_dir_all(&dir).expect("create control origin fixture");
        let deck = dir.join("deck.cir");
        let child = dir.join("child.inc");
        std::fs::write(&deck, "control origin\n.include child.inc\n.end\n")
            .expect("write control owner");
        std::fs::write(
            &child,
            ".control\nop\n.endc\n.subckt shifted a b\nR1 a b 1\n",
        )
        .expect("write control child");

        let error = Netlist::parse_file(&deck).expect_err("missing child .ENDS must fail");
        let child = child.canonicalize().expect("canonical child");
        assert_missing_subcircuit_ends(
            error,
            "shifted",
            "SHIFTED",
            "SHIFTED",
            Some(&child),
            4,
            Some(&child),
            6,
            MissingSubcircuitEndsBoundary::EndOfSource,
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn included_control_warning_retains_child_source_provenance() {
        let dir = cancellation_fixture_path("control-warning-origin");
        std::fs::create_dir_all(&dir).expect("create control warning fixture");
        let deck = dir.join("deck.cir");
        let child = dir.join("child.inc");
        std::fs::write(&deck, "control warning\n.include child.inc\n.end\n")
            .expect("write control warning owner");
        std::fs::write(&child, ".control\nop\nprint v(out)\n.endc\nR1 out 0 1k\n")
            .expect("write included control block");

        let netlist = Netlist::parse_file(&deck).expect("included control block is sanitized");
        let canonical_child = child.canonicalize().expect("canonical child");
        let warning = netlist
            .diagnostics
            .iter()
            .find(|diagnostic| diagnostic.code == "control-command-dropped")
            .expect("sanitizer warning is retained");

        assert_eq!(warning.line, 3);
        assert!(warning.message.contains("'print'"), "{}", warning.message);
        assert_eq!(
            warning.origin,
            Some(NetlistSourceLocation::in_file(&canonical_child, 3))
        );
        let promoted = netlist
            .control_dispositions
            .iter()
            .find(|record| record.command == "op")
            .expect("the promoted command keeps its own record");
        assert_eq!(
            promoted.origin,
            Some(NetlistSourceLocation::in_file(&canonical_child, 2))
        );
        assert_eq!(
            promoted.disposition,
            ControlCommandDisposition::Promoted {
                directives: vec![".op".to_owned()],
            }
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    /// The mixed control block from the audit: one directly promoted command,
    /// one promoted through a scalar `let`, a promotable measurement, an
    /// unused assignment, and two pieces of interactive scripting.
    fn mixed_control_deck() -> &'static str {
        "mixed control\n\
         V1 in 0 1\n\
         R1 in out 1k\n\
         C1 out 0 1n\n\
         .control\n\
         tran 1n 100n\n\
         let tstop = 250n\n\
         let unused = 7\n\
         tran 1n $&tstop\n\
         meas tran vmax max v(out)\n\
         print v(out)\n\
         wrdata out.csv v(out)\n\
         .endc\n\
         .end\n"
    }

    #[test]
    fn every_control_command_reports_its_own_disposition() {
        let netlist = Netlist::parse(mixed_control_deck()).expect("mixed control deck parses");

        let records = netlist
            .control_dispositions
            .iter()
            .map(|record| {
                (
                    record.line,
                    record.command.as_str(),
                    record.disposition.clone(),
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(
            records,
            vec![
                (
                    6,
                    "tran",
                    ControlCommandDisposition::Promoted {
                        directives: vec![".tran 1n 100n".to_owned()],
                    }
                ),
                (
                    7,
                    "let",
                    ControlCommandDisposition::ConsumedByPromotion {
                        name: "TSTOP".to_owned(),
                    }
                ),
                (8, "let", ControlCommandDisposition::Dropped),
                (
                    9,
                    "tran",
                    ControlCommandDisposition::Promoted {
                        directives: vec![".tran 1n {250n}".to_owned()],
                    }
                ),
                (
                    10,
                    "meas",
                    ControlCommandDisposition::Promoted {
                        directives: vec![".meas tran vmax max v(out)".to_owned()],
                    }
                ),
                (11, "print", ControlCommandDisposition::Dropped),
                (12, "wrdata", ControlCommandDisposition::Dropped),
            ]
        );
    }

    #[test]
    fn dropped_control_commands_warn_one_by_one_and_promotions_stay_quiet() {
        let netlist = Netlist::parse(mixed_control_deck()).expect("mixed control deck parses");

        let control: Vec<_> = netlist
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code.starts_with("control-"))
            .collect();
        assert_eq!(
            control
                .iter()
                .map(|diagnostic| diagnostic.line)
                .collect::<Vec<_>>(),
            vec![8, 11, 12],
            "one warning per ignored command and nothing for a promotion"
        );
        assert!(
            control
                .iter()
                .all(|diagnostic| diagnostic.code == "control-command-dropped"),
            "the blanket per-region code is gone"
        );
        assert!(
            control[1].message.contains("'print'"),
            "{}",
            control[1].message
        );
    }

    #[test]
    fn control_promotion_keeps_the_sanitized_deck_byte_identical() {
        let sanitized =
            Netlist::sanitize_control_regions_with_abort(mixed_control_deck(), &NoAbort)
                .expect("mixed control deck sanitizes")
                .0;

        assert_eq!(
            sanitized,
            "mixed control\n\
             V1 in 0 1\n\
             R1 in out 1k\n\
             C1 out 0 1n\n\
             * .control\n\
             * tran 1n 100n\n\
             * let tstop = 250n\n\
             * let unused = 7\n\
             * tran 1n $&tstop\n\
             * meas tran vmax max v(out)\n\
             * print v(out)\n\
             * wrdata out.csv v(out)\n\
             * .endc\n\
             .tran 1n 100n\n\
             .tran 1n {250n}\n\
             .meas tran vmax max v(out)\n\
             .end\n"
        );
    }

    #[test]
    fn strip_control_blocks_sanitizes_without_promoting_or_warning() {
        let stripped = Netlist::strip_control_blocks("deck\n.control\ntran 1n 100n\n.endc\n.end\n")
            .expect("control block strips");

        assert_eq!(
            stripped, "deck\n* .control\n* tran 1n 100n\n* .endc\n.end\n",
            "the sanitize-only entry point never appends a promoted directive"
        );
    }

    #[test]
    fn unmatched_control_boundaries_remain_syntax_errors() {
        let opened = Netlist::parse("deck\n.control\ntran 1n 100n\n.end\n")
            .expect_err(".control without .endc is a syntax error");
        assert!(
            matches!(opened, ParseError::Syntax { line: 2, ref message }
                if message.contains(".CONTROL without a matching .ENDC")),
            "{opened:?}"
        );

        let closed = Netlist::parse("deck\nR1 a 0 1k\n.endc\n.end\n")
            .expect_err(".endc without .control is a syntax error");
        assert!(
            matches!(closed, ParseError::Syntax { line: 3, ref message }
                if message.contains(".ENDC without matching .CONTROL")),
            "{closed:?}"
        );
    }

    #[test]
    fn balanced_includes_preserve_parent_and_nested_subcircuit_scopes() {
        let dir = cancellation_fixture_path("balanced-subckt-includes");
        std::fs::create_dir_all(&dir).expect("create balanced include fixture");
        let deck = dir.join("deck.cir");
        std::fs::write(
            &deck,
            "balanced includes\n\
             .subckt outer a b\n\
             .include body.inc\n\
             .include nested.inc\n\
             .ends outer\n\
             X1 1 0 outer\n\
             .end\n",
        )
        .expect("write balanced include owner");
        std::fs::write(dir.join("body.inc"), "Rbody a b 1\n.end\nRignored 9 0 9\n")
            .expect("write parent body include");
        std::fs::write(
            dir.join("nested.inc"),
            ".subckt inner x y\nRinner x y 2\n.ends inner\n",
        )
        .expect("write balanced nested include");

        let netlist = Netlist::parse_file(&deck).expect("balanced includes parse");
        let outer = netlist
            .subcircuits
            .iter()
            .find(|subckt| subckt.name.eq_ignore_ascii_case("OUTER"))
            .expect("outer subcircuit retained");
        assert!(outer.elements.iter().any(|element| element.name == "RBODY"));
        assert!(
            outer
                .nested_subcircuits
                .iter()
                .any(|subckt| subckt.name.eq_ignore_ascii_case("OUTER.INNER"))
        );
        assert!(
            !outer
                .elements
                .iter()
                .any(|element| element.name == "RIGNORED")
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn included_source_cannot_close_a_parent_subcircuit() {
        let dir = cancellation_fixture_path("cross-source-ends");
        std::fs::create_dir_all(&dir).expect("create cross-source ENDS fixture");
        let deck = dir.join("deck.cir");
        std::fs::write(
            &deck,
            "cross-source ends\n.subckt outer a b\n.include child.inc\n.end\n",
        )
        .expect("write cross-source owner");
        std::fs::write(dir.join("child.inc"), ".ends outer\n").expect("write cross-source closer");

        let error = Netlist::parse_file(&deck).expect_err("child cannot close parent .SUBCKT");
        assert!(
            error
                .to_string()
                .contains("included source closed a .SUBCKT opened by its parent source"),
            "{error}"
        );
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn included_end_card_prevents_cross_source_continuation() {
        let dir = cancellation_fixture_path("included-end-continuation");
        std::fs::create_dir_all(&dir).expect("create included END continuation fixture");
        let deck = dir.join("deck.cir");
        std::fs::write(
            &deck,
            "included END continuation\n.include child.inc\n+ TC=1\n.end\n",
        )
        .expect("write continuation owner");
        std::fs::write(dir.join("child.inc"), "R1 1 0 1k\n.end\n").expect("write terminal child");

        Netlist::parse_file(&deck)
            .expect_err("parent continuation cannot attach across an included .END boundary");
        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn ordinary_include_boundaries_preserve_textual_continuations() {
        let dir = cancellation_fixture_path("include-continuations");
        std::fs::create_dir_all(&dir).expect("create include continuation fixture");

        let parent_base = dir.join("parent-base.cir");
        std::fs::write(
            &parent_base,
            "parent base continuation\nR1 1 0\n.include child-leading.inc\n.end\n",
        )
        .expect("write parent-base owner");
        std::fs::write(dir.join("child-leading.inc"), "+ 1k\n")
            .expect("write child leading continuation");
        let parsed = Netlist::parse_file(&parent_base)
            .expect("child leading continuation extends the parent base line");
        assert_eq!(parsed.elements.len(), 1);

        let child_base = dir.join("child-base.cir");
        std::fs::write(
            &child_base,
            "child base continuation\n.include child-base.inc\n+ 1k\n.end\n",
        )
        .expect("write child-base owner");
        std::fs::write(dir.join("child-base.inc"), "R1 1 0\n").expect("write child base statement");
        let parsed = Netlist::parse_file(&child_base)
            .expect("parent continuation extends the child base line");
        assert_eq!(parsed.elements.len(), 1);

        let subckt_header = dir.join("subckt-header.cir");
        std::fs::write(
            &subckt_header,
            "subckt header continuation\n\
             .subckt cell a b\n\
             .include subckt-params.inc\n\
             R1 a b {VALUE}\n\
             .ends\n\
             X1 1 0 cell VALUE=2\n\
             .end\n",
        )
        .expect("write subckt-header owner");
        std::fs::write(dir.join("subckt-params.inc"), "+ PARAMS: VALUE=1\n")
            .expect("write subckt header continuation");
        let parsed = Netlist::parse_file(&subckt_header)
            .expect("source entry depth is established after a continued .SUBCKT header");
        assert_eq!(parsed.subcircuits.len(), 1);

        let _ = std::fs::remove_dir_all(dir);
    }

    #[test]
    fn analysis_commands_reject_unconsumed_trailing_tokens() {
        for line in [
            ".op garbage",
            ".ac dec 10 1 1Meg garbage",
            ".tran 1n 1u garbage",
        ] {
            let err = Netlist::parse(&format!(
                "analysis trailing tokens\n\
                 V1 out 0 1\n\
                 R1 out 0 1k\n\
                 {line}\n\
                 .end\n"
            ))
            .expect_err("analysis command must reject unconsumed trailing tokens");

            let message = err.to_string();
            assert!(
                message.contains("trailing") || message.contains("Unexpected"),
                "unexpected error for {line}: {message}"
            );
        }
    }

    #[test]
    fn temp_command_rejects_non_numeric_tokens() {
        let err = Netlist::parse(
            "bad temperature card\n\
             V1 out 0 1\n\
             R1 out 0 1k\n\
             .temp bogus\n\
             .op\n\
             .end\n",
        )
        .expect_err(".TEMP with a non-numeric token must fail instead of defaulting to 27 C");

        let message = err.to_string().to_ascii_lowercase();
        assert!(
            message.contains("bogus") || message.contains("unexpected"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn unterminated_model_parameter_list_is_rejected() {
        let err = Netlist::parse(
            "unterminated model params\n\
             D1 out 0 dmod\n\
             .model dmod D(IS=1e-14 RS=1\n\
             .op\n\
             .end\n",
        )
        .expect_err("unterminated parenthesized .MODEL parameters must fail");

        let message = err.to_string();
        assert!(
            message.contains(".MODEL") && message.contains(")"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn malformed_model_parameter_token_is_rejected() {
        let err = Netlist::parse(
            "malformed model params\n\
             D1 out 0 dmod\n\
             .model dmod D(=1 IS=1e-14)\n\
             .op\n\
             .end\n",
        )
        .expect_err("malformed .MODEL parameter tokens must fail instead of being skipped");

        let message = err.to_string();
        assert!(
            message.contains(".MODEL") || message.contains("model parameter"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn noise_analysis_rejects_invalid_sweep_variation() {
        let err = Netlist::parse(
            "bad noise sweep\n\
             V1 in 0 AC 1\n\
             R1 in out 1k\n\
             R2 out 0 1k\n\
             .noise V(out) V1 BOGUS 10 1 1Meg\n\
             .end\n",
        )
        .expect_err("invalid .NOISE sweep variation must fail");

        let message = err.to_string();
        assert!(
            message.contains("BOGUS") && message.contains("frequency variation"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn jfet_off_flag_stays_instance_parameter() {
        let netlist = Netlist::parse(
            "jfet off\n\
             J1 d g s njmod OFF AREA=2 M=3\n\
             .model njmod NJF(BETA=1m VTO=-1)\n\
             .end\n",
        )
        .expect("JFET OFF flag parses");

        match first_jfet(&netlist) {
            ElementKind::Jfet {
                model,
                instance_params,
                ..
            } => {
                assert!(model.eq_ignore_ascii_case("njmod"));
                assert!(
                    instance_params
                        .iter()
                        .any(|(name, value)| name == "OFF" && (*value - 1.0).abs() < f64::EPSILON)
                );
                assert!(
                    instance_params
                        .iter()
                        .any(|(name, value)| name == "AREA" && (*value - 2.0).abs() < f64::EPSILON)
                );
                assert!(
                    instance_params
                        .iter()
                        .any(|(name, value)| name == "M" && (*value - 3.0).abs() < f64::EPSILON)
                );
            }
            _ => unreachable!("first_jfet only returns JFETs"),
        }
    }

    #[test]
    fn mesfet_positional_area_stays_instance_parameter() {
        let netlist = Netlist::parse(
            "mesfet area\n\
             Z1 d g s zm 2 M=4\n\
             .model zm NMF(BETA=1m VTO=-1)\n\
             .end\n",
        )
        .expect("MESFET positional area parses");

        match first_mesfet(&netlist) {
            ElementKind::Mesfet {
                model,
                instance_params,
                ..
            } => {
                assert!(model.eq_ignore_ascii_case("zm"));
                assert!(
                    instance_params
                        .iter()
                        .any(|(name, value)| name == "AREA" && (*value - 2.0).abs() < f64::EPSILON)
                );
                assert!(
                    instance_params
                        .iter()
                        .any(|(name, value)| name == "M" && (*value - 4.0).abs() < f64::EPSILON)
                );
            }
            _ => unreachable!("first_mesfet only returns MESFETs"),
        }
    }

    #[test]
    fn malformed_jfet_parameter_value_is_rejected() {
        let err = Netlist::parse(
            "jfet malformed\n\
             J1 d g s njmod AREA=\n\
             .model njmod NJF(BETA=1m VTO=-1)\n\
             .end\n",
        )
        .expect_err("missing JFET parameter value must fail");

        let message = err.to_string();
        assert!(
            message.contains("Expected value for JFET parameter 'AREA'"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn unsupported_mesfet_instance_token_is_rejected() {
        let err = Netlist::parse(
            "mesfet malformed\n\
             Z1 d g s zm, = AREA=2\n\
             .model zm NMF(BETA=1m VTO=-1)\n\
             .end\n",
        )
        .expect_err("unsupported MESFET tail token must fail");

        let message = err.to_string();
        assert!(
            message.contains("Unsupported MESFET instance token '='"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn diode_positional_area_stays_instance_parameter() {
        let netlist = Netlist::parse(
            "diode area\n\
             D1 a c dmod 2 M=3\n\
             .model dmod D(IS=1n)\n\
             .end\n",
        )
        .expect("diode positional area parses");

        match first_diode(&netlist) {
            ElementKind::Diode {
                model,
                instance_params,
                ..
            } => {
                assert!(model.eq_ignore_ascii_case("dmod"));
                assert!(
                    instance_params
                        .iter()
                        .any(|(name, value)| name == "AREA" && (*value - 2.0).abs() < f64::EPSILON)
                );
                assert!(
                    instance_params
                        .iter()
                        .any(|(name, value)| name == "M" && (*value - 3.0).abs() < f64::EPSILON)
                );
            }
            _ => unreachable!("first_diode only returns diodes"),
        }
    }

    #[test]
    fn malformed_diode_assignment_tail_is_rejected() {
        let err = Netlist::parse(
            "diode malformed\n\
             D1 a c dmod AREA 2\n\
             .model dmod D(IS=1n)\n\
             .end\n",
        )
        .expect_err("missing '=' in diode AREA parameter must fail");

        let message = err.to_string();
        assert!(
            message.contains("diode parameter 'AREA'") && message.contains("expected '='"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn unsupported_diode_instance_token_is_rejected() {
        let err = Netlist::parse(
            "diode malformed\n\
             D1 a c dmod, = AREA=2\n\
             .model dmod D(IS=1n)\n\
             .end\n",
        )
        .expect_err("unsupported diode tail token must fail");

        let message = err.to_string();
        assert!(
            message.contains("Unsupported diode instance token '='"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn bjt_off_flag_stays_instance_parameter() {
        let netlist = Netlist::parse(
            "bjt off\n\
             Q1 c b e qmod OFF AREA=2\n\
             .model qmod NPN(BF=100)\n\
             .end\n",
        )
        .expect("BJT OFF flag parses");

        match first_bjt(&netlist) {
            ElementKind::Bjt {
                model,
                instance_params,
                ..
            } => {
                assert!(model.eq_ignore_ascii_case("qmod"));
                assert!(
                    instance_params
                        .iter()
                        .any(|(name, value)| name == "OFF" && (*value - 1.0).abs() < f64::EPSILON)
                );
                assert!(
                    instance_params
                        .iter()
                        .any(|(name, value)| name == "AREA" && (*value - 2.0).abs() < f64::EPSILON)
                );
            }
            _ => unreachable!("first_bjt only returns BJTs"),
        }
    }

    #[test]
    fn malformed_bjt_assignment_tail_is_rejected_before_substrate_guess() {
        let err = Netlist::parse(
            "bjt malformed\n\
             Q1 c b e qmod AREA 2\n\
             .model qmod NPN(BF=100)\n\
             .end\n",
        )
        .expect_err("missing '=' in BJT AREA parameter must fail");

        let message = err.to_string();
        assert!(
            message.contains("BJT parameter 'AREA'") && message.contains("expected '='"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn unsupported_bjt_instance_token_is_rejected() {
        let err = Netlist::parse(
            "bjt malformed\n\
             Q1 c b e qmod, = AREA=2\n\
             .model qmod NPN(BF=100)\n\
             .end\n",
        )
        .expect_err("unsupported BJT tail token must fail");

        let message = err.to_string();
        assert!(
            message.contains("Unsupported BJT instance token '='"),
            "unexpected error: {message}"
        );
    }

    fn first_source_spec(netlist: &Netlist) -> &SourceSpec {
        netlist
            .elements
            .iter()
            .find_map(|e| match &e.kind {
                ElementKind::VoltageSource(spec) => Some(spec),
                _ => None,
            })
            .expect("voltage source exists")
    }

    #[test]
    fn source_terms_parse_in_any_order() {
        // AC after the transient function (ngspice accepts any order).
        let netlist = Netlist::parse(
            "src order\n\
             Vin 1 0 DC 1 SIN (0 1 100MEG 1NS 0.0) AC 1\n\
             R1 1 0 1k\n\
             .end\n",
        )
        .expect("netlist parses");
        match first_source_spec(&netlist) {
            SourceSpec::DcAcTransient {
                dc_value,
                ac_magnitude,
                transient,
                ..
            } => {
                assert_eq!(*dc_value, 1.0);
                assert_eq!(*ac_magnitude, 1.0);
                assert!(matches!(transient.as_ref(), SourceSpec::Sin { .. }));
            }
            other => panic!("expected DcAcTransient, got {other:?}"),
        }

        // Bare DC level followed by AC.
        let netlist = Netlist::parse(
            "src order\n\
             Vin 1 0 5 AC 2 45\n\
             R1 1 0 1k\n\
             .end\n",
        )
        .expect("netlist parses");
        match first_source_spec(&netlist) {
            SourceSpec::DcAc {
                dc_value,
                ac_magnitude,
                ac_phase,
            } => {
                assert_eq!(*dc_value, 5.0);
                assert_eq!(*ac_magnitude, 2.0);
                assert!((ac_phase - 45.0f64.to_radians()).abs() < 1e-12);
            }
            other => panic!("expected DcAc, got {other:?}"),
        }

        // Transient first, then AC.
        let netlist = Netlist::parse(
            "src order\n\
             Vin 1 0 PULSE(0 1 0 1n 1n 5n 10n) AC 1\n\
             R1 1 0 1k\n\
             .end\n",
        )
        .expect("netlist parses");
        assert!(matches!(
            first_source_spec(&netlist),
            SourceSpec::DcAcTransient { dc_value, .. } if *dc_value == 0.0
        ));

        // Omitted AC magnitude still defaults when followed by a recognized
        // transient source keyword.
        let netlist = Netlist::parse(
            "src order\n\
             Vin 1 0 AC SIN(0 1 1k)\n\
             R1 1 0 1k\n\
             .end\n",
        )
        .expect("omitted AC magnitude before transient parses");
        assert!(matches!(
            first_source_spec(&netlist),
            SourceSpec::DcAcTransient {
                ac_magnitude,
                transient,
                ..
            } if *ac_magnitude == 1.0 && matches!(transient.as_ref(), SourceSpec::Sin { .. })
        ));

        // Ngspice accepts DC with no scalar value when the transient source
        // keyword follows directly.
        let netlist = Netlist::parse(
            "src order\n\
             Vin 1 0 DC PWL(0 0 1m 1)\n\
             R1 1 0 1k\n\
             .tran 1u 1m\n\
             .end\n",
        )
        .expect("omitted DC value before transient parses");
        assert!(matches!(
            first_source_spec(&netlist),
            SourceSpec::DcTransient {
                dc_value,
                transient,
            } if *dc_value == 0.0 && matches!(transient.as_ref(), SourceSpec::Pwl { .. })
        ));
    }

    #[test]
    fn source_dc_terms_ignore_unlabeled_numeric_tail_before_keywords() {
        let netlist = Netlist::parse(
            "source unlabeled tail\n\
             V1 1 0 1 0 2 AC 3 45\n\
             R1 1 0 1k\n\
             .ac lin 1 1k 1k\n\
             .end\n",
        )
        .expect("Xyce-compatible unlabeled source tail should parse");

        assert!(matches!(
            first_source_spec(&netlist),
            SourceSpec::DcAc {
                dc_value,
                ac_magnitude,
                ac_phase,
            } if *dc_value == 1.0
                && *ac_magnitude == 3.0
                && (*ac_phase - 45.0_f64.to_radians()).abs() < 1e-12
        ));
    }

    #[test]
    fn xyce_rf_ports_lower_to_dc_source_and_z0_termination() {
        let netlist = Netlist::parse(
            "xyce rf ports\n\
             P1 OUT 0 DC 2 PORT=1 Z0=75\n\
             P2 LOAD 0 PORT=2 Z0=100\n\
             .dc P1 0 2 1\n\
             .end\n",
        )
        .expect("Xyce RF port cards parse");

        // Every port lowers the same way, driven or not: a generator behind the
        // reference impedance, carrying the declared port number and the node
        // its reference plane sits at. A silent port gets a 0 V generator,
        // which is a short and so leaves the circuit exactly as the bare
        // termination did, but gives S-parameter analysis a branch to drive.
        for (name, node, dc, portnum, z0) in
            [("P1", "OUT", 2.0, 1, 75.0), ("P2", "LOAD", 0.0, 2, 100.0)]
        {
            let internal = format!("__RSPICE_{name}_PORT");
            let element = netlist
                .elements
                .iter()
                .find(|element| element.name == name)
                .unwrap_or_else(|| panic!("{name} keeps its port name"));
            match &element.kind {
                ElementKind::VoltageSource(SourceSpec::RfPort { inner, port }) => {
                    assert!(matches!(**inner, SourceSpec::Dc(value) if value == dc));
                    assert_eq!(port.portnum, portnum);
                    assert_eq!(port.z0, z0);
                    assert_eq!(port.reference_plane.as_deref(), Some(node));
                }
                other => panic!("expected {name} port generator, got {other:?}"),
            }
            assert_eq!(element.nodes, vec![internal.clone(), "0".to_string()]);

            let series = netlist
                .elements
                .iter()
                .find(|element| element.name == format!("__RSPICE_{name}_Z0"))
                .unwrap_or_else(|| panic!("{name} has a series Z0 resistor"));
            match &series.kind {
                ElementKind::Resistor { value, .. } => assert_eq!(*value, z0),
                other => panic!("expected {name} Z0 resistor, got {other:?}"),
            }
            assert_eq!(series.nodes, vec![node.to_string(), internal]);
        }
    }

    #[test]
    fn pulse_source_accepts_an_eighth_pulse_count_argument() {
        let netlist = Netlist::parse(
            "pulse count\n\
             V1 out 0 PULSE(-1 1 0 1e-5 1e-5 5e-4 1e-3 45.0)\n\
             R1 out 0 1k\n\
             .tran 2e-5 2e-3\n\
             .end\n",
        )
        .expect("the eighth PULSE argument should parse");

        assert!(matches!(
            first_source_spec(&netlist),
            SourceSpec::Pulse {
                pulse_count,
                period,
                ..
            } if (*pulse_count - 45.0).abs() < 1e-12 && (*period - 1.0e-3).abs() < 1e-15
        ));
    }

    #[test]
    fn source_distortion_terms_after_sin_are_retained() {
        let netlist = Netlist::parse(
            "distortion source annotation\n\
             V1 1 0 DC 0 AC 1 SIN 0 1 1K 0 0 DISTOF1 0 DISTOF2 0\n\
             R1 1 0 1k\n\
             .ac dec 1 1k 1k\n\
             .end\n",
        )
        .expect("source distortion annotations should parse");

        let spec = first_source_spec(&netlist);
        assert_eq!(
            spec.distortion_f1().expect("DISTOF1 retained").magnitude,
            0.0
        );
        assert_eq!(
            spec.distortion_f2().expect("DISTOF2 retained").magnitude,
            0.0
        );
        assert!(matches!(
            spec,
            SourceSpec::Distortion { inner, .. }
                if matches!(
                    inner.as_ref(),
                    SourceSpec::DcAcTransient { transient, .. }
                        if matches!(transient.as_ref(), SourceSpec::Sin { .. })
                )
        ));
    }

    #[test]
    fn source_distortion_terms_apply_spice_defaults_and_radian_phase() {
        let netlist = Netlist::parse(
            "distortion source defaults\n\
             V1 in 0 DISTOF1 DISTOF2 2 90 DC 0\n\
             R1 in 0 1k\n\
             .disto lin 1 1k 1k 0.9\n\
             .end\n",
        )
        .expect("default and explicit distortion source terms should parse");

        let spec = first_source_spec(&netlist);
        assert_eq!(
            spec.distortion_f1(),
            Some(SourceDistortionTone {
                magnitude: 1.0,
                phase: 0.0,
            })
        );
        let f2 = spec.distortion_f2().expect("DISTOF2 retained");
        assert_eq!(f2.magnitude, 2.0);
        assert!((f2.phase - std::f64::consts::FRAC_PI_2).abs() < 1e-15);
    }

    #[test]
    fn duplicate_source_distortion_tone_is_rejected() {
        let err = Netlist::parse(
            "duplicate distortion tone\n\
             V1 in 0 DISTOF1 1 DISTOF1 2\n\
             R1 in 0 1k\n\
             .end\n",
        )
        .expect_err("duplicate DISTOF1 must be rejected");
        assert!(
            err.to_string()
                .contains("DISTOF1 may be specified at most once")
        );
    }

    #[test]
    fn source_ac_terms_accept_optional_equals() {
        let netlist = Netlist::parse(
            "source ac equals\n\
             V1 1 0 dc=0 ac=1\n\
             I1 0 1 dc=1.27 ac=42mA\n\
             R1 1 0 1k\n\
             .ac lin 1 1k 1k\n\
             .end\n",
        )
        .expect("optional equals after source AC/DC terms should parse");

        let voltage = first_source_spec(&netlist);
        assert!(matches!(
            voltage,
            SourceSpec::DcAc {
                dc_value,
                ac_magnitude,
                ..
            } if *dc_value == 0.0 && *ac_magnitude == 1.0
        ));
        let current = netlist
            .elements
            .iter()
            .find_map(|e| match &e.kind {
                ElementKind::CurrentSource(spec) => Some(spec),
                _ => None,
            })
            .expect("current source exists");
        assert!(matches!(
            current,
            SourceSpec::DcAc {
                dc_value,
                ac_magnitude,
                ..
            } if (*dc_value - 1.27).abs() < 1e-12 && (*ac_magnitude - 0.042).abs() < 1e-12
        ));
    }

    #[test]
    fn source_ac_terms_accept_rf_port_annotations() {
        let netlist = Netlist::parse(
            "source rf port annotations\n\
             V1 p1 0 dc 0 ac 1 portnum 1 z0 75 pwr 1m freq 2.3g phase 45\n\
             R1 p1 0 50\n\
             .ac lin 1 1Meg 1Meg\n\
             .end\n",
        )
        .expect("ngspice source port annotations should parse after AC terms");

        assert!(matches!(
            first_source_spec(&netlist),
            SourceSpec::RfPort {
                inner,
                port,
            } if matches!(
                inner.as_ref(),
                SourceSpec::DcAc {
                    dc_value,
                    ac_magnitude,
                    ac_phase,
                } if *dc_value == 0.0 && *ac_magnitude == 1.0 && *ac_phase == 0.0
            ) && port.portnum == 1
                && port.z0 == 75.0
                && port.power == Some(1.0e-3)
                && port.frequency == Some(2.3e9)
                && port.phase == Some(45.0)
        ));
    }

    #[test]
    fn source_rf_port_defaults_to_ngspice_z0() {
        let netlist = Netlist::parse(
            "source rf port default z0\n\
             V1 p1 0 dc 0 ac 1 portnum 1\n\
             R1 p1 0 50\n\
             .ac lin 1 1Meg 1Meg\n\
             .end\n",
        )
        .expect("source RF port should default z0");

        let port = first_source_spec(&netlist)
            .rf_port()
            .expect("RF port metadata is retained");
        assert_eq!(port.portnum, 1);
        assert_eq!(port.z0, 50.0);
    }

    #[test]
    fn source_ac_dc_equals_after_unparenthesized_transient_parse() {
        let netlist = Netlist::parse(
            "source transient trailing equals\n\
             V1 1 0 SIN 0 1 1k AC=1\n\
             V2 2 0 PULSE 0 1 DC=0 AC=2\n\
             R1 1 0 1k\n\
             R2 2 0 1k\n\
             .ac lin 1 1k 1k\n\
             .end\n",
        )
        .expect("source AC/DC terms with optional equals should parse after transient specs");

        let voltage = first_source_spec(&netlist);
        assert!(matches!(
            voltage,
            SourceSpec::DcAcTransient {
                ac_magnitude,
                transient,
                ..
            } if *ac_magnitude == 1.0 && matches!(transient.as_ref(), SourceSpec::Sin { .. })
        ));

        let pulse = netlist
            .elements
            .iter()
            .find_map(|e| match &e.kind {
                ElementKind::VoltageSource(spec)
                    if matches!(
                        spec,
                        SourceSpec::DcAcTransient {
                            dc_value,
                            ac_magnitude,
                            transient,
                            ..
                        } if *dc_value == 0.0
                            && *ac_magnitude == 2.0
                            && matches!(transient.as_ref(), SourceSpec::Pulse { .. })
                    ) =>
                {
                    Some(spec)
                }
                _ => None,
            })
            .expect("pulse source exists");
        assert!(matches!(
            pulse,
            SourceSpec::DcAcTransient {
                dc_value,
                ac_magnitude,
                transient,
                ..
            } if *dc_value == 0.0
                && *ac_magnitude == 2.0
                && matches!(transient.as_ref(), SourceSpec::Pulse { .. })
        ));
    }

    #[test]
    fn bare_source_dc_levels_accept_spice_unit_suffixes() {
        let netlist = Netlist::parse(
            "source unit suffixes\n\
             V1 1 0 5V\n\
             I1 0 1 10U\n\
             V2 2 0 2K\n\
             R1 1 0 1k\n\
             R2 2 0 1k\n\
             .end\n",
        )
        .expect("bare source DC levels with SPICE unit suffixes should parse");

        let sources = netlist
            .elements
            .iter()
            .filter_map(|element| match &element.kind {
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => Some(spec),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert!(matches!(sources[0], SourceSpec::Dc(value) if (*value - 5.0).abs() < 1e-12));
        assert!(matches!(sources[1], SourceSpec::Dc(value) if (*value - 10e-6).abs() < 1e-18));
        assert!(matches!(sources[2], SourceSpec::Dc(value) if (*value - 2000.0).abs() < 1e-9));
    }

    #[test]
    fn plain_values_and_brace_expressions_share_one_suffix_dialect() {
        // One suffix table serves value positions and expressions alike.
        // Before it did, this deck put V1 and V2 eighteen decades apart: the
        // lexer read `1a` as one ampere-style unit letter while the
        // expression parsers kept ngspice's atto — and V3/V4 forty-fold
        // apart on `mil`, which numparam reads as milli.
        let netlist = Netlist::parse(
            "one suffix dialect\n\
             V1 1 0 1a\n\
             V2 2 0 {1a}\n\
             V3 3 0 1mil\n\
             V4 4 0 {1mil}\n\
             R1 1 0 1\n\
             R2 2 0 1\n\
             R3 3 0 1\n\
             R4 4 0 1\n\
             .end\n",
        )
        .expect("suffixed plain values and brace expressions parse");

        let source_value = |name: &str| -> f64 {
            netlist
                .elements
                .iter()
                .find_map(|element| match &element.kind {
                    ElementKind::VoltageSource(SourceSpec::Dc(value))
                        if element.name.eq_ignore_ascii_case(name) =>
                    {
                        Some(*value)
                    }
                    _ => None,
                })
                .expect("voltage source exists")
        };

        assert_eq!(source_value("V1"), 1.0);
        assert_eq!(source_value("V2"), source_value("V1"));
        assert_eq!(source_value("V3"), 25.4e-6);
        assert_eq!(source_value("V4"), source_value("V3"));
    }

    #[test]
    fn bare_source_dc_levels_accept_bound_parameter_identifiers() {
        let netlist = Netlist::parse(
            "source bound parameter identifiers\n\
             .param VNEG=-10 VPOS=5\n\
             V1 n1 0 VNEG\n\
             V2 n2 0 VPOS\n\
             R1 n1 0 1\n\
             R2 n2 0 1\n\
             .end\n",
        )
        .expect("bare source DC levels may use resolved parameter identifiers");

        let source_value = |name: &str| -> f64 {
            netlist
                .elements
                .iter()
                .find_map(|element| match &element.kind {
                    ElementKind::VoltageSource(SourceSpec::Dc(value))
                        if element.name.eq_ignore_ascii_case(name) =>
                    {
                        Some(*value)
                    }
                    _ => None,
                })
                .expect("voltage source exists")
        };

        assert_eq!(source_value("V1"), -10.0);
        assert_eq!(source_value("V2"), 5.0);
    }

    #[test]
    fn node_names_accept_adjacent_sign_suffixes() {
        let netlist = Netlist::parse(
            "signed node suffixes\n\
             R1 out+ in- 1k\n\
             .end\n",
        )
        .expect("ngspice node names may end in adjacent + or -");

        assert_eq!(netlist.elements[0].nodes, vec!["OUT+", "IN-"]);
    }

    #[test]
    fn node_names_accept_standalone_xyce_punctuation_labels() {
        let netlist = Netlist::parse(
            "standalone punctuation nodes\n\
             R1 + : 2\n\
             V0 1: 0 1\n\
             V1 : 0 1\n\
             .end\n",
        )
        .expect("Xyce standalone punctuation node labels should parse");

        assert_eq!(netlist.elements[0].nodes, vec!["+", ":"]);
        assert_eq!(netlist.elements[1].nodes, vec!["1:", "0"]);
        assert_eq!(netlist.elements[2].nodes, vec![":", "0"]);
    }

    #[test]
    fn digit_leading_node_names_preserve_label_identity() {
        let netlist = Netlist::parse(
            "digit-leading node labels\n\
             R1 1 2a 1k\n\
             R2 2e3 0 1k\n\
             B1 out 0 V={V(2a)+V(2e3)}\n\
             .end\n",
        )
        .expect("digit-leading node labels should parse as node names");

        assert_eq!(netlist.elements[0].nodes, vec!["1", "2A"]);
        assert_eq!(netlist.elements[1].nodes, vec!["2e3", "0"]);
        assert!(matches!(
            &netlist.elements[2].kind,
            ElementKind::BehavioralVoltage { expression, .. } if expression == "V(2a)+V(2e3)"
        ));
    }

    #[test]
    fn resistor_value_model_and_instance_parameters_parse() {
        let netlist = Netlist::parse(
            "modeled resistor\n\
             R1 1 0 100 rmodel l=1u w=10u m=2\n\
             .model rmodel r kf=100e-18 af=1.1\n\
             .end\n",
        )
        .expect("resistor value followed by model and instance params should parse");

        let resistor = netlist
            .elements
            .iter()
            .find_map(|e| match &e.kind {
                ElementKind::Resistor {
                    value,
                    model,
                    instance_params,
                    ..
                } => Some((*value, model.as_deref(), instance_params)),
                _ => None,
            })
            .expect("resistor exists");

        assert_eq!(resistor.0, 100.0);
        assert!(
            resistor
                .1
                .is_some_and(|model| model.eq_ignore_ascii_case("rmodel"))
        );
        assert!(
            resistor
                .2
                .iter()
                .any(|(name, value)| name == "L" && (*value - 1e-6).abs() < 1e-18),
            "L instance parameter should be retained: {:?}",
            resistor.2
        );
        assert!(
            resistor
                .2
                .iter()
                .any(|(name, value)| name == "M" && (*value - 2.0).abs() < 1e-12),
            "M instance parameter should be retained: {:?}",
            resistor.2
        );
    }

    #[test]
    fn resistor_value_followed_by_model_without_instance_params_parse() {
        let netlist = Netlist::parse(
            "modeled resistor without instance params\n\
             R1 1 0 100 rmodel\n\
             .model rmodel r tc1=1e-3\n\
             .end\n",
        )
        .expect("resistor value followed by model should parse");

        let (value, model) = netlist
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Resistor { value, model, .. } => Some((*value, model.as_deref())),
                _ => None,
            })
            .expect("resistor exists");

        assert_eq!(value, 100.0);
        assert!(model.is_some_and(|name| name.eq_ignore_ascii_case("rmodel")));
    }

    #[test]
    fn xyce_value_less_resistors_parse_with_default_diagnostics() {
        let netlist = Netlist::parse(
            "xyce default resistor value\n\
             R1 1 0\n\
             R2 2 0 rmodel\n\
             .model rmodel r rsh=1 level=1\n\
             .end\n",
        )
        .expect("Xyce value-less resistors should parse with warnings");

        assert!(
            netlist.diagnostics.iter().any(|diagnostic| {
                diagnostic.line == 2 && diagnostic.code == "xyce_resistor_missing_value"
            }),
            "plain value-less resistor should emit a missing-value diagnostic: {:?}",
            netlist.diagnostics
        );
        assert!(
            netlist.diagnostics.iter().any(|diagnostic| {
                diagnostic.line == 3 && diagnostic.code == "xyce_resistor_model_missing_value"
            }),
            "model value-less resistor should emit a model-default diagnostic: {:?}",
            netlist.diagnostics
        );
        for name in ["R1", "R2"] {
            let element = netlist
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(name))
                .expect("resistor exists");
            let ElementKind::Resistor {
                value,
                instance_params,
                ..
            } = &element.kind
            else {
                panic!("{name} is not a resistor");
            };
            assert_eq!(*value, 0.0);
            assert!(
                instance_params.iter().any(|(param, _)| {
                    param.eq_ignore_ascii_case(XYCE_DEFAULT_RESISTOR_VALUE_MARKER)
                }),
                "{name} should carry the internal Xyce default marker"
            );
        }
    }

    #[test]
    fn resistor_model_followed_by_unit_suffix_value_parse() {
        let netlist = Netlist::parse(
            "modeled resistor with suffix value override\n\
             .model rseu_d2_lvsres R( r=0.1)\n\
             RLAT_ME N 0 rseu_d2_lvsres 500K\n\
             .end\n",
        )
        .expect("resistor model followed by identifier-shaped value should parse");

        let (value, model) = netlist
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Resistor { value, model, .. } => Some((*value, model.as_deref())),
                _ => None,
            })
            .expect("resistor exists");

        assert_eq!(value, 500_000.0);
        assert!(model.is_some_and(|name| name.eq_ignore_ascii_case("rseu_d2_lvsres")));
    }

    #[test]
    fn passive_model_and_unit_suffix_value_orders_parse() {
        let netlist = Netlist::parse(
            "modeled passive value ordering\n\
             C1 1 0 cmod 1uF IC=1 TEMP=727\n\
             C2 2 0 2uF cmod\n\
             L1 3 0 lmod 10mH TEMP=90\n\
             L2 4 0 20mH lmod\n\
             .model cmod C TC1=1m\n\
             .model lmod L TC2=1u\n\
             .end\n",
        )
        .expect("capacitor and inductor model/value ordering should parse");

        let mut caps = netlist
            .elements
            .iter()
            .filter_map(|element| match &element.kind {
                ElementKind::Capacitor {
                    value,
                    model,
                    initial_voltage,
                    instance_params,
                    ..
                } => Some((
                    element.name.as_str(),
                    *value,
                    model.as_deref(),
                    *initial_voltage,
                    instance_params,
                )),
                _ => None,
            });
        let c1 = caps.next().expect("C1 parsed");
        assert_eq!(c1.0, "C1");
        assert!((c1.1 - 1.0e-6).abs() < 1.0e-18, "C1 value {}", c1.1);
        assert!(c1.2.is_some_and(|model| model.eq_ignore_ascii_case("cmod")));
        assert_eq!(c1.3, Some(1.0));
        assert!(
            c1.4.iter()
                .any(|(name, value)| name == "TEMP" && (*value - 727.0).abs() < 1.0e-12),
            "C1 TEMP instance parameter should be retained: {:?}",
            c1.4
        );
        let c2 = caps.next().expect("C2 parsed");
        assert_eq!(c2.0, "C2");
        assert!((c2.1 - 2.0e-6).abs() < 1.0e-18, "C2 value {}", c2.1);
        assert!(c2.2.is_some_and(|model| model.eq_ignore_ascii_case("cmod")));

        let mut inds = netlist
            .elements
            .iter()
            .filter_map(|element| match &element.kind {
                ElementKind::Inductor {
                    value,
                    model,
                    instance_params,
                    ..
                } => Some((
                    element.name.as_str(),
                    *value,
                    model.as_deref(),
                    instance_params,
                )),
                _ => None,
            });
        let l1 = inds.next().expect("L1 parsed");
        assert_eq!(l1.0, "L1");
        assert!((l1.1 - 10.0e-3).abs() < 1.0e-15, "L1 value {}", l1.1);
        assert!(l1.2.is_some_and(|model| model.eq_ignore_ascii_case("lmod")));
        assert!(
            l1.3.iter()
                .any(|(name, value)| name == "TEMP" && (*value - 90.0).abs() < 1.0e-12),
            "L1 TEMP instance parameter should be retained: {:?}",
            l1.3
        );
        let l2 = inds.next().expect("L2 parsed");
        assert_eq!(l2.0, "L2");
        assert!((l2.1 - 20.0e-3).abs() < 1.0e-15, "L2 value {}", l2.1);
        assert!(l2.2.is_some_and(|model| model.eq_ignore_ascii_case("lmod")));
    }

    #[test]
    fn passive_tc_vectors_canonicalize_without_changing_base_values() {
        let netlist = Netlist::parse(
            "passive TC vectors\n\
             R1 1 0 100 TC=1m,-2u\n\
             C1 2 0 3u TC 4m,-5u\n\
             L1 3 0 6m TC=7m\n\
             .end\n",
        )
        .expect("valid R/C/L TC vector spellings should parse");

        for (name, expected_value, tc1, tc2) in [
            ("R1", 100.0, 1.0e-3, Some(-2.0e-6)),
            ("C1", 3.0e-6, 4.0e-3, Some(-5.0e-6)),
            ("L1", 6.0e-3, 7.0e-3, None),
        ] {
            let (value, params, deferred) = passive_test_state(&netlist.elements, name);
            assert!(
                (value - expected_value).abs() <= expected_value.abs().max(1.0) * 1.0e-12,
                "{name} base value {value}, expected {expected_value}"
            );
            assert!(
                deferred.is_empty(),
                "{name} unexpectedly deferred {deferred:?}"
            );
            assert_unique_passive_param(params, "TC1", tc1);
            if let Some(tc2) = tc2 {
                assert_unique_passive_param(params, "TC2", tc2);
            } else {
                assert!(
                    params
                        .iter()
                        .all(|(parameter, _)| !parameter.eq_ignore_ascii_case("TC2")),
                    "one-component vector must not synthesize TC2: {params:?}"
                );
            }
            assert!(
                params
                    .iter()
                    .all(|(parameter, _)| !parameter.eq_ignore_ascii_case("TC")),
                "raw TC alias must not survive canonicalization: {params:?}"
            );
        }
    }

    #[test]
    fn passive_tc_vector_components_dominate_scalars_component_wise() {
        let netlist = Netlist::parse(
            "passive TC precedence\n\
             R1 1 0 1k TC1=9 TC=1,2 TC1=8 TC2=7\n\
             R2 2 0 1k TC2=4 TC=1\n\
             R3 3 0 1k TC=1,2 TC=3\n\
             C1 4 0 1u TC1=9 TC=1,2 TC1=8 TC2=7\n\
             C2 5 0 1u TC2=4 TC=1\n\
             C3 6 0 1u TC=1,2 TC=3\n\
             L1 7 0 1m TC1=9 TC=1,2 TC1=8 TC2=7\n\
             L2 8 0 1m TC2=4 TC=1\n\
             L3 9 0 1m TC=1,2 TC=3\n\
             .end\n",
        )
        .expect("mixed scalar/vector TC assignments should parse");

        for prefix in ["R", "C", "L"] {
            for (index, expected_tc1, expected_tc2) in [(1, 1.0, 2.0), (2, 1.0, 4.0), (3, 3.0, 2.0)]
            {
                let name = format!("{prefix}{index}");
                let (_, params, deferred) = passive_test_state(&netlist.elements, &name);
                assert!(
                    deferred.is_empty(),
                    "{name} unexpectedly deferred {deferred:?}"
                );
                assert_unique_passive_param(params, "TC1", expected_tc1);
                assert_unique_passive_param(params, "TC2", expected_tc2);
                assert!(
                    params
                        .iter()
                        .all(|(parameter, _)| !parameter.eq_ignore_ascii_case("TC")),
                    "{name} retained raw TC: {params:?}"
                );
            }
        }
    }

    #[test]
    fn deferred_passive_tc_vectors_resolve_in_instance_scope() {
        let netlist = Netlist::parse(
            "deferred passive TC vectors\n\
             X1 1 0 passives PARAMS: A=5 B=6\n\
             .subckt passives p n PARAMS: A=1 B=2\n\
             R1 p n 1k TC1={A+10} TC=1,{B} TC1=9\n\
             C1 p n 1u TC={A},{B} TC2=9\n\
             L1 p n 1m TC1=9 TC={A},2\n\
             .ends\n\
             .end\n",
        )
        .expect("deferred TC vectors should parse");
        let flattened = flatten_netlist_with_models(&netlist)
            .expect("deferred TC vectors should resolve while flattening");

        for (name, expected_tc1, expected_tc2) in [
            ("X1.R1", 1.0, 6.0),
            ("X1.C1", 5.0, 6.0),
            ("X1.L1", 5.0, 2.0),
        ] {
            let (_, params, deferred) = passive_test_state(&flattened.elements, name);
            assert!(deferred.is_empty(), "{name} retained {deferred:?}");
            assert_unique_passive_param(params, "TC1", expected_tc1);
            assert_unique_passive_param(params, "TC2", expected_tc2);
            assert!(
                params
                    .iter()
                    .all(|(parameter, _)| !parameter.eq_ignore_ascii_case("TC")),
                "{name} retained raw TC: {params:?}"
            );
        }
    }

    #[test]
    fn malformed_passive_tc_vectors_fail_closed() {
        for device in ["R1 1 0 1k", "C1 1 0 1u", "L1 1 0 1m"] {
            for invalid in [
                "TC",
                "TC=",
                "TC=,2",
                "TC=1,",
                "TC=1 2",
                "TC=1,,2",
                "TC=1,2,3",
                "TC=1,2, TEMP=4",
            ] {
                let deck = format!("invalid passive TC\n{device} {invalid}\n.end\n");
                assert!(
                    Netlist::parse(&deck).is_err(),
                    "malformed passive vector parsed: {device} {invalid}"
                );
            }
        }
    }

    #[test]
    fn subckt_resistor_bare_r_parameter_flattens_as_value() {
        let netlist = Netlist::parse(
            "subckt bare R parameter value\n\
             X1 1 0 Rsub PARAMS: R=2k\n\
             .subckt Rsub p n PARAMS: R=1k\n\
             R1 p n R\n\
             .ends\n\
             .end\n",
        )
        .expect("bare R parameter in subcircuit resistor should parse");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("bare R parameter should flatten");
        let resistance = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Resistor { value, .. }
                    if element.name.eq_ignore_ascii_case("X1.R1") =>
                {
                    Some(*value)
                }
                _ => None,
            })
            .expect("flattened subcircuit resistor exists");

        assert_eq!(resistance, 2_000.0);
    }

    #[test]
    fn subckt_header_skips_pspice_optional_defaults_before_params() {
        let netlist = Netlist::parse(
            "pspice optional subckt pins\n\
             X1 a b y Gate PARAMS: td=2n\n\
             .subckt Gate a b y\n\
             + optional: DPWR=$G_DPWR DGND=$G_DGND\n\
             + params: td=1n IO_LEVEL=0\n\
             R1 y b 1k\n\
             .ends\n\
             .end\n",
        )
        .expect("PSpice optional subckt defaults should not be normal params");

        let subckt = netlist
            .subcircuits
            .iter()
            .find(|subckt| subckt.name.eq_ignore_ascii_case("Gate"))
            .expect("subcircuit exists");

        assert_eq!(subckt.ports, vec!["A", "B", "Y"]);
        assert!(subckt.params.iter().any(|(name, value)| {
            name.eq_ignore_ascii_case("td") && (*value - 1.0e-9).abs() < 1.0e-21
        }));
        assert!(subckt.params.iter().any(|(name, value)| {
            name.eq_ignore_ascii_case("IO_LEVEL") && (*value - 0.0).abs() < f64::EPSILON
        }));
        assert!(
            subckt
                .params
                .iter()
                .all(|(name, _)| !name.eq_ignore_ascii_case("DPWR")
                    && !name.eq_ignore_ascii_case("DGND")),
            "optional pin defaults must not be numeric subckt params"
        );
        assert!(
            subckt
                .string_params
                .iter()
                .all(|(name, _)| !name.eq_ignore_ascii_case("DPWR")
                    && !name.eq_ignore_ascii_case("DGND")),
            "optional pin defaults must not be string subckt params"
        );
    }

    #[test]
    fn pspice_u_simple_gate_lowers_to_xspice_digital_gate() {
        let netlist = Netlist::parse(
            "pspice u gate\n\
             U1 NAND(3) $G_DPWR $G_DGND a b c y DLY IO_LEVEL=0\n\
             .end\n",
        )
        .expect("simple PSpice U gate should parse through XSPICE lowering");

        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == "U1")
            .expect("U1 exists");

        match &element.kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_nand");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::DigitalVector(vec![
                            "A".to_string(),
                            "B".to_string(),
                            "C".to_string()
                        ]),
                        XspicePort::Digital("Y".to_string())
                    ]
                );
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_gate_ugate_timing_creates_xspice_model_alias() {
        let netlist = Netlist::parse(
            "pspice u gate timing\n\
             U1 NAND(2) $G_DPWR $G_DGND a b y DLY IO_LEVEL=0\n\
             .model DLY UGATE (TPLHTY=10n TPHLTY=20n)\n\
             .end\n",
        )
        .expect("PSpice UGATE timing should create a d_nand model alias");

        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == "U1")
            .expect("U1 exists");

        let alias_name = match &element.kind {
            ElementKind::Xspice {
                model,
                pspice_u_timing,
                ..
            } => {
                assert!(pspice_u_timing.is_none());
                model.as_str()
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        };

        let alias = netlist
            .models
            .iter()
            .find(|model| model.name == alias_name)
            .expect("generated timing alias exists");
        assert_eq!(alias.model_type, "d_nand");
        assert!(
            alias.params.iter().any(|(name, value)| {
                name == "rise_delay" && (*value - 10.0e-9).abs() < 1.0e-21
            })
        );
        assert!(
            alias.params.iter().any(|(name, value)| {
                name == "fall_delay" && (*value - 20.0e-9).abs() < 1.0e-21
            })
        );
        assert!(alias.params.iter().any(|(name, value)| {
            name == "inertial_delay" && (*value - 1.0).abs() < f64::EPSILON
        }));
    }

    #[test]
    fn pspice_u_gate_ugate_timing_resolves_scoped_model_alias() {
        let netlist = Netlist::parse(
            "pspice scoped u timing\n\
             .subckt gate a b y\n\
             U1 NAND(2) DPWR DGND a b y DLY\n\
             .model DLY UGATE (TPLHTY=3n TPHLTY=4n)\n\
             .ends gate\n\
             X1 in1 in2 out gate\n\
             .end\n",
        )
        .expect("PSpice UGATE timing inside subckt should create a scoped alias");

        let subckt = netlist
            .subcircuits
            .iter()
            .find(|subckt| subckt.name.eq_ignore_ascii_case("gate"))
            .expect("subckt exists");
        let alias_name = match &subckt.elements[0].kind {
            ElementKind::Xspice {
                model,
                pspice_u_timing,
                ..
            } => {
                assert!(pspice_u_timing.is_none());
                model.as_str()
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        };
        let alias = netlist
            .models
            .iter()
            .find(|model| model.name == alias_name)
            .expect("generated scoped timing alias exists");
        assert_eq!(alias.model_type, "d_nand");
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "rise_delay" && (*value - 3.0e-9).abs() < 1.0e-21 })
        );
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "fall_delay" && (*value - 4.0e-9).abs() < 1.0e-21 })
        );
    }

    #[test]
    fn pspice_u_gate_ugate_timing_honors_mntymxdly_max_mode() {
        let netlist = Netlist::parse(
            "pspice u gate max timing\n\
             U1 NAND(2) $G_DPWR $G_DGND a b y DLY MNTYMXDLY=2 IO_LEVEL=0\n\
             .model DLY UGATE (TPLHMN=1n TPLHTY=2n TPLHMX=3n TPHLMN=4n TPHLTY=5n TPHLMX=6n)\n\
             .end\n",
        )
        .expect("PSpice UGATE timing should honor MNTYMXDLY=2");

        let alias_name = match &netlist.elements[0].kind {
            ElementKind::Xspice { model, .. } => model.as_str(),
            other => panic!("expected XSPICE lowering, got {other:?}"),
        };
        let alias = netlist
            .models
            .iter()
            .find(|model| model.name == alias_name)
            .expect("generated timing alias exists");
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "rise_delay" && (*value - 3.0e-9).abs() < 1.0e-21 })
        );
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "fall_delay" && (*value - 6.0e-9).abs() < 1.0e-21 })
        );
    }

    #[test]
    fn pspice_u_dff_ueff_timing_creates_xspice_model_alias() {
        let netlist = Netlist::parse(
            "pspice u dff timing\n\
             U1 DFF(1) $G_DPWR $G_DGND $D_HI clear clk data q $D_NC dly\n\
             .model DLY UEFF (TPCLKQLHTY=8n TPCLKQHLTY=9n TPPCQLHTY=2n TPPCQHLTY=3n)\n\
             .end\n",
        )
        .expect("PSpice UEFF timing should create a d_dff model alias");

        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == "U1")
            .expect("U1 exists");

        let alias_name = match &element.kind {
            ElementKind::Xspice {
                model,
                pspice_u_timing,
                ..
            } => {
                assert!(pspice_u_timing.is_none());
                model.as_str()
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        };

        let alias = netlist
            .models
            .iter()
            .find(|model| model.name == alias_name)
            .expect("generated timing alias exists");
        assert_eq!(alias.model_type, "d_dff");
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "clk_delay" && (*value - 9.0e-9).abs() < 1.0e-21 })
        );
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "set_delay" && (*value - 2.0e-9).abs() < 1.0e-21 })
        );
        assert!(
            alias.params.iter().any(|(name, value)| {
                name == "reset_delay" && (*value - 3.0e-9).abs() < 1.0e-21
            })
        );
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "rise_delay" && (*value - 1.0e-9).abs() < 1.0e-21 })
        );
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "fall_delay" && (*value - 1.0e-9).abs() < 1.0e-21 })
        );
    }

    #[test]
    fn pspice_u_jkff_ueff_timing_creates_xspice_model_alias() {
        let netlist = Netlist::parse(
            "pspice u jkff timing\n\
             U3 JKFF(1) $G_DPWR $G_DGND preset clear clk j k q qb dly\n\
             .model DLY UEFF (TPCLKQLHTY=4n TPPCQLHTY=7n)\n\
             .end\n",
        )
        .expect("PSpice UEFF timing should create a d_jkff model alias");

        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == "U3")
            .expect("U3 exists");

        let alias_name = match &element.kind {
            ElementKind::Xspice {
                model,
                pspice_u_timing,
                ..
            } => {
                assert!(pspice_u_timing.is_none());
                model.as_str()
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        };

        let alias = netlist
            .models
            .iter()
            .find(|model| model.name == alias_name)
            .expect("generated timing alias exists");
        assert_eq!(alias.model_type, "d_jkff");
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "clk_delay" && (*value - 4.0e-9).abs() < 1.0e-21 })
        );
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "set_delay" && (*value - 7.0e-9).abs() < 1.0e-21 })
        );
        assert!(
            alias.params.iter().any(|(name, value)| {
                name == "reset_delay" && (*value - 7.0e-9).abs() < 1.0e-21
            })
        );
    }

    #[test]
    fn pspice_u_dlyline_udly_timing_creates_buffer_alias() {
        let netlist = Netlist::parse(
            "pspice u dlyline timing\n\
             U9 DLYLINE $G_DPWR $G_DGND in out dly IO_LEVEL=0\n\
             .model DLY UDLY (DLYTY=12n)\n\
             .end\n",
        )
        .expect("PSpice DLYLINE should lower to d_buffer with UDLY timing");

        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == "U9")
            .expect("U9 exists");

        let alias_name = match &element.kind {
            ElementKind::Xspice {
                model,
                ports,
                pspice_u_timing,
                ..
            } => {
                assert!(pspice_u_timing.is_none());
                assert_eq!(
                    ports,
                    &[
                        XspicePort::Digital("IN".to_string()),
                        XspicePort::Digital("OUT".to_string()),
                    ]
                );
                model.as_str()
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        };

        let alias = netlist
            .models
            .iter()
            .find(|model| model.name == alias_name)
            .expect("generated timing alias exists");
        assert_eq!(alias.model_type, "d_buffer");
        assert!(
            alias.params.iter().any(|(name, value)| {
                name == "rise_delay" && (*value - 12.0e-9).abs() < 1.0e-21
            })
        );
        assert!(
            alias.params.iter().any(|(name, value)| {
                name == "fall_delay" && (*value - 12.0e-9).abs() < 1.0e-21
            })
        );
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "inertial_delay" && value.abs() < f64::EPSILON })
        );
    }

    #[test]
    fn pspice_u_dlyline_udly_timing_honors_parametric_mntymxdly_min_mode() {
        let netlist = Netlist::parse(
            "pspice u dlyline min timing\n\
             .param dlymode=1\n\
             U9 DLYLINE $G_DPWR $G_DGND in out dly MNTYMXDLY={dlymode}\n\
             .model DLY UDLY (DLYMN=2n DLYTY=5n DLYMX=9n)\n\
             .end\n",
        )
        .expect("PSpice UDLY timing should resolve parametric MNTYMXDLY");

        let alias_name = match &netlist.elements[0].kind {
            ElementKind::Xspice { model, .. } => model.as_str(),
            other => panic!("expected XSPICE lowering, got {other:?}"),
        };
        let alias = netlist
            .models
            .iter()
            .find(|model| model.name == alias_name)
            .expect("generated timing alias exists");
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "rise_delay" && (*value - 2.0e-9).abs() < 1.0e-21 })
        );
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "fall_delay" && (*value - 2.0e-9).abs() < 1.0e-21 })
        );
    }

    #[test]
    fn pspice_u_pullup_lowers_to_xspice_pullup() {
        let netlist = Netlist::parse(
            "pspice u pullup\n\
             U10 PULLUP $G_DPWR $G_DGND node\n\
             .end\n",
        )
        .expect("PSpice PULLUP should lower to d_pullup");

        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == "U10")
            .expect("U10 exists");

        match &element.kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_pullup");
                assert_eq!(ports, &[XspicePort::Digital("NODE".to_string())]);
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_pulldn_array_lowers_to_xspice_pulldowns() {
        let netlist = Netlist::parse(
            "pspice u pulldn array\n\
             U11 PULLDN(2) $G_DPWR $G_DGND n1 n2\n\
             .end\n",
        )
        .expect("PSpice PULLDN array should lower to d_pulldown");

        assert_eq!(netlist.elements.len(), 2);
        assert_eq!(netlist.elements[0].name, "U11_0");
        assert_eq!(netlist.elements[1].name, "U11_1");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_pulldown");
                assert_eq!(ports, &[XspicePort::Digital("N1".to_string())]);
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
        match &netlist.elements[1].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_pulldown");
                assert_eq!(ports, &[XspicePort::Digital("N2".to_string())]);
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_bufa_array_lowers_to_buffer_instances_with_timing() {
        let netlist = Netlist::parse(
            "pspice u bufa array\n\
             U12 BUFA(2) $G_DPWR $G_DGND in1 in2 out1 out2 dly\n\
             .model DLY UGATE (TPLHTY=2n TPHLTY=5n)\n\
             .end\n",
        )
        .expect("PSpice BUFA array should lower to d_buffer instances");

        assert_eq!(netlist.elements.len(), 2);
        assert_eq!(netlist.elements[0].name, "U12_0");
        assert_eq!(netlist.elements[1].name, "U12_1");

        match &netlist.elements[1].kind {
            ElementKind::Xspice {
                model,
                ports,
                pspice_u_timing,
                ..
            } => {
                assert!(pspice_u_timing.is_none());
                assert_ne!(model, "d_buffer");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::Digital("IN2".to_string()),
                        XspicePort::Digital("OUT2".to_string())
                    ]
                );
                let alias = netlist
                    .models
                    .iter()
                    .find(|alias| alias.name == *model)
                    .expect("generated timing alias exists");
                assert_eq!(alias.model_type, "d_buffer");
                assert!(alias.params.iter().any(|(name, value)| {
                    name == "rise_delay" && (*value - 2.0e-9).abs() < 1.0e-21
                }));
                assert!(alias.params.iter().any(|(name, value)| {
                    name == "fall_delay" && (*value - 5.0e-9).abs() < 1.0e-21
                }));
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_inva_array_lowers_to_inverter_instances() {
        let netlist = Netlist::parse(
            "pspice u inva array\n\
             U13 INVA(2) $G_DPWR $G_DGND in1 in2 out1 out2 dly\n\
             .end\n",
        )
        .expect("PSpice INVA array should lower to d_inverter instances");

        assert_eq!(netlist.elements.len(), 2);
        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_inverter");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::Digital("IN1".to_string()),
                        XspicePort::Digital("OUT1".to_string())
                    ]
                );
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
        match &netlist.elements[1].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_inverter");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::Digital("IN2".to_string()),
                        XspicePort::Digital("OUT2".to_string())
                    ]
                );
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_anda_array_lowers_to_vector_gate_instances_with_timing() {
        let netlist = Netlist::parse(
            "pspice u anda array\n\
             U14 ANDA(3,2) $G_DPWR $G_DGND a1 b1 c1 a2 b2 c2 y1 y2 dly\n\
             .model DLY UGATE (TPLHTY=4n TPHLTY=6n)\n\
             .end\n",
        )
        .expect("PSpice ANDA array should lower to d_and vector gate instances");

        assert_eq!(netlist.elements.len(), 2);
        assert_eq!(netlist.elements[0].name, "U14_0");
        assert_eq!(netlist.elements[1].name, "U14_1");

        let alias_name = match &netlist.elements[1].kind {
            ElementKind::Xspice {
                model,
                ports,
                pspice_u_timing,
                ..
            } => {
                assert!(pspice_u_timing.is_none());
                assert_eq!(
                    ports,
                    &[
                        XspicePort::DigitalVector(vec![
                            "A2".to_string(),
                            "B2".to_string(),
                            "C2".to_string()
                        ]),
                        XspicePort::Digital("Y2".to_string())
                    ]
                );
                model.as_str()
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        };

        let alias = netlist
            .models
            .iter()
            .find(|alias| alias.name == alias_name)
            .expect("generated timing alias exists");
        assert_eq!(alias.model_type, "d_and");
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "rise_delay" && (*value - 4.0e-9).abs() < 1.0e-21 })
        );
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "fall_delay" && (*value - 6.0e-9).abs() < 1.0e-21 })
        );
    }

    #[test]
    fn pspice_u_xora_array_lowers_to_two_input_xor_instances() {
        let netlist = Netlist::parse(
            "pspice u xora array\n\
             U15 XORA(2) $G_DPWR $G_DGND a1 b1 a2 b2 y1 y2 dly\n\
             .end\n",
        )
        .expect("PSpice XORA array should lower to d_xor instances");

        assert_eq!(netlist.elements.len(), 2);
        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_xor");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::DigitalVector(vec!["A1".to_string(), "B1".to_string()]),
                        XspicePort::Digital("Y1".to_string())
                    ]
                );
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
        match &netlist.elements[1].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_xor");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::DigitalVector(vec!["A2".to_string(), "B2".to_string()]),
                        XspicePort::Digital("Y2".to_string())
                    ]
                );
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_inverter_lowers_to_scalar_xspice_ports() {
        let netlist = Netlist::parse(
            "pspice u inverter\n\
             UINV INV $G_DPWR $G_DGND in out\n\
             .end\n",
        )
        .expect("simple PSpice U inverter should parse through XSPICE lowering");

        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == "UINV")
            .expect("UINV exists");

        match &element.kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_inverter");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::Digital("IN".to_string()),
                        XspicePort::Digital("OUT".to_string())
                    ]
                );
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_dff_lowers_to_xspice_flip_flop() {
        let netlist = Netlist::parse(
            "pspice u dff\n\
             U1 DFF(1) $G_DPWR $G_DGND $D_HI clear clk data q $D_NC dly IO_LEVEL=0\n\
             .end\n",
        )
        .expect("PSpice DFF U-device should lower to d_dff");

        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == "U1")
            .expect("U1 exists");

        match &element.kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_dff");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::Digital("DATA".to_string()),
                        XspicePort::Digital("CLK".to_string()),
                        XspicePort::Null,
                        XspicePort::DigitalInverted("CLEAR".to_string()),
                        XspicePort::Digital("Q".to_string()),
                        XspicePort::Null,
                    ]
                );
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_tff_lowers_to_xspice_toggle_flip_flop() {
        let netlist = Netlist::parse(
            "pspice u tff\n\
             U1 TFF $G_DPWR $G_DGND toggle clk q qb dly\n\
             .end\n",
        )
        .expect("PSpice TFF U-device should lower to the Xyce DIG-compatible model");

        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == "U1")
            .expect("U1 exists");

        match &element.kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "xyce_d_tff");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::Analog("$G_DPWR".to_string()),
                        XspicePort::Analog("$G_DGND".to_string()),
                        XspicePort::Digital("TOGGLE".to_string()),
                        XspicePort::Digital("CLK".to_string()),
                        XspicePort::Conductance("Q".to_string()),
                        XspicePort::Conductance("QB".to_string()),
                    ]
                );
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_digital_constants_create_xspice_drivers() {
        let netlist = Netlist::parse(
            "pspice u digital constants\n\
             U1 DFF(1) $G_DPWR $G_DGND $D_LO clear clk $D_HI q qb dly\n\
             .end\n",
        )
        .expect("PSpice U-device digital constants should get XSPICE drivers");

        assert!(netlist.elements.iter().any(|element| {
            matches!(
                &element.kind,
                ElementKind::Xspice { model, ports, .. }
                    if model == "d_pulldown"
                        && ports == &[XspicePort::Digital("$D_LO".to_string())]
            )
        }));
        assert!(netlist.elements.iter().any(|element| {
            matches!(
                &element.kind,
                ElementKind::Xspice { model, ports, .. }
                    if model == "d_pullup"
                        && ports == &[XspicePort::Digital("$D_HI".to_string())]
            )
        }));

        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == "U1")
            .expect("U1 exists");

        match &element.kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_dff");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::Digital("$D_HI".to_string()),
                        XspicePort::Digital("CLK".to_string()),
                        XspicePort::DigitalInverted("$D_LO".to_string()),
                        XspicePort::DigitalInverted("CLEAR".to_string()),
                        XspicePort::Digital("Q".to_string()),
                        XspicePort::Digital("QB".to_string()),
                    ]
                );
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_dff_array_expands_to_scalar_xspice_instances() {
        let netlist = Netlist::parse(
            "pspice u dff array\n\
             U2 DFF(2) $G_DPWR $G_DGND pre clear clk d1 d2 q1 q2 qb1 qb2 dly\n\
             .end\n",
        )
        .expect("PSpice DFF array should lower to scalar d_dff instances");

        assert_eq!(netlist.elements.len(), 2);
        assert_eq!(netlist.elements[0].name, "U2_0");
        assert_eq!(netlist.elements[1].name, "U2_1");

        match &netlist.elements[1].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_dff");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::Digital("D2".to_string()),
                        XspicePort::Digital("CLK".to_string()),
                        XspicePort::DigitalInverted("PRE".to_string()),
                        XspicePort::DigitalInverted("CLEAR".to_string()),
                        XspicePort::Digital("Q2".to_string()),
                        XspicePort::Digital("QB2".to_string()),
                    ]
                );
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_jkff_lowers_active_low_controls_and_clock() {
        let netlist = Netlist::parse(
            "pspice u jkff\n\
             U3 JKFF(1) $G_DPWR $G_DGND preset clear clk j k q qb dly\n\
             .end\n",
        )
        .expect("PSpice JKFF U-device should lower to d_jkff");

        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == "U3")
            .expect("U3 exists");

        match &element.kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_jkff");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::Digital("J".to_string()),
                        XspicePort::Digital("K".to_string()),
                        XspicePort::DigitalInverted("CLK".to_string()),
                        XspicePort::DigitalInverted("PRESET".to_string()),
                        XspicePort::DigitalInverted("CLEAR".to_string()),
                        XspicePort::Digital("Q".to_string()),
                        XspicePort::Digital("QB".to_string()),
                    ]
                );
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_dltch_array_lowers_to_dlatch_instances() {
        let netlist = Netlist::parse(
            "pspice u dltch array\n\
             U7 DLTCH(2) $G_DPWR $G_DGND preset clear enable d1 d2 q1 q2 qb1 qb2 dly\n\
             .end\n",
        )
        .expect("PSpice DLTCH array should lower to scalar d_dlatch instances");

        assert_eq!(netlist.elements.len(), 2);
        assert_eq!(netlist.elements[0].name, "U7_0");
        assert_eq!(netlist.elements[1].name, "U7_1");

        match &netlist.elements[1].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_dlatch");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::Digital("D2".to_string()),
                        XspicePort::Digital("ENABLE".to_string()),
                        XspicePort::DigitalInverted("PRESET".to_string()),
                        XspicePort::DigitalInverted("CLEAR".to_string()),
                        XspicePort::Digital("Q2".to_string()),
                        XspicePort::Digital("QB2".to_string()),
                    ]
                );
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_dltch_ugff_timing_creates_xspice_model_alias() {
        let netlist = Netlist::parse(
            "pspice u dltch timing\n\
             U7 DLTCH(1) $G_DPWR $G_DGND preset clear enable d q qb dly\n\
             .model DLY UGFF (TPDQLHTY=5n TPDQHLTY=8n TPGQLHTY=3n TPPCQLHTY=2n)\n\
             .end\n",
        )
        .expect("PSpice UGFF timing should create a d_dlatch model alias");

        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == "U7")
            .expect("U7 exists");

        let alias_name = match &element.kind {
            ElementKind::Xspice {
                model,
                pspice_u_timing,
                ..
            } => {
                assert!(pspice_u_timing.is_none());
                model.as_str()
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        };

        let alias = netlist
            .models
            .iter()
            .find(|model| model.name == alias_name)
            .expect("generated timing alias exists");
        assert_eq!(alias.model_type, "d_dlatch");
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "data_delay" && (*value - 8.0e-9).abs() < 1.0e-21 })
        );
        assert!(
            alias.params.iter().any(|(name, value)| {
                name == "enable_delay" && (*value - 3.0e-9).abs() < 1.0e-21
            })
        );
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "set_delay" && (*value - 2.0e-9).abs() < 1.0e-21 })
        );
        assert!(
            alias.params.iter().any(|(name, value)| {
                name == "reset_delay" && (*value - 2.0e-9).abs() < 1.0e-21
            })
        );
    }

    #[test]
    fn pspice_u_srff_lowers_to_srlatch_ports() {
        let netlist = Netlist::parse(
            "pspice u srff\n\
             U8 SRFF(1) $G_DPWR $G_DGND preset clear enable s r q $D_NC dly\n\
             .end\n",
        )
        .expect("PSpice SRFF U-device should lower to d_srlatch");

        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == "U8")
            .expect("U8 exists");

        match &element.kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_srlatch");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::Digital("S".to_string()),
                        XspicePort::Digital("R".to_string()),
                        XspicePort::Digital("ENABLE".to_string()),
                        XspicePort::DigitalInverted("PRESET".to_string()),
                        XspicePort::DigitalInverted("CLEAR".to_string()),
                        XspicePort::Digital("Q".to_string()),
                        XspicePort::Null,
                    ]
                );
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_srff_ugff_timing_creates_xspice_model_alias() {
        let netlist = Netlist::parse(
            "pspice u srff timing\n\
             U8 SRFF(1) $G_DPWR $G_DGND preset clear enable s r q qb dly\n\
             .model DLY UGFF (TPDQLHTY=6n TPGQHLTY=4n TPPCQHLTY=9n)\n\
             .end\n",
        )
        .expect("PSpice UGFF timing should create a d_srlatch model alias");

        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == "U8")
            .expect("U8 exists");

        let alias_name = match &element.kind {
            ElementKind::Xspice {
                model,
                pspice_u_timing,
                ..
            } => {
                assert!(pspice_u_timing.is_none());
                model.as_str()
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        };

        let alias = netlist
            .models
            .iter()
            .find(|model| model.name == alias_name)
            .expect("generated timing alias exists");
        assert_eq!(alias.model_type, "d_srlatch");
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "sr_delay" && (*value - 6.0e-9).abs() < 1.0e-21 })
        );
        assert!(
            alias.params.iter().any(|(name, value)| {
                name == "enable_delay" && (*value - 4.0e-9).abs() < 1.0e-21
            })
        );
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "set_delay" && (*value - 9.0e-9).abs() < 1.0e-21 })
        );
        assert!(
            alias.params.iter().any(|(name, value)| {
                name == "reset_delay" && (*value - 9.0e-9).abs() < 1.0e-21
            })
        );
    }

    #[test]
    fn pspice_u_sequential_devices_reject_required_no_connects() {
        let err = Netlist::parse(
            "pspice u jkff invalid nc\n\
             U4 JKFF(1) $G_DPWR $G_DGND high clear $D_NC j k q qb dly\n\
             .end\n",
        )
        .expect_err("required PSpice JKFF clock cannot be no-connect");

        assert!(
            err.to_string().contains("required clock"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn pspice_u_and3_lowers_to_gate_feeding_tristate() {
        let netlist = Netlist::parse(
            "pspice u and3 tristate\n\
             U16 AND3(3) $G_DPWR $G_DGND a b c enable y dly\n\
             .model DLY UTGATE (TPLHTY=6n TPHLTY=4n)\n\
             .end\n",
        )
        .expect("PSpice AND3 should lower through a zero-delay gate into d_tristate");

        assert_eq!(netlist.elements.len(), 2);
        assert_eq!(netlist.elements[0].name, "U16__GATE");
        match &netlist.elements[0].kind {
            ElementKind::Xspice {
                model,
                ports,
                params,
                pspice_u_timing,
                ..
            } => {
                assert_eq!(model, "d_and");
                assert!(pspice_u_timing.is_none());
                assert_eq!(
                    ports,
                    &[
                        XspicePort::DigitalVector(vec![
                            "A".to_string(),
                            "B".to_string(),
                            "C".to_string()
                        ]),
                        XspicePort::Digital("__PSPICE_U16_TRI".to_string())
                    ]
                );
                assert!(params.iter().any(|(name, value)| {
                    name == "rise_delay" && (*value - 1.0e-12).abs() < f64::EPSILON
                }));
                assert!(params.iter().any(|(name, value)| {
                    name == "fall_delay" && (*value - 1.0e-12).abs() < f64::EPSILON
                }));
                assert!(params.iter().any(|(name, value)| {
                    name == "inertial_delay" && (*value - 1.0).abs() < f64::EPSILON
                }));
            }
            other => panic!("expected primary XSPICE gate, got {other:?}"),
        }

        let alias_name = match &netlist.elements[1].kind {
            ElementKind::Xspice {
                model,
                ports,
                pspice_u_timing,
                ..
            } => {
                assert!(pspice_u_timing.is_none());
                assert_eq!(
                    ports,
                    &[
                        XspicePort::Digital("__PSPICE_U16_TRI".to_string()),
                        XspicePort::Digital("ENABLE".to_string()),
                        XspicePort::Digital("Y".to_string())
                    ]
                );
                model.as_str()
            }
            other => panic!("expected trailing XSPICE tristate, got {other:?}"),
        };

        let alias = netlist
            .models
            .iter()
            .find(|alias| alias.name == alias_name)
            .expect("generated UTGATE alias exists");
        assert_eq!(alias.model_type, "d_tristate");
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "delay" && (*value - 6.0e-9).abs() < 1.0e-21 })
        );
    }

    #[test]
    fn pspice_u_nand3a_array_lowers_to_gate_tristate_pairs() {
        let netlist = Netlist::parse(
            "pspice u nand3a array\n\
             U17 NAND3A(2,2) $G_DPWR $G_DGND a1 b1 a2 b2 enable y1 y2 dly\n\
             .model DLY UTGATE (TPLHTY=3n TPHLTY=5n)\n\
             .end\n",
        )
        .expect("PSpice NAND3A should lower to gate/tristate instance pairs");

        assert_eq!(netlist.elements.len(), 4);
        assert_eq!(netlist.elements[0].name, "U17_0__GATE");
        assert_eq!(netlist.elements[1].name, "U17_0");
        assert_eq!(netlist.elements[2].name, "U17_1__GATE");
        assert_eq!(netlist.elements[3].name, "U17_1");

        match &netlist.elements[2].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_nand");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::DigitalVector(vec!["A2".to_string(), "B2".to_string()]),
                        XspicePort::Digital("__PSPICE_U17_1_TRI".to_string())
                    ]
                );
            }
            other => panic!("expected primary XSPICE gate, got {other:?}"),
        }

        match &netlist.elements[3].kind {
            ElementKind::Xspice {
                model,
                ports,
                pspice_u_timing,
                ..
            } => {
                assert!(pspice_u_timing.is_none());
                assert!(
                    netlist
                        .models
                        .iter()
                        .any(|alias| alias.name == *model && alias.model_type == "d_tristate")
                );
                assert_eq!(
                    ports,
                    &[
                        XspicePort::Digital("__PSPICE_U17_1_TRI".to_string()),
                        XspicePort::Digital("ENABLE".to_string()),
                        XspicePort::Digital("Y2".to_string())
                    ]
                );
            }
            other => panic!("expected trailing XSPICE tristate, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_aoi_compound_lowers_to_zero_delay_terms_and_timed_output() {
        let netlist = Netlist::parse(
            "pspice u aoi compound\n\
             U18 AOI(2,2) $G_DPWR $G_DGND a1 b1 a2 b2 y dly\n\
             .model DLY UGATE (TPLHTY=7n TPHLTY=9n)\n\
             .end\n",
        )
        .expect("PSpice AOI should lower to zero-delay term gates and a timed output gate");

        assert_eq!(netlist.elements.len(), 3);
        assert_eq!(netlist.elements[0].name, "U18_0__GATE");
        assert_eq!(netlist.elements[1].name, "U18_1__GATE");
        assert_eq!(netlist.elements[2].name, "U18");

        match &netlist.elements[0].kind {
            ElementKind::Xspice {
                model,
                ports,
                params,
                pspice_u_timing,
                ..
            } => {
                assert_eq!(model, "d_and");
                assert!(pspice_u_timing.is_none());
                assert_eq!(
                    ports,
                    &[
                        XspicePort::DigitalVector(vec!["A1".to_string(), "B1".to_string()]),
                        XspicePort::Digital("__PSPICE_U18_0_CMP".to_string())
                    ]
                );
                assert!(params.iter().any(|(name, value)| {
                    name == "rise_delay" && (*value - 1.0e-12).abs() < f64::EPSILON
                }));
                assert!(params.iter().any(|(name, value)| {
                    name == "fall_delay" && (*value - 1.0e-12).abs() < f64::EPSILON
                }));
            }
            other => panic!("expected zero-delay term gate, got {other:?}"),
        }

        let alias_name = match &netlist.elements[2].kind {
            ElementKind::Xspice {
                model,
                ports,
                pspice_u_timing,
                ..
            } => {
                assert!(pspice_u_timing.is_none());
                assert_eq!(
                    ports,
                    &[
                        XspicePort::DigitalVector(vec![
                            "__PSPICE_U18_0_CMP".to_string(),
                            "__PSPICE_U18_1_CMP".to_string()
                        ]),
                        XspicePort::Digital("Y".to_string())
                    ]
                );
                model.as_str()
            }
            other => panic!("expected timed output gate, got {other:?}"),
        };

        let alias = netlist
            .models
            .iter()
            .find(|alias| alias.name == alias_name)
            .expect("generated UGATE alias exists");
        assert_eq!(alias.model_type, "d_nor");
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "rise_delay" && (*value - 7.0e-9).abs() < 1.0e-21 })
        );
        assert!(
            alias
                .params
                .iter()
                .any(|(name, value)| { name == "fall_delay" && (*value - 9.0e-9).abs() < 1.0e-21 })
        );
    }

    #[test]
    fn pspice_u_buf3a_array_lowers_to_tristate_instances() {
        let netlist = Netlist::parse(
            "pspice u buf3a array\n\
             U5 BUF3A(2) $G_DPWR $G_DGND in1 in2 enable out1 out2 dly\n\
             .end\n",
        )
        .expect("PSpice BUF3A array should lower to d_tristate instances");

        assert_eq!(netlist.elements.len(), 2);
        assert_eq!(netlist.elements[0].name, "U5_0");
        assert_eq!(netlist.elements[1].name, "U5_1");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_tristate");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::Digital("IN1".to_string()),
                        XspicePort::Digital("ENABLE".to_string()),
                        XspicePort::Digital("OUT1".to_string()),
                    ]
                );
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }

        match &netlist.elements[1].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_tristate");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::Digital("IN2".to_string()),
                        XspicePort::Digital("ENABLE".to_string()),
                        XspicePort::Digital("OUT2".to_string()),
                    ]
                );
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_buf3a_utgate_timing_creates_xspice_model_aliases() {
        let netlist = Netlist::parse(
            "pspice u buf3a timing\n\
             U5 BUF3A(2) $G_DPWR $G_DGND in1 in2 enable out1 out2 dly\n\
             .model DLY UTGATE (TPLHTY=6n TPHLTY=4n)\n\
             .end\n",
        )
        .expect("PSpice UTGATE timing should create d_tristate model aliases");

        assert_eq!(netlist.elements.len(), 2);
        for element in &netlist.elements {
            let alias_name = match &element.kind {
                ElementKind::Xspice {
                    model,
                    pspice_u_timing,
                    ..
                } => {
                    assert!(pspice_u_timing.is_none());
                    model.as_str()
                }
                other => panic!("expected XSPICE lowering, got {other:?}"),
            };

            let alias = netlist
                .models
                .iter()
                .find(|model| model.name == alias_name)
                .expect("generated timing alias exists");
            assert_eq!(alias.model_type, "d_tristate");
            assert!(
                alias
                    .params
                    .iter()
                    .any(|(name, value)| { name == "delay" && (*value - 6.0e-9).abs() < 1.0e-21 })
            );
            assert!(alias.params.iter().any(|(name, value)| {
                name == "inertial_delay" && (*value - 1.0).abs() < f64::EPSILON
            }));
        }
    }

    #[test]
    fn pspice_u_inv3a_lowers_to_tristate_with_inverted_input() {
        let netlist = Netlist::parse(
            "pspice u inv3a\n\
             U6 INV3A(1) $G_DPWR $G_DGND in enable out dly\n\
             .end\n",
        )
        .expect("PSpice INV3A should lower to d_tristate with inverted input");

        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == "U6")
            .expect("U6 exists");

        match &element.kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_tristate");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::DigitalInverted("IN".to_string()),
                        XspicePort::Digital("ENABLE".to_string()),
                        XspicePort::Digital("OUT".to_string()),
                    ]
                );
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_logicexp_lowers_boolean_assignments_to_zero_delay_gates() {
        let netlist = Netlist::parse(
            "pspice u logicexp\n\
             U19 LOGICEXP(3,2) $G_DPWR $G_DGND a b c y sum D0_GATE IO_LEVEL=0\n\
             + LOGIC:\n\
             +   y = {~(a & b) | c}\n\
             +   sum = {a ^ b ^ c}\n\
             .end\n",
        )
        .expect("PSpice LOGICEXP should lower boolean assignments to XSPICE gates");

        assert_eq!(netlist.elements.len(), 3);
        assert_eq!(netlist.elements[0].name, "U19__LOGIC_0");
        assert_eq!(netlist.elements[1].name, "U19__LOGIC_1");
        assert_eq!(netlist.elements[2].name, "U19__LOGIC_2");

        match &netlist.elements[0].kind {
            ElementKind::Xspice {
                model,
                ports,
                params,
                pspice_u_timing,
                ..
            } => {
                assert_eq!(model, "d_nand");
                assert!(pspice_u_timing.is_none());
                assert_eq!(
                    ports,
                    &[
                        XspicePort::DigitalVector(vec!["A".to_string(), "B".to_string()]),
                        XspicePort::Digital("__PSPICE_U19_0_LOGIC".to_string())
                    ]
                );
                assert!(params.iter().any(|(name, value)| {
                    name == "rise_delay" && (*value - 1.0e-12).abs() < f64::EPSILON
                }));
                assert!(params.iter().any(|(name, value)| {
                    name == "fall_delay" && (*value - 1.0e-12).abs() < f64::EPSILON
                }));
            }
            other => panic!("expected first LOGICEXP gate, got {other:?}"),
        }

        match &netlist.elements[1].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_or");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::DigitalVector(vec![
                            "__PSPICE_U19_0_LOGIC".to_string(),
                            "C".to_string()
                        ]),
                        XspicePort::Digital("Y".to_string())
                    ]
                );
            }
            other => panic!("expected final LOGICEXP OR gate, got {other:?}"),
        }

        match &netlist.elements[2].kind {
            ElementKind::Xspice { model, ports, .. } => {
                assert_eq!(model, "d_xor");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::DigitalVector(vec![
                            "A".to_string(),
                            "B".to_string(),
                            "C".to_string()
                        ]),
                        XspicePort::Digital("SUM".to_string())
                    ]
                );
            }
            other => panic!("expected LOGICEXP XOR gate, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_pindly_lowers_outputs_to_delayed_buffers() {
        let netlist = Netlist::parse(
            "pspice u pindly buffers\n\
             U20 PINDLY(2,0,1) $G_DPWR $G_DGND int1 int2 ref out1 out2 IO_STD\n\
             + PINDLY:\n\
             +   out1 out2 = {CASE(DELAY(2ns,-1,6ns))}\n\
             .end\n",
        )
        .expect("PSpice PINDLY should lower delayed outputs to d_buffer instances");

        assert_eq!(netlist.elements.len(), 2);
        assert_eq!(netlist.elements[0].name, "U20_0");
        assert_eq!(netlist.elements[1].name, "U20_1");

        match &netlist.elements[1].kind {
            ElementKind::Xspice {
                model,
                ports,
                params,
                ..
            } => {
                assert_eq!(model, "d_buffer");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::Digital("INT2".to_string()),
                        XspicePort::Digital("OUT2".to_string())
                    ]
                );
                assert!(params.iter().any(|(name, value)| {
                    name == "rise_delay" && (*value - 4.0e-9).abs() < 1.0e-21
                }));
                assert!(params.iter().any(|(name, value)| {
                    name == "fall_delay" && (*value - 4.0e-9).abs() < 1.0e-21
                }));
            }
            other => panic!("expected PINDLY buffer lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_pindly_tristate_lowers_active_low_enable() {
        let netlist = Netlist::parse(
            "pspice u pindly tristate\n\
             U21 PINDLY(1,1,0) $G_DPWR $G_DGND internal oebar output IO_HCT\n\
             + TRISTATE:\n\
             +   ENABLE LO = oebar\n\
             +   output = {CASE(TRN_Z$, DELAY(-1,15ns,25ns))}\n\
             .end\n",
        )
        .expect("PSpice PINDLY TRISTATE should lower to d_tristate");

        assert_eq!(netlist.elements.len(), 1);
        match &netlist.elements[0].kind {
            ElementKind::Xspice {
                model,
                ports,
                params,
                ..
            } => {
                assert_eq!(model, "d_tristate");
                assert_eq!(
                    ports,
                    &[
                        XspicePort::Digital("INTERNAL".to_string()),
                        XspicePort::DigitalInverted("OEBAR".to_string()),
                        XspicePort::Digital("OUTPUT".to_string())
                    ]
                );
                assert!(params.iter().any(|(name, value)| {
                    name == "delay" && (*value - 15.0e-9).abs() < 1.0e-21
                }));
            }
            other => panic!("expected PINDLY tristate lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_pindly_honors_mntymxdly_max_mode() {
        let netlist = Netlist::parse(
            "pspice u pindly max delay mode\n\
             U22 PINDLY(1,0,0) $G_DPWR $G_DGND internal output IO_STD MNTYMXDLY=2\n\
             + PINDLY:\n\
             +   output = {CASE(DELAY(2ns,4ns,8ns))}\n\
             .end\n",
        )
        .expect("PSpice PINDLY should honor MNTYMXDLY=2 as max delay");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { params, .. } => {
                assert!(params.iter().any(|(name, value)| {
                    name == "rise_delay" && (*value - 8.0e-9).abs() < 1.0e-21
                }));
                assert!(params.iter().any(|(name, value)| {
                    name == "fall_delay" && (*value - 8.0e-9).abs() < 1.0e-21
                }));
            }
            other => panic!("expected PINDLY buffer lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_pindly_honors_parametric_mntymxdly_min_mode() {
        let netlist = Netlist::parse(
            "pspice u pindly parametric min delay mode\n\
             .param dlymode=1\n\
             U23 PINDLY(1,0,0) $G_DPWR $G_DGND internal output IO_STD MNTYMXDLY={dlymode}\n\
             + PINDLY:\n\
             +   output = {CASE(DELAY(2ns,4ns,8ns))}\n\
             .end\n",
        )
        .expect("PSpice PINDLY should resolve parametric MNTYMXDLY");

        match &netlist.elements[0].kind {
            ElementKind::Xspice { params, .. } => {
                assert!(params.iter().any(|(name, value)| {
                    name == "rise_delay" && (*value - 2.0e-9).abs() < 1.0e-21
                }));
                assert!(params.iter().any(|(name, value)| {
                    name == "fall_delay" && (*value - 2.0e-9).abs() < 1.0e-21
                }));
            }
            other => panic!("expected PINDLY buffer lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_constraint_accepts_timing_check_sections_without_outputs() {
        let netlist = Netlist::parse(
            "pspice u constraint timing checks\n\
             U24 CONSTRAINT(3) $G_DPWR $G_DGND clk data en IO_STD IO_LEVEL=0\n\
             + FREQ:\n\
             +   NODE=clk\n\
             +   MAXFREQ=32MEG\n\
             + WIDTH:\n\
             +   NODE=clk\n\
             +   MIN_HI=15ns\n\
             +   MIN_LO=15ns\n\
             + SETUP_HOLD:\n\
             +   CLOCK LH = clk\n\
             +   DATA(1) = data\n\
             +   SETUPTIME = 6ns\n\
             .end\n",
        )
        .expect("PSpice CONSTRAINT timing checks should parse as non-driving metadata");

        assert!(
            netlist.elements.is_empty(),
            "CONSTRAINT should not emit circuit-driving elements"
        );
    }

    /// The compiled stimulus program attached to a lowered STIM instance.
    fn pspice_u_stim_program(netlist: &Netlist, name: &str) -> String {
        let element = netlist
            .elements
            .iter()
            .find(|element| element.name.eq_ignore_ascii_case(name))
            .unwrap_or_else(|| panic!("{name} exists"));
        match &element.kind {
            ElementKind::Xspice {
                model,
                string_params,
                ..
            } => {
                assert_eq!(model, "pspice_d_stim");
                string_params
                    .iter()
                    .find(|(key, _)| key == "stim_program")
                    .map(|(_, program)| program.clone())
                    .expect("a lowered STIM carries its compiled program")
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
    }

    #[test]
    fn pspice_u_stim_lowers_to_a_digital_stimulus_instance() {
        let netlist = Netlist::parse(
            "pspice u stim\n\
             U1 STIM(1,1) $G_DPWR $G_DGND OUT IO_STM\n\
             + 0s 0\n\
             + +10ns 1\n\
             + 25ns 0\n\
             .end\n",
        )
        .expect("PSpice STIM should lower to a digital stimulus instance");

        let element = netlist
            .elements
            .iter()
            .find(|element| element.name == "U1")
            .expect("U1 exists");
        match &element.kind {
            ElementKind::Xspice {
                model,
                ports,
                pspice_u_timing,
                ..
            } => {
                assert_eq!(model, "pspice_d_stim");
                assert_eq!(ports, &[XspicePort::DigitalVector(vec!["OUT".to_string()])]);
                // The trailing model on a STIM device is an I/O model, not a
                // timing model: a source has no propagation delay to select.
                assert!(pspice_u_timing.is_none());
            }
            other => panic!("expected XSPICE lowering, got {other:?}"),
        }
        // Times are carried as the exact double the deck's suffixed value
        // resolved to, so the program round-trips rather than re-rounding.
        // `25ns` is 2.5000000000000002e-8 through the shared SPICE value
        // lexer, the same double every other numeric field on the card gets.
        assert_eq!(
            pspice_u_stim_program(&netlist, "U1"),
            "W1 V:A:0.0:0 V:R:1e-8:1 V:A:2.5000000000000002e-8:0"
        );
    }

    #[test]
    fn pspice_u_stim_resolves_labels_to_instruction_indices() {
        let netlist = Netlist::parse(
            "pspice u stim goto\n\
             U1 STIM(1,1) $G_DPWR $G_DGND CLK IO_STM\n\
             + 0s 0\n\
             + LABEL=TICK\n\
             + +10ns 1\n\
             + +10ns 0\n\
             + +0s GOTO TICK -1 TIMES\n\
             .end\n",
        )
        .expect("PSpice STIM should resolve its GOTO label");

        // LABEL=TICK names instruction 1, the first command after the initial
        // drive; -1 is the forever count.
        assert_eq!(
            pspice_u_stim_program(&netlist, "U1"),
            "W1 V:A:0.0:0 V:R:1e-8:1 V:R:1e-8:0 G:R:0.0:1:-1"
        );
    }

    #[test]
    fn pspice_u_stim_expands_a_hexadecimal_format_into_bus_bits() {
        let netlist = Netlist::parse(
            "pspice u stim hex\n\
             U1 STIM(8,44) $G_DPWR $G_DGND B7 B6 B5 B4 B3 B2 B1 B0 IO_STM\n\
             + 0s A5\n\
             + 10ns XZ\n\
             .end\n",
        )
        .expect("PSpice STIM should expand hexadecimal values");

        assert_eq!(
            pspice_u_stim_program(&netlist, "U1"),
            "W8 V:A:0.0:10100101 V:A:1e-8:XXXXZZZZ"
        );
    }

    #[test]
    fn pspice_u_stim_scales_clock_relative_times_by_timestep() {
        let netlist = Netlist::parse(
            "pspice u stim timestep\n\
             U1 STIM(1,1) $G_DPWR $G_DGND OUT IO_STM IO_LEVEL=2 TIMESTEP=5ns\n\
             + 0s 0\n\
             + +2c 1\n\
             + 8c 0\n\
             .end\n",
        )
        .expect("PSpice STIM should scale clock-relative times");

        assert_eq!(
            pspice_u_stim_program(&netlist, "U1"),
            "W1 V:A:0.0:0 V:R:1e-8:1 V:A:4e-8:0"
        );
    }

    #[test]
    fn pspice_u_stim_resolves_parametric_command_times() {
        let netlist = Netlist::parse(
            "pspice u stim parametric time\n\
             .param TCLK=10ns\n\
             U1 STIM(1,1) $G_DPWR $G_DGND OUT IO_STM\n\
             + 0s 0\n\
             + {TCLK} 1\n\
             + {2*TCLK} 0\n\
             .end\n",
        )
        .expect("PSpice STIM should resolve parametric times");

        assert_eq!(
            pspice_u_stim_program(&netlist, "U1"),
            "W1 V:A:0.0:0 V:A:1e-8:1 V:A:2e-8:0"
        );
    }

    #[test]
    fn pspice_u_stim_compiles_counting_and_repeat_commands() {
        let netlist = Netlist::parse(
            "pspice u stim counting\n\
             U1 STIM(2,11) $G_DPWR $G_DGND B1 B0 IO_STM\n\
             + 0s 00\n\
             + REPEAT 3 TIMES\n\
             + +10ns INCR BY 01\n\
             + ENDREPEAT\n\
             + +10ns DECR BY 11\n\
             .end\n",
        )
        .expect("PSpice STIM should compile counting and repeat commands");

        assert_eq!(
            pspice_u_stim_program(&netlist, "U1"),
            "W2 V:A:0.0:00 P:3 I:R:1e-8:1 E D:R:1e-8:3"
        );
    }

    #[test]
    fn pspice_u_stim_inside_a_subcircuit_lowers_per_instance() {
        let netlist = Netlist::parse(
            "pspice u stim in subckt\n\
             X1 OUTA CLOCKSRC\n\
             X2 OUTB CLOCKSRC\n\
             .subckt CLOCKSRC Q\n\
             U1 STIM(1,1) $G_DPWR $G_DGND Q IO_STM\n\
             + 0s 0\n\
             + +10ns 1\n\
             .ends\n\
             .end\n",
        )
        .expect("PSpice STIM inside a subcircuit parses");

        let flattened = flatten_netlist_with_models(&netlist).expect("subcircuit flattens");
        for (instance, node) in [("X1.U1", "OUTA"), ("X2.U1", "OUTB")] {
            let element = flattened
                .elements
                .iter()
                .find(|element| element.name.eq_ignore_ascii_case(instance))
                .unwrap_or_else(|| panic!("{instance} exists"));
            match &element.kind {
                ElementKind::Xspice {
                    model,
                    ports,
                    string_params,
                    ..
                } => {
                    assert_eq!(model, "pspice_d_stim");
                    assert_eq!(ports, &[XspicePort::DigitalVector(vec![node.to_string()])]);
                    assert!(
                        string_params
                            .iter()
                            .any(|(key, program)| key == "stim_program"
                                && program == "W1 V:A:0.0:0 V:R:1e-8:1"),
                        "instance program differs: {string_params:?}"
                    );
                }
                other => panic!("expected XSPICE lowering, got {other:?}"),
            }
        }
    }

    #[test]
    fn pspice_u_stim_reports_a_typed_syntax_error_rather_than_panicking() {
        let err = Netlist::parse(
            "pspice u stim malformed\n\
             U1 STIM(3,11) $G_DPWR $G_DGND A B C IO_STM\n\
             + 0s 00\n\
             .end\n",
        )
        .expect_err("a format that does not sum to the width is refused");

        let ParseError::Syntax { line, message } = err else {
            panic!("expected a typed syntax error");
        };
        assert_eq!(line, 2);
        assert!(
            message.contains("PSpice STIM U-device 'U1'")
                && message.contains("must sum to the width"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn pspice_u_unsupported_frontend_families_fail_closed() {
        let err = Netlist::parse(
            "pspice u ram unsupported slice\n\
             U1 RAM(1) $G_DPWR $G_DGND addr data IO_STD\n\
             .end\n",
        )
        .expect_err("RAM lowering is not implemented in this slice");

        assert!(
            err.to_string().contains("Unsupported PSpice U-device type"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn subckt_lookup_is_case_insensitive_when_flattening() {
        let netlist = Netlist::parse(
            "case insensitive subckt lookup\n\
             X1 1 0 RSUB\n\
             .subckt Rsub p n\n\
             R1 p n 1k\n\
             .ends\n\
             .end\n",
        )
        .expect("mixed-case subcircuit deck parses");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("mixed-case subcircuit flattens");

        assert!(flattened.elements.iter().any(|element| {
            element.name.eq_ignore_ascii_case("X1.R1")
                && matches!(element.kind, ElementKind::Resistor { .. })
        }));
    }

    #[test]
    fn subckt_body_param_shadows_top_level_param_when_flattened() {
        let netlist = Netlist::parse(
            "subckt body param scope\n\
             .param RES=5k\n\
             XR1 1 0 ResSub\n\
             .subckt ResSub 1 2\n\
             .param RES=10k\n\
             R1 1 2 {RES}\n\
             .ends\n\
             .end\n",
        )
        .expect("subcircuit body parameter deck parses");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("subcircuit body parameter flattens");
        let resistance = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Resistor { value, .. } => Some(*value),
                _ => None,
            })
            .expect("flattened resistor exists");

        assert_eq!(resistance, 10_000.0);
    }

    #[test]
    fn unused_subckt_default_with_unresolved_param_parses() {
        Netlist::parse(
            "unused deferred subckt default\n\
             .subckt MaybeUsed a b speed1={speed}\n\
             R1 a b 1k\n\
             .ends\n\
             V1 1 0 1\n\
             Rtop 1 0 1k\n\
             .end\n",
        )
        .expect("unused reusable subckt defaults may depend on caller params");
    }

    #[test]
    fn deferred_subckt_default_feeds_body_param_at_flattening() {
        let netlist = Netlist::parse(
            "deferred subckt default and body param\n\
             X1 1 0 Gate\n\
             .subckt Gate a b vcc1={vcc}\n\
             .param Rout={60/(vcc1)}\n\
             R1 a b {Rout}\n\
             .ends\n\
             .param vcc=3\n\
             .end\n",
        )
        .expect("subckt defaults may reference later caller params");

        let flattened = flatten_netlist_with_models(&netlist)
            .expect("deferred subckt defaults resolve while flattening");
        let resistance = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Resistor { value, .. }
                    if element.name.eq_ignore_ascii_case("X1.R1") =>
                {
                    Some(*value)
                }
                _ => None,
            })
            .expect("flattened resistor exists");

        assert!((resistance - 20.0).abs() < 1.0e-12);
    }

    #[test]
    fn deferred_subckt_initial_condition_resolves_at_flattening() {
        let netlist = Netlist::parse(
            "deferred subckt startup directive\n\
             X1 n Cell\n\
             .subckt Cell out bias={vcc}\n\
             .ic v(out)='bias/2'\n\
             R1 out 0 1k\n\
             .ends\n\
             .param vcc=5\n\
             .end\n",
        )
        .expect("subckt .IC may reference caller params");

        let flattened = flatten_netlist_with_models(&netlist)
            .expect("subckt .IC expression resolves while flattening");
        let ic = flattened
            .scoped_initial_conditions
            .iter()
            .find(|ic| ic.node.eq_ignore_ascii_case("n"))
            .expect("scoped initial condition exists");

        assert!((ic.voltage - 2.5).abs() < 1.0e-12);
        assert!(ic.voltage_expr.is_none());
    }

    #[test]
    fn differential_ic_and_nodeset_targets_preserve_both_terminals() {
        for directive in [".IC V(OUT,REF)=1", ".NODESET V(OUT,REF)=1"] {
            let source =
                format!("differential startup target\nV1 OUT 0 1\nV2 REF 0 0\n{directive}\n.END\n");
            let netlist = Netlist::parse(&source).expect("differential startup target parses");
            let (node, reference, voltage) = if directive.starts_with(".IC") {
                let entry = &netlist.initial_conditions[0];
                (&entry.node, entry.reference.as_deref(), entry.voltage)
            } else {
                let entry = &netlist.node_sets[0];
                (&entry.node, entry.reference.as_deref(), entry.voltage)
            };
            assert_eq!(node, "OUT");
            assert_eq!(reference, Some("REF"));
            assert_eq!(voltage, 1.0);
            let sidecar = &netlist.startup_directives()[0].entries()[0];
            assert_eq!(sidecar.execution_node(), "OUT");
            assert_eq!(sidecar.execution_reference(), Some("REF"));
        }
    }

    #[test]
    fn subckt_instance_params_resolve_same_line_expressions_after_overrides() {
        let netlist = Netlist::parse(
            "subckt instance parameter precedence\n\
             .subckt simple in out PARAMS: par1=2.0 par2=2.0 par3='par1*par2*2.0'\n\
             .param par3=100.0\n\
             Rinside in out 'par3'\n\
             .ends\n\
             V1 1 0 1.0\n\
             R1 1 2 1.0\n\
             Xtest 2 0 simple par1=2.0 par2=3.0 par3='par1+par2'\n\
             .end\n",
        )
        .expect("subcircuit instance parameter deck parses");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("subcircuit instance parameters flatten");
        let resistance = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Resistor { value, .. }
                    if element.name.eq_ignore_ascii_case("Xtest.Rinside") =>
                {
                    Some(*value)
                }
                _ => None,
            })
            .expect("flattened subcircuit resistor exists");

        assert_eq!(resistance, 5.0);
    }

    #[test]
    fn parameter_redefinition_policy_selects_first_or_last_definition() {
        let source = "parameter redefinition policy\n\
             .param value=10\n\
             .param value=20\n\
             R1 1 0 {value}\n\
             .end\n";

        for (policy, expected) in [
            (ParameterRedefinitionPolicy::UseFirst, 10.0),
            (ParameterRedefinitionPolicy::UseLast, 20.0),
        ] {
            let netlist = Netlist::parse_with_options(
                source,
                NetlistParseOptions {
                    parameter_redefinition_policy: policy,
                    ..NetlistParseOptions::default()
                },
            )
            .expect("redefinition policy deck parses");
            assert_eq!(netlist.params.get("value"), Some(expected));
            let resistance = netlist
                .elements
                .iter()
                .find_map(|element| match &element.kind {
                    ElementKind::Resistor { value, .. } => Some(*value),
                    _ => None,
                })
                .expect("resistor exists");
            assert_eq!(resistance, expected);
        }
    }

    #[test]
    fn parameter_redefinition_diagnostics_are_typed_and_independent_from_selection() {
        let source = "parameter redefinition diagnostics\n\
             .param Foo=10\n\
             .param fOO=20\n\
             R1 1 0 {foo}\n\
             .end\n";

        for (selection, expected, selected_word) in [
            (ParameterRedefinitionPolicy::UseFirst, 10.0, "first"),
            (ParameterRedefinitionPolicy::UseLast, 20.0, "last"),
        ] {
            let netlist = Netlist::parse_with_options(
                source,
                NetlistParseOptions {
                    parameter_redefinition_policy: selection,
                    parameter_redefinition_diagnostic_policy:
                        ParameterRedefinitionDiagnosticPolicy::Warning,
                    ..NetlistParseOptions::default()
                },
            )
            .expect("warning-mode redefinition parses");
            assert_eq!(netlist.params.get("foo"), Some(expected));
            assert_eq!(netlist.diagnostics.len(), 1);
            let diagnostic = &netlist.diagnostics[0];
            assert_eq!(diagnostic.code, "parameter-redefinition");
            assert_eq!(diagnostic.line, 3);
            assert_eq!(diagnostic.origin, Some(NetlistSourceLocation::in_memory(3)));
            assert_eq!(
                diagnostic.message,
                format!("Parameter FOO defined more than once. Using {selected_word} one.")
            );
        }

        let error = Netlist::parse_with_options(
            source,
            NetlistParseOptions {
                parameter_redefinition_policy: ParameterRedefinitionPolicy::UseFirst,
                parameter_redefinition_diagnostic_policy:
                    ParameterRedefinitionDiagnosticPolicy::Error,
                ..NetlistParseOptions::default()
            },
        )
        .expect_err("error-mode duplicate is rejected");
        let ParseError::ParameterRedefinition(error) = error else {
            panic!("expected typed parameter-redefinition error");
        };
        assert_eq!(error.duplicate_name, "FOO");
        assert_eq!(error.canonical_name, "FOO");
        assert_eq!(error.kind, ParameterDefinitionKind::Parameter);
        assert_eq!(error.first_origin, NetlistSourceLocation::in_memory(2));
        assert_eq!(error.duplicate_origin, NetlistSourceLocation::in_memory(3));
        assert!(
            error
                .to_string()
                .contains("Parameter FOO defined more than once")
        );
    }

    #[test]
    fn parameter_redefinition_diagnostics_cover_same_line_globals_functions_and_scopes() {
        let warning_options = NetlistParseOptions {
            expression_dialect: ExpressionDialect::Xyce,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
            parameter_redefinition_diagnostic_policy:
                ParameterRedefinitionDiagnosticPolicy::Warning,
            ..NetlistParseOptions::default()
        };
        let netlist = Netlist::parse_with_options(
            "redefinition classes\n\
             .param A=1 a=2\n\
             .global_param G=3\n\
             .global_param g=4\n\
             .param F(x)={x+1}\n\
             .param f(y)={y+2}\n\
             .subckt child in out params: local=5\n\
             .param LOCAL=6\n\
             R1 in out {local}\n\
             .ends\n\
             .end\n",
            warning_options,
        )
        .expect("all redefinition classes parse in warning mode");
        let messages = netlist
            .diagnostics
            .iter()
            .filter(|diagnostic| diagnostic.code == "parameter-redefinition")
            .map(|diagnostic| (diagnostic.line, diagnostic.message.as_str()))
            .collect::<Vec<_>>();
        assert_eq!(messages.len(), 4);
        assert!(messages.iter().any(|(line, message)| {
            *line == 2 && message.starts_with("Parameter A defined more than once")
        }));
        assert!(messages.iter().any(|(line, message)| {
            *line == 4 && message.starts_with("Parameter G defined more than once")
        }));
        assert!(messages.iter().any(|(line, message)| {
            *line == 6 && message.starts_with("Parameter F defined more than once")
        }));
        assert!(messages.iter().any(|(line, message)| {
            *line == 8 && message.starts_with("Parameter LOCAL defined more than once")
        }));
        assert_eq!(netlist.params.get("A"), Some(2.0));
        assert_eq!(netlist.params.get("G"), Some(4.0));

        let independent_scopes = Netlist::parse_with_options(
            "independent scopes\n\
             .param VALUE=1\n\
             .subckt child in out\n\
             .param value=2\n\
             R1 in out {value}\n\
             .ends\n\
             .end\n",
            warning_options,
        )
        .expect("child scope may shadow its parent without a warning");
        assert!(
            independent_scopes
                .diagnostics
                .iter()
                .all(|diagnostic| diagnostic.code != "parameter-redefinition")
        );
    }

    #[test]
    fn resolvable_subckt_default_expression_rebinds_after_instance_overrides() {
        let netlist = Netlist::parse(
            "resolvable subckt default expression\n\
             .subckt simple in out PARAMS: par1=2 par2=2 par3='par1*par2*2'\n\
             Rinside in out {par3}\n\
             .ends\n\
             Xtest 1 0 simple par1=2 par2=12\n\
             .end\n",
        )
        .expect("dependent subcircuit default parses");
        assert!(
            netlist.subcircuits[0]
                .expr_params
                .iter()
                .any(|(name, expression)| {
                    name.eq_ignore_ascii_case("par3")
                        && expression.eq_ignore_ascii_case("par1*par2*2")
                }),
            "expression-valued default remains authoritative at instantiation"
        );

        let flattened = flatten_netlist_with_models(&netlist)
            .expect("dependent subcircuit default flattens after overrides");
        let resistance = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Resistor { value, .. }
                    if element.name.eq_ignore_ascii_case("Xtest.Rinside") =>
                {
                    Some(*value)
                }
                _ => None,
            })
            .expect("flattened resistor exists");
        assert_eq!(resistance, 48.0);
    }

    #[test]
    fn subckt_formal_and_body_parameter_precedence_follows_redefinition_policy() {
        let cases = [
            (
                ".subckt simple in out params: value=10\n.param VALUE=20",
                10.0,
                20.0,
            ),
            (
                ".subckt simple in out params: base=5 value='base*2'\n.param value=30",
                10.0,
                30.0,
            ),
            (
                ".subckt simple in out params: base=5 value=10\n.param value='base*3'",
                10.0,
                15.0,
            ),
        ];

        for (declarations, first, last) in cases {
            let source = format!(
                "subckt formal/body precedence\n{declarations}\nRinside in out {{value}}\n.ends\nXtest 1 0 simple\n.end\n"
            );
            for (policy, expected) in [
                (ParameterRedefinitionPolicy::UseFirst, first),
                (ParameterRedefinitionPolicy::UseLast, last),
            ] {
                let netlist = Netlist::parse_with_options(
                    &source,
                    NetlistParseOptions {
                        parameter_redefinition_policy: policy,
                        ..NetlistParseOptions::default()
                    },
                )
                .expect("formal/body precedence deck parses");
                let flattened = flatten_netlist_with_models(&netlist)
                    .expect("formal/body precedence deck flattens");
                let resistance = flattened
                    .elements
                    .iter()
                    .find_map(|element| match &element.kind {
                        ElementKind::Resistor { value, .. } => Some(*value),
                        _ => None,
                    })
                    .expect("flattened resistor exists");
                assert_eq!(resistance, expected, "{declarations} under {policy:?}");
            }
        }
    }

    #[test]
    fn subckt_instance_parameter_override_wins_under_both_redefinition_policies() {
        let source = "subckt instance precedence\n\
             .subckt simple in out params: value=10\n\
             .param value=20\n\
             Rinside in out {value}\n\
             .ends\n\
             Xtest 1 0 simple value=40\n\
             .end\n";
        for policy in [
            ParameterRedefinitionPolicy::UseFirst,
            ParameterRedefinitionPolicy::UseLast,
        ] {
            let netlist = Netlist::parse_with_options(
                source,
                NetlistParseOptions {
                    parameter_redefinition_policy: policy,
                    ..NetlistParseOptions::default()
                },
            )
            .expect("instance precedence deck parses");
            let flattened =
                flatten_netlist_with_models(&netlist).expect("instance precedence deck flattens");
            let resistance = flattened
                .elements
                .iter()
                .find_map(|element| match &element.kind {
                    ElementKind::Resistor { value, .. } => Some(*value),
                    _ => None,
                })
                .expect("flattened resistor exists");
            assert_eq!(resistance, 40.0, "instance override under {policy:?}");
        }
    }

    #[test]
    fn ignored_parameter_redefinition_does_not_advance_statistical_stream() {
        let options = NetlistParseOptions {
            statistical_mode: StatisticalParamMode::Sample,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseFirst,
            ..NetlistParseOptions::default()
        };
        let retained = Netlist::parse_with_options(
            "ignored statistical duplicate\n\
             .param value=gauss(10,0.1,1)\n\
             .param value=gauss(20,0.1,1)\n\
             .param next=gauss(30,0.1,1)\n\
             .end\n",
            options,
        )
        .expect("duplicate statistical deck parses");
        let baseline = Netlist::parse_with_options(
            "statistical baseline\n\
             .param value=gauss(10,0.1,1)\n\
             .param next=gauss(30,0.1,1)\n\
             .end\n",
            options,
        )
        .expect("statistical baseline parses");
        assert_eq!(retained.params.get("value"), baseline.params.get("value"));
        assert_eq!(retained.params.get("next"), baseline.params.get("next"));
    }

    #[test]
    fn nested_subckt_instance_param_sees_later_caller_body_param() {
        let netlist = Netlist::parse(
            "nested subckt instance parameter sees caller body param\n\
             .param res=1\n\
             Xtop 1 0 top\n\
             .subckt top in out\n\
             Xearly in out child params: r={2*res/1000}\n\
             .param res=1k\n\
             .subckt child a b params: r=1\n\
             R1 a b {1000*r}\n\
             .ends child\n\
             .ends top\n\
             .end\n",
        )
        .expect("nested subcircuit body parameter deck parses");

        let flattened = flatten_netlist_with_models(&netlist)
            .expect("nested subcircuit body parameter deck flattens");
        let resistance = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Resistor { value, .. }
                    if element.name.eq_ignore_ascii_case("Xtop.Xearly.R1") =>
                {
                    Some(*value)
                }
                _ => None,
            })
            .expect("flattened nested resistor exists");

        assert_eq!(resistance, 2000.0);
    }

    #[test]
    fn nested_subckt_instance_param_passes_caller_override() {
        let netlist = Netlist::parse(
            "nested subckt instance parameter precedence\n\
             .subckt simple in out PARAMS: par1=2.0 par2=2.0 par3='par1*par2*2.0'\n\
             .param par3=3000.0\n\
             Xtest2 in out simple2 par3='par3'\n\
             .ends\n\
             .subckt simple2 in out PARAMS: par1=2.0 par2=80.0 par3='par1*par2/4.0'\n\
             .param par3=500.0\n\
             Rinside in out 'par3'\n\
             .ends\n\
             V1 1 0 1.0\n\
             R1 1 2 1.0\n\
             Xtest 2 0 simple par1=2.0 par2=3.0 par3='par1+par2'\n\
             .end\n",
        )
        .expect("nested subcircuit instance parameter deck parses");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("nested subcircuit parameters flatten");
        let resistance = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Resistor { value, .. }
                    if element.name.eq_ignore_ascii_case("Xtest.Xtest2.Rinside") =>
                {
                    Some(*value)
                }
                _ => None,
            })
            .expect("flattened nested subcircuit resistor exists");

        assert_eq!(resistance, 5.0);
    }

    #[test]
    fn subckt_behavioral_resistor_value_expr_remaps_voltage_probe() {
        let netlist = Netlist::parse(
            "subckt solution dependent resistor\n\
             .param scalar=2.0\n\
             X1 2 0 soldepres\n\
             .subckt soldepres 1 2\n\
             Vcontrol cntl 2 2.0\n\
             Rcontrol cntl 2 1.0\n\
             R2 1 2 R={1.0+scalar*V(cntl)}\n\
             .ends\n\
             .end\n",
        )
        .expect("solution-dependent resistor subcircuit parses");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("solution-dependent resistor flattens");
        let expression = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Resistor { value_expr, .. }
                    if element.name.eq_ignore_ascii_case("X1.R2") =>
                {
                    value_expr.as_deref()
                }
                _ => None,
            })
            .expect("flattened solution-dependent resistor expression exists");

        assert!(
            expression.to_ascii_lowercase().contains("v(x1.cntl)"),
            "flattened expression should remap local probe, got {expression}"
        );
    }

    #[test]
    fn subckt_body_function_shadows_top_level_function_when_flattened() {
        let netlist = Netlist::parse(
            "subckt body function scope\n\
             .param TheRes=2k\n\
             .func frobnitz(X) {10*X}\n\
             XR1 1 0 ResSub PARAMS: RES={TheRes}\n\
             .subckt ResSub 1 2 PARAMS: RES=5k\n\
             .func frobnitz(x) {5*x}\n\
             R1 1 2 {frobnitz(RES)}\n\
             .ends\n\
             .end\n",
        )
        .expect("subcircuit body function deck parses");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("subcircuit body function flattens");
        let resistance = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Resistor { value, .. } => Some(*value),
                _ => None,
            })
            .expect("flattened resistor exists");

        assert_eq!(resistance, 10_000.0);
    }

    #[test]
    fn subckt_body_function_expands_inside_behavioral_source_when_flattened() {
        let netlist = Netlist::parse(
            "subckt behavioral function scope\n\
             X1 2 3 1 FooCkt\n\
             X2 4 5 1 FooCkt PARAMS: coef=2\n\
             .subckt FooCkt A B CTL PARAMS: coef=1\n\
             .func F1(X) {coef*X*X}\n\
             B1 A 0 V={F1(V(CTL))}\n\
             R1 A B 10k\n\
             R2 B 0 5k\n\
             .ends\n\
             .end\n",
        )
        .expect("subcircuit behavioral function deck parses");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("subcircuit behavioral function flattens");
        let x1_expression = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::BehavioralVoltage { expression, .. } if element.name == "X1.B1" => {
                    Some(expression.as_str())
                }
                _ => None,
            })
            .expect("X1 behavioral source exists");
        let x2_expression = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::BehavioralVoltage { expression, .. } if element.name == "X2.B1" => {
                    Some(expression.as_str())
                }
                _ => None,
            })
            .expect("X2 behavioral source exists");

        assert_eq!(x1_expression, "((1*V(1))*V(1))");
        assert_eq!(x2_expression, "((2*V(1))*V(1))");
    }

    #[test]
    fn subckt_controlled_source_gain_params_resolve_at_flattening() {
        let netlist = Netlist::parse(
            "subckt controlled source parameter gain scope\n\
             X1 PP 0 SP 0 IdealXfmr PARAMS: TurnsRat=2\n\
             .subckt IdealXfmr PP PN SP SN PARAMS: TurnsRat=1\n\
             Es SP SN PP PN {TurnsRat}\n\
             Fp PN PP Es {TurnsRat}\n\
             .ends\n\
             .end\n",
        )
        .expect("subcircuit controlled-source deck parses");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("controlled source deck flattens");
        let es_gain = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Vcvs {
                    gain, gain_expr, ..
                } if element.name == "X1.ES" => {
                    assert!(gain_expr.is_none(), "flattening must resolve E gain");
                    Some(*gain)
                }
                _ => None,
            })
            .expect("flattened VCVS exists");
        let fp_gain = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Cccs {
                    gain,
                    gain_expr,
                    control_element,
                } if element.name == "X1.FP" => {
                    assert!(gain_expr.is_none(), "flattening must resolve F gain");
                    assert_eq!(control_element, "X1.ES");
                    Some(*gain)
                }
                _ => None,
            })
            .expect("flattened CCCS exists");

        assert_eq!(es_gain, 2.0);
        assert_eq!(fp_gain, 2.0);
    }

    #[test]
    fn subckt_switch_controls_remap_to_hierarchical_nodes_when_flattened() {
        let netlist = Netlist::parse(
            "subckt switch control hierarchy scope\n\
             .model SW VSWITCH (RON=1 ROFF=1MEG VON=1 VOFF=0)\n\
             .model CSW ISWITCH (RON=1 ROFF=1MEG ION=1 IOFF=0)\n\
             X1 A Y VCC TOP\n\
             .subckt TOP a out vcc\n\
             XG a 8 out vcc GATE\n\
             .subckt GATE in ctrl out vcc\n\
             VCTRL ctrl 0 0\n\
             S1 out vcc ctrl 0 SW\n\
             W1 out 0 VCTRL CSW\n\
             S2 out 0 SW CONTROL={V(ctrl)+I(VCTRL)}\n\
             .ends GATE\n\
             .ends TOP\n\
             .end\n",
        )
        .expect("subcircuit switch control deck parses");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("switch control deck flattens");

        let (control_pos, control_neg) = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::VSwitch {
                    control_pos,
                    control_neg,
                    ..
                } if element.name == "X1.XG.S1" => {
                    Some((control_pos.as_str(), control_neg.as_str()))
                }
                _ => None,
            })
            .expect("flattened voltage switch exists");
        assert_eq!(control_pos, "X1.8");
        assert_eq!(control_neg, "0");

        let control_element = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::ISwitch {
                    control_element, ..
                } if element.name == "X1.XG.W1" => Some(control_element.as_str()),
                _ => None,
            })
            .expect("flattened current switch exists");
        assert_eq!(control_element, "X1.XG.VCTRL");

        let control_expression = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::GenericSwitch {
                    control_expression, ..
                } if element.name == "X1.XG.S2" => Some(control_expression.as_str()),
                _ => None,
            })
            .expect("flattened generic switch exists");
        assert_eq!(control_expression, "V(X1.8)+I(X1.XG.VCTRL)");
    }

    #[test]
    fn subckt_body_param_expression_resolves_after_instance_override() {
        let netlist = Netlist::parse(
            "subckt body param expression after instance override\n\
             .param res=1\n\
             Xtop 1 0 top\n\
             .subckt top in out\n\
             Xearly in out child params: r={2*res/1000}\n\
             .param res=1k\n\
             .subckt child a b params: r=1\n\
             .param res2={10*r}\n\
             R1 a b {1000*(res2/10)}\n\
             .ends child\n\
             .ends top\n\
             .end\n",
        )
        .expect("subcircuit body expression deck parses");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("subcircuit body expression flattens");
        let resistance = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::Resistor { value, .. }
                    if element.name.eq_ignore_ascii_case("Xtop.Xearly.R1") =>
                {
                    Some(*value)
                }
                _ => None,
            })
            .expect("flattened body-expression resistor exists");

        assert_eq!(resistance, 2000.0);
    }

    #[test]
    fn xyce_special_character_function_names_parse_and_evaluate() {
        let netlist = Netlist::parse(
            "xyce special character function names\n\
             .func afunc(x) {4+x}\n\
             .func _func(x) {4+x}\n\
             .func #func(x) {4+x}\n\
             .func @func(x) {4+x}\n\
             .func `func(x) {4+x}\n\
             .param p1=1\n\
             R2 2 0 {afunc(p1)}\n\
             R3 2 0 {_func(p1)}\n\
             R4 2 0 {#func(p1)}\n\
             R5 2 0 {@func(p1)}\n\
             R6 2 0 {`func(p1)}\n\
             .end\n",
        )
        .expect("Xyce special-character function names should parse");

        for name in ["AFUNC", "_FUNC", "#FUNC", "@FUNC", "`FUNC"] {
            assert!(
                netlist.params.has_function(name),
                "function {name} should be defined"
            );
        }

        let values = netlist
            .elements
            .iter()
            .filter_map(|element| match &element.kind {
                ElementKind::Resistor {
                    value, value_expr, ..
                } => value_expr
                    .as_deref()
                    .map(|expr| crate::netlist::expr::eval_expression(expr, &netlist.params))
                    .or(Some(Ok(*value))),
                _ => None,
            })
            .collect::<Result<Vec<_>, _>>()
            .expect("Xyce special-character function expressions should evaluate");
        assert_eq!(values, vec![5.0; 5]);
    }

    #[test]
    fn subckt_local_diode_model_expression_resolves_per_instance_when_flattened() {
        let netlist = Netlist::parse(
            "subckt local diode model scope\n\
             X1 1 0 DCell is0=100f\n\
             X2 2 0 DCell is0=200f\n\
             .subckt DCell a b is0=1f\n\
             .model DM D (IS={is0})\n\
             D1 a b DM\n\
             .ends\n\
             .end\n",
        )
        .expect("subcircuit local diode model deck parses");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("subcircuit local diode model flattens");
        let diode_model = |element_name: &str| -> &str {
            flattened
                .elements
                .iter()
                .find_map(|element| match &element.kind {
                    ElementKind::Diode { model, .. } if element.name == element_name => {
                        Some(model.as_str())
                    }
                    _ => None,
                })
                .unwrap_or_else(|| panic!("flattened diode {element_name} exists"))
        };

        let x1_model = diode_model("X1.D1");
        let x2_model = diode_model("X2.D1");
        assert_ne!(x1_model, "DCell::DM");
        assert_ne!(x1_model, x2_model);
        assert_eq!(
            scoped_model_param(&flattened.scoped_models, x1_model, "IS"),
            Some(100e-15)
        );
        assert_eq!(
            scoped_model_param(&flattened.scoped_models, x2_model, "IS"),
            Some(200e-15)
        );
        assert!(
            flattened
                .scoped_models
                .iter()
                .all(|model| model.expr_params.is_empty()),
            "native scoped model expressions must be fully resolved"
        );
    }

    #[test]
    fn subckt_local_model_expression_resolves_caller_scope_functions_when_flattened() {
        let netlist = Netlist::parse(
            "subckt local model caller function scope\n\
             .param base_is=100f\n\
             .func twice(x) {x*2}\n\
             X1 1 0 DCell PARAMS: is0={base_is}\n\
             X2 2 0 DCell PARAMS: is0={twice(base_is)}\n\
             .subckt DCell a b PARAMS: is0=1f\n\
             .model DM D (IS={is0})\n\
             D1 a b DM\n\
             .ends\n\
             .end\n",
        )
        .expect("subcircuit local model function deck parses");

        let flattened = flatten_netlist_with_models(&netlist)
            .expect("subcircuit local model function deck flattens");
        let model_for = |element_name: &str| -> &str {
            flattened
                .elements
                .iter()
                .find_map(|element| match &element.kind {
                    ElementKind::Diode { model, .. } if element.name == element_name => {
                        Some(model.as_str())
                    }
                    _ => None,
                })
                .unwrap_or_else(|| panic!("flattened diode {element_name} exists"))
        };

        assert_eq!(
            scoped_model_param(&flattened.scoped_models, model_for("X1.D1"), "IS"),
            Some(100e-15)
        );
        assert_eq!(
            scoped_model_param(&flattened.scoped_models, model_for("X2.D1"), "IS"),
            Some(200e-15)
        );
    }

    #[test]
    fn subckt_instance_param_expression_uses_caller_scope_before_callee_defaults() {
        let netlist = Netlist::parse(
            "subckt local model caller shadow scope\n\
             .param is0=100f\n\
             .func twice(x) {x*2}\n\
             X1 1 0 DCell PARAMS: is0={is0}\n\
             X2 2 0 DCell PARAMS: is0={twice(is0)}\n\
             .subckt DCell a b PARAMS: is0=1\n\
             .model DM D (IS={is0})\n\
             D1 a b DM\n\
             .ends\n\
             .end\n",
        )
        .expect("same-name caller parameter deck parses");

        let flattened = flatten_netlist_with_models(&netlist)
            .expect("same-name caller parameter deck flattens");
        let model_for = |element_name: &str| -> &str {
            flattened
                .elements
                .iter()
                .find_map(|element| match &element.kind {
                    ElementKind::Diode { model, .. } if element.name == element_name => {
                        Some(model.as_str())
                    }
                    _ => None,
                })
                .unwrap_or_else(|| panic!("flattened diode {element_name} exists"))
        };

        assert_eq!(
            scoped_model_param(&flattened.scoped_models, model_for("X1.D1"), "IS"),
            Some(100e-15)
        );
        assert_eq!(
            scoped_model_param(&flattened.scoped_models, model_for("X2.D1"), "IS"),
            Some(200e-15)
        );
    }

    #[test]
    fn subckt_source_value_resolves_against_instance_scope_when_flattened() {
        let netlist = Netlist::parse(
            "subckt source parameter scope\n\
             .param top_current=15\n\
             Xtest 1 0 testsub PARAMS: CURRENT={top_current}\n\
             .subckt testsub a b PARAMS: CURRENT=1\n\
             I1 a b {CURRENT}\n\
             .ends\n\
             .end\n",
        )
        .expect("subcircuit source parameter deck parses");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("subcircuit source parameter flattens");
        let current = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::CurrentSource(SourceSpec::Dc(value)) if element.name == "Xtest.I1" => {
                    Some(*value)
                }
                _ => None,
            })
            .expect("flattened current source exists");

        assert_eq!(current, 15.0);
    }

    #[test]
    fn subckt_transient_source_values_resolve_against_instance_scope_when_flattened() {
        let netlist = Netlist::parse(
            "subckt transient source parameter scope\n\
             Xtest 1 0 testsub PARAMS: AMP=3\n\
             .subckt testsub a b PARAMS: AMP=1\n\
             V1 a b PULSE(0 {AMP} 0 1n 1n 1u 2u)\n\
             .ends\n\
             .end\n",
        )
        .expect("subcircuit transient source parameter deck parses");

        let flattened =
            flatten_netlist_with_models(&netlist).expect("subcircuit transient source flattens");
        let pulse_high = flattened
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::VoltageSource(SourceSpec::Pulse { v2, .. })
                    if element.name == "Xtest.V1" =>
                {
                    Some(*v2)
                }
                _ => None,
            })
            .expect("flattened pulse source exists");

        assert_eq!(pulse_high, 3.0);
    }

    #[test]
    fn top_level_source_values_resolve_after_later_params() {
        let netlist = Netlist::parse(
            "top-level source parameter order\n\
             V1 in 0 PULSE(0 {V_HI} 0 1n 1n 1u 2u)\n\
             I1 out 0 {I_BIAS}\n\
             .param I_BIAS=25u V_HI=3\n\
             .end\n",
        )
        .expect("top-level source values should resolve after later .param cards");

        let pulse_high = netlist
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::VoltageSource(SourceSpec::Pulse { v2, .. })
                    if element.name.eq_ignore_ascii_case("V1") =>
                {
                    Some(*v2)
                }
                _ => None,
            })
            .expect("pulse source exists");
        let current = netlist
            .elements
            .iter()
            .find_map(|element| match &element.kind {
                ElementKind::CurrentSource(SourceSpec::Dc(value))
                    if element.name.eq_ignore_ascii_case("I1") =>
                {
                    Some(*value)
                }
                _ => None,
            })
            .expect("current source exists");

        assert_eq!(pulse_high, 3.0);
        assert!((current - 25e-6).abs() < 1e-18);
    }

    #[test]
    fn braced_independent_source_expressions_lower_to_behavioral_sources() {
        let netlist = Netlist::parse(
            "independent source expression\n\
             .global_param offset=2\n\
             .func shifted(x) {x+offset}\n\
             V1 out 0 {shifted(1)+TIME}\n\
             I1 load 0 {shifted(2)+TIME}\n\
             .tran 1n 1u\n\
             .end\n",
        )
        .expect("runtime independent-source expressions parse");

        assert!(netlist.elements.iter().any(|element| {
            element.name == "V1"
                && matches!(
                    &element.kind,
                    ElementKind::BehavioralVoltage { expression, .. }
                        if expression.eq_ignore_ascii_case("shifted(1)+TIME")
                )
        }));
        assert!(netlist.elements.iter().any(|element| {
            element.name == "I1"
                && matches!(
                    &element.kind,
                    ElementKind::BehavioralCurrent { expression, .. }
                        if expression.eq_ignore_ascii_case("shifted(2)+TIME")
                )
        }));

        let subcircuit = Netlist::parse(
            "subcircuit source expression\n\
             x1 out load source_cell gain=3\n\
             .subckt source_cell p n gain=1\n\
             V1 p 0 {gain+TIME}\n\
             I1 n 0 {gain+2*TIME}\n\
             .ends\n\
             .tran 1n 1u\n\
             .end\n",
        )
        .expect("subcircuit runtime independent-source expressions parse");
        let flattened = flatten_netlist(&subcircuit).expect("source subcircuit flattens");
        assert!(
            flattened.iter().any(|element| {
                element.name.eq_ignore_ascii_case("X1.V1")
                    && matches!(&element.kind, ElementKind::BehavioralVoltage { .. })
            }),
            "{flattened:#?}"
        );
        assert!(
            flattened.iter().any(|element| {
                element.name.eq_ignore_ascii_case("X1.I1")
                    && matches!(&element.kind, ElementKind::BehavioralCurrent { .. })
            }),
            "{flattened:#?}"
        );
    }

    #[test]
    fn passive_unit_words_after_numeric_values_are_consumed() {
        let netlist = Netlist::parse(
            "passive unit words\n\
             R1 1 0 1.019524e+9Ohms\n\
             L1 1 0 0.05H\n\
             .end\n",
        )
        .expect("passive unit words should parse after numeric values");

        let resistance = netlist
            .elements
            .iter()
            .find_map(|e| match &e.kind {
                ElementKind::Resistor { value, .. } => Some(*value),
                _ => None,
            })
            .expect("resistor exists");
        let inductance = netlist
            .elements
            .iter()
            .find_map(|e| match &e.kind {
                ElementKind::Inductor { value, .. } => Some(*value),
                _ => None,
            })
            .expect("inductor exists");

        assert!((resistance - 1.019524e9).abs() < 1.0);
        assert!((inductance - 0.05).abs() < 1e-15);
    }

    #[test]
    fn malformed_ac_source_terms_are_rejected_not_defaulted() {
        for prefix in ["V1 out 0", "I1 out 0"] {
            let err = Netlist::parse(&format!(
                "bad ac\n\
                 {prefix} AC {{missing_gain}}\n\
                 R1 out 0 1k\n\
                 .ac lin 1 1 1\n\
                 .end\n"
            ))
            .expect_err("malformed AC magnitude must fail");

            let message = err.to_string();
            assert!(
                message.contains("missing_gain") || message.contains("MISSING_GAIN"),
                "unexpected error for {prefix}: {message}"
            );
        }
    }

    #[test]
    fn source_specs_reject_unconsumed_trailing_tokens() {
        for prefix in ["V1 out 0", "I1 out 0"] {
            let err = Netlist::parse(&format!(
                "bad source tail\n\
                 {prefix} DC 5 garbage\n\
                 R1 out 0 1k\n\
                 .op\n\
                 .end\n"
            ))
            .expect_err("source cards must reject unconsumed trailing tokens");

            let message = err.to_string();
            assert!(
                message.contains("garbage") || message.contains("GARBAGE"),
                "unexpected error for {prefix}: {message}"
            );
        }
    }

    #[test]
    fn passive_tails_reject_unconsumed_trailing_tokens() {
        for line in [
            "R1 out 0 1k garbage extra",
            "C1 out 0 1p garbage extra",
            "L1 out 0 1n garbage extra",
        ] {
            let err = Netlist::parse(&format!(
                "bad passive tail\n\
                 {line}\n\
                 .op\n\
                 .end\n"
            ))
            .expect_err("passive cards must reject unconsumed trailing tokens");

            let message = err.to_string();
            assert!(
                message.contains("garbage")
                    || message.contains("GARBAGE")
                    || message.contains("extra")
                    || message.contains("EXTRA"),
                "unexpected error for {line}: {message}"
            );
        }
    }

    #[test]
    fn transient_sources_reject_malformed_or_unpaired_arguments() {
        let pulse = Netlist::parse(
            "bad pulse\n\
             V1 out 0 PULSE(0 1 bogus 1n)\n\
             R1 out 0 1k\n\
             .tran 1n 10n\n\
             .end\n",
        )
        .expect_err("malformed PULSE argument must fail");
        let pulse_message = pulse.to_string();
        let pulse_lowered = pulse_message.to_ascii_lowercase();
        assert!(
            pulse_lowered.contains("pulse") && pulse_lowered.contains("bogus"),
            "unexpected error: {pulse_message}"
        );

        let odd_pwl = Netlist::parse(
            "odd pwl\n\
             V1 out 0 PWL(0 0 1m)\n\
             R1 out 0 1k\n\
             .tran 1n 10n\n\
             .end\n",
        )
        .expect_err("unpaired PWL time/value token must fail");
        let pwl_message = odd_pwl.to_string();
        assert!(
            pwl_message.contains("PWL") && pwl_message.contains("time/value"),
            "unexpected error: {pwl_message}"
        );
    }

    #[test]
    fn pwl_sources_accept_grouped_time_value_pairs() {
        let netlist = Netlist::parse(
            "grouped pwl pairs\n\
             V1 out 0 DC PWL( (0 0.0) (1m 1.0) )\n\
             R1 out 0 1k\n\
             .tran 1u 1m\n\
             .end\n",
        )
        .expect("grouped PWL time/value pairs parse");

        match first_source_spec(&netlist) {
            SourceSpec::DcTransient {
                dc_value,
                transient,
            } => {
                assert_eq!(*dc_value, 0.0);
                match transient.as_ref() {
                    SourceSpec::Pwl {
                        points,
                        delay,
                        repeat_from,
                    } => {
                        assert_eq!(points, &[(0.0, 0.0), (1e-3, 1.0)]);
                        assert_eq!(*delay, 0.0);
                        assert_eq!(*repeat_from, None);
                    }
                    other => panic!("expected PWL transient, got {other:?}"),
                }
            }
            other => panic!("expected DC transient source, got {other:?}"),
        }
    }

    #[test]
    fn pwl_sources_accept_xyce_delay_and_repeat_options() {
        let netlist = Netlist::parse(
            "xyce pwl timing options\n\
             V1 out 0 PWL 0 0 1 2 2 0 R=1 TD=3\n\
             R1 out 0 1k\n\
             .tran 1n 10n\n\
             .end\n",
        )
        .expect("Xyce PWL TD/R options parse");

        match first_source_spec(&netlist) {
            SourceSpec::Pwl {
                points,
                delay,
                repeat_from,
            } => {
                assert_eq!(points, &[(0.0, 0.0), (1.0, 2.0), (2.0, 0.0)]);
                assert_eq!(*delay, 3.0);
                assert_eq!(*repeat_from, Some(1.0));
            }
            other => panic!("expected PWL source, got {other:?}"),
        }
    }

    #[test]
    fn remaining_transient_sources_reject_malformed_arguments() {
        for (source, deck) in [
            (
                "SIN",
                "bad sin\n\
                 V1 out 0 SIN(0 1 bogus)\n\
                 R1 out 0 1k\n\
                 .tran 1n 10n\n\
                 .end\n",
            ),
            (
                "EXP",
                "bad exp\n\
                 V1 out 0 EXP(0 1 bogus)\n\
                 R1 out 0 1k\n\
                 .tran 1n 10n\n\
                 .end\n",
            ),
            (
                "SFFM",
                "bad sffm\n\
                 V1 out 0 SFFM(0 1 bogus)\n\
                 R1 out 0 1k\n\
                 .tran 1n 10n\n\
                 .end\n",
            ),
            (
                "AM",
                "bad am\n\
                 V1 out 0 AM(0 0 1 bogus)\n\
                 R1 out 0 1k\n\
                 .tran 1n 10n\n\
                 .end\n",
            ),
            (
                "TRNOISE",
                "bad trnoise\n\
                 V1 out 0 TRNOISE(1 bogus)\n\
                 R1 out 0 1k\n\
                 .tran 1n 10n\n\
                 .end\n",
            ),
            (
                "TRRANDOM",
                "bad trrandom\n\
                 V1 out 0 TRRANDOM(9 1n)\n\
                 R1 out 0 1k\n\
                 .tran 1n 10n\n\
                 .end\n",
            ),
        ] {
            let err =
                Netlist::parse(deck).expect_err(&format!("malformed {source} argument must fail"));
            let message = err.to_string().to_ascii_lowercase();
            assert!(
                message.contains(&source.to_ascii_lowercase()) || message.contains("expected ')'"),
                "unexpected error for {source}: {message}"
            );
        }
    }

    #[test]
    fn trrandom_and_trnoise_rts_sources_parse_with_ngspice_parameters() {
        let netlist = Netlist::parse(
            "transient random sources\n\
             V1 a 0 DC 0 TRRANDOM(2 1u 2u 3 4)\n\
             I1 b 0 DC 0 TRNOISE(0 0 0 0 5m 18u 30u)\n\
             R1 a 0 1k\n\
             R2 b 0 1k\n\
             .tran 1u 10u\n\
             .end\n",
        )
        .expect("TRRANDOM and RTS TRNOISE parse");

        assert!(netlist.elements.iter().any(|element| matches!(
            &element.kind,
            ElementKind::VoltageSource(SourceSpec::DcTransient { transient, .. })
                if matches!(transient.as_ref(), SourceSpec::TrRandom {
                    distribution: 2,
                    sample_interval,
                    delay,
                    parameter1,
                    parameter2,
                } if (*sample_interval - 1e-6).abs() < 1e-18
                    && (*delay - 2e-6).abs() < 1e-18
                    && *parameter1 == 3.0
                    && *parameter2 == 4.0)
        )));
        assert!(
            netlist.elements.iter().any(|element| matches!(
                &element.kind,
                ElementKind::CurrentSource(SourceSpec::DcTransient { transient, .. })
                    if matches!(transient.as_ref(), SourceSpec::TrNoise {
                        rts_amplitude,
                        rts_capture,
                        rts_emit,
                        ..
                    } if (*rts_amplitude - 5e-3).abs() < 1e-15
                        && (*rts_capture - 18e-6).abs() < 1e-18
                        && (*rts_emit - 30e-6).abs() < 1e-18)
            )),
            "parsed elements: {:?}",
            netlist.elements
        );
    }

    #[test]
    fn pwl_file_options_parse_commas_and_reject_malformed_values() {
        let netlist = Netlist::parse(
            "pwl file options\n\
             V1 out 0 PWL(FILE=\"stim.csv\", TSCALE=1m, VSCALE=2, TOFFSET=3n, VOFFSET=-1)\n\
             R1 out 0 1k\n\
             .tran 1n 10n\n\
             .end\n",
        )
        .expect("PWL FILE options parse");

        match first_source_spec(&netlist) {
            SourceSpec::PwlFile {
                path,
                time_scale,
                value_scale,
                time_offset,
                value_offset,
                delay,
                repeat_from,
            } => {
                assert_eq!(path, "stim.csv");
                assert!((*time_scale - 1e-3).abs() < 1e-15);
                assert!((*value_scale - 2.0).abs() < f64::EPSILON);
                assert!((*time_offset - 3e-9).abs() < 1e-18);
                assert!((*value_offset + 1.0).abs() < f64::EPSILON);
                assert_eq!(*delay, 0.0);
                assert_eq!(*repeat_from, None);
            }
            other => panic!("expected PWL FILE source, got {other:?}"),
        }

        let err = Netlist::parse(
            "bad pwl file options\n\
             V1 out 0 PWL(FILE=\"stim.csv\" TSCALE=bogus)\n\
             R1 out 0 1k\n\
             .tran 1n 10n\n\
             .end\n",
        )
        .expect_err("malformed PWL FILE option must fail");
        let message = err.to_string().to_ascii_lowercase();
        assert!(
            message.contains("pwl file") && message.contains("tscale"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn pwl_file_options_accept_xyce_delay_and_repeat() {
        let netlist = Netlist::parse(
            "pwl file xyce timing options\n\
             V1 out 0 PWL FILE \"stim.csv\" TD=3 R=1 TOFFSET=2\n\
             R1 out 0 1k\n\
             .tran 1n 10n\n\
             .end\n",
        )
        .expect("PWL FILE TD/R options parse");

        match first_source_spec(&netlist) {
            SourceSpec::PwlFile {
                path,
                time_offset,
                delay,
                repeat_from,
                ..
            } => {
                assert_eq!(path, "stim.csv");
                assert_eq!(*time_offset, 2.0);
                assert_eq!(*delay, 3.0);
                assert_eq!(*repeat_from, Some(1.0));
            }
            other => panic!("expected PWL FILE source, got {other:?}"),
        }
    }

    #[test]
    fn pwl_file_options_accept_unquoted_relative_paths() {
        let netlist = Netlist::parse(
            "pwl file unquoted relative path\n\
             V1 out 0 PWL FILE ./stim.csv TD=3 R=1\n\
             R1 out 0 1k\n\
             .tran 1n 10n\n\
             .end\n",
        )
        .expect("PWL FILE unquoted relative path parses");

        match first_source_spec(&netlist) {
            SourceSpec::PwlFile {
                path,
                delay,
                repeat_from,
                ..
            } => {
                assert_eq!(path, "./stim.csv");
                assert_eq!(*delay, 3.0);
                assert_eq!(*repeat_from, Some(1.0));
            }
            other => panic!("expected PWL FILE source, got {other:?}"),
        }
    }

    #[test]
    fn pwl_file_paths_resolve_relative_to_deck_path() {
        let deck_path = std::env::temp_dir()
            .join("rspice-pwl-file-path")
            .join("deck.cir");
        let deck_dir = deck_path.parent().expect("temp deck has parent");
        std::fs::create_dir_all(deck_dir).expect("create temp deck dir");
        let netlist = Netlist::parse_with_path(
            "pwl file relative path\n\
             V1 out 0 PWL FILE \"stim.csv\"\n\
             R1 out 0 1k\n\
             .tran 1n 10n\n\
             .end\n",
            &deck_path,
        )
        .expect("PWL FILE deck parses with path");

        match first_source_spec(&netlist) {
            SourceSpec::PwlFile { path, .. } => {
                assert_eq!(std::path::Path::new(path), deck_dir.join("stim.csv"));
            }
            other => panic!("expected PWL FILE source, got {other:?}"),
        }
    }

    #[test]
    fn transient_source_arguments_reject_explicit_non_finite_values() {
        for (source, deck) in [
            (
                "SIN",
                "bad sin overflow\n\
                 V1 out 0 SIN(0 1 1e309)\n\
                 R1 out 0 1k\n\
                 .tran 1n 10n\n\
                 .end\n",
            ),
            (
                "PULSE",
                "bad pulse overflow\n\
                 V1 out 0 PULSE(0 1 0 1n 1n 5n 1e309)\n\
                 R1 out 0 1k\n\
                 .tran 1n 10n\n\
                 .end\n",
            ),
            (
                "SFFM",
                "bad sffm overflow\n\
                 V1 out 0 SFFM(0 1 1e309)\n\
                 R1 out 0 1k\n\
                 .tran 1n 10n\n\
                 .end\n",
            ),
            (
                "AM",
                "bad am overflow\n\
                 V1 out 0 AM(0 0 1 1k 1e309)\n\
                 R1 out 0 1k\n\
                 .tran 1n 10n\n\
                 .end\n",
            ),
            (
                "EXP",
                "bad exp overflow\n\
                 V1 out 0 EXP(0 1 1n 1e309)\n\
                 R1 out 0 1k\n\
                 .tran 1n 10n\n\
                 .end\n",
            ),
        ] {
            let err = Netlist::parse(deck).expect_err("non-finite source parameter must fail");
            let message = err.to_string().to_ascii_lowercase();
            assert!(
                message.contains(&source.to_ascii_lowercase()) && message.contains("finite"),
                "unexpected error for {source}: {message}"
            );
        }
    }

    #[test]
    fn dc_and_ac_source_terms_reject_explicit_non_finite_values() {
        for (label, deck) in [
            (
                "bare dc",
                "bad bare dc\n\
                 V1 out 0 1e309\n\
                 R1 out 0 1k\n\
                 .op\n\
                 .end\n",
            ),
            (
                "dc keyword",
                "bad dc keyword\n\
                 V1 out 0 DC 1e309\n\
                 R1 out 0 1k\n\
                 .op\n\
                 .end\n",
            ),
            (
                "ac magnitude",
                "bad ac magnitude\n\
                 V1 out 0 AC 1e309\n\
                 R1 out 0 1k\n\
                 .ac lin 1 1 1\n\
                 .end\n",
            ),
            (
                "distortion magnitude",
                "bad distortion magnitude\n\
                 V1 out 0 DISTOF1 1e309\n\
                 R1 out 0 1k\n\
                 .op\n\
                 .end\n",
            ),
        ] {
            let err = Netlist::parse(deck).expect_err("non-finite source term must fail");
            let message = err.to_string().to_ascii_lowercase();
            assert!(
                message.contains("finite"),
                "unexpected error for {label}: {message}"
            );
        }
    }

    #[test]
    fn pwl_file_options_reject_non_finite_or_non_positive_scaling() {
        for (label, option) in [
            ("zero tscale", "TSCALE=0"),
            ("infinite tscale", "TSCALE=1e309"),
            ("infinite vscale", "VSCALE=1e309"),
            ("infinite toffset", "TOFFSET=1e309"),
            ("infinite voffset", "VOFFSET=1e309"),
        ] {
            let err = Netlist::parse(&format!(
                "bad pwl file {label}\n\
                 V1 out 0 PWL(FILE=\"stim.csv\" {option})\n\
                 R1 out 0 1k\n\
                 .tran 1n 10n\n\
                 .end\n"
            ))
            .expect_err("invalid PWL FILE scaling must fail");
            let message = err.to_string().to_ascii_lowercase();
            assert!(
                message.contains("pwl file") && message.contains("finite")
                    || message.contains("pwl file") && message.contains("positive"),
                "unexpected error for {label}: {message}"
            );
        }
    }

    #[test]
    fn source_specs_reject_nonfinite_explicit_parameters() {
        for source in [
            "SIN(0 1 1e309)",
            "PULSE(0 1 0 1n 1n 5n 1e309)",
            "SFFM(0 1 1e309)",
            "AM(0 0 1 1 1e309)",
            "EXP(0 1 0 1e309)",
        ] {
            let deck = format!(
                "bad source\n\
                 V1 1 0 {source}\n\
                 R1 1 0 1k\n\
                 .end\n"
            );
            let message = Netlist::parse(&deck)
                .expect_err("non-finite explicit source value must be rejected")
                .to_string();

            assert!(
                message.contains("finite"),
                "{source} should report a finite-value error, got: {message}"
            );
        }
    }

    #[test]
    fn pwl_file_specs_reject_invalid_scaling_parameters() {
        for option in [
            "TSCALE=0",
            "TSCALE=1e309",
            "VSCALE=1e309",
            "TOFFSET=1e309",
            "VOFFSET=1e309",
        ] {
            let deck = format!(
                "bad pwl file\n\
                 V1 1 0 PWL FILE=\"wave.csv\" {option}\n\
                 R1 1 0 1k\n\
                 .end\n"
            );
            let message = Netlist::parse(&deck)
                .expect_err("invalid PWL FILE scaling must be rejected")
                .to_string();

            assert!(
                message.contains("PWL") && message.contains("finite")
                    || message.contains("positive"),
                "{option} should report an invalid PWL scaling error, got: {message}"
            );
        }
    }

    #[test]
    fn xyce_nonlinear_continuation_aliases_and_selectors_are_typed() {
        use crate::config::NonlinearContinuationMode as Mode;

        for (value, expected) in [
            ("standard", Mode::Standard),
            ("natural", Mode::Natural),
            ("mos", Mode::Mosfet),
            ("gmin", Mode::Gmin),
            ("pseudo", Mode::PseudoTransient),
            ("sourcestep", Mode::SimultaneousSourceStep),
            ("sourcestep2", Mode::SequentialSourceStep),
            ("0", Mode::Standard),
            ("1", Mode::Natural),
            ("2", Mode::Mosfet),
            ("3", Mode::Gmin),
            ("9", Mode::PseudoTransient),
            ("34", Mode::SimultaneousSourceStep),
            ("35", Mode::SequentialSourceStep),
        ] {
            let deck = format!("typed continuation\n.options nonlin continuation={value}\n.end\n");
            let netlist = Netlist::parse(&deck).expect("continuation mode should parse");
            assert_eq!(netlist.options.nonlinear_continuation, Some(expected));
            assert_eq!(
                Mode::from_xyce_selector(expected.xyce_selector()),
                Some(expected)
            );
        }
    }

    #[test]
    fn xyce_nonlinear_continuation_rejects_unknown_modes() {
        for value in ["arbitrary", "4", "33", "1.5"] {
            let deck = format!("bad continuation\n.options nonlin continuation={value}\n.end\n");
            let message = Netlist::parse(&deck)
                .expect_err("unknown continuation mode must fail closed")
                .to_string();
            assert!(message.contains("CONTINUATION"), "{message}");
        }
    }

    #[test]
    fn legacy_xyce_y_gate_ic_formal_resolves_per_subcircuit_instance() {
        let netlist = Netlist::parse(
            "legacy Y gate parameterized initial conditions\n\
             .subckt latch in_a in_b out params: valx=0\n\
             YNAND N1 in_a in_b out DMOD IC=valx\n\
             .ends latch\n\
             Xone a b q1 latch params: valx=1\n\
             Xzero a b q0 latch params: valx=0\n\
             Xtrue a b qt latch params: valx=true\n\
             Xfalse a b qf latch params: valx=false\n\
             .model DMOD DIG (DELAY=20ns)\n\
             .end\n",
        )
        .expect("parameterized legacy Y gates parse");

        let ElementKind::Xspice {
            params,
            expr_params,
            ..
        } = &netlist.subcircuits[0].elements[0].kind
        else {
            panic!("legacy Y gate should lower to an XSPICE element");
        };
        assert!(
            params
                .iter()
                .all(|(name, _)| !name.eq_ignore_ascii_case("ic")),
            "the subcircuit definition default must not be captured as the gate IC"
        );
        assert_eq!(expr_params, &[("ic".to_string(), "VALX".to_string())]);

        let flattened = flatten_netlist_with_models(&netlist)
            .expect("legacy Y gate IC formals resolve while flattening");
        let mut initial_states = flattened
            .elements
            .iter()
            .filter_map(|element| {
                let ElementKind::Xspice {
                    model,
                    params,
                    expr_params,
                    ..
                } = &element.kind
                else {
                    return None;
                };
                if !model.eq_ignore_ascii_case("xyce_legacy_d_nand") {
                    return None;
                }
                assert!(
                    expr_params.is_empty(),
                    "flattening must consume deferred ICs"
                );
                let ic = params
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case("ic"))
                    .map(|(_, value)| *value)
                    .expect("flattened legacy gate has a resolved IC");
                Some((element.name.to_ascii_uppercase(), ic))
            })
            .collect::<Vec<_>>();
        initial_states.sort_by(|left, right| left.0.cmp(&right.0));

        assert_eq!(
            initial_states,
            vec![
                ("XFALSE.N1".to_string(), 0.0),
                ("XONE.N1".to_string(), 1.0),
                ("XTRUE.N1".to_string(), 1.0),
                ("XZERO.N1".to_string(), 0.0),
            ]
        );
    }
}

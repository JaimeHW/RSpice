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

mod ast;
pub mod expr;
mod flattener;
pub mod hierarchy_path;
pub mod include;
pub mod lexer;
pub mod multi_run;
pub mod param_scope;
mod parser;
pub mod source_map;
pub mod spef;
mod topology;
mod xspice_parser;

pub use ast::*;
pub use expr::{ExpressionDialect, ParamContext, RandomState, StatisticalParamMode};
pub use flattener::{
    FlattenedNetlist, Flattener, FlattenerConfig, InstanceMetadata, XspiceAutoBridgeNodeHint,
    flatten_netlist, flatten_netlist_with_models,
};
pub use hierarchy_path::{HierarchyPath, HierarchyPathConfig};
pub use include::{IncludeProcessor, parse_include_directive, parse_lib_directive};
pub use param_scope::{ParamResolver, ParamScope, ScopedParam};
pub use parser::*;
pub use source_map::*;
pub use topology::{
    TopologyReduction, XYCE_DEFAULT_ZERO_RESISTANCE_TOL, reduce_supernode_topology,
};
pub(crate) use xspice_parser::{
    DeferredXspiceStringVectorEntry, encode_deferred_xspice_complex,
    encode_deferred_xspice_complex_vector, parse_deferred_xspice_complex,
    parse_deferred_xspice_complex_vector, parse_xspice_string_vector_literal,
    xspice_model_param_accepts_bare_string, xspice_param_prefers_string_vector,
    xspice_param_preserves_numeric_string,
};

use thiserror::Error;

/// Errors that can occur during netlist parsing
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Syntax error at line {line}: {message}")]
    Syntax { line: usize, message: String },

    #[error("Unknown device type: {0}")]
    UnknownDevice(String),

    #[error("Invalid node reference: {0}")]
    InvalidNode(String),

    #[error("Duplicate element name: {0}")]
    DuplicateName(String),

    #[error("Missing required parameter: {0}")]
    MissingParameter(String),

    #[error("Invalid value: {0}")]
    InvalidValue(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

use crate::analysis::MeasureStatement;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// Represents a parsed netlist ready for circuit construction
#[derive(Debug, Clone)]
pub struct Netlist {
    /// Circuit title (first line of netlist)
    pub title: String,
    /// All circuit elements
    pub elements: Vec<Element>,
    /// Analysis commands
    pub analyses: Vec<AnalysisCommand>,
    /// Named `.DATA` tables retained for table-driven analyses such as
    /// `.STEP DATA=<name>`.
    pub data_tables: Vec<DataTable>,
    /// Model definitions
    pub models: Vec<ModelDef>,
    /// Subcircuit definitions
    pub subcircuits: Vec<SubcircuitDef>,
    /// Parameter definitions from .PARAM statements
    pub params: ParamContext,
    /// Initial conditions from .IC statements
    pub initial_conditions: Vec<InitialCondition>,
    /// Operating-point node voltage hints from .NODESET statements
    pub node_sets: Vec<NodeSet>,
    /// Global nodes from .GLOBAL (not renamed in subcircuits)
    pub global_nodes: HashSet<String>,
    /// Measurement statements from .MEAS commands
    pub measurements: Vec<MeasureStatement>,
    /// Output selection from .SAVE/.PROBE/.PRINT/.PLOT commands
    pub saves: SaveSet,
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
    /// Optional original netlist text used to build this AST.
    /// Stored to support parameter re-application workflows (e.g., sensitivity).
    pub source_text: Option<String>,
    /// Optional source path for the netlist used to resolve relative includes
    /// and model-file references during reparsing workflows.
    pub source_path: Option<PathBuf>,
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
            code: code.into(),
            message: message.into(),
            severity: DiagnosticSeverity::Warning,
        }
    }
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
    /// Parse a netlist from a string
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        Self::parse_with_options(input, NetlistParseOptions::default())
    }

    /// Parse a netlist from a string with explicit parser options.
    pub fn parse_with_options(
        input: &str,
        options: NetlistParseOptions,
    ) -> Result<Self, ParseError> {
        let promoted_input = Self::promote_control_analysis_commands(input);
        let (sanitized, mut diagnostics) =
            Self::strip_control_blocks_with_diagnostics(&promoted_input)?;
        let mut netlist = parser::parse_netlist_with_options(&sanitized, options)?;
        diagnostics.extend(netlist.diagnostics);
        netlist.diagnostics = diagnostics;
        netlist.source_text = Some(input.to_string());
        netlist.source_path = None;
        Ok(netlist)
    }

    /// Parse a netlist from a string with include resolution
    ///
    /// This method preprocesses .include and .lib directives using the specified
    /// file path to resolve relative paths.
    pub fn parse_with_path(input: &str, file_path: &std::path::Path) -> Result<Self, ParseError> {
        Self::parse_with_path_and_options(input, file_path, NetlistParseOptions::default())
    }

    /// Parse a netlist from a string with include resolution and parser options.
    pub fn parse_with_path_and_options(
        input: &str,
        file_path: &std::path::Path,
        options: NetlistParseOptions,
    ) -> Result<Self, ParseError> {
        let processed = Self::preprocess_includes(input, file_path)?;
        let mut netlist = Self::parse_with_options(&processed, options)?;
        Self::normalize_model_string_paths(&mut netlist, file_path);
        Self::normalize_source_file_paths(&mut netlist, file_path);
        Self::apply_spef_includes(&mut netlist, file_path)?;
        netlist.source_text = Some(input.to_string());
        netlist.source_path = Some(file_path.to_path_buf());
        Ok(netlist)
    }

    /// Back-annotate every `.spef_include` referenced by the deck
    /// (paths resolve relative to the deck file).
    fn apply_spef_includes(
        netlist: &mut Netlist,
        deck_path: &std::path::Path,
    ) -> Result<(), ParseError> {
        if netlist.spef_includes.is_empty() {
            return Ok(());
        }
        let base = deck_path
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or_else(|| std::path::Path::new("."));
        for entry in netlist.spef_includes.clone() {
            let candidate = std::path::Path::new(&entry);
            let path = if candidate.is_absolute() {
                candidate.to_path_buf()
            } else {
                base.join(candidate)
            };
            let content = read_file_with_encoding(&path).map_err(|e| ParseError::Syntax {
                line: 0,
                message: format!("failed to read SPEF file `{}`: {e}", path.display()),
            })?;
            let parasitics = spef::SpefFile::parse(&content)?;
            let report = parasitics.apply(netlist);
            log::info!(
                "SPEF `{}`: {} net(s), {} pin(s) rewired ({} skipped), {} R + {} C added",
                path.display(),
                report.nets,
                report.rewired_pins,
                report.skipped_pins,
                report.resistors,
                report.capacitors
            );
        }
        Ok(())
    }

    /// Parse a netlist from a file with include expansion
    pub fn parse_file(path: &std::path::Path) -> Result<Self, ParseError> {
        let content = read_file_with_encoding(path)?;
        Self::parse_with_path(&content, path)
    }

    /// Read a deck file with the same encoding handling `parse_file` uses
    /// (UTF-8 with fallbacks), without parsing — for callers that
    /// preprocess the text first (multi-run expansion).
    pub fn read_source(path: &std::path::Path) -> Result<String, ParseError> {
        Ok(read_file_with_encoding(path)?)
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
        let content = read_file_with_encoding(path)?;
        Self::parse_with_search_paths(&content, path, search_paths)
    }

    /// Parse netlist source text as if it lived at `path`, with additional
    /// include search directories — the source-text twin of
    /// [`Netlist::parse_file_with_search_paths`], for callers that rewrite
    /// the deck before parsing (multi-run expansion, parameter overrides).
    pub fn parse_with_search_paths(
        input: &str,
        path: &std::path::Path,
        search_paths: &[std::path::PathBuf],
    ) -> Result<Self, ParseError> {
        let mut processor = IncludeProcessor::new(path);
        for dir in search_paths {
            processor.add_lib_path(dir.clone());
        }
        let processed = processor.expand_content(input, path)?;
        let mut netlist = Self::parse(&processed)?;
        Self::normalize_model_string_paths(&mut netlist, path);
        Self::normalize_source_file_paths(&mut netlist, path);
        Self::apply_spef_includes(&mut netlist, path)?;
        netlist.source_text = Some(input.to_string());
        netlist.source_path = Some(path.to_path_buf());
        Ok(netlist)
    }

    /// Preprocess netlist content to expand .include and .lib directives
    ///
    /// This method expands all .include and .lib directives in the content,
    /// resolving paths relative to the given file_path.
    pub fn preprocess_includes(
        content: &str,
        file_path: &std::path::Path,
    ) -> Result<String, ParseError> {
        let mut processor = IncludeProcessor::new(file_path);
        processor.expand_content(content, file_path)
    }

    /// Strip .control/.endc blocks from netlist
    ///
    /// Ngspice uses .control blocks for scripting (variable assignment, loops,
    /// conditionals). These contain operators like '>' that break the netlist
    /// parser. We strip them since RSpice runs the circuit directly.
    pub fn strip_control_blocks(input: &str) -> Result<String, ParseError> {
        Ok(Self::strip_control_blocks_with_diagnostics(input)?.0)
    }

    fn promote_control_analysis_commands(input: &str) -> String {
        let promoted = Self::collect_control_promoted_commands(input);
        if promoted.is_empty() {
            return input.to_string();
        }

        let mut result = String::with_capacity(
            input.len()
                + promoted
                    .iter()
                    .map(|command| command.len() + 1)
                    .sum::<usize>(),
        );
        let mut in_control = false;
        let mut inserted = false;

        for line in input.lines() {
            let trimmed = line.trim();
            let head = trimmed.split_whitespace().next().unwrap_or("");

            if head.eq_ignore_ascii_case(".control") {
                in_control = true;
            } else if head.eq_ignore_ascii_case(".endc") {
                in_control = false;
            }

            if !inserted && !in_control && head.eq_ignore_ascii_case(".end") {
                for command in &promoted {
                    result.push_str(command);
                    result.push('\n');
                }
                inserted = true;
            }

            result.push_str(line);
            result.push('\n');
        }

        if !inserted {
            for command in &promoted {
                result.push_str(command);
                result.push('\n');
            }
        }

        result
    }

    fn collect_control_promoted_commands(input: &str) -> Vec<String> {
        let mut promoted = Vec::new();
        let mut in_control = false;

        for line in input.lines() {
            let trimmed = line.trim();
            let head = trimmed.split_whitespace().next().unwrap_or("");

            if head.eq_ignore_ascii_case(".control") {
                in_control = true;
                continue;
            }
            if head.eq_ignore_ascii_case(".endc") {
                in_control = false;
                continue;
            }
            if !in_control {
                continue;
            }

            if let Some(command) = Self::promote_control_netlist_command(line) {
                promoted.push(command);
            }
            if let Some(command) = Self::promote_control_set_command(line) {
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
        }

        promoted
    }

    fn promote_control_netlist_command(line: &str) -> Option<String> {
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
            "tran" => ".tran",
            "meas" => return Self::promote_control_measure_command(".meas", &args),
            "measure" => return Self::promote_control_measure_command(".measure", &args),
            _ => return None,
        };

        let mut promoted = String::from(promoted_command);
        for part in args {
            promoted.push(' ');
            promoted.push_str(&normalize_control_analysis_token(part));
        }
        Some(promoted)
    }

    fn promote_control_measure_command(command: &str, args: &[&str]) -> Option<String> {
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
            promoted.push_str(&normalize_control_analysis_token(part));
        }
        Some(promoted)
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

        let value = control_set_value(parts.next().unwrap_or(""), "digital_delay_type")?;
        Some(format!(".options digital_delay_type={value}"))
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

    fn strip_control_blocks_with_diagnostics(
        input: &str,
    ) -> Result<(String, Vec<ParseDiagnostic>), ParseError> {
        let mut result = String::with_capacity(input.len());
        let mut in_control = false;
        let mut opened_at_line = None;
        let mut diagnostics = Vec::new();

        for (line_index, line) in input.lines().enumerate() {
            let line_num = line_index + 1;
            let trimmed = line.trim();
            let head = trimmed.split_whitespace().next().unwrap_or("");

            if head.eq_ignore_ascii_case(".control") {
                in_control = true;
                opened_at_line = Some(line_num);
                diagnostics.push(ParseDiagnostic::warning(
                    line_num,
                    "control-block-ignored",
                    ".control scripting ignored; simple analysis commands and supported settings are promoted into the parsed deck",
                ));
                result.push_str("* ");
                result.push_str(line);
                result.push('\n');
            } else if head.eq_ignore_ascii_case(".endc") {
                if !in_control {
                    return Err(ParseError::Syntax {
                        line: line_num,
                        message: ".ENDC without matching .CONTROL".to_string(),
                    });
                }
                in_control = false;
                opened_at_line = None;
                result.push_str("* ");
                result.push_str(line);
                result.push('\n');
            } else if in_control {
                // Comment out control block content
                result.push_str("* ");
                result.push_str(line);
                result.push('\n');
            } else {
                result.push_str(line);
                result.push('\n');
            }
        }

        if let Some(line) = opened_at_line {
            return Err(ParseError::Syntax {
                line,
                message: ".CONTROL without a matching .ENDC".to_string(),
            });
        }

        Ok((result, diagnostics))
    }

    fn normalize_model_string_paths(&mut self, file_path: &std::path::Path) {
        let Some(base_dir) = file_path.parent() else {
            return;
        };

        for model in &mut self.models {
            for (name, value) in &mut model.string_params {
                *value = normalize_model_string_path_value(name, value, Some(base_dir));
            }
        }
    }

    fn normalize_source_file_paths(&mut self, file_path: &std::path::Path) {
        let Some(base_dir) = file_path.parent() else {
            return;
        };

        for element in &mut self.elements {
            match &mut element.kind {
                ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                    normalize_source_spec_file_paths(spec, base_dir);
                }
                _ => {}
            }
        }
    }

    /// Add a global node
    pub fn add_global(&mut self, node: &str) {
        self.global_nodes.insert(node.to_uppercase());
    }

    /// Check if a node is global
    pub fn is_global(&self, node: &str) -> bool {
        self.global_nodes.contains(&node.to_uppercase())
    }
}

fn normalize_source_spec_file_paths(spec: &mut SourceSpec, source_base_dir: &Path) {
    match spec {
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
    normalized.ends_with("file") || normalized.ends_with("_file") || normalized.ends_with("path")
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

fn strip_control_inline_comment(line: &str) -> &str {
    line.split_once(';').map_or(line, |(body, _)| body)
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

fn normalize_control_analysis_token(token: &str) -> String {
    token
        .strip_prefix("$&")
        .filter(|name| is_control_parameter_name(name))
        .unwrap_or(token)
        .to_string()
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
            data_tables: Vec::new(),
            models: Vec::new(),
            subcircuits: Vec::new(),
            params: ParamContext::new(),
            initial_conditions: Vec::new(),
            node_sets: Vec::new(),
            global_nodes: HashSet::new(),
            measurements: Vec::new(),
            saves: SaveSet::default(),
            options: SimulationOptions::default(),
            veriloga_includes: Vec::new(),
            spef_includes: Vec::new(),
            diagnostics: Vec::new(),
            source_text: None,
            source_path: None,
        }
    }
}

/// Read a file with automatic encoding detection
///
/// Handles:
/// - UTF-8 with or without BOM
/// - UTF-16 LE (common from Windows Notepad "Unicode" option)
/// - UTF-16 BE
/// - Falls back to Latin-1 if UTF-8 decoding fails
fn read_file_with_encoding(path: &std::path::Path) -> Result<String, std::io::Error> {
    use std::io::Read;

    let mut file = std::fs::File::open(path)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;

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
        match String::from_utf8(bytes.clone()) {
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
                crate::analysis::MeasureType::Max { signal, from, to } => {
                    assert_eq!(signal, "V(OUT)");
                    assert_eq!(*from, None);
                    assert_eq!(*to, None);
                }
                other => panic!("expected MAX measurement, got {other:?}"),
            }
        }
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
             meas ac gainmax max v(out) from=1 to=1k\n\
             .endc\n\
             .end\n",
        )
        .expect("control-block core analyses and measurement parse");

        assert!(netlist.analyses.iter().any(|analysis| matches!(analysis, AnalysisCommand::Op)));
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
    fn mismatched_subckt_end_name_is_rejected() {
        let err = Netlist::parse(
            "mismatched subckt end\n\
             .subckt AMP in out\n\
             R1 in out 1k\n\
             .ends FILTER\n\
             X1 a b AMP\n\
             .end\n",
        )
        .expect_err("mismatched .ENDS name must fail instead of closing the wrong subcircuit");

        let message = err.to_string();
        assert!(
            message.contains("AMP") && message.contains("FILTER"),
            "unexpected error: {message}"
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
    fn xyce_rf_ports_lower_to_dc_source_and_z0_termination() {
        let netlist = Netlist::parse(
            "xyce rf ports\n\
             P1 OUT 0 DC 2 PORT=1 Z0=75\n\
             P2 LOAD 0 PORT=2 Z0=100\n\
             .dc P1 0 2 1\n\
             .end\n",
        )
        .expect("Xyce RF port cards parse");

        let driven = netlist
            .elements
            .iter()
            .find(|element| element.name == "P1")
            .expect("driven port keeps source name");
        match &driven.kind {
            ElementKind::VoltageSource(SourceSpec::Dc(value)) => assert_eq!(*value, 2.0),
            other => panic!("expected driven port voltage source, got {other:?}"),
        }
        assert_eq!(
            driven.nodes,
            vec!["__RSPICE_P1_PORT".to_string(), "0".to_string()]
        );

        let series = netlist
            .elements
            .iter()
            .find(|element| element.name == "__RSPICE_P1_Z0")
            .expect("driven port has series Z0 resistor");
        match &series.kind {
            ElementKind::Resistor { value, .. } => assert_eq!(*value, 75.0),
            other => panic!("expected driven port Z0 resistor, got {other:?}"),
        }
        assert_eq!(
            series.nodes,
            vec!["OUT".to_string(), "__RSPICE_P1_PORT".to_string()]
        );

        let passive = netlist
            .elements
            .iter()
            .find(|element| element.name == "P2")
            .expect("passive port keeps port name");
        match &passive.kind {
            ElementKind::Resistor { value, .. } => assert_eq!(*value, 100.0),
            other => panic!("expected passive port Z0 termination, got {other:?}"),
        }
        assert_eq!(passive.nodes, vec!["LOAD".to_string(), "0".to_string()]);
    }

    #[test]
    fn pulse_source_accepts_xspice_phase_argument() {
        let netlist = Netlist::parse(
            "pulse phase\n\
             V1 out 0 PULSE(-1 1 0 1e-5 1e-5 5e-4 1e-3 45.0)\n\
             R1 out 0 1k\n\
             .tran 2e-5 2e-3\n\
             .end\n",
        )
        .expect("XSPICE PULSE phase argument should parse");

        assert!(matches!(
            first_source_spec(&netlist),
            SourceSpec::Pulse {
                phase,
                period,
                ..
            } if (*phase - 45.0).abs() < 1e-12 && (*period - 1.0e-3).abs() < 1e-15
        ));
    }

    #[test]
    fn source_distortion_terms_after_sin_are_consumed() {
        let netlist = Netlist::parse(
            "distortion source annotation\n\
             V1 1 0 DC 0 AC 1 SIN 0 1 1K 0 0 DISTOF1 0 DISTOF2 0\n\
             R1 1 0 1k\n\
             .ac dec 1 1k 1k\n\
             .end\n",
        )
        .expect("source distortion annotations should parse");

        assert!(matches!(
            first_source_spec(&netlist),
            SourceSpec::DcAcTransient {
                transient,
                ..
            } if matches!(transient.as_ref(), SourceSpec::Sin { .. })
        ));
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
             V1 p1 0 dc 0 ac 1 portnum 1 z0 50\n\
             R1 p1 0 50\n\
             .ac lin 1 1Meg 1Meg\n\
             .end\n",
        )
        .expect("ngspice source port annotations should parse after AC terms");

        assert!(matches!(
            first_source_spec(&netlist),
            SourceSpec::DcAc {
                dc_value,
                ac_magnitude,
                ac_phase,
            } if *dc_value == 0.0 && *ac_magnitude == 1.0 && *ac_phase == 0.0
        ));
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
            "C1 out 0 1p garbage",
            "L1 out 0 1n garbage",
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
}

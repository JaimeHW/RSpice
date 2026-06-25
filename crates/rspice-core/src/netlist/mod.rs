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
mod xspice_parser;

pub use ast::*;
pub use expr::{ParamContext, RandomState};
pub use flattener::{Flattener, FlattenerConfig, InstanceMetadata, flatten_netlist};
pub use hierarchy_path::{HierarchyPath, HierarchyPathConfig};
pub use include::{IncludeProcessor, parse_include_directive, parse_lib_directive};
pub use param_scope::{ParamResolver, ParamScope, ScopedParam};
pub use parser::*;
pub use source_map::*;

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
use std::path::PathBuf;

/// Represents a parsed netlist ready for circuit construction
#[derive(Debug, Clone)]
pub struct Netlist {
    /// Circuit title (first line of netlist)
    pub title: String,
    /// All circuit elements
    pub elements: Vec<Element>,
    /// Analysis commands
    pub analyses: Vec<AnalysisCommand>,
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
        let (sanitized, mut diagnostics) = Self::strip_control_blocks_with_diagnostics(input);
        let mut netlist = parser::parse_netlist(&sanitized)?;
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
        let processed = Self::preprocess_includes(input, file_path)?;
        let mut netlist = Self::parse(&processed)?;
        Self::normalize_model_string_paths(&mut netlist, file_path);
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
    pub fn strip_control_blocks(input: &str) -> String {
        Self::strip_control_blocks_with_diagnostics(input).0
    }

    fn strip_control_blocks_with_diagnostics(input: &str) -> (String, Vec<ParseDiagnostic>) {
        let mut result = String::with_capacity(input.len());
        let mut in_control = false;
        let mut diagnostics = Vec::new();

        for (index, line) in input.lines().enumerate() {
            let line_num = index + 1;
            let trimmed = line.trim().to_lowercase();

            if trimmed.starts_with(".control") {
                in_control = true;
                diagnostics.push(ParseDiagnostic::warning(
                    line_num,
                    "control-block-ignored",
                    ".control block ignored; RSpice parses circuit decks and does not execute ngspice command scripts",
                ));
                result.push_str("* ");
                result.push_str(line);
                result.push('\n');
            } else if trimmed.starts_with(".endc") {
                in_control = false;
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

        (result, diagnostics)
    }

    fn normalize_model_string_paths(&mut self, file_path: &std::path::Path) {
        let Some(base_dir) = file_path.parent() else {
            return;
        };

        for model in &mut self.models {
            for (name, value) in &mut model.string_params {
                if !Self::model_string_param_is_path(name) {
                    continue;
                }

                let candidate = std::path::Path::new(value);
                if candidate.is_absolute() {
                    continue;
                }

                let resolved = base_dir.join(candidate);
                *value = resolved.to_string_lossy().into_owned();
            }
        }
    }

    fn model_string_param_is_path(name: &str) -> bool {
        let normalized = name.trim().to_ascii_lowercase();
        normalized.ends_with("file")
            || normalized.ends_with("_file")
            || normalized.ends_with("path")
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

impl Default for Netlist {
    fn default() -> Self {
        Self {
            title: String::new(),
            elements: Vec::new(),
            analyses: Vec::new(),
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
    fn parses_parenthesized_scalar_model_parameter_values() {
        let netlist = Netlist::parse(
            "parenthesized model scalars\n\
             q1 c b e h1\n\
             .model h1 npn level=8 version=2.40\n\
             + c10 = ( 9.074e-030 ) hf0 = ( 40 ) alces = ( -0.2286 )\n\
             .op\n\
             .end\n",
        )
        .expect("netlist parses");

        let model = netlist
            .models
            .iter()
            .find(|model| model.name.eq_ignore_ascii_case("h1"))
            .expect("model exists");

        assert!(model.params.iter().any(|(name, value)| {
            name.eq_ignore_ascii_case("c10") && (*value - 9.074e-30).abs() < 1.0e-40
        }));
        assert!(model.params.iter().any(|(name, value)| {
            name.eq_ignore_ascii_case("hf0") && (*value - 40.0).abs() < f64::EPSILON
        }));
        assert!(model.params.iter().any(|(name, value)| {
            name.eq_ignore_ascii_case("alces") && (*value + 0.2286).abs() < 1.0e-15
        }));
    }

    #[test]
    fn rejects_unclosed_parenthesized_scalar_model_parameter_values() {
        let error = Netlist::parse(
            "bad parenthesized model scalar\n\
             q1 c b e h1\n\
             .model h1 npn level=8 version=2.40 c10=(9.074e-030\n\
             .op\n\
             .end\n",
        )
        .expect_err("unclosed parenthesized scalar should fail");

        assert!(
            error.to_string().contains("Expected ')'"),
            "error should explain the missing closing paren: {error}"
        );
    }

    #[test]
    fn reports_unsupported_dot_command_as_structured_diagnostic() {
        let netlist = Netlist::parse(
            "unknown command\n\
             V1 in 0 1\n\
             R1 in 0 1k\n\
             .foo compatibility_mode=vendor\n\
             .op\n\
             .end\n",
        )
        .expect("unsupported dot command should not abort parsing");

        assert!(netlist.diagnostics.iter().any(|diagnostic| {
            diagnostic.line == 4
                && diagnostic.code == "unsupported-dot-command"
                && diagnostic.message.to_ascii_lowercase().contains(".foo")
        }));
    }

    #[test]
    fn reports_unknown_options_key_as_structured_diagnostic() {
        let netlist = Netlist::parse(
            "unknown option\n\
             V1 in 0 1\n\
             R1 in 0 1k\n\
             .options reltol=1e-4 vendorcompat=1\n\
             .op\n\
             .end\n",
        )
        .expect("unknown option should not abort parsing");

        assert!(netlist.diagnostics.iter().any(|diagnostic| {
            diagnostic.line == 4
                && diagnostic.code == "unknown-option"
                && diagnostic
                    .message
                    .to_ascii_lowercase()
                    .contains("vendorcompat")
        }));
    }

    #[test]
    fn reports_control_block_stripping_as_structured_diagnostic() {
        let netlist = Netlist::parse(
            "control block\n\
             V1 in 0 1\n\
             R1 in 0 1k\n\
             .control\n\
             run\n\
             .endc\n\
             .op\n\
             .end\n",
        )
        .expect("control block should be stripped before circuit parsing");

        assert!(netlist.diagnostics.iter().any(|diagnostic| {
            diagnostic.line == 4
                && diagnostic.code == "control-block-ignored"
                && diagnostic.message.contains(".control")
        }));
    }

    #[test]
    fn reports_unsupported_mos_instance_token_as_structured_diagnostic() {
        let netlist = Netlist::parse(
            "mos tail token\n\
             M1 d g s b nmod L=1u W=2u @probe\n\
             .model nmod nmos level=1\n\
             .op\n\
             .end\n",
        )
        .expect("unsupported MOS token should not abort parsing");

        assert!(netlist.diagnostics.iter().any(|diagnostic| {
            diagnostic.line == 2
                && diagnostic.code == "unsupported-mos-instance-token"
                && diagnostic.message.contains("@")
        }));
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

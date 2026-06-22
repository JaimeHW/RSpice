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
    /// Optional original netlist text used to build this AST.
    /// Stored to support parameter re-application workflows (e.g., sensitivity).
    pub source_text: Option<String>,
    /// Optional source path for the netlist used to resolve relative includes
    /// and model-file references during reparsing workflows.
    pub source_path: Option<PathBuf>,
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
        let sanitized = Self::strip_control_blocks(input)?;
        let mut netlist = parser::parse_netlist(&sanitized)?;
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
    pub fn strip_control_blocks(input: &str) -> Result<String, ParseError> {
        let mut result = String::with_capacity(input.len());
        let mut in_control = false;
        let mut opened_at_line = None;

        for (line_index, line) in input.lines().enumerate() {
            let line_num = line_index + 1;
            let trimmed = line.trim();
            let head = trimmed.split_whitespace().next().unwrap_or("");

            if head.eq_ignore_ascii_case(".control") {
                in_control = true;
                opened_at_line = Some(line_num);
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

        Ok(result)
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
    fn unsupported_xspice_instance_token_is_rejected() {
        let err = Netlist::parse(
            "xspice malformed\n\
             A1 = in out gain gain=2\n\
             .end\n",
        )
        .expect_err("unsupported XSPICE instance token must fail");

        let message = err.to_string();
        assert!(
            message.contains("Unsupported XSPICE instance token '='"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn unsupported_xspice_bracket_token_is_rejected() {
        let err = Netlist::parse(
            "xspice malformed bracket\n\
             A1 [in = out] gain gain=2\n\
             .end\n",
        )
        .expect_err("unsupported XSPICE bracket token must fail");

        let message = err.to_string();
        assert!(
            message.contains("Unsupported XSPICE digital port token '='"),
            "unexpected error: {message}"
        );
    }

    #[test]
    fn xspice_accepts_commas_as_loose_port_separators() {
        let netlist = Netlist::parse(
            "xspice comma separators\n\
             A1 [in, out], out, gain gain=2\n\
             .end\n",
        )
        .expect("commas are accepted as XSPICE port separators");

        match &netlist.elements[0].kind {
            ElementKind::Xspice {
                model,
                ports,
                params,
            } => {
                assert_eq!(model, "GAIN");
                assert_eq!(
                    ports,
                    &vec![
                        XspicePort::DigitalVector(vec!["IN".to_string(), "OUT".to_string()]),
                        XspicePort::Analog("OUT".to_string()),
                    ]
                );
                assert_eq!(params, &vec![("GAIN".to_string(), 2.0)]);
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
            "R1 out 0 1k garbage",
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
                message.contains("garbage") || message.contains("GARBAGE"),
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
            } => {
                assert_eq!(path, "stim.csv");
                assert!((*time_scale - 1e-3).abs() < 1e-15);
                assert!((*value_scale - 2.0).abs() < f64::EPSILON);
                assert!((*time_offset - 3e-9).abs() < 1e-18);
                assert!((*value_offset + 1.0).abs() < f64::EPSILON);
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

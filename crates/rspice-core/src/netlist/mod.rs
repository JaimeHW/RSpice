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
pub mod param_scope;
mod parser;
mod xspice_parser;

pub use ast::*;
pub use expr::ParamContext;
pub use flattener::{Flattener, FlattenerConfig, InstanceMetadata, flatten_netlist};
pub use hierarchy_path::{HierarchyPath, HierarchyPathConfig};
pub use include::{IncludeProcessor, parse_include_directive, parse_lib_directive};
pub use param_scope::{ParamResolver, ParamScope, ScopedParam};
pub use parser::*;

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
    /// Simulation options from .OPTIONS commands
    pub options: SimulationOptions,
    /// Verilog-A model includes from .VERILOGA statements
    pub veriloga_includes: Vec<VerilogAInclude>,
    /// Optional original netlist text used to build this AST.
    /// Stored to support parameter re-application workflows (e.g., sensitivity).
    pub source_text: Option<String>,
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
        let sanitized = Self::strip_control_blocks(input);
        let mut netlist = parser::parse_netlist(&sanitized)?;
        netlist.source_text = Some(input.to_string());
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
        Ok(netlist)
    }

    /// Parse a netlist from a file with include expansion
    pub fn parse_file(path: &std::path::Path) -> Result<Self, ParseError> {
        let content = read_file_with_encoding(path)?;
        Self::parse_with_path(&content, path)
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
        let mut result = String::with_capacity(input.len());
        let mut in_control = false;

        for line in input.lines() {
            let trimmed = line.trim().to_lowercase();

            if trimmed.starts_with(".control") {
                in_control = true;
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

        result
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
            options: SimulationOptions::default(),
            veriloga_includes: Vec::new(),
            source_text: None,
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
    if bytes.len() % 2 != 0 {
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
    if bytes.len() % 2 != 0 {
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
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_dir(prefix: &str) -> std::path::PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "rspice_netlist_{}_{}_{}",
            prefix,
            std::process::id(),
            stamp
        ))
    }

    #[test]
    fn parse_ignores_control_blocks_in_normal_path() {
        let netlist = Netlist::parse(
            "\
control block parsing
R1 out 0 1k
.control
set noacct
tran 1n 10n
.endc
.op
.end
",
        )
        .expect("netlist with control block should parse");

        assert_eq!(netlist.elements.len(), 1);
        assert_eq!(netlist.analyses.len(), 1);
        assert!(matches!(netlist.analyses[0], AnalysisCommand::Op));
        assert_eq!(
            netlist.source_text.as_deref(),
            Some(
                "\
control block parsing
R1 out 0 1k
.control
set noacct
tran 1n 10n
.endc
.op
.end
"
            )
        );
    }

    #[test]
    fn parse_with_path_resolves_model_string_paths_relative_to_source() {
        let temp_dir = unique_temp_dir("model_string_paths");
        std::fs::create_dir_all(&temp_dir).expect("temp dir should be created");
        let netlist_path = temp_dir.join("deck.cir");
        let stimuli_path = temp_dir.join("stimulus.txt");
        std::fs::write(&stimuli_path, "placeholder").expect("stimulus file");

        let netlist = Netlist::parse_with_path(
            "\
relative path model test
.model src d_source (input_file=\"stimulus.txt\")
A1 [out] src
.end
",
            &netlist_path,
        )
        .expect("netlist should parse");

        let model = netlist
            .models
            .iter()
            .find(|m| m.name.eq_ignore_ascii_case("src"))
            .expect("model should be present");
        let input_file = model
            .string_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("input_file"))
            .map(|(_, value)| value.clone())
            .expect("input_file string param should be preserved");

        assert_eq!(std::path::PathBuf::from(input_file), stimuli_path);
    }
}

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
        parser::parse_netlist(input)
    }

    /// Parse a netlist from a string with include resolution
    ///
    /// This method preprocesses .include and .lib directives using the specified
    /// file path to resolve relative paths.
    pub fn parse_with_path(input: &str, file_path: &std::path::Path) -> Result<Self, ParseError> {
        let processed = Self::preprocess_includes(input, file_path)?;
        Self::parse(&processed)
    }

    /// Parse a netlist from a file with include expansion
    pub fn parse_file(path: &std::path::Path) -> Result<Self, ParseError> {
        let content = read_file_with_encoding(path)?;

        // Preprocess includes
        let processed = Self::preprocess_includes(&content, path)?;

        Self::parse(&processed)
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
        let mut result = String::new();

        for line in content.lines() {
            let trimmed = line.trim();
            let upper = trimmed.to_uppercase();

            if upper.starts_with(".INCLUDE") || upper.starts_with(".INC ") {
                // Parse and process include directive
                if let Some(filename) = parse_include_directive(trimmed) {
                    match processor.process_include(&filename) {
                        Ok(included_content) => {
                            result.push_str(&included_content);
                            result.push('\n');
                        }
                        Err(e) => {
                            // Log warning but don't fail - file may not exist
                            log::warn!("Include failed: {}", e);
                        }
                    }
                }
            } else if upper.starts_with(".LIB") && !upper.starts_with(".LIBS") {
                // Parse and process lib directive
                if let Some((filename, section)) = parse_lib_directive(trimmed) {
                    match processor.process_lib(&filename, section.as_deref()) {
                        Ok(lib_content) => {
                            result.push_str(&lib_content);
                            result.push('\n');
                        }
                        Err(e) => {
                            log::warn!("Library include failed: {}", e);
                        }
                    }
                }
            } else {
                result.push_str(line);
                result.push('\n');
            }
        }

        Ok(result)
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

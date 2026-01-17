//! Netlist parsing module
//!
//! Parses SPICE-compatible netlist files into an AST representation.
//! Uses a robust nom-based lexer for proper tokenization.
//!
//! Supports:
//! - Standard SPICE elements (R, L, C, V, I, D, Q, M, X)
//! - Advanced elements (K, S, W, T)
//! - Controlled sources (E, F, G, H, B)
//! - Analysis commands (.OP, .DC, .AC, .TRAN, .NOISE, .FOUR, .STEP, .TEMP)
//! - File inclusion (.INCLUDE, .LIB)
//! - Subcircuits with parameter passing

mod ast;
pub mod expr;
mod flattener;
pub mod include;
pub mod lexer;
mod parser;

pub use ast::*;
pub use expr::ParamContext;
pub use flattener::{Flattener, FlattenerConfig, flatten_netlist};
pub use include::{IncludeProcessor, parse_include_directive, parse_lib_directive};
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
    /// Global nodes from .GLOBAL (not renamed in subcircuits)
    pub global_nodes: HashSet<String>,
    /// Measurement statements from .MEAS commands
    pub measurements: Vec<MeasureStatement>,
}

impl Netlist {
    /// Parse a netlist from a string
    pub fn parse(input: &str) -> Result<Self, ParseError> {
        parser::parse_netlist(input)
    }

    /// Parse a netlist from a file
    pub fn parse_file(path: &std::path::Path) -> Result<Self, ParseError> {
        let content = std::fs::read_to_string(path)?;
        Self::parse(&content)
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
            global_nodes: HashSet::new(),
            measurements: Vec::new(),
        }
    }
}

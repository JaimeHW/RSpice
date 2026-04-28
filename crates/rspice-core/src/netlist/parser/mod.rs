//! SPICE netlist parser using token-based parsing
//!
//! Parses standard SPICE netlist format with extensions including:
//! - Sloppy syntax (commas, trailing parameters)
//! - PULSE/SIN/PWL/EXP source specifications with parentheses
//! - .PARAM statements with expression evaluation
//! - Subcircuit definitions and instances

use super::expr::eval_expression;
use super::lexer::{LexError, TokenKind, TokenStream, tokenize};
use super::xspice_parser;
use super::{
    AnalysisCommand, BjtType, Element, ElementKind, FreqVariation, InitialCondition, JfetType,
    MesfetType, ModelDef, MonteCarloCommand, MonteCarloDistribution, MosType, Netlist, NodeSet,
    ParamContext, ParametricValue, ParseError, PoleZeroAnalysisType, PoleZeroTransferType,
    SensitivityAcSweep, SimulationOptions, SourceSpec, StepCommand, StepSweep, StepTarget,
    SubcircuitDef, SwitchState, VerilogAInclude,
};
use crate::Value;
use std::collections::{HashMap, HashSet};

mod command_parsers;
mod commands;
mod elements;
mod line;
mod scoping;
mod source_specs;
mod state;
mod tlines;
mod values;

use command_parsers::*;
use commands::*;
use elements::*;
use line::*;
use scoping::*;
use source_specs::parse_source_spec;
use state::*;
use tlines::*;
use values::*;

type MeasureStatement = crate::analysis::MeasureStatement;

//=============================================================================
// Main Parser
//=============================================================================

/// Parse a complete netlist from string
pub fn parse_netlist(input: &str) -> Result<Netlist, ParseError> {
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return Ok(Netlist::default());
    }

    // First line is the title
    let title = lines[0].to_string();
    let mut state = ParseState::new();

    let mut line_num = 1;
    let mut continuation = String::new();

    for line in lines.iter().skip(1) {
        line_num += 1;

        // Strip inline ';' and '$' comments (common SPICE syntax), then trim.
        // We intentionally keep this simple and treat these markers as comment
        // starts only when they appear outside quoted strings.
        let no_inline_comment = strip_inline_semicolon_comment(line);
        let trimmed = no_inline_comment.trim();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }

        // Handle line continuation (+ at start of line)
        if let Some(rest) = trimmed.strip_prefix('+') {
            continuation.push(' ');
            continuation.push_str(rest);
            continue;
        }

        // Process previous continued line if exists
        if !continuation.is_empty() {
            process_line(&continuation, line_num - 1, &mut state)?;
            continuation.clear();
        }

        // Check for .END
        if trimmed.eq_ignore_ascii_case(".end") {
            break;
        }

        // Handle .VERILOGA directive directly (before continuation handling)
        if let Some(include) = parse_veriloga_directive(trimmed) {
            log::debug!("Found .VERILOGA include: {:?}", include.file_path);
            state.push_veriloga_include(include);
            continue; // Skip normal processing
        }

        // Start new continuation or process line
        continuation = trimmed.to_string();
    }

    // Process final line
    if !continuation.is_empty() {
        process_line(&continuation, line_num, &mut state)?;
    }

    state.into_netlist(title, input, line_num)
}
//=============================================================================
// Command Parsing
//=============================================================================

//=============================================================================
// Element Parsing
//=============================================================================

//=============================================================================
// Subcircuit Parsing
//=============================================================================

//=============================================================================
// Model Parameter Parsing
//=============================================================================

//=============================================================================
// Helper Functions
//=============================================================================

//=============================================================================
// New Element Type Parsing
//=============================================================================

//=============================================================================
// Tests
//=============================================================================

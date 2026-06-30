//! SPICE netlist parser using token-based parsing
//!
//! Parses standard SPICE netlist format with extensions including:
//! - Sloppy syntax (commas, trailing parameters)
//! - PULSE/SIN/PWL/EXP source specifications with parentheses
//! - .PARAM statements with expression evaluation
//! - Subcircuit definitions and instances

use super::expr::eval_expression;
use super::lexer::{LexError, TokenKind, TokenStream, parse_spice_value, tokenize};
use super::xspice_parser;
use super::{
    AnalysisCommand, BjtType, Element, ElementKind, FreqVariation, InitialCondition, JfetType,
    MesfetType, ModelDef, MonteCarloCommand, MonteCarloDistribution, MosType, Netlist, NodeSet,
    ParamContext, ParametricValue, ParseDiagnostic, ParseError, PoleZeroAnalysisType,
    PoleZeroTransferType, SaveSet, SaveSignal, SensitivityAcSweep, SimulationOptions, SourceSpec,
    StepCommand, StepSweep, StepTarget, SubcircuitDef, SwitchState, VerilogAInclude,
};
use crate::Value;
use std::collections::{HashMap, HashSet};

mod command_parsers;
mod commands;

pub use commands::parse_save_probe;
mod conditionals;
mod elements;
mod laplace_synthesis;
mod line;
mod scoping;
mod source_specs;
mod state;
mod tlines;
mod values;

use command_parsers::*;
use commands::*;
use conditionals::*;
use elements::*;
use laplace_synthesis::*;
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

    // Seed the statistical expression functions before any parameter
    // evaluation so the deck behaves identically regardless of where the
    // `.options seed=` line appears.
    if let Some(seed) = prescan_random_seed(&lines)? {
        state.params.set_random_seed(seed);
        log::info!("statistical expression functions seeded with {seed} (.options seed)");
    }

    let mut line_num = 1;
    let mut continuation = String::new();
    let mut skipping_data_block = false;
    let mut data_block_line = None;

    for line in lines.iter().skip(1) {
        line_num += 1;

        // Strip inline comments (common SPICE syntax), then trim.
        // We intentionally keep this simple and treat these markers as comment
        // starts only when they appear outside quoted strings.
        let no_inline_comment = strip_inline_semicolon_comment(line);
        let trimmed = no_inline_comment.trim();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }

        // `.DATA` rows belong to multi-run expansion (`netlist::multi_run`),
        // not the circuit; skip the block through `.ENDDATA`.
        let head = trimmed.split_whitespace().next().unwrap_or("");
        if skipping_data_block {
            if head.eq_ignore_ascii_case(".enddata") {
                skipping_data_block = false;
                data_block_line = None;
            }
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
            process_line_gated(&continuation, line_num - 1, &mut state)?;
            continuation.clear();
        }

        // Check for .END
        if trimmed.eq_ignore_ascii_case(".end") {
            break;
        }

        // `.ALTER` ends the base deck; the variants expand textually
        // before parsing (multi-run), so this parse stops here.
        if head.eq_ignore_ascii_case(".alter") {
            log::info!(
                "line {line_num}: .ALTER present; this parse covers the base deck - \
                 run multi-run expansion for the alter variants"
            );
            break;
        }
        if head.eq_ignore_ascii_case(".data") {
            skipping_data_block = true;
            data_block_line = Some(line_num);
            continue;
        }
        if head.eq_ignore_ascii_case(".enddata") {
            return Err(ParseError::Syntax {
                line: line_num,
                message: ".ENDDATA without matching .DATA".to_string(),
            });
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

    if let Some(opened_at_line) = data_block_line {
        return Err(ParseError::Syntax {
            line: opened_at_line,
            message: ".DATA without a matching .ENDDATA".to_string(),
        });
    }

    // Process final line
    if !continuation.is_empty() {
        process_line_gated(&continuation, line_num, &mut state)?;
    }

    if let Some(frame) = state.conditional_stack.last() {
        return Err(ParseError::Syntax {
            line: frame.opened_at_line,
            message: ".if without a matching .endif".to_string(),
        });
    }

    state.into_netlist(title, input, line_num)
}

/// Dispatch one logical line through the conditional gate: conditional
/// directives update the block stack, lines inside false branches are
/// skipped, and everything else flows to `process_line`.
fn process_line_gated(
    line: &str,
    line_num: usize,
    state: &mut ParseState,
) -> Result<(), ParseError> {
    if let Some(directive) = parse_conditional_directive(line) {
        return state.apply_conditional_directive(directive, line_num);
    }
    if state.conditionals_suppress() {
        return Ok(());
    }
    process_line(line, line_num, state)
}

/// Pre-scan for `.options seed=<n>` (alias `rndseed=<n>`) so the statistical
/// expression functions can be seeded before any parameter evaluation,
/// regardless of where the option appears in the deck.
///
/// Scans `.options` logical lines (including `+` continuations); the last
/// occurrence wins, matching SPICE option-override behavior. `seed=random`
/// is ignored here — `parse_options_command` emits the warning for it.
fn prescan_random_seed(lines: &[&str]) -> Result<Option<u64>, ParseError> {
    let mut seed = None;
    let mut in_options = false;
    let mut line_num = 1usize;

    for line in lines.iter().skip(1) {
        line_num += 1;
        let stripped = strip_inline_semicolon_comment(line);
        let trimmed = stripped.trim();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }

        let upper = trimmed.to_uppercase();
        if let Some(rest) = upper.strip_prefix('+') {
            if !in_options {
                continue;
            }
            scan_options_tokens_for_seed(rest, line_num, &mut seed)?;
            continue;
        }

        let body = [".OPTIONS", ".OPTION", ".OPT"]
            .iter()
            .find_map(|prefix| upper.strip_prefix(prefix));
        match body {
            // Require a separator after the command word so unrelated
            // commands starting with `.opt` are not misread.
            Some(rest) if rest.is_empty() || rest.starts_with([' ', '\t']) => {
                in_options = true;
                scan_options_tokens_for_seed(rest, line_num, &mut seed)?;
            }
            _ => in_options = false,
        }
    }

    Ok(seed)
}

/// Scan one (partial) `.options` line for `seed=`/`rndseed=` assignments.
fn scan_options_tokens_for_seed(
    body: &str,
    line_num: usize,
    seed: &mut Option<u64>,
) -> Result<(), ParseError> {
    // Normalize `key = value` to `key=value`, then inspect each token.
    let collapsed: String = body.split('=').map(str::trim).collect::<Vec<_>>().join("=");

    for token in collapsed.split([' ', '\t', ',']) {
        if let Some((key, value)) = token.split_once('=') {
            if key != "SEED" && key != "RNDSEED" {
                continue;
            }
            if value.eq_ignore_ascii_case("RANDOM") {
                continue;
            }
            let parsed = parse_spice_value(value).map_err(|_| ParseError::Syntax {
                line: line_num,
                message: format!("SEED must be a non-negative integer, found `{value}`"),
            })?;
            *seed = Some(parse_seed_option(parsed, line_num)?);
        }
    }
    Ok(())
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

//! SPICE netlist parser using token-based parsing
//!
//! Parses standard SPICE netlist format with extensions including:
//! - Sloppy syntax (commas, trailing parameters)
//! - PULSE/SIN/PWL/EXP source specifications with parentheses
//! - .PARAM statements with expression evaluation
//! - Subcircuit definitions and instances

use super::expr::{eval_expression, eval_expression_complex, prepare_behavioral_expression};
use super::include::{ExpandedSource, ExpandedSourceItem};
use super::lexer::{LexError, TokenKind, TokenStream, parse_spice_value, tokenize};
use super::xspice_parser;
use super::{
    AnalysisCommand, BjtType, DataTable, Element, ElementKind, ExpressionDialect, FftAnalysis,
    FftFormat, FftOutput, FftWindow, FreqVariation, InitialCondition, JfetType, MesfetType,
    MissingSubcircuitEndsBoundary, ModelDef, MonteCarloCommand, MonteCarloDistribution, MosType,
    Netlist, NetlistSourceLocation, NodeSet, ParamContext, ParameterRedefinitionPolicy,
    ParametricValue, ParseDiagnostic, ParseError, ParseWithAbortError, PoleZeroAnalysisType,
    PoleZeroTransferType, PspiceUTiming, PspiceUTimingMode, SaveSet, SaveSignal,
    SensitivityAcSweep, SimulationOptions, SourceRfPort, SourceSpec, StatisticalParamMode,
    StepCommand, StepSweep, StepTarget, SubcircuitDef, SwitchState, VerilogAInclude,
    ensure_parse_not_aborted, finish_non_aborting_parse, poll_parse_abort, poll_parse_text,
};
use crate::Value;
use crate::abort_signal::{AbortSignal, NoAbort};
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
pub(in crate::netlist) use source_specs::parse_source_spec_text;
use state::*;
use tlines::*;
use values::*;

type MeasureStatement = crate::analysis::MeasureStatement;

//=============================================================================
// Main Parser
//=============================================================================

/// Options that affect netlist parsing and immediate parameter evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetlistParseOptions {
    pub statistical_mode: StatisticalParamMode,
    pub expression_dialect: ExpressionDialect,
    pub parameter_redefinition_policy: ParameterRedefinitionPolicy,
}

impl Default for NetlistParseOptions {
    fn default() -> Self {
        Self {
            statistical_mode: StatisticalParamMode::Sample,
            expression_dialect: ExpressionDialect::Ngspice,
            parameter_redefinition_policy: ParameterRedefinitionPolicy::UseLast,
        }
    }
}

#[derive(Debug)]
struct DataTableBuilder {
    opened_at_line: usize,
    name: String,
    params: Vec<String>,
    flat_values: Vec<Value>,
}

#[derive(Debug)]
struct SourceEventSchedule {
    origins: Vec<NetlistSourceLocation>,
    events: HashMap<usize, Vec<ExpandedSourceItem>>,
}

impl SourceEventSchedule {
    fn from_expanded(expanded: &ExpandedSource) -> Self {
        let mut origins = Vec::new();
        let mut events = HashMap::<usize, Vec<ExpandedSourceItem>>::new();
        for item in &expanded.items {
            match item {
                ExpandedSourceItem::Line { origin, .. } => origins.push(origin.clone()),
                event => events.entry(origins.len()).or_default().push(event.clone()),
            }
        }
        Self { origins, events }
    }

    fn origin(&self, zero_based_line: usize) -> Option<&NetlistSourceLocation> {
        self.origins.get(zero_based_line)
    }

    fn take_events(&mut self, before_zero_based_line: usize) -> Vec<ExpandedSourceItem> {
        self.events
            .remove(&before_zero_based_line)
            .unwrap_or_default()
    }
}

#[derive(Debug)]
struct ActiveSourceFrame {
    path: std::path::PathBuf,
    entry_subckt_depth: usize,
}

impl DataTableBuilder {
    fn new(
        opened_at_line: usize,
        line: &str,
        abort: &dyn AbortSignal,
    ) -> Result<Self, ParseWithAbortError> {
        poll_parse_text(abort, line)?;
        let mut fields = line.split_whitespace();
        let _data = fields.next();
        let Some(name) = fields.next() else {
            return Err(ParseError::Syntax {
                line: opened_at_line,
                message: ".DATA requires a table name".to_string(),
            }
            .into());
        };
        let mut params = Vec::new();
        for (index, field) in fields.enumerate() {
            poll_parse_abort(abort, index)?;
            params.push(field.to_string());
        }
        validate_data_table_params_with_abort(opened_at_line, name, &params, abort)?;
        Ok(Self {
            opened_at_line,
            name: name.to_string(),
            params,
            flat_values: Vec::new(),
        })
    }

    fn push_line(
        &mut self,
        line_num: usize,
        line: &str,
        params: &ParamContext,
        abort: &dyn AbortSignal,
    ) -> Result<(), ParseWithAbortError> {
        poll_parse_text(abort, line)?;
        let body = line.strip_prefix('+').unwrap_or(line).trim();
        if body.is_empty() {
            return Ok(());
        }
        let mut fields = body.split_whitespace().peekable();
        if fields.peek().is_none() {
            return Ok(());
        }

        if self.params.is_empty() {
            for (index, field) in fields.enumerate() {
                poll_parse_abort(abort, index)?;
                self.params.push(field.to_string());
            }
            validate_data_table_params_with_abort(line_num, &self.name, &self.params, abort)?;
            return Ok(());
        }

        for (index, field) in fields.enumerate() {
            poll_parse_abort(abort, index)?;
            self.flat_values.push(parse_data_table_value_with_abort(
                line_num, &self.name, field, params, abort,
            )?);
        }
        ensure_parse_not_aborted(abort)?;
        Ok(())
    }

    fn finish(
        self,
        line_num: usize,
        abort: &dyn AbortSignal,
    ) -> Result<DataTable, ParseWithAbortError> {
        if self.params.is_empty() {
            return Err(ParseError::Syntax {
                line: self.opened_at_line,
                message: format!(".DATA {} has no parameter columns", self.name),
            }
            .into());
        }
        let columns = self.params.len();
        if !self.flat_values.len().is_multiple_of(columns) {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    ".DATA {} has {} value(s), which does not fill {} column(s)",
                    self.name,
                    self.flat_values.len(),
                    columns
                ),
            }
            .into());
        }
        let mut rows = Vec::with_capacity(self.flat_values.len() / columns);
        for (index, chunk) in self.flat_values.chunks_exact(columns).enumerate() {
            poll_parse_abort(abort, index)?;
            rows.push(chunk.to_vec());
        }
        ensure_parse_not_aborted(abort)?;
        Ok(DataTable {
            name: self.name,
            params: self.params,
            rows,
        })
    }
}

fn validate_data_table_params_with_abort(
    line_num: usize,
    table_name: &str,
    params: &[String],
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    for (index, param) in params.iter().enumerate() {
        poll_parse_abort(abort, index)?;
        poll_parse_text(abort, param)?;
        let mut chars = param.chars();
        let valid = chars
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic() || ch == '_' || ch == '$')
            && chars.all(|ch| {
                ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' || ch == '.' || ch == ':'
            });
        if !valid {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    ".DATA {table_name} parameter column '{param}' is not a valid parameter name"
                ),
            }
            .into());
        }
    }
    ensure_parse_not_aborted(abort)?;
    Ok(())
}

fn parse_data_table_value_with_abort(
    line_num: usize,
    table_name: &str,
    token: &str,
    params: &ParamContext,
    abort: &dyn AbortSignal,
) -> Result<Value, ParseWithAbortError> {
    poll_parse_text(abort, token)?;
    if let Ok(value) = parse_spice_value(token) {
        return Ok(value);
    }
    let expr = token
        .strip_prefix('{')
        .and_then(|rest| rest.strip_suffix('}'))
        .unwrap_or(token);
    let value = eval_expression(expr, params).map_err(|err| ParseError::Syntax {
        line: line_num,
        message: format!(".DATA {table_name} value '{token}' is not numeric: {err}"),
    })?;
    ensure_parse_not_aborted(abort)?;
    Ok(value)
}

/// Parse a complete netlist from string
pub fn parse_netlist(input: &str) -> Result<Netlist, ParseError> {
    parse_netlist_with_options(input, NetlistParseOptions::default())
}

/// Parse a complete netlist from string with explicit parse options.
pub fn parse_netlist_with_options(
    input: &str,
    options: NetlistParseOptions,
) -> Result<Netlist, ParseError> {
    finish_non_aborting_parse(parse_netlist_with_options_and_abort(
        input, options, &NoAbort,
    ))
}

/// Parse a complete netlist with explicit options and cooperative
/// cancellation.
pub fn parse_netlist_with_options_and_abort(
    input: &str,
    options: NetlistParseOptions,
    abort: &dyn AbortSignal,
) -> Result<Netlist, ParseWithAbortError> {
    parse_netlist_impl(input, options, None, abort)
}

pub(crate) fn parse_expanded_netlist_with_options_and_abort(
    expanded: &ExpandedSource,
    options: NetlistParseOptions,
    abort: &dyn AbortSignal,
) -> Result<Netlist, ParseWithAbortError> {
    let rendered = expanded.render();
    parse_netlist_impl(
        &rendered,
        options,
        Some(SourceEventSchedule::from_expanded(expanded)),
        abort,
    )
}

fn parse_netlist_impl(
    input: &str,
    options: NetlistParseOptions,
    mut source_schedule: Option<SourceEventSchedule>,
    abort: &dyn AbortSignal,
) -> Result<Netlist, ParseWithAbortError> {
    ensure_parse_not_aborted(abort)?;
    let mut lines: Vec<&str> = Vec::new();
    for (index, line) in input.lines().enumerate() {
        poll_parse_abort(abort, index)?;
        lines.push(line);
    }

    if lines.is_empty() {
        return Ok(Netlist::default());
    }

    // First line is the title
    let title = lines[0].to_string();
    let mut state = ParseState::new();
    state.params.set_statistical_mode(options.statistical_mode);
    state
        .params
        .set_expression_dialect(options.expression_dialect);
    state
        .params
        .set_parameter_redefinition_policy(options.parameter_redefinition_policy);

    // Seed the statistical expression functions before any parameter
    // evaluation so the deck behaves identically regardless of where the
    // `.options seed=` line appears.
    if let Some(seed) = prescan_random_seed_with_abort(&lines, abort)? {
        state.params.set_random_seed(seed);
        log::info!("statistical expression functions seeded with {seed} (.options seed)");
    }
    prescan_temperature_options_with_abort(&lines, &mut state, abort)?;

    let mut line_num = 1;
    let mut continuation = String::new();
    let mut continuation_line = None;
    let mut continuation_origin = None;
    let mut data_table: Option<DataTableBuilder> = None;
    let mut active_sources = Vec::new();
    let mut deferred_source_boundaries = Vec::new();
    let mut termination = None;
    let mut root_eof = None;

    process_source_events_at(
        source_schedule.as_mut(),
        0,
        &mut active_sources,
        &mut deferred_source_boundaries,
        &mut continuation,
        &mut continuation_line,
        &mut continuation_origin,
        &mut state,
    )?;

    for (line_index, line) in lines.iter().skip(1).enumerate() {
        let zero_based_line = line_index + 1;
        process_source_events_at(
            source_schedule.as_mut(),
            zero_based_line,
            &mut active_sources,
            &mut deferred_source_boundaries,
            &mut continuation,
            &mut continuation_line,
            &mut continuation_origin,
            &mut state,
        )?;
        poll_parse_abort(abort, line_index)?;
        poll_parse_text(abort, line)?;
        line_num += 1;
        let origin = source_schedule
            .as_ref()
            .and_then(|schedule| schedule.origin(zero_based_line))
            .cloned()
            .unwrap_or_else(|| NetlistSourceLocation::in_memory(line_num));

        // Strip inline comments (common SPICE syntax), then trim.
        // We intentionally keep this simple and treat these markers as comment
        // starts only when they appear outside quoted strings.
        let no_inline_comment = strip_inline_semicolon_comment(line);
        let trimmed = no_inline_comment.trim();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }

        let ordinary_continuation = data_table.is_none() && trimmed.starts_with('+');
        if !ordinary_continuation {
            flush_pending_logical_line(
                &mut continuation,
                &mut continuation_line,
                &mut continuation_origin,
                &mut state,
            )?;
            apply_deferred_source_boundaries(
                &mut deferred_source_boundaries,
                false,
                &mut active_sources,
                &state,
            )?;
        }

        let head = trimmed.split_whitespace().next().unwrap_or("");
        if let Some(table) = data_table.as_mut() {
            if head.eq_ignore_ascii_case(".enddata") {
                let table = data_table
                    .take()
                    .expect(".DATA builder exists while inside data block")
                    .finish(line_num, abort)?;
                state.data_tables.push(table);
            } else if head.eq_ignore_ascii_case(".data") {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: ".DATA cannot be nested inside another .DATA block".to_string(),
                }
                .into());
            } else if is_dot_command_head(head) {
                return Err(ParseError::Syntax {
                    line: table.opened_at_line,
                    message: ".DATA without a matching .ENDDATA".to_string(),
                }
                .into());
            } else {
                table.push_line(line_num, trimmed, &state.params, abort)?;
            }
            continue;
        }

        // Handle line continuation (+ at start of line)
        if let Some(rest) = trimmed.strip_prefix('+') {
            continuation_line.get_or_insert(line_num);
            continuation_origin.get_or_insert_with(|| origin.clone());
            continuation.push(' ');
            continuation.push_str(rest);
            continue;
        }

        // Check for .END
        if trimmed.eq_ignore_ascii_case(".end") {
            termination = Some((MissingSubcircuitEndsBoundary::EndCard, origin));
            break;
        }

        // `.ALTER` ends the base deck; the variants expand textually
        // before parsing (multi-run), so this parse stops here.
        if head.eq_ignore_ascii_case(".alter") {
            log::info!(
                "line {line_num}: .ALTER present; this parse covers the base deck - \
                 run multi-run expansion for the alter variants"
            );
            termination = Some((MissingSubcircuitEndsBoundary::AlterCard, origin));
            break;
        }
        if head.eq_ignore_ascii_case(".data") {
            data_table = Some(DataTableBuilder::new(line_num, trimmed, abort)?);
            continue;
        }
        if head.eq_ignore_ascii_case(".enddata") {
            return Err(ParseError::Syntax {
                line: line_num,
                message: ".ENDDATA without matching .DATA".to_string(),
            }
            .into());
        }

        // Handle .VERILOGA directive directly (before continuation handling)
        if let Some(include) = parse_veriloga_directive(trimmed) {
            log::debug!("Found .VERILOGA include: {:?}", include.file_path);
            state.push_veriloga_include(include);
            continue; // Skip normal processing
        }

        // Start new continuation or process line
        continuation = trimmed.to_string();
        continuation_line = Some(line_num);
        continuation_origin = Some(origin);
    }

    if termination.is_none() {
        process_source_events_at(
            source_schedule.as_mut(),
            lines.len(),
            &mut active_sources,
            &mut deferred_source_boundaries,
            &mut continuation,
            &mut continuation_line,
            &mut continuation_origin,
            &mut state,
        )?;
    }

    if let Some(table) = data_table {
        return Err(ParseError::Syntax {
            line: table.opened_at_line,
            message: ".DATA without a matching .ENDDATA".to_string(),
        }
        .into());
    }

    // Process final line
    if !continuation.is_empty() {
        flush_pending_logical_line(
            &mut continuation,
            &mut continuation_line,
            &mut continuation_origin,
            &mut state,
        )?;
    }
    if termination.is_none() {
        root_eof = apply_deferred_source_boundaries(
            &mut deferred_source_boundaries,
            true,
            &mut active_sources,
            &state,
        )?;
    }

    if let Some(frame) = state.conditional_stack.last() {
        return Err(ParseError::Syntax {
            line: frame.opened_at_line,
            message: ".if without a matching .endif".to_string(),
        }
        .into());
    }

    if let Some((boundary, detected_at)) = termination {
        if let Some(error) = state.missing_subcircuit_ends(detected_at, boundary) {
            return Err(error.into());
        }
    } else {
        let detected_at = root_eof
            .clone()
            .unwrap_or_else(|| NetlistSourceLocation::in_memory(lines.len() + 1));
        if let Some(error) = state.missing_subcircuit_ends(
            detected_at.clone(),
            MissingSubcircuitEndsBoundary::EndOfSource,
        ) {
            return Err(error.into());
        }
    }

    normalize_pspice_u_timing_aliases_with_abort(&mut state, abort)?;
    resolve_top_level_deferred_source_specs_with_abort(&mut state.elements, &state.params, abort)?;
    validate_resistor_model_references_with_abort(&state, abort)?;

    ensure_parse_not_aborted(abort)?;
    state
        .into_netlist(
            title,
            input,
            root_eof.unwrap_or_else(|| NetlistSourceLocation::in_memory(lines.len() + 1)),
        )
        .map_err(ParseWithAbortError::from)
}

#[allow(clippy::too_many_arguments)]
fn process_source_events_at(
    source_schedule: Option<&mut SourceEventSchedule>,
    before_zero_based_line: usize,
    active_sources: &mut Vec<ActiveSourceFrame>,
    deferred_source_boundaries: &mut Vec<ExpandedSourceItem>,
    continuation: &mut String,
    continuation_line: &mut Option<usize>,
    continuation_origin: &mut Option<NetlistSourceLocation>,
    state: &mut ParseState,
) -> Result<(), ParseWithAbortError> {
    let Some(source_schedule) = source_schedule else {
        return Ok(());
    };
    for event in source_schedule.take_events(before_zero_based_line) {
        match event {
            event @ (ExpandedSourceItem::EnterSource { .. }
            | ExpandedSourceItem::ExitSource { .. }) => {
                deferred_source_boundaries.push(event);
            }
            ExpandedSourceItem::EndCard { origin } => {
                flush_pending_logical_line(
                    continuation,
                    continuation_line,
                    continuation_origin,
                    state,
                )?;
                apply_deferred_source_boundaries(
                    deferred_source_boundaries,
                    false,
                    active_sources,
                    state,
                )?;
                let Some(source) = active_sources.last() else {
                    return Err(ParseError::Syntax {
                        line: origin.line,
                        message: format!(
                            "{}: included .END has no active source frame",
                            origin
                        ),
                    }
                    .into());
                };
                validate_source_subckt_depth(
                    state,
                    source,
                    origin,
                    MissingSubcircuitEndsBoundary::EndCard,
                )?;
            }
            ExpandedSourceItem::Line { .. } => {
                unreachable!("line items are represented in the rendered source")
            }
        }
    }
    Ok(())
}

fn apply_deferred_source_boundaries(
    deferred_source_boundaries: &mut Vec<ExpandedSourceItem>,
    defer_root_exit: bool,
    active_sources: &mut Vec<ActiveSourceFrame>,
    state: &ParseState,
) -> Result<Option<NetlistSourceLocation>, ParseWithAbortError> {
    let mut root_eof = None;
    for event in deferred_source_boundaries.drain(..) {
        match event {
            ExpandedSourceItem::EnterSource { path } => {
                active_sources.push(ActiveSourceFrame {
                    path,
                    entry_subckt_depth: state.subckt_stack.len(),
                });
            }
            ExpandedSourceItem::ExitSource { path, eof_line } => {
                let Some(source) = active_sources.last() else {
                    return Err(ParseError::Syntax {
                        line: eof_line,
                        message: format!(
                            "{}:{}: source exit has no matching source entry",
                            path.display(),
                            eof_line
                        ),
                    }
                    .into());
                };
                if source.path != path {
                    return Err(ParseError::Syntax {
                        line: eof_line,
                        message: format!(
                            "source expansion boundary mismatch: entered '{}', exited '{}'",
                            source.path.display(),
                            path.display()
                        ),
                    }
                    .into());
                }
                let detected_at = NetlistSourceLocation::in_file(&path, eof_line);
                if defer_root_exit && active_sources.len() == 1 {
                    root_eof = Some(detected_at);
                } else {
                    validate_source_subckt_depth(
                        state,
                        source,
                        detected_at,
                        MissingSubcircuitEndsBoundary::EndOfSource,
                    )?;
                }
                active_sources.pop();
            }
            ExpandedSourceItem::EndCard { .. } | ExpandedSourceItem::Line { .. } => {
                unreachable!("only source entry/exit events are deferred")
            }
        }
    }
    Ok(root_eof)
}

fn flush_pending_logical_line(
    continuation: &mut String,
    continuation_line: &mut Option<usize>,
    continuation_origin: &mut Option<NetlistSourceLocation>,
    state: &mut ParseState,
) -> Result<(), ParseWithAbortError> {
    if continuation.is_empty() {
        return Ok(());
    }
    let logical_line = continuation_line
        .take()
        .expect("non-empty logical statement records its physical origin");
    let logical_origin = continuation_origin
        .take()
        .unwrap_or_else(|| NetlistSourceLocation::in_memory(logical_line));
    process_line_gated(continuation, logical_line, &logical_origin, state)
        .map_err(ParseWithAbortError::from)?;
    continuation.clear();
    Ok(())
}

fn validate_source_subckt_depth(
    state: &ParseState,
    source: &ActiveSourceFrame,
    detected_at: NetlistSourceLocation,
    boundary: MissingSubcircuitEndsBoundary,
) -> Result<(), ParseWithAbortError> {
    match state.subckt_stack.len().cmp(&source.entry_subckt_depth) {
        std::cmp::Ordering::Greater => Err(state
            .missing_subcircuit_ends(detected_at, boundary)
            .expect("greater subcircuit depth has an open frame")
            .into()),
        std::cmp::Ordering::Less => Err(ParseError::Syntax {
            line: detected_at.line,
            message: format!(
                "{}: included source closed a .SUBCKT opened by its parent source",
                detected_at
            ),
        }
        .into()),
        std::cmp::Ordering::Equal => Ok(()),
    }
}

fn resolve_top_level_deferred_source_specs_with_abort(
    elements: &mut [Element],
    params: &ParamContext,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    for (index, element) in elements.iter_mut().enumerate() {
        poll_parse_abort(abort, index)?;
        let replacement = match &element.kind {
            ElementKind::VoltageSourceDeferred(raw_spec) => Some(
                resolve_top_level_source_kind(&element.name, raw_spec, params, true)
                    .map_err(ParseWithAbortError::from)?,
            ),
            ElementKind::CurrentSourceDeferred(raw_spec) => Some(
                resolve_top_level_source_kind(&element.name, raw_spec, params, false)
                    .map_err(ParseWithAbortError::from)?,
            ),
            _ => None,
        };

        if let Some(kind) = replacement {
            element.kind = kind;
        }
    }

    ensure_parse_not_aborted(abort)
}

fn resolve_top_level_source_kind(
    element_name: &str,
    raw_spec: &str,
    params: &ParamContext,
    voltage_source: bool,
) -> Result<ElementKind, ParseError> {
    match parse_source_spec_text(raw_spec, 0, params) {
        Ok(spec) if voltage_source => Ok(ElementKind::VoltageSource(spec)),
        Ok(spec) => Ok(ElementKind::CurrentSource(spec)),
        Err(source_error) => {
            let Some(expression) = braced_source_expression(raw_spec) else {
                return Err(top_level_source_resolution_error(
                    element_name,
                    raw_spec,
                    source_error,
                ));
            };
            prepare_behavioral_expression(expression, params).map_err(|_| {
                top_level_source_resolution_error(element_name, raw_spec, source_error)
            })?;
            if voltage_source {
                Ok(ElementKind::BehavioralVoltage {
                    expression: expression.to_string(),
                    tc1: 0.0,
                    tc2: 0.0,
                })
            } else {
                Ok(ElementKind::BehavioralCurrent {
                    expression: expression.to_string(),
                    tc1: 0.0,
                    tc2: 0.0,
                })
            }
        }
    }
}

fn braced_source_expression(raw_spec: &str) -> Option<&str> {
    let trimmed = raw_spec.trim();
    let inner = trimmed.strip_prefix('{')?.strip_suffix('}')?.trim();
    (!inner.is_empty()).then_some(inner)
}

fn top_level_source_resolution_error(
    element_name: &str,
    raw_spec: &str,
    error: ParseError,
) -> ParseError {
    ParseError::InvalidValue(format!(
        "source {element_name} specification '{}' could not be resolved after .PARAM processing: {error}",
        raw_spec.trim()
    ))
}

fn normalize_pspice_u_timing_aliases_with_abort(
    state: &mut ParseState,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    ensure_parse_not_aborted(abort)?;
    let source_models = state.models.clone();
    let mut existing_names: HashSet<String> = state
        .models
        .iter()
        .map(|model| model.name.to_ascii_uppercase())
        .collect();
    let mut generated_models = Vec::new();

    normalize_pspice_u_timing_in_elements_with_abort(
        &mut state.elements,
        "TOP",
        &source_models,
        &mut existing_names,
        &mut generated_models,
        abort,
    )?;
    for (index, subckt) in state.subcircuits.iter_mut().enumerate() {
        poll_parse_abort(abort, index)?;
        normalize_pspice_u_timing_in_subckt_with_abort(
            subckt,
            &source_models,
            &mut existing_names,
            &mut generated_models,
            abort,
        )?;
    }

    state.models.extend(generated_models);
    ensure_parse_not_aborted(abort)
}

fn normalize_pspice_u_timing_in_subckt_with_abort(
    subckt: &mut SubcircuitDef,
    source_models: &[ModelDef],
    existing_names: &mut HashSet<String>,
    generated_models: &mut Vec<ModelDef>,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    normalize_pspice_u_timing_in_elements_with_abort(
        &mut subckt.elements,
        &subckt.name,
        source_models,
        existing_names,
        generated_models,
        abort,
    )?;
    for (index, nested) in subckt.nested_subcircuits.iter_mut().enumerate() {
        poll_parse_abort(abort, index)?;
        normalize_pspice_u_timing_in_subckt_with_abort(
            nested,
            source_models,
            existing_names,
            generated_models,
            abort,
        )?;
    }
    Ok(())
}

fn normalize_pspice_u_timing_in_elements_with_abort(
    elements: &mut [Element],
    scope: &str,
    source_models: &[ModelDef],
    existing_names: &mut HashSet<String>,
    generated_models: &mut Vec<ModelDef>,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    for (index, element) in elements.iter_mut().enumerate() {
        poll_parse_abort(abort, index)?;
        let ElementKind::Xspice {
            model,
            pspice_u_timing,
            ..
        } = &mut element.kind
        else {
            continue;
        };

        let Some(timing) = pspice_u_timing.take() else {
            continue;
        };
        let Some(timing_model) = source_models
            .iter()
            .find(|candidate| candidate.name.eq_ignore_ascii_case(&timing.timing_model))
        else {
            continue;
        };
        if !pspice_u_timing_model_supported(model, timing_model) {
            continue;
        }
        let alias_name = unique_pspice_u_timing_model_name(scope, &element.name, existing_names);
        let alias =
            pspice_u_timing_alias_model(&alias_name, model, timing_model, timing.delay_mode)
                .expect("supported PSpice U timing model should create an alias");

        *model = alias_name;
        generated_models.push(alias);
    }
    Ok(())
}

fn pspice_u_timing_model_supported(code_model: &str, timing_model: &ModelDef) -> bool {
    (timing_model.model_type.eq_ignore_ascii_case("UGATE")
        && pspice_u_gate_model_accepts_ugate_timing(code_model))
        || (timing_model.model_type.eq_ignore_ascii_case("UEFF")
            && pspice_u_edge_ff_model_accepts_ueff_timing(code_model))
        || (timing_model.model_type.eq_ignore_ascii_case("UTGATE")
            && pspice_u_tristate_model_accepts_utgate_timing(code_model))
        || (timing_model.model_type.eq_ignore_ascii_case("UGFF")
            && pspice_u_latch_model_accepts_ugff_timing(code_model))
        || (timing_model.model_type.eq_ignore_ascii_case("UDLY")
            && pspice_u_delay_line_model_accepts_udly_timing(code_model))
}

fn pspice_u_timing_alias_model(
    alias_name: &str,
    code_model: &str,
    timing_model: &ModelDef,
    delay_mode: PspiceUTimingMode,
) -> Option<ModelDef> {
    if timing_model.model_type.eq_ignore_ascii_case("UGATE")
        && pspice_u_gate_model_accepts_ugate_timing(code_model)
    {
        return Some(pspice_ugate_alias_model(
            alias_name,
            code_model,
            timing_model,
            delay_mode,
        ));
    }

    if timing_model.model_type.eq_ignore_ascii_case("UEFF")
        && pspice_u_edge_ff_model_accepts_ueff_timing(code_model)
    {
        return Some(pspice_ueff_alias_model(
            alias_name,
            code_model,
            timing_model,
            delay_mode,
        ));
    }

    if timing_model.model_type.eq_ignore_ascii_case("UTGATE")
        && pspice_u_tristate_model_accepts_utgate_timing(code_model)
    {
        return Some(pspice_utgate_alias_model(
            alias_name,
            code_model,
            timing_model,
            delay_mode,
        ));
    }

    if timing_model.model_type.eq_ignore_ascii_case("UGFF")
        && pspice_u_latch_model_accepts_ugff_timing(code_model)
    {
        return Some(pspice_ugff_alias_model(
            alias_name,
            code_model,
            timing_model,
            delay_mode,
        ));
    }

    if timing_model.model_type.eq_ignore_ascii_case("UDLY")
        && pspice_u_delay_line_model_accepts_udly_timing(code_model)
    {
        return Some(pspice_udly_alias_model(
            alias_name,
            code_model,
            timing_model,
            delay_mode,
        ));
    }

    None
}

fn pspice_u_gate_model_accepts_ugate_timing(model: &str) -> bool {
    matches!(
        model.to_ascii_lowercase().as_str(),
        "d_and" | "d_buffer" | "d_inverter" | "d_nand" | "d_nor" | "d_or" | "d_xnor" | "d_xor"
    )
}

fn pspice_u_edge_ff_model_accepts_ueff_timing(model: &str) -> bool {
    matches!(model.to_ascii_lowercase().as_str(), "d_dff" | "d_jkff")
}

fn pspice_u_tristate_model_accepts_utgate_timing(model: &str) -> bool {
    model.eq_ignore_ascii_case("d_tristate")
}

fn pspice_u_latch_model_accepts_ugff_timing(model: &str) -> bool {
    matches!(
        model.to_ascii_lowercase().as_str(),
        "d_dlatch" | "d_srlatch"
    )
}

fn pspice_u_delay_line_model_accepts_udly_timing(model: &str) -> bool {
    model.eq_ignore_ascii_case("d_buffer")
}

fn pspice_ugate_alias_model(
    alias_name: &str,
    code_model: &str,
    timing_model: &ModelDef,
    delay_mode: PspiceUTimingMode,
) -> ModelDef {
    const PSPICE_U_DEFAULT_DELAY: Value = 1.0e-12;

    let mut params = vec![("inertial_delay".to_string(), 1.0)];
    let mut expr_params = Vec::new();
    push_pspice_timing_delay_param(
        timing_model,
        &["TPLHTY", "TPLHMN", "TPLHMX"],
        "rise_delay",
        PSPICE_U_DEFAULT_DELAY,
        delay_mode,
        &mut params,
        &mut expr_params,
    );
    push_pspice_timing_delay_param(
        timing_model,
        &["TPHLTY", "TPHLMN", "TPHLMX"],
        "fall_delay",
        PSPICE_U_DEFAULT_DELAY,
        delay_mode,
        &mut params,
        &mut expr_params,
    );

    ModelDef {
        name: alias_name.to_string(),
        model_type: code_model.to_string(),
        params,
        expr_params,
        string_params: Vec::new(),
        string_vector_params: Vec::new(),
        real_vector_params: Vec::new(),
        real_vector_expr_params: Vec::new(),
        integer_vector_params: Vec::new(),
    }
}

fn pspice_udly_alias_model(
    alias_name: &str,
    code_model: &str,
    timing_model: &ModelDef,
    delay_mode: PspiceUTimingMode,
) -> ModelDef {
    let mut params = vec![("inertial_delay".to_string(), 0.0)];
    let mut expr_params = Vec::new();
    let delay =
        pspice_timing_delay_estimate(timing_model, &["DLYTY", "DLYMN", "DLYMX"], delay_mode)
            .unwrap_or(PspiceTimingDelay::Numeric(1.0e-12));

    push_pspice_timing_delay("rise_delay", delay.clone(), &mut params, &mut expr_params);
    push_pspice_timing_delay("fall_delay", delay, &mut params, &mut expr_params);

    ModelDef {
        name: alias_name.to_string(),
        model_type: code_model.to_string(),
        params,
        expr_params,
        string_params: Vec::new(),
        string_vector_params: Vec::new(),
        real_vector_params: Vec::new(),
        real_vector_expr_params: Vec::new(),
        integer_vector_params: Vec::new(),
    }
}

fn pspice_utgate_alias_model(
    alias_name: &str,
    code_model: &str,
    timing_model: &ModelDef,
    delay_mode: PspiceUTimingMode,
) -> ModelDef {
    let mut params = vec![("inertial_delay".to_string(), 1.0)];
    let mut expr_params = Vec::new();

    let rising =
        pspice_timing_delay_estimate(timing_model, &["TPLHTY", "TPLHMN", "TPLHMX"], delay_mode);
    let falling =
        pspice_timing_delay_estimate(timing_model, &["TPHLTY", "TPHLMN", "TPHLMX"], delay_mode);
    let delay =
        pspice_select_longer_delay(rising, falling).unwrap_or(PspiceTimingDelay::Numeric(1.0e-12));
    push_pspice_timing_delay("delay", delay, &mut params, &mut expr_params);

    ModelDef {
        name: alias_name.to_string(),
        model_type: code_model.to_string(),
        params,
        expr_params,
        string_params: Vec::new(),
        string_vector_params: Vec::new(),
        real_vector_params: Vec::new(),
        real_vector_expr_params: Vec::new(),
        integer_vector_params: Vec::new(),
    }
}

fn pspice_ugff_alias_model(
    alias_name: &str,
    code_model: &str,
    timing_model: &ModelDef,
    delay_mode: PspiceUTimingMode,
) -> ModelDef {
    let mut params = vec![
        ("rise_delay".to_string(), 1.0e-9),
        ("fall_delay".to_string(), 1.0e-9),
    ];
    let mut expr_params = Vec::new();

    let data_rise = pspice_timing_delay_estimate(
        timing_model,
        &["TPDQLHTY", "TPDQLHMN", "TPDQLHMX"],
        delay_mode,
    );
    let data_fall = pspice_timing_delay_estimate(
        timing_model,
        &["TPDQHLTY", "TPDQHLMN", "TPDQHLMX"],
        delay_mode,
    );
    if let Some(delay) = pspice_select_longer_delay(data_rise, data_fall) {
        let target = if code_model.eq_ignore_ascii_case("d_srlatch") {
            "sr_delay"
        } else {
            "data_delay"
        };
        push_pspice_timing_delay(target, delay, &mut params, &mut expr_params);
    }

    let gate_rise = pspice_timing_delay_estimate(
        timing_model,
        &["TPGQLHTY", "TPGQLHMN", "TPGQLHMX"],
        delay_mode,
    );
    let gate_fall = pspice_timing_delay_estimate(
        timing_model,
        &["TPGQHLTY", "TPGQHLMN", "TPGQHLMX"],
        delay_mode,
    );
    if let Some(delay) = pspice_select_longer_delay(gate_rise, gate_fall) {
        push_pspice_timing_delay("enable_delay", delay, &mut params, &mut expr_params);
    }

    push_pspice_pcq_set_reset_delays(timing_model, delay_mode, &mut params, &mut expr_params);

    ModelDef {
        name: alias_name.to_string(),
        model_type: code_model.to_string(),
        params,
        expr_params,
        string_params: Vec::new(),
        string_vector_params: Vec::new(),
        real_vector_params: Vec::new(),
        real_vector_expr_params: Vec::new(),
        integer_vector_params: Vec::new(),
    }
}

fn pspice_ueff_alias_model(
    alias_name: &str,
    code_model: &str,
    timing_model: &ModelDef,
    delay_mode: PspiceUTimingMode,
) -> ModelDef {
    let mut params = vec![
        ("rise_delay".to_string(), 1.0e-9),
        ("fall_delay".to_string(), 1.0e-9),
    ];
    let mut expr_params = Vec::new();

    let clk_rise = pspice_timing_delay_estimate(
        timing_model,
        &["TPCLKQLHTY", "TPCLKQLHMN", "TPCLKQLHMX"],
        delay_mode,
    );
    let clk_fall = pspice_timing_delay_estimate(
        timing_model,
        &["TPCLKQHLTY", "TPCLKQHLMN", "TPCLKQHLMX"],
        delay_mode,
    );
    if let Some(delay) = pspice_select_longer_delay(clk_rise, clk_fall) {
        push_pspice_timing_delay("clk_delay", delay, &mut params, &mut expr_params);
    }

    push_pspice_pcq_set_reset_delays(timing_model, delay_mode, &mut params, &mut expr_params);

    ModelDef {
        name: alias_name.to_string(),
        model_type: code_model.to_string(),
        params,
        expr_params,
        string_params: Vec::new(),
        string_vector_params: Vec::new(),
        real_vector_params: Vec::new(),
        real_vector_expr_params: Vec::new(),
        integer_vector_params: Vec::new(),
    }
}

fn push_pspice_pcq_set_reset_delays(
    timing_model: &ModelDef,
    delay_mode: PspiceUTimingMode,
    params: &mut Vec<(String, Value)>,
    expr_params: &mut Vec<(String, String)>,
) {
    let set_delay = pspice_timing_delay_estimate(
        timing_model,
        &["TPPCQLHTY", "TPPCQLHMN", "TPPCQLHMX"],
        delay_mode,
    );
    let reset_delay = pspice_timing_delay_estimate(
        timing_model,
        &["TPPCQHLTY", "TPPCQHLMN", "TPPCQHLMX"],
        delay_mode,
    );
    match (set_delay, reset_delay) {
        (Some(set), Some(reset)) => {
            push_pspice_timing_delay("set_delay", set, params, expr_params);
            push_pspice_timing_delay("reset_delay", reset, params, expr_params);
        }
        (Some(delay), None) | (None, Some(delay)) => {
            push_pspice_timing_delay("set_delay", delay.clone(), params, expr_params);
            push_pspice_timing_delay("reset_delay", delay, params, expr_params);
        }
        (None, None) => {}
    }
}

fn push_pspice_timing_delay_param(
    timing_model: &ModelDef,
    source_names: &[&str],
    target_name: &str,
    default_value: Value,
    delay_mode: PspiceUTimingMode,
    params: &mut Vec<(String, Value)>,
    expr_params: &mut Vec<(String, String)>,
) {
    if let Some(delay) = pspice_timing_delay_estimate(timing_model, source_names, delay_mode) {
        push_pspice_timing_delay(target_name, delay, params, expr_params);
    } else {
        params.push((target_name.to_string(), default_value));
    }
}

#[derive(Clone)]
enum PspiceTimingDelay {
    Numeric(Value),
    Expr(String),
}

fn pspice_timing_delay_estimate(
    timing_model: &ModelDef,
    source_names: &[&str],
    delay_mode: PspiceUTimingMode,
) -> Option<PspiceTimingDelay> {
    for source_name in pspice_timing_delay_source_order(source_names, delay_mode) {
        if let Some((_, value)) = timing_model
            .params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(source_name))
        {
            return Some(PspiceTimingDelay::Numeric(*value));
        }

        if let Some((_, expr)) = timing_model
            .expr_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(source_name))
        {
            return Some(PspiceTimingDelay::Expr(expr.clone()));
        }
    }

    None
}

fn pspice_timing_delay_source_order<'a>(
    source_names: &'a [&'a str],
    delay_mode: PspiceUTimingMode,
) -> impl Iterator<Item = &'a str> {
    let order = match delay_mode {
        PspiceUTimingMode::Min => [1usize, 0, 2],
        PspiceUTimingMode::Typ => [0usize, 1, 2],
        PspiceUTimingMode::Max => [2usize, 0, 1],
    };
    order
        .into_iter()
        .filter_map(|index| source_names.get(index).copied())
}

fn pspice_select_longer_delay(
    first: Option<PspiceTimingDelay>,
    second: Option<PspiceTimingDelay>,
) -> Option<PspiceTimingDelay> {
    match (first, second) {
        (Some(PspiceTimingDelay::Numeric(lhs)), Some(PspiceTimingDelay::Numeric(rhs))) => {
            Some(PspiceTimingDelay::Numeric(lhs.max(rhs)))
        }
        (Some(delay), None) | (None, Some(delay)) => Some(delay),
        (Some(delay), Some(_)) => Some(delay),
        (None, None) => None,
    }
}

fn push_pspice_timing_delay(
    target_name: &str,
    delay: PspiceTimingDelay,
    params: &mut Vec<(String, Value)>,
    expr_params: &mut Vec<(String, String)>,
) {
    match delay {
        PspiceTimingDelay::Numeric(value) => params.push((target_name.to_string(), value)),
        PspiceTimingDelay::Expr(expr) => expr_params.push((target_name.to_string(), expr)),
    }
}

fn unique_pspice_u_timing_model_name(
    scope: &str,
    element_name: &str,
    existing_names: &mut HashSet<String>,
) -> String {
    let base = sanitize_pspice_u_generated_model_name(&format!(
        "__RSPICE_PSPICE_U_{scope}_{element_name}"
    ));
    if existing_names.insert(base.to_ascii_uppercase()) {
        return base;
    }

    let mut suffix = 1usize;
    loop {
        let candidate = format!("{base}_{suffix}");
        if existing_names.insert(candidate.to_ascii_uppercase()) {
            return candidate;
        }
        suffix += 1;
    }
}

fn sanitize_pspice_u_generated_model_name(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            out.push(ch.to_ascii_uppercase());
        } else {
            out.push('_');
        }
    }
    out
}

fn validate_resistor_model_references_with_abort(
    state: &ParseState,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    ensure_parse_not_aborted(abort)?;
    let models = state
        .models
        .iter()
        .map(|model| model.name.to_ascii_uppercase())
        .collect::<HashSet<_>>();
    validate_resistor_model_references_in_elements_with_abort(&state.elements, &models, abort)?;
    for (index, subckt) in state.subcircuits.iter().enumerate() {
        poll_parse_abort(abort, index)?;
        validate_resistor_model_references_in_subckt_with_abort(subckt, &models, abort)?;
    }
    ensure_parse_not_aborted(abort)
}

fn validate_resistor_model_references_in_subckt_with_abort(
    subckt: &SubcircuitDef,
    models: &HashSet<String>,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    validate_resistor_model_references_in_elements_with_abort(&subckt.elements, models, abort)?;
    for (index, nested) in subckt.nested_subcircuits.iter().enumerate() {
        poll_parse_abort(abort, index)?;
        validate_resistor_model_references_in_subckt_with_abort(nested, models, abort)?;
    }
    Ok(())
}

fn validate_resistor_model_references_in_elements_with_abort(
    elements: &[Element],
    models: &HashSet<String>,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    for (index, element) in elements.iter().enumerate() {
        poll_parse_abort(abort, index)?;
        let ElementKind::Resistor {
            model: Some(model),
            instance_params,
            ..
        } = &element.kind
        else {
            continue;
        };
        if instance_params.iter().any(|(param, _)| {
            param.eq_ignore_ascii_case(crate::netlist::XYCE_DEFAULT_RESISTOR_VALUE_MARKER)
        }) {
            continue;
        }
        let key = model.to_ascii_uppercase();
        if !models.contains(&key) {
            return Err(ParseError::Syntax {
                line: 0,
                message: format!(
                    "Resistor '{}' references unknown model '{}'",
                    element.name, model
                ),
            }
            .into());
        }
    }
    Ok(())
}

fn is_dot_command_head(head: &str) -> bool {
    head.strip_prefix('.')
        .and_then(|rest| rest.chars().next())
        .is_some_and(|ch| ch.is_ascii_alphabetic())
}

fn prescan_temperature_options_with_abort(
    lines: &[&str],
    state: &mut ParseState,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    let mut continuation = String::new();
    let mut continuation_line = None;
    let mut in_options = false;
    let mut line_num = 1usize;

    for (index, line) in lines.iter().skip(1).enumerate() {
        poll_parse_abort(abort, index)?;
        poll_parse_text(abort, line)?;
        line_num += 1;
        let stripped = strip_inline_semicolon_comment(line);
        let trimmed = stripped.trim();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix('+') {
            if in_options {
                continuation.push(' ');
                continuation.push_str(rest.trim());
            }
            continue;
        }

        if !continuation.is_empty() {
            let logical_line = continuation_line
                .take()
                .expect("non-empty .OPTIONS statement records its physical origin");
            scan_temperature_option_line(&continuation, logical_line, state)
                .map_err(ParseWithAbortError::from)?;
            continuation.clear();
        }

        let head = trimmed.split_whitespace().next().unwrap_or("");
        if head.eq_ignore_ascii_case(".options")
            || head.eq_ignore_ascii_case(".option")
            || head.eq_ignore_ascii_case(".opt")
        {
            in_options = true;
            continuation.push_str(trimmed);
            continuation_line = Some(line_num);
        } else {
            in_options = false;
        }
    }

    if !continuation.is_empty() {
        let logical_line = continuation_line
            .take()
            .expect("non-empty .OPTIONS statement records its physical origin");
        scan_temperature_option_line(&continuation, logical_line, state)
            .map_err(ParseWithAbortError::from)?;
    }

    ensure_parse_not_aborted(abort)
}

fn scan_temperature_option_line(
    line: &str,
    line_num: usize,
    state: &mut ParseState,
) -> Result<(), ParseError> {
    let tokens = tokenize(line).map_err(|err| lex_to_parse_error(err, line_num))?;
    let mut stream = TokenStream::new(tokens);
    stream.advance();

    while !stream.is_eof() {
        skip_commas(&mut stream);
        if matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
            break;
        }

        let TokenKind::Ident(key) = &stream.peek().kind else {
            stream.advance();
            continue;
        };
        let key_upper = key.to_ascii_uppercase();
        stream.advance();

        let has_equals = stream.consume(&TokenKind::Equals);
        if !has_equals && matches!(key_upper.as_str(), "DEVICE" | "TOPOLOGY") {
            continue;
        }
        if !matches!(key_upper.as_str(), "TEMP" | "TNOM") {
            if has_equals && !matches!(stream.peek().kind, TokenKind::Newline | TokenKind::Eof) {
                stream.advance();
            }
            continue;
        }

        let value = expect_value(&mut stream, line_num, &state.params)?;
        let parsed = parse_celsius_option(&key_upper, value, line_num)?;
        match key_upper.as_str() {
            "TEMP" => {
                state.options.temp = Some(parsed);
                state.params.set("TEMP", parsed);
                state.params.set("TEMPER", parsed);
                state.params.set(
                    "VT",
                    crate::constants::thermal_voltage(
                        crate::analysis::temperature::celsius_to_kelvin(parsed),
                    ),
                );
            }
            "TNOM" => {
                state.options.tnom = Some(parsed);
                state.params.set("TNOM", parsed);
            }
            _ => {}
        }
    }

    Ok(())
}

/// Dispatch one logical line through the conditional gate: conditional
/// directives update the block stack, lines inside false branches are
/// skipped, and everything else flows to `process_line`.
fn process_line_gated(
    line: &str,
    line_num: usize,
    origin: &NetlistSourceLocation,
    state: &mut ParseState,
) -> Result<(), ParseError> {
    if let Some(directive) = parse_conditional_directive(line) {
        return state.apply_conditional_directive(directive, line_num);
    }
    if state.conditionals_suppress() {
        return Ok(());
    }
    process_line(line, line_num, origin, state)
}

/// Pre-scan for `.options seed=<n>` (alias `rndseed=<n>`) so the statistical
/// expression functions can be seeded before any parameter evaluation,
/// regardless of where the option appears in the deck.
///
/// Scans `.options` logical lines (including `+` continuations); the last
/// occurrence wins, matching SPICE option-override behavior. `seed=random`
/// is ignored here — `parse_options_command` emits the warning for it.
fn prescan_random_seed_with_abort(
    lines: &[&str],
    abort: &dyn AbortSignal,
) -> Result<Option<u64>, ParseWithAbortError> {
    let mut seed = None;
    let mut in_options = false;
    let mut line_num = 1usize;

    for (index, line) in lines.iter().skip(1).enumerate() {
        poll_parse_abort(abort, index)?;
        poll_parse_text(abort, line)?;
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
            scan_options_tokens_for_seed(rest, line_num, &mut seed)
                .map_err(ParseWithAbortError::from)?;
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
                scan_options_tokens_for_seed(rest, line_num, &mut seed)
                    .map_err(ParseWithAbortError::from)?;
            }
            _ => in_options = false,
        }
    }

    ensure_parse_not_aborted(abort)?;
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

#[cfg(test)]
mod cancellation_tests {
    use super::*;

    #[test]
    fn parser_aborts_after_multiple_internal_deck_polls() {
        let mut source = String::from("parser cancellation fixture\n");
        for index in 0..4_096 {
            source.push_str(&format!("R{index} n{index} 0 1k\n"));
        }
        source.push_str(".end\n");
        let abort = crate::abort_signal::CountingAbort::new(100);

        let result =
            parse_netlist_with_options_and_abort(&source, NetlistParseOptions::default(), &abort);

        assert!(matches!(result, Err(ParseWithAbortError::Aborted)));
        assert!(abort.count() > 100, "parser must poll during deck work");
    }

    #[test]
    fn data_row_tokenization_aborts_inside_one_large_logical_line() {
        let mut builder =
            DataTableBuilder::new(2, ".data sweep value", &crate::abort_signal::NoAbort)
                .expect("fixture .DATA header is valid");
        let row = std::iter::repeat_n("1", 8_192)
            .collect::<Vec<_>>()
            .join(" ");
        let abort = crate::abort_signal::CountingAbort::new(7);

        let result = builder.push_line(3, &row, &ParamContext::new(), &abort);

        assert!(matches!(result, Err(ParseWithAbortError::Aborted)));
        assert!(
            abort.count() > 7,
            "one large .DATA row must poll beyond its outer line boundary"
        );
    }
}

#[cfg(test)]
mod logical_line_origin_tests {
    use super::*;

    fn assert_dc_trailing_token_line(source: &str, expected_line: usize) {
        let error = parse_netlist(source).expect_err("excess .DC argument must be rejected");
        match error {
            ParseError::Syntax { line, message } => {
                assert_eq!(line, expected_line);
                assert_eq!(message, ".DC has unexpected trailing token Number(4.0)");
            }
            other => panic!("expected syntax error, got {other:?}"),
        }
    }

    #[test]
    fn logical_statement_error_keeps_origin_across_blank_lines() {
        assert_dc_trailing_token_line("blank-line origin\n.DC V1 -8.0 -4.0 0.0 4.0\n\n.end\n", 2);
    }

    #[test]
    fn logical_statement_error_keeps_origin_across_comment_lines() {
        assert_dc_trailing_token_line(
            "comment-line origin\n.DC V1 -8.0 -4.0 0.0 4.0\n* intervening comment\n.end\n",
            2,
        );
    }

    #[test]
    fn continued_statement_error_reports_base_statement_line() {
        assert_dc_trailing_token_line(
            "continuation origin\n.DC V1 -8.0 -4.0 0.0\n+ 4.0\n\n* comment\n.end\n",
            2,
        );
    }

    #[test]
    fn eof_flush_keeps_origin_across_blank_and_comment_lines() {
        assert_dc_trailing_token_line(
            "EOF origin\n.DC V1 -8.0 -4.0 0.0 4.0\n\n* trailing comment\n",
            2,
        );
    }

    #[test]
    fn multi_continuation_statement_reports_base_statement_line() {
        assert_dc_trailing_token_line(
            "multi-continuation origin\n.DC V1 -8.0\n+ -4.0\n+ 0.0 4.0\n\n.end\n",
            2,
        );
    }

    #[test]
    fn temperature_prescan_error_reports_options_base_line() {
        let source =
            "temperature origin\n.options noop=1\n+ temp=-274\n\n* intervening comment\n.end\n";
        let error = parse_netlist(source).expect_err("invalid TEMP must be rejected by prescan");
        match error {
            ParseError::Syntax { line, message } => {
                assert_eq!(line, 2);
                assert_eq!(
                    message,
                    "TEMP must be finite and above absolute zero, found -274 C"
                );
            }
            other => panic!("expected syntax error, got {other:?}"),
        }
    }
}

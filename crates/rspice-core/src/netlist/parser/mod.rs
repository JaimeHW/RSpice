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
use super::mutual_inductor::{
    MutualInductorSemanticRecord, validate_mutual_inductor_semantic_records_with_abort,
};
use super::remove_unused::{
    designator_type as removeunused_designator_type,
    filter_elements_with_abort as filter_removeunused_elements_with_abort,
};
use super::xspice_parser;
use super::{
    AnalysisCommand, BjtType, DataTable, DeviceInitialConditionDirective,
    DeviceInitialConditionEntry, DeviceInitialConditionError, DeviceInitialConditionSource,
    Element, ElementKind, ExpressionDialect, FftAnalysis, FftFormat, FftOutput, FftWindow,
    FreqVariation, InitialCondition, JfetType, MesfetType, MissingSubcircuitEndsBoundary,
    MissingSubcircuitEndsError, ModelDef, MonteCarloCommand, MonteCarloDistribution, MosType,
    Netlist, NetlistSourceLocation, NodeSet, OutputDirectiveKind, OutputRequest, ParamContext,
    ParameterRedefinitionPolicy, ParametricValue, ParseDiagnostic, ParseError, ParseWithAbortError,
    PoleZeroAnalysisType, PoleZeroTransferType, PspiceUTiming, PspiceUTimingMode,
    RemoveUnusedDeviceType, RemoveUnusedPolicy, SaveSet, SaveSignal, SensitivityAcSweep,
    SimulationOptions, SourceRfPort, SourceSpec, StartupDiagnosticCode,
    StartupDirectiveDisposition, StartupDirectiveEntry, StartupDirectiveKind,
    StartupDirectiveRecord, StartupDirectiveScope, StatisticalParamMode, StepCommand, StepSweep,
    StepTarget, SubcircuitDef, SwitchState, VerilogAInclude, XyceAddResistorMode,
    XyceAddResistorSpec, XyceAddResistorsPolicy, ensure_parse_not_aborted,
    finish_non_aborting_parse, poll_parse_abort, poll_parse_text,
    validate_startup_directives_with_abort,
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

pub(crate) fn parse_device_initial_condition_record(
    record: &str,
    line_num: usize,
    params: &ParamContext,
    origin: &NetlistSourceLocation,
) -> Result<Vec<DeviceInitialConditionEntry>, ParseError> {
    let tokens = tokenize(record).map_err(|error| lex_to_parse_error(error, line_num))?;
    let mut stream = TokenStream::new(tokens);
    stream.skip_newlines();
    parse_device_initial_condition_entries(&mut stream, line_num, params, origin)
}

pub(crate) fn strip_device_initial_condition_record_comment(record: &str) -> &str {
    line::strip_inline_semicolon_comment(record)
}

fn parse_netlist_impl(
    input: &str,
    options: NetlistParseOptions,
    mut source_schedule: Option<SourceEventSchedule>,
    abort: &dyn AbortSignal,
) -> Result<Netlist, ParseWithAbortError> {
    ensure_parse_not_aborted(abort)?;
    let mut original_lines: Vec<&str> = Vec::new();
    for (index, line) in input.lines().enumerate() {
        poll_parse_abort(abort, index)?;
        original_lines.push(line);
    }

    if original_lines.is_empty() {
        return Ok(Netlist::default());
    }
    let preprocess = prescan_root_preprocess(&original_lines, source_schedule.as_ref(), abort)?;
    let transformed_input = apply_root_preprocessing(
        input,
        &original_lines,
        source_schedule.as_ref(),
        preprocess.replace_ground == Some(true),
        abort,
    )?;
    let parse_input = transformed_input.as_str();
    let lines = parse_input.lines().collect::<Vec<_>>();

    // First line is the title
    let title = lines[0].to_string();
    let mut state = ParseState::new();
    for line in preprocess.replace_ground_extra_lines {
        state.diagnostics.push(ParseDiagnostic::warning(
            line,
            "replaceground-extra-parameters",
            "Additional parameters in .PREPROCESS REPLACEGROUND statement; ignoring them",
        ));
    }
    for line in preprocess.add_resistors_extra_lines {
        state.diagnostics.push(ParseDiagnostic::warning(
            line,
            "addresistors-extra-parameters",
            "Additional parameters in .PREPROCESS ADDRESISTORS statement; ignoring them",
        ));
    }
    state.params.set_statistical_mode(options.statistical_mode);
    state
        .params
        .set_expression_dialect(options.expression_dialect);
    state
        .params
        .set_parameter_redefinition_policy(options.parameter_redefinition_policy);
    state.options.replace_ground = preprocess.replace_ground;
    state.options.remove_unused = preprocess.remove_unused;
    state.options.add_resistors = preprocess.add_resistors;

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
        abort,
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
            abort,
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
                abort,
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
            abort,
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
            abort,
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

    if let Some(policy) = state.options.remove_unused.clone() {
        apply_remove_unused_policy_with_abort(
            &mut state.elements,
            &mut state.subcircuits,
            &policy,
            abort,
        )?;
    }
    normalize_pspice_u_timing_aliases_with_abort(&mut state, abort)?;
    resolve_top_level_deferred_source_specs_with_abort(&mut state.elements, &state.params, abort)?;
    validate_resistor_model_references_with_abort(&state, abort)?;

    ensure_parse_not_aborted(abort)?;
    state.into_netlist(
        title,
        input,
        root_eof.unwrap_or_else(|| NetlistSourceLocation::in_memory(lines.len() + 1)),
        abort,
    )
}

fn apply_root_preprocessing(
    input: &str,
    lines: &[&str],
    source_schedule: Option<&SourceEventSchedule>,
    replace_ground: bool,
    abort: &dyn AbortSignal,
) -> Result<String, ParseWithAbortError> {
    let root_path = source_schedule
        .and_then(|schedule| schedule.origin(0))
        .and_then(|origin| origin.path.as_deref());
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum LogicalLinePolicy {
        Rewrite,
        ProtectedDirective,
        AuthoredOutput,
        InertIncludedPreprocess,
    }

    fn logical_line_policy(line: &str, is_root: bool) -> LogicalLinePolicy {
        let head = strip_inline_semicolon_comment(line)
            .split_whitespace()
            .next()
            .unwrap_or_default();
        if !is_root && head.eq_ignore_ascii_case(".PREPROCESS") {
            return LogicalLinePolicy::InertIncludedPreprocess;
        }
        if matches!(
            head.to_ascii_uppercase().as_str(),
            ".SUBCKT" | ".INCLUDE" | ".INC" | ".INCL" | ".LIB" | ".ENDL"
        ) {
            return LogicalLinePolicy::ProtectedDirective;
        }
        // These cards are intentionally parsed from authored spelling. Their
        // execution-facing SaveSet/MEASURE/FOUR fields are normalized after
        // parsing, while OutputRequest retains exact user provenance.
        if matches!(
            head.to_ascii_uppercase().as_str(),
            ".SAVE" | ".PROBE" | ".PRINT" | ".PLOT" | ".MEAS" | ".MEASURE" | ".FOUR" | ".FOURIER"
        ) {
            return LogicalLinePolicy::AuthoredOutput;
        }
        LogicalLinePolicy::Rewrite
    }

    let mut transformed = String::with_capacity(input.len());
    let mut active_source_path: Option<Option<&std::path::Path>> = None;
    let mut logical_policy = LogicalLinePolicy::Rewrite;
    for (index, line) in lines.iter().enumerate() {
        poll_parse_abort(abort, index)?;
        poll_parse_text(abort, line)?;
        let source_path = source_schedule
            .and_then(|schedule| schedule.origin(index))
            .and_then(|origin| origin.path.as_deref());
        let is_root = source_schedule.is_none_or(|_| source_path == root_path);
        if active_source_path != Some(source_path) {
            active_source_path = Some(source_path);
            logical_policy = LogicalLinePolicy::Rewrite;
        }

        let stripped = strip_inline_semicolon_comment(line).trim();
        let is_comment_or_blank = stripped.is_empty() || stripped.starts_with('*');
        let is_continuation = !is_comment_or_blank && stripped.starts_with('+');
        let is_indented_preprocess_comment = line.starts_with([' ', '\t'])
            && !is_continuation
            && stripped
                .split_whitespace()
                .next()
                .is_some_and(|head| head.eq_ignore_ascii_case(".PREPROCESS"));
        if !is_comment_or_blank && !is_continuation && !is_indented_preprocess_comment {
            logical_policy = logical_line_policy(line, is_root);
        }
        if index == 0 {
            logical_policy = LogicalLinePolicy::Rewrite;
        }

        if is_indented_preprocess_comment
            || logical_policy == LogicalLinePolicy::InertIncludedPreprocess
        {
            // Root preprocessing controls are selected before include
            // expansion in Xyce. Drop the entire included logical card so it
            // cannot be parsed later as an active root command.
        } else if replace_ground
            && index != 0
            && logical_policy == LogicalLinePolicy::Rewrite
            && !is_comment_or_blank
        {
            transformed.push_str(&replace_ground_fields_with_abort(line, abort)?);
        } else {
            transformed.push_str(line);
        }
        if index + 1 < lines.len() || input.ends_with('\n') || input.ends_with('\r') {
            transformed.push('\n');
        }
    }
    ensure_parse_not_aborted(abort)?;
    Ok(transformed)
}

#[cfg(test)]
fn replace_ground_fields(line: &str) -> String {
    replace_ground_fields_with_abort(line, &NoAbort)
        .expect("NoAbort cannot cancel ground-field rewriting")
}

fn replace_ground_fields_with_abort(
    line: &str,
    abort: &dyn AbortSignal,
) -> Result<String, ParseWithAbortError> {
    let trimmed = line.trim_start();
    if trimmed.is_empty() || trimmed.starts_with('*') {
        return Ok(line.to_string());
    }

    let mut output = String::with_capacity(line.len());
    let mut cursor = 0usize;

    // Xyce turns a leading continuation marker into whitespace before field
    // tokenization. Preserve it for RSpice's logical-line parser, but do not
    // let it become part of the first field.
    let leading = line.len() - trimmed.len();
    if trimmed.starts_with('+') {
        let marker_end = leading + 1;
        output.push_str(&line[..marker_end]);
        cursor = marker_end;
    }

    while cursor < line.len() {
        poll_parse_abort(abort, cursor)?;
        let character = line[cursor..]
            .chars()
            .next()
            .expect("cursor remains on a character boundary");
        if character == ';' {
            output.push_str(&line[cursor..]);
            break;
        }
        if character == '{' {
            let start = cursor;
            let mut depth = 0usize;
            while cursor < line.len() {
                poll_parse_abort(abort, cursor)?;
                let current = line[cursor..]
                    .chars()
                    .next()
                    .expect("cursor remains on a character boundary");
                cursor += current.len_utf8();
                if current == '{' {
                    depth += 1;
                } else if current == '}' {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        break;
                    }
                }
            }
            output.push_str(&line[start..cursor]);
            continue;
        }
        if matches!(character, '\'' | '"') {
            let quote = character;
            let start = cursor;
            cursor += character.len_utf8();
            while cursor < line.len() {
                poll_parse_abort(abort, cursor)?;
                let current = line[cursor..]
                    .chars()
                    .next()
                    .expect("cursor remains on a character boundary");
                cursor += current.len_utf8();
                if current == quote {
                    break;
                }
            }
            output.push_str(&line[start..cursor]);
            continue;
        }
        if character.is_whitespace() || matches!(character, '(' | ')' | '}' | ',' | '=') {
            output.push(character);
            cursor += character.len_utf8();
            continue;
        }

        let field_start = cursor;
        while cursor < line.len() {
            poll_parse_abort(abort, cursor)?;
            let current = line[cursor..]
                .chars()
                .next()
                .expect("cursor remains on a character boundary");
            if current == ';'
                || current.is_whitespace()
                || matches!(current, '(' | ')' | '{' | '}' | ',' | '=' | '\'')
            {
                break;
            }
            cursor += current.len_utf8();
        }
        let field = &line[field_start..cursor];
        if matches!(
            field.to_ascii_uppercase().as_str(),
            "GND" | "GND!" | "GROUND"
        ) {
            output.push('0');
        } else {
            output.push_str(field);
        }
    }
    ensure_parse_not_aborted(abort)?;
    Ok(output)
}

#[cfg(test)]
mod replaceground_lexical_tests {
    use super::{NetlistParseOptions, apply_root_preprocessing, replace_ground_fields};
    use crate::abort_signal::NoAbort;
    use crate::netlist::{ExpressionDialect, Netlist, ParseError, ParseWithAbortError, SaveSignal};

    fn temporary_include_deck(name: &str) -> (std::path::PathBuf, std::path::PathBuf) {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "rspice-replaceground-{name}-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(&directory).expect("create test directory");
        (directory.join("root.cir"), directory.join("child.inc"))
    }

    #[test]
    fn exact_fields_replace_while_protected_source_text_is_preserved() {
        assert_eq!(
            replace_ground_fields("R1 GND GND! 1 ; GROUND remains a comment"),
            "R1 0 0 1 ; GROUND remains a comment"
        );
        assert_eq!(
            replace_ground_fields("B1 out 0 V={V(GND)+V(GROUND)} label='GND'"),
            "B1 out 0 V={V(GND)+V(GROUND)} label='GND'"
        );
        assert_eq!(
            replace_ground_fields("E1 out 0 VALUE={V(X1:GND)}"),
            "E1 out 0 VALUE={V(X1:GND)}"
        );
        for near_miss in [
            "GND+X", "GND-X", "GND/X", "X:GND", "X.GND", "GND?", "GROUND_1",
        ] {
            let source = format!("R1 out {near_miss} 1");
            assert_eq!(replace_ground_fields(&source), source);
        }
        assert_eq!(
            replace_ground_fields("E1 out 0 GND,GROUND 1"),
            "E1 out 0 0,0 1"
        );
    }

    #[test]
    fn upstream_protected_directive_tails_are_protected_across_continuations() {
        for directive in [".SUBCKT CELL", ".INCLUDE", ".INC", ".INCL", ".LIB", ".ENDL"] {
            let source = format!(
                "protected directive\n.PREPROCESS REPLACEGROUND TRUE\n{directive}\n+ GND GROUND\n"
            );
            let lines = source.lines().collect::<Vec<_>>();
            let transformed = apply_root_preprocessing(&source, &lines, None, true, &NoAbort)
                .expect("NoAbort cannot cancel preprocessing");
            assert!(
                transformed.contains("+ GND GROUND"),
                "directive continuation changed for {directive}: {transformed}"
            );
        }
    }

    #[test]
    fn logical_preprocess_card_selects_policy_and_exact_fields_only() {
        let netlist = Netlist::parse_with_options(
            "logical preprocess\n\
             .PREPROCESS\n\
             + REPLACEGROUND\n\
             + TRUE\n\
             R1 out GROUND_1 1\n\
             R2 out GND 1\n\
             .END\n",
            NetlistParseOptions {
                expression_dialect: ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("continued root preprocessing card is valid");

        assert_eq!(netlist.options.replace_ground, Some(true));
        assert_eq!(netlist.elements[0].nodes, ["OUT", "GROUND_1"]);
        assert_eq!(netlist.elements[1].nodes, ["OUT", "0"]);
    }

    #[test]
    fn title_text_is_inert_during_root_prescan() {
        let netlist = Netlist::parse_with_options(
            ".PREPROCESS REPLACEGROUND TRUE\nR1 out GND! 1\n.END\n",
            NetlistParseOptions {
                expression_dialect: ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("title is not a preprocessing card");
        assert_eq!(netlist.options.replace_ground, None);
        assert_eq!(netlist.elements[0].nodes, ["OUT", "GND!"]);

        let netlist = Netlist::parse_with_options(
            "indented preprocessing comment\n  .PREPROCESS REPLACEGROUND MAYBE\nR1 out GND! 1\n.END\n",
            NetlistParseOptions {
                expression_dialect: ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("an indented new preprocessing card is an inert scan comment");
        assert_eq!(netlist.options.replace_ground, None);
        assert_eq!(netlist.elements[0].nodes, ["OUT", "GND!"]);
    }

    #[test]
    fn whitespace_prefixed_plus_continues_column_one_preprocess_card() {
        let netlist = Netlist::parse_with_options(
            "continued preprocessing\n.PREPROCESS\n   + REPLACEGROUND TRUE\nR1 out GND! 1\n.END\n",
            NetlistParseOptions {
                expression_dialect: ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("whitespace before a continuation marker is accepted");
        assert_eq!(netlist.options.replace_ground, Some(true));
        assert_eq!(netlist.elements[0].nodes, ["OUT", "0"]);
    }

    #[test]
    fn output_continuations_keep_authored_provenance_but_normalize_execution() {
        let source = "authored output\n\
                      V1 out 0 1\n\
                      .PRINT OP V(out)\n\
                      + V(GND!)\n\
                      .OP\n\
                      .PREPROCESS REPLACEGROUND TRUE\n\
                      .END\n";
        let netlist = Netlist::parse_with_options(
            source,
            NetlistParseOptions {
                expression_dialect: ExpressionDialect::Xyce,
                ..Default::default()
            },
        )
        .expect("continued output card parses");

        assert!(netlist.output_requests.iter().any(|request| {
            request
                .dependencies
                .iter()
                .any(|dependency| dependency.symbol == "GND!")
        }));
        assert!(
            netlist
                .saves
                .signals
                .iter()
                .any(|signal| matches!(signal, SaveSignal::Voltage(node) if node == "0"))
        );
    }

    #[test]
    fn root_replaceground_transforms_included_content_and_ignores_child_control() {
        let (root, child) = temporary_include_deck("root-controls-child");
        std::fs::write(
            &child,
            ".PREPROCESS\n+ REPLACEGROUND MAYBE\nR1 out GND! 1k\n",
        )
        .expect("write included deck");
        let source = "root controls preprocessing\n.include child.inc\n.PRINT OP V(out)\n.OP\n.END\n.PREPROCESS REPLACEGROUND TRUE\n";
        let netlist = Netlist::parse_with_path(source, &root)
            .expect("included invalid control is inert and root TRUE applies");
        std::fs::remove_dir_all(root.parent().expect("root has parent"))
            .expect("remove test directory");

        assert_eq!(netlist.elements[0].nodes, ["OUT", "0"]);
        assert_eq!(netlist.options.replace_ground, Some(true));
    }

    #[test]
    fn included_replaceground_cannot_enable_root_preprocessing() {
        let (root, child) = temporary_include_deck("child-cannot-enable");
        std::fs::write(&child, ".PREPROCESS REPLACEGROUND TRUE\nR1 out GROUND 1k\n")
            .expect("write included deck");
        let source = "child control is inert\n.include child.inc\n.END\n";
        let netlist = Netlist::parse_with_path(source, &root)
            .expect("included control is ignored without changing its ordinary node");
        std::fs::remove_dir_all(root.parent().expect("root has parent"))
            .expect("remove test directory");

        assert_eq!(netlist.elements[0].nodes, ["OUT", "GROUND"]);
        assert_eq!(netlist.options.replace_ground, None);
    }

    #[test]
    fn root_replaceground_is_validated_through_physical_eof() {
        for source in [
            "missing value\nR1 1 0 1k\n.END\n.PREPROCESS REPLACEGROUND\n",
            "unknown value\nR1 1 0 1k\n.END\n.PREPROCESS REPLACEGROUND MAYBE\n",
            "duplicate\n.PREPROCESS REPLACEGROUND FALSE\nR1 1 0 1k\n.END\n.PREPROCESS REPLACEGROUND TRUE\n",
        ] {
            assert!(
                Netlist::parse(source).is_err(),
                "invalid root control must fail even after .END: {source}"
            );
        }

        let netlist = Netlist::parse(
            "extra fields\nR1 1 0 1k\n.END\n.PREPROCESS REPLACEGROUND FALSE ignored\n",
        )
        .expect("extra fields are a warning, including after END");
        assert!(netlist.diagnostics.iter().any(|diagnostic| {
            diagnostic.code == "replaceground-extra-parameters" && diagnostic.line == 4
        }));
    }

    #[test]
    fn included_lines_do_not_perturb_root_physical_diagnostic_origin() {
        let (root, child) = temporary_include_deck("root-origin");
        std::fs::write(&child, "R1 1 0 1\nR2 2 0 2\nR3 3 0 3\n").expect("write included deck");
        let source = "physical origin\n.include child.inc\nR4 4 0 4\n.PREPROCESS\n+ REPLACEGROUND MAYBE\n.END\n";
        let error = Netlist::parse_with_path(source, &root)
            .expect_err("invalid root value must retain the root card origin");
        std::fs::remove_dir_all(root.parent().expect("root has parent"))
            .expect("remove test directory");

        assert!(matches!(
            error,
            ParseError::Syntax { line: 4, message }
                if message.contains("Unknown argument MAYBE")
        ));
    }

    #[test]
    fn ground_preprocessing_aborts_inside_one_large_logical_field() {
        let near_miss = format!("GND+{}", "X".repeat(16_384));
        let source =
            format!("abort fixture\n.PREPROCESS REPLACEGROUND TRUE\nR1 out {near_miss} 1\n.END\n");
        let abort = crate::abort_signal::CountingAbort::new(256);
        let result = super::parse_netlist_with_options_and_abort(
            &source,
            NetlistParseOptions {
                expression_dialect: ExpressionDialect::Xyce,
                ..Default::default()
            },
            &abort,
        );
        assert!(matches!(result, Err(ParseWithAbortError::Aborted)));
        assert!(abort.count() > 256);
    }
}

#[cfg(test)]
mod removeunused_tests {
    use super::{NetlistParseOptions, apply_remove_unused_policy_with_abort};
    use crate::netlist::{ExpressionDialect, Netlist, ParseError, RemoveUnusedDeviceType};

    fn xyce_options() -> NetlistParseOptions {
        NetlistParseOptions {
            expression_dialect: ExpressionDialect::Xyce,
            ..Default::default()
        }
    }

    fn element_names(netlist: &Netlist) -> Vec<&str> {
        netlist
            .elements
            .iter()
            .map(|element| element.name.as_str())
            .collect()
    }

    fn temporary_include_deck(name: &str) -> (std::path::PathBuf, std::path::PathBuf) {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos();
        let directory = std::env::temp_dir().join(format!(
            "rspice-removeunused-{name}-{}-{unique}",
            std::process::id()
        ));
        std::fs::create_dir_all(&directory).expect("create test directory");
        (directory.join("root.cir"), directory.join("child.inc"))
    }

    #[test]
    fn selected_two_terminal_families_are_removed_without_false_removal() {
        let netlist = Netlist::parse_with_options(
            "two terminal removal\n\
             .PREPROCESS REMOVEUNUSED C,D,I,L,R,V\n\
             R_DROP same SAME 1\n\
             R_KEEP same other 1\n\
             C_DROP same same 1p\n\
             C_KEEP same other 1p\n\
             D_DROP same same DM\n\
             D_KEEP same other DM\n\
             I_DROP same same 1\n\
             I_KEEP same other 1\n\
             L_DROP same same 1u\n\
             L_KEEP same other 1u\n\
             V_DROP same same 1\n\
             V_KEEP same other 1\n\
             .MODEL DM D\n\
             .END\n",
            xyce_options(),
        )
        .expect("all selected two-terminal families parse");

        assert_eq!(
            element_names(&netlist),
            ["R_KEEP", "C_KEEP", "D_KEEP", "I_KEEP", "L_KEEP", "V_KEEP"]
        );
        let policy = netlist
            .options
            .remove_unused
            .as_ref()
            .expect("typed policy is retained");
        for device_type in [
            RemoveUnusedDeviceType::Capacitor,
            RemoveUnusedDeviceType::Diode,
            RemoveUnusedDeviceType::CurrentSource,
            RemoveUnusedDeviceType::Inductor,
            RemoveUnusedDeviceType::Resistor,
            RemoveUnusedDeviceType::VoltageSource,
        ] {
            assert!(policy.contains(device_type));
        }
    }

    #[test]
    fn mosfet_and_bjt_compare_exactly_the_first_three_nodes() {
        let netlist = Netlist::parse_with_options(
            "three terminal removal\n\
             .PREPROCESS REMOVEUNUSED M Q\n\
             M_DROP n N n bulk NM\n\
             M_KEEP12 n n other bulk NM\n\
             M_KEEP13 n other n bulk NM\n\
             Q_DROP q Q q QM\n\
             Q_KEEP12 q q other QM\n\
             Q_KEEP13 q other q QM\n\
             .MODEL NM NMOS LEVEL=1\n\
             .MODEL QM NPN\n\
             .END\n",
            xyce_options(),
        )
        .expect("M/Q fixtures parse");

        assert_eq!(
            element_names(&netlist),
            ["M_KEEP12", "M_KEEP13", "Q_KEEP12", "Q_KEEP13"]
        );
    }

    #[test]
    fn unselected_and_unsupported_designators_are_never_removed() {
        let netlist = Netlist::parse_with_options(
            "selective removal\n\
             .PREPROCESS REMOVEUNUSED C\n\
             R1 same same 1\n\
             E1 same same same same 1\n\
             C1 same other 1p\n\
             .END\n",
            xyce_options(),
        )
        .expect("unselected devices remain valid");
        assert_eq!(element_names(&netlist), ["R1", "E1", "C1"]);

        let mut synthesized = Netlist::parse_with_options(
            "typed helper\nR_AUTHORED same same 1\n.END\n",
            xyce_options(),
        )
        .expect("typed helper fixture parses");
        synthesized.elements[0].name = "E_SYNTHESIZED_RESISTOR".to_string();
        let mut policy = crate::netlist::RemoveUnusedPolicy::default();
        policy.device_types.insert(RemoveUnusedDeviceType::Resistor);
        apply_remove_unused_policy_with_abort(
            &mut synthesized.elements,
            &mut synthesized.subcircuits,
            &policy,
            &crate::abort_signal::NoAbort,
        )
        .expect("typed helper filtering succeeds");
        assert_eq!(element_names(&synthesized), ["E_SYNTHESIZED_RESISTOR"]);
    }

    #[test]
    fn removal_applies_to_every_subcircuit_definition_depth() {
        let mut netlist = Netlist::parse_with_options(
            "nested removal\n\
             .SUBCKT OUTER a b\n\
             R_OUT_DROP a a 1\n\
             R_OUT_KEEP a b 1\n\
             .ENDS OUTER\n\
             .SUBCKT INNER x y\n\
             R_IN_DROP x x 1\n\
             R_IN_KEEP x y 1\n\
             .ENDS INNER\n\
             .END\n",
            xyce_options(),
        )
        .expect("subcircuit fixtures parse");
        let inner = netlist.subcircuits.remove(1);
        netlist.subcircuits[0].nested_subcircuits.push(inner);
        let mut policy = crate::netlist::RemoveUnusedPolicy::default();
        policy.device_types.insert(RemoveUnusedDeviceType::Resistor);
        apply_remove_unused_policy_with_abort(
            &mut netlist.elements,
            &mut netlist.subcircuits,
            &policy,
            &crate::abort_signal::NoAbort,
        )
        .expect("recursive filtering succeeds");
        let outer = &netlist.subcircuits[0];
        assert_eq!(
            outer
                .elements
                .iter()
                .map(|element| element.name.as_str())
                .collect::<Vec<_>>(),
            ["R_OUT_KEEP"]
        );
        let inner = &outer.nested_subcircuits[0];
        assert_eq!(
            inner
                .elements
                .iter()
                .map(|element| element.name.as_str())
                .collect::<Vec<_>>(),
            ["R_IN_KEEP"]
        );
    }

    #[test]
    fn continuation_commas_repetition_and_after_end_are_supported() {
        let netlist = Netlist::parse_with_options(
            "continued after end\n\
             R1 same same 1\n\
             C1 same same 1p\n\
             .END\n\
             .PREPROCESS\n\
             + REMOVEUNUSED\n\
             + R, R, c\n",
            xyce_options(),
        )
        .expect("continued control after END applies");
        assert!(netlist.elements.is_empty());

        Netlist::parse_with_options(
            "recognized after end\nR1 1 0 1\n.END\n.PREPROCESS ADDRESISTORS ONETERMINAL 1e12\n",
            xyce_options(),
        )
        .expect("recognized ADDRESISTORS remains non-fatal after END");
    }

    #[test]
    fn replaceground_runs_before_redundancy_comparison_in_either_card_order() {
        for controls in [
            ".PREPROCESS REPLACEGROUND TRUE\n.PREPROCESS REMOVEUNUSED R",
            ".PREPROCESS REMOVEUNUSED R\n.PREPROCESS REPLACEGROUND TRUE",
        ] {
            let source =
                format!("ground ordering\n{controls}\nR_DROP GND 0 1\nR_KEEP GND other 1\n.END\n");
            let netlist = Netlist::parse_with_options(&source, xyce_options())
                .expect("both preprocessing card orders are valid");
            assert_eq!(element_names(&netlist), ["R_KEEP"]);
        }
    }

    #[test]
    fn title_and_indented_controls_are_inert() {
        for source in [
            ".PREPROCESS REMOVEUNUSED R\nR1 same same 1\n.END\n",
            "indented control\n  .PREPROCESS REMOVEUNUSED R\nR1 same same 1\n.END\n",
            "indented invalid control\n\t.PREPROCESS REMOVEUNUSED BOGUS\nR1 same same 1\n.END\n",
        ] {
            let netlist = Netlist::parse_with_options(source, xyce_options())
                .expect("inert title/indented control cannot select or diagnose policy");
            assert_eq!(element_names(&netlist), ["R1"]);
            assert!(netlist.options.remove_unused.is_none());
        }
    }

    #[test]
    fn included_controls_are_inert_but_root_policy_applies_to_included_devices() {
        let (root, child) = temporary_include_deck("included-control");
        std::fs::write(
            &child,
            ".PREPROCESS REMOVEUNUSED BOGUS\nR_CHILD same same 1\nC_CHILD same same 1p\n",
        )
        .expect("write included deck");
        let source = "root policy\n.include child.inc\n.PREPROCESS REMOVEUNUSED C\n.END\n";
        let netlist = Netlist::parse_with_path_and_options(source, &root, xyce_options())
            .expect("child control is inert and root policy is global");
        std::fs::remove_dir_all(root.parent().expect("root has parent"))
            .expect("remove test directory");

        assert_eq!(element_names(&netlist), ["R_CHILD"]);
    }

    #[test]
    fn missing_unknown_and_duplicate_cards_are_fatal_at_physical_origin() {
        for (source, line, expected) in [
            (
                "missing\n.PREPROCESS REMOVEUNUSED\n.END\n",
                2,
                "No remove parameters specified",
            ),
            (
                "commas only\n.PREPROCESS REMOVEUNUSED , ,\n.END\n",
                2,
                "No remove parameters specified",
            ),
            (
                "unknown\n.PREPROCESS REMOVEUNUSED RR\n.END\n",
                2,
                "Unknown argument type RR",
            ),
            (
                "duplicate\n.PREPROCESS REMOVEUNUSED R\nR1 1 0 1\n.END\n.PREPROCESS REMOVEUNUSED C\n",
                5,
                "Multiple .PREPROCESS REMOVEUNUSED",
            ),
            (
                "bare after end\nR1 1 0 1\n.END\n.PREPROCESS\n",
                4,
                ".PREPROCESS requires an operation",
            ),
            (
                "unknown operation after end\nR1 1 0 1\n.END\n.PREPROCESS BOGUS X\n",
                4,
                "Unknown .PREPROCESS operation 'BOGUS'",
            ),
        ] {
            let error = Netlist::parse_with_options(source, xyce_options())
                .expect_err("invalid REMOVEUNUSED card must fail");
            assert!(matches!(
                error,
                ParseError::Syntax { line: actual, message }
                    if actual == line && message.contains(expected)
            ));
        }
    }

    #[test]
    fn included_expansion_does_not_shift_invalid_root_card_origin() {
        let (root, child) = temporary_include_deck("physical-origin");
        std::fs::write(&child, "R1 1 0 1\nR2 2 0 2\nR3 3 0 3\n").expect("write included deck");
        let source =
            "root origin\n.include child.inc\nR4 4 0 4\n.PREPROCESS REMOVEUNUSED X\n.END\n";
        let error = Netlist::parse_with_path_and_options(source, &root, xyce_options())
            .expect_err("invalid root selector must fail");
        std::fs::remove_dir_all(root.parent().expect("root has parent"))
            .expect("remove test directory");
        assert!(matches!(
            error,
            ParseError::Syntax { line: 4, message }
                if message.contains("Unknown argument type X")
        ));
    }

    #[test]
    fn removal_is_transactional_when_cancellation_arrives() {
        let mut netlist = Netlist::parse_with_options(
            "abort removal\n.PREPROCESS REMOVEUNUSED R\nR1 same same 1\nR2 same same 2\n.END\n",
            xyce_options(),
        )
        .expect("fixture parses");
        // Reconstitute authored elements to exercise the private filtering
        // transaction directly; ordinary parsing already applied the policy.
        let authored = Netlist::parse_with_options(
            "authored fixture\nR1 same same 1\nR2 same same 2\n.END\n",
            xyce_options(),
        )
        .expect("authored fixture parses");
        netlist.elements = authored.elements;
        let before = element_names(&netlist)
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        let policy = netlist
            .options
            .remove_unused
            .clone()
            .expect("policy exists");
        let abort = crate::abort_signal::CountingAbort::new(1);
        let result = apply_remove_unused_policy_with_abort(
            &mut netlist.elements,
            &mut netlist.subcircuits,
            &policy,
            &abort,
        );
        assert!(matches!(
            result,
            Err(crate::netlist::ParseWithAbortError::Aborted)
        ));
        assert_eq!(
            element_names(&netlist)
                .into_iter()
                .map(str::to_string)
                .collect::<Vec<_>>(),
            before
        );
    }

    #[test]
    fn selected_redundant_names_never_enter_duplicate_registry() {
        for device_lines in [
            "R1 same same 1\nR1 same live 2",
            "R1 same live 2\nR1 same same 1",
        ] {
            let source = format!(
                "top duplicate ordering\n.PREPROCESS REMOVEUNUSED R\n{device_lines}\n.END\n"
            );
            let netlist = Netlist::parse_with_options(&source, xyce_options())
                .expect("the selected redundant duplicate is never registered");
            assert_eq!(element_names(&netlist), ["R1"]);
            assert_eq!(netlist.elements[0].nodes, ["SAME", "LIVE"]);
        }

        for device_lines in ["R1 a a 1\nR1 a b 2", "R1 a b 2\nR1 a a 1"] {
            let source = format!(
                "subcircuit duplicate ordering\n.PREPROCESS REMOVEUNUSED R\n.SUBCKT CELL a b\n{device_lines}\n.ENDS\n.END\n"
            );
            let netlist = Netlist::parse_with_options(&source, xyce_options())
                .expect("subcircuit redundant duplicate is never registered");
            assert_eq!(netlist.subcircuits[0].elements.len(), 1);
            assert_eq!(netlist.subcircuits[0].elements[0].name, "R1");
            assert_eq!(netlist.subcircuits[0].elements[0].nodes, ["A", "B"]);
        }
    }

    #[test]
    fn selected_redundant_card_is_skipped_before_tail_parsing_or_synthesis() {
        let netlist = Netlist::parse_with_options(
            "lexical rejection\n\
             .PREPROCESS REMOVEUNUSED R\n\
             R_MALFORMED same same ) this tail is never parsed\n\
             R_PARASITIC same same 1 RPAR=2 CPAR=3 RSER=4\n\
             .END\n",
            xyce_options(),
        )
        .expect("selected redundant cards are rejected before parsing their tails");
        assert!(netlist.elements.is_empty());

        let error = Netlist::parse_with_options(
            "unselected malformed\n.PREPROCESS REMOVEUNUSED C\nR1 same same )\n.END\n",
            xyce_options(),
        )
        .expect_err("an unselected malformed resistor still reaches its parser");
        assert!(matches!(error, ParseError::Syntax { .. }));
    }

    #[test]
    fn generated_passive_helpers_are_never_independently_rejected() {
        let mut netlist = Netlist::parse_with_options(
            "generated parasitic\nR1 a b 1 RPAR=2\n.END\n",
            xyce_options(),
        )
        .expect("passive parasitic fixture parses");
        let helper = netlist
            .elements
            .iter_mut()
            .find(|element| element.name.ends_with("#PAR"))
            .expect("RPAR helper was synthesized");
        helper.nodes[1] = helper.nodes[0].clone();

        let mut policy = crate::netlist::RemoveUnusedPolicy::default();
        policy.device_types.insert(RemoveUnusedDeviceType::Resistor);
        let filtered = super::filter_removeunused_elements_with_abort(
            &netlist.elements,
            &policy,
            &crate::abort_signal::NoAbort,
        )
        .expect("post-flatten defense filters transactionally");
        assert_eq!(filtered.len(), 2);
        assert!(
            filtered
                .iter()
                .any(|element| element.name.ends_with("#PAR"))
        );
    }

    #[test]
    fn unselected_redundant_name_still_participates_in_duplicate_registry() {
        let error = Netlist::parse_with_options(
            "unselected duplicate\n.PREPROCESS REMOVEUNUSED C\nR1 same same 1\nR1 same live 2\n.END\n",
            xyce_options(),
        )
        .expect_err("an unselected resistor remains a real duplicate");
        assert!(matches!(error, ParseError::DuplicateName { .. }));
    }

    #[test]
    fn post_flatten_filter_rechecks_resolved_subcircuit_actual_nodes() {
        let netlist = Netlist::parse_with_options(
            "resolved actual equality\n\
             .PREPROCESS REMOVEUNUSED R\n\
             .SUBCKT CELL a b\n\
             R_BODY a b 1\n\
             .ENDS CELL\n\
             X1 n n CELL\n\
             .END\n",
            xyce_options(),
        )
        .expect("subcircuit fixture parses");
        assert_eq!(netlist.subcircuits[0].elements.len(), 1);

        let flattened = crate::netlist::flatten_netlist_with_models(&netlist)
            .expect("subcircuit fixture flattens");
        assert!(flattened.elements.is_empty());

        let circuit = crate::Engine::default()
            .build_circuit(&netlist)
            .expect("circuit build omits the newly redundant flattened resistor");
        assert!(circuit.get_node_by_name("n").is_none());
    }

    #[test]
    fn post_flatten_filter_removes_resolved_passive_and_its_synthesized_batch() {
        let netlist = Netlist::parse_with_options(
            "resolved parasitic batch equality\n\
             .PREPROCESS REMOVEUNUSED R\n\
             .SUBCKT CELL a b\n\
             R_BODY a b 1 RSER=2 RPAR=3 CPAR=4\n\
             .ENDS CELL\n\
             X1 n n CELL\n\
             .END\n",
            xyce_options(),
        )
        .expect("passive parasitic subcircuit fixture parses");
        assert_eq!(netlist.subcircuits[0].elements.len(), 4);

        let flattened = crate::netlist::flatten_netlist_with_models(&netlist)
            .expect("passive parasitic subcircuit fixture flattens");
        assert!(
            flattened.elements.is_empty(),
            "the redundant authored passive and every synthesized helper form one batch"
        );

        let circuit = crate::Engine::default()
            .build_circuit(&netlist)
            .expect("circuit build omits the rejected passive batch");
        assert!(circuit.get_node_by_name("n").is_none());
    }

    #[test]
    fn rejected_owner_does_not_capture_an_authored_helper_shaped_name() {
        let netlist = Netlist::parse_with_options(
            "authored helper-name collision\n\
             .PREPROCESS REMOVEUNUSED R\n\
             .SUBCKT CELL a b c\n\
             R1 a b 1 RSER=2\n\
             RR1#PAR a c 7\n\
             .ENDS CELL\n\
             X1 n n live CELL\n\
             .END\n",
            xyce_options(),
        )
        .expect("authored helper-shaped element name is legal");

        let flattened = crate::netlist::flatten_netlist_with_models(&netlist)
            .expect("helper-name collision fixture flattens");
        assert_eq!(flattened.elements.len(), 1);
        assert_eq!(flattened.elements[0].name, "X1.RR1#PAR");
        assert_eq!(flattened.elements[0].nodes, ["N", "LIVE"]);
        assert!(matches!(
            flattened.elements[0].provenance,
            crate::netlist::ElementProvenance::Authored
        ));
    }

    #[test]
    fn authored_helper_shaped_name_is_normally_removed_after_binding() {
        let netlist = Netlist::parse_with_options(
            "authored helper-name redundancy\n\
             .PREPROCESS REMOVEUNUSED R\n\
             .SUBCKT CELL a b\n\
             RR1#PAR a b 7\n\
             .ENDS CELL\n\
             X1 n n CELL\n\
             .END\n",
            xyce_options(),
        )
        .expect("authored helper-shaped redundant fixture parses");

        let flattened = crate::netlist::flatten_netlist_with_models(&netlist)
            .expect("authored helper-shaped redundant fixture flattens");
        assert!(flattened.elements.is_empty());
    }
}

#[derive(Debug, Default)]
struct RootPreprocessPolicy {
    replace_ground: Option<bool>,
    replace_ground_extra_lines: Vec<usize>,
    remove_unused: Option<RemoveUnusedPolicy>,
    add_resistors: Option<XyceAddResistorsPolicy>,
    add_resistors_extra_lines: Vec<usize>,
}

fn prescan_root_preprocess(
    lines: &[&str],
    source_schedule: Option<&SourceEventSchedule>,
    abort: &dyn AbortSignal,
) -> Result<RootPreprocessPolicy, ParseWithAbortError> {
    let root_path = source_schedule
        .and_then(|schedule| schedule.origin(0))
        .and_then(|origin| origin.path.as_deref());
    let mut policy = RootPreprocessPolicy::default();
    let mut first_replace_ground_line = 0usize;
    let mut first_remove_unused_line = 0usize;
    let mut logical_line = String::new();
    let mut logical_origin_line = 0usize;
    for (index, line) in lines.iter().enumerate() {
        poll_parse_abort(abort, index)?;
        poll_parse_text(abort, line)?;
        // The root's first physical record is always its title. Xyce consumes
        // it before parsePreprocess(), so title text can never select policy.
        if index == 0 {
            continue;
        }
        let mut physical_line = index + 1;
        if let Some(schedule) = source_schedule {
            let Some(origin) = schedule.origin(index) else {
                continue;
            };
            if origin.path.as_deref() != root_path {
                scan_root_preprocess_logical_line(
                    &logical_line,
                    logical_origin_line,
                    &mut policy,
                    &mut first_replace_ground_line,
                    &mut first_remove_unused_line,
                    abort,
                )?;
                logical_line.clear();
                logical_origin_line = 0;
                continue;
            }
            physical_line = origin.line;
        }
        let without_comment = strip_inline_semicolon_comment(line);
        let first_nonblank = without_comment.trim_start_matches([' ', '\t']);
        if first_nonblank.is_empty() || first_nonblank.starts_with('*') {
            continue;
        }
        if let Some(continuation) = first_nonblank.strip_prefix('+') {
            if !logical_line.is_empty() {
                logical_line.push(' ');
                logical_line.push_str(continuation.trim());
            }
            continue;
        }

        // During Xyce's root preprocessing scan, a new physical card that
        // starts with horizontal whitespace is a comment. Whitespace is only
        // significant when the first nonblank byte is the continuation '+'
        // handled above.
        if without_comment.starts_with([' ', '\t']) {
            continue;
        }

        scan_root_preprocess_logical_line(
            &logical_line,
            logical_origin_line,
            &mut policy,
            &mut first_replace_ground_line,
            &mut first_remove_unused_line,
            abort,
        )?;
        logical_line.clear();
        logical_line.push_str(without_comment.trim_end());
        logical_origin_line = physical_line;
    }
    scan_root_preprocess_logical_line(
        &logical_line,
        logical_origin_line,
        &mut policy,
        &mut first_replace_ground_line,
        &mut first_remove_unused_line,
        abort,
    )?;
    ensure_parse_not_aborted(abort)?;
    Ok(policy)
}

fn scan_root_preprocess_logical_line(
    logical_line: &str,
    physical_line: usize,
    policy: &mut RootPreprocessPolicy,
    first_replace_ground_line: &mut usize,
    first_remove_unused_line: &mut usize,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    if logical_line.is_empty() {
        return Ok(());
    }
    let fields = xyce_logical_fields_with_abort(logical_line, abort)?;
    if !fields
        .first()
        .is_some_and(|field| field.eq_ignore_ascii_case(".PREPROCESS"))
    {
        return Ok(());
    }
    let Some(operation) = fields.get(1) else {
        return Err(ParseError::Syntax {
            line: physical_line,
            message: ".PREPROCESS requires an operation".to_string(),
        }
        .into());
    };
    if operation.eq_ignore_ascii_case("REPLACEGROUND") {
        return scan_root_replaceground_fields(
            &fields,
            physical_line,
            policy,
            first_replace_ground_line,
        );
    }
    if operation.eq_ignore_ascii_case("REMOVEUNUSED") {
        return scan_root_removeunused_fields(
            &fields,
            physical_line,
            policy,
            first_remove_unused_line,
        );
    }
    if operation.eq_ignore_ascii_case("ADDRESISTORS") {
        return scan_root_addresistors_fields(&fields, physical_line, policy);
    }
    Err(ParseError::Syntax {
        line: physical_line,
        message: format!("Unknown .PREPROCESS operation '{operation}'"),
    }
    .into())
}

fn scan_root_addresistors_fields(
    fields: &[String],
    physical_line: usize,
    policy: &mut RootPreprocessPolicy,
) -> Result<(), ParseWithAbortError> {
    if fields.len() < 4 {
        return Err(ParseError::Syntax {
            line: physical_line,
            message: "Missing resistance value in .PREPROCESS ADDRESISTORS statement".to_string(),
        }
        .into());
    }

    let mode = if fields[2].eq_ignore_ascii_case("ONETERMINAL") {
        XyceAddResistorMode::OneTerminal
    } else if fields[2].eq_ignore_ascii_case("NODCPATH") {
        XyceAddResistorMode::NoDcPath
    } else {
        return Err(ParseError::Syntax {
            line: physical_line,
            message: format!(
                "Unknown argument {} in .PREPROCESS ADDRESISTORS statement",
                fields[2].to_ascii_uppercase()
            ),
        }
        .into());
    };

    let add_resistors = policy
        .add_resistors
        .get_or_insert_with(XyceAddResistorsPolicy::default);
    let destination = match mode {
        XyceAddResistorMode::OneTerminal => &mut add_resistors.one_terminal,
        XyceAddResistorMode::NoDcPath => &mut add_resistors.no_dc_path,
    };
    if let Some(first) = destination.as_ref() {
        return Err(ParseError::Syntax {
            line: physical_line,
            message: format!(
                "Multiple .PREPROCESS ADDRESISTORS {} statements (first at line {})",
                mode.xyce_keyword(),
                first.source_line
            ),
        }
        .into());
    }
    *destination = Some(XyceAddResistorSpec {
        raw_resistance: fields[3].clone(),
        source_line: physical_line,
    });
    if fields.len() > 4 {
        policy.add_resistors_extra_lines.push(physical_line);
    }
    Ok(())
}

fn scan_root_replaceground_fields(
    fields: &[String],
    physical_line: usize,
    policy: &mut RootPreprocessPolicy,
    first_line: &mut usize,
) -> Result<(), ParseWithAbortError> {
    if policy.replace_ground.is_some() {
        return Err(ParseError::Syntax {
            line: physical_line,
            message: format!(
                "Multiple .PREPROCESS REPLACEGROUND statements (first at line {first_line})"
            ),
        }
        .into());
    }
    let Some(value) = fields.get(2) else {
        return Err(ParseError::Syntax {
            line: physical_line,
            message: ".PREPROCESS REPLACEGROUND requires TRUE or FALSE".to_string(),
        }
        .into());
    };
    policy.replace_ground = Some(if value.eq_ignore_ascii_case("TRUE") {
        true
    } else if value.eq_ignore_ascii_case("FALSE") {
        false
    } else {
        return Err(ParseError::Syntax {
            line: physical_line,
            message: format!("Unknown argument {value} in .PREPROCESS REPLACEGROUND statement"),
        }
        .into());
    });
    if fields.len() > 3 {
        policy.replace_ground_extra_lines.push(physical_line);
    }
    *first_line = physical_line;
    Ok(())
}

fn scan_root_removeunused_fields(
    fields: &[String],
    physical_line: usize,
    policy: &mut RootPreprocessPolicy,
    first_line: &mut usize,
) -> Result<(), ParseWithAbortError> {
    if policy.remove_unused.is_some() {
        return Err(ParseError::Syntax {
            line: physical_line,
            message: format!(
                "Multiple .PREPROCESS REMOVEUNUSED statements (first at line {first_line})"
            ),
        }
        .into());
    }

    let mut selected = RemoveUnusedPolicy::default();
    for field in fields.iter().skip(2) {
        if field == "," {
            continue;
        }
        let Some(device_type) = RemoveUnusedDeviceType::from_xyce_selector(field) else {
            return Err(ParseError::Syntax {
                line: physical_line,
                message: format!(
                    "Unknown argument type {} in .PREPROCESS REMOVEUNUSED statement",
                    field.to_ascii_uppercase()
                ),
            }
            .into());
        };
        selected.device_types.insert(device_type);
    }
    if selected.is_empty() {
        return Err(ParseError::Syntax {
            line: physical_line,
            message: "No remove parameters specified in .PREPROCESS REMOVEUNUSED statement"
                .to_string(),
        }
        .into());
    }
    policy.remove_unused = Some(selected);
    *first_line = physical_line;
    Ok(())
}

/// Tokenize one logical record with Xyce's separated-field boundaries. In
/// particular arithmetic punctuation is part of an ordinary field, which is
/// why `GND+X` is not the same field as the exact ground synonym `GND`.
fn xyce_logical_fields_with_abort(
    line: &str,
    abort: &dyn AbortSignal,
) -> Result<Vec<String>, ParseWithAbortError> {
    let mut fields = Vec::new();
    let mut cursor = 0usize;
    while cursor < line.len() {
        poll_parse_abort(abort, cursor)?;
        let character = line[cursor..]
            .chars()
            .next()
            .expect("cursor remains on a character boundary");
        if character == ';' {
            break;
        }
        if character.is_whitespace() {
            cursor += character.len_utf8();
            continue;
        }
        if matches!(character, '(' | ')' | '}' | ',' | '=') {
            fields.push(character.to_string());
            cursor += character.len_utf8();
            continue;
        }
        if character == '{' {
            let start = cursor;
            let mut depth = 0usize;
            while cursor < line.len() {
                poll_parse_abort(abort, cursor)?;
                let current = line[cursor..]
                    .chars()
                    .next()
                    .expect("cursor remains on a character boundary");
                cursor += current.len_utf8();
                if current == '{' {
                    depth += 1;
                } else if current == '}' {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        break;
                    }
                }
            }
            fields.push(line[start..cursor].to_string());
            continue;
        }
        if matches!(character, '\'' | '"') {
            let quote = character;
            let start = cursor;
            cursor += character.len_utf8();
            while cursor < line.len() {
                poll_parse_abort(abort, cursor)?;
                let current = line[cursor..]
                    .chars()
                    .next()
                    .expect("cursor remains on a character boundary");
                cursor += current.len_utf8();
                if current == quote {
                    break;
                }
            }
            fields.push(line[start..cursor].to_string());
            continue;
        }
        let start = cursor;
        while cursor < line.len() {
            poll_parse_abort(abort, cursor)?;
            let current = line[cursor..]
                .chars()
                .next()
                .expect("cursor remains on a character boundary");
            if current == ';'
                || current.is_whitespace()
                || matches!(current, '(' | ')' | '{' | '}' | ',' | '=' | '\'')
            {
                break;
            }
            cursor += current.len_utf8();
        }
        fields.push(line[start..cursor].to_string());
    }
    ensure_parse_not_aborted(abort)?;
    Ok(fields)
}

fn apply_remove_unused_policy_with_abort(
    elements: &mut Vec<Element>,
    subcircuits: &mut Vec<SubcircuitDef>,
    policy: &RemoveUnusedPolicy,
    abort: &dyn AbortSignal,
) -> Result<(), ParseWithAbortError> {
    let staged_elements = filter_removeunused_elements_with_abort(elements, policy, abort)?;
    let staged_subcircuits = stage_removeunused_subcircuits_with_abort(subcircuits, policy, abort)?;
    ensure_parse_not_aborted(abort)?;
    *elements = staged_elements;
    *subcircuits = staged_subcircuits;
    Ok(())
}

fn stage_removeunused_subcircuits_with_abort(
    subcircuits: &[SubcircuitDef],
    policy: &RemoveUnusedPolicy,
    abort: &dyn AbortSignal,
) -> Result<Vec<SubcircuitDef>, ParseWithAbortError> {
    let mut staged = Vec::with_capacity(subcircuits.len());
    for (index, subcircuit) in subcircuits.iter().enumerate() {
        poll_parse_abort(abort, index)?;
        let elements =
            filter_removeunused_elements_with_abort(&subcircuit.elements, policy, abort)?;
        let nested_subcircuits = stage_removeunused_subcircuits_with_abort(
            &subcircuit.nested_subcircuits,
            policy,
            abort,
        )?;
        staged.push(SubcircuitDef {
            name: subcircuit.name.clone(),
            ports: clone_slice_with_parse_abort(&subcircuit.ports, abort)?,
            elements,
            initial_conditions: clone_slice_with_parse_abort(
                &subcircuit.initial_conditions,
                abort,
            )?,
            node_sets: clone_slice_with_parse_abort(&subcircuit.node_sets, abort)?,
            params: clone_slice_with_parse_abort(&subcircuit.params, abort)?,
            expr_params: clone_slice_with_parse_abort(&subcircuit.expr_params, abort)?,
            string_params: clone_slice_with_parse_abort(&subcircuit.string_params, abort)?,
            body_params: clone_slice_with_parse_abort(&subcircuit.body_params, abort)?,
            body_expr_params: clone_slice_with_parse_abort(&subcircuit.body_expr_params, abort)?,
            body_string_params: clone_slice_with_parse_abort(
                &subcircuit.body_string_params,
                abort,
            )?,
            body_functions: clone_slice_with_parse_abort(&subcircuit.body_functions, abort)?,
            local_options: clone_map_with_parse_abort(&subcircuit.local_options, abort)?,
            library_ref: subcircuit.library_ref.clone(),
            nested_subcircuits,
        });
    }
    ensure_parse_not_aborted(abort)?;
    Ok(staged)
}

fn clone_slice_with_parse_abort<T: Clone>(
    values: &[T],
    abort: &dyn AbortSignal,
) -> Result<Vec<T>, ParseWithAbortError> {
    let mut cloned = Vec::with_capacity(values.len());
    for (index, value) in values.iter().enumerate() {
        poll_parse_abort(abort, index)?;
        cloned.push(value.clone());
    }
    ensure_parse_not_aborted(abort)?;
    Ok(cloned)
}

fn clone_map_with_parse_abort<K, V>(
    values: &HashMap<K, V>,
    abort: &dyn AbortSignal,
) -> Result<HashMap<K, V>, ParseWithAbortError>
where
    K: Clone + Eq + std::hash::Hash,
    V: Clone,
{
    let mut cloned = HashMap::with_capacity(values.len());
    for (index, (key, value)) in values.iter().enumerate() {
        poll_parse_abort(abort, index)?;
        cloned.insert(key.clone(), value.clone());
    }
    ensure_parse_not_aborted(abort)?;
    Ok(cloned)
}

fn logical_line_is_selected_redundant_device(
    logical_line: &str,
    policy: &RemoveUnusedPolicy,
    abort: &dyn AbortSignal,
) -> Result<bool, ParseWithAbortError> {
    let fields = xyce_logical_fields_with_abort(logical_line, abort)?;
    let Some(designator) = fields.first().and_then(|name| name.chars().next()) else {
        return Ok(false);
    };
    let Some((device_type, compared_nodes)) = removeunused_designator_type(designator) else {
        return Ok(false);
    };
    if !policy.contains(device_type) || fields.len() <= compared_nodes {
        return Ok(false);
    }
    Ok(fields[1..=compared_nodes]
        .windows(2)
        .all(|pair| pair[0].eq_ignore_ascii_case(&pair[1])))
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
    abort: &dyn AbortSignal,
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
                    abort,
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
                        message: format!("{}: included .END has no active source frame", origin),
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
    abort: &dyn AbortSignal,
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
    if let Some(policy) = state.options.remove_unused.as_ref()
        && logical_line_is_selected_redundant_device(continuation, policy, abort)?
    {
        continuation.clear();
        return Ok(());
    }
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

//! SPICE netlist parser using token-based parsing
//!
//! Parses standard SPICE netlist format with extensions including:
//! - Sloppy syntax (commas, trailing parameters)
//! - PULSE/SIN/PWL/EXP source specifications with parentheses
//! - .PARAM statements with expression evaluation
//! - Subcircuit definitions and instances

use super::expr::{eval_expression, eval_expression_complex};
use super::lexer::{LexError, TokenKind, TokenStream, parse_spice_value, tokenize};
use super::xspice_parser;
use super::{
    AnalysisCommand, BjtType, DataTable, Element, ElementKind, FreqVariation, InitialCondition,
    JfetType, MesfetType, ModelDef, MonteCarloCommand, MonteCarloDistribution, MosType, Netlist,
    NodeSet, ParamContext, ParametricValue, ParseDiagnostic, ParseError, PoleZeroAnalysisType,
    PoleZeroTransferType, PspiceUTiming, SaveSet, SaveSignal, SensitivityAcSweep,
    SimulationOptions, SourceSpec, StatisticalParamMode, StepCommand, StepSweep, StepTarget,
    SubcircuitDef, SwitchState, VerilogAInclude,
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
}

impl Default for NetlistParseOptions {
    fn default() -> Self {
        Self {
            statistical_mode: StatisticalParamMode::Sample,
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

impl DataTableBuilder {
    fn new(opened_at_line: usize, line: &str) -> Result<Self, ParseError> {
        let mut fields = line.split_whitespace();
        let _data = fields.next();
        let Some(name) = fields.next() else {
            return Err(ParseError::Syntax {
                line: opened_at_line,
                message: ".DATA requires a table name".to_string(),
            });
        };
        let params = fields.map(|field| field.to_string()).collect::<Vec<_>>();
        validate_data_table_params(opened_at_line, name, &params)?;
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
    ) -> Result<(), ParseError> {
        let body = line.strip_prefix('+').unwrap_or(line).trim();
        if body.is_empty() {
            return Ok(());
        }
        let fields = body.split_whitespace().collect::<Vec<_>>();
        if fields.is_empty() {
            return Ok(());
        }

        if self.params.is_empty() {
            self.params = fields.iter().map(|field| (*field).to_string()).collect();
            validate_data_table_params(line_num, &self.name, &self.params)?;
            return Ok(());
        }

        for field in fields {
            self.flat_values
                .push(parse_data_table_value(line_num, &self.name, field, params)?);
        }
        Ok(())
    }

    fn finish(self, line_num: usize) -> Result<DataTable, ParseError> {
        if self.params.is_empty() {
            return Err(ParseError::Syntax {
                line: self.opened_at_line,
                message: format!(".DATA {} has no parameter columns", self.name),
            });
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
            });
        }
        let rows = self
            .flat_values
            .chunks_exact(columns)
            .map(|chunk| chunk.to_vec())
            .collect::<Vec<_>>();
        Ok(DataTable {
            name: self.name,
            params: self.params,
            rows,
        })
    }
}

fn validate_data_table_params(
    line_num: usize,
    table_name: &str,
    params: &[String],
) -> Result<(), ParseError> {
    for param in params {
        let mut chars = param.chars();
        let valid = chars
            .next()
            .is_some_and(|ch| ch.is_ascii_alphabetic() || ch == '_' || ch == '$')
            && chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_' || ch == '$' || ch == '.');
        if !valid {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    ".DATA {table_name} parameter column '{param}' is not a valid parameter name"
                ),
            });
        }
    }
    Ok(())
}

fn parse_data_table_value(
    line_num: usize,
    table_name: &str,
    token: &str,
    params: &ParamContext,
) -> Result<Value, ParseError> {
    if let Ok(value) = parse_spice_value(token) {
        return Ok(value);
    }
    let expr = token
        .strip_prefix('{')
        .and_then(|rest| rest.strip_suffix('}'))
        .unwrap_or(token);
    eval_expression(expr, params).map_err(|err| ParseError::Syntax {
        line: line_num,
        message: format!(".DATA {table_name} value '{token}' is not numeric: {err}"),
    })
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
    let lines: Vec<&str> = input.lines().collect();

    if lines.is_empty() {
        return Ok(Netlist::default());
    }

    // First line is the title
    let title = lines[0].to_string();
    let mut state = ParseState::new();
    state.params.set_statistical_mode(options.statistical_mode);

    // Seed the statistical expression functions before any parameter
    // evaluation so the deck behaves identically regardless of where the
    // `.options seed=` line appears.
    if let Some(seed) = prescan_random_seed(&lines)? {
        state.params.set_random_seed(seed);
        log::info!("statistical expression functions seeded with {seed} (.options seed)");
    }
    prescan_temperature_options(&lines, &mut state)?;

    let mut line_num = 1;
    let mut continuation = String::new();
    let mut data_table: Option<DataTableBuilder> = None;

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

        let head = trimmed.split_whitespace().next().unwrap_or("");
        if let Some(table) = data_table.as_mut() {
            if head.eq_ignore_ascii_case(".enddata") {
                let table = data_table
                    .take()
                    .expect(".DATA builder exists while inside data block")
                    .finish(line_num)?;
                state.data_tables.push(table);
            } else if head.eq_ignore_ascii_case(".data") {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: ".DATA cannot be nested inside another .DATA block".to_string(),
                });
            } else if is_dot_command_head(head) {
                return Err(ParseError::Syntax {
                    line: table.opened_at_line,
                    message: ".DATA without a matching .ENDDATA".to_string(),
                });
            } else {
                table.push_line(line_num, trimmed, &state.params)?;
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
            data_table = Some(DataTableBuilder::new(line_num, trimmed)?);
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

    if let Some(table) = data_table {
        return Err(ParseError::Syntax {
            line: table.opened_at_line,
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

    normalize_pspice_u_timing_aliases(&mut state);
    validate_resistor_model_references(&state)?;

    state.into_netlist(title, input, line_num)
}

fn normalize_pspice_u_timing_aliases(state: &mut ParseState) {
    let source_models = state.models.clone();
    let mut existing_names: HashSet<String> = state
        .models
        .iter()
        .map(|model| model.name.to_ascii_uppercase())
        .collect();
    let mut generated_models = Vec::new();

    normalize_pspice_u_timing_in_elements(
        &mut state.elements,
        "TOP",
        &source_models,
        &mut existing_names,
        &mut generated_models,
    );
    for subckt in &mut state.subcircuits {
        normalize_pspice_u_timing_in_subckt(
            subckt,
            &source_models,
            &mut existing_names,
            &mut generated_models,
        );
    }

    state.models.extend(generated_models);
}

fn normalize_pspice_u_timing_in_subckt(
    subckt: &mut SubcircuitDef,
    source_models: &[ModelDef],
    existing_names: &mut HashSet<String>,
    generated_models: &mut Vec<ModelDef>,
) {
    normalize_pspice_u_timing_in_elements(
        &mut subckt.elements,
        &subckt.name,
        source_models,
        existing_names,
        generated_models,
    );
    for nested in &mut subckt.nested_subcircuits {
        normalize_pspice_u_timing_in_subckt(
            nested,
            source_models,
            existing_names,
            generated_models,
        );
    }
}

fn normalize_pspice_u_timing_in_elements(
    elements: &mut [Element],
    scope: &str,
    source_models: &[ModelDef],
    existing_names: &mut HashSet<String>,
    generated_models: &mut Vec<ModelDef>,
) {
    for element in elements {
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
        let alias = pspice_u_timing_alias_model(&alias_name, model, timing_model)
            .expect("supported PSpice U timing model should create an alias");

        *model = alias_name;
        generated_models.push(alias);
    }
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
}

fn pspice_u_timing_alias_model(
    alias_name: &str,
    code_model: &str,
    timing_model: &ModelDef,
) -> Option<ModelDef> {
    if timing_model.model_type.eq_ignore_ascii_case("UGATE")
        && pspice_u_gate_model_accepts_ugate_timing(code_model)
    {
        return Some(pspice_ugate_alias_model(
            alias_name,
            code_model,
            timing_model,
        ));
    }

    if timing_model.model_type.eq_ignore_ascii_case("UEFF")
        && pspice_u_edge_ff_model_accepts_ueff_timing(code_model)
    {
        return Some(pspice_ueff_alias_model(
            alias_name,
            code_model,
            timing_model,
        ));
    }

    if timing_model.model_type.eq_ignore_ascii_case("UTGATE")
        && pspice_u_tristate_model_accepts_utgate_timing(code_model)
    {
        return Some(pspice_utgate_alias_model(
            alias_name,
            code_model,
            timing_model,
        ));
    }

    if timing_model.model_type.eq_ignore_ascii_case("UGFF")
        && pspice_u_latch_model_accepts_ugff_timing(code_model)
    {
        return Some(pspice_ugff_alias_model(
            alias_name,
            code_model,
            timing_model,
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

fn pspice_ugate_alias_model(
    alias_name: &str,
    code_model: &str,
    timing_model: &ModelDef,
) -> ModelDef {
    const PSPICE_U_DEFAULT_DELAY: Value = 1.0e-12;

    let mut params = vec![("inertial_delay".to_string(), 1.0)];
    let mut expr_params = Vec::new();
    push_pspice_timing_delay_param(
        timing_model,
        &["TPLHTY", "TPLHMN", "TPLHMX"],
        "rise_delay",
        PSPICE_U_DEFAULT_DELAY,
        &mut params,
        &mut expr_params,
    );
    push_pspice_timing_delay_param(
        timing_model,
        &["TPHLTY", "TPHLMN", "TPHLMX"],
        "fall_delay",
        PSPICE_U_DEFAULT_DELAY,
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
        integer_vector_params: Vec::new(),
    }
}

fn pspice_utgate_alias_model(
    alias_name: &str,
    code_model: &str,
    timing_model: &ModelDef,
) -> ModelDef {
    let mut params = vec![("inertial_delay".to_string(), 1.0)];
    let mut expr_params = Vec::new();

    let rising = pspice_timing_delay_estimate(timing_model, &["TPLHTY", "TPLHMN", "TPLHMX"]);
    let falling = pspice_timing_delay_estimate(timing_model, &["TPHLTY", "TPHLMN", "TPHLMX"]);
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
        integer_vector_params: Vec::new(),
    }
}

fn pspice_ugff_alias_model(
    alias_name: &str,
    code_model: &str,
    timing_model: &ModelDef,
) -> ModelDef {
    let mut params = vec![
        ("rise_delay".to_string(), 1.0e-9),
        ("fall_delay".to_string(), 1.0e-9),
    ];
    let mut expr_params = Vec::new();

    let data_rise =
        pspice_timing_delay_estimate(timing_model, &["TPDQLHTY", "TPDQLHMN", "TPDQLHMX"]);
    let data_fall =
        pspice_timing_delay_estimate(timing_model, &["TPDQHLTY", "TPDQHLMN", "TPDQHLMX"]);
    if let Some(delay) = pspice_select_longer_delay(data_rise, data_fall) {
        let target = if code_model.eq_ignore_ascii_case("d_srlatch") {
            "sr_delay"
        } else {
            "data_delay"
        };
        push_pspice_timing_delay(target, delay, &mut params, &mut expr_params);
    }

    let gate_rise =
        pspice_timing_delay_estimate(timing_model, &["TPGQLHTY", "TPGQLHMN", "TPGQLHMX"]);
    let gate_fall =
        pspice_timing_delay_estimate(timing_model, &["TPGQHLTY", "TPGQHLMN", "TPGQHLMX"]);
    if let Some(delay) = pspice_select_longer_delay(gate_rise, gate_fall) {
        push_pspice_timing_delay("enable_delay", delay, &mut params, &mut expr_params);
    }

    push_pspice_pcq_set_reset_delays(timing_model, &mut params, &mut expr_params);

    ModelDef {
        name: alias_name.to_string(),
        model_type: code_model.to_string(),
        params,
        expr_params,
        string_params: Vec::new(),
        string_vector_params: Vec::new(),
        real_vector_params: Vec::new(),
        integer_vector_params: Vec::new(),
    }
}

fn pspice_ueff_alias_model(
    alias_name: &str,
    code_model: &str,
    timing_model: &ModelDef,
) -> ModelDef {
    let mut params = vec![
        ("rise_delay".to_string(), 1.0e-9),
        ("fall_delay".to_string(), 1.0e-9),
    ];
    let mut expr_params = Vec::new();

    let clk_rise =
        pspice_timing_delay_estimate(timing_model, &["TPCLKQLHTY", "TPCLKQLHMN", "TPCLKQLHMX"]);
    let clk_fall =
        pspice_timing_delay_estimate(timing_model, &["TPCLKQHLTY", "TPCLKQHLMN", "TPCLKQHLMX"]);
    if let Some(delay) = pspice_select_longer_delay(clk_rise, clk_fall) {
        push_pspice_timing_delay("clk_delay", delay, &mut params, &mut expr_params);
    }

    push_pspice_pcq_set_reset_delays(timing_model, &mut params, &mut expr_params);

    ModelDef {
        name: alias_name.to_string(),
        model_type: code_model.to_string(),
        params,
        expr_params,
        string_params: Vec::new(),
        string_vector_params: Vec::new(),
        real_vector_params: Vec::new(),
        integer_vector_params: Vec::new(),
    }
}

fn push_pspice_pcq_set_reset_delays(
    timing_model: &ModelDef,
    params: &mut Vec<(String, Value)>,
    expr_params: &mut Vec<(String, String)>,
) {
    let set_delay =
        pspice_timing_delay_estimate(timing_model, &["TPPCQLHTY", "TPPCQLHMN", "TPPCQLHMX"]);
    let reset_delay =
        pspice_timing_delay_estimate(timing_model, &["TPPCQHLTY", "TPPCQHLMN", "TPPCQHLMX"]);
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
    params: &mut Vec<(String, Value)>,
    expr_params: &mut Vec<(String, String)>,
) {
    if let Some((_, value)) = source_names.iter().find_map(|source_name| {
        timing_model
            .params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(source_name))
    }) {
        params.push((target_name.to_string(), *value));
        return;
    }

    if let Some((_, expr)) = source_names.iter().find_map(|source_name| {
        timing_model
            .expr_params
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(source_name))
    }) {
        expr_params.push((target_name.to_string(), expr.clone()));
        return;
    }

    params.push((target_name.to_string(), default_value));
}

#[derive(Clone)]
enum PspiceTimingDelay {
    Numeric(Value),
    Expr(String),
}

fn pspice_timing_delay_estimate(
    timing_model: &ModelDef,
    source_names: &[&str],
) -> Option<PspiceTimingDelay> {
    for source_name in source_names {
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

fn validate_resistor_model_references(state: &ParseState) -> Result<(), ParseError> {
    let models = state
        .models
        .iter()
        .map(|model| model.name.to_ascii_uppercase())
        .collect::<HashSet<_>>();
    validate_resistor_model_references_in_elements(&state.elements, &models)?;
    for subckt in &state.subcircuits {
        validate_resistor_model_references_in_subckt(subckt, &models)?;
    }
    Ok(())
}

fn validate_resistor_model_references_in_subckt(
    subckt: &SubcircuitDef,
    models: &HashSet<String>,
) -> Result<(), ParseError> {
    validate_resistor_model_references_in_elements(&subckt.elements, models)?;
    for nested in &subckt.nested_subcircuits {
        validate_resistor_model_references_in_subckt(nested, models)?;
    }
    Ok(())
}

fn validate_resistor_model_references_in_elements(
    elements: &[Element],
    models: &HashSet<String>,
) -> Result<(), ParseError> {
    for element in elements {
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
            });
        }
    }
    Ok(())
}

fn is_dot_command_head(head: &str) -> bool {
    head.strip_prefix('.')
        .and_then(|rest| rest.chars().next())
        .is_some_and(|ch| ch.is_ascii_alphabetic())
}

fn prescan_temperature_options(lines: &[&str], state: &mut ParseState) -> Result<(), ParseError> {
    let mut continuation = String::new();
    let mut in_options = false;
    let mut line_num = 1usize;

    for line in lines.iter().skip(1) {
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
            scan_temperature_option_line(&continuation, line_num - 1, state)?;
            continuation.clear();
        }

        let head = trimmed.split_whitespace().next().unwrap_or("");
        if head.eq_ignore_ascii_case(".options")
            || head.eq_ignore_ascii_case(".option")
            || head.eq_ignore_ascii_case(".opt")
        {
            in_options = true;
            continuation.push_str(trimmed);
        } else {
            in_options = false;
        }
    }

    if !continuation.is_empty() {
        scan_temperature_option_line(&continuation, line_num, state)?;
    }

    Ok(())
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

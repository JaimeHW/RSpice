use super::*;
use crate::Value;
use crate::netlist::lexer::parse_spice_value;
use crate::xspice::CmError;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::UNIX_EPOCH;

//=============================================================================
// Digital Source
//=============================================================================

/// Digital stimulus source from file
#[derive(Debug, Default)]
pub struct DigitalSource;

const D_SOURCE_NO_ROW: i64 = -1;
const D_SOURCE_BEFORE_FIRST_ROW: i64 = -2;
const D_SOURCE_EMITTED_ROW: usize = 0;
const D_SOURCE_SCHEDULED_ROW: usize = 1;

#[derive(Debug, Clone)]
struct DSourceRow {
    time: Value,
    values: Vec<DigitalValue>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct DSourceCacheKey {
    input_file: String,
    width: usize,
    len: u64,
    modified_nanos: u128,
}

fn d_source_cache() -> &'static Mutex<HashMap<DSourceCacheKey, Arc<Vec<DSourceRow>>>> {
    static CACHE: OnceLock<Mutex<HashMap<DSourceCacheKey, Arc<Vec<DSourceRow>>>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn d_source_error(input_file: &str, line: usize, message: impl Into<String>) -> CmError {
    CmError::EvaluationError(format!(
        "d_source input_file '{}' line {}: {}",
        input_file,
        line,
        message.into()
    ))
}

fn d_source_file_error(input_file: &str, message: impl Into<String>) -> CmError {
    CmError::EvaluationError(format!(
        "d_source input_file '{}': {}",
        input_file,
        message.into()
    ))
}

fn d_source_cache_key(input_file: &str, width: usize) -> CmResult<DSourceCacheKey> {
    let metadata =
        fs::metadata(input_file).map_err(|err| d_source_file_error(input_file, err.to_string()))?;
    let modified_nanos = metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);

    Ok(DSourceCacheKey {
        input_file: input_file.to_string(),
        width,
        len: metadata.len(),
        modified_nanos,
    })
}

fn parse_d_source_token(input_file: &str, line: usize, token: &str) -> CmResult<DigitalValue> {
    let mut chars = token.chars();
    let Some(level) = chars.next() else {
        return Err(d_source_error(input_file, line, "empty digital token"));
    };

    let Some(strength) = chars.next() else {
        if level == 'Z' || level == 'z' {
            return Ok(DigitalValue::high_z());
        }
        return Err(d_source_error(
            input_file,
            line,
            format!("digital token '{token}' is missing a strength suffix"),
        ));
    };

    if chars.next().is_some() {
        return Err(d_source_error(
            input_file,
            line,
            format!("digital token '{token}' has extra characters"),
        ));
    }

    let unknown = matches!(level, 'U' | 'u' | 'X' | 'x');
    let strength = strength.to_ascii_lowercase();
    let value = match (level, unknown, strength) {
        ('0', _, 's') => DigitalValue::new(DigitalState::Zero, DigitalStrength::Strong),
        ('1', _, 's') => DigitalValue::new(DigitalState::One, DigitalStrength::Strong),
        (_, true, 's') => DigitalValue::new(DigitalState::Unknown, DigitalStrength::Strong),
        ('0', _, 'r') => DigitalValue::new(DigitalState::ZeroR, DigitalStrength::Resistive),
        ('1', _, 'r') => DigitalValue::new(DigitalState::OneR, DigitalStrength::Resistive),
        (_, true, 'r') => DigitalValue::new(DigitalState::UnknownR, DigitalStrength::Resistive),
        ('0', _, 'z') => DigitalValue::new(DigitalState::ZeroZ, DigitalStrength::HighZ),
        ('1', _, 'z') => DigitalValue::new(DigitalState::OneZ, DigitalStrength::HighZ),
        (_, true, 'z') => DigitalValue::new(DigitalState::UnknownZ, DigitalStrength::HighZ),
        ('0', _, 'u') => DigitalValue::new(DigitalState::Zero, DigitalStrength::Undetermined),
        ('1', _, 'u') => DigitalValue::new(DigitalState::One, DigitalStrength::Undetermined),
        (_, true, 'u') => DigitalValue::new(DigitalState::Unknown, DigitalStrength::Undetermined),
        _ => {
            return Err(d_source_error(
                input_file,
                line,
                format!("invalid digital token '{token}'"),
            ));
        }
    };

    Ok(value)
}

fn parse_d_source_file(input_file: &str, width: usize) -> CmResult<Vec<DSourceRow>> {
    let contents = fs::read_to_string(input_file)
        .map_err(|err| d_source_file_error(input_file, err.to_string()))?;
    let mut rows = Vec::new();
    let mut previous_time = None;

    for (line_idx, line) in contents.lines().enumerate() {
        let line_no = line_idx + 1;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }

        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
        let expected = width + 1;
        if tokens.len() != expected {
            return Err(d_source_error(
                input_file,
                line_no,
                format!(
                    "expected {} token(s) for width {}, got {}",
                    expected,
                    width,
                    tokens.len()
                ),
            ));
        }

        let time = parse_spice_value(tokens[0]).map_err(|err| {
            d_source_error(
                input_file,
                line_no,
                format!("invalid time token '{}': {:?}", tokens[0], err),
            )
        })?;
        if !time.is_finite() || time < 0.0 {
            return Err(d_source_error(
                input_file,
                line_no,
                format!("time must be finite and non-negative, got {}", tokens[0]),
            ));
        }
        if let Some(previous_time) = previous_time
            && time < previous_time
        {
            return Err(d_source_error(
                input_file,
                line_no,
                "time values must be nondecreasing",
            ));
        }
        previous_time = Some(time);

        let values = tokens[1..]
            .iter()
            .map(|token| parse_d_source_token(input_file, line_no, token))
            .collect::<CmResult<Vec<_>>>()?;
        rows.push(DSourceRow { time, values });
    }

    if rows.is_empty() {
        return Err(d_source_file_error(input_file, "contains no stimulus rows"));
    }

    Ok(rows)
}

fn load_d_source_rows(input_file: &str, width: usize) -> CmResult<Arc<Vec<DSourceRow>>> {
    let key = d_source_cache_key(input_file, width)?;
    let cache = d_source_cache();

    {
        let guard = cache
            .lock()
            .map_err(|_| CmError::Internal("d_source stimulus cache poisoned".to_string()))?;
        if let Some(rows) = guard.get(&key) {
            return Ok(Arc::clone(rows));
        }
    }

    let rows = Arc::new(parse_d_source_file(input_file, width)?);
    let mut guard = cache
        .lock()
        .map_err(|_| CmError::Internal("d_source stimulus cache poisoned".to_string()))?;
    guard.insert(key, Arc::clone(&rows));
    Ok(rows)
}

fn d_source_row_indices(rows: &[DSourceRow], time: Value) -> (Option<usize>, Option<usize>) {
    const TIME_EPSILON: Value = 1e-18;
    let mut active = None;
    for (idx, row) in rows.iter().enumerate() {
        if row.time <= time + TIME_EPSILON {
            active = Some(idx);
        } else {
            return (active, Some(idx));
        }
    }

    (active, None)
}

impl CodeModel for DigitalSource {
    fn name(&self) -> &str {
        "d_source"
    }
    fn description(&self) -> &str {
        "Digital stimulus from file"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| vec![PortSpec::vector_output("out", PortType::Digital)])
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::string("input_file", "source.txt"),
                ParamSpec::real("input_load", 1e-12).with_min(0.0),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(2);
        ctx.set_int_state(D_SOURCE_EMITTED_ROW, D_SOURCE_NO_ROW);
        ctx.set_int_state(D_SOURCE_SCHEDULED_ROW, D_SOURCE_NO_ROW);
        Ok(())
    }
    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let input_file = ctx
            .string_param("input_file")
            .filter(|path| !path.trim().is_empty())
            .unwrap_or("source.txt");
        let width = ctx.port_width("out").max(1);
        let rows = load_d_source_rows(input_file, width)?;
        let (active_idx, next_idx) = d_source_row_indices(&rows, ctx.time);
        let emitted_row = ctx.int_state(D_SOURCE_EMITTED_ROW);
        let scheduled_row = ctx.int_state(D_SOURCE_SCHEDULED_ROW);

        if let Some(idx) = active_idx {
            if emitted_row != idx as i64 {
                let row = &rows[idx];
                ctx.set_output_digital_vector("out", row.values.clone(), row.time - ctx.time);
                ctx.set_int_state(D_SOURCE_EMITTED_ROW, idx as i64);
            }
        } else if emitted_row != D_SOURCE_BEFORE_FIRST_ROW {
            ctx.set_output_digital_vector("out", vec![DigitalValue::unknown(); width], 0.0);
            ctx.set_int_state(D_SOURCE_EMITTED_ROW, D_SOURCE_BEFORE_FIRST_ROW);
        }

        if let Some(idx) = next_idx {
            if scheduled_row != idx as i64 {
                let row = &rows[idx];
                ctx.set_output_digital_vector("out", row.values.clone(), row.time - ctx.time);
                ctx.set_int_state(D_SOURCE_SCHEDULED_ROW, idx as i64);
            }
        } else if scheduled_row != D_SOURCE_NO_ROW {
            ctx.set_int_state(D_SOURCE_SCHEDULED_ROW, D_SOURCE_NO_ROW);
        }

        Ok(())
    }
}

/// Digital state machine
#[derive(Debug, Default)]
pub struct DigitalStateMachine;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct DStateCacheKey {
    state_file: String,
    input_width: usize,
    output_width: usize,
    len: u64,
    modified_nanos: u128,
}

#[derive(Debug, Clone)]
struct DStateTransition {
    state: i64,
    outputs: Vec<DigitalValue>,
    inputs: Vec<Option<bool>>,
    next_state: i64,
}

#[derive(Debug, Clone)]
struct DStateTable {
    transitions: Vec<DStateTransition>,
}

fn d_state_cache() -> &'static Mutex<HashMap<DStateCacheKey, Arc<DStateTable>>> {
    static CACHE: OnceLock<Mutex<HashMap<DStateCacheKey, Arc<DStateTable>>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn d_state_file_error(state_file: &str, message: impl Into<String>) -> CmError {
    CmError::EvaluationError(format!(
        "d_state state_file '{}': {}",
        state_file,
        message.into()
    ))
}

fn d_state_error(state_file: &str, line: usize, message: impl Into<String>) -> CmError {
    CmError::EvaluationError(format!(
        "d_state state_file '{}' line {}: {}",
        state_file,
        line,
        message.into()
    ))
}

fn d_state_cache_key(
    state_file: &str,
    input_width: usize,
    output_width: usize,
) -> CmResult<DStateCacheKey> {
    let metadata =
        fs::metadata(state_file).map_err(|err| d_state_file_error(state_file, err.to_string()))?;
    let modified_nanos = metadata
        .modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);

    Ok(DStateCacheKey {
        state_file: state_file.to_string(),
        input_width,
        output_width,
        len: metadata.len(),
        modified_nanos,
    })
}

fn parse_d_state_input_token(state_file: &str, line: usize, token: &str) -> CmResult<Option<bool>> {
    match token {
        "0" => Ok(Some(false)),
        "1" => Ok(Some(true)),
        "x" | "X" => Ok(None),
        _ => Err(d_state_error(
            state_file,
            line,
            format!("invalid input token '{token}'"),
        )),
    }
}

fn parse_d_state_i64(state_file: &str, line: usize, token: &str) -> CmResult<i64> {
    crate::netlist::lexer::parse_spice_value(token)
        .map(|value| value as i64)
        .or_else(|_| token.parse::<i64>())
        .map_err(|err| {
            d_state_error(
                state_file,
                line,
                format!("invalid integer '{token}': {err:?}"),
            )
        })
}

fn parse_d_state_file(
    state_file: &str,
    input_width: usize,
    output_width: usize,
) -> CmResult<DStateTable> {
    let contents = fs::read_to_string(state_file)
        .map_err(|err| d_state_file_error(state_file, err.to_string()))?;
    let mut transitions = Vec::new();
    let mut last_state = None;
    let mut last_outputs: Option<Vec<DigitalValue>> = None;

    for (line_idx, line) in contents.lines().enumerate() {
        let line_no = line_idx + 1;
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('*') {
            continue;
        }

        let tokens: Vec<&str> = trimmed.split_whitespace().collect();
        let header_len = output_width + input_width + 3;
        let continuation_len = input_width + 2;
        let (state, outputs, input_start, arrow_idx, next_idx) = if tokens.len() == header_len {
            let state = parse_d_state_i64(state_file, line_no, tokens[0])?;
            let outputs = tokens[1..1 + output_width]
                .iter()
                .map(|token| parse_d_source_token(state_file, line_no, token))
                .collect::<CmResult<Vec<_>>>()?;
            last_state = Some(state);
            last_outputs = Some(outputs.clone());
            (
                state,
                outputs,
                1 + output_width,
                1 + output_width + input_width,
                header_len - 1,
            )
        } else if tokens.len() == continuation_len {
            let state = last_state.ok_or_else(|| {
                d_state_error(
                    state_file,
                    line_no,
                    "continuation row appears before any state header",
                )
            })?;
            let outputs = last_outputs.clone().ok_or_else(|| {
                d_state_error(
                    state_file,
                    line_no,
                    "continuation row has no previous output vector",
                )
            })?;
            (state, outputs, 0, input_width, continuation_len - 1)
        } else {
            return Err(d_state_error(
                state_file,
                line_no,
                format!(
                    "expected {} header token(s) or {} continuation token(s), got {}",
                    header_len,
                    continuation_len,
                    tokens.len()
                ),
            ));
        };

        if tokens.get(arrow_idx) != Some(&"->") {
            return Err(d_state_error(
                state_file,
                line_no,
                format!("expected '->', got '{}'", tokens[arrow_idx]),
            ));
        }

        let inputs = tokens[input_start..input_start + input_width]
            .iter()
            .map(|token| parse_d_state_input_token(state_file, line_no, token))
            .collect::<CmResult<Vec<_>>>()?;
        let next_state = parse_d_state_i64(state_file, line_no, tokens[next_idx])?;
        transitions.push(DStateTransition {
            state,
            outputs,
            inputs,
            next_state,
        });
    }

    if transitions.is_empty() {
        return Err(d_state_file_error(state_file, "contains no state rows"));
    }

    Ok(DStateTable { transitions })
}

fn load_d_state_table(
    state_file: &str,
    input_width: usize,
    output_width: usize,
) -> CmResult<Arc<DStateTable>> {
    let key = d_state_cache_key(state_file, input_width, output_width)?;
    let cache = d_state_cache();

    {
        let guard = cache
            .lock()
            .map_err(|_| CmError::Internal("d_state table cache poisoned".to_string()))?;
        if let Some(table) = guard.get(&key) {
            return Ok(Arc::clone(table));
        }
    }

    let table = Arc::new(parse_d_state_file(state_file, input_width, output_width)?);
    let mut guard = cache
        .lock()
        .map_err(|_| CmError::Internal("d_state table cache poisoned".to_string()))?;
    guard.insert(key, Arc::clone(&table));
    Ok(table)
}

fn d_state_logic(value: DigitalValue) -> Option<bool> {
    value.state.logic_level()
}

fn d_state_state_code(value: DigitalValue) -> i64 {
    match value.state {
        DigitalState::Zero | DigitalState::ZeroR | DigitalState::ZeroZ => 0,
        DigitalState::One | DigitalState::OneR | DigitalState::OneZ => 1,
        _ => 2,
    }
}

fn d_state_outputs(table: &DStateTable, state: i64) -> Option<Vec<DigitalValue>> {
    table
        .transitions
        .iter()
        .find(|row| row.state == state)
        .map(|row| row.outputs.clone())
}

fn d_state_next(table: &DStateTable, state: i64, inputs: &[DigitalValue]) -> Option<i64> {
    table
        .transitions
        .iter()
        .filter(|row| row.state == state)
        .find(|row| {
            row.inputs
                .iter()
                .zip(inputs.iter())
                .all(|(pattern, value)| {
                    pattern.is_none_or(|expected| d_state_logic(*value) == Some(expected))
                })
        })
        .map(|row| row.next_state)
}

impl CodeModel for DigitalStateMachine {
    fn name(&self) -> &str {
        "d_state"
    }
    fn description(&self) -> &str {
        "Digital state machine"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::vector_input("in", PortType::Digital),
                PortSpec::input("clk", PortType::Digital),
                PortSpec::input("reset", PortType::Digital),
                PortSpec::vector_output("out", PortType::Digital),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("clk_delay", 1.0e-9).with_min(0.0),
                ParamSpec::real("reset_delay", 1.0e-9).with_min(0.0),
                ParamSpec::string("state_file", "state.txt"),
                ParamSpec::integer("reset_state", 0),
                ParamSpec::real("input_load", 1.0e-12).with_min(0.0),
                ParamSpec::real("clk_load", 1.0e-12).with_min(0.0),
                ParamSpec::real("reset_load", 1.0e-12).with_min(0.0),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(4);
        ctx.set_int_state(0, 0); // initialized
        ctx.set_int_state(1, 0); // current_state
        ctx.set_int_state(2, 0); // previous clock state
        ctx.set_int_state(3, 0); // previous reset state
        Ok(())
    }
    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let state_file = ctx
            .string_param("state_file")
            .filter(|path| !path.trim().is_empty())
            .unwrap_or("state.txt");
        let input_width = ctx.port_width("in");
        let output_width = ctx.port_width("out").max(1);
        let table = load_d_state_table(state_file, input_width, output_width)?;
        let reset_state = ctx.param_or("reset_state", 0.0) as i64;
        let clk_delay = ctx.param_or("clk_delay", 1.0e-9);
        let reset_delay = ctx.param_or("reset_delay", 1.0e-9);

        if ctx.time == 0.0 || ctx.int_state(0) == 0 {
            let outputs = d_state_outputs(&table, reset_state).ok_or_else(|| {
                CmError::EvaluationError(format!(
                    "d_state state_file '{state_file}' does not define reset_state {reset_state}"
                ))
            })?;
            ctx.set_int_state(0, 1);
            ctx.set_int_state(1, reset_state);
            ctx.set_int_state(2, 0);
            ctx.set_int_state(3, 0);
            ctx.set_output_digital_vector("out", outputs, 0.0);
            return Ok(());
        }

        let clk = ctx.input_digital("clk").unwrap_or_default();
        let reset = ctx.input_digital("reset").unwrap_or_default();
        let clk_state = d_state_state_code(clk);
        let reset_state_code = d_state_state_code(reset);
        let prev_clk = ctx.int_state(2);
        let prev_reset = ctx.int_state(3);
        let mut current_state = ctx.int_state(1);

        if reset_state_code != prev_reset {
            if reset_state_code == 1 {
                current_state = reset_state;
                let outputs = d_state_outputs(&table, current_state).ok_or_else(|| {
                    CmError::EvaluationError(format!(
                        "d_state state_file '{state_file}' does not define state {current_state}"
                    ))
                })?;
                ctx.set_output_digital_vector("out", outputs, reset_delay);
            }
        } else if reset_state_code != 1 && clk_state != prev_clk && clk_state == 1 {
            let inputs = ctx.input_digital_vector("in");
            if let Some(next_state) = d_state_next(&table, current_state, &inputs) {
                current_state = next_state;
                let outputs = d_state_outputs(&table, current_state).ok_or_else(|| {
                    CmError::EvaluationError(format!(
                        "d_state state_file '{state_file}' does not define state {current_state}"
                    ))
                })?;
                ctx.set_output_digital_vector("out", outputs, clk_delay);
            }
        }

        ctx.set_int_state(1, current_state);
        ctx.set_int_state(2, clk_state);
        ctx.set_int_state(3, reset_state_code);
        Ok(())
    }
}

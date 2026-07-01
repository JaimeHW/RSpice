use super::*;
use crate::Value;
use crate::xspice::{CmError, EvaluationPhase, data_file};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

//=============================================================================
// Digital Source
//=============================================================================

/// Digital stimulus source from file
#[derive(Debug, Default)]
pub struct DigitalSource;

const D_SOURCE_NO_ROW: i64 = -1;
const D_SOURCE_BEFORE_FIRST_ROW: i64 = -2;
const D_SOURCE_LOAD_FAILED: i64 = -3;
const D_SOURCE_EMITTED_ROW: usize = 0;
const D_SOURCE_SCHEDULED_ROW: usize = 1;
const D_SOURCE_ROWS_RESOURCE: &str = "xspice.d_source.rows";
const D_STATE_TABLE_RESOURCE: &str = "xspice.d_state.table";

#[derive(Debug, Clone)]
struct DSourceRow {
    time: Value,
}

#[derive(Debug, Clone)]
struct DSourceRows {
    width: usize,
    rows: Vec<DSourceRow>,
    values: Vec<DigitalValue>,
}

impl DSourceRows {
    fn len(&self) -> usize {
        self.rows.len()
    }

    fn first(&self) -> Option<&DSourceRow> {
        self.rows.first()
    }

    fn row_values(&self, index: usize) -> &[DigitalValue] {
        let start = index * self.width;
        &self.values[start..start + self.width]
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct DSourceCacheKey {
    input_file: String,
    width: usize,
    len: u64,
    modified_nanos: u128,
    content_hash: u64,
    virtual_file: bool,
}

#[derive(Debug, Clone)]
struct DSourceRowsResource {
    input_file: String,
    width: usize,
    virtual_stamp: Option<data_file::DataFileStamp>,
    rows: Option<Arc<DSourceRows>>,
}

#[derive(Debug, Default)]
struct DSourceCache {
    virtual_epoch: u64,
    entries: HashMap<DSourceCacheKey, Arc<DSourceRows>>,
}

impl DSourceCache {
    fn sync_virtual_epoch(&mut self) {
        if data_file::sync_virtual_data_file_epoch(&mut self.virtual_epoch) {
            self.entries.retain(|key, _| !key.virtual_file);
        }
    }

    #[cfg(test)]
    fn clear(&mut self) {
        self.entries.clear();
        self.virtual_epoch = data_file::virtual_data_file_epoch();
    }
}

fn d_source_cache() -> &'static Mutex<DSourceCache> {
    static CACHE: OnceLock<Mutex<DSourceCache>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(DSourceCache::default()))
}

fn lock_d_source_cache() -> MutexGuard<'static, DSourceCache> {
    d_source_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
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

fn d_source_cache_key(
    input_file: &str,
    width: usize,
    stamp: data_file::DataFileStamp,
) -> DSourceCacheKey {
    DSourceCacheKey {
        input_file: input_file.to_string(),
        width,
        len: stamp.len,
        modified_nanos: stamp.modified_nanos,
        content_hash: stamp.content_hash,
        virtual_file: stamp.virtual_file,
    }
}

fn is_digital_table_data_line(line: &str) -> bool {
    !line.starts_with('*')
        && !line
            .trim_start_matches(|ch: char| ch.is_ascii_whitespace() || ch == '*')
            .is_empty()
}

fn tokenize_digital_table_line<'line>(line: &'line str, tokens: &mut Vec<&'line str>) {
    tokens.clear();
    tokens.extend(
        line.split(|ch: char| ch.is_ascii_whitespace() || matches!(ch, '=' | '(' | ')' | ','))
            .filter(|token| !token.is_empty()),
    );
}

fn parse_d_source_token(input_file: &str, line: usize, token: &str) -> CmResult<DigitalValue> {
    let value = match token {
        "0s" => DigitalValue::new(DigitalState::Zero, DigitalStrength::Strong),
        "1s" => DigitalValue::new(DigitalState::One, DigitalStrength::Strong),
        "Us" => DigitalValue::new(DigitalState::Unknown, DigitalStrength::Strong),
        "0r" => DigitalValue::new(DigitalState::ZeroR, DigitalStrength::Resistive),
        "1r" => DigitalValue::new(DigitalState::OneR, DigitalStrength::Resistive),
        "Ur" => DigitalValue::new(DigitalState::UnknownR, DigitalStrength::Resistive),
        "0z" => DigitalValue::new(DigitalState::ZeroZ, DigitalStrength::HighZ),
        "1z" => DigitalValue::new(DigitalState::OneZ, DigitalStrength::HighZ),
        "Uz" => DigitalValue::new(DigitalState::UnknownZ, DigitalStrength::HighZ),
        "0u" => DigitalValue::new(DigitalState::Zero, DigitalStrength::Undetermined),
        "1u" => DigitalValue::new(DigitalState::One, DigitalStrength::Undetermined),
        "Uu" => DigitalValue::new(DigitalState::Unknown, DigitalStrength::Undetermined),
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

fn parse_d_source_contents(
    input_file: &str,
    width: usize,
    contents: &str,
) -> CmResult<DSourceRows> {
    let mut rows = Vec::new();
    let mut values = Vec::new();
    let mut previous_time = None;
    let mut tokens = Vec::with_capacity(width.saturating_add(1));

    for (line_idx, line) in contents.lines().enumerate() {
        let line_no = line_idx + 1;
        if !is_digital_table_data_line(line) {
            continue;
        }

        tokenize_digital_table_line(line, &mut tokens);
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

        let time = data_file::parse_ngspice_spice_value(tokens[0]);
        if !time.is_finite() {
            return Err(d_source_error(
                input_file,
                line_no,
                format!("time must be finite, got {}", tokens[0]),
            ));
        }
        if let Some(previous_time) = previous_time
            && time <= previous_time
        {
            return Err(d_source_error(
                input_file,
                line_no,
                "time values must increase monotonically",
            ));
        }
        previous_time = Some(time);

        for token in &tokens[1..] {
            values.push(parse_d_source_token(input_file, line_no, token)?);
        }
        rows.push(DSourceRow { time });
    }

    if rows.is_empty() {
        return Err(d_source_file_error(input_file, "contains no stimulus rows"));
    }

    Ok(DSourceRows {
        width,
        rows,
        values,
    })
}

fn parse_d_source_file(input_file: &str, width: usize) -> CmResult<DSourceRows> {
    let contents = data_file::read_to_string(input_file)
        .map_err(|err| d_source_file_error(input_file, err))?;
    parse_d_source_contents(input_file, width, &contents)
}

fn load_d_source_rows(
    input_file: &str,
    width: usize,
) -> (Option<data_file::DataFileStamp>, CmResult<Arc<DSourceRows>>) {
    let (contents, stamp) = match data_file::read_to_string_with_stamp(input_file) {
        Ok(file) => file,
        Err(err) => return (None, Err(d_source_file_error(input_file, err))),
    };
    let virtual_stamp = data_file::loaded_virtual_data_file_stamp(stamp);
    let key = d_source_cache_key(input_file, width, stamp);

    {
        let mut guard = lock_d_source_cache();
        guard.sync_virtual_epoch();
        if let Some(rows) = guard.entries.get(&key) {
            return (virtual_stamp, Ok(Arc::clone(rows)));
        }
    }

    let rows = match parse_d_source_contents(input_file, width, &contents) {
        Ok(rows) => Arc::new(rows),
        Err(err) => return (virtual_stamp, Err(err)),
    };
    let mut guard = lock_d_source_cache();
    guard.sync_virtual_epoch();
    guard.entries.insert(key, Arc::clone(&rows));
    (virtual_stamp, Ok(rows))
}

fn load_d_source_rows_for_context(
    ctx: &mut CmContext,
    input_file: &str,
    width: usize,
) -> CmResult<Arc<DSourceRows>> {
    if let Some(resource) = ctx.resource::<DSourceRowsResource>(D_SOURCE_ROWS_RESOURCE)
        && resource.input_file == input_file
        && resource.width == width
        && resource.virtual_stamp == data_file::virtual_data_file_stamp(input_file)
    {
        if let Some(rows) = &resource.rows {
            return Ok(Arc::clone(rows));
        }
        return Err(d_source_file_error(input_file, "previous load failed"));
    }

    let (virtual_stamp, load_result) = load_d_source_rows(input_file, width);
    match load_result {
        Ok(rows) => {
            ctx.set_resource(
                D_SOURCE_ROWS_RESOURCE,
                Arc::new(DSourceRowsResource {
                    input_file: input_file.to_string(),
                    width,
                    virtual_stamp,
                    rows: Some(Arc::clone(&rows)),
                }),
            );
            Ok(rows)
        }
        Err(err) => {
            ctx.set_resource(
                D_SOURCE_ROWS_RESOURCE,
                Arc::new(DSourceRowsResource {
                    input_file: input_file.to_string(),
                    width,
                    virtual_stamp,
                    rows: None,
                }),
            );
            Err(err)
        }
    }
}

const D_SOURCE_TIME_EPSILON: Value = 1e-18;

fn d_source_scan_start(rows: &[DSourceRow], time: Value, emitted_row: i64) -> usize {
    let Some(index) = usize::try_from(emitted_row)
        .ok()
        .filter(|&index| index < rows.len())
    else {
        return 0;
    };

    if rows[index].time <= time + D_SOURCE_TIME_EPSILON {
        index.saturating_add(1)
    } else {
        0
    }
}

fn d_source_row_indices(
    rows: &[DSourceRow],
    time: Value,
    emitted_row: i64,
) -> (Option<usize>, Option<usize>) {
    let start = d_source_scan_start(rows, time, emitted_row);
    let upper = rows[start..].partition_point(|row| row.time <= time + D_SOURCE_TIME_EPSILON);
    let next = start + upper;
    (next.checked_sub(1), (next < rows.len()).then_some(next))
}

fn d_source_set_output(ctx: &mut CmContext, values: &[DigitalValue], delay: Value) {
    if !values.is_empty() {
        ctx.set_output_digital_vector_from_slice("out", values, delay);
    }
}

fn d_source_set_row_state(ctx: &mut CmContext, index: usize, value: i64) {
    if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
        ctx.set_int_state(index, value);
    }
}

fn d_source_set_unknown_output(ctx: &mut CmContext, width: usize) {
    let value = DigitalValue::new(DigitalState::Unknown, DigitalStrength::Undetermined);
    ctx.set_output_digital_vector_from_context_fn("out", width, 0.0, |_, _| value);
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
                ParamSpec::real("input_load", 1e-12),
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
            .unwrap_or("source.txt")
            .to_string();
        let width = ctx.port_width("out");
        if ctx.int_state(D_SOURCE_EMITTED_ROW) == D_SOURCE_LOAD_FAILED {
            return Ok(());
        }
        let rows = match load_d_source_rows_for_context(ctx, &input_file, width) {
            Ok(rows) => rows,
            Err(_) => {
                d_source_set_row_state(ctx, D_SOURCE_EMITTED_ROW, D_SOURCE_LOAD_FAILED);
                d_source_set_row_state(ctx, D_SOURCE_SCHEDULED_ROW, D_SOURCE_NO_ROW);
                return Ok(());
            }
        };
        if rows.first().is_some_and(|row| row.time < 0.0) {
            if ctx.int_state(D_SOURCE_EMITTED_ROW) != D_SOURCE_BEFORE_FIRST_ROW {
                d_source_set_unknown_output(ctx, width);
                d_source_set_row_state(ctx, D_SOURCE_EMITTED_ROW, D_SOURCE_BEFORE_FIRST_ROW);
            }
            d_source_set_row_state(ctx, D_SOURCE_SCHEDULED_ROW, D_SOURCE_NO_ROW);
            return Ok(());
        }
        let emitted_row = ctx.int_state(D_SOURCE_EMITTED_ROW);
        let scheduled_row = ctx.int_state(D_SOURCE_SCHEDULED_ROW);
        let (active_idx, next_idx) = d_source_row_indices(&rows.rows, ctx.time, emitted_row);

        if let Some(idx) = active_idx {
            if emitted_row != idx as i64 {
                let row = &rows.rows[idx];
                d_source_set_output(ctx, rows.row_values(idx), row.time - ctx.time);
                d_source_set_row_state(ctx, D_SOURCE_EMITTED_ROW, idx as i64);
            }
        } else if emitted_row != D_SOURCE_BEFORE_FIRST_ROW {
            d_source_set_unknown_output(ctx, width);
            d_source_set_row_state(ctx, D_SOURCE_EMITTED_ROW, D_SOURCE_BEFORE_FIRST_ROW);
        }

        if let Some(idx) = next_idx {
            if scheduled_row != idx as i64 {
                let row = &rows.rows[idx];
                d_source_set_output(ctx, rows.row_values(idx), row.time - ctx.time);
                d_source_set_row_state(ctx, D_SOURCE_SCHEDULED_ROW, idx as i64);
            }
        } else if scheduled_row != D_SOURCE_NO_ROW {
            d_source_set_row_state(ctx, D_SOURCE_SCHEDULED_ROW, D_SOURCE_NO_ROW);
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
    content_hash: u64,
    virtual_file: bool,
}

#[derive(Debug, Clone)]
struct DStateTableResource {
    state_file: String,
    input_width: usize,
    output_width: usize,
    virtual_stamp: Option<data_file::DataFileStamp>,
    table: Option<Arc<DStateTable>>,
}

#[derive(Debug, Clone)]
struct DStateTransition {
    state: i64,
    inputs: Vec<Option<bool>>,
    input_mask: u64,
    input_bits: u64,
    next_state: i64,
}

#[derive(Debug, Clone)]
struct DStateTable {
    transitions: Vec<DStateTransition>,
    output_width: usize,
    output_values: Vec<DigitalValue>,
    state_ranges: HashMap<i64, DStateRange>,
}

impl DStateTable {
    fn row_outputs(&self, index: usize) -> Option<&[DigitalValue]> {
        let start = index.checked_mul(self.output_width)?;
        let end = start.checked_add(self.output_width)?;
        self.output_values.get(start..end)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DStateRange {
    Contiguous { start: usize, end: usize },
    NonContiguous,
}

#[derive(Debug, Default)]
struct DStateCache {
    virtual_epoch: u64,
    entries: HashMap<DStateCacheKey, Arc<DStateTable>>,
}

impl DStateCache {
    fn sync_virtual_epoch(&mut self) {
        if data_file::sync_virtual_data_file_epoch(&mut self.virtual_epoch) {
            self.entries.retain(|key, _| !key.virtual_file);
        }
    }

    #[cfg(test)]
    fn clear(&mut self) {
        self.entries.clear();
        self.virtual_epoch = data_file::virtual_data_file_epoch();
    }
}

fn d_state_cache() -> &'static Mutex<DStateCache> {
    static CACHE: OnceLock<Mutex<DStateCache>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(DStateCache::default()))
}

fn lock_d_state_cache() -> MutexGuard<'static, DStateCache> {
    d_state_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
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
    stamp: data_file::DataFileStamp,
) -> DStateCacheKey {
    DStateCacheKey {
        state_file: state_file.to_string(),
        input_width,
        output_width,
        len: stamp.len,
        modified_nanos: stamp.modified_nanos,
        content_hash: stamp.content_hash,
        virtual_file: stamp.virtual_file,
    }
}

fn parse_d_state_input_token(state_file: &str, line: usize, token: &str) -> CmResult<Option<bool>> {
    match parse_d_state_input_code(state_file, line, token)? {
        0 => Ok(Some(false)),
        1 => Ok(Some(true)),
        _ => Ok(None),
    }
}

fn parse_d_state_input_code(state_file: &str, line: usize, token: &str) -> CmResult<i64> {
    match token {
        "0" => Ok(0),
        "1" => Ok(1),
        "x" | "X" => Ok(2),
        _ => Err(d_state_error(
            state_file,
            line,
            format!("invalid input token '{token}'"),
        )),
    }
}

fn d_state_input_pattern_masks(inputs: &[Option<bool>]) -> (u64, u64) {
    if inputs.len() > 64 {
        return (0, 0);
    }

    let mut mask = 0;
    let mut bits = 0;
    for (index, input) in inputs.iter().enumerate() {
        if let Some(value) = input {
            let bit = 1_u64 << index;
            mask |= bit;
            if *value {
                bits |= bit;
            }
        }
    }
    (mask, bits)
}

fn parse_d_state_i64(state_file: &str, line: usize, token: &str) -> CmResult<i64> {
    let value = data_file::parse_ngspice_spice_value(token);
    if value.is_finite() {
        Ok(value as i64)
    } else {
        Err(d_state_error(
            state_file,
            line,
            format!("invalid integer '{token}'"),
        ))
    }
}

fn parse_d_state_output_code(state_file: &str, line: usize, token: &str) -> CmResult<i64> {
    match token {
        "0s" => Ok(0),
        "1s" => Ok(1),
        "Us" => Ok(2),
        "0r" => Ok(3),
        "1r" => Ok(4),
        "Ur" => Ok(5),
        "0z" => Ok(6),
        "1z" => Ok(7),
        "Uz" => Ok(8),
        "0u" => Ok(9),
        "1u" => Ok(10),
        "Uu" => Ok(11),
        _ => Err(d_state_error(
            state_file,
            line,
            format!("invalid digital token '{token}'"),
        )),
    }
}

fn d_state_output_from_code(code: i64) -> DigitalValue {
    match code {
        0 => DigitalValue::new(DigitalState::Zero, DigitalStrength::Strong),
        1 => DigitalValue::new(DigitalState::One, DigitalStrength::Strong),
        2 => DigitalValue::new(DigitalState::Unknown, DigitalStrength::Strong),
        3 => DigitalValue::new(DigitalState::ZeroR, DigitalStrength::Resistive),
        4 => DigitalValue::new(DigitalState::OneR, DigitalStrength::Resistive),
        5 => DigitalValue::new(DigitalState::UnknownR, DigitalStrength::Resistive),
        6 => DigitalValue::new(DigitalState::ZeroZ, DigitalStrength::HighZ),
        7 => DigitalValue::new(DigitalState::OneZ, DigitalStrength::HighZ),
        8 => DigitalValue::new(DigitalState::UnknownZ, DigitalStrength::HighZ),
        9 => DigitalValue::new(DigitalState::Zero, DigitalStrength::Undetermined),
        10 => DigitalValue::new(DigitalState::One, DigitalStrength::Undetermined),
        11 => DigitalValue::new(DigitalState::Unknown, DigitalStrength::Undetermined),
        _ => DigitalValue::unknown(),
    }
}

fn parse_d_state_file(
    state_file: &str,
    input_width: usize,
    output_width: usize,
) -> CmResult<DStateTable> {
    let contents =
        data_file::read_to_string(state_file).map_err(|err| d_state_file_error(state_file, err))?;
    parse_d_state_contents(state_file, input_width, output_width, &contents)
}

fn parse_d_state_contents(
    state_file: &str,
    input_width: usize,
    output_width: usize,
    contents: &str,
) -> CmResult<DStateTable> {
    let mut transitions = Vec::new();
    let mut output_values = Vec::new();
    let mut last_state = None;
    let mut stale_bit_code = 0;
    let mut tokens = Vec::with_capacity(output_width.saturating_add(input_width).saturating_add(3));

    for (line_idx, line) in contents.lines().enumerate() {
        let line_no = line_idx + 1;
        if !is_digital_table_data_line(line) {
            continue;
        }

        tokenize_digital_table_line(line, &mut tokens);
        let header_len = output_width + input_width + 3;
        let continuation_len = input_width + 2;
        let (state, input_start, next_idx) = if tokens.len() == header_len {
            let state = parse_d_state_i64(state_file, line_no, tokens[0])?;
            for token in &tokens[1..1 + output_width] {
                let code = parse_d_state_output_code(state_file, line_no, token)?;
                stale_bit_code = code;
                output_values.push(d_state_output_from_code(code));
            }
            last_state = Some(state);
            (state, 1 + output_width, header_len - 1)
        } else if tokens.len() == continuation_len {
            let state = last_state.ok_or_else(|| {
                d_state_error(
                    state_file,
                    line_no,
                    "continuation row appears before any state header",
                )
            })?;
            output_values.extend(std::iter::repeat_n(
                d_state_output_from_code(stale_bit_code),
                output_width,
            ));
            (state, 0, continuation_len - 1)
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

        let mut inputs = Vec::with_capacity(input_width);
        for token in &tokens[input_start..input_start + input_width] {
            let input_code = parse_d_state_input_code(state_file, line_no, token)?;
            stale_bit_code = input_code;
            inputs.push(match input_code {
                0 => Some(false),
                1 => Some(true),
                _ => None,
            });
        }
        let next_state = parse_d_state_i64(state_file, line_no, tokens[next_idx])?;
        let (input_mask, input_bits) = d_state_input_pattern_masks(&inputs);
        transitions.push(DStateTransition {
            state,
            inputs,
            input_mask,
            input_bits,
            next_state,
        });
    }

    if transitions.is_empty() {
        return Err(d_state_file_error(state_file, "contains no state rows"));
    }

    Ok(DStateTable {
        state_ranges: d_state_range_index(&transitions),
        output_width,
        output_values,
        transitions,
    })
}

fn d_state_range_index(transitions: &[DStateTransition]) -> HashMap<i64, DStateRange> {
    let mut ranges = HashMap::new();

    for (idx, row) in transitions.iter().enumerate() {
        match ranges.get_mut(&row.state) {
            Some(DStateRange::Contiguous { end, .. }) if idx == *end + 1 => {
                *end = idx;
            }
            Some(DStateRange::Contiguous { .. }) => {
                ranges.insert(row.state, DStateRange::NonContiguous);
            }
            Some(DStateRange::NonContiguous) => {}
            None => {
                ranges.insert(
                    row.state,
                    DStateRange::Contiguous {
                        start: idx,
                        end: idx,
                    },
                );
            }
        }
    }

    ranges
}

fn load_d_state_table(
    state_file: &str,
    input_width: usize,
    output_width: usize,
) -> (
    Option<data_file::DataFileStamp>,
    CmResult<Option<Arc<DStateTable>>>,
) {
    let (contents, stamp) = match data_file::read_to_string_with_stamp(state_file) {
        Ok(file) => file,
        Err(err) => return (None, Err(d_state_file_error(state_file, err))),
    };
    let virtual_stamp = data_file::loaded_virtual_data_file_stamp(stamp);
    let key = d_state_cache_key(state_file, input_width, output_width, stamp);

    {
        let mut guard = lock_d_state_cache();
        guard.sync_virtual_epoch();
        if let Some(table) = guard.entries.get(&key) {
            return (virtual_stamp, Ok(Some(Arc::clone(table))));
        }
    }

    let table = match parse_d_state_contents(state_file, input_width, output_width, &contents) {
        Ok(table) => Arc::new(table),
        Err(_) => return (virtual_stamp, Ok(None)),
    };
    let mut guard = lock_d_state_cache();
    guard.sync_virtual_epoch();
    guard.entries.insert(key, Arc::clone(&table));
    (virtual_stamp, Ok(Some(table)))
}

fn load_d_state_table_for_context(
    ctx: &mut CmContext,
    state_file: &str,
    input_width: usize,
    output_width: usize,
) -> CmResult<Option<Arc<DStateTable>>> {
    if let Some(resource) = ctx.resource::<DStateTableResource>(D_STATE_TABLE_RESOURCE)
        && resource.state_file == state_file
        && resource.input_width == input_width
        && resource.output_width == output_width
        && resource.virtual_stamp == data_file::virtual_data_file_stamp(state_file)
    {
        return Ok(resource.table.clone());
    }

    let (virtual_stamp, table) = load_d_state_table(state_file, input_width, output_width);
    let table = table?;
    ctx.set_resource(
        D_STATE_TABLE_RESOURCE,
        Arc::new(DStateTableResource {
            state_file: state_file.to_string(),
            input_width,
            output_width,
            virtual_stamp,
            table: table.clone(),
        }),
    );
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

fn d_state_contiguous_range(table: &DStateTable, state: i64) -> Option<(usize, usize)> {
    match table.state_ranges.get(&state) {
        Some(DStateRange::Contiguous { start, end }) => Some((*start, *end)),
        Some(DStateRange::NonContiguous) => None,
        None => (!table.transitions.is_empty()).then_some((0, 0)),
    }
}

fn d_state_outputs(table: &DStateTable, state: i64) -> Option<&[DigitalValue]> {
    let (start, _) = d_state_contiguous_range(table, state)?;
    table.row_outputs(start)
}

fn d_state_input_value_masks(inputs: &[DigitalValue]) -> Option<(u64, u64)> {
    if inputs.len() > 64 {
        return None;
    }

    let mut known_mask = 0;
    let mut bits = 0;
    for (index, input) in inputs.iter().enumerate() {
        if let Some(value) = d_state_logic(*input) {
            let bit = 1_u64 << index;
            known_mask |= bit;
            if value {
                bits |= bit;
            }
        }
    }
    Some((known_mask, bits))
}

fn d_state_transition_matches(
    row: &DStateTransition,
    inputs: &[DigitalValue],
    input_masks: Option<(u64, u64)>,
) -> bool {
    if inputs.len() >= row.inputs.len()
        && let Some((known_mask, input_bits)) = input_masks
    {
        return (known_mask & row.input_mask) == row.input_mask
            && (input_bits & row.input_mask) == row.input_bits;
    }

    row.inputs
        .iter()
        .zip(inputs.iter())
        .all(|(pattern, value)| {
            pattern.is_none_or(|expected| d_state_logic(*value) == Some(expected))
        })
}

fn d_state_next(table: &DStateTable, state: i64, inputs: &[DigitalValue]) -> Option<i64> {
    let (start, end) = d_state_contiguous_range(table, state)?;
    let input_masks = d_state_input_value_masks(inputs);
    table
        .transitions
        .get(start..=end)?
        .iter()
        .find(|row| d_state_transition_matches(row, inputs, input_masks))
        .map(|row| row.next_state)
}

fn d_state_set_int_state(ctx: &mut CmContext, index: usize, value: i64) {
    if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
        ctx.set_int_state(index, value);
    }
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
                PortSpec::vector_input("in", PortType::Digital).nullable(),
                PortSpec::input("clk", PortType::Digital),
                PortSpec::input("reset", PortType::Digital).nullable(),
                PortSpec::vector_output("out", PortType::Digital).with_vector_min_len(1),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("clk_delay", 1.0e-9),
                ParamSpec::real("reset_delay", 1.0e-9),
                ParamSpec::string("state_file", "state.txt"),
                ParamSpec::integer("reset_state", 0),
                ParamSpec::real("input_load", 1.0e-12),
                ParamSpec::real("clk_load", 1.0e-12),
                ParamSpec::real("reset_load", 1.0e-12),
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
            .unwrap_or("state.txt")
            .to_string();
        let input_width = ctx.port_width("in");
        let output_width = ctx.port_width("out").max(1);
        let Some(table) =
            load_d_state_table_for_context(ctx, &state_file, input_width, output_width)?
        else {
            return Ok(());
        };
        let reset_state = ctx.param_or("reset_state", 0.0) as i64;
        let clk_delay = ctx.param_or("clk_delay", 1.0e-9);
        let reset_delay = ctx.param_or("reset_delay", 1.0e-9);

        if ctx.time == 0.0 || ctx.int_state(0) == 0 {
            d_state_set_int_state(ctx, 0, 1);
            d_state_set_int_state(ctx, 1, reset_state);
            d_state_set_int_state(ctx, 2, 0);
            d_state_set_int_state(ctx, 3, 0);
            let Some(outputs) = d_state_outputs(&table, reset_state) else {
                return Ok(());
            };
            ctx.set_output_digital_vector_from_slice("out", outputs, 0.0);
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
                if let Some(outputs) = d_state_outputs(&table, current_state) {
                    ctx.set_output_digital_vector_from_slice("out", outputs, reset_delay);
                } else {
                    d_state_set_int_state(ctx, 1, current_state);
                    d_state_set_int_state(ctx, 2, clk_state);
                    d_state_set_int_state(ctx, 3, reset_state_code);
                    return Ok(());
                }
            }
        } else if reset_state_code != 1 && clk_state != prev_clk && clk_state == 1 {
            let inputs = ctx.input_digital_vector_values("in").unwrap_or(&[]);
            if let Some(next_state) = d_state_next(&table, current_state, inputs) {
                current_state = next_state;
                if let Some(outputs) = d_state_outputs(&table, current_state) {
                    ctx.set_output_digital_vector_from_slice("out", outputs, clk_delay);
                } else {
                    d_state_set_int_state(ctx, 1, current_state);
                    d_state_set_int_state(ctx, 2, clk_state);
                    d_state_set_int_state(ctx, 3, reset_state_code);
                    return Ok(());
                }
            }
        }

        d_state_set_int_state(ctx, 1, current_state);
        d_state_set_int_state(ctx, 2, clk_state);
        d_state_set_int_state(ctx, 3, reset_state_code);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::context::InputValue;
    use crate::xspice::{DigitalState, DigitalStrength, EvaluationPhase};
    use std::sync::{Mutex, MutexGuard, OnceLock};

    fn data_file_test_guard() -> MutexGuard<'static, ()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("data-file test guard")
    }

    fn unregister_test_data_file(path: &str) {
        let _ = data_file::unregister_data_file(path);
    }

    fn poison_d_source_cache_lock() {
        let result = std::panic::catch_unwind(|| {
            let _guard = lock_d_source_cache();
            panic!("poison d_source cache lock for recovery test");
        });
        assert!(result.is_err(), "recovery test must poison the mutex");
    }

    fn poison_d_state_cache_lock() {
        let result = std::panic::catch_unwind(|| {
            let _guard = lock_d_state_cache();
            panic!("poison d_state cache lock for recovery test");
        });
        assert!(result.is_err(), "recovery test must poison the mutex");
    }

    #[test]
    fn digital_table_tokenizer_reuses_buffer_for_cnvgettok_separators() {
        let mut tokens = Vec::new();

        tokenize_digital_table_line(" 1n = (1s, 0s) ", &mut tokens);
        assert_eq!(tokens, vec!["1n", "1s", "0s"]);

        tokenize_digital_table_line("2n,Uz", &mut tokens);
        assert_eq!(
            tokens,
            vec!["2n", "Uz"],
            "tokenizing a shorter row must clear stale tokens from the reusable buffer"
        );
    }

    #[test]
    fn digital_file_backed_cache_locks_recover_after_poison() {
        poison_d_source_cache_lock();
        lock_d_source_cache().clear();

        poison_d_state_cache_lock();
        lock_d_state_cache().clear();
    }

    #[test]
    fn d_source_context_resource_reloads_when_virtual_file_changes() {
        let _guard = data_file_test_guard();
        let input_file = "virtual://d_source/context-resource-reload";
        unregister_test_data_file(input_file);
        data_file::register_data_file(input_file, "0 0s\n1n 1s\n")
            .expect("register first virtual d_source data");

        let mut ctx = CmContext::new();
        let first = load_d_source_rows_for_context(&mut ctx, input_file, 1)
            .expect("load first d_source rows");
        assert_eq!(first.row_values(1), &[DigitalValue::one()][..]);

        data_file::register_data_file(input_file, "0 0s\n1n 0s\n")
            .expect("replace virtual d_source data");
        let second = load_d_source_rows_for_context(&mut ctx, input_file, 1)
            .expect("reload replaced d_source rows");
        assert_eq!(second.row_values(1), &[DigitalValue::zero()][..]);

        unregister_test_data_file(input_file);
    }

    #[test]
    fn d_state_context_resource_reloads_when_virtual_file_changes() {
        let _guard = data_file_test_guard();
        let state_file = "virtual://d_state/context-resource-reload";
        unregister_test_data_file(state_file);
        data_file::register_data_file(state_file, "0 0s 0 -> 0\n")
            .expect("register first virtual d_state table");

        let mut ctx = CmContext::new();
        let first = load_d_state_table_for_context(&mut ctx, state_file, 1, 1)
            .expect("load first d_state table")
            .expect("first d_state table parses");
        assert_eq!(first.row_outputs(0), Some(&[DigitalValue::zero()][..]));

        data_file::register_data_file(state_file, "0 1s 0 -> 0\n")
            .expect("replace virtual d_state table");
        let second = load_d_state_table_for_context(&mut ctx, state_file, 1, 1)
            .expect("reload replaced d_state table")
            .expect("second d_state table parses");
        assert_eq!(second.row_outputs(0), Some(&[DigitalValue::one()][..]));

        unregister_test_data_file(state_file);
    }

    #[test]
    fn digital_file_backed_caches_retire_replaced_virtual_file_entries() {
        let _guard = data_file_test_guard();
        let input_file = "virtual://d_source/cache-retention";
        let state_file = "virtual://d_state/cache-retention";
        unregister_test_data_file(input_file);
        unregister_test_data_file(state_file);
        lock_d_source_cache().clear();
        lock_d_state_cache().clear();

        data_file::register_data_file(input_file, "0 0s\n1n 1s\n")
            .expect("register first virtual d_source data");
        let (_, first_source) = load_d_source_rows(input_file, 1);
        let first_source = first_source.expect("load first virtual d_source data");
        assert_eq!(first_source.row_values(1), &[DigitalValue::one()][..]);

        data_file::register_data_file(input_file, "0 0s\n1n 0s\n")
            .expect("replace virtual d_source data");
        let (_, second_source) = load_d_source_rows(input_file, 1);
        let second_source = second_source.expect("load replaced virtual d_source data");
        assert_eq!(second_source.row_values(1), &[DigitalValue::zero()][..]);

        data_file::register_data_file(state_file, "0 0s 0 -> 0\n")
            .expect("register first virtual d_state data");
        let (_, first_state) = load_d_state_table(state_file, 1, 1);
        let first_state = first_state
            .expect("load first virtual d_state data")
            .expect("first virtual d_state data parses");
        assert_eq!(
            first_state.row_outputs(0),
            Some(&[DigitalValue::zero()][..])
        );

        data_file::register_data_file(state_file, "0 1s 0 -> 0\n")
            .expect("replace virtual d_state data");
        let (_, second_state) = load_d_state_table(state_file, 1, 1);
        let second_state = second_state
            .expect("load replaced virtual d_state data")
            .expect("replaced virtual d_state data parses");
        assert_eq!(
            second_state.row_outputs(0),
            Some(&[DigitalValue::one()][..])
        );

        let cached_d_source_entries = {
            let guard = lock_d_source_cache();
            guard
                .entries
                .keys()
                .filter(|key| key.input_file == input_file)
                .count()
        };
        assert_eq!(cached_d_source_entries, 1);

        let cached_d_state_entries = {
            let guard = lock_d_state_cache();
            guard
                .entries
                .keys()
                .filter(|key| key.state_file == state_file)
                .count()
        };
        assert_eq!(cached_d_state_entries, 1);

        unregister_test_data_file(input_file);
        unregister_test_data_file(state_file);
    }

    #[test]
    fn d_source_before_first_row_drives_unknown_undetermined_like_ngspice() {
        let _guard = data_file_test_guard();
        let input_file = "virtual://d_source/before-first-row";
        unregister_test_data_file(input_file);
        data_file::register_data_file(input_file, "1n 1s\n")
            .expect("register virtual d_source data");

        let model = DigitalSource;
        let mut ctx = CmContext::new();
        ctx.set_port_width("out", 1);
        ctx.set_string_param("input_file", input_file);
        model.init(&mut ctx).expect("d_source init");
        model.evaluate(&mut ctx).expect("d_source evaluate");

        let expected = DigitalValue::new(DigitalState::Unknown, DigitalStrength::Undetermined);
        let events = ctx.take_pending_events();
        assert!(
            events
                .iter()
                .any(|event| event.delay == 0.0 && event.values == vec![expected]),
            "ngspice emits UNKNOWN/UNDETERMINED before the first nonzero stimulus row, got {events:?}"
        );

        unregister_test_data_file(input_file);
    }

    #[test]
    fn d_source_malformed_file_returns_only_initial_state_like_ngspice() {
        let _guard = data_file_test_guard();
        let input_file = "virtual://d_source/malformed";
        unregister_test_data_file(input_file);
        data_file::register_data_file(input_file, "0 0s\n1n 1s\n1n 0s\n")
            .expect("register malformed virtual d_source data");

        let model = DigitalSource;
        let mut ctx = CmContext::new();
        ctx.set_port_width("out", 1);
        ctx.set_string_param("input_file", input_file);
        model.init(&mut ctx).expect("d_source init");

        model
            .evaluate(&mut ctx)
            .expect("ngspice logs malformed d_source files but does not fail evaluation");

        let events = ctx.take_pending_events();
        assert!(
            events.is_empty(),
            "malformed d_source files should leave the source at its initial state, got {events:?}"
        );

        unregister_test_data_file(input_file);
    }

    #[test]
    fn d_source_zero_width_out_accepts_time_only_rows_like_ngspice() {
        let _guard = data_file_test_guard();
        let input_file = "virtual://d_source/zero-width";
        unregister_test_data_file(input_file);
        data_file::register_data_file(input_file, "0\n1n\n")
            .expect("register zero-width virtual d_source data");

        let model = DigitalSource;
        let mut ctx = CmContext::new();
        ctx.set_port_width("out", 0);
        ctx.set_string_param("input_file", input_file);
        model.init(&mut ctx).expect("d_source init");

        model.evaluate(&mut ctx).expect("d_source evaluate at t=0");
        assert_eq!(ctx.int_state(D_SOURCE_EMITTED_ROW), 0);
        assert_eq!(ctx.int_state(D_SOURCE_SCHEDULED_ROW), 1);
        assert!(
            ctx.take_pending_events().is_empty(),
            "zero-width d_source rows should not fabricate output events"
        );

        ctx.time = 1.0e-9;
        model
            .evaluate(&mut ctx)
            .expect("d_source evaluate at next row");
        assert_eq!(ctx.int_state(D_SOURCE_EMITTED_ROW), 1);
        assert_eq!(ctx.int_state(D_SOURCE_SCHEDULED_ROW), D_SOURCE_NO_ROW);
        assert!(
            ctx.take_pending_events().is_empty(),
            "zero-width d_source rows should remain output-event free"
        );

        unregister_test_data_file(input_file);
    }

    #[test]
    fn d_source_parses_row_values_into_contiguous_storage() {
        let rows =
            parse_d_source_contents("virtual://d_source/flat-values", 2, "0 0s 1s\n1n Us 0r\n")
                .expect("d_source table parses");

        assert_eq!(rows.len(), 2);
        assert_eq!(rows.values.len(), 4);
        assert_eq!(
            rows.row_values(0),
            &[DigitalValue::zero(), DigitalValue::one()][..]
        );
        assert_eq!(
            rows.row_values(1),
            &[
                DigitalValue::unknown(),
                DigitalValue::new(DigitalState::ZeroR, DigitalStrength::Resistive),
            ][..]
        );
    }

    #[test]
    fn d_source_row_lookup_uses_emitted_cursor_when_time_is_monotonic() {
        let rows = vec![
            DSourceRow { time: 0.0 },
            DSourceRow { time: 1.0e-9 },
            DSourceRow { time: 2.0e-9 },
            DSourceRow { time: 3.0e-9 },
        ];

        assert_eq!(d_source_scan_start(&rows, 2.5e-9, 1), 2);
        assert_eq!(d_source_row_indices(&rows, 2.5e-9, 1), (Some(2), Some(3)));
        assert_eq!(d_source_row_indices(&rows, 4.0e-9, 3), (Some(3), None));
        assert_eq!(d_source_row_indices(&rows, 3.5e-9, 0), (Some(3), None));

        assert_eq!(d_source_scan_start(&rows, 0.5e-9, 2), 0);
        assert_eq!(d_source_row_indices(&rows, 0.5e-9, 2), (Some(0), Some(1)));
        assert_eq!(
            d_source_row_indices(&rows, -1.0e-9, D_SOURCE_BEFORE_FIRST_ROW),
            (None, Some(0))
        );
    }

    #[test]
    fn d_source_rollbackable_probe_does_not_commit_row_cursors() {
        let _guard = data_file_test_guard();
        let input_file = "virtual://d_source/rollback-cursors";
        unregister_test_data_file(input_file);
        data_file::register_data_file(input_file, "0 0s\n1n 1s\n")
            .expect("register virtual d_source data");

        let model = DigitalSource;
        let mut ctx = CmContext::new();
        ctx.set_port_width("out", 1);
        ctx.set_string_param("input_file", input_file);
        model.init(&mut ctx).expect("d_source init");
        model.evaluate(&mut ctx).expect("d_source evaluate at t=0");
        let _ = ctx.take_pending_events();

        assert_eq!(ctx.int_state(D_SOURCE_EMITTED_ROW), 0);
        assert_eq!(ctx.int_state(D_SOURCE_SCHEDULED_ROW), 1);

        ctx.time = 1.0e-9;
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        model
            .evaluate(&mut ctx)
            .expect("d_source rollback probe evaluates");
        let events = ctx.take_pending_events();
        assert!(
            events
                .iter()
                .any(|event| event.delay == 0.0 && event.values == vec![DigitalValue::one()]),
            "rollbackable probe should still expose the trial row event, got {events:?}"
        );

        assert_eq!(
            ctx.int_state(D_SOURCE_EMITTED_ROW),
            0,
            "rollbackable probe must not commit emitted-row state"
        );
        assert_eq!(
            ctx.int_state(D_SOURCE_SCHEDULED_ROW),
            1,
            "rollbackable probe must not commit scheduled-row state"
        );

        ctx.set_evaluation_phase(EvaluationPhase::DirectEvaluation);
        model
            .evaluate(&mut ctx)
            .expect("d_source direct evaluation after probe");
        assert_eq!(ctx.int_state(D_SOURCE_EMITTED_ROW), 1);
        assert_eq!(ctx.int_state(D_SOURCE_SCHEDULED_ROW), D_SOURCE_NO_ROW);

        unregister_test_data_file(input_file);
    }

    #[test]
    fn d_state_continuation_outputs_use_stale_bit_value_like_ngspice() {
        let _guard = data_file_test_guard();
        let state_file = "virtual://d_state/continuation-stale-bit";
        unregister_test_data_file(state_file);
        data_file::register_data_file(state_file, "0 1s 0 -> 0\n1 -> 0\n0 -> 0\n")
            .expect("register virtual d_state table");

        let table = parse_d_state_file(state_file, 1, 1).expect("state table parses");

        assert_eq!(table.transitions.len(), 3);
        assert_eq!(table.output_values.len(), 3);
        assert_eq!(table.row_outputs(0), Some(&[DigitalValue::one()][..]));
        assert_eq!(table.row_outputs(1), Some(&[DigitalValue::zero()][..]));
        assert_eq!(table.row_outputs(2), Some(&[DigitalValue::one()][..]));

        unregister_test_data_file(state_file);
    }

    #[test]
    fn d_state_multi_output_continuation_repeats_last_input_bit_like_ngspice() {
        let _guard = data_file_test_guard();
        let state_file = "virtual://d_state/multi-output-continuation-stale-bit";
        unregister_test_data_file(state_file);
        data_file::register_data_file(state_file, "0 0s 0s 1 -> 0\n0 -> 0\n")
            .expect("register virtual d_state table");

        let table = parse_d_state_file(state_file, 1, 2).expect("state table parses");

        assert_eq!(table.transitions.len(), 2);
        assert_eq!(
            table.output_values,
            vec![
                DigitalValue::zero(),
                DigitalValue::zero(),
                DigitalValue::one(),
                DigitalValue::one()
            ]
        );
        assert_eq!(
            table.row_outputs(0),
            Some(&[DigitalValue::zero(), DigitalValue::zero()][..])
        );
        assert_eq!(
            table.row_outputs(1),
            Some(&[DigitalValue::one(), DigitalValue::one()][..])
        );

        unregister_test_data_file(state_file);
    }

    #[test]
    fn d_state_noncontiguous_active_state_returns_without_output_like_ngspice() {
        let _guard = data_file_test_guard();
        let state_file = "virtual://d_state/noncontiguous-state";
        unregister_test_data_file(state_file);
        data_file::register_data_file(state_file, "0 1s 0 -> 0\n1 0s 0 -> 1\n0 0s 1 -> 0\n")
            .expect("register virtual d_state table");

        let table = parse_d_state_file(state_file, 1, 1).expect("state table parses");

        assert_eq!(
            d_state_outputs(&table, 0),
            None,
            "ngspice returns from d_state when active state rows are non-contiguous"
        );
        assert_eq!(
            d_state_next(&table, 0, &[DigitalValue::zero()]),
            None,
            "non-contiguous active state rows must not participate in transition matching"
        );
        assert_eq!(
            d_state_outputs(&table, 1),
            Some(&[DigitalValue::zero()][..])
        );

        unregister_test_data_file(state_file);
    }

    #[test]
    fn d_state_missing_active_state_uses_first_row_like_ngspice() {
        let _guard = data_file_test_guard();
        let state_file = "virtual://d_state/missing-state-row-zero-fallback";
        unregister_test_data_file(state_file);
        data_file::register_data_file(state_file, "0 1s 0 -> 2\n")
            .expect("register virtual d_state table");

        let table = parse_d_state_file(state_file, 1, 1).expect("state table parses");

        assert_eq!(d_state_outputs(&table, 1), Some(&[DigitalValue::one()][..]));
        assert_eq!(d_state_next(&table, 1, &[DigitalValue::zero()]), Some(2));

        unregister_test_data_file(state_file);
    }

    #[test]
    fn d_state_table_indexes_contiguous_and_noncontiguous_state_ranges() {
        let _guard = data_file_test_guard();
        let state_file = "virtual://d_state/indexed-ranges";
        unregister_test_data_file(state_file);
        data_file::register_data_file(
            state_file,
            "0 1s 0 -> 0\n1 -> 0\n1 0s 0 -> 1\n0 0s 1 -> 0\n",
        )
        .expect("register virtual d_state table");

        let table = parse_d_state_file(state_file, 1, 1).expect("state table parses");

        assert_eq!(
            table.state_ranges.get(&0),
            Some(&DStateRange::NonContiguous)
        );
        assert_eq!(
            table.state_ranges.get(&1),
            Some(&DStateRange::Contiguous { start: 2, end: 2 })
        );
        assert_eq!(
            d_state_outputs(&table, 2),
            Some(&[DigitalValue::one()][..]),
            "missing states keep ngspice's first-row fallback"
        );

        unregister_test_data_file(state_file);
    }

    #[test]
    fn d_state_cached_input_masks_match_wildcards_and_unknowns() {
        let _guard = data_file_test_guard();
        let state_file = "virtual://d_state/masked-inputs";
        unregister_test_data_file(state_file);
        data_file::register_data_file(state_file, "0 0s 1 x -> 1\n1 1 -> 2\nx x -> 3\n")
            .expect("register virtual d_state table");

        let table = parse_d_state_file(state_file, 2, 1).expect("state table parses");

        assert_eq!(table.transitions[0].input_mask, 0b01);
        assert_eq!(table.transitions[0].input_bits, 0b01);
        assert_eq!(table.transitions[1].input_mask, 0b11);
        assert_eq!(table.transitions[1].input_bits, 0b11);
        assert_eq!(table.transitions[2].input_mask, 0);
        assert_eq!(table.transitions[2].input_bits, 0);

        assert_eq!(
            d_state_next(&table, 0, &[DigitalValue::one(), DigitalValue::unknown()]),
            Some(1),
            "wildcard columns must ignore unknown input values"
        );
        assert_eq!(
            d_state_next(&table, 0, &[DigitalValue::unknown(), DigitalValue::one()]),
            Some(3),
            "unknown required inputs must not satisfy a concrete pattern"
        );
        assert_eq!(
            d_state_next(&table, 0, &[DigitalValue::one(), DigitalValue::one()]),
            Some(1),
            "mask matching must preserve first-row transition priority"
        );
        assert_eq!(
            d_state_next(&table, 0, &[DigitalValue::one()]),
            Some(1),
            "short direct-call inputs keep the legacy zip-based fallback semantics"
        );

        unregister_test_data_file(state_file);
    }

    #[test]
    fn d_state_invalid_output_token_reports_state_file_context() {
        let _guard = data_file_test_guard();
        let state_file = "virtual://d_state/bad-output-token";
        unregister_test_data_file(state_file);
        data_file::register_data_file(state_file, "0 bogus 0 -> 0\n")
            .expect("register malformed virtual d_state table");

        let err = parse_d_state_file(state_file, 1, 1)
            .expect_err("invalid d_state output token should be rejected");
        let message = err.to_string();

        assert!(
            message.contains("d_state state_file"),
            "error should name d_state state_file context, got {message}"
        );
        assert!(
            !message.contains("d_source input_file"),
            "d_state parse errors must not be attributed to d_source, got {message}"
        );

        unregister_test_data_file(state_file);
    }

    #[test]
    fn d_state_malformed_file_returns_without_fatal_error_like_ngspice() {
        let _guard = data_file_test_guard();
        let state_file = "virtual://d_state/malformed";
        unregister_test_data_file(state_file);
        data_file::register_data_file(state_file, " * indented comment\n0 1s bogus 0\n")
            .expect("register malformed virtual d_state data");

        let model = DigitalStateMachine;
        let mut ctx = CmContext::new();
        ctx.set_port_width("in", 0);
        ctx.set_port_width("out", 1);
        ctx.set_string_param("state_file", state_file);
        model.init(&mut ctx).expect("d_state init");

        model
            .evaluate(&mut ctx)
            .expect("ngspice logs malformed d_state files but does not fail evaluation");

        assert!(
            ctx.take_pending_events().is_empty(),
            "malformed d_state files should return before scheduling output events"
        );

        unregister_test_data_file(state_file);
    }

    #[test]
    fn d_state_rollbackable_probe_does_not_commit_current_state_or_edges() {
        let _guard = data_file_test_guard();
        let state_file = "virtual://d_state/rollback-state";
        unregister_test_data_file(state_file);
        data_file::register_data_file(state_file, "0 0s 1 -> 1\n1 1s 0 -> 0\n")
            .expect("register virtual d_state table");

        let model = DigitalStateMachine;
        let mut ctx = CmContext::new();
        ctx.set_port_width("in", 1);
        ctx.set_port_width("out", 1);
        ctx.set_string_param("state_file", state_file);
        ctx.set_param("clk_delay", 2.0e-9);
        ctx.set_input("in", InputValue::DigitalVector(vec![DigitalValue::one()]));
        ctx.set_input_digital("clk", DigitalValue::zero());
        ctx.set_input_digital("reset", DigitalValue::zero());
        model.init(&mut ctx).expect("d_state init");

        model.evaluate(&mut ctx).expect("d_state initializes");
        let _ = ctx.take_pending_events();
        assert_eq!(ctx.int_state(1), 0);
        assert_eq!(ctx.int_state(2), 0);

        ctx.time = 1.0e-9;
        ctx.set_input_digital("clk", DigitalValue::one());
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        model
            .evaluate(&mut ctx)
            .expect("d_state rollback probe evaluates");
        let events = ctx.take_pending_events();
        assert!(
            events
                .iter()
                .any(|event| event.delay == 2.0e-9 && event.values == vec![DigitalValue::one()]),
            "rollbackable clock probe should expose the trial transition output, got {events:?}"
        );
        assert_eq!(
            ctx.int_state(1),
            0,
            "rollbackable clock probe must not commit current state"
        );
        assert_eq!(
            ctx.int_state(2),
            0,
            "rollbackable clock probe must not commit previous clock state"
        );

        ctx.set_evaluation_phase(EvaluationPhase::DirectEvaluation);
        model
            .evaluate(&mut ctx)
            .expect("d_state direct evaluation after probe");
        assert_eq!(ctx.int_state(1), 1);
        assert_eq!(ctx.int_state(2), 1);

        unregister_test_data_file(state_file);
    }
}

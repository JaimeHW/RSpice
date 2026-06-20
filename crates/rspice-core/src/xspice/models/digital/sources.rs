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

fn d_source_values_at(
    rows: &[DSourceRow],
    time: Value,
    width: usize,
) -> (Value, Vec<DigitalValue>) {
    const TIME_EPSILON: Value = 1e-18;
    let mut selected = None;
    for row in rows {
        if row.time <= time + TIME_EPSILON {
            selected = Some(row);
        } else {
            break;
        }
    }

    selected
        .map(|row| (row.time, row.values.clone()))
        .unwrap_or_else(|| (time, vec![DigitalValue::unknown(); width]))
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

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let input_file = ctx
            .string_param("input_file")
            .filter(|path| !path.trim().is_empty())
            .unwrap_or("source.txt");
        let width = ctx.port_width("out").max(1);
        let rows = load_d_source_rows(input_file, width)?;
        let (event_time, values) = d_source_values_at(&rows, ctx.time, width);
        ctx.set_output_digital_vector("out", values, event_time - ctx.time);
        Ok(())
    }
}

/// Digital state machine
#[derive(Debug, Default)]
pub struct DigitalStateMachine;

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
        PARAMS.get_or_init(|| vec![ParamSpec::string("state_file", "").required()])
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
    fn evaluate(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }
}

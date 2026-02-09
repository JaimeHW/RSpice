//! Cadence PSF native binary parsing.
//!
//! This module provides a local parser for Cadence/Spectre PSF binary payloads
//! so waveform import does not depend on external crates.

use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum CadencePsfValue {
    Int(i64),
    Real(f64),
    Text(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedRealSignal {
    pub name: String,
    pub values: Vec<f64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedComplexSignal {
    pub name: String,
    pub values: Vec<(f64, f64)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedCadencePsfBinary {
    pub header: HashMap<String, CadencePsfValue>,
    pub sweeps: Vec<NamedRealSignal>,
    pub real_signals: Vec<NamedRealSignal>,
    pub complex_signals: Vec<NamedComplexSignal>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CadencePsfError {
    message: String,
}

impl CadencePsfError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for CadencePsfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CadencePsfError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SectionKind {
    Header,
    Type,
    Sweep,
    Trace,
    Value,
}

impl SectionKind {
    fn from_u32(value: u32) -> Result<Self, CadencePsfError> {
        match value {
            0 => Ok(Self::Header),
            1 => Ok(Self::Type),
            2 => Ok(Self::Sweep),
            3 => Ok(Self::Trace),
            4 => Ok(Self::Value),
            other => Err(CadencePsfError::new(format!(
                "unexpected section kind id {}",
                other
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct TocEntry {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone)]
struct Toc {
    entries: HashMap<SectionKind, TocEntry>,
}

impl Toc {
    fn section(&self, kind: SectionKind) -> Result<TocEntry, CadencePsfError> {
        self.entries
            .get(&kind)
            .copied()
            .ok_or_else(|| CadencePsfError::new(format!("missing {:?} section in TOC", kind)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DataType {
    Real,
    Complex,
    Other(u32),
}

impl DataType {
    fn from_u32(value: u32) -> Self {
        match value {
            11 => Self::Real,
            12 => Self::Complex,
            other => Self::Other(other),
        }
    }
}

#[derive(Debug, Clone)]
struct SignalRef {
    id: u32,
    name: String,
    type_id: u32,
}

#[derive(Debug, Clone)]
enum TraceDef {
    Signal(SignalRef),
    Group(Vec<SignalRef>),
}

#[derive(Debug, Clone)]
enum SignalValues {
    Real(Vec<f64>),
    Complex(Vec<(f64, f64)>),
}

pub fn parse_cadence_psf_binary(data: &[u8]) -> Result<ParsedCadencePsfBinary, CadencePsfError> {
    let toc = parse_toc(data)?;
    let header = parse_header(data, toc.section(SectionKind::Header)?)?;
    let types = parse_types(data, toc.section(SectionKind::Type)?)?;
    let sweeps = parse_sweeps(data, toc.section(SectionKind::Sweep)?)?;
    let traces = parse_traces(data, toc.section(SectionKind::Trace)?)?;
    let signal_values = parse_values(
        data,
        toc.section(SectionKind::Value)?,
        &header,
        &types,
        &sweeps,
        &traces,
    )?;

    let flat_traces = flatten_traces(&traces);
    let mut real_signals = Vec::new();
    let mut complex_signals = Vec::new();

    for signal in flat_traces {
        match signal_values.get(&signal.id) {
            Some(SignalValues::Real(values)) => real_signals.push(NamedRealSignal {
                name: signal.name.clone(),
                values: values.clone(),
            }),
            Some(SignalValues::Complex(values)) => complex_signals.push(NamedComplexSignal {
                name: signal.name.clone(),
                values: values.clone(),
            }),
            None => {}
        }
    }

    let mut sweep_signals = Vec::new();
    for sweep in &sweeps {
        if let Some(SignalValues::Real(values)) = signal_values.get(&sweep.id) {
            sweep_signals.push(NamedRealSignal {
                name: sweep.name.clone(),
                values: values.clone(),
            });
        }
    }

    Ok(ParsedCadencePsfBinary {
        header,
        sweeps: sweep_signals,
        real_signals,
        complex_signals,
    })
}

fn parse_toc(data: &[u8]) -> Result<Toc, CadencePsfError> {
    if data.len() < 12 {
        return Err(CadencePsfError::new(
            "PSF binary payload too small to contain TOC trailer",
        ));
    }

    let toc_offset = peek_u32(&data[data.len() - 4..]) as usize;
    if toc_offset >= data.len() {
        return Err(CadencePsfError::new(format!(
            "invalid TOC offset {} for payload size {}",
            toc_offset,
            data.len()
        )));
    }

    let toc_bytes = data.len().saturating_sub(toc_offset + 12);
    if toc_bytes == 0 || toc_bytes % 8 != 0 {
        return Err(CadencePsfError::new(format!(
            "invalid TOC span {} bytes",
            toc_bytes
        )));
    }

    let mut starts: Vec<(SectionKind, usize)> = Vec::new();
    let num_entries = toc_bytes / 8;
    for i in 0..num_entries {
        let base = toc_offset + i * 8;
        let kind = SectionKind::from_u32(peek_u32(&data[base..base + 4]))?;
        let start = peek_u32(&data[base + 4..base + 8]) as usize;
        if start >= data.len() {
            return Err(CadencePsfError::new(format!(
                "TOC entry start {} out of range",
                start
            )));
        }
        starts.push((kind, start));
    }

    starts.sort_by_key(|(_, start)| *start);
    let mut entries = HashMap::new();
    for idx in 0..starts.len() {
        let (kind, start) = starts[idx];
        let end = starts
            .get(idx + 1)
            .map(|(_, next_start)| *next_start)
            .unwrap_or(data.len());
        if end <= start {
            return Err(CadencePsfError::new(
                "TOC entries are not strictly increasing",
            ));
        }
        entries.insert(kind, TocEntry { start, end });
    }

    for kind in [
        SectionKind::Header,
        SectionKind::Type,
        SectionKind::Sweep,
        SectionKind::Trace,
        SectionKind::Value,
    ] {
        if !entries.contains_key(&kind) {
            return Err(CadencePsfError::new(format!(
                "PSF binary is missing required {:?} section",
                kind
            )));
        }
    }

    Ok(Toc { entries })
}

fn parse_header(
    file: &[u8],
    entry: TocEntry,
) -> Result<HashMap<String, CadencePsfValue>, CadencePsfError> {
    if entry.start + 8 > file.len() {
        return Err(CadencePsfError::new("header section truncated"));
    }

    let end_of_section = peek_u32(&file[entry.start + 4..entry.start + 8]) as usize;
    if end_of_section > file.len() || end_of_section < entry.start + 8 {
        return Err(CadencePsfError::new("invalid header section end offset"));
    }

    let mut cursor = &file[entry.start + 8..end_of_section];
    let mut header = HashMap::new();
    while cursor.len() > 4 {
        let (name, value) = parse_named_value(&mut cursor)?;
        header.insert(name, value);
    }
    Ok(header)
}

fn parse_types(file: &[u8], entry: TocEntry) -> Result<HashMap<u32, DataType>, CadencePsfError> {
    if entry.start + 16 > file.len() {
        return Err(CadencePsfError::new("type section truncated"));
    }
    let block_type = peek_u32(&file[entry.start + 8..entry.start + 12]);
    if block_type != 22 {
        return Err(CadencePsfError::new(format!(
            "type section expected block 22, got {}",
            block_type
        )));
    }
    let end_of_section = peek_u32(&file[entry.start + 12..entry.start + 16]) as usize;
    if end_of_section > file.len() || end_of_section < entry.start + 16 {
        return Err(CadencePsfError::new("invalid type section end offset"));
    }

    let mut cursor = &file[entry.start + 16..end_of_section];
    let mut types = HashMap::new();
    while cursor.len() > 4 {
        let block = read_u32(&mut cursor)?;
        if block != 16 {
            return Err(CadencePsfError::new(format!(
                "type item expected block 16, got {}",
                block
            )));
        }
        let type_id = read_u32(&mut cursor)?;
        let _name = parse_string(&mut cursor)?;
        let _array_type = read_u32(&mut cursor)?;
        let data_type = read_u32(&mut cursor)?;
        skip_properties(&mut cursor)?;
        types.insert(type_id, DataType::from_u32(data_type));
    }

    Ok(types)
}

fn parse_sweeps(file: &[u8], entry: TocEntry) -> Result<Vec<SignalRef>, CadencePsfError> {
    if entry.start + 8 > file.len() {
        return Err(CadencePsfError::new("sweep section truncated"));
    }
    let end_of_section = peek_u32(&file[entry.start + 4..entry.start + 8]) as usize;
    if end_of_section > file.len() || end_of_section < entry.start + 8 {
        return Err(CadencePsfError::new("invalid sweep section end offset"));
    }

    let mut cursor = &file[entry.start + 8..end_of_section];
    let mut sweeps = Vec::new();
    while cursor.len() > 4 {
        let block = read_u32(&mut cursor)?;
        if block != 16 {
            return Err(CadencePsfError::new(format!(
                "sweep signal expected block 16, got {}",
                block
            )));
        }
        sweeps.push(parse_signal_ref(&mut cursor)?);
    }
    Ok(sweeps)
}

fn parse_traces(file: &[u8], entry: TocEntry) -> Result<Vec<TraceDef>, CadencePsfError> {
    if entry.start + 16 > file.len() {
        return Err(CadencePsfError::new("trace section truncated"));
    }
    let block_type = peek_u32(&file[entry.start + 8..entry.start + 12]);
    if block_type != 22 {
        return Err(CadencePsfError::new(format!(
            "trace section expected block 22, got {}",
            block_type
        )));
    }
    let end_of_section = peek_u32(&file[entry.start + 12..entry.start + 16]) as usize;
    if end_of_section > file.len() || end_of_section < entry.start + 16 {
        return Err(CadencePsfError::new("invalid trace section end offset"));
    }

    let mut cursor = &file[entry.start + 16..end_of_section];
    let mut traces = Vec::new();
    while cursor.len() > 4 {
        let block = read_u32(&mut cursor)?;
        match block {
            16 => traces.push(TraceDef::Signal(parse_signal_ref(&mut cursor)?)),
            17 => traces.push(TraceDef::Group(parse_group_signals(&mut cursor)?)),
            other => {
                return Err(CadencePsfError::new(format!(
                    "unexpected trace block type {}",
                    other
                )));
            }
        }
    }

    Ok(traces)
}

fn parse_values(
    file: &[u8],
    entry: TocEntry,
    header: &HashMap<String, CadencePsfValue>,
    types: &HashMap<u32, DataType>,
    sweeps: &[SignalRef],
    traces: &[TraceDef],
) -> Result<HashMap<u32, SignalValues>, CadencePsfError> {
    if entry.start + 8 > file.len() {
        return Err(CadencePsfError::new("value section truncated"));
    }
    let end_of_section = peek_u32(&file[entry.start + 4..entry.start + 8]) as usize;
    if end_of_section > file.len() || end_of_section < entry.start + 8 {
        return Err(CadencePsfError::new("invalid value section end offset"));
    }

    let mut values: HashMap<u32, SignalValues> = HashMap::new();
    for sweep in sweeps {
        values.insert(sweep.id, SignalValues::Real(Vec::new()));
    }

    let flat_traces = flatten_traces(traces);
    for signal in &flat_traces {
        let dtype = types
            .get(&signal.type_id)
            .copied()
            .ok_or_else(|| CadencePsfError::new(format!("missing type {}", signal.type_id)))?;
        match dtype {
            DataType::Real => {
                values.insert(signal.id, SignalValues::Real(Vec::new()));
            }
            DataType::Complex => {
                values.insert(signal.id, SignalValues::Complex(Vec::new()));
            }
            DataType::Other(other) => {
                return Err(CadencePsfError::new(format!(
                    "unsupported PSF signal data type {} for '{}'",
                    other, signal.name
                )));
            }
        }
    }

    if header.contains_key("PSF window size") {
        parse_windowed_values(
            &file[entry.start + 8..end_of_section],
            header,
            sweeps,
            &flat_traces,
            types,
            &mut values,
        )?;
    } else {
        parse_non_windowed_values(
            &file[entry.start + 8..end_of_section],
            header,
            sweeps,
            &flat_traces,
            types,
            &mut values,
        )?;
    }

    Ok(values)
}

fn parse_non_windowed_values(
    mut cursor: &[u8],
    header: &HashMap<String, CadencePsfValue>,
    sweeps: &[SignalRef],
    flat_traces: &[SignalRef],
    types: &HashMap<u32, DataType>,
    values: &mut HashMap<u32, SignalValues>,
) -> Result<(), CadencePsfError> {
    let sweep_points = header_usize(header, "PSF sweep points")?;
    let sweep_id = sweeps.first().map(|s| s.id);

    for _ in 0..sweep_points {
        let _point_idx = read_u32(&mut cursor)?;
        let _param_kind = read_u32(&mut cursor)?;
        let sweep_value = read_f64(&mut cursor)?;
        if let Some(id) = sweep_id {
            push_real(values, id, sweep_value)?;
        }

        for signal in flat_traces {
            let _unused = read_f64(&mut cursor)?;
            match types
                .get(&signal.type_id)
                .copied()
                .unwrap_or(DataType::Other(0))
            {
                DataType::Real => {
                    let sample = read_f64(&mut cursor)?;
                    push_real(values, signal.id, sample)?;
                }
                DataType::Complex => {
                    let re = read_f64(&mut cursor)?;
                    let im = read_f64(&mut cursor)?;
                    push_complex(values, signal.id, (re, im))?;
                }
                DataType::Other(other) => {
                    return Err(CadencePsfError::new(format!(
                        "unsupported non-windowed signal data type {}",
                        other
                    )));
                }
            }
        }
    }

    Ok(())
}

fn parse_windowed_values(
    mut cursor: &[u8],
    header: &HashMap<String, CadencePsfValue>,
    sweeps: &[SignalRef],
    flat_traces: &[SignalRef],
    types: &HashMap<u32, DataType>,
    values: &mut HashMap<u32, SignalValues>,
) -> Result<(), CadencePsfError> {
    let window_size = header_usize(header, "PSF window size")?;
    let num_traces = header_usize(header, "PSF traces")?;
    let sweep_points = header_usize(header, "PSF sweep points")?;
    let sweep_id = sweeps
        .first()
        .map(|s| s.id)
        .ok_or_else(|| CadencePsfError::new("windowed PSF has no sweep signal"))?;

    let mut offsets = HashMap::new();
    let mut offset = 0usize;
    for signal in flat_traces {
        offsets.insert(signal.id, offset);
        offset = offset
            .checked_add(window_size)
            .ok_or_else(|| CadencePsfError::new("windowed trace offset overflow"))?;
    }

    let initial_block = read_u32(&mut cursor)?;
    if initial_block != 20 {
        return Err(CadencePsfError::new(format!(
            "windowed PSF expected initial block 20, got {}",
            initial_block
        )));
    }
    cursor = parse_zero_pad(cursor)?;

    let mut count = 0usize;
    while count < sweep_points {
        let mut block_type = read_u32(&mut cursor)?;
        if block_type == 20 {
            cursor = parse_zero_pad(cursor)?;
            block_type = read_u32(&mut cursor)?;
        }
        if block_type != 16 {
            return Err(CadencePsfError::new(format!(
                "windowed PSF expected block 16, got {}",
                block_type
            )));
        }
        let block_init = read_u32(&mut cursor)?;
        let window_count = (block_init & 0xffff) as usize;
        for _ in 0..window_count {
            push_real(values, sweep_id, read_f64(&mut cursor)?)?;
        }

        let block_len = num_traces
            .checked_mul(window_size)
            .ok_or_else(|| CadencePsfError::new("windowed PSF block length overflow"))?;
        if cursor.len() < block_len {
            return Err(CadencePsfError::new("windowed PSF value block truncated"));
        }
        let block = &cursor[..block_len];

        for signal in flat_traces {
            let offset = *offsets
                .get(&signal.id)
                .ok_or_else(|| CadencePsfError::new("missing signal offset in windowed parser"))?;
            let data_len = window_count
                .checked_mul(8)
                .ok_or_else(|| CadencePsfError::new("windowed PSF data length overflow"))?;
            let idx = if data_len > window_size {
                offset
            } else {
                offset + (window_size - data_len)
            };
            if idx >= block.len() {
                return Err(CadencePsfError::new(
                    "windowed signal offset out of block bounds",
                ));
            }
            let mut trace_cursor = &block[idx..];

            match types
                .get(&signal.type_id)
                .copied()
                .unwrap_or(DataType::Other(0))
            {
                DataType::Real => {
                    for _ in 0..window_count {
                        push_real(values, signal.id, read_f64(&mut trace_cursor)?)?;
                    }
                }
                DataType::Complex => {
                    for _ in 0..window_count {
                        let re = read_f64(&mut trace_cursor)?;
                        let im = read_f64(&mut trace_cursor)?;
                        push_complex(values, signal.id, (re, im))?;
                    }
                }
                DataType::Other(other) => {
                    return Err(CadencePsfError::new(format!(
                        "unsupported windowed signal data type {}",
                        other
                    )));
                }
            }
        }

        cursor = &cursor[block_len..];
        count += window_count;
    }

    Ok(())
}

fn parse_group_signals(cursor: &mut &[u8]) -> Result<Vec<SignalRef>, CadencePsfError> {
    let _group_id = read_u32(cursor)?;
    let _group_name = parse_string(cursor)?;
    let count = read_u32(cursor)? as usize;

    let mut signals = Vec::with_capacity(count);
    for _ in 0..count {
        let block = read_u32(cursor)?;
        if block != 16 {
            return Err(CadencePsfError::new(format!(
                "trace group expected signal block 16, got {}",
                block
            )));
        }
        signals.push(parse_signal_ref(cursor)?);
    }
    Ok(signals)
}

fn parse_signal_ref(cursor: &mut &[u8]) -> Result<SignalRef, CadencePsfError> {
    let id = read_u32(cursor)?;
    let name = parse_string(cursor)?;
    let type_id = read_u32(cursor)?;
    skip_properties(cursor)?;
    Ok(SignalRef { id, name, type_id })
}

fn skip_properties(cursor: &mut &[u8]) -> Result<(), CadencePsfError> {
    while cursor.len() > 4 {
        let block = peek_u32(cursor);
        if !(33..=35).contains(&block) {
            break;
        }
        let _ = parse_named_value(cursor)?;
    }
    Ok(())
}

fn parse_named_value(cursor: &mut &[u8]) -> Result<(String, CadencePsfValue), CadencePsfError> {
    let block = read_u32(cursor)?;
    let name = parse_string(cursor)?;
    let value = match block {
        33 => CadencePsfValue::Text(parse_string(cursor)?),
        34 => CadencePsfValue::Int(read_u32(cursor)? as i64),
        35 => CadencePsfValue::Real(read_f64(cursor)?),
        other => {
            return Err(CadencePsfError::new(format!(
                "unexpected named-value block {}",
                other
            )));
        }
    };
    Ok((name, value))
}

fn flatten_traces(traces: &[TraceDef]) -> Vec<SignalRef> {
    let mut flat = Vec::new();
    for trace in traces {
        match trace {
            TraceDef::Signal(signal) => flat.push(signal.clone()),
            TraceDef::Group(signals) => flat.extend(signals.iter().cloned()),
        }
    }
    flat
}

fn header_usize(
    header: &HashMap<String, CadencePsfValue>,
    key: &str,
) -> Result<usize, CadencePsfError> {
    match header.get(key) {
        Some(CadencePsfValue::Int(v)) if *v >= 0 => Ok(*v as usize),
        Some(CadencePsfValue::Real(v)) if *v >= 0.0 => Ok(*v as usize),
        Some(_) => Err(CadencePsfError::new(format!(
            "header value '{}' has non-numeric type",
            key
        ))),
        None => Err(CadencePsfError::new(format!(
            "missing required header key '{}'",
            key
        ))),
    }
}

fn parse_zero_pad(mut cursor: &[u8]) -> Result<&[u8], CadencePsfError> {
    let len = read_u32(&mut cursor)? as usize;
    if cursor.len() < len {
        return Err(CadencePsfError::new("zero-pad block truncated"));
    }
    Ok(&cursor[len..])
}

fn push_real(
    values: &mut HashMap<u32, SignalValues>,
    signal_id: u32,
    value: f64,
) -> Result<(), CadencePsfError> {
    let slot = values.get_mut(&signal_id).ok_or_else(|| {
        CadencePsfError::new(format!("missing value vector for signal {}", signal_id))
    })?;
    match slot {
        SignalValues::Real(v) => {
            v.push(value);
            Ok(())
        }
        SignalValues::Complex(_) => Err(CadencePsfError::new(format!(
            "real sample written to complex signal {}",
            signal_id
        ))),
    }
}

fn push_complex(
    values: &mut HashMap<u32, SignalValues>,
    signal_id: u32,
    value: (f64, f64),
) -> Result<(), CadencePsfError> {
    let slot = values.get_mut(&signal_id).ok_or_else(|| {
        CadencePsfError::new(format!("missing value vector for signal {}", signal_id))
    })?;
    match slot {
        SignalValues::Complex(v) => {
            v.push(value);
            Ok(())
        }
        SignalValues::Real(_) => Err(CadencePsfError::new(format!(
            "complex sample written to real signal {}",
            signal_id
        ))),
    }
}

fn parse_string(cursor: &mut &[u8]) -> Result<String, CadencePsfError> {
    let len = read_u32(cursor)? as usize;
    if cursor.len() < len {
        return Err(CadencePsfError::new("string block truncated"));
    }
    let raw = &cursor[..len];
    let value = std::str::from_utf8(raw)
        .map_err(|e| CadencePsfError::new(format!("invalid UTF-8 in PSF string: {}", e)))?
        .to_string();

    let pad = (4 - (len % 4)) % 4;
    if cursor.len() < len + pad {
        return Err(CadencePsfError::new(
            "string padding exceeds remaining bytes",
        ));
    }
    *cursor = &cursor[len + pad..];
    Ok(value)
}

fn read_u32(cursor: &mut &[u8]) -> Result<u32, CadencePsfError> {
    if cursor.len() < 4 {
        return Err(CadencePsfError::new(
            "unexpected end of PSF data while reading u32",
        ));
    }
    let (head, tail) = cursor.split_at(4);
    *cursor = tail;
    Ok(u32::from_be_bytes(
        head.try_into().expect("slice length checked"),
    ))
}

fn read_f64(cursor: &mut &[u8]) -> Result<f64, CadencePsfError> {
    if cursor.len() < 8 {
        return Err(CadencePsfError::new(
            "unexpected end of PSF data while reading f64",
        ));
    }
    let (head, tail) = cursor.split_at(8);
    *cursor = tail;
    Ok(f64::from_be_bytes(
        head.try_into().expect("slice length checked"),
    ))
}

fn peek_u32(data: &[u8]) -> u32 {
    u32::from_be_bytes(data[..4].try_into().expect("slice length checked"))
}

#[cfg(test)]
pub(crate) mod test_helpers {
    use super::*;

    pub(crate) fn build_non_windowed_real_psf() -> Vec<u8> {
        build_simple_non_windowed_psf(false)
    }

    pub(crate) fn build_non_windowed_complex_psf() -> Vec<u8> {
        build_simple_non_windowed_psf(true)
    }

    fn build_simple_non_windowed_psf(complex: bool) -> Vec<u8> {
        let mut bytes = Vec::new();

        let header_start = bytes.len();
        push_u32(&mut bytes, 0);
        let header_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_named_int(&mut bytes, "PSF sweep points", 2);
        let header_end = bytes.len() as u32;
        patch_u32(&mut bytes, header_eofs_pos, header_end);

        let types_start = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 22);
        let types_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 1);
        push_string(&mut bytes, "sigtype");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, if complex { 12 } else { 11 });
        let types_end = bytes.len() as u32;
        patch_u32(&mut bytes, types_eofs_pos, types_end);

        let sweep_start = bytes.len();
        push_u32(&mut bytes, 0);
        let sweep_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 100, "time", 1);
        let sweep_end = bytes.len() as u32;
        patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

        let trace_start = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 22);
        let trace_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 200, "V(out)", 1);
        let trace_end = bytes.len() as u32;
        patch_u32(&mut bytes, trace_eofs_pos, trace_end);

        let value_start = bytes.len();
        push_u32(&mut bytes, 0);
        let value_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);

        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 0.0);
        if complex {
            push_f64(&mut bytes, 1.0);
            push_f64(&mut bytes, 0.5);
        } else {
            push_f64(&mut bytes, 1.0);
        }

        push_u32(&mut bytes, 1);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 1.0);
        push_f64(&mut bytes, 0.0);
        if complex {
            push_f64(&mut bytes, 2.0);
            push_f64(&mut bytes, -0.25);
        } else {
            push_f64(&mut bytes, 2.0);
        }

        let value_end = bytes.len() as u32;
        patch_u32(&mut bytes, value_eofs_pos, value_end);

        let toc_offset = bytes.len();
        for (kind, start) in [
            (0u32, header_start),
            (1u32, types_start),
            (2u32, sweep_start),
            (3u32, trace_start),
            (4u32, value_start),
        ] {
            push_u32(&mut bytes, kind);
            push_u32(&mut bytes, start as u32);
        }
        bytes.extend_from_slice(&[0u8; 8]);
        push_u32(&mut bytes, toc_offset as u32);

        bytes
    }

    fn push_signal_ref(bytes: &mut Vec<u8>, id: u32, name: &str, type_id: u32) {
        push_u32(bytes, id);
        push_string(bytes, name);
        push_u32(bytes, type_id);
    }

    fn push_named_int(bytes: &mut Vec<u8>, name: &str, value: u32) {
        push_u32(bytes, 34);
        push_string(bytes, name);
        push_u32(bytes, value);
    }

    fn push_string(bytes: &mut Vec<u8>, s: &str) {
        push_u32(bytes, s.len() as u32);
        bytes.extend_from_slice(s.as_bytes());
        let pad = (4 - (s.len() % 4)) % 4;
        bytes.extend(std::iter::repeat(0u8).take(pad));
    }

    fn push_f64(bytes: &mut Vec<u8>, value: f64) {
        bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn push_u32(bytes: &mut Vec<u8>, value: u32) {
        bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn patch_u32(bytes: &mut [u8], offset: usize, value: u32) {
        bytes[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::test_helpers::{build_non_windowed_complex_psf, build_non_windowed_real_psf};
    use super::*;

    #[test]
    fn test_parse_non_windowed_real_psf_binary() {
        let bytes = build_non_windowed_real_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].name, "time");
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);

        assert_eq!(parsed.real_signals.len(), 1);
        assert_eq!(parsed.real_signals[0].name, "V(out)");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 2.0]);
        assert!(parsed.complex_signals.is_empty());
    }

    #[test]
    fn test_parse_non_windowed_complex_psf_binary() {
        let bytes = build_non_windowed_complex_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.real_signals.is_empty());

        assert_eq!(parsed.complex_signals.len(), 1);
        assert_eq!(parsed.complex_signals[0].name, "V(out)");
        assert_eq!(
            parsed.complex_signals[0].values,
            vec![(1.0, 0.5), (2.0, -0.25)]
        );
    }

    #[test]
    fn test_parse_truncated_binary_fails() {
        let mut bytes = build_non_windowed_real_psf();
        bytes.truncate(bytes.len().saturating_sub(9));
        let err = parse_cadence_psf_binary(&bytes).expect_err("truncated input should fail");
        assert!(
            err.to_string().contains("TOC")
                || err.to_string().contains("truncated")
                || err.to_string().contains("invalid")
        );
    }
}

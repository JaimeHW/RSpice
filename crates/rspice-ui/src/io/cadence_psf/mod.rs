//! Cadence PSF native binary parsing.
//!
//! This module provides a local parser for Cadence/Spectre PSF binary payloads
//! so waveform import does not depend on external crates.

use std::collections::HashMap;
use std::fmt;

mod cadence_psf_type_meta;
use cadence_psf_type_meta::TypeMetaCache;

mod binary_io;
use binary_io::{
    parse_string, peek_u32, read_f64, read_i32, read_u32, read_u8_padded, skip_opaque_scalar,
};

mod toc;
use toc::{parse_toc, SectionKind, TocEntry};

mod value_decode;
use value_decode::{decode_windowed_dynamic_signal_samples, read_type_value_with_numeric_visit};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DataType {
    Int8,
    Int32,
    Real,
    Complex,
    String,
    Array,
    Struct,
    Other(u32),
}

impl DataType {
    fn from_u32(value: u32) -> Self {
        match value {
            1 => Self::Int8,
            2 => Self::String,
            3 => Self::Array,
            5 => Self::Int32,
            11 => Self::Real,
            12 => Self::Complex,
            16 => Self::Struct,
            other => Self::Other(other),
        }
    }

    fn to_u32(self) -> u32 {
        match self {
            Self::Int8 => 1,
            Self::String => 2,
            Self::Array => 3,
            Self::Int32 => 5,
            Self::Real => 11,
            Self::Complex => 12,
            Self::Struct => 16,
            Self::Other(v) => v,
        }
    }
}

#[derive(Debug, Clone)]
struct TypeDecl {
    name: String,
    kind: TypeKind,
}

#[derive(Debug, Clone)]
enum TypeKind {
    Primitive(DataType),
    Array { element_type_raw: u32 },
    Struct { members: Vec<u32> },
}

#[derive(Debug, Clone, Copy)]
enum ArrayElementType {
    Primitive(DataType),
    TypeRef(u32),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChannelKind {
    Real,
    Complex,
}

#[derive(Debug, Clone)]
struct ChannelSpec {
    suffix: String,
    kind: ChannelKind,
}

#[derive(Debug, Clone)]
struct SignalChannel {
    suffix: String,
    values: SignalValues,
}

#[derive(Debug, Clone, Copy)]
enum NumericSample {
    Real(f64),
    Complex((f64, f64)),
}

pub fn parse_cadence_psf_binary(data: &[u8]) -> Result<ParsedCadencePsfBinary, CadencePsfError> {
    let toc = parse_toc(data)?;
    let header = parse_header(data, toc.section(SectionKind::Header)?)?;
    let types = parse_types(data, toc.section(SectionKind::Type)?)?;
    let sweeps = parse_sweeps(data, toc.section(SectionKind::Sweep)?)?;
    let traces = parse_traces(data, toc.section(SectionKind::Trace)?)?;
    let mut signal_values = parse_values(
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
        if let Some(channels) = signal_values.remove(&signal.id) {
            for channel in channels {
                let name = qualify_signal_name(&signal.name, &channel.suffix);
                match channel.values {
                    SignalValues::Real(values) => {
                        real_signals.push(NamedRealSignal { name, values })
                    }
                    SignalValues::Complex(values) => {
                        complex_signals.push(NamedComplexSignal { name, values })
                    }
                }
            }
        }
    }

    let mut sweep_signals = Vec::new();
    for sweep in &sweeps {
        if let Some(channels) = signal_values.remove(&sweep.id) {
            for channel in channels {
                if let SignalValues::Real(values) = channel.values {
                    sweep_signals.push(NamedRealSignal {
                        name: qualify_signal_name(&sweep.name, &channel.suffix),
                        values,
                    });
                }
            }
        }
    }

    Ok(ParsedCadencePsfBinary {
        header,
        sweeps: sweep_signals,
        real_signals,
        complex_signals,
    })
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
        if consume_zero_word_padding(&mut cursor) {
            break;
        }
        let (name, value) = parse_named_value(&mut cursor)?;
        header.insert(name, value);
    }
    reject_non_zero_trailing_bytes("header", cursor, &[])?;
    Ok(header)
}

fn parse_types(file: &[u8], entry: TocEntry) -> Result<HashMap<u32, TypeDecl>, CadencePsfError> {
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
        if consume_zero_word_padding(&mut cursor) {
            break;
        }
        parse_type_decl(&mut cursor, &mut types)?;
    }
    // Type payloads may end with a trailing struct-terminator word.
    reject_non_zero_trailing_bytes("type", cursor, &[18])?;

    Ok(types)
}

fn parse_type_decl(
    cursor: &mut &[u8],
    types: &mut HashMap<u32, TypeDecl>,
) -> Result<u32, CadencePsfError> {
    let block = read_u32(cursor)?;
    if block != 16 {
        return Err(CadencePsfError::new(format!(
            "type item expected block 16, got {}",
            block
        )));
    }

    let type_id = read_u32(cursor)?;
    let name = parse_string(cursor)?;
    let array_type_raw = read_u32(cursor)?;
    let data_type = DataType::from_u32(read_u32(cursor)?);
    let kind = match data_type {
        DataType::Struct => {
            let mut members = Vec::new();
            while cursor.len() > 4 {
                if peek_u32(cursor) == 18 {
                    let _ = read_u32(cursor)?;
                    break;
                }
                let member_id = parse_type_decl(cursor, types)?;
                members.push(member_id);
            }
            TypeKind::Struct { members }
        }
        DataType::Array => TypeKind::Array {
            element_type_raw: array_type_raw,
        },
        other => TypeKind::Primitive(other),
    };

    skip_properties(cursor)?;
    types.insert(type_id, TypeDecl { name, kind });
    Ok(type_id)
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
        if consume_zero_word_padding(&mut cursor) {
            break;
        }
        let block = read_u32(&mut cursor)?;
        if block != 16 {
            return Err(CadencePsfError::new(format!(
                "sweep signal expected block 16, got {}",
                block
            )));
        }
        sweeps.push(parse_signal_ref(&mut cursor)?);
    }
    reject_non_zero_trailing_bytes("sweep", cursor, &[])?;
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
        if consume_zero_word_padding(&mut cursor) {
            break;
        }
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
    reject_non_zero_trailing_bytes("trace", cursor, &[])?;

    Ok(traces)
}

fn parse_values(
    file: &[u8],
    entry: TocEntry,
    header: &HashMap<String, CadencePsfValue>,
    types: &HashMap<u32, TypeDecl>,
    sweeps: &[SignalRef],
    traces: &[TraceDef],
) -> Result<HashMap<u32, Vec<SignalChannel>>, CadencePsfError> {
    if entry.start + 8 > file.len() {
        return Err(CadencePsfError::new("value section truncated"));
    }
    let end_of_section = peek_u32(&file[entry.start + 4..entry.start + 8]) as usize;
    if end_of_section > file.len() || end_of_section < entry.start + 8 {
        return Err(CadencePsfError::new("invalid value section end offset"));
    }

    let sweep_points = header_usize(header, "PSF sweep points")?;
    let mut values: HashMap<u32, Vec<SignalChannel>> = HashMap::new();
    for sweep in sweeps {
        values.insert(
            sweep.id,
            vec![SignalChannel {
                suffix: String::new(),
                values: SignalValues::Real(Vec::with_capacity(sweep_points)),
            }],
        );
    }

    let flat_traces = flatten_traces(traces);
    let mut type_meta = TypeMetaCache::default();
    let mut signal_has_dynamic_arrays = HashMap::with_capacity(flat_traces.len());
    let is_windowed = header.contains_key("PSF window size");
    for signal in &flat_traces {
        let has_dynamic_arrays = type_meta.contains_array(signal.type_id, types)?;
        signal_has_dynamic_arrays.insert(signal.id, has_dynamic_arrays);
        let _ = types
            .get(&signal.type_id)
            .ok_or_else(|| CadencePsfError::new(format!("missing type {}", signal.type_id)))?;
        if has_dynamic_arrays {
            continue;
        }
        let mut specs = Vec::new();
        collect_channel_specs_for_type(signal.type_id, types, "", &mut specs)?;
        if !specs.is_empty() {
            values.insert(signal.id, init_channels(&specs, sweep_points));
        }
    }

    if is_windowed {
        let trailing = parse_windowed_values(
            &file[entry.start + 8..end_of_section],
            header,
            sweeps,
            &flat_traces,
            types,
            &signal_has_dynamic_arrays,
            &mut values,
            &mut type_meta,
        )?;
        reject_non_zero_trailing_bytes("value", trailing, &[])?;
    } else {
        let trailing = parse_non_windowed_values(
            &file[entry.start + 8..end_of_section],
            header,
            sweeps,
            &flat_traces,
            types,
            &mut values,
        )?;
        reject_non_zero_trailing_bytes("value", trailing, &[])?;
    }

    Ok(values)
}

fn parse_non_windowed_values<'a>(
    mut cursor: &'a [u8],
    header: &HashMap<String, CadencePsfValue>,
    sweeps: &[SignalRef],
    flat_traces: &[SignalRef],
    types: &HashMap<u32, TypeDecl>,
    values: &mut HashMap<u32, Vec<SignalChannel>>,
) -> Result<&'a [u8], CadencePsfError> {
    let sweep_points = header_usize(header, "PSF sweep points")?;
    if header.contains_key("PSF traces") {
        let num_traces = header_usize(header, "PSF traces")?;
        if num_traces != flat_traces.len() {
            return Err(CadencePsfError::new(format!(
                "non-windowed PSF trace count mismatch: header={}, decoded={}",
                num_traces,
                flat_traces.len()
            )));
        }
    }
    let sweep_id = sweeps.first().map(|s| s.id);
    let mut channel_index_cache = build_channel_index_cache(values);

    for point_idx in 0..sweep_points {
        let _point_idx = read_u32(&mut cursor)?;
        let _param_kind = read_u32(&mut cursor)?;
        let sweep_value = read_f64(&mut cursor)?;
        if let Some(id) = sweep_id {
            push_scalar_channel(values, id, 0, NumericSample::Real(sweep_value))?;
        }

        for signal in flat_traces {
            let _unused = read_f64(&mut cursor)?;
            let mut touched_channels = Vec::new();
            read_type_value_with_numeric_visit(
                &mut cursor,
                signal.type_id,
                types,
                "",
                &mut |suffix, sample| {
                    let channel_idx = push_named_channel_cached(
                        values,
                        &mut channel_index_cache,
                        signal.id,
                        suffix,
                        sample,
                        point_idx,
                    )?;
                    touched_channels.push(channel_idx);
                    Ok(())
                },
            )?;
            pad_untouched_channels(values, signal.id, point_idx + 1, &touched_channels)?;
        }
    }

    Ok(cursor)
}

fn parse_windowed_values<'a>(
    mut cursor: &'a [u8],
    header: &HashMap<String, CadencePsfValue>,
    sweeps: &[SignalRef],
    flat_traces: &[SignalRef],
    types: &HashMap<u32, TypeDecl>,
    signal_has_dynamic_arrays: &HashMap<u32, bool>,
    values: &mut HashMap<u32, Vec<SignalChannel>>,
    type_meta: &mut TypeMetaCache,
) -> Result<&'a [u8], CadencePsfError> {
    let window_size = header_usize(header, "PSF window size")?;
    let num_traces = header_usize(header, "PSF traces")?;
    let sweep_points = header_usize(header, "PSF sweep points")?;
    let sweep_id = sweeps
        .first()
        .map(|s| s.id)
        .ok_or_else(|| CadencePsfError::new("windowed PSF has no sweep signal"))?;
    if num_traces != flat_traces.len() {
        return Err(CadencePsfError::new(format!(
            "windowed PSF trace count mismatch: header={}, decoded={}",
            num_traces,
            flat_traces.len()
        )));
    }

    let mut channel_index_cache = build_channel_index_cache(values);

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
        if window_count == 0 {
            return Err(CadencePsfError::new(
                "windowed PSF encountered block with zero samples",
            ));
        }
        let end_count = count
            .checked_add(window_count)
            .ok_or_else(|| CadencePsfError::new("windowed PSF sample count overflow"))?;
        if end_count > sweep_points {
            return Err(CadencePsfError::new(format!(
                "windowed PSF block exceeds declared sweep points: block_end={}, declared={}",
                end_count, sweep_points
            )));
        }
        for _ in 0..window_count {
            push_scalar_channel(
                values,
                sweep_id,
                0,
                NumericSample::Real(read_f64(&mut cursor)?),
            )?;
        }

        let block_len = num_traces
            .checked_mul(window_size)
            .ok_or_else(|| CadencePsfError::new("windowed PSF block length overflow"))?;
        if cursor.len() < block_len {
            return Err(CadencePsfError::new("windowed PSF value block truncated"));
        }
        let block = &cursor[..block_len];

        for (signal_idx, signal) in flat_traces.iter().enumerate() {
            let offset = signal_idx
                .checked_mul(window_size)
                .ok_or_else(|| CadencePsfError::new("windowed trace offset overflow"))?;
            let end = offset
                .checked_add(window_size)
                .ok_or_else(|| CadencePsfError::new("windowed PSF signal range overflow"))?;
            if end > block.len() {
                return Err(CadencePsfError::new(
                    "windowed signal offset out of block bounds",
                ));
            }
            let segment = &block[offset..end];

            if *signal_has_dynamic_arrays.get(&signal.id).ok_or_else(|| {
                CadencePsfError::new("missing signal array/dynamic marker in windowed parser")
            })? {
                decode_windowed_dynamic_signal_samples(
                    segment,
                    signal,
                    window_count,
                    count,
                    types,
                    values,
                    &mut channel_index_cache,
                    type_meta,
                )?;
            } else {
                let channels = values.get(&signal.id).ok_or_else(|| {
                    CadencePsfError::new(format!(
                        "windowed PSF signal '{}' has no numeric channels",
                        signal.name
                    ))
                })?;
                if channels.is_empty() {
                    // Opaque/unsupported scalar payload for this trace: skip segment.
                    continue;
                }
                let sample_width = channel_sample_width(channels)?;
                let data_len = window_count
                    .checked_mul(sample_width)
                    .ok_or_else(|| CadencePsfError::new("windowed PSF data length overflow"))?;
                let idx = window_size.saturating_sub(data_len);
                if idx >= segment.len() {
                    return Err(CadencePsfError::new(
                        "windowed signal offset out of block bounds",
                    ));
                }
                let mut trace_cursor = &segment[idx..];

                let channels = values.get_mut(&signal.id).ok_or_else(|| {
                    CadencePsfError::new(format!("missing value vector for signal {}", signal.id))
                })?;
                for _ in 0..window_count {
                    for channel in channels.iter_mut() {
                        match &mut channel.values {
                            SignalValues::Real(vec) => vec.push(read_f64(&mut trace_cursor)?),
                            SignalValues::Complex(vec) => {
                                let re = read_f64(&mut trace_cursor)?;
                                let im = read_f64(&mut trace_cursor)?;
                                vec.push((re, im));
                            }
                        }
                    }
                }
            }
        }

        cursor = &cursor[block_len..];
        count = end_count;
    }

    Ok(cursor)
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
        34 => CadencePsfValue::Int(read_i32(cursor)? as i64),
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
        Some(CadencePsfValue::Int(v)) => usize::try_from(*v).map_err(|_| {
            CadencePsfError::new(format!(
                "header value '{}' must be a non-negative integer count",
                key
            ))
        }),
        Some(CadencePsfValue::Real(v)) => {
            if !v.is_finite() {
                return Err(CadencePsfError::new(format!(
                    "header value '{}' must be finite",
                    key
                )));
            }
            if *v < 0.0 {
                return Err(CadencePsfError::new(format!(
                    "header value '{}' must be a non-negative integer count",
                    key
                )));
            }
            if v.fract() != 0.0 {
                return Err(CadencePsfError::new(format!(
                    "header value '{}' must be an integer count",
                    key
                )));
            }
            if *v > usize::MAX as f64 {
                return Err(CadencePsfError::new(format!(
                    "header value '{}' exceeds supported range",
                    key
                )));
            }
            Ok(*v as usize)
        }
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
    let (pad, tail) = cursor.split_at(len);
    if pad.iter().any(|byte| *byte != 0) {
        return Err(CadencePsfError::new(
            "zero-pad block contains non-zero bytes",
        ));
    }
    Ok(tail)
}

fn consume_zero_word_padding(cursor: &mut &[u8]) -> bool {
    if cursor.len() < 4 || peek_u32(cursor) != 0 {
        return false;
    }
    if cursor[4..].iter().all(|byte| *byte == 0) {
        *cursor = &[];
        return true;
    }
    false
}

fn reject_non_zero_trailing_bytes(
    section_name: &str,
    trailing: &[u8],
    allowed_words: &[u32],
) -> Result<(), CadencePsfError> {
    if trailing.is_empty() || trailing.iter().all(|byte| *byte == 0) {
        return Ok(());
    }
    if trailing.len() == 4 && allowed_words.contains(&peek_u32(trailing)) {
        return Ok(());
    }

    Err(CadencePsfError::new(format!(
        "{} section has unexpected non-zero trailing bytes ({})",
        section_name,
        trailing.len()
    )))
}

fn qualify_signal_name(base: &str, suffix: &str) -> String {
    if suffix.is_empty() {
        base.to_string()
    } else if suffix.starts_with('[') {
        format!("{}{}", base, suffix)
    } else {
        format!("{}.{}", base, suffix)
    }
}

fn init_channels(specs: &[ChannelSpec], capacity_hint: usize) -> Vec<SignalChannel> {
    specs
        .iter()
        .map(|spec| SignalChannel {
            suffix: spec.suffix.clone(),
            values: match spec.kind {
                ChannelKind::Real => SignalValues::Real(Vec::with_capacity(capacity_hint)),
                ChannelKind::Complex => SignalValues::Complex(Vec::with_capacity(capacity_hint)),
            },
        })
        .collect()
}

fn resolve_array_element_type(
    element_type_raw: u32,
    types: &HashMap<u32, TypeDecl>,
    parent_type_id: Option<u32>,
) -> Result<ArrayElementType, CadencePsfError> {
    if let Some(type_decl) = types.get(&element_type_raw) {
        let raw_as_dtype = DataType::from_u32(element_type_raw);
        match raw_as_dtype {
            DataType::Int8
            | DataType::Int32
            | DataType::Real
            | DataType::Complex
            | DataType::String => match type_decl.kind {
                TypeKind::Primitive(dtype) if dtype == raw_as_dtype => {
                    Ok(ArrayElementType::Primitive(raw_as_dtype))
                }
                _ => Ok(ArrayElementType::TypeRef(element_type_raw)),
            },
            DataType::Array => match type_decl.kind {
                TypeKind::Array { .. } => Ok(ArrayElementType::TypeRef(element_type_raw)),
                _ => resolve_implicit_composite_array_element(types, true, parent_type_id),
            },
            DataType::Struct => match type_decl.kind {
                TypeKind::Struct { .. } => Ok(ArrayElementType::TypeRef(element_type_raw)),
                _ => resolve_implicit_composite_array_element(types, false, parent_type_id),
            },
            DataType::Other(_) => Ok(ArrayElementType::TypeRef(element_type_raw)),
        }
    } else {
        match DataType::from_u32(element_type_raw) {
            DataType::Array => {
                resolve_implicit_composite_array_element(types, true, parent_type_id)
            }
            DataType::Struct => {
                resolve_implicit_composite_array_element(types, false, parent_type_id)
            }
            other => Ok(ArrayElementType::Primitive(other)),
        }
    }
}

fn resolve_implicit_composite_array_element(
    types: &HashMap<u32, TypeDecl>,
    want_array: bool,
    parent_type_id: Option<u32>,
) -> Result<ArrayElementType, CadencePsfError> {
    let mut candidates = Vec::new();
    for (type_id, decl) in types {
        if parent_type_id == Some(*type_id) {
            continue;
        }
        match (&decl.kind, want_array) {
            (TypeKind::Array { .. }, true) => candidates.push(*type_id),
            (TypeKind::Struct { .. }, false) => candidates.push(*type_id),
            _ => {}
        }
    }

    if candidates.len() == 1 {
        return Ok(ArrayElementType::TypeRef(candidates[0]));
    }
    if candidates.is_empty() {
        return Err(CadencePsfError::new(if want_array {
            "array element descriptor ARRAY has no resolvable array type declaration"
        } else {
            "array element descriptor STRUCT has no resolvable struct type declaration"
        }));
    }

    candidates.sort_unstable();
    Err(CadencePsfError::new((if want_array {
            format!(
                "array element descriptor ARRAY is ambiguous across type ids {:?}",
                candidates
            )
        } else {
            format!(
                "array element descriptor STRUCT is ambiguous across type ids {:?}",
                candidates
            )
        }).to_string()))
}

fn collect_channel_specs_for_type(
    type_id: u32,
    types: &HashMap<u32, TypeDecl>,
    prefix: &str,
    specs: &mut Vec<ChannelSpec>,
) -> Result<(), CadencePsfError> {
    let decl = types
        .get(&type_id)
        .ok_or_else(|| CadencePsfError::new(format!("missing type declaration {}", type_id)))?;
    match &decl.kind {
        TypeKind::Primitive(dtype) => match dtype {
            DataType::Int8 | DataType::Int32 | DataType::Real => specs.push(ChannelSpec {
                suffix: prefix.to_string(),
                kind: ChannelKind::Real,
            }),
            DataType::Complex => specs.push(ChannelSpec {
                suffix: prefix.to_string(),
                kind: ChannelKind::Complex,
            }),
            DataType::String | DataType::Array | DataType::Struct => {}
            DataType::Other(_) => {}
        },
        TypeKind::Array { element_type_raw } => {
            collect_channel_specs_for_array_element(
                *element_type_raw,
                types,
                prefix,
                specs,
                Some(type_id),
            )?;
        }
        TypeKind::Struct { members } => {
            for member_id in members {
                let member = types.get(member_id).ok_or_else(|| {
                    CadencePsfError::new(format!("missing struct member type {}", member_id))
                })?;
                let next = if prefix.is_empty() {
                    member.name.clone()
                } else {
                    format!("{}.{}", prefix, member.name)
                };
                collect_channel_specs_for_type(*member_id, types, &next, specs)?;
            }
        }
    }
    Ok(())
}

fn collect_channel_specs_for_array_element(
    element_type_raw: u32,
    types: &HashMap<u32, TypeDecl>,
    prefix: &str,
    specs: &mut Vec<ChannelSpec>,
    parent_type_id: Option<u32>,
) -> Result<(), CadencePsfError> {
    match resolve_array_element_type(element_type_raw, types, parent_type_id)? {
        ArrayElementType::Primitive(dtype) => {
            match dtype {
                DataType::Int8 | DataType::Int32 | DataType::Real => specs.push(ChannelSpec {
                    suffix: prefix.to_string(),
                    kind: ChannelKind::Real,
                }),
                DataType::Complex => specs.push(ChannelSpec {
                    suffix: prefix.to_string(),
                    kind: ChannelKind::Complex,
                }),
                DataType::String => {}
                DataType::Array => return Err(CadencePsfError::new(
                    "array element descriptor resolved to ARRAY without a concrete type reference",
                )),
                DataType::Struct => return Err(CadencePsfError::new(
                    "array element descriptor resolved to STRUCT without a concrete type reference",
                )),
                DataType::Other(_) => {}
            }
        }
        ArrayElementType::TypeRef(type_id) => {
            collect_channel_specs_for_type(type_id, types, prefix, specs)?;
        }
    }
    Ok(())
}

fn channel_sample_width(channels: &[SignalChannel]) -> Result<usize, CadencePsfError> {
    let mut width = 0usize;
    for channel in channels {
        let ch_width = match channel.values {
            SignalValues::Real(_) => 8usize,
            SignalValues::Complex(_) => 16usize,
        };
        width = width
            .checked_add(ch_width)
            .ok_or_else(|| CadencePsfError::new("windowed PSF channel width overflow"))?;
    }
    Ok(width)
}

fn ensure_channel_length(values: &mut SignalValues, len: usize) {
    match values {
        SignalValues::Real(v) => v.resize(len, f64::NAN),
        SignalValues::Complex(v) => v.resize(len, (f64::NAN, f64::NAN)),
    }
}

fn channel_kind(values: &SignalValues) -> ChannelKind {
    match values {
        SignalValues::Real(_) => ChannelKind::Real,
        SignalValues::Complex(_) => ChannelKind::Complex,
    }
}

fn build_channel_index_cache(
    values: &HashMap<u32, Vec<SignalChannel>>,
) -> HashMap<u32, HashMap<String, usize>> {
    let mut cache = HashMap::new();
    for (signal_id, channels) in values {
        let mut per_signal = HashMap::with_capacity(channels.len());
        for (idx, channel) in channels.iter().enumerate() {
            per_signal.insert(channel.suffix.clone(), idx);
        }
        cache.insert(*signal_id, per_signal);
    }
    cache
}

fn push_named_channel_cached(
    values: &mut HashMap<u32, Vec<SignalChannel>>,
    cache: &mut HashMap<u32, HashMap<String, usize>>,
    signal_id: u32,
    suffix: &str,
    sample: NumericSample,
    point_idx: usize,
) -> Result<usize, CadencePsfError> {
    if let Some(per_signal) = cache.get(&signal_id) {
        if let Some(idx) = per_signal.get(suffix) {
            let idx = *idx;
            let channels = values.get_mut(&signal_id).ok_or_else(|| {
                CadencePsfError::new(format!("missing value vector for signal {}", signal_id))
            })?;
            push_scalar_slice(channels.as_mut_slice(), idx, sample)?;
            return Ok(idx);
        }
    }

    let idx = push_named_channel(values, signal_id, suffix, sample, point_idx)?;
    cache
        .entry(signal_id)
        .or_default()
        .insert(suffix.to_string(), idx);
    Ok(idx)
}

fn push_named_channel(
    values: &mut HashMap<u32, Vec<SignalChannel>>,
    signal_id: u32,
    suffix: &str,
    sample: NumericSample,
    point_idx: usize,
) -> Result<usize, CadencePsfError> {
    let channels = values.entry(signal_id).or_default();
    let sample_kind = match sample {
        NumericSample::Real(_) => ChannelKind::Real,
        NumericSample::Complex(_) => ChannelKind::Complex,
    };
    let channel_idx = channels.iter().position(|channel| channel.suffix == suffix);
    let channel_idx = if let Some(idx) = channel_idx {
        idx
    } else {
        let mut values = match sample_kind {
            ChannelKind::Real => SignalValues::Real(Vec::new()),
            ChannelKind::Complex => SignalValues::Complex(Vec::new()),
        };
        ensure_channel_length(&mut values, point_idx);
        channels.push(SignalChannel {
            suffix: suffix.to_string(),
            values,
        });
        channels.len() - 1
    };

    let channel = channels.get_mut(channel_idx).ok_or_else(|| {
        CadencePsfError::new(format!(
            "missing channel index {} for signal {}",
            channel_idx, signal_id
        ))
    })?;
    if channel_kind(&channel.values) != sample_kind {
        return Err(CadencePsfError::new(format!(
            "channel '{}' for signal {} changed numeric kind",
            channel.suffix, signal_id
        )));
    }
    ensure_channel_length(&mut channel.values, point_idx);
    match (&mut channel.values, sample) {
        (SignalValues::Real(v), NumericSample::Real(value)) => v.push(value),
        (SignalValues::Complex(v), NumericSample::Complex(value)) => v.push(value),
        _ => {
            return Err(CadencePsfError::new(
                "internal mismatch while appending channel sample",
            ));
        }
    }
    Ok(channel_idx)
}

fn pad_untouched_channels(
    values: &mut HashMap<u32, Vec<SignalChannel>>,
    signal_id: u32,
    target_len: usize,
    touched_channels: &[usize],
) -> Result<(), CadencePsfError> {
    let Some(channels) = values.get_mut(&signal_id) else {
        return Ok(());
    };
    let mut touched = vec![false; channels.len()];
    for idx in touched_channels {
        let entry = touched.get_mut(*idx).ok_or_else(|| {
            CadencePsfError::new(format!(
                "channel index {} out of range for signal {}",
                idx, signal_id
            ))
        })?;
        *entry = true;
    }
    for (idx, channel) in channels.iter_mut().enumerate() {
        if !touched[idx] {
            ensure_channel_length(&mut channel.values, target_len);
        }
    }
    Ok(())
}

fn push_scalar_channel(
    values: &mut HashMap<u32, Vec<SignalChannel>>,
    signal_id: u32,
    channel_idx: usize,
    sample: NumericSample,
) -> Result<(), CadencePsfError> {
    let channels = values.get_mut(&signal_id).ok_or_else(|| {
        CadencePsfError::new(format!("missing value vector for signal {}", signal_id))
    })?;
    push_scalar_slice(channels.as_mut_slice(), channel_idx, sample)
}

fn push_scalar_slice(
    channels: &mut [SignalChannel],
    channel_idx: usize,
    sample: NumericSample,
) -> Result<(), CadencePsfError> {
    let channel_count = channels.len();
    let channel = channels.get_mut(channel_idx).ok_or_else(|| {
        CadencePsfError::new(format!(
            "decoded sample for channel index {} but only {} channel(s) allocated",
            channel_idx, channel_count
        ))
    })?;
    match (&mut channel.values, sample) {
        (SignalValues::Real(v), NumericSample::Real(value)) => {
            v.push(value);
            Ok(())
        }
        (SignalValues::Complex(v), NumericSample::Complex(value)) => {
            v.push(value);
            Ok(())
        }
        (SignalValues::Real(_), NumericSample::Complex(_)) => Err(CadencePsfError::new(format!(
            "complex sample written to real channel '{}'",
            channel.suffix
        ))),
        (SignalValues::Complex(_), NumericSample::Real(_)) => Err(CadencePsfError::new(format!(
            "real sample written to complex channel '{}'",
            channel.suffix
        ))),
    }
}

#[cfg(test)]
pub(crate) mod test_helpers;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod malformed_tests;

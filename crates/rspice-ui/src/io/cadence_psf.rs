//! Cadence PSF native binary parsing.
//!
//! This module provides a local parser for Cadence/Spectre PSF binary payloads
//! so waveform import does not depend on external crates.

use std::collections::HashMap;
use std::fmt;

#[path = "cadence_psf_type_meta.rs"]
mod cadence_psf_type_meta;
use cadence_psf_type_meta::TypeMetaCache;

#[path = "cadence_psf/binary_io.rs"]
mod binary_io;
use binary_io::{
    parse_string, peek_u32, read_f64, read_i32, read_u32, read_u8_padded, skip_opaque_scalar,
};

#[path = "cadence_psf/toc.rs"]
mod toc;
use toc::{parse_toc, SectionKind, Toc, TocEntry};

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
        if let Some(channels) = signal_values.get(&signal.id) {
            for channel in channels {
                let name = qualify_signal_name(&signal.name, &channel.suffix);
                match &channel.values {
                    SignalValues::Real(values) => real_signals.push(NamedRealSignal {
                        name,
                        values: values.clone(),
                    }),
                    SignalValues::Complex(values) => complex_signals.push(NamedComplexSignal {
                        name,
                        values: values.clone(),
                    }),
                }
            }
        }
    }

    let mut sweep_signals = Vec::new();
    for sweep in &sweeps {
        if let Some(channels) = signal_values.get(&sweep.id) {
            for channel in channels {
                if let SignalValues::Real(values) = &channel.values {
                    sweep_signals.push(NamedRealSignal {
                        name: qualify_signal_name(&sweep.name, &channel.suffix),
                        values: values.clone(),
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
        let (name, value) = parse_named_value(&mut cursor)?;
        header.insert(name, value);
    }
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
        parse_type_decl(&mut cursor, &mut types)?;
    }

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

    let mut values: HashMap<u32, Vec<SignalChannel>> = HashMap::new();
    for sweep in sweeps {
        values.insert(
            sweep.id,
            vec![SignalChannel {
                suffix: String::new(),
                values: SignalValues::Real(Vec::new()),
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
            values.insert(signal.id, init_channels(&specs));
        }
    }

    if is_windowed {
        parse_windowed_values(
            &file[entry.start + 8..end_of_section],
            header,
            sweeps,
            &flat_traces,
            types,
            &signal_has_dynamic_arrays,
            &mut values,
            &mut type_meta,
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
    types: &HashMap<u32, TypeDecl>,
    values: &mut HashMap<u32, Vec<SignalChannel>>,
) -> Result<(), CadencePsfError> {
    let sweep_points = header_usize(header, "PSF sweep points")?;
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

    Ok(())
}

fn parse_windowed_values(
    mut cursor: &[u8],
    header: &HashMap<String, CadencePsfValue>,
    sweeps: &[SignalRef],
    flat_traces: &[SignalRef],
    types: &HashMap<u32, TypeDecl>,
    signal_has_dynamic_arrays: &HashMap<u32, bool>,
    values: &mut HashMap<u32, Vec<SignalChannel>>,
    type_meta: &mut TypeMetaCache,
) -> Result<(), CadencePsfError> {
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
                let idx = if data_len > window_size {
                    0
                } else {
                    window_size - data_len
                };
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

fn qualify_signal_name(base: &str, suffix: &str) -> String {
    if suffix.is_empty() {
        base.to_string()
    } else if suffix.starts_with('[') {
        format!("{}{}", base, suffix)
    } else {
        format!("{}.{}", base, suffix)
    }
}

fn init_channels(specs: &[ChannelSpec]) -> Vec<SignalChannel> {
    specs
        .iter()
        .map(|spec| SignalChannel {
            suffix: spec.suffix.clone(),
            values: match spec.kind {
                ChannelKind::Real => SignalValues::Real(Vec::new()),
                ChannelKind::Complex => SignalValues::Complex(Vec::new()),
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
    Err(CadencePsfError::new(format!(
        "{}",
        if want_array {
            format!(
                "array element descriptor ARRAY is ambiguous across type ids {:?}",
                candidates
            )
        } else {
            format!(
                "array element descriptor STRUCT is ambiguous across type ids {:?}",
                candidates
            )
        }
    )))
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
        SignalValues::Real(v) => {
            while v.len() < len {
                v.push(f64::NAN);
            }
        }
        SignalValues::Complex(v) => {
            while v.len() < len {
                v.push((f64::NAN, f64::NAN));
            }
        }
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

fn find_windowed_dynamic_data_start(
    segment: &[u8],
    signal: &SignalRef,
    window_count: usize,
    types: &HashMap<u32, TypeDecl>,
    type_meta: &mut TypeMetaCache,
) -> Result<usize, CadencePsfError> {
    let min_per_sample = type_meta.min_encoded_size(signal.type_id, types)?;
    let min_total = min_per_sample
        .checked_mul(window_count)
        .ok_or_else(|| CadencePsfError::new("windowed minimum data-size overflow"))?;
    if min_total > segment.len() {
        return Err(CadencePsfError::new(format!(
            "windowed segment for signal '{}' is too short: min_required={}, actual={}",
            signal.name,
            min_total,
            segment.len()
        )));
    }
    let max_start = segment.len() - min_total;

    let mut best_start = scan_dynamic_start_candidates(
        segment,
        signal,
        window_count,
        types,
        (0..=max_start).step_by(4),
    )?;
    if best_start.is_none() {
        best_start =
            scan_dynamic_start_candidates(segment, signal, window_count, types, 0..=max_start)?;
    }

    best_start.map(|(start, _)| start).ok_or_else(|| {
        CadencePsfError::new(format!(
            "unable to locate payload start for windowed array signal '{}'",
            signal.name
        ))
    })
}

fn scan_dynamic_start_candidates<I>(
    segment: &[u8],
    signal: &SignalRef,
    window_count: usize,
    types: &HashMap<u32, TypeDecl>,
    starts: I,
) -> Result<Option<(usize, usize)>, CadencePsfError>
where
    I: IntoIterator<Item = usize>,
{
    let mut best_start = None;
    let mut best_numeric_count = 0usize;

    for start in starts {
        let mut cursor = &segment[start..];
        let mut ok = true;
        let mut numeric_count = 0usize;
        for _ in 0..window_count {
            match count_numeric_type_value(&mut cursor, signal.type_id, types) {
                Ok(count) => {
                    numeric_count = numeric_count.checked_add(count).ok_or_else(|| {
                        CadencePsfError::new("windowed numeric sample count overflow")
                    })?;
                }
                Err(_) => {
                    ok = false;
                    break;
                }
            }
        }
        if ok && cursor.is_empty() {
            match best_start {
                None => {
                    best_start = Some(start);
                    best_numeric_count = numeric_count;
                }
                Some(current_best) => {
                    if numeric_count > best_numeric_count
                        || (numeric_count == best_numeric_count && start < current_best)
                    {
                        best_start = Some(start);
                        best_numeric_count = numeric_count;
                    }
                }
            }
        }
    }

    Ok(best_start.map(|start| (start, best_numeric_count)))
}

fn decode_windowed_dynamic_signal_samples(
    segment: &[u8],
    signal: &SignalRef,
    window_count: usize,
    point_offset: usize,
    types: &HashMap<u32, TypeDecl>,
    values: &mut HashMap<u32, Vec<SignalChannel>>,
    channel_index_cache: &mut HashMap<u32, HashMap<String, usize>>,
    type_meta: &mut TypeMetaCache,
) -> Result<(), CadencePsfError> {
    if window_count == 0 {
        return Ok(());
    }

    let start = find_windowed_dynamic_data_start(segment, signal, window_count, types, type_meta)?;
    let mut cursor = &segment[start..];
    for local_idx in 0..window_count {
        let point_idx = point_offset
            .checked_add(local_idx)
            .ok_or_else(|| CadencePsfError::new("windowed sample point index overflow"))?;
        let mut touched_channels = Vec::new();
        read_type_value_with_numeric_visit(
            &mut cursor,
            signal.type_id,
            types,
            "",
            &mut |suffix, sample| {
                let channel_idx = push_named_channel_cached(
                    values,
                    channel_index_cache,
                    signal.id,
                    suffix,
                    sample,
                    point_idx,
                )?;
                touched_channels.push(channel_idx);
                Ok(())
            },
        )?;
        let target_len = point_idx
            .checked_add(1)
            .ok_or_else(|| CadencePsfError::new("windowed channel length overflow"))?;
        pad_untouched_channels(values, signal.id, target_len, &touched_channels)?;
    }

    if !cursor.is_empty() {
        return Err(CadencePsfError::new(format!(
            "windowed dynamic decode for signal '{}' left {} trailing byte(s)",
            signal.name,
            cursor.len()
        )));
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

fn count_numeric_type_value(
    cursor: &mut &[u8],
    type_id: u32,
    types: &HashMap<u32, TypeDecl>,
) -> Result<usize, CadencePsfError> {
    let decl = types
        .get(&type_id)
        .ok_or_else(|| CadencePsfError::new(format!("missing type declaration {}", type_id)))?;
    match &decl.kind {
        TypeKind::Primitive(dtype) => count_numeric_data_type_value(cursor, *dtype),
        TypeKind::Array { element_type_raw } => {
            let count = read_u32(cursor)? as usize;
            let mut numeric_count = 0usize;
            for _ in 0..count {
                numeric_count = numeric_count
                    .checked_add(count_numeric_array_element_value(
                        cursor,
                        *element_type_raw,
                        types,
                        Some(type_id),
                    )?)
                    .ok_or_else(|| CadencePsfError::new("numeric sample count overflow"))?;
            }
            Ok(numeric_count)
        }
        TypeKind::Struct { members } => {
            let mut numeric_count = 0usize;
            for member_id in members {
                numeric_count = numeric_count
                    .checked_add(count_numeric_type_value(cursor, *member_id, types)?)
                    .ok_or_else(|| CadencePsfError::new("numeric sample count overflow"))?;
            }
            Ok(numeric_count)
        }
    }
}

fn count_numeric_array_element_value(
    cursor: &mut &[u8],
    element_type_raw: u32,
    types: &HashMap<u32, TypeDecl>,
    parent_type_id: Option<u32>,
) -> Result<usize, CadencePsfError> {
    match resolve_array_element_type(element_type_raw, types, parent_type_id)? {
        ArrayElementType::Primitive(dtype) => count_numeric_data_type_value(cursor, dtype),
        ArrayElementType::TypeRef(type_id) => count_numeric_type_value(cursor, type_id, types),
    }
}

fn count_numeric_data_type_value(
    cursor: &mut &[u8],
    dtype: DataType,
) -> Result<usize, CadencePsfError> {
    match dtype {
        DataType::Int8 => {
            let _ = read_u8_padded(cursor)?;
            Ok(1)
        }
        DataType::Int32 => {
            let _ = read_i32(cursor)?;
            Ok(1)
        }
        DataType::Real => {
            let _ = read_f64(cursor)?;
            Ok(1)
        }
        DataType::Complex => {
            let _ = read_f64(cursor)?;
            let _ = read_f64(cursor)?;
            Ok(1)
        }
        DataType::String => {
            let _ = parse_string(cursor)?;
            Ok(0)
        }
        DataType::Array => Err(CadencePsfError::new(
            "array element descriptor resolved to ARRAY without a concrete type reference",
        )),
        DataType::Struct => Err(CadencePsfError::new(
            "array element descriptor resolved to STRUCT without a concrete type reference",
        )),
        DataType::Other(_) => {
            skip_opaque_scalar(cursor)?;
            Ok(0)
        }
    }
}

fn read_type_value_with_numeric_visit<F>(
    cursor: &mut &[u8],
    type_id: u32,
    types: &HashMap<u32, TypeDecl>,
    prefix: &str,
    on_numeric: &mut F,
) -> Result<(), CadencePsfError>
where
    F: FnMut(&str, NumericSample) -> Result<(), CadencePsfError>,
{
    let decl = types
        .get(&type_id)
        .ok_or_else(|| CadencePsfError::new(format!("missing type declaration {}", type_id)))?;
    match &decl.kind {
        TypeKind::Primitive(dtype) => {
            read_data_type_value_with_numeric_visit(cursor, *dtype, prefix, on_numeric)
        }
        TypeKind::Array { element_type_raw } => {
            let count = read_u32(cursor)? as usize;
            for idx in 0..count {
                let next = format!("{}[{}]", prefix, idx);
                read_array_element_value_with_numeric_visit(
                    cursor,
                    *element_type_raw,
                    types,
                    &next,
                    on_numeric,
                    Some(type_id),
                )?;
            }
            Ok(())
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
                read_type_value_with_numeric_visit(cursor, *member_id, types, &next, on_numeric)?;
            }
            Ok(())
        }
    }
}

fn read_array_element_value_with_numeric_visit<F>(
    cursor: &mut &[u8],
    element_type_raw: u32,
    types: &HashMap<u32, TypeDecl>,
    suffix: &str,
    on_numeric: &mut F,
    parent_type_id: Option<u32>,
) -> Result<(), CadencePsfError>
where
    F: FnMut(&str, NumericSample) -> Result<(), CadencePsfError>,
{
    match resolve_array_element_type(element_type_raw, types, parent_type_id)? {
        ArrayElementType::Primitive(dtype) => {
            read_data_type_value_with_numeric_visit(cursor, dtype, suffix, on_numeric)
        }
        ArrayElementType::TypeRef(type_id) => {
            read_type_value_with_numeric_visit(cursor, type_id, types, suffix, on_numeric)
        }
    }
}

fn read_data_type_value_with_numeric_visit<F>(
    cursor: &mut &[u8],
    dtype: DataType,
    suffix: &str,
    on_numeric: &mut F,
) -> Result<(), CadencePsfError>
where
    F: FnMut(&str, NumericSample) -> Result<(), CadencePsfError>,
{
    match dtype {
        DataType::Int8 => on_numeric(suffix, NumericSample::Real(read_u8_padded(cursor)? as f64)),
        DataType::Int32 => on_numeric(suffix, NumericSample::Real(read_i32(cursor)? as f64)),
        DataType::Real => on_numeric(suffix, NumericSample::Real(read_f64(cursor)?)),
        DataType::Complex => {
            let re = read_f64(cursor)?;
            let im = read_f64(cursor)?;
            on_numeric(suffix, NumericSample::Complex((re, im)))
        }
        DataType::String => {
            let _ = parse_string(cursor)?;
            Ok(())
        }
        DataType::Array => Err(CadencePsfError::new(
            "array element descriptor resolved to ARRAY without a concrete type reference",
        )),
        DataType::Struct => Err(CadencePsfError::new(
            "array element descriptor resolved to STRUCT without a concrete type reference",
        )),
        DataType::Other(_) => {
            skip_opaque_scalar(cursor)?;
            Ok(())
        }
    }
}

#[cfg(test)]
pub(crate) mod test_helpers {
    use super::*;

    #[derive(Clone, Copy)]
    enum SampleEncoding {
        Real,
        Complex,
        Int8,
        Int32,
        UnknownWord,
    }

    impl SampleEncoding {
        fn type_code(self) -> u32 {
            match self {
                Self::Real => 11,
                Self::Complex => 12,
                Self::Int8 => 1,
                Self::Int32 => 5,
                Self::UnknownWord => 99,
            }
        }
    }

    pub(crate) fn build_non_windowed_real_psf() -> Vec<u8> {
        build_simple_non_windowed_psf(SampleEncoding::Real)
    }

    pub(crate) fn build_non_windowed_complex_psf() -> Vec<u8> {
        build_simple_non_windowed_psf(SampleEncoding::Complex)
    }

    pub(crate) fn build_non_windowed_int8_psf() -> Vec<u8> {
        build_simple_non_windowed_psf(SampleEncoding::Int8)
    }

    pub(crate) fn build_non_windowed_int32_psf() -> Vec<u8> {
        build_simple_non_windowed_psf(SampleEncoding::Int32)
    }

    pub(crate) fn build_non_windowed_unknown_word_psf() -> Vec<u8> {
        build_simple_non_windowed_psf(SampleEncoding::UnknownWord)
    }

    pub(crate) fn build_non_windowed_struct_psf() -> Vec<u8> {
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
        // Root type: struct with two members.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 1);
        push_string(&mut bytes, "sigtype");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        // Member 1: real scalar
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_string(&mut bytes, "dc");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        // Member 2: complex scalar
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 3);
        push_string(&mut bytes, "ac");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 12);
        push_u32(&mut bytes, 18); // struct end
        let types_end = bytes.len() as u32;
        patch_u32(&mut bytes, types_eofs_pos, types_end);

        let sweep_start = bytes.len();
        push_u32(&mut bytes, 0);
        let sweep_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 100, "time", 2);
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

        // Point 0
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 1.0); // dc
        push_f64(&mut bytes, 2.0); // ac.re
        push_f64(&mut bytes, 0.5); // ac.im

        // Point 1
        push_u32(&mut bytes, 1);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 1.0);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 1.5); // dc
        push_f64(&mut bytes, 2.5); // ac.re
        push_f64(&mut bytes, -0.25); // ac.im

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

    pub(crate) fn build_non_windowed_mixed_real_and_string_psf() -> Vec<u8> {
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
        // Real scalar type
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 1);
        push_string(&mut bytes, "realtype");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        // String scalar type
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_string(&mut bytes, "stringtype");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 2);
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
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 201, "meta", 2);
        let trace_end = bytes.len() as u32;
        patch_u32(&mut bytes, trace_eofs_pos, trace_end);

        let value_start = bytes.len();
        push_u32(&mut bytes, 0);
        let value_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);

        // Point 0
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 1.25);
        push_f64(&mut bytes, 0.0);
        push_string(&mut bytes, "A");

        // Point 1
        push_u32(&mut bytes, 1);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 1.0);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 2.5);
        push_f64(&mut bytes, 0.0);
        push_string(&mut bytes, "B2");

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

    pub(crate) fn build_non_windowed_array_real_psf() -> Vec<u8> {
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
        // Top-level array trace type: real elements.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 1);
        push_string(&mut bytes, "realarray");
        push_u32(&mut bytes, 11);
        push_u32(&mut bytes, 3);
        // Sweep type: scalar real.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_string(&mut bytes, "real");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        let types_end = bytes.len() as u32;
        patch_u32(&mut bytes, types_eofs_pos, types_end);

        let sweep_start = bytes.len();
        push_u32(&mut bytes, 0);
        let sweep_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 100, "time", 2);
        let sweep_end = bytes.len() as u32;
        patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

        let trace_start = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 22);
        let trace_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 200, "V(arr)", 1);
        let trace_end = bytes.len() as u32;
        patch_u32(&mut bytes, trace_eofs_pos, trace_end);

        let value_start = bytes.len();
        push_u32(&mut bytes, 0);
        let value_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);

        // Point 0
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 0.0);
        push_u32(&mut bytes, 2);
        push_f64(&mut bytes, 1.0);
        push_f64(&mut bytes, 2.0);

        // Point 1
        push_u32(&mut bytes, 1);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 1.0);
        push_f64(&mut bytes, 0.0);
        push_u32(&mut bytes, 2);
        push_f64(&mut bytes, 1.5);
        push_f64(&mut bytes, 2.5);

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

    pub(crate) fn build_non_windowed_array_complex_psf() -> Vec<u8> {
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
        // Top-level array trace type: complex elements.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 1);
        push_string(&mut bytes, "complexarray");
        push_u32(&mut bytes, 12);
        push_u32(&mut bytes, 3);
        // Sweep type: scalar real.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_string(&mut bytes, "real");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        let types_end = bytes.len() as u32;
        patch_u32(&mut bytes, types_eofs_pos, types_end);

        let sweep_start = bytes.len();
        push_u32(&mut bytes, 0);
        let sweep_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 100, "time", 2);
        let sweep_end = bytes.len() as u32;
        patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

        let trace_start = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 22);
        let trace_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 200, "I(arr)", 1);
        let trace_end = bytes.len() as u32;
        patch_u32(&mut bytes, trace_eofs_pos, trace_end);

        let value_start = bytes.len();
        push_u32(&mut bytes, 0);
        let value_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);

        // Point 0
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 0.0);
        push_u32(&mut bytes, 2);
        push_f64(&mut bytes, 1.0);
        push_f64(&mut bytes, 0.25);
        push_f64(&mut bytes, 2.0);
        push_f64(&mut bytes, -0.5);

        // Point 1
        push_u32(&mut bytes, 1);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 1.0);
        push_f64(&mut bytes, 0.0);
        push_u32(&mut bytes, 2);
        push_f64(&mut bytes, 1.5);
        push_f64(&mut bytes, 0.125);
        push_f64(&mut bytes, 2.5);
        push_f64(&mut bytes, -0.75);

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

    pub(crate) fn build_non_windowed_struct_with_array_psf() -> Vec<u8> {
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
        // Root type: struct with scalar and array members.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 1);
        push_string(&mut bytes, "sigtype");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        // gain: real scalar
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_string(&mut bytes, "gain");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        // taps: real array
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 3);
        push_string(&mut bytes, "taps");
        push_u32(&mut bytes, 11);
        push_u32(&mut bytes, 3);
        push_u32(&mut bytes, 18);
        // Sweep type: scalar real.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 4);
        push_string(&mut bytes, "real");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        let types_end = bytes.len() as u32;
        patch_u32(&mut bytes, types_eofs_pos, types_end);

        let sweep_start = bytes.len();
        push_u32(&mut bytes, 0);
        let sweep_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 100, "time", 4);
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

        // Point 0
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 10.0); // gain
        push_u32(&mut bytes, 2); // taps count
        push_f64(&mut bytes, 0.1);
        push_f64(&mut bytes, 0.2);

        // Point 1
        push_u32(&mut bytes, 1);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 1.0);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 11.0); // gain
        push_u32(&mut bytes, 2); // taps count
        push_f64(&mut bytes, 0.15);
        push_f64(&mut bytes, 0.25);

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

    pub(crate) fn build_non_windowed_variable_length_array_psf() -> Vec<u8> {
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
        // Top-level array trace type: real elements.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 1);
        push_string(&mut bytes, "realarray");
        push_u32(&mut bytes, 11);
        push_u32(&mut bytes, 3);
        // Sweep type: scalar real.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_string(&mut bytes, "real");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        let types_end = bytes.len() as u32;
        patch_u32(&mut bytes, types_eofs_pos, types_end);

        let sweep_start = bytes.len();
        push_u32(&mut bytes, 0);
        let sweep_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 100, "time", 2);
        let sweep_end = bytes.len() as u32;
        patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

        let trace_start = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 22);
        let trace_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 200, "V(arr)", 1);
        let trace_end = bytes.len() as u32;
        patch_u32(&mut bytes, trace_eofs_pos, trace_end);

        let value_start = bytes.len();
        push_u32(&mut bytes, 0);
        let value_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);

        // Point 0: one array value.
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 0.0);
        push_u32(&mut bytes, 1);
        push_f64(&mut bytes, 1.0);

        // Point 1: three array values.
        push_u32(&mut bytes, 1);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 1.0);
        push_f64(&mut bytes, 0.0);
        push_u32(&mut bytes, 3);
        push_f64(&mut bytes, 1.5);
        push_f64(&mut bytes, 2.5);
        push_f64(&mut bytes, 3.5);

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

    pub(crate) fn build_windowed_real_psf() -> Vec<u8> {
        let mut bytes = Vec::new();

        let header_start = bytes.len();
        push_u32(&mut bytes, 0);
        let header_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_named_int(&mut bytes, "PSF sweep points", 2);
        push_named_int(&mut bytes, "PSF traces", 1);
        push_named_int(&mut bytes, "PSF window size", 24);
        let header_end = bytes.len() as u32;
        patch_u32(&mut bytes, header_eofs_pos, header_end);

        let types_start = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 22);
        let types_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        // Trace type: real scalar.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 1);
        push_string(&mut bytes, "real");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        // Sweep type: real scalar.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_string(&mut bytes, "real");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        let types_end = bytes.len() as u32;
        patch_u32(&mut bytes, types_eofs_pos, types_end);

        let sweep_start = bytes.len();
        push_u32(&mut bytes, 0);
        let sweep_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 100, "time", 2);
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

        push_u32(&mut bytes, 20);
        push_u32(&mut bytes, 0);

        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 1.0);

        let mut trace_payload = Vec::new();
        push_f64(&mut trace_payload, 1.0);
        push_f64(&mut trace_payload, 2.0);
        push_windowed_trace_payload(&mut bytes, &trace_payload, 24);

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

    pub(crate) fn build_windowed_array_real_psf() -> Vec<u8> {
        let mut bytes = Vec::new();

        let header_start = bytes.len();
        push_u32(&mut bytes, 0);
        let header_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_named_int(&mut bytes, "PSF sweep points", 2);
        push_named_int(&mut bytes, "PSF traces", 1);
        push_named_int(&mut bytes, "PSF window size", 48);
        let header_end = bytes.len() as u32;
        patch_u32(&mut bytes, header_eofs_pos, header_end);

        let types_start = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 22);
        let types_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        // Trace type: array of real.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 1);
        push_string(&mut bytes, "realarray");
        push_u32(&mut bytes, 11);
        push_u32(&mut bytes, 3);
        // Sweep type: real scalar.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_string(&mut bytes, "real");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        let types_end = bytes.len() as u32;
        patch_u32(&mut bytes, types_eofs_pos, types_end);

        let sweep_start = bytes.len();
        push_u32(&mut bytes, 0);
        let sweep_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 100, "time", 2);
        let sweep_end = bytes.len() as u32;
        patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

        let trace_start = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 22);
        let trace_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 200, "V(arr)", 1);
        let trace_end = bytes.len() as u32;
        patch_u32(&mut bytes, trace_eofs_pos, trace_end);

        let value_start = bytes.len();
        push_u32(&mut bytes, 0);
        let value_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);

        push_u32(&mut bytes, 20);
        push_u32(&mut bytes, 0);

        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 1.0);

        let mut trace_payload = Vec::new();
        push_u32(&mut trace_payload, 2);
        push_f64(&mut trace_payload, 1.0);
        push_f64(&mut trace_payload, 2.0);
        push_u32(&mut trace_payload, 2);
        push_f64(&mut trace_payload, 1.5);
        push_f64(&mut trace_payload, 2.5);
        push_windowed_trace_payload(&mut bytes, &trace_payload, 48);

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

    pub(crate) fn build_windowed_array_real_unaligned_payload_psf() -> Vec<u8> {
        let mut bytes = Vec::new();

        let header_start = bytes.len();
        push_u32(&mut bytes, 0);
        let header_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_named_int(&mut bytes, "PSF sweep points", 2);
        push_named_int(&mut bytes, "PSF traces", 1);
        // 50-byte windows intentionally force non-4-byte-aligned payload start.
        push_named_int(&mut bytes, "PSF window size", 50);
        let header_end = bytes.len() as u32;
        patch_u32(&mut bytes, header_eofs_pos, header_end);

        let types_start = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 22);
        let types_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        // Trace type: array of real.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 1);
        push_string(&mut bytes, "realarray");
        push_u32(&mut bytes, 11);
        push_u32(&mut bytes, 3);
        // Sweep type: real scalar.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_string(&mut bytes, "real");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        let types_end = bytes.len() as u32;
        patch_u32(&mut bytes, types_eofs_pos, types_end);

        let sweep_start = bytes.len();
        push_u32(&mut bytes, 0);
        let sweep_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 100, "time", 2);
        let sweep_end = bytes.len() as u32;
        patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

        let trace_start = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 22);
        let trace_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 200, "V(arr)", 1);
        let trace_end = bytes.len() as u32;
        patch_u32(&mut bytes, trace_eofs_pos, trace_end);

        let value_start = bytes.len();
        push_u32(&mut bytes, 0);
        let value_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);

        push_u32(&mut bytes, 20);
        push_u32(&mut bytes, 0);

        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 1.0);

        let mut trace_payload = Vec::new();
        push_u32(&mut trace_payload, 2);
        push_f64(&mut trace_payload, 1.0);
        push_f64(&mut trace_payload, 2.0);
        push_u32(&mut trace_payload, 2);
        push_f64(&mut trace_payload, 1.5);
        push_f64(&mut trace_payload, 2.5);
        push_windowed_trace_payload(&mut bytes, &trace_payload, 50);

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

    pub(crate) fn build_windowed_array_complex_psf() -> Vec<u8> {
        let mut bytes = Vec::new();

        let header_start = bytes.len();
        push_u32(&mut bytes, 0);
        let header_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_named_int(&mut bytes, "PSF sweep points", 2);
        push_named_int(&mut bytes, "PSF traces", 1);
        push_named_int(&mut bytes, "PSF window size", 80);
        let header_end = bytes.len() as u32;
        patch_u32(&mut bytes, header_eofs_pos, header_end);

        let types_start = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 22);
        let types_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        // Trace type: array of complex.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 1);
        push_string(&mut bytes, "complexarray");
        push_u32(&mut bytes, 12);
        push_u32(&mut bytes, 3);
        // Sweep type: real scalar.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_string(&mut bytes, "real");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        let types_end = bytes.len() as u32;
        patch_u32(&mut bytes, types_eofs_pos, types_end);

        let sweep_start = bytes.len();
        push_u32(&mut bytes, 0);
        let sweep_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 100, "time", 2);
        let sweep_end = bytes.len() as u32;
        patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

        let trace_start = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 22);
        let trace_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 200, "I(arr)", 1);
        let trace_end = bytes.len() as u32;
        patch_u32(&mut bytes, trace_eofs_pos, trace_end);

        let value_start = bytes.len();
        push_u32(&mut bytes, 0);
        let value_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);

        push_u32(&mut bytes, 20);
        push_u32(&mut bytes, 0);

        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 1.0);

        let mut trace_payload = Vec::new();
        push_u32(&mut trace_payload, 2);
        push_f64(&mut trace_payload, 1.0);
        push_f64(&mut trace_payload, 0.25);
        push_f64(&mut trace_payload, 2.0);
        push_f64(&mut trace_payload, -0.5);
        push_u32(&mut trace_payload, 2);
        push_f64(&mut trace_payload, 1.5);
        push_f64(&mut trace_payload, 0.125);
        push_f64(&mut trace_payload, 2.5);
        push_f64(&mut trace_payload, -0.75);
        push_windowed_trace_payload(&mut bytes, &trace_payload, 80);

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

    pub(crate) fn build_windowed_struct_with_array_psf() -> Vec<u8> {
        let mut bytes = Vec::new();

        let header_start = bytes.len();
        push_u32(&mut bytes, 0);
        let header_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_named_int(&mut bytes, "PSF sweep points", 2);
        push_named_int(&mut bytes, "PSF traces", 1);
        push_named_int(&mut bytes, "PSF window size", 64);
        let header_end = bytes.len() as u32;
        patch_u32(&mut bytes, header_eofs_pos, header_end);

        let types_start = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 22);
        let types_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        // Root type: struct with scalar and array.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 1);
        push_string(&mut bytes, "sigtype");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_string(&mut bytes, "gain");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 3);
        push_string(&mut bytes, "taps");
        push_u32(&mut bytes, 11);
        push_u32(&mut bytes, 3);
        push_u32(&mut bytes, 18);
        // Sweep type: real scalar.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 4);
        push_string(&mut bytes, "real");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        let types_end = bytes.len() as u32;
        patch_u32(&mut bytes, types_eofs_pos, types_end);

        let sweep_start = bytes.len();
        push_u32(&mut bytes, 0);
        let sweep_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 100, "time", 4);
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

        push_u32(&mut bytes, 20);
        push_u32(&mut bytes, 0);

        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 1.0);

        let mut trace_payload = Vec::new();
        push_f64(&mut trace_payload, 10.0);
        push_u32(&mut trace_payload, 2);
        push_f64(&mut trace_payload, 0.1);
        push_f64(&mut trace_payload, 0.2);
        push_f64(&mut trace_payload, 11.0);
        push_u32(&mut trace_payload, 2);
        push_f64(&mut trace_payload, 0.15);
        push_f64(&mut trace_payload, 0.25);
        push_windowed_trace_payload(&mut bytes, &trace_payload, 64);

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

    pub(crate) fn build_windowed_variable_length_array_psf() -> Vec<u8> {
        let mut bytes = Vec::new();

        let header_start = bytes.len();
        push_u32(&mut bytes, 0);
        let header_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_named_int(&mut bytes, "PSF sweep points", 2);
        push_named_int(&mut bytes, "PSF traces", 1);
        push_named_int(&mut bytes, "PSF window size", 48);
        let header_end = bytes.len() as u32;
        patch_u32(&mut bytes, header_eofs_pos, header_end);

        let types_start = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 22);
        let types_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        // Trace type: array of real.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 1);
        push_string(&mut bytes, "realarray");
        push_u32(&mut bytes, 11);
        push_u32(&mut bytes, 3);
        // Sweep type: real scalar.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_string(&mut bytes, "real");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        let types_end = bytes.len() as u32;
        patch_u32(&mut bytes, types_eofs_pos, types_end);

        let sweep_start = bytes.len();
        push_u32(&mut bytes, 0);
        let sweep_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 100, "time", 2);
        let sweep_end = bytes.len() as u32;
        patch_u32(&mut bytes, sweep_eofs_pos, sweep_end);

        let trace_start = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 22);
        let trace_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 200, "V(arr)", 1);
        let trace_end = bytes.len() as u32;
        patch_u32(&mut bytes, trace_eofs_pos, trace_end);

        let value_start = bytes.len();
        push_u32(&mut bytes, 0);
        let value_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);

        push_u32(&mut bytes, 20);
        push_u32(&mut bytes, 0);

        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 1.0);

        let mut trace_payload = Vec::new();
        push_u32(&mut trace_payload, 1);
        push_f64(&mut trace_payload, 1.0);
        push_u32(&mut trace_payload, 3);
        push_f64(&mut trace_payload, 1.5);
        push_f64(&mut trace_payload, 2.5);
        push_f64(&mut trace_payload, 3.5);
        push_windowed_trace_payload(&mut bytes, &trace_payload, 48);

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

    pub(crate) fn build_non_windowed_array_of_struct_psf() -> Vec<u8> {
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
        // Top-level trace: array of struct(type_id=2).
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 1);
        push_string(&mut bytes, "array_of_struct");
        push_u32(&mut bytes, 2);
        push_u32(&mut bytes, 3);
        // Struct element type.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_string(&mut bytes, "elem");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 3);
        push_string(&mut bytes, "dc");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 4);
        push_string(&mut bytes, "ac");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 12);
        push_u32(&mut bytes, 18);
        // Sweep type.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 5);
        push_string(&mut bytes, "real");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        let types_end = bytes.len() as u32;
        patch_u32(&mut bytes, types_eofs_pos, types_end);

        let sweep_start = bytes.len();
        push_u32(&mut bytes, 0);
        let sweep_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 100, "time", 5);
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

        // Point 0
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 0.0);
        push_u32(&mut bytes, 2);
        push_f64(&mut bytes, 1.0);
        push_f64(&mut bytes, 2.0);
        push_f64(&mut bytes, 0.5);
        push_f64(&mut bytes, 1.1);
        push_f64(&mut bytes, 2.1);
        push_f64(&mut bytes, 0.6);

        // Point 1
        push_u32(&mut bytes, 1);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 1.0);
        push_f64(&mut bytes, 0.0);
        push_u32(&mut bytes, 2);
        push_f64(&mut bytes, 1.5);
        push_f64(&mut bytes, 2.5);
        push_f64(&mut bytes, -0.2);
        push_f64(&mut bytes, 1.6);
        push_f64(&mut bytes, 2.6);
        push_f64(&mut bytes, -0.3);

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

    pub(crate) fn build_non_windowed_nested_array_real_psf() -> Vec<u8> {
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
        // Top-level: array of type_id=2.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 1);
        push_string(&mut bytes, "array2d");
        push_u32(&mut bytes, 2);
        push_u32(&mut bytes, 3);
        // Inner type: array of real.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_string(&mut bytes, "inner");
        push_u32(&mut bytes, 11);
        push_u32(&mut bytes, 3);
        // Sweep type.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 3);
        push_string(&mut bytes, "real");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        let types_end = bytes.len() as u32;
        patch_u32(&mut bytes, types_eofs_pos, types_end);

        let sweep_start = bytes.len();
        push_u32(&mut bytes, 0);
        let sweep_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 100, "time", 3);
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

        // Point 0
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 0.0);
        push_u32(&mut bytes, 2);
        push_u32(&mut bytes, 2);
        push_f64(&mut bytes, 1.0);
        push_f64(&mut bytes, 2.0);
        push_u32(&mut bytes, 2);
        push_f64(&mut bytes, 3.0);
        push_f64(&mut bytes, 4.0);

        // Point 1
        push_u32(&mut bytes, 1);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 1.0);
        push_f64(&mut bytes, 0.0);
        push_u32(&mut bytes, 2);
        push_u32(&mut bytes, 2);
        push_f64(&mut bytes, 1.5);
        push_f64(&mut bytes, 2.5);
        push_u32(&mut bytes, 2);
        push_f64(&mut bytes, 3.5);
        push_f64(&mut bytes, 4.5);

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

    pub(crate) fn build_windowed_array_of_struct_psf() -> Vec<u8> {
        let mut bytes = Vec::new();

        let header_start = bytes.len();
        push_u32(&mut bytes, 0);
        let header_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_named_int(&mut bytes, "PSF sweep points", 2);
        push_named_int(&mut bytes, "PSF traces", 1);
        push_named_int(&mut bytes, "PSF window size", 128);
        let header_end = bytes.len() as u32;
        patch_u32(&mut bytes, header_eofs_pos, header_end);

        let types_start = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 22);
        let types_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        // Top-level trace: array of struct(type_id=2).
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 1);
        push_string(&mut bytes, "array_of_struct");
        push_u32(&mut bytes, 2);
        push_u32(&mut bytes, 3);
        // Struct element type.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_string(&mut bytes, "elem");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 3);
        push_string(&mut bytes, "dc");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 4);
        push_string(&mut bytes, "ac");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 12);
        push_u32(&mut bytes, 18);
        // Sweep type.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 5);
        push_string(&mut bytes, "real");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        let types_end = bytes.len() as u32;
        patch_u32(&mut bytes, types_eofs_pos, types_end);

        let sweep_start = bytes.len();
        push_u32(&mut bytes, 0);
        let sweep_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 100, "time", 5);
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

        push_u32(&mut bytes, 20);
        push_u32(&mut bytes, 0);

        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 1.0);

        let mut trace_payload = Vec::new();
        push_u32(&mut trace_payload, 2);
        push_f64(&mut trace_payload, 1.0);
        push_f64(&mut trace_payload, 2.0);
        push_f64(&mut trace_payload, 0.5);
        push_f64(&mut trace_payload, 1.1);
        push_f64(&mut trace_payload, 2.1);
        push_f64(&mut trace_payload, 0.6);
        push_u32(&mut trace_payload, 2);
        push_f64(&mut trace_payload, 1.5);
        push_f64(&mut trace_payload, 2.5);
        push_f64(&mut trace_payload, -0.2);
        push_f64(&mut trace_payload, 1.6);
        push_f64(&mut trace_payload, 2.6);
        push_f64(&mut trace_payload, -0.3);
        push_windowed_trace_payload(&mut bytes, &trace_payload, 128);

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

    pub(crate) fn build_windowed_nested_array_real_psf() -> Vec<u8> {
        let mut bytes = Vec::new();

        let header_start = bytes.len();
        push_u32(&mut bytes, 0);
        let header_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_named_int(&mut bytes, "PSF sweep points", 2);
        push_named_int(&mut bytes, "PSF traces", 1);
        push_named_int(&mut bytes, "PSF window size", 112);
        let header_end = bytes.len() as u32;
        patch_u32(&mut bytes, header_eofs_pos, header_end);

        let types_start = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 22);
        let types_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        // Top-level: array of type_id=2.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 1);
        push_string(&mut bytes, "array2d");
        push_u32(&mut bytes, 2);
        push_u32(&mut bytes, 3);
        // Inner type: array of real.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_string(&mut bytes, "inner");
        push_u32(&mut bytes, 11);
        push_u32(&mut bytes, 3);
        // Sweep type.
        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 3);
        push_string(&mut bytes, "real");
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 11);
        let types_end = bytes.len() as u32;
        patch_u32(&mut bytes, types_eofs_pos, types_end);

        let sweep_start = bytes.len();
        push_u32(&mut bytes, 0);
        let sweep_eofs_pos = bytes.len();
        push_u32(&mut bytes, 0);
        push_u32(&mut bytes, 16);
        push_signal_ref(&mut bytes, 100, "time", 3);
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

        push_u32(&mut bytes, 20);
        push_u32(&mut bytes, 0);

        push_u32(&mut bytes, 16);
        push_u32(&mut bytes, 2);
        push_f64(&mut bytes, 0.0);
        push_f64(&mut bytes, 1.0);

        let mut trace_payload = Vec::new();
        push_u32(&mut trace_payload, 2);
        push_u32(&mut trace_payload, 2);
        push_f64(&mut trace_payload, 1.0);
        push_f64(&mut trace_payload, 2.0);
        push_u32(&mut trace_payload, 2);
        push_f64(&mut trace_payload, 3.0);
        push_f64(&mut trace_payload, 4.0);
        push_u32(&mut trace_payload, 2);
        push_u32(&mut trace_payload, 2);
        push_f64(&mut trace_payload, 1.5);
        push_f64(&mut trace_payload, 2.5);
        push_u32(&mut trace_payload, 2);
        push_f64(&mut trace_payload, 3.5);
        push_f64(&mut trace_payload, 4.5);
        push_windowed_trace_payload(&mut bytes, &trace_payload, 112);

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

    pub(crate) fn build_non_windowed_array_of_struct_bare_descriptor_psf() -> Vec<u8> {
        let mut bytes = build_non_windowed_array_of_struct_psf();
        patch_top_type_array_descriptor(&mut bytes, DataType::Struct.to_u32());
        bytes
    }

    pub(crate) fn build_non_windowed_nested_array_real_bare_descriptor_psf() -> Vec<u8> {
        let mut bytes = build_non_windowed_nested_array_real_psf();
        patch_top_type_array_descriptor(&mut bytes, DataType::Array.to_u32());
        bytes
    }

    pub(crate) fn build_windowed_array_of_struct_bare_descriptor_psf() -> Vec<u8> {
        let mut bytes = build_windowed_array_of_struct_psf();
        patch_top_type_array_descriptor(&mut bytes, DataType::Struct.to_u32());
        bytes
    }

    pub(crate) fn build_windowed_nested_array_real_bare_descriptor_psf() -> Vec<u8> {
        let mut bytes = build_windowed_nested_array_real_psf();
        patch_top_type_array_descriptor(&mut bytes, DataType::Array.to_u32());
        bytes
    }

    fn build_simple_non_windowed_psf(sample_encoding: SampleEncoding) -> Vec<u8> {
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
        push_u32(&mut bytes, sample_encoding.type_code());
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
        push_sample(&mut bytes, sample_encoding, 0);

        push_u32(&mut bytes, 1);
        push_u32(&mut bytes, 0);
        push_f64(&mut bytes, 1.0);
        push_f64(&mut bytes, 0.0);
        push_sample(&mut bytes, sample_encoding, 1);

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

    fn push_sample(bytes: &mut Vec<u8>, sample_encoding: SampleEncoding, sample_idx: usize) {
        match sample_encoding {
            SampleEncoding::Real => match sample_idx {
                0 => push_f64(bytes, 1.0),
                1 => push_f64(bytes, 2.0),
                _ => unreachable!("test helper has exactly two samples"),
            },
            SampleEncoding::Complex => match sample_idx {
                0 => {
                    push_f64(bytes, 1.0);
                    push_f64(bytes, 0.5);
                }
                1 => {
                    push_f64(bytes, 2.0);
                    push_f64(bytes, -0.25);
                }
                _ => unreachable!("test helper has exactly two samples"),
            },
            SampleEncoding::Int8 => match sample_idx {
                0 => push_u8_padded(bytes, 7),
                1 => push_u8_padded(bytes, 255),
                _ => unreachable!("test helper has exactly two samples"),
            },
            SampleEncoding::Int32 => match sample_idx {
                0 => push_i32(bytes, 1024),
                1 => push_i32(bytes, -2),
                _ => unreachable!("test helper has exactly two samples"),
            },
            SampleEncoding::UnknownWord => match sample_idx {
                0 => push_u32(bytes, 0xDEAD_BEEF),
                1 => push_u32(bytes, 0xC001_D00D),
                _ => unreachable!("test helper has exactly two samples"),
            },
        }
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

    fn push_i32(bytes: &mut Vec<u8>, value: i32) {
        bytes.extend_from_slice(&value.to_be_bytes());
    }

    fn push_u8_padded(bytes: &mut Vec<u8>, value: u8) {
        bytes.push(value);
        bytes.extend_from_slice(&[0u8; 3]);
    }

    fn push_windowed_trace_payload(bytes: &mut Vec<u8>, payload: &[u8], window_size: usize) {
        assert!(
            payload.len() <= window_size,
            "payload {} exceeds window_size {}",
            payload.len(),
            window_size
        );
        let mut window_block = vec![0u8; window_size];
        let start = window_size - payload.len();
        window_block[start..].copy_from_slice(payload);
        bytes.extend_from_slice(&window_block);
    }

    fn patch_top_type_array_descriptor(bytes: &mut [u8], new_descriptor: u32) {
        let toc = parse_toc(bytes).expect("fixture must contain valid TOC");
        let entry = toc
            .section(SectionKind::Type)
            .expect("fixture must contain type section");

        assert!(
            entry.start + 16 <= bytes.len(),
            "type section header must exist"
        );
        let block_type = peek_u32(&bytes[entry.start + 8..entry.start + 12]);
        assert_eq!(block_type, 22, "type section must be block 22");

        let mut idx = entry.start + 16;
        assert_eq!(
            peek_u32(&bytes[idx..idx + 4]),
            16,
            "first type item must be block 16"
        );
        idx += 4; // block
        idx += 4; // type id

        let name_len = peek_u32(&bytes[idx..idx + 4]) as usize;
        idx += 4;
        let name_pad = (4 - (name_len % 4)) % 4;
        idx += name_len + name_pad;

        patch_u32(bytes, idx, new_descriptor);
    }

    fn patch_u32(bytes: &mut [u8], offset: usize, value: u32) {
        bytes[offset..offset + 4].copy_from_slice(&value.to_be_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::test_helpers::{
        build_non_windowed_array_complex_psf,
        build_non_windowed_array_of_struct_bare_descriptor_psf,
        build_non_windowed_array_of_struct_psf, build_non_windowed_array_real_psf,
        build_non_windowed_complex_psf, build_non_windowed_int32_psf, build_non_windowed_int8_psf,
        build_non_windowed_mixed_real_and_string_psf,
        build_non_windowed_nested_array_real_bare_descriptor_psf,
        build_non_windowed_nested_array_real_psf, build_non_windowed_real_psf,
        build_non_windowed_struct_psf, build_non_windowed_struct_with_array_psf,
        build_non_windowed_unknown_word_psf, build_non_windowed_variable_length_array_psf,
        build_windowed_array_complex_psf, build_windowed_array_of_struct_bare_descriptor_psf,
        build_windowed_array_of_struct_psf, build_windowed_array_real_psf,
        build_windowed_array_real_unaligned_payload_psf,
        build_windowed_nested_array_real_bare_descriptor_psf, build_windowed_nested_array_real_psf,
        build_windowed_real_psf, build_windowed_struct_with_array_psf,
        build_windowed_variable_length_array_psf,
    };
    use super::*;

    fn patch_header_int(bytes: &mut [u8], key: &str, value: u32) {
        let toc = parse_toc(bytes).expect("fixture must contain valid TOC");
        let entry = toc
            .section(SectionKind::Header)
            .expect("fixture must contain header section");
        let mut idx = entry.start + 8;
        while idx + 4 <= entry.end {
            let block = peek_u32(&bytes[idx..idx + 4]);
            idx += 4;
            if !(33..=35).contains(&block) {
                break;
            }

            let name_len = peek_u32(&bytes[idx..idx + 4]) as usize;
            idx += 4;
            let name_start = idx;
            let name_end = name_start + name_len;
            let name = std::str::from_utf8(&bytes[name_start..name_end]).unwrap_or_default();
            let name_pad = (4 - (name_len % 4)) % 4;
            idx = name_end + name_pad;

            match block {
                33 => {
                    let value_len = peek_u32(&bytes[idx..idx + 4]) as usize;
                    idx += 4;
                    let value_pad = (4 - (value_len % 4)) % 4;
                    idx += value_len + value_pad;
                }
                34 => {
                    if name == key {
                        bytes[idx..idx + 4].copy_from_slice(&value.to_be_bytes());
                        return;
                    }
                    idx += 4;
                }
                35 => {
                    idx += 8;
                }
                _ => unreachable!(),
            }
        }
        panic!("header key '{}' not found in fixture", key);
    }

    fn patch_first_window_block_count(bytes: &mut [u8], window_count: u32) {
        let toc = parse_toc(bytes).expect("fixture must contain valid TOC");
        let entry = toc
            .section(SectionKind::Value)
            .expect("fixture must contain value section");
        let mut idx = entry.start + 8;
        assert_eq!(peek_u32(&bytes[idx..idx + 4]), 20);
        idx += 4;
        let zero_pad_len = peek_u32(&bytes[idx..idx + 4]) as usize;
        idx += 4 + zero_pad_len;
        assert_eq!(peek_u32(&bytes[idx..idx + 4]), 16);
        idx += 4;
        // block_init low 16-bits stores window_count in PSF payloads.
        bytes[idx..idx + 4].copy_from_slice(&window_count.to_be_bytes());
    }

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
    fn test_parse_non_windowed_int8_psf_binary() {
        let bytes = build_non_windowed_int8_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 1);
        assert_eq!(parsed.real_signals[0].name, "V(out)");
        assert_eq!(parsed.real_signals[0].values, vec![7.0, 255.0]);
    }

    #[test]
    fn test_parse_non_windowed_int32_psf_binary() {
        let bytes = build_non_windowed_int32_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 1);
        assert_eq!(parsed.real_signals[0].name, "V(out)");
        assert_eq!(parsed.real_signals[0].values, vec![1024.0, -2.0]);
    }

    #[test]
    fn test_parse_non_windowed_unknown_scalar_type_is_ignored() {
        let bytes = build_non_windowed_unknown_word_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.real_signals.is_empty());
        assert!(parsed.complex_signals.is_empty());
    }

    #[test]
    fn test_parse_non_windowed_struct_psf_binary() {
        let bytes = build_non_windowed_struct_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].name, "time");
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);

        assert_eq!(parsed.real_signals.len(), 1);
        assert_eq!(parsed.real_signals[0].name, "V(out).dc");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);

        assert_eq!(parsed.complex_signals.len(), 1);
        assert_eq!(parsed.complex_signals[0].name, "V(out).ac");
        assert_eq!(
            parsed.complex_signals[0].values,
            vec![(2.0, 0.5), (2.5, -0.25)]
        );
    }

    #[test]
    fn test_parse_non_windowed_string_trace_is_ignored() {
        let bytes = build_non_windowed_mixed_real_and_string_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.real_signals.len(), 1);
        assert_eq!(parsed.real_signals[0].name, "V(out)");
        assert_eq!(parsed.real_signals[0].values, vec![1.25, 2.5]);
        assert!(parsed.complex_signals.is_empty());
    }

    #[test]
    fn test_parse_non_windowed_real_array_psf_binary() {
        let bytes = build_non_windowed_array_real_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 2);
        assert_eq!(parsed.real_signals[0].name, "V(arr)[0]");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(arr)[1]");
        assert_eq!(parsed.real_signals[1].values, vec![2.0, 2.5]);
    }

    #[test]
    fn test_parse_non_windowed_complex_array_psf_binary() {
        let bytes = build_non_windowed_array_complex_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.real_signals.is_empty());
        assert_eq!(parsed.complex_signals.len(), 2);
        assert_eq!(parsed.complex_signals[0].name, "I(arr)[0]");
        assert_eq!(
            parsed.complex_signals[0].values,
            vec![(1.0, 0.25), (1.5, 0.125)]
        );
        assert_eq!(parsed.complex_signals[1].name, "I(arr)[1]");
        assert_eq!(
            parsed.complex_signals[1].values,
            vec![(2.0, -0.5), (2.5, -0.75)]
        );
    }

    #[test]
    fn test_parse_non_windowed_struct_with_array_psf_binary() {
        let bytes = build_non_windowed_struct_with_array_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 3);
        assert_eq!(parsed.real_signals[0].name, "V(out).gain");
        assert_eq!(parsed.real_signals[0].values, vec![10.0, 11.0]);
        assert_eq!(parsed.real_signals[1].name, "V(out).taps[0]");
        assert_eq!(parsed.real_signals[1].values, vec![0.1, 0.15]);
        assert_eq!(parsed.real_signals[2].name, "V(out).taps[1]");
        assert_eq!(parsed.real_signals[2].values, vec![0.2, 0.25]);
    }

    #[test]
    fn test_parse_non_windowed_variable_length_array_pads_missing_values() {
        let bytes = build_non_windowed_variable_length_array_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 3);
        assert_eq!(parsed.real_signals[0].name, "V(arr)[0]");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(arr)[1]");
        assert!(parsed.real_signals[1].values[0].is_nan());
        assert_eq!(parsed.real_signals[1].values[1], 2.5);
        assert_eq!(parsed.real_signals[2].name, "V(arr)[2]");
        assert!(parsed.real_signals[2].values[0].is_nan());
        assert_eq!(parsed.real_signals[2].values[1], 3.5);
    }

    #[test]
    fn test_parse_non_windowed_array_of_struct_psf_binary() {
        let bytes = build_non_windowed_array_of_struct_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert_eq!(parsed.real_signals.len(), 2);
        assert_eq!(parsed.real_signals[0].name, "V(out)[0].dc");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(out)[1].dc");
        assert_eq!(parsed.real_signals[1].values, vec![1.1, 1.6]);
        assert_eq!(parsed.complex_signals.len(), 2);
        assert_eq!(parsed.complex_signals[0].name, "V(out)[0].ac");
        assert_eq!(
            parsed.complex_signals[0].values,
            vec![(2.0, 0.5), (2.5, -0.2)]
        );
        assert_eq!(parsed.complex_signals[1].name, "V(out)[1].ac");
        assert_eq!(
            parsed.complex_signals[1].values,
            vec![(2.1, 0.6), (2.6, -0.3)]
        );
    }

    #[test]
    fn test_parse_non_windowed_array_of_struct_bare_descriptor_psf_binary() {
        let bytes = build_non_windowed_array_of_struct_bare_descriptor_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert_eq!(parsed.real_signals.len(), 2);
        assert_eq!(parsed.real_signals[0].name, "V(out)[0].dc");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(out)[1].dc");
        assert_eq!(parsed.real_signals[1].values, vec![1.1, 1.6]);
        assert_eq!(parsed.complex_signals.len(), 2);
        assert_eq!(parsed.complex_signals[0].name, "V(out)[0].ac");
        assert_eq!(
            parsed.complex_signals[0].values,
            vec![(2.0, 0.5), (2.5, -0.2)]
        );
        assert_eq!(parsed.complex_signals[1].name, "V(out)[1].ac");
        assert_eq!(
            parsed.complex_signals[1].values,
            vec![(2.1, 0.6), (2.6, -0.3)]
        );
    }

    #[test]
    fn test_parse_non_windowed_nested_array_real_psf_binary() {
        let bytes = build_non_windowed_nested_array_real_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 4);
        assert_eq!(parsed.real_signals[0].name, "V(out)[0][0]");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(out)[0][1]");
        assert_eq!(parsed.real_signals[1].values, vec![2.0, 2.5]);
        assert_eq!(parsed.real_signals[2].name, "V(out)[1][0]");
        assert_eq!(parsed.real_signals[2].values, vec![3.0, 3.5]);
        assert_eq!(parsed.real_signals[3].name, "V(out)[1][1]");
        assert_eq!(parsed.real_signals[3].values, vec![4.0, 4.5]);
    }

    #[test]
    fn test_parse_non_windowed_nested_array_real_bare_descriptor_psf_binary() {
        let bytes = build_non_windowed_nested_array_real_bare_descriptor_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 4);
        assert_eq!(parsed.real_signals[0].name, "V(out)[0][0]");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(out)[0][1]");
        assert_eq!(parsed.real_signals[1].values, vec![2.0, 2.5]);
        assert_eq!(parsed.real_signals[2].name, "V(out)[1][0]");
        assert_eq!(parsed.real_signals[2].values, vec![3.0, 3.5]);
        assert_eq!(parsed.real_signals[3].name, "V(out)[1][1]");
        assert_eq!(parsed.real_signals[3].values, vec![4.0, 4.5]);
    }

    #[test]
    fn test_parse_windowed_real_psf_binary() {
        let bytes = build_windowed_real_psf();
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
    fn test_parse_windowed_real_array_psf_binary() {
        let bytes = build_windowed_array_real_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 2);
        assert_eq!(parsed.real_signals[0].name, "V(arr)[0]");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(arr)[1]");
        assert_eq!(parsed.real_signals[1].values, vec![2.0, 2.5]);
    }

    #[test]
    fn test_parse_windowed_real_array_psf_binary_with_unaligned_payload_start() {
        let bytes = build_windowed_array_real_unaligned_payload_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 2);
        assert_eq!(parsed.real_signals[0].name, "V(arr)[0]");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(arr)[1]");
        assert_eq!(parsed.real_signals[1].values, vec![2.0, 2.5]);
    }

    #[test]
    fn test_parse_windowed_complex_array_psf_binary() {
        let bytes = build_windowed_array_complex_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.real_signals.is_empty());
        assert_eq!(parsed.complex_signals.len(), 2);
        assert_eq!(parsed.complex_signals[0].name, "I(arr)[0]");
        assert_eq!(
            parsed.complex_signals[0].values,
            vec![(1.0, 0.25), (1.5, 0.125)]
        );
        assert_eq!(parsed.complex_signals[1].name, "I(arr)[1]");
        assert_eq!(
            parsed.complex_signals[1].values,
            vec![(2.0, -0.5), (2.5, -0.75)]
        );
    }

    #[test]
    fn test_parse_windowed_struct_with_array_psf_binary() {
        let bytes = build_windowed_struct_with_array_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 3);
        assert_eq!(parsed.real_signals[0].name, "V(out).gain");
        assert_eq!(parsed.real_signals[0].values, vec![10.0, 11.0]);
        assert_eq!(parsed.real_signals[1].name, "V(out).taps[0]");
        assert_eq!(parsed.real_signals[1].values, vec![0.1, 0.15]);
        assert_eq!(parsed.real_signals[2].name, "V(out).taps[1]");
        assert_eq!(parsed.real_signals[2].values, vec![0.2, 0.25]);
    }

    #[test]
    fn test_parse_windowed_variable_length_array_pads_missing_values() {
        let bytes = build_windowed_variable_length_array_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 3);
        assert_eq!(parsed.real_signals[0].name, "V(arr)[0]");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(arr)[1]");
        assert!(parsed.real_signals[1].values[0].is_nan());
        assert_eq!(parsed.real_signals[1].values[1], 2.5);
        assert_eq!(parsed.real_signals[2].name, "V(arr)[2]");
        assert!(parsed.real_signals[2].values[0].is_nan());
        assert_eq!(parsed.real_signals[2].values[1], 3.5);
    }

    #[test]
    fn test_parse_windowed_array_of_struct_psf_binary() {
        let bytes = build_windowed_array_of_struct_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert_eq!(parsed.real_signals.len(), 2);
        assert_eq!(parsed.real_signals[0].name, "V(out)[0].dc");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(out)[1].dc");
        assert_eq!(parsed.real_signals[1].values, vec![1.1, 1.6]);
        assert_eq!(parsed.complex_signals.len(), 2);
        assert_eq!(parsed.complex_signals[0].name, "V(out)[0].ac");
        assert_eq!(
            parsed.complex_signals[0].values,
            vec![(2.0, 0.5), (2.5, -0.2)]
        );
        assert_eq!(parsed.complex_signals[1].name, "V(out)[1].ac");
        assert_eq!(
            parsed.complex_signals[1].values,
            vec![(2.1, 0.6), (2.6, -0.3)]
        );
    }

    #[test]
    fn test_parse_windowed_array_of_struct_bare_descriptor_psf_binary() {
        let bytes = build_windowed_array_of_struct_bare_descriptor_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert_eq!(parsed.real_signals.len(), 2);
        assert_eq!(parsed.real_signals[0].name, "V(out)[0].dc");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(out)[1].dc");
        assert_eq!(parsed.real_signals[1].values, vec![1.1, 1.6]);
        assert_eq!(parsed.complex_signals.len(), 2);
        assert_eq!(parsed.complex_signals[0].name, "V(out)[0].ac");
        assert_eq!(
            parsed.complex_signals[0].values,
            vec![(2.0, 0.5), (2.5, -0.2)]
        );
        assert_eq!(parsed.complex_signals[1].name, "V(out)[1].ac");
        assert_eq!(
            parsed.complex_signals[1].values,
            vec![(2.1, 0.6), (2.6, -0.3)]
        );
    }

    #[test]
    fn test_parse_windowed_nested_array_real_psf_binary() {
        let bytes = build_windowed_nested_array_real_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 4);
        assert_eq!(parsed.real_signals[0].name, "V(out)[0][0]");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(out)[0][1]");
        assert_eq!(parsed.real_signals[1].values, vec![2.0, 2.5]);
        assert_eq!(parsed.real_signals[2].name, "V(out)[1][0]");
        assert_eq!(parsed.real_signals[2].values, vec![3.0, 3.5]);
        assert_eq!(parsed.real_signals[3].name, "V(out)[1][1]");
        assert_eq!(parsed.real_signals[3].values, vec![4.0, 4.5]);
    }

    #[test]
    fn test_parse_windowed_nested_array_real_bare_descriptor_psf_binary() {
        let bytes = build_windowed_nested_array_real_bare_descriptor_psf();
        let parsed = parse_cadence_psf_binary(&bytes).expect("parse should succeed");

        assert_eq!(parsed.sweeps.len(), 1);
        assert_eq!(parsed.sweeps[0].values, vec![0.0, 1.0]);
        assert!(parsed.complex_signals.is_empty());
        assert_eq!(parsed.real_signals.len(), 4);
        assert_eq!(parsed.real_signals[0].name, "V(out)[0][0]");
        assert_eq!(parsed.real_signals[0].values, vec![1.0, 1.5]);
        assert_eq!(parsed.real_signals[1].name, "V(out)[0][1]");
        assert_eq!(parsed.real_signals[1].values, vec![2.0, 2.5]);
        assert_eq!(parsed.real_signals[2].name, "V(out)[1][0]");
        assert_eq!(parsed.real_signals[2].values, vec![3.0, 3.5]);
        assert_eq!(parsed.real_signals[3].name, "V(out)[1][1]");
        assert_eq!(parsed.real_signals[3].values, vec![4.0, 4.5]);
    }

    #[test]
    fn test_parse_windowed_rejects_trace_count_mismatch() {
        let mut bytes = build_windowed_real_psf();
        patch_header_int(&mut bytes, "PSF traces", 2);
        let err = parse_cadence_psf_binary(&bytes).expect_err("mismatched trace count must fail");
        assert!(err.to_string().contains("trace count mismatch"));
    }

    #[test]
    fn test_parse_windowed_rejects_zero_sample_block() {
        let mut bytes = build_windowed_real_psf();
        patch_first_window_block_count(&mut bytes, 0);
        let err = parse_cadence_psf_binary(&bytes).expect_err("zero-sample block must fail");
        assert!(err.to_string().contains("zero samples"));
    }

    #[test]
    fn test_parse_windowed_rejects_block_overshoot() {
        let mut bytes = build_windowed_real_psf();
        patch_first_window_block_count(&mut bytes, 3);
        let err = parse_cadence_psf_binary(&bytes)
            .expect_err("block larger than sweep-points declaration must fail");
        assert!(err.to_string().contains("exceeds declared sweep points"));
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

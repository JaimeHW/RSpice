#![allow(clippy::too_many_arguments)]

use super::{
    parse_string, push_named_channel_cached, read_f64, read_i32, read_u32, read_u8_padded,
    resolve_array_element_type, skip_opaque_scalar, ArrayElementType, CadencePsfError, DataType,
    NumericSample, SignalChannel, SignalRef, TypeDecl, TypeKind, TypeMetaCache,
};
use std::collections::HashMap;

pub(super) fn decode_windowed_dynamic_signal_samples(
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
        super::pad_untouched_channels(values, signal.id, target_len, &touched_channels)?;
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

pub(super) fn read_type_value_with_numeric_visit<F>(
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

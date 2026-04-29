use std::collections::HashMap;

use super::types::{
    ArrayElementType, CadencePsfError, ChannelKind, ChannelSpec, DataType, NumericSample,
    SignalChannel, SignalValues, TypeDecl, TypeKind,
};

pub(super) fn qualify_signal_name(base: &str, suffix: &str) -> String {
    if suffix.is_empty() {
        base.to_string()
    } else if suffix.starts_with('[') {
        format!("{}{}", base, suffix)
    } else {
        format!("{}.{}", base, suffix)
    }
}

pub(super) fn init_channels(specs: &[ChannelSpec], capacity_hint: usize) -> Vec<SignalChannel> {
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

pub(super) fn resolve_array_element_type(
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

pub(super) fn resolve_implicit_composite_array_element(
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
    Err(CadencePsfError::new(
        (if want_array {
            format!(
                "array element descriptor ARRAY is ambiguous across type ids {:?}",
                candidates
            )
        } else {
            format!(
                "array element descriptor STRUCT is ambiguous across type ids {:?}",
                candidates
            )
        })
        .to_string(),
    ))
}

pub(super) fn collect_channel_specs_for_type(
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

pub(super) fn collect_channel_specs_for_array_element(
    element_type_raw: u32,
    types: &HashMap<u32, TypeDecl>,
    prefix: &str,
    specs: &mut Vec<ChannelSpec>,
    parent_type_id: Option<u32>,
) -> Result<(), CadencePsfError> {
    match resolve_array_element_type(element_type_raw, types, parent_type_id)? {
        ArrayElementType::Primitive(dtype) => match dtype {
            DataType::Int8 | DataType::Int32 | DataType::Real => specs.push(ChannelSpec {
                suffix: prefix.to_string(),
                kind: ChannelKind::Real,
            }),
            DataType::Complex => specs.push(ChannelSpec {
                suffix: prefix.to_string(),
                kind: ChannelKind::Complex,
            }),
            DataType::String => {}
            DataType::Array => {
                return Err(CadencePsfError::new(
                    "array element descriptor resolved to ARRAY without a concrete type reference",
                ));
            }
            DataType::Struct => {
                return Err(CadencePsfError::new(
                    "array element descriptor resolved to STRUCT without a concrete type reference",
                ));
            }
            DataType::Other(_) => {}
        },
        ArrayElementType::TypeRef(type_id) => {
            collect_channel_specs_for_type(type_id, types, prefix, specs)?;
        }
    }
    Ok(())
}

pub(super) fn channel_sample_width(channels: &[SignalChannel]) -> Result<usize, CadencePsfError> {
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

pub(super) fn ensure_channel_length(values: &mut SignalValues, len: usize) {
    match values {
        SignalValues::Real(v) => v.resize(len, f64::NAN),
        SignalValues::Complex(v) => v.resize(len, (f64::NAN, f64::NAN)),
    }
}

pub(super) fn channel_kind(values: &SignalValues) -> ChannelKind {
    match values {
        SignalValues::Real(_) => ChannelKind::Real,
        SignalValues::Complex(_) => ChannelKind::Complex,
    }
}

pub(super) fn build_channel_index_cache(
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

pub(super) fn push_named_channel_cached(
    values: &mut HashMap<u32, Vec<SignalChannel>>,
    cache: &mut HashMap<u32, HashMap<String, usize>>,
    signal_id: u32,
    suffix: &str,
    sample: NumericSample,
    point_idx: usize,
) -> Result<usize, CadencePsfError> {
    if let Some(per_signal) = cache.get(&signal_id)
        && let Some(idx) = per_signal.get(suffix)
    {
        let idx = *idx;
        let channels = values.get_mut(&signal_id).ok_or_else(|| {
            CadencePsfError::new(format!("missing value vector for signal {}", signal_id))
        })?;
        push_scalar_slice(channels.as_mut_slice(), idx, sample)?;
        return Ok(idx);
    }

    let idx = push_named_channel(values, signal_id, suffix, sample, point_idx)?;
    cache
        .entry(signal_id)
        .or_default()
        .insert(suffix.to_string(), idx);
    Ok(idx)
}

pub(super) fn push_named_channel(
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

pub(super) fn pad_untouched_channels(
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

pub(super) fn push_scalar_channel(
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

pub(super) fn push_scalar_slice(
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

use std::collections::HashMap;

use super::binary_io::{peek_u32, read_f64, read_u32};
use super::cadence_psf_type_meta::TypeMetaCache;
use super::channels::{
    build_channel_index_cache, channel_sample_width, collect_channel_specs_for_type, init_channels,
    pad_untouched_channels, push_named_channel_cached, push_scalar_channel,
};
use super::sections::{
    flatten_traces, header_usize, parse_zero_pad, reject_non_zero_trailing_bytes,
};
use super::toc::TocEntry;
use super::types::{
    CadencePsfError, CadencePsfValue, NumericSample, SignalChannel, SignalRef, SignalValues,
    TraceDef, TypeDecl,
};
use super::value_decode::{
    decode_windowed_dynamic_signal_samples, read_type_value_with_numeric_visit,
};

pub(super) fn parse_values(
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

pub(super) fn parse_non_windowed_values<'a>(
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

pub(super) fn parse_windowed_values<'a>(
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

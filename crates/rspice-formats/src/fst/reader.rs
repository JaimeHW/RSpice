//! Streaming FST hierarchy and event decoding.

use std::collections::{BTreeMap, HashMap};
use std::io::{BufReader, Cursor};

use super::{
    DecodedFst, FstEvent, FstLimits, FstRawValue, FstSignal, adapter_error, preflight_fst,
};

/// Decode a preflighted FST stream, delivering each bounded event before the
/// reader advances to the next one.
pub fn decode_fst(
    bytes: &[u8],
    limits: FstLimits,
    format: &str,
    mut on_event: impl FnMut(FstEvent<'_>),
) -> Result<DecodedFst, String> {
    let geometry = preflight_fst(bytes, limits, format)?;
    let cursor = Cursor::new(bytes);
    let mut reader = fst_reader::FstReader::open(BufReader::new(cursor))
        .map_err(|error| adapter_error(format, format_args!("invalid FST container: {error}")))?;
    let header = reader.get_header();
    if header.var_count as usize > limits.max_columns.saturating_sub(1) {
        return Err(adapter_error(format, "FST signal-count limit exceeded"));
    }
    if !(-30..=30).contains(&header.timescale_exponent) {
        return Err(adapter_error(
            format,
            "FST timescale exponent is outside the supported finite range",
        ));
    }
    let mut scopes = Vec::new();
    let mut scope_identity_bytes = 0_usize;
    let mut by_handle: BTreeMap<usize, Vec<FstSignal>> = BTreeMap::new();
    let mut hierarchy_entries = 0_usize;
    let mut hierarchy_signals = 0_usize;
    let mut hierarchy_error = None;
    let max_signal_name_bytes = limits.max_signal_name_bytes;
    let maximum_handle = usize::try_from(header.max_handle)
        .map_err(|_| adapter_error(format, "FST header signal count does not fit this target"))?;
    reader
        .read_hierarchy(|entry| {
            if hierarchy_error.is_some() {
                return;
            }
            hierarchy_entries += 1;
            if hierarchy_entries > limits.max_values {
                hierarchy_error = Some("FST hierarchy-entry limit exceeded".to_owned());
                return;
            }
            match entry {
                fst_reader::FstHierarchyEntry::Scope { name, .. } => {
                    let separator = usize::from(!scopes.is_empty());
                    let Some(next_identity_bytes) = scope_identity_bytes
                        .checked_add(separator)
                        .and_then(|length| length.checked_add(name.len()))
                    else {
                        hierarchy_error = Some("FST hierarchy identity length overflow".to_owned());
                        return;
                    };
                    if name.is_empty() || name.len() > max_signal_name_bytes {
                        hierarchy_error = Some(format!(
                            "FST scope identity exceeds {max_signal_name_bytes} bytes or is empty"
                        ));
                    } else if next_identity_bytes > max_signal_name_bytes {
                        hierarchy_error = Some(format!(
                            "FST scope path exceeds {max_signal_name_bytes} bytes"
                        ));
                    } else if scopes.len() >= limits.max_columns {
                        hierarchy_error = Some("FST hierarchy-depth limit exceeded".to_owned());
                    } else {
                        scope_identity_bytes = next_identity_bytes;
                        scopes.push(name);
                    }
                }
                fst_reader::FstHierarchyEntry::UpScope => {
                    if let Some(name) = scopes.pop() {
                        scope_identity_bytes = if scopes.is_empty() {
                            0
                        } else {
                            scope_identity_bytes.saturating_sub(name.len() + 1)
                        };
                    } else {
                        hierarchy_error =
                            Some("FST hierarchy closes a scope that was not open".to_owned());
                    }
                }
                fst_reader::FstHierarchyEntry::Var {
                    name,
                    length,
                    handle,
                    ..
                } => {
                    hierarchy_signals += 1;
                    if hierarchy_signals > limits.max_columns.saturating_sub(1) {
                        hierarchy_error =
                            Some("FST hierarchy signal/alias limit exceeded".to_owned());
                        return;
                    }
                    let handle = handle.get_index();
                    if handle >= maximum_handle {
                        hierarchy_error = Some(
                            "FST hierarchy references an out-of-range signal handle".to_owned(),
                        );
                        return;
                    }
                    if geometry.widths[handle] != length as usize {
                        hierarchy_error = Some(format!(
                            "FST hierarchy width {length} for '{name}' disagrees with geometry width {}",
                            geometry.widths[handle]
                        ));
                        return;
                    }
                    if length == 0 || length > 53 {
                        hierarchy_error = Some(format!(
                            "FST hierarchy signal '{name}' has unsupported width {length}"
                        ));
                        return;
                    }
                    if name.is_empty() || name.len() > max_signal_name_bytes {
                        hierarchy_error = Some(format!(
                            "FST signal identity exceeds {max_signal_name_bytes} bytes or is empty"
                        ));
                        return;
                    }
                    let mut full_name = scopes.join(".");
                    if !full_name.is_empty() {
                        full_name.push('.');
                    }
                    full_name.push_str(&name);
                    if full_name.len() > max_signal_name_bytes {
                        hierarchy_error = Some(format!(
                            "FST hierarchy signal identity exceeds {max_signal_name_bytes} bytes"
                        ));
                        return;
                    }
                    by_handle.entry(handle).or_default().push(FstSignal {
                        name: full_name,
                        width: Some(length as usize),
                    });
                }
                _ => {}
            }
        })
        .map_err(|error| {
            adapter_error(
                format,
                format_args!("could not read FST hierarchy: {error}"),
            )
        })?;
    if let Some(error) = hierarchy_error {
        return Err(adapter_error(format, error));
    }
    if !scopes.is_empty() {
        return Err(adapter_error(format, "FST hierarchy has unclosed scopes"));
    }
    if by_handle.len() != maximum_handle {
        return Err(adapter_error(
            format,
            "FST hierarchy unique-signal count disagrees with geometry",
        ));
    }
    let total_signals = by_handle.values().map(Vec::len).sum::<usize>();
    if total_signals == 0 || total_signals > limits.max_columns.saturating_sub(1) {
        return Err(adapter_error(
            format,
            "FST contains no signals or too many aliases",
        ));
    }
    let handles = by_handle
        .keys()
        .map(|index| fst_reader::FstSignalHandle::from_index(*index))
        .collect::<Vec<_>>();
    let handle_order = by_handle.keys().copied().collect::<Vec<_>>();
    let handle_to_signal = handle_order
        .iter()
        .enumerate()
        .map(|(signal, handle)| (*handle, signal))
        .collect::<HashMap<_, _>>();
    let canonical_signals = handle_order
        .iter()
        .map(|handle| FstSignal {
            name: by_handle[handle][0].name.clone(),
            width: by_handle[handle][0].width,
        })
        .collect::<Vec<_>>();
    let mut event_count = 0_usize;
    let callback_result = reader.read_signals(
        &fst_reader::FstFilter::filter_signals(handles),
        |tick, handle, value| {
            if event_count >= limits.max_values {
                return Err("FST event-count limit exceeded".to_owned());
            }
            let signal = *handle_to_signal
                .get(&handle.get_index())
                .ok_or_else(|| "FST returned an undeclared signal handle".to_owned())?;
            let (sample, raw) = match value {
                fst_reader::FstSignalValue::String(bits) => {
                    (logic_bits_to_f64(bits, format)?, FstRawValue::Logic(bits))
                }
                fst_reader::FstSignalValue::Real(value) if value.is_finite() => {
                    (value, FstRawValue::Real(value))
                }
                fst_reader::FstSignalValue::Real(_) => {
                    return Err("FST contains a non-finite real value".to_owned());
                }
            };
            on_event(FstEvent {
                tick,
                signal,
                sample,
                raw,
            });
            event_count += 1;
            Ok(())
        },
    );
    callback_result.map_err(|error| {
        adapter_error(format, format_args!("could not read FST events: {error:?}"))
    })?;
    let mut aliases = Vec::new();
    for (canonical_index, handle) in handle_order.iter().enumerate() {
        for alias in by_handle[handle].iter().skip(1) {
            aliases.push((canonical_index, alias.name.clone()));
        }
    }
    Ok(DecodedFst {
        timescale_seconds: 10_f64.powi(header.timescale_exponent as i32),
        signals: canonical_signals,
        aliases,
    })
}

fn logic_bits_to_f64(bits: &[u8], format: &str) -> Result<f64, String> {
    if bits.is_empty() || bits.len() > 53 {
        return Err(adapter_error(
            format,
            "digital vector is empty or wider than 53 exact bits",
        ));
    }
    let mut value = 0_u64;
    for bit in bits {
        value = value
            .checked_mul(2)
            .ok_or_else(|| adapter_error(format, "digital vector overflow"))?;
        match bit {
            b'0' => {}
            b'1' => value += 1,
            _ => {
                return Err(adapter_error(
                    format,
                    "digital vector contains X/Z/U/W/- state that cannot be losslessly mapped to an analog trace",
                ));
            }
        }
    }
    Ok(value as f64)
}

pub fn looks_like_fst(bytes: &[u8]) -> bool {
    let mut cursor = Cursor::new(bytes);
    fst_reader::is_fst_file(&mut cursor)
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::{FstLimits, FstRawValue, decode_fst, looks_like_fst};
    use fst_writer::{
        FstFileType, FstInfo, FstScopeType, FstSignalType, FstVarDirection, FstVarType,
    };

    #[test]
    fn reader_streams_real_generated_events_without_a_result_owner() {
        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "rspice-formats-fst-{}-{nonce}.fst",
            std::process::id()
        ));
        let info = FstInfo {
            start_time: 0,
            timescale_exponent: -9,
            version: "RSpice formats test".to_owned(),
            date: "2026-09-24".to_owned(),
            file_type: FstFileType::Verilog,
        };
        let mut header = fst_writer::open_fst(&path, &info).expect("open FST fixture");
        header
            .scope("top", "top", FstScopeType::Module)
            .expect("FST scope");
        let clock = header
            .var(
                "clock",
                FstSignalType::bit_vec(1),
                FstVarType::Wire,
                FstVarDirection::Implicit,
                None,
            )
            .expect("FST signal");
        header.up_scope().expect("FST upscope");
        let mut body = header.finish().expect("FST header");
        for (tick, bit) in [(0, b"0".as_slice()), (1, b"1".as_slice())] {
            body.time_change(tick).expect("FST time change");
            body.signal_change(clock, bit).expect("FST signal change");
        }
        body.finish().expect("FST finish");
        let bytes = std::fs::read(&path).expect("read FST fixture");
        std::fs::remove_file(&path).expect("remove FST fixture");
        assert!(looks_like_fst(&bytes));

        let mut events = Vec::new();
        let decoded = decode_fst(
            &bytes,
            FstLimits {
                max_bytes: 1024 * 1024,
                max_columns: 16,
                max_rows: 16,
                max_values: 64,
                max_signal_name_bytes: 1024,
            },
            "fst",
            |event| {
                let FstRawValue::Logic(bits) = event.raw else {
                    panic!("logic signal must preserve its raw bits");
                };
                events.push((event.tick, event.signal, event.sample, bits.to_vec()));
            },
        )
        .expect("decode FST");
        assert_eq!(decoded.signals.len(), 1);
        assert_eq!(decoded.signals[0].name, "top.clock");
        assert_eq!(decoded.signals[0].width, Some(1));
        assert!(decoded.aliases.is_empty());
        assert_eq!(decoded.timescale_seconds, 1e-9);
        assert_eq!(
            events,
            [(0, 0, 0.0, b"0".to_vec()), (1, 0, 1.0, b"1".to_vec())]
        );
    }
}

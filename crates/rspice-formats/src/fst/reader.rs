//! Streaming FST hierarchy and event decoding.

use std::collections::{BTreeMap, HashMap};
use std::io::{BufReader, Cursor};

use super::{
    DecodedFst, FstEvent, FstLimits, FstRawValue, FstReadError, FstReadFailure, FstSignal,
    adapter_error, preflight_fst, read_error,
};

/// Decode a preflighted FST stream, delivering each bounded event before the
/// reader advances to the next one.
pub fn decode_fst(
    bytes: &[u8],
    limits: FstLimits,
    format: &str,
    mut on_event: impl FnMut(FstEvent<'_>),
) -> Result<DecodedFst, FstReadError> {
    let geometry = preflight_fst(bytes, limits, format)?;
    let cursor = Cursor::new(bytes);
    let mut reader = fst_reader::FstReader::open(BufReader::new(cursor))
        .map_err(|error| read_error(format, FstReadFailure::Container(error)))?;
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
                    tpe,
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
                    // A real's declared length is a writer's choice (8 bytes, or 64 bits); its
                    // samples are the double the geometry marks it with.
                    let real = tpe.is_real();
                    if real && geometry.widths[handle] != 8 {
                        hierarchy_error = Some(format!(
                            "FST real variable '{name}' is not stored as a real number"
                        ));
                        return;
                    }
                    if !real && geometry.widths[handle] != length as usize {
                        hierarchy_error = Some(format!(
                            "FST hierarchy width {length} for '{name}' disagrees with geometry width {}",
                            geometry.widths[handle]
                        ));
                        return;
                    }
                    if !real && (length == 0 || length > 53) {
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
                        width: (!real).then_some(length as usize),
                    });
                }
                _ => {}
            }
        })
        .map_err(|error| read_error(format, FstReadFailure::Hierarchy(error)))?;
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
                    (logic_bits_to_f64(bits)?, FstRawValue::Logic(bits))
                }
                fst_reader::FstSignalValue::Real(value) if value.is_finite() => {
                    (Some(value), FstRawValue::Real(value))
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
    callback_result.map_err(|error| read_error(format, FstReadFailure::Events(error)))?;
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

/// The unsigned integer a logic value's bits spell, a weak drive (`l`, `h`) at its level; `None`
/// when a bit is unknown or high impedance (`x`, `z`, `u`, `w`, `-`) and they spell none.
fn logic_bits_to_f64(bits: &[u8]) -> Result<Option<f64>, String> {
    if bits.is_empty() || bits.len() > 53 {
        return Err(format!(
            "a logic value holds {} bits; an imported vector holds 1 to 53",
            bits.len()
        ));
    }
    let mut value = 0_u64;
    for bit in bits {
        value <<= 1;
        match bit {
            b'0' | b'l' | b'L' => {}
            b'1' | b'h' | b'H' => value += 1,
            _ => return Ok(None),
        }
    }
    Ok(Some(value as f64))
}

/// Whether `bytes` open as an FST file does: its header block (type 0, 329 bytes long, with
/// the real number *e* the format writes to state its byte order), or a whole file gzip-wrapped
/// (type 254 and the gzip magic). A first byte alone matches too much: a MAT v4 file and
/// UTF-16 text both start with one an FST block can have.
pub fn looks_like_fst(bytes: &[u8]) -> bool {
    const HEADER_BLOCK: u8 = 0;
    const WRAPPED_FILE: u8 = 254;
    const HEADER_SECTION_BYTES: u64 = 329;
    let section = bytes
        .get(1..9)
        .and_then(|section| <[u8; 8]>::try_from(section).ok())
        .map(u64::from_be_bytes);
    match bytes.first() {
        Some(&HEADER_BLOCK) if section == Some(HEADER_SECTION_BYTES) => bytes
            .get(25..33)
            .and_then(|marker| <[u8; 8]>::try_from(marker).ok())
            .is_some_and(|marker| {
                [f64::from_le_bytes(marker), f64::from_be_bytes(marker)]
                    .contains(&std::f64::consts::E)
            }),
        Some(&WRAPPED_FILE) => bytes.get(17..19) == Some(&[0x1f, 0x8b]),
        _ => false,
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::{FstLimits, FstRawValue, decode_fst, looks_like_fst};
    use fst_writer::{
        FstFileType, FstInfo, FstScopeType, FstSignalType, FstVarDirection, FstVarType,
    };

    fn limits() -> FstLimits {
        FstLimits {
            max_bytes: 1024 * 1024,
            max_columns: 16,
            max_rows: 16,
            max_values: 64,
            max_signal_name_bytes: 1024,
        }
    }

    fn fst_bytes(last_bit: &[u8]) -> Vec<u8> {
        // The clock alone is not unique: macOS reports whole microseconds, so
        // two tests on parallel threads could share a file and delete it
        // under each other.
        static FIXTURES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let nonce = FIXTURES.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
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
        for (tick, bit) in [(0, b"0".as_slice()), (1, last_bit)] {
            body.time_change(tick).expect("FST time change");
            body.signal_change(clock, bit).expect("FST signal change");
        }
        body.finish().expect("FST finish");
        let bytes = std::fs::read(&path).expect("read FST fixture");
        std::fs::remove_file(&path).expect("remove FST fixture");
        bytes
    }

    #[test]
    fn reader_streams_real_generated_events_without_a_result_owner() {
        let bytes = fst_bytes(b"1");
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
            [
                (0, 0, Some(0.0), b"0".to_vec()),
                (1, 0, Some(1.0), b"1".to_vec())
            ]
        );
    }

    #[test]
    fn an_unknown_or_floating_bit_streams_with_no_sample_and_keeps_its_code() {
        for code in [b"x".as_slice(), b"z", b"u", b"-"] {
            let mut events = Vec::new();
            decode_fst(&fst_bytes(code), limits(), "fst", |event| {
                let FstRawValue::Logic(bits) = event.raw else {
                    panic!("logic signal must preserve its raw bits");
                };
                events.push((event.sample, bits.to_vec()));
            })
            .expect("an unknown bit imports");
            assert_eq!(events, [(Some(0.0), b"0".to_vec()), (None, code.to_vec())]);
        }
        let mut weak = Vec::new();
        decode_fst(&fst_bytes(b"h"), limits(), "fst", |event| {
            weak.push(event.sample);
        })
        .unwrap();
        assert_eq!(weak, [Some(0.0), Some(1.0)]);
    }

    #[test]
    fn a_real_variable_streams_its_doubles_under_no_bit_width() {
        let path = std::env::temp_dir().join(format!(
            "rspice-formats-fst-real-{}.fst",
            std::process::id()
        ));
        let info = FstInfo {
            start_time: 0,
            timescale_exponent: -12,
            version: "RSpice formats test".to_owned(),
            date: "2026-11-24".to_owned(),
            file_type: FstFileType::Verilog,
        };
        let mut header = fst_writer::open_fst(&path, &info).expect("open FST fixture");
        header.scope("top", "top", FstScopeType::Module).unwrap();
        let level = header
            .var(
                "level",
                FstSignalType::real(),
                FstVarType::Real,
                FstVarDirection::Implicit,
                None,
            )
            .unwrap();
        header.up_scope().unwrap();
        let mut body = header.finish().unwrap();
        for (tick, value) in [(0, 0.25_f64), (7, -1.5e-3)] {
            body.time_change(tick).unwrap();
            body.signal_change(level, &value.to_le_bytes()).unwrap();
        }
        body.finish().unwrap();
        let bytes = std::fs::read(&path).unwrap();
        std::fs::remove_file(&path).unwrap();

        let mut events = Vec::new();
        let decoded = decode_fst(&bytes, limits(), "fst", |event| {
            let FstRawValue::Real(value) = event.raw else {
                panic!("a real variable streams doubles");
            };
            events.push((event.tick, event.sample, value));
        })
        .expect("a real variable imports");
        assert_eq!(decoded.signals[0].name, "top.level");
        assert_eq!(decoded.signals[0].width, None);
        assert_eq!(events, [(0, Some(0.25), 0.25), (7, Some(-1.5e-3), -1.5e-3)]);
    }

    #[test]
    fn an_event_refusal_keeps_its_source_and_says_it_in_plain_words() {
        use std::error::Error as _;
        // Three hierarchy entries and two events: a budget of four admits the hierarchy and
        // refuses nothing, so the refusal is built as the event callback returns it.
        let error = super::read_error(
            "fst",
            super::FstReadFailure::Events(fst_reader::ReadSignalsError::CallbackError(
                "FST contains a non-finite real value".to_owned(),
            )),
        );
        assert_eq!(
            error.to_string(),
            "fst import: FST contains a non-finite real value"
        );
        assert!(
            error
                .reason
                .source()
                .unwrap()
                .is::<fst_reader::ReadSignalsError<String>>()
        );
    }
}

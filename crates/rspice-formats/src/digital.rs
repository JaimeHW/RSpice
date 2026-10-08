//! Bounded VCD/FST decoding, retained event evidence and sampled projection.

use crate::numeric::MAX_EXACT_F64_INTEGER;
use rspice_results::analysis_payload::AnalysisResultPayload;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::events::imported_event_payload;
use rspice_results::result_import::ResultImportFormat;
use rspice_results::result_import::waveforms::{
    ImportedSignal, ImportedWaveforms, WaveformImportLimits, assemble_imported_waveforms,
    validate_name,
};
use rspice_results::waveform::RetainedWaveform;
use std::collections::{BTreeSet, HashSet};
use std::io::Cursor;
use std::sync::Arc;

/// Resource policy supplied by the importing host, including decoded sample limits.
#[derive(Debug, Clone, Copy)]
pub struct DigitalImportLimits {
    pub max_bytes: u64,
    pub samples: WaveformImportLimits,
}

/// Decoded samples and exact evidence before the host grants import authority.
#[derive(Debug)]
pub struct DecodedDigital {
    pub data: ImportedWaveforms,
    pub notes: Vec<String>,
    pub event_payload: Option<AnalysisResultPayload>,
}

#[derive(Debug)]
pub enum DigitalReadError {
    InvalidData {
        format: ResultImportFormat,
        detail: String,
    },
    Vcd {
        format: ResultImportFormat,
        source: rspice_core::io::VcdError,
    },
    EventProjection {
        format: ResultImportFormat,
        source: rspice_core::execution::EventProjectionError,
    },
    EventTimeCount {
        format: ResultImportFormat,
        count: usize,
        min: usize,
        max: usize,
    },
    InexactTimestamp {
        format: ResultImportFormat,
        tick: u64,
    },
    #[cfg(feature = "fst")]
    Fst(crate::fst::FstReadError),
    ResultData(String),
}

impl std::fmt::Display for DigitalReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidData { format, detail } => {
                write!(f, "{} import: {detail}", format.canonical_id())
            }
            Self::Vcd { format, source } => write!(f, "{} import: {source}", format.canonical_id()),
            Self::EventProjection { format, source } => {
                write!(f, "{} import: {source}", format.canonical_id())
            }
            Self::EventTimeCount {
                format,
                count,
                min,
                max,
            } => write!(
                f,
                "{} import: digital trace has {count} distinct event times; expected {min}..={max}",
                format.canonical_id()
            ),
            Self::InexactTimestamp { format, tick } => write!(
                f,
                "{} import: digital timestamp tick {tick} cannot be represented exactly as f64",
                format.canonical_id()
            ),
            #[cfg(feature = "fst")]
            Self::Fst(source) => source.fmt(f),
            Self::ResultData(detail) => f.write_str(detail),
        }
    }
}

impl std::error::Error for DigitalReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Vcd { source, .. } => Some(source),
            Self::EventProjection { source, .. } => Some(source),
            #[cfg(feature = "fst")]
            Self::Fst(source) => Some(source),
            _ => None,
        }
    }
}

fn adapter_error(format: ResultImportFormat, detail: impl std::fmt::Display) -> DigitalReadError {
    DigitalReadError::InvalidData {
        format,
        detail: detail.to_string(),
    }
}

#[derive(Debug)]
struct DigitalSignal {
    name: String,
    /// Bits this signal's values are mapped from onto one exact `f64` sample,
    /// or `None` when the source already carries the sample as a real number
    /// and no integer mapping is involved.
    width: Option<usize>,
}

#[derive(Debug)]
struct DigitalEvent {
    tick: u64,
    signal: usize,
    value: f64,
}

/// The sample an unknown or high-impedance logic value imports as.
///
/// The same level a native run's own tabular projection gives a digital value
/// that is neither 0 nor 1, so an imported waveform and a solved one draw an
/// unresolved net the same way.
///
/// This is a property of the *analog grid* only. The exact event evidence
/// beside it keeps the four-state code the file recorded, so nothing about
/// what the source said is lost by this level existing.
const UNKNOWN_LOGIC_LEVEL: f64 = 0.5;

/// The widest vector whose unsigned integer value an `f64` sample holds
/// exactly.
///
/// A wider vector is not refused: it reaches the grid as one column per bit,
/// named the way the codec names a bus member, and reaches the retained
/// evidence as a declaration over those members like any other bus. The
/// widths a *bus* is bounded by are the engine's —
/// `rspice_core::engine::MAX_DIGITAL_BUS_WIDTH` and
/// `rspice_core::execution::MAX_BUS_EVENT_CELLS` — and the codec applies them.
const MAX_EXACT_VECTOR_BITS: u32 = 53;

/// Import a VCD file through the core codec.
///
/// The codec owns the grammar — four-state values, aliases, dump blocks,
/// timescales, vector variables and every refusal. This adapter owns two
/// mappings of what the codec returns:
///
/// - the **exact event evidence**, which is
///   [`rspice_core::execution::vcd_event_histories`] verbatim: four-state
///   codes at the ticks the writer recorded, with each vector variable as a
///   `Import` bus declaration over `name[k]` member traces. Nothing is
///   flattened and nothing is refused for width here.
/// - the **analog-grid projection** the table and waveform sheets read: one
///   union grid of event times, one `f64` per column per row. That grid has
///   no four-state value and no word wider than [`MAX_EXACT_VECTOR_BITS`], so
///   an `x` or `z` lands at [`UNKNOWN_LOGIC_LEVEL`] and a wider vector is
///   spread over one column per bit. Both are counted and stated in the
///   dataset's notes.
pub fn decode_vcd(
    bytes: &[u8],
    format: ResultImportFormat,
    policy: DigitalImportLimits,
) -> Result<DecodedDigital, DigitalReadError> {
    let mut limits = rspice_core::ResourceLimits::default();
    limits.max_external_data_bytes = usize::try_from(policy.max_bytes).unwrap_or(usize::MAX);
    limits.max_external_data_values = policy.samples.max_values;
    limits.max_result_values = policy.samples.max_values;
    let document = rspice_core::io::parse_vcd_reader_with_limits(Cursor::new(bytes), limits)
        .map_err(|source| DigitalReadError::Vcd { format, source })?;

    // The evidence first, and from the whole document: every refusal a bus
    // can earn — a width past the engine's ceiling, a declared range that
    // disagrees with the variable's width, two variables that reduce to one
    // name — is the codec's, in the codec's words.
    let histories = rspice_core::execution::vcd_event_histories(&document)
        .map_err(|source| DigitalReadError::EventProjection { format, source })?;
    let event_payload = imported_event_payload(&histories);

    let mut signals: Vec<DigitalSignal> = Vec::new();
    let mut aliases = Vec::new();
    let mut events = Vec::new();
    let mut unknown_changes = 0_usize;
    let mut expanded_vectors = 0_usize;
    // The codec pushes one declaration per vector variable in the order the
    // document declares them, which is the order this loop walks, so the
    // n-th wide vector here is the n-th wide declaration there. That is where
    // the member names come from: this adapter never spells one itself.
    let mut wide_buses = histories
        .digital_buses
        .iter()
        .filter(|bus| bus.members.len() > MAX_EXACT_VECTOR_BITS as usize);
    for signal in document.signals {
        let mut names = signal
            .variables
            .iter()
            .map(rspice_core::io::VcdVariable::scoped_name);
        let Some(name) = names.next() else {
            continue;
        };
        let wide = signal.kind == rspice_core::io::VcdSignalKind::Logic
            && signal.width > MAX_EXACT_VECTOR_BITS;
        let columns: Vec<(String, Option<usize>)> = if wide {
            let bus = wide_buses.next().ok_or_else(|| {
                adapter_error(
                    format,
                    format_args!("VCD vector '{name}' has no declaration to spread over its bits"),
                )
            })?;
            expanded_vectors += 1;
            // The codec names a member `base[k]` against the *unscoped* base,
            // because reading a dump back drops the scope path every variable
            // shares. A grid column is named the way every other column of
            // this grid is — scoped — so the bit-select the codec produced is
            // carried over onto the scoped base. Neither half is spelled
            // here: the base comes from the codec's own range grammar, and
            // the suffix from the member name the codec built.
            let (scoped_base, _) = rspice_core::execution::split_bus_notation(&name);
            bus.members
                .iter()
                .map(|member| {
                    let suffix = member.get(bus.name.len()..).unwrap_or_default();
                    (format!("{scoped_base}{suffix}"), Some(1))
                })
                .collect()
        } else {
            let width = match signal.kind {
                rspice_core::io::VcdSignalKind::Real => None,
                rspice_core::io::VcdSignalKind::Logic => Some(signal.width as usize),
            };
            vec![(name, width)]
        };
        if signals.len() + columns.len() > policy.samples.max_columns.saturating_sub(1) {
            return Err(adapter_error(format, "VCD signal-count limit exceeded"));
        }
        let first = signals.len();
        for (name, width) in columns {
            signals.push(DigitalSignal { name, width });
        }
        // An alias of a spread vector names the whole word, which no single
        // column of the grid is; it is dropped there and kept in the evidence.
        if !wide {
            aliases.extend(names.map(|alias| (first, alias)));
        }
        for change in signal.changes {
            if events.len() >= policy.samples.max_values {
                return Err(adapter_error(format, "VCD event-count limit exceeded"));
            }
            match change.value {
                rspice_core::io::VcdValue::Real(value) => events.push(DigitalEvent {
                    tick: change.tick,
                    signal: first,
                    value,
                }),
                rspice_core::io::VcdValue::Logic(bits) if wide => {
                    for (offset, bit) in bits.iter().enumerate() {
                        events.push(DigitalEvent {
                            tick: change.tick,
                            signal: first + offset,
                            value: match bit {
                                rspice_core::io::VcdBit::Zero => 0.0,
                                rspice_core::io::VcdBit::One => 1.0,
                                _ => {
                                    unknown_changes += 1;
                                    UNKNOWN_LOGIC_LEVEL
                                }
                            },
                        });
                    }
                }
                rspice_core::io::VcdValue::Logic(bits) => {
                    let value = match vcd_bits_to_f64(&bits) {
                        Some(value) => value,
                        None => {
                            unknown_changes += 1;
                            UNKNOWN_LOGIC_LEVEL
                        }
                    };
                    events.push(DigitalEvent {
                        tick: change.tick,
                        signal: first,
                        value,
                    });
                }
            }
        }
    }

    let mut parsed = digital_events_to_dataset(
        format,
        document.timescale.seconds(),
        signals,
        events,
        policy.samples,
    )?;
    append_digital_aliases(format, &mut parsed.data, aliases, policy.samples)?;
    if unknown_changes > 0 {
        parsed.notes.push(format!(
            "{unknown_changes} sampled values were unknown (x) or high impedance (z); each is \
             plotted at the {UNKNOWN_LOGIC_LEVEL} level a solved run's digital projection uses. \
             The retained event history keeps the four-state code the file recorded."
        ));
    }
    if expanded_vectors > 0 {
        parsed.notes.push(format!(
            "{} wider than {MAX_EXACT_VECTOR_BITS} bits, which no f64 sample holds exactly; each \
             is plotted as one column per bit. The retained event history carries them whole, as \
             declared buses.",
            vector_variables(expanded_vectors)
        ));
    }
    if let Some(payload) = event_payload {
        if let AnalysisResultPayload::TransientEvents { digital_buses, .. } = &payload
            && !digital_buses.is_empty()
        {
            parsed.notes.push(if digital_buses.len() == 1 {
                "1 vector variable is retained as a declared digital bus over its member traces."
                    .to_owned()
            } else {
                format!(
                    "{} retained as declared digital buses over their member traces.",
                    vector_variables(digital_buses.len())
                )
            });
        }
        parsed.event_payload = Some(payload);
    }
    Ok(parsed)
}

/// "1 vector variable is" / "3 vector variables are", so a note reads as a
/// sentence at either count.
fn vector_variables(count: usize) -> String {
    if count == 1 {
        "1 vector variable is".to_owned()
    } else {
        format!("{count} vector variables are")
    }
}

/// The unsigned integer a four-state vector denotes, or `None` when any bit is
/// unknown or high impedance and the vector therefore denotes no integer.
fn vcd_bits_to_f64(bits: &[rspice_core::io::VcdBit]) -> Option<f64> {
    let mut value = 0_u64;
    for bit in bits {
        value = value.checked_mul(2)?;
        match bit {
            rspice_core::io::VcdBit::Zero => {}
            rspice_core::io::VcdBit::One => value += 1,
            rspice_core::io::VcdBit::Unknown | rspice_core::io::VcdBit::HighImpedance => {
                return None;
            }
        }
    }
    Some(value as f64)
}

#[cfg(feature = "fst")]
fn fst_limits(policy: DigitalImportLimits) -> crate::fst::FstLimits {
    crate::fst::FstLimits {
        max_bytes: policy.max_bytes,
        max_columns: policy.samples.max_columns,
        max_rows: policy.samples.max_rows,
        max_values: policy.samples.max_values,
        max_signal_name_bytes: policy.samples.max_signal_name_bytes,
    }
}

#[cfg(feature = "fst")]
pub fn decode_fst(
    bytes: &[u8],
    format: ResultImportFormat,
    policy: DigitalImportLimits,
) -> Result<DecodedDigital, DigitalReadError> {
    let mut events = Vec::new();
    let mut recorded: Vec<Vec<rspice_core::io::VcdChange>> = Vec::new();
    let decoded =
        crate::fst::decode_fst(bytes, fst_limits(policy), format.canonical_id(), |event| {
            if recorded.len() <= event.signal {
                recorded.resize_with(event.signal + 1, Vec::new);
            }
            let value = match event.raw {
                crate::fst::FstRawValue::Logic(bits) => rspice_core::io::VcdValue::Logic(
                    bits.iter().copied().map(fst_bit).collect::<Vec<_>>(),
                ),
                crate::fst::FstRawValue::Real(value) => rspice_core::io::VcdValue::Real(value),
            };
            recorded[event.signal].push(rspice_core::io::VcdChange {
                tick: event.tick,
                value,
            });
            events.push(DigitalEvent {
                tick: event.tick,
                signal: event.signal,
                value: event.sample,
            });
        })
        .map_err(DigitalReadError::Fst)?;
    recorded.resize_with(decoded.signals.len(), Vec::new);
    let signals = decoded
        .signals
        .into_iter()
        .map(|signal| DigitalSignal {
            name: signal.name,
            width: signal.width,
        })
        .collect::<Vec<_>>();
    let event_payload = fst_event_payload(&signals, recorded, decoded.timescale_seconds, format)?;
    let mut parsed = digital_events_to_dataset(
        format,
        decoded.timescale_seconds,
        signals,
        events,
        policy.samples,
    )?;
    append_digital_aliases(format, &mut parsed.data, decoded.aliases, policy.samples)?;
    if let Some(payload) = event_payload {
        if let AnalysisResultPayload::TransientEvents { digital_buses, .. } = &payload
            && !digital_buses.is_empty()
        {
            parsed.notes.push(format!(
                "{} vector variables are retained as declared digital buses over their member \
                 traces.",
                digital_buses.len()
            ));
        }
        parsed.event_payload = Some(payload);
    }
    Ok(parsed)
}
/// One FST four-state character as the VCD bit it means.
///
/// FST's alphabet is wider than VCD's: `h`/`l` are weak drives and `u`/`w`/`-`
/// are flavours of not-known. VCD has four states, so the weak drives keep
/// their level and everything else that is not a level is unknown — the same
/// collapse the dump writer applies to the twelve XSPICE states.
#[cfg(feature = "fst")]
const fn fst_bit(byte: u8) -> rspice_core::io::VcdBit {
    match byte {
        b'0' | b'l' | b'L' => rspice_core::io::VcdBit::Zero,
        b'1' | b'h' | b'H' => rspice_core::io::VcdBit::One,
        b'z' | b'Z' => rspice_core::io::VcdBit::HighImpedance,
        _ => rspice_core::io::VcdBit::Unknown,
    }
}

/// The exact event evidence an FST carried, through the same codec a dump
/// goes through.
///
/// The changes are restated as a `VcdDocument` and handed to
/// [`rspice_core::execution::vcd_event_histories`], so an FST vector and a VCD
/// vector become the same declaration over the same `name[k]` members by the
/// same rule. Nothing about bus naming, ranges or widths is decided here.
///
/// `None` when the file's tick is not a duration VCD can state; the sampled
/// grid still carries the whole file, and no evidence is invented for it.
#[cfg(feature = "fst")]
fn fst_event_payload(
    signals: &[DigitalSignal],
    recorded: Vec<Vec<rspice_core::io::VcdChange>>,
    timescale_seconds: f64,
    format: ResultImportFormat,
) -> Result<Option<AnalysisResultPayload>, DigitalReadError> {
    let Some(timescale) = rspice_core::io::VcdTimescale::ALL
        .into_iter()
        .find(|scale| scale.seconds() == timescale_seconds)
    else {
        return Ok(None);
    };
    let mut document = rspice_core::io::VcdDocument::new(timescale);
    for (index, (signal, changes)) in signals.iter().zip(recorded).enumerate() {
        let (kind, width) = match signal.width {
            Some(width) => (
                rspice_core::io::VcdSignalKind::Logic,
                u32::try_from(width)
                    .map_err(|_| adapter_error(format, "FST signal width overflow"))?,
            ),
            None => (rspice_core::io::VcdSignalKind::Real, 64),
        };
        document.signals.push(rspice_core::io::VcdSignal {
            identifier: index.to_string(),
            variables: vec![rspice_core::io::VcdVariable {
                scope: Vec::new(),
                name: signal.name.clone(),
            }],
            width,
            kind,
            changes,
        });
    }
    let histories = rspice_core::execution::vcd_event_histories(&document)
        .map_err(|source| DigitalReadError::EventProjection { format, source })?;
    Ok(imported_event_payload(&histories))
}

fn digital_events_to_dataset(
    format: ResultImportFormat,
    timescale_seconds: f64,
    signals: Vec<DigitalSignal>,
    mut events: Vec<DigitalEvent>,
    limits: WaveformImportLimits,
) -> Result<DecodedDigital, DigitalReadError> {
    let min_rows = limits.min_rows;
    let max_rows = limits.max_rows;
    if !timescale_seconds.is_finite() || timescale_seconds <= 0.0 {
        return Err(adapter_error(
            format,
            "digital timescale is not finite and positive",
        ));
    }
    if signals.is_empty() {
        return Err(adapter_error(format, "digital source declares no signals"));
    }
    for signal in &signals {
        validate_name(format, "signal", &signal.name, limits.max_signal_name_bytes)
            .map_err(DigitalReadError::ResultData)?;
        if let Some(width) = signal.width
            && (width == 0 || width > MAX_EXACT_VECTOR_BITS as usize)
        {
            return Err(adapter_error(
                format,
                format_args!(
                    "signal '{}' width {width} cannot be represented exactly",
                    signal.name
                ),
            ));
        }
    }
    events.sort_by_key(|event| event.tick);
    let ticks = events
        .iter()
        .map(|event| event.tick)
        .collect::<BTreeSet<_>>();
    if ticks.len() < min_rows || ticks.len() > max_rows {
        return Err(DigitalReadError::EventTimeCount {
            format,
            count: ticks.len(),
            min: min_rows,
            max: max_rows,
        });
    }
    // Sparse events can expand to every signal at every distinct tick. Admit
    // that dense storage before allocating columns or walking their samples.
    let columns = signals
        .len()
        .checked_add(1)
        .ok_or_else(|| adapter_error(format, "digital column count overflow"))?;
    if columns > limits.max_columns {
        return Err(adapter_error(format, "digital signal-count limit exceeded"));
    }
    let retained_values = ticks
        .len()
        .checked_mul(columns)
        .ok_or_else(|| adapter_error(format, "digital retained-value count overflow"))?;
    if retained_values > limits.max_values {
        return Err(adapter_error(
            format,
            format_args!(
                "the source expands to {retained_values} numeric values; the limit is {}",
                limits.max_values
            ),
        ));
    }
    let mut states = vec![None; signals.len()];
    // Cloning an empty Vec does not preserve its reserved capacity.
    let mut values = (0..signals.len())
        .map(|_| Vec::with_capacity(ticks.len()))
        .collect::<Vec<_>>();
    let mut coordinate = Vec::with_capacity(ticks.len());
    let mut events = events.into_iter().peekable();
    for tick in ticks {
        if tick > MAX_EXACT_F64_INTEGER {
            return Err(DigitalReadError::InexactTimestamp { format, tick });
        }
        while events.peek().is_some_and(|event| event.tick == tick) {
            let event = events.next().expect("peeked event exists");
            if event.signal >= states.len() {
                return Err(adapter_error(
                    format,
                    "digital event references an unknown signal",
                ));
            }
            states[event.signal] = Some(event.value);
        }
        if states.iter().any(Option::is_none) {
            return Err(adapter_error(
                format,
                format_args!(
                    "not every digital signal has a known 0/1/vector value at initial tick {tick}"
                ),
            ));
        }
        let time = (tick as f64) * timescale_seconds;
        if !time.is_finite() {
            return Err(adapter_error(
                format,
                "scaled digital timestamp is not finite",
            ));
        }
        coordinate.push(time);
        for (column, state) in values.iter_mut().zip(&states) {
            column.push(state.expect("all states checked"));
        }
    }
    let signals = signals
        .into_iter()
        .zip(values)
        .map(|(signal, real)| ImportedSignal {
            name: signal.name,
            real,
            imag: None,
            unit: None,
        })
        .collect();
    Ok(DecodedDigital {
        data: assemble_imported_waveforms(
            format,
            AnalysisType::Transient,
            "time",
            coordinate,
            signals,
            limits,
        )
        .map_err(DigitalReadError::ResultData)?,
        notes: Vec::new(),
        event_payload: None,
    })
}

fn append_digital_aliases(
    format: ResultImportFormat,
    parsed: &mut ImportedWaveforms,
    aliases: Vec<(usize, String)>,
    limits: WaveformImportLimits,
) -> Result<(), DigitalReadError> {
    if parsed.waveforms.len().saturating_add(aliases.len()) > limits.max_columns.saturating_sub(1) {
        return Err(adapter_error(
            format,
            "digital aliases exceed the signal-count limit",
        ));
    }
    let mut known = parsed
        .waveforms
        .iter()
        .map(|waveform| waveform.name.to_ascii_lowercase())
        .collect::<HashSet<_>>();
    let mut alias_waveforms = Vec::with_capacity(aliases.len());
    for (canonical_index, name) in aliases {
        validate_name(format, "signal", &name, limits.max_signal_name_bytes)
            .map_err(DigitalReadError::ResultData)?;
        if !known.insert(name.to_ascii_lowercase()) {
            return Err(adapter_error(
                format,
                format_args!("duplicate digital signal identity '{name}'"),
            ));
        }
        let canonical = parsed.waveforms.get(canonical_index).ok_or_else(|| {
            adapter_error(
                format,
                "digital alias references an unknown canonical signal",
            )
        })?;
        alias_waveforms.push(RetainedWaveform::new(
            name,
            Arc::clone(&canonical.x),
            Arc::clone(&canonical.y),
        ));
    }
    parsed.waveforms.extend(alias_waveforms);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn limits() -> DigitalImportLimits {
        DigitalImportLimits {
            max_bytes: 64 * 1024 * 1024,
            samples: WaveformImportLimits {
                min_rows: 1,
                max_rows: 1_000_000,
                max_columns: 1_024,
                max_values: 64 * 1024 * 1024 / std::mem::size_of::<f64>(),
                max_signal_name_bytes: 1_024,
            },
        }
    }

    #[test]
    fn sparse_vcd_checks_the_expanded_grid_budget() {
        let mut source = String::from(
            "$timescale 1 ns $end\n$scope module top $end\n$var wire 1 ! a $end\n$var wire 1 \" b $end\n$var wire 1 # c $end\n$upscope $end\n$enddefinitions $end\n#0\n0!\n0\"\n0#\n",
        );
        for tick in 1..20 {
            source.push_str(&format!("#{tick}\n{}!\n", tick % 2));
        }
        let mut policy = limits();
        policy.samples.max_values = 79;
        let error = decode_vcd(source.as_bytes(), ResultImportFormat::Vcd, policy).unwrap_err();
        assert!(
            error
                .to_string()
                .contains("expands to 80 numeric values; the limit is 79"),
            "{error}"
        );
        policy.samples.max_values = 80;
        let decoded = decode_vcd(source.as_bytes(), ResultImportFormat::Vcd, policy).unwrap();
        assert_eq!(decoded.data.sample_count, 20);
        assert_eq!(decoded.data.waveforms.len(), 3);
        assert!(
            decoded.data.waveforms[1]
                .y
                .iter()
                .all(|value| *value == 0.0)
        );
        assert!(decoded.event_payload.is_some());
    }

    #[test]
    fn dense_digital_expansion_is_admitted_before_allocating_or_projecting_samples() {
        let mut policy = limits().samples;
        policy.max_values = 3;
        let signals = || {
            vec![DigitalSignal {
                name: "clock".into(),
                width: Some(1),
            }]
        };
        let events = |signal| {
            vec![
                DigitalEvent {
                    tick: 0,
                    signal,
                    value: 0.0,
                },
                DigitalEvent {
                    tick: 1,
                    signal,
                    value: 1.0,
                },
            ]
        };
        // Two sparse events retain two coordinates and two signal values. A bad
        // signal reference must not be visited before the dense budget refusal.
        let error = digital_events_to_dataset(
            ResultImportFormat::Vcd,
            1e-9,
            signals(),
            events(usize::MAX),
            policy,
        )
        .unwrap_err();
        assert!(
            error
                .to_string()
                .contains("expands to 4 numeric values; the limit is 3"),
            "{error}"
        );
        policy.max_values = 4;
        let decoded =
            digital_events_to_dataset(ResultImportFormat::Vcd, 1e-9, signals(), events(0), policy)
                .unwrap();
        assert_eq!(decoded.data.waveforms[0].x.as_slice(), [0.0, 1e-9]);
        assert_eq!(decoded.data.waveforms[0].y.as_slice(), [0.0, 1.0]);
    }

    #[test]
    fn vcd_imports_unknown_and_high_impedance_at_the_projection_level() {
        let vcd = b"$timescale 1 ns $end\n$scope module top $end\n$var wire 1 ! a $end\n$var wire 2 \" bus $end\n$upscope $end\n$enddefinitions $end\n#0\nx!\nb0x \"\n#1\n1!\nb01 \"\n#2\nz!\nb11 \"\n";
        let parsed =
            decode_vcd(vcd, ResultImportFormat::Vcd, limits()).expect("four-state VCD imports");
        assert_eq!(parsed.data.waveforms[0].y.as_slice(), &[0.5, 1.0, 0.5]);
        assert_eq!(
            parsed.data.waveforms[1].y.as_slice(),
            &[0.5, 1.0, 3.0],
            "a vector with any unknown bit denotes no integer"
        );
        let note = &parsed.notes[0];
        assert!(
            note.starts_with("3 sampled values were unknown (x) or high impedance (z)"),
            "unexpected note: {note}"
        );
        assert!(note.contains("0.5"), "unexpected note: {note}");
        assert!(
            note.contains("keeps the four-state code the file recorded"),
            "the level is the grid's decision, not a loss: {note}"
        );
    }

    #[test]
    fn vcd_aliases_share_one_timeline_under_their_own_names() {
        let vcd = b"$timescale 1 ns $end\n$scope module top $end\n$var wire 1 ! clk $end\n$var wire 1 ! clock $end\n$upscope $end\n$enddefinitions $end\n#0\n0!\n#5\n1!\n";
        let parsed = decode_vcd(vcd, ResultImportFormat::Vcd, limits()).expect("aliased VCD");
        assert_eq!(
            parsed
                .data
                .waveforms
                .iter()
                .map(|waveform| waveform.name.as_str())
                .collect::<Vec<_>>(),
            vec!["top.clk", "top.clock"]
        );
        assert_eq!(parsed.data.waveforms[0].y, parsed.data.waveforms[1].y);
        assert!(Arc::ptr_eq(
            &parsed.data.waveforms[0].y,
            &parsed.data.waveforms[1].y
        ));
    }

    #[test]
    fn vcd_keeps_the_changes_a_dumpoff_block_records() {
        let vcd = b"$timescale 1 ns $end\n$scope module top $end\n$var wire 1 ! a $end\n$upscope $end\n$enddefinitions $end\n#0\n$dumpvars\n0!\n$end\n#5\n1!\n#10\n$dumpoff\nx!\n$end\n";
        let parsed =
            decode_vcd(vcd, ResultImportFormat::Vcd, limits()).expect("VCD with a dump block");
        assert_eq!(
            parsed.data.waveforms[0].y.as_slice(),
            &[0.0, 1.0, 0.5],
            "a $dumpoff block records that the signals stopped being dumped"
        );
        assert_eq!(parsed.data.sample_count, 3);
    }

    #[test]
    fn host_limits_refuse_columns_and_rows_without_underflow() {
        let vcd =
            b"$timescale 1 ns $end\n$var wire 1 ! clk $end\n$enddefinitions $end\n#0\n0!\n#1\n1!\n";
        for max_columns in [0, 1] {
            let mut policy = limits();
            policy.samples.max_columns = max_columns;
            assert!(
                decode_vcd(vcd, ResultImportFormat::Vcd, policy)
                    .unwrap_err()
                    .to_string()
                    .contains("signal-count limit exceeded")
            );
        }
        let mut policy = limits();
        policy.samples.max_rows = 1;
        assert!(
            decode_vcd(vcd, ResultImportFormat::Vcd, policy)
                .unwrap_err()
                .to_string()
                .contains("2 distinct event times; expected 1..=1")
        );
    }

    #[test]
    fn vcd_encoding_failure_retains_the_core_parser_cause() {
        use std::error::Error as _;
        let error = decode_vcd(&[0xff], ResultImportFormat::Vcd, limits()).unwrap_err();
        let DigitalReadError::Vcd { format, source } = &error else {
            panic!("expected parser failure: {error}");
        };
        assert_eq!(*format, ResultImportFormat::Vcd);
        assert!(matches!(source, rspice_core::io::VcdError::Encoding(_)));
        assert!(error.source().unwrap().is::<rspice_core::io::VcdError>());
        assert_eq!(error.to_string(), format!("vcd import: {source}"));
    }
}

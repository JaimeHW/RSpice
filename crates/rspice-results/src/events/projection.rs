//! Exact event ordering, borrowed row readouts and declared-bus interpretation.
//!
//! The host admits evidence and binds cached projections to dataset revisions.
//! Row queries read only indexed samples; they do not revalidate a whole trace.
//! Scan callbacks report work without importing a host's instrumentation or UI.

use super::{DigitalBusEvidence, DigitalEventTraceEvidence};
use crate::analysis_payload::AnalysisResultPayload;
use crate::analysis_result::AnalysisResult;
use crate::waveform::RetainedWaveform;

/// Where one row of the event history came from.
///
/// Exact rows retain sparse timestamps from the engine or an imported file.
/// Projected rows are reconstructed from `D(..)`/`E(..)` waveforms, which were
/// sampled on a common grid — the distinction is reported, never hidden,
/// because a projected time is an approximation of the real one. A `Bus` row
/// is exact and derived: the word is reassembled from member rows that are
/// themselves exact, so it is never available for a projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventSelectionSource {
    ExactDigital,
    ExactReal,
    ProjectedDigital,
    ProjectedReal,
    Bus,
}

/// One event row's position in the merged, time-ordered history.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EventOrderEntry {
    source: EventSelectionSource,
    /// Index of the trace, waveform, or — for a `Bus` row — the declaration
    /// this row reads.
    trace_index: usize,
    point_index: usize,
    time_s: f64,
    initial: bool,
}

/// One declared bus, reassembled once for the sheet that draws it.
///
/// The word at each event is `rspice_core::execution::bus_events` reading the
/// member histories; nothing here holds a value the members do not.
#[derive(Debug, Clone, PartialEq)]
pub struct BusTimeline {
    /// The declaration's name with its declared range, as the sheet spells it.
    label: String,
    /// Bus name without the range — the selection key.
    name: String,
    /// Member node names, declared MSB first.
    members: Vec<String>,
    /// Every time a member changed, with the code each member held at it.
    events: rspice_core::execution::BusEventTable,
    /// Why this bus has no rows, when it has none.
    refusal: Option<String>,
}

/// One immutable merge of the retained schedules and declared buses.
#[derive(Debug, Clone, PartialEq)]
pub struct EventOrder {
    exact: bool,
    rows: Vec<EventOrderEntry>,
    buses: Vec<BusTimeline>,
    current_names: Vec<String>,
    current_rows: Vec<(usize, usize)>,
}

impl BusTimeline {
    pub fn label(&self) -> &str {
        &self.label
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn members(&self) -> &[String] {
        &self.members
    }
    pub fn events(&self) -> &rspice_core::execution::BusEventTable {
        &self.events
    }
}

impl EventOrder {
    pub fn exact(&self) -> bool {
        self.exact
    }
    pub fn rows(&self) -> &[EventOrderEntry] {
        &self.rows
    }
    pub fn buses(&self) -> &[BusTimeline] {
        &self.buses
    }
    pub fn current_names(&self) -> &[String] {
        &self.current_names
    }
    pub fn current_rows(&self) -> &[(usize, usize)] {
        &self.current_rows
    }
}

/// How a bus word is spelled.
///
/// Binary is the VCD spelling — one character per member, declared MSB first,
/// with `x` for unknown and `z` for high impedance — and is the only one that
/// can spell every word a run can produce. The other three denote an integer,
/// which a word with an unknown or high-impedance bit does not have.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BusRadix {
    #[default]
    Binary,
    Hex,
    Unsigned,
    Signed,
}

impl BusRadix {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Binary => "binary",
            Self::Hex => "hexadecimal",
            Self::Unsigned => "unsigned decimal",
            Self::Signed => "signed decimal",
        }
    }
}

/// The widest bus whose unsigned and signed values a machine integer holds
/// exactly. A wider word still has a hexadecimal and a binary spelling, both
/// of which are exact at any width because neither is arithmetic.
const MAX_DECIMAL_BUS_BITS: usize = 64;

/// What one bus word is shown as, and why it is not what was asked for.
#[derive(Clone)]
pub struct BusWord {
    text: String,
    /// The reason the requested radix was not used, when it was not.
    fallback: Option<&'static str>,
}

const UNRESOLVED_BITS_FALLBACK: &str =
    "the word carries unknown (x) or high-impedance (z) bits, which denote no integer";
const WIDE_WORD_FALLBACK: &str = "the bus is wider than 64 bits, which no exact machine integer \
     spells; hexadecimal states the same word";

/// Spell one bus word: the codes each member held, declared MSB first.
///
/// The bits come from `rspice_core::execution::event_code_to_vcd_bit`, which
/// is the one place a code becomes a bit — the same mapping the dump uses, so
/// a word read here and a word read out of an exported VCD cannot disagree.
fn bus_word(codes: &[Option<u8>], radix: BusRadix) -> BusWord {
    use rspice_core::execution::event_code_to_vcd_bit;
    use rspice_core::io::VcdBit;

    let bits = codes
        .iter()
        .map(|code| event_code_to_vcd_bit(*code))
        .collect::<Vec<_>>();
    let binary = || {
        bits.iter()
            .map(|bit| bit.map_or('?', VcdBit::as_char))
            .collect::<String>()
    };
    let resolved = bits
        .iter()
        .map(|bit| match bit {
            Some(VcdBit::Zero) => Some(false),
            Some(VcdBit::One) => Some(true),
            _ => None,
        })
        .collect::<Option<Vec<_>>>();
    let Some(resolved) = resolved else {
        return BusWord {
            text: binary(),
            fallback: (radix != BusRadix::Binary).then_some(UNRESOLVED_BITS_FALLBACK),
        };
    };
    match radix {
        BusRadix::Binary => BusWord {
            text: binary(),
            fallback: None,
        },
        BusRadix::Hex => BusWord {
            text: hex_word(&resolved),
            fallback: None,
        },
        BusRadix::Unsigned | BusRadix::Signed => {
            if resolved.len() > MAX_DECIMAL_BUS_BITS {
                return BusWord {
                    text: hex_word(&resolved),
                    fallback: Some(WIDE_WORD_FALLBACK),
                };
            }
            let magnitude = resolved
                .iter()
                .fold(0_u64, |value, bit| (value << 1) | u64::from(*bit));
            let text = if radix == BusRadix::Signed {
                // Two's complement in the declared width: the sign bit is the
                // declared MSB, not bit 63, so a narrow bus signs at its own
                // width. Shifting up and back does that in one step.
                let spare = MAX_DECIMAL_BUS_BITS - resolved.len();
                let signed = (magnitude << spare) as i64 >> spare;
                signed.to_string()
            } else {
                magnitude.to_string()
            };
            BusWord {
                text,
                fallback: None,
            }
        }
    }
}

/// The hexadecimal spelling of a fully resolved word, declared MSB first.
///
/// Grouped from the least significant bit so the last digit is always a whole
/// nibble; a width that is not a multiple of four leaves the leading digit
/// short, which is what it is.
fn hex_word(bits: &[bool]) -> String {
    let mut digits = Vec::with_capacity(bits.len().div_ceil(4));
    let mut nibble = 0_u8;
    let mut filled = 0_u32;
    for bit in bits.iter().rev() {
        nibble |= u8::from(*bit) << filled;
        filled += 1;
        if filled == 4 {
            digits.push(nibble);
            nibble = 0;
            filled = 0;
        }
    }
    if filled > 0 {
        digits.push(nibble);
    }
    let mut text = String::with_capacity(digits.len() + 2);
    text.push_str("0x");
    for digit in digits.iter().rev() {
        text.push(char::from_digit(u32::from(*digit), 16).unwrap_or('?'));
    }
    text
}

#[derive(Clone, Copy)]
pub struct LogicCode {
    value: &'static str,
    strength: &'static str,
}

#[derive(Clone)]
pub enum EventValue {
    Digital {
        code: u8,
        decoded: LogicCode,
    },
    Real(f64),
    /// A whole bus word at one bus event, already spelled in the sheet's
    /// radix. The word is derived, so it carries the reason it is not in the
    /// radix that was asked for whenever that radix cannot state it.
    Bus(BusWord),
}

impl EventValue {
    pub fn identity(&self) -> u64 {
        match self {
            Self::Digital { code, .. } => u64::from(*code),
            Self::Real(value) => value.to_bits(),
            // A bus row is never compared for change compression: the
            // reassembly already emits one row per change of the whole word.
            Self::Bus(_) => 0,
        }
    }

    pub fn display(&self) -> String {
        match self {
            Self::Digital { decoded, .. } => decoded.value.to_owned(),
            Self::Real(value) => format!("{value:.17e}"),
            Self::Bus(word) => word.text.clone(),
        }
    }

    pub const fn strength(&self) -> &'static str {
        match self {
            Self::Digital { decoded, .. } => decoded.strength,
            Self::Real(_) => "real-valued",
            Self::Bus(_) => "per member",
        }
    }

    pub const fn domain(&self) -> &'static str {
        match self {
            Self::Digital { .. } => "digital",
            Self::Real(_) => "real",
            Self::Bus(_) => "digital bus",
        }
    }
}

impl BusWord {
    pub fn fallback(&self) -> Option<&'static str> {
        self.fallback
    }
}

#[derive(Clone)]
pub struct EventRow<'a> {
    source: EventSelectionSource,
    trace_name: &'a str,
    signal_name: &'a str,
    point_index: usize,
    event_ordinal: usize,
    time_s: f64,
    value: EventValue,
    initial: bool,
}

impl EventRow<'_> {
    pub const fn exact(&self) -> bool {
        matches!(
            self.source,
            EventSelectionSource::ExactDigital
                | EventSelectionSource::ExactReal
                // A bus word is reassembled from exact member histories at
                // the exact time one of them changed. Nothing is resampled.
                | EventSelectionSource::Bus
        )
    }
}

impl EventRow<'_> {
    pub fn source(&self) -> EventSelectionSource {
        self.source
    }
    pub fn trace_name(&self) -> &str {
        self.trace_name
    }
    pub fn signal_name(&self) -> &str {
        self.signal_name
    }
    pub fn point_index(&self) -> usize {
        self.point_index
    }
    pub fn event_ordinal(&self) -> usize {
        self.event_ordinal
    }
    pub fn time_s(&self) -> f64 {
        self.time_s
    }
    pub fn value(&self) -> &EventValue {
        &self.value
    }
    pub fn initial(&self) -> bool {
        self.initial
    }
}

fn logic_code(value: f64) -> Option<(u8, LogicCode)> {
    if !value.is_finite() || value.fract() != 0.0 || !(0.0..=12.0).contains(&value) {
        return None;
    }
    let code = value as u8;
    let decoded = match code {
        0 => LogicCode {
            value: "0",
            strength: "strong",
        },
        1 => LogicCode {
            value: "1",
            strength: "strong",
        },
        2 => LogicCode {
            value: "X",
            strength: "strong",
        },
        3 => LogicCode {
            value: "0",
            strength: "resistive",
        },
        4 => LogicCode {
            value: "1",
            strength: "resistive",
        },
        5 => LogicCode {
            value: "X",
            strength: "resistive",
        },
        6 => LogicCode {
            value: "0",
            strength: "high-Z",
        },
        7 => LogicCode {
            value: "1",
            strength: "high-Z",
        },
        8 => LogicCode {
            value: "X",
            strength: "high-Z",
        },
        9 => LogicCode {
            value: "0",
            strength: "undetermined",
        },
        10 => LogicCode {
            value: "1",
            strength: "undetermined",
        },
        11 => LogicCode {
            value: "X",
            strength: "undetermined",
        },
        12 => LogicCode {
            value: "Z",
            strength: "high-Z",
        },
        _ => return None,
    };
    Some((code, decoded))
}

fn logic_code_u8(code: u8) -> Option<LogicCode> {
    logic_code(f64::from(code)).map(|(_, decoded)| decoded)
}

fn event_signal_name(name: &str) -> Option<(&str, bool)> {
    if let Some(signal) = name
        .strip_prefix("D(")
        .and_then(|name| name.strip_suffix(')'))
    {
        Some((signal, true))
    } else {
        name.strip_prefix("E(")
            .and_then(|name| name.strip_suffix(')'))
            .map(|signal| (signal, false))
    }
}

fn event_value(waveform: &RetainedWaveform, value: f64) -> Option<EventValue> {
    let (_, digital) = event_signal_name(&waveform.name)?;
    if digital {
        logic_code(value).map(|(code, decoded)| EventValue::Digital { code, decoded })
    } else {
        value.is_finite().then_some(EventValue::Real(value))
    }
}

fn event_time_axis_is_valid(values: &[f64]) -> bool {
    !values.is_empty()
        && values.iter().all(|value| value.is_finite())
        && values.windows(2).all(|pair| pair[0] < pair[1])
}

pub fn waveform_is_event(waveform: &RetainedWaveform, on_sample_scan: impl FnOnce()) -> bool {
    let Some((_, digital)) = event_signal_name(&waveform.name) else {
        return false;
    };
    on_sample_scan();
    if waveform.x.len() != waveform.y.len() || !event_time_axis_is_valid(&waveform.x) {
        return false;
    }
    if digital {
        waveform
            .y
            .iter()
            .copied()
            .all(|value| logic_code(value).is_some())
    } else {
        waveform.y.iter().all(|value| value.is_finite())
    }
}

/// Reassemble every declared bus over the traces it names.
///
/// The word at each event comes from `rspice_core::execution::bus_events`,
/// which is the only reassembly in the product. A declaration whose members
/// are more than one reassembly holds at once is refused by that function, in
/// its own words, and the refusal is carried here so the sheet can state it
/// where the rows would have been rather than showing an empty bus.
fn build_bus_timelines(
    digital_traces: &[DigitalEventTraceEvidence],
    digital_buses: &[DigitalBusEvidence],
) -> Vec<BusTimeline> {
    use rspice_core::execution::{BusMemberHistory, bus_events};

    // One history per member, in the member's own point order. The retained
    // evidence is already sorted in time — `validate_event_times` refuses one
    // that is not — so nothing here reorders anything.
    let histories = digital_traces
        .iter()
        .map(|trace| {
            (
                trace.node_name.as_str(),
                trace
                    .points
                    .iter()
                    .map(|point| (point.time_s, point.value_code))
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<std::collections::HashMap<_, _>>();

    digital_buses
        .iter()
        .map(|bus| {
            let members = bus
                .members
                .iter()
                .map(|member| BusMemberHistory {
                    points: histories
                        .get(member.as_str())
                        .map_or(&[][..], Vec::as_slice),
                })
                .collect::<Vec<_>>();
            let (events, refusal) = match bus_events(&members) {
                Ok(events) => (events, None),
                Err(error) => (Vec::new(), Some(error.to_string())),
            };
            BusTimeline {
                label: format!("{}[{}:{}]", bus.name, bus.msb, bus.lsb),
                name: bus.name.clone(),
                members: bus.members.clone(),
                events,
                refusal,
            }
        })
        .collect()
}

pub fn build_event_order<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
    expanded: &std::collections::BTreeSet<String>,
    mut on_projection_scan: impl FnMut(),
) -> EventOrder {
    if let Some(AnalysisResultPayload::TransientEvents {
        digital_traces,
        real_traces,
        digital_buses,
        current_impulses,
        ..
    }) = analysis.result_payload.as_ref()
    {
        let buses = build_bus_timelines(digital_traces, digital_buses);
        // A member of a collapsed bus is not listed on its own: the bus row
        // is the same change, stated once as the word it belongs to. A bus
        // that was refused hides nothing — its members are all there is.
        let hidden = buses
            .iter()
            .filter(|bus| bus.refusal.is_none() && !expanded.contains(&bus.name))
            .flat_map(|bus| bus.members.iter().map(String::as_str))
            .collect::<std::collections::HashSet<_>>();
        let mut rows = Vec::new();
        for (trace_index, trace) in digital_traces.iter().enumerate() {
            if hidden.contains(trace.node_name.as_str()) {
                continue;
            }
            for (point_index, point) in trace.points.iter().enumerate() {
                rows.push(EventOrderEntry {
                    source: EventSelectionSource::ExactDigital,
                    trace_index,
                    point_index,
                    time_s: point.time_s,
                    initial: point_index == 0,
                });
            }
        }
        for (bus_index, bus) in buses.iter().enumerate() {
            for (point_index, (time_s, _)) in bus.events.iter().enumerate() {
                rows.push(EventOrderEntry {
                    source: EventSelectionSource::Bus,
                    trace_index: bus_index,
                    point_index,
                    time_s: *time_s,
                    initial: point_index == 0,
                });
            }
        }
        for (trace_index, trace) in real_traces.iter().enumerate() {
            for (point_index, point) in trace.points.iter().enumerate() {
                rows.push(EventOrderEntry {
                    source: EventSelectionSource::ExactReal,
                    trace_index,
                    point_index,
                    time_s: point.time_s,
                    initial: point_index == 0,
                });
            }
        }
        sort_event_order(analysis, &buses, &mut rows);
        let mut current_names = Vec::new();
        let mut current_rows = Vec::new();
        if let Some(history) = current_impulses {
            for (trace_index, trace) in history.traces.iter().enumerate() {
                current_names.push(trace.owner.to_string());
                current_rows
                    .extend((0..trace.points.len()).map(|point_index| (trace_index, point_index)));
            }
            current_rows.sort_by(|&(left_trace, left_point), &(right_trace, right_point)| {
                history.traces[left_trace].points[left_point]
                    .time
                    .total_cmp(&history.traces[right_trace].points[right_point].time)
                    .then_with(|| current_names[left_trace].cmp(&current_names[right_trace]))
            });
        }
        return EventOrder {
            exact: true,
            rows,
            buses,
            current_names,
            current_rows,
        };
    }

    // Legacy fallback: preserve access to old project files, but label these
    // rows as projections because the accepted transient grid is not the
    // original sparse event schedule.
    let mut rows = Vec::new();
    for (waveform_index, waveform) in analysis
        .waveforms
        .iter()
        .enumerate()
        .filter(|(_, waveform)| waveform_is_event(waveform.as_ref(), &mut on_projection_scan))
    {
        let waveform = waveform.as_ref();
        if event_signal_name(&waveform.name).is_none() {
            continue;
        }
        let mut previous = None;
        for (sample_index, (&time_s, &raw_value)) in
            waveform.x.iter().zip(waveform.y.iter()).enumerate()
        {
            let Some(value) = event_value(waveform, raw_value) else {
                continue;
            };
            let identity = value.identity();
            if previous == Some(identity) {
                continue;
            }
            rows.push(EventOrderEntry {
                source: if matches!(value, EventValue::Digital { .. }) {
                    EventSelectionSource::ProjectedDigital
                } else {
                    EventSelectionSource::ProjectedReal
                },
                trace_index: waveform_index,
                point_index: sample_index,
                time_s,
                initial: previous.is_none(),
            });
            previous = Some(identity);
        }
    }
    sort_event_order(analysis, &[], &mut rows);
    EventOrder {
        exact: false,
        rows,
        buses: Vec::new(),
        current_names: Vec::new(),
        current_rows: Vec::new(),
    }
}

fn sort_event_order<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
    buses: &[BusTimeline],
    rows: &mut [EventOrderEntry],
) {
    rows.sort_by(|left, right| {
        left.time_s
            .total_cmp(&right.time_s)
            .then_with(|| {
                event_entry_signal_name(analysis, buses, *left)
                    .cmp(event_entry_signal_name(analysis, buses, *right))
            })
            .then_with(|| left.point_index.cmp(&right.point_index))
    });
}

fn event_entry_signal_name<'a, W: AsRef<RetainedWaveform>>(
    analysis: &'a AnalysisResult<W>,
    buses: &'a [BusTimeline],
    entry: EventOrderEntry,
) -> &'a str {
    match entry.source {
        EventSelectionSource::ExactDigital => analysis
            .result_payload
            .as_ref()
            .and_then(|payload| match payload {
                AnalysisResultPayload::TransientEvents { digital_traces, .. } => {
                    digital_traces.get(entry.trace_index)
                }
                _ => None,
            })
            .map_or("", |trace| trace.node_name.as_str()),
        EventSelectionSource::ExactReal => analysis
            .result_payload
            .as_ref()
            .and_then(|payload| match payload {
                AnalysisResultPayload::TransientEvents { real_traces, .. } => {
                    real_traces.get(entry.trace_index)
                }
                _ => None,
            })
            .map_or("", |trace| trace.node_name.as_str()),
        EventSelectionSource::ProjectedDigital | EventSelectionSource::ProjectedReal => analysis
            .waveforms
            .get(entry.trace_index)
            .and_then(|waveform| event_signal_name(&waveform.as_ref().name).map(|(name, _)| name))
            .unwrap_or(""),
        EventSelectionSource::Bus => buses
            .get(entry.trace_index)
            .map_or("", |bus| bus.label.as_str()),
    }
}

pub fn event_row_from_entry<'a, W: AsRef<RetainedWaveform>>(
    analysis: &'a AnalysisResult<W>,
    buses: &'a [BusTimeline],
    radix: BusRadix,
    entry: EventOrderEntry,
    event_ordinal: usize,
) -> Option<EventRow<'a>> {
    match entry.source {
        EventSelectionSource::ExactDigital => {
            let AnalysisResultPayload::TransientEvents { digital_traces, .. } =
                analysis.result_payload.as_ref()?
            else {
                return None;
            };
            let trace = digital_traces.get(entry.trace_index)?;
            let point = trace.points.get(entry.point_index)?;
            Some(EventRow {
                source: entry.source,
                trace_name: &trace.node_name,
                signal_name: &trace.node_name,
                point_index: entry.point_index,
                event_ordinal,
                time_s: point.time_s,
                value: EventValue::Digital {
                    code: point.value_code,
                    decoded: logic_code_u8(point.value_code)?,
                },
                initial: entry.initial,
            })
        }
        EventSelectionSource::ExactReal => {
            let AnalysisResultPayload::TransientEvents { real_traces, .. } =
                analysis.result_payload.as_ref()?
            else {
                return None;
            };
            let trace = real_traces.get(entry.trace_index)?;
            let point = trace.points.get(entry.point_index)?;
            Some(EventRow {
                source: entry.source,
                trace_name: &trace.node_name,
                signal_name: &trace.node_name,
                point_index: entry.point_index,
                event_ordinal,
                time_s: point.time_s,
                value: EventValue::Real(point.value),
                initial: entry.initial,
            })
        }
        EventSelectionSource::ProjectedDigital | EventSelectionSource::ProjectedReal => {
            let waveform = analysis.waveforms.get(entry.trace_index)?.as_ref();
            let (signal_name, _) = event_signal_name(&waveform.name)?;
            let (&time_s, &raw_value) = waveform
                .x
                .get(entry.point_index)
                .zip(waveform.y.get(entry.point_index))?;
            Some(EventRow {
                source: entry.source,
                trace_name: &waveform.name,
                signal_name,
                point_index: entry.point_index,
                event_ordinal,
                time_s,
                value: event_value(waveform, raw_value)?,
                initial: entry.initial,
            })
        }
        EventSelectionSource::Bus => {
            let bus = buses.get(entry.trace_index)?;
            let (time_s, codes) = bus.events.get(entry.point_index)?;
            Some(EventRow {
                source: entry.source,
                trace_name: &bus.name,
                signal_name: &bus.label,
                point_index: entry.point_index,
                event_ordinal,
                time_s: *time_s,
                value: EventValue::Bus(bus_word(codes, radix)),
                initial: entry.initial,
            })
        }
    }
}

/// Resolve a named point without rescanning its trace; the host checks selection identity.
pub fn event_row_at_name<'a, W: AsRef<RetainedWaveform>>(
    analysis: &'a AnalysisResult<W>,
    buses: &'a [BusTimeline],
    radix: BusRadix,
    source: EventSelectionSource,
    trace_name: &str,
    point_index: usize,
) -> Option<EventRow<'a>> {
    let entry = match source {
        EventSelectionSource::ExactDigital => {
            let AnalysisResultPayload::TransientEvents { digital_traces, .. } =
                analysis.result_payload.as_ref()?
            else {
                return None;
            };
            let (trace_index, trace) = digital_traces
                .iter()
                .enumerate()
                .find(|(_, trace)| trace.node_name == trace_name)?;
            let point = trace.points.get(point_index)?;
            EventOrderEntry {
                source,
                trace_index,
                point_index,
                time_s: point.time_s,
                initial: point_index == 0,
            }
        }
        EventSelectionSource::ExactReal => {
            let AnalysisResultPayload::TransientEvents { real_traces, .. } =
                analysis.result_payload.as_ref()?
            else {
                return None;
            };
            let (trace_index, trace) = real_traces
                .iter()
                .enumerate()
                .find(|(_, trace)| trace.node_name == trace_name)?;
            let point = trace.points.get(point_index)?;
            EventOrderEntry {
                source,
                trace_index,
                point_index,
                time_s: point.time_s,
                initial: point_index == 0,
            }
        }
        EventSelectionSource::ProjectedDigital | EventSelectionSource::ProjectedReal => {
            let (trace_index, waveform) = analysis
                .waveforms
                .iter()
                .enumerate()
                .find(|(_, waveform)| waveform.as_ref().name == trace_name)?;
            let waveform = waveform.as_ref();
            let (&time_s, &raw_value) = waveform
                .x
                .get(point_index)
                .zip(waveform.y.get(point_index))?;
            let current = event_value(waveform, raw_value)?;
            let expected_source = if matches!(current, EventValue::Digital { .. }) {
                EventSelectionSource::ProjectedDigital
            } else {
                EventSelectionSource::ProjectedReal
            };
            if source != expected_source {
                return None;
            }
            let previous = point_index
                .checked_sub(1)
                .and_then(|index| waveform.y.get(index))
                .and_then(|value| event_value(waveform, *value))
                .map(|value| value.identity());
            if previous == Some(current.identity()) {
                return None;
            }
            EventOrderEntry {
                source,
                trace_index,
                point_index,
                time_s,
                initial: previous.is_none(),
            }
        }
        EventSelectionSource::Bus => {
            let (bus_index, bus) = buses
                .iter()
                .enumerate()
                .find(|(_, bus)| bus.name == trace_name)?;
            let (time_s, _) = bus.events.get(point_index)?;
            EventOrderEntry {
                source,
                trace_index: bus_index,
                point_index,
                time_s: *time_s,
                initial: point_index == 0,
            }
        }
    };
    event_row_from_entry(analysis, buses, radix, entry, 0)
}

/// The bus half of the sheet's subtitle, or nothing when none is declared.
///
/// A sheet with no declaration says nothing about buses at all: a "0 buses"
/// count would suggest the run might have had one, and no run without a
/// vector port or an imported vector ever can.
pub fn bus_subtitle(buses: &[BusTimeline]) -> String {
    if buses.is_empty() {
        return String::new();
    }
    let members = buses.iter().map(|bus| bus.members.len()).sum::<usize>();
    format!(
        " · {} bus{} over {members} members",
        buses.len(),
        if buses.len() == 1 { "" } else { "es" }
    )
}

/// What the sheet has to say about its buses beyond the rows themselves.
///
/// Each note is an exact count of something the reader would otherwise have
/// to infer from an absence: a word the chosen radix cannot state, or a
/// declaration that was refused reassembly and therefore has no rows at all.
pub fn bus_notes(buses: &[BusTimeline], radix: BusRadix) -> Vec<String> {
    let mut notes = Vec::new();
    for bus in buses.iter().filter(|bus| bus.refusal.is_some()) {
        notes.push(format!(
            "{} has no bus rows: {}. Its {} member traces are listed individually.",
            bus.label,
            bus.refusal.as_deref().unwrap_or_default(),
            bus.members.len()
        ));
    }
    if radix == BusRadix::Binary {
        return notes;
    }
    let mut fallbacks: std::collections::BTreeMap<&'static str, usize> =
        std::collections::BTreeMap::new();
    for bus in buses {
        for (_, codes) in &bus.events {
            if let Some(reason) = bus_word(codes, radix).fallback {
                *fallbacks.entry(reason).or_default() += 1;
            }
        }
    }
    for (reason, count) in fallbacks {
        notes.push(format!(
            "{count} bus words are not shown in {}: {reason}.",
            radix.label()
        ));
    }
    notes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digital_code_contract_rejects_non_integral_values() {
        assert!(logic_code(12.0).is_some());
        assert!(logic_code(2.5).is_none());
        assert!(logic_code(13.0).is_none());
    }

    #[test]
    fn a_word_wider_than_a_machine_integer_falls_back_to_hexadecimal() {
        let ones = vec![Some(1_u8); 65];
        let word = bus_word(&ones, BusRadix::Unsigned);
        assert_eq!(word.fallback, Some(WIDE_WORD_FALLBACK));
        assert_eq!(word.text, "0x1ffffffffffffffff");
        assert_eq!(bus_word(&ones, BusRadix::Hex).fallback, None);
        assert_eq!(bus_word(&ones, BusRadix::Binary).text, "1".repeat(65));
    }

    #[test]
    fn the_binary_word_is_the_spelling_the_dump_writes() {
        let codes = [Some(0_u8), Some(1), Some(2), Some(12), None];
        assert_eq!(bus_word(&codes, BusRadix::Binary).text, "01xzx");
    }

    #[test]
    fn projected_rows_borrow_names_and_refuse_non_changes_and_missing_points() {
        let mut analysis: AnalysisResult = AnalysisResult::new(
            1,
            crate::analysis_type::AnalysisType::Transient,
            "TRAN",
            0.0,
        );
        analysis.waveforms.push(RetainedWaveform::new(
            "D(clk)",
            vec![0.0, 1.0, 2.0],
            vec![0.0, 0.0, 1.0],
        ));
        let mut scans = 0;
        let order = build_event_order(&analysis, &Default::default(), || scans += 1);
        assert_eq!(scans, 1);
        assert_eq!(order.rows().len(), 2);
        let row = event_row_at_name(
            &analysis,
            order.buses(),
            BusRadix::Binary,
            EventSelectionSource::ProjectedDigital,
            "D(clk)",
            2,
        )
        .unwrap();
        assert_eq!(
            row.trace_name().as_ptr(),
            analysis.waveforms[0].name.as_ptr()
        );
        assert_eq!(row.value().identity(), 1);
        for point in [1, usize::MAX] {
            assert!(
                event_row_at_name(
                    &analysis,
                    order.buses(),
                    BusRadix::Binary,
                    EventSelectionSource::ProjectedDigital,
                    "D(clk)",
                    point
                )
                .is_none()
            );
        }
        assert!(
            event_row_at_name(
                &analysis,
                order.buses(),
                BusRadix::Binary,
                EventSelectionSource::ProjectedReal,
                "D(clk)",
                2
            )
            .is_none()
        );
    }
}

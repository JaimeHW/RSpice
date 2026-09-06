//! Exact committed XSPICE digital and real-valued event history.

use egui::{RichText, Ui};
use egui_extras::{Column, TableBuilder};

use crate::state::{AnalysisResult, AnalysisResultPayload, AnalysisType, WaveformData};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{SegmentedWidth, chip, section_header, segmented};
use crate::workbench::AppState;

use super::strip::StripHeader;
use super::{AnalysisPresentationKey, SheetContext, panel_note, stat_table, well_hint};

const ROW_HEIGHT: f32 = 28.0;
const HEADER_HEIGHT: f32 = 31.0;

/// Where one row of the event history came from.
///
/// Exact rows are the schedule the event solver committed. Projected rows are
/// reconstructed from an older project's `D(..)`/`E(..)` waveforms, which were
/// sampled on the analog grid — the distinction is reported, never hidden,
/// because a projected time is an approximation of the real one. A `Bus` row
/// is exact and derived: the word is reassembled from member rows that are
/// themselves exact, so it is never available for a projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EventSelectionSource {
    ExactDigital,
    ExactReal,
    ProjectedDigital,
    ProjectedReal,
    Bus,
}

/// One event row's position in the merged, time-ordered history.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct EventOrderEntry {
    pub source: EventSelectionSource,
    /// Index of the trace, waveform, or — for a `Bus` row — the declaration
    /// this row reads.
    pub trace_index: usize,
    pub point_index: usize,
    pub time_s: f64,
    pub initial: bool,
}

/// One declared bus, reassembled once for the sheet that draws it.
///
/// The word at each event is `rspice_core::execution::bus_events` reading the
/// member histories; nothing here holds a value the members do not.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BusTimeline {
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

/// The merged event order for one analysis.
///
/// Built once per generation of one analysis rather than per frame: merging
/// every node's schedule is O(events log events), and the answer only changes
/// when the evidence does. The data version is part of the key for the same
/// reason it is part of the evidence-validity memo's — a run still accepting
/// points republishes one analysis identity with a longer history, so identity
/// alone would freeze the sheet at whatever it held on the frame it opened.
///
/// The expanded set is part of the key too: showing a bus's members adds rows
/// rather than filtering them, so the order itself differs. Expanding is a
/// click, so rebuilding then costs what opening the sheet costs.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct EventOrderCache {
    pub analysis: AnalysisPresentationKey,
    pub data_version: u64,
    /// Whether the rows are the committed schedule rather than a projection.
    pub exact: bool,
    pub rows: Vec<EventOrderEntry>,
    /// Buses declared over this analysis' digital traces, in declaration order.
    pub(super) buses: Vec<BusTimeline>,
    /// The bus names whose member rows this order includes.
    expanded: std::collections::BTreeSet<String>,
}

/// One event picked out of the history.
///
/// Addressed by node name rather than trace ordinal so the selection survives
/// a re-run that registers event nodes in a different order. A `Bus` row's
/// `trace_name` is the bus name, for the same reason.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct DigitalEventSelection {
    pub analysis: AnalysisPresentationKey,
    pub source: EventSelectionSource,
    pub trace_name: String,
    pub point_index: usize,
}

/// How a bus word is spelled.
///
/// Binary is the VCD spelling — one character per member, declared MSB first,
/// with `x` for unknown and `z` for high impedance — and is the only one that
/// can spell every word a run can produce. The other three denote an integer,
/// which a word with an unknown or high-impedance bit does not have.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum BusRadix {
    #[default]
    Binary,
    Hex,
    Unsigned,
    Signed,
}

impl BusRadix {
    pub(super) const OPTIONS: [&'static str; 4] = ["BIN", "HEX", "DEC", "±DEC"];

    const fn index(self) -> usize {
        match self {
            Self::Binary => 0,
            Self::Hex => 1,
            Self::Unsigned => 2,
            Self::Signed => 3,
        }
    }

    const fn from_index(index: usize) -> Self {
        match index {
            1 => Self::Hex,
            2 => Self::Unsigned,
            3 => Self::Signed,
            _ => Self::Binary,
        }
    }

    const fn label(self) -> &'static str {
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
struct BusWord {
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
struct LogicCode {
    value: &'static str,
    strength: &'static str,
}

#[derive(Clone)]
enum EventValue {
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
    fn identity(&self) -> u64 {
        match self {
            Self::Digital { code, .. } => u64::from(*code),
            Self::Real(value) => value.to_bits(),
            // A bus row is never compared for change compression: the
            // reassembly already emits one row per change of the whole word.
            Self::Bus(_) => 0,
        }
    }

    fn display(&self) -> String {
        match self {
            Self::Digital { decoded, .. } => decoded.value.to_owned(),
            Self::Real(value) => format!("{value:.17e}"),
            Self::Bus(word) => word.text.clone(),
        }
    }

    const fn strength(&self) -> &'static str {
        match self {
            Self::Digital { decoded, .. } => decoded.strength,
            Self::Real(_) => "real-valued",
            Self::Bus(_) => "per member",
        }
    }

    const fn domain(&self) -> &'static str {
        match self {
            Self::Digital { .. } => "digital",
            Self::Real(_) => "real",
            Self::Bus(_) => "digital bus",
        }
    }
}

#[derive(Clone)]
struct EventRow<'a> {
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
    const fn exact(&self) -> bool {
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

fn event_value(waveform: &WaveformData, value: f64) -> Option<EventValue> {
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

fn real_event_values_are_valid(values: &[f64]) -> bool {
    let Some(first_value) = values.iter().position(|value| value.is_finite()) else {
        return false;
    };
    values[..first_value].iter().all(|value| value.is_nan())
        && values[first_value..].iter().all(|value| value.is_finite())
}

fn waveform_is_event(waveform: &WaveformData) -> bool {
    let Some((_, digital)) = event_signal_name(&waveform.name) else {
        return false;
    };
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
        real_event_values_are_valid(&waveform.y)
    }
}

/// Whether this analysis has an event schedule worth a sheet.
///
/// A live partial qualifies alongside a completed run: the events it carries
/// are the ones the engine has already committed, arriving as the run accepts
/// them, and the manifest states the run is still going. Withholding the sheet
/// until the run ends would hide evidence the result model already holds,
/// which is exactly what the waveform sheets stopped doing when live partial
/// analog landed.
fn analysis_is_renderable(analysis: &AnalysisResult) -> bool {
    (analysis.success || analysis.is_live_partial())
        && analysis.analysis_type == AnalysisType::Transient
        && (matches!(
            analysis.result_payload.as_ref(),
            Some(AnalysisResultPayload::TransientEvents {
                digital_traces,
                real_traces,
                ..
            }) if !digital_traces.is_empty() || !real_traces.is_empty()
        ) || analysis.waveforms.iter().any(waveform_is_event))
}

/// Whether the tab strip offers an events sheet for the active analysis.
///
/// The validity half resolves the workspace memo rather than peeking into it.
/// Peeking and treating a miss as a pass meant evidence nobody had validated
/// yet — which is every analysis on the frame it first appears — was offered
/// whatever validation would have said about it, while the SOA, Smith and
/// Optimization gates beside it were failing closed on the same question.
/// Resolving it also fills the memo, so the walk happens once per dataset
/// generation rather than once per gate.
pub(super) fn active_analysis_is_renderable(state: &AppState) -> bool {
    let Some(run) = state.simulation.active_run() else {
        return false;
    };
    let Some(analysis) = state.simulation.active_analysis() else {
        return false;
    };
    analysis_is_renderable(analysis)
        && super::analysis_evidence_is_valid(state, run.dataset_id, analysis)
}

/// Reassemble every declared bus over the traces it names.
///
/// The word at each event comes from `rspice_core::execution::bus_events`,
/// which is the only reassembly in the product. A declaration whose members
/// are more than one reassembly holds at once is refused by that function, in
/// its own words, and the refusal is carried here so the sheet can state it
/// where the rows would have been rather than showing an empty bus.
fn build_bus_timelines(
    digital_traces: &[crate::state::DigitalEventTraceEvidence],
    digital_buses: &[crate::state::DigitalBusEvidence],
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

fn build_event_order(
    analysis: &AnalysisResult,
    analysis_key: AnalysisPresentationKey,
    data_version: u64,
    expanded: &std::collections::BTreeSet<String>,
) -> EventOrderCache {
    if let Some(AnalysisResultPayload::TransientEvents {
        digital_traces,
        real_traces,
        digital_buses,
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
        return EventOrderCache {
            analysis: analysis_key,
            data_version,
            exact: true,
            rows,
            buses,
            expanded: expanded.clone(),
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
        .filter(|(_, waveform)| waveform_is_event(waveform))
    {
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
    EventOrderCache {
        analysis: analysis_key,
        data_version,
        exact: false,
        rows,
        buses: Vec::new(),
        expanded: expanded.clone(),
    }
}

fn sort_event_order(
    analysis: &AnalysisResult,
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

fn event_entry_signal_name<'a>(
    analysis: &'a AnalysisResult,
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
            .and_then(|waveform| event_signal_name(&waveform.name).map(|(name, _)| name))
            .unwrap_or(""),
        EventSelectionSource::Bus => buses
            .get(entry.trace_index)
            .map_or("", |bus| bus.label.as_str()),
    }
}

fn event_row_from_entry<'a>(
    analysis: &'a AnalysisResult,
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
            let waveform = analysis.waveforms.get(entry.trace_index)?;
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

/// The whole sheet as rows, for the tests that read it without drawing.
#[cfg(test)]
fn event_rows_with(
    analysis: &AnalysisResult,
    radix: BusRadix,
    expanded: &std::collections::BTreeSet<String>,
) -> (EventOrderCache, Vec<String>) {
    let cache = build_event_order(
        analysis,
        AnalysisPresentationKey::new(crate::product::DatasetId::new(), analysis),
        0,
        expanded,
    );
    let rows = cache
        .rows
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(index, entry)| {
            event_row_from_entry(analysis, &cache.buses, radix, entry, index + 1).map(|row| {
                format!(
                    "{} {} {}",
                    row.signal_name,
                    row.value.display(),
                    row.value.domain()
                )
            })
        })
        .collect();
    (cache, rows)
}

/// The scalar rows of a history, for the tests that read one without drawing.
///
/// No declaration is resolved here — a bus row needs the reassembled table,
/// which `event_rows_with` returns beside its rows. A bussed history read
/// through this helper reports its member rows only, which is what every
/// caller of it is asking about.
#[cfg(test)]
fn event_rows(analysis: &AnalysisResult) -> Vec<EventRow<'_>> {
    build_event_order(
        analysis,
        AnalysisPresentationKey::new(crate::product::DatasetId::new(), analysis),
        0,
        &std::collections::BTreeSet::new(),
    )
    .rows
    .iter()
    .copied()
    .enumerate()
    .filter_map(|(index, entry)| {
        event_row_from_entry(analysis, &[], BusRadix::Binary, entry, index + 1)
    })
    .collect()
}

fn event_row_for_selection<'a>(
    analysis: &'a AnalysisResult,
    buses: &'a [BusTimeline],
    radix: BusRadix,
    selection: &DigitalEventSelection,
) -> Option<EventRow<'a>> {
    let entry = match selection.source {
        EventSelectionSource::ExactDigital => {
            let AnalysisResultPayload::TransientEvents { digital_traces, .. } =
                analysis.result_payload.as_ref()?
            else {
                return None;
            };
            let (trace_index, trace) = digital_traces
                .iter()
                .enumerate()
                .find(|(_, trace)| trace.node_name == selection.trace_name)?;
            let point = trace.points.get(selection.point_index)?;
            EventOrderEntry {
                source: selection.source,
                trace_index,
                point_index: selection.point_index,
                time_s: point.time_s,
                initial: selection.point_index == 0,
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
                .find(|(_, trace)| trace.node_name == selection.trace_name)?;
            let point = trace.points.get(selection.point_index)?;
            EventOrderEntry {
                source: selection.source,
                trace_index,
                point_index: selection.point_index,
                time_s: point.time_s,
                initial: selection.point_index == 0,
            }
        }
        EventSelectionSource::ProjectedDigital | EventSelectionSource::ProjectedReal => {
            let (trace_index, waveform) = analysis
                .waveforms
                .iter()
                .enumerate()
                .find(|(_, waveform)| waveform.name == selection.trace_name)?;
            if !waveform_is_event(waveform) {
                return None;
            }
            let (&time_s, &raw_value) = waveform
                .x
                .get(selection.point_index)
                .zip(waveform.y.get(selection.point_index))?;
            let current = event_value(waveform, raw_value)?;
            let expected_source = if matches!(current, EventValue::Digital { .. }) {
                EventSelectionSource::ProjectedDigital
            } else {
                EventSelectionSource::ProjectedReal
            };
            if selection.source != expected_source {
                return None;
            }
            let previous = waveform.y[..selection.point_index]
                .iter()
                .filter_map(|value| event_value(waveform, *value))
                .next_back()
                .map(|value| value.identity());
            if previous == Some(current.identity()) {
                return None;
            }
            EventOrderEntry {
                source: selection.source,
                trace_index,
                point_index: selection.point_index,
                time_s,
                initial: previous.is_none(),
            }
        }
        EventSelectionSource::Bus => {
            let (bus_index, bus) = buses
                .iter()
                .enumerate()
                .find(|(_, bus)| bus.name == selection.trace_name)?;
            let (time_s, _) = bus.events.get(selection.point_index)?;
            EventOrderEntry {
                source: selection.source,
                trace_index: bus_index,
                point_index: selection.point_index,
                time_s: *time_s,
                initial: selection.point_index == 0,
            }
        }
    };
    event_row_from_entry(analysis, buses, radix, entry, 0)
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
    let Some((analysis_key, structurally_renderable)) =
        state.simulation.active_run().and_then(|run| {
            state.simulation.active_analysis().map(|analysis| {
                (
                    AnalysisPresentationKey::new(run.dataset_id, analysis),
                    analysis_is_renderable(analysis),
                )
            })
        })
    else {
        well_hint(ui, "Select a dataset with retained event traces");
        return;
    };
    if !structurally_renderable {
        well_hint(
            ui,
            "The active analysis has no valid retained XSPICE event traces",
        );
        return;
    }
    if !super::retained_evidence_is_valid(state, analysis_key) {
        well_hint(ui, "The retained XSPICE event evidence is invalid");
        return;
    }
    let analysis = state
        .simulation
        .active_analysis()
        .expect("active analysis was resolved above");

    let data_version = state.simulation.data_version;
    let expanded = state.ui.results.expanded_event_buses.clone();
    let radix = state.ui.results.event_bus_radix;
    if state.ui.results.event_order_cache.as_ref().is_none_or(|c| {
        c.analysis != analysis_key || c.data_version != data_version || c.expanded != expanded
    }) {
        state.ui.results.event_order_cache = Some(build_event_order(
            analysis,
            analysis_key,
            data_version,
            &expanded,
        ));
    }
    let cache = state
        .ui
        .results
        .event_order_cache
        .as_ref()
        .expect("event order cache was initialized above");
    let exact = cache.exact;
    StripHeader::new(
        "EVENTS",
        &format!(
            "{} · {} {}{}",
            analysis.label,
            cache.rows.len(),
            if exact {
                "committed events"
            } else {
                "projected changes"
            },
            bus_subtitle(&cache.buses),
        ),
        &[],
    )
    .show(ui);
    if !exact {
        panel_note(
            ui,
            "Legacy accepted-sample projection. Re-run this analysis to retain exact sparse event timestamps.",
        );
    }
    for note in bus_notes(&cache.buses, radix) {
        panel_note(ui, &note);
    }

    let selected = state.ui.results.selected_digital_event.clone();
    let mut requested = None;
    let mut toggled_bus = None;
    let min_width = 908.0_f32.max(ui.available_width());
    egui::ScrollArea::horizontal()
        .id_salt("rspice.results.events-horizontal")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.set_min_width(min_width);
            TableBuilder::new(ui)
                .id_salt("rspice.results.events")
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                // Every column clips. `egui_extras` grows a cell that does not
                // fit and pushes the rest of its row along with it, so one
                // long value used to shift four columns on that row alone and
                // the table stopped being a table wherever the numbers were
                // widest. The time column is sized so its own widest content
                // — a `{:.17e}` second with a three-digit exponent — never
                // reaches the clip.
                .column(Column::initial(84.0).clip(true))
                .column(Column::initial(176.0).clip(true))
                .column(Column::remainder().at_least(190.0).clip(true))
                .column(Column::initial(152.0).clip(true))
                .column(Column::initial(128.0).clip(true))
                .column(Column::initial(112.0).clip(true))
                .header(HEADER_HEIGHT, |mut header| {
                    for label in [
                        "EVENT",
                        "PHYSICAL TIME",
                        "SIGNAL",
                        "VALUE",
                        "DOMAIN",
                        "KIND",
                    ] {
                        header.col(|ui| table_header(ui, label));
                    }
                })
                .body(|body| {
                    body.rows(ROW_HEIGHT, cache.rows.len(), |mut row| {
                        let row_index = row.index();
                        let Some(row_data) = cache.rows.get(row_index).copied().and_then(|entry| {
                            event_row_from_entry(
                                analysis,
                                &cache.buses,
                                radix,
                                entry,
                                row_index + 1,
                            )
                        }) else {
                            return;
                        };
                        let is_selected = selected.as_ref().is_some_and(|selection| {
                            selection.analysis == analysis_key
                                && selection.source == row_data.source
                                && selection.trace_name == row_data.trace_name
                                && selection.point_index == row_data.point_index
                        });
                        row.set_selected(is_selected);
                        row.col(|ui| {
                            if ui
                                .selectable_label(
                                    is_selected,
                                    format!("#{:04}", row_data.event_ordinal),
                                )
                                .clicked()
                            {
                                requested = Some(DigitalEventSelection {
                                    analysis: analysis_key,
                                    source: row_data.source,
                                    trace_name: row_data.trace_name.to_owned(),
                                    point_index: row_data.point_index,
                                });
                            }
                        });
                        row.col(|ui| {
                            mono(ui, &format!("{:.17e} s", row_data.time_s));
                        });
                        row.col(|ui| {
                            mono(ui, row_data.signal_name);
                            // The disclosure rides the bus row rather than a
                            // seventh column: it belongs to the one row it
                            // acts on, and a column that is blank on every
                            // scalar row would cost the table width to say
                            // nothing about them.
                            if row_data.source == EventSelectionSource::Bus {
                                let open = expanded.contains(row_data.trace_name);
                                let response = chip(ui, "BITS", open);
                                // The chip reads `BITS` on every bus row, so
                                // its published name says which bus it opens
                                // — two declarations on screen are otherwise
                                // the same word to anything not looking at
                                // the column beside it.
                                let name = format!("Bits of {}", row_data.signal_name);
                                response.widget_info(|| {
                                    egui::WidgetInfo::selected(
                                        egui::WidgetType::SelectableLabel,
                                        ui.is_enabled(),
                                        open,
                                        &name,
                                    )
                                });
                                let response = response.on_hover_text(if open {
                                    "Hide the member rows this word is reassembled from"
                                } else {
                                    "Show the member rows this word is reassembled from"
                                });
                                if response.clicked() {
                                    toggled_bus = Some(row_data.trace_name.to_owned());
                                }
                            }
                        });
                        row.col(|ui| {
                            let text = row_data.value.display();
                            let response = mono(ui, &text);
                            // A word is as wide as the bus is: the column
                            // clips rather than shoving the table along, so
                            // the whole word has to be reachable without
                            // selecting the row.
                            if row_data.source == EventSelectionSource::Bus {
                                response.on_hover_text(text);
                            }
                        });
                        row.col(|ui| {
                            ui.label(row_data.value.domain());
                        });
                        row.col(|ui| {
                            ui.label(if row_data.initial {
                                "initial"
                            } else {
                                "change"
                            });
                        });
                    });
                });
        });
    if let Some(selection) = requested {
        state.ui.results.selected_digital_event = Some(selection);
    }
    if let Some(bus) = toggled_bus
        && !state.ui.results.expanded_event_buses.remove(&bus)
    {
        state.ui.results.expanded_event_buses.insert(bus);
    }
}

/// The bus half of the sheet's subtitle, or nothing when none is declared.
///
/// A sheet with no declaration says nothing about buses at all: a "0 buses"
/// count would suggest the run might have had one, and no run without a
/// vector port or an imported vector ever can.
fn bus_subtitle(buses: &[BusTimeline]) -> String {
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
fn bus_notes(buses: &[BusTimeline], radix: BusRadix) -> Vec<String> {
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

/// The Radix control, in the sheet's own domain bar.
///
/// One setting for the sheet rather than one per bus: a radix is how the
/// reader is reading, not a property of any one declaration, and the sheets
/// beside this one keep their domain controls the same way.
pub(super) fn domain_bar(ui: &mut Ui, context: &mut SheetContext<'_>) -> bool {
    let declares_bus = context
        .simulation
        .active_analysis()
        .and_then(|analysis| analysis.result_payload.as_ref())
        .is_some_and(|payload| {
            matches!(
                payload,
                AnalysisResultPayload::TransientEvents { digital_buses, .. }
                    if !digital_buses.is_empty()
            )
        });
    if !declares_bus {
        return false;
    }
    let mut index = context.results.event_bus_radix.index();
    if segmented(
        ui,
        "rspice.results.events.radix",
        &BusRadix::OPTIONS,
        &mut index,
        SegmentedWidth::Natural,
    ) {
        context.results.event_bus_radix = BusRadix::from_index(index);
    }
    true
}

/// Why the event inspector has nothing to show for a selection that exists.
const EVENT_SELECTION_NO_DATASET: &str = "No dataset is open, so the selected event has nothing to resolve against. \
     Open the dataset it was taken from, or select an event row here.";
const EVENT_SELECTION_UNRETAINED_ANALYSIS: &str = "The analysis this event was selected from is no longer retained in the \
     active dataset. Select an event row again.";
const EVENT_SELECTION_OTHER_ANALYSIS: &str = "Select an event row in the active analysis.";
const EVENT_SELECTION_UNRETAINED_ROW: &str = "The retained trace or sample this event named is no longer present in the \
     analysis. Select an event row again.";

/// Why the inspector cannot show the event a selection names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EventSelectionBlock {
    note: &'static str,
    /// Whether the selection itself is unrecoverable and should be dropped.
    /// A dataset that is merely not open right now is not: it can come back,
    /// and dropping the selection would lose the reader's place for a
    /// condition that is about the workspace rather than about the evidence.
    stale: bool,
}

/// A selection outlives the evidence it names: closing the dataset, or
/// re-running the analysis into a different retained shape, leaves the panel
/// holding an identity nothing resolves. Three of those four outcomes drew an
/// empty panel — no heading, no reason — which reads as a broken pane rather
/// than as a selection that has expired.
fn event_selection_block(
    state: &AppState,
    selection: &DigitalEventSelection,
) -> Option<EventSelectionBlock> {
    let block = |note, stale| Some(EventSelectionBlock { note, stale });
    let Some(run) = state.simulation.active_run() else {
        return block(EVENT_SELECTION_NO_DATASET, false);
    };
    let Some((analysis_index, analysis)) = selection.analysis.resolve(run) else {
        return block(EVENT_SELECTION_UNRETAINED_ANALYSIS, true);
    };
    if state.simulation.active_analysis_idx != Some(analysis_index) {
        return block(EVENT_SELECTION_OTHER_ANALYSIS, true);
    }
    let radix = state.ui.results.event_bus_radix;
    if event_row_for_selection(analysis, selected_buses(state, selection), radix, selection)
        .is_none()
    {
        return block(EVENT_SELECTION_UNRETAINED_ROW, true);
    }
    None
}

/// The reassembled buses of the analysis a selection names, when the sheet's
/// cache holds them.
///
/// The inspector never rebuilds them: a bus is only selectable from the sheet,
/// which builds the cache before it draws a row, so a `Bus` selection whose
/// cache has been replaced by another analysis' resolves to nothing and the
/// panel says the row has expired — which is what happened.
fn selected_buses<'a>(state: &'a AppState, selection: &DigitalEventSelection) -> &'a [BusTimeline] {
    state
        .ui
        .results
        .event_order_cache
        .as_ref()
        .filter(|cache| cache.analysis == selection.analysis)
        .map_or(&[][..], |cache| cache.buses.as_slice())
}

pub fn right_panel(ui: &mut Ui, state: &mut AppState) {
    let Some(selection) = state.ui.results.selected_digital_event.clone() else {
        section_header(ui, "Event selection", None);
        panel_note(
            ui,
            "Select an event row to inspect its retained value change.",
        );
        return;
    };
    if let Some(block) = event_selection_block(state, &selection) {
        if block.stale {
            state.ui.results.selected_digital_event = None;
        }
        section_header(ui, "Event selection", None);
        panel_note(ui, block.note);
        return;
    }
    let radix = state.ui.results.event_bus_radix;
    let buses = selected_buses(state, &selection);
    let Some(event) = state
        .simulation
        .active_run()
        .and_then(|run| selection.analysis.resolve(run))
        .and_then(|(_, analysis)| event_row_for_selection(analysis, buses, radix, &selection))
    else {
        // Unreachable: the block above resolved this exact row over the same
        // unchanged state. Stated rather than asserted, because a panel that
        // says nothing is the defect this function is fixing.
        section_header(ui, "Event selection", None);
        panel_note(ui, EVENT_SELECTION_UNRETAINED_ROW);
        return;
    };

    section_header(
        ui,
        "Selected event",
        Some(if event.exact() { "EXACT" } else { "PROJECTED" }),
    );
    let mut stats = vec![
        ("Signal", event.signal_name.to_owned(), true),
        ("Physical time", format!("{:.17e} s", event.time_s), true),
        (
            if event.exact() {
                "Trace event"
            } else {
                "Source sample"
            },
            format!("#{}", selection.point_index + 1),
            false,
        ),
        ("Value", event.value.display(), true),
        ("Domain", event.value.domain().to_owned(), false),
    ];
    let mut bus_fallback = None;
    match &event.value {
        EventValue::Digital { code, .. } => {
            stats.push(("Retained code", code.to_string(), false));
            stats.push(("Drive strength", event.value.strength().to_owned(), false));
        }
        EventValue::Bus(word) => {
            stats.push(("Radix", radix.label().to_owned(), false));
            if let Some(bus) = buses.iter().find(|bus| bus.name == selection.trace_name) {
                stats.push(("Members", bus.members.len().to_string(), false));
                stats.push(("Bit order", bus.members.join(" "), false));
            }
            bus_fallback = word.fallback;
        }
        EventValue::Real(_) => {}
    }
    stat_table(ui, &stats);
    if let Some(reason) = bus_fallback {
        panel_note(ui, &format!("Not shown in {}: {reason}.", radix.label()));
    }
    panel_note(
        ui,
        match event.source {
            EventSelectionSource::Bus => {
                "This word is reassembled from the member histories beside it, at the exact time one of them changed. Every member keeps its own retained event code; the word holds no value they do not."
            }
            _ if event.exact() => {
                "This is an exact committed sparse event. Same-time transitions are preserved in per-trace order; cross-node delta-cycle ordering is not retained by the engine result contract."
            }
            _ => {
                "This row was reconstructed from a legacy accepted-sample projection. Its original sparse event timestamp is unavailable; re-run the analysis for exact evidence."
            }
        },
    );
}

fn table_header(ui: &mut Ui, label: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(label)
            .font(theme::mono(tokens::FS_0, FontWeight::SemiBold))
            .color(t.color.text_faint),
    );
}

fn mono(ui: &mut Ui, text: &str) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    ui.label(
        RichText::new(text)
            .font(theme::mono(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn committed_events(digital: &[(f64, u8)]) -> AnalysisResultPayload {
        AnalysisResultPayload::TransientEvents {
            digital_traces: vec![crate::state::DigitalEventTraceEvidence {
                node_name: "clk".to_owned(),
                points: digital
                    .iter()
                    .map(
                        |(time_s, value_code)| crate::state::DigitalEventPointEvidence {
                            time_s: *time_s,
                            value_code: *value_code,
                        },
                    )
                    .collect(),
            }],
            real_traces: Vec::new(),
            digital_buses: Vec::new(),
        }
    }

    #[test]
    fn a_running_analysis_offers_the_events_it_has_already_committed() {
        let running = AnalysisResult::live_transient_partial(1, AnalysisType::Transient, "TRAN")
            .with_result_payload(committed_events(&[(0.0, 0), (1.0e-9, 1)]));
        assert!(
            running.is_live_partial(),
            "the fixture must be the provisional result the controller publishes"
        );
        assert!(
            analysis_is_renderable(&running),
            "a run still accepting points has committed events worth showing"
        );

        let failed = AnalysisResult::failed(1, AnalysisType::Transient, "TRAN", "converge")
            .with_result_payload(committed_events(&[(0.0, 0), (1.0e-9, 1)]));
        assert!(
            !analysis_is_renderable(&failed),
            "only a live partial joins successful results; a failure stays out"
        );
    }

    #[test]
    fn the_event_order_remembers_which_generation_of_the_evidence_it_merged() {
        let analysis = AnalysisResult::live_transient_partial(1, AnalysisType::Transient, "TRAN")
            .with_result_payload(committed_events(&[(0.0, 0), (1.0e-9, 1)]));
        let key = AnalysisPresentationKey::new(crate::product::DatasetId::new(), &analysis);
        let first = build_event_order(&analysis, key, 7, &std::collections::BTreeSet::new());

        let longer = AnalysisResult::live_transient_partial(1, AnalysisType::Transient, "TRAN")
            .with_result_payload(committed_events(&[(0.0, 0), (1.0e-9, 1), (2.0e-9, 0)]));
        let second = build_event_order(&longer, key, 8, &std::collections::BTreeSet::new());

        assert_eq!(first.data_version, 7);
        assert_eq!(second.data_version, 8);
        assert_eq!(first.rows.len(), 2);
        assert_eq!(
            second.rows.len(),
            3,
            "the same analysis identity carries a longer history one generation later"
        );
    }

    #[test]
    fn event_rows_keep_initial_value_and_only_projected_changes() {
        let waveform = WaveformData::new(
            "D(clk)",
            vec![0.0, 1.0, 2.0, 3.0],
            vec![0.0, 0.0, 1.0, 1.0],
            "#fff",
        );
        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![waveform]);
        let rows = event_rows(&analysis);
        assert_eq!(rows.len(), 2);
        assert!(rows[0].initial);
        assert_eq!(rows[1].point_index, 2);
        assert!(matches!(rows[1].value, EventValue::Digital { code: 1, .. }));
        assert!(!rows[0].exact());
    }

    #[test]
    fn real_event_rows_preserve_projected_value_changes() {
        let waveform = WaveformData::new(
            "E(control)",
            vec![0.0, 1.0, 2.0],
            vec![f64::NAN, 0.25, 0.5],
            "#fff",
        );
        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![waveform]);
        let rows = event_rows(&analysis);
        assert_eq!(rows.len(), 2);
        assert!(rows[0].initial);
        assert!(matches!(rows[1].value, EventValue::Real(value) if value == 0.5));
    }

    #[test]
    fn real_event_trace_rejects_an_unknown_after_the_first_committed_value() {
        let waveform = WaveformData::new(
            "E(control)",
            vec![0.0, 1.0, 2.0],
            vec![f64::NAN, 0.25, f64::NAN],
            "#fff",
        );
        assert!(!waveform_is_event(&waveform));
    }

    #[test]
    fn digital_code_contract_rejects_non_integral_values() {
        assert!(logic_code(12.0).is_some());
        assert!(logic_code(2.5).is_none());
        assert!(logic_code(13.0).is_none());
    }

    #[test]
    fn exact_event_rows_preserve_between_sample_and_same_time_transitions() {
        let payload = AnalysisResultPayload::TransientEvents {
            digital_traces: vec![crate::state::DigitalEventTraceEvidence {
                node_name: "clk".to_owned(),
                points: vec![
                    crate::state::DigitalEventPointEvidence {
                        time_s: 0.25,
                        value_code: 0,
                    },
                    crate::state::DigitalEventPointEvidence {
                        time_s: 0.5,
                        value_code: 1,
                    },
                    crate::state::DigitalEventPointEvidence {
                        time_s: 0.5,
                        value_code: 0,
                    },
                ],
            }],
            real_traces: Vec::new(),
            digital_buses: Vec::new(),
        };
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![WaveformData::new(
                "D(clk)",
                vec![0.0, 1.0],
                vec![2.0, 0.0],
                "#fff",
            )])
            .with_result_payload(payload);
        let rows = event_rows(&analysis);
        assert_eq!(rows.len(), 3);
        assert_eq!(rows[1].time_s.to_bits(), 0.5_f64.to_bits());
        assert_eq!(rows[2].time_s.to_bits(), 0.5_f64.to_bits());
        assert_eq!(rows[1].point_index, 1);
        assert_eq!(rows[2].point_index, 2);
        assert!(rows.iter().all(EventRow::exact));
    }

    #[test]
    fn exact_event_order_is_deterministic_without_claiming_cross_node_delta_order() {
        let payload = AnalysisResultPayload::TransientEvents {
            digital_traces: vec![
                crate::state::DigitalEventTraceEvidence {
                    node_name: "a".to_owned(),
                    points: vec![
                        crate::state::DigitalEventPointEvidence {
                            time_s: 0.5,
                            value_code: 0,
                        },
                        crate::state::DigitalEventPointEvidence {
                            time_s: 0.5,
                            value_code: 1,
                        },
                    ],
                },
                crate::state::DigitalEventTraceEvidence {
                    node_name: "z".to_owned(),
                    points: vec![crate::state::DigitalEventPointEvidence {
                        time_s: 0.5,
                        value_code: 1,
                    }],
                },
            ],
            real_traces: Vec::new(),
            digital_buses: Vec::new(),
        };
        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_result_payload(payload);
        let rows = event_rows(&analysis);

        assert_eq!(
            rows.iter().map(|row| row.signal_name).collect::<Vec<_>>(),
            ["a", "a", "z"]
        );
        assert_eq!(rows[0].point_index, 0);
        assert_eq!(rows[1].point_index, 1);
    }

    #[test]
    fn selection_resolution_rejects_a_projected_non_change_sample() {
        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("D(clk)", vec![0.0, 1.0, 2.0], vec![0.0, 0.0, 1.0], "#fff"),
            ]);
        let analysis_key =
            AnalysisPresentationKey::new(crate::product::DatasetId::new(), &analysis);
        let stale = DigitalEventSelection {
            analysis: analysis_key,
            source: EventSelectionSource::ProjectedDigital,
            trace_name: "D(clk)".to_owned(),
            point_index: 1,
        };
        let changed = DigitalEventSelection {
            point_index: 2,
            ..stale.clone()
        };

        assert!(event_row_for_selection(&analysis, &[], BusRadix::Binary, &stale).is_none());
        assert_eq!(
            event_row_for_selection(&analysis, &[], BusRadix::Binary, &changed)
                .expect("changed projected sample")
                .point_index,
            2
        );
    }

    /// Every way the inspector can fail to resolve a selection says which one
    /// it was, and drops the selection only when the evidence is gone for
    /// good. Three of these four returned silently, drawing a panel with no
    /// heading and no text at all.
    #[test]
    fn an_unresolvable_event_selection_is_explained_rather_than_drawn_empty() {
        use crate::state::SimulationRun;

        let analysis =
            AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
                WaveformData::new("D(clk)", vec![0.0, 1.0, 2.0], vec![0.0, 0.0, 1.0], "#fff"),
            ]);
        let other = AnalysisResult::new(2, AnalysisType::Transient, "TRAN2").with_waveforms(vec![
            WaveformData::new("D(clk)", vec![0.0, 1.0, 2.0], vec![0.0, 0.0, 1.0], "#fff"),
        ]);
        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis.clone());
        run.add_analysis(other);
        let dataset_id = run.dataset_id;

        let mut state = AppState::default();

        // No dataset at all: explained, and the selection is kept because the
        // dataset can come back.
        let selection = DigitalEventSelection {
            analysis: AnalysisPresentationKey::new(dataset_id, &analysis),
            source: EventSelectionSource::ProjectedDigital,
            trace_name: "D(clk)".to_owned(),
            point_index: 2,
        };
        assert_eq!(
            event_selection_block(&state, &selection),
            Some(EventSelectionBlock {
                note: EVENT_SELECTION_NO_DATASET,
                stale: false,
            })
        );

        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));
        assert!(state.simulation.select_analysis(0));

        // The row resolves: nothing blocks the inspector.
        assert_eq!(event_selection_block(&state, &selection), None);

        // A selection naming an analysis this dataset never retained.
        let unretained = DigitalEventSelection {
            analysis: AnalysisPresentationKey::new(
                dataset_id,
                &AnalysisResult::new(99, AnalysisType::Transient, "GONE"),
            ),
            ..selection.clone()
        };
        assert_eq!(
            event_selection_block(&state, &unretained),
            Some(EventSelectionBlock {
                note: EVENT_SELECTION_UNRETAINED_ANALYSIS,
                stale: true,
            })
        );

        // A selection whose trace no longer carries that sample index.
        let past_the_end = DigitalEventSelection {
            point_index: 97,
            ..selection.clone()
        };
        assert_eq!(
            event_selection_block(&state, &past_the_end),
            Some(EventSelectionBlock {
                note: EVENT_SELECTION_UNRETAINED_ROW,
                stale: true,
            })
        );

        // A selection on a retained analysis that is not the active one.
        assert!(state.simulation.select_analysis(1));
        assert_eq!(
            event_selection_block(&state, &selection),
            Some(EventSelectionBlock {
                note: EVENT_SELECTION_OTHER_ANALYSIS,
                stale: true,
            })
        );
    }
}

#[cfg(test)]
mod availability_tests {
    use super::*;
    use crate::state::SimulationRun;

    /// Availability fails closed on evidence nobody has validated yet.
    ///
    /// The gate read the workspace validity memo and treated a miss as a
    /// pass, so an analysis whose retained evidence has never been checked —
    /// which is every analysis on the frame it first appears — was offered
    /// regardless of what validation would have said. Its siblings (SOA,
    /// Smith, Optimization) resolve the memo instead of peeking at it, and
    /// therefore close.
    #[test]
    fn never_validated_evidence_is_not_offered_as_an_events_sheet() {
        let payload = AnalysisResultPayload::TransientEvents {
            digital_traces: vec![crate::state::DigitalEventTraceEvidence {
                node_name: "clk".to_owned(),
                points: vec![crate::state::DigitalEventPointEvidence {
                    time_s: 0.25,
                    value_code: 0,
                }],
            }],
            real_traces: Vec::new(),
            digital_buses: Vec::new(),
        };
        // A waveform with more coordinates than values is exactly what
        // `validate_retained_evidence` exists to refuse.
        let mut corrupt = WaveformData::new("V(out)", vec![0.0, 1.0], vec![0.0, 1.0], "#fff");
        corrupt.y = std::sync::Arc::new(vec![0.0]);
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![corrupt])
            .with_result_payload(payload);
        assert!(
            analysis.validate_retained_evidence().is_err(),
            "the fixture has to be invalid for the gate to have anything to refuse"
        );
        assert!(
            analysis_is_renderable(&analysis),
            "the fixture has to be renderable so only validity can close the gate"
        );

        let mut run = SimulationRun::new(1);
        run.add_analysis(analysis);
        let mut state = AppState::default();
        state.simulation.runs = vec![run];
        assert!(state.simulation.select_run(0));
        state
            .ui
            .results
            .retained_evidence_validity
            .borrow_mut()
            .clear();

        assert!(
            !active_analysis_is_renderable(&state),
            "an unvalidated, invalid analysis was offered its sheet"
        );
    }
}

#[cfg(test)]
mod bus_tests {
    use super::*;
    use crate::state::{
        DigitalBusEvidence, DigitalBusSourceEvidence, DigitalEventPointEvidence,
        DigitalEventTraceEvidence,
    };

    /// BUS-L2's counter: `count[1:0]` over `count#1` and `count#0`, counting
    /// `00 01 10 11` on a 5 ns tick — the history that lane pinned end to end.
    fn counter(members: &[(&str, &[(f64, u8)])], msb: i64, lsb: i64) -> AnalysisResult {
        let digital_traces = members
            .iter()
            .map(|(name, points)| DigitalEventTraceEvidence {
                node_name: (*name).to_owned(),
                points: points
                    .iter()
                    .map(|(time_s, value_code)| DigitalEventPointEvidence {
                        time_s: *time_s,
                        value_code: *value_code,
                    })
                    .collect(),
            })
            .collect();
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_result_payload(
            AnalysisResultPayload::TransientEvents {
                digital_traces,
                real_traces: Vec::new(),
                digital_buses: vec![DigitalBusEvidence {
                    name: "count".to_owned(),
                    msb,
                    lsb,
                    members: members.iter().map(|(name, _)| (*name).to_owned()).collect(),
                    source: DigitalBusSourceEvidence::Engine,
                }],
            },
        )
    }

    fn two_bit_counter() -> AnalysisResult {
        counter(
            &[
                ("count#1", &[(0.0, 0), (10.0e-9, 1)]),
                (
                    "count#0",
                    &[(0.0, 0), (5.0e-9, 1), (10.0e-9, 0), (15.0e-9, 1)],
                ),
            ],
            1,
            0,
        )
    }

    fn collapsed() -> std::collections::BTreeSet<String> {
        std::collections::BTreeSet::new()
    }

    #[test]
    fn a_declared_bus_replaces_its_member_rows_with_one_word_per_change() {
        let analysis = two_bit_counter();
        let (cache, rows) = event_rows_with(&analysis, BusRadix::Binary, &collapsed());
        assert_eq!(cache.buses.len(), 1);
        assert_eq!(
            rows,
            vec![
                "count[1:0] 00 digital bus".to_owned(),
                "count[1:0] 01 digital bus".to_owned(),
                "count[1:0] 10 digital bus".to_owned(),
                "count[1:0] 11 digital bus".to_owned(),
            ],
            "a collapsed bus lists its four words and none of its six member changes"
        );
    }

    #[test]
    fn expanding_a_bus_lists_its_members_beside_the_word() {
        let analysis = two_bit_counter();
        let expanded = std::collections::BTreeSet::from(["count".to_owned()]);
        let (_, rows) = event_rows_with(&analysis, BusRadix::Binary, &expanded);
        assert_eq!(
            rows.iter()
                .filter(|row| row.starts_with("count[1:0]"))
                .count(),
            4
        );
        assert_eq!(
            rows.iter().filter(|row| row.starts_with("count#")).count(),
            6,
            "every member change is listed once the bus is opened to its bits"
        );
    }

    #[test]
    fn each_radix_spells_the_same_word_and_says_when_it_cannot() {
        let analysis = two_bit_counter();
        let last = |radix| {
            event_rows_with(&analysis, radix, &collapsed())
                .1
                .last()
                .cloned()
                .expect("the counter has bus rows")
        };
        assert_eq!(last(BusRadix::Binary), "count[1:0] 11 digital bus");
        assert_eq!(last(BusRadix::Hex), "count[1:0] 0x3 digital bus");
        assert_eq!(last(BusRadix::Unsigned), "count[1:0] 3 digital bus");
        assert_eq!(
            last(BusRadix::Signed),
            "count[1:0] -1 digital bus",
            "two bits of ones is minus one, signed at the declared width"
        );

        // Code 12 is high impedance and code 2 is unknown: neither denotes a
        // digit, so every radix but binary falls back and says how many words.
        let unresolved = counter(
            &[
                ("count#1", &[(0.0, 0), (10.0e-9, 12)]),
                ("count#0", &[(0.0, 2)]),
            ],
            1,
            0,
        );
        let (cache, rows) = event_rows_with(&unresolved, BusRadix::Hex, &collapsed());
        assert_eq!(
            rows,
            vec![
                "count[1:0] 0x digital bus".to_owned(),
                "count[1:0] zx digital bus".to_owned(),
            ]
        );
        assert_eq!(
            bus_notes(&cache.buses, BusRadix::Hex),
            vec![format!(
                "2 bus words are not shown in hexadecimal: {UNRESOLVED_BITS_FALLBACK}."
            )]
        );
        assert!(
            bus_notes(&cache.buses, BusRadix::Binary).is_empty(),
            "binary spells every word a run can produce, so it never has a note"
        );
    }

    #[test]
    fn a_member_the_run_never_stated_reads_as_unknown_until_it_does() {
        // `count#1` has no point before 10 ns, so the run has not said what
        // the bit is — which VCD spells `x`, exactly as the dump does.
        let analysis = counter(
            &[
                ("count#1", &[(10.0e-9, 1)]),
                ("count#0", &[(0.0, 0), (10.0e-9, 1)]),
            ],
            1,
            0,
        );
        let (_, rows) = event_rows_with(&analysis, BusRadix::Binary, &collapsed());
        assert_eq!(
            rows,
            vec![
                "count[1:0] x0 digital bus".to_owned(),
                "count[1:0] 11 digital bus".to_owned(),
            ]
        );
    }

    #[test]
    fn a_result_with_no_declaration_paints_no_bus_row_and_no_bus_subtitle() {
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_result_payload(
            AnalysisResultPayload::TransientEvents {
                digital_traces: vec![DigitalEventTraceEvidence {
                    node_name: "clk".to_owned(),
                    points: vec![
                        DigitalEventPointEvidence {
                            time_s: 0.0,
                            value_code: 0,
                        },
                        DigitalEventPointEvidence {
                            time_s: 1.0e-9,
                            value_code: 1,
                        },
                    ],
                }],
                real_traces: Vec::new(),
                digital_buses: Vec::new(),
            },
        );
        let (cache, rows) = event_rows_with(&analysis, BusRadix::Hex, &collapsed());
        assert!(cache.buses.is_empty());
        assert!(bus_subtitle(&cache.buses).is_empty());
        assert!(bus_notes(&cache.buses, BusRadix::Hex).is_empty());
        assert!(rows.iter().all(|row| row.ends_with("digital")));
    }

    #[test]
    fn the_subtitle_counts_the_declarations_and_their_members() {
        let analysis = two_bit_counter();
        let (cache, _) = event_rows_with(&analysis, BusRadix::Binary, &collapsed());
        assert_eq!(bus_subtitle(&cache.buses), " · 1 bus over 2 members");
    }

    /// A word wider than a machine integer still has an exact hexadecimal and
    /// binary spelling; the two decimal radices say so rather than rounding.
    #[test]
    fn a_word_wider_than_a_machine_integer_falls_back_to_hexadecimal() {
        let ones = vec![Some(1_u8); 65];
        let word = bus_word(&ones, BusRadix::Unsigned);
        assert_eq!(word.fallback, Some(WIDE_WORD_FALLBACK));
        assert_eq!(word.text, "0x1ffffffffffffffff");
        assert_eq!(bus_word(&ones, BusRadix::Hex).fallback, None);
        assert_eq!(bus_word(&ones, BusRadix::Binary).text, "1".repeat(65));
    }

    /// The sheet's binary spelling is the dump's, character for character:
    /// both are `rspice_core::execution::event_code_to_vcd_bit`, so a word
    /// read here and the same word read out of an exported VCD agree.
    #[test]
    fn the_binary_word_is_the_spelling_the_dump_writes() {
        let codes = [Some(0_u8), Some(1), Some(2), Some(12), None];
        assert_eq!(bus_word(&codes, BusRadix::Binary).text, "01xzx");
    }
}

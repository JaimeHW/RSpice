//! Digital and real event histories, projected onto a VCD document.
//!
//! [`crate::execution::transient_projection_signals`] flattens an event
//! history onto the analog time grid, which is what a table export needs and
//! what a waveform viewer plots. This is the other projection: it keeps the
//! irregular timeline and the four-state value, which is what an event dump
//! is for.
//!
//! Two things are decided here rather than in [`crate::io::vcd`], because a
//! format module never names a result type: how a twelve-state
//! [`DigitalValue`] becomes one VCD bit, and which `$timescale` the times are
//! written at once each is rounded to the femtosecond.
//!
//! # Buses
//!
//! VCD is the one route whose grammar already had a place for a bus: a `$var`
//! declares a width, and a wide one takes `b…` value changes. A declared bus
//! is therefore written as one vector variable, its member scalars are *not*
//! written beside it, and the inverse reads a wide logic variable back as a
//! declaration plus one synthesized member trace per bit. Nothing is lost by
//! suppressing the members: a scalar `$var` carried no strength either, so the
//! dump says exactly what it said before, in one variable instead of N.

use thiserror::Error;

use crate::Value;
use crate::engine::{
    DigitalBusDeclaration, DigitalBusError, DigitalBusSource, DigitalTrace, DigitalTracePoint,
    MAX_DIGITAL_BUS_WIDTH, RealTrace, RealTracePoint, canonical_event_name,
    validate_digital_bus_table,
};
use crate::execution::event_bus::{
    BusMemberHistory, BusReassemblyTooLarge, bus_events, split_bus_notation,
};
use crate::io::vcd::{
    VCD_WRITER_VERSION, VcdBit, VcdChange, VcdDocument, VcdMagnitude, VcdSignal, VcdSignalKind,
    VcdTimeUnit, VcdTimescale, VcdValue, VcdVariable, is_writable_scope_name,
    is_writable_variable_name,
};
use crate::xspice::{DigitalState, DigitalStrength, DigitalValue};
use std::collections::BTreeSet;

/// Femtoseconds in one second: the finest tick VCD's `$timescale` can name.
const FEMTOSECONDS_PER_SECOND: f64 = 1e15;

/// The largest timestamp at VCD's coarsest (100 second) tick period.
/// Intermediate femtoseconds need more than 64 bits even for a one-day run.
const MAX_EVENT_FEMTOSECONDS: u128 = u64::MAX as u128 * VcdTimescale::ALL[0].femtoseconds() as u128;

/// Width a `real` variable conventionally declares.
const REAL_VARIABLE_WIDTH: u32 = 64;

/// An event history that cannot be written as VCD.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum EventProjectionError {
    /// The scope name would not survive the `$scope` grammar.
    #[error("'{scope}' cannot name a VCD scope")]
    Scope {
        /// The rejected scope name.
        scope: String,
    },

    /// A node name would not survive the `$var` grammar.
    #[error("node '{node}' cannot name a VCD variable")]
    NodeName {
        /// The rejected node name.
        node: String,
    },

    /// Two values one signal held at different recorded times would land on
    /// one femtosecond tick, where the dump could not say which came first.
    #[error(
        "node '{node}' changes at {time} s and at {other_time} s, which share femtosecond tick \
         {tick}; a VCD dump cannot order the two values"
    )]
    TimeCollision {
        /// The node whose event time was rejected.
        node: String,
        /// The rejected time, in seconds.
        time: Value,
        /// The different time already assigned to this tick, in seconds.
        other_time: Value,
        /// The conflicting position in whole femtoseconds.
        tick: u128,
    },

    /// A net is marked as observed by an HDL whose time precision is not a
    /// tick period a VCD `$timescale` can name.
    #[error(
        "node '{node}' is observed by an HDL with a time precision of {precision} s, which no VCD \
         timescale names"
    )]
    HdlPrecision {
        /// The node whose precision was rejected.
        node: String,
        /// The rejected precision, in seconds.
        precision: Value,
    },

    /// No one VCD timescale can retain both the event spacing and full duration.
    #[error("no VCD timescale can retain every event time within its 64-bit tick range")]
    TimescaleRange,

    /// The integer VCD time would lose precision in an event history.
    #[error(
        "variable '{variable}' at VCD tick {tick} cannot be represented in event-history \
         seconds without losing time precision"
    )]
    InexactTick {
        /// The VCD variable whose timestamp cannot be imported.
        variable: String,
        /// The rejected integer position.
        tick: u64,
    },

    /// The time is negative, not finite, or past the last tick addressable
    /// even at the coarsest VCD timescale.
    #[error("node '{node}' has an event at {time} s, which is outside the range VCD can address")]
    UnrepresentableTime {
        /// The node whose event time was rejected.
        node: String,
        /// The rejected time, in seconds.
        time: Value,
    },

    /// An event history ran backwards.
    #[error("node '{node}' has an event at {time} s after a later one")]
    UnorderedTime {
        /// The node whose event history was rejected.
        node: String,
        /// The time that arrived out of order, in seconds.
        time: Value,
    },

    /// A real event value was infinite or NaN.
    #[error("node '{node}' has a non-finite value at {time} s")]
    NonFiniteValue {
        /// The node whose value was rejected.
        node: String,
        /// The time of the rejected value, in seconds.
        time: Value,
    },

    /// A bus declaration is not well formed against the traces it names.
    #[error(transparent)]
    Bus(#[from] DigitalBusError),

    /// A bus is well formed but larger than one reassembly holds.
    #[error("digital bus '{bus}': {source}")]
    BusTooLarge {
        /// The bus whose reassembly was refused.
        bus: String,
        /// The budget that refused it.
        source: BusReassemblyTooLarge,
    },

    /// A wide `$var` declares a range whose width is not the variable's.
    #[error(
        "variable '{variable}' declares {width} bits but names the range [{msb}:{lsb}], which is {range} bit(s)"
    )]
    BusRangeWidth {
        /// The `$var` reference as declared.
        variable: String,
        /// Width the `$var` declared.
        width: u32,
        /// Most significant index the reference named.
        msb: i64,
        /// Least significant index the reference named.
        lsb: i64,
        /// Width that range describes.
        range: u64,
    },

    /// A value change carried a different number of bits than its variable is
    /// wide.
    #[error(
        "variable '{variable}' is {width} bits wide but a change at tick {tick} carries {found}"
    )]
    VectorWidth {
        /// The `$var` reference as declared.
        variable: String,
        /// Width the `$var` declared.
        width: u32,
        /// The tick the offending change sits at.
        tick: u64,
        /// Number of bits the change carried.
        found: usize,
    },

    /// A logic variable carried a real change, or a real one carried bits.
    #[error("variable '{variable}' carries a value of the wrong kind at tick {tick}")]
    VariableKind {
        /// The `$var` reference as declared.
        variable: String,
        /// The tick the offending change sits at.
        tick: u64,
    },

    /// Two variables reduce to one trace name.
    #[error("'{node}' is declared by more than one variable")]
    DuplicateNode {
        /// The name two variables both reduced to.
        node: String,
    },
}

/// The VCD bit a digital value shows as.
///
/// The level survives and the strength does not, because VCD has four bit
/// states and no drive strength:
///
/// | [`DigitalState`] | VCD |
/// |---|---|
/// | `Zero`, `ZeroR`, `ZeroZ` | `0` |
/// | `One`, `OneR`, `OneZ` | `1` |
/// | `Unknown`, `UnknownR`, `UnknownZ` | `x` |
/// | `HighZ` | `z` |
///
/// So a resistive one and a strong one are both `1` in the dump, and
/// [`DigitalValue::strength`] is dropped. A consumer that needs the band is
/// looking for [`DigitalValue::event_code`], not for a VCD file.
pub fn digital_value_to_vcd_bit(value: DigitalValue) -> VcdBit {
    match value.state {
        DigitalState::Zero | DigitalState::ZeroR | DigitalState::ZeroZ => VcdBit::Zero,
        DigitalState::One | DigitalState::OneR | DigitalState::OneZ => VcdBit::One,
        DigitalState::Unknown | DigitalState::UnknownR | DigitalState::UnknownZ => VcdBit::Unknown,
        DigitalState::HighZ => VcdBit::HighImpedance,
    }
}

/// The VCD bit one bus member shows as at one bus event.
///
/// The argument is what [`crate::execution::bus_events`] hands back for a
/// member: the `0..=12` event code it held, or `None` when the run had not
/// stated its value yet. Those two cases spell differently for different
/// reasons, and this is the one place either is decided:
///
/// - `None` — the member has no recorded point at or before the event, so the
///   run has not said what the bit is. VCD spells that `x`.
/// - `Some(code)` — the level of [`DigitalValue::from_event_code`], through
///   [`digital_value_to_vcd_bit`], so the strength drops exactly as it does
///   for a scalar.
///
/// The return is `None` only for a code that is not a digital event code at
/// all. That cannot come from a history this crate recorded — every code is
/// [`DigitalValue::event_code`] of a real value — so a caller reading a
/// document or a pickle it did not write refuses it in its own words rather
/// than spelling it as a value the run never held.
pub fn event_code_to_vcd_bit(code: Option<u8>) -> Option<VcdBit> {
    match code {
        None => Some(VcdBit::Unknown),
        Some(code) => DigitalValue::from_event_code(code).map(digital_value_to_vcd_bit),
    }
}

/// The digital value a VCD bit reads back as.
///
/// The inverse of [`digital_value_to_vcd_bit`] cannot recover a strength the
/// format never stored, so it returns the strongly driven member of each
/// level: `0` and `1` come back [`DigitalStrength::Strong`], `x` comes back a
/// strong [`DigitalState::Unknown`], and `z` comes back
/// [`DigitalValue::high_z`] — the only value in its level, since a strongly
/// driven high impedance is a contradiction.
pub fn digital_value_from_vcd_bit(bit: VcdBit) -> DigitalValue {
    match bit {
        VcdBit::Zero => DigitalValue::new(DigitalState::Zero, DigitalStrength::Strong),
        VcdBit::One => DigitalValue::new(DigitalState::One, DigitalStrength::Strong),
        VcdBit::Unknown => DigitalValue::new(DigitalState::Unknown, DigitalStrength::Strong),
        VcdBit::HighImpedance => DigitalValue::high_z(),
    }
}

/// Project accepted event histories onto a VCD document.
///
/// Every trace becomes one variable inside a single `$scope module <scope>`:
/// a digital trace as a one-bit `wire`, a real trace as a `real`. Values keep
/// their own irregular timeline — nothing is resampled onto the analog grid.
///
/// # Buses
///
/// A declared bus becomes one `wire` of its own width, named
/// `<name> [<msb>:<lsb>]` — the space form, which is what the reference field
/// of a `$var` holds and what the common Verilog dumpers write — with a `b…`
/// change at every time one of its members changes, bits most significant
/// first in declaration order. A member that has no recorded point yet is
/// written `x`, because at that time the run has not stated its value.
///
/// **The member scalars of a bus are not written.** Their content is in the
/// vector, bit for bit, and a scalar `$var` carried no strength either, so the
/// dump says exactly what it would have said in N variables instead of one.
/// [`vcd_event_histories`] reads them back.
///
/// Buses are declared first, in declaration order, then the digital traces
/// that belong to no bus, then the real traces. A projection with no bus is
/// byte-identical to what this produced before buses existed.
///
/// # Timescale
///
/// Times are seconds; VCD counts integer ticks. Each event time is rounded to
/// the nearest femtosecond, VCD's finest tick, and the `$timescale` chosen is
/// the coarsest whose period divides every one of them — so a run whose events
/// all land on nanoseconds is written `1 ns` with small tick numbers rather
/// than `1 fs` with large ones. An event an analog solution dated between two
/// femtoseconds (an A/D threshold crossing, nearly always) moves by less than
/// half of one. What rounding must not do is merge two different values one
/// signal held at different recorded times onto one tick, where the dump could
/// not say which came first: that is refused as
/// [`EventProjectionError::TimeCollision`]. Values recorded at one identical
/// time are delta changes and are kept in order, and different signals may
/// share a tick.
///
/// A net an HDL reads or drives ([`DigitalTrace::hdl_time_precision`]) is
/// dumped as that HDL saw it instead. Its trace keeps the physical instants
/// (an A/D output changes where the analog solution crossed the threshold),
/// but the HDL dated each change at its nearest tick, so each change rounds to
/// the nearest multiple of the design precision, and several changes on one
/// tick, the HDL's delta cycles, collapse to the last: the value the HDL
/// observed once they settled. Such a dump's `$timescale` is the design
/// precision (finer only if a net no HDL observes needs it), which is the tick
/// the HDL's own `$dumpvars` writes, so the two dumps line up.
/// The chosen period must also keep every tick within `u64`; otherwise the
/// combination of duration and event spacing is refused as
/// [`EventProjectionError::TimescaleRange`]. Intermediate femtoseconds use
/// `u128`, so long runs can use the full range of coarser tick periods.
///
/// With no positive event time there is nothing to choose between the scales,
/// and the finest is used.
pub fn event_vcd_document(
    scope: &str,
    digital_traces: &[DigitalTrace],
    real_traces: &[RealTrace],
    digital_buses: &[DigitalBusDeclaration],
) -> Result<VcdDocument, EventProjectionError> {
    if !is_writable_scope_name(scope) {
        return Err(EventProjectionError::Scope {
            scope: scope.to_string(),
        });
    }
    validate_digital_bus_table(
        digital_buses,
        digital_traces.iter().map(|trace| trace.node_name.as_str()),
    )?;

    let mut pending: Vec<PendingSignal> = Vec::new();
    let mut claimed: BTreeSet<String> = BTreeSet::new();
    for bus in digital_buses {
        pending.push(bus_signal(bus, digital_traces)?);
        claimed.extend(
            bus.members
                .iter()
                .map(|member| canonical_event_name(member)),
        );
    }
    for trace in digital_traces {
        // Every member is validated to belong to exactly one bus, so the
        // vector above already carries this trace bit for bit.
        if claimed.contains(&canonical_event_name(&trace.node_name)) {
            continue;
        }
        let grid = TimeGrid::of(&trace.node_name, trace.hdl_time_precision)?;
        let mut changes = GridChanges::new(grid);
        for point in &trace.points {
            changes.push(
                &trace.node_name,
                point.time,
                VcdValue::Logic(vec![digital_value_to_vcd_bit(point.value)]),
            )?;
        }
        pending.push(PendingSignal {
            name: checked_node_name(&trace.node_name)?,
            kind: VcdSignalKind::Logic,
            width: 1,
            points: changes.points,
            hdl_period: grid.hdl_period(),
        });
    }
    for trace in real_traces {
        let mut points = Vec::new();
        let mut times = SignalTimes::default();
        for point in &trace.points {
            let placed = times.place(&trace.node_name, point.time)?;
            if !point.value.is_finite() {
                return Err(EventProjectionError::NonFiniteValue {
                    node: trace.node_name.clone(),
                    time: point.time,
                });
            }
            push_change(
                &mut points,
                &trace.node_name,
                point.time,
                placed,
                VcdValue::Real(point.value),
            )?;
        }
        pending.push(PendingSignal {
            name: checked_node_name(&trace.node_name)?,
            kind: VcdSignalKind::Real,
            width: REAL_VARIABLE_WIDTH,
            points,
            hdl_period: None,
        });
    }

    let timescale = choose_timescale(
        pending.iter().flat_map(|signal| &signal.points),
        pending.iter().filter_map(|signal| signal.hdl_period).min(),
    )?;
    let period = u128::from(timescale.femtoseconds());
    let mut document = VcdDocument::new(timescale);
    document.version = VCD_WRITER_VERSION.to_string();
    document.signals = pending
        .into_iter()
        .map(|signal| VcdSignal {
            identifier: String::new(),
            variables: vec![VcdVariable {
                scope: vec![scope.to_string()],
                name: signal.name,
            }],
            width: signal.width,
            kind: signal.kind,
            changes: signal
                .points
                .into_iter()
                .map(|(femtoseconds, value)| VcdChange {
                    // The timescale admits every quotient to the u64 range.
                    tick: (femtoseconds / period) as u64,
                    value,
                })
                .collect(),
        })
        .collect();
    document.assign_canonical_identifiers();
    Ok(document)
}

/// One trace, converted but not yet placed on a tick grid.
struct PendingSignal {
    name: String,
    kind: VcdSignalKind,
    width: u32,
    points: Vec<(u128, VcdValue)>,
    /// The HDL tick, in femtoseconds, the points were rounded to, if an HDL
    /// observes the signal.
    hdl_period: Option<u128>,
}

/// The `$var` reference a bus is declared under: `name [msb:lsb]`.
///
/// The space form is what the reference field of a `$var` holds — the reader
/// joins the fields after the identifier with a single space — and it is what
/// the common Verilog dumpers write. [`split_bus_notation`] accepts the
/// closed-up form too, so a foreign dump written either way reads back.
fn bus_reference(bus: &DigitalBusDeclaration) -> String {
    format!("{} [{}:{}]", bus.name, bus.msb, bus.lsb)
}

/// Convert one declared bus into the vector variable that carries it.
fn bus_signal(
    bus: &DigitalBusDeclaration,
    digital_traces: &[DigitalTrace],
) -> Result<PendingSignal, EventProjectionError> {
    let reference = bus_reference(bus);
    let name = checked_node_name(&reference)?;

    // Each member's own times are validated under the member's own name, so a
    // time out of order or out of range is reported against the node that
    // recorded it rather than against the bus it was folded into.
    let mut histories: Vec<Vec<(Value, u8)>> = Vec::with_capacity(bus.members.len());
    // A word is on its HDL's tick only when an HDL observes every bit of it;
    // one vector variable cannot carry two grids.
    let mut hdl_precision = Some(Value::INFINITY);
    for member in &bus.members {
        let key = canonical_event_name(member);
        let trace = digital_traces
            .iter()
            .find(|trace| canonical_event_name(&trace.node_name) == key)
            .ok_or_else(|| DigitalBusError::UnknownMember {
                bus: bus.name.clone(),
                member: member.clone(),
            })?;
        let mut times = SignalTimes::default();
        for point in &trace.points {
            times.place(&trace.node_name, point.time)?;
        }
        hdl_precision = hdl_precision
            .zip(trace.hdl_time_precision)
            .map(|(finest, precision)| finest.min(precision));
        histories.push(
            trace
                .points
                .iter()
                .map(|point| (point.time, point.value.event_code()))
                .collect(),
        );
    }

    let members: Vec<BusMemberHistory<'_>> = histories
        .iter()
        .map(|points| BusMemberHistory { points })
        .collect();
    let grid = TimeGrid::of(
        &reference,
        hdl_precision.filter(|precision| precision.is_finite()),
    )?;
    let mut changes = GridChanges::new(grid);
    let events = bus_events(&members).map_err(|source| EventProjectionError::BusTooLarge {
        bus: bus.name.clone(),
        source,
    })?;
    for (time, codes) in events {
        let bits = codes
            .into_iter()
            // Every code here is `event_code` of a value read two statements
            // above, so `event_code_to_vcd_bit` cannot answer `None`; a member
            // with no point yet is `Some(VcdBit::Unknown)`, which is what the
            // run has said about it.
            .map(|code| event_code_to_vcd_bit(code).unwrap_or(VcdBit::Unknown))
            .collect();
        changes.push(&reference, time, VcdValue::Logic(bits))?;
    }

    Ok(PendingSignal {
        name,
        kind: VcdSignalKind::Logic,
        width: bus.width() as u32,
        points: changes.points,
        hdl_period: grid.hdl_period(),
    })
}

/// The name one bit of a bus is recorded under when a dump is read back.
///
/// `name[k]` is VCD's own bit-select reference grammar, so a member trace
/// synthesized here is spelled the way the same dump would have spelled that
/// bit had the writer declared it separately.
fn vcd_member_name(base: &str, index: i64) -> String {
    format!("{base}[{index}]")
}

/// The event histories one VCD document declares.
///
/// The inverse of [`event_vcd_document`], at the resolution VCD keeps: bit
/// states survive, drive strength does not, and every level comes back
/// strongly driven per [`digital_value_from_vcd_bit`].
#[derive(Debug, Clone, Default, PartialEq)]
pub struct VcdEventHistories {
    /// Scalar digital histories, including the members synthesized from every
    /// vector variable.
    pub digital_traces: Vec<DigitalTrace>,
    /// Buses declared by the vector variables, in declaration order.
    pub digital_buses: Vec<DigitalBusDeclaration>,
    /// Real-valued histories.
    pub real_traces: Vec<RealTrace>,
}

/// Read a VCD document back as event histories.
///
/// # Names
///
/// The scope path every variable shares is dropped and the rest of the path is
/// kept, so a dump this crate wrote — one scope, one variable per node —
/// returns the node names it started from, while a hierarchical foreign dump
/// keeps enough of its path to stay unambiguous. Two variables that reduce to
/// one name are refused rather than merged.
///
/// # Vectors
///
/// A logic variable wider than one bit is a bus: its reference supplies the
/// declared range when it carries one and `[width-1:0]` when it does not, and
/// each bit becomes a member trace named `name[k]` in declaration order. The
/// declaration is [`DigitalBusSource::Import`] — a dump is a claim by whatever
/// wrote it, not a fact about a run this build made.
///
/// # What it cannot recover
///
/// Drive strength, which VCD never stored, and the member names a producer
/// originally used: a vector says what its bits are, not what the nodes
/// carrying them were called. Member histories are change-compressed on the
/// way in, because that is the invariant a recorded history has.
/// Integer timestamps that cannot retain their tick when converted to the
/// histories' binary64 seconds are refused as [`EventProjectionError::InexactTick`].
pub fn vcd_event_histories(
    document: &VcdDocument,
) -> Result<VcdEventHistories, EventProjectionError> {
    let time_at = |variable: &str, tick| {
        document
            .timescale
            .seconds_at_tick(tick)
            .ok_or_else(|| EventProjectionError::InexactTick {
                variable: variable.to_owned(),
                tick,
            })
    };
    let shared = shared_scope_depth(&document.signals);
    let mut histories = VcdEventHistories::default();
    let mut seen: BTreeSet<String> = BTreeSet::new();

    for signal in &document.signals {
        let Some(variable) = signal.variables.first() else {
            continue;
        };
        let reference = trailing_scoped_name(variable, shared);
        match signal.kind {
            VcdSignalKind::Real => {
                let mut points = Vec::with_capacity(signal.changes.len());
                for change in &signal.changes {
                    let VcdValue::Real(value) = change.value else {
                        return Err(EventProjectionError::VariableKind {
                            variable: reference,
                            tick: change.tick,
                        });
                    };
                    points.push(RealTracePoint {
                        time: time_at(&reference, change.tick)?,
                        value,
                    });
                }
                claim_node(&mut seen, &reference)?;
                histories.real_traces.push(RealTrace {
                    node_name: reference,
                    points,
                });
            }
            VcdSignalKind::Logic if signal.width == 1 => {
                let mut points = Vec::with_capacity(signal.changes.len());
                for change in &signal.changes {
                    // The width check inside `logic_bits` fixes the count at
                    // one, so this loop runs exactly once per change.
                    for bit in logic_bits(&reference, signal.width, change)? {
                        points.push(DigitalTracePoint {
                            time: time_at(&reference, change.tick)?,
                            value: digital_value_from_vcd_bit(*bit),
                        });
                    }
                }
                claim_node(&mut seen, &reference)?;
                histories.digital_traces.push(DigitalTrace {
                    node_name: reference,
                    points,
                    hdl_time_precision: None,
                });
            }
            VcdSignalKind::Logic => {
                let (base, declared) = split_bus_notation(&reference);
                // The width bound is applied to the declared width before any
                // member is materialized, so a hostile dump cannot make a
                // reader build the list it is about to be refused for.
                if u64::from(signal.width) > u64::from(MAX_DIGITAL_BUS_WIDTH) {
                    return Err(DigitalBusError::WidthTooLarge {
                        bus: base.to_owned(),
                        width: u64::from(signal.width),
                    }
                    .into());
                }
                let width = i64::from(signal.width);
                let (msb, lsb) = match declared {
                    Some((msb, lsb)) => {
                        let range = msb.abs_diff(lsb).saturating_add(1);
                        if range != u64::from(signal.width) {
                            return Err(EventProjectionError::BusRangeWidth {
                                variable: reference.clone(),
                                width: signal.width,
                                msb,
                                lsb,
                                range,
                            });
                        }
                        (msb, lsb)
                    }
                    None => (width - 1, 0),
                };
                let step = if msb >= lsb { -1 } else { 1 };
                let indices: Vec<i64> = (0..width).map(|offset| msb + step * offset).collect();
                let mut members: Vec<DigitalTrace> = indices
                    .iter()
                    .map(|index| DigitalTrace {
                        node_name: vcd_member_name(base, *index),
                        points: Vec::new(),
                        hdl_time_precision: None,
                    })
                    .collect();
                for change in &signal.changes {
                    let bits = logic_bits(&reference, signal.width, change)?;
                    let time = time_at(&reference, change.tick)?;
                    for (member, bit) in members.iter_mut().zip(bits) {
                        let value = digital_value_from_vcd_bit(*bit);
                        // A recorded history keeps changes only, which is the
                        // invariant every other producer of one holds to.
                        if member.points.last().is_some_and(|last| last.value == value) {
                            continue;
                        }
                        member.points.push(DigitalTracePoint { time, value });
                    }
                }
                for member in &members {
                    claim_node(&mut seen, &member.node_name)?;
                }
                histories.digital_buses.push(DigitalBusDeclaration::new(
                    base,
                    msb,
                    lsb,
                    members
                        .iter()
                        .map(|member| member.node_name.clone())
                        .collect(),
                    DigitalBusSource::Import,
                )?);
                histories.digital_traces.extend(members);
            }
        }
    }

    validate_digital_bus_table(
        &histories.digital_buses,
        histories
            .digital_traces
            .iter()
            .map(|trace| trace.node_name.as_str()),
    )?;
    Ok(histories)
}

/// The bits one logic change carries, refusing a change of the wrong shape.
fn logic_bits<'a>(
    variable: &str,
    width: u32,
    change: &'a VcdChange,
) -> Result<&'a [VcdBit], EventProjectionError> {
    let VcdValue::Logic(bits) = &change.value else {
        return Err(EventProjectionError::VariableKind {
            variable: variable.to_owned(),
            tick: change.tick,
        });
    };
    if bits.len() != width as usize {
        return Err(EventProjectionError::VectorWidth {
            variable: variable.to_owned(),
            width,
            tick: change.tick,
            found: bits.len(),
        });
    }
    Ok(bits)
}

/// Record a name, refusing a second variable that reduces to the same one.
fn claim_node(seen: &mut BTreeSet<String>, node: &str) -> Result<(), EventProjectionError> {
    if seen.insert(canonical_event_name(node)) {
        Ok(())
    } else {
        Err(EventProjectionError::DuplicateNode {
            node: node.to_owned(),
        })
    }
}

/// How many leading scope levels every variable in the document shares.
///
/// A dump this crate wrote puts every variable in one scope, which is the
/// analysis name and not part of any node's identity, so dropping the shared
/// prefix is what makes reading one back a fixed point. A dump with real
/// hierarchy shares nothing above the top and keeps its whole path.
fn shared_scope_depth(signals: &[VcdSignal]) -> usize {
    let mut variables = signals.iter().filter_map(|signal| signal.variables.first());
    let Some(first) = variables.next() else {
        return 0;
    };
    let mut shared = first.scope.len();
    for variable in variables {
        shared = shared.min(
            first
                .scope
                .iter()
                .zip(variable.scope.iter())
                .take_while(|(left, right)| left == right)
                .count(),
        );
    }
    shared
}

/// A variable's name with the shared scope prefix dropped.
fn trailing_scoped_name(variable: &VcdVariable, shared: usize) -> String {
    let mut name = String::new();
    for level in variable.scope.iter().skip(shared) {
        name.push_str(level);
        name.push('.');
    }
    name.push_str(&variable.name);
    name
}

fn checked_node_name(node: &str) -> Result<String, EventProjectionError> {
    if is_writable_variable_name(node) {
        Ok(node.to_string())
    } else {
        Err(EventProjectionError::NodeName {
            node: node.to_string(),
        })
    }
}

/// One signal's event times, placed on the femtosecond grid in order.
#[derive(Default)]
struct SignalTimes {
    /// The previous point's recorded time and tick.
    last: Option<(Value, u128)>,
}

impl SignalTimes {
    /// Place `time` on the grid: its tick, and the previous point's time when
    /// that point is a different recorded time on the same tick.
    fn place(
        &mut self,
        node: &str,
        time: Value,
    ) -> Result<(u128, Option<Value>), EventProjectionError> {
        if self.last.is_some_and(|(earlier, _)| time < earlier) {
            return Err(EventProjectionError::UnorderedTime {
                node: node.to_string(),
                time,
            });
        }
        let tick = event_femtoseconds(node, time)?;
        let merged = self
            .last
            .filter(|&(earlier, earlier_tick)| earlier_tick == tick && earlier != time)
            .map(|(earlier, _)| earlier);
        self.last = Some((time, tick));
        Ok((tick, merged))
    }
}

/// Append one change to a signal's points. A change merged onto the tick of a
/// different value recorded at another time is refused; one that repeats the
/// value already there carries nothing and is dropped.
fn push_change(
    points: &mut Vec<(u128, VcdValue)>,
    node: &str,
    time: Value,
    (tick, merged): (u128, Option<Value>),
    value: VcdValue,
) -> Result<(), EventProjectionError> {
    if let Some(other_time) = merged
        && let Some((_, last)) = points.last()
    {
        if *last == value {
            return Ok(());
        }
        return Err(EventProjectionError::TimeCollision {
            node: node.to_string(),
            time,
            other_time,
            tick,
        });
    }
    points.push((tick, value));
    Ok(())
}

/// The tick grid one signal's changes are placed on.
#[derive(Debug, Clone, Copy)]
enum TimeGrid {
    /// No HDL observes the signal: each time rounds to the nearest
    /// femtosecond, and two values merged onto one tick are refused.
    Femtosecond,
    /// An HDL observes the signal: each time rounds to the HDL's nearest tick,
    /// as the HDL dated the change, and the last value of a tick stands.
    Hdl {
        /// The HDL's time precision, in seconds.
        precision: Value,
        /// The same precision in whole femtoseconds.
        period: u128,
    },
}

impl TimeGrid {
    /// The grid a signal observed by an HDL of `precision` seconds is on.
    fn of(node: &str, precision: Option<Value>) -> Result<Self, EventProjectionError> {
        let Some(precision) = precision else {
            return Ok(Self::Femtosecond);
        };
        VcdTimescale::ALL
            .into_iter()
            .map(|timescale| u128::from(timescale.femtoseconds()))
            .find(|&period| {
                let seconds = period as f64 / FEMTOSECONDS_PER_SECOND;
                (seconds - precision).abs() <= seconds * 1e-9
            })
            .map(|period| Self::Hdl { precision, period })
            .ok_or_else(|| EventProjectionError::HdlPrecision {
                node: node.to_string(),
                precision,
            })
    }

    /// The HDL tick in femtoseconds, if this is an HDL's grid.
    fn hdl_period(self) -> Option<u128> {
        match self {
            Self::Femtosecond => None,
            Self::Hdl { period, .. } => Some(period),
        }
    }
}

/// One signal's changes, placed on its grid in order.
struct GridChanges {
    grid: TimeGrid,
    times: SignalTimes,
    points: Vec<(u128, VcdValue)>,
}

impl GridChanges {
    fn new(grid: TimeGrid) -> Self {
        Self {
            grid,
            times: SignalTimes::default(),
            points: Vec::new(),
        }
    }

    /// Place one change recorded at `time`.
    ///
    /// On an HDL's grid the time rounds to the HDL's nearest tick, which is
    /// the rule the HDL dated an analog crossing by. Several changes on one
    /// tick are the HDL's delta cycles there: the last one is what the HDL
    /// observed once they settled, so it replaces the others, and a tick that
    /// settles back where the previous tick left the signal carries no change.
    fn push(
        &mut self,
        node: &str,
        time: Value,
        value: VcdValue,
    ) -> Result<(), EventProjectionError> {
        let placed = self.times.place(node, time)?;
        let TimeGrid::Hdl { precision, period } = self.grid else {
            return push_change(&mut self.points, node, time, placed, value);
        };
        let unrepresentable = || EventProjectionError::UnrepresentableTime {
            node: node.to_string(),
            time,
        };
        let ticks = (time / precision).round();
        if !ticks.is_finite() || ticks < 0.0 || ticks > MAX_EVENT_FEMTOSECONDS as f64 {
            return Err(unrepresentable());
        }
        let tick = (ticks as u128)
            .checked_mul(period)
            .filter(|&tick| tick <= MAX_EVENT_FEMTOSECONDS)
            .ok_or_else(unrepresentable)?;
        if self.points.last().is_some_and(|(last, _)| *last == tick) {
            self.points.pop();
        }
        if self.points.last().is_some_and(|(_, held)| *held == value) {
            return Ok(());
        }
        self.points.push((tick, value));
        Ok(())
    }
}

/// Round one event time to the nearest whole femtosecond.
///
/// Retain the low integer bits when scaling: above 2^53 femtoseconds the
/// binary64 product alone can move an edge by whole ticks, so the exact
/// multiplication residual is folded back in integer arithmetic.
fn event_femtoseconds(node: &str, time: Value) -> Result<u128, EventProjectionError> {
    if !time.is_finite() || time < 0.0 {
        return Err(EventProjectionError::UnrepresentableTime {
            node: node.to_string(),
            time,
        });
    }
    let scaled = time * FEMTOSECONDS_PER_SECOND;
    if scaled > MAX_EVENT_FEMTOSECONDS as f64 {
        return Err(EventProjectionError::UnrepresentableTime {
            node: node.to_string(),
            time,
        });
    }
    let integral = scaled.round_ties_even();
    // Fused arithmetic recovers the multiplication residual without rounding
    // the product first. Apply its whole-tick correction in integer arithmetic
    // so it is not lost again when added to a large binary64 timestamp.
    let residual = time.mul_add(FEMTOSECONDS_PER_SECOND, -integral);
    let correction = residual.round_ties_even();
    let tick = u128::try_from(integral as i128 + correction as i128).map_err(|_| {
        EventProjectionError::UnrepresentableTime {
            node: node.to_string(),
            time,
        }
    })?;
    if tick > MAX_EVENT_FEMTOSECONDS {
        return Err(EventProjectionError::UnrepresentableTime {
            node: node.to_string(),
            time,
        });
    }
    Ok(tick)
}

/// The coarsest timescale that divides every tick, and no coarser than
/// `hdl_period` when an HDL observes a signal: such a dump is written at the
/// design's own precision, the tick its `$dumpvars` would use.
fn choose_timescale<'a>(
    points: impl Iterator<Item = &'a (u128, VcdValue)> + Clone,
    hdl_period: Option<u128>,
) -> Result<VcdTimescale, EventProjectionError> {
    let finest = VcdTimescale {
        magnitude: VcdMagnitude::One,
        unit: VcdTimeUnit::Femtoseconds,
    };
    let hdl_timescale = hdl_period.and_then(|period| {
        VcdTimescale::ALL
            .into_iter()
            .find(|candidate| u128::from(candidate.femtoseconds()) == period)
    });
    if points.clone().all(|(femtoseconds, _)| *femtoseconds == 0) {
        return Ok(hdl_timescale.unwrap_or(finest));
    }
    VcdTimescale::ALL
        .into_iter()
        .filter(|candidate| {
            hdl_period.is_none_or(|cap| u128::from(candidate.femtoseconds()) <= cap)
        })
        .find(|candidate| {
            let period = u128::from(candidate.femtoseconds());
            points.clone().all(|(femtoseconds, _)| {
                femtoseconds % period == 0 && femtoseconds / period <= u128::from(u64::MAX)
            })
        })
        .ok_or(EventProjectionError::TimescaleRange)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::{DigitalTracePoint, RealTracePoint};

    fn digital_trace(node: &str, points: &[(Value, DigitalValue)]) -> DigitalTrace {
        DigitalTrace {
            node_name: node.to_string(),
            points: points
                .iter()
                .map(|(time, value)| DigitalTracePoint {
                    time: *time,
                    value: *value,
                })
                .collect(),
            hdl_time_precision: None,
        }
    }

    fn real_trace(node: &str, points: &[(Value, Value)]) -> RealTrace {
        RealTrace {
            node_name: node.to_string(),
            points: points
                .iter()
                .map(|(time, value)| RealTracePoint {
                    time: *time,
                    value: *value,
                })
                .collect(),
        }
    }

    const STRENGTHS: [DigitalStrength; 4] = [
        DigitalStrength::Strong,
        DigitalStrength::Resistive,
        DigitalStrength::HighZ,
        DigitalStrength::Undetermined,
    ];

    #[test]
    fn every_strength_band_keeps_its_level() {
        let levels = [
            (DigitalState::Zero, VcdBit::Zero),
            (DigitalState::ZeroR, VcdBit::Zero),
            (DigitalState::ZeroZ, VcdBit::Zero),
            (DigitalState::One, VcdBit::One),
            (DigitalState::OneR, VcdBit::One),
            (DigitalState::OneZ, VcdBit::One),
            (DigitalState::Unknown, VcdBit::Unknown),
            (DigitalState::UnknownR, VcdBit::Unknown),
            (DigitalState::UnknownZ, VcdBit::Unknown),
            (DigitalState::HighZ, VcdBit::HighImpedance),
        ];
        for (state, expected) in levels {
            for strength in STRENGTHS {
                assert_eq!(
                    digital_value_to_vcd_bit(DigitalValue::new(state, strength)),
                    expected,
                    "{state:?} at {strength:?}"
                );
            }
        }
    }

    #[test]
    fn the_reverse_mapping_drives_strongly() {
        assert_eq!(
            digital_value_from_vcd_bit(VcdBit::Zero),
            DigitalValue::new(DigitalState::Zero, DigitalStrength::Strong)
        );
        assert_eq!(
            digital_value_from_vcd_bit(VcdBit::One),
            DigitalValue::new(DigitalState::One, DigitalStrength::Strong)
        );
        assert_eq!(
            digital_value_from_vcd_bit(VcdBit::Unknown),
            DigitalValue::new(DigitalState::Unknown, DigitalStrength::Strong)
        );
        assert_eq!(
            digital_value_from_vcd_bit(VcdBit::HighImpedance),
            DigitalValue::high_z()
        );
        // Every bit survives the round trip back to itself, whatever the
        // strength the level came from.
        for state in [
            DigitalState::ZeroR,
            DigitalState::OneZ,
            DigitalState::UnknownR,
            DigitalState::HighZ,
        ] {
            for strength in STRENGTHS {
                let bit = digital_value_to_vcd_bit(DigitalValue::new(state, strength));
                assert_eq!(
                    digital_value_to_vcd_bit(digital_value_from_vcd_bit(bit)),
                    bit
                );
            }
        }
    }

    #[test]
    fn a_member_code_spells_the_bit_the_dump_shows_and_an_absent_one_spells_unknown() {
        // A held code spells its level, whatever the strength the level came
        // from — the same table `digital_value_to_vcd_bit` applies to a scalar.
        for state in [
            DigitalState::Zero,
            DigitalState::ZeroR,
            DigitalState::One,
            DigitalState::OneZ,
            DigitalState::UnknownR,
            DigitalState::HighZ,
        ] {
            for strength in STRENGTHS {
                let value = DigitalValue::new(state, strength);
                assert_eq!(
                    event_code_to_vcd_bit(Some(value.event_code())),
                    Some(digital_value_to_vcd_bit(value)),
                    "{value:?} must spell as the scalar projection spells it"
                );
            }
        }

        // A member the run has not stated yet is unknown, which is a bit —
        // it just is not a bit anything drove.
        assert_eq!(event_code_to_vcd_bit(None), Some(VcdBit::Unknown));

        // A code no value encodes is not a bit at all, so the caller decides
        // what to say about it rather than being handed `x`.
        let unencodable = (0..=u8::MAX)
            .find(|code| DigitalValue::from_event_code(*code).is_none())
            .expect("not every byte is an event code");
        assert_eq!(event_code_to_vcd_bit(Some(unencodable)), None);
    }

    #[test]
    fn digital_traces_become_one_bit_wires_under_the_analysis_scope() {
        let document = event_vcd_document(
            "tran",
            &[digital_trace(
                "clk",
                &[
                    (0.0, DigitalValue::zero()),
                    (5e-9, DigitalValue::one()),
                    (10e-9, DigitalValue::high_z()),
                ],
            )],
            &[],
            &[],
        )
        .expect("projection");

        assert_eq!(document.timescale.to_string(), "1 ns");
        assert_eq!(document.version, VCD_WRITER_VERSION);
        let signal = document.signals.first().expect("one signal");
        assert_eq!(signal.width, 1);
        assert_eq!(signal.kind, VcdSignalKind::Logic);
        assert_eq!(signal.identifier, "!");
        let variable = signal.variables.first().expect("one variable");
        assert_eq!(variable.scoped_name(), "tran.clk");
        let ticks: Vec<u64> = signal.changes.iter().map(|change| change.tick).collect();
        assert_eq!(ticks, vec![0, 5, 10]);
        assert_eq!(
            signal.changes.last().map(|change| change.value.clone()),
            Some(VcdValue::Logic(vec![VcdBit::HighImpedance]))
        );
    }

    #[test]
    fn real_traces_become_real_variables() {
        let document = event_vcd_document(
            "tran",
            &[],
            &[real_trace("out", &[(0.0, 1.5), (1e-12, -2.25)])],
            &[],
        )
        .expect("projection");
        assert_eq!(document.timescale.to_string(), "1 ps");
        let signal = document.signals.first().expect("one signal");
        assert_eq!(signal.kind, VcdSignalKind::Real);
        assert_eq!(signal.width, 64);
        assert_eq!(
            signal.changes,
            vec![
                VcdChange {
                    tick: 0,
                    value: VcdValue::Real(1.5)
                },
                VcdChange {
                    tick: 1,
                    value: VcdValue::Real(-2.25)
                },
            ]
        );
    }

    #[test]
    fn the_timescale_is_the_coarsest_exact_one() {
        let cases: [(&[Value], &str, &[u64]); 5] = [
            (&[0.0, 10e-9, 20e-9], "10 ns", &[0, 1, 2]),
            (&[0.0, 5e-9, 10e-9], "1 ns", &[0, 5, 10]),
            (&[0.0, 1e-12, 3e-12], "1 ps", &[0, 1, 3]),
            (&[0.0, 2e-15, 3e-15], "1 fs", &[0, 2, 3]),
            (&[0.0, 100e-9, 300e-9], "100 ns", &[0, 1, 3]),
        ];
        for (times, expected, ticks) in cases {
            let points: Vec<(Value, DigitalValue)> = times
                .iter()
                .enumerate()
                .map(|(index, time)| {
                    (
                        *time,
                        if index % 2 == 0 {
                            DigitalValue::zero()
                        } else {
                            DigitalValue::one()
                        },
                    )
                })
                .collect();
            let document = event_vcd_document("tran", &[digital_trace("clk", &points)], &[], &[])
                .expect("projection");
            assert_eq!(document.timescale.to_string(), expected, "{times:?}");
            let signal = document.signals.first().expect("one signal");
            let observed: Vec<u64> = signal.changes.iter().map(|change| change.tick).collect();
            assert_eq!(observed, ticks, "{times:?}");
        }
    }

    /// An A/D crossing is dated wherever the analog solution put it, nearly
    /// never on a whole femtosecond; it is written at the nearest one.
    #[test]
    fn a_time_between_femtoseconds_takes_the_nearest_one() {
        let document = event_vcd_document(
            "tran",
            &[digital_trace(
                "clk",
                &[
                    (0.0, DigitalValue::zero()),
                    (1.4e-15, DigitalValue::one()),
                    (2.6e-15, DigitalValue::zero()),
                    (1.885_714_285_714_285_7e-7, DigitalValue::one()),
                ],
            )],
            &[],
            &[],
        )
        .expect("every time has a nearest femtosecond");
        assert_eq!(document.timescale.to_string(), "1 fs");
        let ticks: Vec<u64> = document.signals[0]
            .changes
            .iter()
            .map(|change| change.tick)
            .collect();
        assert_eq!(ticks, [0, 1, 3, 188_571_429]);
    }

    /// Rounding may not merge two different values one signal held at
    /// different times onto one tick, where the dump could not order them; a
    /// repeat of the value already there is no change. Other signals may share
    /// the tick.
    #[test]
    fn a_change_rounded_onto_a_different_value_is_refused_by_name() {
        for time in [Value::from_bits(1), 1e-30, 1e-25, 0.25e-15] {
            for real in [false, true] {
                let collided = if real {
                    event_vcd_document(
                        "tran",
                        &[],
                        &[real_trace("out", &[(0.0, 0.0), (time, 1.0)])],
                        &[],
                    )
                } else {
                    event_vcd_document(
                        "tran",
                        &[digital_trace(
                            "out",
                            &[(0.0, DigitalValue::zero()), (time, DigitalValue::one())],
                        )],
                        &[],
                        &[],
                    )
                };
                let error = collided.expect_err("two values on one tick have no order");
                assert_eq!(
                    error,
                    EventProjectionError::TimeCollision {
                        node: "out".to_owned(),
                        time,
                        other_time: 0.0,
                        tick: 0,
                    }
                );
                assert!(error.to_string().contains("out"), "{error}");
            }
            let repeated = event_vcd_document(
                "tran",
                &[
                    digital_trace(
                        "out",
                        &[(0.0, DigitalValue::one()), (time, DigitalValue::one())],
                    ),
                    digital_trace("other", &[(time, DigitalValue::one())]),
                ],
                &[],
                &[],
            )
            .expect("a repeated value and another signal's change both fit");
            assert_eq!(document_ticks(&repeated, 0), [0]);
            assert_eq!(document_ticks(&repeated, 1), [0]);
        }
    }

    fn document_ticks(document: &VcdDocument, signal: usize) -> Vec<u64> {
        document.signals[signal]
            .changes
            .iter()
            .map(|change| change.tick)
            .collect()
    }

    fn document_bits(document: &VcdDocument, signal: usize) -> Vec<String> {
        document.signals[signal]
            .changes
            .iter()
            .map(|change| match &change.value {
                VcdValue::Logic(bits) => bits.iter().map(|bit| bit.to_string()).collect(),
                VcdValue::Real(value) => value.to_string(),
            })
            .collect()
    }

    fn hdl_trace(precision: Value, points: &[(Value, DigitalValue)]) -> DigitalTrace {
        DigitalTrace {
            hdl_time_precision: Some(precision),
            ..digital_trace("in", points)
        }
    }

    /// An A/D crossing feeding an HDL is recorded where the analog solution
    /// crossed, 188.5714286 ns; the HDL (`timescale 1ns/1ps`) saw it at its
    /// nearest tick, 188571 ps, and the dump says what the HDL saw, at the
    /// design precision.
    #[test]
    fn a_net_an_hdl_observes_changes_at_the_hdl_tick() {
        let document = event_vcd_document(
            "tran",
            &[hdl_trace(
                1e-12,
                &[
                    (0.0, DigitalValue::zero()),
                    (7e-8, DigitalValue::one()),
                    (1.885_714_285_714_285_7e-7, DigitalValue::zero()),
                ],
            )],
            &[],
            &[],
        )
        .expect("an HDL net dumps at the HDL tick");
        assert_eq!(document.timescale.to_string(), "1 ps");
        assert_eq!(document_ticks(&document, 0), [0, 70_000, 188_571]);
    }

    /// Several changes of an HDL net on one tick are its delta cycles: the
    /// last value stands, and a tick that settles back where it started is no
    /// change at all. A sine starting on the A/D threshold records the input
    /// rising 1e-20 s after time zero; the HDL saw `in=1` at tick 0.
    #[test]
    fn changes_of_an_hdl_net_on_one_tick_keep_the_last_value() {
        let document = event_vcd_document(
            "tran",
            &[hdl_trace(
                1e-12,
                &[
                    (0.0, DigitalValue::zero()),
                    (1e-20, DigitalValue::one()),
                    (5.0001e-9, DigitalValue::zero()),
                    (5.0003e-9, DigitalValue::one()),
                    (9.0e-9, DigitalValue::zero()),
                    (9.0002e-9, DigitalValue::one()),
                    (9.0004e-9, DigitalValue::zero()),
                ],
            )],
            &[],
            &[],
        )
        .expect("delta cycles collapse to the last value");
        assert_eq!(document_ticks(&document, 0), [0, 9_000]);
        assert_eq!(document_bits(&document, 0), ["1", "0"]);
        // The same history on no HDL's grid is still refused.
        let unobserved = event_vcd_document(
            "tran",
            &[digital_trace(
                "in",
                &[(0.0, DigitalValue::zero()), (1e-20, DigitalValue::one())],
            )],
            &[],
            &[],
        );
        assert!(matches!(
            unobserved,
            Err(EventProjectionError::TimeCollision { .. })
        ));
    }

    /// The dump is written at the design precision even when every change
    /// lands on a coarser tick, and finer only for a net no HDL observes.
    #[test]
    fn an_hdl_dump_is_written_at_the_design_precision() {
        let hdl = hdl_trace(
            1e-12,
            &[(0.0, DigitalValue::zero()), (1e-6, DigitalValue::one())],
        );
        let document =
            event_vcd_document("tran", std::slice::from_ref(&hdl), &[], &[]).expect("dumps");
        assert_eq!(document.timescale.to_string(), "1 ps");
        assert_eq!(document_ticks(&document, 0), [0, 1_000_000]);

        let analog = digital_trace(
            "cmp",
            &[(0.0, DigitalValue::zero()), (1.6e-15, DigitalValue::one())],
        );
        let document = event_vcd_document("tran", &[hdl, analog], &[], &[]).expect("dumps");
        assert_eq!(document.timescale.to_string(), "1 fs");
        assert_eq!(document_ticks(&document, 0), [0, 1_000_000_000]);
        assert_eq!(document_ticks(&document, 1), [0, 2]);

        let error = event_vcd_document(
            "tran",
            &[hdl_trace(3e-12, &[(0.0, DigitalValue::zero())])],
            &[],
            &[],
        )
        .expect_err("3 ps is no VCD tick");
        assert!(
            matches!(error, EventProjectionError::HdlPrecision { .. }),
            "{error}"
        );
    }

    #[test]
    fn nanosecond_times_are_not_mistaken_for_inexact_ones() {
        // `1e-9 * 1e15` is not exactly 1_000_000 in binary64. A residual test
        // against zero would reject a run of ordinary nanosecond edges.
        let points: Vec<(Value, DigitalValue)> = (0..64)
            .map(|index| {
                (
                    f64::from(index) * 1e-9,
                    if index % 2 == 0 {
                        DigitalValue::zero()
                    } else {
                        DigitalValue::one()
                    },
                )
            })
            .collect();
        let document = event_vcd_document("tran", &[digital_trace("clk", &points)], &[], &[])
            .expect("projection");
        assert_eq!(document.timescale.to_string(), "1 ns");
        let signal = document.signals.first().expect("one signal");
        let ticks: Vec<u64> = signal.changes.iter().map(|change| change.tick).collect();
        assert_eq!(ticks, (0..64).collect::<Vec<u64>>());
    }

    #[test]
    fn an_empty_projection_uses_the_finest_scale() {
        let document = event_vcd_document("tran", &[], &[], &[]).expect("projection");
        assert_eq!(document.timescale.to_string(), "1 fs");
        assert!(document.signals.is_empty());

        let only_zero = event_vcd_document(
            "tran",
            &[digital_trace("clk", &[(0.0, DigitalValue::zero())])],
            &[],
            &[],
        )
        .expect("projection");
        assert_eq!(only_zero.timescale.to_string(), "1 fs");
    }

    #[test]
    fn unwritable_names_and_impossible_times_are_refused() {
        assert_eq!(
            event_vcd_document("two words", &[], &[], &[]).expect_err("scope"),
            EventProjectionError::Scope {
                scope: "two words".to_string()
            }
        );
        assert_eq!(
            event_vcd_document(
                "tran",
                &[digital_trace("clk\tenable", &[(0.0, DigitalValue::zero())])],
                &[],
                &[]
            )
            .expect_err("node name"),
            EventProjectionError::NodeName {
                node: "clk\tenable".to_string()
            }
        );
        assert_eq!(
            event_vcd_document(
                "tran",
                &[digital_trace("$end", &[(0.0, DigitalValue::zero())])],
                &[],
                &[]
            )
            .expect_err("node name"),
            EventProjectionError::NodeName {
                node: "$end".to_string()
            }
        );
        assert_eq!(
            event_vcd_document(
                "tran",
                &[digital_trace("clk", &[(-1e-9, DigitalValue::zero())])],
                &[],
                &[]
            )
            .expect_err("negative time"),
            EventProjectionError::UnrepresentableTime {
                node: "clk".to_string(),
                time: -1e-9
            }
        );
        assert_eq!(
            event_vcd_document(
                "tran",
                &[digital_trace(
                    "clk",
                    &[(2e-9, DigitalValue::zero()), (1e-9, DigitalValue::one())]
                )],
                &[],
                &[]
            )
            .expect_err("unordered"),
            EventProjectionError::UnorderedTime {
                node: "clk".to_string(),
                time: 1e-9
            }
        );
        assert_eq!(
            event_vcd_document("tran", &[], &[real_trace("out", &[(0.0, f64::NAN)])], &[])
                .expect_err("non-finite"),
            EventProjectionError::NonFiniteValue {
                node: "out".to_string(),
                time: 0.0
            }
        );
    }
}

#[cfg(test)]
mod bus_tests {
    use super::*;
    use crate::io::vcd::{parse_vcd_reader_with_limits, write_vcd};
    use crate::resource::ResourceLimits;

    fn trace(node: &str, points: &[(Value, DigitalValue)]) -> DigitalTrace {
        DigitalTrace {
            node_name: node.to_string(),
            points: points
                .iter()
                .map(|(time, value)| DigitalTracePoint {
                    time: *time,
                    value: *value,
                })
                .collect(),
            hdl_time_precision: None,
        }
    }

    fn bus(name: &str, msb: i64, lsb: i64, members: &[&str]) -> DigitalBusDeclaration {
        DigitalBusDeclaration::new(
            name,
            msb,
            lsb,
            members.iter().map(|member| (*member).to_string()).collect(),
            DigitalBusSource::Schematic,
        )
        .expect("the fixture bus is well formed")
    }

    /// A two-bit bus whose members change at three distinct times, plus one
    /// scalar that belongs to no bus.
    fn two_bit_fixture() -> (Vec<DigitalTrace>, Vec<DigitalBusDeclaration>) {
        let traces = vec![
            trace(
                "d1",
                &[(0.0, DigitalValue::zero()), (2e-9, DigitalValue::one())],
            ),
            trace(
                "d0",
                &[
                    (0.0, DigitalValue::one()),
                    (1e-9, DigitalValue::zero()),
                    (2e-9, DigitalValue::one()),
                ],
            ),
            trace("clk", &[(0.0, DigitalValue::zero())]),
        ];
        (traces, vec![bus("d", 1, 0, &["d1", "d0"])])
    }

    #[test]
    fn a_bus_becomes_one_vector_variable_and_its_members_are_suppressed() {
        let (traces, buses) = two_bit_fixture();
        let document = event_vcd_document("tran", &traces, &[], &buses).expect("projection");

        let names: Vec<String> = document
            .signals
            .iter()
            .map(|signal| {
                signal
                    .variables
                    .first()
                    .expect("a signal declares a name")
                    .name
                    .clone()
            })
            .collect();
        assert_eq!(
            names,
            vec!["d [1:0]".to_string(), "clk".to_string()],
            "the members are in the vector, and a scalar that belongs to no bus stays"
        );

        let vector = document.signals.first().expect("the bus signal");
        assert_eq!(vector.width, 2);
        assert_eq!(vector.kind, VcdSignalKind::Logic);
        assert_eq!(
            vector.changes,
            vec![
                VcdChange {
                    tick: 0,
                    value: VcdValue::Logic(vec![VcdBit::Zero, VcdBit::One]),
                },
                VcdChange {
                    tick: 1,
                    value: VcdValue::Logic(vec![VcdBit::Zero, VcdBit::Zero]),
                },
                VcdChange {
                    tick: 2,
                    value: VcdValue::Logic(vec![VcdBit::One, VcdBit::One]),
                },
            ],
            "bits are most significant first, in declaration order"
        );
    }

    #[test]
    fn a_member_with_no_point_yet_is_written_x() {
        let traces = vec![
            trace("d1", &[(0.0, DigitalValue::one())]),
            trace("d0", &[(3e-9, DigitalValue::zero())]),
        ];
        let document = event_vcd_document("tran", &traces, &[], &[bus("d", 1, 0, &["d1", "d0"])])
            .expect("projection");
        let vector = document.signals.first().expect("the bus signal");
        assert_eq!(
            vector.changes.first().map(|change| change.value.clone()),
            Some(VcdValue::Logic(vec![VcdBit::One, VcdBit::Unknown])),
            "the run has not stated bit 0 yet, and x is what VCD spells that"
        );
    }

    #[test]
    fn a_bus_over_a_trace_the_run_did_not_record_is_refused_by_name() {
        let traces = vec![trace("d1", &[(0.0, DigitalValue::one())])];
        assert_eq!(
            event_vcd_document("tran", &traces, &[], &[bus("d", 1, 0, &["d1", "d0"])])
                .expect_err("a member with no trace has no bits to write"),
            EventProjectionError::Bus(DigitalBusError::UnknownMember {
                bus: "d".to_string(),
                member: "d0".to_string(),
            })
        );
    }

    #[test]
    fn a_projection_with_no_bus_is_unchanged() {
        let traces = vec![trace("clk", &[(0.0, DigitalValue::zero())])];
        let with_empty_table = event_vcd_document("tran", &traces, &[], &[]).expect("projection");
        let signal = with_empty_table.signals.first().expect("one signal");
        assert_eq!(signal.width, 1);
        assert_eq!(
            signal
                .variables
                .first()
                .expect("a signal declares a name")
                .name,
            "clk"
        );
    }

    #[test]
    fn a_dump_with_two_buses_survives_write_parse_write_byte_for_byte() {
        let traces = vec![
            trace(
                "a1",
                &[(0.0, DigitalValue::zero()), (1e-9, DigitalValue::one())],
            ),
            trace("a0", &[(0.0, DigitalValue::one())]),
            trace("b2", &[(0.0, DigitalValue::high_z())]),
            trace("b1", &[(2e-9, DigitalValue::zero())]),
            trace(
                "b0",
                &[(0.0, DigitalValue::one()), (3e-9, DigitalValue::zero())],
            ),
            trace("reset", &[(0.0, DigitalValue::zero())]),
        ];
        let buses = vec![
            bus("a", 1, 0, &["a1", "a0"]),
            bus("b", 0, 2, &["b0", "b1", "b2"]),
        ];
        let document = event_vcd_document("tran", &traces, &[], &buses).expect("projection");

        let mut first = Vec::new();
        write_vcd(&mut first, &document).expect("write");
        let reparsed = parse_vcd_reader_with_limits(
            &mut std::io::Cursor::new(first.clone()),
            ResourceLimits::default(),
        )
        .expect("parse");
        let mut second = Vec::new();
        write_vcd(&mut second, &reparsed).expect("rewrite");
        assert_eq!(
            first, second,
            "a two-bus dump is a fixed point of the codec"
        );

        let text = String::from_utf8(first).expect("VCD is ASCII here");
        assert!(text.contains("$var wire 2 ! a [1:0] $end"), "{text}");
        assert!(text.contains("$var wire 3 \" b [0:2] $end"), "{text}");
        assert!(
            !text.contains(" a1 "),
            "member scalars are suppressed: {text}"
        );
    }

    #[test]
    fn a_dump_reads_back_as_the_declaration_and_members_it_was_written_from() {
        // The names round trip because the members were already spelled the
        // way VCD spells a bit-select; an engine bus whose members are called
        // DATA#3 comes back as DATA[3], which is the dump's grammar and not
        // the deck's.
        let traces = vec![
            trace(
                "d[1]",
                &[(0.0, DigitalValue::zero()), (2e-9, DigitalValue::one())],
            ),
            trace("d[0]", &[(0.0, DigitalValue::one())]),
            trace("clk", &[(0.0, DigitalValue::zero())]),
        ];
        let buses = vec![bus("d", 1, 0, &["d[1]", "d[0]"])];
        let document = event_vcd_document("tran", &traces, &[], &buses).expect("projection");

        let histories = vcd_event_histories(&document).expect("import");
        assert_eq!(histories.digital_buses.len(), 1);
        let imported = &histories.digital_buses[0];
        assert_eq!(imported.name, "d");
        assert_eq!((imported.msb, imported.lsb), (1, 0));
        assert_eq!(
            imported.members,
            vec!["d[1]".to_string(), "d[0]".to_string()]
        );
        assert_eq!(
            imported.source,
            DigitalBusSource::Import,
            "a dump is a claim by whatever wrote it"
        );

        let names: Vec<&str> = histories
            .digital_traces
            .iter()
            .map(|trace| trace.node_name.as_str())
            .collect();
        assert_eq!(
            names,
            vec!["d[1]", "d[0]", "clk"],
            "buses are declared first, so their members come back first"
        );
        let restored = histories
            .digital_traces
            .iter()
            .find(|trace| trace.node_name == "d[1]")
            .expect("member 1");
        assert_eq!(
            restored.points,
            vec![
                DigitalTracePoint {
                    time: 0.0,
                    value: DigitalValue::zero(),
                },
                DigitalTracePoint {
                    time: 2e-9,
                    value: DigitalValue::one(),
                },
            ],
            "a member history keeps changes only, like every other recorded one"
        );
    }

    #[test]
    fn a_wide_variable_with_no_declared_range_is_read_as_width_minus_one_down_to_zero() {
        let document = VcdDocument {
            date: String::new(),
            version: VCD_WRITER_VERSION.to_string(),
            comments: Vec::new(),
            timescale: VcdTimescale {
                magnitude: VcdMagnitude::One,
                unit: VcdTimeUnit::Nanoseconds,
            },
            signals: vec![VcdSignal {
                identifier: "!".to_string(),
                variables: vec![VcdVariable {
                    scope: vec!["top".to_string()],
                    name: "count".to_string(),
                }],
                width: 3,
                kind: VcdSignalKind::Logic,
                changes: vec![VcdChange {
                    tick: 0,
                    value: VcdValue::Logic(vec![VcdBit::Zero, VcdBit::One, VcdBit::HighImpedance]),
                }],
            }],
        };
        let histories = vcd_event_histories(&document).expect("import");
        let bus = histories.digital_buses.first().expect("one bus");
        assert_eq!((bus.name.as_str(), bus.msb, bus.lsb), ("count", 2, 0));
        assert_eq!(
            bus.members,
            vec![
                "count[2]".to_string(),
                "count[1]".to_string(),
                "count[0]".to_string()
            ]
        );
    }

    #[test]
    fn a_declared_range_that_does_not_match_the_width_is_refused() {
        let document = VcdDocument {
            date: String::new(),
            version: VCD_WRITER_VERSION.to_string(),
            comments: Vec::new(),
            timescale: VcdTimescale {
                magnitude: VcdMagnitude::One,
                unit: VcdTimeUnit::Nanoseconds,
            },
            signals: vec![VcdSignal {
                identifier: "!".to_string(),
                variables: vec![VcdVariable {
                    scope: Vec::new(),
                    name: "data[7:0]".to_string(),
                }],
                width: 2,
                kind: VcdSignalKind::Logic,
                changes: Vec::new(),
            }],
        };
        assert_eq!(
            vcd_event_histories(&document).expect_err("2 bits are not [7:0]"),
            EventProjectionError::BusRangeWidth {
                variable: "data[7:0]".to_string(),
                width: 2,
                msb: 7,
                lsb: 0,
                range: 8,
            }
        );
    }

    #[test]
    fn a_vector_variable_is_named_in_the_space_form_the_dumpers_write() {
        let declaration = bus("data", 1, 0, &["data[1]", "data[0]"]);
        assert_eq!(bus_reference(&declaration), "data [1:0]");
        assert!(
            is_writable_variable_name(&bus_reference(&declaration)),
            "the space form survives the $var grammar"
        );
    }
}

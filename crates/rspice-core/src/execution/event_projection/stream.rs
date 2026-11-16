//! [`super::event_vcd_document`] of event histories read one point at a time.
//!
//! A run whose events stay in a results store writes its dump without holding them: one pass
//! over every signal's points checks them and chooses the timescale, as the document's
//! construction does, and a second merges the signals' changes in the order
//! [`crate::io::write_vcd`] writes a document's, by tick and then by signal. The dump is byte
//! for byte the one the document gives, and a history the document refuses is refused with
//! the same error.

use super::*;
use crate::execution::event_bus::MAX_BUS_EVENT_CELLS;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// One trace's points in recorded order.
pub type EventPoints<'s, T> =
    Box<dyn Iterator<Item = Result<(Value, T), EventProjectionError>> + 's>;

/// The points of a run's event traces, each read from its first as often as a dump asks.
pub trait EventSource {
    /// The points of the digital trace at `index` among the traces the dump is given.
    fn digital(&self, index: usize) -> EventPoints<'_, DigitalValue>;
    /// The points of the real trace at `index` among the traces the dump is given.
    fn real(&self, index: usize) -> EventPoints<'_, Value>;
}

/// The traces' own points, for a dump of histories held in memory.
#[derive(Debug, Clone, Copy)]
pub struct HeldEvents<'t> {
    pub digital: &'t [DigitalTrace],
    pub real: &'t [RealTrace],
}

impl EventSource for HeldEvents<'_> {
    fn digital(&self, index: usize) -> EventPoints<'_, DigitalValue> {
        Box::new(
            self.digital
                .get(index)
                .into_iter()
                .flat_map(|trace| trace.points.iter())
                .map(|point| Ok((point.time, point.value))),
        )
    }

    fn real(&self, index: usize) -> EventPoints<'_, Value> {
        Box::new(
            self.real
                .get(index)
                .into_iter()
                .flat_map(|trace| trace.points.iter())
                .map(|point| Ok((point.time, point.value))),
        )
    }
}

/// How one declared signal's changes are read.
enum SignalPlan {
    Digital {
        trace: usize,
        node: String,
        grid: TimeGrid,
    },
    Real {
        trace: usize,
        node: String,
    },
    Bus {
        reference: String,
        members: Vec<usize>,
        grid: TimeGrid,
    },
}

/// A dump of event histories whose changes are read from their source as they are written.
pub struct EventVcdStream<S: EventSource> {
    header: VcdDocument,
    plans: Vec<SignalPlan>,
    period: u128,
    source: S,
}

/// [`super::event_vcd_document`] of `digital_traces` and `real_traces`, whose points `source`
/// reads rather than the traces: the document's header, and its changes on demand through
/// [`EventVcdStream::changes`]. Every point is read once here, so a history the document
/// refuses is refused here with the same error.
pub fn event_vcd_stream<S: EventSource>(
    scope: &str,
    digital_traces: &[DigitalTrace],
    real_traces: &[RealTrace],
    digital_buses: &[DigitalBusDeclaration],
    source: S,
) -> Result<EventVcdStream<S>, EventProjectionError> {
    let points = &source;
    if !is_writable_scope_name(scope) {
        return Err(EventProjectionError::Scope {
            scope: scope.to_string(),
        });
    }
    validate_digital_bus_table(
        digital_buses,
        digital_traces.iter().map(|trace| trace.node_name.as_str()),
    )?;

    let mut plans = Vec::new();
    let mut declared: Vec<(String, VcdSignalKind, u32)> = Vec::new();
    let mut ticks = TickDivisor::default();
    let mut hdl_period: Option<u128> = None;
    let mut claimed: BTreeSet<String> = BTreeSet::new();
    let mut read = |plan: &SignalPlan| -> Result<(), EventProjectionError> {
        for change in signal_changes(plan, points) {
            ticks.push(change?.0);
        }
        if let SignalPlan::Digital { grid, .. } | SignalPlan::Bus { grid, .. } = plan
            && let Some(period) = grid.hdl_period()
        {
            hdl_period = Some(hdl_period.map_or(period, |known| known.min(period)));
        }
        Ok(())
    };
    for bus in digital_buses {
        let reference = bus_reference(bus);
        let name = checked_node_name(&reference)?;
        let mut members = Vec::with_capacity(bus.members.len());
        let mut total = 0usize;
        let mut hdl_precision = Some(Value::INFINITY);
        for member in &bus.members {
            let key = canonical_event_name(member);
            let (index, trace) = digital_traces
                .iter()
                .enumerate()
                .find(|(_, trace)| canonical_event_name(&trace.node_name) == key)
                .ok_or_else(|| DigitalBusError::UnknownMember {
                    bus: bus.name.clone(),
                    member: member.clone(),
                })?;
            let mut times = SignalTimes::default();
            for point in points.digital(index) {
                times.place(&trace.node_name, point?.0)?;
                total = total.saturating_add(1);
            }
            hdl_precision = hdl_precision
                .zip(trace.hdl_time_precision)
                .map(|(finest, precision)| finest.min(precision));
            members.push(index);
        }
        let grid = TimeGrid::of(
            &reference,
            hdl_precision.filter(|precision| precision.is_finite()),
        )?;
        // [`bus_events`]' refusals, with its numbers.
        let too_large = |cells| EventProjectionError::BusTooLarge {
            bus: bus.name.clone(),
            source: BusReassemblyTooLarge {
                width: members.len(),
                cells,
            },
        };
        if total > MAX_BUS_EVENT_CELLS {
            return Err(too_large(total));
        }
        if total > 0 {
            let mut times = BusTimes::open(&members, points);
            let mut distinct = 0usize;
            while times.next()?.is_some() {
                distinct += 1;
            }
            let cells = distinct.saturating_mul(members.len());
            if cells > MAX_BUS_EVENT_CELLS {
                return Err(too_large(cells));
            }
        }
        let plan = SignalPlan::Bus {
            reference,
            members,
            grid,
        };
        read(&plan)?;
        plans.push(plan);
        declared.push((name, VcdSignalKind::Logic, bus.width() as u32));
        claimed.extend(
            bus.members
                .iter()
                .map(|member| canonical_event_name(member)),
        );
    }
    for (index, trace) in digital_traces.iter().enumerate() {
        if claimed.contains(&canonical_event_name(&trace.node_name)) {
            continue;
        }
        let plan = SignalPlan::Digital {
            trace: index,
            node: trace.node_name.clone(),
            grid: TimeGrid::of(&trace.node_name, trace.hdl_time_precision)?,
        };
        read(&plan)?;
        plans.push(plan);
        declared.push((
            checked_node_name(&trace.node_name)?,
            VcdSignalKind::Logic,
            1,
        ));
    }
    for (index, trace) in real_traces.iter().enumerate() {
        let plan = SignalPlan::Real {
            trace: index,
            node: trace.node_name.clone(),
        };
        read(&plan)?;
        plans.push(plan);
        declared.push((
            checked_node_name(&trace.node_name)?,
            VcdSignalKind::Real,
            REAL_VARIABLE_WIDTH,
        ));
    }

    let timescale = ticks.timescale(hdl_period)?;
    let mut header = VcdDocument::new(timescale);
    header.version = VCD_WRITER_VERSION.to_string();
    header.signals = declared
        .into_iter()
        .map(|(name, kind, width)| VcdSignal {
            identifier: String::new(),
            variables: vec![VcdVariable {
                scope: vec![scope.to_string()],
                name,
            }],
            width,
            kind,
            changes: Vec::new(),
        })
        .collect();
    header.assign_canonical_identifiers();
    Ok(EventVcdStream {
        header,
        plans,
        period: u128::from(timescale.femtoseconds()),
        source,
    })
}

impl<S: EventSource> EventVcdStream<S> {
    /// The dump's declarations, with no changes.
    pub fn header(&self) -> &VcdDocument {
        &self.header
    }

    /// Every change, read from the source again, as `(tick, signal, value)` in the order
    /// [`crate::io::write_vcd`] writes them: by tick, then by signal, each signal's in order.
    pub fn changes(
        &self,
    ) -> impl Iterator<Item = Result<(u64, usize, VcdValue), EventProjectionError>> + '_ {
        let mut signals = self
            .plans
            .iter()
            .map(|plan| signal_changes(plan, &self.source))
            .collect::<Vec<_>>();
        // Each signal with its next change, which the heap orders by tick and then by signal.
        let mut heads: Vec<Option<VcdValue>> = Vec::with_capacity(signals.len());
        let mut order = BinaryHeap::new();
        let mut failed = None;
        for (index, signal) in signals.iter_mut().enumerate() {
            heads.push(match signal.next() {
                Some(Ok((femtoseconds, value))) => {
                    order.push(Reverse((self.tick(femtoseconds), index)));
                    Some(value)
                }
                Some(Err(error)) => {
                    failed.get_or_insert(error);
                    None
                }
                None => None,
            });
        }
        std::iter::from_fn(move || {
            if let Some(error) = failed.take() {
                order.clear();
                return Some(Err(error));
            }
            let Reverse((tick, index)) = order.pop()?;
            let (head, signal) = heads.get_mut(index).zip(signals.get_mut(index))?;
            let value = head.take()?;
            match signal.next() {
                Some(Ok((femtoseconds, next))) => {
                    *head = Some(next);
                    order.push(Reverse((self.tick(femtoseconds), index)));
                }
                Some(Err(error)) => failed = Some(error),
                None => {}
            }
            Some(Ok((tick, index, value)))
        })
    }

    /// The timescale admits every quotient to the u64 range.
    fn tick(&self, femtoseconds: u128) -> u64 {
        (femtoseconds / self.period) as u64
    }
}

/// One signal's changes on its grid, as `(femtoseconds, value)`.
type Changes<'s> = Box<dyn Iterator<Item = Result<(u128, VcdValue), EventProjectionError>> + 's>;

fn signal_changes<'s, S: EventSource>(plan: &'s SignalPlan, source: &'s S) -> Changes<'s> {
    match plan {
        SignalPlan::Digital { trace, node, grid } => Box::new(GridStream::new(
            node,
            *grid,
            false,
            source.digital(*trace).map(|point| {
                point.map(|(time, value)| {
                    (time, VcdValue::Logic(vec![digital_value_to_vcd_bit(value)]))
                })
            }),
        )),
        SignalPlan::Real { trace, node } => Box::new(GridStream::new(
            node,
            TimeGrid::Femtosecond,
            true,
            source
                .real(*trace)
                .map(|point| point.map(|(time, value)| (time, VcdValue::Real(value)))),
        )),
        SignalPlan::Bus {
            reference,
            members,
            grid,
        } => Box::new(GridStream::new(
            reference,
            *grid,
            false,
            BusRows::open(members, source),
        )),
    }
}

/// [`GridChanges`] and [`push_change`] over points read one at a time. On an HDL's grid a
/// change waits until a later tick shows it is not replaced by another on its own tick.
struct GridStream<'s, I> {
    node: &'s str,
    grid: TimeGrid,
    /// A real signal, whose values must be finite.
    real: bool,
    points: I,
    times: SignalTimes,
    /// The last change given.
    given: Option<VcdValue>,
    /// The change an HDL's grid holds back, which a change on its tick replaces.
    pending: Option<(u128, VcdValue)>,
}

impl<'s, I> GridStream<'s, I> {
    fn new(node: &'s str, grid: TimeGrid, real: bool, points: I) -> Self {
        Self {
            node,
            grid,
            real,
            points,
            times: SignalTimes::default(),
            given: None,
            pending: None,
        }
    }

    /// Place one point; the change it lets out, if any.
    fn push(
        &mut self,
        time: Value,
        value: VcdValue,
    ) -> Result<Option<(u128, VcdValue)>, EventProjectionError> {
        let node = self.node;
        let (tick, merged) = self.times.place(node, time)?;
        if self.real && matches!(value, VcdValue::Real(real) if !real.is_finite()) {
            return Err(EventProjectionError::NonFiniteValue {
                node: node.to_string(),
                time,
            });
        }
        let TimeGrid::Hdl { precision, period } = self.grid else {
            // [`push_change`].
            if let Some(other_time) = merged
                && let Some(last) = &self.given
            {
                if *last == value {
                    return Ok(None);
                }
                return Err(EventProjectionError::TimeCollision {
                    node: node.to_string(),
                    time,
                    other_time,
                    tick,
                });
            }
            self.given = Some(value.clone());
            return Ok(Some((tick, value)));
        };
        // [`GridChanges::push`] on an HDL's grid.
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
        if self
            .pending
            .as_ref()
            .is_some_and(|(pending, _)| *pending == tick)
        {
            self.pending = None;
        }
        let held = self
            .pending
            .as_ref()
            .map(|(_, value)| value)
            .or(self.given.as_ref());
        if held == Some(&value) {
            return Ok(None);
        }
        let released = self.pending.replace((tick, value));
        if let Some((_, value)) = &released {
            self.given = Some(value.clone());
        }
        Ok(released)
    }
}

impl<I> Iterator for GridStream<'_, I>
where
    I: Iterator<Item = Result<(Value, VcdValue), EventProjectionError>>,
{
    type Item = Result<(u128, VcdValue), EventProjectionError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let Some(point) = self.points.next() else {
                return self.pending.take().map(Ok);
            };
            match point.and_then(|(time, value)| self.push(time, value)) {
                Ok(Some(change)) => return Some(Ok(change)),
                Ok(None) => {}
                Err(error) => {
                    self.pending = None;
                    return Some(Err(error));
                }
            }
        }
    }
}

/// A member's points, one looked ahead.
struct Member<'s> {
    points: EventPoints<'s, u8>,
    next: Option<(Value, u8)>,
}

impl<'s> Member<'s> {
    fn open<S: EventSource>(source: &'s S, trace: usize) -> Result<Self, EventProjectionError> {
        let mut points: EventPoints<'s, u8> = Box::new(
            source
                .digital(trace)
                .map(|point| point.map(|(time, value)| (time, value.event_code()))),
        );
        let next = points.next().transpose()?;
        Ok(Self { points, next })
    }

    fn advance(&mut self) -> Result<(), EventProjectionError> {
        self.next = self.points.next().transpose()?;
        Ok(())
    }
}

/// The distinct times of a bus's members in [`bus_events`]' order.
struct BusTimes<'s> {
    members: Result<Vec<Member<'s>>, Option<EventProjectionError>>,
}

impl<'s> BusTimes<'s> {
    fn open<S: EventSource>(members: &[usize], source: &'s S) -> Self {
        Self {
            members: members
                .iter()
                .map(|&trace| Member::open(source, trace))
                .collect::<Result<_, _>>()
                .map_err(Some),
        }
    }

    fn next(&mut self) -> Result<Option<Value>, EventProjectionError> {
        let members = match &mut self.members {
            Ok(members) => members,
            Err(error) => return error.take().map_or(Ok(None), Err),
        };
        let Some(time) = members
            .iter()
            .filter_map(|member| member.next.map(|(time, _)| time))
            .min_by(Value::total_cmp)
        else {
            return Ok(None);
        };
        for member in members.iter_mut() {
            while member
                .next
                .is_some_and(|(next, _)| next.total_cmp(&time).is_eq())
            {
                member.advance()?;
            }
        }
        Ok(Some(time))
    }
}

/// [`bus_events`] over members read one point at a time: at each distinct time, what each
/// member holds then, as one vector value.
struct BusRows<'s> {
    times: BusTimes<'s>,
    values: Result<Vec<Member<'s>>, Option<EventProjectionError>>,
    held: Vec<Option<u8>>,
}

impl<'s> BusRows<'s> {
    fn open<S: EventSource>(members: &[usize], source: &'s S) -> Self {
        Self {
            times: BusTimes::open(members, source),
            values: members
                .iter()
                .map(|&trace| Member::open(source, trace))
                .collect::<Result<_, _>>()
                .map_err(Some),
            held: vec![None; members.len()],
        }
    }

    fn row(&mut self) -> Result<Option<(Value, VcdValue)>, EventProjectionError> {
        let Some(time) = self.times.next()? else {
            return Ok(None);
        };
        let members = match &mut self.values {
            Ok(members) => members,
            Err(error) => return error.take().map_or(Ok(None), Err),
        };
        for (member, held) in members.iter_mut().zip(self.held.iter_mut()) {
            while let Some((point_time, code)) = member.next {
                if point_time > time {
                    break;
                }
                *held = Some(code);
                member.advance()?;
            }
        }
        let bits = self
            .held
            .iter()
            // As [`bus_signal`] reads each code.
            .map(|&code| event_code_to_vcd_bit(code).unwrap_or(VcdBit::Unknown))
            .collect();
        Ok(Some((time, VcdValue::Logic(bits))))
    }
}

impl Iterator for BusRows<'_> {
    type Item = Result<(Value, VcdValue), EventProjectionError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.row().transpose()
    }
}

/// What [`choose_timescale`] reads of every change's femtoseconds: their greatest common
/// divisor, which a tick period divides exactly when it divides each of them, and the largest.
#[derive(Default)]
struct TickDivisor {
    divisor: u128,
    largest: u128,
}

impl TickDivisor {
    fn push(&mut self, femtoseconds: u128) {
        let (mut a, mut b) = (self.divisor, femtoseconds);
        while b != 0 {
            (a, b) = (b, a % b);
        }
        self.divisor = a;
        self.largest = self.largest.max(femtoseconds);
    }

    /// [`choose_timescale`] of the changes pushed.
    fn timescale(&self, hdl_period: Option<u128>) -> Result<VcdTimescale, EventProjectionError> {
        let finest = VcdTimescale {
            magnitude: VcdMagnitude::One,
            unit: VcdTimeUnit::Femtoseconds,
        };
        let hdl_timescale = hdl_period.and_then(|period| {
            VcdTimescale::ALL
                .into_iter()
                .find(|candidate| u128::from(candidate.femtoseconds()) == period)
        });
        if self.largest == 0 {
            return Ok(hdl_timescale.unwrap_or(finest));
        }
        VcdTimescale::ALL
            .into_iter()
            .filter(|candidate| {
                hdl_period.is_none_or(|cap| u128::from(candidate.femtoseconds()) <= cap)
            })
            .find(|candidate| {
                let period = u128::from(candidate.femtoseconds());
                self.divisor.is_multiple_of(period) && self.largest / period <= u128::from(u64::MAX)
            })
            .ok_or(EventProjectionError::TimescaleRange)
    }
}

#[cfg(test)]
mod tests;

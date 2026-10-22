//! Logic, bus and real lanes as the exact event traces state them.
//!
//! A lane holds the instants at which its value changed and the value from
//! each, read straight off the committed event history. Nothing here is
//! resampled: the `D()`/`E()` waveform columns are a projection onto the
//! analog grid and are never a lane's source.
//!
//! Every lane reads right-continuously. At an event instant it holds the new
//! value, which is what a measure, a probe and the cursor all agree on.

use rspice_core::execution::{BusMemberHistory, bus_events, event_code_to_vcd_bit};
use rspice_core::io::VcdBit;
use rspice_results::events::projection::{BusRadix, BusWord, bus_word};
use rspice_results::events::{
    DigitalBusEvidence, DigitalEventTraceEvidence, RealEventTraceEvidence,
};

/// One level of a scalar digital net, as a VCD bit spells it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicLevel {
    Low,
    High,
    Unknown,
    HighImpedance,
}

impl LogicLevel {
    /// The level an XSPICE event code denotes, through the one mapping the
    /// dump and the bus words also use.
    pub fn from_event_code(code: u8) -> Option<Self> {
        Some(match event_code_to_vcd_bit(Some(code))? {
            VcdBit::Zero => Self::Low,
            VcdBit::One => Self::High,
            VcdBit::Unknown => Self::Unknown,
            VcdBit::HighImpedance => Self::HighImpedance,
        })
    }

    /// The strongly driven code of this level (high impedance has one).
    const fn event_code(self) -> u8 {
        match self {
            Self::Low => 0,
            Self::High => 1,
            Self::Unknown => 2,
            Self::HighImpedance => 12,
        }
    }

    pub const fn symbol(self) -> &'static str {
        match self {
            Self::Low => "0",
            Self::High => "1",
            Self::Unknown => "x",
            Self::HighImpedance => "z",
        }
    }
}

/// The index of the value held at `t`: the last instant at or before it.
fn held_index(times: &[f64], t: f64) -> Option<usize> {
    times
        .partition_point(|&instant| instant <= t)
        .checked_sub(1)
}

/// The first instant strictly after `t`.
fn next_instant(times: &[f64], t: f64) -> Option<f64> {
    times
        .get(times.partition_point(|&instant| instant <= t))
        .copied()
}

/// The last instant strictly before `t`.
fn previous_instant(times: &[f64], t: f64) -> Option<f64> {
    times
        .partition_point(|&instant| instant < t)
        .checked_sub(1)
        .map(|index| times[index])
}

/// Keep only the instants at which the value changed.
///
/// Points at one instant collapse to the last of them, the value the net
/// settled on there; points that repeat the held value are not changes.
fn changes<V: PartialEq>(points: impl IntoIterator<Item = (f64, V)>) -> (Vec<f64>, Vec<V>) {
    let mut times: Vec<f64> = Vec::new();
    let mut values: Vec<V> = Vec::new();
    for (time, value) in points {
        if !time.is_finite() {
            continue;
        }
        if times.last() == Some(&time) {
            times.pop();
            values.pop();
        }
        if values.last() == Some(&value) {
            continue;
        }
        times.push(time);
        values.push(value);
    }
    (times, values)
}

/// A scalar digital event trace: `D(net)`.
#[derive(Debug, Clone, PartialEq)]
pub struct LogicLane {
    name: String,
    times: Vec<f64>,
    levels: Vec<LogicLevel>,
}

impl LogicLane {
    /// A lane over committed `(time, event code)` points in time order.
    /// A code outside the XSPICE table reads as unknown.
    pub fn from_events(
        name: impl Into<String>,
        points: impl IntoIterator<Item = (f64, u8)>,
    ) -> Self {
        let (times, levels) = changes(points.into_iter().map(|(time, code)| {
            (
                time,
                LogicLevel::from_event_code(code).unwrap_or(LogicLevel::Unknown),
            )
        }));
        Self {
            name: name.into(),
            times,
            levels,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Change instants, strictly increasing; the first is the initial value.
    pub fn times(&self) -> &[f64] {
        &self.times
    }

    /// The level from each instant in [`Self::times`].
    pub fn levels(&self) -> &[LogicLevel] {
        &self.levels
    }

    /// The level held at `t`, right-continuously; `None` before the net was
    /// first observed.
    pub fn level_at(&self, t: f64) -> Option<LogicLevel> {
        held_index(&self.times, t).map(|index| self.levels[index])
    }

    pub fn next_edge(&self, t: f64) -> Option<f64> {
        next_instant(&self.times, t)
    }

    pub fn previous_edge(&self, t: f64) -> Option<f64> {
        previous_instant(&self.times, t)
    }

    fn event_points(&self) -> Vec<(f64, u8)> {
        self.times
            .iter()
            .zip(&self.levels)
            .map(|(time, level)| (*time, level.event_code()))
            .collect()
    }
}

/// Where a bus lane's grouping came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusOrigin {
    /// The run declared it: an engine vector port, a deck or a schematic bus.
    Declared,
    /// The reader grouped logic lanes into it.
    User,
}

/// A word of digital nets: `D(bus)`.
#[derive(Debug, Clone, PartialEq)]
pub struct BusLane {
    name: String,
    members: Vec<String>,
    origin: BusOrigin,
    times: Vec<f64>,
    /// The code each member held from each instant, declared MSB first.
    words: Vec<Box<[Option<u8>]>>,
    /// Why the word could not be reassembled, when it could not.
    refusal: Option<String>,
}

impl BusLane {
    /// Reassemble a bus from its members' committed points, MSB first.
    ///
    /// The word at each instant is `rspice_core::execution::bus_events`
    /// reading the member histories, the same reassembly the Events sheet
    /// shows. A bus too large to reassemble keeps its row and says why.
    pub fn assemble(
        name: impl Into<String>,
        members: Vec<String>,
        origin: BusOrigin,
        histories: &[Vec<(f64, u8)>],
    ) -> Self {
        let name = name.into();
        let borrowed: Vec<BusMemberHistory<'_>> = histories
            .iter()
            .map(|points| BusMemberHistory { points })
            .collect();
        let (times, words, refusal) = match bus_events(&borrowed) {
            Ok(table) => {
                let (times, words) = changes(
                    table
                        .into_iter()
                        .map(|(time, codes)| (time, codes.into_boxed_slice())),
                );
                (times, words, None)
            }
            Err(error) => (
                Vec::new(),
                Vec::new(),
                Some(format!(
                    "{} members over {} event cells is too large to draw as one word",
                    error.width, error.cells
                )),
            ),
        };
        Self {
            name,
            members,
            origin,
            times,
            words,
            refusal,
        }
    }

    /// A user bus over logic lanes, the first lane its MSB.
    pub fn from_logic_lanes(name: impl Into<String>, lanes: &[&LogicLane]) -> Self {
        let histories: Vec<_> = lanes.iter().map(|lane| lane.event_points()).collect();
        Self::assemble(
            name,
            lanes.iter().map(|lane| lane.name.clone()).collect(),
            BusOrigin::User,
            &histories,
        )
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Member net names, declared MSB first.
    pub fn members(&self) -> &[String] {
        &self.members
    }

    pub const fn origin(&self) -> BusOrigin {
        self.origin
    }

    pub fn refusal(&self) -> Option<&str> {
        self.refusal.as_deref()
    }

    /// Word-change instants, strictly increasing.
    pub fn times(&self) -> &[f64] {
        &self.times
    }

    /// The member codes held from instant `index`, MSB first.
    pub fn word(&self, index: usize) -> Option<&[Option<u8>]> {
        self.words.get(index).map(AsRef::as_ref)
    }

    /// Whether the word from instant `index` has a bit that is not 0 or 1.
    pub fn word_is_unresolved(&self, index: usize) -> bool {
        self.word(index).is_some_and(|codes| {
            codes.iter().any(|code| {
                !matches!(
                    event_code_to_vcd_bit(*code),
                    Some(VcdBit::Zero | VcdBit::One)
                )
            })
        })
    }

    /// The word from instant `index`, spelled in `radix` or in its fallback.
    pub fn spell(&self, index: usize, radix: BusRadix) -> Option<BusWord> {
        self.word(index).map(|codes| bus_word(codes, radix))
    }

    /// The word held at `t`, right-continuously, spelled in `radix`.
    pub fn spell_at(&self, t: f64, radix: BusRadix) -> Option<BusWord> {
        held_index(&self.times, t).and_then(|index| self.spell(index, radix))
    }

    pub fn next_edge(&self, t: f64) -> Option<f64> {
        next_instant(&self.times, t)
    }

    pub fn previous_edge(&self, t: f64) -> Option<f64> {
        previous_instant(&self.times, t)
    }
}

/// A real-valued event trace: `E(net)`.
#[derive(Debug, Clone, PartialEq)]
pub struct RealLane {
    name: String,
    times: Vec<f64>,
    values: Vec<f64>,
}

impl RealLane {
    /// A lane over committed `(time, value)` points in time order.
    pub fn from_events(
        name: impl Into<String>,
        points: impl IntoIterator<Item = (f64, f64)>,
    ) -> Self {
        // Compared by bit pattern: a step to the same value is no step, and
        // NaN is a value a real net can hold.
        let (times, values) = changes(
            points
                .into_iter()
                .map(|(time, value)| (time, value.to_bits())),
        );
        Self {
            name: name.into(),
            times,
            values: values.into_iter().map(f64::from_bits).collect(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn times(&self) -> &[f64] {
        &self.times
    }

    pub fn values(&self) -> &[f64] {
        &self.values
    }

    /// The value held at `t`, right-continuously.
    pub fn value_at(&self, t: f64) -> Option<f64> {
        held_index(&self.times, t).map(|index| self.values[index])
    }

    pub fn next_edge(&self, t: f64) -> Option<f64> {
        next_instant(&self.times, t)
    }

    pub fn previous_edge(&self, t: f64) -> Option<f64> {
        previous_instant(&self.times, t)
    }

    /// The step plot through `end`: horizontal, then vertical at each
    /// instant, never sloped. Step `k` contributes the vertices
    /// `(t_k, v_{k-1})` and `(t_k, v_k)`; the last value holds to `end`.
    pub fn step_polyline(&self, end: f64) -> (Vec<f64>, Vec<f64>) {
        let mut x = Vec::with_capacity(self.times.len() * 2 + 1);
        let mut y = Vec::with_capacity(self.times.len() * 2 + 1);
        for (index, (time, value)) in self.times.iter().zip(&self.values).enumerate() {
            if index > 0 {
                x.push(*time);
                y.push(self.values[index - 1]);
            }
            x.push(*time);
            y.push(*value);
        }
        if let (Some(last_time), Some(last_value)) = (self.times.last(), self.values.last())
            && end > *last_time
        {
            x.push(end);
            y.push(*last_value);
        }
        (x, y)
    }
}

/// Every lane one transient run's exact event history can draw.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct EventLanes {
    pub logic: Vec<LogicLane>,
    pub buses: Vec<BusLane>,
    pub real: Vec<RealLane>,
}

impl EventLanes {
    /// The lanes of a `TransientEvents` payload: one logic lane per digital
    /// trace, one bus lane per declared bus, one real lane per real trace.
    pub fn from_evidence(
        digital: &[DigitalEventTraceEvidence],
        real: &[RealEventTraceEvidence],
        buses: &[DigitalBusEvidence],
    ) -> Self {
        let points = |trace: &DigitalEventTraceEvidence| {
            trace
                .points
                .iter()
                .map(|point| (point.time_s, point.value_code))
                .collect::<Vec<_>>()
        };
        let buses = buses
            .iter()
            .map(|bus| {
                let histories: Vec<_> = bus
                    .members
                    .iter()
                    .map(|member| {
                        digital
                            .iter()
                            .find(|trace| &trace.node_name == member)
                            .map(points)
                            .unwrap_or_default()
                    })
                    .collect();
                BusLane::assemble(
                    format!("{}[{}:{}]", bus.name, bus.msb, bus.lsb),
                    bus.members.clone(),
                    BusOrigin::Declared,
                    &histories,
                )
            })
            .collect();
        Self {
            logic: digital
                .iter()
                .map(|trace| LogicLane::from_events(trace.node_name.clone(), points(trace)))
                .collect(),
            buses,
            real: real
                .iter()
                .map(|trace| {
                    RealLane::from_events(
                        trace.node_name.clone(),
                        trace.points.iter().map(|point| (point.time_s, point.value)),
                    )
                })
                .collect(),
        }
    }

    pub fn logic(&self, name: &str) -> Option<&LogicLane> {
        self.logic.iter().find(|lane| lane.name == name)
    }

    pub fn real(&self, name: &str) -> Option<&RealLane> {
        self.real.iter().find(|lane| lane.name == name)
    }

    /// A declared bus by its name, with or without its range.
    pub fn bus(&self, name: &str) -> Option<&BusLane> {
        self.buses.iter().find(|lane| {
            lane.name == name
                || lane
                    .name
                    .strip_prefix(name)
                    .is_some_and(|range| range.starts_with('['))
        })
    }

    /// Group logic lanes, named MSB first, into a user bus. `None` when a
    /// member is not a logic lane of this run.
    pub fn user_bus(&self, name: &str, members: &[&str]) -> Option<BusLane> {
        let lanes = members
            .iter()
            .map(|member| self.logic(member))
            .collect::<Option<Vec<_>>>()?;
        Some(BusLane::from_logic_lanes(name, &lanes))
    }
}

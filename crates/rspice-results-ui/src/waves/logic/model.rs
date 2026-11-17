//! Logic, bus and real lanes as the exact event traces state them.
//!
//! A lane holds the instants at which its value changed and the value from
//! each, read straight off the committed event history. Nothing here is
//! resampled: the `D()`/`E()` waveform columns are a projection onto the
//! analog grid and are never a lane's source.
//!
//! Every lane reads right-continuously. At an event instant it holds the new
//! value, which is what a measure, a probe and the cursor all agree on.

use rspice_results::engine_evidence::{
    BusMemberHistory, VcdBit, bus_events, event_code_to_vcd_bit,
};
use rspice_results::events::projection::{
    BusRadix, BusWord, bus_word, bus_word_named, word_unsigned,
};
use rspice_results::events::{
    DigitalBusEvidence, DigitalEventTraceEvidence, RealEventTraceEvidence,
};
use rspice_results::visualization_document::BusValueName;
use std::sync::{Arc, Mutex};

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
    /// The instants, by index, from which the level is unknown.
    unknown: Vec<usize>,
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
        Self::from_changes(name.into(), times, levels)
    }

    /// The lane `digital()` reads an analog signal as: its step waveform,
    /// two samples at each edge, with 0 and 1 the levels and a hole the X.
    /// Any other value is no level and reads as unknown too.
    pub fn from_levels(name: impl Into<String>, x: &[f64], y: &[f64]) -> Self {
        let (times, levels) = changes(x.iter().zip(y).map(|(&time, &value)| {
            let level = if value == 0.0 {
                LogicLevel::Low
            } else if value == 1.0 {
                LogicLevel::High
            } else {
                LogicLevel::Unknown
            };
            (time, level)
        }));
        Self::from_changes(name.into(), times, levels)
    }

    /// A lane over its change instants and the level from each, with the
    /// unknown stretches indexed.
    fn from_changes(name: String, times: Vec<f64>, levels: Vec<LogicLevel>) -> Self {
        let unknown = levels
            .iter()
            .enumerate()
            .filter(|(_, level)| **level == LogicLevel::Unknown)
            .map(|(index, _)| index)
            .collect();
        Self {
            name,
            times,
            levels,
            unknown,
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

    /// The instants, by index and increasing, from which the level is
    /// unknown.
    pub fn unknown(&self) -> &[usize] {
        &self.unknown
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
    /// The instants, by index, from which the word has a bit that is not 0
    /// or 1.
    unresolved: Vec<usize>,
}

/// Whether a word has a bit that is not 0 or 1.
fn word_is_unresolved(codes: &[Option<u8>]) -> bool {
    codes.iter().any(|code| {
        !matches!(
            event_code_to_vcd_bit(*code),
            Some(VcdBit::Zero | VcdBit::One)
        )
    })
}

impl BusLane {
    /// Reassemble a bus from its members' committed points, MSB first.
    ///
    /// The word at each instant is `rspice_results::engine_evidence::bus_events`
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
        let unresolved = words
            .iter()
            .enumerate()
            .filter(|(_, codes)| word_is_unresolved(codes))
            .map(|(index, _)| index)
            .collect();
        Self {
            name,
            members,
            origin,
            times,
            words,
            refusal,
            unresolved,
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
        self.unresolved.binary_search(&index).is_ok()
    }

    /// The instants, by index and increasing, from which the word has a bit
    /// that is not 0 or 1.
    pub fn unresolved(&self) -> &[usize] {
        &self.unresolved
    }

    /// The word from instant `index`, spelled in `radix` or in its fallback.
    pub fn spell(&self, index: usize, radix: BusRadix) -> Option<BusWord> {
        self.word(index).map(|codes| bus_word(codes, radix))
    }

    /// The word from instant `index`, as the reader named it, or else spelled
    /// in `radix`.
    pub fn spell_named(
        &self,
        index: usize,
        radix: BusRadix,
        names: &[BusValueName],
    ) -> Option<BusWord> {
        self.word(index)
            .map(|codes| bus_word_named(codes, radix, names))
    }

    /// The word held at `t`, right-continuously, as the reader named it, or
    /// else spelled in `radix`.
    pub fn spell_at_named(
        &self,
        t: f64,
        radix: BusRadix,
        names: &[BusValueName],
    ) -> Option<BusWord> {
        held_index(&self.times, t).and_then(|index| self.spell_named(index, radix, names))
    }

    /// The unsigned value of the word held at `t`, when its bits are all 0
    /// or 1 and there are at most 64 of them.
    pub fn value_at(&self, t: f64) -> Option<u64> {
        held_index(&self.times, t)
            .and_then(|index| self.word(index))
            .and_then(word_unsigned)
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
    /// A stored run's histories, which its lanes read a window of: its lanes here are named
    /// and hold no points. `None` when every lane holds its whole history.
    stored: Option<StoredHistories>,
    /// The window these lanes were read over, when they are a stored run's.
    window: Option<(f64, f64)>,
}

/// The last few windows a stored run's lanes were read over, each by its bounds' bits, most
/// recent last.
type ReadWindows = Arc<Mutex<Vec<((u64, u64), Arc<EventLanes>)>>>;

/// The histories of a stored run, kept by the lanes that read windows of them, and the last few
/// windows they read.
#[derive(Clone)]
struct StoredHistories {
    digital: Vec<DigitalEventTraceEvidence>,
    real: Vec<RealEventTraceEvidence>,
    buses: Vec<DigitalBusEvidence>,
    windows: ReadWindows,
}

/// Windows of a stored run's lanes kept at once: the view's, and the cursors'.
const STORED_WINDOWS: usize = 4;

impl PartialEq for StoredHistories {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.windows, &other.windows)
    }
}

impl std::fmt::Debug for StoredHistories {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "StoredHistories({} digital, {} real)",
            self.digital.len(),
            self.real.len()
        )
    }
}

impl EventLanes {
    /// The lanes of a `TransientEvents` payload: one logic lane per digital
    /// trace, one bus lane per declared bus, one real lane per real trace.
    ///
    /// A history its run's store keeps is not read here: its lane is named and empty until
    /// [`Self::over`] reads the window a view shows.
    pub fn from_evidence(
        digital: &[DigitalEventTraceEvidence],
        real: &[RealEventTraceEvidence],
        buses: &[DigitalBusEvidence],
    ) -> Self {
        let stored = digital.iter().any(|trace| trace.points.is_stored())
            || real.iter().any(|trace| trace.points.is_stored());
        if !stored {
            return Self::read(digital, real, buses);
        }
        let mut lanes = Self::read(
            &digital
                .iter()
                .map(|trace| DigitalEventTraceEvidence {
                    node_name: trace.node_name.clone(),
                    points: Default::default(),
                })
                .collect::<Vec<_>>(),
            &real
                .iter()
                .map(|trace| RealEventTraceEvidence {
                    node_name: trace.node_name.clone(),
                    points: Default::default(),
                })
                .collect::<Vec<_>>(),
            buses,
        );
        lanes.stored = Some(StoredHistories {
            digital: digital.to_vec(),
            real: real.to_vec(),
            buses: buses.to_vec(),
            windows: Arc::default(),
        });
        lanes
    }

    /// Whether these lanes are a stored run's, read a window at a time by [`Self::over`].
    pub fn reads_windows(&self) -> bool {
        self.stored.is_some()
    }

    /// The window these lanes were read over: `None` for lanes that hold every point.
    pub fn window(&self) -> Option<(f64, f64)> {
        self.window
    }

    /// The lanes over `t0 ..= t1`. A stored run's lanes read the changes there, the one held
    /// into the window and the one after it, from its store, and keep the last few windows they
    /// read; a lane a reader added keeps every point. Lanes that hold every point are
    /// themselves.
    pub fn over(self: &Arc<Self>, t0: f64, t1: f64) -> Arc<Self> {
        let Some(stored) = &self.stored else {
            return Arc::clone(self);
        };
        let (t0, t1) = (t0.min(t1), t0.max(t1));
        let key = (t0.to_bits(), t1.to_bits());
        let mut windows = stored.windows.lock().unwrap_or_else(|p| p.into_inner());
        if let Some(at) = windows.iter().position(|(held, _)| *held == key) {
            let hit = windows.remove(at);
            let lanes = Arc::clone(&hit.1);
            windows.push(hit);
            return lanes;
        }
        // Each history is read as far on each side as its lane's value changes, so an edge
        // stepped to from the window is one. A history that cannot be read shows no points;
        // its failure is reported with the run's other unreadable stored data.
        let level = |code: u8| LogicLevel::from_event_code(code).unwrap_or(LogicLevel::Unknown);
        let digital: Vec<_> = stored
            .digital
            .iter()
            .map(|trace| DigitalEventTraceEvidence {
                node_name: trace.node_name.clone(),
                points: trace
                    .points
                    .around(t0, t1, |a, b| level(a.value_code) == level(b.value_code))
                    .unwrap_or_default()
                    .into(),
            })
            .collect();
        let real: Vec<_> = stored
            .real
            .iter()
            .map(|trace| RealEventTraceEvidence {
                node_name: trace.node_name.clone(),
                points: trace
                    .points
                    .around(t0, t1, |a, b| a.value == b.value)
                    .unwrap_or_default()
                    .into(),
            })
            .collect();
        let mut lanes = Self::read(&digital, &real, &stored.buses);
        // A lane a reader added (a `digital()` expression's) follows the stored ones.
        lanes
            .logic
            .extend(self.logic.iter().skip(stored.digital.len()).cloned());
        lanes
            .buses
            .extend(self.buses.iter().skip(stored.buses.len()).cloned());
        lanes
            .real
            .extend(self.real.iter().skip(stored.real.len()).cloned());
        lanes.window = Some((t0, t1));
        let lanes = Arc::new(lanes);
        if windows.len() == STORED_WINDOWS {
            windows.remove(0);
        }
        windows.push((key, Arc::clone(&lanes)));
        lanes
    }

    /// Every lane of histories that hold every point.
    fn read(
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
            stored: None,
            window: None,
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

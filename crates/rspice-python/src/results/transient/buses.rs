//! Typed access to the digital buses a result declares, and to their events.
//!
//! A bus is a declaration, not a recording. The engine commits state per node,
//! so a bus has no points of its own: its value at a time is each member's held
//! value in declaration order, declared most significant bit first. Nothing in
//! this module performs that reassembly — [`rspice_core::execution::bus_events`]
//! is the one implementation of it, shared with the VCD projection, the rawfile
//! bus plots and the browser binding, so a word read here is the word every
//! other route shows.
//!
//! # Two containers, one pair of accessors
//!
//! `TransientResult` and `CompressedTransientResult` both carry the table and
//! the member traces it names, and compression never touches an event history,
//! so both answer with the same rows. The bodies below take the two slices
//! rather than either container.
//!
//! # What a row spells
//!
//! `bits` are the members' `0..=12` event codes, the encoding the typed
//! document, the rawfile event plots and the accepted-sample hook all speak,
//! with `None` for a member the run has not stated a value for yet — a missing
//! bit, which is not a bit that is zero. `value` is the same word in VCD's four
//! states, which is what a logic viewer shows and what `to_vcd()` writes; both
//! spellings come from core's own
//! [`rspice_core::execution::event_code_to_vcd_bit`] — the same call the VCD
//! projection and the browser handle make — so they cannot drift apart.

use super::*;

use rspice_core::SimulationError;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::{DigitalBusDeclaration, DigitalTrace};
use rspice_core::execution::{BusMemberHistory, bus_events, event_code_to_vcd_bit};
use rspice_core::xspice::DigitalValue;

/// One declared digital bus: the word, and the conductors that carry it.
///
/// The range is carried exactly as it was declared. A descending `[7:0]` and an
/// ascending `[0:7]` are different declarations and neither is normalized,
/// because the bit order is the author's; `members` runs from the declared MSB
/// to the declared LSB either way.
#[pyclass(name = "DigitalBus", module = "rspice")]
#[derive(Debug, PartialEq, Eq)]
pub struct PyDigitalBus {
    /// Bus name, without any range suffix.
    #[pyo3(get)]
    pub name: String,
    /// Declared most significant index, exactly as declared.
    #[pyo3(get)]
    pub msb: i64,
    /// Declared least significant index, exactly as declared.
    #[pyo3(get)]
    pub lsb: i64,
    /// Member node names in declaration order, declared MSB first.
    #[pyo3(get)]
    pub members: Vec<String>,
    /// Who declared this bus: `engine`, `schematic` or `import`.
    #[pyo3(get)]
    pub source: String,
}

#[pymethods]
impl PyDigitalBus {
    fn __repr__(&self) -> String {
        format!(
            "DigitalBus(name='{}', msb={}, lsb={}, members={:?}, source='{}')",
            self.name, self.msb, self.lsb, self.members, self.source
        )
    }
}

/// One time at which at least one member of a bus changed, and the whole word
/// at it.
///
/// A member that did not change at this time carries the value it held, because
/// a digital history records only changes; a member with no recorded point yet
/// carries `None` in `bits` and `x` in `value`.
#[pyclass(name = "BusEvent", module = "rspice")]
#[derive(Debug, PartialEq)]
pub struct PyBusEvent {
    /// Accepted analog time, in seconds, at which the word took this value.
    #[pyo3(get)]
    pub time_s: f64,
    /// One `0..=12` XSPICE event code per member, declared MSB first, or
    /// `None` for a member the run has not stated a value for yet.
    #[pyo3(get)]
    pub bits: Vec<Option<u8>>,
    /// The same word in VCD's four states — `0`, `1`, `x`, `z` per member,
    /// declared MSB first. The drive strength `bits` carries is not in it,
    /// because VCD never stored one.
    #[pyo3(get)]
    pub value: String,
}

#[pymethods]
impl PyBusEvent {
    fn __repr__(&self) -> String {
        format!(
            "BusEvent(time_s={:.6e}, value='{}')",
            self.time_s, self.value
        )
    }
}

/// The declared bus table of one result, in declaration order.
pub(crate) fn digital_bus_list(buses: &[DigitalBusDeclaration]) -> Vec<PyDigitalBus> {
    buses
        .iter()
        .map(|bus| PyDigitalBus {
            name: bus.name.clone(),
            msb: bus.msb,
            lsb: bus.lsb,
            members: bus.members.clone(),
            source: digital_bus_source_label(bus.source).to_string(),
        })
        .collect()
}

/// Why a bus could not be read, at the resolution the exception types have.
///
/// Keeping this apart from `PyErr` is what lets the reassembly be tested
/// without an interpreter: building a `PyErr` attaches to one.
#[derive(Debug, PartialEq, Eq)]
enum BusAccessError {
    /// The result declares no such bus. A name error, like an unknown node.
    Unknown(String),
    /// The bus exists but cannot be answered: the result contradicts its own
    /// declaration, or the history is past what this accessor materializes.
    Invalid(String),
}

impl BusAccessError {
    fn into_pyerr(self) -> PyErr {
        match self {
            Self::Unknown(message) => crate::errors::key_error(message),
            Self::Invalid(message) => crate::errors::value_error(message),
        }
    }
}

/// A declaration and the member histories it names, resolved against a result.
struct ResolvedBus<'a> {
    name: &'a str,
    members: Vec<&'a DigitalTrace>,
}

/// Find a bus by name and bind every member to the history that carries it.
///
/// The name resolves ASCII-case-insensitively, as a deck node name and every
/// other event route's name does. The row ceiling is checked here, before any
/// history is copied: a bus has at most as many events as its members have
/// committed changes between them, so that sum bounds the work exactly.
fn resolve_bus<'a>(
    buses: &'a [DigitalBusDeclaration],
    digital_traces: &'a [DigitalTrace],
    name: &str,
) -> Result<ResolvedBus<'a>, BusAccessError> {
    let bus = buses
        .iter()
        .find(|bus| bus.name.eq_ignore_ascii_case(name))
        .ok_or_else(|| {
            BusAccessError::Unknown(format!(
                "no digital bus named '{name}' is declared by this result; it declares {}. A run \
                 declares a bus for a vector boundary port of a mixed Verilog-AMS module, and an \
                 imported artifact declares whatever its vector variables did",
                if buses.is_empty() {
                    "none".to_string()
                } else {
                    format!(
                        "{:?}",
                        buses.iter().map(|bus| &bus.name).collect::<Vec<_>>()
                    )
                }
            ))
        })?;

    let mut members: Vec<&DigitalTrace> = Vec::with_capacity(bus.members.len());
    for member in &bus.members {
        members.push(
            digital_traces
                .iter()
                .find(|trace| trace.node_name.eq_ignore_ascii_case(member))
                .ok_or_else(|| {
                    BusAccessError::Invalid(format!(
                        "digital bus '{}' declares member '{member}', which this result records no \
                         event history for; a bus is retained whole or not at all, so this result \
                         contradicts its own declaration",
                        bus.name
                    ))
                })?,
        );
    }

    let changes = members
        .iter()
        .map(|trace| trace.points.len())
        .fold(0usize, usize::saturating_add);
    if changes > MAX_DIGITAL_EVENT_ROWS {
        return Err(BusAccessError::Invalid(format!(
            "digital bus '{}' has {changes} committed member changes and so up to that many \
             events, past the {MAX_DIGITAL_EVENT_ROWS}-row limit this accessor materializes; read \
             the whole history through document() or write it with to_vcd()",
            bus.name
        )));
    }

    Ok(ResolvedBus {
        name: &bus.name,
        members,
    })
}

/// Reassemble one bus out of its members, through core's one implementation.
///
/// The histories are copied into the `(time, code)` form core reassembles from
/// rather than borrowed, because that is the encoding every consumer speaks
/// while a `DigitalTrace` stores the pair as a value. `resolve_bus` has already
/// bounded that copy.
fn reassemble(
    bus: &ResolvedBus<'_>,
    abort: &dyn AbortSignal,
) -> Result<Vec<PyBusEvent>, SimulationError> {
    let mut histories: Vec<Vec<(f64, u8)>> = Vec::new();
    histories
        .try_reserve_exact(bus.members.len())
        .map_err(|_| allocation_error(bus.name))?;
    for trace in &bus.members {
        let mut points: Vec<(f64, u8)> = Vec::new();
        points
            .try_reserve_exact(trace.points.len())
            .map_err(|_| allocation_error(bus.name))?;
        for (index, point) in trace.points.iter().enumerate() {
            if index.is_multiple_of(ABORT_POLL_ROWS) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            points.push((point.time, point.value.event_code()));
        }
        histories.push(points);
    }
    let borrowed: Vec<BusMemberHistory<'_>> = histories
        .iter()
        .map(|points| BusMemberHistory { points })
        .collect();

    let events = bus_events(&borrowed);
    let mut rows: Vec<PyBusEvent> = Vec::new();
    rows.try_reserve_exact(events.len())
        .map_err(|_| allocation_error(bus.name))?;
    for (index, (time, bits)) in events.into_iter().enumerate() {
        if index.is_multiple_of(ABORT_POLL_ROWS) && abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let value = vcd_spelling(bus.name, time, &bits)?;
        rows.push(PyBusEvent {
            time_s: time,
            bits,
            value,
        });
    }
    Ok(rows)
}

fn allocation_error(bus: &str) -> SimulationError {
    SimulationError::Circuit(format!(
        "could not allocate the member histories of digital bus '{bus}'"
    ))
}

/// One bus event in VCD's four states, declared MSB first.
///
/// Core spells a member — held code or not yet observed — in exactly one
/// place, and this is the same call the VCD projection and the browser handle
/// make, so one bus word cannot read three ways. A code that decodes to no bit
/// cannot come from an event history this build recorded — every code is
/// `DigitalValue::event_code` of a real value — so it is refused by number
/// rather than spelled as anything.
fn vcd_spelling(bus: &str, time: f64, bits: &[Option<u8>]) -> Result<String, SimulationError> {
    let mut spelled = String::with_capacity(bits.len());
    for bit in bits {
        let spelling = event_code_to_vcd_bit(*bit).ok_or_else(|| {
            SimulationError::Circuit(format!(
                "digital bus '{bus}' holds the unrecognized event code {} at {time} s",
                // Only a recorded code can fail to spell; an unobserved member
                // is `x`, so this branch is always reached with a `Some`.
                bit.unwrap_or_default()
            ))
        })?;
        spelled.push(spelling.as_char());
    }
    Ok(spelled)
}

/// Every event of one declared bus, reassembled by core from its members.
///
/// The reassembly runs on the interruptible worker with the GIL released and
/// the abort polled, for the reason the digital accessor states: the history is
/// borrowed across that release, which is sound because this class exposes no
/// `&mut self` entry point and no `__setstate__`.
pub(crate) fn bus_event_rows(
    py: Python<'_>,
    buses: &[DigitalBusDeclaration],
    digital_traces: &[DigitalTrace],
    name: &str,
) -> PyResult<Vec<PyBusEvent>> {
    let bus = resolve_bus(buses, digital_traces, name).map_err(BusAccessError::into_pyerr)?;
    crate::abort::run_interruptible_unregistered(py, |abort| reassemble(&bus, abort))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::engine::{DigitalBusSource, DigitalTracePoint};
    use rspice_core::xspice::{DigitalState, DigitalStrength};

    /// The accessor without its interpreter: the same resolution, the same
    /// bound and the same reassembly, so a test exercises the shipped path.
    fn rows(
        buses: &[DigitalBusDeclaration],
        digital_traces: &[DigitalTrace],
        name: &str,
    ) -> Result<Vec<PyBusEvent>, BusAccessError> {
        let bus = resolve_bus(buses, digital_traces, name)?;
        reassemble(&bus, &NoAbort).map_err(|error| BusAccessError::Invalid(error.to_string()))
    }

    fn trace(node: &str, points: &[(f64, DigitalState, DigitalStrength)]) -> DigitalTrace {
        DigitalTrace {
            node_name: node.to_string(),
            points: points
                .iter()
                .map(|(time, state, strength)| DigitalTracePoint {
                    time: *time,
                    value: DigitalValue::new(*state, *strength),
                })
                .collect(),
        }
    }

    /// A two-bit bus on a five-nanosecond grid: the history BUS-L2's
    /// `output [1:0] count` records, so the word counts `00 01 10 11`.
    fn counter() -> (Vec<DigitalBusDeclaration>, Vec<DigitalTrace>) {
        let traces = vec![
            trace(
                "COUNT#1",
                &[
                    (0.0, DigitalState::Zero, DigitalStrength::Strong),
                    (1.0e-8, DigitalState::One, DigitalStrength::Strong),
                ],
            ),
            trace(
                "COUNT#0",
                &[
                    (0.0, DigitalState::Zero, DigitalStrength::Strong),
                    (5.0e-9, DigitalState::One, DigitalStrength::Strong),
                    (1.0e-8, DigitalState::Zero, DigitalStrength::Strong),
                    (1.5e-8, DigitalState::One, DigitalStrength::Strong),
                ],
            ),
        ];
        let bus = DigitalBusDeclaration::new(
            "x1.count",
            1,
            0,
            vec!["COUNT#1".to_string(), "COUNT#0".to_string()],
            DigitalBusSource::Engine,
        )
        .expect("the fixture declaration is well formed");
        (vec![bus], traces)
    }

    #[test]
    fn a_declaration_carries_its_range_and_its_members_in_order() {
        let (buses, _) = counter();
        let listed = digital_bus_list(&buses);
        assert_eq!(listed.len(), 1);
        assert_eq!(listed[0].name, "x1.count");
        assert_eq!((listed[0].msb, listed[0].lsb), (1, 0));
        assert_eq!(listed[0].members, ["COUNT#1", "COUNT#0"]);
        assert_eq!(listed[0].source, "engine");
    }

    /// An ascending range is carried as written rather than normalized, and its
    /// members stay in the order the declaration gave them.
    #[test]
    fn an_ascending_range_is_not_normalized() {
        let bus = DigitalBusDeclaration::new(
            "d",
            0,
            1,
            vec!["d0".to_string(), "d1".to_string()],
            DigitalBusSource::Import,
        )
        .expect("an ascending declaration is well formed");
        let listed = digital_bus_list(std::slice::from_ref(&bus));
        assert_eq!((listed[0].msb, listed[0].lsb), (0, 1));
        assert_eq!(listed[0].members, ["d0", "d1"]);
        assert_eq!(listed[0].source, "import");
    }

    #[test]
    fn a_bus_event_is_every_time_a_member_changed_with_the_whole_word() {
        let (buses, traces) = counter();
        let rows = rows(&buses, &traces, "x1.count").expect("the bus answers");
        assert_eq!(
            rows.iter()
                .map(|row| (row.time_s, row.value.as_str()))
                .collect::<Vec<_>>(),
            [(0.0, "00"), (5.0e-9, "01"), (1.0e-8, "10"), (1.5e-8, "11")]
        );
        assert_eq!(rows[2].bits, vec![Some(1), Some(0)]);
    }

    /// The row ceiling is checked against the members' committed changes,
    /// which is the exact upper bound on the number of events a bus can have,
    /// so the refusal comes before the work rather than after it.
    #[test]
    fn a_history_past_the_row_ceiling_is_refused_by_number() {
        let mut points: Vec<(f64, DigitalState, DigitalStrength)> =
            Vec::with_capacity(MAX_DIGITAL_EVENT_ROWS + 1);
        for index in 0..=MAX_DIGITAL_EVENT_ROWS {
            points.push((
                index as f64 * 1e-12,
                DigitalState::Zero,
                DigitalStrength::Strong,
            ));
        }
        let traces = vec![trace("wide#1", &points), trace("wide#0", &points)];
        let bus = DigitalBusDeclaration::new(
            "wide",
            1,
            0,
            vec!["wide#1".to_string(), "wide#0".to_string()],
            DigitalBusSource::Engine,
        )
        .expect("the fixture declaration is well formed");
        let error = rows(std::slice::from_ref(&bus), &traces, "wide")
            .expect_err("a history past the ceiling must be refused");
        let BusAccessError::Invalid(message) = &error else {
            panic!("a ceiling is a value error, not {error:?}");
        };
        assert!(message.contains("committed member changes"), "{message}");
        assert!(message.contains("to_vcd()"), "{message}");
    }

    /// The name resolves the way a deck node name does.
    #[test]
    fn a_bus_name_resolves_case_insensitively() {
        let (buses, traces) = counter();
        assert_eq!(
            rows(&buses, &traces, "X1.COUNT").expect("case does not matter"),
            rows(&buses, &traces, "x1.count").expect("the bus answers"),
        );
    }

    /// A member with no recorded point yet is a missing bit, not a zero one.
    #[test]
    fn a_member_not_yet_observed_is_missing_rather_than_zero() {
        let traces = vec![
            trace(
                "q#1",
                &[(1.0e-8, DigitalState::One, DigitalStrength::Strong)],
            ),
            trace("q#0", &[(0.0, DigitalState::Zero, DigitalStrength::Strong)]),
        ];
        let bus = DigitalBusDeclaration::new(
            "q",
            1,
            0,
            vec!["q#1".to_string(), "q#0".to_string()],
            DigitalBusSource::Engine,
        )
        .expect("the fixture declaration is well formed");
        let rows = rows(std::slice::from_ref(&bus), &traces, "q").expect("the bus answers");
        assert_eq!(rows[0].bits, vec![None, Some(0)]);
        assert_eq!(rows[0].value, "x0");
    }

    /// The four-state spelling drops the strength the codes keep: the two
    /// answer different questions about one value.
    #[test]
    fn the_four_state_spelling_drops_the_strength_the_codes_keep() {
        let traces = vec![
            trace("b#1", &[(0.0, DigitalState::HighZ, DigitalStrength::HighZ)]),
            trace(
                "b#0",
                &[(0.0, DigitalState::OneR, DigitalStrength::Resistive)],
            ),
        ];
        let bus = DigitalBusDeclaration::new(
            "b",
            1,
            0,
            vec!["b#1".to_string(), "b#0".to_string()],
            DigitalBusSource::Schematic,
        )
        .expect("the fixture declaration is well formed");
        let rows = rows(std::slice::from_ref(&bus), &traces, "b").expect("b answers");
        assert_eq!(rows[0].value, "z1");
        assert_eq!(
            rows[0].bits,
            vec![
                Some(DigitalValue::new(DigitalState::HighZ, DigitalStrength::HighZ).event_code()),
                Some(
                    DigitalValue::new(DigitalState::OneR, DigitalStrength::Resistive).event_code()
                ),
            ]
        );
    }

    #[test]
    fn an_undeclared_bus_is_refused_by_name_listing_the_declared_ones() {
        let (buses, traces) = counter();
        let error = rows(&buses, &traces, "data").expect_err("an undeclared bus must be refused");
        let BusAccessError::Unknown(message) = &error else {
            panic!("an unknown bus is a name error, not {error:?}");
        };
        assert!(message.contains("'data'"), "{message}");
        assert!(message.contains("x1.count"), "{message}");
    }

    #[test]
    fn a_result_with_no_bus_says_so_rather_than_answering_empty() {
        let (_, traces) = counter();
        let error = rows(&[], &traces, "x1.count").expect_err("nothing is declared");
        let BusAccessError::Unknown(message) = &error else {
            panic!("an unknown bus is a name error, not {error:?}");
        };
        assert!(message.contains("declares none"), "{message}");
    }

    /// A declaration whose member has no trace is a broken result, not an empty
    /// answer: the refusal says which member, and why it cannot happen.
    #[test]
    fn a_member_with_no_trace_is_refused_rather_than_skipped() {
        let (buses, mut traces) = counter();
        traces.pop();
        let error =
            rows(&buses, &traces, "x1.count").expect_err("a missing member must be refused");
        let BusAccessError::Invalid(message) = &error else {
            panic!("a broken result is a value error, not {error:?}");
        };
        assert!(message.contains("COUNT#0"), "{message}");
        assert!(message.contains("whole or not at all"), "{message}");
    }
}

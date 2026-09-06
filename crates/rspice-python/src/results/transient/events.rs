//! Typed access to a transient's XSPICE event histories, digital and real.
//!
//! An event history is not a column on the analog grid. It is the sparse list
//! of values the event solver committed, at the times it committed them, and
//! the value carries a drive strength that the flattened `D(node)` projection
//! drops: `saved_signals`, `to_csv` and the raw exporters all read that
//! flattened column, so none of them can answer what a node actually held.
//! The shared document's JSON view does carry the histories in full, but only
//! as untyped Python data a caller has to walk. This is the typed route.
//!
//! A value is spelled with the labels the shared result document publishes —
//! core's own `DigitalStateTag`/`DigitalStrengthTag`, which this result's own
//! pickle carries too — so one node's history reads the same whether it
//! arrived through this accessor, through a pickle, through `document()`, or
//! through the browser binding. The `0..=12` event code that encodes the same
//! pair as a single integer is carried beside them, because that is the
//! spelling the rawfile event plots, the accepted-sample hook and the browser
//! worker contract use.
//!
//! `CompressedTransientResult::digital_trace` answers the same question with
//! the *other* core spelling, `rspice_core::engine::digital_state_tag`, which
//! hyphenates where this one underscores (`zero-resistive` against
//! `zero_resistive`). That is why this accessor is not named `digital_trace`:
//! one name returning two spellings of one state would be worse than two
//! names. The two vocabularies want converging, which is a change to a
//! published accessor rather than a new one.
//!
//! A **real** event node has no vocabulary problem and no flattened column at
//! all: [`real_nodes`] and [`real_trace`] are the same names and the same row
//! shape `CompressedTransientResult` publishes, so the two containers answer
//! one question one way.
//!
//! # The dump
//!
//! [`vcd_bytes`] is the projection `rspice run -f vcd` publishes, not a second
//! one: the core [`event_vcd_document`] under one `$scope module events`,
//! serialised by the core `write_vcd`.
//! `crates/rspice-cli/src/commands/vcd_io.rs` builds it the same way, and
//! `a_dump_is_byte_identical_to_the_command_line_projection` pins the bytes.

use super::*;

use rspice_core::AbortSignal;
use rspice_core::engine::{DigitalTrace, DigitalTracePoint, TransientResult};
use rspice_core::execution::event_vcd_document;

/// The `$scope module` every RSpice dump declares its nodes under.
///
/// `rspice-cli`'s `commands/vcd_io.rs` and the GUI's exporter both use this
/// exact string. Three dumps of one run differ the moment they disagree.
const EVENT_SCOPE: &str = "events";

/// Largest event history this binding will materialise as Python objects.
///
/// One row is a small Python object holding two label strings, so two million
/// of them cost the same order of memory as the document view's own 512 MiB
/// budget. A history past the ceiling is refused by number rather than
/// silently truncated, and the refusal names the two routes that carry an
/// unbounded history: `document()` and `to_vcd()`.
///
/// [`super::buses`] charges the same ceiling against the number of events a
/// bus can have, so one accessor's ceiling is the binding's ceiling.
pub(super) const MAX_DIGITAL_EVENT_ROWS: usize = 2_000_000;

/// Points converted between two cancellation checks.
///
/// The conversion of one point is a few tens of nanoseconds, so polling every
/// point would cost more than the work; polling every few thousand keeps the
/// observed latency of a `KeyboardInterrupt` under a millisecond.
pub(super) const ABORT_POLL_ROWS: usize = 4_096;

/// One committed digital event: when it happened, and what the node became.
///
/// `state` and `strength` are the two independent axes of an XSPICE digital
/// value, spelled exactly as the pickled result and the shared result document
/// spell them. `code` is the same pair as the single `0..=12` integer
/// `rspice_core::xspice::DigitalValue::event_code` produces, which is what a
/// rawfile event plot stores and what the browser worker contract carries — so
/// a caller moving between surfaces is moving between two encodings of one
/// value, not between two resolutions.
#[pyclass(name = "DigitalEvent", module = "rspice")]
#[derive(Debug, PartialEq)]
pub struct PyDigitalEvent {
    /// Accepted analog time, in seconds, at which this value was first seen.
    #[pyo3(get)]
    pub time_s: f64,
    /// Resolved logic level: `zero`, `one`, `unknown`, their `_resistive` and
    /// `_high_z` qualified forms, or `high_z`.
    #[pyo3(get)]
    pub state: String,
    /// Drive strength band: `strong`, `resistive`, `high_z`, `undetermined`.
    #[pyo3(get)]
    pub strength: String,
    /// The XSPICE event code in `0..=12` encoding the same state/strength
    /// pair.
    #[pyo3(get)]
    pub code: u8,
}

#[pymethods]
impl PyDigitalEvent {
    fn __repr__(&self) -> String {
        format!(
            "DigitalEvent(time_s={:.6e}, state='{}', strength='{}', code={})",
            self.time_s, self.state, self.strength, self.code
        )
    }
}

impl PyDigitalEvent {
    fn of(point: &DigitalTracePoint) -> Self {
        Self {
            time_s: point.time,
            state: digital_state_label(point.value.state).to_string(),
            strength: digital_strength_label(point.value.strength).to_string(),
            code: point.value.event_code(),
        }
    }
}

/// Node names with a committed digital event history, in capture order.
///
/// Capture order is the order the run first observed each node, which is the
/// order every other event route publishes them in — the rawfile event plots,
/// the document's `digitalTraces`, and the dump's `$var` declarations.
pub(super) fn digital_nodes(result: &TransientResult) -> Vec<String> {
    result
        .digital_traces
        .iter()
        .map(|trace| trace.node_name.clone())
        .collect()
}

/// Node names with a committed real-valued event history, in capture order.
///
/// A real event node carries an `f64` on its own event timeline rather than a
/// logic level, and it has no column on the analog grid at all, so this and
/// [`real_trace`] are the only typed route to one.
pub(super) fn real_nodes(result: &TransientResult) -> Vec<String> {
    result
        .real_traces
        .iter()
        .map(|trace| trace.node_name.clone())
        .collect()
}

/// One real event node's whole committed history, as `(time, value)` rows.
///
/// The name resolves ASCII-case-insensitively, and the rows are the shape
/// `CompressedTransientResult::real_trace` returns, so the two containers
/// answer one question one way.
pub(super) fn real_trace(result: &TransientResult, node: &str) -> PyResult<Vec<(f64, f64)>> {
    result
        .real_traces
        .iter()
        .find(|trace| trace.node_name.eq_ignore_ascii_case(node))
        .map(|trace| {
            trace
                .points
                .iter()
                .map(|point| (point.time, point.value))
                .collect()
        })
        .ok_or_else(|| crate::errors::key_error(format!("unknown XSPICE real event node '{node}'")))
}

/// A node the result recorded no event history for.
///
/// The message says how a node comes to be missing rather than only that it
/// is: event tracing is switched off by `.OPTIONS XSPICE ESAVE`, and a deck
/// that names its outputs selects an event node by bare name only.
fn unknown_digital_node_error(result: &TransientResult, node: &str) -> PyErr {
    crate::errors::key_error(format!(
        "no XSPICE digital event history was recorded for node '{node}'; this result carries {}. \
         A node is absent when it is not an event node, when '.OPTIONS XSPICE ESAVE' switched \
         event tracing off, or when the deck's output cards did not select it by bare name",
        if result.digital_traces.is_empty() {
            "none".to_string()
        } else {
            format!("{:?}", digital_nodes(result))
        }
    ))
}

/// One digital node's whole committed event history, as typed rows.
///
/// The name resolves the way every other event route resolves one, through
/// the core [`TransientResult::digital_trace_named`]: ASCII-case-insensitively,
/// because a deck node name is.
///
/// The history is borrowed across the GIL release rather than copied into it,
/// which is sound for the reason the Fourier accessors state: this class
/// exposes no `&mut self` entry point and no `__setstate__`, so nothing Python
/// can call while the worker runs can move the points out from under it.
pub(super) fn digital_events(
    py: Python<'_>,
    result: &TransientResult,
    node: &str,
) -> PyResult<Vec<PyDigitalEvent>> {
    let points = result
        .digital_trace_named(node)
        .ok_or_else(|| unknown_digital_node_error(result, node))?;
    if points.len() > MAX_DIGITAL_EVENT_ROWS {
        return Err(crate::errors::value_error(format!(
            "node '{node}' has {} committed events, past the {MAX_DIGITAL_EVENT_ROWS}-row limit \
             this accessor materialises; read the whole history through document() or write it \
             with to_vcd()",
            points.len()
        )));
    }
    crate::abort::run_interruptible_unregistered(py, |abort| {
        let mut rows: Vec<PyDigitalEvent> = Vec::new();
        rows.try_reserve_exact(points.len()).map_err(|_| {
            rspice_core::SimulationError::Circuit(format!(
                "could not allocate {} digital event rows for node '{node}'",
                points.len()
            ))
        })?;
        for (index, point) in points.iter().enumerate() {
            if index.is_multiple_of(ABORT_POLL_ROWS) && abort.is_aborted() {
                return Err(rspice_core::SimulationError::Aborted);
            }
            rows.push(PyDigitalEvent::of(point));
        }
        Ok(rows)
    })
}

/// Refusal for a result whose transient captured no event node at all.
///
/// The GUI's exporter refuses the same case for the same reason, and this is
/// the one place the three dumps of one run deliberately differ: `rspice run
/// -f vcd` warns and publishes a dump that declares no signal, because a
/// command that was asked for a file produces one. A library call that
/// returned a signal-less dump would be indistinguishable from a successful
/// export of a result that had events.
const NO_EVENT_HISTORY: &str = "this transient captured no XSPICE digital or real event node, so a Value Change Dump would \
     declare no signal and record no change; use to_csv() or to_raw() for the waveform table";

/// Project this result's event histories onto a dump, byte for byte as the
/// command line publishes it.
///
/// The declared buses of the result are passed through, so a declared bus is
/// written as one vector `$var` rather than as its member scalars. No engine
/// run declares one today — the boundary that would is still refused — so this
/// table is empty on every result this binding produces, and the dump is the
/// command line's byte for byte. `crates/rspice-cli/src/commands/vcd_io.rs`
/// passes an empty table unconditionally, so the two would part company for a
/// result whose buses were declared by something other than the engine.
///
/// The projection has no cancellation point of its own in core, so the work
/// runs on the interruptible worker for the GIL release rather than for early
/// cancellation: a `KeyboardInterrupt` raised during it is observed when the
/// projection returns.
pub(super) fn vcd_bytes(py: Python<'_>, result: &TransientResult) -> PyResult<Vec<u8>> {
    if result.digital_traces.is_empty() && result.real_traces.is_empty() {
        return Err(crate::errors::value_error(NO_EVENT_HISTORY));
    }
    crate::abort::run_interruptible_unregistered(py, |_abort| {
        Ok(event_document_bytes(
            &result.digital_traces,
            &result.real_traces,
            &result.digital_buses,
        ))
    })?
    .map_err(crate::errors::value_error)
}

/// The dump itself: one core projection, one core serialisation.
fn event_document_bytes(
    digital_traces: &[DigitalTrace],
    real_traces: &[rspice_core::engine::RealTrace],
    digital_buses: &[rspice_core::engine::DigitalBusDeclaration],
) -> Result<Vec<u8>, String> {
    let document = event_vcd_document(EVENT_SCOPE, digital_traces, real_traces, digital_buses)
        .map_err(|error| format!("this event history cannot be dumped exactly: {error}"))?;
    let mut bytes = Vec::new();
    rspice_core::io::write_vcd(&mut bytes, &document)
        .map_err(|error| format!("the Value Change Dump could not be written: {error}"))?;
    Ok(bytes)
}

/// The dump as text.
///
/// VCD is ASCII, and every byte written comes from a Rust `String`, so the
/// decode cannot fail; it is checked rather than asserted because this crate
/// denies the panic conveniences on every production path.
pub(super) fn vcd_text(py: Python<'_>, result: &TransientResult) -> PyResult<String> {
    String::from_utf8(vcd_bytes(py, result)?).map_err(|error| {
        crate::errors::value_error(format!("the Value Change Dump is not text: {error}"))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::engine::{RealTrace, RealTracePoint};
    use rspice_core::xspice::{DigitalState, DigitalStrength, DigitalValue};

    fn digital(node: &str, points: &[(f64, DigitalState, DigitalStrength)]) -> DigitalTrace {
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

    fn real(node: &str, points: &[(f64, f64)]) -> RealTrace {
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

    fn traces() -> (Vec<DigitalTrace>, Vec<RealTrace>) {
        (
            vec![
                digital(
                    "clk",
                    &[
                        (0.0, DigitalState::Zero, DigitalStrength::Strong),
                        (1e-8, DigitalState::One, DigitalStrength::Strong),
                        (2e-8, DigitalState::ZeroR, DigitalStrength::Resistive),
                        (3e-8, DigitalState::HighZ, DigitalStrength::HighZ),
                    ],
                ),
                digital(
                    "d",
                    &[(0.0, DigitalState::Unknown, DigitalStrength::Undetermined)],
                ),
            ],
            vec![real("ctrl", &[(0.0, 0.0), (2e-8, -1.5)])],
        )
    }

    /// Every row spells its value the way the pickled result spells it, and
    /// carries the event code that re-encodes to the same pair.
    #[test]
    fn a_row_carries_the_pickles_labels_and_the_codes_own_value() {
        let (digital_traces, _) = traces();
        let points = &digital_traces[0].points;
        let rows: Vec<PyDigitalEvent> = points.iter().map(PyDigitalEvent::of).collect();

        assert_eq!(
            rows.iter()
                .map(|row| (row.state.as_str(), row.strength.as_str()))
                .collect::<Vec<_>>(),
            [
                ("zero", "strong"),
                ("one", "strong"),
                ("zero_resistive", "resistive"),
                ("high_z", "high_z"),
            ]
        );
        assert_eq!(
            rows.iter().map(|row| row.code).collect::<Vec<_>>(),
            [0, 1, 3, 12]
        );
        for (row, point) in rows.iter().zip(points) {
            assert_eq!(row.time_s, point.time);
            assert_eq!(
                DigitalValue::from_event_code(row.code),
                Some(DigitalValue::new(point.value.state, point.value.strength)),
                "the row's code does not decode to the value it was built from"
            );
        }
    }

    /// The dump this binding produces is the one `rspice run -f vcd`
    /// publishes. The expected bytes are built the way
    /// `crates/rspice-cli/src/commands/vcd_io.rs`'s `event_document` builds
    /// them — the same core projection under the same scope, serialised by the
    /// same core writer — so anything but equality means the two exports of one
    /// run disagree.
    #[test]
    fn a_dump_is_byte_identical_to_the_command_line_projection() {
        let (digital_traces, real_traces) = traces();
        let ours = event_document_bytes(&digital_traces, &real_traces, &[])
            .expect("the fixture history dumps");

        let document = event_vcd_document("events", &digital_traces, &real_traces, &[])
            .expect("the command line's projection accepts the same history");
        let mut expected = Vec::new();
        rspice_core::io::write_vcd(&mut expected, &document).expect("the dump serialises");

        assert_eq!(ours, expected);

        let text = String::from_utf8(ours).expect("a dump is ASCII text");
        assert!(text.contains("$scope module events $end"), "{text}");
        assert!(text.contains("$var wire 1 ! clk $end"), "{text}");
        assert!(text.contains("$var wire 1 \" d $end"), "{text}");
        assert!(text.contains("$var real 64 # ctrl $end"), "{text}");
    }

    /// A declared bus is written as one vector rather than as its members,
    /// which is what makes the dump readable as the word the run held.
    #[test]
    fn a_declared_bus_is_written_as_one_vector() {
        let digital_traces = vec![
            digital(
                "q#1",
                &[
                    (0.0, DigitalState::Zero, DigitalStrength::Strong),
                    (1e-8, DigitalState::One, DigitalStrength::Strong),
                ],
            ),
            digital("q#0", &[(0.0, DigitalState::One, DigitalStrength::Strong)]),
        ];
        let bus = rspice_core::engine::DigitalBusDeclaration::new(
            "q",
            1,
            0,
            vec!["q#1".to_string(), "q#0".to_string()],
            rspice_core::engine::DigitalBusSource::Schematic,
        )
        .expect("the fixture declaration is well formed");

        let text = String::from_utf8(
            event_document_bytes(&digital_traces, &[], std::slice::from_ref(&bus))
                .expect("the fixture history dumps"),
        )
        .expect("a dump is ASCII text");

        assert!(text.contains("$var wire 2 ! q [1:0] $end"), "{text}");
        assert!(!text.contains("q#1"), "the members are not written: {text}");
        assert!(text.contains("b01 !"), "{text}");
        assert!(text.contains("b11 !"), "{text}");
    }

    /// An event time no timescale carries is refused rather than quantised,
    /// and the refusal reaches this surface as the core's own words.
    #[test]
    fn an_inexact_event_time_is_refused_rather_than_moved() {
        let error = event_document_bytes(
            &[digital(
                "d",
                &[(1.5e-16, DigitalState::One, DigitalStrength::Strong)],
            )],
            &[],
            &[],
        )
        .expect_err("half a femtosecond has no tick at any scale");
        assert!(error.contains("cannot be dumped exactly"), "{error}");
        assert!(error.contains("femtoseconds"), "{error}");
    }
}

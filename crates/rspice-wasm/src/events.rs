//! The typed digital event view, and the Value Change Dump, of one result.
//!
//! [`crate::document`] projects the descriptors of a result; the event
//! histories are not descriptors. They are the sparse list of values the event
//! solver committed, at the times it committed them, and they crossed the
//! boundary only inside the handle's whole-document JSON export until this
//! module existed — so a page that wanted one node's edges had to decode a
//! document that restates every waveform in the run.
//!
//! Nothing here decides what a value means. A row carries the document's own
//! `DigitalStateTag`/`DigitalStrengthTag` spellings, and the `0..=12` code
//! beside them is `rspice_core::xspice::DigitalValue::event_code` — the
//! encoding the accepted-sample hook, the rawfile event plots and the GUI's
//! worker contract already agree on.
//!
//! # The dump
//!
//! [`vcd_text`] is the projection `rspice run -f vcd` publishes, not a second
//! one: the core `event_vcd_document` under one `$scope module events`,
//! serialised by the core `write_vcd`, exactly as
//! `crates/rspice-cli/src/commands/vcd_io.rs` and the GUI's exporter build it.
//! `a_dump_is_byte_identical_to_the_command_line_projection` pins the bytes.

use rspice_core::engine::{
    DigitalBusDeclaration, DigitalTrace, DigitalTracePoint, RealTrace, RealTracePoint,
};
use rspice_core::execution::event_vcd_document;
use rspice_core::execution::result_document::{
    DigitalEventTrace, RealEventTrace, TransientPayload,
};
use rspice_core::execution::{AnalysisResultDocument, ResultPayload};
use rspice_core::xspice::DigitalValue;
use serde::Serialize;

use crate::DetailedWasmResult;
use crate::errors::WasmError;

/// The `$scope module` every RSpice dump declares its nodes under.
///
/// `rspice-cli`'s `commands/vcd_io.rs` and the GUI's exporter use this exact
/// string. Three dumps of one run differ the moment they disagree.
const EVENT_SCOPE: &str = "events";

/// Transfer values one event row is charged against the handle's ceiling.
///
/// A row crosses as a time and a code plus two short tags. Charging two keeps
/// it on the same budget as a window point, which is what makes
/// `maxResultValues` mean one thing across every transfer this handle does.
const VALUES_PER_EVENT_ROW: usize = 2;

/// One committed digital event of one node.
#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DigitalEventRow {
    /// Accepted analog time, in seconds, at which this value was first seen.
    pub time: f64,
    /// Resolved logic level, in the document's own spelling.
    pub state: rspice_core::execution::result_document::DigitalStateTag,
    /// Drive strength band, in the document's own spelling.
    pub strength: rspice_core::execution::result_document::DigitalStrengthTag,
    /// The `0..=12` XSPICE event code encoding the same state/strength pair.
    pub code: u8,
}

impl DigitalEventRow {
    fn project(point: &rspice_core::execution::result_document::DigitalEventPoint) -> Self {
        Self {
            time: point.time,
            state: point.state,
            strength: point.strength,
            code: DigitalValue::new(point.state.into(), point.strength.into()).event_code(),
        }
    }
}

/// The event inventory of one result, without any of its points.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DigitalNodeDescriptor {
    /// Netlist node name, as the run resolved it.
    pub node_name: String,
    /// Committed changes this node recorded.
    pub event_count: usize,
    /// The bus this node is a member of, and its position in it, when a bus
    /// declares it. `null` for a node that belongs to no declared bus.
    pub bus: Option<BusMembership>,
}

/// Where one member sits in the word it belongs to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BusMembership {
    /// Bus name, without any range suffix.
    pub name: String,
    /// Position in the declaration, counted from the declared MSB.
    pub position: usize,
}

/// The transient payload of one result, or a refusal naming what it is.
///
/// An envelope carries a continued transient, and its event histories are that
/// transient's, so it answers here exactly as a `tran` result does — which is
/// what the payload descriptor already does for the compression certificate
/// and the `.FFT` children.
fn transient_payload(document: &AnalysisResultDocument) -> DetailedWasmResult<&TransientPayload> {
    match document.payload() {
        ResultPayload::Tran(transient) => Ok(transient),
        ResultPayload::Envelope(envelope) => Ok(&envelope.transient),
        other => Err(Box::new(WasmError::new(
            format!(
                "a {} result carries no event history; only a transient captures XSPICE digital \
                 and real event nodes",
                other.result_kind().tag()
            ),
            "unsupported_result_family",
            "unsupported_feature",
        ))),
    }
}

/// Every digital event node of one result, with its change count and its bus.
pub(crate) fn digital_nodes(
    document: &AnalysisResultDocument,
) -> DetailedWasmResult<Vec<DigitalNodeDescriptor>> {
    let payload = transient_payload(document)?;
    let mut nodes = Vec::new();
    nodes
        .try_reserve_exact(payload.digital_traces.len())
        .map_err(|_| allocation_error("digital event node descriptors"))?;
    for trace in &payload.digital_traces {
        nodes.push(DigitalNodeDescriptor {
            node_name: trace.node_name.clone(),
            event_count: trace.points.len(),
            bus: bus_membership(payload, &trace.node_name),
        });
    }
    Ok(nodes)
}

/// The bus that claims one member, if the document declares one.
///
/// A document is refused on load unless every member names a trace and no two
/// buses claim one conductor, so the first match is the only match.
fn bus_membership(payload: &TransientPayload, node_name: &str) -> Option<BusMembership> {
    payload.digital_buses.iter().find_map(|bus| {
        bus.members
            .iter()
            .position(|member| member.eq_ignore_ascii_case(node_name))
            .map(|position| BusMembership {
                name: bus.name.clone(),
                position,
            })
    })
}

/// One digital node's whole committed event history, as typed rows.
///
/// The name resolves ASCII-case-insensitively, as a deck node name does and as
/// every other event route resolves one.
///
/// The history is charged against the handle's own transfer ceiling, so a
/// tighter `maxResultValues` tightens this the way it tightens a window: the
/// browser never learns of a budget that only one call obeys.
pub(crate) fn digital_events(
    document: &AnalysisResultDocument,
    node: &str,
    maximum_transfer_values: usize,
) -> DetailedWasmResult<Vec<DigitalEventRow>> {
    let payload = transient_payload(document)?;
    let trace = payload
        .digital_traces
        .iter()
        .find(|trace| trace.node_name.eq_ignore_ascii_case(node))
        .ok_or_else(|| unknown_node_error(payload, node))?;

    let requested = trace.points.len().saturating_mul(VALUES_PER_EVENT_ROW);
    if requested > maximum_transfer_values {
        return Err(Box::new(WasmError::new(
            format!(
                "the event history of '{}' requires {requested} transfer values but the limit is \
                 {maximum_transfer_values}",
                trace.node_name
            ),
            "invalid_result_window",
            "result_transfer",
        )));
    }

    let mut rows = Vec::new();
    rows.try_reserve_exact(trace.points.len())
        .map_err(|_| allocation_error("digital event rows"))?;
    for point in &trace.points {
        rows.push(DigitalEventRow::project(point));
    }
    Ok(rows)
}

/// A node this result recorded no digital event history for.
fn unknown_node_error(payload: &TransientPayload, node: &str) -> Box<WasmError> {
    Box::new(WasmError::new(
        format!(
            "no XSPICE digital event history was recorded for node '{node}'; this result carries \
             {} digital event node(s)",
            payload.digital_traces.len()
        ),
        "unknown_event_node",
        "input_validation",
    ))
}

/// Project this result's event histories onto a dump, byte for byte as the
/// command line publishes it.
///
/// The document's declared buses are passed through, so a declared bus is
/// written as one vector `$var` rather than as its member scalars.
/// `crates/rspice-cli/src/commands/vcd_io.rs` passes an empty table
/// unconditionally, so the two would part company for a document whose buses
/// were declared by a frontend or read out of an imported artifact; no run
/// declares one, so every result this crate produces dumps identically.
pub(crate) fn vcd_text(document: &AnalysisResultDocument) -> DetailedWasmResult<String> {
    let payload = transient_payload(document)?;
    if payload.digital_traces.is_empty() && payload.real_traces.is_empty() {
        return Err(Box::new(WasmError::new(
            "this transient captured no XSPICE digital or real event node, so a Value Change Dump \
             would declare no signal and record no change"
                .to_owned(),
            "no_event_history",
            "unsupported_feature",
        )));
    }

    let digital = core_digital_traces(&payload.digital_traces)?;
    let real = core_real_traces(&payload.real_traces)?;
    let mut buses = Vec::new();
    buses
        .try_reserve_exact(payload.digital_buses.len())
        .map_err(|_| allocation_error("digital bus declarations"))?;
    for bus in &payload.digital_buses {
        buses.push(DigitalBusDeclaration::from(bus));
    }

    let projected = event_vcd_document(EVENT_SCOPE, &digital, &real, &buses)
        .map_err(|error| projection_error(format!("{error}")))?;
    let mut bytes = Vec::new();
    rspice_core::io::write_vcd(&mut bytes, &projected)
        .map_err(|error| projection_error(format!("{error}")))?;
    String::from_utf8(bytes).map_err(|error| projection_error(format!("{error}")))
}

/// The engine's own trace type, rebuilt from the document's.
///
/// The two are the same history in two spellings — the document carries the
/// state and the strength as tags, the engine as the value they name — and
/// the tag conversions are core's own bijections, so nothing is decided here.
fn core_digital_traces(traces: &[DigitalEventTrace]) -> DetailedWasmResult<Vec<DigitalTrace>> {
    let mut built = Vec::new();
    built
        .try_reserve_exact(traces.len())
        .map_err(|_| allocation_error("digital event traces"))?;
    for trace in traces {
        let mut points = Vec::new();
        points
            .try_reserve_exact(trace.points.len())
            .map_err(|_| allocation_error("digital event points"))?;
        for point in &trace.points {
            points.push(DigitalTracePoint {
                time: point.time,
                value: DigitalValue::new(point.state.into(), point.strength.into()),
            });
        }
        built.push(DigitalTrace {
            node_name: trace.node_name.clone(),
            points,
        });
    }
    Ok(built)
}

fn core_real_traces(traces: &[RealEventTrace]) -> DetailedWasmResult<Vec<RealTrace>> {
    let mut built = Vec::new();
    built
        .try_reserve_exact(traces.len())
        .map_err(|_| allocation_error("real event traces"))?;
    for trace in traces {
        let mut points = Vec::new();
        points
            .try_reserve_exact(trace.points.len())
            .map_err(|_| allocation_error("real event points"))?;
        for point in &trace.points {
            points.push(RealTracePoint {
                time: point.time,
                value: point.value,
            });
        }
        built.push(RealTrace {
            node_name: trace.node_name.clone(),
            points,
        });
    }
    Ok(built)
}

fn projection_error(message: String) -> Box<WasmError> {
    Box::new(WasmError::new(
        format!("this event history cannot be dumped exactly: {message}"),
        "invalid_event_projection",
        "result_projection",
    ))
}

fn allocation_error(object: &'static str) -> Box<WasmError> {
    Box::new(WasmError::new(
        format!("could not allocate {object}"),
        "result_allocation_failed",
        "result_projection",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runners::deck::run_authored_deck_document_detailed;

    /// A digital bridge and a real event node, so both kinds of event timeline
    /// are present. This is the deck `rspice-cli`'s conversion tests use.
    const EVENT_DECK: &str = "browser event deck
v1 in 0 pulse(0 5 0 1n 1n 5n 10n)
abridge1 [in] [d] adc
adac [d] [out] dac
aobs out rnode obs
rout out 0 1k
.model adc adc_bridge(in_low=1 in_high=4)
.model dac dac_bridge(out_low=0 out_high=5 out_undef=2.5)
.model obs v_to_real(gain=2)
.tran 1n 20n
.end
";

    const ANALOG_DECK: &str = "browser analog deck
V1 in 0 PULSE(0 1 0 1n 1n 20n 40n)
R1 in out 1k
C1 out 0 1p
.TRAN 1n 20n
.END
";

    fn document(source: &str) -> AnalysisResultDocument {
        let mut execution = run_authored_deck_document_detailed(source)
            .unwrap_or_else(|error| panic!("deck must run: {}", error.message));
        execution
            .results
            .drain(..)
            .next()
            .expect("the deck publishes one result")
    }

    /// The inventory names every captured event node and counts its changes,
    /// and carries no point of its own.
    #[test]
    fn the_inventory_names_every_event_node_and_counts_its_changes() {
        let document = document(EVENT_DECK);
        let nodes = digital_nodes(&document).expect("a transient answers");
        assert!(!nodes.is_empty(), "the bridge captures a digital node");
        for node in &nodes {
            assert!(node.event_count > 0, "{}", node.node_name);
            assert!(node.bus.is_none(), "no run declares a bus today");
            let rows = digital_events(&document, &node.node_name, usize::MAX)
                .expect("a named node answers");
            assert_eq!(rows.len(), node.event_count);
        }
    }

    /// A row carries the document's own spellings and the code that encodes
    /// the same pair, so the two encodings cannot drift apart.
    #[test]
    fn a_row_carries_the_documents_spelling_and_the_codes_own_value() {
        let document = document(EVENT_DECK);
        let nodes = digital_nodes(&document).expect("a transient answers");
        let first = nodes.first().expect("one digital node");
        let rows =
            digital_events(&document, &first.node_name, usize::MAX).expect("the node answers");

        let mut previous = f64::NEG_INFINITY;
        for row in &rows {
            assert!(row.time >= previous, "event times are non-decreasing");
            previous = row.time;
            let value = DigitalValue::new(row.state.into(), row.strength.into());
            assert_eq!(value.event_code(), row.code);
            assert_eq!(
                DigitalValue::from_event_code(row.code),
                Some(DigitalValue::new(value.state, value.strength)),
                "the row's code does not decode to the value it was built from"
            );
        }
    }

    /// The name resolves the way a deck node name does, and an unknown one
    /// fails closed with the documented code.
    #[test]
    fn a_node_name_resolves_case_insensitively_and_an_unknown_one_fails_closed() {
        let document = document(EVENT_DECK);
        let nodes = digital_nodes(&document).expect("a transient answers");
        let first = nodes.first().expect("one digital node");
        let shouted = first.node_name.to_ascii_uppercase();
        assert_eq!(
            digital_events(&document, &shouted, usize::MAX).expect("case does not matter"),
            digital_events(&document, &first.node_name, usize::MAX).expect("the node answers"),
        );

        let error = *digital_events(&document, "not_a_node", usize::MAX)
            .expect_err("an unknown node must fail closed");
        assert_eq!(error.code, "unknown_event_node");
        assert_eq!(error.category, "input_validation");
    }

    /// The transfer ceiling bounds this the way it bounds a window: a history
    /// that does not fit is refused by number rather than truncated.
    #[test]
    fn an_over_budget_history_is_refused_rather_than_truncated() {
        let document = document(EVENT_DECK);
        let nodes = digital_nodes(&document).expect("a transient answers");
        let first = nodes.first().expect("one digital node");
        let error = *digital_events(&document, &first.node_name, 1)
            .expect_err("a one-value budget cannot carry a history");
        assert_eq!(error.code, "invalid_result_window");
        assert_eq!(error.category, "result_transfer");
        assert!(
            error.message.contains("the limit is 1"),
            "{}",
            error.message
        );
    }

    /// The dump this binding produces is the one `rspice run -f vcd`
    /// publishes. The expected bytes are built the way
    /// `crates/rspice-cli/src/commands/vcd_io.rs`'s `event_document` builds
    /// them, so anything but equality means two exports of one run disagree.
    #[test]
    fn a_dump_is_byte_identical_to_the_command_line_projection() {
        let document = document(EVENT_DECK);
        let ours = vcd_text(&document).expect("the run's event history dumps");

        let payload = match document.payload() {
            ResultPayload::Tran(transient) => transient,
            other => panic!("a .tran publishes a tran payload, not {other:?}"),
        };
        let digital = core_digital_traces(&payload.digital_traces).expect("traces convert");
        let real = core_real_traces(&payload.real_traces).expect("traces convert");
        let projected =
            event_vcd_document("events", &digital, &real, &[]).expect("the projection accepts");
        let mut expected = Vec::new();
        rspice_core::io::write_vcd(&mut expected, &projected).expect("the dump serialises");

        assert_eq!(ours.as_bytes(), expected.as_slice());
        assert!(ours.contains("$scope module events $end"), "{ours}");
        assert!(ours.contains("$timescale"), "{ours}");
    }

    /// A result with no event history is refused rather than handed a dump
    /// that declares no signal.
    #[test]
    fn a_result_with_no_event_history_is_refused_by_what_a_dump_carries() {
        let document = document(ANALOG_DECK);
        let error = *vcd_text(&document).expect_err("an analog run has nothing to dump");
        assert_eq!(error.code, "no_event_history");
        assert!(
            error.message.contains("declare no signal"),
            "{}",
            error.message
        );

        assert!(
            digital_nodes(&document)
                .expect("a transient answers")
                .is_empty()
        );
    }

    /// A family that captures no event timeline says so by name rather than
    /// answering with an empty list.
    #[test]
    fn a_family_with_no_event_timeline_refuses_by_name() {
        let document = document("browser operating point\nV1 in 0 1\nR1 in 0 1k\n.OP\n.END\n");
        let error = *digital_nodes(&document).expect_err("an .op has no event history");
        assert_eq!(error.code, "unsupported_result_family");
        assert!(error.message.contains("op result"), "{}", error.message);
    }
}

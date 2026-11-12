//! Exact retained digital/real events, bus declarations, and history validation.

pub mod projection;

use crate::analysis_payload::AnalysisResultPayload;
use crate::current_impulses::CurrentImpulseHistoryEvidence;
use crate::simulation_values::{DigitalEventPoint, EventNodeHistory, RealEventPoint};
use crate::validation::require_non_empty;
use crate::voltage_impulses::VoltageImpulseHistoryEvidence;
use rspice_core::engine::{DigitalTrace, DigitalTracePoint, RealTrace, RealTracePoint};
use rspice_core::xspice::DigitalValue;

/// One committed digital event on an XSPICE event node.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DigitalEventPointEvidence {
    pub time_s: f64,
    /// XSPICE 12-state event code, 0..=12. The producer is
    /// `rspice_core::xspice::DigitalValue::event_code`.
    pub value_code: u8,
}

/// The committed event history of one XSPICE digital node.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DigitalEventTraceEvidence {
    pub node_name: String,
    pub points: Vec<DigitalEventPointEvidence>,
}

/// One committed real-valued event on an XSPICE event node.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RealEventPointEvidence {
    pub time_s: f64,
    pub value: f64,
}

/// The committed event history of one XSPICE real-valued node.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RealEventTraceEvidence {
    pub node_name: String,
    pub points: Vec<RealEventPointEvidence>,
}

/// Who declared a digital bus that a retained result carries.
///
/// The same three claims `rspice_core::engine::DigitalBusSource` makes, kept
/// as this crate's own enum so a retained project file's encoding is owned
/// here and does not move when the engine's derives do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DigitalBusSourceEvidence {
    /// A model declared it: a vector port on the mixed-signal boundary.
    Engine,
    /// The schematic declared it: a drawn bus over its member nets.
    Schematic,
    /// A file claimed it: a VCD vector variable, or a rawfile bus plot.
    Import,
}

impl From<rspice_core::engine::DigitalBusSource> for DigitalBusSourceEvidence {
    fn from(value: rspice_core::engine::DigitalBusSource) -> Self {
        match value {
            rspice_core::engine::DigitalBusSource::Engine => Self::Engine,
            rspice_core::engine::DigitalBusSource::Schematic => Self::Schematic,
            rspice_core::engine::DigitalBusSource::Import => Self::Import,
        }
    }
}

impl From<DigitalBusSourceEvidence> for rspice_core::engine::DigitalBusSource {
    fn from(value: DigitalBusSourceEvidence) -> Self {
        match value {
            DigitalBusSourceEvidence::Engine => Self::Engine,
            DigitalBusSourceEvidence::Schematic => Self::Schematic,
            DigitalBusSourceEvidence::Import => Self::Import,
        }
    }
}

/// One digital bus declared over retained event traces.
///
/// A bus is a *declaration*, never a fourth kind of history: the members keep
/// their own [`DigitalEventTraceEvidence`], and every word this crate shows is
/// reassembled from them by `rspice_core::execution::bus_events`. Nothing here
/// stores a value, so a bus and its members can never disagree.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DigitalBusEvidence {
    /// Bus name, without any range suffix.
    pub name: String,
    /// Declared most significant index, exactly as declared.
    pub msb: i64,
    /// Declared least significant index, exactly as declared.
    pub lsb: i64,
    /// Member node names in declaration order, declared MSB first. Each names
    /// a retained digital event trace.
    pub members: Vec<String>,
    /// Who declared this bus.
    pub source: DigitalBusSourceEvidence,
}

impl From<&rspice_core::engine::DigitalBusDeclaration> for DigitalBusEvidence {
    fn from(value: &rspice_core::engine::DigitalBusDeclaration) -> Self {
        Self {
            name: value.name.clone(),
            msb: value.msb,
            lsb: value.lsb,
            members: value.members.clone(),
            source: value.source.into(),
        }
    }
}

impl From<&DigitalBusEvidence> for rspice_core::engine::DigitalBusDeclaration {
    fn from(value: &DigitalBusEvidence) -> Self {
        Self {
            name: value.name.clone(),
            msb: value.msb,
            lsb: value.lsb,
            members: value.members.clone(),
            source: value.source.into(),
        }
    }
}

/// Build the core event traces this crate's retained evidence stands for.
///
/// Refuses an unrecognised event code by name. The retained-evidence check
/// already bounds `value_code` at 12, so reaching this is a corrupted
/// dataset — which is exactly the case that must not be written out as if it
/// were a level.
pub fn core_event_traces(
    digital_traces: &[DigitalEventTraceEvidence],
    real_traces: &[RealEventTraceEvidence],
) -> Result<(Vec<DigitalTrace>, Vec<RealTrace>), String> {
    let mut digital = Vec::with_capacity(digital_traces.len());
    for trace in digital_traces {
        let mut points = Vec::with_capacity(trace.points.len());
        for point in &trace.points {
            let value = DigitalValue::from_event_code(point.value_code).ok_or_else(|| {
                format!(
                    "node '{}' records event code {} at {} s, which is not one of the thirteen \
                     XSPICE event codes",
                    trace.node_name, point.value_code, point.time_s
                )
            })?;
            points.push(DigitalTracePoint {
                time: point.time_s,
                value,
            });
        }
        digital.push(DigitalTrace {
            node_name: trace.node_name.clone(),
            points,
            hdl_time_precision: None,
        });
    }
    let real = real_traces
        .iter()
        .map(|trace| RealTrace {
            node_name: trace.node_name.clone(),
            points: trace
                .points
                .iter()
                .map(|point| RealTracePoint {
                    time: point.time_s,
                    value: point.value,
                })
                .collect(),
        })
        .collect();
    Ok((digital, real))
}

/// The retained event evidence for histories a digital importer decoded.
///
/// Both importers land here, so a VCD bus and an FST bus are one shape: the
/// same declaration type over the same member traces, carrying the same
/// four-state codes. `None` when the source recorded no event at all, which
/// is what an analog-only file is.
pub fn imported_event_payload(
    histories: &rspice_core::execution::VcdEventHistories,
) -> Option<AnalysisResultPayload> {
    if histories.digital_traces.is_empty() && histories.real_traces.is_empty() {
        return None;
    }
    let digital_traces = histories
        .digital_traces
        .iter()
        .map(|trace| DigitalEventTraceEvidence {
            node_name: trace.node_name.clone(),
            points: trace
                .points
                .iter()
                .map(|point| DigitalEventPointEvidence {
                    time_s: point.time,
                    value_code: point.value.event_code(),
                })
                .collect(),
        })
        .collect();
    let real_traces = histories
        .real_traces
        .iter()
        .map(|trace| RealEventTraceEvidence {
            node_name: trace.node_name.clone(),
            points: trace
                .points
                .iter()
                .map(|point| RealEventPointEvidence {
                    time_s: point.time,
                    value: point.value,
                })
                .collect(),
        })
        .collect();
    let digital_buses = histories
        .digital_buses
        .iter()
        .map(DigitalBusEvidence::from)
        .collect();
    Some(AnalysisResultPayload::TransientEvents {
        current_impulses: None,
        voltage_impulses: None,
        digital_traces,
        real_traces,
        digital_buses,
    })
}

/// Committed digital and real events, and exact signed current impulses.
/// Empty current traces can retain an explicit coverage claim even when no
/// charge events occurred; unavailable legacy histories carry `None`.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransientEventHistory {
    /// Exact charge observations; absent means unknown, not zero charge.
    pub current_impulses: Option<CurrentImpulseHistoryEvidence>,
    /// Integrated voltage and its derivatives, with separate coverage.
    pub voltage_impulses: Option<VoltageImpulseHistoryEvidence>,
    pub digital: Vec<EventNodeHistory<DigitalEventPoint>>,
    pub real: Vec<EventNodeHistory<RealEventPoint>>,
    /// Buses the run declared over `digital`, in declaration order.
    ///
    /// A declaration only, carried beside the member histories it names — the
    /// engine states which conductors are one word, never what the word is.
    pub digital_buses: Vec<DigitalBusEvidence>,
    /// Where each traced HDL net or variable was declared and which
    /// processes drive it, by the trace or bus it was published under. Empty
    /// without an HDL instance. Like a bus, a claim about traces beside it:
    /// it makes no history non-empty on its own.
    pub hdl_sources: Vec<HdlSourceEvidence>,
}

/// Where one traced HDL net or variable was declared and which processes
/// drive it: the engine's `HdlTraceSource`, kept as this crate's own type so
/// a retained or transported encoding is owned here.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HdlSourceEvidence {
    /// The trace or bus the run published the signal under.
    pub trace: String,
    /// The deck instance whose design declares it (`xtb`).
    pub instance: String,
    /// Its name in that design (`ok`; `u1.q` inside a child instance).
    pub identifier: String,
    /// The file and line that declared it, when the compile placed it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub declaration: Option<HdlSourceLineEvidence>,
    /// Every process that writes the signal, in the design's process order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub drivers: Vec<HdlDriverEvidence>,
}

/// A file and a one-based line in it.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HdlSourceLineEvidence {
    pub file: String,
    pub line: u32,
}

/// One process that drives a traced HDL signal.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HdlDriverEvidence {
    /// The scope it is written in (`xtb`, or `xtb.u1` inside a child).
    pub scope: String,
    pub kind: HdlDriverKindEvidence,
    /// The line it was written on, when the compile placed it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<HdlSourceLineEvidence>,
}

/// What kind of process drives a signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HdlDriverKindEvidence {
    /// An `initial` block (IEEE 1364-2005 section 9.9.1).
    Initial,
    /// An `always` block (section 9.9.2).
    Always,
    /// A continuous assignment, a gate or a port connection (section 6.1).
    Assign,
}

impl From<&rspice_core::engine::HdlSourceLine> for HdlSourceLineEvidence {
    fn from(value: &rspice_core::engine::HdlSourceLine) -> Self {
        Self {
            file: value.file.clone(),
            line: value.line,
        }
    }
}

impl From<&rspice_core::engine::HdlTraceSource> for HdlSourceEvidence {
    fn from(value: &rspice_core::engine::HdlTraceSource) -> Self {
        use rspice_core::engine::HdlDriverKind;
        Self {
            trace: value.trace.clone(),
            instance: value.instance.clone(),
            identifier: value.identifier.clone(),
            declaration: value.declaration.as_ref().map(Into::into),
            drivers: value
                .drivers
                .iter()
                .map(|driver| HdlDriverEvidence {
                    scope: driver.scope.clone(),
                    kind: match driver.kind {
                        HdlDriverKind::Initial => HdlDriverKindEvidence::Initial,
                        HdlDriverKind::Always => HdlDriverKindEvidence::Always,
                        HdlDriverKind::Assign => HdlDriverKindEvidence::Assign,
                    },
                    source: driver.source.as_ref().map(Into::into),
                })
                .collect(),
        }
    }
}

impl TransientEventHistory {
    /// Whether this run committed no event history at all.
    ///
    /// A bus is a claim *about* member traces, so a table with no traces
    /// under it says nothing and does not make a history non-empty; the
    /// engine cannot produce one, and `validate_digital_bus_table` refuses it
    /// if anything ever does.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.digital.is_empty()
            && self.real.is_empty()
            && self.current_impulses.is_none()
            && self.voltage_impulses.is_none()
    }
}

/// Validate retained event histories and their declarations without copying samples.
pub fn validate_event_history(
    digital_traces: &[DigitalEventTraceEvidence],
    real_traces: &[RealEventTraceEvidence],
    digital_buses: &[DigitalBusEvidence],
    current_impulses: Option<&CurrentImpulseHistoryEvidence>,
    voltage_impulses: Option<&VoltageImpulseHistoryEvidence>,
) -> Result<(), String> {
    if digital_traces.is_empty()
        && real_traces.is_empty()
        && current_impulses.is_none()
        && voltage_impulses.is_none()
    {
        return Err("event payload contains no retained event history".to_owned());
    }
    if let Some(history) = current_impulses {
        history.validate()?;
    }
    if let Some(history) = voltage_impulses {
        history.validate()?;
    }
    let mut seen = std::collections::BTreeSet::new();
    for trace in digital_traces {
        require_non_empty(&trace.node_name, "event node identity")?;
        if !seen.insert(trace.node_name.as_str()) {
            return Err(format!(
                "event node '{}' is retained more than once",
                trace.node_name
            ));
        }
        let times = trace.points.iter().map(|point| point.time_s);
        validate_event_times(&trace.node_name, times)?;
        // The typed decoder owns the encoding: a code it refuses
        // is not a state this build can name, and asking it here
        // leaves one spelling of that bound rather than a
        // constant beside it that has to be kept in step.
        if trace.points.iter().any(|point| {
            rspice_core::xspice::DigitalValue::from_event_code(point.value_code).is_none()
        }) {
            return Err(format!(
                "event node '{}' has a value outside the XSPICE 12-state encoding",
                trace.node_name
            ));
        }
    }
    for trace in real_traces {
        require_non_empty(&trace.node_name, "event node identity")?;
        if !seen.insert(trace.node_name.as_str()) {
            return Err(format!(
                "event node '{}' is retained more than once",
                trace.node_name
            ));
        }
        let times = trace.points.iter().map(|point| point.time_s);
        validate_event_times(&trace.node_name, times)?;
        if trace.points.iter().any(|point| !point.value.is_finite()) {
            return Err(format!(
                "event node '{}' has a non-finite value",
                trace.node_name
            ));
        }
    }
    // Width, membership, uniqueness and the 4,096-member ceiling
    // are the engine's rules about a bus, not this crate's, and
    // they are asked of the engine's own validator so a table the
    // GUI accepts is exactly one the CLI and the bindings accept.
    let declarations = digital_buses
        .iter()
        .map(rspice_core::engine::DigitalBusDeclaration::from)
        .collect::<Vec<_>>();
    rspice_core::engine::validate_digital_bus_table(
        &declarations,
        digital_traces.iter().map(|trace| trace.node_name.as_str()),
    )
    .map_err(|error| error.to_string())?;
    Ok(())
}

/// An event history is a schedule: nonnegative, finite, and non-decreasing.
///
/// Non-decreasing, not strictly increasing. An event-driven solver settles a
/// node through several delta cycles at one physical time, and every one of
/// those transitions is a committed event with its own value. Their order is
/// the order they were committed in, which is the order they are stored in —
/// so a repeated timestamp is evidence, not corruption.
fn validate_event_times(node_name: &str, times: impl Iterator<Item = f64>) -> Result<(), String> {
    let mut previous: Option<f64> = None;
    let mut count = 0usize;
    for time in times {
        count += 1;
        if !time.is_finite() || time < 0.0 {
            return Err(format!(
                "event node '{node_name}' has an invalid event time"
            ));
        }
        if previous.is_some_and(|previous| previous > time) {
            return Err(format!(
                "event node '{node_name}' events must not move backwards in time"
            ));
        }
        previous = Some(time);
    }
    if count == 0 {
        return Err(format!("event node '{node_name}' retained no events"));
    }
    Ok(())
}

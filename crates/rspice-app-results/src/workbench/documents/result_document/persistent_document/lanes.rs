//! A project result document's event lanes: projected into the strip's
//! session layout before the pane is drawn, and kept after it.
//!
//! The document holds a lane as its trace's kind (logic or real), a bus's
//! radix and expansion and the reader's groupings as user buses, and the row
//! order as the pane's event-lane order. A bus the run declared is no column
//! of the source, so the document names it the only way it can: as a user
//! bus over its member traces under the declared `name[msb:lsb]`, written only
//! once the reader reads it in another radix or opens it.
//!
//! A native mixed run keeps its nets only as event history, so the source has
//! no column and the document no trace for them. The document holds such a
//! net by its `D()` or `E()` name instead: listed in the pane's order while
//! it is plotted, and named as a member of a bus over such nets.
use std::collections::{HashMap, HashSet};

use crate::results::visualization_document::{
    DocumentEdit, EventLaneOrder, EventLaneRef, PaneId, Trace, TraceId, TraceLane, TraceLaneKind,
    UserBus, VisualizationDocument,
};
use crate::state::{AnalysisResult, AnalysisResultPayload};
use rspice_results_ui::waves::lanes::{BusView, EventLaneLayout, LaneKey, UserLaneBus};

/// What a run's event history names: its digital and real nets, each
/// declared bus's label with its members' `D()` columns, and the `D()` and
/// `E()` names of the nets the analysis has no column for.
pub(super) struct EventNames {
    digital: HashSet<String>,
    real: HashSet<String>,
    buses: Vec<(String, Vec<String>)>,
    history_only: Vec<String>,
}

impl EventNames {
    pub(super) fn of(analysis: &AnalysisResult) -> Option<Self> {
        let Some(AnalysisResultPayload::TransientEvents {
            digital_traces,
            real_traces,
            digital_buses,
            ..
        }) = analysis.result_payload.as_ref()
        else {
            return None;
        };
        let columns: HashSet<&str> = analysis
            .waveforms
            .iter()
            .map(|waveform| waveform.name.as_str())
            .collect();
        let history_only = digital_traces
            .iter()
            .map(|trace| format!("D({})", trace.node_name))
            .chain(
                real_traces
                    .iter()
                    .map(|trace| format!("E({})", trace.node_name)),
            )
            .filter(|name| !columns.contains(name.as_str()))
            .collect();
        Some(Self {
            history_only,
            digital: digital_traces
                .iter()
                .map(|trace| trace.node_name.clone())
                .collect(),
            real: real_traces
                .iter()
                .map(|trace| trace.node_name.clone())
                .collect(),
            buses: digital_buses
                .iter()
                .map(|bus| {
                    (
                        format!("{}[{}:{}]", bus.name, bus.msb, bus.lsb),
                        bus.members.iter().map(|net| format!("D({net})")).collect(),
                    )
                })
                .collect(),
        })
    }

    /// The lane kind a column draws as, when the history records its net.
    fn lane_kind(&self, column: &str) -> Option<TraceLaneKind> {
        let inner = |prefix: &str| {
            column
                .strip_prefix(prefix)
                .and_then(|rest| rest.strip_suffix(')'))
        };
        if inner("D(").is_some_and(|net| self.digital.contains(net)) {
            Some(TraceLaneKind::Logic)
        } else if inner("E(").is_some_and(|net| self.real.contains(net)) {
            Some(TraceLaneKind::Real)
        } else {
            None
        }
    }

    /// The `D()` and `E()` names of the nets the analysis has no column for.
    pub(super) fn history_only(&self) -> &[String] {
        &self.history_only
    }

    fn declares(&self, label: &str, members: &[String]) -> bool {
        self.buses
            .iter()
            .any(|(declared, columns)| declared == label && columns == members)
    }
}

/// The column a document trace was provisioned from.
fn column(trace: &Trace) -> &str {
    trace.source_signal().unwrap_or(&trace.label)
}

/// The layout the document records for `pane`, keeping the session's
/// selection, which is the reader's and not the document's. A pane that
/// records no lanes yet takes the session's whole: a view saved from the
/// strip keeps the lanes the reader was looking at.
pub(super) fn projected_layout(
    document: &VisualizationDocument,
    pane: PaneId,
    names: &EventNames,
    session: Option<&EventLaneLayout>,
) -> EventLaneLayout {
    if let Some(session) = session
        && document.event_lane_order(pane).is_empty()
        && !document.user_buses().iter().any(|bus| bus.pane_id == pane)
    {
        return session.clone();
    }
    let columns: HashMap<TraceId, &str> = document
        .traces()
        .iter()
        .filter(|trace| trace.pane_id == pane)
        .map(|trace| (trace.id, column(trace)))
        .collect();
    let mut layout = EventLaneLayout {
        selected: session
            .map(|layout| layout.selected.clone())
            .unwrap_or_default(),
        ..EventLaneLayout::default()
    };
    layout.order = document
        .event_lane_order(pane)
        .iter()
        .filter_map(|lane| match lane {
            EventLaneRef::Trace { trace_id } => columns
                .get(trace_id)
                .map(|column| LaneKey::Signal((*column).to_owned())),
            EventLaneRef::Bus { name } => Some(LaneKey::Bus(name.clone())),
            // A real net is a step trace, not a row of the stack.
            EventLaneRef::Net { name } => name
                .starts_with("D(")
                .then(|| LaneKey::Signal(name.clone())),
        })
        .collect();
    for bus in document
        .user_buses()
        .iter()
        .filter(|bus| bus.pane_id == pane)
    {
        let Some(members) = bus
            .members
            .iter()
            .map(|id| columns.get(id).map(|column| (*column).to_owned()))
            .chain(bus.nets.iter().map(|net| Some(net.clone())))
            .collect::<Option<Vec<_>>>()
        else {
            continue;
        };
        layout.buses.insert(
            bus.name.clone(),
            BusView {
                radix: bus.radix,
                expanded: bus.expanded,
            },
        );
        if !names.declares(&bus.name, &members) {
            layout.user_buses.push(UserLaneBus {
                name: bus.name.clone(),
                members,
            });
        }
    }
    layout
}

/// The history-only nets the document has on `pane`: those its order lists
/// and the members of the buses it groups or orders. `None` while the pane
/// records no lanes at all, when the reader's own choice stands.
pub(super) fn plotted_history_nets(
    document: &VisualizationDocument,
    pane: PaneId,
    names: &EventNames,
) -> Option<HashSet<String>> {
    let order = document.event_lane_order(pane);
    let buses: Vec<&UserBus> = document
        .user_buses()
        .iter()
        .filter(|bus| bus.pane_id == pane)
        .collect();
    if order.is_empty() && buses.is_empty() {
        return None;
    }
    let mut plotted: HashSet<String> = buses
        .iter()
        .flat_map(|bus| bus.nets.iter().cloned())
        .collect();
    for lane in order {
        match lane {
            EventLaneRef::Net { name } => {
                plotted.insert(name.clone());
            }
            EventLaneRef::Bus { name } => plotted.extend(
                names
                    .buses
                    .iter()
                    .filter(|(label, _)| label == name)
                    .flat_map(|(_, members)| members.iter().cloned()),
            ),
            EventLaneRef::Trace { .. } => {}
        }
    }
    plotted.retain(|name| names.history_only.contains(name));
    Some(plotted)
}

/// The edits that bring `pane`'s lanes in the document in line with
/// `layout`, the rows as drawn, and `plotted`, the history-only nets on the
/// plot: each plotted event column's lane kind, the buses, then the order,
/// which lists a plotted history-only net by name. Nothing when they already
/// agree.
pub(super) fn lane_edits(
    document: &VisualizationDocument,
    pane: PaneId,
    names: &EventNames,
    layout: &EventLaneLayout,
    plotted: &[&str],
) -> Vec<DocumentEdit> {
    let traces: Vec<&Trace> = document
        .traces()
        .iter()
        .filter(|trace| trace.pane_id == pane)
        .collect();
    let history_only = |name: &str| names.history_only.iter().any(|net| net == name);
    let mut edits = Vec::new();
    let trace_of = |column_name: &str| {
        traces
            .iter()
            .find(|trace| column(trace) == column_name)
            .copied()
    };
    let mut logic: HashSet<TraceId> = HashSet::new();
    for trace in traces.iter().filter(|trace| trace.visible) {
        if let Some(kind) = names.lane_kind(column(trace)) {
            mark(&mut edits, &mut logic, trace, kind);
        }
    }

    // A user bus as the reader grouped it; a declared bus only once it is
    // read other than the default way.
    let mut desired: Vec<UserBus> = Vec::new();
    // Members are all traces, or all nets the source keeps only as history.
    let bus = |name: &str, members: &[String]| -> Option<UserBus> {
        let (members, nets) = match members
            .iter()
            .map(|member| trace_of(member))
            .collect::<Option<Vec<_>>>()
        {
            Some(traces) => (traces.iter().map(|trace| trace.id).collect(), Vec::new()),
            None if members.iter().all(|member| history_only(member)) => {
                (Vec::new(), members.to_vec())
            }
            None => return None,
        };
        let view = layout.bus_view(name);
        Some(UserBus {
            pane_id: pane,
            name: name.to_owned(),
            members,
            nets,
            radix: view.radix,
            expanded: view.expanded,
        })
    };
    for user in &layout.user_buses {
        desired.extend(bus(&user.name, &user.members));
    }
    for (label, members) in &names.buses {
        if layout.bus_view(label) != BusView::default() {
            desired.extend(bus(label, members));
        }
    }
    for wanted in &desired {
        for member in &wanted.members {
            if let Some(trace) = traces.iter().find(|trace| trace.id == *member)
                && !logic.contains(member)
            {
                mark(&mut edits, &mut logic, trace, TraceLaneKind::Logic);
            }
        }
    }
    let held: Vec<&UserBus> = document
        .user_buses()
        .iter()
        .filter(|bus| bus.pane_id == pane)
        .collect();
    for gone in held
        .iter()
        .filter(|held| !desired.iter().any(|wanted| wanted.name == held.name))
    {
        edits.push(DocumentEdit::RemoveUserBus {
            pane_id: pane,
            name: gone.name.clone(),
        });
    }
    for wanted in desired {
        if !held.iter().any(|held| **held == wanted) {
            edits.push(DocumentEdit::DefineUserBus(wanted));
        }
    }

    let mut order: Vec<EventLaneRef> = layout
        .order
        .iter()
        .filter_map(|key| match key {
            LaneKey::Signal(signal) if history_only(signal) => plotted
                .contains(&signal.as_str())
                .then(|| EventLaneRef::Net {
                    name: signal.clone(),
                }),
            LaneKey::Signal(signal) => trace_of(signal)
                .filter(|trace| logic.contains(&trace.id))
                .map(|trace| EventLaneRef::Trace { trace_id: trace.id }),
            LaneKey::Bus(name) => Some(EventLaneRef::Bus { name: name.clone() }),
        })
        .collect();
    // A plotted real net is a step trace, in no row, and is listed after
    // the rows so the document still knows it is on the pane.
    order.extend(
        plotted
            .iter()
            .filter(|name| name.starts_with("E("))
            .map(|name| EventLaneRef::Net {
                name: (*name).to_owned(),
            }),
    );
    if order.as_slice() != document.event_lane_order(pane) {
        edits.push(DocumentEdit::SetEventLaneOrder(EventLaneOrder {
            pane_id: pane,
            lanes: order,
        }));
    }
    edits
}

/// Record `trace` as a lane of `kind` when the document still holds it as a
/// value trace, and note the logic lanes an order or a bus may name.
fn mark(
    edits: &mut Vec<DocumentEdit>,
    logic: &mut HashSet<TraceId>,
    trace: &Trace,
    kind: TraceLaneKind,
) {
    if kind == TraceLaneKind::Logic {
        logic.insert(trace.id);
    }
    if trace.lane.kind == TraceLaneKind::Value {
        edits.push(DocumentEdit::SetTraceLane {
            trace_id: trace.id,
            lane: TraceLane { kind, ..trace.lane },
        });
    }
}

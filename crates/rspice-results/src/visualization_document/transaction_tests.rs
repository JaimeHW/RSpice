//! A transaction applied in place: a refused one leaves the view exactly as
//! it was, a long mixed history undoes and redoes exactly, and the removals
//! of one transaction act as each would on its own, in their order.

use super::*;

/// A document of one table with three signals, a second pane, an axis
/// link between the panes' time axes, two logic lanes on a user bus and an
/// event-lane order, a cursor and a marker.
struct View {
    document: VisualizationDocument,
    pane: PaneId,
    second: PaneId,
    bits: [TraceId; 2],
    real: TraceId,
    cursor: CursorId,
    marker: MarkerId,
}

fn table(binding: DatasetBinding) -> SourceDataset {
    let column = |key: &str, role| {
        SourceColumn::new(key, key, ValueType::Real, role, None).expect("valid column")
    };
    SourceDataset::new(
        binding,
        vec![
            column("time", ColumnRole::Coordinate),
            column("D(b[1])", ColumnRole::Signal),
            column("D(b[0])", ColumnRole::Signal),
            column("V(out)", ColumnRole::Signal),
        ],
        vec![
            SourceRow::new(vec![TypedValue::Real(0.0); 4]),
            SourceRow::new(vec![
                TypedValue::Real(1.0e-8),
                TypedValue::Real(1.0),
                TypedValue::Real(0.0),
                TypedValue::Real(0.25),
            ]),
        ],
    )
    .expect("valid dataset")
}

fn commit(document: &mut VisualizationDocument, edits: Vec<DocumentEdit>) -> Vec<EntityRef> {
    document
        .transact(document.revision(), edits)
        .expect("the transaction commits")
        .created
}

fn axis(pane_id: PaneId, label: &str, orientation: AxisOrientation) -> DocumentEdit {
    DocumentEdit::AddAxis(NewAxis {
        pane_id,
        label: label.to_owned(),
        orientation,
        scale: AxisScale::Linear,
        unit: None,
        range: None,
    })
}

fn logic() -> TraceLane {
    TraceLane {
        kind: TraceLaneKind::Logic,
        ..TraceLane::default()
    }
}

fn view() -> View {
    let binding = DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([9; 32]));
    let mut document = VisualizationDocument::new("Transactions", vec![table(binding)]).unwrap();
    let pane = document.panes()[0].id;
    let page_id = document.pages()[0].id;
    let created = commit(
        &mut document,
        vec![
            axis(pane, "Time", AxisOrientation::Horizontal),
            axis(pane, "Value", AxisOrientation::VerticalLeft),
            DocumentEdit::AddPane {
                page_id,
                title: "Second".to_owned(),
                kind: PaneKind::Cartesian,
            },
        ],
    );
    let [
        EntityRef::Axis(x_axis),
        EntityRef::Axis(y_axis),
        EntityRef::Pane(second),
    ] = created[..]
    else {
        unreachable!("two axes and a pane were added");
    };
    let created = commit(
        &mut document,
        vec![axis(second, "Time", AxisOrientation::Horizontal)],
    );
    let [EntityRef::Axis(second_x)] = created[..] else {
        unreachable!("an axis was added");
    };
    let trace = |signal: &str| {
        DocumentEdit::AddTrace(NewTrace {
            pane_id: pane,
            binding,
            signal_key: signal.to_owned(),
            coordinate_key: "time".to_owned(),
            row_predicates: Vec::new(),
            x_axis_id: x_axis,
            y_axis_id: y_axis,
            label: signal.to_owned(),
        })
    };
    let created = commit(
        &mut document,
        vec![
            trace("D(b[1])"),
            trace("D(b[0])"),
            trace("V(out)"),
            DocumentEdit::AddCursor {
                pane_id: pane,
                axis_id: x_axis,
                position: TypedValue::Real(0.0),
                label: "C1".to_owned(),
            },
            DocumentEdit::AddLinkGroup {
                label: "Time".to_owned(),
                kind: LinkKind::HorizontalViewport,
                members: vec![EntityRef::Axis(x_axis), EntityRef::Axis(second_x)],
            },
        ],
    );
    let [
        EntityRef::Trace(bit1),
        EntityRef::Trace(bit0),
        EntityRef::Trace(real),
        EntityRef::Cursor(cursor),
        EntityRef::LinkGroup(_),
    ] = created[..]
    else {
        unreachable!("three traces, a cursor and a link group were added");
    };
    let created = commit(
        &mut document,
        vec![
            DocumentEdit::SetTraceLane {
                trace_id: bit1,
                lane: logic(),
            },
            DocumentEdit::SetTraceLane {
                trace_id: bit0,
                lane: logic(),
            },
            DocumentEdit::DefineUserBus(UserBus {
                pane_id: pane,
                name: "b".to_owned(),
                members: vec![bit1, bit0],
                nets: Vec::new(),
                radix: BusRadix::Unsigned,
                expanded: false,
                value_names: Vec::new(),
            }),
            DocumentEdit::SetEventLaneOrder(EventLaneOrder {
                pane_id: pane,
                lanes: vec![
                    EventLaneRef::Trace { trace_id: bit0 },
                    EventLaneRef::Trace { trace_id: bit1 },
                ],
            }),
            DocumentEdit::AddMarker {
                pane_id: pane,
                trace_id: real,
                coordinate: TypedValue::Real(1.0e-8),
                label: "M1".to_owned(),
            },
        ],
    );
    let [EntityRef::Marker(marker)] = created[..] else {
        unreachable!("a marker was added");
    };
    View {
        document,
        pane,
        second,
        bits: [bit1, bit0],
        real,
        cursor,
        marker,
    }
}

fn bytes(document: &VisualizationDocument) -> Vec<u8> {
    serde_json::to_vec(document).expect("the document serializes")
}

/// A transaction refused by an edit it cannot apply, or by what its edits
/// leave behind, leaves the view byte-identical, whatever the edits before
/// the refusal made, changed or took out; and the view keeps working as it
/// did, the signals it plots included.
#[test]
fn a_refused_transaction_leaves_the_view_byte_identical() {
    let View {
        mut document,
        pane,
        second,
        bits: [bit1, bit0],
        real,
        cursor,
        marker,
    } = view();
    let page_id = document.pages()[0].id;
    let busy = || {
        vec![
            DocumentEdit::MoveCursor {
                cursor_id: cursor,
                position: TypedValue::Real(1.0e-8),
            },
            DocumentEdit::SetTraceColor {
                trace_id: bit0,
                color: Some(TraceColor::from_rgb(1, 2, 3)),
            },
            DocumentEdit::Rename {
                entity: EntityRef::Pane(pane),
                value: "Renamed".to_owned(),
            },
            DocumentEdit::AddPane {
                page_id,
                title: "Third".to_owned(),
                kind: PaneKind::Cartesian,
            },
            DocumentEdit::SetPaneHidden {
                pane_id: second,
                hidden: true,
            },
            DocumentEdit::Remove(EntityRef::Marker(marker)),
            DocumentEdit::Remove(EntityRef::Trace(real)),
            DocumentEdit::Remove(EntityRef::Trace(bit1)),
        ]
    };
    let refusals: [(&str, DocumentEdit); 3] = [
        (
            "an edit of a trace the transaction took out",
            DocumentEdit::SetTraceVisibility {
                trace_id: bit1,
                visible: false,
            },
        ),
        (
            "a lane of an event-lane order made a value lane",
            DocumentEdit::SetTraceLane {
                trace_id: bit0,
                lane: TraceLane::default(),
            },
        ),
        (
            "a pane whose time axis is linked to another pane's",
            DocumentEdit::Remove(EntityRef::Pane(second)),
        ),
    ];
    for (what, refused) in refusals {
        let before = bytes(&document);
        let revision = document.revision();
        let mut edits = busy();
        edits.push(refused);
        assert!(
            document.transact(revision, edits).is_err(),
            "{what} is refused"
        );
        assert_eq!(document.revision(), revision, "{what}: no new revision");
        assert_eq!(bytes(&document), before, "{what}: nothing changed");
    }
    commit(
        &mut document,
        vec![DocumentEdit::SetSignalsPlotted {
            pane_id: pane,
            signals: vec!["D(b[1])".to_owned(), "V(out)".to_owned()],
            plotted: false,
        }],
    );
    let hidden: Vec<TraceId> = document
        .traces()
        .iter()
        .filter(|trace| !trace.visible)
        .map(|trace| trace.id)
        .collect();
    assert_eq!(
        hidden,
        vec![bit1, real],
        "the traces a refused transaction took out are found again by their signals"
    );
}

/// What a document holds for undo to compare: everything but its revision
/// and the identity it makes next, neither of which steps back.
fn held(document: &VisualizationDocument) -> serde_json::Value {
    let mut value = serde_json::to_value(document).expect("the document serializes");
    let object = value.as_object_mut().expect("a document is an object");
    object.remove("revision");
    object.remove("next_serial");
    object.remove("next_marker_number");
    value
}

/// Every step of a long history of mixed edits (made, changed and taken
/// out entities of every kind, settings, buses and lane orders, a pane
/// taken out with everything on it) undoes back to the view before it and
/// redoes to the view after it, exactly.
#[test]
fn a_long_mixed_history_undoes_and_redoes_exactly() {
    let View {
        mut document,
        pane,
        second,
        bits: [bit1, bit0],
        real,
        cursor,
        marker,
    } = view();
    let page_id = document.pages()[0].id;
    let history: Vec<Vec<DocumentEdit>> = vec![
        vec![DocumentEdit::MoveCursor {
            cursor_id: cursor,
            position: TypedValue::Real(1.0e-8),
        }],
        vec![
            DocumentEdit::SetTraceColor {
                trace_id: bit1,
                color: Some(TraceColor::from_rgb(4, 5, 6)),
            },
            DocumentEdit::SetTraceVisibility {
                trace_id: real,
                visible: false,
            },
        ],
        vec![DocumentEdit::Rename {
            entity: EntityRef::Trace(real),
            value: "Output".to_owned(),
        }],
        vec![
            DocumentEdit::AddPane {
                page_id,
                title: "Third".to_owned(),
                kind: PaneKind::Cartesian,
            },
            DocumentEdit::SetPaneMaximized {
                pane_id: pane,
                maximized: true,
            },
        ],
        vec![DocumentEdit::SetReadoutPin {
            pane_id: pane,
            sheet: crate::result_presentation::ResultViewer::Waves,
            x: Some(1.0e-8),
        }],
        vec![DocumentEdit::RemoveUserBus {
            pane_id: pane,
            name: "b".to_owned(),
        }],
        vec![
            DocumentEdit::Remove(EntityRef::Marker(marker)),
            DocumentEdit::Remove(EntityRef::Trace(bit1)),
            DocumentEdit::Remove(EntityRef::Cursor(cursor)),
        ],
        vec![DocumentEdit::AddMarker {
            pane_id: pane,
            trace_id: bit0,
            coordinate: TypedValue::Real(0.0),
            label: "M2".to_owned(),
        }],
        vec![DocumentEdit::ClearMarkers { pane_id: None }],
        vec![
            DocumentEdit::Remove(EntityRef::LinkGroup(document.link_groups()[0].id)),
            DocumentEdit::Remove(EntityRef::Pane(second)),
        ],
        vec![DocumentEdit::Remove(EntityRef::Trace(real))],
    ];
    let mut views = vec![held(&document)];
    let mut steps = Vec::new();
    for edits in history {
        let (_, step) = document
            .transact_with_undo(document.revision(), edits)
            .expect("each edit of the history commits");
        steps.push(step);
        views.push(held(&document));
    }
    let mut redo = Vec::new();
    while let Some(step) = steps.pop() {
        let (_, reverse) = document
            .take_step(document.revision(), step)
            .expect("each step undoes");
        redo.push(reverse);
        assert_eq!(
            held(&document),
            views[steps.len()],
            "undo {} puts back the view before it",
            steps.len() + 1
        );
    }
    let mut at = 0;
    while let Some(step) = redo.pop() {
        let (_, reverse) = document
            .take_step(document.revision(), step)
            .expect("each step redoes");
        steps.push(reverse);
        at += 1;
        assert_eq!(
            held(&document),
            views[at],
            "redo {at} puts back the view after it"
        );
    }
}

/// The removals of one transaction act as each would on its own, in their
/// order: a marker taken out before its trace lets the trace go, one taken
/// out after does not; an entity taken out twice is refused; what is left
/// keeps its order; and a bus of history-only nets, which holds no trace,
/// stays when a trace goes.
#[test]
fn the_removals_of_one_transaction_follow_their_order() {
    let View {
        mut document,
        pane,
        bits: [bit1, bit0],
        real,
        marker,
        ..
    } = view();
    commit(
        &mut document,
        vec![DocumentEdit::DefineUserBus(UserBus {
            pane_id: pane,
            name: "nets".to_owned(),
            members: Vec::new(),
            nets: vec!["D(n[1])".to_owned(), "D(n[0])".to_owned()],
            radix: BusRadix::Unsigned,
            expanded: false,
            value_names: Vec::new(),
        })],
    );
    let refused = document.transact(
        document.revision(),
        vec![
            DocumentEdit::Remove(EntityRef::Trace(real)),
            DocumentEdit::Remove(EntityRef::Marker(marker)),
        ],
    );
    assert_eq!(
        refused.unwrap_err(),
        VisualizationError::EntityInUse {
            entity: EntityRef::Trace(real),
            dependent: EntityRef::Marker(marker),
        }
    );
    let refused = document.transact(
        document.revision(),
        vec![
            DocumentEdit::Remove(EntityRef::Trace(bit0)),
            DocumentEdit::Remove(EntityRef::Trace(bit0)),
        ],
    );
    assert_eq!(
        refused.unwrap_err(),
        VisualizationError::EntityNotFound(EntityRef::Trace(bit0))
    );
    let receipt = document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::Remove(EntityRef::Marker(marker)),
                DocumentEdit::Remove(EntityRef::Trace(real)),
                DocumentEdit::Remove(EntityRef::Trace(bit1)),
            ],
        )
        .expect("a marker taken out first lets its trace go");
    assert_eq!(
        receipt.tombstoned,
        vec![
            EntityRef::Marker(marker),
            EntityRef::Trace(real),
            EntityRef::Trace(bit1),
        ]
    );
    let left: Vec<TraceId> = document.traces().iter().map(|trace| trace.id).collect();
    assert_eq!(left, vec![bit0]);
    let buses: Vec<(&str, &[TraceId])> = document
        .user_buses()
        .iter()
        .map(|bus| (bus.name.as_str(), bus.members.as_slice()))
        .collect();
    assert_eq!(
        buses,
        vec![("b", &[bit0][..]), ("nets", &[][..])],
        "the trace bus narrows and the net bus stays"
    );
}

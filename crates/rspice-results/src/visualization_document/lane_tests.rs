//! Event lanes and user buses: what the waveform stack draws a trace as, and
//! the logic lanes the reader grouped into a word, survive a save and reload,
//! migrate in from schema 8 unchanged, and keep the document graph valid.

use super::*;

fn source_binding() -> DatasetBinding {
    DatasetBinding::new(DatasetId::new(), ContentDigest::from_bytes([7; 32]))
}

fn event_dataset(binding: DatasetBinding) -> SourceDataset {
    let column = |key: &str, role| {
        SourceColumn::new(key, key, ValueType::Real, role, None).expect("valid column")
    };
    SourceDataset::new(
        binding,
        vec![
            column("time", ColumnRole::Coordinate),
            column("D(b[1])", ColumnRole::Signal),
            column("D(b[0])", ColumnRole::Signal),
            column("E(w)", ColumnRole::Signal),
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

/// A document whose first pane holds three traces: two bits and a real net.
fn lane_document() -> (VisualizationDocument, PaneId, [TraceId; 3]) {
    let source = source_binding();
    let mut document =
        VisualizationDocument::new("Event review", vec![event_dataset(source)]).unwrap();
    let pane = document.panes()[0].id;
    let axis = |label: &str, orientation| {
        DocumentEdit::AddAxis(NewAxis {
            pane_id: pane,
            label: label.to_owned(),
            orientation,
            scale: AxisScale::Linear,
            unit: None,
            range: None,
        })
    };
    let receipt = document
        .transact(
            document.revision(),
            vec![
                axis("Time", AxisOrientation::Horizontal),
                axis("Value", AxisOrientation::VerticalLeft),
            ],
        )
        .unwrap();
    let [EntityRef::Axis(x_axis), EntityRef::Axis(y_axis)] = receipt.created[..] else {
        unreachable!("two axes were added");
    };
    let trace = |signal: &str| {
        DocumentEdit::AddTrace(NewTrace {
            pane_id: pane,
            binding: source,
            signal_key: signal.to_owned(),
            coordinate_key: "time".to_owned(),
            row_predicates: Vec::new(),
            x_axis_id: x_axis,
            y_axis_id: y_axis,
            label: signal.to_owned(),
        })
    };
    let receipt = document
        .transact(
            document.revision(),
            vec![trace("D(b[1])"), trace("D(b[0])"), trace("E(w)")],
        )
        .unwrap();
    let [
        EntityRef::Trace(bit1),
        EntityRef::Trace(bit0),
        EntityRef::Trace(real),
    ] = receipt.created[..]
    else {
        unreachable!("three traces were added");
    };
    (document, pane, [bit1, bit0, real])
}

fn logic() -> TraceLane {
    TraceLane {
        kind: TraceLaneKind::Logic,
        ..TraceLane::default()
    }
}

/// The lanes, radix and user bus of a reviewed event page.
fn reviewed_lanes(document: &mut VisualizationDocument, pane: PaneId, traces: [TraceId; 3]) {
    let [bit1, bit0, real] = traces;
    document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::SetTraceLane {
                    trace_id: bit1,
                    lane: logic(),
                },
                DocumentEdit::SetTraceLane {
                    trace_id: bit0,
                    lane: logic(),
                },
                DocumentEdit::SetTraceLane {
                    trace_id: real,
                    lane: TraceLane {
                        kind: TraceLaneKind::Real,
                        ..TraceLane::default()
                    },
                },
                DocumentEdit::DefineUserBus(UserBus {
                    pane_id: pane,
                    name: "pair".to_owned(),
                    members: vec![bit1, bit0],
                    nets: Vec::new(),
                    radix: BusRadix::Hex,
                    expanded: true,
                    value_names: Vec::new(),
                }),
            ],
        )
        .unwrap();
}

#[test]
fn lane_kinds_radix_and_user_buses_survive_a_save_and_reload() {
    let (mut document, pane, traces) = lane_document();
    reviewed_lanes(&mut document, pane, traces);

    let saved = serde_json::to_string(&document).unwrap();
    let reloaded: VisualizationDocument = serde_json::from_str(&saved).unwrap();

    assert_eq!(reloaded, document);
    let kinds: Vec<_> = reloaded
        .traces()
        .iter()
        .map(|trace| trace.lane.kind)
        .collect();
    assert_eq!(
        kinds,
        [
            TraceLaneKind::Logic,
            TraceLaneKind::Logic,
            TraceLaneKind::Real
        ]
    );
    assert_eq!(
        reloaded.user_buses(),
        [UserBus {
            pane_id: pane,
            name: "pair".to_owned(),
            members: vec![traces[0], traces[1]],
            nets: Vec::new(),
            radix: BusRadix::Hex,
            expanded: true,
            value_names: Vec::new(),
        }]
    );
    assert_eq!(
        reloaded.content_digest().unwrap(),
        document.content_digest().unwrap()
    );
}

#[test]
fn a_bus_lane_keeps_its_radix_and_expansion_across_a_reload() {
    let (mut document, _, [bit1, ..]) = lane_document();
    let lane = TraceLane {
        kind: TraceLaneKind::Bus,
        radix: BusRadix::Signed,
        expanded: true,
    };
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::SetTraceLane {
                trace_id: bit1,
                lane,
            }],
        )
        .unwrap();
    let saved = serde_json::to_value(&document).unwrap();
    assert_eq!(
        saved["traces"][0]["lane"],
        serde_json::json!({"kind": "bus", "radix": "signed", "expanded": true})
    );
    let reloaded: VisualizationDocument = serde_json::from_value(saved).unwrap();
    assert_eq!(reloaded.traces()[0].lane, lane);
}

/// A bus's named words are part of its definition: saved, reloaded, and
/// refused when they are not distinct and ascending. A document from before
/// they existed cannot hold any.
#[test]
fn a_bus_keeps_its_value_names_across_a_reload() {
    let (mut document, pane, [bit1, bit0, _]) = lane_document();
    let named = |value: u64, name: &str| BusValueName {
        value,
        name: name.to_owned(),
    };
    for (trace_id, kind) in [(bit1, TraceLaneKind::Logic), (bit0, TraceLaneKind::Logic)] {
        document
            .transact(
                document.revision(),
                vec![DocumentEdit::SetTraceLane {
                    trace_id,
                    lane: TraceLane {
                        kind,
                        ..TraceLane::default()
                    },
                }],
            )
            .unwrap();
    }
    let bus = |value_names: Vec<BusValueName>| UserBus {
        pane_id: pane,
        name: "state".to_owned(),
        members: vec![bit1, bit0],
        nets: Vec::new(),
        radix: BusRadix::Unsigned,
        expanded: false,
        value_names,
    };
    let states = vec![named(0, "IDLE"), named(1, "LOAD"), named(3, "DONE")];
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::DefineUserBus(bus(states.clone()))],
        )
        .unwrap();
    let saved = serde_json::to_value(&document).unwrap();
    assert_eq!(saved["schema_version"], serde_json::json!(12));
    assert_eq!(
        saved["user_buses"][0]["value_names"][2],
        serde_json::json!({"value": 3, "name": "DONE"})
    );
    let reloaded: VisualizationDocument = serde_json::from_value(saved.clone()).unwrap();
    assert_eq!(reloaded.user_buses()[0].value_names, states);
    assert_eq!(bus_value_name(&states, 3), Some("DONE"));
    assert_eq!(bus_value_name(&states, 2), None);

    let mut legacy = saved;
    legacy["schema_version"] = serde_json::json!(11);
    assert!(serde_json::from_value::<VisualizationDocument>(legacy).is_err());

    for unordered in [
        vec![named(1, "LOAD"), named(0, "IDLE")],
        vec![named(1, "LOAD"), named(1, "AGAIN")],
    ] {
        assert!(
            document
                .transact(
                    document.revision(),
                    vec![DocumentEdit::DefineUserBus(bus(unordered))],
                )
                .is_err()
        );
    }
}

/// A schema-8 document is one whose every trace is a value lane and that
/// groups no bus. It loads unchanged, and its saved bytes differ only in the
/// version it states.
#[test]
fn a_schema_eight_document_loads_unchanged_and_round_trips() {
    let (document, _, _) = lane_document();
    let mut legacy = serde_json::to_value(&document).unwrap();
    legacy["schema_version"] = serde_json::json!(8);
    assert!(legacy.get("user_buses").is_none());
    assert!(
        legacy["traces"]
            .as_array()
            .unwrap()
            .iter()
            .all(|trace| trace.get("lane").is_none()),
        "a value lane writes nothing, so a schema-8 trace is already one"
    );

    let migrated: VisualizationDocument = serde_json::from_value(legacy.clone()).unwrap();
    assert_eq!(
        migrated.schema_version,
        VisualizationDocument::SCHEMA_VERSION
    );
    assert_eq!(migrated, document);
    assert!(migrated.traces().iter().all(|trace| trace.lane.is_value()));
    assert!(migrated.user_buses().is_empty());

    let mut resaved = serde_json::to_value(&migrated).unwrap();
    resaved["schema_version"] = serde_json::json!(8);
    assert_eq!(resaved, legacy);

    let again: VisualizationDocument =
        serde_json::from_value(serde_json::to_value(&migrated).unwrap()).unwrap();
    assert_eq!(again, migrated);
}

#[test]
fn an_older_document_cannot_carry_lanes_its_schema_never_had() {
    let (mut document, pane, traces) = lane_document();
    reviewed_lanes(&mut document, pane, traces);
    let mut legacy = serde_json::to_value(&document).unwrap();
    legacy["schema_version"] = serde_json::json!(8);
    let error = serde_json::from_value::<VisualizationDocument>(legacy)
        .unwrap_err()
        .to_string();
    assert!(error.contains("schema 9"), "{error}");
}

#[test]
fn a_user_bus_groups_distinct_logic_lanes_of_its_own_pane() {
    let (mut document, pane, [bit1, bit0, real]) = lane_document();
    reviewed_lanes(&mut document, pane, [bit1, bit0, real]);
    let before = document.clone();
    let bus = |members: Vec<TraceId>| {
        DocumentEdit::DefineUserBus(UserBus {
            pane_id: pane,
            name: "bad".to_owned(),
            members,
            nets: Vec::new(),
            radix: BusRadix::Binary,
            expanded: false,
            value_names: Vec::new(),
        })
    };
    for members in [vec![], vec![bit1, bit1], vec![bit1, real]] {
        assert!(
            document
                .transact(document.revision(), vec![bus(members.clone())])
                .is_err(),
            "{members:?} is not a bus"
        );
    }
    // Turning a member back into a value lane would orphan the bus's word.
    assert!(
        document
            .transact(
                document.revision(),
                vec![DocumentEdit::SetTraceLane {
                    trace_id: bit0,
                    lane: TraceLane::default(),
                }],
            )
            .is_err()
    );
    assert_eq!(document, before);

    // Redefining by name replaces; removing by name ungroups and keeps lanes.
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::DefineUserBus(UserBus {
                pane_id: pane,
                name: "pair".to_owned(),
                members: vec![bit0, bit1],
                nets: Vec::new(),
                radix: BusRadix::Unsigned,
                expanded: false,
                value_names: Vec::new(),
            })],
        )
        .unwrap();
    assert_eq!(document.user_buses().len(), 1);
    assert_eq!(document.user_buses()[0].members, [bit0, bit1]);
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::RemoveUserBus {
                pane_id: pane,
                name: "pair".to_owned(),
            }],
        )
        .unwrap();
    assert!(document.user_buses().is_empty());
    assert_eq!(document.traces().len(), 3);
}

#[test]
fn removing_a_member_lane_narrows_its_bus_and_the_last_member_ends_it() {
    let (mut document, pane, [bit1, bit0, real]) = lane_document();
    reviewed_lanes(&mut document, pane, [bit1, bit0, real]);
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::Remove(EntityRef::Trace(bit1))],
        )
        .unwrap();
    assert_eq!(document.user_buses()[0].members, [bit0]);
    document
        .transact(
            document.revision(),
            vec![DocumentEdit::Remove(EntityRef::Trace(bit0))],
        )
        .unwrap();
    assert!(document.user_buses().is_empty());
}

fn order(pane: PaneId, lanes: Vec<EventLaneRef>) -> DocumentEdit {
    DocumentEdit::SetEventLaneOrder(EventLaneOrder {
        pane_id: pane,
        lanes,
    })
}

/// The order the reader put a pane's lanes in, a declared bus among them,
/// survives a save and reload in schema 10.
#[test]
fn an_event_lane_order_survives_a_save_and_reload() {
    let (mut document, pane, [bit1, bit0, real]) = lane_document();
    reviewed_lanes(&mut document, pane, [bit1, bit0, real]);
    let lanes = vec![
        EventLaneRef::Bus {
            name: "x1.step[3:0]".to_owned(),
        },
        EventLaneRef::Trace { trace_id: bit0 },
        EventLaneRef::Bus {
            name: "pair".to_owned(),
        },
        EventLaneRef::Trace { trace_id: bit1 },
    ];
    document
        .transact(document.revision(), vec![order(pane, lanes.clone())])
        .unwrap();
    let saved = serde_json::to_value(&document).unwrap();
    assert_eq!(
        saved["schema_version"],
        serde_json::json!(VisualizationDocument::SCHEMA_VERSION)
    );
    assert_eq!(
        saved["event_lane_orders"][0]["lanes"][0],
        serde_json::json!({"kind": "bus", "name": "x1.step[3:0]"})
    );
    let reloaded: VisualizationDocument = serde_json::from_value(saved).unwrap();
    assert_eq!(reloaded.event_lane_order(pane), lanes.as_slice());

    // A schema-9 document never had an order to carry.
    let mut legacy = serde_json::to_value(&document).unwrap();
    legacy["schema_version"] = serde_json::json!(9);
    let error = serde_json::from_value::<VisualizationDocument>(legacy)
        .unwrap_err()
        .to_string();
    assert!(error.contains("schema 10"), "{error}");
}

/// An order names each lane once, and only logic traces of its own pane;
/// removing a lane, ungrouping a bus or emptying the order forgets the row.
#[test]
fn an_event_lane_order_names_its_own_logic_lanes_once() {
    let (mut document, pane, [bit1, bit0, real]) = lane_document();
    reviewed_lanes(&mut document, pane, [bit1, bit0, real]);
    let before = document.clone();
    for lanes in [
        vec![
            EventLaneRef::Trace { trace_id: bit1 },
            EventLaneRef::Trace { trace_id: bit1 },
        ],
        vec![EventLaneRef::Trace { trace_id: real }],
    ] {
        assert!(
            document
                .transact(document.revision(), vec![order(pane, lanes.clone())])
                .is_err(),
            "{lanes:?} is not an order"
        );
    }
    assert_eq!(document, before);

    let pair = EventLaneRef::Bus {
        name: "pair".to_owned(),
    };
    document
        .transact(
            document.revision(),
            vec![order(
                pane,
                vec![
                    EventLaneRef::Trace { trace_id: bit0 },
                    pair.clone(),
                    EventLaneRef::Trace { trace_id: bit1 },
                ],
            )],
        )
        .unwrap();
    document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::RemoveUserBus {
                    pane_id: pane,
                    name: "pair".to_owned(),
                },
                DocumentEdit::Remove(EntityRef::Trace(bit1)),
            ],
        )
        .unwrap();
    assert_eq!(
        document.event_lane_order(pane),
        [EventLaneRef::Trace { trace_id: bit0 }]
    );
    document
        .transact(document.revision(), vec![order(pane, Vec::new())])
        .unwrap();
    assert!(document.event_lane_order(pane).is_empty());
    let saved = serde_json::to_value(&document).unwrap();
    assert!(saved.get("event_lane_orders").is_none());
}

/// A native mixed run keeps its nets only as event history, so the source
/// has no trace for them. The pane lists such a net by its `D()` name while
/// it is plotted, and a bus over such nets names them the same way; both
/// survive a save and reload, and a bus never mixes the two kinds.
#[test]
fn history_only_nets_are_held_by_name() {
    let (mut document, pane, [bit1, _, _]) = lane_document();
    let nets: Vec<String> = ["D(B1)", "D(B0)"].map(str::to_owned).to_vec();
    let bus = |members: Vec<TraceId>, nets: Vec<String>| UserBus {
        pane_id: pane,
        name: "code".to_owned(),
        members,
        nets,
        radix: BusRadix::Unsigned,
        expanded: true,
        value_names: Vec::new(),
    };
    let lanes = vec![
        EventLaneRef::Net {
            name: "D(SAMP)".to_owned(),
        },
        EventLaneRef::Bus {
            name: "code".to_owned(),
        },
    ];
    document
        .transact(
            document.revision(),
            vec![
                DocumentEdit::DefineUserBus(bus(Vec::new(), nets.clone())),
                order(pane, lanes.clone()),
            ],
        )
        .unwrap();
    let saved = serde_json::to_value(&document).unwrap();
    assert_eq!(
        saved["event_lane_orders"][0]["lanes"][0],
        serde_json::json!({"kind": "net", "name": "D(SAMP)"})
    );
    let reloaded: VisualizationDocument = serde_json::from_value(saved).unwrap();
    assert_eq!(reloaded.event_lane_order(pane), lanes.as_slice());
    assert_eq!(reloaded.user_buses(), [bus(Vec::new(), nets.clone())]);

    for refused in [
        bus(vec![bit1], nets.clone()),
        bus(Vec::new(), vec![nets[0].clone(), nets[0].clone()]),
    ] {
        assert!(
            document
                .transact(
                    document.revision(),
                    vec![DocumentEdit::DefineUserBus(refused.clone())]
                )
                .is_err(),
            "{refused:?} is not a bus"
        );
    }
}

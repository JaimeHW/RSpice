//! Which lanes a strip shows, in what order, and what the reader's requests
//! do to them, over a projected transient with an exact event history.
use super::*;
use crate::waves::ProjectionOptions;
use crate::waves::logic::LaneStackResponse;
use rspice_results::analysis_payload::AnalysisResultPayload;
use rspice_results::events::{
    DigitalBusEvidence, DigitalBusSourceEvidence, DigitalEventPointEvidence,
    DigitalEventTraceEvidence, RealEventPointEvidence, RealEventTraceEvidence,
};
use rspice_results::{
    analysis_result::AnalysisResult, analysis_type::AnalysisType,
    executed_deck::ExecutedDeckArchive, waveform::RetainedTrace,
};
use rspice_ui_kit::tokens::Tokens;
use std::collections::HashSet;

const NS: f64 = 1.0e-9;

fn digital(name: &str, points: &[(f64, u8)]) -> DigitalEventTraceEvidence {
    DigitalEventTraceEvidence {
        node_name: name.to_owned(),
        points: points
            .iter()
            .map(|(time_s, value_code)| DigitalEventPointEvidence {
                time_s: *time_s,
                value_code: *value_code,
            })
            .collect(),
    }
}

/// A transient whose columns are `V(a)`, `D(clk)`, `D(q[1])`, `D(q[0])` and
/// `E(w)`, its history declaring the bus `q[1:0]`. Every column but `V(a)`
/// starts hidden unless `plotted` names it.
fn model(plotted: &[&str]) -> StripModel {
    let grid: Vec<f64> = (0..=10).map(|k| f64::from(k) * 10.0 * NS).collect();
    let column = |name: &str| {
        let mut trace = RetainedTrace::new(name, grid.clone(), vec![0.0; grid.len()], "#fff");
        trace.visible = name == "V(a)" || plotted.contains(&name);
        trace
    };
    let mut run = rspice_results::run::SimulationRun::new(
        1,
        0.0,
        rspice_results::run::ExecutionTarget::LocalDesktop,
    );
    run.analyses.push(
        AnalysisResult::new(1, AnalysisType::Transient, "Tran", 0.0)
            .with_waveforms(
                ["V(a)", "D(clk)", "D(q[1])", "D(q[0])", "E(w)"]
                    .into_iter()
                    .map(column)
                    .collect(),
            )
            .with_result_payload(AnalysisResultPayload::TransientEvents {
                current_impulses: None,
                voltage_impulses: None,
                digital_traces: vec![
                    digital("clk", &[(0.0, 0), (10.0 * NS, 1), (20.0 * NS, 0)]),
                    digital("q[1]", &[(0.0, 0), (30.0 * NS, 1)]),
                    digital("q[0]", &[(0.0, 1), (20.0 * NS, 0)]),
                ],
                real_traces: vec![RealEventTraceEvidence {
                    node_name: "w".to_owned(),
                    points: vec![
                        RealEventPointEvidence {
                            time_s: 0.0,
                            value: 0.25,
                        },
                        RealEventPointEvidence {
                            time_s: 40.0 * NS,
                            value: 0.75,
                        },
                    ],
                }],
                digital_buses: vec![DigitalBusEvidence {
                    name: "q".to_owned(),
                    msb: 1,
                    lsb: 0,
                    members: vec!["q[1]".to_owned(), "q[0]".to_owned()],
                    source: DigitalBusSourceEvidence::Engine,
                }],
                hdl_sources: Vec::new(),
            }),
    );
    let mut models = crate::waves::build_models(
        &[&run],
        &ExecutedDeckArchive::default(),
        &mut DerivedSeries::default(),
        &Tokens::default(),
        ProjectionOptions {
            phase_continuous: false,
            complex_display: Default::default(),
            ac: Default::default(),
            selection: None,
            hidden_family_traces: &HashSet::new(),
        },
        |_| None,
    );
    crate::waves::extent::resolve_x_ranges(&mut models);
    models.remove(0)
}

fn labels(specs: &[LaneSpec]) -> Vec<&str> {
    specs.iter().map(LaneSpec::label).collect()
}

/// A `D()` or `E()` column with an exact history is a lane, never one of
/// the strip's value traces, so it opens no pane of its own resampling.
#[test]
fn event_columns_leave_the_value_traces_for_lanes() {
    let model = model(&["D(clk)", "E(w)"]);
    let names: Vec<&str> = model
        .traces
        .iter()
        .map(|trace| trace.name.as_str())
        .collect();
    assert_eq!(names, ["V(a)"]);
    let lanes: Vec<(&str, LaneTraceKind, bool)> = model
        .lane_traces
        .iter()
        .map(|lane| (lane.net.as_str(), lane.kind, lane.visible))
        .collect();
    assert_eq!(
        lanes,
        [
            ("clk", LaneTraceKind::Logic, true),
            ("q[1]", LaneTraceKind::Logic, false),
            ("q[0]", LaneTraceKind::Logic, false),
            ("w", LaneTraceKind::Real, true),
        ]
    );
}

/// A bus whose members are all plotted is one row with its members under
/// it; with one member missing, the other stands alone.
#[test]
fn a_bus_is_one_row_once_every_member_is_plotted() {
    let partial = model(&["D(clk)", "D(q[0])"]);
    assert_eq!(labels(&lane_specs(&partial, None)), ["clk", "q[0]"]);

    let whole = model(&["D(clk)", "D(q[1])", "D(q[0])"]);
    let specs = lane_specs(&whole, None);
    assert_eq!(labels(&specs), ["clk", "q[1:0]"]);
    let LaneSpec::Bus {
        origin,
        members,
        view,
        ..
    } = &specs[1]
    else {
        panic!("q[1:0] is a bus row");
    };
    assert_eq!(*origin, BusOrigin::Declared);
    assert_eq!(*view, BusView::default());
    let nets: Vec<&str> = members.iter().map(|member| member.net.as_str()).collect();
    assert_eq!(nets, ["q[1]", "q[0]"]);
    assert_eq!(stack_height(&specs), LOGIC_ROW_HEIGHT + BUS_ROW_HEIGHT);
}

/// The reader's order holds; a row newly plotted joins at the bottom.
#[test]
fn the_readers_order_holds_and_new_rows_join_at_the_bottom() {
    let model = model(&["D(clk)", "D(q[1])", "D(q[0])"]);
    let mut layout = EventLaneLayout {
        order: vec![
            LaneKey::Bus("q[1:0]".to_owned()),
            LaneKey::Signal("D(gone)".to_owned()),
        ],
        ..EventLaneLayout::default()
    };
    let specs = lane_specs(&model, Some(&layout));
    assert_eq!(labels(&specs), ["q[1:0]", "clk"]);
    reconcile(&mut layout, &specs);
    assert_eq!(
        layout.order,
        [
            LaneKey::Bus("q[1:0]".to_owned()),
            LaneKey::Signal("D(clk)".to_owned())
        ]
    );
}

/// Radix, expansion, grouping, ungrouping and moves each change the layout
/// and nothing else.
#[test]
fn the_readers_requests_edit_the_layout() {
    let model = model(&["D(clk)", "D(q[1])", "D(q[0])"]);
    let mut layout = EventLaneLayout::default();
    let specs = lane_specs(&model, Some(&layout));
    apply_response(
        &mut layout,
        &specs,
        LaneStackResponse {
            set_radix: Some((1, BusRadix::Signed)),
            toggle_expanded: Some(1),
            ..LaneStackResponse::default()
        },
    );
    let specs = lane_specs(&model, Some(&layout));
    assert_eq!(
        layout.bus_view("q[1:0]"),
        BusView {
            radix: BusRadix::Signed,
            expanded: true
        }
    );
    assert_eq!(
        stack_height(&specs),
        3.0f32.mul_add(LOGIC_ROW_HEIGHT, BUS_ROW_HEIGHT)
    );

    apply_response(
        &mut layout,
        &specs,
        LaneStackResponse {
            move_row: Some((0, true)),
            ..LaneStackResponse::default()
        },
    );
    let specs = lane_specs(&model, Some(&layout));
    assert_eq!(labels(&specs), ["q[1:0]", "clk"]);

    // Grouping two logic rows makes a user bus of them, MSB first.
    let model = model_with_two_logic_rows();
    let mut layout = EventLaneLayout::default();
    let specs = lane_specs(&model, Some(&layout));
    assert_eq!(labels(&specs), ["clk", "q[0]"]);
    apply_response(
        &mut layout,
        &specs,
        LaneStackResponse {
            group: Some(vec![0, 1]),
            ..LaneStackResponse::default()
        },
    );
    assert_eq!(
        layout.user_buses,
        [UserLaneBus {
            name: "bus1".to_owned(),
            members: vec!["D(clk)".to_owned(), "D(q[0])".to_owned()],
        }]
    );
    let specs = lane_specs(&model, Some(&layout));
    assert_eq!(labels(&specs), ["bus1"]);
    assert_eq!(layout.selected, [LaneKey::Bus("bus1".to_owned())]);
    let mut derived = DerivedSeries::default();
    let user = user_bus_lanes(&mut derived, &model, &specs);
    let word = user[0].as_ref().expect("the user bus reassembles");
    assert_eq!(word.members(), ["clk", "q[0]"]);

    apply_response(
        &mut layout,
        &specs,
        LaneStackResponse {
            ungroup: Some(0),
            ..LaneStackResponse::default()
        },
    );
    assert!(layout.user_buses.is_empty());
    assert_eq!(labels(&lane_specs(&model, Some(&layout))), ["clk", "q[0]"]);
}

fn model_with_two_logic_rows() -> StripModel {
    model(&["D(clk)", "D(q[0])"])
}

/// An `E()` lane is a step trace in the pane of its unit and nowhere else,
/// reaching the strip's last instant; it reads exactly at a cursor.
#[test]
fn a_real_lane_steps_in_the_pane_of_its_unit() {
    let model = model(&["E(w)"]);
    let panes = model.unit_panes();
    assert_eq!(panes.len(), 1, "E(w) reads in V, the pane V(a) owns");
    let steps = real_step_traces(&model, &panes[0]);
    assert_eq!(steps.len(), 1);
    assert_eq!(steps[0].label, "E(w)");
    assert_eq!(*steps[0].x, [0.0, 40.0 * NS, 40.0 * NS, 100.0 * NS]);
    assert_eq!(*steps[0].y, [0.25, 0.25, 0.75, 0.75]);

    let rows = lane_readout_rows(
        &model,
        &[],
        &[],
        CursorPair {
            a: Some(40.0 * NS),
            b: Some(39.0 * NS),
        },
        4,
    );
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0].name, "E(w)");
    assert!(rows[0].a.starts_with("750"), "{}", rows[0].a);
    assert!(rows[0].b.starts_with("250"), "{}", rows[0].b);
}

/// The cursor table reads each lane right-continuously at A and B.
#[test]
fn the_cursor_table_reads_logic_and_bus_lanes_at_both_cursors() {
    let model = model(&["D(clk)", "D(q[1])", "D(q[0])"]);
    let layout = EventLaneLayout {
        buses: [(
            "q[1:0]".to_owned(),
            BusView {
                radix: BusRadix::Binary,
                expanded: false,
            },
        )]
        .into(),
        ..EventLaneLayout::default()
    };
    let specs = lane_specs(&model, Some(&layout));
    let rows = lane_readout_rows(
        &model,
        &specs,
        &vec![None; specs.len()],
        CursorPair {
            a: Some(10.0 * NS),
            b: Some(30.0 * NS),
        },
        4,
    );
    let cells: Vec<(&str, &str, &str)> = rows
        .iter()
        .map(|row| (row.name.as_str(), row.a.as_str(), row.b.as_str()))
        .collect();
    assert_eq!(cells, [("clk", "1", "0"), ("q[1:0]", "01", "10")]);
}

#[test]
fn the_cursor_reads_a_bus_in_octal_and_ascii() {
    let model = model(&["D(clk)", "D(q[1])", "D(q[0])"]);
    for (radix, a, b) in [
        (BusRadix::Octal, "0o1", "0o2"),
        (BusRadix::Ascii, "\"\\x01\"", "\"\\x02\""),
    ] {
        let layout = EventLaneLayout {
            buses: [(
                "q[1:0]".to_owned(),
                BusView {
                    radix,
                    expanded: false,
                },
            )]
            .into(),
            ..EventLaneLayout::default()
        };
        let specs = lane_specs(&model, Some(&layout));
        let rows = lane_readout_rows(
            &model,
            &specs,
            &vec![None; specs.len()],
            CursorPair {
                a: Some(10.0 * NS),
                b: Some(30.0 * NS),
            },
            4,
        );
        assert_eq!(
            (rows[1].a.as_str(), rows[1].b.as_str()),
            (a, b),
            "{radix:?}"
        );
    }
}

/// A bus's named words read as their names in the readout, the others in
/// the radix, and the value column is wide enough for the longest name.
#[test]
fn a_named_bus_word_reads_as_its_name() {
    let model = model(&["D(clk)", "D(q[1])", "D(q[0])"]);
    let mut layout = EventLaneLayout::default();
    let specs = lane_specs(&model, Some(&layout));
    let names = vec![
        BusValueName {
            value: 1,
            name: "LOAD".to_owned(),
        },
        BusValueName {
            value: 3,
            name: "UNREACHED".to_owned(),
        },
    ];
    apply_response(
        &mut layout,
        &specs,
        LaneStackResponse {
            set_value_names: Some((1, names.clone())),
            ..LaneStackResponse::default()
        },
    );
    assert_eq!(layout.value_names("q[1:0]"), names);
    let specs = lane_specs(&model, Some(&layout));
    assert_eq!(value_column_chars(&specs), "UNREACHED".len());
    let rows = lane_readout_rows(
        &model,
        &specs,
        &vec![None; specs.len()],
        CursorPair {
            a: Some(10.0 * NS),
            b: Some(30.0 * NS),
        },
        4,
    );
    assert_eq!((rows[1].a.as_str(), rows[1].b.as_str()), ("LOAD", "0x2"));
    // Naming nothing removes the bus's entry.
    apply_response(
        &mut layout,
        &specs,
        LaneStackResponse {
            set_value_names: Some((1, Vec::new())),
            ..LaneStackResponse::default()
        },
    );
    assert!(layout.value_names.is_empty());
}

/// A row's Analog asks for the staircase of what it draws: a bus by its
/// `D()` name, a net by its `D()` column.
#[test]
fn a_rows_analog_names_what_it_draws() {
    let model = model(&["D(clk)", "D(q[1])", "D(q[0])"]);
    let mut layout = EventLaneLayout::default();
    let specs = lane_specs(&model, Some(&layout));
    let asked = |layout: &mut EventLaneLayout, index| {
        apply_response(
            layout,
            &specs,
            LaneStackResponse {
                analog: Some(index),
                ..LaneStackResponse::default()
            },
        )
        .add_expression
    };
    assert_eq!(asked(&mut layout, 0).as_deref(), Some("analog(D(clk))"));
    assert_eq!(asked(&mut layout, 1).as_deref(), Some("analog(D(q[1:0]))"));
}

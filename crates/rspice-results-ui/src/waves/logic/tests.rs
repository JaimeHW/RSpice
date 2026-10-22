//! Model tests on the closed-form mixed fixture: `q` toggles every 10 ns
//! from 0, `p` every 7 ns from 0, and the 4-bit `b` counts on each rising
//! edge of `q`. Every expected instant is the fixture's own, so equality is
//! exact: a lane never computes a time, it only keeps the ones it was given.

use super::*;
use rspice_results::events::{
    DigitalBusEvidence, DigitalBusSourceEvidence, DigitalEventPointEvidence,
    DigitalEventTraceEvidence, RealEventTraceEvidence,
};

pub(super) const NS: f64 = 1.0e-9;

fn trace(name: &str, points: impl IntoIterator<Item = (f64, u8)>) -> DigitalEventTraceEvidence {
    DigitalEventTraceEvidence {
        node_name: name.to_owned(),
        points: points
            .into_iter()
            .map(|(time_s, value_code)| DigitalEventPointEvidence { time_s, value_code })
            .collect(),
    }
}

/// `q`, `p` and the four bits of `b` over a 100 ns run, plus `b`'s
/// declaration.
pub(super) fn fixture() -> (Vec<DigitalEventTraceEvidence>, Vec<DigitalBusEvidence>) {
    let q = trace(
        "q",
        (0..=10).map(|k| (f64::from(k) * 10.0 * NS, (k % 2) as u8)),
    );
    let p = trace(
        "p",
        (0..=14).map(|k| (f64::from(k) * 7.0 * NS, (k % 2) as u8)),
    );
    // `b` counts on q's rising edges at 10, 30, 50, 70 and 90 ns.
    let count_at = |k: u32| (0..k).filter(|edge| edge % 2 == 0).count() as u8;
    let bits = (0..4u8).rev().map(|bit| {
        trace(
            &format!("b[{bit}]"),
            (0..=10u32).map(|k| (f64::from(k) * 10.0 * NS, (count_at(k) >> bit) & 1)),
        )
    });
    let mut digital = vec![q, p];
    digital.extend(bits);
    let bus = DigitalBusEvidence {
        name: "b".to_owned(),
        msb: 3,
        lsb: 0,
        members: (0..4).rev().map(|bit| format!("b[{bit}]")).collect(),
        source: DigitalBusSourceEvidence::Engine,
    };
    (digital, vec![bus])
}

pub(super) fn fixture_lanes() -> EventLanes {
    let (digital, buses) = fixture();
    EventLanes::from_evidence(&digital, &[] as &[RealEventTraceEvidence], &buses)
}

#[test]
fn logic_transitions_land_on_the_exact_event_instants() {
    let lanes = fixture_lanes();
    let q = lanes.logic("q").expect("q is a logic lane");
    let expected: Vec<f64> = (0..=10).map(|k| f64::from(k) * 10.0 * NS).collect();
    assert_eq!(q.times(), expected, "q changes at 0, 10, 20, … ns exactly");
    assert_eq!(q.levels()[0], LogicLevel::Low);
    assert_eq!(q.levels()[1], LogicLevel::High);

    // Drawn, each change after the first is one edge at exactly the mapped
    // instant: nothing is snapped or resampled on the way to the screen.
    let map = TimeMapping {
        t0: 0.0,
        t1: 100.0 * NS,
        left: 50.0,
        right: 1050.0,
    };
    let edges: Vec<(f32, usize)> = plan_lane(q.times(), &map)
        .into_iter()
        .filter_map(|stroke| match stroke {
            LaneStroke::Edge { x, index } => Some((x, index)),
            _ => None,
        })
        .collect();
    assert_eq!(edges.len(), 10, "{edges:?}");
    for (x, index) in edges {
        assert_eq!(x, map.x(q.times()[index]));
        assert!((x - (50.0 + 100.0 * index as f32)).abs() <= 0.5, "{x}");
    }
}

#[test]
fn the_bus_word_from_fifty_to_seventy_nanoseconds_is_three_in_every_radix() {
    let lanes = fixture_lanes();
    let bus = lanes.bus("b").expect("b is a declared bus");
    assert_eq!(bus.name(), "b[3:0]");
    assert_eq!(bus.members(), ["b[3]", "b[2]", "b[1]", "b[0]"]);
    let index = bus
        .times()
        .iter()
        .position(|time| *time == 50.0 * NS)
        .expect("the word changes at 50 ns");
    assert_eq!(bus.times()[index + 1], 70.0 * NS, "segment [50n, 70n)");
    for (radix, spelled) in [
        (BusRadix::Binary, "0011"),
        (BusRadix::Hex, "0x3"),
        (BusRadix::Unsigned, "3"),
        (BusRadix::Signed, "3"),
    ] {
        let word = bus.spell(index, radix).unwrap();
        assert_eq!(word.text(), spelled, "{radix:?}");
        assert_eq!(word.fallback(), None, "{radix:?}");
        assert_eq!(bus.spell_at(55.0 * NS, radix).unwrap().text(), spelled);
    }
}

#[test]
fn a_word_with_an_unknown_bit_falls_back_to_binary_and_says_why() {
    let (mut digital, buses) = fixture();
    // b[1] goes unknown (strong X, code 2) from 80 ns.
    let b1 = digital
        .iter_mut()
        .find(|trace| trace.node_name == "b[1]")
        .unwrap();
    b1.points.retain(|point| point.time_s < 80.0 * NS);
    b1.points.push(DigitalEventPointEvidence {
        time_s: 80.0 * NS,
        value_code: 2,
    });
    let lanes = EventLanes::from_evidence(&digital, &[], &buses);
    let bus = lanes.bus("b").unwrap();
    let index = bus
        .times()
        .iter()
        .position(|time| *time == 80.0 * NS)
        .unwrap();
    assert!(bus.word_is_unresolved(index));
    assert!(!bus.word_is_unresolved(index - 1));
    let word = bus.spell(index, BusRadix::Hex).unwrap();
    assert_eq!(word.text(), "01x0");
    let reason = word.fallback().expect("hexadecimal cannot spell an x bit");
    assert!(reason.contains("unknown (x)"), "{reason}");
    assert_eq!(bus.spell(index, BusRadix::Binary).unwrap().fallback(), None);
}

#[test]
fn grouped_logic_lanes_read_the_same_words_as_the_declared_bus() {
    let lanes = fixture_lanes();
    let user = lanes
        .user_bus("mine", &["b[3]", "b[2]", "b[1]", "b[0]"])
        .expect("every member is a logic lane");
    let declared = lanes.bus("b").unwrap();
    assert_eq!(user.origin(), BusOrigin::User);
    assert_eq!(user.times(), declared.times());
    for index in 0..user.times().len() {
        assert_eq!(user.word(index), declared.word(index));
    }
    assert!(lanes.user_bus("bad", &["q", "nope"]).is_none());
}

#[test]
fn repeated_values_and_same_instant_points_are_not_transitions() {
    let lane = LogicLane::from_events(
        "n",
        [(0.0, 0), (1.0, 0), (2.0, 1), (2.0, 0), (2.0, 1), (3.0, 4)],
    );
    // 0 holds through 1.0; at 2.0 the net settles on 1; a resistive 1 at
    // 3.0 is the same level.
    assert_eq!(lane.times(), [0.0, 2.0]);
    assert_eq!(lane.levels(), [LogicLevel::Low, LogicLevel::High]);
    let z = LogicLane::from_events("z", [(0.0, 12), (1.0, 2)]);
    assert_eq!(z.levels(), [LogicLevel::HighImpedance, LogicLevel::Unknown]);
}

/// Several changes inside one pixel column draw as one activity block, and a
/// lane's plan is bounded by its columns, not by its changes: a million
/// transitions across 400 columns plan in at most a few strokes per column.
#[test]
fn a_million_transitions_plan_in_work_proportional_to_the_pixel_columns() {
    let count = 1_000_000_u32;
    let lane = LogicLane::from_events(
        "fast",
        (0..count).map(|k| (f64::from(k) * NS, (k % 2) as u8)),
    );
    assert_eq!(lane.times().len(), count as usize);
    let columns = 400.0;
    let map = TimeMapping {
        t0: 0.0,
        t1: f64::from(count) * NS,
        left: 0.0,
        right: columns,
    };
    let strokes = plan_lane(lane.times(), &map);
    // Each step of the plan emits at least one stroke and runs at most two
    // bisections, so the stroke count bounds the work done.
    assert!(
        strokes.len() <= 3 * columns as usize,
        "{} strokes for {columns} columns",
        strokes.len()
    );
    assert!(
        matches!(
            strokes.as_slice(),
            [LaneStroke::Activity { x0, x1 }] if *x0 == 0.0 && *x1 == columns
        ),
        "{:?}",
        &strokes[..strokes.len().min(6)]
    );

    // Zoomed in to a window of 40 changes over 400 columns, every change is
    // an exact edge again.
    let zoom = TimeMapping {
        t0: 500_000.0 * NS,
        t1: 500_040.0 * NS,
        ..map
    };
    let edges = plan_lane(lane.times(), &zoom)
        .into_iter()
        .filter(|stroke| matches!(stroke, LaneStroke::Edge { .. }))
        .count();
    assert_eq!(edges, 40);
}

#[test]
fn a_dense_column_between_sparse_edges_is_one_block_and_the_held_value_resumes() {
    // Edges at 10 and 90; a burst of five changes inside column 50.
    let mut points = vec![(0.0, 0), (10.0, 1)];
    points.extend((0..5).map(|k| (50.0 + f64::from(k) * 0.1, (k % 2) as u8)));
    points.push((90.0, 1));
    let lane = LogicLane::from_events("burst", points);
    let map = TimeMapping {
        t0: 0.0,
        t1: 100.0,
        left: 0.0,
        right: 100.0,
    };
    let strokes = plan_lane(lane.times(), &map);
    assert_eq!(
        strokes,
        [
            LaneStroke::Hold {
                x0: 0.0,
                x1: 10.0,
                index: 0
            },
            LaneStroke::Edge { x: 10.0, index: 1 },
            LaneStroke::Hold {
                x0: 10.0,
                x1: 50.0,
                index: 1
            },
            LaneStroke::Activity { x0: 50.0, x1: 51.0 },
            LaneStroke::Hold {
                x0: 51.0,
                x1: 90.0,
                index: 6
            },
            LaneStroke::Edge { x: 90.0, index: 7 },
            LaneStroke::Hold {
                x0: 90.0,
                x1: 100.0,
                index: 7
            },
        ]
    );
}

/// A real net stepping to 0.25 at 5 ns and to 0.75 at 9 ns: each step adds
/// `(t_k, v_{k-1})` then `(t_k, v_k)`, so every segment is horizontal or
/// vertical and none is sloped.
#[test]
fn the_real_step_holds_then_steps_and_never_slopes() {
    let lane = RealLane::from_events("w", [(0.0, 0.0), (5.0 * NS, 0.25), (9.0 * NS, 0.75)]);
    let (x, y) = lane.step_polyline(13.0 * NS);
    assert_eq!(x, [0.0, 5.0 * NS, 5.0 * NS, 9.0 * NS, 9.0 * NS, 13.0 * NS]);
    assert_eq!(y, [0.0, 0.0, 0.25, 0.25, 0.75, 0.75]);
    for k in 1..lane.times().len() {
        let (t, before, after) = (lane.times()[k], lane.values()[k - 1], lane.values()[k]);
        let vertex = x
            .iter()
            .zip(&y)
            .position(|vertex| vertex == (&t, &before))
            .expect("the held value reaches the instant");
        assert_eq!((x[vertex + 1], y[vertex + 1]), (t, after));
    }
    for segment in 0..x.len() - 1 {
        assert!(
            x[segment] == x[segment + 1] || y[segment] == y[segment + 1],
            "segment {segment} is sloped"
        );
    }

    let trace = real_step_trace(&lane, 13.0 * NS, Color32::WHITE, 9);
    assert_eq!(trace.label, "E(w)");
    assert_eq!(trace.y_extremes, Some((0.0, 0.75)));
    assert_eq!(*trace.x, x);
    assert_eq!(lane.value_at(9.0 * NS), Some(0.75), "right-continuous");
    assert_eq!(lane.value_at(8.999 * NS), Some(0.25));
}

//! LTspice `.machine` state machines against LTspice.
//!
//! QSPICE has no `.machine`, so LTspice 26.1.1 batch runs of the same decks
//! decide every value here.

use rspice_core::engine::Engine;
use rspice_core::netlist::{ElementKind, Netlist};

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}\n{deck}"))
}

fn parse_error(deck: &str) -> String {
    match Netlist::parse(deck) {
        Ok(_) => panic!("deck parses:\n{deck}"),
        Err(error) => error.to_string(),
    }
}

fn build_error(deck: &str) -> String {
    match Engine::default().build_circuit(&parse(deck)) {
        Ok(_) => panic!("deck builds:\n{deck}"),
        Err(error) => error.to_string(),
    }
}

struct Run {
    time: Vec<f64>,
    waves: Vec<(String, Vec<f64>)>,
}

impl Run {
    fn new(deck: &str, stop: f64, nodes: &[&str]) -> Self {
        let tran = Engine::default()
            .run_tran(&parse(deck), stop, stop / 1000.0)
            .expect("transient solves");
        let waves = nodes
            .iter()
            .map(|node| {
                let wave = tran
                    .try_voltage_waveform_named(node)
                    .unwrap_or_else(|| panic!("no waveform for {node}"));
                (node.to_string(), wave.to_vec())
            })
            .collect();
        Self {
            time: tran.time.clone(),
            waves,
        }
    }

    fn wave(&self, node: &str) -> &[f64] {
        &self
            .waves
            .iter()
            .find(|(name, _)| name == node)
            .unwrap_or_else(|| panic!("no waveform for {node}"))
            .1
    }

    fn at(&self, node: &str, time: f64) -> f64 {
        let wave = self.wave(node);
        let index = self.time.partition_point(|t| *t < time).max(1);
        let (t0, t1) = (self.time[index - 1], self.time[index]);
        let fraction = if t1 > t0 {
            (time - t0) / (t1 - t0)
        } else {
            0.0
        };
        wave[index - 1] + fraction * (wave[index] - wave[index - 1])
    }

    /// Each time `node` crosses `level`, interpolated, signed by direction.
    fn crossings(&self, node: &str, level: f64) -> Vec<f64> {
        let wave = self.wave(node);
        let mut crossings = Vec::new();
        for index in 1..wave.len() {
            let (a, b) = (wave[index - 1] - level, wave[index] - level);
            if (a < 0.0) != (b < 0.0) {
                let (t0, t1) = (self.time[index - 1], self.time[index]);
                let at = t0 + (t1 - t0) * (-a) / (b - a);
                crossings.push(if b > a { at } else { -at });
            }
        }
        crossings
    }
}

/// `actual` crossings match LTspice's `expected` (rising positive, falling
/// negative) within `tolerance` seconds each.
fn assert_crossings(label: &str, actual: &[f64], expected: &[f64], tolerance: f64) {
    let report = || {
        format!(
            "{label}: RSpice {:?}\n vs LTspice {:?}",
            actual.iter().map(|t| t * 1e6).collect::<Vec<_>>(),
            expected.iter().map(|t| t * 1e6).collect::<Vec<_>>()
        )
    };
    assert_eq!(actual.len(), expected.len(), "{}", report());
    for (actual, expected) in actual.iter().zip(expected) {
        assert!(
            actual.signum() == expected.signum() && (actual - expected).abs() <= tolerance,
            "{} (tolerance {tolerance:e} s)",
            report()
        );
    }
}

#[test]
fn a_machine_block_parses_into_one_element() {
    let netlist = parse(
        "* machine
V1 1 0 1
R1 2 0 1k
.mach 1u
.rule * S1 V(1) > .5
.state S0 0
.state S1 1
.output (2) state
.output (3, 4) V(1)*2
.endmach
.end
",
    );
    let machine = netlist
        .elements
        .iter()
        .find_map(|element| match &element.kind {
            ElementKind::StateMachine(machine) => Some((element, machine)),
            _ => None,
        });
    let (element, machine) = machine.expect("a state machine element");
    assert_eq!(element.nodes, ["2", "0", "3", "4"]);
    assert_eq!(machine.tripdt.as_deref(), Some("1u"));
    assert_eq!(machine.states.len(), 2);
    assert_eq!(machine.rules[0].from, None);
    assert_eq!(machine.rules[0].to, 1);
    assert_eq!(machine.outputs, ["state", "V(1)*2"]);
}

#[test]
fn malformed_blocks_name_the_line() {
    let cases = [
        (
            "* x\n.machine\n.state A 0\n.rule A Z 1\n.endmachine\n.end\n",
            "line 4",
            "unknown state 'Z'",
        ),
        (
            "* x\n.machine\n.state A 0\n.state a 1\n.endmachine\n.end\n",
            "line 4",
            "state 'a' is already defined",
        ),
        (
            "* x\n.machine\n.state A 0\nR1 1 0 1k\n.endmachine\n.end\n",
            "line 4",
            "found 'R1'",
        ),
        (
            "* x\n.machine\n.state A 0\n.end\n",
            "line 2",
            "has no .endmachine",
        ),
        ("* x\n.state A 0\n.end\n", "line 2", "inside a .machine"),
        (
            "* x\n.machine\n.output 2 state\n.endmachine\n.end\n",
            "line 3",
            "expected .output (<node>",
        ),
    ];
    for (deck, line, message) in cases {
        let error = parse_error(deck);
        assert!(
            error.contains(line) && error.contains(message),
            "{error:?} should name {line} and say {message:?}"
        );
    }
    let error = build_error(
        "* x\nV1 1 0 1\nR1 2 0 1k\n.machine\n.state A 0\n.output (2) foo\n.endmachine\n.end\n",
    );
    assert!(error.contains("foo"), "{error}");
    // LTspice: "No such node."
    let error = build_error(
        "* x
V1 1 0 1
R1 2 0 1k
.machine
.state A 0
.state B 1
.rule A B V(typo) > .5
         .output (2) state
.endmachine
.end
",
    );
    assert!(error.contains("TYPO, which no element connects"), "{error}");
}

#[test]
fn outputs_are_zero_in_the_operating_point() {
    let deck = "* operating point
V1 x 0 1
R1 2 0 1K
R2 3 0 1K
.machine
.state A 3
.state B 5
.rule A B V(x) > .5
.output (2) state
.output (3) V(x)*2
.endmachine
.end
";
    let op = Engine::default()
        .run_dc_op(&parse(deck))
        .expect("operating point solves");
    for node in ["2", "3"] {
        let value = op.try_voltage_named(node).expect("node voltage");
        assert!(value.abs() < 1e-9, "V({node}) = {value}");
    }
}

#[test]
fn help_examples_switch_where_ltspice_does() {
    // The divide-by-two example of LTspice's help, with its reset.
    let deck = "* divide by 2
V1 1 0 pulse(0 1 0 1u 1u .5m 1m)
V2 c 0 pulse(0 1 0 1u 1u 5m 10m)
R1 2 0 1K
R2 3 0 1K
R3 4 0 1K
R4 5 0 1K
.machine
.state S0a 0
.state S0b 0
.state S1a 1
.state S1b 1
.rule S0a S0b V(1) < .5
.rule S0b S1a V(1) > .5
.rule S1a S1b V(1) < .5
.rule S1b S0a V(1) > .5
.rule * S0a V(c) > .5
.output (2) V(1) < .5
.output (3) V(1) > .5
.output (4) state
.output (5) state==S1a
.endmachine
.end
";
    let run = Run::new(deck, 30e-3, &["2", "4", "5"]);
    // LTspice 26.1.1 batch: the state output rises on the rising input
    // edges at 6, 8, 16, 18 ... ms and falls on those at 7, 9, 17 ... ms.
    let edges: Vec<f64> = [
        6.0, -7.0, 8.0, -9.0, 16.0, -17.0, 18.0, -19.0, 26.0, -27.0, 28.0, -29.0,
    ]
    .iter()
    .map(|ms: &f64| ms.signum() * (ms.abs() * 1e-3 + 0.5e-6))
    .collect();
    assert_crossings("state", &run.crossings("4", 0.5), &edges, 1e-9);
    assert_crossings("state==S1a", &run.crossings("5", 0.5), &edges, 1e-9);
    // An output reading a node follows it from one accepted point to the
    // next: the inverter is low while the input is high and high after.
    for (time, expected) in [
        (0.25e-3, 0.0),
        (0.75e-3, 1.0),
        (10.25e-3, 0.0),
        (10.75e-3, 1.0),
    ] {
        let value = run.at("2", time);
        assert!(
            (value - expected).abs() < 1e-9,
            "V(2) at {time} s = {value}"
        );
    }
}

#[test]
fn rules_fire_one_per_point_with_wildcards_first_from_time_zero() {
    let deck = "* rule order
V1 x 0 1
R1 2 0 1K
R2 4 0 1K
R3 5 0 1K
R4 6 0 1K
.machine
.state A 1
.state B 2
.state C 3
.state D 4
.rule A B V(x) > .5
.rule B C 1
.rule C D 0.5
.output (2) state
.endmachine
.machine
.state P 1
.state Q 2
.state R 3
.rule P Q 1
.rule * R V(x) > .5
.output (4) state
.endmachine
.machine
.state P 1
.state Q 2
.rule P Q 1
.rule * P V(x) > .5
.output (5) state
.endmachine
.machine
.state P 1
.state Q 2
.rule P P 1
.rule P Q 1
.output (6) state
.endmachine
.end
";
    let run = Run::new(deck, 1e-3, &["2", "4", "5", "6"]);
    // LTspice 26.1.1 batch: A moves to B on the operating point and one rule
    // fires per step after it, a condition of exactly 0.5 included; a `*`
    // rule is tried first and holds the machine even when it re-enters the
    // state it is in, as a rule into its own state does.
    for (node, expected) in [("2", 4.0), ("4", 3.0), ("5", 1.0), ("6", 1.0)] {
        let value = run.at(node, 1e-6);
        assert!((value - expected).abs() < 1e-9, "V({node}) = {value}");
    }
    let first = run.wave("2")[1];
    assert!(
        (first - 2.0).abs() < 1e-9,
        "V(2) after the first step = {first}"
    );
}

#[test]
fn a_counter_with_a_wildcard_reset_counts_where_ltspice_does() {
    let deck = "* 2-bit counter with a wildcard reset
Vclk clk 0 PULSE(0 1 1u 10n 10n 0.5u 1u)
Vrst rst 0 PWL(0 0 6.2u 0 6.21u 1 6.7u 1 6.71u 0)
R1 cnt 0 1k
R2 b0 0 1k
.machine
.state L0 0
.state H1 1
.state L1 1
.state H2 2
.state L2 2
.state H3 3
.state L3 3
.state H0 0
.rule * L0 V(rst) > .5
.rule L0 H1 V(clk) > .5
.rule H1 L1 V(clk) < .5
.rule L1 H2 V(clk) > .5
.rule H2 L2 V(clk) < .5
.rule L2 H3 V(clk) > .5
.rule H3 L3 V(clk) < .5
.rule L3 H0 V(clk) > .5
.rule H0 L0 V(clk) < .5
.output (cnt) state
.output (b0) state==1 | state==3
.endmachine
.end
";
    let run = Run::new(deck, 12e-6, &["cnt", "b0"]);
    // LTspice 26.1.1 batch: each count lands where the clock crosses 0.5
    // (n + 0.005 us), and the reset where its ramp does (6.205 us).
    let us = |edges: &[f64]| -> Vec<f64> { edges.iter().map(|t| t * 1e-6).collect() };
    let tolerance = 1e-12;
    assert_crossings(
        "cnt > 0.5",
        &run.crossings("cnt", 0.5),
        &us(&[1.005, -4.005, 5.005, -6.205, 7.005, -10.005, 11.005]),
        tolerance,
    );
    assert_crossings(
        "cnt > 1.5",
        &run.crossings("cnt", 1.5),
        &us(&[2.005, -4.005, 6.005, -6.205, 8.005, -10.005]),
        tolerance,
    );
    assert_crossings(
        "cnt > 2.5",
        &run.crossings("cnt", 2.5),
        &us(&[3.005, -4.005, 9.005, -10.005]),
        tolerance,
    );
    assert_crossings(
        "b0",
        &run.crossings("b0", 0.5),
        &us(&[
            1.005, -2.005, 3.005, -4.005, 5.005, -6.005, 7.005, -8.005, 9.005, -10.005, 11.005,
        ]),
        tolerance,
    );
}

#[test]
fn a_traffic_light_times_itself_on_the_capacitor_it_discharges() {
    let deck = "* traffic-light sequencer
I1 0 tmr PULSE(0 1u 0 1n)
Rt tmr 0 100Meg
C1 tmr 0 1n
Bd tmr 0 I=V(d)*V(tmr)*1m
Rl lamp 0 1k
Rd d 0 1k
.machine
.state R 0
.state RC 10
.state G 1
.state GC 11
.state Y 2
.state YC 12
.rule R RC V(tmr) > 3
.rule RC G V(tmr) < .1
.rule G GC V(tmr) > 2
.rule GC Y V(tmr) < .1
.rule Y YC V(tmr) > 1
.rule YC R V(tmr) < .1
.output (lamp) state - 10*(state >= 10)
.output (d) state >= 10
.endmachine
.end
";
    let run = Run::new(deck, 20e-3, &["lamp", "d"]);
    // LTspice 26.1.1 batch with a 20 ns maximum step: red until the timer
    // reaches 3 V, green until 2 V and amber until 1 V, each followed by a
    // discharge to 0.1 V. At its default step LTspice ends the first
    // discharge 49 ns early, against the 3.41 us the timer's exponential
    // takes; the edges drift apart by a few nanoseconds a phase.
    let ms = |edges: &[f64]| -> Vec<f64> { edges.iter().map(|t| t * 1e-3).collect() };
    assert_crossings(
        "discharge",
        &run.crossings("d", 0.5),
        &ms(&[
            3.045921251,
            -3.049332024,
            4.969552948,
            -4.972558108,
            5.877542312,
            -5.879853857,
            8.825725140,
            -8.829135915,
            10.749356970,
            -10.752362130,
            11.657346327,
            -11.659657875,
            14.605529423,
            -14.608940202,
            16.529161649,
            -16.532166819,
            17.437151725,
            -17.439463277,
        ]),
        20e-9,
    );
    assert_crossings(
        "amber",
        &run.crossings("lamp", 1.5),
        &ms(&[
            4.972558108,
            -5.879853856,
            10.752362130,
            -11.659657873,
            16.532166819,
            -17.439463272,
        ]),
        20e-9,
    );
}

/// The state output's crossings of a machine that goes high above 0.5 and
/// low below -0.5 of a 1 kHz sine, with the `.machine` card `machine`.
fn sine_trips(machine: &str) -> Vec<f64> {
    let deck = format!(
        "* tripdt
V1 1 0 sin(0 1 1k)
R1 2 0 1K
{machine}
.state lo 0
.state hi 1
.rule lo hi V(1) > .5
.rule hi lo V(1) < -.5
.output (2) state
.endmachine
.end
"
    );
    Run::new(&deck, 3e-3, &["2"]).crossings("2", 0.5)
}

#[test]
fn trips_are_located_to_within_tripdt() {
    // The sine crosses 0.5 at 1/12 ms and -0.5 at 7/12 ms of each period.
    let crossings: Vec<f64> = [1.0, -7.0, 13.0, -19.0, 25.0, -31.0]
        .iter()
        .map(|twelfths: &f64| twelfths.signum() * twelfths.abs() / 12.0 * 1e-3)
        .collect();
    // LTspice 26.1.1 batch: without tripdt the output switches within a few
    // femtoseconds of the crossing.
    assert_crossings(".machine", &sine_trips(".machine"), &crossings, 1e-9);
    // With tripdt the trip lands within tripdt past the crossing and the
    // output switches over the step after it, of at most tripdt: LTspice's
    // edges trail the crossing by 0.67 to 1.21 us at tripdt=1u and by 5 to
    // 9.5 ns at 10n.
    for (card, tripdt) in [(".machine tripdt=1u", 1e-6), (".machine 10n", 10e-9)] {
        let edges = sine_trips(card);
        assert_eq!(edges.len(), crossings.len(), "{card}: {edges:?}");
        for (edge, crossing) in edges.iter().zip(&crossings) {
            let lag = edge.abs() - crossing.abs();
            assert!(
                edge.signum() == crossing.signum() && (0.0..=1.5 * tripdt).contains(&lag),
                "{card}: edge {edge:e} s for the crossing at {crossing:e} s"
            );
        }
    }
}

#[test]
fn machines_in_subcircuits_read_their_instance_nodes_and_parameters() {
    let deck = "* toggles in subcircuits
V1 in 0 pulse(0 1 0 1u 1u .5m 1m)
X1 in o1 o2 tog vth=0.5
X2 in p1 p2 tog vth=0.8
.subckt tog a q qb vth=0.5
R1 q 0 1K
R2 qb 0 1K
.machine
.state lo 0
.state lo2 0
.state hi 1
.state hi2 1
.rule lo lo2 V(a) > vth
.rule lo2 hi V(a) < vth
.rule hi hi2 V(a) > vth
.rule hi2 lo V(a) < vth
.output (q) state
.output (qb) 1-state
.endmachine
.ends
.end
";
    let run = Run::new(deck, 5e-3, &["o1", "p1"]);
    // LTspice 26.1.1 batch: each toggles where the falling input crosses its
    // own threshold, 501.5 us into the period at 0.5 and 501.2 us at 0.8.
    let us = |start: f64| -> Vec<f64> {
        (0..5)
            .map(|period| {
                let at = (f64::from(period) * 1e3 + start) * 1e-6;
                if period % 2 == 0 { at } else { -at }
            })
            .collect()
    };
    assert_crossings("o1", &run.crossings("o1", 0.5), &us(501.5), 1e-9);
    assert_crossings("p1", &run.crossings("p1", 0.5), &us(501.2), 1e-9);
}

#[test]
fn rules_read_branch_currents() {
    let deck = "* overcurrent latches
Vs in 0 PWL(0 0 1m 2)
Rl in 0 1k
L1 in a 1m
Ra a 0 2k
R1 2 0 1K
R2 3 0 1K
.machine
.state ok 0
.state trip 1
.rule ok trip -I(Vs) > 1.5m
.output (2) state
.endmachine
.machine
.state ok 0
.state trip 1
.rule ok trip I(L1) > 0.4m
.output (3) state
.endmachine
.end
";
    let run = Run::new(deck, 1e-3, &["2", "3"]);
    // LTspice 26.1.1 batch: the source current reaches 1.5 mA at 500.167 us
    // and the inductor's 0.4 mA at 400.5 us, half a microsecond behind the
    // ramp.
    assert_crossings("I(Vs)", &run.crossings("2", 0.5), &[500.166667e-6], 5e-9);
    assert_crossings("I(L1)", &run.crossings("3", 0.5), &[400.500002e-6], 5e-9);
}

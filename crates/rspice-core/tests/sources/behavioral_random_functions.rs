//! `rand`, `random` and `white`: noise waveforms that are pure functions of
//! their argument and the deck's `.options seed`, identical in behavioral
//! sources and parameters. QSPICE implements only `random` and decides its
//! shape (linear between per-integer samples); `rand` (held samples) and
//! `white` (smoothsteps through samples less 0.5) follow LTspice 26.1.1.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::expr::{Function, seeded_random};
use rspice_core::netlist::Netlist;
use std::collections::HashMap;

const STEPS: usize = 8;
const SPAN: usize = 12;

fn operating_point(deck: &str) -> HashMap<String, f64> {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let op = Engine::new(SimulationConfig::default())
        .run_dc_op(&netlist)
        .expect("operating point solves");
    op.node_names
        .iter()
        .map(|name| name.to_ascii_lowercase())
        .zip(op.node_voltages.iter().copied())
        .collect()
}

/// One source per function and grid point `x = k/8` across `[0, SPAN]`.
fn sampling_deck(options: &str) -> String {
    let mut deck = String::from("* random function samples\n");
    deck.push_str(options);
    for k in 0..=SPAN * STEPS {
        let x = k as f64 / STEPS as f64;
        for name in ["rand", "random", "white"] {
            deck.push_str(&format!("B{name}{k} {name}{k} 0 V={name}({x})\n"));
        }
    }
    deck.push_str(".param held=rand(2.5) joined=random(2.5) smooth=white(2.5)\n");
    deck.push_str("Vheld held 0 {held}\nVjoined joined 0 {joined}\nVsmooth smooth 0 {smooth}\n");
    deck.push_str(".end\n");
    deck
}

fn sample(op: &HashMap<String, f64>, name: &str, k: usize) -> f64 {
    op[&format!("{name}{k}")]
}

#[test]
fn waveforms_hold_join_and_smooth_their_integer_samples() {
    let op = operating_point(&sampling_deck(""));
    for n in 0..SPAN {
        let start = sample(&op, "rand", n * STEPS);
        let end = sample(&op, "rand", (n + 1) * STEPS);
        assert!((0.0..1.0).contains(&start));
        for j in 0..STEPS {
            let k = n * STEPS + j;
            let f = j as f64 / STEPS as f64;
            let shape = f * f * (3.0 - 2.0 * f);
            assert_eq!(sample(&op, "rand", k), start, "rand is held at {k}/8");
            let random = start + (end - start) * f;
            assert!((sample(&op, "random", k) - random).abs() < 1e-12);
            let white = start - 0.5 + (end - start) * shape;
            assert!((sample(&op, "white", k) - white).abs() < 1e-12);
            assert!(sample(&op, "white", k).abs() <= 0.5);
        }
    }
    // Parameters draw exactly the samples behavioral sources do.
    for (node, function) in [("held", "rand"), ("joined", "random"), ("smooth", "white")] {
        assert_eq!(op[node], sample(&op, function, 2 * STEPS + STEPS / 2));
    }
}

#[test]
fn the_deck_seed_reseeds_every_evaluator_reproducibly() {
    let default = operating_point(&sampling_deck(""));
    let seeded = operating_point(&sampling_deck(".options seed=7\n"));
    assert_eq!(seeded, operating_point(&sampling_deck(".options seed=7\n")));
    let differing = (0..=SPAN * STEPS)
        .filter(|&k| sample(&default, "rand", k) != sample(&seeded, "rand", k))
        .count();
    assert!(
        differing > SPAN * STEPS / 2,
        "seed 7 left {differing} samples unchanged"
    );
    // A call that names its own seed ignores the deck's.
    let explicit = operating_point(
        "* explicit seed\n.options seed=7\nB1 a 0 V=rand(5.5,1)\nB2 b 0 V=rand(5.5)\n.end\n",
    );
    assert_eq!(explicit["a"], sample(&default, "rand", 5 * STEPS));
    assert_eq!(explicit["b"], sample(&seeded, "rand", 5 * STEPS));
}

#[test]
fn samples_are_uniform_and_independent() {
    let samples: Vec<f64> = (-20_000..20_000)
        .map(|k| seeded_random(Function::Rand, 1.0, k as f64).0)
        .collect();
    let n = samples.len() as f64;
    let mean = samples.iter().sum::<f64>() / n;
    let variance = samples.iter().map(|u| (u - mean).powi(2)).sum::<f64>() / n;
    let lag = samples
        .windows(2)
        .map(|pair| (pair[0] - mean) * (pair[1] - mean))
        .sum::<f64>()
        / (n * variance);
    assert!((mean - 0.5).abs() < 0.005, "mean {mean}");
    assert!((variance - 1.0 / 12.0).abs() < 0.002, "variance {variance}");
    assert!(lag.abs() < 0.02, "lag-one correlation {lag}");
    for bin in 0..10 {
        let count = samples
            .iter()
            .filter(|u| (**u * 10.0) as usize == bin)
            .count() as f64;
        assert!((count / n - 0.1).abs() < 0.006, "decile {bin}: {count}");
    }
}

#[test]
fn random_and_white_have_continuous_slopes_through_the_newton_jacobian() {
    // A source solved through its own argument: the Jacobian must carry the
    // interpolation slope for Newton to land on the fixed point.
    let deck = "* implicit noise
R1 x 0 1
B1 0 x I=2+random(V(x)*3)+white(V(x)*5)
.end
";
    let op = operating_point(deck);
    let x = op["x"];
    let residual = 2.0
        + seeded_random(Function::Random, 1.0, 3.0 * x).0
        + seeded_random(Function::White, 1.0, 5.0 * x).0
        - x;
    assert!(residual.abs() < 1e-9, "x {x}, residual {residual}");
}

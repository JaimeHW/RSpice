//! End-to-end tests for the statistical expression functions
//! (`gauss`/`agauss`/`unif`/`aunif`/2-arg `limit`) and `.options seed=`.
//!
//! These pin the contracts a Monte-Carlo / mismatch flow depends on:
//! every instance evaluation draws a distinct value from one netlist-wide
//! stream, the whole sequence is reproducible for a given seed, and the
//! seed option works no matter where it appears in the deck.

use rspice_core::netlist::{ElementKind, Netlist, flatten_netlist};

/// Collect flattened resistor values in netlist order.
fn flattened_resistor_values(netlist: &Netlist) -> Vec<f64> {
    flatten_netlist(netlist)
        .expect("flatten")
        .iter()
        .filter_map(|element| match &element.kind {
            ElementKind::Resistor { value, .. } => Some(*value),
            _ => None,
        })
        .collect()
}

#[test]
fn instance_param_draws_are_distinct_and_reproducible() {
    let deck = "\
* per-instance mismatch via agauss
.options seed=7
.subckt unit a b mult=1
r1 a b {1k*mult}
.ends
x1 1 0 unit mult={agauss(1,0.05,1)}
x2 2 0 unit mult={agauss(1,0.05,1)}
.end
";
    let first = flattened_resistor_values(&Netlist::parse(deck).expect("parse"));
    assert_eq!(first.len(), 2, "expected two flattened resistors");
    assert_ne!(
        first[0], first[1],
        "instances must draw distinct mismatch values"
    );
    for v in &first {
        assert!(
            (v - 1000.0).abs() < 500.0,
            "draw {v} implausibly far from nominal 1k (agauss sigma 50)"
        );
    }

    // Identical deck, identical seed: identical draws.
    let replay = flattened_resistor_values(&Netlist::parse(deck).expect("parse"));
    assert_eq!(first, replay, "same seed must reproduce the same draws");

    // Different seed: different draws.
    let reseeded_deck = deck.replace("seed=7", "seed=8");
    let reseeded = flattened_resistor_values(&Netlist::parse(&reseeded_deck).expect("parse"));
    assert_ne!(first, reseeded, "a different seed must change the draws");
}

#[test]
fn param_statements_share_one_seeded_stream() {
    let deck = "\
* params drawing from the shared stream
.options seed=3
.param a={agauss(0,1,1)}
.param b={agauss(0,1,1)}
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    let a = netlist.params.get("a").expect("param a");
    let b = netlist.params.get("b").expect("param b");
    assert_ne!(a, b, "consecutive .param draws must differ");

    let netlist2 = Netlist::parse(deck).expect("parse");
    assert_eq!(a, netlist2.params.get("a").unwrap());
    assert_eq!(b, netlist2.params.get("b").unwrap());
}

#[test]
fn seed_option_is_position_independent() {
    let seed_first = "\
* seed before params
.options seed=11
.param a={aunif(1,0.5)}
.end
";
    let seed_last = "\
* seed after params
.param a={aunif(1,0.5)}
.options seed=11
.end
";
    let first = Netlist::parse(seed_first).expect("parse");
    let last = Netlist::parse(seed_last).expect("parse");
    assert_eq!(
        first.params.get("a"),
        last.params.get("a"),
        ".options seed must apply regardless of its position in the deck"
    );
    assert_eq!(first.options.seed, Some(11));
    assert_eq!(last.options.seed, Some(11));
}

#[test]
fn seed_option_supports_continuation_and_rndseed_alias() {
    let deck = "\
* seed via continuation line
.options reltol=1e-4
+ rndseed = 21
.param a={aunif(0,1)}
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    assert_eq!(netlist.options.seed, Some(21));

    let explicit = "\
* same seed inline
.options rndseed=21
.param a={aunif(0,1)}
.end
";
    let other = Netlist::parse(explicit).expect("parse");
    assert_eq!(
        netlist.params.get("a"),
        other.params.get("a"),
        "continuation-line seed must behave like the inline form"
    );
}

#[test]
fn flattener_resolves_deferred_draws_per_instance() {
    // `nomm` is defined after the instances, so the instance expressions
    // cannot resolve at parse time and must be drawn by the flattener —
    // exercising the shared-stream handoff into hierarchy expansion.
    let deck = "\
* deferred draws resolved during flattening
.options seed=5
.subckt unit a b mult=1
r1 a b {1k*mult}
.ends
x1 1 0 unit mult={aunif(nomm,0.2)}
x2 2 0 unit mult={aunif(nomm,0.2)}
.param nomm=1
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    let values = flattened_resistor_values(&netlist);
    assert_eq!(values.len(), 2, "expected two flattened resistors");
    assert_ne!(
        values[0], values[1],
        "flatten-time draws must be distinct per instance"
    );
    for v in &values {
        assert!(
            (v - 1000.0).abs() <= 200.0 + 1e-9,
            "resistor {v} outside aunif(1,0.2)*1k bounds"
        );
    }

    // Full parse+flatten reproducibility.
    let replay = flattened_resistor_values(&Netlist::parse(deck).expect("parse"));
    assert_eq!(values, replay, "parse+flatten must be reproducible");
}

#[test]
fn invalid_seed_values_are_rejected() {
    let negative = "\
* bad seed
.options seed=-3
.end
";
    assert!(
        Netlist::parse(negative).is_err(),
        "negative seed must be rejected"
    );

    let fractional = "\
* bad seed
.options seed=1.5
.end
";
    assert!(
        Netlist::parse(fractional).is_err(),
        "fractional seed must be rejected"
    );
}

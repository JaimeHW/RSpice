//! The transistor-terminal activity bound places CMOS digital edges.
//!
//! Charge LTE alone lets a switching CMOS stage take steps whose terminal
//! voltages swing by volts, which smears edge timing down an inverter chain.
//! The 0.4 V activity bound (`DEVICE_ACTIVITY_STEP_BOUND`) is what keeps the
//! edges on time, whatever model the transistors use. Each chain here pins
//! both halves: with the bound its edges sit on a dense-step reference, and
//! with the bound lifted the same deck moves them, so a deck cannot pass while
//! the bound has silently stopped acting on its transistor family.

use rspice_core::engine::{Engine, SimulationConfig, TransientResult};
use rspice_core::netlist::Netlist;

/// An inverter chain behind one input pulse, and how to judge its edges.
struct Chain {
    deck: String,
    tstop: f64,
    max_step: f64,
    /// Dense ceiling for the reference run: tens of points per stage delay.
    reference_max_step: f64,
    stages: usize,
    mid_rail: f64,
}

/// Six level-1 CMOS inverters, each loaded with 20 fF.
fn level1_chain() -> Chain {
    let deck = "\
* six-inverter level-1 CMOS chain
.model nch nmos level=1 vto=0.7 kp=110u gamma=0.45 phi=0.7 lambda=0.03 tox=20n
+ cgso=0.25n cgdo=0.25n cgbo=0.2n cj=0.4m mj=0.5 cjsw=0.3n mjsw=0.33 pb=0.8
.model pch pmos level=1 vto=-0.8 kp=45u gamma=0.5 phi=0.7 lambda=0.04 tox=20n
+ cgso=0.25n cgdo=0.25n cgbo=0.2n cj=0.5m mj=0.5 cjsw=0.35n mjsw=0.33 pb=0.8
.subckt inv a z vdd vss
mp z a vdd vdd pch w=5u l=1u ad=10p as=10p pd=9u ps=9u
mn z a vss vss nch w=2u l=1u ad=4p as=4p pd=6u ps=6u
.ends inv
vdd vdd 0 dc 5.0
vin n0 0 pulse(0 5.0 5n 1n 1n 19n 40n)
x0 n0 n1 vdd 0 inv
c0 n1 0 20f
x1 n1 n2 vdd 0 inv
c1 n2 0 20f
x2 n2 n3 vdd 0 inv
c2 n3 0 20f
x3 n3 n4 vdd 0 inv
c3 n4 0 20f
x4 n4 n5 vdd 0 inv
c4 n5 0 20f
x5 n5 n6 vdd 0 inv
c5 n6 0 20f
.tran 4n 40n
.end
";
    Chain {
        deck: deck.to_string(),
        tstop: 40.0e-9,
        max_step: 4.0e-9,
        reference_max_step: 5.0e-12,
        stages: 6,
        mid_rail: 2.5,
    }
}

/// Six BSIM4 inverters on the PTM 45 nm high-performance cards, each loaded
/// with 5 fF.
fn bsim4_chain() -> Chain {
    let cards = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/iscas85/technology/45nm_HP.pm"),
    )
    .expect("the PTM 45 nm cards are in the repository");
    let deck = format!(
        "\
* six-inverter BSIM4 chain
{cards}
.subckt inv a z vdd vss
mp z a vdd vdd pmos l=0.12u w=1.0u as=0.265p ad=0.265p ps=2.53u pd=2.53u
mn z a vss vss nmos l=0.12u w=0.5u as=0.1325p ad=0.1325p ps=1.53u pd=1.53u
.ends inv
vdd vdd 0 dc 1.0
vin n0 0 pulse(0 1.0 0.25n 50p 50p 0.95n 2n)
x0 n0 n1 vdd 0 inv
c0 n1 0 5f
x1 n1 n2 vdd 0 inv
c1 n2 0 5f
x2 n2 n3 vdd 0 inv
c2 n3 0 5f
x3 n3 n4 vdd 0 inv
c3 n4 0 5f
x4 n4 n5 vdd 0 inv
c4 n5 0 5f
x5 n5 n6 vdd 0 inv
c5 n6 0 5f
.tran 0.2n 2n
.end
"
    );
    Chain {
        deck,
        tstop: 2.0e-9,
        max_step: 0.2e-9,
        reference_max_step: 1.0e-12,
        stages: 6,
        mid_rail: 0.5,
    }
}

fn run(chain: &Chain, activity_bound: Option<f64>, max_step: f64) -> TransientResult {
    let netlist = Netlist::parse(&chain.deck).expect("inverter chain deck parses");
    let mut config = rspice_core::engine::resolve_simulation_config(
        &SimulationConfig::default(),
        Some(&netlist.options),
        &Default::default(),
    );
    if let Some(bound) = activity_bound {
        config.transient_node_activity_bound = bound;
    }
    let engine = Engine::new(config);
    let result = engine
        .run_tran(&netlist, chain.tstop, max_step)
        .expect("the inverter chain transient completes");
    assert_eq!(engine.convergence_quality().force_accepted_points, 0);
    result
}

/// Linearly interpolated mid-rail crossings of `node`, in time order.
fn mid_rail_crossings(result: &TransientResult, node: &str, level: f64) -> Vec<f64> {
    let waveform = result
        .try_voltage_waveform_named(node)
        .unwrap_or_else(|| panic!("node {node} in {:?}", result.node_names));
    result
        .time
        .windows(2)
        .zip(waveform.windows(2))
        .filter(|(_, v)| (v[0] - level) * (v[1] - level) < 0.0)
        .map(|(t, v)| t[0] + (level - v[0]) * (t[1] - t[0]) / (v[1] - v[0]))
        .collect()
}

/// Largest edge-time difference from the reference over every stage output.
fn worst_edge_error(chain: &Chain, result: &TransientResult, reference: &TransientResult) -> f64 {
    let mut worst = 0.0_f64;
    for stage in 1..=chain.stages {
        let node = format!("n{stage}");
        let edges = mid_rail_crossings(result, &node, chain.mid_rail);
        let expected = mid_rail_crossings(reference, &node, chain.mid_rail);
        assert_eq!(
            edges.len(),
            2,
            "{node} should switch once on each input edge: {edges:?}"
        );
        assert_eq!(expected.len(), 2, "reference {node}: {expected:?}");
        for (edge, want) in edges.iter().zip(&expected) {
            worst = worst.max((edge - want).abs());
        }
    }
    worst
}

/// The bound holds `chain`'s edges within `bounded_limit` of the dense
/// reference, and lifting it moves an edge by at least `unbounded_floor`.
fn assert_bound_places_edges(chain: &Chain, bounded_limit: f64, unbounded_floor: f64) {
    let reference = run(chain, None, chain.reference_max_step);
    let bounded = run(chain, None, chain.max_step);
    let unbounded = run(chain, Some(1.0e30), chain.max_step);

    let bounded_error = worst_edge_error(chain, &bounded, &reference);
    assert!(
        bounded_error <= bounded_limit,
        "with the activity bound an edge sits {:.2} ps off the dense reference",
        bounded_error * 1e12
    );
    let unbounded_error = worst_edge_error(chain, &unbounded, &reference);
    assert!(
        unbounded_error >= unbounded_floor,
        "with the activity bound lifted the worst edge is only {:.2} ps off, so \
         this deck no longer shows what the bound buys",
        unbounded_error * 1e12
    );
    assert!(
        bounded.time.len() > unbounded.time.len(),
        "the bound must be what adds the edge timepoints: {} bounded vs {} unbounded",
        bounded.time.len(),
        unbounded.time.len()
    );
}

#[test]
fn activity_bound_keeps_inverter_chain_edges_on_the_dense_reference() {
    // One stage delay is about 120 ps on this chain.
    assert_bound_places_edges(&level1_chain(), 3.0e-12, 10.0e-12);
}

#[test]
fn activity_bound_keeps_bsim4_inverter_chain_edges_on_the_dense_reference() {
    // One stage delay is about 15 ps on this chain. The bound holds the worst
    // edge near 4.5 ps from the reference; lifted, it drifts to near 15 ps.
    // BSIM4 is a family of its own in the engine, so this fails if it ever
    // drops out of the bound.
    assert_bound_places_edges(&bsim4_chain(), 6.0e-12, 10.0e-12);
}

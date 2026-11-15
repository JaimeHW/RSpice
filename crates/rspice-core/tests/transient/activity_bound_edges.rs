//! The nonlinear-terminal activity bound places classic-MOS digital edges.
//!
//! Charge LTE alone lets a switching CMOS stage take steps whose terminal
//! voltages swing by volts, which smears edge timing down an inverter chain.
//! The 0.4 V activity bound (`DEVICE_ACTIVITY_STEP_BOUND`) is what keeps the
//! edges on time. This pins both halves: with the bound the chain's edges sit
//! on a dense-step reference, and with the bound lifted the same deck moves
//! them, so the deck cannot pass while the bound has silently stopped acting.

use rspice_core::engine::{Engine, SimulationConfig, TransientResult};
use rspice_core::netlist::Netlist;

/// Six level-1 CMOS inverters, each loaded with 20 fF, behind one input pulse.
const CHAIN: &str = "\
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

const TSTOP: f64 = 40.0e-9;
const MAX_STEP: f64 = 4.0e-9;
/// Dense ceiling for the reference run: about 25 points per stage delay.
const REFERENCE_MAX_STEP: f64 = 5.0e-12;
const STAGES: usize = 6;

fn run(activity_bound: Option<f64>, max_step: f64) -> TransientResult {
    let netlist = Netlist::parse(CHAIN).expect("inverter chain deck parses");
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
        .run_tran(&netlist, TSTOP, max_step)
        .expect("the inverter chain transient completes");
    assert_eq!(engine.convergence_quality().force_accepted_points, 0);
    result
}

/// Linearly interpolated mid-rail crossings of `node`, in time order.
fn mid_rail_crossings(result: &TransientResult, node: &str) -> Vec<f64> {
    let waveform = result
        .try_voltage_waveform_named(node)
        .unwrap_or_else(|| panic!("node {node} in {:?}", result.node_names));
    let level = 2.5;
    result
        .time
        .windows(2)
        .zip(waveform.windows(2))
        .filter(|(_, v)| (v[0] - level) * (v[1] - level) < 0.0)
        .map(|(t, v)| t[0] + (level - v[0]) * (t[1] - t[0]) / (v[1] - v[0]))
        .collect()
}

/// Largest edge-time difference from the reference over every stage output.
fn worst_edge_error(result: &TransientResult, reference: &TransientResult) -> f64 {
    let mut worst = 0.0_f64;
    for stage in 1..=STAGES {
        let node = format!("n{stage}");
        let edges = mid_rail_crossings(result, &node);
        let expected = mid_rail_crossings(reference, &node);
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

#[test]
fn activity_bound_keeps_inverter_chain_edges_on_the_dense_reference() {
    let reference = run(None, REFERENCE_MAX_STEP);
    let bounded = run(None, MAX_STEP);
    let unbounded = run(Some(1.0e30), MAX_STEP);

    // One stage delay is about 120 ps on this chain.
    let bounded_error = worst_edge_error(&bounded, &reference);
    assert!(
        bounded_error <= 3.0e-12,
        "with the activity bound an edge sits {:.2} ps off the dense reference",
        bounded_error * 1e12
    );
    let unbounded_error = worst_edge_error(&unbounded, &reference);
    assert!(
        unbounded_error >= 10.0e-12,
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

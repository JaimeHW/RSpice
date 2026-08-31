//! Bit-reproducibility gates: identical inputs must produce bit-identical
//! results run over run, with parallel features enabled. This is the
//! release determinism policy (no timing-dependent numerics, ever)
//! pinned as a test.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

const DECK: &str = "\
* determinism bench: nonlinear + reactive + controlled
v1 in 0 sin(0 2 1meg)
r1 in a 1k
d1 a 0 dmod
q1 c b 0 qmod
rb a b 47k
rc in c 2k
c1 a 0 100p
l1 c 0 10u
e1 out 0 poly(1) a 0 0 0.5 0.1
rl out 0 1k
.model dmod D IS=1e-14 N=1.6 RS=20
.model qmod NPN (IS=1e-15 BF=120)
.tran 1n 2u
.end
";

fn run_transient() -> (Vec<f64>, Vec<Vec<f64>>) {
    let netlist = Netlist::parse(DECK).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_tran(&netlist, 2e-6, 1e-9)
        .expect("transient runs");
    (result.time.clone(), result.voltages.clone())
}

#[test]
fn transient_is_bit_identical_across_runs() {
    let (t1, v1) = run_transient();
    let (t2, v2) = run_transient();

    assert_eq!(t1.len(), t2.len(), "identical step counts");
    assert!(
        t1.iter().zip(&t2).all(|(a, b)| a.to_bits() == b.to_bits()),
        "time grids must be bit-identical"
    );
    for (w1, w2) in v1.iter().zip(&v2) {
        assert!(
            w1.iter().zip(w2).all(|(a, b)| a.to_bits() == b.to_bits()),
            "waveforms must be bit-identical"
        );
    }
}

#[test]
fn operating_point_is_bit_identical_across_runs() {
    let netlist = Netlist::parse(DECK).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let a = engine.run_dc_op(&netlist).expect("op solves");
    let b = engine.run_dc_op(&netlist).expect("op solves");
    assert!(
        a.node_voltages
            .iter()
            .zip(&b.node_voltages)
            .all(|(x, y)| x.to_bits() == y.to_bits()),
        "operating points must be bit-identical"
    );
}

/// All-NAND ripple adder: four full-adder stages of nine gates each, driven
/// through an `adc_bridge` and read back through a `dac_bridge`.
///
/// Gate-dominated on purpose. The settle loop dispatches these gates from the
/// event fan-out rather than by walking every instance, so the run's output now
/// depends on which instances a delta cycle chose to evaluate and in what
/// order. A deck of two-input NANDs with a shared carry chain is where a
/// dispatch that lost an instance, or visited one out of registration order,
/// would show up first.
const GATE_DECK: &str = "\
* determinism bench: 36-gate all-NAND ripple adder
.model nandmod d_nand(rise_delay=0.7e-9 fall_delay=0.7e-9 input_load=0.5e-12)
.model adcmod adc_bridge(in_low=1 in_high=2)
.model dacmod dac_bridge(out_low=0 out_high=3 out_undef=1.5 t_rise=0.5e-9 t_fall=0.5e-9)
.subckt nand in1 in2 out
a1 [in1 in2] out nandmod
.ends nand
.subckt onebit 1 2 3 4 5
x1   1  2  7   nand
x2   1  7  8   nand
x3   2  7  9   nand
x4   8  9 10   nand
x5   3 10 11   nand
x6   3 11 12   nand
x7  10 11 13   nand
x8  12 13  4   nand
x9  11  7  5   nand
.ends onebit
va0 na0 0 dc 0 pulse(0 3 0 0.4ns 0.4ns 5ns 12ns)
va1 na1 0 dc 0 pulse(0 3 0 0.4ns 0.4ns 9ns 20ns)
va2 na2 0 dc 0 pulse(0 3 0 0.4ns 0.4ns 3ns 8ns)
va3 na3 0 dc 0 pulse(0 3 0 0.4ns 0.4ns 7ns 30ns)
vb0 nb0 0 dc 0 pulse(0 3 0 0.4ns 0.4ns 11ns 26ns)
vb1 nb1 0 dc 0 pulse(0 3 0 0.4ns 0.4ns 4ns 14ns)
vb2 nb2 0 dc 0 pulse(0 3 0 0.4ns 0.4ns 13ns 34ns)
vb3 nb3 0 dc 0 pulse(0 3 0 0.4ns 0.4ns 6ns 18ns)
vz nz 0 dc 0
aadc [na0 na1 na2 na3 nb0 nb1 nb2 nb3 nz] [a0 a1 a2 a3 b0 b1 b2 b3 cin] adcmod
x0 a0 b0 cin s0 c1 onebit
x1 a1 b1 c1  s1 c2 onebit
x2 a2 b2 c2  s2 c3 onebit
x3 a3 b3 c3  s3 c4 onebit
adac [s0 s3 c4] [o0 o3 o4] dacmod
r0 o0 0 1k
r3 o3 0 1k
r4 o4 0 1k
.end
";

fn run_gate_transient() -> (Vec<f64>, Vec<Vec<f64>>) {
    let netlist = Netlist::parse(GATE_DECK).expect("gate deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_tran(&netlist, 20e-9, 100e-12)
        .expect("gate transient runs");
    (result.time.clone(), result.voltages.clone())
}

#[test]
fn gate_level_transient_is_bit_identical_across_runs() {
    let (t1, v1) = run_gate_transient();
    let (t2, v2) = run_gate_transient();

    assert!(
        t1.len() > 100,
        "expected a populated time grid, got {}",
        t1.len()
    );
    assert_eq!(t1.len(), t2.len(), "identical step counts");
    assert!(
        t1.iter().zip(&t2).all(|(a, b)| a.to_bits() == b.to_bits()),
        "gate-level time grids must be bit-identical"
    );
    for (w1, w2) in v1.iter().zip(&v2) {
        assert!(
            w1.iter().zip(w2).all(|(a, b)| a.to_bits() == b.to_bits()),
            "gate-level waveforms must be bit-identical"
        );
    }
}

#[test]
fn ac_sweep_is_bit_identical_across_runs() {
    let netlist = Netlist::parse(
        "\
* ac determinism
v1 in 0 dc 0 ac 1
r1 in out 1k
c1 out 0 1n
.ac dec 40 1 1g
.end
",
    )
    .expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());

    let freqs: Vec<f64> = (0..=360).map(|i| 10f64.powf(i as f64 / 40.0)).collect();
    let a = engine.run_ac(&netlist, &freqs).expect("ac runs");
    let b = engine.run_ac(&netlist, &freqs).expect("ac runs");
    assert_eq!(a.len(), b.len());
    for (ra, rb) in a.iter().zip(&b) {
        for (va, vb) in ra.voltages.iter().zip(&rb.voltages) {
            assert!(
                va.re.to_bits() == vb.re.to_bits() && va.im.to_bits() == vb.im.to_bits(),
                "parallel AC sweep must be bit-identical"
            );
        }
    }
}

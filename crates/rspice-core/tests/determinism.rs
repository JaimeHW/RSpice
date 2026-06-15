//! Bit-reproducibility gates: identical inputs must produce bit-identical
//! results run over run, with parallel features enabled. This is the
//! commercial-qualification policy (no timing-dependent numerics, ever)
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

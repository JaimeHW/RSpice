//! LTspice and QSPICE deck syntax against LTspice and QSPICE.
//!
//! Reference values are LTspice 26.1.1 and QSPICE64 batch runs of the same
//! decks. Where the two disagree QSPICE decides; where QSPICE cannot read a
//! construct, LTspice does.

use rspice_core::engine::Engine;
use rspice_core::netlist::Netlist;

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}\n{deck}"))
}

fn assert_close(label: &str, actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{label}: {actual:.9e} vs reference {expected:.9e} (tolerance {tolerance:e})"
    );
}

fn op_voltage(netlist: &Netlist, node: &str) -> f64 {
    Engine::default()
        .run_dc_op(netlist)
        .expect("operating point solves")
        .try_voltage_named(node)
        .unwrap_or_else(|| panic!("no voltage for {node}"))
}

/// Node voltages at `time`, interpolated between the transient's points.
fn tran_voltages(netlist: &Netlist, stop: f64, time: f64, nodes: &[&str]) -> Vec<f64> {
    let tran = Engine::default()
        .run_tran(netlist, stop, stop / 2000.0)
        .expect("transient solves");
    let index = tran.time.partition_point(|t| *t < time).max(1);
    let (t0, t1) = (tran.time[index - 1], tran.time[index]);
    let fraction = if t1 > t0 {
        (time - t0) / (t1 - t0)
    } else {
        0.0
    };
    nodes
        .iter()
        .map(|node| {
            let wave = tran
                .try_voltage_waveform_named(node)
                .unwrap_or_else(|| panic!("no waveform for {node}"));
            wave[index - 1] + fraction * (wave[index] - wave[index - 1])
        })
        .collect()
}

// LTspice: V(b)=0.632120 at 1 ms, V(c)=0.367880 (I(L1)=0.632120 mA), V(e)=3,
// V(bb)=10. QSPICE: 0.632120, 0.632120 mA, 3, 10.
#[test]
fn elements_read_parameters_declared_after_them() {
    let deck = "* forward parameters
V1 a 0 PULSE(0 1 0 1n 1n 1 2)
R1 a b 1k
C1 b 0 {Cx}
R2 a c 1k
L1 c 0 {Lx}
E1 e 0 a 0 {gain}
V2 v 0 {vdc}
B1 bb 0 V={kk*V(v)}
X1 a d rc rr={rv}
.subckt rc in out rr=1
R1 in out {rr}
C1 out 0 {cl}
.param cl=1u
.ends
.param rv={2*half}
.tran 0 2m 0 1u
.param Cx=1u Lx=1 gain=3 vdc=2 kk=5
.param half=500
.end
";
    let values = tran_voltages(&parse(deck), 2e-3, 1e-3, &["b", "c", "e", "bb", "d"]);
    assert_close("V(b)", values[0], 0.632120, 2e-4);
    assert_close("V(c)", values[1], 0.367880, 2e-4);
    assert_close("V(e)", values[2], 3.0, 1e-9);
    assert_close("V(bb)", values[3], 10.0, 1e-9);
    assert_close("V(d)", values[4], 0.632120, 2e-4);
}

// LTspice and QSPICE: V(g)=2, V(h)=-0.5.
#[test]
fn controlled_source_gains_read_parameters_declared_after_them() {
    let netlist = parse(
        "* forward gains
V1 a 0 1
R1 a 0 1k
G1 0 g a 0 {gm}
R2 g 0 1k
H1 h 0 V1 {rt}
R3 h 0 1k
.op
.param gm=2m rt=500
.end
",
    );
    assert_close("V(g)", op_voltage(&netlist, "g"), 2.0, 1e-9);
    assert_close("V(h)", op_voltage(&netlist, "h"), -0.5, 1e-9);
}

// LTspice and QSPICE: V(b)=0.632120 at 1 ms.
#[test]
fn an_element_reads_a_parameter_an_include_after_it_declares() {
    let directory =
        std::env::temp_dir().join(format!("rspice-forward-include-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    std::fs::write(directory.join("values.lib"), ".param Cx=1u\n").unwrap();
    let deck = directory.join("deck.cir");
    std::fs::write(
        &deck,
        "* forward include\nV1 a 0 PULSE(0 1 0 1n 1n 1 2)\nR1 a b 1k\nC1 b 0 {Cx}\n\
         .tran 0 2m 0 1u\n.inc values.lib\n.end\n",
    )
    .unwrap();
    let netlist = Netlist::parse_file(&deck).expect("deck parses");
    std::fs::remove_dir_all(&directory).ok();
    let values = tran_voltages(&netlist, 2e-3, 1e-3, &["b"]);
    assert_close("V(b)", values[0], 0.632120, 2e-4);
}

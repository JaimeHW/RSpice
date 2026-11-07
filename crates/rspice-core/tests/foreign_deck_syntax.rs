//! LTspice and QSPICE deck syntax against LTspice and QSPICE.
//!
//! Reference values are LTspice 26.1.1 and QSPICE64 batch runs of the same
//! decks. Where the two disagree QSPICE decides; where QSPICE cannot read a
//! construct, LTspice does.

use rspice_core::analysis::measure::MeasureResult;
use rspice_core::analysis::measure_signals::{
    evaluate_ac_measurements, evaluate_tran_measurements,
};
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

// LTspice: V(b)=V(b2)=0.643766; QSPICE: V(b)=V(b2)=0.643765. Both read
// `type=` and `mfg=` as annotations that leave the diode unchanged.
#[test]
fn part_annotations_on_a_model_card_leave_the_device_unchanged() {
    let netlist = parse(
        "* model annotations
V1 a 0 1
R1 a b 100
D1 b 0 dsi
R2 a b2 100
D2 b2 0 dref
R3 a b3 100
D3 b3 0 dsk
.model dsi D(Is=2.52n Rs=.568 N=1.752 Cjo=4p M=.4 tt=20n Iave=200m Vpk=75 mfg=OnSemi type=silicon)
.model dref D(Is=2.52n Rs=.568 N=1.752 Cjo=4p M=.4 tt=20n)
.model dsk D(Is=2.52n Rs=.568 N=1.752 vendor=Acme type=schottky pn=1N914)
.op
.end
",
    );
    let annotated = netlist
        .models
        .iter()
        .find(|model| model.name.eq_ignore_ascii_case("dsi"))
        .expect("model dsi");
    assert!(
        annotated
            .string_params
            .iter()
            .any(|(name, value)| name.eq_ignore_ascii_case("TYPE")
                && value.eq_ignore_ascii_case("silicon")),
        "{:?}",
        annotated.string_params
    );
    let reference = op_voltage(&netlist, "b2");
    assert_close("V(b2)", reference, 0.643765, 2e-6);
    assert_close("V(b)", op_voltage(&netlist, "b"), reference, 1e-12);
    assert_close("V(b3)", op_voltage(&netlist, "b3"), reference, 1e-12);
}

// LTspice: V(b)=0.632120 at 1 ms, V(c)=9.99900e-5; QSPICE: 0.632120, 9.99900e-5.
#[test]
fn part_annotations_on_passive_instances_are_set_aside() {
    let netlist = parse(
        "* instance annotations
V1 a 0 PULSE(0 1 0 1n 1n 1 2)
R1 a b 1k
C1 b 0 1u V=6.3 Irms=0 Rser=0.01 Lser=0 mfg=\"KEMET\" pn=\"C1206C105K4RAC\" type=\"X7R\"
C2 b 0 1p mfg=Rubycon pn=10SWZ47M type=\"Al electrolytic\"
R2 a c 1k
L1 c 0 1m Ipk=1 Rser=0.1 Rpar=0 Cpar=0 mfg=\"Coilcraft\" pn=\"XAL4020-102\"
R3 a 0 1k mfg=Vishay pn=CRCW08051K00
.tran 0 2m 0 1u
.end
",
    );
    let values = tran_voltages(&netlist, 2e-3, 1e-3, &["b", "c"]);
    assert_close("V(b)", values[0], 0.632120, 2e-4);
    assert_close("V(c)", values[1], 9.999e-5, 1e-8);
}

// LTspice 26.1.1 decides: QSPICE ignores VTO and DIR on a G source ("Ignoring
// unknown instance parameter"). LTspice: 0.01089, -0.71289, 0, 0.025, 0.07,
// 0.06889 and 0.0050625.
#[test]
fn a_g_source_with_vto_and_dir_is_a_one_sided_square_law() {
    let netlist = parse(
        "* G square law
V1 c1 0 1
G1 0 o1 c1 0 100m vto=670m dir=1
R1 o1 0 1
V2 c2 0 -2
G2 0 o2 c2 0 100m vto=670m dir=-1
R2 o2 0 1
V3 c3 0 0.5
G3 0 o3 c3 0 100m vto=670m dir=1
R3 o3 0 1
V4 c4 0 0.5
G4 0 o4 c4 0 100m dir=1
R4 o4 0 1
V5 c5 0 0.7
G5 0 o5 c5 0 100m vto=670m dir=0.5
R5 o5 0 1
V6 c6 0 1.5
G6 0 o6 c6 0 100m dir=2 vto=670m
R6 o6 0 1
X1 c6 o7 sq
R7 o7 0 1
.subckt sq c o
G1 0 o c 0 1m vto=-.75 dir=1
.ends
.op
.end
",
    );
    let op = Engine::default()
        .run_dc_op(&netlist)
        .expect("operating point solves");
    for (node, expected) in [
        ("o1", 0.01089),
        ("o2", -0.71289),
        ("o3", 0.0),
        ("o4", 0.025),
        ("o5", 0.07),
        ("o6", 0.06889),
        ("o7", 0.0050625),
    ] {
        let value = op.try_voltage_named(node).expect("node voltage");
        assert_close(&format!("V({node})"), value, expected, 1e-9);
    }
}

// QSPICE: V(b)=0.264241 and V(c)=0.323323 at 1 ms, the exact step responses
// 1-(1+t/T)e^(-t/T) and 1-(1+u+u²/2)e^(-u). LTspice's 0.264592 and 0.322882
// differ by its frequency-domain realization; QSPICE decides.
#[test]
fn a_laplace_power_written_with_a_double_star_is_a_power() {
    let netlist = parse(
        "* laplace double star
V1 in 0 PULSE(0 1 0 1n 1n 1 2)
E2 b 0 in 0 Laplace=1/(1+.001*s)**2
E3 c 0 in 0 Laplace=1./(1+.0005*s)**3
.tran 0 5m 0 1u
.end
",
    );
    let values = tran_voltages(&netlist, 5e-3, 1e-3, &["b", "c"]);
    assert_close("V(b)", values[0], 0.264241, 2e-4);
    assert_close("V(c)", values[1], 0.323323, 2e-4);
}

// QSPICE and LTspice: V(o1)=1000 and V(o2)=3.
#[test]
fn instance_parameters_the_subcircuit_does_not_declare_reach_its_body() {
    let netlist = parse(
        "* undeclared instance parameters
V1 in 0 1
X1 in 0 o1 amp Aol=1k
R1 o1 0 1k
X2 in 0 o2 amp2 G=3 Unused=7
R2 o2 0 1k
.subckt amp 1 2 3
E1 3 0 1 2 {Aol}
.ends amp
.subckt amp2 1 2 3 G=1
E1 3 0 1 2 {G}
.ends amp2
.op
.end
",
    );
    assert_close("V(o1)", op_voltage(&netlist, "o1"), 1000.0, 1e-9);
    assert_close("V(o2)", op_voltage(&netlist, "o2"), 3.0, 1e-9);
}

fn measurement(results: &[MeasureResult], name: &str) -> f64 {
    let result = results
        .iter()
        .find(|result| result.name.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| panic!("no measurement {name} in {results:?}"));
    result
        .value
        .unwrap_or_else(|| panic!("{name} failed: {:?}", result.error))
}

// QSPICE (QPOST) decides; LTspice has no Phase(): tmp=0.99998,
// gain=0.998032, ft=2760.03 Hz, ph=-59.9545 deg, gdb=-1.44507,
// gre=0.716957, gim=-0.450477, f3=1587.78 Hz.
#[test]
fn ac_measurements_take_parts_of_a_complex_probe_with_functions() {
    let netlist = parse(
        "* meas ac complex functions
V1 in 0 AC 1
R1 in out 1k
C1 out 0 100n
.ac dec 20 10 1meg
.MEAS AC tmp max mag(V(out))
.MEAS AC GAIN FIND mag(V(out)) AT 100
.MEAS AC FT when mag(V(out)) = 0.5
.MEAS AC ph find Phase(V(out)) at FT
.MEAS AC gdb FIND db(V(out)) AT 1k
.MEAS AC gre FIND re(V(out)) AT 1k
.MEAS AC gim FIND im(V(out)) AT 1k
.MEAS AC f3 when db(V(out)) = -3
.end
",
    );
    let frequencies = (0..=100)
        .map(|k| 10.0 * 10f64.powf(f64::from(k) / 20.0))
        .collect::<Vec<_>>();
    let sweep = Engine::default()
        .run_ac(&netlist, &frequencies)
        .expect("AC sweep solves");
    let results = evaluate_ac_measurements(&netlist, &sweep);
    assert_close("tmp", measurement(&results, "tmp"), 0.99998, 1e-5);
    assert_close("gain", measurement(&results, "gain"), 0.998032, 1e-6);
    assert_close("ft", measurement(&results, "ft"), 2760.03, 5.0);
    assert_close("ph", measurement(&results, "ph"), -59.9545, 0.2);
    assert_close("gdb", measurement(&results, "gdb"), -1.44507, 1e-4);
    assert_close("gre", measurement(&results, "gre"), 0.716957, 1e-5);
    assert_close("gim", measurement(&results, "gim"), -0.450477, 1e-5);
    assert_close("f3", measurement(&results, "f3"), 1587.78, 5.0);
}

// QSPICE (QPOST) decides: t1=489.531 us, vin=0.0657325, slope=-6269.56 V/s;
// LTspice: 489.531 us, 0.0657331, -6270.30 V/s.
#[test]
fn a_measurement_finds_a_value_at_another_measurements_result() {
    let netlist = parse(
        "* meas at another measurement
V1 in 0 SIN(0 1 1k) AC 1
R1 in out 1k
C1 out 0 100n
.tran 0 2m 0 1u
.MEAS TRAN t1 WHEN V(out)=0.5 FALL=1
.MEAS TRAN vin FIND V(in) AT t1
.MEAS TRAN slope DERIV V(in) AT=t1
.end
",
    );
    let tran = Engine::default()
        .run_tran(&netlist, 2e-3, 1e-6)
        .expect("transient solves");
    let results = evaluate_tran_measurements(&netlist, &tran);
    assert_close("t1", measurement(&results, "t1"), 489.531e-6, 2e-8);
    assert_close("vin", measurement(&results, "vin"), 0.0657325, 1e-4);
    assert_close("slope", measurement(&results, "slope"), -6269.56, 2.0);
}

// LTspice and QSPICE: V(o1)=-10.75 and V(o2)=-1.47.
#[test]
fn a_probe_reads_a_node_whose_name_starts_with_a_sign() {
    let netlist = parse(
        "* probes of +V and -V
V1 +V 0 DC 15
V2 -V 0 DC -15
V3 in 0 DC 2
X1 in o1 +V -V clampy
R1 o1 0 1k
B2 o2 0 V=V(in,+V)*0.1+V(-V,in)*0.01
R2 o2 0 1k
.subckt clampy a o p n
B1 o 0 V=V(a,p)*0.5+V(n,a)*0.25
.ends clampy
.op
.end
",
    );
    assert_close("V(o1)", op_voltage(&netlist, "o1"), -10.75, 1e-9);
    assert_close("V(o2)", op_voltage(&netlist, "o2"), -1.47, 1e-9);
}

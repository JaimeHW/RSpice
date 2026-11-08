//! LTspice and QSPICE deck syntax against LTspice and QSPICE.
//!
//! Reference values are LTspice 26.1.1 and QSPICE64 batch runs of the same
//! decks. Where the two disagree QSPICE decides; where QSPICE cannot read a
//! construct, LTspice does.

use rspice_core::abort_signal::NoAbort;
use rspice_core::analysis::evaluate_op_measurements_with_abort;
use rspice_core::analysis::measure::MeasureResult;
use rspice_core::analysis::measure_signals::{
    evaluate_ac_measurements, evaluate_tran_measurements,
};
use rspice_core::engine::{Engine, SimulationConfig, SpiceDialect};
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

// QSPICE (QPOST) decides: vout=3.52533, iR1=1.47467 mA, pw=5.1987 mW,
// vd=0.529717; LTspice: 3.525329, 1.474671 mA, 5.198700 mW, 0.529718.
#[test]
fn measurements_read_the_operating_point() {
    let netlist = parse(
        "* meas on the operating point
V1 in 0 5
R1 in out 1k
R2 out 0 3k
D1 out d dmod
R3 d 0 10k
.model dmod D(IS=2.52n RS=0.568 N=1.752)
.op
.meas op vout find V(out)
.meas op iR1 find I(R1)
.meas op pw param vout*iR1
.meas op vd find V(out)-V(d)
.end
",
    );
    let operating_point = Engine::default()
        .run_dc_op(&netlist)
        .expect("operating point solves");
    let results = evaluate_op_measurements_with_abort(&netlist, &operating_point, &NoAbort)
        .expect("measurements evaluate");
    assert_close("vout", measurement(&results, "vout"), 3.52533, 2e-5);
    assert_close("iR1", measurement(&results, "iR1"), 1.47467e-3, 2e-8);
    assert_close("pw", measurement(&results, "pw"), 5.1987e-3, 2e-7);
    assert_close("vd", measurement(&results, "vd"), 0.529717, 2e-5);
}

/// `uplim` and `dnlim` at operating points across the knee, by QSPICE's law.
// QSPICE: uplim(V(in),0.5,1) and dnlim(V(in),0.5,1) at in = -2, -0.25, 0, 1,
// 2.5 and -0.499. LTspice bends toward the limit exponentially instead
// (uplim(-0.25,...) = -0.278801); QSPICE decides. Neither accepts the
// functions in a .param.
#[test]
fn soft_limits_bend_toward_their_limit_past_the_knee() {
    let levels = |input: f64| {
        let netlist = parse(&format!(
            "* soft limits
V1 in 0 {input}
B1 dn 0 V=dnlim(V(in),0.5,1)
B2 up 0 V=uplim(V(in),0.5,1)
R1 dn 0 1k
R2 up 0 1k
.end
"
        ));
        (op_voltage(&netlist, "up"), op_voltage(&netlist, "dn"))
    };
    for (input, up, dn) in [
        (-2.0, -2.0, 0.722_222_2),
        (-0.25, -0.3, 0.863_636_4),
        (0.0, -0.166_666_7, 0.9),
        (1.0, 0.1, 1.166_666_7),
        (2.5, 0.25, 2.5),
        (-0.499, -0.499_001_0, 0.833_444_5),
    ] {
        let (actual_up, actual_dn) = levels(input);
        assert_close(&format!("uplim at {input}"), actual_up, up, 1e-6);
        assert_close(&format!("dnlim at {input}"), actual_dn, dn, 1e-6);
    }
    // Value and slope are continuous where each limit leaves the identity.
    let step = 1e-6;
    for (knee, read) in [(-0.5, 0usize), (1.5, 1usize)] {
        let side = |input: f64| {
            let (up, dn) = levels(input);
            [up, dn][read]
        };
        let below = (side(knee) - side(knee - step)) / step;
        let above = (side(knee + step) - side(knee)) / step;
        assert_close(&format!("value at the knee {knee}"), side(knee), knee, 1e-9);
        assert_close(&format!("slope across the knee {knee}"), above, below, 1e-4);
    }
}

// QSPICE: V(cc)=4.432971, V(d)=3.858144. LTspice: 4.432987, 3.858142.
#[test]
fn a_model_card_ends_a_parameter_list_its_line_leaves_open() {
    let netlist = parse(
        "* unclosed model parameter list
V1 c 0 5
R1 c cc 1k
VB b 0 0.7
Q1 cc b 0 QX
.model QX NPN(BF=50 IS=1e-15
D1 c d DX
R2 d 0 1k
.model DX D(IS=1e-12 N=2
.end
",
    );
    assert_close("V(cc)", op_voltage(&netlist, "cc"), 4.432971, 2e-5);
    assert_close("V(d)", op_voltage(&netlist, "d"), 3.858144, 2e-5);
}

/// A switch with a negative hysteresis turns on smoothly across the band.
// QSPICE and LTspice agree: across Vt -/+ |Vh| the log conductance follows
// 3u^2 - 2u^3. V(a) = 1 mA / g for the voltage switch at Vc = 0.15, 0.3,
// 0.5, 0.7, 0.85; the same law for the current switch at Ic = 0.15, 0.45,
// 0.75.
#[test]
fn a_negative_hysteresis_switch_turns_on_smoothly() {
    let voltage = |control: f64| {
        op_voltage(
            &parse(&format!(
                "* smooth voltage switch
V1 c 0 {control}
I2 0 a 1m
S1 a 0 c 0 SX
.model SX SW(Ron=1 Roff=1Meg Vt=.5 Vh=-.4)
.end
"
            )),
            "a",
        )
    };
    for (control, expected) in [
        (0.15, 856.282_7),
        (0.3, 115.478_2),
        (0.5, 1.0),
        (0.7, 8.659_643e-3),
        (0.85, 1.167_839e-3),
    ] {
        assert_close(
            &format!("S1 at {control} V"),
            voltage(control),
            expected,
            2e-6 * expected,
        );
    }
    let current = |control: f64| {
        op_voltage(
            &parse(&format!(
                "* smooth current switch
I1 0 c {control}
VC c 0 0
I2 0 a 1m
W1 a 0 VC WX
.model WX CSW(Ron=1 Roff=1Meg It=.5 Ih=-.4)
.end
"
            )),
            "a",
        )
    };
    for (control, expected) in [(0.15, 856.282_7), (0.45, 3.627_190), (0.75, 3.578_582e-3)] {
        assert_close(
            &format!("W1 at {control} A"),
            current(control),
            expected,
            2e-6 * expected,
        );
    }
}

/// `noiseless` on a model card quiets every device built from it.
// QSPICE and LTspice at 1 kHz: with the flag only R1 reaches V(b),
// 2.906043e-28 V^2/Hz; without it Q1 adds 5.416252e-27 and S1 2.906043e-28.
#[test]
fn a_noiseless_model_card_quiets_its_devices() {
    let deck = |flag: &str| {
        parse(&format!(
            "* noiseless model cards
V1 a 0 1 AC 1
R1 a b 1k
D1 b 0 DX
.model DX D(Ron=100 Vfwd=0.5 {flag})
S1 b 0 a 0 SX
.model SX SW(Ron=1k Roff=1Meg Vt=0.5 Vh=0.1 {flag})
Q1 b a 0 QX
.model QX NPN(BF=50 IS=1e-16 {flag})
.end
"
        ))
    };
    let noise = |netlist: &Netlist| {
        Engine::default()
            .run_noise_named_with_input_source(netlist, "b", None, "V1", &[1e3], 300.15)
            .expect("noise solves")
            .remove(0)
    };
    let quiet = noise(&deck("noiseless"));
    assert_close(
        "quiet total",
        quiet.output_noise_density,
        2.906_043e-28,
        1e-33,
    );
    for contribution in &quiet.contributions {
        assert!(
            contribution.identity.device.eq_ignore_ascii_case("R1")
                || contribution.output_contribution == 0.0,
            "{contribution:?}"
        );
    }
    let loud = noise(&deck(""));
    let device = |name: &str| -> f64 {
        loud.contributions
            .iter()
            .filter(|contribution| contribution.identity.device.eq_ignore_ascii_case(name))
            .map(|contribution| contribution.output_contribution)
            .sum()
    };
    assert_close("Q1", device("Q1"), 5.416_252e-27, 1e-32);
    assert_close("S1", device("S1"), 2.906_043e-28, 1e-33);
}

// LTspice 26.1.1 batch: V(a)=0.25, V(b)=-0.2, V(c)=1, V(d)=0.3125 (QSPICE
// ignores `load`). A load is dissipative: full current from 0.5 V across it,
// I0·u(2-u) with u = V/0.5 below that, and the resistor 2·I0·u under 0 V.
#[test]
fn a_current_source_flagged_load_draws_nothing_without_a_supply() {
    let netlist = parse(
        "current loads\n\
         V1 s 0 1\n\
         R1 s a 1k\n\
         I1 a 0 1m load\n\
         V2 t 0 -1\n\
         R2 t b 1k\n\
         I2 b 0 DC 1m LOAD\n\
         R3 s c 1k\n\
         I3 c 0 -1m load\n\
         X1 s d sub ib=2m\n\
         .subckt sub p q ib=1m\n\
         R1 p q 400\n\
         I1 q 0 {ib} load\n\
         .ends\n\
         .op\n\
         .end\n",
    );
    assert_close("V(a)", op_voltage(&netlist, "a"), 0.25, 1e-9);
    assert_close("V(b)", op_voltage(&netlist, "b"), -0.2, 1e-9);
    assert_close("V(c)", op_voltage(&netlist, "c"), 1.0, 1e-9);
    assert_close("V(d)", op_voltage(&netlist, "d"), 0.3125, 1e-8);

    // LTspice 26.1.1 batch: a waveform load carries max(I(t), 0) through the
    // same knee. V(a)=0.95 at 0.25 ms and 0.9 at 0.5 ms; V(c)=0.25 at the
    // sine's crest and 1 while the sine is negative.
    let netlist = parse(
        "waveform loads\n\
         V1 s 0 1\n\
         R1 s a 100\n\
         I1 a 0 PWL(0 0 1m 2m) load\n\
         R2 s c 1k\n\
         I2 c 0 SIN(0 1m 1k) load\n\
         .tran 0 1m 0 1u\n\
         .end\n",
    );
    let crest = tran_voltages(&netlist, 1e-3, 0.25e-3, &["a", "c"]);
    assert_close("V(a) at 0.25 ms", crest[0], 0.95, 1e-6);
    assert_close("V(c) at 0.25 ms", crest[1], 0.25, 1e-4);
    let trough = tran_voltages(&netlist, 1e-3, 0.75e-3, &["a", "c"]);
    assert_close("V(a) at 0.75 ms", trough[0], 0.85, 1e-6);
    assert_close("V(c) at 0.75 ms", trough[1], 1.0, 1e-9);
}

// QSPICE 2026 batch, which alone reads `Vsat`: V(a)=V(d)=0.25, V(b)=-0.5,
// V(c)=4*sqrt(2)-4 at the operating point; on the ramp V(f)=2(3-sqrt(7)) at
// 0.25 ms and 2(2-sqrt(3)) at 0.5 ms. The law is I*g(V/Vsat) with the knee
// of an LTspice load, for a current of either sign.
#[test]
fn a_current_source_saturates_below_its_vsat() {
    let netlist = parse(
        "saturating current sources\n\
         V1 s 0 1\n\
         R1 s a 1k\n\
         I1 a 0 DC 1m Vsat=0.5\n\
         V2 t 0 -1\n\
         R2 t b 1k\n\
         I2 b 0 DC 1m Vsat=2\n\
         R3 s c 1k\n\
         I3 c 0 DC -1m Vsat=4\n\
         X1 s d sat vs=0.5\n\
         .subckt sat p q vs=1\n\
         R1 p q 1k\n\
         I1 q 0 {1m} Vsat={vs}\n\
         .ends\n\
         .op\n\
         .end\n",
    );
    assert_close("V(a)", op_voltage(&netlist, "a"), 0.25, 1e-8);
    assert_close("V(b)", op_voltage(&netlist, "b"), -0.5, 1e-8);
    assert_close(
        "V(c)",
        op_voltage(&netlist, "c"),
        4.0 * 2f64.sqrt() - 4.0,
        1e-8,
    );
    assert_close("V(d)", op_voltage(&netlist, "d"), 0.25, 1e-8);

    let netlist = parse(
        "saturating waveform\n\
         V1 s 0 1\n\
         R1 s f 1k\n\
         I1 f 0 PWL(0 0 1m 2m) Vsat=2\n\
         .tran 0 1m 0 1u\n\
         .end\n",
    );
    let quarter = tran_voltages(&netlist, 1e-3, 0.25e-3, &["f"]);
    assert_close(
        "V(f) at 0.25 ms",
        quarter[0],
        2.0 * (3.0 - 7f64.sqrt()),
        1e-5,
    );
    let half = tran_voltages(&netlist, 1e-3, 0.5e-3, &["f"]);
    assert_close("V(f) at 0.5 ms", half[0], 2.0 * (2.0 - 3f64.sqrt()), 1e-5);
}

// QSPICE 2026 batch: V(a)=2.0, V(b)=6.0e-3 (LTspice 26.1.1 the same).
#[test]
fn passive_parasitics_written_as_subcircuit_expressions_follow_the_instance() {
    let netlist = parse(
        "parameterized parasitics\n\
         I1 0 a 1m\n\
         X1 a 0 cap radj=2k\n\
         I2 0 b 1m\n\
         X2 b 0 ind rs=3\n\
         .subckt cap p n radj=1k\n\
         C1 p n 1u Rpar={radj}\n\
         .ends\n\
         .subckt ind p n rs=1\n\
         L1 p n 1m Rser={rs*2}\n\
         .ends\n\
         .op\n\
         .end\n",
    );
    assert_close("V(a)", op_voltage(&netlist, "a"), 2.0, 1e-9);
    assert_close("V(b)", op_voltage(&netlist, "b"), 6.0e-3, 1e-12);
}

// QSPICE 2026 and LTspice 26.1.1 batch: V(o1)=V(o4)=2.866667e-3,
// V(o2)=1.7e-3, V(o3)=4 (held at the last pair beyond the table).
#[test]
fn a_controlled_source_follows_its_lookup_table() {
    let netlist = parse(
        "control tables\n\
         V1 a 0 25\n\
         R1 o1 0 1\n\
         G1 0 o1 a 0 tbl=(10 0 20 2.8m 35 3m)\n\
         V2 b 0 1.1\n\
         R2 o2 0 1\n\
         G2 0 o2 b 0 table(0.95,0m 0.969,0.5m 1,1m 1.06,1.5m 1.16,2m 1.34,2.5m 1.64,3m)\n\
         E3 o3 0 a 0 table=(0,0 10,1 20,4)\n\
         X1 a o4 sub\n\
         .subckt sub p o\n\
         R1 o 0 1\n\
         G1 0 o p 0 table( 10 0 20 2.8m 35 3m)\n\
         .ends\n\
         .op\n\
         .end\n",
    );
    assert_close("V(o1)", op_voltage(&netlist, "o1"), 2.866_666_667e-3, 1e-12);
    assert_close("V(o2)", op_voltage(&netlist, "o2"), 1.7e-3, 1e-12);
    assert_close("V(o3)", op_voltage(&netlist, "o3"), 4.0, 1e-9);
    assert_close("V(o4)", op_voltage(&netlist, "o4"), 2.866_666_667e-3, 1e-12);
}

// QSPICE 2026 and LTspice 26.1.1 batch at 127 C: V(a)=1.123, V(b)=1.1234,
// V(c)=0.9, the polynomial R0·(1 + tc1·dT + tc2·dT² + tc3·dT³ + ...).
#[test]
fn a_resistor_tc_is_a_polynomial_of_any_length() {
    let netlist = parse(
        "resistor tc\n\
         I1 0 a 1m\n\
         R1 a 0 1k tc=1m 2u 3n\n\
         I2 0 b 1m\n\
         R2 b 0 1k tc = 1m, 2u, 3n, 4p noiseless\n\
         I3 0 c 1m\n\
         R3 c 0 1k tc=-1m\n\
         .temp 127\n\
         .op\n\
         .end\n",
    );
    assert_close("V(a)", op_voltage(&netlist, "a"), 1.123, 1e-12);
    assert_close("V(b)", op_voltage(&netlist, "b"), 1.1234, 1e-12);
    assert_close("V(c)", op_voltage(&netlist, "c"), 0.9, 1e-12);
}

// QSPICE 2026 and LTspice 26.1.1 batch: V(a)=4, V(n§x)=3.
#[test]
fn a_section_sign_belongs_to_element_and_node_names() {
    let netlist = parse(
        "section sign\n\
         I§G1 0 a 1m\n\
         R§1 a n§x 1k\n\
         R2 n§x 0 3k\n\
         .op\n\
         .end\n",
    );
    assert_close("V(a)", op_voltage(&netlist, "a"), 4.0, 1e-9);
    assert_close("V(n§x)", op_voltage(&netlist, "n§x"), 3.0, 1e-9);
}

// LTspice 26.1.1 batch: V(o)=1 and V(p)=1 at the operating point; on the
// ramp V(o)=V(a)/2 and, inside the subcircuit, V(s)=V(a)/4, and the machine
// leaves LO when I(R2) passes 0.5 mA at 0.5 ms.
#[test]
fn a_resistor_current_is_readable_in_expressions() {
    let netlist = parse(
        "resistor currents\n\
         V1 a 0 2\n\
         R1 a b 1k\n\
         R2 b 0 1k\n\
         B1 o 0 V=I(R1)*1k\n\
         R3 o 0 1\n\
         B2 p 0 V=I(R2)*I(R1)*1e6\n\
         R4 p 0 1\n\
         .op\n\
         .end\n",
    );
    assert_close("V(o)", op_voltage(&netlist, "o"), 1.0, 1e-9);
    assert_close("V(p)", op_voltage(&netlist, "p"), 1.0, 1e-9);

    let netlist = parse(
        "resistor currents in time\n\
         V1 a 0 PWL(0 0 1m 2)\n\
         R1 a b 1k\n\
         R2 b 0 1k\n\
         B1 o 0 V=I(R1)*1k\n\
         R3 o 0 1\n\
         X1 a s cell\n\
         .subckt cell in out\n\
         RA in m 2k\n\
         RB m 0 2k\n\
         B1 out 0 V=I(RA)*1k\n\
         RL out 0 1\n\
         .ends\n\
         .machine\n\
         .state LO 0\n\
         .state HI 1\n\
         .rule LO HI I(R2) > 0.5m\n\
         .output (q) state\n\
         .endmachine\n\
         R5 q 0 1k\n\
         .tran 0 1m 0 1u\n\
         .end\n",
    );
    let early = tran_voltages(&netlist, 1e-3, 0.4e-3, &["o", "s", "q"]);
    assert_close("V(o) at 0.4 ms", early[0], 0.4, 1e-6);
    assert_close("V(s) at 0.4 ms", early[1], 0.2, 1e-6);
    assert_close("V(q) at 0.4 ms", early[2], 0.0, 1e-9);
    let late = tran_voltages(&netlist, 1e-3, 0.51e-3, &["q"]);
    assert_close("V(q) at 0.51 ms", late[0], 1.0, 1e-9);

    let error = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
        .run_dc_op(&parse(
            "xyce lead current\nV1 a 0 2\nR1 a 0 1k\nB1 o 0 V={I(R1)*1k}\nR3 o 0 1\n.op\n.end\n",
        ))
        .expect_err("Xyce reads no resistor current");
    assert!(
        error
            .to_string()
            .contains("lead_current_not_solution_variable"),
        "{error}"
    );
}

// LTspice 26.1.1 batch; QSPICE refuses every one of these reads. A diode,
// a B, I or G current source and a W switch report their current from the
// first terminal to the second, and an S switch the other way round. B9
// reads its own current: LTspice stops its iteration at V(q) = -1.99951,
// short of the fixed point -2 a Newton solve reaches.
#[test]
fn a_branchless_device_current_is_readable_in_expressions() {
    let netlist = parse(
        "lead currents of branchless devices\n\
         V1 a 0 1\n\
         R1 a b 100\n\
         D1 b 0 DX\n\
         .model DX D(Is=1e-14)\n\
         B1 c 0 I=2m*V(a)\n\
         R2 c 0 1k\n\
         I1 d 0 3m\n\
         R5 d 0 1k\n\
         G1 e 0 a 0 4m\n\
         R6 e 0 1k\n\
         S1 a f a 0 SWX\n\
         S2 0 h a 0 SWX\n\
         R7 f 0 50\n\
         R9 a h 50\n\
         .model SWX SW(Ron=50 Roff=1Meg Vt=0.5)\n\
         W1 a k V1 WX\n\
         R10 k 0 50\n\
         .model WX CSW(Ron=50 Roff=1Meg It=-1)\n\
         B9 q 0 I=1m+0.5*I(B9)\n\
         R11 q 0 1k\n\
         B2 o1 0 V=1k*I(D1)\n\
         B3 o2 0 V=1k*I(B1)\n\
         B4 o3 0 V=1k*I(I1)\n\
         B5 o4 0 V=1k*I(G1)\n\
         B6 o5 0 V=1k*I(S1)\n\
         B7 o6 0 V=1k*I(S2)\n\
         B8 o7 0 V=1k*I(W1)\n\
         X1 a s cell\n\
         .subckt cell in out\n\
         RA in m 100\n\
         DA m 0 DX\n\
         BA out 0 V=1k*I(DA)\n\
         .ends\n\
         .op\n\
         .end\n",
    );
    let op = Engine::default()
        .run_dc_op(&netlist)
        .expect("operating point solves");
    let voltage = |node: &str| {
        op.try_voltage_named(node)
            .unwrap_or_else(|| panic!("no voltage for {node}"))
    };
    // The diode's own operating point sits 3 ppm from LTspice's; its read
    // current is the one through R1.
    let diode_current = (voltage("a") - voltage("b")) / 100.0;
    for node in ["o1", "s"] {
        assert_close(&format!("V({node})"), voltage(node), 3.15188233011747, 2e-5);
        assert_close(
            &format!("V({node})"),
            voltage(node),
            1e3 * diode_current,
            1e-9,
        );
    }
    for (node, expected) in [
        ("o2", 2.0),
        ("o3", 3.0),
        ("o4", 4.0),
        ("o5", -10.00000000025),
        ("o6", 10.00000000025),
        ("o7", 10.0),
        ("q", -2.0),
    ] {
        assert_close(&format!("V({node})"), voltage(node), expected, 1e-6);
    }

    let error = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce))
        .run_dc_op(&parse(
            "xyce lead current\nV1 a 0 1\nD1 a 0 DX\n.model DX D\nB1 o 0 V={I(D1)*1k}\nR3 o 0 1\n.op\n.end\n",
        ))
        .expect_err("Xyce reads no diode current");
    assert!(
        error
            .to_string()
            .contains("lead_current_not_solution_variable"),
        "{error}"
    );
}

// QSPICE 2026 batch: V(o1)=2.867132353, V(o2)=4.687499999 and
// V(o3)=0.974575022; LTspice 26.1.1 agrees to 4 ppm. Each device takes the
// card of its own family among the subcircuit's same-named ones.
#[test]
fn same_named_models_of_different_families_serve_their_own_devices() {
    let netlist = parse(
        "same-named models\n\
         V1 a 0 5\n\
         X1 a o1 o2 o3 cell\n\
         .subckt cell vcc o1 o2 o3\n\
         R1 vcc o1 10k\n\
         Q1 o1 b 0 N\n\
         RB vcc b 1Meg\n\
         R2 vcc o2 100\n\
         M1 o2 g 0 0 N\n\
         VG g 0 3\n\
         R3 vcc o3 1k\n\
         D1 o3 m IQ\n\
         S1 m 0 vcc 0 IQ\n\
         .model N NPN(Bf=50)\n\
         .model N VDMOS(Vto=0.5 Kp=0.001)\n\
         .model IQ D(Is=1e-12)\n\
         .model IQ SW(Ron=100 Roff=1Meg Vt=2)\n\
         .ends\n\
         .op\n\
         .end\n",
    );
    let op = Engine::default()
        .run_dc_op(&netlist)
        .expect("operating point solves");
    for (node, expected, tolerance) in [
        ("o1", 2.867132353, 1e-5),
        ("o2", 4.687499999, 1e-6),
        ("o3", 0.974575022, 1e-5),
    ] {
        let actual = op
            .try_voltage_named(node)
            .unwrap_or_else(|| panic!("no voltage for {node}"));
        assert_close(&format!("V({node})"), actual, expected, tolerance);
    }
}

// QSPICE 2026 and LTspice 26.1.1 batches agree: inv() is 1 below 0.5 and 0
// from 0.5 up.
#[test]
fn inv_is_the_logical_inverse_from_one_half() {
    let netlist = parse(
        "inv\n\
         V1 a 0 0.4\n\
         V2 b 0 0.6\n\
         V3 c 0 0.5\n\
         V4 d 0 -3\n\
         B1 o1 0 V=inv(V(a))\n\
         B2 o2 0 V=inv(V(b))\n\
         B3 o3 0 V=inv(V(c))\n\
         B4 o4 0 V=inv(V(d))\n\
         B5 o5 0 V=inv(V(a))*inv(V(d))+2*INV(V(b))\n\
         .op\n\
         .end\n",
    );
    let op = Engine::default()
        .run_dc_op(&netlist)
        .expect("operating point solves");
    for (node, expected) in [
        ("o1", 1.0),
        ("o2", 0.0),
        ("o3", 0.0),
        ("o4", 1.0),
        ("o5", 1.0),
    ] {
        let actual = op
            .try_voltage_named(node)
            .unwrap_or_else(|| panic!("no voltage for {node}"));
        assert_close(&format!("V({node})"), actual, expected, 1e-12);
    }
    let error = Netlist::parse("inv arity\nB1 o 0 V=inv(1,2)\nR1 o 0 1\n.op\n.end\n")
        .map(|netlist| Engine::default().run_dc_op(&netlist).map(|_| ()))
        .map_or_else(
            |error| error.to_string(),
            |run| run.err().map(|e| e.to_string()).unwrap_or_default(),
        );
    assert!(
        error
            .to_ascii_lowercase()
            .contains("inv expects 1 argument"),
        "{error}"
    );
}

// QSPICE 2026 batch, which alone reads a switch model written on the switch
// card (LTspice refuses it): V(a)=V(b)=V(g)=0.759746927, V(e)=9.900990e-3,
// V(f)=1.960784e-2, V(h)=0.999000999.
#[test]
fn a_switch_takes_its_model_parameters_on_its_own_card() {
    let netlist = parse(
        "inline switch models\n\
         V1 c 0 0.5\n\
         V2 s 0 1\n\
         R1 s a 1k\n\
         S1 a 0 c 0 Ron=10 Roff=1Meg Vt=.5 Vh=-.4\n\
         R2 s b 1k\n\
         S2 b 0 c 0 SWM\n\
         .model SWM SW(Ron=10 Roff=1Meg Vt=.5 Vh=-.4)\n\
         V3 d 0 1\n\
         R3 s e 1k\n\
         S3 e 0 d 0 Ron=10 Roff=1Meg Vt=.5 Vh=.1\n\
         X1 s f c sub ron=20\n\
         .subckt sub p o c ron=1\n\
         R1 p o 1k\n\
         S1 o 0 c 0 Ron={ron} Roff=1Meg Vt=.3 Vh=.1\n\
         .ends\n\
         R4 s g 1k\n\
         S4 g 0 c 0 Ron=10 Roff=1Meg Vt=.5 Vh=-.4 OFF ttol=5n\n\
         R5 s h 1k\n\
         S5 h 0 c 0 Ron = 10 , Roff = 1Meg , Vt = 1.5 , Vh = .1\n\
         .op\n\
         .end\n",
    );
    for node in ["a", "b", "g"] {
        assert_close(node, op_voltage(&netlist, node), 0.759_746_926_6, 1e-9);
    }
    assert_close("V(e)", op_voltage(&netlist, "e"), 9.900_990_099e-3, 1e-12);
    assert_close("V(f)", op_voltage(&netlist, "f"), 1.960_784_314e-2, 1e-11);
    assert_close("V(h)", op_voltage(&netlist, "h"), 0.999_000_999, 1e-9);
}

// QSPICE 2026 batch at 1 kHz: with R1 and J2 quiet V(o) carries J1 alone,
// 1.022101e-15 V^2/Hz; all three noisy give 2.060779e-15. LTspice refuses
// `noiseless` on a JFET card.
#[test]
fn a_noiseless_instance_flag_quiets_that_device() {
    let deck = |flag: &str, bare: &str| {
        parse(&format!(
            "noiseless instances\n\
             .param X={flag}\n\
             V1 d 0 10\n\
             R1 d o 1k noiseless=X\n\
             J1 o g 0 JM\n\
             J2 o h 0 JM {bare}\n\
             V2 g 0 -1 AC 1\n\
             V3 h 0 -1\n\
             R3 d q 1k noiseless=0\n\
             .model JM NJF Vto=-2 beta=1m KF=1e-15\n\
             .end\n"
        ))
    };
    let noise = |netlist: &Netlist| {
        Engine::default()
            .run_noise_named_with_input_source(netlist, "o", None, "V2", &[1e3], 300.15)
            .expect("noise solves")
            .remove(0)
    };
    let quiet = noise(&deck("1", "noiseless"));
    assert_close(
        "quiet V(o) noise",
        quiet.output_noise_density,
        1.022_101e-15,
        1e-20,
    );
    for contribution in &quiet.contributions {
        let device = contribution.identity.device.to_ascii_uppercase();
        assert!(
            !matches!(device.as_str(), "R1" | "J2") || contribution.output_contribution == 0.0,
            "{contribution:?}"
        );
    }
    let loud = noise(&deck("0", ""));
    assert_close(
        "loud V(o) noise",
        loud.output_noise_density,
        2.060_779e-15,
        1e-20,
    );
}

// LTspice 26.1.1 and QSPICE 2026 batch agree: with QF=5 and w0=2*pi*1k the
// band-pass gives V(b)=1 at 1 kHz and 0.01746725-0.1310044j at 2 kHz, the
// low-pass V(c)=0.5-0.5j and 0.2-0.4j. PSpice's LAPLACE form is the same
// low-pass.
#[test]
fn a_laplace_expression_reads_parameters_braced_or_bare() {
    let netlist = parse(
        "laplace parameters\n\
         .param QF=5\n\
         .param w0=2*pi*1K\n\
         V1 a 0 AC 1\n\
         E1 b 0 a 0 Laplace={w0/QF}*s/(s*s+{w0/QF}*s+{w0*w0})\n\
         R1 b 0 1k\n\
         E2 c 0 a 0 Laplace=w0/(s+w0)\n\
         R2 c 0 1k\n\
         E3 d 0 LAPLACE {V(a)} = {w0/(s+w0)}\n\
         R3 d 0 1k\n\
         .end\n",
    );
    let sweep = Engine::default()
        .run_ac(&netlist, &[1e3, 2e3])
        .expect("AC sweep solves");
    for (point, node, re, im) in [
        (0, "b", 1.0, 0.0),
        (0, "c", 0.5, -0.5),
        (0, "d", 0.5, -0.5),
        (1, "b", 1.746_724_890_829_694e-2, -1.310_043_668_122_27e-1),
        (1, "c", 0.2, -0.4),
        (1, "d", 0.2, -0.4),
    ] {
        let result = &sweep[point];
        let index = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node))
            .unwrap_or_else(|| panic!("V({node}) is solved"));
        let voltage = result.voltages[index];
        assert_close(&format!("Re V({node}) #{point}"), voltage.re, re, 1e-9);
        assert_close(&format!("Im V({node}) #{point}"), voltage.im, im, 1e-9);
    }
}

// LTspice 26.1.1 batch, `.tran 0 60u 0 0.1u startup`: V1 DC 5 and I1 DC 1m
// start the operating point at zero and rise linearly to their value at
// exactly 20 us; the SIN, PULSE and PWL sources follow their waveforms from
// t = 0. V(a), V(c), V(e), V(f) = 0, 0, 1, 1 at 0; 2.5, 0.5, 3, 1.4 at 10 us;
// 5, 1, 3, 2.2 at 30 us. QSPICE accepts the word and ignores it.
#[test]
fn a_startup_transient_ramps_the_dc_sources_from_zero() {
    let netlist = parse(
        "startup\n\
         V1 a 0 5\n\
         R1 a 0 1k\n\
         V2 b 0 SIN(1 1 100k)\n\
         R2 b 0 1k\n\
         I1 0 c 1m\n\
         R3 c 0 1k\n\
         I2 0 e PULSE(1m 3m 5u 1n 1n 100u 200u)\n\
         R5 e 0 1k\n\
         V4 f 0 PWL(0 1 50u 3)\n\
         R6 f 0 1k\n\
         .tran 0 60u 0 0.1u startup\n\
         .end\n",
    );
    for (time, expected) in [
        (0.0, [0.0, 1.0, 0.0, 1.0, 1.0]),
        (10e-6, [2.5, f64::NAN, 0.5, 3.0, 1.4]),
        (30e-6, [5.0, f64::NAN, 1.0, 3.0, 2.2]),
    ] {
        let values = tran_voltages(&netlist, 60e-6, time, &["a", "b", "c", "e", "f"]);
        for ((node, value), expected) in ["a", "b", "c", "e", "f"].iter().zip(values).zip(expected)
        {
            if !expected.is_nan() {
                assert_close(&format!("V({node}) at {time:e}"), value, expected, 1e-9);
            }
        }
    }
    assert_close(
        "V(a) at the operating point",
        op_voltage(&netlist, "a"),
        5.0,
        1e-12,
    );
}

// QSPICE 2026 steps `V1 a 0 SIN(0 1 100k)` under `.tran 5u 5m` every
// 0.24-0.8 us, whatever TSTEP is, and its waveform read between those points
// stays within 7.85e-3 of the sine; LTspice 26.1.1 within 3.8e-2. Stepped at
// TSTEP, two points a period, it strays by most of the amplitude.
#[test]
fn a_sine_source_is_resolved_whatever_tstep_says() {
    let netlist = parse(
        "sine alone\n\
         V1 a 0 SIN(0 1 100k)\n\
         R1 a 0 1k\n\
         .tran 5u 5m\n\
         .end\n",
    );
    let tran = Engine::default()
        .run_tran(&netlist, 5e-3, 5e-6)
        .expect("transient solves");
    let wave = tran
        .try_voltage_waveform_named("a")
        .expect("V(a) is solved");
    let mut worst = 0.0f64;
    for k in 0..20_000 {
        let time = 5e-3 * f64::from(k) / 20_000.0;
        let index = tran
            .time
            .partition_point(|t| *t < time)
            .clamp(1, tran.time.len() - 1);
        let (t0, t1) = (tran.time[index - 1], tran.time[index]);
        let value = wave[index - 1] + (wave[index] - wave[index - 1]) * (time - t0) / (t1 - t0);
        worst = worst.max((value - (std::f64::consts::TAU * 1e5 * time).sin()).abs());
    }
    assert!(
        worst <= 7.85e-3,
        "V(a) strays {worst:e} from the sine between points"
    );
}

// QSPICE 2026 steps `V1 a 0 SFFM(0 1 100k 5 10k)` under `.tran 5u 5m` at the
// carrier's pace, about as often as a 100 kHz sine, and its waveform read
// between those points strays within 1.13e-2 of the exact one; LTspice
// 26.1.1 within 1.97e-2. RSpice resolves the carrier swung up to 150 kHz.
#[test]
fn an_sffm_source_is_resolved_whatever_tstep_says() {
    let netlist = parse(
        "sffm alone\n\
         V1 a 0 SFFM(0 1 100k 5 10k)\n\
         R1 a 0 1k\n\
         .tran 5u 5m\n\
         .end\n",
    );
    let tran = Engine::default()
        .run_tran(&netlist, 5e-3, 5e-6)
        .expect("transient solves");
    let wave = tran
        .try_voltage_waveform_named("a")
        .expect("V(a) is solved");
    let exact = |time: f64| {
        use std::f64::consts::TAU;
        (TAU * 1e5 * time + 5.0 * (TAU * 1e4 * time).sin()).sin()
    };
    let mut worst = 0.0f64;
    for k in 0..20_000 {
        let time = 5e-3 * f64::from(k) / 20_000.0;
        let index = tran
            .time
            .partition_point(|t| *t < time)
            .clamp(1, tran.time.len() - 1);
        let (t0, t1) = (tran.time[index - 1], tran.time[index]);
        let value = wave[index - 1] + (wave[index] - wave[index - 1]) * (time - t0) / (t1 - t0);
        worst = worst.max((value - exact(time)).abs());
    }
    assert!(
        worst <= 1.13e-2,
        "V(a) strays {worst:e} from the SFFM between points"
    );
}

// ngspice-46 and LTspice: 16, 3, 3 and 2 (g = 2, v(a) = v(b) = 1).
#[test]
fn a_braced_parameter_that_starts_a_value_expression_keeps_the_rest_of_the_line() {
    let netlist = parse(
        "* braced head
.param g=2
va a 0 1
vb b 0 1
b1 o1 0 v={g}*(v(a)+v(b))^3
r1 o1 0 1k
e2 o2 0 value={g}*v(a)+1
r2 o2 0 1k
g3 0 o3 value={g}*v(a)+1
r3 o3 0 1
b4 o4 0 v={g*v(a)}
r4 o4 0 1k
.op
.end
",
    );
    for (node, expected) in [("o1", 16.0), ("o2", 3.0), ("o3", 3.0), ("o4", 2.0)] {
        assert_close(node, op_voltage(&netlist, node), expected, 1e-9);
    }
}

// ngspice-46 reads any card that starts `.param` as one, and then takes the
// `.elseif` branch: V(m) = 5k / (2k + 5k).
#[test]
fn ngspice_reads_a_parameters_card_as_a_param_card() {
    let netlist = parse(
        "* parameters spelling
.parameters a=2 b=3
v1 n1 0 1
r1 n1 m {1k*a}
.if (a==1)
r5 m 0 7k
.elseif (a==2)
r5 m 0 5k
.else
r5 m 0 9k
.endif
.op
.end
",
    );
    assert_close("V(m)", op_voltage(&netlist, "m"), 5.0 / 7.0, 1e-12);
}

// ngspice-46: a `+` line before any card is dropped ("Illegal continuation
// line: ignored."), `\\` ends a line that the next one continues, and `.opts`
// is `.options`. V(m) = 1: the 1e6 mil resistor is 25.4 ohm.
#[test]
fn ngspice_line_continuations_and_the_opts_spelling() {
    let netlist = parse(
        "* ngspice lines
//  a comment-only line
+ so this continuation continues nothing
.opts reltol=1e-4
v1 in 0 2
r3 in m \\\\
1e6mil
r4 m 0 25.4
.op
.end
",
    );
    assert_eq!(netlist.options.reltol, Some(1e-4));
    assert_eq!(netlist.elements.len(), 3, "{:?}", netlist.elements);
    assert!(
        netlist
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "orphan-continuation"),
        "{:?}",
        netlist.diagnostics
    );
    assert_close("V(m)", op_voltage(&netlist, "m"), 1.0, 1e-9);
}

// ngspice-46: V(out) = 0.5, V(o3) = 0.396004 (LTspice and QSPICE 0.396005);
// the AC response and a 10 ns transient match ngspice to six digits too.
// A root line binds a URC card written after it; a line inside a subcircuit
// binds one written before the subcircuit.
#[test]
fn a_u_card_with_a_urc_model_is_ngspices_distributed_rc_line() {
    let netlist = parse(
        "* urc
V1 in 0 1
U1 in out 0 umod L=1m N=6
RL out 0 1k
.model dmod URC(RPERL=1meg CPERL=1n ISPERL=1e-12 RSPERL=10)
.subckt line a b
U3 a b 0 dmod L=1m N=4
.ends
X1 in o3 line
R3 o3 0 1k
.model umod URC(K=1.5 FMAX=1G RPERL=1meg CPERL=1n)
.op
.end
",
    );
    let count = |prefix: &str| {
        netlist
            .elements
            .iter()
            .filter(|element| element.name.starts_with(prefix))
            .count()
    };
    assert_eq!((count("RU1#"), count("CU1#")), (12, 11));
    assert_close("V(out)", op_voltage(&netlist, "out"), 0.5, 1e-9);
    assert_close("V(o3)", op_voltage(&netlist, "o3"), 0.396004, 2e-6);
}

// A URC card that gives ISPERL, even as zero, shunts each section with a
// diode, as ngspice's does; ngspice-46, LTspice and QSPICE all run such a
// line, and V(out) = 0.5 in ngspice-46.
#[test]
fn a_urc_card_with_a_zero_isperl_shunts_the_line_with_diodes() {
    let netlist = parse(
        "* urc isperl=0
V1 in 0 1
U1 in out 0 umod L=1m N=6
RL out 0 1k
.model umod URC(RPERL=1meg CPERL=1n ISPERL=0)
.op
.end
",
    );
    let count = |prefix: &str| {
        netlist
            .elements
            .iter()
            .filter(|element| element.name.starts_with(prefix))
            .count()
    };
    assert_eq!((count("DU1#"), count("CU1#")), (11, 0));
    assert_close("V(out)", op_voltage(&netlist, "out"), 0.5, 1e-8);
}

// HSPICE's `.DCVOLT` is its `.IC`; Xyce reads both spellings the same way
// (QSPICE ignores the card and LTspice rejects it).
#[test]
fn a_dcvolt_card_sets_initial_conditions_like_ic() {
    let netlist = parse(
        "* dcvolt
c1 a 0 1n
r1 a 0 1k
c2 b 0 1n
r2 b 0 1k
.dcvolt v(a)=1
.dcvolt b 2
.tran 10n 5u uic
.end
",
    );
    let initial = |node: &str| {
        netlist
            .initial_conditions
            .iter()
            .find(|condition| condition.node.eq_ignore_ascii_case(node))
            .map(|condition| condition.voltage)
    };
    assert_eq!((initial("a"), initial("b")), (Some(1.0), Some(2.0)));
}

// QSPICE: 1 mA into 1k with GSHUNT=1m gives V(a) = 0.5 at the operating
// point and in the transient, while the AC analysis leaves the shunt out
// (Re V(a) = 0.999961 at 1 kHz).
#[test]
fn gshunt_loads_every_node_in_the_operating_point_and_transient_only() {
    let netlist = parse(
        "* gshunt
.option gshunt=1m
i1 0 a dc 1m ac 1m
r1 a 0 1k
c1 a 0 1n
.op
.end
",
    );
    assert_close("V(a) op", op_voltage(&netlist, "a"), 0.5, 1e-9);
    assert_close(
        "V(a) tran",
        tran_voltages(&netlist, 1e-4, 1e-4, &["a"])[0],
        0.5,
        1e-6,
    );
    let point = Engine::default()
        .run_ac(&netlist, &[1e3])
        .expect("AC solves")
        .pop()
        .expect("one AC point");
    let index = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("a"))
        .expect("node a");
    assert_close("Re V(a) ac", point.voltages[index].re, 0.999961, 1e-6);
}

// 1 V through R1 = 1k into R2 = 1k with GSHUNT=1m on both nodes. The shunt
// loads every DC-domain answer: QSPICE's .dc gives V(a) = 0.333 and LTspice's
// .tf gives gain 1/3, Zin 600, Zout 333.33. DC .sens is the slope of that
// operating point. QSPICE's .ac leaves the shunt out, so the frequency domain
// sees R1 || R2 alone: the pole is at (G1 + G2) / C = 2e6 rad/s.
#[test]
fn gshunt_is_in_every_dc_domain_analysis_and_no_frequency_domain_one() {
    use rspice_core::analysis::AcSensitivityOutput;
    let netlist = parse(
        "* gshunt divider
.option gshunt=1m
v1 in 0 dc 1 ac 1
r1 in a 1k
r2 a 0 1k
c1 a 0 1n
.op
.end
",
    );
    let engine = Engine::default();
    assert_close("V(a) op", op_voltage(&netlist, "a"), 1.0 / 3.0, 1e-9);

    let sweep = engine
        .run_dc_sweep(&netlist, "v1", 1.0, 1.0, 1.0)
        .expect("DC sweep solves");
    let swept = sweep[0].1.try_voltage_named("a").expect("V(a)");
    assert_close("V(a) dc", swept, 1.0 / 3.0, 1e-9);

    let transfer = engine
        .run_transfer_function(&netlist, "a", None, false, "v1")
        .expect(".tf solves");
    assert_close("tf gain", transfer.gain, 1.0 / 3.0, 1e-9);
    assert_close("tf zin", transfer.input_impedance, 600.0, 1e-6);
    assert_close("tf zout", transfer.output_impedance, 1000.0 / 3.0, 1e-6);

    let output = engine
        .build_circuit(&netlist)
        .expect("circuit builds")
        .get_node_by_name("a")
        .expect("node a");
    let sens = engine
        .run_sensitivity_dc_complete(
            &netlist,
            AcSensitivityOutput::Voltage {
                positive: output,
                negative: None,
            },
            &["R1".to_owned(), "R2".to_owned(), "V1".to_owned()],
        )
        .expect(".sens solves");
    // dV(a)/dR1 = -(G2 + g) V1 / (1 + R1 (G2 + g))^2, dV(a)/dR2 = R1 G2^2 V1 / (same).
    assert_close(
        "sens r1",
        sens.get("R1").expect("R1").absolute,
        -2.0e-3 / 9.0,
        1e-12,
    );
    assert_close(
        "sens r2",
        sens.get("R2").expect("R2").absolute,
        1.0e-3 / 9.0,
        1e-12,
    );
    assert_close(
        "sens v1",
        sens.get("V1").expect("V1").absolute,
        1.0 / 3.0,
        1e-9,
    );

    let point = engine
        .run_ac(&netlist, &[1.0])
        .expect("AC solves")
        .pop()
        .expect("one AC point");
    let index = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("a"))
        .expect("node a");
    assert_close("Re V(a) ac", point.voltages[index].re, 0.5, 1e-6);

    let roots = engine
        .run_pz(&netlist, output, output)
        .expect("pole-zero analysis solves");
    assert_eq!(roots.poles.len(), 1, "poles={:?}", roots.poles);
    assert_close("pz pole", roots.poles[0].re, -2.0e6, 1.0);
}

// QSPICE: V(a)=1, V(b)=4.545455, V(e)=V(f)=4.999995e-6; LTspice agrees to
// 1e-8. LTspice's own libraries name switch models `800`, `4K` and `50µA`.
#[test]
fn switch_models_may_be_named_like_numbers() {
    let netlist = parse(
        "* switch models whose names read as numbers
V1 in 0 5
VC c 0 1
S1 in a c 0 800
R1 a 0 1k
S2 in b c 0 50µA
R2 b 0 1k
S3 in e c 0 4K
R3 e 0 1k
W1 in f VC 2A
R4 f 0 1k
.model 800 SW(Ron=4k Roff=1G Vt=.5 Vh=-.4)
.model 50µA SW(Ron=100 Roff=1T Vt=.5 Vh=-.1)
.model 4K SW(Ron=2k Roff=1G Vt=1.5 Vh=-.4)
.model 2A CSW(Ron=3k Roff=1G It=1 Ih=.1)
.op
.end
",
    );
    assert_close("V(a)", op_voltage(&netlist, "a"), 1.0, 1e-6);
    assert_close("V(b)", op_voltage(&netlist, "b"), 4.545455, 1e-6);
    assert_close("V(e)", op_voltage(&netlist, "e"), 4.999995e-6, 1e-10);
    assert_close("V(f)", op_voltage(&netlist, "f"), 4.999995e-6, 1e-10);
}

// QSPICE and LTspice: V(g)=V(h)=1e-3 and V(x1.o2)=2; V(c)=0.632120 at 1 ms
// (QSPICE 0.6321200, LTspice 0.6321204).
#[test]
fn behavioral_sources_take_a_parallel_resistance_and_capacitance() {
    let netlist = parse(
        "* B sources with Rpar and Cpar
V1 a 0 1
Rs a 0 1k
B7 0 g I=1m*abs(I(Rs)) Rpar=1k Cpar=10p
B8 0 h I=1m*abs(I(Rs))
R8 h 0 1k
X1 a k sub
.subckt sub p o
Rq p 0 1k
B9 0 o I=1m*abs(I(Rq)) Rpar=1k Cpar=10p
B10 0 o2 I=2m Rpar={rp}
.param rp=1k
.ends sub
.op
.end
",
    );
    assert_close("V(g)", op_voltage(&netlist, "g"), 1e-3, 1e-12);
    assert_close("V(h)", op_voltage(&netlist, "h"), 1e-3, 1e-12);
    assert_close("V(k)", op_voltage(&netlist, "k"), 1e-3, 1e-12);
    assert_close("V(x1.o2)", op_voltage(&netlist, "x1.o2"), 2.0, 1e-9);

    let netlist = parse(
        "* B source Rpar and Cpar in a subcircuit
V1 a 0 PULSE(0 1 0 1n 1n 1 2)
X1 a c bload
.subckt bload in out
B1 0 out I=1m*V(in) Rpar=1k Cpar={cp}
.param cp=1u
.ends bload
.tran 1u 2m
.end
",
    );
    let values = tran_voltages(&netlist, 2e-3, 1e-3, &["c"]);
    assert_close("V(c) at 1 ms", values[0], 0.632_120, 2e-5);
}

// LTspice and QSPICE: V(o1)=22, V(o2)=6 and V(o4)=6.
#[test]
fn a_probe_reads_a_net_whose_name_carries_a_sign_inside_or_at_the_end() {
    let netlist = parse(
        "* probes of nets whose names carry a sign inside or at the end
V1 IN+ 0 3
V2 IN- 0 1
V3 TC-HEAT 0 2
B1 o1 0 V=V(IN+,IN-)*10+V(TC-HEAT)
X1 IN+ IN- o2 amp
Vsense+ o2 o3 0
R3 o3 0 1k
B4 o4 0 V=I(Vsense+)*1k
R4 o4 0 1k
.subckt amp a b o
R1 a mid+ 1k
R2 mid+ b 1k
B2 o 0 V=V(mid+)*2+V(a)-V(b)
.ends amp
.op
.end
",
    );
    assert_close("V(o1)", op_voltage(&netlist, "o1"), 22.0, 1e-9);
    assert_close("V(o2)", op_voltage(&netlist, "o2"), 6.0, 1e-9);
    assert_close("V(o4)", op_voltage(&netlist, "o4"), 6.0, 1e-9);
}

// QSPICE: V(b)=0.3038914; LTspice: 0.3038917.
#[test]
fn a_model_name_may_join_a_digit_led_word_with_a_hyphen() {
    let netlist = parse(
        "* a diode model whose name ends in a hyphen and a digit-led word
V1 a 0 5
R1 a b 1k
D1 b 0 CMDSH-4E
.model CMDSH-4E D(Is=82.9n Rs=2.02 N=1.04)
.op
.end
",
    );
    assert_close("V(b)", op_voltage(&netlist, "b"), 0.303_891_4, 1e-6);
}

// QSPICE: V(b)=8.205128, V(c)=9.786084, V(d)=2.806655, V(e)=3.333333;
// LTspice agrees to 1e-8.
#[test]
fn a_resistance_written_without_braces_is_read_to_the_end_of_the_card() {
    let netlist = parse(
        "* behavioral resistors written without braces
.param Mode=1
V1 a 0 10
V4 rval 0 350
R1 a b R=V(rval)
R2 b 0 R=if(Mode>0.5,1.6k,100)
R3 a c R=9.1K*EXP(3490/(V(rval)+273) - 3490/300)
R4 c 0 1k
R5 a d R=(V(rval,b)/V(a,c))**2/1k+10
R6 d 0 1k
X1 a e sub
.subckt sub p q
R7 p q R=2*V(p)**2
R8 q 0 100
.ends sub
.op
.end
",
    );
    assert_close("V(b)", op_voltage(&netlist, "b"), 8.205_128, 1e-6);
    assert_close("V(c)", op_voltage(&netlist, "c"), 9.786_084, 1e-6);
    assert_close("V(d)", op_voltage(&netlist, "d"), 2.806_655, 1e-6);
    assert_close("V(e)", op_voltage(&netlist, "e"), 3.333_333, 1e-6);
}

// C1's charge 1u*x+0.5u*x² charges through 1k as -v-2ln(1-v) = t'/1ms, with
// t' the time since the middle of the 1 us edge; LTspice lands within 3e-5
// of it, QSPICE within 4e-4. C2 is a 2 uF capacitor read through V(k) and L1
// a 1 H inductor at these currents. QSPICE: V(c)=0.221005, 0.393318,
// 0.632029 and V(d)=0.606835, 0.368064, 0.135403 at 0.5, 1 and 2 ms;
// LTspice agrees to 1e-6.
#[test]
fn a_capacitor_given_its_charge_and_an_inductor_given_its_flux() {
    let netlist = parse(
        "* charge-defined capacitors and a flux-defined inductor
V1 a 0 PULSE(0 1 0 1u 1u 1 2)
R1 a b 1k
C1 b 0 Q=1u*x+0.5u*x*x
Vk k 0 1
R2 a c 1k
C2 c 0 Q=x*1u*(1+V(k))
R3 a d 1k
L1 d 0 Flux=1*x+0.5*x*x*x
.tran 1u 3m
.end
",
    );
    let exact_b = |time: f64| {
        let target = (time - 0.5e-6) / 1e-3;
        let mut v: f64 = 0.5;
        for _ in 0..50 {
            let residual = -v - 2.0 * (1.0 - v).ln() - target;
            v -= residual / (-1.0 + 2.0 / (1.0 - v));
        }
        v
    };
    for (time, c, d) in [
        (0.5e-3, 0.221_005, 0.606_835),
        (1e-3, 0.393_318, 0.368_064),
        (2e-3, 0.632_029, 0.135_403),
    ] {
        let values = tran_voltages(&netlist, 3e-3, time, &["b", "c", "d"]);
        assert_close(&format!("V(b) at {time}"), values[0], exact_b(time), 5e-4);
        assert_close(&format!("V(c) at {time}"), values[1], c, 2e-4);
        assert_close(&format!("V(d) at {time}"), values[2], d, 2e-4);
    }
}

// LTspice 26.1.1 and QSPICE 2026 agree. With NoJacob, B1's partials stay out
// of the Jacobian, so Newton iterates V(b) = 2 - V(b)^2 as a fixed point from
// 0 and settles on the root -2, where B2, with its Jacobian, finds 1. In AC a
// NoJacob source answers nothing: V(e) and V(f) are 0 where V(g) is 3.
#[test]
fn a_behavioral_source_flagged_nojacob_leaves_its_partials_out() {
    let netlist = parse(
        "* nojacob
         V1 a 0 2 AC 1
         R1 a b 1k
         B1 b 0 I=V(b)**2/1k NoJacob
         R2 a c 1k
         B2 c 0 I=V(c)**2/1k
         B3 0 d I=table(V(a), 0, 4.5m, 10, 9m) Rpar=1k Cpar=10n NoJacob
         B4 e 0 V=V(a)*3 nojacob
         R4 e 0 1k
         B5 f 0 I=V(a)*1m NoJacob
         R5 f 0 1k
         B6 g 0 V=V(a)*3
         R6 g 0 1k
         .end
",
    );
    for (node, want) in [("b", -2.0), ("c", 1.0), ("d", 5.4), ("e", 6.0), ("f", -2.0)] {
        assert_close(
            &format!("V({node})"),
            op_voltage(&netlist, node),
            want,
            1e-9,
        );
    }
    let sweep = Engine::default()
        .run_ac(&netlist, &[1e3])
        .expect("AC sweep solves");
    for (node, want) in [("e", 0.0), ("f", 0.0), ("g", 3.0)] {
        let index = sweep[0]
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node))
            .unwrap_or_else(|| panic!("V({node}) is solved"));
        assert_close(
            &format!("AC V({node})"),
            sweep[0].voltages[index].norm(),
            want,
            1e-12,
        );
    }
}

// QSPICE 2026 batch at 1 ms; LTspice 26.1.1 agrees on `DC 1+2` and
// `DC U(-prb)` and refuses the rest. `V(q)` and `V(u)` are an RC and an RL
// driven by a 1 V/ms ramp: 1 - 2(1 - e^-0.5) and 0.5(1 - e^-2).
#[test]
fn a_value_written_as_unbraced_arithmetic_reads_as_one_expression() {
    let netlist = parse(
        "unbraced arithmetic\n\
         .param prb=-1\n\
         .param Vx=5\n\
         V2 c 0 DC U(-prb)\n\
         R2 c 0 1k\n\
         V3 d 0 DC 0 DC=2\n\
         R3 d 0 1k\n\
         V4 e 0 DC DC=3\n\
         R4 e 0 1k\n\
         V5 f 0 DC 0 DC=Vx\n\
         R5 f 0 1k\n\
         V6 g 0 DC U(prb) AC 1\n\
         R6 g 0 1k\n\
         V7 h 0 DC 1+2\n\
         R7 h 0 1k\n\
         V8 k 0 1+2\n\
         R8 k 0 1k\n\
         V9 m 0 2 +1\n\
         R9 m 0 1k\n\
         I1 0 ra 1m\n\
         Ra ra 0 1k +1k\n\
         I2 0 rb 1m\n\
         Rb rb 0 1k -100\n\
         I3 0 rc 1m\n\
         Rc rc 0 1k*2\n\
         I4 0 rd 1m\n\
         Rd rd 0 {1k} * 3\n\
         I5 0 ia DC U(-prb)*1m\n\
         R10 ia 0 1k\n\
         V10 p 0 PWL(0 0 1m 1)\n\
         R11 p q 1k\n\
         C1 q 0 1u +1u\n\
         V11 s 0 PWL(0 0 1m 1)\n\
         R12 s u 1k\n\
         L1 u 0 1 -0.5\n\
         .tran 1m\n\
         .end\n",
    );
    let probes = [
        ("c", 1.0, 1e-9),
        ("d", 2.0, 1e-9),
        ("e", 3.0, 1e-9),
        ("f", 5.0, 1e-9),
        ("g", 0.0, 1e-9),
        ("h", 3.0, 1e-9),
        ("k", 3.0, 1e-9),
        ("m", 3.0, 1e-9),
        ("ra", 2.0, 1e-9),
        ("rb", 0.9, 1e-9),
        ("rc", 2.0, 1e-9),
        ("rd", 3.0, 1e-9),
        ("ia", 1.0, 1e-9),
        ("q", 0.2130615684, 1e-5),
        ("u", 0.4323321680, 1e-5),
    ];
    let nodes: Vec<&str> = probes.iter().map(|(node, _, _)| *node).collect();
    let values = tran_voltages(&netlist, 1e-3, 1e-3, &nodes);
    for ((node, expected, tolerance), actual) in probes.iter().zip(values) {
        assert_close(&format!("V({node})"), actual, *expected, *tolerance);
    }
}

// LTspice 26.1.1 and QSPICE 2026 batches agree: 5, 0.5, 10, 6 and 3.
#[test]
fn a_parameter_value_may_follow_its_name_without_an_equals_sign() {
    let netlist = parse(
        "param without equals\n\
         .param A 5\n\
         .param B table(A,0,0,10,1)\n\
         .param C {A*2}\n\
         .param D A+1 F=3\n\
         V1 a 0 {A}\n\
         R1 a 0 1k\n\
         V2 b 0 {B}\n\
         R2 b 0 1k\n\
         V3 c 0 {C}\n\
         R3 c 0 1k\n\
         V4 d 0 {D}\n\
         R4 d 0 1k\n\
         V5 e 0 {F}\n\
         R5 e 0 1k\n\
         .op\n\
         .end\n",
    );
    for (node, expected) in [("a", 5.0), ("b", 0.5), ("c", 10.0), ("d", 6.0), ("e", 3.0)] {
        assert_close(
            &format!("V({node})"),
            op_voltage(&netlist, node),
            expected,
            1e-12,
        );
    }
    let error = Netlist::parse("no value\n.param A\n.end\n").expect_err("a name alone");
    assert!(
        error
            .to_string()
            .contains("Expected '=' after parameter name 'A'"),
        "{error}"
    );
}

// QSPICE 2026: "Didn't find a model for "D" -- defaults assumed", and the
// same for "X"; both diodes sit at 0.684811103 V. LTspice refuses both.
#[test]
fn a_diode_whose_model_is_missing_takes_the_default_parameters() {
    let netlist = parse(
        "default diode\n\
         V1 a 0 1\n\
         R1 a b 100\n\
         D1 b 0 D\n\
         R2 a c 100\n\
         D2 c 0 X\n\
         .op\n\
         .end\n",
    );
    for node in ["b", "c"] {
        assert_close(
            &format!("V({node})"),
            op_voltage(&netlist, node),
            0.6848111031,
            1e-6,
        );
    }
    let error = Engine::new(SimulationConfig::default().with_spice_dialect(SpiceDialect::Ngspice))
        .run_dc_op(&netlist)
        .expect_err("ngspice refuses a diode without its model");
    assert!(
        error.to_string().contains("references unknown model 'D'"),
        "{error}"
    );
}

// QSPICE 2026 and LTspice 26.1.1 batches: a SIN stops after NCYCLES and
// holds the phase its last cycle ends on; a count that is not positive runs
// on. Past the end QSPICE lets a damped envelope decay on (V(c) is
// 1 + 2e^(-200 t)) where LTspice freezes it at 2.5576; QSPICE decides.
#[test]
fn a_sin_stops_after_its_cycle_count() {
    let netlist = parse(
        "sin ncycles\n\
         V1 a 0 SIN(0.5 1 1k 0.2m 0 30 2)\n\
         R1 a 0 1k\n\
         V2 b 0 SIN(0 1 1k 0 100 0 1.5)\n\
         R2 b 0 1k\n\
         V3 c 0 SIN(1 2 1k 0 200 0 1.25)\n\
         R3 c 0 1k\n\
         V4 d 0 SIN(0 1 1k 0 0 0 0)\n\
         R4 d 0 1k\n\
         .tran 0 4m 0 1u\n\
         .end\n",
    );
    for (time, node, expected) in [
        (0.1e-3, "a", 1.0),
        (0.6e-3, "a", 0.604528463),
        (2.5e-3, "a", 1.0),
        (0.6e-3, "b", -0.553552667),
        (2.5e-3, "b", 0.0),
        (1.6e-3, "c", 2.452298081),
        (3.7e-3, "c", 1.954227833),
        (3.7e-3, "d", -0.951056516),
    ] {
        let actual = tran_voltages(&netlist, 4e-3, time, &[node])[0];
        assert_close(&format!("V({node}) at {time:e}"), actual, expected, 1e-4);
    }
}

// LTspice 26.1.1 (QSPICE has no such function): `smallsig()` is 0 in `.op` and
// `.tran` and 1 through `.ac` and `.noise`, their operating points included.
#[test]
fn smallsig_is_one_through_ac_and_noise_only() {
    let netlist = parse(
        "* smallsig
B1 a 0 V=smallsig()+2
R1 a 0 1k
V1 in 0 0 AC 1
B2 out 0 V=V(a)*V(in)
R2 out 0 1k
.op
.end
",
    );
    assert_close("V(a) op", op_voltage(&netlist, "a"), 2.0, 1e-12);
    assert_close(
        "V(a) tran",
        tran_voltages(&netlist, 1e-3, 1e-3, &["a"])[0],
        2.0,
        1e-9,
    );
    let out = |names: &[String]| {
        names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .expect("node out")
    };
    let ac = Engine::default()
        .run_ac(&netlist, &[1e3])
        .expect("AC solves")
        .pop()
        .expect("one AC point");
    assert_close("V(out) ac", ac.voltages[out(&ac.node_names)].re, 3.0, 1e-9);
    let noise = Engine::default()
        .run_noise_named_with_input_source(&netlist, "out", None, "V1", &[1e3], 300.15)
        .expect("noise solves")
        .pop()
        .expect("one noise point");
    assert_close(
        "V(out) noise",
        noise.voltages[out(&noise.node_names)].re,
        3.0,
        1e-9,
    );
}

// LTspice 26.1.1 and QSPICE: `mc(x,y)` is a uniform draw between x*(1-y) and
// x*(1+y) (twelve steps of this deck gave R1 between 635 and 1496 ohm).
#[test]
fn mc_is_a_uniform_draw_about_its_first_argument() {
    let netlist = parse(
        "* mc
V1 a 0 1
R1 a b {mc(1k, 0.5)}
R2 b 0 1k
.op
.end
",
    );
    let r1 = 1e3 * (1.0 / op_voltage(&netlist, "b") - 1.0);
    assert!((500.0..=1500.0).contains(&r1), "R1 = {r1}");
    assert!(
        (r1 - 1e3).abs() > 1e-6,
        "R1 = {r1} is the nominal, not a draw"
    );
}

/// One `.step` run per entry: node voltage `node` from each run's operating
/// point, with each run materialized through the sweep plan.
fn step_op_voltages(netlist: &Netlist, node: &str) -> Vec<f64> {
    let engine = Engine::default();
    let plan = step_plan(&engine, netlist);
    (0..plan.total_runs())
        .map(|run| {
            let materialized = engine
                .materialize_step_run(&plan, run)
                .expect("run materializes");
            op_voltage(materialized.netlist(), node)
        })
        .collect()
}

fn step_plan<'a>(engine: &Engine, netlist: &'a Netlist) -> rspice_core::engine::StepPlan<'a> {
    let steps = netlist
        .analyses
        .iter()
        .filter_map(|analysis| match analysis {
            rspice_core::netlist::AnalysisCommand::Step(step) => Some(step.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    engine
        .plan_step_commands(
            netlist,
            &steps,
            rspice_core::engine::StepPlanLimits::from_resource_limits(
                engine.config().resource_limits,
            ),
        )
        .expect("the sweep plans")
}

// QSPICE and LTspice 26.1.1 both draw a random function afresh at every `.step`
// point: QSPICE 1.532, 0.8326, 1.156, 1.240 mA and LTspice 1.574, 0.749, 0.681,
// 1.237 mA for one resistor over four runs. RSpice's stream is its own.
#[test]
fn a_random_function_draws_again_at_every_step_point() {
    let distinct = |values: &[f64]| {
        values
            .iter()
            .enumerate()
            .all(|(i, a)| values[..i].iter().all(|b| (a - b).abs() > 1e-9))
    };
    for deck in [
        // A `.param` drawn while the deck is read.
        "* param draw\n.step param run 0 3 1\n.param r=mc(1k,0.5)\nI1 0 a 1m\nR1 a 0 {r}\n.op\n.end\n",
        // An element's value, drawn when the circuit is built.
        "* element draw\n.step param run 0 3 1\nI1 0 a 1m\nR1 a 0 {mc(1k,0.5)}\n.op\n.end\n",
        // A temperature sweep redraws too.
        "* temp sweep\n.step temp list 25 50 75 100\n.param r=mc(1k,0.5)\nI1 0 a 1m\nR1 a 0 {r}\n.op\n.end\n",
        // A sweep of a device redraws the deck's values as well.
        "* device sweep\n.step V9 list 1 2 3 4\n.param r=mc(1k,0.5)\nI1 0 a 1m\nR1 a 0 {r}\nV9 c 0 1\nR9 c 0 1k\n.op\n.end\n",
    ] {
        let netlist = parse(deck);
        let first = step_op_voltages(&netlist, "a");
        let again = step_op_voltages(&netlist, "a");
        assert_eq!(
            first, again,
            "a deck reproduces its draws run to run:\n{deck}"
        );
        assert!(
            first.iter().all(|v| (0.5..=1.5).contains(v)),
            "{first:?}\n{deck}"
        );
        assert!(
            distinct(&first),
            "every point draws again: {first:?}\n{deck}"
        );
    }
}

// The first run of a sweep is the deck's own sample, so a one-point sweep agrees
// with the deck run alone.
#[test]
fn the_first_step_point_is_the_decks_own_draw() {
    let body =
        ".param r=mc(1k,0.5)\nI1 0 a 1m\nR1 a 0 {r}\nI2 0 b 1m\nR2 b 0 {mc(1k,0.5)}\n.op\n.end\n";
    let alone = parse(&format!("* alone\n{body}"));
    let swept = parse(&format!("* one point\n.step param run list 7\n{body}"));
    let runs = step_op_voltages(&swept, "a");
    assert_eq!(runs.len(), 1);
    assert_close("first run", runs[0], op_voltage(&alone, "a"), 1e-12);
}

// A `.param` draw and an element draw are different samples: rewinding the
// stream at a build must not hand the first element the draw the deck's own
// `.param` already took. QSPICE and LTspice 26.1.1 draw them independently.
#[test]
fn an_element_draw_is_not_the_parameter_draw_before_it() {
    let netlist = parse(
        "* draws\n.param r=mc(1k,0.5)\nI1 0 a 1m\nR1 a 0 {r}\nI2 0 b 1m\nR2 b 0 {mc(1k,0.5)}\n\
         I3 0 c 1m\nR3 c 0 {mc(1k,0.5)}\n.op\n.end\n",
    );
    let (a, b, c) = (
        op_voltage(&netlist, "a"),
        op_voltage(&netlist, "b"),
        op_voltage(&netlist, "c"),
    );
    assert!((a - b).abs() > 1e-9, "R1 and R2 share a draw: {a} {b}");
    assert!((b - c).abs() > 1e-9, "R2 and R3 share a draw: {b} {c}");
    // Rebuilding draws the same values again.
    assert_close("rebuild", op_voltage(&netlist, "a"), a, 1e-15);
    assert_close("rebuild", op_voltage(&netlist, "b"), b, 1e-15);
}

// The replay sensitivity runs on one stepped circuit perturbs a parameter and
// must see that run's draws unchanged, or the difference it takes would hold
// the random change of another sample.
#[test]
fn a_parameter_replay_of_a_step_run_keeps_that_runs_draws() {
    let netlist = parse(
        "* draws\n.step param run 0 3 1\n.param p=1k draw=mc(1k,0.5)\nI1 0 a 1m\nR1 a 0 {draw}\n\
         I2 0 b 1m\nR2 b 0 {mc(1k,0.5)}\nI3 0 c 1m\nR3 c 0 {p}\n.op\n.end\n",
    );
    let engine = Engine::default();
    let plan = step_plan(&engine, &netlist);
    let nominal: Vec<_> = (0..plan.total_runs())
        .map(|run| engine.materialize_step_run(&plan, run).unwrap())
        .collect();
    let mut seen = Vec::new();
    for run in &nominal {
        let (a, b) = (
            op_voltage(run.netlist(), "a"),
            op_voltage(run.netlist(), "b"),
        );
        seen.push((a, b));
        let (replayed, _) =
            Engine::create_perturbed_netlist_multi(run.netlist(), &[("P".to_owned(), 2000.0)])
                .expect("replay");
        assert_eq!(
            op_voltage(&replayed, "a"),
            a,
            "the .param draw changed on replay"
        );
        assert_eq!(
            op_voltage(&replayed, "b"),
            b,
            "the element draw changed on replay"
        );
        assert_close("perturbed parameter", op_voltage(&replayed, "c"), 2.0, 1e-9);
    }
    assert!(
        seen.windows(2).all(|pair| pair[0] != pair[1]),
        "the runs drew different samples: {seen:?}"
    );
}

// QSPICE and LTspice 26.1.1 sweep a `.ac list` or `.noise ... list` in ascending
// order; QSPICE keeps a repeated point (LTspice drops it). Values are QSPICE's.
#[test]
fn ac_and_noise_take_a_list_of_frequencies() {
    let netlist = parse(
        "* ac list
V1 in 0 0 AC 1
R1 in out 1k
C1 out 0 1u
.ac list 1k 1 {2*5} 1k
.end
",
    );
    let (_, points) = Engine::default()
        .run_ac_data(&netlist, &netlist.data_tables[0].name)
        .expect("AC solves");
    let frequencies = points
        .iter()
        .map(|point| point.frequency)
        .collect::<Vec<_>>();
    assert_eq!(frequencies, [1.0, 10.0, 1e3, 1e3]);
    let index = points[0]
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("node out");
    let at_1k = points[2].voltages[index];
    assert_close("Re V(out)", at_1k.re, 2.470452303185767e-2, 1e-12);
    assert_close("Im V(out)", at_1k.im, -1.552230961346476e-1, 1e-12);

    let netlist = parse(
        "* noise list
V1 in 0 0 AC 1
R1 in out 1k
R2 out 0 2k
C1 out 0 1u
.noise V(out) V1 list 1 10 1k 100k
.end
",
    );
    let (_, points) = Engine::default()
        .run_noise_data_named_with_input_source(
            &netlist,
            "out",
            None,
            "V1",
            &netlist.data_tables[0].name,
            300.15,
        )
        .expect("noise solves");
    assert_eq!(points.len(), 4);
    for (point, expected) in points.iter().zip([
        1.105052070396838e-17,
        1.10313590138714e-17,
        5.958555168984793e-19,
        6.298116196694071e-23,
    ]) {
        assert_close(
            &format!("onoise at {} Hz", point.frequency),
            point.output_noise_density,
            expected,
            1e-3 * expected,
        );
    }
}

// QSPICE: a trailing `shorted` flag changes nothing, V(b) = 0.5 (LTspice refuses
// the card, and QSPICE decides).
#[test]
fn a_resistor_flagged_shorted_is_solved_as_written() {
    let netlist = parse(
        "* shorted flag
V1 a 0 1
R1 a b 1k shorted
R2 b 0 1k
.op
.end
",
    );
    assert_close("V(b)", op_voltage(&netlist, "b"), 0.5, 1e-12);
}

fn assert_exp_samples(card: &str, options: &str, stop: f64, samples: &[(f64, f64)]) {
    let netlist = parse(&format!(
        "exp train\nV1 a 0 {card}\nR1 a 0 1k\n.tran 0 {stop}\n{options}\n.end\n"
    ));
    for &(time, expected) in samples {
        let value = tran_voltages(&netlist, stop, time, &["a"])[0];
        assert_close(&format!("{card} at {time}"), value, expected, 4e-4);
    }
}

// QSPICE64 build Oct 27 2026: each repeat picks up on the first pulse's curve.
#[test]
fn a_seventh_exp_value_repeats_it_the_way_qspice_does() {
    assert_exp_samples(
        "EXP(0 1 1u 1u 5u 2u 7u)",
        "",
        30e-6,
        &[
            (2e-6, 0.632118),
            (7e-6, 0.604053),
            (10e-6, 0.914328),
            (12e-6, 0.988406),
            (13e-6, 0.791368),
            (14e-6, 0.481007),
            (16e-6, 0.767120),
            (26e-6, 0.988406),
        ],
    );
    assert_exp_samples(
        "EXP(0 1 1u 1u 5u 2u 20u)",
        "",
        30e-6,
        &[
            (22e-6, 0.632324),
            (25e-6, 0.981694),
            (26e-6, 0.992989),
            (28e-6, 0.366866),
        ],
    );
}

// QSPICE64 build Oct 27 2026: three pulses to a train, one train every 50 us.
#[test]
fn a_counted_exp_train_repeats_the_way_qspice_does() {
    assert_exp_samples(
        "EXP(0 1 1u 2u 3u 40u 10u 3 50u)",
        "",
        90e-6,
        &[
            (12e-6, 0.859901),
            (15e-6, 0.869259),
            (20e-6, 0.785238),
            (52e-6, 0.675877),
            (55e-6, 0.872904),
            (63e-6, 0.875818),
            (65e-6, 0.852288),
            (80e-6, 0.761825),
        ],
    );
}

// LTspice 26.1.1: every pulse adds to the tails of the earlier ones, with the
// option standing anywhere in the deck.
#[test]
fn the_ltspice_exp_repeat_adds_each_pulse_to_the_earlier_tails() {
    let ltspice = ".options rspice_exp_repeat=ltspice";
    assert_exp_samples(
        "EXP(0 1 1u 1u 5u 2u 7u)",
        ltspice,
        30e-6,
        &[
            (2e-6, 0.632116),
            (6e-6, 0.599795),
            (8e-6, 0.222218),
            (9e-6, 0.767116),
            (10e-6, 0.946625),
            (12e-6, 1.011865),
            (13e-6, 0.618104),
            (14e-6, 0.376509),
            (16e-6, 0.771203),
            (19e-6, 1.012777),
        ],
    );
    assert_exp_samples(
        "EXP(0 1 1u 1u 5u 2u 20u)",
        ltspice,
        30e-6,
        &[
            (22e-6, 0.632319),
            (25e-6, 0.98173),
            (26e-6, 0.599822),
            (28e-6, 0.222229),
        ],
    );
    assert_exp_samples(
        "EXP(0 1 1u 2u 3u 40u 10u 3 50u)",
        ltspice,
        35e-6,
        &[
            (5e-6, 0.8159),
            (10e-6, 0.8283),
            (15e-6, 1.5558),
            (20e-6, 1.4820),
            (25e-6, 2.1327),
            (30e-6, 1.9912),
        ],
    );
}

#[test]
fn the_exp_repeat_option_names_what_it_accepts() {
    let refusal = Netlist::parse(
        "exp train\nV1 a 0 1\nR1 a 0 1k\n.options rspice_exp_repeat=spectre\n.op\n.end\n",
    )
    .expect_err("an unknown reading is refused");
    assert!(
        refusal.to_string().contains("RSPICE_EXP_REPEAT"),
        "{refusal}"
    );
}

/// The collector current of `Z1 c g 0 {instance}` from an operating point.
fn igbt_collector_current(model: &str, instance: &str, extra: &str, vc: f64, vg: f64) -> f64 {
    let deck = format!(
        "transistor\nVc c 0 {vc}\nVg g 0 {vg}\nZ1 c g 0 {instance}\n.model N1 {model}\n\
         B1 i 0 V=-I(Vc)\nR1 i 0 1\n{extra}\n.op\n.end\n"
    );
    op_voltage(&parse(&deck), "i")
}

fn assert_igbt_current(label: &str, actual: f64, expected: f64, relative: f64) {
    assert_close(label, actual, expected, relative * expected.abs());
}

// LTspice 26.1.1 batch runs of the same operating points.
#[test]
fn an_igbt_follows_ltspice_across_its_regions() {
    for (vc, vg, ic) in [
        (0.5, 10.0, 0.00707467),
        (1.0, 10.0, 1.51923),
        (2.5, 10.0, 6.52478),
        (5.0, 10.0, 10.673),
        (10.0, 10.0, 11.3134),
        (25.0, 10.0, 11.6605),
        (2.5, 15.0, 10.8611),
        (10.0, 15.0, 31.1723),
        (5.0, 20.0, 29.1056),
        (25.0, 20.0, 59.6124),
        (10.0, 4.66, 7.805096e-6),
        (10.0, 4.70, 2.278354e-4),
        (10.0, 5.0, 1.012380e-1),
        (40.0, 0.0, 3.907555e-7),
        (200.0, 0.0, 1.541254e-6),
        (100.0, 8.0, 6.481147),
        (700.0, 8.0, 55.87534),
    ] {
        assert_igbt_current(
            &format!("Vc {vc}, Vg {vg}"),
            igbt_collector_current("NIGBT", "N1", "", vc, vg),
            ic,
            4e-3,
        );
    }
}

// LTspice 26.1.1: a positional area replaces the card's AREA, and m multiplies.
#[test]
fn an_igbt_area_and_multiplier_follow_ltspice() {
    for (vc, ic) in [(0.5, 1.888948), (1.5, 10.61203), (2.5, 17.97035)] {
        assert_igbt_current(
            &format!("area 2, Vc {vc}"),
            igbt_collector_current("NIGBT", "N1 2", "", vc, 10.0),
            ic,
            4e-3,
        );
    }
    assert_igbt_current(
        "m=2",
        igbt_collector_current("NIGBT", "N1 m=2", "", 1.5, 10.0),
        6.993762,
        4e-3,
    );
}

// LTspice 26.1.1: a P device is the mirror of its N twin.
#[test]
fn a_pigbt_mirrors_ltspice() {
    assert_igbt_current(
        "PIGBT",
        igbt_collector_current("PIGBT", "N1", "", -1.5, -10.0),
        -3.496881,
        4e-3,
    );
}

// LTspice 26.1.1 at .temp 125 and .temp -40.
#[test]
fn an_igbt_follows_ltspice_over_temperature() {
    for (celsius, vc, vg, ic) in [
        (125.0, 1.0, 10.0, 0.53938),
        (125.0, 2.5, 10.0, 5.0777),
        (125.0, 25.0, 10.0, 9.9978),
        (-40.0, 1.0, 10.0, 2.5044),
        (-40.0, 2.5, 10.0, 8.3195),
        (-40.0, 25.0, 10.0, 14.05),
    ] {
        assert_igbt_current(
            &format!("{celsius} C, Vc {vc}"),
            igbt_collector_current("NIGBT", "N1", &format!(".temp {celsius}"), vc, vg),
            ic,
            8e-3,
        );
    }
}

// LTspice 26.1.1: a 10 ohm gate drive, 1 ohm collector load, gate driven by AC.
#[test]
fn an_igbt_small_signal_gain_and_input_follow_ltspice() {
    let netlist = parse(
        "gain\nVc vc 0 10\nRc vc c 1\nVg gp 0 10 AC 1\nRg gp g 10\nZ1 c g 0 N1\n.model N1 NIGBT\n\
         .ac list 1k 1Meg\n.end\n",
    );
    let sweep = Engine::default()
        .run_ac(&netlist, &[1e3, 1e6])
        .expect("AC sweep solves");
    for (point, node, re, im, tolerance) in [
        (0, "c", -0.353416, 3.98149e-4, 1e-5),
        (0, "g", 1.0, -1.8883e-4, 1e-5),
        (1, "c", -0.281022, 0.0848514, 1e-3),
        (1, "g", 0.964253, -0.175703, 1e-3),
    ] {
        let result = &sweep[point];
        let index = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node))
            .unwrap_or_else(|| panic!("V({node}) is solved"));
        let voltage = result.voltages[index];
        assert_close(&format!("Im V({node}) #{point}"), voltage.im, im, tolerance);
        assert_close(&format!("Re V({node}) #{point}"), voltage.re, re, 1e-3);
    }
}

// LTspice 26.1.1: a clamped inductive load switched by a 10 ohm gate drive.
#[test]
fn an_igbt_switches_an_inductor_as_ltspice_does() {
    let netlist = parse(
        "switching\nZ1 c g 0 N1\nRg gp g 10\nVg gp 0 PULSE(0 15 1u 20n 20n 5u 20u)\nVcc vcc 0 300\n\
         L1 vcc c 100u\nD1 c vcc Dx\n.model Dx D Ron=1m Roff=1Meg Vfwd=.8\n.model N1 NIGBT\n\
         B1 i 0 V=I(L1)\nR1 i 0 1\n.end\n",
    );
    let times = [1.5e-6, 4e-6, 6e-6, 6.2e-6, 6.3e-6, 8e-6];
    let voltages = tran_samples(&netlist, 12e-6, 2e-9, &times, "c");
    let currents = tran_samples(&netlist, 12e-6, 2e-9, &times, "i");
    for ((time, voltage), (expected, tolerance)) in times.iter().zip(&voltages).zip([
        (1.27579, 0.01),
        (2.39182, 0.01),
        (3.51922, 0.02),
        (132.568, 3.0),
        (225.178, 3.0),
        (300.812, 0.2),
    ]) {
        assert_close(&format!("V(c) at {time}"), *voltage, expected, tolerance);
    }
    for ((time, current), expected) in times
        .iter()
        .zip(&currents)
        .zip([1.43373, 8.88762, 14.8289, 15.3344, 15.4541, 15.4772])
    {
        assert_close(&format!("I(L1) at {time}"), *current, expected, 0.02);
    }
}

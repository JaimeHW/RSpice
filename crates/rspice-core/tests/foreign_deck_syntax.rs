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

//! LTspice special-function `A` devices against LTspice.
//!
//! Reference values are LTspice 26.1.1 batch runs of the same decks.

use rspice_core::Complex64;
use rspice_core::analysis::NoiseContributionProbe;
use rspice_core::analysis::ac::AcResult;
use rspice_core::engine::Engine;
use rspice_core::netlist::{ElementKind, Netlist, SpecialFunction};

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}\n{deck}"))
}

fn ac_voltage(point: &AcResult, node: &str) -> Complex64 {
    let index = point
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("missing node {node:?} in {:?}", point.node_names));
    point.voltages[index]
}

fn assert_close(label: &str, actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{label}: {actual:.9e} vs LTspice {expected:.9e} (tolerance {tolerance:e})"
    );
}

fn build_error(deck: &str) -> String {
    match Engine::default().build_circuit(&parse(deck)) {
        Ok(_) => panic!("deck builds:\n{deck}"),
        Err(error) => error.to_string(),
    }
}

#[test]
fn an_eight_terminal_line_naming_a_special_function_is_the_native_device() {
    let netlist = parse("* routing\nA1 0 in 0 0 0 0 out 0 ota g=1m iout=1 linear\n.end\n");
    let element = &netlist.elements[0];
    let ElementKind::SpecialFunction {
        function,
        instance_params,
        ..
    } = &element.kind
    else {
        panic!("not a special function: {:?}", element.kind);
    };
    assert_eq!(*function, SpecialFunction::Ota);
    assert_eq!(element.nodes.len(), 8);
    assert!(instance_params.contains(&("LINEAR".to_string(), 1.0)));

    // The same shape naming an XSPICE `.model` card stays an XSPICE line.
    let xspice = parse("* xspice\nA1 1 2 3 4 5 6 7 8 buf\n.model buf d_buffer\n.end\n");
    assert!(
        matches!(xspice.elements[0].kind, ElementKind::Xspice { .. }),
        "{:?}",
        xspice.elements[0].kind
    );
}

#[test]
fn ota_operating_points_match_ltspice() {
    let deck = "* ota operating points
V1 in 0 10m
Vm m 0 2
A1 0 in 0 0 0 0 o1 0 OTA G=1m Iout=10u Vhigh=5 Vlow=-5
R1 o1 0 1k
A2 in 0 0 0 0 0 o2 0 OTA G=1m Iout=10u
R2 o2 0 1Meg
A3 0 in m 0 0 0 o3 0 OTA G=1m Iout=10u Vhigh=5 Vlow=-5
R3 o3 0 1k
A4 0 in 0 0 0 0 o4 0 OTA G=1m Iout=10u Isink=-2u Vhigh=5 Vlow=-5
R4 o4 0 1k
A5 0 in 0 0 0 0 o5 0 OTA G=1m Iout=10u Vhigh=2 Rclamp=1k epsilon=0.5
R5 o5 0 1Meg
A6 0 in 0 0 0 0 o6 0 OTA G=1m Iout=10u ref=5m linear Vhigh=5 Vlow=-5
R6 o6 0 1k
A7 0 in 0 0 0 0 o7 0 OTA G=1m Iout=10u Vhigh=5 Vlow=-5 Rout=100k
.op
.end
";
    let op = Engine::default()
        .run_dc_op(&parse(deck))
        .expect("OP solves");
    let v = |node: &str| op.try_voltage_named(node).unwrap();
    // Iout·tanh(G·vd/Iout) into 1k.
    assert_close("tanh drive", v("o1"), 7.615_941_551_9e-3, 1e-9);
    // Negative drive held at Vlow = 0 by the 1-ohm clamp.
    assert_close("low clamp", v("o2"), -7.615_933_943_6e-6, 1e-10);
    // The multiplier scales the argument, not the limit.
    assert_close("multiplier", v("o3"), 9.640_275_791_1e-3, 1e-9);
    // Isink moves the centre of the tanh.
    assert_close("isink", v("o4"), 9.586_657_642_4e-3, 1e-9);
    // The epsilon-softened 1k clamp above Vhigh = 2.
    assert_close("soft clamp", v("o5"), 2.074_441_240_2, 1e-6);
    // Linear transfer with a 5 mV offset is unbounded by Iout.
    assert_close("linear offset", v("o6"), 0.015, 1e-9);
    // Rout alone loads the output.
    assert_close("rout", v("o7"), 0.761_594_155_96, 1e-7);
}

#[test]
fn ota_small_signal_gain_rolls_off_on_rout_and_cout() {
    let deck = "* ota ac
V1 in 0 0 AC 1
A1 0 in 0 0 0 0 out 0 OTA G=1m Iout=20u Vhigh=10 Vlow=-10 Rout=10k Cout=1n
.end
";
    let netlist = parse(deck);
    let engine = Engine::default();
    let frequencies = [1.0, 1e4, 1e6];
    let ac = engine.run_ac(&netlist, &frequencies).expect("AC solves");
    // LTspice 26.1.1 batch: 9.99999998, 8.46733016, 0.159134790.
    for (point, expected) in ac.iter().zip([9.999_999_98, 8.467_330_16, 0.159_134_790]) {
        assert_close(
            "|H|",
            ac_voltage(point, "out").norm(),
            expected,
            2e-6 * expected,
        );
    }
    let phase = ac_voltage(&ac[1], "out").arg().to_degrees();
    let expected = -(2.0 * std::f64::consts::PI * 1e4 * 1e-5)
        .atan()
        .to_degrees();
    assert_close("phase at the pole", phase, expected, 1e-6);
}

#[test]
fn ota_refuses_what_ltspice_does_not_define() {
    let unknown = build_error("* bad\nA1 0 in 0 0 0 0 out 0 OTA gain=2\nR1 out 0 1k\n.end\n");
    assert!(unknown.contains("GAIN"), "{unknown}");
    let rail = build_error("* bad\nA1 0 in 0 0 vn 0 out 0 OTA\nR1 out 0 1k\n.end\n");
    assert!(rail.contains("terminal 5"), "{rail}");
    let window = build_error("* bad\nA1 0 in 0 0 0 0 out 0 OTA Vhigh=0 Vlow=1\n.end\n");
    assert!(window.contains("Vhigh above Vlow"), "{window}");
}

fn tran_at(deck: &str, stop: f64, step: f64, probes: &[(&str, f64)]) -> Vec<f64> {
    let tran = Engine::default()
        .run_tran(&parse(deck), stop, step)
        .expect("transient solves");
    probes
        .iter()
        .map(|(node, time)| {
            let wave = tran
                .try_voltage_waveform_named(node)
                .unwrap_or_else(|| panic!("no waveform for {node}"));
            let index = tran.time.partition_point(|t| t < time).max(1);
            let (t0, t1) = (tran.time[index - 1], tran.time[index]);
            let fraction = if t1 > t0 {
                (time - t0) / (t1 - t0)
            } else {
                0.0
            };
            wave[index - 1] + fraction * (wave[index] - wave[index - 1])
        })
        .collect()
}

#[test]
fn gate_operating_points_match_ltspice() {
    let deck = "* gate operating points
Va a 0 1
Vb b 0 0
Vc c 0 1
Vx x 0 2.5
Vcm cm 0 1
A1 a b 0 0 0 qb1 q1 0 AND
A2 a b 0 0 0 qb2 q2 0 OR
A3 a b c 0 0 qb3 q3 0 XOR
A4 a b 0 0 0 qb4 q4 0 XOR
A5 b 0 0 0 0 qb5 q5 0 BUF
A6 x cm cm cm cm qb6 q6 cm BUF Vhigh=3 Vlow=-1 Rout=10
R6 q6 0 1k
R6b qb6 0 1k
A7 x 0 0 0 0 0 q7 0 SCHMITT Vt=2 Vh=0.2 Rhigh=50
R7 q7 0 50
.op
.end
";
    let op = Engine::default()
        .run_dc_op(&parse(deck))
        .expect("OP solves");
    let v = |node: &str| op.try_voltage_named(node).unwrap();
    for (node, expected) in [
        ("q1", 0.0),
        ("qb1", 1.0),
        ("q2", 1.0),
        ("q3", 0.0),
        ("q4", 1.0),
        ("q5", 0.0),
        ("qb5", 1.0),
        // Levels and threshold sit above the common at 1 V; 10 ohms into 1k.
        ("q6", 3.960_396_039_6),
        ("qb6", 0.0),
        // Rhigh = 50 into 50 ohms.
        ("q7", 0.5),
    ] {
        assert_close(node, v(node), expected, 1e-9);
    }
}

#[test]
fn gate_delay_slew_and_output_rc_match_ltspice() {
    let deck = "* gate transients
V1 in 0 PWL(0 0 10n 0 10.001n 1 50n 1 50.001n 0 70n 0 70.001n 1 72n 1 72.001n 0)
A1 in 0 0 0 0 0 q1 0 BUF Td=5n
A2 in 0 0 0 0 0 q2 0 BUF Trise=20n
A3 in 0 0 0 0 qb3 q3 0 BUF Trise=10n Tfall=4n
A4 in 0 0 0 0 0 q4 0 BUF Tau=10n Rout=1k
R4 q4 0 1k
A5 in 0 0 0 0 0 q5 0 BUF Rhigh=100 Rlow=10 Vlow=-1 Ref=0.5
R5 q5 0 100
.end
";
    let probes = [
        // Td = 5 ns, and a 2 ns pulse is shorter than the hold time.
        ("q1", 14.9e-9, 0.0),
        ("q1", 15.1e-9, 1.0),
        ("q1", 73e-9, 0.0),
        ("q1", 78e-9, 0.0),
        // Trise = 20 ns: a linear ramp to 95 %, then tau = 1 ns.
        ("q2", 20e-9, 0.499_96),
        ("q2", 29e-9, 0.949_96),
        ("q2", 31e-9, 0.993_229),
        ("q2", 60e-9, 0.500_042),
        // Trise = 10 ns, Tfall = 4 ns, one settling constant.
        ("q3", 20e-9, 0.989_495),
        ("q3", 52e-9, 0.500_213),
        ("qb3", 52e-9, 0.199_915),
        // Tau = 10 ns behind 1k into 1k: 5 ns.
        ("q4", 15e-9, 0.316_043),
        ("q4", 20e-9, 0.432_326),
        ("q4", 60e-9, 0.067_651_7),
        // Rhigh and Rlow follow the output level.
        ("q5", 40e-9, 0.5),
        ("q5", 90e-9, -100.0 / 110.0),
    ];
    let points: Vec<(&str, f64)> = probes.iter().map(|(n, t, _)| (*n, *t)).collect();
    let values = tran_at(deck, 100e-9, 0.02e-9, &points);
    for ((node, time, expected), actual) in probes.iter().zip(values) {
        assert_close(&format!("{node} at {time:e}"), actual, *expected, 2e-3);
    }
}

#[test]
fn schmitt_triggers_trip_where_ltspice_does() {
    let deck = "* schmitt trips
V1 in 0 PWL(0 0 100n 3 200n 0)
Vb b 0 2
A1 in 0 0 0 0 0 q1 0 SCHMITT Vhigh=3
A2 in 0 0 0 0 qb2 0 0 SCHMITT Vhigh=3 Vt=1
A3 b in 0 0 0 0 q3 0 SCHMITT Vt=0.6 Vh=0.3
.end
";
    let probes = [
        // Defaults Vt = 1.5, Vh = Vt/20: trips at 1.575 V (52.5 ns) and
        // 1.425 V (152.5 ns).
        ("q1", 52.3e-9, 0.0),
        ("q1", 52.8e-9, 3.0),
        ("q1", 152.3e-9, 3.0),
        ("q1", 152.8e-9, 0.0),
        // Vt = 1, Vh = 0.05 on the inverted output: 35 ns and 168.33 ns.
        ("qb2", 34.8e-9, 3.0),
        ("qb2", 35.3e-9, 0.0),
        ("qb2", 168.1e-9, 0.0),
        ("qb2", 168.6e-9, 3.0),
        // Differential: V(b) - V(in) falls through 0.3 V at 56.67 ns and
        // rises through 0.9 V at 163.33 ns.
        ("q3", 56.4e-9, 1.0),
        ("q3", 56.9e-9, 0.0),
        ("q3", 163.1e-9, 0.0),
        ("q3", 163.6e-9, 1.0),
    ];
    let points: Vec<(&str, f64)> = probes.iter().map(|(n, t, _)| (*n, *t)).collect();
    let values = tran_at(deck, 200e-9, 0.05e-9, &points);
    for ((node, time, expected), actual) in probes.iter().zip(values) {
        assert_close(&format!("{node} at {time:e}"), actual, *expected, 1e-3);
    }
}

fn ota_noise_density(deck: &str, mechanism: &str, frequencies: &[f64]) -> Vec<f64> {
    Engine::default()
        .run_noise_named_with_input_source(&parse(deck), "out", None, "V1", frequencies, 300.15)
        .expect("noise solves")
        .iter()
        .map(|point| {
            let probe = NoiseContributionProbe::parse(&format!("DNO(A1,{mechanism})")).unwrap();
            point
                .contribution(&probe)
                .expect("OTA mechanism reported")
                .sqrt()
        })
        .collect()
}

#[test]
fn ota_input_noise_matches_ltspice() {
    let voltage = "* ota en
V1 in 0 0 AC 1
A1 0 in 0 0 0 0 out 0 OTA G=1m Iout=20u Vhigh=10 Vlow=-10 Rout=10k en=10n enk=100
.end
";
    // LTspice 26.1.1 batch, V(A1.en): en*sqrt(1 + enk/f) through G*Rout = 10.
    let densities = ota_noise_density(voltage, "en", &[1.0, 10.0, 1e4]);
    for (actual, expected) in densities
        .into_iter()
        .zip([1.004_988e-6, 3.316_625e-7, 1.004_988e-7])
    {
        assert_close("en", actual, expected, 1e-6 * expected);
    }
    let current = "* ota in
V1 s 0 0 AC 1
Ra a 0 1k
Rb b s 3k
A1 a b 0 0 0 0 out 0 OTA G=1m Iout=20u Vhigh=10 Vlow=-10 Rout=10k in=1p ink=10
.end
";
    // LTspice 26.1.1 batch, V(A1.in): in*sqrt(1 + ink/f) across Ra + Rb, times 10.
    let densities = ota_noise_density(current, "in", &[1.0, 1e4]);
    for (actual, expected) in densities.into_iter().zip([1.326_650e-7, 4.001_999e-8]) {
        assert_close("in", actual, expected, 1e-6 * expected);
    }
}

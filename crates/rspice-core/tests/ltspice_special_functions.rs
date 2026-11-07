//! LTspice special-function `A` devices against LTspice.
//!
//! Reference values are LTspice 26.1.1 batch runs of the same decks. QSPICE
//! cannot read these `A` lines, so where it votes it runs an equivalent
//! circuit; LTspice alone decides each device's parameter defaults.

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
    assert!(
        rail.contains("terminal 5 is connected to node VN")
            && rail.contains("terminals 5 and 6 on the common node"),
        "{rail}"
    );
    let window = build_error("* bad\nA1 0 in 0 0 0 0 out 0 OTA Vhigh=0 Vlow=1\n.end\n");
    assert!(window.contains("Vhigh above Vlow"), "{window}");
    let varying = build_error("* bad\nA1 0 in 0 0 0 0 out 0 OTA g=1m*(1+freq)\n.end\n");
    assert!(varying.contains("parameter 'G'"), "{varying}");
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
    // QSPICE's HMITT device with the same thresholds trips at 52.50 and
    // 152.50 ns, 35.00 and 168.33 ns, and 56.67 and 163.33 ns.
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

#[test]
fn schmitt_reference_and_default_hysteresis_trip_where_ltspice_does() {
    // The input ramps from -3 V to 3 V over 4 ms and back over the next 4.
    let deck = "* schmitt reference and default hysteresis
V1 in 0 PWL(0 -3 4m 3 8m -3)
A1 in 0 0 0 0 0 o1 0 SCHMITT Vt=-1
A2 in 0 0 0 0 0 o2 0 SCHMITT Vt=2
A3 in 0 0 0 0 0 o3 0 SCHMITT Ref=2
A4 in 0 0 0 0 0 o4 0 SCHMITT Vt=1 Ref=2 Vh=0.1
A5 in 0 0 0 0 0 o5 0 OR Ref=0.5 Vh=0.2
.end
";
    // LTspice 26.1.1 trips: the default hysteresis is |Vt|/20, `Ref` is the
    // threshold when `Vt` is absent, and a gate switches at `Ref` with no
    // hysteresis though it accepts `Vh`.
    let trips = [
        ("o1", -0.95, -1.05),
        ("o2", 2.1, 1.9),
        ("o3", 2.1, 1.9),
        ("o4", 1.1, 0.9),
        ("o5", 0.5, 0.5),
    ];
    let rising = |v: f64| (v + 3.0) / 1.5e3;
    let falling = |v: f64| 4e-3 + (3.0 - v) / 1.5e3;
    let mut probes = Vec::new();
    for (node, up, down) in trips {
        probes.push((node, rising(up - 0.02), 0.0));
        probes.push((node, rising(up + 0.02), 1.0));
        probes.push((node, falling(down + 0.02), 1.0));
        probes.push((node, falling(down - 0.02), 0.0));
    }
    let points: Vec<(&str, f64)> = probes.iter().map(|(n, t, _)| (*n, *t)).collect();
    let values = tran_at(deck, 8e-3, 1e-6, &points);
    for ((node, time, expected), actual) in probes.iter().zip(values) {
        assert_close(&format!("{node} at {time:e}"), actual, *expected, 1e-3);
    }
}

fn ota_noise_density(deck: &str, mechanism: &str, frequencies: &[f64]) -> Vec<f64> {
    device_noise_density(deck, "A1", mechanism, frequencies)
}

fn device_noise_density(
    deck: &str,
    device: &str,
    mechanism: &str,
    frequencies: &[f64],
) -> Vec<f64> {
    Engine::default()
        .run_noise_named_with_input_source(&parse(deck), "out", None, "V1", frequencies, 300.15)
        .expect("noise solves")
        .iter()
        .map(|point| {
            let probe =
                NoiseContributionProbe::parse(&format!("DNO({device},{mechanism})")).unwrap();
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
    // QSPICE, with a 6.0328k resistor's thermal noise for en into an ideal
    // transconductor and a noiseless load: 1.000e-7 white, as here.
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
    // QSPICE, with a 16.576k resistor between the inputs for in (its loading
    // cancelled) and noiseless Ra, Rb and load: 4.000e-8 white, as here.
    let densities = ota_noise_density(current, "in", &[1.0, 1e4]);
    for (actual, expected) in densities.into_iter().zip([1.326_650e-7, 4.001_999e-8]) {
        assert_close("in", actual, expected, 1e-6 * expected);
    }
}

/// A deck, the device and noise mechanism it probes, the frequencies probed
/// and LTspice's densities there.
type NoiseDensityCase = (&'static str, &'static str, &'static str, [f64; 3], [f64; 3]);

#[test]
fn ota_noise_densities_written_as_expressions_of_frequency_match_ltspice() {
    // LTspice 26.1.1 batch, V(A1.en) and V(x1:A1.in), each density through
    // G*Rout = 10. LTspice reads a value across blanks inside parentheses or
    // beside an operator, evaluates it at every frequency point, keeps the
    // flicker corner on top, and reads node voltages at the operating point.
    let cases: [NoiseDensityCase; 4] = [
        (
            "* en of freq, written with blanks
V1 in 0 0 AC 1
A1 0 in 0 0 0 0 out 0 OTA G=1m Iout=20u Vhigh=10 Vlow=-10 Rout=10k en=10n+ 20n*min(freq,1k)/ (1k+ freq) Vlow=-10
.end
",
            "A1",
            "en",
            [1.0, 1e3, 1e4],
            [1.001_998e-7, 2.0e-7, 1.181_818e-7],
        ),
        (
            "* en of freq under a flicker corner
V1 in 0 0 AC 1
A1 0 in 0 0 0 0 out 0 OTA G=1m Iout=20u Vhigh=10 Vlow=-10 Rout=10k en=10n*(1+freq/1k) enk=100
.end
",
            "A1",
            "en",
            [1.0, 10.0, 1e5],
            [1.005_993e-6, 3.349_791e-7, 1.010_505e-5],
        ),
        (
            "* in as a table of freq, inside a subcircuit
V1 s 0 0 AC 1
Ra a 0 1k
Rb b s 3k
X1 a b out amp ia=1p
.subckt amp a b out ia=2p
.param ib=3
A1 a b 0 0 0 0 out 0 OTA G=1m Iout=20u Vhigh=10 Vlow=-10 Rout=10k in=table(freq, 10, {ia}, 10k, {ia*ib})
.ends
.end
",
            "X1.A1",
            "in",
            [1.0, 100.0, 1e4],
            [4.0e-8, 4.072_072e-8, 1.2e-7],
        ),
        (
            "* en reading a node voltage
V1 in 0 0 AC 1
V2 b 0 2
A1 0 in 0 0 0 0 out 0 OTA G=1m Iout=20u Vhigh=10 Vlow=-10 Rout=10k en=V(b)*5n/(freq**0.5)
.end
",
            "A1",
            "en",
            [1.0, 100.0, 1e5],
            [1.0e-7, 1.0e-8, 3.162_278e-10],
        ),
    ];
    for (deck, device, mechanism, frequencies, expected) in cases {
        let densities = device_noise_density(deck, device, mechanism, &frequencies);
        for ((actual, expected), frequency) in densities.into_iter().zip(expected).zip(frequencies)
        {
            assert_close(
                &format!("{device} {mechanism} at {frequency} Hz"),
                actual,
                expected,
                2e-6 * expected,
            );
        }
    }
}

#[test]
fn flip_flops_and_counters_hold_their_initial_state_in_the_operating_point() {
    let deck = "* sequential operating points
Vh hi 0 1
Vh5 hi5 0 5
A1 hi 0 0 0 0 qb1 q1 0 SRFLOP
A2 0 0 0 hi 0 qb2 q2 0 DFLOP
A3 hi 0 hi 0 0 qb3 q3 0 DFLOP Vhigh=3 Vlow=-1 Rout=10
R3 q3 0 1k
A4 hi5 0 0 0 0 p2 p1 0 COUNTER cycles=4 Vhigh=5 Rhigh=100
R4 p1 0 100
A5 hi 0 0 0 0 p2b p1b 0 COUNTER cycles=0
.op
.end
";
    let op = Engine::default()
        .run_dc_op(&parse(deck))
        .expect("OP solves");
    let v = |node: &str| op.try_voltage_named(node).unwrap();
    // LTspice 26.1.1 batch: S and PRE wait for the first time step.
    for (node, expected) in [
        ("q1", 0.0),
        ("qb1", 1.0),
        ("q2", 0.0),
        ("qb2", 1.0),
        ("q3", -1000.0 / 1010.0),
        ("qb3", 3.0),
        ("p1", 2.5),
        ("p2", 0.0),
        ("p1b", 0.0),
        ("p2b", 0.0),
    ] {
        assert_close(node, v(node), expected, 1e-9);
    }
}

#[test]
fn flip_flops_and_counters_switch_where_ltspice_does() {
    let deck = "* flip-flops and counters
Vc clk 0 PULSE(0 1 5n 1p 1p 5n 10n)
Vs s 0 PWL(0 0 20n 0 20.01n 1 22n 1 22.01n 0 40n 0 40.01n 1 50n 1 50.01n 0 100n 0 100.01n 1 130n 1 130.01n 0 175n 0 175.01n 1 177n 1 177.01n 0)
Vr r 0 PWL(0 0 40n 0 40.01n 1 45n 1 45.01n 0 60n 0 60.01n 1 62n 1 62.01n 0 110n 0 110.01n 1 112n 1 112.01n 0 160n 0 160.01n 1 190n 1 190.01n 0)
A1 s r 0 0 0 qb1 q1 0 SRFLOP
Vd d 0 PWL(0 0 30n 0 30.01n 1 70n 1 70.01n 0)
Vp pre 0 PWL(0 0 82n 0 82.01n 1 98n 1 98.01n 0 120n 0 120.01n 1 128n 1 128.01n 0)
Vcl clr 0 PWL(0 0 32n 0 32.01n 1 48n 1 48.01n 0 120n 0 120.01n 1 124n 1 124.01n 0)
A2 d 0 clk pre clr qb2 q2 0 DFLOP
Vd3 d3 0 PWL(0 0 56n 0 56.01n 1 76n 1 76.01n 0)
A3 d3 0 clk 0 0 qb3 q3 0 DFLOP Td=3n
Vs4 s4 0 PWL(0 0 140n 0 140.01n 1 140.5n 1 140.51n 0)
Vr4 r4 0 PWL(0 0 141n 0 141.01n 1 141.5n 1 141.51n 0)
A4 s4 r4 0 0 0 qb4 q4 0 SRFLOP Td=3n
Vx x 0 PWL(0 0 62n 0 62.01n 1 68n 1 68.01n 0)
A5 clk x 0 0 0 p2 p1 0 COUNTER cycles=4
A6 clk 0 0 0 0 p2b p1b 0 COUNTER cycles=3 Trise=2n Td=5n
.end
";
    let probes = [
        // SRFLOP: R holds Q low over S, and S sets it again once R falls.
        ("q1", 21e-9, 1.0),
        ("q1", 41e-9, 0.0),
        ("q1", 46e-9, 1.0),
        ("q1", 61e-9, 0.0),
        ("q1", 101e-9, 1.0),
        ("q1", 111e-9, 0.0),
        ("q1", 113e-9, 1.0),
        ("q1", 161e-9, 0.0),
        ("q1", 176e-9, 0.0),
        // DFLOP: CLR masks the edges at 35 and 45 ns, PRE those at 85 and
        // 95 ns, and CLR wins over PRE from 120 to 124 ns.
        ("q2", 36e-9, 0.0),
        ("q2", 56e-9, 1.0),
        ("qb2", 56e-9, 0.0),
        ("q2", 76e-9, 0.0),
        ("q2", 83e-9, 1.0),
        ("q2", 96e-9, 1.0),
        ("q2", 106e-9, 0.0),
        ("q2", 122e-9, 0.0),
        ("q2", 125e-9, 1.0),
        ("q2", 136e-9, 0.0),
        // Td = 3 ns after the edges at 65 and 85 ns; D rising 1 ns after
        // the edge at 55 ns waits for the next one.
        ("q3", 67.9e-9, 0.0),
        ("q3", 68.1e-9, 1.0),
        ("q3", 87.9e-9, 1.0),
        ("q3", 88.1e-9, 0.0),
        // A 1 ns set-then-reset is shorter than Td and never shows.
        ("q4", 143.5e-9, 0.0),
        ("q4", 144.5e-9, 0.0),
        // COUNTER cycles=4: Phi1 for counts 0 and 1, Phi2 for 2 and 3; RST
        // from 62 to 68 ns zeroes the count and swallows the edge at 65 ns.
        ("p1", 4e-9, 1.0),
        ("p2", 16e-9, 1.0),
        ("p1", 26e-9, 0.0),
        ("p1", 36e-9, 1.0),
        ("p2", 61e-9, 1.0),
        ("p1", 64e-9, 1.0),
        ("p1", 66e-9, 1.0),
        ("p1", 76e-9, 1.0),
        ("p2", 86e-9, 1.0),
        ("p2", 96e-9, 1.0),
        ("p1", 106e-9, 1.0),
    ];
    let points: Vec<(&str, f64)> = probes.iter().map(|(n, t, _)| (*n, *t)).collect();
    let values = tran_at(deck, 200e-9, 0.1e-9, &points);
    for ((node, time, expected), actual) in probes.iter().zip(values) {
        assert_close(&format!("{node} at {time:e}"), actual, *expected, 1e-3);
    }

    // COUNTER cycles=3 slews over Trise = 2 ns from the clock edge whatever
    // Td says. LTspice 26.1.1 batch: 0.48763 and 0.51237 at 6 ns, 0.487903
    // at 16 ns; its counter runs about 13 mV ahead of its own slew at the
    // start of each ramp, inside this tolerance.
    let ramps = [
        ("p1b", 6e-9, 0.48763),
        ("p2b", 6e-9, 0.51237),
        ("p2b", 16e-9, 0.487903),
    ];
    let points: Vec<(&str, f64)> = ramps.iter().map(|(n, t, _)| (*n, *t)).collect();
    let values = tran_at(deck, 20e-9, 0.02e-9, &points);
    for ((node, time, expected), actual) in ramps.iter().zip(values) {
        assert_close(&format!("{node} at {time:e}"), actual, *expected, 0.02);
    }
}

#[test]
fn a_counter_clock_high_in_the_operating_point_counts_at_the_first_step() {
    let deck = "* clock high from the start
Vh hi 0 1
A1 hi 0 0 0 0 p2 p1 0 COUNTER cycles=2
A2 hi 0 hi 0 0 qb2 q2 0 DFLOP
A3 hi 0 0 0 0 qb3 q3 0 SRFLOP
.end
";
    // LTspice 26.1.1 batch: the counter steps to count 1 at once, the
    // DFLOP sees no edge, and the SRFLOP sets on the first step.
    let values = tran_at(
        deck,
        10e-9,
        0.1e-9,
        &[("p1", 5e-9), ("p2", 5e-9), ("q2", 5e-9), ("q3", 5e-9)],
    );
    for ((node, expected), actual) in [("p1", 0.0), ("p2", 1.0), ("q2", 0.0), ("q3", 1.0)]
        .into_iter()
        .zip(values)
    {
        assert_close(node, actual, expected, 1e-9);
    }
}

#[test]
fn a_counter_needs_cycles() {
    let error = build_error("* bad\nA1 clk 0 0 0 0 p2 p1 0 COUNTER\nVc clk 0 1\n.end\n");
    assert!(error.contains("cycles"), "{error}");
}

#[test]
fn phase_detector_steps_where_ltspice_does() {
    let deck = "* phase detectors
Va a 0 PULSE(0 1 10n 1p 1p 50n 100n)
Vb b 0 PULSE(0 1 30n 1p 1p 50n 100n)
A1 a b 0 0 0 0 q1 0 PHASEDET Iout=15u
R1 q1 0 1k
A2 b a 0 0 0 0 q2 0 PHASEDET Vhigh=2.5 Vlow=-0.5
R2 q2 0 1Meg
Vf f 0 PULSE(0 1 10n 1p 1p 20n 40n)
A3 f b 0 0 0 0 q3 0 PHASEDET
R3 q3 0 1k
Vm m 0 0.3
A4 a m 0 0 0 0 q4 0 PHASEDET Ref=0.2
R4 q4 0 1k
.end
";
    // LTspice 26.1.1 batch.
    let probes = [
        // A leads B by 20 ns: Iout while A is ahead.
        ("q1", 15e-9, 0.015),
        ("q1", 35e-9, 0.0),
        ("q1", 115e-9, 0.015),
        // Swapped inputs sink 100 uA, held at Vlow through the 10 ohm clamp.
        ("q2", 15e-9, -0.500_995),
        ("q2", 35e-9, 0.0),
        // A at 25 MHz against B at 10 MHz: the state stops at +1, so B's edge
        // at 30 ns, and the coincident edges at 130 ns, bring it to zero.
        ("q3", 15e-9, 0.1),
        ("q3", 35e-9, 0.0),
        ("q3", 95e-9, 0.1),
        ("q3", 135e-9, 0.0),
        // B is high from the operating point on, which counts as an edge.
        ("q4", 5e-9, -0.1),
        ("q4", 15e-9, 0.0),
        ("q4", 115e-9, 0.1),
    ];
    let points: Vec<(&str, f64)> = probes.iter().map(|(n, t, _)| (*n, *t)).collect();
    let values = tran_at(deck, 150e-9, 0.1e-9, &points);
    for ((node, time, expected), actual) in probes.iter().zip(values) {
        assert_close(&format!("{node} at {time:e}"), actual, *expected, 1e-5);
    }
}

#[test]
fn sample_and_hold_tracks_and_samples_as_ltspice_does() {
    let deck = "* sample and hold
Vi in 0 SINE(0.3 1 10Meg)
Vck ck 0 PULSE(0 1 25n 1p 1p 10n 50n)
Vsh sh 0 PWL(0 0 20n 0 20.01n 1 40n 1 40.01n 0)
Vn inn 0 0.25
A1 in 0 0 sh 0 0 o1 0 SAMPLEHOLD
A2 in inn ck 0 0 0 o2 0 SAMPLEHOLD Rout=10
R2 o2 0 1k
A3 in 0 ck 0 0 0 o3 0 SAMPLEHOLD Vhigh=0.5 Vlow=-0.2
.end
";
    // LTspice 26.1.1 batch, which tracks one time step late: within 1 mV.
    let probes = [
        // S/H: zero until S/H rises at 20 ns, then the input until it falls
        // at 40 ns.
        ("o1", 10e-9, 0.0),
        ("o1", 30e-9, 1.251_06),
        ("o1", 45e-9, 0.887_633),
        ("o1", 80e-9, 0.887_633),
        // CLK at 25 and 75 ns samples V(in) - 0.25 behind 10 ohms into 1k.
        ("o2", 10e-9, 0.0),
        ("o2", 30e-9, 1.0396),
        ("o2", 80e-9, -0.940_594),
        // The limits hold the output within 0.8 V/1001 of Vhigh = 0.5 and
        // 0.5 V/1001 of Vlow = -0.2.
        ("o3", 30e-9, 0.500_799),
        ("o3", 80e-9, -0.200_5),
    ];
    let points: Vec<(&str, f64)> = probes.iter().map(|(n, t, _)| (*n, *t)).collect();
    let values = tran_at(deck, 100e-9, 0.01e-9, &points);
    for ((node, time, expected), actual) in probes.iter().zip(values) {
        assert_close(&format!("{node} at {time:e}"), actual, *expected, 1e-3);
    }
    let both = build_error("* bad\nV1 in 0 1\nA1 in 0 in in 0 0 o 0 SAMPLEHOLD\n.end\n");
    assert!(both.contains("not both"), "{both}");
    let neither = build_error("* bad\nV1 in 0 1\nA1 in 0 0 0 0 0 o 0 SAMPLEHOLD\n.end\n");
    assert!(neither.contains("CLK (terminal 3) or S/H"), "{neither}");
}

#[test]
fn modulator_follows_ltspice_in_frequency_and_amplitude() {
    let deck = "* modulators
Vf fm 0 0.5
Va am 0 0.5
A1 fm 0 0 0 0 c1 s1 0 MODULATOR mark=2Meg space=1Meg
A2 fm am 0 0 0 c2 s2 0 MODULATOR mark=2Meg space=1Meg
R2 s2 0 1k
Vf3 fm3 0 PWL(0 0 1u 0 1.001u 1)
A3 fm3 0 0 0 0 c3 s3 0 MODULATOR mark=2Meg space=1Meg
A4 0 0 0 0 0 c4 s4 0 MODULATOR mark=2Meg space=1Meg
.end
";
    // LTspice 26.1.1 batch.
    let probes = [
        // 1.5 MHz from FM = 0.5 V.
        ("s1", 100e-9, 0.809_009),
        ("c1", 100e-9, 0.587_78),
        ("s1", 166.667e-9, 0.999_99),
        // Amplitude 0.5 behind the 1 ohm default into 1k.
        ("s2", 100e-9, 0.404_101),
        ("c2", 250e-9, -0.353_55),
        // Space until FM steps to 1 V at 1 us, then mark.
        ("s3", 100e-9, 0.587_783),
        ("c3", 1.25e-6, -0.999_985),
        // An unused FM reads 1 V: the mark frequency.
        ("s4", 100e-9, 0.951_04),
        ("c4", 100e-9, 0.309_012),
    ];
    let points: Vec<(&str, f64)> = probes.iter().map(|(n, t, _)| (*n, *t)).collect();
    let values = tran_at(deck, 1.5e-6, 1e-9, &points);
    for ((node, time, expected), actual) in probes.iter().zip(values) {
        assert_close(&format!("{node} at {time:e}"), actual, *expected, 2e-3);
    }
    let op = Engine::default()
        .run_dc_op(&parse(deck))
        .expect("OP solves");
    assert_close("c2", op.try_voltage_named("c2").unwrap(), 0.5, 1e-9);
    assert_close("s2", op.try_voltage_named("s2").unwrap(), 0.0, 1e-9);
    let missing = build_error("* bad\nA1 0 0 0 0 0 c s 0 MODULATOR mark=1k\n.end\n");
    assert!(missing.contains("space"), "{missing}");
}

#[test]
fn varistor_clamps_beyond_its_breakdown_as_ltspice_does() {
    let deck = |v: f64| {
        format!(
            "* varistors
V1 in 0 {v}
Vc c 0 2
Vn n 0 -2
R1 in o1 1k
A1 c 0 0 0 0 0 o1 0 VARISTOR rclamp=10
R2 in o2 1k
A2 c 0 0 0 0 0 o2 0 VARISTOR
R3 in o3 1k
A3 n 0 0 0 0 0 o3 0 VARISTOR rclamp=10
.op
.end
"
        )
    };
    // LTspice 26.1.1 batch (.dc V1 -5 5).
    for (v, o1, o2) in [
        (-5.0, -2.029_702_970_3, -2.002_997_003_0),
        (1.5, 1.5, 1.5),
        (3.0, 2.009_900_990_1, 2.000_999_001_0),
        (5.0, 2.029_702_970_3, 2.002_997_003_0),
    ] {
        let op = Engine::default()
            .run_dc_op(&parse(&deck(v)))
            .expect("OP solves");
        let node = |name: &str| op.try_voltage_named(name).unwrap();
        assert_close(&format!("o1 at {v}"), node("o1"), o1, 1e-9);
        assert_close(&format!("o2 at {v}"), node("o2"), o2, 1e-9);
        // The breakdown is the magnitude of the control voltage.
        assert_close(&format!("o3 at {v}"), node("o3"), o1, 1e-9);
    }
}

#[test]
fn a_varistor_table_reads_its_breakdown_at_v2_minus_v1() {
    // A3's breakdown is -3: it conducts everywhere and jumps at -3 V, where
    // LTspice finds no operating point, so it runs on the positive side only.
    let deck = |v: f64, negative_breakdown: bool| {
        let a3 = if negative_breakdown {
            "R3 in o3 1k
A3 0 c 0 0 0 0 o3 0 VARISTOR table(0 0 1 -1 2 -2 3 -3 4 -4)
"
        } else {
            ""
        };
        format!(
            "* varistor tables
V1 in 0 {v}
Va a 0 10.5
Vc c 0 3
R1 in o1 1k
A1 0 a 0 0 0 0 o1 0 VARISTOR table(.5 0 10.5 10)
R2 in o2 1k
A2 c 0 0 0 0 0 o2 0 VARISTOR table(-5 5 0 0) rclamp=10
{a3}R4 in o4 1k
X1 a o4 vt top=8
.subckt vt a o top=10
A1 0 a 0 0 0 0 o 0 VARISTOR table(.5 0 10.5 {{top}})
.ends
.op
.end
"
        )
    };
    // LTspice 26.1.1 (.dc V1 -20 20 1, and .op at -12 without A3).
    for (v, o1, o2, o3, o4) in [
        (0.0, 0.0, 0.0, Some(-2.997_002_997_003), 0.0),
        (5.0, 5.0, 3.019_801_980_198, Some(-2.992_007_992_008), 5.0),
        (
            12.0,
            10.001_998_001_998,
            3.089_108_910_891,
            Some(-2.985_014_985_015),
            8.003_996_003_996,
        ),
        (
            20.0,
            10.009_990_009_990,
            3.168_316_831_683,
            Some(-2.977_022_977_023),
            8.011_988_011_988,
        ),
        (
            -12.0,
            -10.001_998_001_998,
            -3.089_108_910_891,
            None,
            -8.003_996_003_996,
        ),
    ] {
        let op = Engine::default()
            .run_dc_op(&parse(&deck(v, o3.is_some())))
            .expect("OP solves");
        let node = |name: &str| op.try_voltage_named(name).unwrap();
        assert_close(&format!("o1 at {v}"), node("o1"), o1, 1e-9);
        assert_close(&format!("o2 at {v}"), node("o2"), o2, 1e-9);
        if let Some(o3) = o3 {
            assert_close(&format!("o3 at {v}"), node("o3"), o3, 1e-9);
        }
        assert_close(&format!("o4 at {v}"), node("o4"), o4, 1e-9);
    }
}

#[test]
fn ota_source_limit_is_also_spelled_isrc_and_isource() {
    let deck = "* ota source limit spellings
V1 p 0 1
A1 p 0 0 0 0 0 o1 0 OTA g=1m Isrc=43u Vlow=-1e308 Vhigh=1e308
R1 o1 0 1k
A2 p 0 0 0 0 0 o2 0 OTA g=1m Iout=43u Vlow=-1e308 Vhigh=1e308
R2 o2 0 1k
A3 p 0 0 0 0 0 o3 0 OTA g=1m Isource=43u Vlow=-1e308 Vhigh=1e308
R3 o3 0 1k
A4 0 p 0 0 0 0 o4 0 OTA g=1m Isrc=90u Isink=-20u Vlow=-1e308 Vhigh=1e308 asym
R4 o4 0 1k
A5 p 0 0 0 0 0 o5 0 OTA g=1m Isrc=90u Isink=-20u Vlow=-1e308 Vhigh=1e308 asym
R5 o5 0 1k
A6 p 0 0 0 0 0 o6 0 OTA g=10u Isrc=90u Isink=-20u Vlow=-1e308 Vhigh=1e308 asym
R6 o6 0 1k
.op
.end
";
    let op = Engine::default()
        .run_dc_op(&parse(deck))
        .expect("OP solves");
    let v = |node: &str| op.try_voltage_named(node).unwrap();
    // LTspice 26.1.1 batch: -43 mV three times, 90 mV, -20 mV, -9.242343 mV.
    for node in ["o1", "o2", "o3"] {
        assert_close(node, v(node), -0.043, 1e-9);
    }
    assert_close("o4", v("o4"), 0.09, 1e-9);
    assert_close("o5", v("o5"), -0.02, 1e-9);
    assert_close("o6", v("o6"), -9.242_343e-3, 1e-9);
}

#[test]
fn ota_common_mode_current_noise_matches_ltspice() {
    // LTspice 26.1.1 batch, V(A1.icm): incm*sqrt(1 + incmk/f) is one current
    // driven into both inputs, so the difference across 1k and 2k loads
    // carries |1k - 2k|*incm, not their quadrature sum.
    let difference = "* ota incm read as a difference
V1 s 0 0 AC 1
Ra a 0 1k
Rb b s 2k
E1 out 0 a b 1
A1 a b 0 0 0 0 0 0 OTA g=0 incm=1p incmk=10
.end
";
    let densities = ota_noise_density(difference, "icm", &[1.0, 10.0, 1e3]);
    for (actual, expected) in densities
        .into_iter()
        .zip([3.316_625e-9, 1.414_214e-9, 1.004_988e-9])
    {
        assert_close("icm difference", actual, expected, 1e-6 * expected);
    }
    let single = "* ota incm into one input load
V1 s 0 0 AC 1
Ra a 0 1k
Rb out s 2k
A1 a out 0 0 0 0 0 0 OTA g=0 incm=1p incmk=10
.end
";
    let densities = ota_noise_density(single, "icm", &[1.0, 10.0, 1e3]);
    for (actual, expected) in densities
        .into_iter()
        .zip([6.633_250e-9, 2.828_427e-9, 2.009_975e-9])
    {
        assert_close("icm", actual, expected, 1e-6 * expected);
    }
}

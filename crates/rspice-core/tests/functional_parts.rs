//! The functional parts of the embedded functional library, checked against
//! their own parameters.
//!
//! Each part resolves by name with no include. Every check is analytic: the
//! deck sets a parameter and the test measures the behaviour that parameter
//! names. Where LTspice has an equivalent part, a few scalars from an
//! LTspice 26.1.1 batch run of the same deck are pinned beside the analytic
//! value; QSPICE has no netlist form of a generic op-amp to vote with.

use rspice_core::engine::{Engine, TransientResult};
use rspice_core::netlist::Netlist;

fn parse(deck: &str) -> Netlist {
    Netlist::parse(deck).unwrap_or_else(|error| panic!("deck parses: {error}\n{deck}"))
}

fn assert_close(label: &str, actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{label}: {actual:.9e} vs {expected:.9e} (tolerance {tolerance:e})"
    );
}

fn op_voltages(deck: &str, nodes: &[&str]) -> Vec<f64> {
    let op = Engine::default()
        .run_dc_op(&parse(deck))
        .unwrap_or_else(|error| panic!("operating point solves: {error}\n{deck}"));
    nodes
        .iter()
        .map(|node| {
            op.try_voltage_named(node)
                .unwrap_or_else(|| panic!("no voltage for {node}"))
        })
        .collect()
}

fn op_currents(deck: &str, sources: &[&str]) -> Vec<f64> {
    let op = Engine::default()
        .run_dc_op(&parse(deck))
        .unwrap_or_else(|error| panic!("operating point solves: {error}\n{deck}"));
    sources
        .iter()
        .map(|source| {
            op.branch_current_named(source)
                .unwrap_or_else(|| panic!("no current for {source}"))
        })
        .collect()
}

/// The time `node` first crosses `level` rising, interpolated.
fn rising_crossing(time: &[f64], wave: &[f64], level: f64) -> f64 {
    let index = wave
        .windows(2)
        .position(|pair| pair[0] < level && pair[1] >= level)
        .unwrap_or_else(|| panic!("never rises through {level}"));
    let fraction = (level - wave[index]) / (wave[index + 1] - wave[index]);
    time[index] + fraction * (time[index + 1] - time[index])
}

/// The time `node` first crosses `level` falling, interpolated.
fn falling_crossing(time: &[f64], wave: &[f64], level: f64) -> f64 {
    let inverted = wave.iter().map(|value| -value).collect::<Vec<_>>();
    rising_crossing(time, &inverted, -level)
}

/// The value of `wave` at `at`, interpolated.
fn value_at(time: &[f64], wave: &[f64], at: f64) -> f64 {
    let index = time
        .windows(2)
        .position(|pair| pair[0] <= at && at <= pair[1])
        .unwrap_or_else(|| panic!("{at} is outside the run"));
    let fraction = (at - time[index]) / (time[index + 1] - time[index]);
    wave[index] + fraction * (wave[index + 1] - wave[index])
}

fn transient(deck: &str, stop: f64, step: f64) -> TransientResult {
    Engine::default()
        .run_tran(&parse(deck), stop, step)
        .unwrap_or_else(|error| panic!("transient solves: {error}\n{deck}"))
}

fn waveform<'a>(tran: &'a TransientResult, node: &str) -> &'a [f64] {
    tran.try_voltage_waveform_named(node)
        .unwrap_or_else(|| panic!("no waveform for {node}"))
}

const SUPPLIES: &str = "VP vp 0 5\nVN vn 0 -5\n";

#[test]
fn op_amp_open_loop_response_is_its_gain_and_gain_bandwidth() {
    // Open loop for AC, closed at DC through a 1 GOhm / 1 F servo.
    let deck = "* op-amp open loop
VP vp 0 15
VN vn 0 -15
VIN in 0 0 AC 1
X1 in fb vp vn out RSPICE_FN_OPAMP AVOL=200k GBW=10Meg
RF out fb 1G
CF fb 0 1
.end
";
    let frequencies = [1.0, 10.0, 1e6, 1e7];
    let ac = Engine::default()
        .run_ac(&parse(deck), &frequencies)
        .expect("AC solves");
    let out = |index: usize| {
        let point = &ac[index];
        let node = point
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case("out"))
            .expect("out solved");
        point.voltages[node]
    };
    // H(f) = AVOL / (1 + j f AVOL / GBW).
    for (index, frequency) in frequencies.iter().enumerate() {
        let pole = 1e7 / 2e5;
        let expected = 2e5 / (1.0 + (frequency / pole).powi(2)).sqrt();
        assert_close("|H|", out(index).norm(), expected, 2e-3 * expected);
    }
    // LTspice 26.1.1 batch, UniversalOpAmp2 with equal parameters: 105.836 dB
    // and -11.31 degrees at 10 Hz, -90.00 degrees at 1 MHz, unity gain at
    // 9.988 MHz.
    assert_close("dB at 10 Hz", 20.0 * out(1).norm().log10(), 105.836, 0.02);
    assert_close("phase at 10 Hz", out(1).arg().to_degrees(), -11.31, 0.05);
    assert_close("phase at 1 MHz", out(2).arg().to_degrees(), -90.0, 0.05);
    assert_close("|H(GBW)|", out(3).norm(), 1.0, 2e-3);
}

#[test]
fn op_amp_slews_at_its_slew_rate() {
    let deck = format!(
        "* op-amp slew
{SUPPLIES}VIN in 0 PULSE(-2 2 1u 1n 1n 5u 10u)
X1 in out vp vn out RSPICE_FN_OPAMP SLEW=10Meg
RL out 0 10k
.end
"
    );
    let tran = Engine::default()
        .run_tran(&parse(&deck), 3e-6, 1e-9)
        .expect("transient solves");
    let wave = tran.try_voltage_waveform_named("out").expect("out solved");
    let rising = rising_crossing(&tran.time, wave, 1.0) - rising_crossing(&tran.time, wave, -1.0);
    // LTspice 26.1.1 batch, UniversalOpAmp2 Slew=10Meg: 9.931 V/us over the
    // same crossings.
    assert_close("slew rate", 2.0 / rising, 10e6, 0.01 * 10e6);
}

#[test]
fn op_amp_output_swing_stops_its_headroom_inside_the_supply_pins() {
    // Open loop, overdriven both ways, lightly loaded: the output sits at the
    // clamp less the drop across ROUT.
    let deck = |vp: f64, headroom: f64| {
        format!(
            "* op-amp swing
VP vp 0 {vp}
VN vn 0 -5
VH hi 0 1
VL lo 0 -1
XH hi 0 vp vn outh RSPICE_FN_OPAMP HEADROOM={headroom}
XL lo 0 vp vn outl RSPICE_FN_OPAMP HEADROOM={headroom}
RH outh 0 1Meg
RL outl 0 1Meg
.end
"
        )
    };
    for (vp, headroom) in [(5.0, 0.5), (5.0, 0.0), (12.0, 1.2)] {
        let [high, low] = op_voltages(&deck(vp, headroom), &["outh", "outl"])[..] else {
            unreachable!()
        };
        let top = vp - headroom;
        let bottom = -5.0 + headroom;
        assert_close("high clip", high, top * (1.0 - 50e-6), 1e-4);
        assert_close("low clip", low, bottom * (1.0 - 50e-6), 1e-4);
    }
}

#[test]
fn op_amp_output_current_stops_at_its_limits_and_comes_from_its_supply_pins() {
    let deck = format!(
        "* op-amp current limit
{SUPPLIES}VS in 0 2
X1 in out vp vn out RSPICE_FN_OPAMP ISRC=20m ISNK=30m IQ=1m
RL out 0 10
.end
"
    );
    let [out] = op_voltages(&deck, &["out"])[..] else {
        unreachable!()
    };
    assert_close("sourced current", out / 10.0, 20e-3, 0.005 * 20e-3);
    let [vp, vn] = op_currents(&deck, &["VP", "VN"])[..] else {
        unreachable!()
    };
    // Sourced current leaves V+; V- returns only the quiescent current.
    assert_close("V+ current", -vp, 1e-3 + out / 10.0, 1e-6);
    assert_close("V- current", vn, 1e-3, 1e-6);

    let sink = deck.replace("VS in 0 2", "VS in 0 -2");
    let [out] = op_voltages(&sink, &["out"])[..] else {
        unreachable!()
    };
    // The limit is the asymptote of a hyperbolic tangent: with the internal
    // node at the rail this load reaches 99.6 % of it.
    assert_close("sunk current", out / 10.0, -30e-3, 0.005 * 30e-3);
    // LTspice 26.1.1 batch, UniversalOpAmp2 Ilimit=25m into 10 Ohm: 24.9995 mA.
    let [vp, vn] = op_currents(&sink, &["VP", "VN"])[..] else {
        unreachable!()
    };
    assert_close("V+ current while sinking", -vp, 1e-3, 1e-6);
    assert_close("V- current while sinking", vn, 1e-3 - out / 10.0, 1e-6);
}

#[test]
fn op_amp_input_offset_and_bias_currents_are_its_parameters() {
    let deck = format!(
        "* op-amp offsets
{SUPPLIES}VS in 0 0.5
X1 in out1 vp vn out1 RSPICE_FN_OPAMP VOS=1m
RB in2 0 1Meg
X2 in2 out2 vp vn out2 RSPICE_FN_OPAMP IB=100n IOS=20n
.end
"
    );
    let [out1, out2] = op_voltages(&deck, &["out1", "out2"])[..] else {
        unreachable!()
    };
    // LTspice 26.1.1 batch, UniversalOpAmp2 Vos=1m follower: +1.000 mV.
    assert_close("follower with offset", out1, 0.501, 1e-5);
    // IN+ draws IB + IOS/2 through 1 MOhm.
    assert_close("bias drop", out2, -110e-9 * 1e6, 1e-5);
}

#[test]
fn op_amp_noise_densities_are_its_parameters() {
    let noise = |deck: &str, frequencies: &[f64]| -> Vec<f64> {
        Engine::default()
            .run_noise_named_with_input_source(
                &parse(deck),
                "out",
                None,
                "VIN",
                frequencies,
                300.15,
            )
            .expect("noise solves")
            .iter()
            .map(|point| point.output_noise_density.sqrt())
            .collect()
    };
    // A follower driven from a stiff source shows en alone, with its corner.
    let follower = "* op-amp en
VP vp 0 15
VN vn 0 -15
VIN in 0 0 AC 1
X1 in out vp vn out RSPICE_FN_OPAMP EN=10n ENK=100 IN=0
.end
";
    for (density, frequency) in noise(follower, &[10.0, 1e4])
        .into_iter()
        .zip([10.0_f64, 1e4])
    {
        let expected = 10e-9 * (1.0 + 100.0 / frequency).sqrt();
        assert_close("en", density, expected, 2e-3 * expected);
    }
    // LTspice 26.1.1 batch, UniversalOpAmp2 with equal parameters: gain 10,
    // 10 kOhm source, en 10n/100 Hz, in 1p/100 Hz.
    let amplifier = "* op-amp noise
VP vp 0 15
VN vn 0 -15
VIN src 0 0 AC 1
RS src inp 10k
X1 inp fb vp vn out RSPICE_FN_OPAMP AVOL=200k EN=10n ENK=100 IN=1p INK=100
RF out fb 9k
RG fb 0 1k
.end
";
    for (density, expected) in noise(amplifier, &[10.0, 1e3, 1e5]).into_iter().zip([
        5.086_580e-7,
        2.052_627e-7,
        1.989_207e-7,
    ]) {
        assert_close("output noise", density, expected, 3e-3 * expected);
    }
}

#[test]
fn every_functional_subcircuit_parses_and_its_default_instance_solves() {
    // Each part with its defaults, its positive supply pin at 5 V, its
    // negative or ground pin at ground and every other pin loaded to ground.
    let library = parse(rspice_core_base::builtin_lib::FUNCTIONAL_LIB);
    assert!(!library.subcircuits.is_empty());
    for subcircuit in &library.subcircuits {
        let mut deck = format!("* {} defaults\nVSUP sup 0 5\n", subcircuit.name);
        let mut probed = Vec::new();
        let mut nodes = Vec::new();
        for port in &subcircuit.ports {
            match port.to_ascii_uppercase().as_str() {
                "VP" | "VCC" => nodes.push("sup".to_owned()),
                "VN" | "GND" => nodes.push("0".to_owned()),
                _ => {
                    let node = format!("n_{port}");
                    deck.push_str(&format!("R_{port} {node} 0 100k\n"));
                    nodes.push(node.clone());
                    probed.push(node);
                }
            }
        }
        deck.push_str(&format!(
            "X1 {} {}\n.end\n",
            nodes.join(" "),
            subcircuit.name
        ));
        let probes = probed.iter().map(String::as_str).collect::<Vec<_>>();
        for (node, voltage) in probes.iter().zip(op_voltages(&deck, &probes)) {
            assert!(
                voltage.is_finite() && voltage.abs() <= 5.0 + 1e-6,
                "{} {node}: {voltage}",
                subcircuit.name
            );
        }
    }
}

const LOGIC_SUPPLIES: &str = "VP vp 0 5\nVN vn 0 0\n";

#[test]
fn comparator_trips_half_its_hysteresis_either_side_of_its_offset() {
    // A 20 mV/us ramp through a 1 V reference, up and back down. The output
    // moves TPD plus half an edge, 1.5 ns, after the input crosses.
    let deck = format!(
        "* comparator thresholds
{LOGIC_SUPPLIES}VIN in 0 PWL(0 0.9 10u 1.1 20u 0.9)
VR ref 0 1
X1 in ref vp vn out RSPICE_FN_COMPARATOR HYST=20m TPD=1n TRISE=1n TFALL=1n
RL out 0 100k
X2 in ref vp vn shifted RSPICE_FN_COMPARATOR HYST=20m VOS=5m TPD=1n TRISE=1n TFALL=1n
RL2 shifted 0 100k
.end
"
    );
    let tran = transient(&deck, 20e-6, 10e-9);
    let input = waveform(&tran, "in");
    for (node, offset) in [("out", 0.0), ("shifted", 5e-3)] {
        let output = waveform(&tran, node);
        let rise = rising_crossing(&tran.time, output, 2.5) - 1.5e-9;
        let fall = falling_crossing(&tran.time, output, 2.5) - 1.5e-9;
        let rising = value_at(&tran.time, input, rise);
        let falling = value_at(&tran.time, input, fall);
        assert_close("rising threshold", rising, 1.01 - offset, 0.2e-3);
        assert_close("falling threshold", falling, 0.99 - offset, 0.2e-3);
    }
}

#[test]
fn comparator_output_moves_its_propagation_delay_after_the_input_and_slews_its_edge_times() {
    // The input steps through the 1.0005 V rising threshold 0.05 ns into its
    // 0.1 ns edge at 1 us, and back down through 0.9995 V just after 6 us.
    let deck = format!(
        "* comparator delay
{LOGIC_SUPPLIES}VIN in 0 PULSE(0.9 1.1 1u 0.1n 0.1n 5u 10u)
VR ref 0 1
X1 in ref vp vn out RSPICE_FN_COMPARATOR TPD=100n TRISE=10n TFALL=20n
RL out 0 100k
.end
"
    );
    let tran = transient(&deck, 8e-6, 1e-9);
    let out = waveform(&tran, "out");
    let crossed_up = 1e-6 + 0.050_25e-9;
    let crossed_down = 6.0001e-6 + 0.050_25e-9;
    // The edge starts TPD after the crossing and is half done TRISE/2 later.
    let at = |level| rising_crossing(&tran.time, out, level);
    assert_close("rising delay", at(2.5) - crossed_up, 100e-9 + 5e-9, 0.2e-9);
    assert_close("10-90% rise", at(4.5) - at(0.5), 0.8 * 10e-9, 0.2e-9);
    let at = |level| falling_crossing(&tran.time, out, level);
    assert_close(
        "falling delay",
        at(2.5) - crossed_down,
        100e-9 + 10e-9,
        0.2e-9,
    );
    assert_close("90-10% fall", at(0.5) - at(4.5), 0.8 * 20e-9, 0.2e-9);
    // High is V+ divided between ROUT and the load.
    let high = out.iter().copied().fold(f64::MIN, f64::max);
    assert_close("output high", high, 5.0 * 100e3 / (100e3 + 50.0), 1e-6);
}

#[test]
fn comparator_open_collector_output_only_sinks() {
    // A 10 kOhm pull-up to its own 12 V rail: high is the pull-up rail, not
    // V+. Low, the stage sinks the pull-up current through ROUT up to its
    // limit: I = ISNK*tanh(V/(ROUT*ISNK)).
    let deck = |input: f64| {
        format!(
            "* open-collector comparator
{LOGIC_SUPPLIES}VPU pu 0 12
VIN in 0 {input}
VR ref 0 1
X1 in ref vp vn out RSPICE_FN_COMPARATOR OC=1
RPU pu out 10k
.end
"
        )
    };
    let high = op_voltages(&deck(1.1), &["out"])[0];
    assert_close("released", high, 12.0, 1e-6);
    let low = op_voltages(&deck(0.9), &["out"])[0];
    let sunk = (12.0 - low) / 10e3;
    assert_close(
        "sink law",
        sunk,
        20e-3 * (low / (50.0 * 20e-3)).tanh(),
        1e-9,
    );
    assert!((0.05..0.07).contains(&low), "low {low}");
    // V+ carries the quiescent current alone; the sunk current returns to V-.
    let currents = op_currents(&deck(0.9), &["vp", "vn"]);
    assert_close("I(V+)", -currents[0], 500e-6, 1e-9);
    assert_close("I(V-)", currents[1], 500e-6 + sunk, 1e-9);
}

/// An OTA with 100 uA into IABC between +-5 V supplies, its output held at
/// ground by a 0 V source or left open, as `output` says.
fn ota_deck(input: f64, parameters: &str, output: &str) -> String {
    format!(
        "* ota
{SUPPLIES}IB vp iabc 100u
VIN in 0 {input}
X1 in 0 iabc vp vn out RSPICE_FN_OTA {parameters}
{output}
.end
"
    )
}

#[test]
fn ota_output_current_is_its_bias_current_times_tanh_of_the_input_over_its_linear_range() {
    for input in [-0.2, -0.05, 1e-3, 0.05, 0.2] {
        let current = op_currents(&ota_deck(input, "", "VO out 0 0"), &["vo"])[0];
        let expected = 100e-6 * (input / 51.7e-3).tanh();
        assert_close("I(OUT)", current, expected, 1e-12 + 1e-6 * expected.abs());
    }
    // Small-signal transconductance IABC/VLIN, here 1 mS.
    let current = op_currents(&ota_deck(1e-4, "VLIN=0.1", "VO out 0 0"), &["vo"])[0];
    assert_close("gm", current / 1e-4, 1e-3, 1e-6);
    // VOS cancels an equal and opposite input.
    let current = op_currents(&ota_deck(-10e-3, "VOS=10m", "VO out 0 0"), &["vo"])[0];
    assert_close("offset", current, 0.0, 1e-12);
}

#[test]
fn ota_output_stops_its_headroom_inside_the_supply_pins() {
    // Open, the 100 uA output current would drive ROUT to 1 kV; the clamp
    // holds it HEADROOM inside each supply pin, plus 100 uA across 1 Ohm.
    for (parameters, headroom) in [("", 1.0), ("HEADROOM=0.25", 0.25)] {
        let high = op_voltages(&ota_deck(0.2, parameters, ""), &["out"])[0];
        assert_close("high", high, 5.0 - headroom, 0.2e-3);
        let low = op_voltages(&ota_deck(-0.2, parameters, ""), &["out"])[0];
        assert_close("low", low, -5.0 + headroom, 0.2e-3);
    }
}

#[test]
fn ota_bias_input_sits_above_v_minus_and_its_current_returns_there() {
    // 0.65 V plus 100 uA across 10 Ohm.
    let deck = ota_deck(0.2, "", "VO out 0 0");
    let bias = op_voltages(&deck, &["iabc"])[0];
    assert_close("IABC", bias, -5.0 + 0.65 + 100e-6 * 10.0, 1e-6);
    // The bias current and the sourced output current come from V+ (the
    // bias through IB); V- returns the bias current and the quiescent one.
    let sourced = 100e-6 * (0.2_f64 / 51.7e-3).tanh();
    let currents = op_currents(&deck, &["vp", "vn"]);
    assert_close("I(V+)", -currents[0], 1e-3 + 100e-6 + sourced, 1e-9);
    assert_close("I(V-)", currents[1], 1e-3 + 100e-6, 1e-9);
}

#[test]
fn ota_input_bias_currents_and_noise_are_its_parameters() {
    // Each input returns to ground through 10 kOhm: IN+ draws IB + IOS/2,
    // IN- draws IB - IOS/2, and RIN joins them.
    let deck = format!(
        "* ota input stage
{SUPPLIES}IB vp iabc 100u
RP inp 0 10k
RN inn 0 10k
X1 inp inn iabc vp vn out RSPICE_FN_OTA IB=1u IOS=0.2u RIN=1Meg
VO out 0 0
.end
"
    );
    let [inp, inn] = op_voltages(&deck, &["inp", "inn"])[..] else {
        unreachable!()
    };
    assert_close("V(IN+)", inp, -10.980_392e-3, 1e-8);
    assert_close("V(IN-)", inn, -9.019_608e-3, 1e-8);

    let noise = |deck: &str, frequencies: &[f64]| -> Vec<f64> {
        Engine::default()
            .run_noise_named_with_input_source(
                &parse(deck),
                "out",
                None,
                "VIN",
                frequencies,
                300.15,
            )
            .expect("noise solves")
            .iter()
            .map(|point| point.output_noise_density.sqrt())
            .collect()
    };
    // 100 uA of bias into a noiseless 1 kOhm load beside ROUT: the noise at
    // the output is the input's times IABC/VLIN times the load.
    let gain = 100e-6 / 51.7e-3 * (1e3 * 10e6 / (1e3 + 10e6));
    let ota = |source: &str, parameters: &str| {
        format!(
            "* ota noise
{SUPPLIES}IB vp iabc 100u
{source}
X1 inp 0 iabc vp vn out RSPICE_FN_OTA {parameters}
RL out 0 1k NOISY=0
.end
"
        )
    };
    // Driven from a stiff source, the input carries EN alone, with its corner.
    let stiff = ota("VIN inp 0 0 AC 1", "ENK=100");
    for (density, frequency) in noise(&stiff, &[10.0, 1e4]).into_iter().zip([10.0_f64, 1e4]) {
        let expected = 7e-9 * (1.0 + 100.0 / frequency).sqrt() * gain;
        assert_close("en", density, expected, 2e-3 * expected);
    }
    // IN flows through a noiseless 10 kOhm source resistance.
    let resistive = ota("VIN src 0 0 AC 1\nRS src inp 10k NOISY=0", "EN=0 IN=1p");
    let density = noise(&resistive, &[1e4])[0];
    let expected = 1e-12 * 10e3 * gain;
    assert_close("in", density, expected, 2e-3 * expected);
}

#[test]
fn gates_follow_their_truth_tables_at_their_levels_for_two_to_five_inputs() {
    // Every input combination on every gate drawn with two to five inputs,
    // the inputs it leaves off tied to ground; both levels on the inverter
    // and the buffer. Unloaded, the output sits at its level. XOR is high
    // when exactly one input is.
    type Truth = fn(&[bool]) -> bool;
    let gates: [(&str, Truth); 5] = [
        ("RSPICE_FN_AND", |inputs| inputs.iter().all(|&high| high)),
        ("RSPICE_FN_NAND", |inputs| !inputs.iter().all(|&high| high)),
        ("RSPICE_FN_OR", |inputs| inputs.iter().any(|&high| high)),
        ("RSPICE_FN_NOR", |inputs| !inputs.iter().any(|&high| high)),
        ("RSPICE_FN_XOR", |inputs| {
            inputs.iter().filter(|&&high| high).count() == 1
        }),
    ];
    let level = |high: bool| if high { 3.3 } else { 0.0 };
    let names = ["a", "b", "c", "d", "e"];
    for count in 2..=5 {
        for pattern in 0..1_u32 << count {
            let inputs = (0..count)
                .map(|bit| pattern & (1 << bit) != 0)
                .collect::<Vec<_>>();
            let mut deck = String::from("* gates\n");
            for (name, &high) in names.iter().zip(&inputs) {
                deck.push_str(&format!("V{name} {name} 0 {}\n", level(high)));
            }
            let pins = (0..5)
                .map(|pin| if pin < count { names[pin] } else { "0" })
                .collect::<Vec<_>>()
                .join(" ");
            for (index, (gate, _)) in gates.iter().enumerate() {
                deck.push_str(&format!("X{index} {pins} y{index} {gate} VHIGH=3.3\n"));
            }
            deck.push_str(
                "XI a yi RSPICE_FN_INV VHIGH=3.3\nXB a yb RSPICE_FN_BUF VHIGH=3.3\n.end\n",
            );
            let nodes = ["y0", "y1", "y2", "y3", "y4", "yi", "yb"];
            let outputs = op_voltages(&deck, &nodes);
            let expected = gates
                .iter()
                .map(|(_, truth)| truth(&inputs))
                .chain([!inputs[0], inputs[0]]);
            for ((node, output), high) in nodes.iter().zip(outputs).zip(expected) {
                assert_close(&format!("{node} for {inputs:?}"), output, level(high), 1e-9);
            }
        }
    }
}

#[test]
fn an_input_is_high_from_the_midpoint_of_the_levels() {
    // VHIGH=3.3, VLOW=0.3: the threshold is 1.8 V.
    let deck = |input: f64| {
        format!("* threshold\nVA a 0 {input}\nX1 a y RSPICE_FN_BUF VHIGH=3.3 VLOW=0.3\n.end\n")
    };
    assert_close("below", op_voltages(&deck(1.79), &["y"])[0], 0.3, 1e-9);
    assert_close("at", op_voltages(&deck(1.8), &["y"])[0], 3.3, 1e-9);
}

#[test]
fn a_gate_output_moves_its_delay_after_the_input_and_slews_its_edge_times() {
    // A 0.1 ns input edge at 1 us crosses 2.5 V 0.05 ns in. A 5 ns pulse is
    // shorter than the 10 ns delay and never reaches the output.
    let deck = "* gate timing
VA a 0 PULSE(0 5 1u 0.1n 0.1n 1u 4u)
VG g 0 PULSE(0 5 1u 0.1n 0.1n 5n 4u)
X1 a y RSPICE_FN_BUF
X2 a yn RSPICE_FN_INV TFALL=20n
X3 g yg RSPICE_FN_BUF
.end
";
    let tran = transient(deck, 3e-6, 1e-9);
    let crossed = 1e-6 + 0.05e-9;
    let y = waveform(&tran, "y");
    let at = |level| rising_crossing(&tran.time, y, level);
    assert_close("buffer delay", at(2.5) - crossed, 10e-9 + 2.5e-9, 0.2e-9);
    assert_close("10-90% rise", at(4.5) - at(0.5), 0.8 * 5e-9, 0.2e-9);
    let yn = waveform(&tran, "yn");
    let at = |level| falling_crossing(&tran.time, yn, level);
    assert_close("inverter delay", at(2.5) - crossed, 10e-9 + 10e-9, 0.2e-9);
    assert_close("90-10% fall", at(0.5) - at(4.5), 0.8 * 20e-9, 0.2e-9);
    let glitch = waveform(&tran, "yg")
        .iter()
        .copied()
        .fold(f64::MIN, f64::max);
    assert_close("suppressed pulse", glitch, 0.0, 1e-6);
}

#[test]
fn schmitt_triggers_switch_half_their_hysteresis_either_side_of_their_threshold() {
    // A 0.5 V/us ramp up and back down; the output moves TD plus half an
    // edge, 1.5 ns, after the input crosses.
    let deck = "* schmitt
VR r 0 PWL(0 0 10u 5 20u 0)
X1 r ys RSPICE_FN_SCHMITT_BUF TD=1n TRISE=1n TFALL=1n
X2 r yi RSPICE_FN_SCHMITT_INV TD=1n TRISE=1n TFALL=1n VT=2 HYST=1
.end
";
    let tran = transient(deck, 20e-6, 10e-9);
    let ramp = waveform(&tran, "r");
    let input_at = |time: f64| value_at(&tran.time, ramp, time - 1.5e-9);
    let ys = waveform(&tran, "ys");
    assert_close(
        "buffer up",
        input_at(rising_crossing(&tran.time, ys, 2.5)),
        2.75,
        1e-3,
    );
    assert_close(
        "buffer down",
        input_at(falling_crossing(&tran.time, ys, 2.5)),
        2.25,
        1e-3,
    );
    let yi = waveform(&tran, "yi");
    assert_close(
        "inverter up",
        input_at(falling_crossing(&tran.time, yi, 2.5)),
        2.5,
        1e-3,
    );
    assert_close(
        "inverter down",
        input_at(rising_crossing(&tran.time, yi, 2.5)),
        1.5,
        1e-3,
    );
}

/// The times `wave` rises through `level`, interpolated.
fn rising_crossings(time: &[f64], wave: &[f64], level: f64) -> Vec<f64> {
    wave.windows(2)
        .enumerate()
        .filter(|(_, pair)| pair[0] < level && pair[1] >= level)
        .map(|(index, pair)| {
            let fraction = (level - pair[0]) / (pair[1] - pair[0]);
            time[index] + fraction * (time[index + 1] - time[index])
        })
        .collect()
}

/// The values the 555's Process preset fills in for a CMOS part; the
/// subcircuit's own defaults are the bipolar part.
const CMOS_555: &str = "DROP=0 ROUT=50 ISRC=10m ISNK=100m TEDGE=20n IQ=100u RDIV=100k";

#[test]
fn timer_555_astable_runs_at_its_textbook_frequency_and_duty_cycle_in_both_processes() {
    // RA=1k, RB=10k, C=10n: f = 1.44/((RA+2RB)C) = 6857 Hz, high for
    // (RA+RB)/(RA+2RB) = 52.4% of each period. The discharge transistor's
    // saturation stretches the low time by about 2%.
    for (tech, params) in [("Bipolar", ""), ("CMOS", CMOS_555)] {
        let deck = format!(
            "* 555 astable
VCC vcc 0 5
RA vcc dis 1k
RB dis thr 10k
C1 thr 0 10n
X1 thr thr vcc out dis ctl vcc 0 RSPICE_FN_555 {params}
CC ctl 0 10n
RL out 0 10k
.end
"
        );
        let tran = transient(&deck, 1.5e-3, 10e-9);
        let out = waveform(&tran, "out");
        let rises = rising_crossings(&tran.time, out, 1.5);
        assert!(rises.len() >= 8, "{tech}: {} cycles", rises.len());
        let first = rises[2];
        let last = rises[rises.len() - 1];
        let period = (last - first) / (rises.len() - 3) as f64;
        let frequency = 1.0 / period;
        let analytic = 1.44 / ((1e3 + 2.0 * 10e3) * 10e-9);
        assert!(
            (frequency / analytic - 1.0).abs() < 0.03,
            "{tech}: {frequency} Hz vs {analytic} Hz"
        );
        let settled = tran.time.iter().position(|&t| t >= first).unwrap();
        let fall = falling_crossing(&tran.time[settled..], &out[settled..], 1.5);
        let duty = (fall - first) / period;
        assert!((duty - 11.0 / 21.0).abs() < 0.015, "{tech}: duty {duty}");
        // The capacitor swings between a third and two thirds of VCC.
        let cap = &waveform(&tran, "thr")[settled..];
        let top = cap.iter().copied().fold(f64::MIN, f64::max);
        let bottom = cap.iter().copied().fold(f64::MAX, f64::min);
        assert_close("upper threshold", top, 5.0 * 2.0 / 3.0, 5e-3);
        assert_close("lower threshold", bottom, 5.0 / 3.0, 5e-3);
    }
}

#[test]
fn timer_555_monostable_pulse_is_one_point_one_rc_in_both_processes() {
    // R=10k, C=10n: a 1 us trigger at 20 us starts a 110 us pulse.
    for (tech, params) in [("Bipolar", ""), ("CMOS", CMOS_555)] {
        let deck = format!(
            "* 555 monostable
VCC vcc 0 5
VTR trig 0 PULSE(5 0 20u 10n 10n 1u 1)
R1 vcc cap 10k
C1 cap 0 10n
X1 trig cap vcc out cap ctl vcc 0 RSPICE_FN_555 {params}
CC ctl 0 10n
RL out 0 10k
.end
"
        );
        let tran = transient(&deck, 200e-6, 10e-9);
        let out = waveform(&tran, "out");
        let start = tran.time.iter().position(|&t| t >= 10e-6).unwrap();
        let rise = rising_crossing(&tran.time[start..], &out[start..], 1.5);
        let fall = falling_crossing(&tran.time[start..], &out[start..], 1.5);
        assert!((rise - 20e-6).abs() < 0.2e-6, "{tech}: triggered at {rise}");
        let width = fall - rise;
        assert!(
            (width / 110e-6 - 1.0).abs() < 0.01,
            "{tech}: pulse {width} s"
        );
    }
}

#[test]
fn timer_555_reset_holds_the_output_low() {
    let deck = "* 555 reset
VCC vcc 0 5
VRS rst 0 0.3
RA vcc dis 1k
RB dis thr 10k
C1 thr 0 10n
X1 thr thr rst out dis ctl vcc 0 RSPICE_FN_555
RL out 0 10k
.end
";
    // The latch keeps its power-on state through the operating point and
    // takes the reset at the first step.
    let tran = transient(deck, 200e-6, 10e-9);
    let start = tran.time.iter().position(|&t| t >= 1e-6).unwrap();
    let high = waveform(&tran, "out")[start..]
        .iter()
        .copied()
        .fold(f64::MIN, f64::max);
    assert!(high < 0.1, "output reached {high} under reset");
}

#[test]
fn phase_detector_pumps_its_current_for_the_lead_between_the_edges() {
    // A leads B by 0.2 us every 1 us: each cycle sources 100 uA for 0.2 us
    // into 1 nF, 20 mV; within the first lead the ramp is 0.1 V/us.
    let deck = "* phase detector
VA a 0 PULSE(0 5 1u 1n 1n 0.5u 1u)
VB b 0 PULSE(0 5 1.2u 1n 1n 0.5u 1u)
X1 a b pd RSPICE_FN_PHASEDET
CP pd 0 1n
X2 b a pdn RSPICE_FN_PHASEDET VLOW=-5
CN pdn 0 1n
.end
";
    let tran = transient(deck, 4.9e-6, 1e-9);
    let pd = waveform(&tran, "pd");
    assert_close(
        "first lead",
        value_at(&tran.time, pd, 1.15e-6),
        0.1495e-1,
        0.05e-3,
    );
    assert_close(
        "four cycles",
        value_at(&tran.time, pd, 4.9e-6),
        0.08,
        0.2e-3,
    );
    // B leading pumps the other way.
    let pdn = waveform(&tran, "pdn");
    assert_close("lagging", value_at(&tran.time, pdn, 4.9e-6), -0.08, 0.2e-3);
}

#[test]
fn counter_divides_its_clock_by_n_with_its_duty_cycle() {
    // A 1 MHz clock; each output rises once per N edges and stays high for
    // round(DUTY*N) of them.
    let deck = "* counter
VC clk 0 PULSE(0 5 1u 1n 1n 0.5u 1u)
VR rst 0 0
X2 clk rst q2 RSPICE_FN_COUNTER
X3 clk rst q3 RSPICE_FN_COUNTER N=3
X4 clk rst q4 RSPICE_FN_COUNTER N=4 DUTY=0.25
.end
";
    let tran = transient(deck, 14e-6, 1e-9);
    for (node, n, high_counts) in [("q2", 2.0, 1.0), ("q3", 3.0, 2.0), ("q4", 4.0, 1.0)] {
        let out = waveform(&tran, node);
        let rises = rising_crossings(&tran.time, out, 2.5);
        let period = rises[2] - rises[1];
        assert_close(node, period, n * 1e-6, 1e-9);
        let start = tran.time.iter().position(|&t| t >= rises[1]).unwrap();
        let fall = falling_crossing(&tran.time[start..], &out[start..], 2.5);
        assert_close(node, fall - rises[1], high_counts * 1e-6, 1e-9);
    }
}

#[test]
fn counter_reset_holds_the_count_at_zero() {
    // Held in reset the output stays at count zero, high, through the clock.
    let deck = "* counter reset
VC clk 0 PULSE(0 5 1u 1n 1n 0.5u 1u)
VR rst 0 5
X1 clk rst q RSPICE_FN_COUNTER N=2
.end
";
    let tran = transient(deck, 6e-6, 1e-9);
    let low = waveform(&tran, "q")
        .iter()
        .copied()
        .fold(f64::MAX, f64::min);
    assert!(low > 4.9, "reset output fell to {low}");
}

#[test]
fn sample_and_hold_tracks_while_sampling_and_holds_the_last_value() {
    // A 100 kHz, 1 V sine; SAMPLE is high from 2.5 us to 3.5 us.
    let deck = "* sample and hold
VS s 0 PULSE(0 5 2.5u 1n 1n 1u 4u)
VI in 0 SIN(0 1 100k)
X1 in s sh RSPICE_FN_SAMPLEHOLD
.end
";
    let tran = transient(deck, 6e-6, 1e-9);
    let sh = waveform(&tran, "sh");
    let input = waveform(&tran, "in");
    assert_close(
        "tracking",
        value_at(&tran.time, sh, 3e-6),
        value_at(&tran.time, input, 3e-6),
        5e-4,
    );
    // Held at the input's value at the falling edge, within the input's
    // 0.37 V/us slope over a step either side.
    let held = value_at(&tran.time, input, 3.5e-6);
    for at in [4e-6, 5e-6, 6e-6] {
        assert_close("holding", value_at(&tran.time, sh, at), held, 1e-3);
    }
}

#[test]
fn sample_and_hold_adds_its_offset_and_droops_at_its_rate_while_holding() {
    // As above, sampling from 2.5 us to 3.5 us and again from 6.5 us, with a
    // 10 mV offset and a 10 mV/us droop.
    let deck = "* sample and hold offset and droop
VS s 0 PULSE(0 5 2.5u 1n 1n 1u 4u)
VI in 0 SIN(0 1 100k)
X1 in s sh RSPICE_FN_SAMPLEHOLD VOS=10m DROOP=10k
.end
";
    let tran = transient(deck, 7e-6, 1e-9);
    let sh = waveform(&tran, "sh");
    let input = waveform(&tran, "in");
    let at = |wave: &[f64], time: f64| value_at(&tran.time, wave, time);
    assert_close("tracking", at(sh, 3e-6), at(input, 3e-6) + 10e-3, 5e-4);
    let held = at(input, 3.5e-6) + 10e-3;
    for time in [4e-6, 5e-6, 6e-6] {
        let drooped = held - 10e3 * (time - 3.5e-6);
        assert_close("drooping", at(sh, time), drooped, 1e-3);
    }
    // Sampling again clears the droop.
    assert_close(
        "tracking again",
        at(sh, 7e-6),
        at(input, 7e-6) + 10e-3,
        5e-4,
    );
}

/// The magnitude of the impedance a crystal with `parameters` presents at
/// each frequency, measured as the voltage an AC unit current develops.
fn crystal_impedance(parameters: &str, frequencies: &[f64]) -> Vec<f64> {
    let deck = format!(
        "* crystal impedance\nI1 0 a AC 1\nRDC a 0 1T\nX1 a 0 RSPICE_FN_CRYSTAL {parameters}\n.end\n"
    );
    Engine::default()
        .run_ac(&parse(&deck), frequencies)
        .expect("AC solves")
        .iter()
        .map(|point| {
            let node = point
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("a"))
                .expect("a solved");
            point.voltages[node].norm()
        })
        .collect()
}

/// The frequency within `span` (relative) of `centre` at which the crystal's
/// impedance is least, or greatest, on a grid of `span`/200 steps, and that
/// impedance.
fn crystal_extremum(parameters: &str, centre: f64, span: f64, least: bool) -> (f64, f64) {
    let frequencies = (-200..=200)
        .map(|step| centre * (1.0 + span * f64::from(step) / 200.0))
        .collect::<Vec<_>>();
    let impedance = crystal_impedance(parameters, &frequencies);
    let order = |a: &usize, b: &usize| impedance[*a].total_cmp(&impedance[*b]);
    let index = if least {
        (0..impedance.len()).min_by(order)
    } else {
        (0..impedance.len()).max_by(order)
    }
    .expect("a sweep");
    (frequencies[index], impedance[index])
}

#[test]
fn crystal_resonates_in_series_at_fs_through_its_esr_and_in_parallel_above_it() {
    // fs = 1/(2 pi sqrt(L1 C1)) and fp = fs sqrt(1 + C1/C0), with the
    // impedance at fs equal to ESR; the second case sets C1 through Q.
    // QSPICE batch with gmin=0, the same arm from its own L, C and R on a
    // 5 Hz grid: fs 10.000000 MHz, |Z(fs)| 24.9995 Ohm, fp 10.018735 MHz.
    let watch_c1 = 1.0 / (2.0 * std::f64::consts::PI * 32.768e3 * 50e3 * 35e3);
    for (parameters, fs, esr, c1, c0) in [
        ("", 10e6, 25.0, 15e-15, 4e-12),
        (
            "FS=32.768k ESR=35k C0=1.3p Q=50k",
            32.768e3,
            35e3,
            watch_c1,
            1.3e-12,
        ),
    ] {
        let (series, at_series) = crystal_extremum(parameters, fs, 1e-4, true);
        assert_close("fs", series, fs, 1e-6 * fs);
        assert_close("|Z(fs)|", at_series, esr, 1e-3 * esr);
        let fp = fs * (1.0 + c1 / c0).sqrt();
        let (parallel, _) = crystal_extremum(parameters, fp, 1e-4, false);
        assert_close("fp", parallel, fp, 1e-6 * fp);
    }
}

#[test]
fn an_overtone_crystal_also_resonates_at_its_fundamental() {
    // Marked at its third overtone: the fundamental resonates at FS/3
    // through ESR/9, and the marked mode keeps its own FS and ESR.
    let parameters = "FS=30Meg ESR=40 C0=4p C1=1.6f OVERTONE=3";
    let (fundamental, at_fundamental) = crystal_extremum(parameters, 10e6, 1e-4, true);
    assert_close("fs/3", fundamental, 10e6, 1e-6 * 10e6);
    assert_close("|Z(fs/3)|", at_fundamental, 40.0 / 9.0, 1e-3 * 40.0 / 9.0);
    let (series, at_series) = crystal_extremum(parameters, 30e6, 1e-4, true);
    assert_close("fs", series, 30e6, 1e-6 * 30e6);
    assert_close("|Z(fs)|", at_series, 40.0, 1e-3 * 40.0);
    // A fundamental crystal has no second resonance.
    let (_, below) = crystal_extremum("FS=30Meg ESR=40 C0=4p C1=1.6f", 10e6, 1e-4, true);
    assert!(below > 1e3, "{below}");
}

#[test]
fn varistor_clamps_at_its_breakdown_plus_its_current_through_rclamp_either_way() {
    // V = VBREAK + I RCLAMP beyond breakdown, the same either way; below it
    // no current flows. LTspice 26.1.1 batch, VARISTOR rclamp=5.6 on a
    // 430 V control: 710.0 V at 50 A and -542.0 V at -20 A.
    let deck = "* varistor
I1 0 a 50
RA a 0 1G
X1 a 0 RSPICE_FN_VARISTOR
I2 0 b -20
RB b 0 1G
X2 b 0 RSPICE_FN_VARISTOR
V3 c 0 400
X3 c 0 RSPICE_FN_VARISTOR
I4 0 d 2
RD d 0 1G
X4 d 0 RSPICE_FN_VARISTOR VBREAK=33 RCLAMP=0.5
.end
";
    let voltages = op_voltages(deck, &["a", "b", "d"]);
    assert_close("50 A", voltages[0], 710.0, 1e-3);
    assert_close("-20 A", voltages[1], -542.0, 1e-3);
    assert_close("2 A, 33 V part", voltages[2], 34.0, 1e-4);
    let below = op_currents(deck, &["V3"])[0];
    assert!(below.abs() < 1e-9, "{below}");
}

/// When the default 1 A, 0.5 A²s fuse opens under a 12 V source whose load
/// draws `current` through it from t = 0: where the voltage across it passes
/// half the source.
fn fuse_opening_time(current: f64, expected: f64) -> f64 {
    let load = 12.0 / current - 0.1;
    let deck = format!(
        "* fuse at {current} A
V1 in 0 PULSE(0 12 0 1u 1u 10 20)
RL in a {load}
X1 a 0 RSPICE_FN_FUSE
.end
"
    );
    let tran = transient(&deck, 1.5 * expected, expected / 2000.0);
    rising_crossing(&tran.time, waveform(&tran, "a"), 6.0)
}

#[test]
fn fuse_opens_at_its_melting_i2t_and_its_thermal_law() {
    // The element melts after -tau ln(1 - (IRATED/I)^2), tau = I2T/IRATED^2:
    // within a few percent of I2T/I^2 at five and ten times the rating, and
    // 15% slower than it at twice the rating, where the element cools while
    // it heats.
    let tau = 0.5;
    for (overload, adiabatic_tolerance) in [(5.0, 0.03), (10.0, 0.01)] {
        let adiabatic = 0.5 / (overload * overload);
        let opened = fuse_opening_time(overload, adiabatic);
        assert_close(
            "against I2T/I^2",
            opened,
            adiabatic,
            adiabatic_tolerance * adiabatic,
        );
    }
    let thermal = -tau * (1.0f64 - 0.25).ln();
    assert_close(
        "2x rating",
        fuse_opening_time(2.0, thermal),
        thermal,
        0.01 * thermal,
    );
}

#[test]
fn fuse_stays_open_once_blown_and_carries_its_rating_indefinitely() {
    // 3 A blows it near 59 ms; at 0.3 s the source falls to 0.5 A worth, yet
    // the element stays open. Beside it, 0.95 A for ten time constants
    // never melts the element.
    let deck = "* fuse latch
V1 in 0 PWL(0 0 1u 12 0.3 12 0.30001 2)
RL in a 3.9
X1 a 0 RSPICE_FN_FUSE
V2 in2 0 PWL(0 0 1u 12)
RL2 in2 b 12.5315789
X2 b 0 RSPICE_FN_FUSE
.end
";
    let tran = transient(deck, 5.0, 2e-4);
    let a = waveform(&tran, "a");
    let opened = rising_crossing(&tran.time, a, 6.0);
    let thermal = -0.5 * (1.0f64 - 1.0 / 9.0).ln();
    assert_close("3x rating", opened, thermal, 0.02 * thermal);
    assert_close("open at 1 s", value_at(&tran.time, a, 1.0), 2.0, 1e-6);
    assert_close("open at 5 s", value_at(&tran.time, a, 5.0), 2.0, 1e-6);
    let b = waveform(&tran, "b");
    assert_close("closed at 5 s", value_at(&tran.time, b, 5.0), 0.095, 1e-6);
}

#[test]
fn relay_pulls_in_and_drops_out_at_its_coil_voltages_after_its_operate_and_release_times() {
    // The coil ramps 0 -> 12 V over 1 s and back over the next: with its
    // 1.5 ms L/R lag (18 mV on the ramp) the contacts change over TOP after
    // the coil passes 9 V and TREL after it falls past 1.2 V. A second coil
    // steps to 12 V at 10 ms and off at 50 ms: its current crosses pull-in
    // after -L/R ln(1/4) and drop-out after L/R ln(10).
    let deck = "* relay
VC c 0 PWL(0 0 1 12 2 0)
X1 c 0 0 no nc RSPICE_FN_RELAY
VP p 0 5
RNO p no 1k
RNC p nc 1k
VS s 0 PWL(0 0 10m 0 10.001m 12 50m 12 50.001m 0)
X2 s 0 0 no2 nc2 RSPICE_FN_RELAY
RNO2 p no2 1k
RNC2 p nc2 1k
X3 s 0 0 no3 nc3 RSPICE_FN_RELAY FORM=0
RNO3 p no3 1k
RNC3 p nc3 1k
.end
";
    let tran = transient(deck, 2.0, 1e-4);
    let no = waveform(&tran, "no");
    let closed = falling_crossing(&tran.time, no, 2.5);
    assert_close("pull-in", 12.0 * (closed - 5e-3), 9.018, 0.01);
    let reopened = tran
        .time
        .iter()
        .zip(no)
        .position(|(&time, &v)| time > 1.5 && v > 2.5)
        .map(|index| tran.time[index])
        .expect("drops out");
    assert_close("drop-out", 12.0 * (2.0 - (reopened - 2e-3)), 1.182, 0.01);
    let nc = waveform(&tran, "nc");
    assert_close(
        "NC open while operated",
        value_at(&tran.time, nc, 1.0),
        5.0,
        1e-4,
    );
    assert_close(
        "NC closed at rest",
        value_at(&tran.time, nc, 0.5),
        2.5e-4,
        1e-6,
    );
    // Hysteresis: at 5 V the coil holds the contacts on the way down but
    // not on the way up.
    assert!(value_at(&tran.time, no, 5.0 / 12.0) > 4.9);
    assert!(value_at(&tran.time, no, 2.0 - 5.0 / 12.0) < 1e-3);

    let tau = 0.6 / 400.0;
    let step = transient(
        &deck.replace("VC c 0 PWL(0 0 1 12 2 0)", "VC c 0 0"),
        80e-3,
        2e-6,
    );
    let no2 = waveform(&step, "no2");
    let operate = falling_crossing(&step.time, no2, 2.5) - 10e-3;
    assert_close("operate", operate, -tau * 0.25f64.ln() + 5e-3, 20e-6);
    let release = step
        .time
        .iter()
        .zip(no2)
        .position(|(&time, &v)| time > 50e-3 && v > 2.5)
        .map(|index| step.time[index])
        .expect("releases")
        - 50e-3;
    assert_close("release", release, tau * 10f64.ln() + 2e-3, 20e-6);
    // A make contact never connects NC.
    let nc3 = waveform(&step, "nc3");
    assert!(nc3.iter().all(|&v| v > 5.0 - 1e-5));
}

#[test]
fn optocoupler_collector_current_is_ctr_times_led_current_and_follows_it_with_tau() {
    // 10 mA into the LED from 10 us: the collector, held 10 V above the
    // emitter through 100 Ohm, settles at CTR x 10 mA = 2 mA and reaches
    // 1 - 1/e of it TAU after the step. The LED drops VF at IFT, and a load
    // asking half the photocurrent holds the output at VCESAT.
    let deck = "* optocoupler
IF1 0 a1 PULSE(0 10m 10u 1n 1n 50u 100u)
RA1 a1 0 1Meg
X1 a1 0 c1 0 RSPICE_FN_OPTO
VCC vcc 0 10
RL vcc c1 100
VF2 a2 0 1.15
X2 a2 0 c2 0 RSPICE_FN_OPTO
VC2 c2 0 5
IF3 0 a3 10m
RA3 a3 0 1Meg
X3 a3 0 c3 0 RSPICE_FN_OPTO
IL3 c3 0 -1m
.end
";
    let tran = transient(deck, 60e-6, 10e-9);
    let collector = |at: f64| (10.0 - value_at(&tran.time, waveform(&tran, "c1"), at)) / 100.0;
    assert_close("CTR x IF", collector(59e-6), 2e-3, 2e-6);
    assert_close(
        "response at TAU",
        collector(12e-6) / collector(59e-6),
        1.0 - (-1.0f64).exp(),
        2e-3,
    );
    assert_close(
        "VF at IFT",
        value_at(&tran.time, waveform(&tran, "a1"), 59e-6),
        1.15,
        1e-3,
    );
    let currents = op_currents(deck, &["VC2"]);
    assert_close("CTR x IFT at VF", -currents[0], 2e-3, 2e-6);
    let saturated = op_voltages(deck, &["c3"])[0];
    assert_close("VCESAT at half the photocurrent", saturated, 0.4, 1e-3);
}

#[test]
fn diac_breaks_over_at_plus_and_minus_vbo_and_folds_back_by_dv() {
    // A source ramping to +40 V and then -40 V through 1 kOhm and a 100 Ohm
    // sense resistor: the diac blocks to +-VBO, leaking IBO there, then
    // conducts at VBO - DV + I RON either way.
    let deck = "* diac
V1 s 0 PWL(0 0 1m 40 2m 40 2.001m -40 3m -40)
R1 s a 1k
X1 a b RSPICE_FN_DIAC
RL b 0 100
.end
";
    let tran = transient(deck, 3e-3, 0.2e-6);
    let a = waveform(&tran, "a");
    let b = waveform(&tran, "b");
    let across = a.iter().zip(b).map(|(a, b)| a - b).collect::<Vec<_>>();
    let peak = across.iter().copied().fold(f64::MIN, f64::max);
    let trough = across.iter().copied().fold(f64::MAX, f64::min);
    assert_close("+VBO", peak, 32.0, 0.01);
    assert_close("-VBO", trough, -32.0, 0.01);
    let leak = value_at(&tran.time, b, 0.5e-3) / 100.0;
    let blocking = value_at(&tran.time, &across, 0.5e-3);
    assert_close("leakage", leak, blocking * 50e-6 / 32.0, 1e-9);
    for at in [1.5e-3, 2.5e-3] {
        let current = value_at(&tran.time, b, at) / 100.0;
        let on = value_at(&tran.time, &across, at);
        assert_close(
            "on state",
            on,
            (32.0 - 5.0 + 10.0 * current.abs()).copysign(current),
            1e-6,
        );
        assert!(current.abs() > 10e-3, "{current}");
    }
}

#[test]
fn scr_latches_on_a_gate_pulse_above_igt_and_turns_off_below_ih() {
    // 12 V through 100 Ohm. X1's 10 us gate pulse of 1.5 IGT turns it on,
    // and it stays on with the gate removed until the supply falls to
    // 1.3 V, where the anode current drops under IH; it then stays off when
    // the supply returns. X2's pulse of 0.9 IGT never turns it on.
    let deck = "* scr
VA s 0 PWL(0 12 100u 12 110u 1.3 200u 1.3 210u 12)
RL s a 100
IG 0 g PULSE(0 300u 10u 100n 100n 10u 1)
RG g 0 1Meg
X1 a g 0 RSPICE_FN_SCR
RL2 s a2 100
IG2 0 g2 PULSE(0 180u 10u 100n 100n 10u 1)
RG2 g2 0 1Meg
X2 a2 g2 0 RSPICE_FN_SCR
.end
";
    let tran = transient(deck, 300e-6, 0.1e-6);
    let a = waveform(&tran, "a");
    // On at 110 mA: VTM - RT (ITM - I).
    let on = 1.7 - 0.6 * (1.2 - (12.0 - value_at(&tran.time, a, 50e-6)) / 100.0);
    assert_close(
        "latched with the gate removed",
        value_at(&tran.time, a, 50e-6),
        on,
        5e-3,
    );
    assert_close("off below IH", value_at(&tran.time, a, 190e-6), 1.3, 1e-3);
    assert_close("stays off", value_at(&tran.time, a, 290e-6), 12.0, 1e-3);
    // X2 carries only its leakage, so its anode follows the supply.
    let (s, a2) = (waveform(&tran, "s"), waveform(&tran, "a2"));
    assert!(
        s.iter().zip(a2).all(|(s, a2)| s - a2 < 1e-3),
        "X2 turned on below IGT"
    );
    // VGT at IGT: the gate junction alone, at DC.
    let gate = op_voltages(
        "* scr gate\nIG 0 g 200u\nX1 a g 0 RSPICE_FN_SCR\nRA a 0 1k\n.end\n",
        &["g"],
    );
    assert_close("VGT at IGT", gate[0], 0.8, 1e-6);
}

#[test]
fn scr_holds_down_to_ih_and_needs_il_to_latch() {
    // Triggered, then left to its anode current as the supply ramps down:
    // the thyristor turns off where that current passes IH. Triggered into
    // 5.3 mA, between IH and IL, it drops out once the gate goes; into
    // 7.9 mA it stays on.
    let deck = "* scr hold
VA s 0 PWL(0 12 100u 12 5.1m 1.2)
RL s a 100
IG 0 g PULSE(0 1m 10u 100n 100n 20u 1)
RG g 0 1Meg
X1 a g 0 RSPICE_FN_SCR
VB s3 0 12
RL3 s3 a3 2.1k
X3 a3 g 0 RSPICE_FN_SCR
RL4 s3 a4 1.4k
X4 a4 g 0 RSPICE_FN_SCR
.end
";
    let tran = transient(deck, 5.1e-3, 1e-6);
    let (s, a) = (waveform(&tran, "s"), waveform(&tran, "a"));
    // Conducting, the anode sits under 1.1 V; off, it follows the supply,
    // still above 1.4 V when the current passes IH.
    let off = rising_crossing(&tran.time, a, 1.2);
    let before = off - 2e-6;
    let current = (value_at(&tran.time, s, before) - value_at(&tran.time, a, before)) / 100.0;
    assert!(
        (4.95e-3..5.15e-3).contains(&current),
        "released at {current} A"
    );
    let a3 = value_at(&tran.time, waveform(&tran, "a3"), 200e-6);
    assert!(a3 > 11.0, "latched below IL: {a3}");
    let a4 = value_at(&tran.time, waveform(&tran, "a4"), 200e-6);
    assert!(a4 < 2.0, "did not latch above IL: {a4}");
}

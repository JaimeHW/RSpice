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
    let rising = rising_crossing(&tran.time, &wave, 1.0) - rising_crossing(&tran.time, &wave, -1.0);
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
        let rise = rising_crossing(&tran.time, &output, 2.5) - 1.5e-9;
        let fall = falling_crossing(&tran.time, &output, 2.5) - 1.5e-9;
        let rising = value_at(&tran.time, &input, rise);
        let falling = value_at(&tran.time, &input, fall);
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
    let at = |level| rising_crossing(&tran.time, &out, level);
    assert_close("rising delay", at(2.5) - crossed_up, 100e-9 + 5e-9, 0.2e-9);
    assert_close("10-90% rise", at(4.5) - at(0.5), 0.8 * 10e-9, 0.2e-9);
    let at = |level| falling_crossing(&tran.time, &out, level);
    assert_close("falling delay", at(2.5) - crossed_down, 100e-9 + 10e-9, 0.2e-9);
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
    assert_close("sink law", sunk, 20e-3 * (low / (50.0 * 20e-3)).tanh(), 1e-9);
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

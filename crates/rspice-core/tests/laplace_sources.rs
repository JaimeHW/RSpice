//! LAPLACE controlled sources validated against closed forms in every
//! analysis domain: DC operating point, AC magnitude, and transient step
//! response. The parse-time state-space realization must behave exactly
//! like the transfer function it encodes.

use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

const FC: f64 = 1000.0; // 1 kHz corner for the first-order tests
const WC: f64 = 2.0 * std::f64::consts::PI * FC;

fn op_voltage(deck: &str, node: &str) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let op = engine.run_dc_op(&netlist).expect("operating point solves");
    let idx = op
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from OP result"));
    op.node_voltages[idx]
}

fn ac_magnitude(deck: &str, node: &str, freq: f64) -> f64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let results = engine.run_ac(&netlist, &[freq]).expect("ac solves");
    let result = &results[0];
    let idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from AC result"));
    result.voltages[idx].norm()
}

fn lowpass_deck() -> String {
    format!(
        "\
* first-order laplace lowpass
vin in 0 dc 1 ac 1
e1 out 0 laplace {{v(in)}} = {{1/(1+s/{WC})}}
rl out 0 1k
.end
"
    )
}

#[test]
fn lowpass_dc_gain_is_unity() {
    let v = op_voltage(&lowpass_deck(), "out");
    assert!((v - 1.0).abs() < 1e-6, "H(0) = 1: got {v}");
}

#[test]
fn lowpass_ac_matches_the_closed_form() {
    // |H(j2πf)| = 1/sqrt(1 + (f/fc)^2)
    for (freq, _label) in [
        (FC, "corner"),
        (10.0 * FC, "decade above"),
        (FC / 10.0, "decade below"),
    ] {
        let expected = 1.0 / (1.0 + (freq / FC).powi(2)).sqrt();
        let got = ac_magnitude(&lowpass_deck(), "out", freq);
        assert!(
            (got - expected).abs() / expected < 0.01,
            "|H| at {freq} Hz: got {got}, want {expected}"
        );
    }
}

#[test]
fn lowpass_step_response_matches_the_time_constant() {
    // Drive with a fast-edge pulse; v(out) = 1 - exp(-t/tau), tau = 1/wc.
    let tau = 1.0 / WC;
    let deck = format!(
        "\
* laplace lowpass step
vin in 0 pulse(0 1 0 1n 1n 1 2)
e1 out 0 laplace {{v(in)}} = {{1/(1+s/{WC})}}
rl out 0 1k
.tran {step} {stop}
.end
",
        step = tau / 200.0,
        stop = 3.0 * tau,
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let result = engine
        .run_tran(&netlist, 3.0 * tau, tau / 200.0)
        .expect("transient runs");
    let out_idx = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out present");

    for target in [0.5 * tau, 1.0 * tau, 2.0 * tau] {
        let (k, _) = result
            .time
            .iter()
            .enumerate()
            .min_by(|a, b| {
                (a.1 - target)
                    .abs()
                    .partial_cmp(&(b.1 - target).abs())
                    .unwrap()
            })
            .unwrap();
        let t = result.time[k];
        let got = result.voltages[out_idx][k];
        let expected = 1.0 - (-t / tau).exp();
        assert!(
            (got - expected).abs() < 0.01,
            "step response at t={t}: got {got}, want {expected}"
        );
    }
}

#[test]
fn second_order_resonance_peaks_at_one_over_two_zeta() {
    // H = wn^2 / (s^2 + 2*zeta*wn*s + wn^2), zeta = 0.2:
    // |H(j*wn)| = 1/(2*zeta) = 2.5 exactly at the undamped natural frequency.
    let fn_hz = 10_000.0;
    let wn = 2.0 * std::f64::consts::PI * fn_hz;
    let zeta = 0.2;
    let deck = format!(
        "\
* laplace resonator
vin in 0 dc 0 ac 1
e1 out 0 laplace {{v(in)}} = {{{wn2}/(s^2 + {two_zeta_wn}*s + {wn2})}}
rl out 0 1k
.end
",
        wn2 = wn * wn,
        two_zeta_wn = 2.0 * zeta * wn,
    );
    let got = ac_magnitude(&deck, "out", fn_hz);
    let expected = 1.0 / (2.0 * zeta);
    assert!(
        (got - expected).abs() / expected < 0.02,
        "resonant gain: got {got}, want {expected}"
    );

    let dc = op_voltage(&deck.replace("dc 0 ac 1", "dc 1"), "out");
    assert!((dc - 1.0).abs() < 1e-6, "resonator H(0)=1: got {dc}");
}

#[test]
fn highpass_with_feedthrough_blocks_dc_and_passes_high_frequencies() {
    // H = s/(s + wc): numerator degree equals denominator degree, so the
    // realization carries direct feedthrough d = 1.
    let deck = format!(
        "\
* laplace highpass
vin in 0 dc 1 ac 1
e1 out 0 laplace {{v(in)}} = {{s/(s+{WC})}}
rl out 0 1k
.end
"
    );
    let dc = op_voltage(&deck, "out");
    assert!(dc.abs() < 1e-6, "highpass blocks DC: got {dc}");

    let hf = ac_magnitude(&deck, "out", 100.0 * FC);
    assert!(
        (hf - 1.0).abs() < 0.01,
        "highpass passes two decades above the corner: got {hf}"
    );
    let corner = ac_magnitude(&deck, "out", FC);
    let expected = std::f64::consts::FRAC_1_SQRT_2;
    assert!(
        (corner - expected).abs() / expected < 0.01,
        "highpass corner magnitude: got {corner}, want {expected}"
    );
}

#[test]
fn pure_gain_laplace_is_a_plain_scale() {
    let deck = "\
* laplace pure gain
vin in 0 dc 1.2
e1 out 0 laplace {v(in)} = {2.5}
rl out 0 1k
.end
";
    let v = op_voltage(deck, "out");
    assert!((v - 3.0).abs() < 1e-9, "2.5 * 1.2 = 3.0: got {v}");
}

#[test]
fn g_form_matches_the_equivalent_linear_vccs() {
    let laplace = "\
* g-form laplace gain
vin in 0 dc 2
g1 0 out laplace {v(in)} = {2m}
rl out 0 1k
.op
.end
";
    let linear = "\
* linear vccs reference
vin in 0 dc 2
g1 0 out in 0 2m
rl out 0 1k
.op
.end
";
    let vl = op_voltage(laplace, "out");
    let vr = op_voltage(linear, "out");
    assert!(
        (vl - vr).abs() < 1e-9,
        "G LAPLACE pure gain must equal the linear VCCS: {vl} vs {vr}"
    );
}

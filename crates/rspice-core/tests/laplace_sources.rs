//! LAPLACE controlled sources validated against closed forms in every
//! analysis domain: DC operating point, AC magnitude, and transient step
//! response. The parse-time state-space realization must behave exactly
//! like the transfer function it encodes.

#[path = "common/op_point.rs"]
mod op_point;

use op_point::op_voltage;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

const FC: f64 = 1000.0; // 1 kHz corner for the first-order tests
const WC: f64 = 2.0 * std::f64::consts::PI * FC;

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

/// `exp(-s*1m)/(1+s*0.2m)` in LTspice's E form: QSPICE refuses the delay
/// factor ("Can't numerically factor"), so LTspice 26.1.1 decides.
const DELAYED_LOWPASS: &str = "E1 out 0 in 0 Laplace=exp(-s*1m)/(1+s*0.2m)";

fn delayed_lowpass_value(deck: &str, node: &str, frequency: f64) -> num_complex::Complex64 {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let results = Engine::new(SimulationConfig::default())
        .run_ac(&netlist, &[frequency])
        .expect("ac solves");
    let result = &results[0];
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case(node))
        .expect("node present");
    result.voltages[index]
}

#[test]
fn delay_factor_rotates_the_phase_without_touching_the_magnitude() {
    // LTspice 26.1.1 AC: |H| = 0.992197 at -43.162 deg at 100 Hz.
    let deck =
        format!("* delayed lowpass\nvin in 0 dc 1 ac 1\n{DELAYED_LOWPASS}\nrl out 0 1k\n.end\n");
    assert!((op_voltage(&deck, "out") - 1.0).abs() < 1e-8);
    for frequency in [100.0, 1e3, 1e4] {
        let omega = 2.0 * std::f64::consts::PI * frequency;
        let want = num_complex::Complex64::from_polar(1.0, -omega * 1e-3)
            / num_complex::Complex64::new(1.0, omega * 0.2e-3);
        let got = delayed_lowpass_value(&deck, "out", frequency);
        assert!(
            (got - want).norm() < 1e-6 * want.norm(),
            "H at {frequency} Hz: got {got}, want {want}"
        );
    }
}

#[test]
fn delay_factor_shifts_the_step_response() {
    // LTspice 26.1.1 transient: 0.392 at 1.6 ms and 0.918 at 2 ms.
    let deck = format!(
        "* delayed lowpass step\nvin in 0 pulse(0 1 0.5m 1u 1u 10 20)\n{DELAYED_LOWPASS}\nrl out 0 1k\n.end\n"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, 3e-3, 1e-6)
        .expect("transient runs");
    let out = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out present");
    let start = 0.5e-3 + 0.5e-6 + 1e-3;
    for (k, &t) in result.time.iter().enumerate() {
        let want = if t <= start {
            0.0
        } else {
            1.0 - (-(t - start) / 0.2e-3).exp()
        };
        let got = result.voltages[out][k];
        assert!((got - want).abs() < 5e-3, "t={t}: got {got}, want {want}");
    }
}

#[test]
fn irrational_laplace_forms_are_refused_by_name() {
    for (transfer, needle) in [
        ("sqrt(s)/(1+s)", "sqrt() is not realizable"),
        ("exp(-s*s*1m)", "must be a delay"),
    ] {
        let deck = format!("* refusal\nvin in 0 1\nE1 out 0 in 0 Laplace={transfer}\n.end\n");
        let error = Netlist::parse(&deck).expect_err("refused").to_string();
        assert!(error.contains(needle), "{transfer}: {error}");
    }
}

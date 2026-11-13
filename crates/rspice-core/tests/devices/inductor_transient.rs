//! Analytic regression pins for inductor transient companions.
//!
//! The vendored ngspice regression corpus contains no deck that runs an
//! inductor through `.tran`, which let a sign/coefficient error in the
//! inductor companion model (CompanionCoefficients::inductor_veq + the branch
//! stamp) survive while 105 decks passed: every L in transient made the
//! companion recursion non-contractive, compounding ~2x per step until node
//! voltages hit the +-1 kV clamp and the stepper collapsed to femtosecond dt.
//!
//! These tests pin RL and RLC step responses to closed-form solutions so any
//! future regression in the L companion path (any integration method) fails
//! loudly and immediately.

use rspice_core::{Engine, Netlist};

#[test]
fn mutual_inductance_extremes_preserve_ac_transfer() {
    let engine = Engine::default();
    for (inductance, turns) in [
        (1e200, 1.0_f64),
        (1e200, 2.0),
        (1e-200, 1.0),
        (1e-200, 2.0),
        (1e-160, 1.0),
        (1e-160, 2.0),
    ] {
        let secondary = inductance * turns * turns;
        for coupling in [-0.5, 0.0, 0.5] {
            let deck = Netlist::parse(&format!(
                "mutual product range\nV1 in 0 AC 1\nR1 in p 1\nL1 p 0 {inductance:e}\nL2 s 0 {secondary:e}\nK1 L1 L2 {coupling}\n.end\n"
            )).unwrap();
            let point = engine
                .run_ac(&deck, &[1.0 / inductance])
                .unwrap()
                .pop()
                .unwrap();
            let output = point
                .node_names
                .iter()
                .position(|name| name.eq_ignore_ascii_case("s"))
                .unwrap();
            let expected = Complex64::new(0.0, std::f64::consts::TAU * coupling * turns)
                / Complex64::new(1.0, std::f64::consts::TAU);
            assert!(
                (point.voltages[output] - expected).norm() < 2e-14,
                "L={inductance:e}, k={coupling}: {} vs {expected}",
                point.voltages[output]
            );
        }
    }
}

#[test]
fn signed_coupling_preserves_ac_phase_and_transient_startup() {
    use rspice_core::engine::{SimulationConfig, SpiceDialect};
    for dialect in [SpiceDialect::Ngspice, SpiceDialect::Xyce] {
        let engine = Engine::new(SimulationConfig::default().with_spice_dialect(dialect));
        for coefficient in [-0.75, 0.0, 0.75] {
            for (secondary, orientation) in [("s 0", 1.0), ("0 s", -1.0)] {
                let deck = Netlist::parse(&format!(
                    "signed transformer\nV1 in 0 DC 0.5 AC 1 SIN(0.5 1 1k)\nR1 in p 50\nL1 p 0 10m\nL2 {secondary} 40m\nK1 L1 L2 {coefficient}\n.end\n"
                )).unwrap();
                let dc = engine.run_dc_op(&deck).unwrap();
                for node in ["p", "s"] {
                    assert!(dc.try_voltage_named(node).unwrap().abs() < 1e-12);
                }
                let mutual = coefficient * orientation * 0.02;
                for point in engine.run_ac(&deck, &[100.0, 1000.0, 10000.0]).unwrap() {
                    let omega = std::f64::consts::TAU * point.frequency;
                    let expected =
                        Complex64::new(0.0, omega * mutual) / Complex64::new(50.0, omega * 0.01);
                    let index = point
                        .node_names
                        .iter()
                        .position(|name| name.eq_ignore_ascii_case("s"))
                        .unwrap();
                    assert!(
                        (point.voltages[index] - expected).norm() < 2e-12,
                        "{dialect:?}, k={coefficient}, orientation={orientation}, f={}: {} vs {expected}",
                        point.frequency,
                        point.voltages[index]
                    );
                }
                let transient = engine.run_tran(&deck, 1e-3, 1e-6).unwrap();
                let values = node_series(&transient.node_names, &transient.voltages, "s");
                let winding = transient
                    .branch_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("L1"))
                    .unwrap();
                let source = transient
                    .branch_names
                    .iter()
                    .position(|name| name.eq_ignore_ascii_case("V1"))
                    .unwrap();
                let omega = std::f64::consts::TAU * 1000.0;
                let impedance = Complex64::new(50.0, omega * 0.01);
                for (index, (&time, &actual)) in transient.time.iter().zip(values).enumerate() {
                    let tone = Complex64::from_polar(1.0, omega * time);
                    let decay = (-5000.0 * time).exp();
                    let expected = (mutual * (Complex64::new(0.0, omega) * tone + 5000.0 * decay)
                        / impedance)
                        .im;
                    let expected_current = 0.01 + ((tone - decay) / impedance).im;
                    assert!(
                        (actual - expected).abs() < 0.001,
                        "{dialect:?}, k={coefficient}, orientation={orientation}, t={time}: {actual} vs {expected}"
                    );
                    assert!(
                        (transient.branch_currents[winding][index] - expected_current).abs() < 1e-5
                    );
                    assert!(
                        (transient.branch_currents[winding][index]
                            + transient.branch_currents[source][index])
                            .abs()
                            < 1e-12
                    );
                }
            }
        }
    }
}

fn node_series<'a>(names: &[String], voltages: &'a [Vec<f64>], want: &str) -> &'a [f64] {
    let idx = names
        .iter()
        .position(|n| n.eq_ignore_ascii_case(want))
        .unwrap_or_else(|| panic!("node {want} not found in {names:?}"));
    &voltages[idx]
}

/// RL step: V(out) across L decays as V * exp(-t*R/L) after the source edge.
#[test]
fn rl_step_matches_analytic() {
    let deck = "\
* RL step response
V1 in 0 PULSE(0 1 1u 1u 1u 1 2)
R1 in out 35.19
L1 out 0 10m
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 3e-3, 10e-6)
        .expect("RL transient must converge");

    let v_out = node_series(&result.node_names, &result.voltages, "out");
    let r = 35.19f64;
    let l = 10e-3f64;
    // Edge midpoint: delay 1u + half the 1u rise.
    let t0 = 1.5e-6;

    let mut max_err = 0.0f64;
    for (k, &t) in result.time.iter().enumerate() {
        let v = v_out[k];
        assert!(
            v.is_finite() && v.abs() < 1.2,
            "V(out) diverged at t={t:.3e}: {v:.3e} (companion instability)"
        );
        if t < 10e-6 {
            continue; // skip the source edge itself
        }
        let analytic = (-(t - t0) * r / l).exp();
        max_err = max_err.max((v - analytic).abs());
    }
    assert!(
        max_err < 8e-3,
        "RL step deviates from analytic decay by {max_err:.3e} V (> 8 mV)"
    );
}

/// Series RLC (zeta = 0.35, f0 = 800 Hz): V(out) follows the textbook
/// underdamped unit-step response, including ~30.9 % overshoot.
#[test]
fn rlc_underdamped_matches_analytic() {
    let deck = "\
* series RLC unit step (zeta = 0.35, f0 = 800 Hz)
V1 in 0 PULSE(0 1 0 1u 1u 1 2)
R1 in n1 35.19
L1 n1 out 10m
C1 out 0 3957.9n
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 6e-3, 10e-6)
        .expect("RLC transient must converge");

    let v_out = node_series(&result.node_names, &result.voltages, "out");
    let zeta = 0.35f64;
    let w0 = 2.0 * std::f64::consts::PI * 800.0;
    let wd = w0 * (1.0 - zeta * zeta).sqrt();
    let t0 = 0.5e-6; // mid-rise of the 1u edge

    let mut max_err = 0.0f64;
    let mut v_peak = 0.0f64;
    for (k, &t) in result.time.iter().enumerate() {
        let v = v_out[k];
        assert!(
            v.is_finite() && v.abs() < 2.0,
            "V(out) diverged at t={t:.3e}: {v:.3e} (companion instability)"
        );
        v_peak = v_peak.max(v);
        if t < 5e-6 {
            continue;
        }
        let tt = t - t0;
        let analytic = 1.0
            - (-zeta * w0 * tt).exp()
                * ((wd * tt).cos() + zeta / (1.0 - zeta * zeta).sqrt() * (wd * tt).sin());
        max_err = max_err.max((v - analytic).abs());
    }
    assert!(
        max_err < 25e-3,
        "RLC step deviates from analytic response by {max_err:.3e} V (> 25 mV)"
    );
    // Analytic overshoot for zeta=0.35 is 30.9 %.
    assert!(
        (1.25..=1.36).contains(&v_peak),
        "RLC overshoot off: peak {v_peak:.4} V, expected ~1.309 V"
    );
    let v_final = *v_out.last().unwrap();
    assert!(
        (v_final - 1.0).abs() < 2e-3,
        "RLC did not settle to 1 V: final {v_final:.4} V"
    );
}

/// RL driven by a DC source starts at the operating point and must stay
/// there: V(out) is identically zero. The broken companion turned this
/// constant solution into a geometric blow-up.
#[test]
fn rl_dc_source_stays_at_operating_point() {
    let deck = "\
* RL with DC source: nothing should happen
V1 in 0 1
R1 in out 35.19
L1 out 0 10m
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 2e-3, 10e-6)
        .expect("DC RL transient must converge");

    let v_out = node_series(&result.node_names, &result.voltages, "out");
    for (k, &t) in result.time.iter().enumerate() {
        assert!(
            v_out[k].abs() < 1e-3,
            "constant-solution drift at t={t:.3e}: V(out)={:.3e}",
            v_out[k]
        );
    }
}

//=============================================================================
// Coupled inductors (K elements) — phasor-exact steady-state pins
//=============================================================================

use rspice_core::Complex64;

/// Last-cycle peak |v(node)| once the start-up transient has decayed.
fn late_cycle_peak(time: &[f64], v: &[f64], t_from: f64) -> f64 {
    time.iter()
        .zip(v)
        .filter(|(t, _)| **t >= t_from)
        .map(|(_, v)| v.abs())
        .fold(0.0f64, f64::max)
}

/// Exact steady-state secondary node amplitude for a transformer driven by
/// V*sin(wt) through r1, with secondary loaded by r_load:
///   (r1 + jwL1) I1 + jwM I2 = V
///   jwM I1 + (r_load + jwL2) I2 = 0,  v_sec = -I2 * r_load
/// Open secondary (r_load = inf): v_sec = jwM * V / (r1 + jwL1).
fn phasor_secondary_amplitude(
    v: f64,
    w: f64,
    r1: f64,
    l1: f64,
    l2: f64,
    m: f64,
    r_load: Option<f64>,
) -> f64 {
    let jw = Complex64::new(0.0, w);
    let z11 = Complex64::new(r1, 0.0) + jw * l1;
    match r_load {
        None => (jw * m * v / z11).norm(),
        Some(rl) => {
            let z12 = jw * m;
            let z22 = Complex64::new(rl, 0.0) + jw * l2;
            let det = z11 * z22 - z12 * z12;
            let i2 = -(z12 * v) / det;
            (i2 * rl).norm()
        }
    }
}

/// Open-secondary transformer: the induced EMF must match jwM*I1 exactly.
/// Before the fix the secondary sat at identically 0 V.
#[test]
fn coupled_open_secondary_emf_matches_phasor() {
    let deck = "\
* open-secondary transformer, K = 0.5
V1 in 0 SIN(0 1 1k)
R1 in p1 50
L1 p1 0 10m
L2 s1 0 10m
K1 L1 L2 0.5
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 5e-3, 5e-6)
        .expect("transformer transient must converge");
    let v_s1 = node_series(&result.node_names, &result.voltages, "s1");

    for (k, &t) in result.time.iter().enumerate() {
        assert!(
            v_s1[k].is_finite() && v_s1[k].abs() < 2.0,
            "secondary diverged at t={t:.3e}: {:.3e}",
            v_s1[k]
        );
    }

    let w = 2.0 * std::f64::consts::PI * 1000.0;
    let m = 0.5 * (10e-3f64 * 10e-3f64).sqrt();
    let expected = phasor_secondary_amplitude(1.0, w, 50.0, 10e-3, 10e-3, m, None);
    let measured = late_cycle_peak(&result.time, v_s1, 4e-3);
    assert!(
        measured > 0.05,
        "secondary is dead ({measured:.4} V) — coupling not stamped"
    );
    let rel = (measured - expected).abs() / expected;
    assert!(
        rel < 0.03,
        "open-secondary EMF off by {:.1}%: measured {measured:.4} V, phasor {expected:.4} V",
        rel * 100.0
    );
}

/// Loaded 1:1 transformer, K = 0.98: secondary amplitude must match the exact
/// two-mesh phasor solution.
#[test]
fn coupled_loaded_transformer_matches_phasor() {
    let deck = "\
* loaded 1:1 transformer, K = 0.98
V1 in 0 SIN(0 1 1k)
R1 in p1 10
L1 p1 0 100m
L2 s1 0 100m
K1 L1 L2 0.98
RL s1 0 1k
.end
";
    let netlist = Netlist::parse(deck).expect("parse");
    let result = Engine::default()
        .run_tran(&netlist, 5e-3, 5e-6)
        .expect("loaded transformer transient must converge");
    let v_s1 = node_series(&result.node_names, &result.voltages, "s1");

    let w = 2.0 * std::f64::consts::PI * 1000.0;
    let m = 0.98 * (100e-3f64 * 100e-3f64).sqrt();
    let expected = phasor_secondary_amplitude(1.0, w, 10.0, 100e-3, 100e-3, m, Some(1000.0));
    let measured = late_cycle_peak(&result.time, v_s1, 4e-3);
    assert!(
        measured > 0.1,
        "secondary is dead ({measured:.4} V) — coupling not stamped"
    );
    let rel = (measured - expected).abs() / expected;
    assert!(
        rel < 0.03,
        "loaded secondary off by {:.1}%: measured {measured:.4} V, phasor {expected:.4} V",
        rel * 100.0
    );
}

/// Run a deck under its configuration's dialect, parsing it as that dialect
/// reads it, with Xyce's default maximum step: a tenth of the window.
fn run_tran_in(
    config: rspice_core::engine::SimulationConfig,
    deck: &str,
    stop: f64,
) -> rspice_core::engine::TransientResult {
    run_tran_with_max_step(config, deck, stop, 0.1 * stop)
}

/// Run a deck under its configuration's dialect, parsing it as that dialect
/// reads it.
fn run_tran_with_max_step(
    config: rspice_core::engine::SimulationConfig,
    deck: &str,
    stop: f64,
    step: f64,
) -> rspice_core::engine::TransientResult {
    let dialect = config.spice_dialect;
    let netlist = Netlist::parse_with_options(
        deck,
        rspice_core::netlist::NetlistParseOptions {
            spice_dialect: Some(dialect),
            ..Default::default()
        },
    )
    .expect("parse");
    Engine::new(config)
        .run_tran(&netlist, stop, step)
        .unwrap_or_else(|error| panic!("{dialect:?} transient failed: {error}"))
}

/// The value a node publishes at an exact breakpoint.
fn published_at(result: &rspice_core::engine::TransientResult, node: &str, time: f64) -> f64 {
    let index = result
        .time
        .iter()
        .position(|sample| (sample - time).abs() <= 1e-12)
        .unwrap_or_else(|| panic!("no sample at {time:e} s"));
    node_series(&result.node_names, &result.voltages, node)[index]
}

/// A current ramp forced through an inductor (Xyce INDUCTOR/inductor.cir and
/// BUG_458): V(2) = L di/dt is -50 V on the 1 ms rise, 0 on the top and
/// +50 V on the 1 ms fall. SPICE publishes the operating point at t = 0 and,
/// at each corner, the solution of the step that ends there, its left limit.
/// ngspice 46 and Xyce 7.10 both print V(1), V(2) = 0, 0 at t = 0;
/// -50.005, -50 at 1 ms; -0.005, 0 at 11 ms; 50, 50 at 12 ms. The Xyce
/// dialect, whose ideal current-source cutset takes the physical event path,
/// published the outgoing state instead: -50 V at t = 0 and the next ramp at
/// every corner.
#[test]
fn a_current_ramp_into_an_inductor_publishes_the_operating_point_and_each_corners_left_limit() {
    use rspice_core::engine::SpiceDialect;
    let deck = "\
Netlist to Test the Xyce Inductor Model
I1 1 0 PULSE(0 5V 0 1MS 1MS 10MS 25MS)
L1 2 0 10mH
R1 1 2 0.001
.TRAN 0.1MS 20MS
.PRINT TRAN V(1) V(2)
.END
";
    for dialect in [
        SpiceDialect::Xyce,
        SpiceDialect::Ngspice,
        SpiceDialect::BestAvailable,
    ] {
        let config = rspice_core::engine::SimulationConfig::default().with_spice_dialect(dialect);
        let result = run_tran_in(config, deck, 20e-3);
        for (time, v1, v2) in [
            (0.0, 0.0, 0.0),
            (1e-3, -50.005, -50.0),
            (11e-3, -0.005, 0.0),
            (12e-3, 50.0, 50.0),
        ] {
            for (node, expected) in [("1", v1), ("2", v2)] {
                let actual = published_at(&result, node, time);
                assert!(
                    (actual - expected).abs() <= 1e-6 * expected.abs() + 1e-9,
                    "{dialect:?} V({node}) at {time:e} s: {actual} != {expected}"
                );
            }
        }
    }
}

/// The same ramp under the Xyce deck's own options at a small maximum step.
/// A fixed step added up from the 12 ms corner stops a few ulps short of the
/// 20 ms horizon (0.1 ms: 4.9e-17 s, above Xyce's floor there), and solving
/// that remainder made the inductor's companion singular. Xyce 7.10 never
/// solves it (`StepErrorControl::isFinished`); with `.options timeint
/// delmax=1e-4` (and 4e-5, 2e-5) it prints V(1), V(2) = -50.005, -50 at
/// 1 ms; -0.005, 0 at 11 ms; 50, 50 at 12 ms; and 0, 0 within 2e-14 at 20 ms.
#[test]
fn a_current_ramp_into_an_inductor_reaches_the_horizon_at_small_xyce_steps() {
    use rspice_core::engine::SpiceDialect;
    let deck = "Netlist to Test the Xyce Inductor Model
I1 1 0 PULSE(0 5V 0 1MS 1MS 10MS 25MS)
L1 2 0 10mH
R1 1 2 0.001
.options timeint newbpstepping=0 reltol=1.0e-4
.TRAN 0.1MS 20MS
.PRINT TRAN V(1) V(2)
.END
";
    for max_step in [1e-4, 4e-5, 2e-5] {
        let config =
            rspice_core::engine::SimulationConfig::default().with_spice_dialect(SpiceDialect::Xyce);
        let result = run_tran_with_max_step(config, deck, 20e-3, max_step);
        assert_eq!(result.time.last().copied(), Some(20e-3));
        for (time, v1, v2) in [
            (1e-3, -50.005, -50.0),
            (11e-3, -0.005, 0.0),
            (12e-3, 50.0, 50.0),
            (20e-3, 0.0, 0.0),
        ] {
            for (node, expected) in [("1", v1), ("2", v2)] {
                let actual = published_at(&result, node, time);
                assert!(
                    (actual - expected).abs() <= 1e-6 * expected.abs() + 1e-9,
                    "max step {max_step:e}: V({node}) at {time:e} s: {actual} != {expected}"
                );
            }
        }
    }
}

/// Xyce MULTIPLICITY_FACTOR/inductor.cir: an inductor of M parallel copies
/// is L/M, and LMOD's TC1=0.2 from TNOM=20 makes L x3 at 30 C, under the
/// same current ramps. Each branch's V(xA) at t = 0 and at the corners is
/// what Xyce 7.10 prints at `.options device TEMP=20` and `TEMP=30`: the
/// operating point, then the left limit of each ramp, 0 at 11 ms and L di/dt
/// at 1 ms and 12 ms.
#[test]
fn multiplicity_inductors_publish_each_corners_left_limit_like_xyce() {
    use rspice_core::engine::SpiceDialect;
    for (temperature, l2_multiplicity, millihenries) in [
        (20, "0.5", [10.0, 10.0, 5.0, 5.0, 5.0, 15.0]),
        (30, "1", [10.0, 5.0, 5.0, 15.0, 15.0, 15.0]),
    ] {
        let deck = format!(
            "Test the multiplicity factor (M) for Inductor device
I1 1 0 PULSE(0 5V 0 1MS 1MS 10MS 25MS)
L1 1a 0 L=10mH
R1 1 1a 0.001
I2 2 0 PULSE(0 5V 0 1MS 1MS 10MS 25MS)
L2 2a 0 L=5mH M={l2_multiplicity}
R2 2 2a 0.001
I3 3 0 PULSE(0 5V 0 1MS 1MS 10MS 25MS)
L3 3a 0 L=5mH
R3 3 3a 0.001
I4 4 0 PULSE(0 5V 0 1MS 1MS 10MS 25MS)
L4 4a 0 LMOD L=10mH M=2
R4 4 4a 0.001
I5 5 0 PULSE(0 5V 0 1MS 1MS 10MS 25MS)
L5 5a 0 LMOD L=5mH
R5 5 5a 0.001
.MODEL LMOD L(TNOM=20 TC1=0.2)
I6 6 0 PULSE(0 5V 0 1MS 1MS 10MS 25MS)
L6 6a 0 L=15mH
R6 6 6a 0.001
.options timeint newbpstepping=0 reltol=1.0e-4
.TRAN 0.1MS 20MS
.END
"
        );
        let config = rspice_core::engine::SimulationConfig {
            temperature: 273.15 + f64::from(temperature),
            ..rspice_core::engine::SimulationConfig::default()
                .with_spice_dialect(SpiceDialect::Xyce)
        };
        let result = run_tran_in(config, &deck, 20e-3);
        for (branch, millihenries) in millihenries.into_iter().enumerate() {
            let node = format!("{}a", branch + 1);
            // L di/dt for a 5 A ramp over 1 ms.
            let ramp = millihenries * 5.0;
            for (time, expected) in [(0.0, 0.0), (1e-3, -ramp), (11e-3, 0.0), (12e-3, ramp)] {
                let actual = published_at(&result, &node, time);
                assert!(
                    (actual - expected).abs() <= 1e-6 * expected.abs() + 1e-9,
                    "{temperature} C, V({node}) at {time:e} s: {actual} != {expected}"
                );
            }
        }
    }
}

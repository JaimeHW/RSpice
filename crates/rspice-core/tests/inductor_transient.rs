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

fn node_series<'a>(
    names: &[String],
    voltages: &'a [Vec<f64>],
    want: &str,
) -> &'a [f64] {
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
    let expected =
        phasor_secondary_amplitude(1.0, w, 10.0, 100e-3, 100e-3, m, Some(1000.0));
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

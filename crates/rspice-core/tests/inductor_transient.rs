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

//! Shooting-PSS validation against closed-form periodic steady states.
//!
//! These are the first analytic-truth gates for the shooting solver (the RF
//! roadmap's Tier-0 policy): a sine-driven RC has an exact sinusoidal steady
//! state, so the converged orbit, the periodicity residual, and the Floquet
//! multiplier are all checkable without any reference simulator.

use rspice_core::analysis::PssConfig;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

const F0: f64 = 1.0e6; // 1 MHz drive
const R: f64 = 1.0e3;
const C: f64 = 159.154943091895e-12; // RC corner ~ 1 MHz (w*RC = 1)

fn run_rc_pss() -> rspice_core::engine::PssAnalysisResult {
    let deck = format!(
        "\
* sine-driven rc
v1 in 0 sin(0 1 {F0})
r1 in out {R}
c1 out 0 {C}
.end
"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let config = PssConfig::new(F0).with_tstab_periods(8).with_tolerance(1e-7);
    engine.run_pss(&netlist, config).expect("PSS converges")
}

#[test]
fn rc_steady_state_matches_the_analytic_solution() {
    let result = run_rc_pss();

    // Periodicity itself: the converged orbit closes.
    assert!(
        result.final_residual < 1e-4,
        "periodicity residual must be small, got {}",
        result.final_residual
    );

    // Closed form: |H| = 1/sqrt(1 + (wRC)^2) with wRC = 1 -> amplitude
    // 1/sqrt(2) = 0.7071 V on the capacitor.
    let pss = &result.result;
    let out_idx = pss
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .unwrap_or_else(|| panic!("out missing from PSS waveforms: {:?}", pss.node_names));

    let amplitude = pss.waveforms[out_idx]
        .values
        .iter()
        .fold(0.0f64, |acc, v| acc.max(v.abs()));
    let expected = std::f64::consts::FRAC_1_SQRT_2;
    assert!(
        (amplitude - expected).abs() / expected < 0.02,
        "capacitor amplitude within 2% of 1/sqrt(2): got {amplitude}"
    );
}

#[test]
fn rc_floquet_multiplier_matches_exp_minus_t_over_rc() {
    let result = run_rc_pss();

    // One reactive state: the single Floquet multiplier of a linear RC is
    // exactly exp(-T/RC), independent of the drive.
    assert_eq!(
        result.floquet_multipliers.len(),
        1,
        "one reactive state -> one multiplier"
    );
    let mu = result.floquet_multipliers[0].norm();
    let expected = (-(1.0 / F0) / (R * C)).exp();
    // The multiplier is tiny (exp(-2*pi) ~ 1.9e-3) and comes from a
    // finite-difference Jacobian, so demand the right order of magnitude
    // and sign rather than tight relative accuracy — a zero or order-one
    // multiplier must fail.
    assert!(
        mu > 0.5 * expected && mu < 2.0 * expected,
        "Floquet multiplier within 2x of exp(-T/RC): got {mu}, want {expected}"
    );
    assert!(
        result.floquet_multipliers[0].im.abs() < 1e-6,
        "RC multiplier is real"
    );
}

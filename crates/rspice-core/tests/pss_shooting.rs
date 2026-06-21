//! Shooting-PSS validation against closed-form periodic steady states.
//!
//! These are the first analytic-truth gates for the shooting solver (the RF
//! roadmap's Tier-0 policy): a sine-driven RC has an exact sinusoidal steady
//! state, so the converged orbit, the periodicity residual, and the Floquet
//! multiplier are all checkable without any reference simulator.

use rspice_core::analysis::PssConfig;
use rspice_core::engine::{Engine, SimulationConfig, SimulationError};
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
    let config = PssConfig::new(F0)
        .with_tstab_periods(8)
        .with_tolerance(1e-7);
    engine.run_pss(&netlist, config).expect("PSS converges")
}

#[test]
fn pss_rejects_zero_max_iterations_as_invalid_config() {
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

    let err = engine
        .run_pss(&netlist, PssConfig::new(F0).with_max_iterations(0))
        .expect_err("zero max_iterations must be rejected");

    match err {
        SimulationError::Circuit(message) => {
            assert_eq!(message, "Invalid PSS config: max_iterations must be > 0");
        }
        other => panic!("expected invalid PSS config error, got {other:?}"),
    }
}

#[test]
fn pss_rejects_invalid_public_numeric_config_as_invalid_config() {
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

    let invalid_cases = vec![
        (
            "non-finite fundamental",
            {
                let mut config = PssConfig::new(f64::NAN);
                config.period_guess = 1e-9;
                config
            },
            "Invalid PSS config: fundamental_freq must be finite and >= 0",
        ),
        (
            "negative fundamental",
            {
                let mut config = PssConfig::new(-F0);
                config.period_guess = 1e-9;
                config
            },
            "Invalid PSS config: fundamental_freq must be finite and >= 0",
        ),
        (
            "negative tstab",
            PssConfig::new(F0).with_tstab(-1e-6),
            "Invalid PSS config: tstab must be finite and >= 0",
        ),
        (
            "zero tolerance",
            PssConfig::new(F0).with_tolerance(0.0),
            "Invalid PSS config: tolerance must be finite and > 0",
        ),
        (
            "non-finite abstol",
            {
                let mut config = PssConfig::new(F0);
                config.abstol = f64::INFINITY;
                config
            },
            "Invalid PSS config: abstol must be finite and > 0",
        ),
        (
            "invalid period guess",
            {
                let mut config = PssConfig::autonomous();
                config.period_guess = 0.0;
                config
            },
            "Invalid PSS config: period_guess must be finite and > 0",
        ),
        (
            "out of range damping",
            {
                let mut config = PssConfig::new(F0);
                config.damping_factor = 1.5;
                config
            },
            "Invalid PSS config: damping_factor must be finite and in [0.1, 1.0]",
        ),
        (
            "invalid period change",
            {
                let mut config = PssConfig::autonomous();
                config.max_period_change = f64::NAN;
                config
            },
            "Invalid PSS config: max_period_change must be finite and > 0",
        ),
        (
            "invalid grid density",
            {
                let mut config = PssConfig::new(F0);
                config.points_per_period = 0;
                config
            },
            "Invalid PSS config: points_per_period must be >= 16",
        ),
    ];

    for (case, config, expected) in invalid_cases {
        let err = match engine.run_pss(&netlist, config) {
            Ok(_) => panic!("{case} must be rejected"),
            Err(err) => err,
        };

        match err {
            SimulationError::Circuit(message) => {
                assert_eq!(message, expected, "{case}");
            }
            other => panic!("{case}: expected invalid PSS config error, got {other:?}"),
        }
    }
}

#[test]
fn driven_pss_ignores_autonomous_period_controls() {
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
    let mut config = PssConfig::new(F0)
        .with_tstab_periods(8)
        .with_tolerance(1e-7);
    config.period_guess = 0.0;
    config.max_period_change = f64::NAN;

    let result = engine
        .run_pss(&netlist, config)
        .expect("driven PSS must ignore autonomous-only period controls");

    assert!(
        result.final_residual < 1e-4,
        "periodicity residual must be small, got {}",
        result.final_residual
    );
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
    // The fixed-grid period map is smooth in the initial state, so the
    // central-difference monodromy reaches real derivative accuracy: demand
    // 1% on exp(-T/RC) ~ 1.87e-3, which the adaptive-grid forward
    // difference could never deliver.
    assert!(
        (mu - expected).abs() < 0.01 * expected,
        "Floquet multiplier within 1% of exp(-T/RC): got {mu}, want {expected}"
    );
    assert!(
        result.floquet_multipliers[0].im.abs() < 1e-6,
        "RC multiplier is real"
    );
}

/// A square-wave-driven RC has a closed-form periodic steady state: with
/// a = exp(-T/(2RC)), the capacitor rides exponential segments between
/// V_min = a/(1+a) and V_max = 1/(1+a). Landing on the PULSE edges requires
/// the PSS integrator to honor source breakpoints; without them the orbit
/// smears by an LTE-sized step at every edge.
#[test]
fn pulse_driven_rc_matches_the_closed_form_steady_state() {
    // T = 1us, RC = T/2 -> a = exp(-1).
    let deck = "\
* square-wave rc
v1 in 0 pulse(0 1 0 1n 1n 0.499u 1u)
r1 in out 1k
c1 out 0 0.5n
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let config = PssConfig::new(1.0e6)
        .with_tstab_periods(8)
        .with_tolerance(1e-7);
    let result = engine.run_pss(&netlist, config).expect("PSS converges");

    let pss = &result.result;
    let out_idx = pss
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node present");
    let values = &pss.waveforms[out_idx].values;
    let v_max = values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let v_min = values.iter().cloned().fold(f64::INFINITY, f64::min);

    let a = (-1.0f64).exp();
    let expected_max = 1.0 / (1.0 + a);
    let expected_min = a / (1.0 + a);

    assert!(
        (v_max - expected_max).abs() < 0.015 * expected_max,
        "steady-state peak must be 1/(1+e^-1): got {v_max:.5}, want {expected_max:.5}"
    );
    assert!(
        (v_min - expected_min).abs() < 0.015 * expected_min,
        "steady-state trough must be e^-1/(1+e^-1): got {v_min:.5}, want {expected_min:.5}"
    );
}

/// Autonomous shooting: a weakly nonlinear LC negative-resistance oscillator
/// (van der Pol form, eps = g1*sqrt(L/C) = 0.05) has period
/// T = 2*pi*sqrt(LC)*(1 + eps^2/16 + ...), within 0.02% of 2*pi*sqrt(LC),
/// and a describing-function amplitude sqrt(4*g1/(3*g3)). The period must
/// come out of the (n+1)-unknown Newton, not the coarse detector, and the
/// Floquet spectrum must carry the structural unity multiplier of an
/// autonomous orbit.
#[test]
fn lc_oscillator_period_solves_to_the_analytic_value() {
    // L = C = 1u -> sqrt(LC) = 1us, T0 = 6.28319us, sqrt(L/C) = 1 ohm.
    let deck = "* negative-resistance lc oscillator
l1 osc 0 1u
c1 osc 0 1u
b1 osc 0 i=-0.05*v(osc)+0.025*v(osc)*v(osc)*v(osc)
i1 0 osc pulse(0 1 10u 10n 10n 1u 1)
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let config = PssConfig::autonomous()
        .with_period_guess(6.3e-6)
        .with_tstab_periods(30)
        .with_tolerance(1e-6)
        .with_max_iterations(60);
    let result = engine.run_pss(&netlist, config).expect("PSS converges");

    let t0 = 2.0 * std::f64::consts::PI * 1.0e-6;
    let eps: f64 = 0.05;
    let t_expected = t0 * (1.0 + eps * eps / 16.0);
    assert!(
        (result.period - t_expected).abs() < 1e-3 * t_expected,
        "oscillator period must solve to the van der Pol value: got {:.6e}, want {:.6e}",
        result.period,
        t_expected
    );

    // Structural unity Floquet multiplier of the autonomous orbit.
    let unity_error = result
        .floquet_multipliers
        .iter()
        .map(|m| (m - num_complex::Complex64::new(1.0, 0.0)).norm())
        .fold(f64::INFINITY, f64::min);
    assert!(
        unity_error < 0.05,
        "autonomous orbit must carry a unity Floquet multiplier; nearest is {unity_error:.3} away; all: {:?}",
        result.floquet_multipliers
    );

    // Describing-function amplitude sqrt(4*0.05/(3*0.025)) = 1.633 V.
    let pss = &result.result;
    let osc_idx = pss
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("osc"))
        .expect("osc node present");
    let amplitude = pss.waveforms[osc_idx]
        .values
        .iter()
        .fold(0.0f64, |acc, v| acc.max(v.abs()));
    let a_expected = (4.0f64 * 0.05 / (3.0 * 0.025)).sqrt();
    assert!(
        (amplitude - a_expected).abs() < 0.04 * a_expected,
        "limit-cycle amplitude must match the describing function: got {amplitude:.4}, want {a_expected:.4}"
    );
}

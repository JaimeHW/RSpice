//! Oscillator phase noise (Demir PPV) against the closed-form RLC result.
//!
//! For a high-Q parallel-RLC negative-resistance oscillator whose only
//! modeled noise source is the tank resistor, the adjoint unity Floquet
//! mode of the (weakly perturbed) symmetric LC orbit is exactly
//! v1(t) = (cos(w0 t), Z0 sin(w0 t)) / (A w0), which satisfies Demir's
//! normalization v1^T(t) ds/dt = 1 identically. Projecting the tank-node
//! current source b = (1/C, 0) gives
//!
//!   c = (1/T) integral S_i cos^2(w0 t) / (A w0 C)^2 dt
//!     = S_i / (2 A^2 w0^2 C^2),       S_i = 4 k T / R,
//!
//! and the carrier-normalized Lorentzian sideband (paper Eq. 23/24)
//! L(f_m) = f0^2 c / (pi^2 f0^4 c^2 + f_m^2) falls at exactly
//! -20 dB/decade in the white region. The gate pins c, the absolute level,
//! and the slope.

use rspice_core::analysis::PssConfig;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

const K_B: f64 = 1.380649e-23;
const T_REF: f64 = 300.15;

#[test]
fn rlc_oscillator_phase_noise_matches_the_demir_constant() {
    // L = C = 1u (Z0 = 1 ohm, f0 = 159.155 kHz), tank R = 1k. The cubic
    // b-source restores the net -0.05 S small-signal startup conductance,
    // so the orbit keeps the van der Pol amplitude A = sqrt(4*0.05/0.075).
    let deck = "\
* noisy negative-resistance lc oscillator
l1 osc 0 1u
c1 osc 0 1u
r1 osc 0 1k
b1 osc 0 i=-0.051*v(osc)+0.025*v(osc)*v(osc)*v(osc)
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

    let offsets = [1.0e3, 1.0e4, 1.0e5];
    let result = engine
        .run_pnoise_oscillator(&netlist, config, &offsets)
        .expect("oscillator pnoise completes");

    // Analytic diffusion constant for the tank-resistor source.
    let (l, c_tank, r): (f64, f64, f64) = (1.0e-6, 1.0e-6, 1.0e3);
    let w0 = 1.0 / (l * c_tank).sqrt();
    let f0 = w0 / (2.0 * std::f64::consts::PI);
    let a2 = 4.0 * (0.051 - 1.0 / r) / (3.0 * 0.025); // describing function
    let s_i = 4.0 * K_B * T_REF / r;
    let c_expected = s_i / (2.0 * a2 * w0 * w0 * c_tank * c_tank);

    assert!(
        (result.diffusion_constant - c_expected).abs() < 0.05 * c_expected,
        "diffusion constant must match the closed-form adjoint projection: \
         got {:.4e}, want {:.4e}",
        result.diffusion_constant,
        c_expected
    );

    // Absolute level in the white (-20 dB/dec) region: L = f0^2 c / f_m^2.
    for (i, &fm) in offsets.iter().enumerate() {
        let expected_dbc = 10.0 * (f0 * f0 * c_expected / (fm * fm)).log10();
        assert!(
            (result.phase_noise_dbc[i] - expected_dbc).abs() < 1.0,
            "L({fm:.0e}) must sit within 1 dB of the analytic Lorentzian: \
             got {:.2} dBc/Hz, want {expected_dbc:.2} dBc/Hz",
            result.phase_noise_dbc[i]
        );
    }

    // Slope: each decade of offset must drop 20 dB (within 0.5 dB).
    let slope1 = result.phase_noise_dbc[0] - result.phase_noise_dbc[1];
    let slope2 = result.phase_noise_dbc[1] - result.phase_noise_dbc[2];
    assert!(
        (slope1 - 20.0).abs() < 0.5 && (slope2 - 20.0).abs() < 0.5,
        "white-region slope must be -20 dB/decade: got {slope1:.2}, {slope2:.2}"
    );
}

#[test]
fn oscillator_phase_noise_honors_quiet_resistors() {
    let deck = "\
* quiet tank resistor must not generate noise
l1 osc 0 1u
c1 osc 0 1u
r1 osc 0 1k noisy=0
b1 osc 0 i=-0.051*v(osc)+0.025*v(osc)*v(osc)*v(osc)
i1 0 osc pulse(0 1 10u 10n 10n 1u 1)
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let config = PssConfig::autonomous()
        .with_period_guess(6.3e-6)
        .with_tstab_periods(30)
        .with_tolerance(1e-6)
        .with_max_iterations(60);

    let error = Engine::default()
        .run_pnoise_oscillator(&netlist, config, &[1.0e3])
        .expect_err("a quiet resistor must be excluded from oscillator noise");
    assert!(
        error.to_string().contains("no modeled noise sources"),
        "unexpected error: {error}"
    );
}

#[test]
fn oscillator_phase_noise_rejects_invalid_offsets_before_solving() {
    let netlist = Netlist::parse("V1 out 0 1\nR1 out 0 1k\n.end").expect("deck parses");
    for offsets in [&[0.0][..], &[-1.0][..], &[f64::NAN][..]] {
        let error = Engine::default()
            .run_pnoise_oscillator(&netlist, PssConfig::autonomous(), offsets)
            .expect_err("invalid offsets must be rejected");
        assert!(
            error.to_string().contains("strictly positive"),
            "unexpected error: {error}"
        );
    }
}

//! Oscillator phase noise (Demir PPV) against the closed-form RLC result.
//!
//! For a high-Q parallel-RLC negative-resistance oscillator whose only
//! modeled noise source is the tank resistor, the adjoint unity Floquet
//! mode of the (weakly perturbed) symmetric LC orbit is exactly
//! v1(t) = (cos(w0 t), Z0 sin(w0 t)) / (A w0), which satisfies Demir's
//! normalization v1^T(t) ds/dt = 1 identically. Projecting the tank-node
//! current source b = (1/C, 0) gives
//!
//!   c = (1/T) integral (S_i/2) cos^2(w0 t) / (A w0 C)^2 dt
//!     = S_i / (4 A^2 w0^2 C^2),       S_i = 4 k T / R,
//!
//! where S_i is one-sided and the diffusion uses two-sided S_i/2
//! (TCAS-I 2000, Eqs. 7, 24, 44). The carrier-normalized sideband (Eq. 41)
//! L(f_m) = f0^2 c / (pi^2 f0^4 c^2 + f_m^2) falls at exactly
//! -20 dB/decade in the white region. The gate pins c, the absolute level,
//! and the slope.

use rspice_core::analysis::PssConfig;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

const K_B: f64 = 1.380649e-23;
const T_REF: f64 = 400.15;

#[test]
fn rlc_oscillator_phase_noise_matches_the_demir_constant() {
    // L = C = 1u (Z0 = 1 ohm, f0 = 159.155 kHz), tank R = 1k. The cubic
    // b-source restores the net -0.05 S small-signal startup conductance,
    // so the orbit keeps the van der Pol amplitude A = sqrt(4*0.05/0.075).
    let deck = "\
* noisy negative-resistance lc oscillator
l1 osc 0 1u
c1 osc 0 1u
b1 osc 0 i=-0.051*v(osc)+0.025*v(osc)*v(osc)*v(osc)
i1 0 osc pulse(0 1 10u 10n 10n 1u 1)
.options rshunt=1k temp=127
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
    let mut result = engine
        .run_pnoise_oscillator(&netlist, config, &offsets)
        .expect("oscillator pnoise completes");

    assert!(!result.phase_noise_contributors.is_empty());
    for index in 0..offsets.len() {
        let sum: f64 = result
            .phase_noise_contributors
            .iter()
            .map(|(_, values)| values[index])
            .sum();
        let expected = result.phase_error_psd[index];
        assert!((sum / expected - 1.0).abs() < 1e-12);
        let phase_expected =
            2.0 * result.diffusion_constant / (result.period * offsets[index]).powi(2);
        assert!((expected / phase_expected - 1.0).abs() < 1e-12);
    }
    assert_eq!(result.integrated_phase_noise, None);
    result.integrate_band().unwrap();
    assert!(result.integrated_phase_noise.unwrap() > 0.0);

    // Analytic diffusion constant for the tank-resistor source.
    let (l, c_tank, r): (f64, f64, f64) = (1.0e-6, 1.0e-6, 1.0e3);
    let w0 = 1.0 / (l * c_tank).sqrt();
    let f0 = w0 / (2.0 * std::f64::consts::PI);
    let a2 = 4.0 * (0.051 - 1.0 / r) / (3.0 * 0.025); // describing function
    let s_i = 4.0 * K_B * T_REF / r;
    let c_expected = s_i / (4.0 * a2 * w0 * w0 * c_tank * c_tank);

    assert!(
        (result.diffusion_constant - c_expected).abs() < 0.05 * c_expected,
        "diffusion constant must match the closed-form adjoint projection: \
         got {:.4e}, want {:.4e}",
        result.diffusion_constant,
        c_expected
    );

    // Absolute level in the white (-20 dB/dec) region: L = f0^2 c / f_m^2.
    for (i, &fm) in offsets.iter().enumerate() {
        // Current noise times the cycle-averaged squared phase sensitivity
        // (1 / (2 A^2 C^2)), followed by the phase integrator 1/(2 pi f)^2.
        let phase_expected =
            s_i / (2.0 * a2 * c_tank.powi(2) * (std::f64::consts::TAU * fm).powi(2));
        assert!((result.phase_error_psd[i] / phase_expected - 1.0).abs() < 0.05);
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

#[test]
fn autonomous_sampled_pnoise_preserves_phase_diffusion() {
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::engine::{
        PeriodicNoiseRequest, PeriodicNoiseSampling, PeriodicNoiseSidebands,
    };
    let netlist = Netlist::parse("Sampled LC oscillator\nl1 osc 0 1u\nc1 osc 0 1u\ngneg osc 0 osc 0 -0.051\nd1 osc 0 limiter\nd2 0 osc limiter\n.model limiter d(is=1p n=1)\ni1 0 osc pulse(0 1 10u 10n 10n 1u 1)\n.options rshunt=1k temp=127\n.pnoise lin 3 1e-16 1e-15 out=osc sampling=delay refout=osc periods=1 maxsideband=64 integratednoise=yes\n.end\n").unwrap();
    let engine = Engine::default().resolved_for_netlist(&netlist);
    let pss = engine
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::autonomous()
                .with_period_guess(6.3e-6)
                .with_tstab_periods(30)
                .with_harmonics(128)
                .with_tolerance(1e-6)
                .with_max_iterations(60),
            &NoAbort,
        )
        .unwrap();
    let offsets = [1e-16, 1e-15, 1.0, 10.0];
    let sampling = PeriodicNoiseSampling::Edge {
        edge: Default::default(),
    };
    let result = engine
        .run_pnoise_from_pss_request_with_abort(
            &netlist,
            &PeriodicNoiseRequest {
                offsets: &offsets,
                output_node: "osc",
                output_ref: None,
                input_source: None,
                max_sideband: 64,
                sidebands: PeriodicNoiseSidebands::default(),
                sampling: Some(&sampling),
            },
            &pss,
            &NoAbort,
        )
        .unwrap();
    let phase = engine
        .run_pnoise_oscillator_from_pss_with_abort(
            &netlist,
            pss.config().clone(),
            &offsets,
            &pss,
            &NoAbort,
        )
        .unwrap();
    let diffusion = phase.diffusion_constant;
    for (&offset, &density) in offsets.iter().zip(&result.output_noise) {
        let expected = 2.0 * diffusion / (std::f64::consts::TAU * offset).powi(2);
        assert!(
            (density / expected - 1.0).abs() < 0.05,
            "{offset} Hz: {density:e} vs {expected:e}"
        );
    }
    assert!((result.output_noise[0] / result.output_noise[1] - 100.0).abs() < 0.1);
    let rspice_core::netlist::AnalysisCommand::Pnoise(card) = &netlist.analyses[0] else {
        panic!()
    };
    let delay = engine
        .run_pnoise_card_from_pss_with_abort(&netlist, card, &pss, &NoAbort)
        .unwrap();
    let rspice_core::engine::PeriodicNoiseResult::Driven {
        result: delay_result,
        ..
    } = delay
    else {
        panic!("sampled autonomous noise must retain timing density")
    };
    assert!(delay_result.integrated_output_noise.unwrap() > 0.0);
    let expected_delay = 2.0 * diffusion * pss.analysis().result.period.powi(2);
    for density in delay_result.output_noise {
        assert!(
            (density / expected_delay - 1.0).abs() < 0.05,
            "close-in period jitter: {density:e} vs {expected_delay:e}"
        );
    }
}

#[test]
fn autonomous_sampled_pnoise_retains_timing_through_memoryless_line_ports() {
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::engine::{
        PeriodicNoiseRequest, PeriodicNoiseSampling, PeriodicNoiseSidebands,
    };
    // A zero-length line has physical port-current unknowns but no delay
    // history. It is supported by shooting PSS. An ideal buffer and quiet
    // matched termination leave the tank's edge timing unchanged.
    let netlist = Netlist::parse(
        "Buffered oscillator timing\n\
        l1 osc 0 1u\nc1 osc 0 1u\nr1 osc 0 1k\n\
        gneg osc 0 osc 0 -0.051\nd1 osc 0 limiter\nd2 0 osc limiter\n\
        .model limiter d(is=1p n=1)\ni1 0 osc pulse(0 1 10u 10n 10n 1u 1)\n\
        ebuf drive 0 osc 0 1\nrs drive near 50 noisy=0\n\
        o1 near 0 far 0 line\n.model line ltra r=0.05 c=20p len=0\n\
        rl far 0 50 noisy=0\n\
        .options temp=127\n.end\n",
    )
    .unwrap();
    let engine = Engine::default().resolved_for_netlist(&netlist);
    let pss = engine
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::autonomous()
                .with_period_guess(6.3e-6)
                .with_tstab_periods(30)
                .with_harmonics(128)
                .with_tolerance(1e-6)
                .with_max_iterations(60),
            &NoAbort,
        )
        .unwrap();
    let offsets = [1e-16, 1.0, 100.0];
    let sampling = PeriodicNoiseSampling::Edge {
        edge: Default::default(),
    };
    let run = |node| {
        engine
            .run_pnoise_from_pss_request_with_abort(
                &netlist,
                &PeriodicNoiseRequest {
                    offsets: &offsets,
                    output_node: node,
                    output_ref: None,
                    input_source: None,
                    max_sideband: 24,
                    sidebands: PeriodicNoiseSidebands::default(),
                    sampling: Some(&sampling),
                },
                &pss,
                &NoAbort,
            )
            .unwrap()
    };
    let tank = run("osc");
    let delayed = run("far");
    for (expected, actual) in tank.output_noise.iter().zip(delayed.output_noise) {
        assert!(actual.is_finite() && actual > 0.0);
        assert!(
            (actual / expected - 1.0).abs() < 0.005,
            "matched line changed edge timing density: {actual:e} vs {expected:e}"
        );
    }
}

#[test]
fn autonomous_sampled_pnoise_behavioral_limiter_matches_independent_ppv() {
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::engine::{
        PeriodicNoiseRequest, PeriodicNoiseSampling, PeriodicNoiseSidebands,
    };
    let netlist = Netlist::parse(
        "Behavioral LC oscillator\n\
        l1 osc 0 1u\nc1 osc 0 1u\nr1 osc 0 1k\n\
        b1 osc 0 i=-0.051*v(osc)+0.025*v(osc)*v(osc)*v(osc)\n\
        bkick 0 osc i=spice_pulse(0,1,10u,10n,10n,1u,1)\n.end\n",
    )
    .unwrap();
    let engine = Engine::default();
    let pss = engine
        .run_pss_operating_point_with_abort(
            &netlist,
            PssConfig::autonomous()
                .with_period_guess(6.3e-6)
                .with_tstab_periods(30)
                .with_harmonics(64)
                .with_tolerance(1e-6)
                .with_max_iterations(60),
            &NoAbort,
        )
        .unwrap();
    let offsets = [1e-16, 1e-15, 10.0];
    let sampling = PeriodicNoiseSampling::Edge {
        edge: Default::default(),
    };
    let sampled = engine
        .run_pnoise_from_pss_request_with_abort(
            &netlist,
            &PeriodicNoiseRequest {
                offsets: &offsets,
                output_node: "osc",
                output_ref: None,
                input_source: None,
                max_sideband: 24,
                sidebands: PeriodicNoiseSidebands::default(),
                sampling: Some(&sampling),
            },
            &pss,
            &NoAbort,
        )
        .unwrap();
    let phase = engine
        .run_pnoise_oscillator_from_pss_with_abort(
            &netlist,
            pss.config().clone(),
            &offsets,
            &pss,
            &NoAbort,
        )
        .unwrap();
    for (&offset, density) in offsets.iter().zip(sampled.output_noise) {
        let expected = 2.0 * phase.diffusion_constant / (std::f64::consts::TAU * offset).powi(2);
        assert!(
            (density / expected - 1.0).abs() < 0.05,
            "{offset}: {density:e} vs {expected:e}"
        );
    }
}

//! Periodic-AC validation against closed-form answers.
//!
//! Two analytic gates pin the conversion-matrix solve:
//! 1. A linear RC low-pass under PAC must reproduce the ordinary AC transfer
//!    at every SIGNED sideband frequency offset + k*f0, with exactly zero
//!    sideband conversion.
//! 2. A switch chopper driven by a sinusoidal LO is the classic textbook
//!    mixer: a 50% duty square-wave conductance whose +/-1-harmonic
//!    conversion gain is A/pi and whose direct feedthrough is A/2.

use num_complex::Complex64;
use rspice_core::analysis::advanced::pac::PacConfig;
use rspice_core::engine::{Engine, PacAnalysisResult, SimulationConfig};
use rspice_core::netlist::Netlist;

const F0: f64 = 1.0e6;

fn run_pac(deck: &str, config: PacConfig) -> PacAnalysisResult {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    engine.run_pac(&netlist, config).expect("PAC completes")
}

#[test]
fn linear_rc_pac_matches_the_ac_transfer_at_signed_sideband_frequencies() {
    // RC corner exactly at 1 MHz: H(f) = 1 / (1 + j f/fc) with fc = 1 MHz.
    let deck = "\
* linear rc low-pass
vin in 0 dc 0 ac 1
r1 in out 1k
c1 out 0 159.154943091895p
.end
";
    let config = PacConfig::new()
        .with_fundamental(F0)
        .with_sweep(1.0e5, 5.0e5, 3)
        .with_sweep_type(rspice_core::analysis::advanced::pac::PacSweepType::Linear)
        .with_sidebands(-2, 2)
        .with_input_source("vin")
        .with_output_node("out");

    let analysis = run_pac(deck, config);
    let result = &analysis.result;
    let out_idx = result.node_index("out").expect("out node present");

    let fc = 1.0e6;
    for (freq_idx, &offset) in result.frequencies.clone().iter().enumerate() {
        for m in -2..=2 {
            // The input excited at sideband m responds only at sideband m,
            // with the ordinary AC transfer at the signed frequency.
            let f_abs = offset + (m as f64) * F0;
            let expected = Complex64::new(1.0, 0.0) / Complex64::new(1.0, f_abs / fc);

            let got = result.conversion_matrix.get(freq_idx, m, m);
            assert!(
                (got - expected).norm() < 1e-3 * expected.norm(),
                "diagonal transfer at offset {offset:.3e}, sideband {m}: got {got}, want {expected}"
            );

            for k in -2..=2 {
                if k == m {
                    continue;
                }
                let leak = result.conversion_matrix.get(freq_idx, k, m).norm();
                assert!(
                    leak < 1e-9,
                    "linear circuit must not convert sidebands: |H[{k},{m}]| = {leak:.3e}"
                );
            }
        }

        // The m = 0 column doubles as the per-node sideband spectra.
        let direct = result.voltage(out_idx, freq_idx, 0);
        let expected = Complex64::new(1.0, 0.0) / Complex64::new(1.0, offset / fc);
        assert!(
            (direct - expected).norm() < 1e-3 * expected.norm(),
            "sideband data at offset {offset:.3e}: got {direct}, want {expected}"
        );
    }
}

#[test]
fn switch_chopper_conversion_gain_converges_to_the_square_wave_coefficients() {
    // LO-driven series switch: the small-signal transfer rfin -> out toggles
    // between Rload/(Rload + ron) ~ 0.999 and ~1e-6 at 50% duty. The exact
    // LTV answer has |H[+/-1, 0]| = A/pi and H[0,0] = A/2; the truncated
    // conversion matrix approaches it as the sideband count grows (hard
    // switching converges ~1/K, exactly as in commercial conversion-matrix
    // solvers), so the test pins both the K=16 values and the convergence.
    let deck = "\
* series chopper mixer
vlo ctl 0 sin(0 1 1meg)
vrf rfin 0 dc 0
s1 rfin out ctl 0 swmod
rload out 0 1k
cload out 0 1f
.model swmod sw vt=0 ron=1 roff=1e9 smooth=1m
.end
";
    let chopper_h = |k: i32| -> (f64, f64, f64) {
        let config = PacConfig::new()
            .with_fundamental(F0)
            .with_sweep(1.0e4, 1.0e4, 1)
            .with_sweep_type(rspice_core::analysis::advanced::pac::PacSweepType::Linear)
            .with_sidebands(-k, k)
            .with_input_source("vrf")
            .with_output_node("out");
        let analysis = run_pac(deck, config);
        assert!(analysis.converged, "operating point must converge");
        let cm = &analysis.result.conversion_matrix;
        (
            cm.get(0, 1, 0).norm(),
            cm.get(0, -1, 0).norm(),
            cm.get(0, 0, 0).norm(),
        )
    };

    let a = 1000.0 / 1001.0; // on-state divider ratio
    let fundamental_exact = a / std::f64::consts::PI;
    let direct_exact = a / 2.0;

    let (h1_coarse, _, h0_coarse) = chopper_h(3);
    let (h1, h1_down, h0) = chopper_h(16);

    assert!(
        (h1 - fundamental_exact).abs() < 0.01 * fundamental_exact,
        "K=16 upper-sideband conversion must be within 1% of A/pi: got {h1:.5}, want {fundamental_exact:.5}"
    );
    assert!(
        (h1_down - fundamental_exact).abs() < 0.01 * fundamental_exact,
        "K=16 lower-sideband conversion must be within 1% of A/pi: got {h1_down:.5}"
    );
    assert!(
        (h0 - direct_exact).abs() < 0.08 * direct_exact,
        "K=16 direct feedthrough must approach A/2: got {h0:.5}, want {direct_exact:.5}"
    );

    // Truncation error must shrink as sidebands are added.
    assert!(
        (h1 - fundamental_exact).abs() < (h1_coarse - fundamental_exact).abs() / 3.0,
        "fundamental conversion must converge with sideband count: K=3 {h1_coarse:.5}, K=16 {h1:.5}"
    );
    assert!(
        (h0 - direct_exact).abs() < (h0_coarse - direct_exact).abs() / 2.0,
        "direct feedthrough must converge with sideband count: K=3 {h0_coarse:.5}, K=16 {h0:.5}"
    );
}

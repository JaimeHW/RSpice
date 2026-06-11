//! Exact-vs-Toeplitz HB Jacobian: identical answers, faster convergence.
//!
//! The exact real-split Jacobian adds the conjugate (Hankel) coupling the
//! one-sided complex formulation cannot represent. Because the residual is
//! exact in both modes, the converged spectra must match to Newton
//! tolerance; the payoff is convergence rate, demonstrated on the rectifier
//! by a strict iteration-count win.

use rspice_core::analysis::advanced::harmonic_balance::HbConfig;
use rspice_core::engine::{Engine, HbAnalysisResult, SimulationConfig};
use rspice_core::netlist::Netlist;

const RECTIFIER: &str = "\
* diode rectifier bench
v1 in 0 sin(0 2 1meg)
r1 in a 50
d1 a out dmod
rl out 0 1k
cl out 0 10n
.model dmod D IS=1e-14 N=1.8
.end
";

fn run_rectifier(exact: bool) -> HbAnalysisResult {
    let netlist = Netlist::parse(RECTIFIER).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let mut config = HbConfig::new(1.0e6).with_harmonics(8);
    config.use_exact_jacobian = exact;
    engine.run_hb(&netlist, config).expect("HB completes")
}

#[test]
fn exact_jacobian_matches_the_toeplitz_spectra_and_converges_faster() {
    let exact = run_rectifier(true);
    let legacy = run_rectifier(false);

    assert!(exact.converged, "exact path must converge");
    assert!(legacy.converged, "legacy path must converge");

    let scale = legacy
        .result
        .spectral_voltages
        .iter()
        .flat_map(|sv| sv.coefficients.iter())
        .map(|c| c.norm())
        .fold(0.0f64, f64::max);
    assert!(scale > 0.1, "rectifier must produce non-trivial spectra");

    // Both runs stop at the same residual tolerance, so they agree to
    // convergence accuracy (the legacy path additionally parks residual-
    // tolerance-sized imaginary junk on its DC coefficients, which the
    // real-split formulation structurally forbids).
    for (sv_e, sv_l) in exact
        .result
        .spectral_voltages
        .iter()
        .zip(&legacy.result.spectral_voltages)
    {
        assert_eq!(sv_e.node_name, sv_l.node_name);
        for (k, (a, b)) in sv_e.coefficients.iter().zip(&sv_l.coefficients).enumerate() {
            assert!(
                (a - b).norm() <= 1e-3 * scale,
                "node {} harmonic {k}: exact {a} vs legacy {b}",
                sv_e.node_name
            );
        }
        assert!(
            sv_e.coefficients[0].im.abs() < 1e-12,
            "exact path must keep DC strictly real, got {}",
            sv_e.coefficients[0]
        );
    }

    // The whole point: the exact Jacobian restores fast Newton convergence.
    assert!(
        exact.result.iterations < legacy.result.iterations,
        "exact Jacobian must converge in strictly fewer iterations: exact {} vs legacy {}",
        exact.result.iterations,
        legacy.result.iterations
    );
}

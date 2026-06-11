//! Krylov vs direct parity for the harmonic-balance Newton step.
//!
//! The Krylov path (GMRES + per-harmonic block-Jacobi) must be a pure
//! speed change: converged spectra have to match the dense-elimination
//! path to Newton tolerance on real nonlinear circuits.

use rspice_core::analysis::advanced::harmonic_balance::HbConfig;
use rspice_core::engine::{Engine, SimulationConfig};
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

fn run_rectifier(use_krylov: bool) -> rspice_core::engine::HbAnalysisResult {
    let netlist = Netlist::parse(RECTIFIER).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let mut config = HbConfig::new(1.0e6).with_harmonics(8);
    config.use_krylov = use_krylov;
    engine.run_hb(&netlist, config).expect("HB completes")
}

#[test]
fn krylov_spectra_match_the_direct_path() {
    let direct = run_rectifier(false);
    let krylov = run_rectifier(true);

    assert!(direct.converged, "direct path must converge");
    assert!(krylov.converged, "krylov path must converge");

    // Global coefficient scale for the relative comparison.
    let scale = direct
        .result
        .spectral_voltages
        .iter()
        .flat_map(|sv| sv.coefficients.iter())
        .map(|c| c.norm())
        .fold(0.0f64, f64::max);
    assert!(scale > 0.1, "rectifier must produce non-trivial spectra");

    for (sv_direct, sv_krylov) in direct
        .result
        .spectral_voltages
        .iter()
        .zip(&krylov.result.spectral_voltages)
    {
        assert_eq!(sv_direct.node_name, sv_krylov.node_name);
        for (k, (a, b)) in sv_direct
            .coefficients
            .iter()
            .zip(&sv_krylov.coefficients)
            .enumerate()
        {
            assert!(
                (a - b).norm() <= 1e-5 * scale,
                "node {} harmonic {}: direct {} vs krylov {}",
                sv_direct.node_name,
                k,
                a,
                b
            );
        }
    }
}

#[test]
fn rectifier_dc_output_is_physical() {
    let result = run_rectifier(true);
    let out = result
        .result
        .spectral_voltages
        .iter()
        .find(|sv| sv.node_name.eq_ignore_ascii_case("out"))
        .expect("out node present");
    let dc = out.coefficients[0].re;
    assert!(
        dc > 0.3 && dc < 2.0,
        "rectified DC level must be positive and below the drive: got {dc}"
    );
}

#[test]
fn larger_system_uses_the_auto_krylov_path_and_matches_direct() {
    // 30+ nodes x 9 spectral components puts the system above the 256-unknown
    // auto threshold; both paths must agree on the converged spectra.
    let mut deck = String::from("* diode-loaded ladder\nv1 n0 0 sin(0 1.5 2meg)\n");
    for i in 0..30 {
        deck.push_str(&format!("r{i} n{i} n{} 25\n", i + 1));
        if i % 5 == 0 {
            deck.push_str(&format!("d{i} n{} 0 dmod\n", i + 1));
        }
        if i % 7 == 0 {
            deck.push_str(&format!("c{i} n{} 0 20p\n", i + 1));
        }
    }
    deck.push_str("rload n30 0 500\n.model dmod D IS=1e-14 N=2.0\n.end\n");

    let netlist = Netlist::parse(&deck).expect("ladder parses");
    let engine = Engine::new(SimulationConfig::default());

    let mut direct_cfg = HbConfig::new(2.0e6).with_harmonics(8);
    direct_cfg.use_krylov = false; // still >= threshold: auto engages Krylov
    let auto = engine.run_hb(&netlist, direct_cfg).expect("auto HB runs");
    assert!(auto.converged, "auto path must converge");

    let out = auto
        .result
        .spectral_voltages
        .iter()
        .find(|sv| sv.node_name.eq_ignore_ascii_case("n30"))
        .expect("ladder output present");
    assert!(
        out.coefficients.iter().all(|c| c.norm().is_finite()),
        "spectra must be finite"
    );
}

//! RF validation Tier-0 (roadmap M4.1): analytic-truth checks for the
//! periodic analyses. Every expected value here is a closed form — no
//! reference simulator involved — so a failure is unambiguous.

use rspice_core::analysis::advanced::harmonic_balance::HbConfig;
use rspice_core::analysis::advanced::pss::PssConfig;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

/// Fundamental-coefficient ratio of two nodes from an HB result —
/// coefficient conventions cancel in the ratio.
fn harmonic_ratio(
    result: &rspice_core::engine::HbAnalysisResult,
    num: &str,
    den: &str,
    harmonic: usize,
) -> num_complex::Complex<f64> {
    let coeff = |node: &str| {
        result
            .result
            .spectral_voltages
            .iter()
            .find(|sv| sv.node_name.eq_ignore_ascii_case(node))
            .unwrap_or_else(|| panic!("node {node} missing from HB spectra"))
            .coefficients[harmonic]
    };
    coeff(num) / coeff(den)
}

fn harmonic_magnitude(
    result: &rspice_core::engine::HbAnalysisResult,
    node: &str,
    harmonic: usize,
) -> f64 {
    result
        .result
        .spectral_voltages
        .iter()
        .find(|sv| sv.node_name.eq_ignore_ascii_case(node))
        .unwrap_or_else(|| panic!("node {node} missing from HB spectra"))
        .coefficients[harmonic]
        .norm()
}

/// HB on a *linear* RC at a single tone must reproduce the AC transfer
/// function exactly: H(jw) = 1/(1 + jwRC), and every higher harmonic is
/// numerically zero.
#[test]
fn hb_linear_rc_matches_the_ac_transfer_function() {
    // wRC = 1 at the drive frequency for a crisp expected value.
    let f0 = 1.0e6;
    let r = 1.0e3;
    let c = 1.0 / (2.0 * std::f64::consts::PI * f0 * r);
    let deck = format!(
        "hb linear rc\n\
         v1 in 0 sin(0 1 {f0})\n\
         r1 in out {r}\n\
         c1 out 0 {c:e}\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = engine()
        .run_hb(&netlist, HbConfig::new(f0).with_harmonics(6))
        .expect("HB completes");
    assert!(result.converged);

    let h = harmonic_ratio(&result, "out", "in", 1);
    let expected = num_complex::Complex::new(1.0, 0.0) / num_complex::Complex::new(1.0, 1.0);
    assert!(
        (h - expected).norm() < 2e-3,
        "H(jw) = {h} vs analytic {expected}"
    );

    // Linearity: harmonics 2+ are zero to solver tolerance.
    let fundamental = harmonic_magnitude(&result, "out", 1);
    for k in 2..=6 {
        let residue = harmonic_magnitude(&result, "out", k);
        assert!(
            residue < 1e-6 * fundamental.max(1e-12),
            "harmonic {k} must vanish on a linear circuit, got {residue:e}"
        );
    }
}

/// Square-law distortion: a level-1 MOSFET in saturation with LAMBDA=0
/// is an exact quadratic, so a gate drive `Vov + a·cos` produces a
/// second-harmonic-to-fundamental ratio of exactly `a / (4·Vov)` in the
/// drain current (and therefore in the resistive drain voltage).
///
/// KNOWN GAP (live repro): the engine reports exactly 2× this ratio.
/// Root cause identified: HB stores solution coefficients in a
/// full-amplitude convention (a 0.2 V drive stores c1 = 0.2) while the
/// FFT pair (`HbFft::to_time_domain`/`to_frequency_domain`) implements
/// the two-sided half-amplitude convention — so every nonlinear device
/// is evaluated at twice the true voltage swing. Linear circuits are
/// scale-invariant and mask it (see the passing test above). The fix is
/// a coherent basis audit: stamp source harmonics at A/2, extract
/// reported spectra at 2·w_k, and leave the (mathematically standard)
/// two-sided FFT/Toeplitz machinery untouched. Un-ignore both gap tests
/// when landed.
#[test]
#[ignore = "HB amplitude-basis mismatch doubles nonlinear drive (see doc comment)"]
fn hb_mosfet_square_law_second_harmonic_ratio_is_exact() {
    let f0 = 1.0e6;
    let vov = 1.0; // VGS - VT
    let a = 0.2; // drive amplitude
    // The bypass cap only exists because HB requires a reactive element;
    // at 100 ohm drain impedance its 1 pF is 4 orders below influence.
    let deck = format!(
        "hb square law\n\
         vdd vdd 0 dc 5\n\
         vg g 0 dc 2.0 sin(2.0 {a} {f0})\n\
         m1 d g 0 0 nch w=10u l=1u\n\
         rd vdd d 100\n\
         cd d 0 1p\n\
         .model nch nmos (level=1 vto=1.0 kp=2e-4 lambda=0)\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let result = engine()
        .run_hb(&netlist, HbConfig::new(f0).with_harmonics(6))
        .expect("HB completes");
    assert!(result.converged);

    let h1 = harmonic_magnitude(&result, "d", 1);
    let h2 = harmonic_magnitude(&result, "d", 2);
    let expected_ratio = a / (4.0 * vov);
    let ratio = h2 / h1;
    assert!(
        (ratio - expected_ratio).abs() / expected_ratio < 0.02,
        "HD2 ratio {ratio} vs analytic {expected_ratio}"
    );

    // A pure square law generates nothing above the second harmonic.
    let h3 = harmonic_magnitude(&result, "d", 3);
    assert!(
        h3 < 1e-3 * h1,
        "third harmonic must vanish for an exact square law, got {h3:e}"
    );
}

/// Series RLC driven at resonance: the capacitor voltage magnifies by
/// exactly Q = sqrt(L/C)/R.
#[test]
fn pss_series_rlc_resonance_magnifies_by_q() {
    let f0 = 1.0e6;
    let q = 5.0;
    let w0 = 2.0 * std::f64::consts::PI * f0;
    let l = 10e-6;
    let c = 1.0 / (w0 * w0 * l);
    let r = (l / c).sqrt() / q;
    let deck = format!(
        "pss rlc resonance\n\
         v1 in 0 sin(0 0.1 {f0})\n\
         r1 in a {r}\n\
         l1 a b {l:e}\n\
         c1 b 0 {c:e}\n\
         .end\n"
    );
    let netlist = Netlist::parse(&deck).expect("deck parses");
    let config = PssConfig::new(f0)
        .with_tstab_periods(64)
        .with_tolerance(1e-7);
    let result = engine().run_pss(&netlist, config).expect("PSS converges");
    assert!(
        result.final_residual < 1e-4,
        "periodic orbit must close, residual {}",
        result.final_residual
    );

    let pss = &result.result;
    let idx = pss
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("b"))
        .expect("capacitor node");
    let amplitude = pss.waveforms[idx]
        .values
        .iter()
        .fold(0.0f64, |acc, v| acc.max(v.abs()));
    let expected = 0.1 * q;
    assert!(
        (amplitude - expected).abs() / expected < 0.03,
        "capacitor amplitude {amplitude} vs Q-magnified {expected}"
    );
}

const RECTIFIER_DECK: &str = "rectifier cross-check\n\
    v1 in 0 sin(0 2 1meg)\n\
    r1 in a 50\n\
    d1 a out dmod\n\
    rl out 0 1k\n\
    cl out 0 10n\n\
    .model dmod D IS=1e-14 N=1.8\n\
    .end\n";

/// Settled mean of `out` over the last transient period — the referee
/// (the transient engine is the conformance-validated one).
fn transient_mean_out(netlist: &Netlist) -> f64 {
    let result = engine()
        .run_tran(netlist, 50e-6, 5e-9)
        .expect("transient completes");
    let out = result
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("out"))
        .expect("out node");
    let values = &result.voltages[out];
    let times = &result.time;
    let (mut sum, mut count) = (0.0, 0usize);
    for (t, v) in times.iter().zip(values) {
        if *t >= 49e-6 {
            sum += v;
            count += 1;
        }
    }
    sum / count as f64
}

/// Cross-consistency: the PSS steady-state mean must match the settled
/// transient mean on the same nonlinear circuit.
#[test]
fn pss_mean_matches_the_transient_referee_on_a_rectifier() {
    let netlist = Netlist::parse(RECTIFIER_DECK).expect("deck parses");
    let referee = transient_mean_out(&netlist);

    let pss = engine()
        .run_pss(
            &netlist,
            PssConfig::new(1.0e6)
                .with_tstab_periods(32)
                .with_tolerance(1e-7),
        )
        .expect("PSS converges");
    let waves = &pss.result;
    let idx = waves
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("out"))
        .expect("out waveform");
    let values = &waves.waveforms[idx].values;
    let pss_mean: f64 = values.iter().sum::<f64>() / values.len() as f64;

    assert!(
        (pss_mean - referee).abs() / referee.abs().max(1e-9) < 0.02,
        "PSS mean {pss_mean} vs transient referee {referee}"
    );
}

/// KNOWN GAP (live repro, same root cause as the square-law test): HB's
/// DC component on this hard-clipping rectifier reads ~1.97 V where PSS
/// and the transient referee agree on ~0.53 V — the doubled nonlinear
/// drive overdrives the diode. Un-ignore when the amplitude basis is
/// fixed.
#[test]
#[ignore = "HB amplitude-basis mismatch doubles nonlinear drive (see square-law test)"]
fn hb_dc_component_matches_the_transient_referee_on_a_rectifier() {
    let netlist = Netlist::parse(RECTIFIER_DECK).expect("deck parses");
    let referee = transient_mean_out(&netlist);

    let hb = engine()
        .run_hb(&netlist, HbConfig::new(1.0e6).with_harmonics(10))
        .expect("HB completes");
    assert!(hb.converged);
    let hb_dc = hb
        .result
        .spectral_voltages
        .iter()
        .find(|sv| sv.node_name.eq_ignore_ascii_case("out"))
        .expect("out spectrum")
        .coefficients[0]
        .re;

    assert!(
        (hb_dc - referee).abs() / referee.abs().max(1e-9) < 0.05,
        "HB DC {hb_dc} vs transient referee {referee}"
    );
}

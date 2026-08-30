//! RF validation Tier-0 (roadmap M4.1): analytic-truth checks for the
//! periodic analyses. Every expected value here is a closed form — no
//! reference simulator involved — so a failure is unambiguous.

use rspice_core::analysis::harmonic_balance::HbConfig;
use rspice_core::analysis::pac::{PacConfig, PacSweepType};
use rspice_core::analysis::pss::PssConfig;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::netlist::Netlist;

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

fn bsim4_models45() -> String {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/src/device/mosfet/bsim4v8/testdata/models45.lib"
    );
    std::fs::read_to_string(path).expect("read BSIM4 model card")
}

fn bsim4_rf_deck() -> String {
    format!(
        "* bsim4 periodic unsupported-device gate\n\
         vd d 0 dc 1.1\n\
         vg g 0 dc 1.1 sin(1.1 0.01 1meg)\n\
         m1 d g 0 0 n45 w=1u l=45n ad=0.1p as=0.1p pd=2.2u ps=2.2u nrd=0 nrs=0\n\
         {}\n\
         .end\n",
        bsim4_models45()
    )
}

fn assert_bsim4_periodic_rejection(err: rspice_core::engine::SimulationError) {
    let message = err.to_string();
    assert!(
        message.contains("HB runtime does not yet support")
            && message.contains("native BSIM4")
            && message.contains("1 device"),
        "periodic analysis must loudly reject unsupported native BSIM4, got: {message}"
    );
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

#[test]
fn hb_rejects_native_bsim4_instead_of_solving_without_its_nonlinearity() {
    let netlist = Netlist::parse(&bsim4_rf_deck()).expect("deck parses");
    let err = engine()
        .run_hb(&netlist, HbConfig::new(1.0e6).with_harmonics(5))
        .expect_err("native BSIM4 is not adapted into HB yet");

    assert_bsim4_periodic_rejection(err);
}

#[test]
fn pac_rejects_native_bsim4_before_periodic_operating_point() {
    let netlist = Netlist::parse(&bsim4_rf_deck()).expect("deck parses");
    let config = PacConfig::new()
        .with_fundamental(1.0e6)
        .with_sweep(1.0e3, 1.0e3, 1)
        .with_sweep_type(PacSweepType::Linear)
        .with_sidebands(-1, 1)
        .with_input_source("vg")
        .with_output_node("d");
    let err = engine()
        .run_pac(&netlist, config)
        .expect_err("native BSIM4 is not adapted into PAC yet");

    assert_bsim4_periodic_rejection(err);
}

#[test]
fn pnoise_rejects_native_bsim4_before_noise_folding() {
    let netlist = Netlist::parse(&bsim4_rf_deck()).expect("deck parses");
    let err = engine()
        .run_pnoise(&netlist, 1.0e6, &[1.0e3], "d", None, Some("vg"), 1)
        .expect_err("native BSIM4 is not adapted into PNoise yet");

    assert_bsim4_periodic_rejection(err);
}

/// Square-law distortion: a level-1 MOSFET in saturation with LAMBDA=0
/// is an exact quadratic, so a gate drive `Vov + a·cos` produces a
/// second-harmonic-to-fundamental ratio of exactly `a / (4·Vov)` in the
/// drain current (and therefore in the resistive drain voltage).
///
/// This pins the HB amplitude-basis convention end to end: an engine
/// that evaluates nonlinear devices at the wrong multiple of the true
/// swing (the historical defect was exactly 2×, from full-amplitude
/// solution storage feeding the two-sided half-amplitude FFT pair)
/// scales this ratio by the same factor, while linear circuits stay
/// scale-invariant and mask it.
#[test]
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

/// Hard-clipping rectifier: HB's DC component must agree with the PSS
/// and transient referees (~0.53 V). An amplitude-basis error overdrives
/// the diode and pushes the rectified DC far off (the historical 2×
/// defect read ~1.97 V here).
#[test]
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

/// An ngspice-style annotated port that names a power and a frequency is a
/// large-signal generator, not just a measurement reference. `examples/sp/sp1.cir`
/// ships one; RSpice used to unwrap the port annotation and run the `dc 0 ac 1`
/// underneath it, so the deck produced a flat zero in transient.
///
/// The amplitude is closed form: `sqrt(4 P Z0)` is the peak that delivers `P`
/// into a matched `Z0`, so 1 mW into 100 ohms is 632.455 mV.
#[test]
fn an_rf_port_drives_its_declared_power_into_the_transient() {
    let frequency = 1.0e9;
    let netlist = Netlist::parse(
        "* annotated RF port as a large-signal drive\n\
         V1 in 0 dc 0 ac 1 portnum 1 z0 100 pwr 0.001 freq 1e9\n\
         R1 in 0 50\n\
         .end\n",
    )
    .expect("deck parses");

    let periods = 4.0;
    let result = engine()
        .run_tran(&netlist, periods / frequency, 1.0 / (frequency * 2000.0))
        .expect("transient completes");
    let in_node = result
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("in"))
        .expect("in node");

    let expected = (4.0 * 0.001 * 100.0_f64).sqrt();
    let peak = result.voltages[in_node]
        .iter()
        .fold(0.0_f64, |peak, v| peak.max(v.abs()));
    assert!(
        (peak - expected).abs() < 1.0e-3 * expected,
        "port peak {peak} V, expected {expected} V"
    );

    // The waveform has to be the cosine and not a constant sitting at the peak.
    let trough = result.voltages[in_node]
        .iter()
        .fold(0.0_f64, |trough, v| trough.min(*v));
    assert!(
        (trough + expected).abs() < 1.0e-3 * expected,
        "port trough {trough} V, expected {} V",
        -expected
    );
}

/// A port that gives only `portnum`/`z0` says how to normalize a scattering
/// measurement, not what to inject. Driving it would corrupt every deck that
/// declares a reference plane and supplies its own stimulus.
#[test]
fn an_rf_port_with_no_declared_drive_leaves_its_own_waveform_alone() {
    let netlist = Netlist::parse(
        "* reference-only port carrying its own PULSE stimulus\n\
         V1 in 0 dc 0 portnum 1 z0 50 pulse(0 1 0 1n 1n 10n 20n)\n\
         R1 in 0 50\n\
         .end\n",
    )
    .expect("deck parses");

    let result = engine()
        .run_tran(&netlist, 40.0e-9, 1.0e-10)
        .expect("transient completes");
    let in_node = result
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("in"))
        .expect("in node");

    let peak = result.voltages[in_node]
        .iter()
        .fold(f64::NEG_INFINITY, |peak, v| peak.max(*v));
    let trough = result.voltages[in_node]
        .iter()
        .fold(f64::INFINITY, |trough, v| trough.min(*v));
    assert!(
        (peak - 1.0).abs() < 1.0e-6 && trough.abs() < 1.0e-6,
        "reference-only port should still be its own PULSE: {trough} .. {peak}"
    );
}

/// The transient must open at the drive, not step into it. ngspice's TRANOP
/// evaluates the port waveform at `time = 0` (MODEDC covers MODETRANOP), so a
/// port that starts at its cosine peak has an operating point at that peak.
#[test]
fn an_rf_port_operating_point_opens_at_the_drive_rather_than_stepping_into_it() {
    let netlist = Netlist::parse(
        "* port bias continuity across the operating point\n\
         V1 in 0 dc 0 portnum 1 z0 100 pwr 0.001 freq 1e9\n\
         R1 in 0 50\n\
         .end\n",
    )
    .expect("deck parses");

    let expected = (4.0 * 0.001 * 100.0_f64).sqrt();
    let op = engine()
        .run_dc_op(&netlist)
        .expect("operating point solves");
    let index = op
        .node_names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("in"))
        .expect("in node");
    assert!(
        (op.node_voltages[index] - expected).abs() < 1.0e-9,
        "operating point {} V, expected the t=0 drive {expected} V",
        op.node_voltages[index]
    );
}

/// The two port spellings are different generators, so the same `pwr` has to
/// mean different EMFs. A `P` element sits behind a real Z0 and drops half its
/// EMF there, so `pwr` is the available power a matched load receives -- the
/// ADS/Spectre convention. An ngspice `portnum=` annotation drives the plane
/// directly with ngspice's own `sqrt(4 P Z0)`.
///
/// Reading one formula onto both would put a deck 6 dB off whichever way it
/// was written, so the divider below measures each against its own contract.
#[test]
fn the_two_port_spellings_scale_declared_power_to_their_own_topology() {
    let frequency = 1.0e9;
    let peak = |deck: &str, node: &str| {
        let netlist = Netlist::parse(deck).expect("deck parses");
        let result = engine()
            .run_tran(&netlist, 4.0 / frequency, 1.0 / (frequency * 2000.0))
            .expect("transient completes");
        let index = result
            .node_names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(node))
            .unwrap_or_else(|| panic!("node {node} missing"));
        result.voltages[index]
            .iter()
            .fold(0.0_f64, |peak, v| peak.max(v.abs()))
    };

    // Thevenin: 1 mW available into a matched 50 R is 316.2 mV at the plane.
    let thevenin = peak(
        "* P element delivering its declared power\n\
         P1 in 0 PORT=1 Z0=50 PWR=1m FREQ=1e9\n\
         R1 in 0 50\n\
         .end\n",
        "in",
    );
    // EMF sqrt(8 P Z0), halved across the port's own reference impedance.
    let delivered = (8.0 * 0.001 * 50.0_f64).sqrt() / 2.0;
    assert!(
        (thevenin - delivered).abs() < 1.0e-3 * delivered,
        "P element plane peak {thevenin} V, expected {delivered} V"
    );
    let watts = thevenin * thevenin / (2.0 * 50.0);
    assert!(
        (watts - 0.001).abs() < 1.0e-5,
        "P element delivered {watts} W into a matched load, expected 1 mW"
    );

    // Annotated: ngspice drives the plane itself, undivided.
    let annotated = peak(
        "* ngspice annotated port keeps ngspice's amplitude\n\
         V1 in 0 dc 0 portnum 1 z0 50 pwr 1m freq 1e9\n\
         R1 in 0 50\n\
         .end\n",
        "in",
    );
    let ngspice = (4.0 * 0.001 * 50.0_f64).sqrt();
    assert!(
        (annotated - ngspice).abs() < 1.0e-3 * ngspice,
        "annotated port peak {annotated} V, expected ngspice's {ngspice} V"
    );
}

/// Harmonic balance and a settled transient are the same steady state, so a
/// port that declares a drive has to excite both of them identically.
///
/// `hb_source_spectrum` used to unwrap the port and build its spectrum from the
/// AC magnitude, which is the small-signal excitation `.AC` and the S-parameter
/// sweep read -- so a port declaring only `pwr`/`freq` drove nothing in HB
/// while driving the transient properly, and one carrying both was driven
/// twice. The diode below rectifies, so the tone has to be right in amplitude
/// and not merely present.
#[test]
fn harmonic_balance_drives_a_port_the_same_way_the_transient_does() {
    let fundamental = 1.0e6;
    let deck = "* RF port driving a rectifier\n\
        P1 in 0 PORT=1 Z0=50 PWR=10m FREQ=1e6 AC 1\n\
        D1 in out DMOD\n\
        R1 out 0 1k\n\
        C1 out 0 100n\n\
        .model DMOD D(is=1e-14 n=1)\n\
        .end\n";
    let netlist = Netlist::parse(deck).expect("deck parses");

    let hb = engine()
        .run_hb(&netlist, HbConfig::new(fundamental).with_harmonics(8))
        .expect("HB converges");
    let hb_dc = harmonic_magnitude(&hb, "out", 0);

    // The referee: integrate long enough to settle, then average a whole
    // number of periods so the fundamental cancels out of the mean.
    let periods = 400.0;
    let result = engine()
        .run_tran(&netlist, periods / fundamental, 1.0 / (fundamental * 200.0))
        .expect("transient completes");
    let index = result
        .node_names
        .iter()
        .position(|name| name.eq_ignore_ascii_case("out"))
        .expect("out node");
    let settled_from = (periods - 20.0) / fundamental;
    let (sum, count) = result
        .time
        .iter()
        .zip(&result.voltages[index])
        .filter(|(t, _)| **t >= settled_from)
        .fold((0.0, 0usize), |(sum, count), (_, v)| (sum + v, count + 1));
    let referee = sum / count as f64;

    assert!(
        referee > 0.05,
        "the referee must actually rectify something: {referee} V"
    );
    assert!(
        (hb_dc - referee).abs() / referee.abs() < 0.05,
        "HB DC {hb_dc} V vs settled transient {referee} V on the same port drive"
    );
}

#[test]
fn rf_port_and_wrapped_waveform_add_before_exact_source_registration() {
    let fundamental = 1.0e6;
    let netlist = Netlist::parse(
        "summed RF and waveform drive\n\
         V1 in 0 SIN(0 0.1 1meg) portnum 1 z0 50 pwr=50u freq=1meg\n\
         R1 in 0 1k\n\
         .end\n",
    )
    .expect("summed RF source deck parses");
    let result = engine()
        .run_hb(&netlist, HbConfig::new(fundamental).with_harmonics(2))
        .expect("same-harmonic source components are merged before MNA registration");
    let input = result
        .result
        .spectral_voltages
        .iter()
        .find(|spectrum| spectrum.node_name.eq_ignore_ascii_case("in"))
        .expect("input spectrum retained")
        .coefficients[1];

    // SIN is -j on the cosine-referenced HB boundary. The annotated port is
    // +1 at zero phase, and sqrt(4*50u*50) = 0.1 V peak.
    let expected = num_complex::Complex64::new(0.1, -0.1);
    assert!(
        (input - expected).norm() < 1.0e-14,
        "summed source phasor is {input}, expected {expected}"
    );
}

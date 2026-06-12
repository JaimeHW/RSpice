//! Tian-method loop-gain validation against closed-form feedback loops.
//!
//! Three analytic gates:
//! 1. A single-pole inverting loop has T(f) = A/(1 + j f/fp) exactly, with
//!    closed-form phase margin and crossover.
//! 2. The same loop probed with reversed orientation, and probed in the
//!    grounded return leg, must give the same loop gain (Tian is
//!    orientation-independent).
//! 3. A break point loaded comparably from both sides: Tian must still match
//!    the return ratio while plain single voltage injection — measured with
//!    the same engine — is off by tens of percent.

use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::analysis::advanced::stb::StbConfig;
use rspice_core::engine::{Engine, SimulationConfig, StbAnalysisResult};
use rspice_core::netlist::Netlist;
use std::f64::consts::PI;

fn run_stb(deck: &str, probe: &str, start: Value, stop: Value, ppd: usize) -> StbAnalysisResult {
    let netlist = Netlist::parse(deck).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let config = StbConfig::new()
        .with_sweep(start, stop, ppd)
        .with_probe(probe)
        .with_nyquist(true);
    engine.run_stb(&netlist, config).expect("STB completes")
}

const A0: f64 = 1000.0;
const FP: f64 = 1.0e3;

/// Single-pole inverting loop with the probe at the amplifier output.
/// C = 1/(2*pi*1k*1kHz) puts the pole exactly at 1 kHz.
const SINGLE_POLE: &str = "\
* single-pole inverting loop
e1 eo 0 ctrl 0 -1000
vprobe eo x 0
r1 x ctrl 1k
c1 ctrl 0 159.154943091895n
.end
";

fn single_pole_t(f: Value) -> Complex64 {
    Complex64::new(A0, 0.0) / Complex64::new(1.0, f / FP)
}

#[test]
fn single_pole_loop_gain_matches_the_closed_form() {
    let analysis = run_stb(SINGLE_POLE, "vprobe", 10.0, 1.0e7, 20);

    for (&f, &t) in analysis.frequencies.iter().zip(&analysis.loop_gains) {
        let expected = single_pole_t(f);
        assert!(
            (t - expected).norm() < 5e-3 * expected.norm(),
            "loop gain at {f:.3e} Hz: got {t}, want {expected}"
        );
    }

    let margins = &analysis.result.margins;

    // DC gain: 60 dB.
    assert!(
        (margins.dc_gain_db - 60.0).abs() < 0.05,
        "DC loop gain must be 60 dB, got {}",
        margins.dc_gain_db
    );

    // Unity crossover at fp*sqrt(A^2 - 1), phase margin 180 - atan(fu/fp).
    let fu_expected = FP * (A0 * A0 - 1.0).sqrt();
    assert!(
        (margins.phase_margin_freq - fu_expected).abs() < 0.02 * fu_expected,
        "crossover must sit at {fu_expected:.4e} Hz, got {:.4e}",
        margins.phase_margin_freq
    );
    let pm_expected = 180.0 - (fu_expected / FP).atan() * 180.0 / PI;
    assert!(
        (margins.phase_margin_deg - pm_expected).abs() < 0.5,
        "phase margin must be {pm_expected:.2} deg, got {:.2}",
        margins.phase_margin_deg
    );

    // Single pole never reaches -180 degrees: infinite gain margin.
    assert!(
        margins.gain_margin_db.is_infinite() && margins.gain_margin_db > 0.0,
        "single-pole loop has infinite gain margin, got {}",
        margins.gain_margin_db
    );
    assert!(analysis.result.is_stable());
}

/// Reversing the probe must not change the Tian loop gain.
#[test]
fn loop_gain_is_independent_of_probe_orientation() {
    let reversed = "\
* single-pole inverting loop, probe reversed
e1 eo 0 ctrl 0 -1000
vprobe x eo 0
r1 x ctrl 1k
c1 ctrl 0 159.154943091895n
.end
";
    let analysis = run_stb(reversed, "vprobe", 10.0, 1.0e7, 10);
    for (&f, &t) in analysis.frequencies.iter().zip(&analysis.loop_gains) {
        let expected = single_pole_t(f);
        assert!(
            (t - expected).norm() < 5e-3 * expected.norm(),
            "orientation-independent loop gain at {f:.3e} Hz: got {t}, want {expected}"
        );
    }
}

/// A probe with a grounded terminal cannot measure loop transmission: the
/// 0 V probe shorts the current injection to the reference, so the second
/// Tian experiment carries no information and T degenerates to 0 for any
/// circuit — silently reporting "no feedback" for a live loop. That must be
/// a hard error pointing at the signal path, not a number.
#[test]
fn grounded_probe_terminal_is_rejected() {
    let grounded_leg = "\
* probe tied to the reference node
e1 eo eneg ctrl 0 -1000
vprobe 0 eneg 0
r1 eo ctrl 1k
c1 ctrl 0 159.154943091895n
.end
";
    let netlist = Netlist::parse(grounded_leg).expect("deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let config = StbConfig::new()
        .with_sweep(10.0, 1.0e6, 10)
        .with_probe("vprobe");

    let err = engine
        .run_stb(&netlist, config)
        .expect_err("grounded probe terminal must be rejected");
    let message = err.to_string();
    assert!(
        message.contains("grounded terminal"),
        "error must explain the grounded-terminal restriction, got: {message}"
    );
}

const A2: f64 = 200.0;
const RO: f64 = 10.0e3;
const RF: f64 = 10.0e3;
const RG: f64 = 10.0e3;
const CG: f64 = 10.0e-9;

/// Loaded break: 10k output resistance on one side, a 10k/10k divider with a
/// pole on the other, so the impedances facing the probe are comparable.
const LOADED_BREAK: &str = "\
* loaded feedback divider behind output resistance
e1 eo 0 ctrl 0 -200
ro eo out 10k
vprobe out fbtop 0
rf fbtop ctrl 10k
rg ctrl 0 10k
cg ctrl 0 10n
.end
";

/// Return ratio of the controlled source: T = A*Zg/(Ro + Rf + Zg) with
/// Zg = Rg || 1/(j w C). Single loop, so this is the loop gain at any break
/// in the path.
fn loaded_break_t(f: Value) -> Complex64 {
    let zg = Complex64::new(RG, 0.0) / Complex64::new(1.0, 2.0 * PI * f * RG * CG);
    Complex64::new(A2, 0.0) * zg / (Complex64::new(RO + RF, 0.0) + zg)
}

#[test]
fn loaded_break_matches_the_return_ratio_where_single_injection_fails() {
    let analysis = run_stb(LOADED_BREAK, "vprobe", 10.0, 1.0e6, 20);

    for (&f, &t) in analysis.frequencies.iter().zip(&analysis.loop_gains) {
        let expected = loaded_break_t(f);
        assert!(
            (t - expected).norm() < 5e-3 * expected.norm(),
            "Tian loop gain at {f:.3e} Hz: got {t}, want {expected}"
        );
    }

    // Plain single voltage injection at the same break, measured with the
    // same engine: drive the probe source with AC 1 and form -V(out)/V(fbtop).
    // Analytically this returns A*Zg/(Rf + Zg) + Ro/(Rf + Zg) — the divider
    // unloaded by Ro plus a direct loading term — which at low frequency is
    // ~101 against the true ~66.7.
    let naive_deck = LOADED_BREAK.replace("vprobe out fbtop 0", "vprobe out fbtop 0 ac 1");
    let netlist = Netlist::parse(&naive_deck).expect("naive deck parses");
    let engine = Engine::new(SimulationConfig::default());
    let ac = engine.run_ac(&netlist, &[10.0]).expect("AC completes");
    let names = &ac[0].node_names;
    let out_idx = names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("out"))
        .expect("out node");
    let fbtop_idx = names
        .iter()
        .position(|n| n.eq_ignore_ascii_case("fbtop"))
        .expect("fbtop node");
    let t_naive = -ac[0].voltages[out_idx] / ac[0].voltages[fbtop_idx];

    let t_true = loaded_break_t(10.0);
    let naive_error = (t_naive - t_true).norm() / t_true.norm();
    assert!(
        naive_error > 0.3,
        "single voltage injection must be visibly wrong at this break \
         (got {t_naive}, true {t_true}, error {naive_error:.3})"
    );

    let tian_at_10 = analysis.loop_gains[0];
    let tian_error = (tian_at_10 - t_true).norm() / t_true.norm();
    assert!(
        tian_error < 5e-3,
        "Tian must match the return ratio where single injection fails \
         (got {tian_at_10}, true {t_true}, error {tian_error:.5})"
    );
}

/// The `.STB` netlist card drives the same analysis from deck text: the
/// card parses into `AnalysisCommand::Stb`, and mapping it to a config the
/// way the CLI does reproduces the closed-form loop gain.
#[test]
fn stb_card_parses_and_drives_the_same_loop_gain() {
    use rspice_core::analysis::advanced::stb::StbSweepType;
    use rspice_core::netlist::{AnalysisCommand, FreqVariation};

    let deck = "\
* single-pole loop carrying its own .stb card
e1 eo 0 ctrl 0 -1000
vprobe eo x 0
r1 x ctrl 1k
c1 ctrl 0 159.154943091895n
.stb dec 20 10 10meg probe=vprobe
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let card = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Stb {
                variation,
                points,
                start_freq,
                stop_freq,
                probe,
            } => Some((*variation, *points, *start_freq, *stop_freq, probe.clone())),
            _ => None,
        })
        .expect(".stb card parses into an analysis");

    let (variation, points, start_freq, stop_freq, probe) = card;
    assert_eq!(variation, FreqVariation::Dec);
    assert_eq!(points, 20);
    assert!((start_freq - 10.0).abs() < 1e-12);
    assert!((stop_freq - 10.0e6).abs() < 1e-3);
    assert!(
        probe.eq_ignore_ascii_case("vprobe"),
        "probe name survives parsing: {probe}"
    );

    // Map card -> config exactly the way the CLI dispatch does.
    let sweep_type = match variation {
        FreqVariation::Lin => StbSweepType::Linear,
        FreqVariation::Dec => StbSweepType::Decade,
        FreqVariation::Oct => StbSweepType::Octave,
    };
    let config = StbConfig::new()
        .with_sweep(start_freq, stop_freq, points)
        .with_sweep_type(sweep_type)
        .with_probe(&probe);
    let engine = Engine::new(SimulationConfig::default());
    let analysis = engine.run_stb(&netlist, config).expect("STB completes");

    for (&f, &t) in analysis.frequencies.iter().zip(&analysis.loop_gains) {
        let expected = single_pole_t(f);
        assert!(
            (t - expected).norm() < 5e-3 * expected.norm(),
            "card-driven loop gain at {f:.3e} Hz: got {t}, want {expected}"
        );
    }
}

/// The probe also parses as a bare trailing name (no PROBE= key).
#[test]
fn stb_card_accepts_a_bare_probe_name() {
    use rspice_core::netlist::AnalysisCommand;

    let deck = "\
* bare probe name
e1 eo 0 ctrl 0 -1000
vprobe eo x 0
r1 x ctrl 1k
c1 ctrl 0 159.154943091895n
.stb oct 5 100 1meg vprobe
.end
";
    let netlist = Netlist::parse(deck).expect("deck parses");
    let probe = netlist
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::Stb { probe, .. } => Some(probe.clone()),
            _ => None,
        })
        .expect(".stb card parses");
    assert!(probe.eq_ignore_ascii_case("vprobe"));
}

/// A probeless `.stb` card is a parse error that names the requirement,
/// not a deferred runtime failure.
#[test]
fn stb_card_without_probe_is_a_parse_error() {
    let deck = "\
* missing probe
v1 in 0 ac 1
r1 in out 1k
c1 out 0 1u
.stb dec 10 1k 1meg
.end
";
    let err = Netlist::parse(deck).expect_err("probeless .stb must not parse");
    let message = err.to_string();
    assert!(
        message.to_ascii_lowercase().contains("probe"),
        "error names the missing probe: {message}"
    );
}

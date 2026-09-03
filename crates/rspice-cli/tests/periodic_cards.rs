//! End-to-end contracts for the authored periodic large-signal family.
//!
//! `.PSS` and `.HB` are carriers; `.PAC`, `.PNOISE` and `.ENVELOPE` linearize
//! or continue around the carrier the canonical plan bound them to. These
//! tests drive the real binary against circuits whose answers are known
//! analytically, so a route that runs but computes the wrong thing fails here
//! rather than merely producing a well-shaped artifact.

mod common;

use common::{AxisRunSet, read_json, test_dir};

use std::path::{Path, PathBuf};
use std::process::{Command, Output};

/// A first-order RC low-pass driven at 1 GHz by a small tone.
///
/// `R = 1 kΩ` and `C = 159.154943091895 pF` put the corner at exactly
/// `f_c = 1/(2πRC) = 1 MHz`, so the small-signal transfer at an offset `f` is
/// `1/(1 + j f/f_c)` with no fitting constant anywhere in the check.
const RC_CARRIER: &str = "* RC low-pass under a periodic carrier\n\
                          V1 in 0 SIN(0 0.001 1G)\n\
                          R1 in out 1k\n\
                          C1 out 0 159.154943091895p\n";

/// Corner frequency of [`RC_CARRIER`], in hertz.
const RC_CORNER_HZ: f64 = 1e6;

/// A resistive divider under the same carrier. Both legs are 1 kΩ, so the
/// output sees a 500 Ω Thevenin source and the transfer from `V1` is exactly
/// one half.
const DIVIDER_CARRIER: &str = "* resistive divider under a periodic carrier\n\
                               V1 in 0 SIN(0 0.01 1G)\n\
                               R1 in out 1k\n\
                               R2 out 0 1k\n";

fn run_deck(dir: &Path, circuit: &str, cards: &str, extra: &[&str]) -> (Output, PathBuf) {
    let deck = dir.join("deck.sp");
    std::fs::write(&deck, format!("{circuit}{cards}.END\n")).expect("write deck");
    let output_path = dir.join("result.json");
    let mut args: Vec<String> = vec![
        "--quiet".to_string(),
        "--error-format".to_string(),
        "json".to_string(),
        "run".to_string(),
        deck.to_string_lossy().into_owned(),
        "-o".to_string(),
        output_path.to_string_lossy().into_owned(),
        "-f".to_string(),
        "json".to_string(),
    ];
    args.extend(extra.iter().map(|arg| (*arg).to_string()));
    let output = Command::new(env!("CARGO_BIN_EXE_rspice"))
        .args(&args)
        .env_remove("RSPICE_OUTPUT_FORMAT")
        .env_remove("RSPICE_TEMPERATURE")
        .env_remove("RUST_LOG")
        .output()
        .expect("run rspice");
    (output, output_path)
}

fn assert_ran(output: &Output) {
    assert!(
        output.status.success(),
        "run failed ({:?}): {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn error_document(output: &Output) -> serde_json::Value {
    serde_json::from_slice(&output.stderr).unwrap_or_else(|error| {
        panic!(
            "stderr must be exactly one JSON document: {error}; stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

fn artifact(requested: &Path, tag: &str) -> PathBuf {
    let stem = requested
        .file_stem()
        .expect("output stem")
        .to_string_lossy();
    requested.with_file_name(format!("{stem}.{tag}.json"))
}

/// One real series of a published document, by canonical name.
fn real_series(document: &serde_json::Value, name: &str) -> Vec<f64> {
    signal(document, name)["values"]["samples"]
        .as_array()
        .unwrap_or_else(|| panic!("series '{name}' carries no real samples in {document:#}"))
        .iter()
        .map(|sample| sample.as_f64().expect("finite sample"))
        .collect()
}

fn signal<'doc>(document: &'doc serde_json::Value, name: &str) -> &'doc serde_json::Value {
    document["signals"]
        .as_array()
        .unwrap_or_else(|| panic!("document has no signals array: {document:#}"))
        .iter()
        .find(|signal| {
            signal["descriptor"]["canonicalName"]
                .as_str()
                .is_some_and(|candidate| candidate.eq_ignore_ascii_case(name))
        })
        .unwrap_or_else(|| panic!("document has no series '{name}' in {document:#}"))
}

/// One complex series of a published document, restricted to a PAC sideband.
fn sideband_series(document: &serde_json::Value, name: &str, sideband: i64) -> Vec<(f64, f64)> {
    let signal = document["signals"]
        .as_array()
        .expect("signals array")
        .iter()
        .find(|signal| {
            signal["descriptor"]["canonicalName"]
                .as_str()
                .is_some_and(|candidate| candidate.eq_ignore_ascii_case(name))
                && signal["qualifier"]["sideband"].as_i64() == Some(sideband)
        })
        .unwrap_or_else(|| panic!("no series '{name}' at sideband {sideband}: {document:#}"));
    signal["values"]["samples"]
        .as_array()
        .expect("complex samples")
        .iter()
        .map(|sample| {
            (
                sample["real"].as_f64().expect("real part"),
                sample["imaginary"].as_f64().expect("imaginary part"),
            )
        })
        .collect()
}

fn assert_close(actual: f64, expected: f64, tolerance: f64, what: &str) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{what}: expected {expected:e}, got {actual:e} (tolerance {tolerance:e})"
    );
}

/// `.PAC` publishes the small-signal transfer of the network it linearizes,
/// which for the RC carrier is the exact one-pole response at every swept
/// offset. The carrier it names is the `.HB` instance the plan bound it to.
#[test]
fn a_pac_sweep_publishes_the_rc_transfer_at_every_offset() {
    let dir = test_dir("pac_transfer");
    let (output, requested) = run_deck(
        &dir,
        RC_CARRIER,
        ".HB 1G\n.PAC DEC 2 1meg 10meg INPUT=V1 OUT=V(out) MAXSIDEBAND=1\n",
        &[],
    );
    assert_ran(&output);

    let document = read_json(&artifact(&requested, "pac-001"));
    assert_eq!(document["resultKind"], "pac");
    assert_eq!(document["analysis"]["tag"], "pac-001");
    assert_eq!(
        document["parentAnalysis"]["tag"], "hb-001",
        "a PAC document names the carrier it linearized around"
    );
    assert_eq!(document["payload"]["fundamentalFrequency"], 1e9);

    let offsets = document["axes"][0]["values"]["values"]
        .as_array()
        .expect("offset axis")
        .iter()
        .map(|value| value.as_f64().expect("finite offset"))
        .collect::<Vec<_>>();
    let response = sideband_series(&document, "v(out)", 0);
    assert_eq!(
        response.len(),
        offsets.len(),
        "the response does not cover the swept offsets"
    );
    for (offset, (real, imaginary)) in offsets.iter().zip(response) {
        // H(f) = 1 / (1 + j f/f_c) for a unit input at the input sideband.
        let ratio = offset / RC_CORNER_HZ;
        let denominator = 1.0 + ratio * ratio;
        assert_close(real, 1.0 / denominator, 1e-9, "Re H at {offset} Hz");
        assert_close(imaginary, -ratio / denominator, 1e-9, "Im H at {offset} Hz");
    }
}

/// A driven `.PNOISE` run reports the absolute output noise PSD of the network
/// around its carrier. The divider presents a 500 Ω Thevenin source, so the
/// total is `4kTR` with each resistor contributing exactly half, and the
/// input-referred density is that divided by the squared transfer of one half.
#[test]
fn a_driven_pnoise_spectrum_is_the_thermal_density_of_its_network() {
    let dir = test_dir("pnoise_thermal");
    let (output, requested) = run_deck(
        &dir,
        DIVIDER_CARRIER,
        ".HB 1G\n.PNOISE DEC 2 1meg 10meg OUT=V(out) INPUT=V1 MAXSIDEBAND=1\n",
        &[],
    );
    assert_ran(&output);

    let document = read_json(&artifact(&requested, "pnoise-001"));
    assert_eq!(document["resultKind"], "pnoise");
    assert_eq!(document["analysis"]["tag"], "pnoise-001");
    assert_eq!(document["parentAnalysis"]["tag"], "hb-001");

    // 4kT * 500 ohm at the engine's default 300.15 K reference.
    let boltzmann = 1.380_649e-23_f64;
    let expected_total = 4.0 * boltzmann * 300.15 * 500.0;
    let total = real_series(&document, "output_noise");
    assert!(!total.is_empty(), "the sweep published no offsets");
    for density in &total {
        assert_close(
            *density,
            expected_total,
            expected_total * 1e-6,
            "output noise density",
        );
    }
    // A thermal source is white, so every offset carries the same density and
    // the two equal resistors split it evenly.
    for leg in ["contribution:r1 thermal", "contribution:r2 thermal"] {
        for density in real_series(&document, leg) {
            assert_close(
                density,
                expected_total / 2.0,
                expected_total * 1e-6,
                "one leg's contribution",
            );
        }
    }
    // Referring to the input divides by |H|^2 = 1/4.
    for density in real_series(&document, "input_referred_noise") {
        assert_close(
            density,
            expected_total * 4.0,
            expected_total * 4e-6,
            "input-referred density",
        );
    }
}

/// `.ENVELOPE` continues the exact `.HB` carrier the plan bound it to, so the
/// published trajectory is the modulated tone's slow-time evolution: it covers
/// the authored slow-time window, respects the authored maximum step, and its
/// output amplitude never exceeds what the RC network passes at the carrier.
#[test]
fn an_envelope_run_continues_its_carrier_across_the_authored_slow_time() {
    let dir = test_dir("envelope_carrier");
    let duration = 2e-9_f64;
    let max_step = 2e-10_f64;
    let (output, requested) = run_deck(
        &dir,
        RC_CARRIER,
        ".HB 1G\n.ENVELOPE TSTOP=2n MAXSTEP=0.2n\n",
        &[],
    );
    assert_ran(&output);

    let document = read_json(&artifact(&requested, "env-001"));
    assert_eq!(document["resultKind"], "envelope");
    assert_eq!(document["analysis"]["tag"], "env-001");
    assert_eq!(document["parentAnalysis"]["tag"], "hb-001");
    assert_eq!(document["payload"]["carrier"]["fundamentalFrequency"], 1e9);
    assert_eq!(document["payload"]["carrier"]["converged"], true);
    assert_eq!(
        document["payload"]["continuation"]["slowTimeDuration"], duration,
        "the continuation must publish the authored slow-time window"
    );

    let times = document["axes"][0]["values"]["values"]
        .as_array()
        .expect("time axis")
        .iter()
        .map(|value| value.as_f64().expect("finite time"))
        .collect::<Vec<_>>();
    assert!(times.len() > 1, "an envelope needs a trajectory");
    assert_close(
        times.last().copied().expect("final time"),
        duration,
        duration * 1e-9,
        "the trajectory must reach the authored stop time",
    );
    for pair in times.windows(2) {
        assert!(
            pair[1] > pair[0] && pair[1] - pair[0] <= max_step * (1.0 + 1e-9),
            "slow-time step {} exceeds the authored maximum {max_step:e}",
            pair[1] - pair[0]
        );
    }

    // 1 V driven through the RC at 1 GHz, ten decades above the 1 MHz corner:
    // |H| = 1/sqrt(1 + (f/f_c)^2), so the envelope can never exceed that.
    let ceiling = 1.0 / (1.0 + (1e9 / RC_CORNER_HZ).powi(2)).sqrt();
    for value in real_series(&document, "v(out)") {
        assert!(
            value.abs() <= ceiling * 1.05,
            "the continued output {value:e} exceeds what the carrier network passes ({ceiling:e})"
        );
    }
}

/// A `.PSS` card under a `.STEP` axis runs at every coordinate and publishes
/// its own document there. The stepped resistance moves the RC corner, so each
/// coordinate's steady-state amplitude is the analytic magnitude at its own
/// resistance rather than a repeat of the first coordinate's.
#[test]
fn a_stepped_pss_card_solves_each_coordinate_against_its_own_network() {
    let dir = test_dir("pss_axis");
    let (output, requested) = run_deck(
        &dir,
        "* stepped RC low-pass\n\
         .param rload=1k\n\
         V1 in 0 SIN(0 1 1G)\n\
         R1 in out {rload}\n\
         C1 out 0 1p\n",
        ".step param rload list 1k 2k\n.PSS FUND=1G\n",
        &[],
    );
    assert_ran(&output);

    let run_set = AxisRunSet::read(&requested);
    assert_eq!(run_set.coordinates.len(), 2);
    for (index, resistance) in [(1usize, 1e3_f64), (2, 2e3)] {
        let coordinate = run_set.coordinate(index);
        let document = read_json(coordinate.only_artifact());
        assert_eq!(document["resultKind"], "pss");
        assert_eq!(document["analysis"]["tag"], "pss-001");
        assert_eq!(
            document["coordinate"]["assignments"][0]["value"]["value"], resistance,
            "coordinate {index} does not carry its own stepped resistance"
        );

        // |H(f0)| = 1 / sqrt(1 + (2 pi f0 R C)^2) for a 1 V drive.
        let omega_rc = 2.0 * std::f64::consts::PI * 1e9 * resistance * 1e-12;
        let expected_peak = 1.0 / (1.0 + omega_rc * omega_rc).sqrt();
        let peak = real_series(&document, "v(out)")
            .into_iter()
            .fold(f64::NEG_INFINITY, f64::max);
        assert_close(
            peak,
            expected_peak,
            expected_peak * 5e-3,
            "steady-state peak at this coordinate",
        );
    }
}

/// An authored Monte Carlo card under a `.STEP` axis runs once per coordinate,
/// and each coordinate draws its own reproducible stream: the samples are
/// stable across runs and distinct between coordinates, so a parametric study
/// is not one sample repeated.
#[test]
fn a_stepped_monte_carlo_card_draws_a_distinct_reproducible_stream_per_coordinate() {
    let deck = "* stepped Monte Carlo over a resistive divider\n\
                .param rload=1k\n\
                V1 in 0 DC 1\n\
                R1 in out 1k\n\
                R2 out 0 {rload}\n";
    let cards = ".step param rload list 1k 4k\n.mc 4 seed 7 gauss 0.05\n";

    let samples = |tag: &str| -> Vec<Vec<f64>> {
        let dir = test_dir(tag);
        let (output, requested) = run_deck(&dir, deck, cards, &[]);
        assert_ran(&output);
        let run_set = AxisRunSet::read(&requested);
        assert_eq!(run_set.coordinates.len(), 2);
        run_set
            .coordinates
            .iter()
            .map(|coordinate| {
                let document = read_json(coordinate.only_artifact());
                assert_eq!(document["resultKind"], "monte-carlo");
                assert_eq!(document["analysis"]["tag"], "mc-001");
                document["payload"]["statistics"]
                    .as_array()
                    .expect("Monte Carlo statistics")
                    .iter()
                    .find(|entry| entry["name"] == "V(OUT)")
                    .expect("the divider output is a tracked variable")["samples"]
                    .as_array()
                    .expect("per-run samples")
                    .iter()
                    .map(|sample| sample.as_f64().expect("finite sample"))
                    .collect::<Vec<_>>()
            })
            .collect()
    };

    let first = samples("mc_axis_first");
    let second = samples("mc_axis_second");
    assert_eq!(
        first, second,
        "a coordinate's Monte Carlo stream must be reproducible run to run"
    );
    assert_eq!(
        first[0].len(),
        4,
        "every coordinate runs the authored trials"
    );
    assert_ne!(
        first[0], first[1],
        "two coordinates must not draw the identical variation vector"
    );

    // The nominal divider outputs are 1/2 and 4/5; a 5% parameter spread keeps
    // every trial near its own coordinate's nominal rather than the other's.
    for (samples, nominal) in first.iter().zip([0.5_f64, 0.8]) {
        for sample in samples {
            assert!(
                (sample - nominal).abs() < 0.05,
                "a trial at nominal {nominal} produced {sample}"
            );
        }
    }
}

/// A deck with two carriers binds each dependent card to the one the canonical
/// plan named, not to whichever carrier ran last. Both `.PAC` cards here sweep
/// the same network, so only the declared parent tells them apart — which is
/// exactly the binding a surface must not re-derive for itself.
#[test]
fn two_carriers_each_keep_the_dependent_card_the_plan_bound_to_them() {
    let dir = test_dir("two_carriers");
    let (output, requested) = run_deck(
        &dir,
        RC_CARRIER,
        ".PSS FUND=1G\n\
         .HB 1G\n\
         .PAC DEC 2 1meg 10meg INPUT=V1 OUT=V(out) MAXSIDEBAND=1 FROM=PSS\n\
         .PAC DEC 2 1meg 10meg INPUT=V1 OUT=V(out) MAXSIDEBAND=1 FROM=HB\n",
        &[],
    );
    assert_ran(&output);

    for (analysis, carrier) in [("pac-001", "pss-001"), ("pac-002", "hb-001")] {
        let document = read_json(&artifact(&requested, analysis));
        assert_eq!(document["analysis"]["tag"], analysis);
        assert_eq!(
            document["parentAnalysis"]["tag"], carrier,
            "{analysis} named the wrong carrier"
        );
        // Both carriers describe the same linear network, so both sweeps must
        // reproduce the same transfer; a mis-bound card would still publish,
        // which is why the parent identity is checked as well as the numbers.
        let response = sideband_series(&document, "v(out)", 0);
        let (real, imaginary) = response[0];
        assert_close(real, 0.5, 1e-6, "Re H at the corner");
        assert_close(imaginary, -0.5, 1e-6, "Im H at the corner");
    }
}

/// A dependent card with no carrier in the deck is refused while planning,
/// before any solver work: there is nothing for it to linearize around and
/// inventing a carrier would be a fabricated operating point.
#[test]
fn a_dependent_card_without_a_carrier_is_refused_before_any_solve() {
    let dir = test_dir("orphan_pac");
    let (output, requested) = run_deck(
        &dir,
        RC_CARRIER,
        ".PAC DEC 2 1meg 10meg INPUT=V1 OUT=V(out) MAXSIDEBAND=1\n",
        &[],
    );
    assert_eq!(
        output.status.code(),
        Some(2),
        "an unbindable card is a usage error; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let message = error_document(&output)["error"]["message"]
        .as_str()
        .expect("a refusal explains itself")
        .to_owned();
    assert!(
        message.contains(".PAC") && message.contains(".PSS") && message.contains(".HB"),
        "the refusal must name the card and the carriers it accepts: {message}"
    );
    assert!(
        !requested.exists() && !artifact(&requested, "pac-001").exists(),
        "a refused deck publishes no artifact"
    );
}

/// A `.PAC` card whose carrier cannot supply the harmonics its sideband span
/// needs is a typed refusal, not a silently truncated sweep.
#[test]
fn a_sideband_span_wider_than_its_carrier_is_refused() {
    let dir = test_dir("pac_span");
    let (output, requested) = run_deck(
        &dir,
        RC_CARRIER,
        ".HB 1G\n.PAC DEC 2 1meg 10meg INPUT=V1 OUT=V(out) MAXSIDEBAND=40\n",
        &[],
    );
    assert!(
        !output.status.success(),
        "an unsatisfiable span was accepted"
    );
    assert_ne!(
        output.status.code(),
        Some(101),
        "authored input must not panic"
    );
    let message = error_document(&output)["error"]["message"]
        .as_str()
        .expect("a refusal explains itself")
        .to_owned();
    assert!(
        message.contains("harmonics"),
        "the refusal must name the harmonic capacity it needed: {message}"
    );
    assert!(
        !artifact(&requested, "pac-001").exists(),
        "a refused PAC card publishes no artifact"
    );
}

/// `--pss-freq` and an authored `.PSS` card both request a periodic steady
/// state, so combining them stays an explicit conflict rather than one route
/// silently winning.
#[test]
fn the_pss_flag_and_an_authored_pss_card_are_an_explicit_conflict() {
    let dir = test_dir("flag_conflict");
    let (output, requested) = run_deck(&dir, RC_CARRIER, ".PSS FUND=1G\n", &["--pss-freq", "1e9"]);

    assert_eq!(
        output.status.code(),
        Some(2),
        "a command-line/deck conflict is a usage error; stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json = error_document(&output);
    assert_eq!(json["error"]["code"], "invalid_argument");
    let message = json["error"]["message"]
        .as_str()
        .expect("the conflict names both sources");
    assert!(
        message.contains("--pss-freq") && message.contains(".PSS"),
        "unexpected conflict message: {message}"
    );
    assert!(!requested.exists(), "a refused deck publishes no artifact");
}

/// The `--pss-freq` route is unchanged by the authored-card route: it still
/// supersedes the deck's own cards and publishes under the bare mode tag.
#[test]
fn the_pss_flag_still_runs_a_deck_without_an_authored_card() {
    let dir = test_dir("flag_only");
    let (output, requested) = run_deck(&dir, RC_CARRIER, ".OP\n", &["--pss-freq", "1e9"]);

    assert_ran(&output);
    assert!(
        requested.exists(),
        "the flag route still publishes its artifact"
    );
    let document = read_json(&requested);
    assert_eq!(document["resultKind"], "pss");
    assert_eq!(document["analysis"]["tag"], "pss-001");
}

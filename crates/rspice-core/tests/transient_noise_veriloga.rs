//! Transient noise (`.TRAN … NOISEFMAX=`) beyond the native device catalog.
//!
//! The native mechanisms have their own analytic oracles beside the injection
//! code. This suite owns what a transient-noise run promises about everything
//! else that can carry a noise source, and about the run's own contract:
//!
//! * Verilog-A noise functions — in runtime modules, in mixed Verilog-AMS
//!   hosts and in generated devices — are rendered, at their own densities;
//! * one noise-function call reaching several branches is one coherent
//!   process, and two calls are independent whatever their names;
//! * flicker and tabulated densities keep their shape in time;
//! * noise the module shapes through `ddt`, `idt` or `laplace_*` renders at
//!   `|H|²·S`, and shaping no finite-state filter represents, or whose
//!   dynamics move with the bias, is refused by name;
//! * the run is a function of its seed alone, rejected steps included;
//! * the run seed is one seed, whatever authored the noise;
//! * a transient-noise run that would need its noise streams persisted is a
//!   capability refusal, typed, at both places that can be asked for one.
//!
//! Every statistical assertion states its interval and where the interval
//! comes from before it runs, and every run is seeded, so a failure is a
//! defect rather than a draw.

use rspice_core::abort_signal::NoAbort;
use rspice_core::engine::{TransientCheckpointStart, TransientStartupMode};
use rspice_core::identity::AnalysisKind;
use rspice_core::{Engine, Netlist, SimulationConfig, SimulationError};
#[cfg(feature = "veriloga")]
use std::collections::BTreeMap;

/// The one token both checkpoint refusals carry.
const CHECKPOINT_TOKEN: &str = "analysis.tran.checkpoint.transient_noise";

fn engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

/// Assert a refusal is the typed transient-noise checkpoint refusal.
fn assert_checkpoint_refusal(error: &SimulationError, site: &str) {
    let SimulationError::UnsupportedCapability(refusal) = error else {
        panic!("{site}: expected a capability refusal, got {error}");
    };
    assert_eq!(refusal.capability, CHECKPOINT_TOKEN, "{site}: {error}");
    assert_eq!(
        error.descriptor().analysis_kind,
        Some(AnalysisKind::Tran),
        "{site}: the refusal must name the transient as its analysis kind"
    );
    assert!(
        error.to_string().contains("NOISEFMAX"),
        "{site}: the detail must still name the keyword for a person: {error}"
    );
}

/// F8: a checkpoint or resume of a transient-noise run is refused as a
/// capability, at the streamed-request validator and at the run itself.
#[test]
fn checkpointing_a_transient_noise_run_is_a_typed_capability_refusal() {
    let netlist = Netlist::parse(
        "checkpointed transient noise\n\
         R1 out 0 1k\n\
         C1 out 0 1p\n\
         .TRAN 1n 100n NOISEFMAX=500meg NOISESEED=7\n\
         .END\n",
    )
    .expect("the deck parses");

    let validator = engine()
        .validate_transient_checkpoint_request(
            &netlist,
            100.0e-9,
            1.0e-9,
            TransientCheckpointStart::Fresh(TransientStartupMode::OperatingPoint),
            &[50.0e-9],
            &NoAbort,
        )
        .expect_err("a scheduled checkpoint of a transient-noise run must be refused");
    assert_checkpoint_refusal(&validator, "streamed checkpoint validator");

    let run = engine()
        .run_tran_checkpointed(&netlist, 100.0e-9, 1.0e-9)
        .expect_err("a retained final checkpoint of a transient-noise run must be refused");
    assert_checkpoint_refusal(&run, "checkpointed transient run");
}

/// One run, one seed: an authored `TRNOISE` source draws from `NOISESEED=`
/// when the transient card names one, exactly as device noise does, and
/// from `.OPTIONS SEED` otherwise.
#[test]
fn trnoise_draws_from_the_transient_cards_noise_seed() {
    // An ideal source pins `in` to the TRNOISE value, so the resistor's own
    // injected thermal current cannot move the observed waveform.
    let waveform = |options: &str, card_seed: &str| {
        let deck = format!(
            "trnoise seed\n\
             {options}\n\
             V1 in 0 TRNOISE(1m 1n 0 0)\n\
             R1 in 0 1k\n\
             .TRAN 1n 200n NOISEFMAX=500meg {card_seed}\n\
             .END\n"
        );
        let netlist = Netlist::parse(&deck).expect("the deck parses");
        let result = engine()
            .run_tran(&netlist, 200.0e-9, 1.0e-9)
            .expect("the run converges");
        let values = result
            .try_voltage_waveform_named("in")
            .expect("the source node is retained")
            .to_vec();
        assert!(
            values.iter().any(|value| *value != 0.0),
            "the TRNOISE source must draw"
        );
        (result.time, values)
    };

    let card = waveform("", "NOISESEED=42");
    let options = waveform(".OPTIONS SEED=42", "");
    let both = waveform(".OPTIONS SEED=7", "NOISESEED=42");
    let other = waveform(".OPTIONS SEED=7", "");

    assert_eq!(
        card, options,
        "NOISESEED=42 and .OPTIONS SEED=42 name the same run seed"
    );
    assert_eq!(
        both, card,
        "the transient card's NOISESEED wins over .OPTIONS SEED"
    );
    assert_ne!(
        other.1, card.1,
        "a different run seed must draw a different TRNOISE realization"
    );
}

// ---------------------------------------------------------------------------
// Verilog-A noise processes
// ---------------------------------------------------------------------------

#[cfg(feature = "veriloga")]
mod veriloga {
    use super::*;
    use rspice_core::engine::TransientResult;
    use std::io::Write;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static MODEL_SEQUENCE: AtomicU64 = AtomicU64::new(0);

    /// A `.va` written to a unique path, deleted when the guard drops. The
    /// Verilog-A cache is keyed by path, so a shared name is a shared entry.
    struct ModelFile(PathBuf);

    impl ModelFile {
        fn new(name: &str, source: &str) -> Self {
            let sequence = MODEL_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let mut path = std::env::temp_dir();
            path.push(format!(
                "rspice_transient_noise_{name}_{}_{sequence}.va",
                std::process::id()
            ));
            let mut file = std::fs::File::create(&path).expect("create the model file");
            file.write_all(source.as_bytes()).expect("write the model");
            Self(path)
        }

        fn deck_path(&self) -> String {
            self.0.display().to_string().replace('\\', "/")
        }
    }

    impl Drop for ModelFile {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.0);
        }
    }

    fn run(deck: &str, tstop: f64, max_step: f64) -> TransientResult {
        let netlist = Netlist::parse(deck).expect("the deck parses");
        engine()
            .run_tran(&netlist, tstop, max_step)
            .unwrap_or_else(|error| panic!("the deck must run: {error}"))
    }

    /// The held noise samples a node shows, keyed by sample boundary.
    ///
    /// Every sample boundary `j·NT` is a solver breakpoint, and a node whose
    /// voltage is an algebraic function of the injected currents shows there
    /// the sample held over `((j-1)·NT, j·NT]`.
    fn held_samples(result: &TransientResult, node: &str, nt: f64) -> BTreeMap<usize, f64> {
        let waveform = result
            .try_voltage_waveform_named(node)
            .unwrap_or_else(|| panic!("node '{node}' is retained"));
        let mut samples = BTreeMap::new();
        for (time, value) in result.time.iter().zip(waveform) {
            let boundary = (time / nt).round();
            if boundary >= 1.0 && (time - boundary * nt).abs() <= 1.0e-6 * nt {
                // The first point at a boundary is the one that closes its
                // hold interval; a later point within this tolerance may
                // already lie in the next interval, as when an event
                // micro-steps just past the boundary.
                samples.entry(boundary as usize).or_insert(*value);
            }
        }
        samples
    }

    /// The samples whose hold interval ends in `(from, to]`.
    fn window(samples: &BTreeMap<usize, f64>, nt: f64, from: f64, to: f64) -> Vec<f64> {
        let first = (from / nt).round() as usize + 1;
        let last = (to / nt).round() as usize;
        let values: Vec<f64> = (first..=last)
            .map(|boundary| {
                *samples
                    .get(&boundary)
                    .unwrap_or_else(|| panic!("the run did not land on boundary {boundary}"))
            })
            .collect();
        values
    }

    fn mean(values: &[f64]) -> f64 {
        values.iter().sum::<f64>() / values.len() as f64
    }

    /// Population standard deviation about the record's own mean.
    fn std_dev(values: &[f64]) -> f64 {
        let centre = mean(values);
        (values.iter().map(|v| (v - centre).powi(2)).sum::<f64>() / values.len() as f64).sqrt()
    }

    fn correlation(left: &[f64], right: &[f64]) -> f64 {
        let (left_mean, right_mean) = (mean(left), mean(right));
        let covariance: f64 = left
            .iter()
            .zip(right)
            .map(|(l, r)| (l - left_mean) * (r - right_mean))
            .sum();
        covariance / (left.len() as f64 * std_dev(left) * std_dev(right))
    }

    fn assert_within(label: &str, measured: f64, low: f64, high: f64) {
        println!("{label}: {measured:e} in [{low:e}, {high:e}]");
        assert!(
            (low..=high).contains(&measured),
            "{label}: {measured:e} is outside [{low:e}, {high:e}]"
        );
    }

    /// A one-sided Welch periodogram of a sampled sequence, per hertz:
    /// Hann-windowed segments at 50% overlap, record mean removed.
    fn welch(samples: &[f64], nt: f64, segment: usize) -> Vec<(f64, f64)> {
        use rustfft::{FftPlanner, num_complex::Complex};
        let centre = mean(samples);
        let window: Vec<f64> = (0..segment)
            .map(|index| {
                0.5 - 0.5 * (std::f64::consts::TAU * index as f64 / (segment as f64 - 1.0)).cos()
            })
            .collect();
        let normalization: f64 = window.iter().map(|w| w * w).sum();
        let fft = FftPlanner::new().plan_fft_forward(segment);
        let mut accumulated = vec![0.0; segment / 2 + 1];
        let mut segments = 0usize;
        let mut start = 0usize;
        while start + segment <= samples.len() {
            let mut buffer: Vec<Complex<f64>> = samples[start..start + segment]
                .iter()
                .zip(&window)
                .map(|(value, weight)| Complex::new((value - centre) * weight, 0.0))
                .collect();
            fft.process(&mut buffer);
            for (bin, slot) in accumulated.iter_mut().enumerate() {
                let fold = if bin == 0 || bin == segment / 2 {
                    1.0
                } else {
                    2.0
                };
                *slot += fold * nt * buffer[bin].norm_sqr() / normalization;
            }
            segments += 1;
            start += segment / 2;
        }
        assert!(segments > 0, "the record must cover one segment");
        let resolution = 1.0 / (segment as f64 * nt);
        accumulated
            .into_iter()
            .enumerate()
            .map(|(bin, total)| (bin as f64 * resolution, total / segments as f64))
            .collect()
    }

    /// The bin-by-bin mean of several periodograms of one length.
    fn average_estimates(estimates: &[Vec<(f64, f64)>]) -> Vec<(f64, f64)> {
        (0..estimates[0].len())
            .map(|bin| {
                let density = estimates
                    .iter()
                    .map(|estimate| estimate[bin].1)
                    .sum::<f64>()
                    / estimates.len() as f64;
                (estimates[0][bin].0, density)
            })
            .collect()
    }

    /// Mean measured and model density over the bins in `[low, high]`. The
    /// model is averaged over the same bins, so the comparison is of densities
    /// rather than of a band mean against a point value.
    fn band(
        estimate: &[(f64, f64)],
        low: f64,
        high: f64,
        model: impl Fn(f64) -> f64,
    ) -> (f64, f64) {
        let bins: Vec<&(f64, f64)> = estimate
            .iter()
            .filter(|(frequency, _)| *frequency >= low && *frequency <= high)
            .collect();
        assert!(!bins.is_empty(), "the band holds no bin");
        let count = bins.len() as f64;
        (
            bins.iter().map(|(_, density)| density).sum::<f64>() / count,
            bins.iter()
                .map(|(frequency, _)| model(*frequency))
                .sum::<f64>()
                / count,
        )
    }

    /// Least-squares slope of `log10(density)` against `log10(frequency)`.
    fn log_log_slope(estimate: &[(f64, f64)], low: f64, high: f64) -> f64 {
        let points: Vec<(f64, f64)> = estimate
            .iter()
            .filter(|(f, d)| *f >= low && *f <= high && *d > 0.0)
            .map(|(f, d)| (f.log10(), d.log10()))
            .collect();
        let count = points.len() as f64;
        let mean_x = points.iter().map(|(x, _)| x).sum::<f64>() / count;
        let mean_y = points.iter().map(|(_, y)| y).sum::<f64>() / count;
        let covariance: f64 = points
            .iter()
            .map(|(x, y)| (x - mean_x) * (y - mean_y))
            .sum();
        let variance: f64 = points.iter().map(|(x, _)| (x - mean_x).powi(2)).sum();
        covariance / variance
    }

    const NOISY_MIXED: &str = r#"
`include "disciplines.vams"
`timescale 1ns/1ps
module noisy_mixed(p, n, q);
  inout p, n; electrical p, n; output q; reg q;
  initial q = 1'b0; always #50 q = ~q;
  analog I(p, n) <+ V(p, n)/1000.0 + white_noise(1e-18, "w");
endmodule
"#;

    const NOISY_VA: &str = r#"
`include "disciplines.vams"
`timescale 1ns/1ps
module noisy_va(p, n);
  inout p, n; electrical p, n;
  analog I(p, n) <+ V(p, n)/1000.0 + white_noise(1e-18, "w");
endmodule
"#;

    /// The sample interval of `.TRAN 1n 2u NOISEFMAX=500meg`.
    const NT: f64 = 1.0e-9;

    /// Test 1, the P0.6 probe `p06a-trnoise-module` verbatim: a mixed host,
    /// its analog-only twin and a native resistor as the control, in one run.
    ///
    /// Each module is a 1 kΩ conductance with a `white_noise(1e-18)` current,
    /// so its node holds `σ = sqrt(1e-18·1e6·5e8) = 22.36 mV`; the resistor's
    /// is `sqrt(4kT·1k·5e8) = 91.0 µV`. Over `[0.1u, 2u]` the 1900 held
    /// samples are independent, and the 99% χ² interval of a standard
    /// deviation from 1900 samples is ±4.2%, which gives the probe's
    /// intervals. Before P0.9 both modules read σ = 0.
    #[test]
    fn mixed_and_runtime_module_noise_reach_the_transient() {
        let mixed = ModelFile::new("noisy_mixed", NOISY_MIXED);
        let analog = ModelFile::new("noisy_va", NOISY_VA);
        let deck = format!(
            "* p06a-trnoise-module\n\
             x1 p 0 q noisy_mixed\n\
             rq q 0 10k\n\
             x2 pv 0 noisy_va\n\
             rn pn 0 1k\n\
             .va \"{}\" noisy_mixed\n\
             .va \"{}\" noisy_va\n\
             .tran 1n 2u NOISEFMAX=500meg NOISESEED=7\n\
             .end\n",
            mixed.deck_path(),
            analog.deck_path()
        );
        let result = run(&deck, 2.0e-6, 1.0e-9);
        let toggles = result
            .digital_trace_named("q")
            .expect("the mixed host's digital output is traced")
            .len();
        assert!(
            toggles >= 40,
            "x1 must run as a mixed host: q toggles every 50 ns, traced {toggles} points"
        );

        let sigma = |node: &str| {
            std_dev(&window(
                &held_samples(&result, node, NT),
                NT,
                0.1e-6,
                2.0e-6,
            ))
        };
        assert_within("σ(V(p)) mixed host (V)", sigma("p"), 21.4e-3, 23.3e-3);
        assert_within("σ(V(pv)) runtime module (V)", sigma("pv"), 21.4e-3, 23.3e-3);
        assert_within(
            "σ(V(pn)) native resistor (V)",
            sigma("pn"),
            87.2e-6,
            94.8e-6,
        );
    }

    const COHERENT: &str = r#"
`include "disciplines.vams"
module coherent(a, b, c, d);
  inout a, b, c, d; electrical a, b, c, d;
  real n;
  analog begin
    n = white_noise(1e-18);
    V(a, b) <+ 1.0*n;
    V(c, d) <+ -2.0*n;
  end
endmodule
"#;

    const NAMED_TWICE: &str = r#"
`include "disciplines.vams"
module named_twice(a, b, c, d);
  inout a, b, c, d; electrical a, b, c, d;
  analog begin
    V(a, b) <+ white_noise(1e-18, "x");
    V(c, d) <+ white_noise(1e-18, "x");
  end
endmodule
"#;

    /// Test 2, LRM 2.4 §4.6.4.6: correlation follows the noise-function call.
    ///
    /// One call reaching two potential branches is one process: the branches
    /// carry `n` and `-2n` from one train, so their correlation is -1 and
    /// their σ ratio 2, both up to rounding alone; -0.999 and 2 ± 0.02 leave
    /// room for nothing statistical. Two calls that share the label "x" are
    /// two processes: the correlation of 1900 independent sample pairs has
    /// standard deviation `1/sqrt(1900)`, so `|ρ| < 4/sqrt(N)` is a 4σ bound.
    #[test]
    fn correlation_follows_the_noise_function_call_not_its_name() {
        let coherent = ModelFile::new("coherent", COHERENT);
        let named = ModelFile::new("named_twice", NAMED_TWICE);
        let deck = format!(
            "* noise correlation follows the call\n\
             x1 a 0 c 0 coherent\n\
             ra a 0 1k\n\
             rc c 0 1k\n\
             x2 e 0 f 0 named_twice\n\
             re e 0 1k\n\
             rf f 0 1k\n\
             .va \"{}\" coherent\n\
             .va \"{}\" named_twice\n\
             .tran 1n 2u NOISEFMAX=500meg NOISESEED=7\n\
             .end\n",
            coherent.deck_path(),
            named.deck_path()
        );
        let result = run(&deck, 2.0e-6, 1.0e-9);
        let record = |node: &str| window(&held_samples(&result, node, NT), NT, 0.1e-6, 2.0e-6);
        let (a, c) = (record("a"), record("c"));
        let rho = correlation(&a, &c);
        println!("ρ(V(a), V(c)) = {rho}");
        assert!(
            rho < -0.999,
            "one call's two injections must be coherent: ρ = {rho}"
        );
        assert_within("σ(V(c))/σ(V(a))", std_dev(&c) / std_dev(&a), 1.98, 2.02);
        assert_within("σ(V(a)) (V)", std_dev(&a), 21.4e-6, 23.3e-6);

        let (e, f) = (record("e"), record("f"));
        let rho = correlation(&e, &f);
        let bound = 4.0 / (e.len() as f64).sqrt();
        println!("ρ(V(e), V(f)) = {rho}, bound {bound}");
        assert!(
            rho.abs() < bound,
            "two calls sharing a name must stay independent: ρ = {rho}, bound {bound}"
        );
    }

    /// Instances in a bank: independent records of one process kind, which is
    /// what an estimate's precision rests on.
    const BANK: usize = 16;

    /// A bank of `BANK` instances of `module`, each on its own node `p<k>`,
    /// run as `.TRAN 10n 200u NOISEFMAX=50meg NOISEFMIN=10k NOISESEED=7`, and
    /// the bin-averaged Welch estimate of their node voltages.
    fn bank_estimate(module: &str, source: &str, segment: usize) -> Vec<(f64, f64)> {
        const BANK_NT: f64 = 10.0e-9;
        let model = ModelFile::new(module, source);
        let mut deck = format!("* {module} bank\n");
        for instance in 1..=BANK {
            deck.push_str(&format!("x{instance} p{instance} 0 {module}\n"));
        }
        deck.push_str(&format!(
            ".va \"{}\" {module}\n\
             .tran 10n 200u NOISEFMAX=50meg NOISEFMIN=10k NOISESEED=7\n\
             .end\n",
            model.deck_path()
        ));
        let result = run(&deck, 200.0e-6, 10.0e-9);
        let estimates: Vec<Vec<(f64, f64)>> = (1..=BANK)
            .map(|instance| {
                let samples = window(
                    &held_samples(&result, &format!("p{instance}"), BANK_NT),
                    BANK_NT,
                    0.0,
                    200.0e-6,
                );
                welch(&samples, BANK_NT, segment)
            })
            .collect();
        average_estimates(&estimates)
    }

    /// Test 3: `flicker_noise(1e-20, 1.0)` into its own 1 kΩ holds the node
    /// at `1e-20·(1k)²/f = 1e-14/f` V²/Hz.
    ///
    /// Sixteen 20000-sample records in 4096-sample segments give 128
    /// segment periodograms, so one bin's estimate has a relative standard
    /// deviation near 1/sqrt(128·0.7) ≈ 11% once the 50% overlap is counted.
    /// The slope fit over the 81 bins of 20 k–2 MHz then has a standard error
    /// near 0.03, so ±0.1 is over 3σ. The density is compared over
    /// 200 k–2 MHz, the 74 bins where the kernel's truncation at NOISEFMIN —
    /// a ripple of relative size `(f/fmin)^-1/2/sqrt(pi)` — is below 5%; the
    /// band mean's standard error is under 3%, so 10% is over 3σ.
    #[test]
    fn flicker_noise_keeps_its_slope_and_density_in_time() {
        let estimate = bank_estimate(
            "flick",
            r#"
`include "disciplines.vams"
module flick(p, n);
  inout p, n; electrical p, n;
  analog I(p, n) <+ V(p, n)/1000.0 + flicker_noise(1e-20, 1.0, "f");
endmodule
"#,
            4096,
        );
        let slope = log_log_slope(&estimate, 20.0e3, 2.0e6);
        assert_within("flicker slope over 20 k–2 MHz", slope, -1.1, -0.9);
        let (measured, model) = band(&estimate, 200.0e3, 2.0e6, |f| 1.0e-14 / f);
        assert_within(
            "flicker density over 200 k–2 MHz against 1e-14/f (ratio)",
            measured / model,
            0.9,
            1.1,
        );
    }

    /// Test 4: `noise_table` renders its interpolated table in time.
    ///
    /// The table falls linearly from 1e-18 A²/Hz at 1 kHz to 1e-20 A²/Hz at
    /// 1 MHz and is clamped above, so through the module's own 1 kΩ the node
    /// holds `1e6·T(f)` V²/Hz. Two bands two decades apart: 100–600 kHz on
    /// the slope and 5–20 MHz on the clamp. 2048-sample segments give 288
    /// periodograms over the bank, about 6% per bin; the narrower band holds
    /// 11 bins, so its mean's standard error is under 3% and 10% is over 3σ.
    #[test]
    fn a_noise_table_renders_its_interpolated_density() {
        let estimate = bank_estimate(
            "tabled",
            r#"
`include "disciplines.vams"
module tabled(p, n);
  inout p, n; electrical p, n;
  analog I(p, n) <+ V(p, n)/1000.0 + noise_table('{1e3, 1e-18, 1e6, 1e-20}, "t");
endmodule
"#,
            2048,
        );
        let table = |f: f64| {
            let clamped = f.clamp(1.0e3, 1.0e6);
            1.0e6 * (1.0e-18 + (clamped - 1.0e3) / (1.0e6 - 1.0e3) * (1.0e-20 - 1.0e-18))
        };
        for (low, high) in [(100.0e3, 600.0e3), (5.0e6, 20.0e6)] {
            let (measured, model) = band(&estimate, low, high, table);
            assert_within(
                &format!("table density over {low:e}–{high:e} Hz (ratio)"),
                measured / model,
                0.9,
                1.1,
            );
        }
    }

    /// The low-pass of test 6: `laplace_np(n, {1}, {-1e6, 0})` is
    /// `H(s) = 1/(1 + s/1e6)` (LRM 2.4 §4.5.13: a pole entry is the root of
    /// `1 − s/p`, so the stable pole is the negative one).
    const SHAPED: &str = r#"
`include "disciplines.vams"
module shaped(p, n);
  inout p, n; electrical p, n;
  analog I(p, n) <+ V(p, n)/1000.0 + laplace_np(white_noise(1e-18, "b"), '{1}, '{-1e6, 0});
endmodule
"#;

    /// The same analog body in a mixed Verilog-AMS host.
    const SHAPED_MIXED: &str = r#"
`include "disciplines.vams"
`timescale 1ns/1ps
module shaped_mixed(p, n, q);
  inout p, n; electrical p, n; output q; reg q;
  initial q = 1'b0; always #50 q = ~q;
  analog I(p, n) <+ V(p, n)/1000.0 + laplace_np(white_noise(1e-18, "b"), '{1}, '{-1e6, 0});
endmodule
"#;

    /// `|H(j2πf)|²·S·R²` against the bank's estimate over each band, with
    /// `injected(f) = |H(j2πf)|²·S` in A²/Hz and `R` the module's 1 kΩ.
    ///
    /// Every band ends at or below `fmax/4`, where the backward difference
    /// and the sampled record together move a pure power of `s` by at most
    /// `(θ/(2·sin(θ/2)))^±2 − 1 = 5.3%` at `θ = 2πf·NT = π/4`; the bands
    /// average over their bins, which keeps the discretization under 3%. The
    /// first two bins are left out: they carry the Hann window's leakage of
    /// each segment's own mean. 2048-sample segments give 288 periodograms
    /// over the bank, about 6% per bin, and every band holds at least 9
    /// bins, so its mean's standard error is under 2.5%: 10% is over 4σ.
    fn assert_shaped_density(
        label: &str,
        estimate: &[(f64, f64)],
        bands: &[(f64, f64)],
        injected: impl Fn(f64) -> f64,
    ) {
        let model = |f: f64| 1.0e6 * injected(f);
        for (low, high) in bands {
            let (measured, expected) = band(estimate, *low, *high, model);
            assert_within(
                &format!("{label} density over {low:e}–{high:e} Hz (ratio)"),
                measured / expected,
                0.9,
                1.1,
            );
        }
    }

    /// Test 6: noise the module shapes through `laplace_np` renders, with
    /// the density `|H|²·S` below `fmax/4`.
    ///
    /// `S = 1e-18` A²/Hz into 1 kΩ and `|H|² = 1/(1 + (2πf/1e6)²)`. The
    /// filter's 1 µs time constant is a hundredth of the 100 µs warm-up the
    /// 10 kHz `NOISEFMIN` sets, so the record opens stationary.
    #[test]
    fn a_laplace_shaped_process_renders_its_transfer() {
        let estimate = bank_estimate("shaped", SHAPED, 2048);
        assert_shaped_density(
            "laplace_np low-pass",
            &estimate,
            &[(90.0e3, 500.0e3), (1.0e6, 12.5e6)],
            |f| 1.0e-18 / (1.0 + (std::f64::consts::TAU * f / 1.0e6).powi(2)),
        );
    }

    /// An `idt`-shaped process is the integral of its noise:
    /// `|H|² = 1/(2πf)²`.
    ///
    /// The integral is a random walk, so each segment carries its own trend,
    /// whose Hann-windowed leakage falls as the fourth power of the bin
    /// index against the `1/f²` density; the bands start at bin 6, where it
    /// is under 0.1%.
    #[test]
    fn an_idt_shaped_process_renders_its_transfer() {
        let estimate = bank_estimate(
            "integrated",
            r#"
`include "disciplines.vams"
module integrated(p, n);
  inout p, n; electrical p, n;
  analog I(p, n) <+ V(p, n)/1000.0 + idt(white_noise(1e-18, "i"), 0.0);
endmodule
"#,
            2048,
        );
        assert_shaped_density(
            "idt",
            &estimate,
            &[(290.0e3, 1.0e6), (1.0e6, 12.5e6)],
            |f| 1.0e-18 / (std::f64::consts::TAU * f).powi(2),
        );
    }

    /// A `ddt`-shaped process is the derivative of its noise:
    /// `|H|² = (2πf)²`.
    #[test]
    fn a_ddt_shaped_process_renders_its_transfer() {
        let estimate = bank_estimate(
            "differentiated",
            r#"
`include "disciplines.vams"
module differentiated(p, n);
  inout p, n; electrical p, n;
  analog I(p, n) <+ V(p, n)/1000.0 + ddt(white_noise(1e-30, "d"));
endmodule
"#,
            2048,
        );
        assert_shaped_density("ddt", &estimate, &[(90.0e3, 1.0e6), (1.0e6, 12.5e6)], |f| {
            1.0e-30 * (std::f64::consts::TAU * f).powi(2)
        });
    }

    /// A shaped process in a mixed host renders through the same collector
    /// and filter as its analog-only twin: one instance name, one mechanism
    /// and one call give one seed, so the two runs inject the same samples.
    #[test]
    fn a_shaped_process_renders_identically_in_a_mixed_host() {
        let analog = ModelFile::new("shaped", SHAPED);
        let mixed = ModelFile::new("shaped_mixed", SHAPED_MIXED);
        let samples = |instance: &str, model: &ModelFile, module: &str| {
            let deck = format!(
                "* shaped noise route\n\
                 {instance}\n\
                 rq q 0 10k\n\
                 .va \"{}\" {module}\n\
                 .tran 1n 2u NOISEFMAX=500meg NOISESEED=7\n\
                 .end\n",
                model.deck_path()
            );
            let result = run(&deck, 2.0e-6, 1.0e-9);
            (
                window(&held_samples(&result, "p", NT), NT, 0.0, 2.0e-6),
                result
                    .digital_trace_named("q")
                    .map_or(0, |trace| trace.len()),
            )
        };
        let (runtime, _) = samples("x1 p 0 shaped", &analog, "shaped");
        let (host, toggles) = samples("x1 p 0 q shaped_mixed", &mixed, "shaped_mixed");
        assert!(
            toggles >= 40,
            "x1 must run as a mixed host: traced {toggles} points"
        );
        assert!(
            runtime.iter().any(|value| *value != 0.0),
            "the shaped process must inject"
        );
        for (index, (left, right)) in runtime.iter().zip(&host).enumerate() {
            assert!(
                (left - right).abs() <= 1.0e-9 * left.abs().max(right.abs()),
                "sample {index}: runtime {left:e}, mixed host {right:e}"
            );
        }
    }

    /// Run a deck expected to refuse, returning the typed refusal.
    fn refused(name: &str, source: &str, deck_body: &str) -> SimulationError {
        let model = ModelFile::new(name, source);
        let deck = format!(
            "* refused shaping\n\
             {deck_body}\n\
             .va \"{}\" {name}\n\
             .tran 1n 2u NOISEFMAX=500meg NOISESEED=7\n\
             .end\n",
            model.deck_path()
        );
        let netlist = Netlist::parse(&deck).expect("the deck parses");
        engine()
            .run_tran(&netlist, 2.0e-6, 1.0e-9)
            .expect_err("the shaping must be refused")
    }

    fn assert_shaping_refusal(error: &SimulationError, token: &str, module: &str) {
        let SimulationError::UnsupportedCapability(refusal) = error else {
            panic!("expected a capability refusal, got {error}");
        };
        assert_eq!(refusal.capability, token, "{error}");
        assert_eq!(error.descriptor().analysis_kind, Some(AnalysisKind::Tran));
        assert_eq!(error.instance_name(), Some("x1"));
        assert_eq!(refusal.module.as_deref(), Some(module));
    }

    /// Noise shaped through `absdelay` has no finite-state realization, and
    /// is refused by name with its instance and module.
    #[test]
    fn non_rational_shaping_is_a_typed_refusal() {
        let error = refused(
            "delayed",
            r#"
`include "disciplines.vams"
module delayed(p, n);
  inout p, n; electrical p, n;
  analog I(p, n) <+ V(p, n)/1000.0 + absdelay(white_noise(1e-18, "a"), 1n);
endmodule
"#,
            "x1 p 0 delayed",
        );
        assert_shaping_refusal(
            &error,
            "analysis.tran.noise.non_rational_shaping",
            "delayed",
        );
        assert!(error.to_string().contains("absdelay"), "{error}");
    }

    /// A filter whose input map moves with the bias cannot be continued from
    /// the state integrated under the old one, and is refused by name.
    #[test]
    fn bias_dependent_shaping_is_a_typed_refusal() {
        let error = refused(
            "driftshape",
            r#"
`include "disciplines.vams"
module driftshape(p, n, c);
  inout p, n, c; electrical p, n, c;
  analog I(p, n) <+ V(p, n)/1000.0 + idt(V(c)*white_noise(1e-18, "s"), 0.0);
endmodule
"#,
            "x1 p 0 c driftshape\nVc c 0 PWL(0 1 1u 2)",
        );
        assert_shaping_refusal(
            &error,
            "analysis.tran.noise.bias_dependent_shaping",
            "driftshape",
        );
        assert!(error.to_string().contains("t="), "{error}");
    }

    /// F8 holds for shaped noise: its filter state is not in a checkpoint.
    #[test]
    fn checkpointing_a_shaped_noise_run_is_a_typed_capability_refusal() {
        let model = ModelFile::new("shaped", SHAPED);
        let netlist = Netlist::parse(&format!(
            "* checkpointed shaped noise\n\
             x1 p 0 shaped\n\
             .va \"{}\" shaped\n\
             .tran 1n 100n NOISEFMAX=500meg NOISESEED=7\n\
             .end\n",
            model.deck_path()
        ))
        .expect("the deck parses");
        let validator = engine()
            .validate_transient_checkpoint_request(
                &netlist,
                100.0e-9,
                1.0e-9,
                TransientCheckpointStart::Fresh(TransientStartupMode::OperatingPoint),
                &[50.0e-9],
                &NoAbort,
            )
            .expect_err("a scheduled checkpoint of a shaped-noise run must be refused");
        assert_checkpoint_refusal(&validator, "streamed checkpoint validator");
        let run = engine()
            .run_tran_checkpointed(&netlist, 100.0e-9, 1.0e-9)
            .expect_err("a retained checkpoint of a shaped-noise run must be refused");
        assert_checkpoint_refusal(&run, "checkpointed transient run");
    }

    /// A flicker exponent is frozen at installation: the train realizes the
    /// slope it was drawn for, so a bias that moves the exponent refuses the
    /// run by name rather than injecting a spectrum the module no longer
    /// states.
    #[test]
    fn a_bias_dependent_flicker_exponent_is_a_typed_refusal() {
        let model = ModelFile::new(
            "driftflick",
            r#"
`include "disciplines.vams"
module driftflick(p, n, c);
  inout p, n, c; electrical p, n, c;
  analog I(p, n) <+ V(p, n)/1000.0 + flicker_noise(1e-20, 1.0 + V(c), "d");
endmodule
"#,
        );
        let deck = format!(
            "* bias-dependent flicker exponent\n\
             x1 p 0 c driftflick\n\
             Vc c 0 PWL(0 0 1u 0.5)\n\
             .va \"{}\" driftflick\n\
             .tran 1n 2u NOISEFMAX=500meg NOISESEED=7\n\
             .end\n",
            model.deck_path()
        );
        let netlist = Netlist::parse(&deck).expect("the deck parses");
        let error = engine()
            .run_tran(&netlist, 2.0e-6, 1.0e-9)
            .expect_err("a moving exponent must be refused");
        let SimulationError::UnsupportedCapability(refusal) = &error else {
            panic!("expected a capability refusal, got {error}");
        };
        assert_eq!(
            refusal.capability,
            "analysis.tran.noise.bias_dependent_spectrum"
        );
        assert_eq!(error.descriptor().analysis_kind, Some(AnalysisKind::Tran));
        assert_eq!(error.instance_name(), Some("x1"));
        assert_eq!(refusal.module.as_deref(), Some("driftflick"));
    }

    /// Test 7: a run is a function of its seed. Two identical runs are bit
    /// identical, and a run whose tight RELTOL rejects steps the default run
    /// accepts holds, at every one of its accepted times, the very sample the
    /// default run holds over the same interval: samples are functions of
    /// their index, and amplitudes move only at accepted points.
    ///
    /// The rejections are the capacitor's charge-truncation test — ngspice's
    /// LTE — at the pulse edges, which the run counts as timestep reductions.
    #[test]
    fn the_injected_samples_survive_rejected_steps() {
        assert_samples_survive_rejected_steps("noisy_va", NOISY_VA);
    }

    /// Test 7 for a shaped process: its filter state advances only at
    /// accepted points, so a rejected step restamps the sample the accepted
    /// one will see.
    #[test]
    fn shaped_samples_survive_rejected_steps() {
        assert_samples_survive_rejected_steps("shaped", SHAPED);
    }

    fn assert_samples_survive_rejected_steps(module: &str, source: &str) {
        let model = ModelFile::new(module, source);
        let deck = |options: &str| {
            format!(
                "* determinism across rejected steps\n\
                 {options}\n\
                 x1 p 0 {module}\n\
                 V1 in 0 PULSE(0 1 100n 1n 1n 200n 400n)\n\
                 R1 in c 1k\n\
                 C1 c 0 10p\n\
                 .va \"{}\" {module}\n\
                 .tran 1n 1u NOISEFMAX=500meg NOISESEED=7\n\
                 .end\n",
                model.deck_path()
            )
        };
        let run_counting = |options: &str| {
            let netlist = Netlist::parse(&deck(options)).expect("the deck parses");
            let engine = engine();
            let result = engine
                .run_tran(&netlist, 1.0e-6, 1.0e-9)
                .unwrap_or_else(|error| panic!("the deck must run: {error}"));
            (result, engine.convergence_quality().timestep_reductions)
        };

        let (first, default_rejections) = run_counting("");
        let (again, _) = run_counting("");
        assert_eq!(
            first.time, again.time,
            "the same seed replays the same grid"
        );
        assert_eq!(
            first.try_voltage_waveform_named("p"),
            again.try_voltage_waveform_named("p"),
            "the same seed replays the same samples bit for bit"
        );

        let (tight, rejections) = run_counting(".OPTIONS RELTOL=1e-7");
        println!("rejected steps: {default_rejections} at default RELTOL, {rejections} tight");
        assert!(
            rejections > default_rejections,
            "the tight run must reject steps the default run accepts"
        );
        let reference = held_samples(&first, "p", NT);
        let waveform = tight
            .try_voltage_waveform_named("p")
            .expect("p is retained");
        let mut compared = 0usize;
        for (time, value) in tight.time.iter().zip(waveform).skip(1) {
            // The hold interval `((j-1)·NT, j·NT]` holding this time.
            let boundary = (time / NT - 1.0e-9).ceil() as usize;
            let expected = reference[&boundary];
            assert!(
                (value - expected).abs() <= 1.0e-12 * expected.abs(),
                "at t={time:e} s the tight run holds {value:e}, the default run {expected:e}"
            );
            compared += 1;
        }
        assert!(
            compared > reference.len(),
            "the tight run must accept points the default run did not ({compared} against {})",
            reference.len()
        );
    }
}

// ---------------------------------------------------------------------------
// Generated devices
// ---------------------------------------------------------------------------

#[cfg(all(
    feature = "veriloga-model-diode-cmc",
    feature = "veriloga-builtins-noise"
))]
mod generated {
    use super::*;

    /// Test 5: a generated device's grouped shot noise reaches the transient.
    ///
    /// `DIODE_CMC`, given a 1 mm² junction so it conducts tens of µA, is
    /// forward biased by an ideal source and its cathode is held at ground by
    /// a 0 V sensing source, so both ends of its junction
    /// are AC grounds: every ampere of its shot-noise current flows through
    /// the sensing source, whatever the junction's own resistance and
    /// capacitance. The sensed current's variance is therefore the density
    /// `2qI` times the band `fmax`, the `2qI·R²·fmax` of a unit
    /// transresistance. The run is 16 µs rather than the suite's usual 2 µs:
    /// 15900 independent held samples make the variance estimate's relative
    /// standard error `sqrt(2/15900)` = 1.1%, so ±5% is a 4.5σ interval.
    #[test]
    fn generated_grouped_noise_reaches_the_transient() {
        const NT: f64 = 1.0e-9;
        const FMAX: f64 = 5.0e8;
        let netlist = Netlist::parse(
            "* generated grouped shot noise\n\
             V1 a 0 0.75\n\
             X1 a k DIODE_CMC AB=1e-6\n\
             Vsense k 0 0\n\
             .tran 1n 16u NOISEFMAX=500meg NOISESEED=7\n\
             .end\n",
        )
        .expect("the deck parses");
        let result = engine()
            .run_tran(&netlist, 16.0e-6, 1.0e-9)
            .unwrap_or_else(|error| panic!("the deck must run: {error}"));
        let current = result
            .try_branch_current_waveform_named("Vsense")
            .expect("the sensing source's current is retained");
        let bias = current[0].abs();
        assert!(bias > 1.0e-6, "the diode must conduct: {bias:e} A");
        let samples: Vec<f64> = result
            .time
            .iter()
            .zip(current)
            .filter(|(time, _)| {
                let boundary = (*time / NT).round();
                boundary > 100.0 && (*time - boundary * NT).abs() <= 1.0e-6 * NT
            })
            .map(|(_, value)| *value)
            .collect();
        assert!(
            samples.len() >= 15_800,
            "found {} held samples",
            samples.len()
        );
        let centre = samples.iter().sum::<f64>() / samples.len() as f64;
        let variance =
            samples.iter().map(|v| (v - centre).powi(2)).sum::<f64>() / samples.len() as f64;
        let charge = 1.602_176_634e-19;
        let expected = 2.0 * charge * bias * FMAX;
        let ratio = variance / expected;
        println!("σ²/(2qI·fmax) = {ratio} at I = {bias:e} A");
        assert!(
            (0.95..=1.05).contains(&ratio),
            "the sensed shot-noise variance is {ratio} of 2qI·fmax"
        );
    }
}

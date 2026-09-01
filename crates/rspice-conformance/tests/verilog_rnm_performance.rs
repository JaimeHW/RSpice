//! What the RNM representation of a reference block costs, against the analog
//! one, over the same simulated timespan.
//!
//! # Why this is a separate target
//!
//! Two reasons, and the second is the one that matters. It is timing-sensitive,
//! so it belongs in nightly rather than in the per-push tier where a loaded
//! runner would make it flap. And it is only meaningful in a release build: the
//! analog side is dominated by matrix work that `opt-level = 0` slows by roughly
//! an order of magnitude, and the RNM side by a compiler front end that it slows
//! by a different one — so a debug measurement is not a slower version of the
//! answer, it is a different ratio.
//!
//! `.github/workflows/nightly.yml` runs it with `--release --nocapture`. Run it
//! by hand the same way:
//!
//! ```text
//! cargo test -p rspice-conformance --test verilog_rnm_performance --release -- --nocapture
//! ```
//!
//! # What is measured
//!
//! Per block, best-of-[`REPEATS`] wall time for each representation, over the
//! same simulated timespan:
//!
//! * **analog** — `Netlist::parse` and `Engine::run_tran`, at
//!   [`SimulationConfig::default`] and a transient ceiling of a twentieth of the
//!   sample period. The settings a user gets without asking. Slowing this side
//!   down would inflate every ratio below, so it is fixed by the block
//!   definition and not by this file.
//! * **RNM** — one `run_digital_verilog`, which compiles the design and runs it.
//!
//! Best-of rather than mean: the quantity of interest is the cost of the work,
//! and a scheduler interruption can only ever add to it.
//!
//! # The compile share, and why it is separated
//!
//! `run_digital_verilog` compiles its source on every call, so a short run is
//! mostly front end. That is a true cost of the API as it stands and the
//! headline ratio charges the RNM for it. But P3's question — what does a
//! real-number model cost to *evaluate* — is about the other part, so the
//! measurement decomposes the two without any private API: the same design is
//! run again with ten times the vectors, and
//!
//! ```text
//!   marginal per vector = (t(10N) - t(N)) / (9N)
//!   execution for N     = marginal * N
//!   fixed cost          = t(N) - execution
//! ```
//!
//! The ten-times run covers ten times the timespan and is used for nothing but
//! this slope; the headline numbers are all from the block's own length.
//!
//! # The per-point columns, which are what the ratio actually is
//!
//! A ratio of wall times over the same timespan hides where it came from. The
//! two sides do not evaluate the same number of times: the analog engine takes
//! an accepted step every `max_step` or finer, and the RNM evaluates once per
//! vector. The table therefore also reports the cost of *one* evaluation on each
//! side, and that is the number that says what a real-number model is buying.

#![cfg(feature = "verilog-digital")]

use std::time::{Duration, Instant};

use rspice_conformance::suites::verilog::rnm::{self, RnmBlock};

/// Timed repetitions per measurement. Best-of, not mean.
const REPEATS: usize = 5;

/// Vector multiple used to separate the RNM's fixed cost from its marginal one.
const SLOPE_MULTIPLE: usize = 10;

/// P3's exit gate: the ratio the program is aiming at.
const TARGET_SPEEDUP: f64 = 100.0;

/// Where the recorded numbers came from, so a reader can tell whether the ones
/// they are looking at are comparable.
const BASELINE_MACHINE: &str = "recorded on Windows 11, x86-64, release profile, 2026-08-31";

/// One block's measurement.
struct Measurement {
    name: &'static str,
    timespan: f64,
    samples: usize,
    analog_points: usize,
    analog: Duration,
    rnm: Duration,
    rnm_execution: Duration,
}

impl Measurement {
    fn ratio(&self) -> f64 {
        self.analog.as_secs_f64() / self.rnm.as_secs_f64()
    }

    fn execution_ratio(&self) -> f64 {
        self.analog.as_secs_f64() / self.rnm_execution.as_secs_f64().max(f64::MIN_POSITIVE)
    }

    /// Microseconds per accepted transient point.
    fn analog_per_point_us(&self) -> f64 {
        self.analog.as_secs_f64() * 1e6 / self.analog_points as f64
    }

    /// Microseconds per RNM vector, compile cost excluded.
    fn rnm_per_vector_us(&self) -> f64 {
        self.rnm_execution.as_secs_f64() * 1e6 / self.samples as f64
    }
}

fn best_of(mut run: impl FnMut()) -> Duration {
    let mut best = Duration::MAX;
    for _ in 0..REPEATS {
        let start = Instant::now();
        run();
        best = best.min(start.elapsed());
    }
    best
}

/// The same block with its vector list repeated, for the slope measurement.
fn stretched(block: &RnmBlock, times: usize) -> RnmBlock {
    let mut stretched = block.clone();
    let original = block.stimulus.vectors.clone();
    stretched.stimulus.vectors = std::iter::repeat_n(original, times).flatten().collect();
    stretched
}

fn measure(block: &RnmBlock) -> Measurement {
    let analog_points = rnm::run_analog(block)
        .expect("the analog representation must run")
        .time
        .len();
    let analog = best_of(|| {
        rnm::run_analog(block).expect("the analog representation must run");
    });
    let rnm_wall = best_of(|| {
        rnm::run_rnm(block).expect("the RNM representation must run");
    });

    let long = stretched(block, SLOPE_MULTIPLE);
    let long_wall = best_of(|| {
        rnm::run_rnm(&long).expect("the stretched RNM run must run");
    });

    // The slope, clamped at zero: a stretched run that measured *faster* than
    // the short one is noise, and a negative marginal cost is not a number to
    // publish.
    let marginal = (long_wall.as_secs_f64() - rnm_wall.as_secs_f64())
        / ((SLOPE_MULTIPLE - 1) as f64)
        / block.samples() as f64;
    let execution = (marginal.max(0.0) * block.samples() as f64).min(rnm_wall.as_secs_f64());

    Measurement {
        name: block.name,
        timespan: block.timespan(),
        samples: block.samples(),
        analog_points,
        analog,
        rnm: rnm_wall,
        rnm_execution: Duration::from_secs_f64(execution),
    }
}

/// Measure every block and report the ratio against P3's target.
///
/// # The measurement
///
/// Release profile, best of five, same simulated timespan on both sides.
/// `BASELINE_MACHINE` says where. **P3's 100x exit gate is not met by these
/// blocks**, and the numbers say why rather than being rounded up:
///
/// ```text
/// block               samples  analog(ms)  rnm(ms)  ratio  exec(ms)  exec x  tran pts  us/pt  us/vec
/// r2r_dac                  16      2.3325   0.0973  23.97    0.0439   53.08       630  3.702   2.746
/// schmitt_hysteresis       16      3.4726   0.1080  32.15    0.0446   77.80       652  5.326   2.790
/// flash_quantizer          13      4.0616   0.1510  26.90    0.0568   71.51       508  7.995   4.369
/// ramp_integrator          32      2.3655   0.2101  11.26    0.1348   17.55      1268  1.866   4.212
/// ```
///
/// Two independent shortfalls, and they want different fixes:
///
/// 1. **The whole call includes a compile.** `run_digital_verilog` runs the
///    Verilog front end on every call, and at these run lengths that is most of
///    the RNM's time — compare the `rnm` and `exec` columns. A host that
///    compiled once and ran many stimuli would pay the `exec` column, which is
///    where the 55x–86x numbers come from.
/// 2. **These blocks are deliberately tiny.** The per-evaluation columns the
///    test prints show it plainly: the analog engine costs a few microseconds
///    per accepted point on a five-node network, and the RNM costs a comparable
///    few microseconds per vector. Almost none of the speedup is a cheaper
///    evaluation — it is *fewer* evaluations, because the analog side takes
///    twenty accepted steps per sample interval and the RNM takes one. That
///    ratio is set by `STEPS_PER_SAMPLE`, not by the abstraction.
///
/// So the shape of the result is: **RNM speedup scales with the analog circuit's
/// size, and these blocks have none to give.** `ramp_integrator` — three nodes,
/// linear, the smallest analog circuit here — has the worst ratio of the four,
/// which is exactly that prediction. Reaching 100x needs a block whose analog
/// representation is a real subcircuit, and reference blocks cannot be that and
/// stay hand-verifiable. Meeting the gate is a measurement to make against a
/// production-scale AMS block, not against these.
///
/// # What this asserts, and what it only reports
///
/// It asserts that the RNM representation is *faster* than the analog one on
/// every block, which is the claim the whole abstraction rests on: a real-number
/// model that cost more than the circuit it replaces would have no reason to
/// exist. It does not assert the 100x exit gate — a wall-clock ratio asserted in
/// CI is a test that fails when a runner is busy, and the honest thing to do
/// with a target a measurement does not meet is to print the measurement, not to
/// move the target or slow the other side down.
#[test]
fn rnm_costs_less_than_the_analog_representation_of_every_block() {
    let measurements: Vec<Measurement> = rnm::blocks().iter().map(measure).collect();

    println!("\nRNM against analog, same simulated timespan, best of {REPEATS}");
    println!("recorded baseline: {BASELINE_MACHINE}");
    println!(
        "{:<20} {:>8} {:>9} {:>11} {:>10} {:>7} {:>10} {:>8} {:>10} {:>12} {:>12}",
        "block",
        "samples",
        "span(us)",
        "analog(ms)",
        "rnm(ms)",
        "ratio",
        "exec(ms)",
        "exec x",
        "tran pts",
        "us/tran pt",
        "us/vector"
    );
    for measurement in &measurements {
        println!(
            "{:<20} {:>8} {:>9.3} {:>11.4} {:>10.4} {:>7.2} {:>10.4} {:>8.2} {:>10} {:>12.3} {:>12.3}",
            measurement.name,
            measurement.samples,
            measurement.timespan * 1e6,
            measurement.analog.as_secs_f64() * 1e3,
            measurement.rnm.as_secs_f64() * 1e3,
            measurement.ratio(),
            measurement.rnm_execution.as_secs_f64() * 1e3,
            measurement.execution_ratio(),
            measurement.analog_points,
            measurement.analog_per_point_us(),
            measurement.rnm_per_vector_us(),
        );
    }

    let slowest = measurements
        .iter()
        .map(Measurement::ratio)
        .fold(f64::INFINITY, f64::min);
    let slowest_execution = measurements
        .iter()
        .map(Measurement::execution_ratio)
        .fold(f64::INFINITY, f64::min);
    println!(
        "\nP3 exit gate is {TARGET_SPEEDUP:.0}x. Worst whole-call ratio {slowest:.2}x; worst \
         ratio against the RNM's execution alone {slowest_execution:.2}x."
    );
    if slowest < TARGET_SPEEDUP {
        println!(
            "The gate is NOT met on these blocks. Two causes, both visible above: \
             `run_digital_verilog` recompiles its source on every call, so the `exec` column is \
             what a host that compiled once would pay; and the `us/tran pt` and `us/vector` \
             columns are comparable, so the speedup is fewer evaluations rather than cheaper \
             ones. Reference blocks are small enough to verify by hand, which is the same thing \
             as being small enough to simulate quickly."
        );
    }

    for measurement in &measurements {
        assert!(
            measurement.ratio() > 1.0,
            "`{}` RNM took {:?} against the analog representation's {:?} over the same {} us — a \
             real-number model that costs more than the circuit it abstracts has no reason to \
             exist",
            measurement.name,
            measurement.rnm,
            measurement.analog,
            measurement.timespan * 1e6,
        );
    }
}

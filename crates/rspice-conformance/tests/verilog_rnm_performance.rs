//! What the RNM representation of a block costs, against the analog one, over
//! the same simulated timespan.
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
//! * **RNM, whole call** — one `run_digital_verilog`, which compiles the design
//!   and runs it.
//! * **RNM, compiled run** — one `CompiledDigitalDesign::run` against a design
//!   compiled once, outside the timing loop.
//! * **RNM, compile** — one `CompiledDigitalDesign::compile`, on its own.
//!
//! Best-of rather than mean: the quantity of interest is the cost of the work,
//! and a scheduler interruption can only ever add to it.
//!
//! # The two RNM columns are the point of the table
//!
//! Until the compile-once split landed there was one RNM number and it included
//! a compile, so a short run was mostly front end. The measurement separated the
//! two by a slope — the same design run again with ten times the vectors, so
//! that
//!
//! ```text
//!   marginal per vector = (t(10N) - t(N)) / (9N)
//!   execution for N     = marginal * N
//! ```
//!
//! — and reported the estimate as `exec`. That estimate is kept, beside the
//! directly measured `run`, for exactly one reason: the two are now independent
//! measurements of the same quantity, and their agreement is what says the split
//! did what it claimed rather than moving work somewhere the timer cannot see.
//! `compile + run` reconstructing `rnm` says the same thing from the other side.
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
    analog_nodes: usize,
    analog: Duration,
    /// One `run_digital_verilog`: compile and run.
    rnm: Duration,
    /// One `CompiledDigitalDesign::compile`, alone.
    rnm_compile: Duration,
    /// One `CompiledDigitalDesign::run` against an already-compiled design.
    rnm_run: Duration,
    /// The slope estimate of the same quantity as `rnm_run`, kept because two
    /// independent estimates agreeing is the evidence.
    rnm_execution: Duration,
}

impl Measurement {
    /// Analog against the whole RNM call, compile included.
    fn ratio(&self) -> f64 {
        self.analog.as_secs_f64() / self.rnm.as_secs_f64()
    }

    /// Analog against a run of an already-compiled design.
    fn run_ratio(&self) -> f64 {
        self.analog.as_secs_f64() / self.rnm_run.as_secs_f64().max(f64::MIN_POSITIVE)
    }

    /// Analog against the slope estimate of the execution.
    fn execution_ratio(&self) -> f64 {
        self.analog.as_secs_f64() / self.rnm_execution.as_secs_f64().max(f64::MIN_POSITIVE)
    }

    /// Microseconds per accepted transient point.
    fn analog_per_point_us(&self) -> f64 {
        self.analog.as_secs_f64() * 1e6 / self.analog_points as f64
    }

    /// Microseconds per RNM vector, measured on a compiled design.
    fn rnm_per_vector_us(&self) -> f64 {
        self.rnm_run.as_secs_f64() * 1e6 / self.samples as f64
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
    let analog_result = rnm::run_analog(block).expect("the analog representation must run");
    let analog_points = analog_result.time.len();
    let analog_nodes = analog_result.node_names.len();
    let analog = best_of(|| {
        rnm::run_analog(block).expect("the analog representation must run");
    });

    let rnm_wall = best_of(|| {
        rnm::run_rnm(block).expect("the RNM representation must run");
    });
    let compile_wall = best_of(|| {
        rnm::compile_rnm(block).expect("the RNM design must compile");
    });

    let compiled = rnm::compile_rnm(block).expect("the RNM design must compile");
    // The compiled route must be computing the same thing as the whole call, or
    // the ratio below is a ratio between two different computations.
    assert_eq!(
        rnm::run_compiled_rnm(&compiled, block).expect("the compiled design must run"),
        rnm::run_rnm(block).expect("the RNM representation must run"),
        "`{}` compiled run and whole call disagree",
        block.name
    );
    let run_wall = best_of(|| {
        rnm::run_compiled_rnm(&compiled, block).expect("the compiled design must run");
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
        analog_nodes,
        analog,
        rnm: rnm_wall,
        rnm_compile: compile_wall,
        rnm_run: run_wall,
        rnm_execution: Duration::from_secs_f64(execution),
    }
}

/// Every block this target measures: the five hand-verifiable reference blocks,
/// then the production-scale one.
///
/// The production block is last and separate because it is a different kind of
/// evidence. The five above it exist to be checkable by hand and are small for
/// that reason; it exists to be the size a user meets, and is measured against
/// the same target for that reason.
fn measured_blocks() -> Vec<RnmBlock> {
    let mut blocks = rnm::blocks();
    blocks.push(rnm::flash_adc_7bit());
    blocks
}

/// Measure every block and report the ratio against P3's target.
///
/// # The measurement
///
/// Release profile, best of five, same simulated timespan on both sides.
/// `BASELINE_MACHINE` says where.
///
/// ```text
/// block              samples  analog(ms)  rnm(ms)   ratio  compile(ms)  run(ms)   run x  nodes  tran pts  us/pt  us/vec
/// r2r_dac                 16      2.4600   0.1281   19.20       0.0484   0.0633   38.86      8       630   3.905   3.956
/// schmitt_hysteresis      16      3.7227   0.1609   23.14       0.0616   0.0862   43.19      4       652   5.710   5.387
/// flash_quantizer         13      4.4181   0.1746   25.30       0.0779   0.0845   52.29      9       508   8.697   6.500
/// ramp_integrator         32      2.5190   0.3343    7.54       0.0684   0.2525    9.98      1      1268   1.987   7.891
/// rc_lowpass              16      1.4261   0.1387   10.28       0.0515   0.0710   20.09      2       628   2.271   4.438
/// flash_adc_7bit         138    931.3399   2.8071  331.78       0.1800   2.5925  359.24    257      5508 169.089  18.786
/// ```
///
/// One run's numbers, not an average of several. A second run on the same
/// machine minutes later gave 359.82x and 386.24x on the last row, so read the
/// ratios as good to about a tenth — which is why the gate is reported rather
/// than asserted, and why nothing here is quoted to more precision than it has.
///
/// # What the ratio decomposes into
///
/// Exactly, and it is worth writing down because it says what each column is
/// for. Both sides cover the same timespan, so
///
/// ```text
///   ratio = (accepted points / vectors) * (us per point / us per vector)
/// ```
///
/// The first factor is about 40 on every row: it is `STEPS_PER_SAMPLE` and the
/// breakpoints the `PWL` corners force, and it is the same whatever the block
/// is. So the *whole* of the difference between a reference block's ratio and
/// this one's is the second factor — the cost of one evaluation on each side —
/// and that factor is 1.0 on `r2r_dac` and 9.0 on `flash_adc_7bit`.
///
/// # P3's exit gate, and the configuration that meets it
///
/// **Met, on `flash_adc_7bit`, on both RNM columns.** The whole call — front end
/// included, which is what a caller with one stimulus pays — is 331.78x cheaper
/// than the analog representation of the same converter over the same simulated
/// timespan. A run of an already-compiled design is 359.24x cheaper.
///
/// The configuration, stated so that the number can be reproduced or disputed:
///
/// * A 7-bit flash converter: a 128-rung reference string, 127 behavioural
///   comparators, a 128-resistor thermometer summer. 257 analog nodes, 129
///   voltage-source branch rows, 127 `tanh` evaluations per Newton iteration.
/// * The analog side at [`SimulationConfig::default`] — no tolerance is relaxed
///   and no accuracy is bought back — with the same `max_step` ceiling of a
///   twentieth of a sample period that every other block here uses. It is not
///   padded: every one of the 127 comparators is a code boundary the stimulus
///   crosses.
/// * The same 138-vector stimulus and the same 13.8 us of simulated time on both
///   sides.
/// * The two representations agreed to within a bound derived from the block's
///   physics before either was timed; `verilog_rnm_agreement` is that gate, and
///   the observed disagreement is seven orders under the bound.
///
/// # Why the reference blocks do not meet it and this one does
///
/// Both per-evaluation columns grew, and the ratio is what they grew *by*. The
/// RNM's cost per vector went from four to eight microseconds on the reference
/// blocks to nineteen on the converter — it did grow, because the real-number
/// model has ten real assignments where the small blocks have one to four. The
/// analog's cost per accepted point went from two to nine microseconds to a
/// hundred and sixty-nine, because it is set by the size of the matrix and the
/// number of nonlinear devices in it.
///
/// So a factor of three against a factor of thirty, and the difference between
/// them is the gate. That is the claim a real-number model makes, now measured
/// rather than asserted: **the cost of an RNM is set by how much arithmetic the
/// model writes, and the cost of what it replaces is set by how much circuit
/// there is — so the speedup is a statement about the circuit, not about the
/// digital host.** A reference block cannot supply that circuit and stay
/// hand-verifiable, which is why the five above are still at tens.
///
/// # What compiling once bought
///
/// Read `rnm` against `compile + run`. On the reference blocks the front end is
/// a third to a half of the whole call, so hoisting it roughly doubles the ratio
/// — `r2r_dac` goes from 19.20x to 38.86x, `rc_lowpass` from 10.28x to 20.09x.
/// On the converter it is six per cent of the call and moves the ratio from
/// 331.78x to 359.24x.
///
/// That ordering is itself a result, and it is the opposite way round from what
/// the earlier measurement predicted: compile cost tracks the size of the
/// *design*, and an RNM's design does not grow with the circuit it replaces. The
/// converter's model is ten lines whether it stands in for 257 nodes or for a
/// thousand. So the split matters most exactly where the ratio is smallest, and
/// the gate would have been met without it.
///
/// `exec` and `exec x` are the slope estimate this target used before the split
/// existed, kept beside the measured `run`. They agree to four per cent on the
/// converter and to within a fifth on the small blocks, which is what says the
/// split moved the compile out of the timed region rather than moving work
/// somewhere the timer cannot see.
///
/// # What this asserts, and what it only reports
///
/// It asserts that the RNM representation is *faster* than the analog one on
/// every block, on both the whole-call and the compiled-run column — the claim
/// the whole abstraction rests on, since a real-number model that cost more than
/// the circuit it replaces would have no reason to exist. It does not assert the
/// 100x gate even now that a block meets it: a wall-clock ratio asserted in CI is
/// a test that fails when a runner is busy. The gate is reported, and the
/// numbers above are what it was met by.
#[test]
fn rnm_costs_less_than_the_analog_representation_of_every_block() {
    let measurements: Vec<Measurement> = measured_blocks().iter().map(measure).collect();

    println!("\nRNM against analog, same simulated timespan, best of {REPEATS}");
    println!("recorded baseline: {BASELINE_MACHINE}");
    println!(
        "{:<20} {:>8} {:>9} {:>11} {:>10} {:>9} {:>11} {:>10} {:>9} {:>10} {:>8} {:>7} {:>9} {:>11} {:>11}",
        "block",
        "samples",
        "span(us)",
        "analog(ms)",
        "rnm(ms)",
        "ratio",
        "compile(ms)",
        "run(ms)",
        "run x",
        "exec(ms)",
        "exec x",
        "nodes",
        "tran pts",
        "us/tran pt",
        "us/vector"
    );
    for measurement in &measurements {
        println!(
            "{:<20} {:>8} {:>9.3} {:>11.4} {:>10.4} {:>9.2} {:>11.4} {:>10.4} {:>9.2} {:>10.4} {:>8.2} {:>7} {:>9} {:>11.3} {:>11.3}",
            measurement.name,
            measurement.samples,
            measurement.timespan * 1e6,
            measurement.analog.as_secs_f64() * 1e3,
            measurement.rnm.as_secs_f64() * 1e3,
            measurement.ratio(),
            measurement.rnm_compile.as_secs_f64() * 1e3,
            measurement.rnm_run.as_secs_f64() * 1e3,
            measurement.run_ratio(),
            measurement.rnm_execution.as_secs_f64() * 1e3,
            measurement.execution_ratio(),
            measurement.analog_nodes,
            measurement.analog_points,
            measurement.analog_per_point_us(),
            measurement.rnm_per_vector_us(),
        );
    }

    let best_whole_call = measurements
        .iter()
        .map(Measurement::ratio)
        .fold(0.0, f64::max);
    let best_compiled_run = measurements
        .iter()
        .map(Measurement::run_ratio)
        .fold(0.0, f64::max);
    println!(
        "\nP3 exit gate is {TARGET_SPEEDUP:.0}x. Best whole-call ratio {best_whole_call:.2}x; \
         best ratio against a compiled design's run {best_compiled_run:.2}x."
    );
    let meeting: Vec<&str> = measurements
        .iter()
        .filter(|measurement| measurement.ratio() >= TARGET_SPEEDUP)
        .map(|measurement| measurement.name)
        .collect();
    if meeting.is_empty() {
        println!(
            "The gate is NOT met by any block measured here. The per-evaluation columns say \
             where to look: if `us/tran pt` and `us/vector` are comparable, the speedup is \
             fewer evaluations rather than cheaper ones, and that ratio is set by the \
             transient ceiling rather than by the abstraction."
        );
    } else {
        println!(
            "The gate is met, on the whole call, by: {}. Read the last two columns for why: \
             both per-evaluation costs grow with the block, and the ratio is the factor \
             between how fast they grow.",
            meeting.join(", ")
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
        assert!(
            measurement.run_ratio() > 1.0,
            "`{}` compiled RNM run took {:?} against the analog representation's {:?}",
            measurement.name,
            measurement.rnm_run,
            measurement.analog,
        );
    }
}

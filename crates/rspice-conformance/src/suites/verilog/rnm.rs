//! Dual-representation reference blocks: the same circuit modelled twice.
//!
//! # Why this suite exists
//!
//! There is no open Verilog-AMS oracle. `verilog_ams` answers that with a
//! reference model per case — an independent computation of the trace in Rust —
//! and says plainly that this is weaker than two foreign simulators agreeing.
//! This suite answers it a second, independent way, and the two answers do not
//! share a failure mode.
//!
//! Each block here is authored twice:
//!
//! * **Analog.** A SPICE deck — resistor ladders, behavioural sources,
//!   capacitors — simulated by RSpice's transient engine. Nothing in it knows
//!   what the block is *for*; it is a network, and the answer falls out of an
//!   MNA solve.
//! * **RNM.** A `wreal` Verilog-AMS design run through
//!   [`run_digital_verilog`], where the block's behaviour is written as the
//!   closed form a real-number model would state it in.
//!
//! The two computations share no code below the harness: one integrates a
//! matrix through time, the other evaluates real expressions on an event
//! kernel. So when they agree to a bound derived from the block's own physics,
//! agreeing by accident would take two independent mistakes that happen to
//! cancel — and when they disagree, one of the two representations is wrong.
//!
//! # The RNM's ceiling today, stated rather than worked around
//!
//! **A real-valued variable cannot hold state across a clock edge.** The
//! canonical-IR digital lowering has no discrete-domain real *variable*:
//!
//! * `real state;` at module level lowers as an analog variable, and a process
//!   that writes it is refused — "`state` is not a discrete-domain signal;
//!   assigning a module-level analog variable from a process has no lowered
//!   form yet".
//! * `wreal state;` is a net, and IEEE 1364-2005 section 6.2 lets a procedural
//!   assignment write only a variable, so the semantic pass refuses that too.
//! * `$realtobits` / `$bitstoreal` — the LRM's own bridge, and the one the
//!   refusals above name — are "not supported yet inside a discrete-domain
//!   expression", so state cannot be parked in a `reg [63:0]` either.
//! * A `parameter real` is likewise not a discrete-domain signal, so every real
//!   constant below is written inline.
//!
//! What that rules out is the block this suite would otherwise open with: an RC
//! low-pass whose RNM is a discrete-time single-pole update, where the declared
//! tolerance would be the discretisation error at the sample rate. It cannot be
//! written today, and it is *not* written in a form that pretends otherwise.
//!
//! What is left is still four independent mechanisms, because state can live in
//! four-state `reg`s and reach the real domain as a *condition* — which is
//! exactly the bridge Verilog-AMS LRM 2.4 section 3.7 leaves open:
//!
//! | block | direction | mechanism |
//! |---|---|---|
//! | [`r2r_dac`] | bits → real | binary-weighted superposition |
//! | [`schmitt_hysteresis`] | real → real, with memory | threshold + one-bit state |
//! | [`flash_quantizer`] | real → bits → real | reference ladder + comparators |
//! | [`ramp_integrator`] | bit → real, accumulating | discrete integration |
//!
//! # Time alignment
//!
//! The digital host's tick is one nanosecond and its sample instants are exact:
//! vector `k` applies at `k * SAMPLE_PERIOD_NS` and is sampled at
//! `k * SAMPLE_PERIOD_NS + SETTLE_NS`. Every deck here is written in the same
//! nanosecond time base, so the two runs cover the *same* simulated timespan
//! rather than a rescaled one.
//!
//! The analog side is variable-step, so its accepted grid does not contain
//! those instants. [`TransientResult`] exposes no interpolating reader for an
//! arbitrary time — `output_projection` with `OUTPUTTIMEPOINTS` demands exact
//! accepted breakpoints, and `TransientResultCompressed::interpolate` is on the
//! compressed result, not this one — so **the harness interpolates linearly
//! between the two accepted points bracketing each sample instant**, in
//! [`interpolate`]. That is stated rather than hidden because it is a modelling
//! choice: on a waveform with curvature it costs `h^2 |y''| / 8`, which each
//! block's bound accounts for or argues is negligible.
//!
//! # How a tolerance is derived
//!
//! Never fitted to a run. Every bound is a sum of named terms, each computed
//! from a declared parameter, and the largest of them is always the engine's
//! own published accuracy promise: [`RELTOL`] relative on a node voltage plus
//! [`VNTOL`] absolute. A bound tightened to sit just above an observed error
//! would be a golden dressed as an analysis — it would pass because the
//! observation was copied into it, and it would fail on a machine whose
//! rounding differs.
//!
//! [`run_digital_verilog`]: rspice_core::xspice::verilog::run_digital_verilog
//! [`TransientResult`]: rspice_core::engine::TransientResult

use std::time::{Duration, Instant};

use rspice_core::constants::{RELTOL, VNTOL};
use rspice_core::engine::{Engine, SimulationConfig, TransientResult};
use rspice_core::netlist::Netlist;
use rspice_core::xspice::verilog::{
    DigitalClock, DigitalPort, DigitalRunReport, DigitalStimulus, run_digital_verilog,
};

// ===========================================================================
// The shared time base
// ===========================================================================

/// Time between successive vectors, in digital ticks (= nanoseconds).
pub const SAMPLE_PERIOD_NS: u64 = 100;
/// The same period in seconds, for the decks.
pub const SAMPLE_PERIOD_S: f64 = SAMPLE_PERIOD_NS as f64 * 1e-9;
/// Ticks after a vector is applied at which both sides are sampled.
pub const SETTLE_NS: u64 = 50;
/// The same settle in seconds.
pub const SETTLE_S: f64 = SETTLE_NS as f64 * 1e-9;

/// Transient ceiling, as a fraction of the sample period.
///
/// Twenty points per sample interval is a resolution a user would ask for when
/// they intend to look at the waveform, which is the setting this suite has to
/// use: the performance leg compares wall-clock against the RNM, and slowing
/// the analog side down with an accuracy nobody would choose would be measuring
/// the harness rather than the two representations.
const STEPS_PER_SAMPLE: f64 = 20.0;

/// Width of the transition a stepped `PWL` source is given.
///
/// A `PWL` needs two distinct times to change value. One picosecond is a
/// hundredth of the fastest time constant any block here has, so the ramp is
/// over long before a sample instant — but it is not free, and
/// [`ramp_integrator`] charges its bound for it rather than calling it zero.
const PWL_EDGE_S: f64 = 1e-12;

// ===========================================================================
// Bounds
// ===========================================================================

/// One agreement bound, kept as the terms it was derived from.
///
/// The terms survive into the failure report, because "0.006 V exceeded" says
/// nothing about which physical effect the bound was supposed to cover, and a
/// reader deciding whether the bound or the block is wrong needs that.
#[derive(Debug, Clone)]
pub struct Bound {
    /// Named contributions, in volts.
    pub terms: Vec<(&'static str, f64)>,
}

impl Bound {
    /// The engine's own accuracy promise on a node at `full_scale` volts.
    ///
    /// `RELTOL * |V| + VNTOL` is the criterion the Newton iteration is judged
    /// against, so it is the tightest bound that can be claimed *a priori*
    /// about any node voltage this engine reports. Every block starts here and
    /// adds its own physics.
    fn solver(full_scale: f64) -> (&'static str, f64) {
        (
            "solver convergence: RELTOL * full-scale + VNTOL",
            RELTOL * full_scale.abs() + VNTOL,
        )
    }

    fn new(terms: Vec<(&'static str, f64)>) -> Self {
        Self { terms }
    }

    /// The declared bound: the sum of the terms.
    pub fn total(&self) -> f64 {
        self.terms.iter().map(|(_, value)| value).sum()
    }

    /// The derivation, one term per line, for a failure report.
    pub fn derivation(&self) -> String {
        let mut text = String::new();
        for (name, value) in &self.terms {
            text.push_str(&format!("      {value:>12.6e}  {name}\n"));
        }
        text.push_str(&format!(
            "      {:>12.6e}  = declared bound\n",
            self.total()
        ));
        text
    }
}

// ===========================================================================
// A block
// ===========================================================================

/// One analog node compared against one RNM output port.
#[derive(Debug, Clone)]
pub struct SignalPair {
    /// Node name in the deck.
    pub analog_node: &'static str,
    /// Real-valued output port in the RNM design.
    pub rnm_port: &'static str,
    /// What the pair is, for a report.
    pub carries: &'static str,
    /// The declared agreement bound and its derivation.
    pub bound: Bound,
}

/// A reference block, authored twice.
#[derive(Debug, Clone)]
pub struct RnmBlock {
    /// Stable name, used in reports and in the required-block list.
    pub name: &'static str,
    /// What the block models, in one line.
    pub models: &'static str,
    /// Why both representations are honest ones for it.
    pub why_both_are_honest: &'static str,
    /// The analog deck.
    pub deck: String,
    /// The RNM design.
    pub design: String,
    /// The stimulus that drives the RNM.
    pub stimulus: DigitalStimulus,
    /// Signals compared between the two.
    pub pairs: Vec<SignalPair>,
}

impl RnmBlock {
    /// Number of vectors, which is also the number of compared sample points.
    pub fn samples(&self) -> usize {
        self.stimulus.vectors.len()
    }

    /// Simulated timespan, in seconds. The same on both sides by construction.
    pub fn timespan(&self) -> f64 {
        self.samples() as f64 * SAMPLE_PERIOD_S
    }

    /// The instants at which both representations are read.
    pub fn sample_times(&self) -> Vec<f64> {
        (0..self.samples())
            .map(|k| k as f64 * SAMPLE_PERIOD_S + SETTLE_S)
            .collect()
    }

    /// Transient ceiling this block is run with.
    pub fn max_step(&self) -> f64 {
        SAMPLE_PERIOD_S / STEPS_PER_SAMPLE
    }
}

// ===========================================================================
// Running each side
// ===========================================================================

/// Run the analog representation.
///
/// A default [`SimulationConfig`]: the settings a user gets without asking for
/// anything. The performance leg depends on that — an analog run slowed down by
/// tolerances nobody would choose would inflate the ratio it reports.
pub fn run_analog(block: &RnmBlock) -> Result<TransientResult, String> {
    let netlist = Netlist::parse(&block.deck)
        .map_err(|error| format!("`{}` analog deck does not parse: {error}", block.name))?;
    let engine = Engine::new(SimulationConfig::default());
    engine
        .run_tran(&netlist, block.timespan(), block.max_step())
        .map_err(|error| format!("`{}` analog transient failed: {error}", block.name))
}

/// Run the RNM representation.
pub fn run_rnm(block: &RnmBlock) -> Result<DigitalRunReport, String> {
    run_digital_verilog(&block.design, &block.stimulus)
        .map_err(|error| format!("`{}` RNM design does not run: {error}", block.name))
}

/// Linear interpolation of a variable-step waveform at an arbitrary time.
///
/// The harness's own, for the reason the module documentation gives: the public
/// [`TransientResult`] has no reader that will interpolate at a time that is not
/// an accepted breakpoint. Outside the accepted window the endpoint is held,
/// which cannot happen here — every sample instant is strictly inside `[0,
/// tstop)` — but is defined rather than left to an index panic.
///
/// [`TransientResult`]: rspice_core::engine::TransientResult
pub fn interpolate(times: &[f64], values: &[f64], target: f64) -> f64 {
    if times.is_empty() {
        return f64::NAN;
    }
    if target <= times[0] {
        return values[0];
    }
    let last = times.len() - 1;
    if target >= times[last] {
        return values[last];
    }
    let index = times.partition_point(|time| *time < target);
    let (t0, t1) = (times[index - 1], times[index]);
    let (v0, v1) = (values[index - 1], values[index]);
    if t1 <= t0 {
        return v1;
    }
    v0 + (v1 - v0) * (target - t0) / (t1 - t0)
}

// ===========================================================================
// Comparison
// ===========================================================================

/// One compared point, in the shape a plot wants.
#[derive(Debug, Clone, Copy)]
pub struct Point {
    /// Sample instant, seconds.
    pub time: f64,
    /// Vector index.
    pub step: usize,
    /// Analog value, interpolated onto `time`.
    pub analog: f64,
    /// RNM value at `time`.
    pub rnm: f64,
    /// `analog - rnm`.
    pub error: f64,
    /// The declared bound `|error|` is held to.
    pub bound: f64,
}

impl Point {
    /// Whether this point is within its bound.
    pub fn agrees(&self) -> bool {
        self.error.abs() <= self.bound
    }
}

/// One signal pair's comparison across every sample.
#[derive(Debug, Clone)]
pub struct PairAgreement {
    /// The pair compared.
    pub pair: SignalPair,
    /// Every sample, in order.
    pub points: Vec<Point>,
}

impl PairAgreement {
    /// The largest `|error|` observed.
    pub fn max_error(&self) -> f64 {
        self.points
            .iter()
            .map(|point| point.error.abs())
            .fold(0.0, f64::max)
    }

    /// Points that exceeded the bound.
    pub fn violations(&self) -> Vec<&Point> {
        self.points.iter().filter(|point| !point.agrees()).collect()
    }

    /// The whole trace as a table, for a failing assertion.
    pub fn table(&self) -> String {
        let mut text = format!(
            "    {:>4}  {:>13}  {:>16}  {:>16}  {:>13}  {:>13}  {}\n",
            "step", "time(s)", "analog(V)", "rnm(V)", "error(V)", "bound(V)", "ok"
        );
        for point in &self.points {
            text.push_str(&format!(
                "    {:>4}  {:>13.6e}  {:>16.9e}  {:>16.9e}  {:>13.6e}  {:>13.6e}  {}\n",
                point.step,
                point.time,
                point.analog,
                point.rnm,
                point.error,
                point.bound,
                if point.agrees() { "" } else { "EXCEEDED" }
            ));
        }
        text
    }
}

/// A whole block's comparison, plus what each side cost.
#[derive(Debug, Clone)]
pub struct BlockAgreement {
    /// Block name.
    pub name: &'static str,
    /// One entry per signal pair.
    pub pairs: Vec<PairAgreement>,
    /// Wall time of the analog representation, parse and solve.
    pub analog_wall: Duration,
    /// Wall time of the RNM representation, compile and run.
    pub rnm_wall: Duration,
}

impl BlockAgreement {
    /// Whether every pair stayed within its bound.
    pub fn agrees(&self) -> bool {
        self.pairs.iter().all(|pair| pair.violations().is_empty())
    }

    /// A report naming every disagreement, with the bound's derivation beside
    /// it and the whole trace under it.
    pub fn failure_report(&self) -> String {
        let mut text = String::new();
        for pair in &self.pairs {
            let violations = pair.violations();
            if violations.is_empty() {
                continue;
            }
            text.push_str(&format!(
                "\n  `{}`: analog node `{}` vs RNM port `{}` ({})\n",
                self.name, pair.pair.analog_node, pair.pair.rnm_port, pair.pair.carries
            ));
            text.push_str(&format!(
                "    {} of {} samples exceeded the bound; worst |error| = {:.6e} V\n",
                violations.len(),
                pair.points.len(),
                pair.max_error()
            ));
            text.push_str("    the bound, as derived:\n");
            text.push_str(&pair.pair.bound.derivation());
            text.push_str(&pair.table());
        }
        text
    }
}

/// Run both representations of one block and compare them.
pub fn compare(block: &RnmBlock) -> Result<BlockAgreement, String> {
    let analog_start = Instant::now();
    let analog = run_analog(block)?;
    let analog_wall = analog_start.elapsed();

    let rnm_start = Instant::now();
    let report = run_rnm(block)?;
    let rnm_wall = rnm_start.elapsed();

    if report.observations.len() != block.samples() {
        return Err(format!(
            "`{}` RNM produced {} observation(s) for {} vector(s)",
            block.name,
            report.observations.len(),
            block.samples()
        ));
    }

    let times = block.sample_times();
    let mut pairs = Vec::with_capacity(block.pairs.len());
    for pair in &block.pairs {
        let waveform = analog
            .try_voltage_waveform_named(pair.analog_node)
            .ok_or_else(|| {
                format!(
                    "`{}` deck has no node `{}`; it has {:?}",
                    block.name, pair.analog_node, analog.node_names
                )
            })?;
        let bound = pair.bound.total();
        let mut points = Vec::with_capacity(times.len());
        for (step, &time) in times.iter().enumerate() {
            let rnm = rnm_value(&report, step, pair.rnm_port).ok_or_else(|| {
                format!(
                    "`{}` RNM observation {step} has no port `{}`",
                    block.name, pair.rnm_port
                )
            })?;
            let value = interpolate(&analog.time, waveform, time);
            points.push(Point {
                time,
                step,
                analog: value,
                rnm,
                error: value - rnm,
                bound,
            });
        }
        pairs.push(PairAgreement {
            pair: pair.clone(),
            points,
        });
    }

    Ok(BlockAgreement {
        name: block.name,
        pairs,
        analog_wall,
        rnm_wall,
    })
}

/// Read one real-valued port out of one observation.
///
/// The trace renders a real in Rust's shortest round-tripping form, so parsing
/// it back is exact — no tolerance is spent on the rendering.
fn rnm_value(report: &DigitalRunReport, step: usize, port: &str) -> Option<f64> {
    report
        .observations
        .get(step)?
        .values
        .iter()
        .find(|(name, _)| name == port)
        .and_then(|(_, value)| value.parse().ok())
}

// ===========================================================================
// Stimulus and deck helpers
// ===========================================================================

/// A `PWL` body holding `levels[k]` over `[k * Ts, (k + 1) * Ts)`.
///
/// The transition is placed at the vector's *apply* instant, which is where the
/// digital host applies the same vector, so the two sides change together.
fn pwl_stepping_at_apply(levels: &[f64]) -> String {
    pwl_stepping(levels, 0.0)
}

/// A `PWL` body holding `levels[k]` over `(t_{k-1}, t_k]`, where `t_k` is the
/// sample instant.
///
/// Used where what matters is the value *between* two samples rather than at
/// one: [`ramp_integrator`]'s current is integrated over the interval, so the
/// interval is what the schedule has to name.
fn pwl_stepping_at_samples(levels: &[f64]) -> String {
    // One period earlier than the apply-time form, then shifted by the settle:
    // the transition into `levels[k]` lands on `t_{k-1}`.
    let mut text = format!("0 {:?}", levels[0]);
    for (index, level) in levels.iter().enumerate().skip(1) {
        let edge = (index - 1) as f64 * SAMPLE_PERIOD_S + SETTLE_S;
        text.push_str(&format!(
            " {:?} {:?} {edge:?} {level:?}",
            edge - PWL_EDGE_S,
            levels[index - 1]
        ));
    }
    text
}

fn pwl_stepping(levels: &[f64], offset: f64) -> String {
    let mut text = format!("0 {:?}", levels[0]);
    for (index, level) in levels.iter().enumerate().skip(1) {
        let edge = index as f64 * SAMPLE_PERIOD_S + offset;
        text.push_str(&format!(
            " {:?} {:?} {edge:?} {level:?}",
            edge - PWL_EDGE_S,
            levels[index - 1]
        ));
    }
    text
}

/// A stimulus whose only input is a real port driven by `levels`.
fn real_input_stimulus(
    module: &'static str,
    input: &'static str,
    outputs: &[&'static str],
    levels: &[f64],
    clocked: bool,
) -> DigitalStimulus {
    DigitalStimulus {
        module: Some(module.to_string()),
        inputs: vec![DigitalPort {
            name: input.to_string(),
            width: 0,
        }],
        outputs: outputs
            .iter()
            .map(|name| DigitalPort {
                name: (*name).to_string(),
                width: 0,
            })
            .collect(),
        clock: clocked.then(|| DigitalClock {
            port: "clk".to_string(),
            half_period: SETTLE_NS,
        }),
        step: SAMPLE_PERIOD_NS,
        settle: SETTLE_NS,
        vectors: levels
            .iter()
            .map(|level| vec![format!("{level:?}")])
            .collect(),
    }
}

// ===========================================================================
// Block 1 — the R-2R ladder
// ===========================================================================

/// Full-scale reference of the DAC and of the quantizer, volts.
const VREF: f64 = 5.0;
/// Ladder rung resistance, ohms. The arms are twice this, which is the ladder.
const LADDER_R: f64 = 1.0e3;
/// Load on the DAC's output node, farads.
const DAC_LOAD_C: f64 = 1.0e-12;
/// Bits in the DAC.
const DAC_BITS: u32 = 4;

/// A 4-bit voltage-mode R-2R ladder against a binary-weighted sum.
///
/// # What it models
///
/// Digital-to-analog conversion: a four-bit code chosen by the stimulus, read
/// out as a voltage. The stimulus walks all sixteen codes, so every rung is
/// exercised alone and in company.
///
/// # Why both representations are honest
///
/// The analog side is the ladder itself — eight resistors and a termination —
/// and it never computes `code / 16`. The output voltage is whatever the MNA
/// solve of a five-node network makes it, so a rung wired to the wrong node, or
/// an arm that is `R` where it should be `2R`, changes the answer. The RNM side
/// is the closed form a real-number model of a DAC states: the reference times
/// the code over full scale. Mis-weighting one bit moves the output by at least
/// one LSB — 312.5 mV — which is sixty-two times the bound below.
///
/// # The bound, derived
///
/// * **Settling.** The output node's Thevenin resistance is `R` (the `2R` arm in
///   parallel with `R` plus the rest of the ladder, which is itself `R`), so
///   `tau = R * C = 1 ns`. The code changes at the vector's apply instant and
///   both sides are read `SETTLE_NS = 50 ns` later: `exp(-50) = 1.9e-22` of a
///   full-scale swing. Below the smallest denormal that matters here, and
///   carried anyway so the reader can see it was considered.
/// * **`gmin`.** The convergence policy's floor is `gmin_target = 1e-15 S`, and
///   a shunt of that across a `1 kΩ` Thevenin source perturbs the divider by
///   `gmin * R = 1e-12` relative — `5e-12 V` at full scale.
/// * **Interpolation.** The waveform is flat at every sample instant (settled
///   fifty time constants), so the linear interpolation between accepted points
///   has no curvature to miss.
/// * **Solver.** `RELTOL * VREF + VNTOL`, and this term is four orders larger
///   than the rest put together. It is the engine's promise about any node
///   voltage; a bound below it would be asserting an accuracy the engine does
///   not offer.
pub fn r2r_dac() -> RnmBlock {
    let codes: Vec<u32> = (0..1 << DAC_BITS).collect();
    let mut deck = String::from("r-2r ladder dac, four bits\n");
    for bit in (0..DAC_BITS).rev() {
        let levels: Vec<f64> = codes
            .iter()
            .map(|code| if code >> bit & 1 == 1 { VREF } else { 0.0 })
            .collect();
        deck.push_str(&format!(
            "VB{bit} b{bit} 0 PWL({})\n",
            pwl_stepping_at_apply(&levels)
        ));
        deck.push_str(&format!("RARM{bit} b{bit} n{bit} {:?}\n", 2.0 * LADDER_R));
    }
    deck.push_str(&format!("RTERM n0 0 {:?}\n", 2.0 * LADDER_R));
    for rung in 0..DAC_BITS - 1 {
        deck.push_str(&format!("RRUNG{rung} n{rung} n{} {LADDER_R:?}\n", rung + 1));
    }
    deck.push_str(&format!("CLOAD n{} 0 {DAC_LOAD_C:?}\n", DAC_BITS - 1));
    deck.push_str(".end\n");

    let full_scale = f64::from(1u32 << DAC_BITS);
    let design = format!(
        "\
module rnm_r2r_dac(code, vout);
  input [{msb}:0] code;
  output wreal vout;
  wire [{msb}:0] code;
  assign vout = ({VREF:?} / {full_scale:?}) *
      ((code[3] ? 8.0 : 0.0) + (code[2] ? 4.0 : 0.0)
     + (code[1] ? 2.0 : 0.0) + (code[0] ? 1.0 : 0.0));
endmodule
",
        msb = DAC_BITS - 1,
    );

    let tau = LADDER_R * DAC_LOAD_C;
    let bound = Bound::new(vec![
        (
            "settling: exp(-settle / (R * C)) * VREF",
            (-SETTLE_S / tau).exp() * VREF,
        ),
        (
            "gmin shunt: gmin_target * R * VREF",
            1e-15 * LADDER_R * VREF,
        ),
        Bound::solver(VREF),
    ]);

    RnmBlock {
        name: "r2r_dac",
        models: "a 4-bit voltage-mode R-2R ladder converting a code to a voltage",
        why_both_are_honest: "the deck is the ladder and solves for its output; the RNM states the \
             binary-weighted closed form, and neither derives the other",
        deck,
        design,
        stimulus: DigitalStimulus {
            module: Some("rnm_r2r_dac".to_string()),
            inputs: vec![DigitalPort {
                name: "code".to_string(),
                width: DAC_BITS,
            }],
            outputs: vec![DigitalPort {
                name: "vout".to_string(),
                width: 0,
            }],
            clock: None,
            step: SAMPLE_PERIOD_NS,
            settle: SETTLE_NS,
            vectors: codes
                .iter()
                .map(|code| vec![format!("{code:04b}")])
                .collect(),
        },
        pairs: vec![SignalPair {
            analog_node: "n3",
            rnm_port: "vout",
            carries: "the converted output voltage",
            bound,
        }],
    }
}

// ===========================================================================
// Block 2 — the Schmitt trigger
// ===========================================================================

/// Comparator supply, volts. Both blocks that use a comparator share it.
const VDD: f64 = 5.0;
/// The comparator's own reference, volts.
const VCOMP_REF: f64 = 2.5;
/// Gain of the behavioural limiter, per volt.
///
/// A `tanh` reaches a rail to within `2 exp(-2 g d)` at an overdrive `d`, so
/// this and the stimulus's guard band together decide how ideal the rail is.
const COMPARATOR_GAIN: f64 = 1.0e3;
/// Input resistor of the Schmitt's summing node, ohms.
const SCHMITT_R_IN: f64 = 1.0e4;
/// Feedback resistor, ohms. Its ratio to `SCHMITT_R_IN` is the hysteresis.
const SCHMITT_R_FB: f64 = 4.0e4;
/// Output series resistance, ohms.
const SCHMITT_R_OUT: f64 = 10.0;
/// Output capacitance, farads.
const SCHMITT_C_OUT: f64 = 1.0e-10;
/// Closest the stimulus comes to either threshold, volts.
const SCHMITT_GUARD_V: f64 = 12.5e-3;

/// A non-inverting Schmitt trigger against a one-bit-state real compare.
///
/// # What it models
///
/// A comparator with hysteresis: the block whose output depends on where the
/// input has *been*, not only where it is. This is the only block here with
/// memory, which is what makes it load-bearing — the other three would pass
/// unchanged if the RNM forgot everything between samples.
///
/// # Why both representations are honest
///
/// The analog side has no thresholds written in it. It has a limiting
/// high-gain stage and a resistor from its output back to its summing node, and
/// the two trip points *emerge* from `K = R_in / R_fb`:
///
/// ```text
///   V(p) = (V(in) / R_in + V(out) / R_fb) / (1 / R_in + 1 / R_fb)
///   trip when V(p) = VCOMP_REF, so
///     rising  threshold = VCOMP_REF * (1 + K)          = 3.125 V
///     falling threshold = VCOMP_REF * (1 + K) - VDD*K  = 1.875 V
/// ```
///
/// The RNM states those two closed forms and a one-bit `reg` choosing between
/// them. So the analog side discovers the hysteresis by solving a feedback
/// network and the RNM asserts it; if the feedback resistor were wrong, or the
/// loop wired to the wrong node, the two would part company at a trip point.
///
/// # What this block resolves
///
/// The stimulus is a staircase that approaches each threshold to within
/// `SCHMITT_GUARD_V = 12.5 mV` from both sides. A threshold error larger than
/// that moves a transition to a different vector, and the two representations
/// then differ by the full `VDD` — a thousand times the bound. A threshold
/// error *smaller* than the guard band is invisible to this block, and that is
/// the honest statement of its resolution: 0.4% of the rising threshold.
///
/// # The bound, derived
///
/// * **Feedback loading.** The RNM says the output is a rail. The analog output
///   is not quite: the feedback resistor pulls it toward `V(p)` through the
///   output series resistance, by
///   `|V(p) - V(amp)| * R_out / (R_fb + R_out) <= VDD * R_out / (R_fb + R_out)`
///   = 1.25 mV. This is the term that exists because the circuit is a circuit.
/// * **Finite gain.** At the guard band the summing node is
///   `SCHMITT_GUARD_V * R_fb / (R_in + R_fb) = 10 mV` from the comparator
///   reference, and the limiter is within `VDD * exp(-2 * g * d)` of its rail —
///   about `2e-8 V`.
/// * **Settling.** `tau = R_out * C_out = 1 ns`; read 50 ns after the step.
/// * **Interpolation.** Flat at every sample instant, as above.
/// * **Solver.** `RELTOL * VDD + VNTOL`, again the largest term.
pub fn schmitt_hysteresis() -> RnmBlock {
    let ratio = SCHMITT_R_IN / SCHMITT_R_FB;
    let rising = VCOMP_REF * (1.0 + ratio);
    let falling = rising - VDD * ratio;

    // Up through the rising threshold, then back down through the falling one,
    // closing to the guard band on both sides of each. The coarse levels in
    // between are there so the block is exercised well away from a trip point
    // too, where a representation that had simply latched would still pass.
    let levels = vec![
        0.0,
        1.0,
        2.0,
        3.0,
        rising - SCHMITT_GUARD_V,
        rising + SCHMITT_GUARD_V,
        3.5,
        4.5,
        3.0,
        2.5,
        2.0,
        falling + SCHMITT_GUARD_V,
        falling - SCHMITT_GUARD_V,
        1.5,
        1.0,
        0.0,
    ];

    let deck = format!(
        "schmitt trigger with resistive positive feedback\n\
         VIN vin 0 PWL({levels})\n\
         RIN vin p {SCHMITT_R_IN:?}\n\
         RFB out p {SCHMITT_R_FB:?}\n\
         BAMP amp 0 V={{{VCOMP_REF:?} + {VCOMP_REF:?}*tanh({COMPARATOR_GAIN:?}*(V(p)-{VCOMP_REF:?}))}}\n\
         ROUT amp out {SCHMITT_R_OUT:?}\n\
         COUT out 0 {SCHMITT_C_OUT:?}\n\
         .end\n",
        levels = pwl_stepping_at_apply(&levels),
    );

    let design = format!(
        "\
module rnm_schmitt(clk, vin, vout);
  input clk;
  input wreal vin;
  output wreal vout;
  reg state;
  initial state = 1'b0;
  always @(posedge clk)
    if (state) state = (vin > ({VCOMP_REF:?} * (1.0 + {ratio:?}) - {VDD:?} * {ratio:?})) ? 1'b1 : 1'b0;
    else       state = (vin > ({VCOMP_REF:?} * (1.0 + {ratio:?}))) ? 1'b1 : 1'b0;
  assign vout = state ? {VDD:?} : 0.0;
endmodule
"
    );

    let overdrive = SCHMITT_GUARD_V * SCHMITT_R_FB / (SCHMITT_R_IN + SCHMITT_R_FB);
    let tau = SCHMITT_R_OUT * SCHMITT_C_OUT;
    let bound = Bound::new(vec![
        (
            "feedback loading: VDD * R_out / (R_fb + R_out)",
            VDD * SCHMITT_R_OUT / (SCHMITT_R_FB + SCHMITT_R_OUT),
        ),
        (
            "finite gain at the guard band: VDD * exp(-2 * gain * overdrive)",
            VDD * (-2.0 * COMPARATOR_GAIN * overdrive).exp(),
        ),
        (
            "settling: exp(-settle / (R_out * C_out)) * VDD",
            (-SETTLE_S / tau).exp() * VDD,
        ),
        Bound::solver(VDD),
    ]);

    RnmBlock {
        name: "schmitt_hysteresis",
        models: "a comparator with hysteresis, whose output depends on where the input has been",
        why_both_are_honest: "the deck's trip points emerge from a feedback resistor ratio it never names; \
             the RNM names them as closed forms and keeps one bit of state",
        deck,
        design,
        stimulus: real_input_stimulus("rnm_schmitt", "vin", &["vout"], &levels, true),
        pairs: vec![SignalPair {
            analog_node: "out",
            rnm_port: "vout",
            carries: "the comparator output rail",
            bound,
        }],
    }
}

// ===========================================================================
// Block 3 — the flash quantizer
// ===========================================================================

/// Comparators in the flash quantizer, and so the rungs in its reference string
/// minus one.
const FLASH_COMPARATORS: usize = 3;
/// Resistance of one rung of the reference string, ohms.
const FLASH_STRING_R: f64 = 1.0e3;
/// Closest the stimulus comes to any tap, volts.
const FLASH_GUARD_V: f64 = 12.5e-3;

/// A resistor-string flash quantizer against three real comparisons.
///
/// # What it models
///
/// Analog-to-digital conversion and back: a real input against three thresholds
/// generated by a reference ladder, giving a thermometer code, summed back into
/// a reconstructed voltage. It runs the [`r2r_dac`] direction backwards and then
/// forwards again, so a sign or an ordering error in either domain shows.
///
/// # Why both representations are honest
///
/// The analog thresholds are *node voltages* on a four-rung string — the
/// comparators are written against `V(t1)`, `V(t2)`, `V(t3)`, never against a
/// number — so the taps are whatever the string divides `VREF` into. The RNM
/// writes `VREF * k / 4` for each. A rung of the wrong value, or a comparator
/// wired to the wrong tap, separates them.
///
/// The reconstruction is a resistive summer of the three comparator outputs
/// against the RNM's sum of three conditionals, which is the [`r2r_dac`] check
/// in a different topology: equal weights rather than binary ones.
///
/// # What this block resolves
///
/// The stimulus closes to `FLASH_GUARD_V = 12.5 mV` on both sides of all three
/// taps. A tap error larger than that flips a comparator on a different vector
/// and moves the reconstruction by a full `VREF / 4 = 1.25 V`.
///
/// # The bound, derived
///
/// * **Ladder loading.** None to charge for, and that is a fact about the deck
///   rather than an approximation: a `V(node)` reference inside a behavioural
///   source draws no current, so the string's taps are its unloaded divisions
///   and the summer does not load the comparators either.
/// * **Finite gain.** As in [`schmitt_hysteresis`], at `FLASH_GUARD_V` of
///   overdrive — here undivided, since the comparator sees the input directly —
///   `VDD * exp(-2 * gain * guard)`.
/// * **Interpolation.** The comparators are ideal sources with no reactance, so
///   the node is at its final value the instant the input settles.
/// * **Solver.** `RELTOL * full-scale + VNTOL`, per node: `VDD` for a
///   comparator rail and `3 * VREF / 4` for the reconstruction.
pub fn flash_quantizer() -> RnmBlock {
    let taps: Vec<f64> = (1..=FLASH_COMPARATORS)
        .map(|index| VREF * index as f64 / (FLASH_COMPARATORS + 1) as f64)
        .collect();

    // Coarse levels, plus a pair straddling every tap at the guard band.
    let mut levels = vec![0.25];
    for tap in &taps {
        levels.push(tap - FLASH_GUARD_V);
        levels.push(tap + FLASH_GUARD_V);
        levels.push(tap + 0.4);
    }
    levels.extend([4.5, 2.0, 0.25]);

    let mut deck = String::from("flash quantizer with a resistor-string reference\n");
    deck.push_str(&format!("VREFS ref 0 {VREF:?}\n"));
    deck.push_str(&format!(
        "RSTR{FLASH_COMPARATORS} ref t{FLASH_COMPARATORS} {FLASH_STRING_R:?}\n"
    ));
    for rung in (1..FLASH_COMPARATORS).rev() {
        deck.push_str(&format!(
            "RSTR{rung} t{} t{rung} {FLASH_STRING_R:?}\n",
            rung + 1
        ));
    }
    deck.push_str(&format!("RSTR0 t1 0 {FLASH_STRING_R:?}\n"));
    deck.push_str(&format!(
        "VIN vin 0 PWL({})\n",
        pwl_stepping_at_apply(&levels)
    ));
    for index in 1..=FLASH_COMPARATORS {
        deck.push_str(&format!(
            "BC{index} c{index} 0 V={{{half:?} + {half:?}*tanh({COMPARATOR_GAIN:?}*(V(vin)-V(t{index})))}}\n",
            half = VDD / 2.0,
        ));
        deck.push_str(&format!("RSUM{index} c{index} recon {FLASH_STRING_R:?}\n"));
    }
    deck.push_str(&format!("RSUMT recon 0 {FLASH_STRING_R:?}\n"));
    deck.push_str(".end\n");

    let quarter = VREF / (FLASH_COMPARATORS + 1) as f64;
    let mut design = String::from(
        "\
module rnm_flash(vin, q1, q2, q3, recon);
  input wreal vin;
  output wreal q1;
  output wreal q2;
  output wreal q3;
  output wreal recon;
  wire hi1;
  wire hi2;
  wire hi3;
",
    );
    for index in 1..=FLASH_COMPARATORS {
        design.push_str(&format!(
            "  assign hi{index} = (vin > ({VREF:?} * {index}.0 / {divisor:?}));\n",
            divisor = (FLASH_COMPARATORS + 1) as f64,
        ));
    }
    for index in 1..=FLASH_COMPARATORS {
        design.push_str(&format!("  assign q{index} = hi{index} ? {VDD:?} : 0.0;\n"));
    }
    design.push_str("  assign recon = (hi1 ? ");
    design.push_str(&format!("{quarter:?} : 0.0)\n               + (hi2 ? {quarter:?} : 0.0)\n               + (hi3 ? {quarter:?} : 0.0);\n"));
    design.push_str("endmodule\n");

    let rail_bound = || {
        Bound::new(vec![
            (
                "finite gain at the guard band: VDD * exp(-2 * gain * guard)",
                VDD * (-2.0 * COMPARATOR_GAIN * FLASH_GUARD_V).exp(),
            ),
            Bound::solver(VDD),
        ])
    };
    let recon_full_scale = FLASH_COMPARATORS as f64 * quarter;
    let recon_bound = Bound::new(vec![
        (
            "finite gain at the guard band, summed over the comparators",
            FLASH_COMPARATORS as f64 * quarter * (-2.0 * COMPARATOR_GAIN * FLASH_GUARD_V).exp(),
        ),
        Bound::solver(recon_full_scale),
    ]);

    let mut pairs: Vec<SignalPair> = (1..=FLASH_COMPARATORS)
        .map(|index| SignalPair {
            analog_node: match index {
                1 => "c1",
                2 => "c2",
                _ => "c3",
            },
            rnm_port: match index {
                1 => "q1",
                2 => "q2",
                _ => "q3",
            },
            carries: "one comparator rail against its reference-string tap",
            bound: rail_bound(),
        })
        .collect();
    pairs.push(SignalPair {
        analog_node: "recon",
        rnm_port: "recon",
        carries: "the equally-weighted reconstruction of the thermometer code",
        bound: recon_bound,
    });

    RnmBlock {
        name: "flash_quantizer",
        models: "a three-comparator flash quantizer and the reconstruction of its code",
        why_both_are_honest: "the deck's thresholds are node voltages on a divider it never names; the RNM \
             names them arithmetically, and the reconstruction is a resistive summer against \
             a sum of conditionals",
        deck,
        design,
        stimulus: real_input_stimulus(
            "rnm_flash",
            "vin",
            &["q1", "q2", "q3", "recon"],
            &levels,
            false,
        ),
        pairs,
    }
}

// ===========================================================================
// Block 4 — the gated integrator
// ===========================================================================

/// Charging current, amperes.
const RAMP_I: f64 = 1.0e-6;
/// Integrating capacitance, farads.
const RAMP_C: f64 = 1.0e-12;
/// Shunt across the capacitor, ohms.
///
/// Not decoration: a current source into a lone capacitor leaves the node with
/// no DC path to ground and `.op` refuses it. Large enough that the droop it
/// causes is a bound term rather than a behaviour.
const RAMP_R_LEAK: f64 = 1.0e12;

/// A current-source integrator against a counting accumulator.
///
/// # What it models
///
/// Discrete integration with a hold: charge accumulating on a capacitor while
/// an enable is asserted, frozen while it is not. The one block here whose
/// output depends on the whole history of its input rather than its present
/// value.
///
/// # Why both representations are honest
///
/// The analog side integrates: a gated current into a capacitor, whose voltage
/// at any instant is the charge that has arrived divided by `C`. The RNM side
/// accumulates: a counter that advances on each enabled clock edge, converted to
/// a voltage by the step `I * Ts / C` — a weighted sum over the counter's bits,
/// which is the only way a four-state value reaches the real domain (Verilog-AMS
/// LRM 2.4 section 3.7, and see this module's header for why the state has to
/// live in bits at all).
///
/// So one side discovers `Q = ∫ i dt` by integrating a matrix through time and
/// the other asserts `Q = n * I * Ts`. A gate that opened one sample early, or a
/// step of the wrong size, separates them by `0.1 V` — thirty times the bound.
///
/// # Alignment, which this block has to be explicit about
///
/// The other three blocks compare a *level*, so they only need the input to
/// change at the same instant on both sides. This one compares an *integral*, so
/// the harness has to agree with itself about which interval each vector's
/// enable governs. The convention: **vector `k`'s enable is the current flowing
/// over `(t_{k-1}, t_k]`**, where `t_k` is the sample instant. The deck's `PWL`
/// steps to `I * e[k]` at `t_{k-1}` — that is [`pwl_stepping_at_samples`] — and
/// the RNM's counter advances by `e[k]` at the clock edge that lands on `t_k`.
/// `e[0]` is held at zero because there is only half a period before the first
/// sample and no counter increment can be worth half a step.
///
/// # The bound, derived
///
/// * **Leakage.** The shunt makes the true solution `I*R*(1 - exp(-t/RC))`
///   rather than `I*t/C`, short by `V * t / (2 R C)`. Bounded above over the
///   whole run by `V_max * T_total / (R * C)`.
/// * **`PWL` corners.** Each transition ramps over `PWL_EDGE_S` instead of
///   stepping, delivering half an edge's worth of charge at the wrong time:
///   `I * PWL_EDGE_S / (2 C)` per corner, over every corner in the schedule.
/// * **Interpolation.** The waveform is a straight line between corners and the
///   corners are at the sample instants, so linear interpolation is exact
///   except within one `PWL_EDGE_S` of a corner — already charged above.
/// * **Solver.** `RELTOL * full-scale + VNTOL`.
pub fn ramp_integrator() -> RnmBlock {
    // Enabled, held, enabled again, held, enabled: the pattern that separates
    // "counts clock edges" from "counts enabled clock edges".
    let enables: Vec<u32> = (0..32)
        .map(|k: usize| u32::from(k != 0 && !(9..14).contains(&k) && !(20..23).contains(&k)))
        .collect();
    let currents: Vec<f64> = enables
        .iter()
        .map(|enable| f64::from(*enable) * RAMP_I)
        .collect();

    let deck = format!(
        "gated current-source integrator\n\
         IRAMP 0 ramp PWL({schedule})\n\
         CINT ramp 0 {RAMP_C:?}\n\
         RLEAK ramp 0 {RAMP_R_LEAK:?}\n\
         .end\n",
        schedule = pwl_stepping_at_samples(&currents),
    );

    let step_volts = RAMP_I * SAMPLE_PERIOD_S / RAMP_C;
    let design = format!(
        "\
module rnm_ramp(clk, en, vout);
  input clk;
  input en;
  output wreal vout;
  reg [5:0] count;
  initial count = 6'd0;
  always @(posedge clk) if (en) count = count + 6'd1;
  assign vout = ({RAMP_I:?} * {SAMPLE_PERIOD_S:?} / {RAMP_C:?}) *
      ((count[5] ? 32.0 : 0.0) + (count[4] ? 16.0 : 0.0) + (count[3] ? 8.0 : 0.0)
     + (count[2] ? 4.0 : 0.0) + (count[1] ? 2.0 : 0.0) + (count[0] ? 1.0 : 0.0));
endmodule
"
    );

    let enabled: u32 = enables.iter().sum();
    let full_scale = f64::from(enabled) * step_volts;
    let total_time = enables.len() as f64 * SAMPLE_PERIOD_S;
    let corners = 2 * enables.len();
    let bound = Bound::new(vec![
        (
            "leakage: V_max * T_total / (R * C)",
            full_scale * total_time / (RAMP_R_LEAK * RAMP_C),
        ),
        (
            "PWL corners: corners * I * edge / (2 C)",
            corners as f64 * RAMP_I * PWL_EDGE_S / (2.0 * RAMP_C),
        ),
        Bound::solver(full_scale),
    ]);

    RnmBlock {
        name: "ramp_integrator",
        models: "a gated integrator whose output is the whole history of its enable",
        why_both_are_honest: "the deck integrates a current onto a capacitor; the RNM counts enabled edges \
             and scales by I*Ts/C, and neither computes the other's quantity",
        deck,
        design,
        stimulus: DigitalStimulus {
            module: Some("rnm_ramp".to_string()),
            inputs: vec![DigitalPort {
                name: "en".to_string(),
                width: 1,
            }],
            outputs: vec![DigitalPort {
                name: "vout".to_string(),
                width: 0,
            }],
            clock: Some(DigitalClock {
                port: "clk".to_string(),
                half_period: SETTLE_NS,
            }),
            step: SAMPLE_PERIOD_NS,
            settle: SETTLE_NS,
            vectors: enables
                .iter()
                .map(|enable| vec![format!("{enable}")])
                .collect(),
        },
        pairs: vec![SignalPair {
            analog_node: "ramp",
            rnm_port: "vout",
            carries: "the accumulated capacitor voltage",
            bound,
        }],
    }
}

/// Every reference block, in a fixed order.
pub fn blocks() -> Vec<RnmBlock> {
    vec![
        r2r_dac(),
        schmitt_hysteresis(),
        flash_quantizer(),
        ramp_integrator(),
    ]
}

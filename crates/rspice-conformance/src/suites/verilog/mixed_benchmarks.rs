//! Three mixed-signal circuits, each simulated twice: once with its digital
//! half executed by the Verilog-AMS interleave, once with every part of it
//! analog.
//!
//! # What these are for
//!
//! [`rnm`](super::rnm) authors a block twice to check a *value domain* — a
//! resistor ladder against the closed form a real-number model states. These
//! author a block twice to check a *time* domain: the same converter, loop or
//! modulator, with the same analog front end, differing only in whether its
//! decisions are taken by processes on an event wheel or by behavioural sources
//! on the analog grid. If the interleave's synchronisation were wrong — an edge
//! dated at the wrong instant, a feedback level committed a timepoint late, a
//! trial's state surviving a rejection — these are the circuits where it would
//! show, because each of them closes a loop through the boundary and integrates
//! the result over thousands of decisions.
//!
//! # How a bound is derived here
//!
//! [`rnm`]'s discipline, unchanged: never fitted to a run. Every bound is a sum
//! of named terms, each computed from a declared parameter of the circuit, and
//! the report prints the terms beside the observed error so a reader can see
//! which physical effect the bound was supposed to cover. Two terms recur,
//! because they are what the two representations actually differ by:
//!
//! * **Tick quantization.** A boundary transition the interleave publishes is
//!   dated at the floor of its analog timepoint on the one-nanosecond grid, so
//!   it can be up to one tick early against the analog reference's continuous
//!   crossing. A circuit with `E` boundary transitions per period `T` therefore
//!   sees an average difference of at most `E * 1 ns / T` of full scale.
//! * **Finite comparator gain.** Every analog comparator here is
//!   `V/2 * (1 + tanh(g * overdrive))`, the shape [`rnm`]'s flash blocks use, so
//!   its departure from a step is `V * exp(-2 * g * guard)` at a stated guard
//!   band — and each benchmark states the guard its design actually holds.
//!
//! # What is simplified, and said so
//!
//! Each benchmark carries a list of the simplifications its design makes
//! against the textbook circuit, and [`BenchmarkOutcome::report`] prints them.
//! They are all of one kind: a construct the mixed route does not yet carry —
//! vector boundary ports, bidirectional discrete ports, instance parameters —
//! designed around rather than faked.

use std::fmt::Write as _;
use std::io::Write as _;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant};

use rspice_core::constants::{RELTOL, VNTOL};
use rspice_core::engine::TransientResult;
use rspice_core::netlist::Netlist;
use rspice_core::{Engine, SimulationConfig};

// ===========================================================================
// Shared constants
// ===========================================================================

/// Supply the boundary converts against, which is also the deck default an
/// auto-bridged mixed port derives its thresholds and levels from.
pub const VSUP: f64 = 3.3;

/// Half the supply: where an A/D boundary switches.
pub const VTHRESHOLD: f64 = VSUP / 2.0;

/// Gain of every analog comparator, per volt of overdrive.
///
/// One number for all three benchmarks so the `exp(-2 * g * guard)` term is
/// comparable between them, and large enough that each design's own guard band
/// drives that term far below the solver's promise.
pub const COMPARATOR_GAIN: f64 = 50.0;

/// The digital time base a mixed module runs on, in seconds.
pub const TICK: f64 = 1.0e-9;

/// The engine's promise about any node voltage it reports.
fn solver_bound(full_scale: f64) -> (&'static str, f64) {
    (
        "solver convergence: RELTOL * full-scale + VNTOL",
        RELTOL * full_scale.abs() + VNTOL,
    )
}

/// A comparator's departure from a step at `guard` volts of overdrive.
fn finite_gain_bound(full_scale: f64, guard: f64) -> (&'static str, f64) {
    (
        "finite comparator gain at the design's guard band: V * exp(-2 * g * guard)",
        full_scale * (-2.0 * COMPARATOR_GAIN * guard).exp(),
    )
}

// ===========================================================================
// Bounds and measurements
// ===========================================================================

/// One agreement bound, kept as the terms it was derived from.
#[derive(Debug, Clone)]
pub struct Bound {
    /// What the terms are measured in, for the report.
    pub unit: &'static str,
    /// Named contributions, in `unit`.
    pub terms: Vec<(&'static str, f64)>,
}

impl Bound {
    pub fn new(unit: &'static str, terms: Vec<(&'static str, f64)>) -> Self {
        Self { unit, terms }
    }

    /// The declared bound: the sum of the terms.
    pub fn total(&self) -> f64 {
        self.terms.iter().map(|(_, value)| value).sum()
    }

    /// The derivation, one term per line.
    pub fn derivation(&self) -> String {
        let mut text = String::new();
        for (name, value) in &self.terms {
            let _ = writeln!(text, "        {value:>12.6e}  {name}");
        }
        let _ = writeln!(
            text,
            "        {:>12.6e}  = declared bound ({})",
            self.total(),
            self.unit
        );
        text
    }
}

/// One quantity measured on both representations.
#[derive(Debug, Clone)]
pub struct Measurement {
    /// What was measured.
    pub quantity: String,
    /// The mixed representation's answer.
    pub mixed: f64,
    /// The all-analog representation's answer.
    pub reference: f64,
    /// The declared bound on their difference.
    pub bound: Bound,
}

impl Measurement {
    pub fn error(&self) -> f64 {
        (self.mixed - self.reference).abs()
    }

    pub fn agrees(&self) -> bool {
        self.error() <= self.bound.total()
    }

    /// How much room the bound had. Reported so a bound that is only just
    /// holding is visible before it stops.
    pub fn margin(&self) -> f64 {
        if self.error() > 0.0 {
            self.bound.total() / self.error()
        } else {
            f64::INFINITY
        }
    }
}

/// One benchmark's two runs and what they cost.
#[derive(Debug, Clone)]
pub struct BenchmarkOutcome {
    pub name: &'static str,
    /// What the circuit is.
    pub models: &'static str,
    /// Which parts of it the mixed representation puts in the discrete domain.
    pub mixed_is: &'static str,
    /// How the reference does the same job with no discrete domain at all.
    pub reference_is: &'static str,
    /// Departures from the textbook circuit, and why.
    pub simplifications: &'static [&'static str],
    pub measurements: Vec<Measurement>,
    pub mixed_wall: Duration,
    pub reference_wall: Duration,
    pub mixed_points: usize,
    pub reference_points: usize,
}

impl BenchmarkOutcome {
    pub fn agrees(&self) -> bool {
        self.measurements.iter().all(Measurement::agrees)
    }

    /// The smallest margin any measurement had.
    pub fn worst_margin(&self) -> f64 {
        self.measurements
            .iter()
            .map(Measurement::margin)
            .fold(f64::INFINITY, f64::min)
    }

    /// The whole benchmark as a table, for `--nocapture` and for a failure.
    pub fn report(&self) -> String {
        let mut text = format!("\n{}: {}\n", self.name, self.models);
        let _ = writeln!(text, "  mixed:     {}", self.mixed_is);
        let _ = writeln!(text, "  reference: {}", self.reference_is);
        for note in self.simplifications {
            let _ = writeln!(text, "  simplified: {note}");
        }
        let _ = writeln!(
            text,
            "  wall clock: mixed {:?} over {} accepted points, reference {:?} over {} — \
             mixed/reference {:.2}x",
            self.mixed_wall,
            self.mixed_points,
            self.reference_wall,
            self.reference_points,
            self.mixed_wall.as_secs_f64() / self.reference_wall.as_secs_f64().max(1.0e-12)
        );
        let _ = writeln!(
            text,
            "  {:<44}  {:>14}  {:>14}  {:>12}  {:>12}  {:>7}  ok",
            "quantity", "mixed", "reference", "error", "bound", "margin"
        );
        for measurement in &self.measurements {
            let _ = writeln!(
                text,
                "  {:<44}  {:>14.6e}  {:>14.6e}  {:>12.4e}  {:>12.4e}  {:>6.1}x  {}",
                measurement.quantity,
                measurement.mixed,
                measurement.reference,
                measurement.error(),
                measurement.bound.total(),
                measurement.margin(),
                if measurement.agrees() { "" } else { "EXCEEDED" }
            );
        }
        for measurement in &self.measurements {
            if !measurement.agrees() {
                let _ = writeln!(text, "  `{}` exceeded, as derived:", measurement.quantity);
                text.push_str(&measurement.bound.derivation());
            }
        }
        text
    }
}

// ===========================================================================
// Running a deck
// ===========================================================================

static MODEL_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// A `.va` written to a unique path, deleted when the guard drops.
pub struct ModelFile(PathBuf);

impl ModelFile {
    pub fn new(name: &str, source: &str) -> Self {
        let sequence = MODEL_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let mut path = std::env::temp_dir();
        path.push(format!(
            "rspice_mixed_bench_{name}_{}_{sequence}.va",
            std::process::id()
        ));
        let mut file = std::fs::File::create(&path).expect("create the benchmark's model file");
        file.write_all(source.as_bytes())
            .expect("write the benchmark's model file");
        Self(path)
    }

    pub fn deck_path(&self) -> String {
        self.0.display().to_string().replace('\\', "/")
    }
}

impl Drop for ModelFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.0);
    }
}

/// A run and what it cost, parse included.
///
/// Parse time is inside the measurement because it is inside what a user pays,
/// and because leaving it out would flatter the mixed side: its deck is longer
/// and it compiles a `.va` besides.
struct Timed {
    result: TransientResult,
    wall: Duration,
}

fn run(label: &str, deck: &str, tstop: f64, max_step: f64) -> Result<Timed, String> {
    let started = Instant::now();
    let netlist =
        Netlist::parse(deck).map_err(|error| format!("`{label}` does not parse: {error}"))?;
    let result = Engine::new(SimulationConfig::default())
        .run_tran(&netlist, tstop, max_step)
        .map_err(|error| format!("`{label}` does not run: {error}"))?;
    Ok(Timed {
        wall: started.elapsed(),
        result,
    })
}

/// The last accepted sample at or before `time`.
///
/// Held rather than interpolated, because every quantity these benchmarks read
/// is a held level — a converter's output between conversions, a bitstream
/// between clock edges — and interpolating one would blur an edge the design
/// deliberately puts nowhere near the sampling instant.
fn held(result: &TransientResult, node: &str, time: f64) -> Result<f64, String> {
    let wave = result.try_voltage_waveform_named(node).ok_or_else(|| {
        format!(
            "node `{node}` is not in the result: {:?}",
            result.node_names
        )
    })?;
    let index = result
        .time
        .iter()
        .rposition(|candidate| *candidate <= time)
        .ok_or_else(|| format!("no accepted timepoint at or before {time:e} s"))?;
    Ok(wave[index])
}

/// Mean of a node over `[start, start + window)`, over the accepted grid.
fn windowed_mean(
    result: &TransientResult,
    node: &str,
    start: f64,
    window: f64,
) -> Result<f64, String> {
    let wave = result
        .try_voltage_waveform_named(node)
        .ok_or_else(|| format!("node `{node}` is not in the result"))?;
    let mut sum = 0.0;
    let mut count = 0usize;
    for (index, &time) in result.time.iter().enumerate() {
        if time >= start && time < start + window {
            sum += wave[index];
            count += 1;
        }
    }
    if count == 0 {
        return Err(format!(
            "no accepted timepoints in [{start:e}, +{window:e})"
        ));
    }
    Ok(sum / count as f64)
}

/// Times at which a node crosses `VTHRESHOLD` going up, inside a window.
fn rising_crossings(result: &TransientResult, node: &str, from: f64) -> Result<Vec<f64>, String> {
    let wave = result
        .try_voltage_waveform_named(node)
        .ok_or_else(|| format!("node `{node}` is not in the result"))?;
    let mut crossings = Vec::new();
    let mut above = false;
    for (index, &time) in result.time.iter().enumerate() {
        let now = wave[index] > VTHRESHOLD;
        if now && !above && time >= from && index > 0 {
            // Linear interpolation of the crossing inside the accepted step,
            // so the frequency this yields is not quantized by the step size.
            let (t0, t1) = (result.time[index - 1], time);
            let (v0, v1) = (wave[index - 1], wave[index]);
            crossings.push(if (v1 - v0).abs() > f64::EPSILON {
                t0 + (t1 - t0) * (VTHRESHOLD - v0) / (v1 - v0)
            } else {
                t1
            });
        }
        above = now;
    }
    Ok(crossings)
}

// ===========================================================================
// (a) A four-bit successive-approximation converter
// ===========================================================================

/// Bits in the converter.
const SAR_BITS: u32 = 4;
/// Levels the converter resolves.
const SAR_LEVELS: f64 = 16.0;
/// One code step.
const SAR_LSB: f64 = VSUP / SAR_LEVELS;
/// Half the internal clock period, in nanoseconds, as the module writes it.
const SAR_HALF_CLOCK_NS: u32 = 10;
/// One conversion: a reset phase and one phase per bit.
const SAR_CONVERSION_NS: f64 = SAR_HALF_CLOCK_NS as f64 * 2.0 * (SAR_BITS as f64 + 1.0);
/// Codes the staircase visits.
const SAR_CODES: [u32; 6] = [0, 3, 6, 9, 12, 15];
/// Ladder rung resistance. Large against the boundary's own 20 ohm Thevenin
/// source resistance, so the bridge does not become part of the ladder.
const SAR_LADDER_R: f64 = 100.0e3;

/// The successive-approximation register: five phases, one comparator decision
/// each after the first.
const SAR_MODULE: &str = r#"
`include "disciplines.vams"
module sar4(p, n, cmp, b3, b2, b1, b0);
    inout p, n;
    electrical p, n;
    input cmp;
    output b3, b2, b1, b0;
    wire cmp;
    reg b3, b2, b1, b0;
    reg clk;
    reg [2:0] phase;
    initial clk = 1'b0;
    initial phase = 3'd0;
    initial b3 = 1'b0;
    initial b2 = 1'b0;
    initial b1 = 1'b0;
    initial b0 = 1'b0;
    always #10 clk = ~clk;
    always @(posedge clk) begin
        case (phase)
            3'd0: begin b3 <= 1'b1; b2 <= 1'b0; b1 <= 1'b0; b0 <= 1'b0; phase <= 3'd1; end
            3'd1: begin b3 <= cmp; b2 <= 1'b1; phase <= 3'd2; end
            3'd2: begin b2 <= cmp; b1 <= 1'b1; phase <= 3'd3; end
            3'd3: begin b1 <= cmp; b0 <= 1'b1; phase <= 3'd4; end
            default: begin b0 <= cmp; phase <= 3'd0; end
        endcase
    end
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

/// The staircase both representations convert, held at mid-code.
///
/// Mid-code because a converter's answer is only defined away from its decision
/// thresholds: half an LSB of margin is 103 mV, which is thirty times the
/// solver's promise about the node the decision is taken on.
fn sar_staircase() -> String {
    let mut text = String::new();
    for (index, code) in SAR_CODES.iter().enumerate() {
        let level = (f64::from(*code) + 0.5) * SAR_LSB;
        let edge = index as f64 * SAR_CONVERSION_NS * 1.0e-9;
        if index == 0 {
            let _ = write!(text, "0 {level:?}");
        } else {
            let previous = (f64::from(SAR_CODES[index - 1]) + 0.5) * SAR_LSB;
            let _ = write!(
                text,
                " {:?} {previous:?} {edge:?} {level:?}",
                edge - 1.0e-12
            );
        }
    }
    text
}

/// Track-and-hold in front of both converters, identical in each.
fn sar_front_end() -> String {
    format!(
        "vsrc src 0 pwl({})\nrsh src vin 100\ncsh vin 0 1p\n",
        sar_staircase()
    )
}

fn sar_mixed_deck(model: &ModelFile) -> String {
    let mut deck =
        String::from("* four-bit sar: analog track-and-hold, r-2r dac, analog comparator\n");
    deck.push_str(&sar_front_end());
    deck.push_str("x1 p 0 cmp b3 b2 b1 b0 sar4\n");
    deck.push_str("rp p 0 1meg\n");
    for bit in 0..SAR_BITS {
        let _ = writeln!(deck, "rarm{bit} b{bit} nd{bit} {:?}", 2.0 * SAR_LADDER_R);
    }
    let _ = writeln!(deck, "rterm nd0 0 {:?}", 2.0 * SAR_LADDER_R);
    for rung in 0..SAR_BITS - 1 {
        let _ = writeln!(deck, "rrung{rung} nd{rung} nd{} {SAR_LADDER_R:?}", rung + 1);
    }
    let _ = writeln!(deck, "rout nd{} vdac 1.0", SAR_BITS - 1);
    deck.push_str("cdac vdac 0 0.01p\n");
    let _ = writeln!(
        deck,
        "bcmp cmp 0 V={{{VTHRESHOLD:?} + {VTHRESHOLD:?}*tanh({COMPARATOR_GAIN:?}*(V(vin)-V(vdac)))}}"
    );
    deck.push_str("rcmp cmp 0 1meg\n");
    let _ = writeln!(deck, ".va \"{}\" sar4", model.deck_path());
    deck.push_str(".end\n");
    deck
}

fn sar_reference_deck() -> String {
    let taps = (1u32 << SAR_BITS) - 1;
    let string_r = 10.0e3;
    let mut deck =
        String::from("* four-bit flash reference: comparator string and resistive summer\n");
    deck.push_str(&sar_front_end());
    let _ = writeln!(deck, "vrefs ref 0 {VSUP:?}");
    let _ = writeln!(deck, "rstr{taps} ref t{taps} {string_r:?}");
    for rung in (1..taps).rev() {
        let _ = writeln!(deck, "rstr{rung} t{} t{rung} {string_r:?}", rung + 1);
    }
    let _ = writeln!(deck, "rstr0 t1 0 {string_r:?}");
    for index in 1..=taps {
        let _ = writeln!(
            deck,
            "bc{index} c{index} 0 V={{{VTHRESHOLD:?} + {VTHRESHOLD:?}*tanh({COMPARATOR_GAIN:?}*(V(vin)-V(t{index})))}}"
        );
        let _ = writeln!(deck, "rsum{index} c{index} recon {string_r:?}");
    }
    let _ = writeln!(deck, "rsumt recon 0 {string_r:?}");
    deck.push_str(".end\n");
    deck
}

/// The declared bound on one reconstructed conversion, in volts.
///
/// Three terms, and the third is the one that matters:
///
/// * **Finite comparator gain** at the design's half-LSB guard band. Both
///   representations use the same comparator shape, so this is what each one's
///   decision can be wrong by, not their difference — carried because a bound
///   that omits the term it is protecting against says nothing about it.
/// * **The boundary's source resistance in the ladder.** A D/A boundary drives
///   through 20 ohms into a 200 kilohm arm, so each bit's contribution is low
///   by that ratio: `VSUP * 20 / (2 * SAR_LADDER_R)`.
/// * **The solver's promise** about the node each answer is read from.
fn sar_bound() -> Bound {
    Bound::new(
        "V",
        vec![
            finite_gain_bound(VSUP, SAR_LSB / 2.0),
            (
                "the D/A boundary's 20 ohm source resistance inside a 2R ladder arm",
                VSUP * 20.0 / (2.0 * SAR_LADDER_R),
            ),
            solver_bound(VSUP),
        ],
    )
}

/// A four-bit successive-approximation converter against a flash reference.
///
/// # What the two share and what they do not
///
/// Both convert the same staircase through the same track-and-hold, and both
/// decide with the same comparator shape. Below that they share nothing: the
/// mixed converter takes four sequential decisions in a register clocked by the
/// module's own time wheel, feeding an R-2R ladder through four D/A boundaries;
/// the reference takes fifteen simultaneous decisions against a resistor string
/// and adds them in a resistive summer. A synchronisation defect in the
/// interleave — a comparator sampled a phase early, a bit committed a timepoint
/// late — moves the mixed answer by at least one LSB, which is two thousand
/// times the bound below.
pub fn sar_adc() -> Result<BenchmarkOutcome, String> {
    let model = ModelFile::new("sar4", SAR_MODULE);
    let tstop = SAR_CODES.len() as f64 * SAR_CONVERSION_NS * 1.0e-9;
    let mixed = run("sar mixed", &sar_mixed_deck(&model), tstop, 1.0e-9)?;
    let reference = run("sar reference", &sar_reference_deck(), tstop, 1.0e-9)?;

    let mut measurements = Vec::new();
    for (index, code) in SAR_CODES.iter().enumerate() {
        // Late in the conversion window, after the last bit has settled and
        // before the next conversion's reset phase.
        let at = (index as f64 + 0.98) * SAR_CONVERSION_NS * 1.0e-9;
        let mixed_volts = held(&mixed.result, "vdac", at)?;
        let reference_volts = held(&reference.result, "recon", at)?;
        let expected = f64::from(*code) * SAR_LSB;
        for (label, volts) in [("mixed", mixed_volts), ("reference", reference_volts)] {
            let observed = (volts / SAR_LSB).round() as i64;
            if observed != i64::from(*code) {
                return Err(format!(
                    "the {label} converter answered code {observed} where the input sits at \
                     mid-code {code} ({volts} V against {expected} V)"
                ));
            }
        }
        measurements.push(Measurement {
            quantity: format!("reconstructed level at conversion {index} (code {code})"),
            mixed: mixed_volts,
            reference: reference_volts,
            bound: sar_bound(),
        });
    }

    Ok(BenchmarkOutcome {
        name: "sar_adc",
        models: "a four-bit successive-approximation converter over six input codes",
        mixed_is: "analog track-and-hold, R-2R ladder and comparator; the successive- \
                   approximation register is four processes on the module's own clock, reaching \
                   the ladder through four D/A boundaries and the comparator through one A/D",
        reference_is: "the same track-and-hold driving fifteen comparators against a resistor \
                       string, summed resistively — no discrete domain anywhere",
        simplifications: &[
            "the register's four bits leave the module as four scalar ports rather than one \
             four-bit port: the mixed route bridges scalar boundaries only, and a vector port \
             is refused by name rather than split behind the author's back",
            "the reference is a flash converter rather than the same register written as \
             behavioural analog: a successive-approximation register is sequential, and \
             expressing its state in behavioural sources would need the sampling element this \
             benchmark is trying to hold something against",
        ],
        measurements,
        mixed_wall: mixed.wall,
        reference_wall: reference.wall,
        mixed_points: mixed.result.time.len(),
        reference_points: reference.result.time.len(),
    })
}

// ===========================================================================
// (b) A phase-locked loop
// ===========================================================================

/// Reference frequency the loop must acquire.
const PLL_REF_HZ: f64 = 1.0e6;
/// The VCO's free-running frequency at zero control.
const PLL_F0_HZ: f64 = 0.9e6;
/// The VCO's tuning gain.
const PLL_KV_HZ_PER_V: f64 = 0.2e6;
/// Loop filter series resistance.
const PLL_R1: f64 = 100.0e3;
/// Loop filter zero resistance.
const PLL_RZ: f64 = 20.0e3;
/// Loop filter capacitance.
const PLL_C: f64 = 100.0e-12;
const PLL_TSTOP: f64 = 60.0e-6;
const PLL_STEP: f64 = 20.0e-9;
/// Window the settled behaviour is measured over.
const PLL_SETTLED_WINDOW: f64 = 20.0e-6;
/// How close to its settled value the control voltage must come, and stay, to
/// count as locked.
///
/// A specification, not a fit. It has to be wider than the phase detector's own
/// ripple at the control node, because otherwise "lock" would be the last time
/// the ripple happened to be small rather than the time the loop arrived. The
/// filter's high-frequency floor is `Rz / (R1 + Rz)` of the detector's swing —
/// a sixth of 3.3 V — and averaging one reference period cancels the
/// fundamental at twice the reference but not its harmonics. A tenth of the
/// settled value is above what survives that and far below the excursion the
/// loop makes on its way in, which is a third of the settled value.
const PLL_LOCK_BAND: f64 = 0.10;

/// Reference clock, VCO, and loop filter — identical in both representations.
///
/// The VCO's phase is `sdt` of the control voltage rather than a capacitor,
/// because an ideal integrator with a capacitor has no operating point: its
/// DC gain is infinite and the bias solve puts the phase node at whatever the
/// conditioning conductance makes of a constant current. `sdt` starts at zero
/// by construction, which is the initial condition a phase actually has.
///
/// The three-quarter-cycle phase offset starts the VCO at a rail rather than at
/// a zero crossing, so both phase detectors read the same thing at `t = 0`. At
/// a crossing the smooth one reads a half and the discrete one reads a zero,
/// and the loop's acquisition is sensitive enough to that first microsecond
/// that the two would land in different basins.
fn pll_plant() -> String {
    format!(
        "vref refin 0 pulse(0 {VSUP:?} 0 5n 5n 495n 1000n)\n\
         rref refin 0 1meg\n\
         bvco vcoin 0 V={{{VTHRESHOLD:?} + {VTHRESHOLD:?}*tanh({gain:?}*sin(6.283185307179586*({PLL_F0_HZ:?}*time + {PLL_KV_HZ_PER_V:?}*sdt(V(vctrl)) + 0.75)))}}\n\
         rvco vcoin 0 1meg\n\
         rlf pderr vctrl {PLL_R1:?}\n\
         clf vctrl nz {PLL_C:?}\n\
         rz nz 0 {PLL_RZ:?}\n\
         rvc vctrl 0 1000meg\n",
        gain = 2.0 * COMPARATOR_GAIN,
    )
}

/// The phase detector: an exclusive-or of the two square waves.
const PLL_MODULE: &str = r#"
`include "disciplines.vams"
module xor_pd(p, n, refin, vcoin, pderr);
    inout p, n;
    electrical p, n;
    input refin, vcoin;
    output pderr;
    wire refin, vcoin;
    reg pderr;
    initial pderr = 1'b0;
    always @(refin or vcoin) pderr = refin ^ vcoin;
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

fn pll_mixed_deck(model: &ModelFile) -> String {
    format!(
        "* type-2 pll: analog VCO and loop filter, discrete exclusive-or phase detector\n\
         {plant}\
         x1 p 0 refin vcoin pderr xor_pd\n\
         rp p 0 1meg\n\
         .va \"{}\" xor_pd\n\
         .end\n",
        model.deck_path(),
        plant = pll_plant()
    )
}

fn pll_reference_deck() -> String {
    // The same exclusive-or as a smooth analog function: `a + b - 2ab` on two
    // comparator outputs normalised to [0, 1]. It is the standard's own truth
    // table wherever the comparators are saturated, and its departure from one
    // where they are not is the finite-gain term the bound carries.
    let sign = |node: &str| {
        format!(
            "(0.5+0.5*tanh({gain:?}*(V({node})-{VTHRESHOLD:?})))",
            gain = 2.0 * COMPARATOR_GAIN
        )
    };
    let (a, b) = (sign("refin"), sign("vcoin"));
    format!(
        "* type-2 pll, all analog: a smooth exclusive-or phase detector\n\
         {plant}\
         bpd pderr 0 V={{{VSUP:?}*({a} + {b} - 2.0*{a}*{b})}}\n\
         .end\n",
        plant = pll_plant()
    )
}

/// The first time after which the control voltage stays inside a band around
/// its settled value, measured on one-microsecond means so the phase
/// detector's own ripple does not decide the answer.
fn pll_lock_time(result: &TransientResult, band: f64) -> Result<(f64, f64), String> {
    let window = 1.0 / PLL_REF_HZ;
    let steps = (PLL_TSTOP / window) as usize;
    let mut means = Vec::with_capacity(steps);
    for step in 0..steps {
        means.push(windowed_mean(
            result,
            "vctrl",
            step as f64 * window,
            window,
        )?);
    }
    let settled = *means.last().ok_or("the run produced no windows")?;
    let mut lock = f64::NAN;
    for (step, mean) in means.iter().enumerate() {
        if (mean - settled).abs() > band * settled.abs() {
            lock = f64::NAN;
        } else if lock.is_nan() {
            lock = step as f64 * window;
        }
    }
    if lock.is_nan() {
        return Err("the loop never settled inside the band".to_string());
    }
    Ok((lock, settled))
}

/// The two representations' phase detectors differ by the tick grid alone.
///
/// The exclusive-or's output changes on every edge of either input. At lock
/// there are two reference edges and two VCO edges per reference period, and
/// the mixed detector dates each of them at the floor of its analog timepoint —
/// so up to one tick early. The output is a rail, so a mis-timed edge is a
/// `VSUP`-tall sliver one tick wide, and the loop filter's time constant is far
/// longer than a reference period, which makes what reaches the control node
/// the *average*: `VSUP * 4 ticks / period`.
fn pll_control_bound() -> Bound {
    Bound::new(
        "V",
        vec![
            (
                "tick quantization: VSUP * (4 boundary edges per reference period) * 1 ns / period",
                VSUP * 4.0 * TICK * PLL_REF_HZ,
            ),
            finite_gain_bound(VSUP, VTHRESHOLD / 2.0),
            solver_bound(VSUP),
        ],
    )
}

/// A phase-locked loop with the phase detector on either side of the boundary.
///
/// # What the two share and what they do not
///
/// The reference clock, the VCO and the loop filter are the same deck text in
/// both, because what is being compared is the detector: in one it is a process
/// woken by two A/D boundaries and driving a D/A one, in the other it is a
/// behavioural source with no discrete domain in it at all. Everything the loop
/// then does — acquisition, ringing, the phase error it settles at — is an
/// integration of that detector's output over sixty microseconds, so a
/// detector whose edges were dated wrongly would not merely shift the answer,
/// it would change which frequency the loop acquired.
pub fn pll() -> Result<BenchmarkOutcome, String> {
    let model = ModelFile::new("xor_pd", PLL_MODULE);
    let mixed = run("pll mixed", &pll_mixed_deck(&model), PLL_TSTOP, PLL_STEP)?;
    let reference = run("pll reference", &pll_reference_deck(), PLL_TSTOP, PLL_STEP)?;

    let settled_from = PLL_TSTOP - PLL_SETTLED_WINDOW;
    let mut frequencies = Vec::new();
    for (label, result) in [("mixed", &mixed.result), ("reference", &reference.result)] {
        let crossings = rising_crossings(result, "vcoin", settled_from)?;
        if crossings.len() < 4 {
            return Err(format!(
                "the {label} VCO produced {} settled edges, which is not a frequency",
                crossings.len()
            ));
        }
        let span = crossings[crossings.len() - 1] - crossings[0];
        frequencies.push((crossings.len() - 1) as f64 / span);
    }

    let (mixed_lock, mixed_settled) = pll_lock_time(&mixed.result, PLL_LOCK_BAND)?;
    let (reference_lock, reference_settled) = pll_lock_time(&reference.result, PLL_LOCK_BAND)?;

    // Each loop must have acquired the reference, or comparing them compares
    // two circuits that are not doing the job.
    for (label, frequency) in [("mixed", frequencies[0]), ("reference", frequencies[1])] {
        let acquired = (frequency - PLL_REF_HZ).abs() / PLL_REF_HZ;
        if acquired > 0.01 {
            return Err(format!(
                "the {label} loop settled at {:.4} MHz against a {:.4} MHz reference, so it \
                 never acquired",
                frequency / 1.0e6,
                PLL_REF_HZ / 1.0e6
            ));
        }
    }

    // The locked frequency's own bound. The VCO's law is
    // `f = F0 + KV * vctrl`, so a control-voltage difference of `d` volts is a
    // frequency difference of `KV * d` hertz — and the edge-counting
    // measurement adds half a cycle over the window it counts across.
    let frequency_bound = Bound::new(
        "Hz",
        vec![
            (
                "the control-voltage bound through the VCO's tuning gain: KV * bound",
                PLL_KV_HZ_PER_V * pll_control_bound().total(),
            ),
            (
                "edge-counting resolution: half a cycle over the settled window",
                0.5 / PLL_SETTLED_WINDOW,
            ),
        ],
    );

    // The lock time's bound. The two trajectories differ by at most the
    // control-voltage bound; near the lock threshold the trajectory is
    // approaching its settled value at the loop's own rate, so that difference
    // maps to a time through the slope, and the one-microsecond windows the
    // measurement uses add their own width.
    let natural = (loop_gain() / ((PLL_R1 + PLL_RZ) * PLL_C)).sqrt();
    let damping = 0.5 * natural * (PLL_RZ * PLL_C + 1.0 / loop_gain());
    let approach_rate = mixed_settled.abs() * damping * natural * (-1.0f64).exp();
    let lock_bound = Bound::new(
        "s",
        vec![
            (
                "the control-voltage bound divided by the trajectory's approach rate",
                pll_control_bound().total() / approach_rate,
            ),
            (
                "the width of the window the measurement averages over",
                1.0 / PLL_REF_HZ,
            ),
        ],
    );

    Ok(BenchmarkOutcome {
        name: "pll",
        models: "a type-2 phase-locked loop acquiring a 1 MHz reference from a 0.9 MHz \
                 free-running VCO",
        mixed_is: "the exclusive-or phase detector is a process woken by two A/D boundaries and \
                   driving the loop filter through a D/A one",
        reference_is: "the same detector as a behavioural source: a smooth exclusive-or of two \
                       comparator outputs",
        simplifications: &[
            "an exclusive-or detector rather than a phase-frequency detector with a charge \
             pump: a PFD's two outputs and its reset path need a bidirectional discrete port \
             for the charge pump's tri-state, which the mixed route refuses by name",
            "the VCO's phase is `sdt` of the control voltage rather than a charge on a \
             capacitor, because an ideal integrator has no operating point",
        ],
        measurements: vec![
            Measurement {
                quantity: "settled control voltage".to_string(),
                mixed: mixed_settled,
                reference: reference_settled,
                bound: pll_control_bound(),
            },
            Measurement {
                quantity: "locked VCO frequency".to_string(),
                mixed: frequencies[0],
                reference: frequencies[1],
                bound: frequency_bound,
            },
            Measurement {
                quantity: "lock time".to_string(),
                mixed: mixed_lock,
                reference: reference_lock,
                bound: lock_bound,
            },
        ],
        mixed_wall: mixed.wall,
        reference_wall: reference.wall,
        mixed_points: mixed.result.time.len(),
        reference_points: reference.result.time.len(),
    })
}

/// The loop's open-loop gain, `Kpd * Kvco`, in reciprocal seconds.
///
/// An exclusive-or detector driven by square waves puts out `VSUP * phi / pi`
/// on average, so `Kpd = VSUP / pi` volts per radian; the VCO contributes
/// `2 * pi * KV` radians per second per volt.
fn loop_gain() -> f64 {
    (VSUP / std::f64::consts::PI) * (2.0 * std::f64::consts::PI * PLL_KV_HZ_PER_V)
}

// ===========================================================================
// (c) A first-order sigma-delta modulator
// ===========================================================================

/// Clock period of the modulator, in seconds.
const SD_CLOCK: f64 = 100.0e-9;
/// Clocks the comparison runs over.
const SD_CLOCKS: usize = 512;
/// The DC input both modulators convert.
const SD_INPUT: f64 = 2.0;
/// The integrator's gain: one volt of input moves it one volt per clock.
const SD_INTEGRATOR_GAIN: f64 = 1.0 / SD_CLOCK;
/// Ones the counter accumulates before it toggles its carry.
///
/// Four rather than a larger modulus because the carry is what the decimated
/// count has to be read back from, and the residual it leaves — up to
/// `modulus - 1` ones out of `SD_CLOCKS` — is the whole of the counter's
/// contribution to the bound. A wider counter would be a coarser decimator, not
/// a better one.
const SD_CARRY_MODULUS: usize = 4;

/// The analog front end: an `sdt` integrator and a comparator, with the
/// feedback node named by the caller.
fn sd_front_end(feedback: &str) -> String {
    format!(
        "vin vin 0 {SD_INPUT:?}\n\
         bni ni 0 V={{{SD_INTEGRATOR_GAIN:?}*sdt(V(vin)-V({feedback}))}}\n\
         rni ni 0 1meg\n\
         bcmp cmp 0 V={{{VTHRESHOLD:?} + {VTHRESHOLD:?}*tanh({COMPARATOR_GAIN:?}*V(ni))}}\n\
         rcmp cmp 0 1meg\n"
    )
}

/// The quantizer and the decimator, in the discrete domain.
///
/// The counter's carry is what leaves the module, because a boundary port
/// carries one bit: four ones toggle it, so the number of toggles times four is
/// the accumulated count to within the modulus.
const SD_MODULE: &str = r#"
`include "disciplines.vams"
module sd_dec(p, n, cmp, bs, carry);
    inout p, n;
    electrical p, n;
    input cmp;
    output bs, carry;
    wire cmp;
    reg bs, carry, clk;
    reg [1:0] ones;
    initial clk = 1'b0;
    initial bs = 1'b0;
    initial carry = 1'b0;
    initial ones = 2'd0;
    always #50 clk = ~clk;
    always @(posedge clk) begin
        bs <= cmp;
        if (cmp) begin
            if (ones == 2'd3) carry <= ~carry;
            ones <= ones + 2'd1;
        end
    end
    analog I(p, n) <+ V(p, n) / 1000000.0;
endmodule
"#;

fn sd_mixed_deck(model: &ModelFile) -> String {
    format!(
        "* first-order sigma-delta: analog loop, discrete quantizer and decimator\n\
         {front}\
         x1 p 0 cmp bs carry sd_dec\n\
         rp p 0 1meg\n\
         rbs bs 0 1meg\n\
         rcy carry 0 1meg\n\
         .va \"{}\" sd_dec\n\
         .end\n",
        model.deck_path(),
        front = sd_front_end("bs")
    )
}

/// The same modulator with an all-analog master-slave sampling latch.
///
/// A single track-and-hold will not do, and the reason is worth stating: while
/// its switch is closed the loop is continuous, and a continuous first-order
/// modulator with a smooth comparator has a static equilibrium — the integrator
/// input goes to zero with the quantizer sitting part way between its rails,
/// and the modulator stops modulating. Two holds on opposite clock phases break
/// that path: whichever phase the clock is in, one of the two switches is open,
/// so no instant has a closed loop through the quantizer.
fn sd_reference_deck() -> String {
    format!(
        "* first-order sigma-delta, all analog: a master-slave sampling latch\n\
         {front}\
         vclk clkn 0 pulse(0 {VSUP:?} 0 1n 1n 49n 100n)\n\
         vclkb clkbn 0 pulse({VSUP:?} 0 0 1n 1n 49n 100n)\n\
         s1 cmp mstr clkn 0 swmod\n\
         cm mstr 0 1p\n\
         rm mstr 0 1000meg\n\
         s2 mstr slv clkbn 0 swmod\n\
         cs slv 0 1p\n\
         rs slv 0 1000meg\n\
         .model swmod sw(vt={VTHRESHOLD:?} vh=0.1 ron=100 roff=1g)\n\
         bfb fb 0 V={{{VTHRESHOLD:?} + {VTHRESHOLD:?}*tanh({COMPARATOR_GAIN:?}*(V(slv)-{VTHRESHOLD:?}))}}\n\
         rfb fb 0 1meg\n\
         .end\n",
        front = sd_front_end("fb")
    )
}

/// The bitstream's mean, read at a fixed phase of each clock period.
fn sd_bitstream_mean(result: &TransientResult, node: &str) -> Result<f64, String> {
    let mut ones = 0usize;
    for clock in 0..SD_CLOCKS {
        let at = clock as f64 * SD_CLOCK + 0.9 * SD_CLOCK;
        if held(result, node, at)? > VTHRESHOLD {
            ones += 1;
        }
    }
    Ok(ones as f64 / SD_CLOCKS as f64)
}

/// The bound on a decimated mean, as a fraction of full scale.
///
/// The physics is a conservation statement rather than a tolerance. Over `N`
/// clocks the integrator's state moves by the accumulated difference between
/// the input and the feedback, scaled by the integrator's own gain — and the
/// gain here is one volt per volt per clock, so
///
/// ```text
/// x_N - x_0 = sum_k (vin - fb_k)
/// mean(fb) = vin - (x_N - x_0) / N
/// ```
///
/// The integrator's state is bounded by the loop: the comparator forces the
/// feedback to oppose it, so a step can carry it at most `VSUP` past the
/// threshold in either direction and it is turned around on the next clock.
/// That bounds `|x_N - x_0|` by `2 * VSUP` and the mean error by `2 * VSUP / N`
/// volts, which is `2 / N` of full scale.
fn sd_mean_bound(includes_counter_quantization: bool) -> Bound {
    let mut terms = vec![
        (
            "first-order DC balance: the integrator's bounded excursion over N clocks, twice",
            2.0 * (2.0 * VSUP / SD_CLOCKS as f64) / VSUP,
        ),
        (
            "finite comparator gain at one integrator step of overdrive",
            finite_gain_bound(VSUP, VSUP / 2.0).1 / VSUP,
        ),
        (
            "solver convergence on the sampled level",
            RELTOL + VNTOL / VSUP,
        ),
    ];
    if includes_counter_quantization {
        terms.push((
            "the carry counter's modulus over N clocks",
            SD_CARRY_MODULUS as f64 / SD_CLOCKS as f64,
        ));
    }
    Bound::new("fraction of full scale", terms)
}

/// A first-order sigma-delta modulator with the quantizer and decimator on
/// either side of the boundary.
///
/// # What the two share and what they do not
///
/// The integrator, the comparator and the input are the same deck text. What
/// differs is the sampling element and the decimator: the mixed modulator
/// samples the comparator into a register on its own clock and counts the ones
/// in a three-bit counter whose carry leaves the module; the reference samples
/// into a master-slave pair of switched holds and its bitstream is read off the
/// analog feedback node.
///
/// A first-order modulator's decimated mean is not a tolerance question. The
/// integrator cannot run away, so over `N` clocks the feedback's average has to
/// equal the input to within the integrator's own excursion divided by `N`, and
/// that is what both representations are held to — separately, against the
/// input, and then against each other.
pub fn sigma_delta() -> Result<BenchmarkOutcome, String> {
    let model = ModelFile::new("sd_dec", SD_MODULE);
    let tstop = SD_CLOCKS as f64 * SD_CLOCK;
    let mixed = run("sigma-delta mixed", &sd_mixed_deck(&model), tstop, 5.0e-9)?;
    let reference = run("sigma-delta reference", &sd_reference_deck(), tstop, 5.0e-9)?;

    let mixed_mean = sd_bitstream_mean(&mixed.result, "bs")?;
    let reference_mean = sd_bitstream_mean(&reference.result, "fb")?;

    // The decimated mean the module's own counter reports, from its carry.
    let carries = mixed
        .result
        .digital_trace_named("carry")
        .ok_or("the mixed run has no `carry` trace")?
        .len()
        .saturating_sub(1);
    let decimated = (carries * SD_CARRY_MODULUS) as f64 / SD_CLOCKS as f64;

    let expected = SD_INPUT / VSUP;
    Ok(BenchmarkOutcome {
        name: "sigma_delta",
        models: "a first-order sigma-delta modulator converting a 2 V input on a 3.3 V \
                 reference over 512 clocks",
        mixed_is: "the comparator is sampled into a register across an A/D boundary, the ones \
                   are counted in a two-bit counter, and both the bitstream and the counter's \
                   carry leave the module across D/A boundaries",
        reference_is: "the same loop with a master-slave pair of switched sample-and-holds as \
                       the quantizer, and the bitstream read off the analog feedback node",
        simplifications: &[
            "the decimator's count leaves the module as a carry rather than as a word: a \
             boundary port carries one bit, so four ones toggle the carry and the toggles are \
             counted outside",
            "the reference's sampling element is a master-slave pair rather than a single \
             track-and-hold, because a single one leaves the loop continuous while its switch \
             is closed and a continuous first-order modulator with a smooth comparator has a \
             static equilibrium it stops modulating at",
        ],
        measurements: vec![
            Measurement {
                quantity: "decimated mean of the bitstream".to_string(),
                mixed: mixed_mean,
                reference: reference_mean,
                bound: sd_mean_bound(false),
            },
            Measurement {
                quantity: "decimated mean against the input, mixed".to_string(),
                mixed: mixed_mean,
                reference: expected,
                bound: sd_mean_bound(false),
            },
            Measurement {
                quantity: "decimated mean against the input, reference".to_string(),
                mixed: reference_mean,
                reference: expected,
                bound: sd_mean_bound(false),
            },
            Measurement {
                quantity: "the module's own counter against the bitstream".to_string(),
                mixed: decimated,
                reference: mixed_mean,
                bound: sd_mean_bound(true),
            },
        ],
        mixed_wall: mixed.wall,
        reference_wall: reference.wall,
        mixed_points: mixed.result.time.len(),
        reference_points: reference.result.time.len(),
    })
}

/// Every benchmark, in the order the report prints them.
pub fn all() -> Vec<fn() -> Result<BenchmarkOutcome, String>> {
    vec![sar_adc, pll, sigma_delta]
}

/// The names every benchmark run must produce, so one going missing fails on
/// its own name rather than on a count.
pub const REQUIRED_BENCHMARKS: [&str; 3] = ["sar_adc", "pll", "sigma_delta"];

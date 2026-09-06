//! Native execution of digital IEEE 1364-2005 Verilog.
//!
//! A caller hands this module Verilog source and a stimulus and gets back one
//! observation per vector. Everything between — compiling the source to a
//! [`CanonicalDigitalPlan`], resolving its nets, running its processes on the
//! event kernel — is here or in the two submodules below it.
//!
//! # Two entry points, one for each shape of caller
//!
//! [`run_digital_verilog`] is the one-call route and compiles on every call.
//! [`CompiledDigitalDesign`] is the same route with the compile hoisted out, for
//! a caller with many stimuli and one design; the one-call route is literally
//! the composition of its two halves. See its documentation for what is shared
//! between two runs of one compiled design (the plan, which is immutable) and
//! what is not (all of the running state).
//!
//! # Why it lives under `xspice`
//!
//! Not because it is an XSPICE code model; it is not. Because of the layering
//! ratchet in `tests/module_layering.rs`. This host is built on
//! [`event_scheduler`](super::event_scheduler), the discrete-event kernel,
//! which sits at rank 9 for reasons its own module documentation gives. A
//! module that names the kernel must sit strictly above rank 9, and the only
//! rank above it is `circuit`'s — which this has no business being part of.
//! Sitting *beside* the kernel is the honest placement available today: same
//! rank, no upward edge, and no rank renumbering in a file several lanes share.
//!
//! # The time-unit ruling
//!
//! See [`TIME_UNIT_RULING`]. It is a constant rather than a comment because a
//! caller comparing this against another simulator has to be able to read it.
//!
//! Mixed modules are executed through [`MixedSignalHost`], whose trial
//! transaction aligns the digital event slot with each analog Newton solve.
//! The boundary is explicit: an ADC bridge publishes accepted analog samples
//! with hysteresis, and a DAC bridge stamps a Thevenin equivalent. Each bridge
//! carries *one bit* of one discrete signal, so a vector port is one bridge per
//! conductor — the deck names one node per bit — while the discrete half still
//! sees whole-vector transitions, because the A/D settle composes a port's bit
//! drives into one write. Bidirectional coercion remains fail-closed until its
//! resolution semantics are represented directly.
//!
//! # Where a `wreal` meets an analog node
//!
//! Not here yet, and the boundary's rulings are recorded here because they
//! have a right answer worth writing down before somebody guesses one. The
//! mixed host above now implements the time half of them —
//! [`TimeResolution::seconds_to_floor_ticks`](super::event_scheduler::TimeResolution::seconds_to_floor_ticks)
//! is the floor, and the crossing an A/D bridge is dated by is interpolated
//! inside the accepted step rather than snapped to the tick. That host applies
//! a *second* time mapping the ruling below does not cover, because it is not
//! about advancing anything: an A/D transition's own timestamp names the tick
//! its event lands on, and Verilog-AMS LRM 2.4 section 7.3.6.1 fixes that at
//! the nearest tick rather than the floor — see
//! [`MixedSignalHost::settle_analog_bridges`].
//! What a `wreal` still needs from this section is the *driver-resolution*
//! hazard below, which is not about time at all.
//!
//! **The two event worlds do not share a tick encoding**:
//!
//! * the circuit's queue keys an event by `f64::to_bits(seconds)`, which is
//!   exact and unquantized because XSPICE event times are chosen by code models
//!   and by the step controller rather than lying on a declared grid;
//! * this host keys one by an integer count of the declared time unit, which
//!   at [`TIME_UNIT_RULING`]'s 1 ns is a coarse grid indeed.
//!
//! No mapping between the two is exact in both directions, so the choice is
//! which property to keep, and there is one answer that keeps the right ones
//! *for advancing the digital world*:
//! **floor an analog time to the tick at or before it, and publish an event at
//! the unquantized analog time.** Flooring is monotone, so a non-decreasing
//! sequence of accepted analog times gives [`DigitalHost::advance_to`] a
//! non-decreasing sequence of ticks; it never runs the digital world past an
//! instant the integrator has accepted, which rounding to nearest would;
//! and two analog times inside one tick collapse rather than reorder, which is
//! what a declared precision *means*. Publishing at the analog time rather than
//! at the tick's seconds is what keeps D5 clause 2 — the step controller stops
//! bit-exactly at an event time — untouched by the grid.
//!
//! **One hazard to check when that boundary is wired.** The two sides resolve
//! multiple drivers differently. A circuit real event node *sums* its drivers
//! (`circuit::external_models`), while Verilog-AMS LRM 2.4 section 6.5.3 permits
//! exactly one driver of a `wreal` and the front end refuses a second. A
//! published `wreal` must therefore be the only driver of the node it lands on,
//! or the analog side sees a sum neither standard asked for, silently.
//!
//! The bridge halves already exist as code models — `real_to_v` and
//! `v_to_real` (sample on accepted step, no threshold, no breakpoint), both
//! planned in by `engine::builder`'s `plan_xspice_auto_bridges`, the single
//! planner a connect-module route extends — and neither needs anything from
//! this host.
//!
//! # Where the connect-module route stands
//!
//! Verilog-AMS LRM 2.4 clause 7's decisions — which discipline a net resolves
//! to, which connect module a mixed-discipline connection needs, where the
//! instance goes and how its ports bind — are made by
//! [`rspice_veriloga::connect`], on a signal's net-segment hierarchy. What is
//! missing between there and the planner named above is the *hierarchy*: this
//! host runs a digital design and [`MixedSignalHost`] bridges a fixed
//! boundary, and neither elaborates a Verilog-AMS module tree into
//! `CircuitData` nodes. Until one does, a resolved connect module has no node
//! to be planned onto, which is why the planner has no connect-module input
//! rather than an empty one.
//!
//! [`DigitalHost::advance_to`]: host::DigitalHost::advance_to

pub(crate) mod host;
mod mixed;
pub(crate) mod store;
#[cfg(test)]
mod tests;

use std::sync::Arc;

use rspice_veriloga::canonical_ir::digital::CanonicalDigitalPlan;
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::four_state::FourStateBit;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

use crate::xspice::event_scheduler::{SchedulerLimits, TimeResolution};

use host::DigitalHost;
pub use host::DigitalRunError;
pub use mixed::{MixedSignalCheckpoint, MixedSignalError, MixedSignalHost};

/// What one tick of the digital host's clock is, and why it is not read from
/// the source.
///
/// **One tick is one nanosecond, and one Verilog time unit is one tick.**
///
/// IEEE 1364-2005 section 19.8 makes the time unit a property of the
/// `` `timescale `` directive in effect at each module, and section 19.9 lets a
/// design mix several. Honouring that needs two things this compiler does not
/// have: a preprocessor that reads the directive, and a per-module scale factor
/// carried down to every `#delay` so that a `#1` in a `1ns/1ns` module and a
/// `#1` in a `1ps/1ps` module become different tick counts. The canonical IR
/// carries the delay as a bare integer of *time units*
/// ([`DigitalWaitRequest::Delay`]) and there is nowhere in it to put the
/// factor.
///
/// So the directive is refused by name — [`DigitalRunError::TimescaleDirective`]
/// — rather than read and ignored, which would silently scale every delay in a
/// `1ps` design by a thousand.
///
/// One nanosecond is the choice because it is what the oracle harness's own
/// generated testbench declares (`` `timescale 1ns/1ns ``), and RSpice owns
/// that testbench. A unit equal to the precision also makes `#N` exactly `N`
/// ticks, with no rounding to argue about. The kernel's grid is exact to
/// `2^51 - 1` ticks, which at this resolution is about twenty-six days of
/// simulated time.
///
/// [`DigitalWaitRequest::Delay`]: rspice_veriloga::canonical_ir::digital_eval::DigitalWaitRequest::Delay
/// Crate-private because a caller does not need the string: the refusal that
/// cites it — [`DigitalRunError::TimescaleDirective`] — prints it, and the
/// documentation above is what a reader comparing this against another
/// simulator actually has to read.
pub(crate) const TIME_UNIT_RULING: &str = "one tick is 1 ns; one Verilog time unit is one tick; \
                                           a `timescale directive is refused rather than applied";

/// Decimal exponent of the tick, as [`TimeResolution`] spells it.
const TIME_UNIT_EXPONENT: i8 = -9;

/// One port of the design, as a stimulus names it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalPort {
    /// The port's name in the design.
    pub name: String,
    /// Declared width in bits. A scalar is one, and **zero is a real-valued
    /// (`wreal`) port**, which Verilog-AMS LRM 2.4 section 3.7 gives no bits at
    /// all.
    ///
    /// Zero rather than an `Option` or a second field, because the compiler
    /// already spells "has no bit width" that way — a process-local `real` and
    /// a real net both carry width zero through the whole front end — and two
    /// spellings of one fact are two chances for them to disagree.
    ///
    /// It is not what decides the port's domain. The *design* decides: the
    /// net-type keyword its author wrote is read back from the compiled plan,
    /// and a stimulus whose width says otherwise is refused with
    /// [`DigitalRunError::StimulusValueDomain`] rather than believed.
    pub width: u32,
}

/// A free-running clock the stimulus drives rather than the vectors.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalClock {
    /// The input port carrying the clock.
    pub port: String,
    /// Half period, in time units.
    pub half_period: u64,
}

/// Everything a caller supplies alongside the source.
///
/// The timing rule is the oracle harness's, restated so that this crate does
/// not have to name the harness's types: inputs for vector `k` are applied at
/// `k * step`, and outputs are sampled at `k * step + settle`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalStimulus {
    /// Which module of the source to run, for a multi-module file.
    pub module: Option<String>,
    /// Inputs the vector columns drive, in column order. The clock is not one
    /// of these.
    pub inputs: Vec<DigitalPort>,
    /// Ports sampled into the trace, in the order they appear in an
    /// observation.
    pub outputs: Vec<DigitalPort>,
    /// A clock the stimulus drives, if the design has one.
    pub clock: Option<DigitalClock>,
    /// Time units between successive vectors.
    pub step: u64,
    /// Time units after a vector is applied at which outputs are sampled.
    pub settle: u64,
    /// One entry per vector; each is one four-state spelling per driven input,
    /// most significant bit first, exactly as wide as the port.
    pub vectors: Vec<Vec<String>>,
}

/// One sample of every observed output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalObservation {
    /// Zero-based vector index this observation belongs to.
    pub step: usize,
    /// Output name and its rendered value, in stimulus order.
    ///
    /// A four-state port renders as its `%b` spelling over `0 1 x z`. A real
    /// port renders as a decimal that reads back to the same `f64` exactly —
    /// Rust's shortest round-tripping form — so a trace can be compared
    /// textually without a tolerance deciding what counts as equal.
    pub values: Vec<(String, String)>,
}

/// The result of one digital run.
///
/// # One observation per vector, and what that is not
///
/// The resolution is the stimulus's, not the host's. A design may change a
/// signal many times between two sample instants — a `#delay` chain, a clock
/// the stimulus drives, a delta cycle — and none of it appears here; what
/// appears is the value at `k * step + settle`, which is what a `.stim`-style
/// harness asks for.
///
/// A caller that wants the transitions themselves has no route to them today.
/// The compile-once split does not open one: [`CompiledDigitalDesign::run`]
/// builds and drops its host inside the call, so there is nothing left for a
/// reader to read afterwards, and a per-tick observation would have to be
/// collected *during* the run — a callback or a signal subscription on the
/// stimulus, not an accessor on this. That is a change to what a run is asked
/// for rather than a change to what it returns, so it is left undone and
/// recorded here rather than approximated with a finer `step`, which would
/// change what the design sees as well as what the caller sees.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalRunReport {
    /// One observation per vector, in order.
    pub observations: Vec<DigitalObservation>,
}

/// One digital design, compiled once and runnable any number of times.
///
/// # Why the split exists
///
/// [`run_digital_verilog`] runs the Verilog front end on every call. For a
/// caller with one stimulus that is the honest cost of the run, and it is what
/// the convenience call still charges. For a caller with many — a sweep, a
/// regression harness, a measurement that wants to know what *evaluating* a
/// real-number model costs — it is a compile repeated for nothing, and at
/// realistic run lengths it is most of the wall time. `.../tests/
/// verilog_rnm_performance.rs` measures both columns side by side, which is the
/// evidence this split was worth making.
///
/// # What is shared between runs, and what is not
///
/// Shared: the compiled plan, which is immutable — it is the front end's output
/// and nothing downstream writes to it. Not shared: **every** piece of running
/// state. [`Self::run`] builds a fresh [`DigitalHost`] — signal store, event
/// queue, process resumption slots, sensitivity index — for each call and drops
/// it when the call returns, so there is no state for one run to leave behind
/// for the next and no way for two designs' runs to interleave into each other.
/// That is a structural property rather than a discipline: the only thing a run
/// can reach across a call boundary is behind an `Arc` it holds by shared
/// reference.
///
/// # The relationship to [`MixedSignalHost`]
///
/// [`MixedSignalHost::compile`] is already compile-once by construction — it
/// keeps its `DigitalHost` for the whole of an outer transient, because the
/// analog solver drives it one trial at a time and the digital state has to
/// survive between trials. This type gives the vector-driven route the same
/// shape for the opposite reason: its state must *not* survive between runs.
/// The two meet at [`DigitalHost::from_plan`], which is the one place a
/// compiled plan becomes a running host; neither builds its own.
pub struct CompiledDigitalDesign {
    /// The front end's output, shared by every host built from it.
    plan: Arc<CanonicalDigitalPlan>,
    /// The module that was compiled, so a stimulus naming another one is
    /// refused rather than silently run against this.
    module: String,
    /// Fixed here rather than per run, so two runs of one design cannot be on
    /// different time bases. See [`TIME_UNIT_RULING`].
    resolution: TimeResolution,
}

impl CompiledDigitalDesign {
    /// Compile one module of a digital Verilog source.
    ///
    /// Every refusal [`run_digital_verilog`] makes before anything runs is made
    /// here, in the same order: the `` `timescale `` scan, the front end, and
    /// the two checks that the module is digital and only digital.
    pub fn compile(source: &str, module: Option<&str>) -> Result<Self, DigitalRunError> {
        if let Some(line) = first_timescale_directive(source) {
            return Err(DigitalRunError::TimescaleDirective { line });
        }

        let compiler = VerilogACompiler::new(CompilerOptions::default());
        let artifact = compiler
            .compile_canonical_ir_module(source, module)
            .map_err(|error| DigitalRunError::Compile {
                detail: error.to_string(),
            })?;

        if artifact.digital.is_empty() {
            return Err(DigitalRunError::NoDigitalContent {
                module: artifact.mir.module_name.to_string(),
            });
        }
        if !artifact.mir.equations.is_empty() {
            return Err(DigitalRunError::MixedSignalModule {
                module: artifact.mir.module_name.to_string(),
                equations: artifact.mir.equations.len(),
            });
        }

        Ok(Self {
            module: artifact.mir.module_name.to_string(),
            plan: Arc::new(artifact.digital),
            resolution: TimeResolution::new(TIME_UNIT_EXPONENT)?,
        })
    }

    /// Run one stimulus against this design.
    ///
    /// Nothing of a previous run reaches this one: the host is built here and
    /// dropped at the end, so the design starts at time zero with every process
    /// unstarted and every signal at its declared initial value, exactly as a
    /// fresh [`run_digital_verilog`] would.
    pub fn run(&self, stimulus: &DigitalStimulus) -> Result<DigitalRunReport, DigitalRunError> {
        if let Some(requested) = stimulus.module.as_deref()
            && requested != self.module
        {
            return Err(DigitalRunError::StimulusModule {
                compiled: self.module.clone(),
                requested: requested.to_string(),
            });
        }
        self.execute(stimulus)
    }
}

/// What the design is and how big it is, not what it contains.
///
/// Written out rather than derived because the derived form is the whole
/// canonical plan — every process's control-flow graph — which is what a
/// compiler dump is for and not what a failing assertion should print.
impl std::fmt::Debug for CompiledDigitalDesign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CompiledDigitalDesign")
            .field("module", &self.module)
            .field("signals", &self.plan.signals.len())
            .field("processes", &self.plan.processes.len())
            .finish()
    }
}

/// Compile digital Verilog and run it against a stimulus.
///
/// The whole route, in one call: preprocess and compile the source to a
/// canonical digital plan, build a signal store and a process host over it,
/// start every process at time zero, then walk the stimulus applying inputs and
/// sampling outputs on the timing rule [`DigitalStimulus`] states.
///
/// It is the composition of [`CompiledDigitalDesign::compile`] and
/// [`CompiledDigitalDesign::run`] and nothing else, so a caller with one
/// stimulus keeps the one-call route and a caller with many can hoist the
/// compile out of its loop.
pub fn run_digital_verilog(
    source: &str,
    stimulus: &DigitalStimulus,
) -> Result<DigitalRunReport, DigitalRunError> {
    CompiledDigitalDesign::compile(source, stimulus.module.as_deref())?.execute(stimulus)
}

impl CompiledDigitalDesign {
    /// The run itself, with the module agreement already decided.
    ///
    /// Split from [`Self::run`] so that [`run_digital_verilog`], which compiled
    /// the module the stimulus named and therefore cannot disagree with it,
    /// does not pay a string comparison to prove it.
    fn execute(&self, stimulus: &DigitalStimulus) -> Result<DigitalRunReport, DigitalRunError> {
        let mut host = DigitalHost::from_plan(
            Arc::clone(&self.plan),
            self.resolution,
            SchedulerLimits::default(),
        );
        host.start()?;

        // Resolve every name once. A stimulus naming a port the design does not
        // declare is a mistake in the caller, and finding it before anything runs
        // makes the diagnostic name the port rather than a time.
        //
        // The design is asked what each port carries, and the stimulus is checked
        // against that answer rather than trusted for it: a `.stim` that has
        // drifted away from its design would otherwise drive a real net with bits
        // and produce a trace that looks like a run.
        let inputs = stimulus
            .inputs
            .iter()
            .map(|port| resolve_port(&host, port))
            .collect::<Result<Vec<_>, DigitalRunError>>()?;
        let outputs = stimulus
            .outputs
            .iter()
            .map(|port| resolve_port(&host, port))
            .collect::<Result<Vec<_>, DigitalRunError>>()?;
        let clock = stimulus
            .clock
            .as_ref()
            .map(|clock| Ok::<_, DigitalRunError>((host.signal(&clock.port)?, clock.half_period)))
            .transpose()?;

        let mut observations = Vec::with_capacity(stimulus.vectors.len());
        let mut clock_level = FourStateBit::Zero;
        let mut next_clock_edge = clock.map(|(_, half)| half);

        // `initial clk = 1'b0;` — the level the free-running clock holds before its
        // first edge, which is what the generated testbench declares.
        if let Some((signal, _)) = clock {
            host.force(signal, FourStateValue::splat(1, FourStateBit::Zero), 0)?;
        }

        for (index, vector) in stimulus.vectors.iter().enumerate() {
            let apply_at = (index as u64)
                .checked_mul(stimulus.step)
                .ok_or(DigitalRunError::TickOverflow)?;
            let sample_at = apply_at
                .checked_add(stimulus.settle)
                .ok_or(DigitalRunError::TickOverflow)?;

            // The clock is advanced first at a tick both it and a vector fall on.
            // Two `initial`/`always` blocks writing different signals at one time
            // are a race IEEE 1364-2005 leaves to the simulator; fixing the order
            // here makes the run reproducible, and the corpus's clocked designs are
            // edge-triggered on the rising edge, which never coincides with a
            // vector boundary under `step = 2 * half_period`.
            if let Some((signal, half)) = clock {
                while let Some(edge) = next_clock_edge {
                    if edge > apply_at {
                        break;
                    }
                    host.advance_to(edge)?;
                    clock_level = invert(clock_level);
                    host.force(signal, FourStateValue::splat(1, clock_level), edge)?;
                    next_clock_edge = Some(
                        edge.checked_add(half)
                            .ok_or(DigitalRunError::TickOverflow)?,
                    );
                }
            }

            host.advance_to(apply_at)?;
            for (port, spelling) in inputs.iter().zip(vector) {
                let ResolvedPort {
                    signal,
                    width,
                    name,
                } = port;
                if *width == 0 {
                    let value: f64 =
                        spelling
                            .parse()
                            .map_err(|_| DigitalRunError::RealSpelling {
                                port: name.clone(),
                                spelling: spelling.clone(),
                            })?;
                    host.force_real(*signal, value, apply_at)?;
                    continue;
                }
                let value =
                    parse_four_state(spelling).ok_or_else(|| DigitalRunError::VectorSpelling {
                        port: name.clone(),
                        spelling: spelling.clone(),
                    })?;
                if value.width() != *width {
                    return Err(DigitalRunError::VectorWidth {
                        port: name.clone(),
                        declared: *width,
                        offered: value.width(),
                    });
                }
                host.force(*signal, value, apply_at)?;
            }

            // Sample after the settle, with every clock edge in between delivered.
            if let Some((signal, half)) = clock {
                while let Some(edge) = next_clock_edge {
                    if edge > sample_at {
                        break;
                    }
                    host.advance_to(edge)?;
                    clock_level = invert(clock_level);
                    host.force(signal, FourStateValue::splat(1, clock_level), edge)?;
                    next_clock_edge = Some(
                        edge.checked_add(half)
                            .ok_or(DigitalRunError::TickOverflow)?,
                    );
                }
            }
            host.advance_to(sample_at)?;

            observations.push(DigitalObservation {
                step: index,
                values: outputs
                    .iter()
                    .map(|port| {
                        let rendered = if port.width == 0 {
                            host.read_real(port.signal)
                                .map(render_real)
                                .unwrap_or_default()
                        } else {
                            host.read(port.signal)
                                .map(FourStateValue::spelling)
                                .unwrap_or_default()
                        };
                        (port.name.clone(), rendered)
                    })
                    .collect(),
            });
        }

        Ok(DigitalRunReport { observations })
    }
}

/// One stimulus port, resolved against the compiled design.
struct ResolvedPort {
    signal: rspice_veriloga::canonical_ir::ids::DigitalSignalId,
    /// The design's answer, not the stimulus's: zero for a real net.
    width: u32,
    name: String,
}

/// Resolve one stimulus port and check that the two agree about its domain.
fn resolve_port(host: &DigitalHost, port: &DigitalPort) -> Result<ResolvedPort, DigitalRunError> {
    let signal = host.signal(&port.name)?;
    let real = host.is_real(signal);
    if real != (port.width == 0) {
        return Err(DigitalRunError::StimulusValueDomain {
            name: port.name.clone(),
            port_is_real: real,
        });
    }
    Ok(ResolvedPort {
        signal,
        width: port.width,
        name: port.name.clone(),
    })
}

/// Render a real for a trace column.
///
/// `{:?}` rather than `{}`, which are different for an `f64`: both round-trip,
/// and only the debug form keeps the decimal point that says the column is a
/// real. `1.0` printing as `1` would be indistinguishable from a one-bit port.
fn render_real(value: f64) -> String {
    format!("{value:?}")
}

fn invert(bit: FourStateBit) -> FourStateBit {
    match bit {
        FourStateBit::Zero => FourStateBit::One,
        FourStateBit::One => FourStateBit::Zero,
        other => other,
    }
}

/// Read a `%b`-style four-state spelling, most significant bit first.
fn parse_four_state(spelling: &str) -> Option<FourStateValue> {
    if spelling.is_empty() {
        return None;
    }
    let mut bits = Vec::with_capacity(spelling.len());
    for character in spelling.chars() {
        bits.push(match character {
            '0' => FourStateBit::Zero,
            '1' => FourStateBit::One,
            'x' | 'X' => FourStateBit::Unknown,
            'z' | 'Z' | '?' => FourStateBit::HighImpedance,
            _ => return None,
        });
    }
    Some(FourStateValue::from_bits_msb_first(&bits))
}

/// The first `` `timescale `` directive in the source, if any.
///
/// A textual scan of line starts, deliberately conservative: the directive is
/// only legal at the start of a line outside a comment, and this host must
/// refuse it rather than let the lexer decide what to do with a backtick it
/// does not recognise. A false positive costs a refusal on a source that names
/// `timescale` at the start of a line after a backtick, which is the directive.
fn first_timescale_directive(source: &str) -> Option<usize> {
    let mut in_block_comment = false;
    for (index, raw) in source.lines().enumerate() {
        let mut line = raw;
        if in_block_comment {
            match line.find("*/") {
                Some(end) => {
                    in_block_comment = false;
                    line = &line[end + 2..];
                }
                None => continue,
            }
        }
        let code = line.split("//").next().unwrap_or_default();
        if let Some(start) = code.find("/*")
            && !code[start..].contains("*/")
        {
            in_block_comment = true;
        }
        let trimmed = code.trim_start();
        if let Some(rest) = trimmed.strip_prefix('`')
            && rest.trim_start().starts_with("timescale")
        {
            return Some(index + 1);
        }
    }
    None
}

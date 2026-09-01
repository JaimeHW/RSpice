//! Native execution of digital IEEE 1364-2005 Verilog.
//!
//! A caller hands this module Verilog source and a stimulus and gets back one
//! observation per vector. Everything between — compiling the source to a
//! [`CanonicalDigitalPlan`], resolving its nets, running its processes on the
//! event kernel — is here or in the two submodules below it.
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
//! The boundary is explicit: scalar ADC bridges publish accepted analog
//! samples with hysteresis, and scalar DAC bridges stamp Thevenin equivalents.
//! Vector and bidirectional coercions remain fail-closed until their resolution
//! semantics are represented directly.
//!
//! # Where a `wreal` meets an analog node
//!
//! Not here yet, and the boundary's rulings are recorded here because they
//! have a right answer worth writing down before somebody guesses one. The
//! mixed host above does not settle them: it *refuses* any trial time off its
//! integer-nanosecond grid, which dissolves the time-translation question
//! rather than answering it, and an LTE-controlled transient does not land on
//! integer nanoseconds — so the general boundary still needs exactly what
//! follows.
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
//! which property to keep, and there is one answer that keeps the right ones:
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
//! The bridge halves already exist as code models — `real_to_v` (planned in by
//! `engine::builder`'s `plan_xspice_auto_bridges`, the single planner a
//! connect-module route extends) and `v_to_real` (sample on accepted step, no
//! threshold, no breakpoint) — and neither needs anything from this host.
//!
//! [`DigitalHost::advance_to`]: host::DigitalHost::advance_to

pub(crate) mod host;
mod mixed;
pub(crate) mod store;
#[cfg(test)]
mod tests;

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DigitalRunReport {
    /// One observation per vector, in order.
    pub observations: Vec<DigitalObservation>,
}

/// Compile digital Verilog and run it against a stimulus.
///
/// The whole route, in one call: preprocess and compile the source to a
/// canonical digital plan, build a signal store and a process host over it,
/// start every process at time zero, then walk the stimulus applying inputs and
/// sampling outputs on the timing rule [`DigitalStimulus`] states.
pub fn run_digital_verilog(
    source: &str,
    stimulus: &DigitalStimulus,
) -> Result<DigitalRunReport, DigitalRunError> {
    if let Some(line) = first_timescale_directive(source) {
        return Err(DigitalRunError::TimescaleDirective { line });
    }

    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let artifact = compiler
        .compile_canonical_ir_module(source, stimulus.module.as_deref())
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

    let plan = &artifact.digital;
    let resolution = TimeResolution::new(TIME_UNIT_EXPONENT)?;
    let mut host = DigitalHost::new(plan, resolution, SchedulerLimits::default());
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
                let value: f64 = spelling
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

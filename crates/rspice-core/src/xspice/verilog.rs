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
    /// Declared width in bits. A scalar is one.
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
    /// Output name and its four-state spelling, in stimulus order.
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
    let inputs = stimulus
        .inputs
        .iter()
        .map(|port| Ok((host.signal(&port.name)?, port.width, port.name.clone())))
        .collect::<Result<Vec<_>, DigitalRunError>>()?;
    let outputs = stimulus
        .outputs
        .iter()
        .map(|port| Ok((host.signal(&port.name)?, port.name.clone())))
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
        for ((signal, width, name), spelling) in inputs.iter().zip(vector) {
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
                .map(|(signal, name)| {
                    let spelling = host
                        .read(*signal)
                        .map(FourStateValue::spelling)
                        .unwrap_or_default();
                    (name.clone(), spelling)
                })
                .collect(),
        });
    }

    Ok(DigitalRunReport { observations })
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

//! Playback substrate for the PSpice `U<name> STIM(...)` digital stimulus.
//!
//! The netlist front end compiles a `STIM` command list into a canonical
//! *stimulus program* string and attaches it to a `pspice_d_stim` code-model
//! instance. This module owns the other half of that contract: it decodes the
//! program, expands it into a transition table bounded by the transient stop
//! time, and replays the table onto an event-driven digital output vector.
//!
//! Keeping the loop expansion here rather than in the parser is what makes
//! `GOTO <label> -1 TIMES` (repeat forever) representable at all: the parser
//! cannot know `tstop`, so a forever loop has no finite table until run time.
//!
//! # Program grammar
//!
//! ```text
//! program := "W" width ( " " instruction )*
//! instruction := "V" ":" origin ":" time ":" bits      -- drive a bus value
//!              | "G" ":" origin ":" time ":" target ":" count  -- goto
//!              | "I" ":" origin ":" time ":" step      -- increment value
//!              | "D" ":" origin ":" time ":" step      -- decrement value
//!              | "P" ":" count                          -- repeat block start
//!              | "E"                                    -- repeat block end
//! origin := "A" (absolute time) | "R" (relative to the previous command)
//! time := seconds, Rust `{:?}` float formatting
//! bits := exactly `width` characters drawn from `0`, `1`, `X`, `Z`, MSB first
//! target := instruction index, already resolved from the source `LABEL=`
//! count := iteration count; `-1` means forever
//! ```
//!
//! The grammar is generated, never authored: every diagnostic below reports a
//! front-end/back-end contract break, not a user mistake. User mistakes are
//! rejected with typed parse errors before a program is ever emitted.

use super::*;
use crate::Value;
use crate::xspice::{CmError, EvaluationPhase};
use std::sync::{Arc, OnceLock};

/// PSpice `STIM` digital stimulus playback.
#[derive(Debug, Default)]
pub(crate) struct PspiceDigitalStimulus;

/// Instance parameter carrying the compiled stimulus program.
///
/// The netlist front end spells this name literally: `netlist` sits below
/// `xspice` in the layer order and may not read a constant from it.
const STIM_PROGRAM_PARAM: &str = "stim_program";

const STIM_ROWS_RESOURCE: &str = "xspice.pspice_d_stim.rows";
const STIM_NO_ROW: i64 = -1;
const STIM_BEFORE_FIRST_ROW: i64 = -2;
const STIM_EMITTED_ROW: usize = 0;
const STIM_SCHEDULED_ROW: usize = 1;
const STIM_TIME_EPSILON: Value = 1e-18;

/// Interpreter step budget beyond the emitted-row budget.
///
/// A program whose loop body advances time by zero would otherwise spin: the
/// horizon check never trips because time never moves. The budget converts
/// that into a diagnostic instead of a hang. It is generous enough that no
/// time-advancing program reaches it before the row cap does.
const STIM_STEP_BUDGET_SLACK: usize = 4096;

//=============================================================================
// Program representation
//=============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StimOrigin {
    /// Absolute time in seconds.
    Absolute,
    /// Offset from the previously executed command's time.
    Relative,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct StimTime {
    origin: StimOrigin,
    seconds: Value,
}

impl StimTime {
    fn resolve(self, current: Value) -> Value {
        match self.origin {
            StimOrigin::Absolute => self.seconds,
            StimOrigin::Relative => current + self.seconds,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum StimInstruction {
    /// Drive the output bus with an explicit per-bit pattern.
    Drive { time: StimTime, bits: Vec<StimBit> },
    /// Jump to `target`, at most `count` times (`-1` meaning forever).
    Goto {
        time: StimTime,
        target: usize,
        count: i64,
    },
    /// Add `step` to the bus interpreted as an unsigned integer.
    Increment { time: StimTime, step: u64 },
    /// Subtract `step` from the bus interpreted as an unsigned integer.
    Decrement { time: StimTime, step: u64 },
    /// Open a repeat block iterating `count` times (`-1` meaning forever).
    Repeat { count: i64 },
    /// Close the innermost repeat block.
    EndRepeat,
}

/// One decoded value character position of the output bus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StimBit {
    Zero,
    One,
    Unknown,
    HighZ,
}

impl StimBit {
    fn from_program_char(character: char) -> Option<Self> {
        match character {
            '0' => Some(Self::Zero),
            '1' => Some(Self::One),
            'X' => Some(Self::Unknown),
            'Z' => Some(Self::HighZ),
            _ => None,
        }
    }

    fn digital_value(self) -> DigitalValue {
        match self {
            Self::Zero => DigitalValue::zero(),
            Self::One => DigitalValue::one(),
            Self::Unknown => DigitalValue::unknown(),
            Self::HighZ => DigitalValue::high_z(),
        }
    }

    /// Numeric weight for `INCR BY` / `DECR BY`, absent for non-numeric bits.
    fn numeric_bit(self) -> Option<u64> {
        match self {
            Self::Zero => Some(0),
            Self::One => Some(1),
            Self::Unknown | Self::HighZ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct StimProgram {
    width: usize,
    instructions: Vec<StimInstruction>,
}

//=============================================================================
// Program decoding
//=============================================================================

fn stim_program_error(message: impl Into<String>) -> CmError {
    CmError::EvaluationError(format!("pspice_d_stim: {}", message.into()))
}

fn decode_program(program: &str) -> CmResult<StimProgram> {
    let mut tokens = program.split_ascii_whitespace();
    let header = tokens
        .next()
        .ok_or_else(|| stim_program_error("stimulus program is empty"))?;
    let width = header
        .strip_prefix('W')
        .and_then(|digits| digits.parse::<usize>().ok())
        .ok_or_else(|| {
            stim_program_error(format!(
                "stimulus program must start with a 'W<width>' header, found '{header}'"
            ))
        })?;
    if width == 0 {
        return Err(stim_program_error("stimulus program declares zero outputs"));
    }

    let mut instructions = Vec::new();
    for token in tokens {
        instructions.push(decode_instruction(token, width)?);
    }
    Ok(StimProgram {
        width,
        instructions,
    })
}

fn decode_instruction(token: &str, width: usize) -> CmResult<StimInstruction> {
    let mut fields = token.split(':');
    let opcode = fields
        .next()
        .ok_or_else(|| stim_program_error(format!("empty stimulus instruction '{token}'")))?;
    let malformed = || stim_program_error(format!("malformed stimulus instruction '{token}'"));

    let instruction = match opcode {
        "V" => {
            let time = decode_time(fields.next().ok_or_else(malformed)?, fields.next())?;
            let bits = decode_bits(fields.next().ok_or_else(malformed)?, width)?;
            StimInstruction::Drive { time, bits }
        }
        "G" => {
            let time = decode_time(fields.next().ok_or_else(malformed)?, fields.next())?;
            let target = fields
                .next()
                .and_then(|value| value.parse::<usize>().ok())
                .ok_or_else(malformed)?;
            let count = fields
                .next()
                .and_then(|value| value.parse::<i64>().ok())
                .ok_or_else(malformed)?;
            StimInstruction::Goto {
                time,
                target,
                count,
            }
        }
        "I" | "D" => {
            let time = decode_time(fields.next().ok_or_else(malformed)?, fields.next())?;
            let step = fields
                .next()
                .and_then(|value| value.parse::<u64>().ok())
                .ok_or_else(malformed)?;
            if opcode == "I" {
                StimInstruction::Increment { time, step }
            } else {
                StimInstruction::Decrement { time, step }
            }
        }
        "P" => {
            let count = fields
                .next()
                .and_then(|value| value.parse::<i64>().ok())
                .ok_or_else(malformed)?;
            StimInstruction::Repeat { count }
        }
        "E" => StimInstruction::EndRepeat,
        other => {
            return Err(stim_program_error(format!(
                "unknown stimulus opcode '{other}' in '{token}'"
            )));
        }
    };
    if fields.next().is_some() {
        return Err(malformed());
    }
    Ok(instruction)
}

fn decode_time(origin: &str, seconds: Option<&str>) -> CmResult<StimTime> {
    let origin = match origin {
        "A" => StimOrigin::Absolute,
        "R" => StimOrigin::Relative,
        other => {
            return Err(stim_program_error(format!(
                "stimulus time origin must be 'A' or 'R', found '{other}'"
            )));
        }
    };
    let seconds = seconds
        .and_then(|value| value.parse::<Value>().ok())
        .filter(|value| value.is_finite())
        .ok_or_else(|| stim_program_error("stimulus time must be a finite number of seconds"))?;
    Ok(StimTime { origin, seconds })
}

fn decode_bits(bits: &str, width: usize) -> CmResult<Vec<StimBit>> {
    let decoded = bits
        .chars()
        .map(StimBit::from_program_char)
        .collect::<Option<Vec<_>>>()
        .ok_or_else(|| {
            stim_program_error(format!(
                "stimulus value '{bits}' contains a character outside 0/1/X/Z"
            ))
        })?;
    if decoded.len() != width {
        return Err(stim_program_error(format!(
            "stimulus value '{bits}' has {} bits but the bus is {width} wide",
            decoded.len()
        )));
    }
    Ok(decoded)
}

//=============================================================================
// Expansion
//=============================================================================

#[derive(Debug, Clone, PartialEq)]
struct StimRows {
    width: usize,
    times: Vec<Value>,
    values: Vec<DigitalValue>,
}

impl StimRows {
    fn row_values(&self, index: usize) -> &[DigitalValue] {
        let start = index * self.width;
        &self.values[start..start + self.width]
    }
}

#[derive(Debug)]
struct StimRowsResource {
    program: String,
    horizon: Value,
    rows: Arc<StimRows>,
}

/// Expand `program` into every transition at or before `horizon`.
///
/// `max_rows` bounds the table; a program that would emit more is a resource
/// refusal, not a truncation, so a deck never silently loses stimulus.
fn expand_program(program: &StimProgram, horizon: Value, max_rows: usize) -> CmResult<StimRows> {
    let width = program.width;
    let mut rows = StimRows {
        width,
        times: Vec::new(),
        values: Vec::new(),
    };
    let mut bits = vec![StimBit::Unknown; width];
    let mut time = 0.0;
    let mut pc = 0usize;
    let mut goto_taken = vec![0i64; program.instructions.len()];
    let mut repeat_stack: Vec<(usize, i64)> = Vec::new();
    let budget = max_rows.saturating_add(STIM_STEP_BUDGET_SLACK);

    for step in 0..=budget {
        if step == budget {
            return Err(stim_program_error(
                "stimulus program exceeded its interpreter budget without reaching the transient \
                 stop time; a loop body that advances time by zero cannot terminate",
            ));
        }
        let Some(instruction) = program.instructions.get(pc) else {
            break;
        };
        match instruction {
            StimInstruction::Drive {
                time: at,
                bits: pattern,
            } => {
                let next = at.resolve(time);
                if next > horizon + STIM_TIME_EPSILON {
                    break;
                }
                time = next;
                bits.clone_from(pattern);
                push_row(&mut rows, time, &bits, max_rows)?;
                pc += 1;
            }
            StimInstruction::Increment { time: at, step } => {
                let next = at.resolve(time);
                if next > horizon + STIM_TIME_EPSILON {
                    break;
                }
                time = next;
                bits = stepped_bits(&bits, *step, true);
                push_row(&mut rows, time, &bits, max_rows)?;
                pc += 1;
            }
            StimInstruction::Decrement { time: at, step } => {
                let next = at.resolve(time);
                if next > horizon + STIM_TIME_EPSILON {
                    break;
                }
                time = next;
                bits = stepped_bits(&bits, *step, false);
                push_row(&mut rows, time, &bits, max_rows)?;
                pc += 1;
            }
            StimInstruction::Goto {
                time: at,
                target,
                count,
            } => {
                let next = at.resolve(time);
                if next > horizon + STIM_TIME_EPSILON {
                    break;
                }
                time = next;
                let taken = goto_taken.get_mut(pc).ok_or_else(|| {
                    stim_program_error("stimulus GOTO counter is missing for its instruction")
                })?;
                if *count < 0 || *taken < *count {
                    *taken = taken.saturating_add(1);
                    pc = *target;
                } else {
                    pc += 1;
                }
            }
            StimInstruction::Repeat { count } => {
                repeat_stack.push((pc, *count));
                pc += 1;
            }
            StimInstruction::EndRepeat => {
                let (repeat_pc, remaining) = repeat_stack.pop().ok_or_else(|| {
                    stim_program_error("stimulus ENDREPEAT has no matching REPEAT")
                })?;
                if remaining < 0 {
                    repeat_stack.push((repeat_pc, remaining));
                    pc = repeat_pc + 1;
                } else if remaining > 1 {
                    repeat_stack.push((repeat_pc, remaining - 1));
                    pc = repeat_pc + 1;
                } else {
                    pc += 1;
                }
            }
        }
    }
    Ok(rows)
}

fn push_row(rows: &mut StimRows, time: Value, bits: &[StimBit], max_rows: usize) -> CmResult<()> {
    if rows.times.len() >= max_rows {
        return Err(stim_program_error(format!(
            "stimulus program expands past the {max_rows}-transition resource limit"
        )));
    }
    rows.times.try_reserve(1).map_err(|error| {
        stim_program_error(format!("unable to reserve a stimulus transition: {error}"))
    })?;
    rows.values.try_reserve(bits.len()).map_err(|error| {
        stim_program_error(format!(
            "unable to reserve stimulus transition values: {error}"
        ))
    })?;
    rows.times.push(time);
    rows.values
        .extend(bits.iter().map(|bit| bit.digital_value()));
    Ok(())
}

/// Apply `INCR BY` / `DECR BY` to the bus read as an MSB-first unsigned integer.
///
/// PSpice defines the stepping only over numeric bus values. A bus carrying any
/// `X` or `Z` bit has no integer to step, so the whole bus becomes unknown —
/// the same fail-visible choice the digital primitives make when an operand is
/// unknown, rather than inventing a value for the undefined bits.
fn stepped_bits(bits: &[StimBit], step: u64, increment: bool) -> Vec<StimBit> {
    let width = bits.len();
    let Some(current) = bits.iter().try_fold(0u64, |accumulator, bit| {
        bit.numeric_bit().map(|value| (accumulator << 1) | value)
    }) else {
        return vec![StimBit::Unknown; width];
    };
    let modulus = if width >= 64 {
        u64::MAX
    } else {
        (1u64 << width) - 1
    };
    let next = if increment {
        current.wrapping_add(step)
    } else {
        current.wrapping_sub(step)
    } & modulus;
    (0..width)
        .map(|index| {
            let shift = width - 1 - index;
            if (next >> shift) & 1 == 1 {
                StimBit::One
            } else {
                StimBit::Zero
            }
        })
        .collect()
}

//=============================================================================
// Run-time playback
//=============================================================================

fn stim_program_param(ctx: &CmContext) -> &str {
    ctx.string_param(STIM_PROGRAM_PARAM).unwrap_or("")
}

/// Row budget for one instance, derived from the engine's resource policy.
fn stim_max_rows(ctx: &CmContext) -> usize {
    ctx.resource_limits().max_external_data_values.max(1)
}

/// Expansion horizon.
///
/// Outside `.tran` there is no future to expand into: only the transitions at
/// or before time zero can affect an operating point or a DC sweep.
fn stim_horizon(ctx: &CmContext) -> Value {
    ctx.transient_stop_time().unwrap_or(0.0)
}

fn load_rows(ctx: &mut CmContext) -> CmResult<Arc<StimRows>> {
    let program = stim_program_param(ctx).to_string();
    let horizon = stim_horizon(ctx);
    if let Some(resource) = ctx.resource::<StimRowsResource>(STIM_ROWS_RESOURCE)
        && resource.program == program
        && resource.horizon == horizon
    {
        return Ok(resource.rows.clone());
    }

    let rows = Arc::new(expand_program(
        &decode_program(&program)?,
        horizon,
        stim_max_rows(ctx),
    )?);
    ctx.set_resource(
        STIM_ROWS_RESOURCE,
        Arc::new(StimRowsResource {
            program,
            horizon,
            rows: rows.clone(),
        }),
    );
    Ok(rows)
}

/// Index of the row in effect at `time`, and of the next row after it.
///
/// The scan restarts from the beginning whenever the emitted cursor is ahead of
/// `time`, so a rejected step that rewinds the clock re-derives the correct row
/// instead of holding a value the circuit has not reached yet.
fn row_indices(times: &[Value], time: Value, emitted_row: i64) -> (Option<usize>, Option<usize>) {
    let start = match usize::try_from(emitted_row)
        .ok()
        .filter(|index| *index < times.len())
    {
        Some(index) if times[index] <= time + STIM_TIME_EPSILON => index.saturating_add(1),
        _ => 0,
    };
    let upper = times[start..].partition_point(|row| *row <= time + STIM_TIME_EPSILON);
    let next = start + upper;
    (next.checked_sub(1), (next < times.len()).then_some(next))
}

fn set_row_state(ctx: &mut CmContext, index: usize, value: i64) {
    if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
        ctx.set_int_state(index, value);
    }
}

fn set_unknown_output(ctx: &mut CmContext, width: usize) -> CmResult<()> {
    let value = DigitalValue::new(DigitalState::Unknown, DigitalStrength::Undetermined);
    ctx.set_output_digital_vector_from_context_fn("out", width, 0.0, |_, _| value)
}

impl CodeModel for PspiceDigitalStimulus {
    fn name(&self) -> &str {
        "pspice_d_stim"
    }

    fn description(&self) -> &str {
        "PSpice STIM digital stimulus"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| vec![PortSpec::vector_output("out", PortType::Digital)])
    }

    fn parameters(&self) -> &[ParamSpec] {
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| vec![ParamSpec::string(STIM_PROGRAM_PARAM, "")])
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_int_states(2);
        ctx.set_int_state(STIM_EMITTED_ROW, STIM_NO_ROW);
        ctx.set_int_state(STIM_SCHEDULED_ROW, STIM_NO_ROW);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let rows = load_rows(ctx)?;
        let width = rows.width;
        let emitted_row = ctx.int_state(STIM_EMITTED_ROW);
        let scheduled_row = ctx.int_state(STIM_SCHEDULED_ROW);
        let (active, next) = row_indices(&rows.times, ctx.time, emitted_row);

        match active {
            Some(index) if emitted_row != index as i64 => {
                let delay = rows.times[index] - ctx.time;
                ctx.set_output_digital_vector_from_slice("out", rows.row_values(index), delay);
                set_row_state(ctx, STIM_EMITTED_ROW, index as i64);
            }
            Some(_) => {}
            None if emitted_row != STIM_BEFORE_FIRST_ROW => {
                set_unknown_output(ctx, width)?;
                set_row_state(ctx, STIM_EMITTED_ROW, STIM_BEFORE_FIRST_ROW);
            }
            None => {}
        }

        match next {
            Some(index) if scheduled_row != index as i64 => {
                let delay = rows.times[index] - ctx.time;
                ctx.set_output_digital_vector_from_slice("out", rows.row_values(index), delay);
                set_row_state(ctx, STIM_SCHEDULED_ROW, index as i64);
                // Asking for the breakpoint as well as publishing it through
                // `transient_breakpoints` keeps the stimulus correct even if the
                // breakpoint pass ever runs without a known stop time. Doing it
                // here rather than on every evaluation makes it one request per
                // row, and a rewind that resets the cursor re-requests.
                if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
                    ctx.request_breakpoint(rows.times[index]);
                }
            }
            Some(_) => {}
            None if scheduled_row != STIM_NO_ROW => {
                set_row_state(ctx, STIM_SCHEDULED_ROW, STIM_NO_ROW);
            }
            None => {}
        }

        Ok(())
    }

    fn transient_breakpoints(&self, ctx: &CmContext) -> CmResult<Vec<Value>> {
        let rows = expand_program(
            &decode_program(stim_program_param(ctx))?,
            stim_horizon(ctx),
            stim_max_rows(ctx),
        )?;
        Ok(rows.times)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn expand(program: &str, horizon: Value) -> StimRows {
        expand_program(
            &decode_program(program).expect("program decodes"),
            horizon,
            4096,
        )
        .expect("program expands")
    }

    /// Compare a transition table by time and bit pattern.
    ///
    /// Times are compared with a tolerance because a chain of `+` increments
    /// accumulates in floating point exactly as PSpice's does: three 10 ns
    /// steps land one ULP off 30 ns. That drift is the arithmetic, not a
    /// defect, and it is orders of magnitude below the scheduler's own epsilon.
    fn assert_rows(rows: &StimRows, expected: &[(Value, &str)]) {
        let actual = tokens(rows);
        assert_eq!(actual.len(), expected.len(), "row count: {actual:?}");
        for (index, ((time, bits), (expected_time, expected_bits))) in
            actual.iter().zip(expected).enumerate()
        {
            assert!(
                (time - expected_time).abs() <= 1.0e-18,
                "row {index} time {time:e} should be {expected_time:e}: {actual:?}"
            );
            assert_eq!(bits, expected_bits, "row {index} bits: {actual:?}");
        }
    }

    fn tokens(rows: &StimRows) -> Vec<(Value, String)> {
        rows.times
            .iter()
            .enumerate()
            .map(|(index, time)| {
                let bits = rows
                    .row_values(index)
                    .iter()
                    .map(|value| match (value.state, value.strength) {
                        (DigitalState::Zero, _) => '0',
                        (DigitalState::One, _) => '1',
                        (DigitalState::HighZ, _) => 'Z',
                        _ => 'X',
                    })
                    .collect::<String>();
                (*time, bits)
            })
            .collect()
    }

    #[test]
    fn absolute_and_relative_times_build_one_transition_table() {
        let rows = expand("W1 V:A:0.0:0 V:R:1e-8:1 V:R:1e-8:0", 1.0e-7);
        assert_rows(&rows, &[(0.0, "0"), (1.0e-8, "1"), (2.0e-8, "0")]);
    }

    #[test]
    fn finite_goto_repeats_its_body_the_requested_number_of_times() {
        // Body runs once, then the GOTO takes the jump twice: three passes.
        let rows = expand("W1 V:A:0.0:0 V:R:1e-8:1 V:R:1e-8:0 G:R:0.0:1:2", 1.0e-6);
        assert_eq!(rows.times.len(), 7);
        let last = rows.times.last().copied().expect("a final transition");
        assert!(
            (last - 6.0e-8).abs() <= 1.0e-18,
            "final transition at {last:e}"
        );
    }

    #[test]
    fn forever_goto_stops_at_the_expansion_horizon() {
        let rows = expand("W1 V:A:0.0:0 V:R:1e-8:1 V:R:1e-8:0 G:R:0.0:1:-1", 5.5e-8);
        assert_rows(
            &rows,
            &[
                (0.0, "0"),
                (1.0e-8, "1"),
                (2.0e-8, "0"),
                (3.0e-8, "1"),
                (4.0e-8, "0"),
                (5.0e-8, "1"),
            ],
        );
    }

    #[test]
    fn repeat_block_iterates_its_body() {
        let rows = expand("W1 P:2 V:R:1e-8:1 V:R:1e-8:0 E", 1.0e-6);
        assert_rows(
            &rows,
            &[(1.0e-8, "1"), (2.0e-8, "0"), (3.0e-8, "1"), (4.0e-8, "0")],
        );
    }

    #[test]
    fn increment_and_decrement_wrap_within_the_bus_width() {
        let rows = expand("W2 V:A:0.0:10 I:R:1e-8:1 I:R:1e-8:1 D:R:1e-8:3", 1.0e-6);
        assert_rows(
            &rows,
            &[(0.0, "10"), (1.0e-8, "11"), (2.0e-8, "00"), (3.0e-8, "01")],
        );
    }

    #[test]
    fn stepping_an_unknown_bus_yields_an_unknown_bus() {
        let rows = expand("W2 V:A:0.0:1X I:R:1e-8:1", 1.0e-6);
        assert_rows(&rows, &[(0.0, "1X"), (1.0e-8, "XX")]);
    }

    #[test]
    fn high_impedance_values_survive_expansion() {
        let rows = expand("W2 V:A:0.0:ZZ V:R:1e-8:01", 1.0e-6);
        assert_rows(&rows, &[(0.0, "ZZ"), (1.0e-8, "01")]);
    }

    #[test]
    fn a_loop_that_never_advances_time_is_refused_not_hung() {
        let error = expand_program(
            &decode_program("W1 V:A:0.0:0 G:R:0.0:0:-1").expect("program decodes"),
            1.0e-6,
            8,
        )
        .expect_err("a zero-advance forever loop is refused");
        let CmError::EvaluationError(message) = error else {
            panic!("expected an evaluation error");
        };
        assert!(
            message.contains("interpreter budget") || message.contains("resource limit"),
            "unexpected diagnostic: {message}"
        );
    }

    #[test]
    fn row_budget_refuses_rather_than_truncates() {
        let error = expand_program(
            &decode_program("W1 V:A:0.0:0 V:R:1e-9:1 G:R:0.0:1:-1").expect("program decodes"),
            1.0,
            4,
        )
        .expect_err("an over-long expansion is refused");
        let CmError::EvaluationError(message) = error else {
            panic!("expected an evaluation error");
        };
        assert!(
            message.contains("4-transition resource limit"),
            "unexpected diagnostic: {message}"
        );
    }

    #[test]
    fn malformed_programs_report_the_offending_token() {
        let error = decode_program("W1 Q:A:0.0:0").expect_err("unknown opcode is refused");
        let CmError::EvaluationError(message) = error else {
            panic!("expected an evaluation error");
        };
        assert!(
            message.contains("unknown stimulus opcode 'Q'"),
            "unexpected diagnostic: {message}"
        );
    }

    #[test]
    fn value_width_must_match_the_declared_bus() {
        let error = decode_program("W2 V:A:0.0:1").expect_err("a short value is refused");
        let CmError::EvaluationError(message) = error else {
            panic!("expected an evaluation error");
        };
        assert!(
            message.contains("has 1 bits but the bus is 2 wide"),
            "unexpected diagnostic: {message}"
        );
    }

    #[test]
    fn row_lookup_rewinds_when_the_clock_goes_backwards() {
        let times = [0.0, 1.0e-8, 2.0e-8];
        assert_eq!(row_indices(&times, 2.0e-8, 1), (Some(2), None));
        // A rejected step rewound the clock: the cursor must not stay ahead.
        assert_eq!(row_indices(&times, 0.5e-8, 2), (Some(0), Some(1)));
    }
}

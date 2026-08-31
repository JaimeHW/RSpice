//! PSpice `U<name> STIM(<width>,<format>)` digital stimulus front end.
//!
//! A `STIM` U-device declares a bus of digital outputs and a command list that
//! drives it. This module compiles the command list into the canonical
//! *stimulus program* string that the `pspice_d_stim` code model replays, and
//! lowers the device to an ordinary event-driven XSPICE instance. Everything
//! after this point is the existing digital substrate: no new execution path.
//!
//! # Why compile rather than expand
//!
//! `GOTO <label> -1 TIMES` repeats forever, so there is no finite transition
//! table until the transient stop time is known — and the parser cannot know
//! it. The command list is therefore lowered to instructions, with labels
//! already resolved to instruction indices, and the loop is run at simulation
//! time. Times and radix values *are* resolved here, so a malformed deck is a
//! parse error rather than a run-time surprise.
//!
//! # Program grammar
//!
//! Documented in full on the consuming side, in
//! `crate::xspice::models::digital::pspice_stim`. Briefly: a `W<width>` header
//! followed by space-separated instructions, each a colon-separated record
//! whose first field is the opcode (`V` drive, `G` goto, `I` increment,
//! `D` decrement, `P` repeat, `E` endrepeat).

use super::*;
use crate::netlist::{ElementProvenance, XspicePort};

/// Code model that replays the compiled program.
const STIM_CODE_MODEL: &str = "pspice_d_stim";

/// Instance parameter carrying the compiled program.
///
/// Spelled literally because `netlist` sits below `xspice` in the layer order
/// and may not read the constant from the model that defines it.
const STIM_PROGRAM_PARAM: &str = "stim_program";

/// Widest bus that `INCR BY` / `DECR BY` can step.
///
/// Stepping reads the bus as an unsigned integer; past 64 bits there is no
/// integer to read. Plain `<time> <value>` commands have no such limit.
const STIM_MAX_STEPPABLE_WIDTH: usize = 64;

pub(super) fn parse_pspice_u_stim(
    name: &str,
    fields: &[String],
    line_num: usize,
    elements: &mut Vec<Element>,
    params: &ParamContext,
) -> Result<(), ParseError> {
    let shape = parse_stim_shape(&fields[1], name, line_num)?;
    let pins = &fields[4..];
    if pins.len() < shape.width + 1 {
        return Err(stim_error(
            name,
            line_num,
            format!(
                "declares {} output node(s) and so needs {} node(s) followed by an I/O model name, \
                 but only {} field(s) follow the power pins",
                shape.width,
                shape.width,
                pins.len()
            ),
        ));
    }

    let mut nodes = Vec::with_capacity(shape.width);
    for pin in &pins[..shape.width] {
        if pspice_u_is_no_connect(pin) {
            return Err(stim_error(
                name,
                line_num,
                format!("cannot use {pin} as a stimulus output node"),
            ));
        }
        if pin.contains('=') {
            return Err(stim_error(
                name,
                line_num,
                format!(
                    "expected {} output node(s) before the I/O model name, but found the \
                     assignment '{pin}'",
                    shape.width
                ),
            ));
        }
        nodes.push(normalize_pspice_u_node(pin));
    }

    let io_model = &pins[shape.width];
    if io_model.contains('=') {
        return Err(stim_error(
            name,
            line_num,
            format!(
                "requires an I/O model name after its {} output node(s), but found the assignment \
                 '{io_model}'",
                shape.width
            ),
        ));
    }

    let (timestep, command_start) = parse_stim_options(name, &pins[shape.width + 1..], params, line_num)?;
    let commands = &pins[shape.width + 1 + command_start..];
    let program = compile_stim_program(name, &shape, timestep, commands, params, line_num)?;

    elements.push(Element {
        name: name.to_string(),
        kind: ElementKind::Xspice {
            model: STIM_CODE_MODEL.to_string(),
            pspice_u_timing: None,
            ports: vec![XspicePort::DigitalVector(nodes)],
            params: Vec::new(),
            expr_params: Vec::new(),
            string_params: vec![(STIM_PROGRAM_PARAM.to_string(), program)],
            string_expr_params: Vec::new(),
            string_vector_params: Vec::new(),
            string_vector_expr_params: Vec::new(),
            real_vector_params: Vec::new(),
            real_vector_expr_params: Vec::new(),
        },
        nodes: Vec::new(),
        provenance: ElementProvenance::Authored,
    });
    Ok(())
}

fn stim_error(name: &str, line_num: usize, message: impl AsRef<str>) -> ParseError {
    ParseError::Syntax {
        line: line_num,
        message: format!("PSpice STIM U-device '{}' {}", name, message.as_ref()),
    }
}

//=============================================================================
// STIM(<width>, <format>)
//=============================================================================

/// Declared bus width and the per-character radix widths that encode it.
struct StimShape {
    width: usize,
    /// Bits contributed by each character of a value string, in order.
    radix_bits: Vec<usize>,
}

fn parse_stim_shape(raw: &str, name: &str, line_num: usize) -> Result<StimShape, ParseError> {
    let malformed = || {
        stim_error(
            name,
            line_num,
            format!("has a malformed type field '{raw}'; expected STIM(<width>,<format>)"),
        )
    };
    let (_, tail) = raw.trim().split_once('(').ok_or_else(malformed)?;
    let body = tail.strip_suffix(')').ok_or_else(malformed)?;
    let mut parts = body.split(',');
    let width = parts
        .next()
        .and_then(|part| part.trim().parse::<usize>().ok())
        .ok_or_else(malformed)?;
    let format = parts.next().ok_or_else(malformed)?.trim();
    if parts.next().is_some() {
        return Err(malformed());
    }
    if width == 0 {
        return Err(stim_error(
            name,
            line_num,
            "declares a zero-wide bus; STIM needs at least one output node",
        ));
    }
    if format.is_empty() {
        return Err(malformed());
    }

    let mut radix_bits = Vec::with_capacity(format.len());
    for digit in format.chars() {
        // PSpice's format digits name the number of signals one value
        // character carries: 1 binary, 3 octal, 4 hexadecimal. Nothing else is
        // documented, and accepting an undocumented radix would silently
        // reinterpret a deck rather than report it.
        let bits = match digit {
            '1' => 1,
            '3' => 3,
            '4' => 4,
            other => {
                return Err(stim_error(
                    name,
                    line_num,
                    format!(
                        "has format digit '{other}' in '{format}'; each digit must be 1 (binary), \
                         3 (octal), or 4 (hexadecimal)"
                    ),
                ));
            }
        };
        radix_bits.push(bits);
    }

    let declared: usize = radix_bits.iter().sum();
    if declared != width {
        return Err(stim_error(
            name,
            line_num,
            format!(
                "declares width {width} but its format '{format}' describes {declared} signal(s); \
                 the format digits must sum to the width"
            ),
        ));
    }
    Ok(StimShape { width, radix_bits })
}

//=============================================================================
// IO_LEVEL / TIMESTEP option block
//=============================================================================

/// Read the option assignments between the I/O model and the command list.
///
/// Returns the resolved `TIMESTEP`, when given, and the number of option
/// fields consumed.
fn parse_stim_options(
    name: &str,
    tokens: &[String],
    params: &ParamContext,
    line_num: usize,
) -> Result<(Option<Value>, usize), ParseError> {
    let mut timestep = None;
    let mut index = 0usize;
    while index < tokens.len() {
        let Some((key, value, consumed)) = split_stim_assignment(tokens, index) else {
            break;
        };
        let key = key.to_ascii_uppercase();
        // `LABEL=<name>` is a command, not an option, and a command list may
        // open with one. Ending the option block here keeps the "unknown
        // option" refusal below available for a genuine misspelling.
        if key == "LABEL" {
            break;
        }
        match key.as_str() {
            // The existing U-device front end accepts IO_LEVEL positionally and
            // does not model interface-subcircuit selection; STIM follows it.
            "IO_LEVEL" => {}
            "TIMESTEP" => {
                let seconds = parse_numeric_field_value(&value, params, line_num)?;
                if !seconds.is_finite() || seconds <= 0.0 {
                    return Err(stim_error(
                        name,
                        line_num,
                        format!("has TIMESTEP={value}; it must be a positive number of seconds"),
                    ));
                }
                timestep = Some(seconds);
            }
            "STIMULUS" => {
                return Err(stim_error(
                    name,
                    line_num,
                    format!(
                        "references stimulus definition '{value}' via STIMULUS=; RSpice has no \
                         .STIMULUS card, so write the command list inline on the U-device instead"
                    ),
                ));
            }
            "MNTYMXDLY" => {
                return Err(stim_error(
                    name,
                    line_num,
                    "carries MNTYMXDLY; a stimulus source has no timing model to select a \
                     min/typ/max delay from, so remove it",
                ));
            }
            other => {
                return Err(stim_error(
                    name,
                    line_num,
                    format!(
                        "has unknown option '{other}'; STIM accepts IO_LEVEL and TIMESTEP before \
                         its command list"
                    ),
                ));
            }
        }
        index += consumed;
    }
    Ok((timestep, index))
}

/// Recognize `KEY=VALUE`, `KEY= VALUE`, `KEY =VALUE` and `KEY = VALUE`.
///
/// Returns the key, the value, and how many fields the assignment spans.
fn split_stim_assignment(tokens: &[String], index: usize) -> Option<(String, String, usize)> {
    let field = tokens.get(index)?.trim();
    if let Some((key, value)) = field.split_once('=') {
        let key = key.trim();
        if key.is_empty() {
            return None;
        }
        if !value.trim().is_empty() {
            return Some((key.to_string(), value.trim().to_string(), 1));
        }
        let value = tokens.get(index + 1)?.trim();
        return Some((key.to_string(), value.to_string(), 2));
    }
    let separator = tokens.get(index + 1)?.trim();
    if let Some(value) = separator.strip_prefix('=') {
        if !value.is_empty() {
            return Some((field.to_string(), value.to_string(), 2));
        }
        let value = tokens.get(index + 2)?.trim();
        return Some((field.to_string(), value.to_string(), 3));
    }
    None
}

//=============================================================================
// Command list
//=============================================================================

/// A time field, before its origin is resolved against the previous command.
#[derive(Debug, Clone, Copy)]
struct StimCommandTime {
    relative: bool,
    seconds: Value,
}

impl StimCommandTime {
    fn origin(self) -> char {
        if self.relative { 'R' } else { 'A' }
    }
}

/// One partially compiled instruction, before `LABEL=` targets are resolved.
enum StimEmit {
    Fixed(String),
    Goto {
        time: StimCommandTime,
        label: String,
        count: i64,
    },
}

fn compile_stim_program(
    name: &str,
    shape: &StimShape,
    timestep: Option<Value>,
    commands: &[String],
    params: &ParamContext,
    line_num: usize,
) -> Result<String, ParseError> {
    if commands.is_empty() {
        return Err(stim_error(
            name,
            line_num,
            "has no stimulus commands; add at least one '<time> <value>' pair",
        ));
    }

    let mut emits: Vec<StimEmit> = Vec::new();
    let mut labels: HashMap<String, usize> = HashMap::new();
    let mut repeat_depth = 0usize;
    let mut index = 0usize;

    while index < commands.len() {
        let token = commands[index].trim();
        let upper = token.to_ascii_uppercase();

        if upper == "ENDREPEAT" {
            if repeat_depth == 0 {
                return Err(stim_error(
                    name,
                    line_num,
                    "has an ENDREPEAT with no matching REPEAT",
                ));
            }
            repeat_depth -= 1;
            emits.push(StimEmit::Fixed("E".to_string()));
            index += 1;
            continue;
        }

        if upper == "REPEAT" {
            let (count, consumed) = parse_stim_repeat_count(name, commands, index + 1, line_num)?;
            repeat_depth += 1;
            emits.push(StimEmit::Fixed(format!("P:{count}")));
            index += 1 + consumed;
            continue;
        }

        if upper.starts_with("LABEL") && let Some((key, value, consumed)) =
            split_stim_assignment(commands, index)
            && key.eq_ignore_ascii_case("LABEL")
        {
            let label = value.to_ascii_uppercase();
            if labels.insert(label.clone(), emits.len()).is_some() {
                return Err(stim_error(
                    name,
                    line_num,
                    format!("defines label '{value}' more than once"),
                ));
            }
            index += consumed;
            continue;
        }

        let time = parse_stim_time(name, token, timestep, params, line_num)?;
        index += 1;
        let Some(next) = commands.get(index) else {
            return Err(stim_error(
                name,
                line_num,
                format!(
                    "ends after time '{token}' with no value, GOTO, INCR or DECR to go with it"
                ),
            ));
        };
        let keyword = next.trim().to_ascii_uppercase();
        match keyword.as_str() {
            "GOTO" => {
                let (label, count, consumed) =
                    parse_stim_goto(name, commands, index + 1, line_num)?;
                emits.push(StimEmit::Goto { time, label, count });
                index += 1 + consumed;
            }
            "INCR" | "DECR" => {
                let step =
                    parse_stim_step(name, shape, commands, index + 1, keyword.as_str(), line_num)?;
                let opcode = if keyword == "INCR" { 'I' } else { 'D' };
                emits.push(StimEmit::Fixed(format!(
                    "{opcode}:{}:{:?}:{step}",
                    time.origin(),
                    time.seconds
                )));
                index += 3;
            }
            _ => {
                let bits = decode_stim_value(name, shape, next.trim(), line_num)?;
                emits.push(StimEmit::Fixed(format!(
                    "V:{}:{:?}:{bits}",
                    time.origin(),
                    time.seconds
                )));
                index += 1;
            }
        }
    }

    if repeat_depth > 0 {
        return Err(stim_error(
            name,
            line_num,
            format!("has {repeat_depth} REPEAT block(s) with no matching ENDREPEAT"),
        ));
    }

    let mut program = format!("W{}", shape.width);
    for emit in &emits {
        program.push(' ');
        match emit {
            StimEmit::Fixed(text) => program.push_str(text),
            StimEmit::Goto { time, label, count } => {
                let target = labels.get(label).copied().ok_or_else(|| {
                    stim_error(
                        name,
                        line_num,
                        format!(
                            "jumps to label '{label}' which is never defined; add a \
                             'LABEL={label}' command"
                        ),
                    )
                })?;
                program.push_str(&format!(
                    "G:{}:{:?}:{target}:{count}",
                    time.origin(),
                    time.seconds
                ));
            }
        }
    }
    Ok(program)
}

/// `REPEAT FOREVER` or `REPEAT <n> TIMES`.
fn parse_stim_repeat_count(
    name: &str,
    commands: &[String],
    index: usize,
    line_num: usize,
) -> Result<(i64, usize), ParseError> {
    let Some(token) = commands.get(index) else {
        return Err(stim_error(
            name,
            line_num,
            "ends after REPEAT; write 'REPEAT FOREVER' or 'REPEAT <n> TIMES'",
        ));
    };
    if token.trim().eq_ignore_ascii_case("FOREVER") {
        return Ok((-1, 1));
    }
    let count = token.trim().parse::<i64>().ok().filter(|count| *count > 0);
    let Some(count) = count else {
        return Err(stim_error(
            name,
            line_num,
            format!(
                "has 'REPEAT {}'; write 'REPEAT FOREVER' or 'REPEAT <n> TIMES' with a positive n",
                token.trim()
            ),
        ));
    };
    // The trailing TIMES keyword is noise once the count is read.
    let consumed = if commands
        .get(index + 1)
        .is_some_and(|token| token.trim().eq_ignore_ascii_case("TIMES"))
    {
        2
    } else {
        1
    };
    Ok((count, consumed))
}

/// `GOTO <label> <n> TIMES`, with `-1` meaning forever.
fn parse_stim_goto(
    name: &str,
    commands: &[String],
    index: usize,
    line_num: usize,
) -> Result<(String, i64, usize), ParseError> {
    let Some(label) = commands.get(index) else {
        return Err(stim_error(
            name,
            line_num,
            "ends after GOTO; write 'GOTO <label> <n> TIMES'",
        ));
    };
    let label = label.trim();
    if label.eq_ignore_ascii_case("UNTIL") {
        return Err(stim_error(
            name,
            line_num,
            "uses the conditional 'GOTO ... UNTIL' form, which RSpice does not implement; write \
             'GOTO <label> <n> TIMES' with a counted repeat instead",
        ));
    }
    let Some(count) = commands.get(index + 1) else {
        return Err(stim_error(
            name,
            line_num,
            format!("has 'GOTO {label}' with no repeat count; write 'GOTO {label} <n> TIMES'"),
        ));
    };
    let count = count.trim();
    let parsed = count.parse::<i64>().ok().filter(|value| *value >= -1);
    let Some(parsed) = parsed else {
        return Err(stim_error(
            name,
            line_num,
            format!(
                "has 'GOTO {label} {count}'; the repeat count must be a non-negative integer, or \
                 -1 to repeat forever"
            ),
        ));
    };
    let consumed = if commands
        .get(index + 2)
        .is_some_and(|token| token.trim().eq_ignore_ascii_case("TIMES"))
    {
        3
    } else {
        2
    };
    Ok((label.to_ascii_uppercase(), parsed, consumed))
}

/// `INCR BY <value>` / `DECR BY <value>`, the value in the declared radix.
fn parse_stim_step(
    name: &str,
    shape: &StimShape,
    commands: &[String],
    index: usize,
    keyword: &str,
    line_num: usize,
) -> Result<u64, ParseError> {
    if !commands
        .get(index)
        .is_some_and(|token| token.trim().eq_ignore_ascii_case("BY"))
    {
        return Err(stim_error(
            name,
            line_num,
            format!("has {keyword} without BY; write '{keyword} BY <value>'"),
        ));
    }
    if shape.width > STIM_MAX_STEPPABLE_WIDTH {
        return Err(stim_error(
            name,
            line_num,
            format!(
                "steps a {}-bit bus with {keyword} BY; stepping reads the bus as an unsigned \
                 integer and is limited to {STIM_MAX_STEPPABLE_WIDTH} bits",
                shape.width
            ),
        ));
    }
    let Some(value) = commands.get(index + 1) else {
        return Err(stim_error(
            name,
            line_num,
            format!("ends after '{keyword} BY' with no value"),
        ));
    };
    let bits = decode_stim_value(name, shape, value.trim(), line_num)?;
    bits.chars()
        .try_fold(0u64, |accumulator, bit| match bit {
            '0' => Some(accumulator << 1),
            '1' => Some((accumulator << 1) | 1),
            _ => None,
        })
        .ok_or_else(|| {
            stim_error(
                name,
                line_num,
                format!(
                    "has '{keyword} BY {}'; a step value carries no X or Z digits",
                    value.trim()
                ),
            )
        })
}

//=============================================================================
// Times
//=============================================================================

/// Parse one command time field.
///
/// A leading `+` makes the time an increment from the previous command. A
/// trailing `c` or `C` counts `TIMESTEP` intervals instead of seconds.
///
/// PSpice's manual is read two ways on clock-relative times: as a `c` suffix,
/// and as "an unsuffixed number is in TIMESTEP units". RSpice implements the
/// suffix. It is the reading that cannot silently change what an ordinary
/// `10ns` means, so a deck that never writes `c` behaves the same under either
/// reading, and one that does is explicit about it.
fn parse_stim_time(
    name: &str,
    token: &str,
    timestep: Option<Value>,
    params: &ParamContext,
    line_num: usize,
) -> Result<StimCommandTime, ParseError> {
    let trimmed = token.trim();
    let (relative, magnitude) = match trimmed.strip_prefix('+') {
        Some(rest) => (true, rest.trim()),
        None => (false, trimmed),
    };
    if magnitude.is_empty() {
        return Err(stim_error(
            name,
            line_num,
            format!("has an empty time field '{token}'"),
        ));
    }

    let clocks = magnitude
        .strip_suffix(['c', 'C'])
        .filter(|head| !head.is_empty());
    let seconds = match clocks {
        Some(head) => {
            let Some(timestep) = timestep else {
                return Err(stim_error(
                    name,
                    line_num,
                    format!(
                        "uses the clock-relative time '{trimmed}' but declares no TIMESTEP; add \
                         'TIMESTEP=<value>' before the command list or write the time in seconds"
                    ),
                ));
            };
            parse_numeric_field_value(head, params, line_num)? * timestep
        }
        None => parse_numeric_field_value(magnitude, params, line_num)?,
    };

    if !seconds.is_finite() {
        return Err(stim_error(
            name,
            line_num,
            format!("has a non-finite time '{trimmed}'"),
        ));
    }
    if relative && seconds < 0.0 {
        return Err(stim_error(
            name,
            line_num,
            format!("has the negative time increment '{trimmed}'; stimulus time never runs backwards"),
        ));
    }
    Ok(StimCommandTime { relative, seconds })
}

//=============================================================================
// Values
//=============================================================================

/// Decode one value string into `width` program bit characters, MSB first.
///
/// The leftmost value character drives the leftmost output nodes, matching
/// PSpice's most-significant-first bus node ordering.
fn decode_stim_value(
    name: &str,
    shape: &StimShape,
    value: &str,
    line_num: usize,
) -> Result<String, ParseError> {
    let characters: Vec<char> = value.chars().collect();
    if characters.len() != shape.radix_bits.len() {
        return Err(stim_error(
            name,
            line_num,
            format!(
                "has value '{value}' with {} character(s), but its format needs exactly {}",
                characters.len(),
                shape.radix_bits.len()
            ),
        ));
    }

    let mut bits = String::with_capacity(shape.width);
    for (character, &radix_bits) in characters.iter().zip(shape.radix_bits.iter()) {
        let upper = character.to_ascii_uppercase();
        match upper {
            'X' | 'Z' => {
                for _ in 0..radix_bits {
                    bits.push(upper);
                }
            }
            // RSpice's digital substrate carries level and strength, not the
            // rising/falling transition states PSpice's six-state logic adds.
            // Refusing is the honest answer: mapping R to 1 and F to 0 would
            // report a settled level the deck never asked for.
            'R' | 'F' => {
                return Err(stim_error(
                    name,
                    line_num,
                    format!(
                        "has value '{value}' containing the transition state '{upper}'; RSpice's \
                         digital states are 0, 1, X and Z, so drive the settled level instead"
                    ),
                ));
            }
            _ => {
                let Some(digit) = upper.to_digit(16).filter(|digit| {
                    // Guard the radix: an octal position must not accept 8 or 9.
                    (*digit as u64) < (1u64 << radix_bits)
                }) else {
                    return Err(stim_error(
                        name,
                        line_num,
                        format!(
                            "has value '{value}' whose character '{character}' is not a \
                             {radix_bits}-bit digit; use 0-{} , X or Z there",
                            radix_digit_ceiling(radix_bits)
                        ),
                    ));
                };
                for shift in (0..radix_bits).rev() {
                    bits.push(if (digit >> shift) & 1 == 1 { '1' } else { '0' });
                }
            }
        }
    }
    Ok(bits)
}

fn radix_digit_ceiling(radix_bits: usize) -> char {
    match radix_bits {
        1 => '1',
        3 => '7',
        _ => 'F',
    }
}

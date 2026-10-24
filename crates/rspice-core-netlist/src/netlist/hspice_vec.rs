//! HSPICE digital vector files (`.VEC 'file'`).
//!
//! The format is HSPICE's, as the *HSPICE Reference Manual: Commands and
//! Control Options* (D-2010.12), chapter 4 "Digital Vector File Commands",
//! and the *HSPICE User Guide: Basic Simulation and Analysis* (H-2013.03),
//! chapter 10, "Specifying a Digital Vector File and Mixed Mode Stimuli",
//! define it. A file is a `RADIX` line, then the vector pattern and waveform
//! statements in any order, then rows of tabular data:
//!
//! ```text
//! RADIX 1 1 4 4          ; one digit per vector: its number of bits, 1 to 4
//! IO i i o o             ; input, output, bidirectional or unused
//! VNAME clk start b[7:0] ; the innermost [m:n] is the bit range, MSB first
//! TUNIT ns
//! TRISE 1 / TFALL 1 / SLOPE 1 / TDELAY / IDELAY / ODELAY   value [mask]
//! VIH / VIL / VOH / VOL / VTH / VREF / OUT / TRIZ          value [mask]
//! 0    0 1 XX            ; time, then one digit per vector in its radix
//! 50   1 1 XX
//! ```
//!
//! A mask holds one digit per vector, in that vector's radix; each set bit
//! selects the bit of the vector in the same place, MSB first. Without a
//! mask a statement applies to every signal it can apply to. Comments start
//! with `;` anywhere on a line, and a line starting with `+` continues the
//! one before.
//!
//! **Lowering.** RSpice realizes a file the way HSPICE does: each input bit
//! is a piecewise-linear voltage source from its node to its `VREF` node,
//! at VIL or VIH, ramping over TRISE or TFALL from the row's time plus its
//! input delay; each output bit is checked at every row's time plus its
//! output delay, a `1` needing V >= VOH and a `0` needing V <= VOL. Those
//! checks report through the `.ASSERT` report.
//!
//! **Refused**, each by name, because the manuals leave its meaning open or
//! RSpice does not realize it: bidirectional (`IO b`) columns and `ENABLE`;
//! `OPTION CBC`; an input `Z`; a nonzero `OUT`/`OUTZ`; one of VOH and VOL
//! without the other; Verilog-sized values; a statement after the first row;
//! a multi-bit vector without `VNAME`; and any keyword not listed here.

use std::path::Path;

use crate::Value;
use crate::netlist::{
    AssertAnalysis, AssertCheck, AssertLevel, AssertSample, AssertStatement, Element, ElementKind,
    NetlistSourceLocation, SourceSpec,
};

/// A `.VEC` card: the vector file a deck applies and checks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VectorFileReference {
    /// The file as the card names it. A path-backed parse resolves a
    /// relative name from the deck's directory; a deck parsed from memory
    /// keeps the spelling, which a prepared run registers the file under.
    pub path: String,
    /// The level an output mismatch is reported at: `LEVEL=` on the card,
    /// ERROR by default.
    pub level: AssertLevel,
    /// Where the card was written.
    pub origin: NetlistSourceLocation,
}

impl VectorFileReference {
    /// The name the file's checks report under: its file name.
    #[doc(hidden)]
    pub fn report_name(&self) -> String {
        Path::new(&self.path).file_name().map_or_else(
            || self.path.clone(),
            |name| name.to_string_lossy().into_owned(),
        )
    }
}

/// A line of a vector file that RSpice refuses or cannot read.
#[derive(Debug, Clone, PartialEq, Eq)]
#[doc(hidden)]
pub struct VectorFileError {
    /// The one-based line, 0 for the file as a whole.
    pub line: usize,
    pub message: String,
}

fn refuse<T>(line: usize, message: impl Into<String>) -> Result<T, VectorFileError> {
    Err(VectorFileError {
        line,
        message: message.into(),
    })
}

/// A vector's direction (`IO`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Direction {
    Input,
    Output,
    Bidirectional,
    Unused,
}

/// One bit's state in one row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BitState {
    Zero,
    One,
    /// `X`: an input drives zero; an expected output is not checked.
    Unknown,
    /// `Z`: an input floats behind TRIZ; an expected output is not checked.
    HighImpedance,
    /// `U`: an input drives zero; an expected output is not checked.
    Uninitialized,
    /// `H`: an input drives one behind OUT.
    ResistiveOne,
    /// `L`: an input drives zero behind OUT.
    ResistiveZero,
}

impl BitState {
    fn symbol(self) -> char {
        match self {
            Self::Zero => '0',
            Self::One => '1',
            Self::Unknown => 'X',
            Self::HighImpedance => 'Z',
            Self::Uninitialized => 'U',
            Self::ResistiveOne => 'H',
            Self::ResistiveZero => 'L',
        }
    }
}

/// One signal bit of a file, with every setting the statements gave it.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct VectorBit {
    /// The node it drives or is checked on.
    pub(crate) name: String,
    /// Its vector's one-based column.
    pub(crate) column: usize,
    pub(crate) direction: Direction,
    /// VIH and VIL, volts.
    pub(crate) high: Value,
    pub(crate) low: Value,
    /// TRISE and TFALL, or SLOPE, seconds.
    pub(crate) rise: Value,
    pub(crate) fall: Value,
    /// TDELAY/IDELAY and TDELAY/ODELAY, seconds.
    pub(crate) input_delay: Value,
    pub(crate) output_delay: Value,
    /// VREF: the node an input's source is referred to.
    pub(crate) reference: String,
    /// OUT/OUTZ and TRIZ, ohms.
    pub(crate) resistance: Value,
    pub(crate) tristate: Value,
    /// VOH and VOL as stated, and VTH, volts.
    pub(crate) voh: Option<Value>,
    pub(crate) vol: Option<Value>,
    pub(crate) vth: Value,
    /// The lines of the last OUT/OUTZ and the last VOH or VOL statement that
    /// set this bit; 0 when none did.
    pub(crate) resistance_line: usize,
    pub(crate) threshold_line: usize,
}

/// One row of tabular data.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct VectorRow {
    pub(crate) line: usize,
    /// The row's time, seconds, before any delay.
    pub(crate) time: Value,
    /// One state per bit, in [`VectorFile::bits`] order.
    pub(crate) states: Vec<BitState>,
}

/// A parsed vector file.
#[derive(Debug, Clone, PartialEq)]
#[doc(hidden)]
pub struct VectorFile {
    pub(crate) bits: Vec<VectorBit>,
    pub(crate) rows: Vec<VectorRow>,
    /// The line of the `IO` statement; 0 when the file has none.
    pub(crate) io_line: usize,
    /// The lines of the `ENABLE` statements.
    pub(crate) enables: Vec<usize>,
}

// ---------------------------------------------------------------------------
// Defaults (Reference Manual, chapter 4; User Guide, "Parameter Usage")
// ---------------------------------------------------------------------------

/// VIH: "If you do not specify the logic high voltage ... HSPICE assumes 3.3."
const DEFAULT_VIH: Value = 3.3;
/// VIL: "The default is 0.0."
const DEFAULT_VIL: Value = 0.0;
/// VTH: "If you do not specify the threshold voltage ... HSPICE assumes 1.65."
const DEFAULT_VTH: Value = 1.65;
/// SLOPE: "the default slope value is 0.1 ns".
const DEFAULT_SLOPE: Value = 0.1e-9;
/// TRIZ: "If you do not include a TRIZ statement, the default is 1000M."
const DEFAULT_TRIZ: Value = 1.0e9;

// ---------------------------------------------------------------------------
// Lines
// ---------------------------------------------------------------------------

struct Logical {
    line: usize,
    text: String,
}

/// The file's statements: comments (`;` to the end of a line) removed, a
/// `+` line joined to the one before, blank lines dropped.
fn logical_lines(text: &str) -> Result<Vec<Logical>, VectorFileError> {
    let mut lines: Vec<Logical> = Vec::new();
    for (index, raw) in text.lines().enumerate() {
        let line = index + 1;
        let content = raw.split(';').next().unwrap_or_default().trim();
        if content.is_empty() {
            continue;
        }
        if let Some(rest) = content.strip_prefix('+') {
            let Some(last) = lines.last_mut() else {
                return refuse(line, "a '+' continuation line continues no statement");
            };
            last.text.push(' ');
            last.text.push_str(rest.trim());
            continue;
        }
        lines.push(Logical {
            line,
            text: content.to_owned(),
        });
    }
    Ok(lines)
}

// ---------------------------------------------------------------------------
// Values
// ---------------------------------------------------------------------------

/// A plain decimal number, as the vector statements and rows write one.
fn plain_number(token: &str) -> Option<Value> {
    let looks_numeric = token
        .chars()
        .all(|c| c.is_ascii_digit() || matches!(c, '.' | '+' | '-' | 'e' | 'E'))
        && token.chars().any(|c| c.is_ascii_digit());
    looks_numeric
        .then(|| token.parse::<Value>().ok())
        .flatten()
        .filter(|value| value.is_finite())
}

/// A statement's value: a plain number, or a `.PARAM` name (User Guide,
/// "Parameter Usage": PERIOD, the delays, SLOPE, TRISE, TFALL and the
/// voltages take one).
fn statement_value(
    token: &str,
    keyword: &str,
    line: usize,
    param: &dyn Fn(&str) -> Option<Value>,
) -> Result<Value, VectorFileError> {
    if let Some(value) = plain_number(token) {
        return Ok(value);
    }
    if token
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
    {
        return match param(token) {
            Some(value) if value.is_finite() => Ok(value),
            _ => refuse(
                line,
                format!(
                    "{keyword} {token}: '{token}' is neither a number nor a .PARAM of the deck"
                ),
            ),
        };
    }
    refuse(
        line,
        format!("{keyword} {token}: '{token}' is not a number"),
    )
}

/// OUT/OUTZ and TRIZ take ohms in SPICE syntax (`15.1Meg`) or a `.PARAM`.
fn resistance_value(
    token: &str,
    keyword: &str,
    line: usize,
    param: &dyn Fn(&str) -> Option<Value>,
) -> Result<Value, VectorFileError> {
    if let Ok(value) = crate::netlist::lexer::parse_spice_value_complete(token)
        && value.is_finite()
    {
        return Ok(value);
    }
    statement_value(token, keyword, line, param)
}

/// The seconds one `TUNIT` time unit is: `fs`, `ps`, `ns`, `us` or `ms`,
/// optionally scaled by a number written before it (`999ns`, `.99 ps`,
/// `.99e+6ps`), as the Reference Manual's TUNIT examples write it.
fn time_unit(tokens: &[&str], line: usize) -> Result<Value, VectorFileError> {
    fn unit(text: &str) -> Option<Value> {
        Some(match text.to_ascii_lowercase().as_str() {
            "fs" => 1e-15,
            "ps" => 1e-12,
            "ns" => 1e-9,
            "us" => 1e-6,
            "ms" => 1e-3,
            _ => return None,
        })
    }
    let bad = || {
        refuse(
            line,
            format!(
                "TUNIT {}: the unit is fs, ps, ns, us or ms, optionally after a number \
                 (TUNIT 999ns, TUNIT .99 ps)",
                tokens.join(" ")
            ),
        )
    };
    let (scale, unit_text) = match tokens {
        [single] => {
            let split = single.len().saturating_sub(2);
            if !single.is_char_boundary(split) {
                return bad();
            }
            let (number, unit_text) = single.split_at(split);
            if number.is_empty() {
                (1.0, unit_text)
            } else {
                match plain_number(number) {
                    Some(scale) => (scale, unit_text),
                    None => return bad(),
                }
            }
        }
        [number, unit_text] => match plain_number(number) {
            Some(scale) => (scale, *unit_text),
            None => return bad(),
        },
        _ => return bad(),
    };
    match unit(unit_text) {
        Some(seconds) if scale > 0.0 => Ok(scale * seconds),
        _ => bad(),
    }
}

/// A value in time units, in seconds. Dividing by the exact power of ten
/// keeps an integer count of nanoseconds exact: `10` ns is the double
/// nearest 1e-8, as `1e-8` written in a deck is.
fn seconds(value: Value, unit: Value) -> Value {
    for (seconds, divisor) in [
        (1e-15, 1e15),
        (1e-12, 1e12),
        (1e-9, 1e9),
        (1e-6, 1e6),
        (1e-3, 1e3),
    ] {
        if unit == seconds {
            return value / divisor;
        }
    }
    value * unit
}

// ---------------------------------------------------------------------------
// Vectors, digits and masks
// ---------------------------------------------------------------------------

/// The value of a digit in a vector `width` bits wide (Reference Manual,
/// RADIX, Table 2: 0-1, 0-3, 0-7, 0-F).
fn digit(c: char, width: u8) -> Option<u8> {
    let value = c.to_digit(16)? as u8;
    (value < (1u8 << width)).then_some(value)
}

fn radix_name(width: u8) -> &'static str {
    match width {
        1 => "binary",
        2 => "base-4",
        3 => "octal",
        _ => "hexadecimal",
    }
}

/// One digit per vector: the concatenation of `tokens` with blanks removed.
fn per_vector(tokens: &[&str]) -> Vec<char> {
    tokens.iter().flat_map(|token| token.chars()).collect()
}

/// The bits a mask selects, in bit order.
fn mask_bits(
    tokens: &[&str],
    widths: &[u8],
    keyword: &str,
    line: usize,
) -> Result<Vec<bool>, VectorFileError> {
    let digits = per_vector(tokens);
    if digits.len() != widths.len() {
        return refuse(
            line,
            format!(
                "{keyword}: the mask '{}' has {} digit(s), but RADIX declares {} vector(s), \
                 one digit each",
                tokens.join(" "),
                digits.len(),
                widths.len()
            ),
        );
    }
    let mut bits = Vec::new();
    for (index, (c, width)) in digits.into_iter().zip(widths).enumerate() {
        let Some(value) = digit(c, *width) else {
            return refuse(
                line,
                format!(
                    "{keyword}: mask digit '{c}' of column {} is not a digit of its \
                     {width}-bit ({}) vector",
                    index + 1,
                    radix_name(*width)
                ),
            );
        };
        for bit in (0..*width).rev() {
            bits.push(value & (1 << bit) != 0);
        }
    }
    Ok(bits)
}

/// The bit names one `VNAME` entry gives: the innermost `[m:n]` is the bit
/// range, MSB first, and whatever surrounds it stays in each name (User
/// Guide, "Digital Vector File Example": `va[[1:0]]` names `va[1]` and
/// `va[0]`, `vb[12:1]` names `vb12` ... `vb1`).
fn expand_name(entry: &str, line: usize) -> Result<Vec<String>, VectorFileError> {
    let bytes = entry.as_bytes();
    let mut range = None;
    for (open, &byte) in bytes.iter().enumerate() {
        if byte != b'[' {
            continue;
        }
        let Some(close) = entry[open..].find(']').map(|offset| open + offset) else {
            continue;
        };
        let inner = &entry[open + 1..close];
        if let Some((first, last)) = inner.split_once(':')
            && let (Ok(first), Ok(last)) = (first.parse::<usize>(), last.parse::<usize>())
        {
            range = Some((open, close, first, last));
        }
    }
    let Some((open, close, first, last)) = range else {
        if entry.contains(['[', ']']) && entry.contains(':') {
            return refuse(
                line,
                format!("VNAME {entry}: a bit range is [first:last], two whole numbers"),
            );
        }
        return Ok(vec![entry.to_owned()]);
    };
    let (prefix, suffix) = (&entry[..open], &entry[close + 1..]);
    let indices: Vec<usize> = if first >= last {
        (last..=first).rev().collect()
    } else {
        (first..=last).collect()
    };
    Ok(indices
        .into_iter()
        .map(|index| format!("{prefix}{index}{suffix}"))
        .collect())
}

// ---------------------------------------------------------------------------
// Statements
// ---------------------------------------------------------------------------

/// The keywords the Reference Manual's chapter 4 lists.
const KEYWORDS: &[&str] = &[
    "RADIX", "IO", "VNAME", "TUNIT", "PERIOD", "TSKIP", "TDELAY", "IDELAY", "ODELAY", "SLOPE",
    "TRISE", "TFALL", "VIH", "VIL", "VOH", "VOL", "VTH", "VREF", "OUT", "OUTZ", "TRIZ", "ENABLE",
];

/// A per-signal statement as written, applied in file order once the
/// vectors are known.
struct Setting {
    line: usize,
    keyword: String,
    value: SettingValue,
    /// The selected bits; `None` for every bit.
    mask: Option<Vec<bool>>,
}

enum SettingValue {
    Number(Value),
    Node(String),
}

/// What the header statements declared.
#[derive(Default)]
struct Header {
    widths: Vec<u8>,
    directions: Option<(usize, Vec<Direction>)>,
    names: Option<(usize, Vec<String>)>,
    time_unit: Option<Value>,
    period: Option<(usize, Value)>,
    skip_times: Option<usize>,
    settings: Vec<Setting>,
    enables: Vec<usize>,
}

/// Parse a vector file. `param` resolves a `.PARAM` name a statement uses.
#[doc(hidden)]
pub fn parse(
    text: &str,
    param: &dyn Fn(&str) -> Option<Value>,
) -> Result<VectorFile, VectorFileError> {
    let lines = logical_lines(text)?;
    let Some(first) = lines.first() else {
        return refuse(0, "the file holds no RADIX statement and no vector");
    };
    let first_word = first.text.split_whitespace().next().unwrap_or_default();
    if !first_word.eq_ignore_ascii_case("RADIX") {
        return refuse(
            first.line,
            format!(
                "the first statement is {first_word}, but a vector file's first non-comment \
                 line must be its RADIX statement"
            ),
        );
    }
    let mut header = Header::default();
    let mut data = Vec::new();
    for logical in &lines {
        let tokens: Vec<&str> = logical.text.split_whitespace().collect();
        let keyword = tokens[0].to_ascii_uppercase();
        if KEYWORDS.contains(&keyword.as_str()) || keyword == "OPTION" {
            if let Some(row) = data.first().map(|row: &&Logical| row.line) {
                return refuse(
                    logical.line,
                    format!(
                        "{keyword} follows the tabular data that starts at line {row}; every \
                         statement comes before the first row"
                    ),
                );
            }
            statement(&mut header, &keyword, &tokens[1..], logical.line, param)?;
        } else if is_row_start(
            tokens[0],
            header.period.is_some() && header.skip_times.is_none(),
        ) {
            data.push(logical);
        } else {
            return refuse(
                logical.line,
                format!(
                    "{} is not a vector file keyword RSpice reads; the keywords are RADIX, IO, \
                     VNAME, TUNIT, PERIOD, TSKIP, TDELAY, IDELAY, ODELAY, SLOPE, TRISE, TFALL, \
                     VIH, VIL, VOH, VOL, VTH, VREF, OUT, OUTZ, TRIZ and ENABLE",
                    tokens[0]
                ),
            );
        }
    }
    let bits = resolve_bits(&header)?;
    let rows = rows(&header, &bits, &data)?;
    Ok(VectorFile {
        bits,
        rows,
        io_line: header.directions.as_ref().map_or(0, |(line, _)| *line),
        enables: header.enables,
    })
}

/// Whether a line that is not a statement is a row: one that starts with
/// its time, or, under PERIOD without TSKIP, with its first value.
fn is_row_start(first: &str, values_only: bool) -> bool {
    if values_only {
        first.contains('\'')
            || first
                .chars()
                .all(|c| c.is_ascii_hexdigit() || "XxZzUuHL".contains(c))
    } else {
        first
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_digit() || matches!(c, '.' | '-' | '+'))
    }
}

fn statement(
    header: &mut Header,
    keyword: &str,
    arguments: &[&str],
    line: usize,
    param: &dyn Fn(&str) -> Option<Value>,
) -> Result<(), VectorFileError> {
    match keyword {
        "RADIX" => {
            if !header.widths.is_empty() {
                return refuse(line, "a vector file holds one RADIX statement");
            }
            let digits = per_vector(arguments);
            if digits.is_empty() {
                return refuse(line, "RADIX declares no vector");
            }
            for c in digits {
                match c.to_digit(10) {
                    Some(width @ 1..=4) => header.widths.push(width as u8),
                    _ => {
                        return refuse(
                            line,
                            format!("RADIX digit '{c}': a vector has 1 to 4 bits"),
                        );
                    }
                }
            }
        }
        "IO" => {
            let mut directions = Vec::new();
            for c in per_vector(arguments) {
                directions.push(match c.to_ascii_lowercase() {
                    'i' => Direction::Input,
                    'o' => Direction::Output,
                    'b' => Direction::Bidirectional,
                    'u' => Direction::Unused,
                    _ => {
                        return refuse(
                            line,
                            format!("IO '{c}': a vector is i(nput), o(utput), b(idirectional) or u(nused)"),
                        );
                    }
                });
            }
            if directions.len() != header.widths.len() {
                return refuse(
                    line,
                    format!(
                        "IO gives {} direction(s), but RADIX declares {} vector(s)",
                        directions.len(),
                        header.widths.len()
                    ),
                );
            }
            header.directions = Some((line, directions));
        }
        "VNAME" => {
            let mut names = Vec::new();
            for entry in arguments {
                names.push(expand_name(entry, line)?);
            }
            let bits: Vec<String> = names.iter().flatten().cloned().collect();
            let total: usize = header.widths.iter().map(|width| usize::from(*width)).sum();
            if bits.len() != total {
                return refuse(
                    line,
                    format!(
                        "VNAME names {} bit(s), but RADIX declares {total}",
                        bits.len()
                    ),
                );
            }
            // A name covers whole vectors: its last bit ends one.
            let mut ends = std::collections::BTreeSet::new();
            let mut bit = 0usize;
            for width in &header.widths {
                bit += usize::from(*width);
                ends.insert(bit);
            }
            let mut bit = 0usize;
            for (entry, expanded) in arguments.iter().zip(&names) {
                bit += expanded.len();
                if !ends.contains(&bit) {
                    return refuse(
                        line,
                        format!(
                            "VNAME {entry} ends inside a vector; a name covers one or more \
                             whole vectors"
                        ),
                    );
                }
            }
            header.names = Some((line, bits));
        }
        "TUNIT" => header.time_unit = Some(time_unit(arguments, line)?),
        "PERIOD" => {
            if let Some((previous, _)) = header.period {
                return refuse(
                    line,
                    format!("PERIOD is already given at line {previous}; a file has one period"),
                );
            }
            let [value] = arguments else {
                return refuse(line, "PERIOD takes one time interval");
            };
            let value = statement_value(value, "PERIOD", line, param)?;
            if value <= 0.0 {
                return refuse(
                    line,
                    format!("PERIOD {value}: the interval must be positive"),
                );
            }
            header.period = Some((line, value));
        }
        "TSKIP" => {
            if !arguments.is_empty() {
                return refuse(
                    line,
                    "TSKIP stands alone: it makes each row's leading time field ignored",
                );
            }
            header.skip_times = Some(line);
        }
        "ENABLE" => {
            // Parsed for its line; the bidirectional columns it controls are
            // refused when the file is applied.
            header.enables.push(line);
        }
        "OPTION" => {
            return refuse(
                line,
                format!(
                    "OPTION {}: context-based control (OPTION CBC) switches bidirectional \
                     columns between input and output, and RADIX drives no bidirectional column",
                    arguments.join(" ")
                ),
            );
        }
        _ => {
            let Some((value, mask)) = arguments.split_first() else {
                return refuse(line, format!("{keyword} requires a value"));
            };
            let value = match keyword {
                "VREF" => {
                    if plain_number(value).is_some_and(|value| value == 0.0) {
                        SettingValue::Node("0".into())
                    } else {
                        SettingValue::Node((*value).to_owned())
                    }
                }
                "OUT" | "OUTZ" | "TRIZ" => {
                    SettingValue::Number(resistance_value(value, keyword, line, param)?)
                }
                _ => SettingValue::Number(statement_value(value, keyword, line, param)?),
            };
            if header.widths.is_empty() {
                return refuse(line, format!("{keyword} comes before RADIX"));
            }
            let mask = if mask.is_empty() {
                None
            } else {
                Some(mask_bits(mask, &header.widths, keyword, line)?)
            };
            header.settings.push(Setting {
                line,
                keyword: keyword.to_owned(),
                value,
                mask,
            });
        }
    }
    Ok(())
}

/// Every bit with its settings: defaults, then each statement in file order,
/// the last one that selects a bit winning.
fn resolve_bits(header: &Header) -> Result<Vec<VectorBit>, VectorFileError> {
    let unit = header.time_unit.unwrap_or(1e-9);
    let names = match &header.names {
        Some((_, names)) => names.clone(),
        None => {
            // "If you do not specify VNAME, HSPICE ... assigns a default name
            // to each signal: V1, V2, V3, and so on" -- a name per signal,
            // which names a bit only when every vector is one bit wide.
            if let Some(column) = header.widths.iter().position(|width| *width > 1) {
                return refuse(
                    0,
                    format!(
                        "the file has no VNAME, and column {} is a {}-bit vector; the default \
                         names V1, V2, ... name one-bit signals only",
                        column + 1,
                        header.widths[column]
                    ),
                );
            }
            (1..=header.widths.len()).map(|n| format!("V{n}")).collect()
        }
    };
    // "If you do not specify the IO command, HSPICE ... assumes that all
    // signals are input signals."
    let directions = header.directions.as_ref().map_or_else(
        || vec![Direction::Input; header.widths.len()],
        |(_, directions)| directions.clone(),
    );
    let mut bits = Vec::new();
    for (column, width) in header.widths.iter().enumerate() {
        for _ in 0..*width {
            bits.push(VectorBit {
                name: names[bits.len()].clone(),
                column: column + 1,
                direction: directions[column],
                high: DEFAULT_VIH,
                low: DEFAULT_VIL,
                rise: Value::NAN,
                fall: Value::NAN,
                input_delay: 0.0,
                output_delay: 0.0,
                reference: "0".into(),
                resistance: 0.0,
                tristate: DEFAULT_TRIZ,
                voh: None,
                vol: None,
                vth: DEFAULT_VTH,
                resistance_line: 0,
                threshold_line: 0,
            });
        }
    }
    let mut slope = vec![DEFAULT_SLOPE; bits.len()];
    let mut rise = vec![None; bits.len()];
    let mut fall = vec![None; bits.len()];
    for setting in &header.settings {
        for (index, bit) in bits.iter_mut().enumerate() {
            if setting.mask.as_ref().is_some_and(|mask| !mask[index]) {
                continue;
            }
            let number = match &setting.value {
                SettingValue::Number(value) => *value,
                SettingValue::Node(node) => {
                    bit.reference.clone_from(node);
                    continue;
                }
            };
            let time = seconds(number, unit);
            match setting.keyword.as_str() {
                "TDELAY" => {
                    bit.input_delay = time;
                    bit.output_delay = time;
                }
                "IDELAY" => bit.input_delay = time,
                "ODELAY" => bit.output_delay = time,
                "SLOPE" => slope[index] = time,
                "TRISE" => rise[index] = Some(time),
                "TFALL" => fall[index] = Some(time),
                "VIH" => bit.high = number,
                "VIL" => bit.low = number,
                "VOH" => {
                    bit.voh = Some(number);
                    bit.threshold_line = setting.line;
                }
                "VOL" => {
                    bit.vol = Some(number);
                    bit.threshold_line = setting.line;
                }
                "VTH" => bit.vth = number,
                "OUT" | "OUTZ" => {
                    bit.resistance = number;
                    bit.resistance_line = setting.line;
                }
                "TRIZ" => bit.tristate = number,
                _ => unreachable!("{} is a per-signal keyword", setting.keyword),
            }
        }
    }
    for (index, bit) in bits.iter_mut().enumerate() {
        // "If you do not use any TRISE command ..., HSPICE uses the value
        // defined in the slope command" -- whatever order they come in.
        bit.rise = rise[index].unwrap_or(slope[index]);
        bit.fall = fall[index].unwrap_or(slope[index]);
    }
    Ok(bits)
}

/// The rows, with their times in seconds and one state per bit.
fn rows(
    header: &Header,
    bits: &[VectorBit],
    data: &[&Logical],
) -> Result<Vec<VectorRow>, VectorFileError> {
    let unit = header.time_unit.unwrap_or(1e-9);
    if header.skip_times.is_some() && header.period.is_none() {
        return refuse(
            header.skip_times.unwrap_or_default(),
            "TSKIP ignores each row's time field in favour of PERIOD, and the file gives no \
             PERIOD",
        );
    }
    let timed = header.period.is_none() || header.skip_times.is_some();
    let mut rows: Vec<VectorRow> = Vec::with_capacity(data.len());
    for (index, logical) in data.iter().enumerate() {
        let line = logical.line;
        let tokens: Vec<&str> = logical.text.split_whitespace().collect();
        let (time, values) = if timed {
            let Some(time) = plain_number(tokens[0]) else {
                return refuse(line, format!("'{}' is not a row time", tokens[0]));
            };
            let time = match header.period {
                Some((_, period)) => seconds(period * index as Value, unit),
                None => seconds(time, unit),
            };
            (time, &tokens[1..])
        } else {
            let period = header.period.map_or(0.0, |(_, period)| period);
            (seconds(period * index as Value, unit), &tokens[..])
        };
        if let Some(previous) = rows.last()
            && time <= previous.time
        {
            return refuse(
                line,
                format!(
                    "the row's time {time:e} s is not after the time {:e} s of the row at line \
                     {}; rows run in time order",
                    previous.time, previous.line
                ),
            );
        }
        if values.iter().any(|token| token.contains('\'')) {
            return refuse(
                line,
                "a Verilog-sized value (<size>'<base><number>) is not read; write one digit per \
                 vector in its radix",
            );
        }
        let digits = per_vector(values);
        let widths = &header.widths;
        if digits.len() != widths.len() {
            return refuse(
                line,
                format!(
                    "the row gives {} value(s), but RADIX declares {} vector(s), one digit each",
                    digits.len(),
                    widths.len()
                ),
            );
        }
        let mut states = Vec::with_capacity(bits.len());
        for (column, (c, width)) in digits.into_iter().zip(widths).enumerate() {
            let state = match c {
                'X' | 'x' => Some(BitState::Unknown),
                'Z' | 'z' => Some(BitState::HighImpedance),
                'U' | 'u' => Some(BitState::Uninitialized),
                'H' => Some(BitState::ResistiveOne),
                'L' => Some(BitState::ResistiveZero),
                _ => None,
            };
            if let Some(state) = state {
                states.extend(std::iter::repeat_n(state, usize::from(*width)));
                continue;
            }
            let Some(value) = digit(c, *width) else {
                return refuse(
                    line,
                    format!(
                        "'{c}' in column {} is not a digit of its {width}-bit ({}) vector, nor \
                         a state (X, Z, U, H, L)",
                        column + 1,
                        radix_name(*width)
                    ),
                );
            };
            for bit in (0..*width).rev() {
                states.push(if value & (1 << bit) != 0 {
                    BitState::One
                } else {
                    BitState::Zero
                });
            }
        }
        rows.push(VectorRow { line, time, states });
    }
    Ok(rows)
}

// ---------------------------------------------------------------------------
// Lowering
// ---------------------------------------------------------------------------

impl VectorFile {
    /// Refuse what RSpice does not realize: bidirectional columns and their
    /// ENABLE, an input `Z`, a nonzero OUT, an output `H` or `L`, and an
    /// output bit with one of VOH and VOL.
    #[doc(hidden)]
    pub fn check_realizable(&self) -> Result<(), VectorFileError> {
        if let Some(bit) = self
            .bits
            .iter()
            .find(|bit| bit.direction == Direction::Bidirectional)
        {
            return refuse(
                self.io_line,
                format!(
                    "column {} ({}) is bidirectional (IO b); RSpice does not drive a \
                     bidirectional column or its ENABLE",
                    bit.column, bit.name
                ),
            );
        }
        if let Some(line) = self.enables.first() {
            return refuse(
                *line,
                "ENABLE controls bidirectional columns, and the file declares none",
            );
        }
        for bit in &self.bits {
            if bit.direction == Direction::Input && bit.resistance != 0.0 {
                return refuse(
                    bit.resistance_line,
                    format!(
                        "OUT/OUTZ gives input {} a {} ohm output resistance; the manuals \
                         disagree on whether a 0 or 1 drives through it (OUT: \"in series with \
                         all vector inputs\"; the input state table: \"Resistance set to 0\"), \
                         so RSpice drives inputs ideally and refuses a nonzero OUT",
                        bit.name, bit.resistance
                    ),
                );
            }
            if bit.direction == Direction::Output && bit.voh.is_some() != bit.vol.is_some() {
                let (given, missing) = if bit.voh.is_some() {
                    ("VOH", "VOL")
                } else {
                    ("VOL", "VOH")
                };
                return refuse(
                    bit.threshold_line,
                    format!(
                        "output {} has {given} but no {missing}; HSPICE uses VTH only when \
                         neither is given, and the manuals state two defaults for {missing}, \
                         so give both",
                        bit.name
                    ),
                );
            }
        }
        for row in &self.rows {
            for (bit, state) in self.bits.iter().zip(&row.states) {
                match (bit.direction, state) {
                    (Direction::Input, BitState::HighImpedance) => {
                        return refuse(
                            row.line,
                            format!(
                                "input {} (column {}) is Z; RSpice does not float an input \
                                 behind its TRIZ impedance ({:e} ohm)",
                                bit.name, bit.column, bit.tristate
                            ),
                        );
                    }
                    (Direction::Output, BitState::ResistiveOne | BitState::ResistiveZero) => {
                        return refuse(
                            row.line,
                            format!(
                                "output {} (column {}) expects {}, which is not an expected \
                                 output state (0, 1, X, U, Z)",
                                bit.name,
                                bit.column,
                                state.symbol()
                            ),
                        );
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    /// One ideal piecewise-linear source per input bit, from its node to its
    /// VREF node. A change starts at the row's time plus the bit's input
    /// delay and ramps over TRISE (rising) or TFALL (falling); before its
    /// first row a bit holds its first level. `0`, `L`, `X` and `U` drive
    /// VIL; `1` and `H` drive VIH.
    #[doc(hidden)]
    pub fn input_sources(&self, name_prefix: &str) -> Result<Vec<Element>, VectorFileError> {
        let mut elements = Vec::new();
        for (index, bit) in self.bits.iter().enumerate() {
            if bit.direction != Direction::Input || self.rows.is_empty() {
                continue;
            }
            let level = |state: BitState| match state {
                BitState::One | BitState::ResistiveOne => bit.high,
                _ => bit.low,
            };
            let first = &self.rows[0];
            let mut level_now = level(first.states[index]);
            let mut points = vec![(first.time + bit.input_delay, level_now)];
            for row in &self.rows[1..] {
                let next = level(row.states[index]);
                if next == level_now {
                    continue;
                }
                let start = row.time + bit.input_delay;
                let ramp = if next > level_now { bit.rise } else { bit.fall };
                if ramp.is_nan() || ramp <= 0.0 {
                    return refuse(
                        row.line,
                        format!(
                            "input {} changes with a {} of {ramp:e} s; a transition takes a \
                             positive time",
                            bit.name,
                            if next > level_now {
                                "rise time"
                            } else {
                                "fall time"
                            }
                        ),
                    );
                }
                let (last_time, _) = *points.last().expect("a first point");
                if start < last_time {
                    return refuse(
                        row.line,
                        format!(
                            "input {} changes at {start:e} s, before its previous transition \
                             ends at {last_time:e} s",
                            bit.name
                        ),
                    );
                }
                if start > last_time {
                    points.push((start, level_now));
                }
                points.push((start + ramp, next));
                level_now = next;
            }
            // The deck's lexer folds every name to ASCII capitals, so a
            // source spelled as the file spells it would drive a node of its
            // own beside the deck's.
            elements.push(Element {
                name: format!("{name_prefix}{}", bit.name).to_ascii_uppercase(),
                kind: ElementKind::VoltageSource(SourceSpec::Pwl {
                    points: from_time_zero(points),
                    delay: 0.0,
                    repeat_from: None,
                }),
                nodes: vec![
                    bit.name.to_ascii_uppercase(),
                    bit.reference.to_ascii_uppercase(),
                ],
                provenance: Default::default(),
            });
        }
        Ok(elements)
    }

    /// One sampled check per output bit: at each row's time plus the bit's
    /// output delay, a `1` needs V >= VOH and a `0` needs V <= VOL (VTH for
    /// both when the file gives neither); `X`, `U` and `Z` are not checked.
    #[doc(hidden)]
    pub fn output_checks(&self, reference: &VectorFileReference) -> Vec<AssertStatement> {
        let file = reference.report_name();
        // The rows are in the vector file, wherever the card is.
        let location = |line| NetlistSourceLocation::in_file(&reference.path, line);
        let mut statements = Vec::new();
        for (index, bit) in self.bits.iter().enumerate() {
            if bit.direction != Direction::Output {
                continue;
            }
            let (voh, vol) = match (bit.voh, bit.vol) {
                (Some(voh), Some(vol)) => (voh, vol),
                _ => (bit.vth, bit.vth),
            };
            let samples = self
                .rows
                .iter()
                .filter_map(|row| {
                    let high = match row.states[index] {
                        BitState::One => true,
                        BitState::Zero => false,
                        _ => return None,
                    };
                    Some(AssertSample {
                        time: row.time + bit.output_delay,
                        high,
                        threshold: if high { voh } else { vol },
                        origin: location(row.line),
                    })
                })
                .collect();
            statements.push(AssertStatement {
                name: file.clone(),
                check: AssertCheck::Sampled {
                    node: bit.name.clone(),
                    column: bit.column,
                    samples,
                },
                from: None,
                to: None,
                duration: None,
                level: reference.level,
                message: None,
                analyses: vec![AssertAnalysis::Tran],
                ignore_unknown: false,
                origin: reference.origin.clone(),
            });
        }
        statements
    }
}

/// A waveform whose first points fall before time zero (a negative delay)
/// starts at time zero, at its level there.
fn from_time_zero(points: Vec<(Value, Value)>) -> Vec<(Value, Value)> {
    let Some(first_kept) = points.iter().position(|(time, _)| *time >= 0.0) else {
        let last = points.last().map_or(0.0, |(_, value)| *value);
        return vec![(0.0, last)];
    };
    if first_kept == 0 {
        return points;
    }
    let (t0, v0) = points[first_kept - 1];
    let (t1, v1) = points[first_kept];
    let mut kept = Vec::with_capacity(points.len() - first_kept + 1);
    if t1 > 0.0 {
        kept.push((0.0, v0 + (v1 - v0) * (0.0 - t0) / (t1 - t0)));
    }
    kept.extend_from_slice(&points[first_kept..]);
    kept
}

#[cfg(test)]
#[path = "hspice_vec/tests.rs"]
mod tests;

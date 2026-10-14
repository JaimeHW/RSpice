//! `$display`-family formatting (IEEE 1364-2005 section 17.1.1).
//!
//! The arguments arrive as the site table describes them: string literals,
//! which are format specifications, empty arguments, and evaluated values.
//! Every format specification, width rule and `x`/`z` digit rule follows the
//! standard; where it leaves a choice open the choice is Icarus Verilog's, so
//! that a gold file written by Icarus reads the same here. Each expected string
//! in the tests below is an Icarus 13.0 run of an RSpice-authored probe.

use num_bigint::{BigInt, BigUint, Sign};
use rspice_veriloga::canonical_ir::digital::DigitalRadix;
use rspice_veriloga::canonical_ir::digital_eval::DigitalScalar;
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::four_state::FourStateBit;
use serde::{Deserialize, Serialize};

/// `$timeformat`'s settings (IEEE 1364-2005 section 17.3.2).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TimeFormat {
    /// The power of ten `%t` prints in.
    pub units: i8,
    /// Digits after the decimal point.
    pub precision: u8,
    pub suffix: String,
    /// The field width `%t` pads to.
    pub min_width: u32,
}

impl TimeFormat {
    /// Section 17.3.2's defaults: the design's finest precision, no fraction,
    /// no suffix, twenty columns.
    pub(crate) fn default_for(design_precision: i8) -> Self {
        Self {
            units: design_precision,
            precision: 0,
            suffix: String::new(),
            min_width: 20,
        }
    }
}

/// One argument as the formatter sees it.
#[derive(Debug, Clone, Copy)]
pub(crate) enum FormatArgument<'a> {
    /// `$display(a,,b)`: printed as one space.
    Empty,
    /// A string literal: a format specification where it stands, text where a
    /// `%s` consumes it.
    Literal(&'a str),
    /// An evaluated value and whether it is signed.
    Value(&'a DigitalScalar, bool),
    /// A bare `$realtime` and the decimals it prints with when no
    /// specification consumes it; a specification reads it as a real.
    RealTime(&'a DigitalScalar, u8),
}

/// What a format reads besides its arguments.
pub(crate) struct FormatContext<'a> {
    /// How an argument no specification consumes is printed.
    pub radix: DigitalRadix,
    /// `%m`: the calling scope's full path.
    pub scope: &'a str,
    /// `%l`: the calling module's definition name, or the `library.cell` of
    /// its library binding.
    pub module: &'a str,
    /// The calling module's time unit, which a `%t` operand is counted in.
    pub time_unit: i8,
    pub time_format: &'a TimeFormat,
}

/// Format a whole argument list the way `$display` does, without the newline.
pub(crate) fn format(arguments: &[FormatArgument<'_>], context: &FormatContext<'_>) -> String {
    let mut out = String::new();
    let mut next = 0;
    while next < arguments.len() {
        let argument = arguments[next];
        next += 1;
        match argument {
            FormatArgument::Literal(text) => {
                next = interpret(text, arguments, next, &mut out, context);
            }
            FormatArgument::Empty => out.push(' '),
            FormatArgument::Value(value, signed) => {
                let spec = Spec::natural(match context.radix {
                    DigitalRadix::Decimal => 'd',
                    DigitalRadix::Binary => 'b',
                    DigitalRadix::Octal => 'o',
                    DigitalRadix::Hex => 'h',
                });
                if matches!(value, DigitalScalar::Real(_)) {
                    let mut spec = Spec::natural('g');
                    spec.alternate = true;
                    render(&spec, argument_of(value, signed), &mut out, context);
                } else {
                    render(&spec, argument_of(value, signed), &mut out, context);
                }
            }
            FormatArgument::RealTime(value, decimals) => {
                let mut spec = Spec::natural('f');
                spec.precision = Some(usize::from(decimals));
                render(&spec, argument_of(value, true), &mut out, context);
            }
        }
    }
    out
}

fn argument_of(value: &DigitalScalar, signed: bool) -> FormatArgument<'_> {
    FormatArgument::Value(value, signed)
}

/// One parsed `%` specification.
#[derive(Debug, Clone)]
struct Spec {
    conversion: char,
    left: bool,
    zero: bool,
    /// `None`: the natural width; `Some(0)`: the minimum.
    width: Option<usize>,
    precision: Option<usize>,
    alternate: bool,
}

impl Spec {
    fn natural(conversion: char) -> Self {
        Self {
            conversion,
            left: false,
            zero: false,
            width: None,
            precision: None,
            alternate: false,
        }
    }
}

/// Walk one format string, consuming arguments from `next`; returns the
/// index of the first argument it did not consume.
fn interpret(
    text: &str,
    arguments: &[FormatArgument<'_>],
    mut next: usize,
    out: &mut String,
    context: &FormatContext<'_>,
) -> usize {
    let mut characters = text.char_indices().peekable();
    while let Some((start, character)) = characters.next() {
        if character != '%' {
            out.push(character);
            continue;
        }
        let mut spec = Spec::natural('?');
        if let Some((_, '-')) = characters.peek() {
            spec.left = true;
            characters.next();
        }
        let mut digits = String::new();
        while let Some((_, digit)) = characters
            .peek()
            .copied()
            .filter(|(_, c)| c.is_ascii_digit())
        {
            digits.push(digit);
            characters.next();
        }
        if !digits.is_empty() {
            if digits.len() > 1 && digits.starts_with('0') {
                spec.zero = true;
            }
            spec.width = digits.parse().ok();
        }
        if let Some((_, '.')) = characters.peek() {
            characters.next();
            let mut digits = String::new();
            while let Some((_, digit)) = characters
                .peek()
                .copied()
                .filter(|(_, c)| c.is_ascii_digit())
            {
                digits.push(digit);
                characters.next();
            }
            spec.precision = Some(digits.parse().unwrap_or(0));
        }
        let Some((end, conversion)) = characters.next() else {
            // A trailing `%` has nothing to convert; print it as written.
            out.push_str(&text[start..]);
            break;
        };
        spec.conversion = conversion.to_ascii_lowercase();
        match spec.conversion {
            '%' => out.push('%'),
            'm' => pad(out, context.scope, &spec, context.scope.len()),
            'l' => {
                // IEEE 1364-2005 section 13.6: `library.cell`. A module a
                // library binding placed is known by that name already; any
                // other is the library `work`'s.
                let library = if context.module.contains('.') {
                    context.module.to_owned()
                } else {
                    format!("work.{}", context.module)
                };
                pad(out, &library, &spec, library.len());
            }
            'h' | 'x' | 'd' | 'o' | 'b' | 'c' | 's' | 't' | 'e' | 'f' | 'g' | 'v' | 'u' | 'z' => {
                if spec.conversion == 'x' {
                    spec.conversion = 'h';
                }
                match arguments.get(next) {
                    Some(argument) => {
                        next += 1;
                        render(&spec, *argument, out, context);
                    }
                    // No argument left: the specification has nothing to
                    // print, so it is printed as written.
                    None => out.push_str(&text[start..end + conversion.len_utf8()]),
                }
            }
            _ => out.push_str(&text[start..end + conversion.len_utf8()]),
        }
    }
    next
}

/// Right- or left-justify `text` in the specification's field.
fn pad(out: &mut String, text: &str, spec: &Spec, natural: usize) {
    let width = spec.width.unwrap_or(natural);
    let length = text.chars().count();
    if length >= width {
        out.push_str(text);
        return;
    }
    let fill = width - length;
    if spec.left {
        out.push_str(text);
        out.extend(std::iter::repeat_n(' ', fill));
    } else if spec.zero && !text.starts_with('-') {
        out.extend(std::iter::repeat_n('0', fill));
        out.push_str(text);
    } else if spec.zero {
        out.push('-');
        out.extend(std::iter::repeat_n('0', fill));
        out.push_str(&text[1..]);
    } else {
        out.extend(std::iter::repeat_n(' ', fill));
        out.push_str(text);
    }
}

/// A string literal's characters as a packed value, eight bits each.
fn packed_string(text: &str) -> FourStateValue {
    let bytes: Vec<u8> = text
        .chars()
        .map(|character| character as u32 as u8)
        .collect();
    let width = (bytes.len() as u32 * 8).max(8);
    let mut value = FourStateValue::zero(width);
    for (position, byte) in bytes.iter().rev().enumerate() {
        for bit in 0..8 {
            if byte >> bit & 1 == 1 {
                value.set_bit(position as u32 * 8 + bit, FourStateBit::One);
            }
        }
    }
    value
}

/// Real-to-integer conversion for an integral specification: round half away
/// from zero, as an assignment does.
fn real_as_integer(value: f64) -> Option<BigInt> {
    if !value.is_finite() {
        return None;
    }
    let rounded = value.round();
    BigInt::parse_bytes(format!("{rounded:.0}").as_bytes(), 10)
}

/// What a radix specification prints for a real's rounded `integer`: a
/// negative value a 64-bit integer holds as its two's complement in 64 bits,
/// anything else as itself.
fn radix_operand(integer: &BigInt) -> BigInt {
    match i64::try_from(integer) {
        Ok(negative) if negative < 0 => BigInt::from(negative as u64),
        _ => integer.clone(),
    }
}

fn render(
    spec: &Spec,
    argument: FormatArgument<'_>,
    out: &mut String,
    context: &FormatContext<'_>,
) {
    let (value, signed) = match argument {
        FormatArgument::Empty => {
            out.push(' ');
            return;
        }
        FormatArgument::Literal(text) => {
            if spec.conversion == 's' {
                // A literal printed as text keeps every character.
                let natural = if spec.width.is_none() {
                    0
                } else {
                    text.chars().count()
                };
                pad(out, text, spec, natural);
                return;
            }
            let value = DigitalScalar::FourState(packed_string(text));
            render_value(spec, &value, false, out, context);
            return;
        }
        FormatArgument::Value(value, signed) => (value, signed),
        FormatArgument::RealTime(value, _) => (value, true),
    };
    render_value(spec, value, signed, out, context);
}

fn render_value(
    spec: &Spec,
    value: &DigitalScalar,
    signed: bool,
    out: &mut String,
    context: &FormatContext<'_>,
) {
    match spec.conversion {
        'e' | 'f' | 'g' => {
            let real = match value {
                DigitalScalar::Real(real) => Some(*real),
                DigitalScalar::FourState(bits) => bits.to_real(signed).ok(),
                DigitalScalar::Integer(integer) => Some(f64::from(*integer)),
                DigitalScalar::Effect => None,
            };
            let text = match real {
                Some(real) => format_real(real, spec),
                None => "x".to_string(),
            };
            pad(out, &text, spec, 0);
        }
        't' => {
            let text = format_time(value, signed, context);
            let width = spec.width.unwrap_or(context.time_format.min_width as usize);
            let spec = Spec {
                width: Some(width),
                ..spec.clone()
            };
            pad(out, &text, &spec, width);
        }
        'v' => {
            let bits = match value {
                DigitalScalar::FourState(bits) => bits.clone(),
                DigitalScalar::Integer(integer) => {
                    FourStateValue::from_integer(32, i128::from(*integer))
                }
                _ => FourStateValue::splat(1, FourStateBit::Unknown),
            };
            let text = (0..bits.width())
                .rev()
                .map(|index| match bits.bit(index) {
                    FourStateBit::Zero => "St0",
                    FourStateBit::One => "St1",
                    FourStateBit::Unknown => "StX",
                    FourStateBit::HighImpedance => "HiZ",
                })
                .collect::<Vec<_>>()
                .join(" ");
            pad(out, &text, spec, 0);
        }
        _ => {
            let bits = match value {
                DigitalScalar::FourState(bits) => bits.clone(),
                DigitalScalar::Integer(integer) => {
                    FourStateValue::from_integer(32, i128::from(*integer))
                }
                DigitalScalar::Real(real) => {
                    // Icarus prints a real under an integral specification as
                    // its rounded integer at its minimal width. In a radix, a
                    // negative one prints as the 64-bit two's complement the
                    // real converts to: `%0x` of -1.0 is `ffffffffffffffff`.
                    let text = match real_as_integer(*real) {
                        Some(integer) => match spec.conversion {
                            'h' => radix_operand(&integer).to_str_radix(16),
                            'o' => radix_operand(&integer).to_str_radix(8),
                            'b' => radix_operand(&integer).to_str_radix(2),
                            'c' => char::from(
                                (integer.to_u32_digits().1.first().copied().unwrap_or(0) & 0xff)
                                    as u8,
                            )
                            .to_string(),
                            _ => integer.to_string(),
                        },
                        None => format_real(*real, &Spec::natural('g')),
                    };
                    pad(out, &text, spec, 0);
                    return;
                }
                DigitalScalar::Effect => FourStateValue::splat(1, FourStateBit::Unknown),
            };
            render_bits(spec, &bits, signed, out);
        }
    }
}

/// The digit an all-same group of bits prints as, or the summary of a mixed
/// one: `x`/`z` when every bit is, `X`/`Z` when some are (section 17.1.1.3).
fn digit_of(bits: &[FourStateBit]) -> Option<char> {
    let unknown = bits
        .iter()
        .filter(|bit| **bit == FourStateBit::Unknown)
        .count();
    let high = bits
        .iter()
        .filter(|bit| **bit == FourStateBit::HighImpedance)
        .count();
    if unknown == bits.len() {
        Some('x')
    } else if high == bits.len() {
        Some('z')
    } else if unknown > 0 || (high > 0 && unknown + high == bits.len()) {
        Some('X')
    } else if high > 0 {
        Some('Z')
    } else {
        None
    }
}

fn magnitude(bits: &FourStateValue) -> BigUint {
    let mut value = BigUint::from(0u8);
    for index in (0..bits.width()).rev() {
        value <<= 1;
        if bits.bit(index) == FourStateBit::One {
            value += 1u8;
        }
    }
    value
}

/// The characters the largest value of `width` bits takes in decimal,
/// including a sign for a signed value (section 17.1.1.2's automatic width).
fn decimal_width(width: u32, signed: bool) -> usize {
    if width == 0 {
        return 1;
    }
    if signed {
        let largest = BigUint::from(1u8) << (width as usize - 1);
        largest.to_string().len() + 1
    } else {
        let largest = (BigUint::from(1u8) << width as usize) - 1u8;
        largest.to_string().len()
    }
}

fn render_bits(spec: &Spec, bits: &FourStateValue, signed: bool, out: &mut String) {
    let width = bits.width();
    match spec.conversion {
        'd' => {
            let all: Vec<FourStateBit> = (0..width).map(|index| bits.bit(index)).collect();
            let text = match digit_of(&all) {
                Some(digit) => digit.to_string(),
                None => {
                    let magnitude = magnitude(bits);
                    if signed && width > 0 && bits.bit(width - 1) == FourStateBit::One {
                        let negative =
                            BigInt::from_biguint(Sign::Plus, BigUint::from(1u8) << width as usize)
                                - BigInt::from_biguint(Sign::Plus, magnitude);
                        format!("-{negative}")
                    } else {
                        magnitude.to_string()
                    }
                }
            };
            pad(out, &text, spec, decimal_width(width, signed));
        }
        'h' | 'o' | 'b' => {
            let group = match spec.conversion {
                'h' => 4,
                'o' => 3,
                _ => 1,
            };
            let digits = width.div_ceil(group).max(1);
            let mut text = String::with_capacity(digits as usize);
            for digit in (0..digits).rev() {
                let low = digit * group;
                let high = (low + group).min(width);
                let group_bits: Vec<FourStateBit> =
                    (low..high).map(|index| bits.bit(index)).collect();
                let character = match digit_of(&group_bits) {
                    Some(summary) => summary,
                    None => {
                        let value = group_bits
                            .iter()
                            .enumerate()
                            .filter(|(_, bit)| **bit == FourStateBit::One)
                            .fold(0u32, |sum, (index, _)| sum | 1 << index);
                        char::from_digit(value, 16).expect("digit")
                    }
                };
                text.push(character);
            }
            if spec.width == Some(0) {
                // `%0h`: the minimum, without leading zero digits.
                let trimmed = text.trim_start_matches('0');
                text = if trimmed.is_empty() {
                    "0".into()
                } else {
                    trimmed.into()
                };
            }
            let natural = text.len();
            pad(out, &text, spec, natural);
        }
        'c' => {
            let byte = (0..8.min(width)).fold(0u8, |byte, index| {
                if bits.bit(index) == FourStateBit::One {
                    byte | 1 << index
                } else {
                    byte
                }
            });
            pad(out, &char::from(byte).to_string(), spec, 1);
        }
        's' => {
            let bytes = width.div_ceil(8);
            let mut text = String::new();
            for byte in (0..bytes).rev() {
                let value = (0..8).fold(0u8, |value, bit| {
                    let index = byte * 8 + bit;
                    if index < width && bits.bit(index) == FourStateBit::One {
                        value | 1 << bit
                    } else {
                        value
                    }
                });
                // A NUL is no character; the field keeps its width.
                if value != 0 {
                    text.push(char::from(value));
                }
            }
            let natural = if spec.width == Some(0) {
                0
            } else {
                bytes as usize
            };
            pad(out, &text, spec, natural);
        }
        'u' | 'z' => {
            // Raw words, least significant first, each in little-endian byte
            // order (section 17.1.1.7): `%u` the value plane, `%z` value and
            // unknown planes interleaved.
            let words = width.div_ceil(32).max(1);
            for word in 0..words {
                let mut value = 0u32;
                let mut unknown = 0u32;
                for bit in 0..32 {
                    let index = word * 32 + bit;
                    if index >= width {
                        break;
                    }
                    match bits.bit(index) {
                        FourStateBit::One => value |= 1 << bit,
                        FourStateBit::Unknown => {
                            if spec.conversion == 'z' {
                                value |= 1 << bit;
                            }
                            unknown |= 1 << bit;
                        }
                        FourStateBit::HighImpedance => unknown |= 1 << bit,
                        FourStateBit::Zero => {}
                    }
                }
                for byte in value.to_le_bytes() {
                    out.push(char::from(byte));
                }
                if spec.conversion == 'z' {
                    for byte in unknown.to_le_bytes() {
                        out.push(char::from(byte));
                    }
                }
            }
        }
        _ => {}
    }
}

/// C's `%e`/`%f`/`%g`, which section 17.1.1 names for real values.
fn format_real(value: f64, spec: &Spec) -> String {
    if value.is_nan() {
        return "nan".into();
    }
    if value.is_infinite() {
        return if value < 0.0 {
            "-inf".into()
        } else {
            "inf".into()
        };
    }
    let precision = spec.precision.unwrap_or(6);
    match spec.conversion {
        'f' => format!("{value:.precision$}"),
        'e' => exponential(value, precision),
        _ => {
            let precision = precision.max(1);
            let exponent = exponent_of(value, precision - 1);
            let mut text = if exponent < -4 || exponent >= precision as i32 {
                exponential(value, precision - 1)
            } else {
                let decimals = (precision as i32 - 1 - exponent).max(0) as usize;
                format!("{value:.decimals$}")
            };
            if spec.alternate {
                if !text.contains('.') {
                    match text.find('e') {
                        Some(position) => text.insert(position, '.'),
                        None => text.push('.'),
                    }
                }
            } else if text.contains('.') {
                let (mantissa, exponent) = match text.find('e') {
                    Some(position) => text.split_at(position),
                    None => (text.as_str(), ""),
                };
                let mantissa = mantissa.trim_end_matches('0').trim_end_matches('.');
                text = format!("{mantissa}{exponent}");
            }
            text
        }
    }
}

/// The decimal exponent `%e` with `precision` digits would print.
fn exponent_of(value: f64, precision: usize) -> i32 {
    let text = format!("{value:.precision$e}");
    text.split_once('e')
        .and_then(|(_, exponent)| exponent.parse().ok())
        .unwrap_or(0)
}

fn exponential(value: f64, precision: usize) -> String {
    let text = format!("{value:.precision$e}");
    let Some((mantissa, exponent)) = text.split_once('e') else {
        return text;
    };
    let exponent: i32 = exponent.parse().unwrap_or(0);
    let sign = if exponent < 0 { '-' } else { '+' };
    format!("{mantissa}e{sign}{:02}", exponent.unsigned_abs())
}

/// `%t`: a time counted in the calling module's units, printed in
/// `$timeformat`'s units with its precision and suffix (section 17.3.2).
/// Integral times scale exactly; a real one scales in double precision.
fn format_time(value: &DigitalScalar, signed: bool, context: &FormatContext<'_>) -> String {
    let format = context.time_format;
    let shift = i32::from(context.time_unit) - i32::from(format.units);
    let precision = format.precision as usize;
    let number = match value {
        DigitalScalar::Real(real) => {
            let scaled = real * 10f64.powi(shift);
            format!("{scaled:.precision$}")
        }
        other => {
            let integer = match other {
                DigitalScalar::FourState(bits) => {
                    if bits.has_unknown() {
                        return "x".into();
                    }
                    let magnitude = BigInt::from_biguint(Sign::Plus, magnitude(bits));
                    if signed && bits.width() > 0 && bits.bit(bits.width() - 1) == FourStateBit::One
                    {
                        magnitude - (BigInt::from(1u8) << bits.width() as usize)
                    } else {
                        magnitude
                    }
                }
                DigitalScalar::Integer(integer) => BigInt::from(*integer),
                _ => return "x".into(),
            };
            decimal_scaled(integer, shift, precision)
        }
    };
    format!("{number}{}", format.suffix)
}

/// `integer * 10^shift`, rounded half away from zero to `precision` decimals.
fn decimal_scaled(integer: BigInt, shift: i32, precision: usize) -> String {
    let ten = BigInt::from(10u8);
    // Work in units of 10^-precision.
    let total = shift + precision as i32;
    let negative = integer.sign() == Sign::Minus;
    let magnitude = if negative { -integer } else { integer };
    let scaled = if total >= 0 {
        magnitude * ten.pow(total as u32)
    } else {
        let divisor = ten.pow(total.unsigned_abs());
        (magnitude * 2u8 + &divisor) / (divisor * 2u8)
    };
    let digits = scaled.to_string();
    let text = if precision == 0 {
        digits
    } else {
        let digits = format!("{digits:0>width$}", width = precision + 1);
        let (whole, fraction) = digits.split_at(digits.len() - precision);
        format!("{whole}.{fraction}")
    };
    if negative
        && text
            .chars()
            .any(|character| character != '0' && character != '.')
    {
        format!("-{text}")
    } else {
        text
    }
}

/// `$printtimescale`'s spelling of a power of ten (section 17.3.1): `1ns`,
/// `100ps`, `10us`.
pub(crate) fn time_unit_name(exponent: i8) -> String {
    let exponent = i32::from(exponent);
    let (base, unit) = match exponent {
        e if e >= 0 => (0, "s"),
        e if e >= -3 => (-3, "ms"),
        e if e >= -6 => (-6, "us"),
        e if e >= -9 => (-9, "ns"),
        e if e >= -12 => (-12, "ps"),
        _ => (-15, "fs"),
    };
    let multiplier = 10u64.pow((exponent - base).clamp(0, 18) as u32);
    format!("{multiplier}{unit}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bits(spelling: &str) -> DigitalScalar {
        let bits: Vec<FourStateBit> = spelling
            .chars()
            .map(|character| match character {
                '0' => FourStateBit::Zero,
                '1' => FourStateBit::One,
                'x' => FourStateBit::Unknown,
                _ => FourStateBit::HighImpedance,
            })
            .collect();
        DigitalScalar::FourState(FourStateValue::from_bits_msb_first(&bits))
    }

    fn unsigned(width: u32, value: u64) -> DigitalScalar {
        DigitalScalar::FourState(FourStateValue::from_u64(width, value))
    }

    fn signed(width: u32, value: i128) -> DigitalScalar {
        DigitalScalar::FourState(FourStateValue::from_integer(width, value))
    }

    fn run(arguments: &[FormatArgument<'_>]) -> String {
        run_with(arguments, DigitalRadix::Decimal, -10)
    }

    fn run_with(arguments: &[FormatArgument<'_>], radix: DigitalRadix, precision: i8) -> String {
        let time_format = TimeFormat::default_for(precision);
        format(
            arguments,
            &FormatContext {
                radix,
                scope: "f1",
                module: "f1",
                time_unit: -9,
                time_format: &time_format,
            },
        )
    }

    use FormatArgument::{Empty, Literal, RealTime, Value};

    #[test]
    fn decimal_widths_follow_the_value_width_and_signedness() {
        let a = unsigned(8, 5);
        let s = signed(8, -3);
        let i = signed(32, -42);
        assert_eq!(
            run(&[
                Literal("d:[%d][%0d][%5d][%05d]"),
                Value(&a, false),
                Value(&a, false),
                Value(&a, false),
                Value(&a, false)
            ]),
            "d:[  5][5][    5][00005]"
        );
        assert_eq!(
            run(&[
                Literal("sd:[%d][%0d][%d]"),
                Value(&s, true),
                Value(&s, true),
                Value(&i, true)
            ]),
            "sd:[  -3][-3][        -42]"
        );
        let big = unsigned(32, 123456);
        let w64 = unsigned(64, 0x0123_4567_89ab_cdef);
        assert_eq!(
            run(&[
                Literal("big:[%d][%h][%0d]"),
                Value(&big, false),
                Value(&w64, false),
                Value(&w64, false)
            ]),
            "big:[    123456][0123456789abcdef][81985529216486895]"
        );
        let all_ones = DigitalScalar::FourState(FourStateValue::splat(128, FourStateBit::One));
        let negative = signed(128, -5);
        assert_eq!(
            run(&[
                Literal("wide:[%d][%0d]"),
                Value(&all_ones, false),
                Value(&negative, true)
            ]),
            "wide:[340282366920938463463374607431768211455][-5]"
        );
    }

    #[test]
    fn radix_digits_and_unknown_bits_follow_section_17_1_1_3() {
        let w = unsigned(16, 0xbeef);
        let a = unsigned(8, 5);
        assert_eq!(
            run(&[
                Literal("h:[%h][%0h][%H][%5h]"),
                Value(&w, false),
                Value(&w, false),
                Value(&w, false),
                Value(&a, false)
            ]),
            "h:[beef][beef][beef][   05]"
        );
        let mixed = bits("01xz10x1");
        assert_eq!(
            run(&[
                Literal("b:[%b][%0b][%b]"),
                Value(&a, false),
                Value(&a, false),
                Value(&mixed, false)
            ]),
            "b:[00000101][101][01xz10x1]"
        );
        assert_eq!(
            run(&[
                Literal("o:[%o][%0o][%o]"),
                Value(&a, false),
                Value(&a, false),
                Value(&w, false)
            ]),
            "o:[005][5][137357]"
        );
        let xa = bits("xxxxxxxx");
        assert_eq!(
            run(&[
                Literal("x:[%d][%h][%b][%o]"),
                Value(&xa, false),
                Value(&xa, false),
                Value(&xa, false),
                Value(&xa, false)
            ]),
            "x:[  x][xx][xxxxxxxx][xxx]"
        );
        let z4 = bits("zzzz");
        assert_eq!(
            run(&[
                Literal("z:[%d][%h][%b][%o]"),
                Value(&z4, false),
                Value(&z4, false),
                Value(&z4, false),
                Value(&z4, false)
            ]),
            "z:[ z][z][zzzz][zz]"
        );
        assert_eq!(
            run(&[
                Literal("mx:[%h][%o][%d]"),
                Value(&mixed, false),
                Value(&mixed, false),
                Value(&mixed, false)
            ]),
            "mx:[XX][1XX][  X]"
        );
        let h12 = bits("0000xxxx0011");
        assert_eq!(run(&[Literal("h12:[%h]"), Value(&h12, false)]), "h12:[0x3]");
        let sx = bits("1x000000");
        assert_eq!(
            run(&[Literal("sd-x:[%d]"), Value(&sx, true)]),
            "sd-x:[   X]"
        );
        let one = unsigned(1, 1);
        assert_eq!(
            run(&[
                Literal("1bit:[%d][%b][%h]"),
                Value(&one, false),
                Value(&one, false),
                Value(&one, false)
            ]),
            "1bit:[1][1][1]"
        );
    }

    #[test]
    fn characters_strings_and_literals() {
        assert_eq!(
            run(&[
                Literal("c:[%c][%c]"),
                Value(&unsigned(8, 65), false),
                Value(&unsigned(16, 0x4142), false)
            ]),
            "c:[A][B]"
        );
        let abc = unsigned(24, 0x616263);
        assert_eq!(
            run(&[
                Literal("s:[%s][%0s][%10s]"),
                Value(&abc, false),
                Value(&abc, false),
                Value(&abc, false)
            ]),
            "s:[abc][abc][       abc]"
        );
        let hi = unsigned(24, 0x006869);
        assert_eq!(
            run(&[Literal("snull:[%s]"), Value(&hi, false)]),
            "snull:[ hi]"
        );
        assert_eq!(
            run(&[Literal("str-lit-d:[%d][%h]"), Literal("AB"), Literal("AB")]),
            "str-lit-d:[16706][4142]"
        );
        assert_eq!(
            run(&[Literal("pct:[%%][\\][\"][\t]")]),
            "pct:[%][\\][\"][\t]"
        );
        assert_eq!(run(&[Literal("m:[%m]")]), "m:[f1]");
        assert_eq!(run(&[Literal("l:[%l]")]), "l:[work.f1]");
    }

    /// IEEE 1364-2005 section 13.6: `%l` prints `library.cell`. A module a
    /// library binding placed is known by that name already.
    #[test]
    fn library_binding_prints_the_bound_library() {
        let time_format = TimeFormat::default_for(-10);
        let context = FormatContext {
            radix: DigitalRadix::Decimal,
            scope: "x1.u2",
            module: "gateLib.leaf",
            time_unit: -9,
            time_format: &time_format,
        };
        assert_eq!(
            format(&[Literal("%m %l")], &context),
            "x1.u2 gateLib.leaf"
        );
    }

    #[test]
    fn negative_reals_print_in_a_radix_as_their_64_bit_complement() {
        // Icarus 13: -1.0 and -2.6 print as 64-bit two's complements under
        // `%x`/`%b`, at their minimal width with or without `0`; 2.6 as 3.
        let (minus_one, minus_more, plus) = (
            DigitalScalar::Real(-1.0),
            DigitalScalar::Real(-2.6),
            DigitalScalar::Real(2.6),
        );
        assert_eq!(
            run(&[
                Literal("[%x][%0b][%0x][%x][%0d]"),
                Value(&minus_one, true),
                Value(&minus_more, true),
                Value(&minus_more, true),
                Value(&plus, true),
                Value(&minus_more, true)
            ]),
            format!(
                "[ffffffffffffffff][{}01][fffffffffffffffd][3][-3]",
                "1".repeat(62)
            )
        );
    }

    #[test]
    fn a_bare_realtime_prints_with_its_modules_precision_digits() {
        // Icarus 13 under `timescale 1ns/100ps at 1.5 ns:
        // `$display("g:", $realtime)` prints `g:1.5`, `"%f"` reads it as a
        // real, and `$realtime + 1.0` is no longer bare.
        let now = DigitalScalar::Real(1.5);
        let sum = DigitalScalar::Real(2.5);
        assert_eq!(
            run(&[
                Literal("g:"),
                RealTime(&now, 1),
                Literal("|f:%f|"),
                RealTime(&now, 1),
                Value(&sum, true),
                Literal("|"),
                RealTime(&now, 0)
            ]),
            "g:1.5|f:1.500000|2.50000|2"
        );
    }

    #[test]
    fn reals_follow_c_conversions() {
        let r = DigitalScalar::Real(3.25);
        assert_eq!(
            run(&[
                Literal("e:[%e][%f][%g][%0e][%10.3f][%-10.2f|]"),
                Value(&r, true),
                Value(&r, true),
                Value(&r, true),
                Value(&r, true),
                Value(&r, true),
                Value(&r, true)
            ]),
            "e:[3.250000e+00][3.250000][3.25][3.250000e+00][     3.250][3.25      |]"
        );
        let (small, large, tenth) = (
            DigitalScalar::Real(1.0e-7),
            DigitalScalar::Real(123456789.0),
            DigitalScalar::Real(0.0001),
        );
        assert_eq!(
            run(&[
                Literal("ee:[%e][%g][%g][%g]"),
                Value(&small, true),
                Value(&small, true),
                Value(&large, true),
                Value(&tenth, true)
            ]),
            "ee:[1.000000e-07][1e-07][1.23457e+08][0.0001]"
        );
        assert_eq!(
            run(&[
                Literal("real-as-int:[%d][%h]"),
                Value(&r, true),
                Value(&r, true)
            ]),
            "real-as-int:[3][3]"
        );
        let (a, i) = (unsigned(8, 5), signed(32, -42));
        assert_eq!(
            run(&[
                Literal("int-as-real:[%f][%e]"),
                Value(&a, false),
                Value(&i, true)
            ]),
            "int-as-real:[5.000000][-4.200000e+01]"
        );
        let rounding: Vec<DigitalScalar> = [2.5, 3.5, -2.5, 0.4999]
            .iter()
            .map(|value| DigitalScalar::Real(*value))
            .collect();
        assert_eq!(
            run(&[
                Literal("round:[%d][%d][%d][%d]"),
                Value(&rounding[0], true),
                Value(&rounding[1], true),
                Value(&rounding[2], true),
                Value(&rounding[3], true)
            ]),
            "round:[3][4][-3][0]"
        );
        let e2: Vec<DigitalScalar> = [12345.678, 2.5, 1.0 / 3.0, -0.000123]
            .iter()
            .map(|value| DigitalScalar::Real(*value))
            .collect();
        assert_eq!(
            run(&[
                Literal("e2:[%.2e][%.0f][%.10g][%15.4e]"),
                Value(&e2[0], true),
                Value(&e2[1], true),
                Value(&e2[2], true),
                Value(&e2[3], true)
            ]),
            "e2:[1.23e+04][2][0.3333333333][    -1.2300e-04]"
        );
        let defaults: Vec<DigitalScalar> = [1.0e-7, 123456.0, 0.1, 1.0e20, 100.0]
            .iter()
            .map(|value| DigitalScalar::Real(*value))
            .collect();
        assert_eq!(
            run(&[
                Value(&defaults[0], true),
                Literal(" "),
                Value(&defaults[1], true),
                Literal(" "),
                Value(&defaults[2], true),
                Literal(" "),
                Value(&defaults[3], true),
                Literal(" "),
                Value(&defaults[4], true)
            ]),
            "1.00000e-07 123456. 0.100000 1.00000e+20 100.000"
        );
    }

    #[test]
    fn unformatted_arguments_use_the_task_radix_and_empty_arguments_print_a_space() {
        let (a, s) = (unsigned(8, 5), signed(8, -3));
        assert_eq!(
            run(&[
                Literal("no format"),
                Value(&a, false),
                Literal(" and "),
                Value(&s, true)
            ]),
            "no format  5 and   -3"
        );
        assert_eq!(run(&[Value(&a, false), Value(&s, true)]), "  5  -3");
        assert_eq!(
            run(&[
                Literal("a=%d"),
                Value(&a, false),
                Literal(" s=%d"),
                Value(&s, true)
            ]),
            "a=  5 s=  -3"
        );
        let s_bits = unsigned(8, 0xfd);
        assert_eq!(
            run_with(
                &[Value(&a, false), Literal(" "), Value(&s_bits, true)],
                DigitalRadix::Binary,
                -10
            ),
            "00000101 11111101"
        );
        assert_eq!(
            run_with(
                &[Value(&a, false), Literal(" "), Value(&s_bits, true)],
                DigitalRadix::Hex,
                -10
            ),
            "05 fd"
        );
        assert_eq!(
            run_with(
                &[Value(&a, false), Literal(" "), Value(&s_bits, true)],
                DigitalRadix::Octal,
                -10
            ),
            "005 375"
        );
        assert_eq!(run(&[Value(&a, false), Empty, Value(&a, false)]), "  5   5");
        let seven = signed(32, 7);
        let r = DigitalScalar::Real(1.5);
        assert_eq!(
            run(&[
                Literal("real default:"),
                Value(&r, true),
                Literal(" int default:"),
                Value(&seven, true)
            ]),
            "real default:1.50000 int default:          7"
        );
        let time = unsigned(64, 0);
        assert_eq!(run(&[Value(&time, false)]), "                   0");
    }

    #[test]
    fn time_values_scale_from_the_module_unit_to_the_time_format() {
        // A 1 ns module in a design whose finest precision is 100 ps.
        let realtime = DigitalScalar::Real(1.3);
        let time = unsigned(64, 1);
        let zero = unsigned(64, 0);
        assert_eq!(
            run(&[
                Literal("t1:[%t][%10t][%-12t|]"),
                Value(&DigitalScalar::Real(0.0), true),
                Value(&zero, false),
                Value(&zero, false)
            ]),
            "t1:[                   0][         0][0           |]"
        );
        assert_eq!(
            run(&[
                Literal("t2:[%t][%0t][%t]"),
                Value(&realtime, true),
                Value(&realtime, true),
                Value(&time, false)
            ]),
            "t2:[                  13][13][                  10]"
        );
        let picoseconds = TimeFormat {
            units: -12,
            precision: 2,
            suffix: " ps".into(),
            min_width: 15,
        };
        let context = FormatContext {
            radix: DigitalRadix::Decimal,
            scope: "f2",
            module: "f2",
            time_unit: -9,
            time_format: &picoseconds,
        };
        assert_eq!(
            format(
                &[
                    Literal("t3:[%t][%0t]"),
                    Value(&realtime, true),
                    Value(&time, false)
                ],
                &context
            ),
            "t3:[     1300.00 ps][1000.00 ps]"
        );
        let nanoseconds = TimeFormat {
            units: -9,
            precision: 0,
            suffix: String::new(),
            min_width: 5,
        };
        let context = FormatContext {
            time_format: &nanoseconds,
            ..context
        };
        assert_eq!(
            format(&[Literal("t4:[%t]"), Value(&realtime, true)], &context),
            "t4:[    1]"
        );
        // Probe t1: a 1 ns module's 1.234 at 10 ps prints 123.
        assert_eq!(
            run_with(
                &[
                    Literal("%t|%0t"),
                    Value(&DigitalScalar::Real(1.234), true),
                    Value(&DigitalScalar::Real(1.234), true)
                ],
                DigitalRadix::Decimal,
                -11
            ),
            "                 123|123"
        );
    }

    #[test]
    fn strengths_print_as_strong_or_high_impedance_until_strengths_land() {
        assert_eq!(
            run(&[
                Literal("v:[%v][%v][%v]"),
                Value(&bits("1"), false),
                Value(&bits("x"), false),
                Value(&bits("z"), false)
            ]),
            "v:[St1][StX][HiZ]"
        );
    }

    #[test]
    fn raw_formats_write_little_endian_words() {
        assert_eq!(
            run(&[Literal("u:[%u]"), Value(&unsigned(8, 0x41), false)]),
            "u:[A\0\0\0]"
        );
        assert_eq!(
            run(&[Literal("z:[%z]"), Value(&unsigned(8, 0x41), false)]),
            "z:[A\0\0\0\0\0\0\0]"
        );
    }

    #[test]
    fn time_unit_names_match_printtimescale() {
        assert_eq!(time_unit_name(-9), "1ns");
        assert_eq!(time_unit_name(-10), "100ps");
        assert_eq!(time_unit_name(-5), "10us");
        assert_eq!(time_unit_name(0), "1s");
        assert_eq!(time_unit_name(-15), "1fs");
    }
}

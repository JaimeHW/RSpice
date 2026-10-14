//! `$fscanf` and `$sscanf` (IEEE 1364-2005 section 17.2.4.3).
//!
//! The format is read the way C's `scanf` reads one: white space matches any
//! run of white space, an ordinary character matches itself, and a
//! conversion skips leading white space (except `%c`, `%u`, `%z` and `%m`)
//! and then reads as much of its field as it can. Where section 17.2.4.3
//! leaves a choice open the choice is Icarus Verilog's, so that a test
//! written against Icarus reads the same here; each expected value in the
//! tests below is an Icarus 13.0 run of an RSpice-authored probe
//! (`scratchpad/p2l5/probe/p1.v`, `p2.v`).
//!
//! - `%d` reads an optional sign and decimal digits, or one `x`, `z` or `?`,
//!   which makes the whole value unknown or high impedance.
//! - `%b`, `%o` and `%h` read digits of their radix mixed with `x`, `z` and
//!   `?`; a leading `x` or `z` extends to the target's width, as a literal's
//!   does (section 3.5.1).
//! - `_` may separate digits, but not start a number.
//! - The call returns -1 (EOF) only when the input is exhausted before
//!   anything is read; otherwise it returns the number of targets assigned,
//!   `%m` included.

use num_bigint::{BigInt, Sign};
use rspice_veriloga::canonical_ir::digital::DigitalValueClass;
use rspice_veriloga::canonical_ir::digital_eval::DigitalScalar;
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::four_state::FourStateBit;

use super::OpenFile;

/// Where the scanner reads from.
pub(crate) trait ScanSource {
    /// The next byte, or `None` at the end (which an open file records as
    /// its end-of-file indicator, as C's `getc` does).
    fn peek(&mut self) -> Option<u8>;
    fn advance(&mut self);
}

/// A string being scanned (`$sscanf`).
pub(crate) struct TextSource<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> TextSource<'a> {
    pub(crate) fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, at: 0 }
    }
}

impl ScanSource for TextSource<'_> {
    fn peek(&mut self) -> Option<u8> {
        self.bytes.get(self.at).copied()
    }
    fn advance(&mut self) {
        self.at += 1;
    }
}

impl ScanSource for OpenFile {
    fn peek(&mut self) -> Option<u8> {
        let byte = OpenFile::peek(self);
        if byte.is_none() {
            self.eof = true;
        }
        byte
    }
    fn advance(&mut self) {
        self.next_byte();
    }
}

/// One conversion's value, before it is fitted to its target.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Scanned {
    /// Digits of a radix, most significant first.
    Bits(Vec<FourStateBit>),
    /// A value of exactly these bits (`%u`, `%z`, `%c`).
    Word(FourStateValue),
    Integer(BigInt),
    Real(f64),
    /// Characters, first first (`%s`, `%m`).
    Text(Vec<u8>),
}

/// What the call reads besides its input.
pub(crate) struct ScanContext<'a> {
    /// `%m`: the calling scope.
    pub scope: &'a str,
    /// `%t`: how many powers of ten the `$timeformat` unit is above the
    /// calling module's.
    pub time_shift: i32,
    /// `%t`: the `$timeformat` precision the value is rounded to first.
    pub time_precision: u8,
}

/// `%t`: a value in `$timeformat` units, rounded to its precision and then
/// scaled to the module's unit by an exact power of ten, as Icarus does, so
/// `8.125` read with two digits of precision is `8.13`.
fn scan_time(value: f64, context: &ScanContext<'_>) -> f64 {
    let digits = 10f64.powi(i32::from(context.time_precision));
    let rounded = (value * digits).round() / digits;
    let scale = 10f64.powi(context.time_shift.abs());
    if context.time_shift >= 0 {
        rounded * scale
    } else {
        rounded / scale
    }
}

/// A whole call's result.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ScanOutcome {
    /// Assigned targets, or -1 when the input ran out before anything was
    /// read.
    pub count: i32,
    /// One per target, in order; `None` leaves the target as it was.
    pub values: Vec<Option<Scanned>>,
}

fn is_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

struct Scanner<'s> {
    source: &'s mut dyn ScanSource,
    /// Whether any input byte was consumed.
    consumed: bool,
    /// Whether the input ran out.
    exhausted: bool,
}

impl Scanner<'_> {
    fn peek(&mut self) -> Option<u8> {
        let byte = self.source.peek();
        if byte.is_none() {
            self.exhausted = true;
        }
        byte
    }

    fn take(&mut self) {
        self.source.advance();
        self.consumed = true;
    }

    fn skip_space(&mut self) {
        while self.peek().is_some_and(is_space) {
            self.take();
        }
    }

    /// Read digits accepted by `digit`, with `_` between them, up to `width`
    /// characters.
    fn digits(&mut self, width: &mut usize, digit: impl Fn(u8) -> bool) -> Vec<u8> {
        let mut out = Vec::new();
        while *width > 0 {
            let Some(byte) = self.peek() else { break };
            if digit(byte) || (byte == b'_' && !out.is_empty()) {
                self.take();
                *width -= 1;
                if byte != b'_' {
                    out.push(byte);
                }
            } else {
                break;
            }
        }
        out
    }
}

/// Scan `source` by `format`, for `targets` assignable targets: each one's
/// class, which `%u`, `%z` and `%c` read their widths from.
pub(crate) fn scan(
    source: &mut dyn ScanSource,
    format: &[u8],
    targets: &[DigitalValueClass],
    context: &ScanContext<'_>,
) -> ScanOutcome {
    let mut scanner = Scanner {
        source,
        consumed: false,
        exhausted: false,
    };
    let mut values = vec![None; targets.len()];
    let mut count = 0;
    let mut next = 0;
    let mut position = 0;
    // Whether a conversion was attempted and found the input at its end
    // before anything had been read.
    let mut ended_early = false;
    'format: while position < format.len() {
        let byte = format[position];
        position += 1;
        if is_space(byte) {
            scanner.skip_space();
            continue;
        }
        if byte != b'%' {
            match scanner.peek() {
                Some(input) if input == byte => scanner.take(),
                None => {
                    ended_early = !scanner.consumed;
                    break;
                }
                _ => break,
            }
            continue;
        }
        let suppress = format.get(position) == Some(&b'*');
        if suppress {
            position += 1;
        }
        let mut width_digits = 0usize;
        let mut explicit = false;
        while let Some(digit) = format.get(position).filter(|byte| byte.is_ascii_digit()) {
            width_digits = width_digits
                .saturating_mul(10)
                .saturating_add(usize::from(digit - b'0'));
            explicit = true;
            position += 1;
        }
        let Some(&conversion) = format.get(position) else {
            break;
        };
        position += 1;
        let mut width = if explicit && width_digits > 0 {
            width_digits
        } else {
            usize::MAX
        };
        let conversion = conversion.to_ascii_lowercase();
        let class = (!suppress).then(|| targets.get(next).copied()).flatten();
        let scanned = match conversion {
            b'%' => {
                scanner.skip_space();
                match scanner.peek() {
                    Some(b'%') => {
                        scanner.take();
                        continue;
                    }
                    None => {
                        ended_early = !scanner.consumed;
                        break;
                    }
                    _ => break,
                }
            }
            b'm' => Some(Scanned::Text(context.scope.as_bytes().to_vec())),
            b'c' => match scanner.peek() {
                Some(byte) => {
                    scanner.take();
                    Some(Scanned::Word(FourStateValue::from_u64(8, u64::from(byte))))
                }
                None => {
                    ended_early = !scanner.consumed;
                    break;
                }
            },
            b'u' | b'z' => {
                // A field width counts bits; without one, the target's
                // width, and one word for a suppressed field.
                let bits = if explicit && width_digits > 0 {
                    u32::try_from(width_digits).unwrap_or(u32::MAX)
                } else {
                    match class {
                        Some(DigitalValueClass::FourState { width, .. }) => width,
                        _ => 32,
                    }
                };
                match raw_words(&mut scanner, bits, conversion == b'z') {
                    Some(value) => Some(Scanned::Word(value)),
                    // A binary read is atomic: a short word is the end of
                    // the input even after bytes were read, so a call that
                    // assigned nothing yet returns EOF, as Icarus's does.
                    None => {
                        ended_early = true;
                        break;
                    }
                }
            }
            _ => {
                scanner.skip_space();
                if scanner.peek().is_none() {
                    ended_early = !scanner.consumed;
                    break;
                }
                match conversion {
                    b'd' => decimal(&mut scanner, &mut width),
                    b'b' => radix(&mut scanner, &mut width, 1),
                    b'o' => radix(&mut scanner, &mut width, 3),
                    b'h' | b'x' => radix(&mut scanner, &mut width, 4),
                    b'e' | b'f' | b'g' => real(&mut scanner, &mut width).map(Scanned::Real),
                    b't' => real(&mut scanner, &mut width)
                        .map(|value| Scanned::Real(scan_time(value, context))),
                    b's' => {
                        let mut text = Vec::new();
                        while width > 0 {
                            match scanner.peek() {
                                Some(byte) if !is_space(byte) => {
                                    scanner.take();
                                    text.push(byte);
                                    width -= 1;
                                }
                                _ => break,
                            }
                        }
                        (!text.is_empty()).then_some(Scanned::Text(text))
                    }
                    b'v' => strength(&mut scanner),
                    // An unknown conversion ends the scan, as C's does.
                    _ => break 'format,
                }
            }
        };
        let Some(scanned) = scanned else { break };
        if !suppress {
            if let Some(slot) = values.get_mut(next) {
                *slot = Some(scanned);
                count += 1;
            }
            next += 1;
        }
    }
    if count == 0 && ended_early && scanner.exhausted {
        count = -1;
    }
    ScanOutcome { count, values }
}

fn decimal(scanner: &mut Scanner<'_>, width: &mut usize) -> Option<Scanned> {
    let first = scanner.peek()?;
    if matches!(first, b'x' | b'X' | b'z' | b'Z' | b'?') {
        scanner.take();
        *width -= 1;
        let bit = if matches!(first, b'z' | b'Z') {
            FourStateBit::HighImpedance
        } else {
            FourStateBit::Unknown
        };
        return Some(Scanned::Bits(vec![bit]));
    }
    let negative = first == b'-';
    if matches!(first, b'-' | b'+') {
        scanner.take();
        *width -= 1;
    }
    let digits = scanner.digits(width, |byte| byte.is_ascii_digit());
    if digits.is_empty() {
        return None;
    }
    let magnitude = BigInt::parse_bytes(&digits, 10)?;
    Some(Scanned::Integer(if negative {
        -magnitude
    } else {
        magnitude
    }))
}

fn radix(scanner: &mut Scanner<'_>, width: &mut usize, bits_per_digit: u32) -> Option<Scanned> {
    let radix = 1u32 << bits_per_digit;
    let digits = scanner.digits(width, |byte| {
        matches!(byte, b'x' | b'X' | b'z' | b'Z' | b'?')
            || char::from(byte)
                .to_digit(16)
                .is_some_and(|digit| digit < radix)
    });
    if digits.is_empty() {
        return None;
    }
    let mut bits = Vec::with_capacity(digits.len() * bits_per_digit as usize);
    for digit in digits {
        match digit {
            b'x' | b'X' | b'?' => {
                bits.extend(std::iter::repeat_n(
                    FourStateBit::Unknown,
                    bits_per_digit as usize,
                ));
            }
            b'z' | b'Z' => {
                bits.extend(std::iter::repeat_n(
                    FourStateBit::HighImpedance,
                    bits_per_digit as usize,
                ));
            }
            _ => {
                let value = char::from(digit).to_digit(16).expect("checked digit");
                for bit in (0..bits_per_digit).rev() {
                    bits.push(if value >> bit & 1 == 1 {
                        FourStateBit::One
                    } else {
                        FourStateBit::Zero
                    });
                }
            }
        }
    }
    Some(Scanned::Bits(bits))
}

/// C's floating-point field: sign, digits, a point, digits, an exponent.
fn real(scanner: &mut Scanner<'_>, width: &mut usize) -> Option<f64> {
    let mut text = String::new();
    let take = |scanner: &mut Scanner<'_>, width: &mut usize, text: &mut String| {
        if *width == 0 {
            return false;
        }
        let Some(byte) = scanner.peek() else {
            return false;
        };
        scanner.take();
        *width -= 1;
        text.push(char::from(byte));
        true
    };
    if matches!(scanner.peek(), Some(b'+' | b'-')) {
        take(scanner, width, &mut text);
    }
    let mut digits = 0;
    while *width > 0 && scanner.peek().is_some_and(|byte| byte.is_ascii_digit()) {
        take(scanner, width, &mut text);
        digits += 1;
    }
    if *width > 0 && scanner.peek() == Some(b'.') {
        take(scanner, width, &mut text);
        while *width > 0 && scanner.peek().is_some_and(|byte| byte.is_ascii_digit()) {
            take(scanner, width, &mut text);
            digits += 1;
        }
    }
    if digits == 0 {
        return None;
    }
    if *width > 0 && matches!(scanner.peek(), Some(b'e' | b'E')) {
        take(scanner, width, &mut text);
        if *width > 0 && matches!(scanner.peek(), Some(b'+' | b'-')) {
            take(scanner, width, &mut text);
        }
        let mut exponent = 0;
        while *width > 0 && scanner.peek().is_some_and(|byte| byte.is_ascii_digit()) {
            take(scanner, width, &mut text);
            exponent += 1;
        }
        // An exponent needs a digit: `2.ea` and `2.e` match nothing, as in
        // Icarus, and what was read stays read.
        if exponent == 0 {
            return None;
        }
    }
    text.parse().ok()
}

/// `%u` and `%z`: raw 32-bit words, least significant first, each in
/// little-endian byte order (section 17.2.4.3); `%z` follows each value word
/// with its unknown plane. The value is `width` bits, read as whole words:
/// a `%16u` consumes one word and keeps its low half, as Icarus does.
fn raw_words(scanner: &mut Scanner<'_>, width: u32, planes: bool) -> Option<FourStateValue> {
    let words = width.div_ceil(32).max(1);
    let read_word = |scanner: &mut Scanner<'_>| -> Option<u32> {
        let mut bytes = [0u8; 4];
        for byte in &mut bytes {
            *byte = scanner.peek()?;
            scanner.take();
        }
        Some(u32::from_le_bytes(bytes))
    };
    // Read every word before sizing the value, so a width far past the
    // input allocates no more than the input holds.
    let mut read = Vec::new();
    for _ in 0..words {
        let aval = read_word(scanner)?;
        let bval = if planes { read_word(scanner)? } else { 0 };
        read.push((aval, bval));
    }
    let mut value = FourStateValue::zero(width.max(1));
    for (word, (aval, bval)) in (0u32..).zip(read) {
        for bit in 0..32 {
            let index = word * 32 + bit;
            if index >= width {
                break;
            }
            let state = match (aval >> bit & 1, bval >> bit & 1) {
                (0, 0) => FourStateBit::Zero,
                (1, 0) => FourStateBit::One,
                (0, _) => FourStateBit::HighImpedance,
                _ => FourStateBit::Unknown,
            };
            value.set_bit(index, state);
        }
    }
    Some(value)
}

/// `%v`: one strength-and-value triple (`St0`, `Pu1`, `HiZ`, `StX`), read
/// as its value.
fn strength(scanner: &mut Scanner<'_>) -> Option<Scanned> {
    let mut text = Vec::new();
    for _ in 0..3 {
        let byte = scanner.peek()?;
        scanner.take();
        text.push(byte);
    }
    let bit = match text.as_slice() {
        [b'H', b'i', b'Z'] => FourStateBit::HighImpedance,
        [_, _, b'0'] => FourStateBit::Zero,
        [_, _, b'1'] => FourStateBit::One,
        [_, _, b'X' | b'x'] => FourStateBit::Unknown,
        _ => return None,
    };
    Some(Scanned::Bits(vec![bit]))
}

/// Fit a scanned value to its target, as an assignment would.
pub(crate) fn fit(scanned: &Scanned, class: DigitalValueClass) -> DigitalScalar {
    match class {
        DigitalValueClass::Real => DigitalScalar::Real(match scanned {
            Scanned::Real(value) => *value,
            Scanned::Integer(value) => value.to_string().parse().unwrap_or(0.0),
            Scanned::Bits(bits) => FourStateValue::from_bits_msb_first(bits)
                .to_real(false)
                .unwrap_or(0.0),
            Scanned::Word(value) => value.to_real(false).unwrap_or(0.0),
            Scanned::Text(bytes) => packed_bytes(64, bytes).to_real(false).unwrap_or(0.0),
        }),
        DigitalValueClass::FourState { width, .. } => DigitalScalar::FourState(match scanned {
            Scanned::Bits(bits) => literal_bits(bits, width),
            Scanned::Word(value) => value.resized(width),
            Scanned::Integer(value) => integer_bits(value, width),
            Scanned::Real(value) => FourStateValue::from_real(width, *value)
                .unwrap_or_else(|_| FourStateValue::splat(width, FourStateBit::Unknown)),
            Scanned::Text(bytes) => packed_bytes(width, bytes),
        }),
    }
}

/// Digits to `width` bits: a leading `x` or `z` extends with itself, anything
/// else with zero (section 3.5.1); extra digits drop from the top.
fn literal_bits(bits: &[FourStateBit], width: u32) -> FourStateValue {
    let fill = match bits.first() {
        Some(FourStateBit::Unknown) => FourStateBit::Unknown,
        Some(FourStateBit::HighImpedance) => FourStateBit::HighImpedance,
        _ => FourStateBit::Zero,
    };
    let mut value = FourStateValue::splat(width, fill);
    for (index, bit) in bits.iter().rev().enumerate() {
        if index as u32 >= width {
            break;
        }
        value.set_bit(index as u32, *bit);
    }
    value
}

/// A signed integer to `width` bits of two's complement.
fn integer_bits(value: &BigInt, width: u32) -> FourStateValue {
    let modulus = BigInt::from(1u8) << width as usize;
    let mut wrapped = value % &modulus;
    if wrapped.sign() == Sign::Minus {
        wrapped += &modulus;
    }
    let (_, bytes) = wrapped.to_bytes_le();
    let mut out = FourStateValue::zero(width);
    for (index, byte) in bytes.iter().enumerate() {
        for bit in 0..8 {
            let position = index as u32 * 8 + bit;
            if position < width && byte >> bit & 1 == 1 {
                out.set_bit(position, FourStateBit::One);
            }
        }
    }
    out
}

/// Characters as a packed string value: the last character in the least
/// significant byte, zero-filled above, the first characters dropped when the
/// target is too narrow.
pub(crate) fn packed_bytes(width: u32, bytes: &[u8]) -> FourStateValue {
    let mut value = FourStateValue::zero(width);
    for (position, byte) in bytes.iter().rev().enumerate() {
        for bit in 0..8 {
            let index = position as u32 * 8 + bit;
            if index >= width {
                return value;
            }
            if byte >> bit & 1 == 1 {
                value.set_bit(index, FourStateBit::One);
            }
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    fn class(width: u32) -> DigitalValueClass {
        DigitalValueClass::FourState {
            width,
            signed: false,
        }
    }

    fn scan_text(input: &str, format: &str, widths: &[u32]) -> (i32, Vec<Option<String>>) {
        let classes: Vec<_> = widths.iter().map(|width| class(*width)).collect();
        let outcome = scan(
            &mut TextSource::new(input.as_bytes()),
            format.as_bytes(),
            &classes,
            &ScanContext {
                scope: "p1",
                time_shift: 0,
                time_precision: 0,
            },
        );
        let values = outcome
            .values
            .iter()
            .zip(&classes)
            .map(|(value, class)| {
                value.as_ref().map(|value| match fit(value, *class) {
                    DigitalScalar::FourState(bits) => bits
                        .bits_msb_first()
                        .iter()
                        .map(|bit| match bit {
                            FourStateBit::Zero => '0',
                            FourStateBit::One => '1',
                            FourStateBit::Unknown => 'x',
                            FourStateBit::HighImpedance => 'z',
                        })
                        .collect(),
                    other => format!("{other:?}"),
                })
            })
            .collect();
        (outcome.count, values)
    }

    fn hex(value: u64, width: u32) -> String {
        format!("{value:0width$b}", width = width as usize)
    }

    /// Icarus 13.0, probes `p1.v` and `p2.v`.
    #[test]
    fn returns_and_conversions_match_icarus() {
        assert_eq!(scan_text("", "%d", &[32]), (-1, vec![None]));
        assert_eq!(scan_text("   ", "%d", &[32]), (0, vec![None]));
        assert_eq!(scan_text("abc", "%d", &[32]), (0, vec![None]));
        assert_eq!(scan_text("", "x%d", &[32]).0, -1);
        assert_eq!(scan_text("abc", "x%d", &[32]).0, 0);
        assert_eq!(scan_text("ab", "ab%d", &[32]).0, 0);
        let (count, values) = scan_text("-12 x", "%d %d", &[32, 32]);
        assert_eq!(count, 2);
        assert_eq!(values[0].as_deref(), Some(hex(0xffff_fff4, 32).as_str()));
        assert_eq!(values[1].as_deref(), Some("x".repeat(32).as_str()));
        let (count, values) = scan_text("1z 3_4", "%d %d", &[32, 32]);
        assert_eq!(count, 2);
        assert_eq!(values[0].as_deref(), Some(hex(1, 32).as_str()));
        assert_eq!(values[1].as_deref(), Some("z".repeat(32).as_str()));
        let (count, values) = scan_text("ff 10 77", "%h %b %o", &[32, 32, 16]);
        assert_eq!(count, 3);
        assert_eq!(values[0].as_deref(), Some(hex(0xff, 32).as_str()));
        assert_eq!(values[1].as_deref(), Some(hex(2, 32).as_str()));
        assert_eq!(values[2].as_deref(), Some(hex(0x3f, 16).as_str()));
        let (_, values) = scan_text("12345", "%2d%d", &[32, 32]);
        assert_eq!(values[0].as_deref(), Some(hex(12, 32).as_str()));
        assert_eq!(values[1].as_deref(), Some(hex(345, 32).as_str()));
        let (count, values) = scan_text("9 8", "%*d %d", &[32]);
        assert_eq!(
            (count, values[0].as_deref()),
            (1, Some(hex(8, 32).as_str()))
        );
        assert_eq!(scan_text("1 2", "%d,%d", &[32, 32]).0, 1);
        let (_, values) = scan_text("hello world", "%s", &[16]);
        assert_eq!(values[0].as_deref(), Some(hex(0x6c6f, 16).as_str()));
        assert_eq!(scan_text("-5", "%h", &[32]).0, 0);
        assert_eq!(
            scan_text("+7", "%d", &[32]).1[0].as_deref(),
            Some(hex(7, 32).as_str())
        );
        assert_eq!(
            scan_text("99999999999", "%d", &[32]).1[0].as_deref(),
            Some(hex(0x4876_e7ff, 32).as_str())
        );
        assert_eq!(
            scan_text("x5", "%d", &[32]).1[0].as_deref(),
            Some("x".repeat(32).as_str())
        );
        assert_eq!(
            scan_text("1x", "%b", &[32]).1[0].as_deref(),
            Some(format!("{}1x", "0".repeat(30)).as_str())
        );
        assert_eq!(
            scan_text("z_1", "%h", &[32]).1[0].as_deref(),
            Some(format!("{}0001", "z".repeat(28)).as_str())
        );
        assert_eq!(
            scan_text("?", "%b", &[32]).1[0].as_deref(),
            Some("x".repeat(32).as_str())
        );
        assert_eq!(
            scan_text("5_6", "%d", &[32]).1[0].as_deref(),
            Some(hex(56, 32).as_str())
        );
        assert_eq!(scan_text("_5", "%d", &[32]).0, 0);
        assert_eq!(scan_text("-", "%d", &[32]).0, 0);
        assert_eq!(
            scan_text(" q", "%c", &[32]).1[0].as_deref(),
            Some(hex(0x20, 32).as_str())
        );
        assert_eq!(
            scan_text("1.5", "%d", &[32]).1[0].as_deref(),
            Some(hex(1, 32).as_str())
        );
        assert_eq!(
            scan_text("2.6", "%f", &[32]).1[0].as_deref(),
            Some(hex(3, 32).as_str())
        );
        assert_eq!(
            scan_text("%5", "%%%d", &[32]).1[0].as_deref(),
            Some(hex(5, 32).as_str())
        );
        let (count, values) = scan_text("abcdefghijk", "%5s%s", &[64, 32]);
        assert_eq!(count, 2);
        assert_eq!(
            values[0].as_deref(),
            Some(hex(0x0061_6263_6465, 64).as_str())
        );
        assert_eq!(values[1].as_deref(), Some(hex(0x6869_6a6b, 32).as_str()));
        let (count, values) = scan_text("12", "%m", &[96]);
        assert_eq!(count, 1);
        assert_eq!(values[0].as_deref(), Some(hex(0x7031, 96).as_str()));
    }

    #[test]
    fn reals_read_c_syntax() {
        let real = |input: &str| {
            let outcome = scan(
                &mut TextSource::new(input.as_bytes()),
                b"%g",
                &[DigitalValueClass::Real],
                &ScanContext {
                    scope: "",
                    time_shift: 0,
                    time_precision: 0,
                },
            );
            outcome.values[0]
                .as_ref()
                .map(|value| fit(value, DigitalValueClass::Real))
        };
        assert_eq!(real("-1.5e-1"), Some(DigitalScalar::Real(-0.15)));
        assert_eq!(real(".5"), Some(DigitalScalar::Real(0.5)));
        assert_eq!(real("3.5e2"), Some(DigitalScalar::Real(350.0)));
        assert_eq!(real("e5"), None);
        // An exponent with no digit matches nothing (ivtest `scanf4`).
        assert_eq!(real("2.ea"), None);
        assert_eq!(real("2.e+a"), None);
        assert_eq!(real("2.e"), None);
    }

    /// `%t` rounds to the `$timeformat` precision before it scales (ivtest
    /// `scanf2` and `scanf4`, which Icarus passes).
    #[test]
    fn times_round_to_the_timeformat_precision_then_scale() {
        let time = |input: &str, time_shift: i32, time_precision: u8| {
            let outcome = scan(
                &mut TextSource::new(input.as_bytes()),
                b"%t",
                &[DigitalValueClass::Real],
                &ScanContext {
                    scope: "",
                    time_shift,
                    time_precision,
                },
            );
            match outcome.values[0]
                .as_ref()
                .map(|value| fit(value, DigitalValueClass::Real))
            {
                Some(DigitalScalar::Real(value)) => value,
                other => panic!("{other:?}"),
            }
        };
        for (input, shift, precision, expected) in [
            ("8.125", 6, 2, 8_130_000.0),
            ("10.0625", 0, 3, 10.063),
            ("10.03125", 0, 3, 10.031),
            ("10.543", 0, 1, 10.5),
            ("1234567.89", -2, 2, 12_345.678_9),
        ] {
            let value = time(input, shift, precision);
            assert!(
                (value - expected).abs() <= 1e-9 * expected,
                "{input}: {value} against {expected}"
            );
        }
    }

    /// `%u` reads whole words: a field width counts bits, rounded up to
    /// words, and a short word is the end of the input even after other
    /// conversions consumed bytes (ivtest `fscanf_u` and `fscanf_u_warn`).
    #[test]
    fn raw_words_take_their_width_in_whole_words() {
        let words = |values: &[u32]| -> Vec<u8> {
            values
                .iter()
                .flat_map(|value| value.to_le_bytes())
                .collect()
        };
        let raw = |bytes: &[u8], format: &str, widths: &[u32]| {
            let classes: Vec<_> = widths.iter().map(|width| class(*width)).collect();
            let outcome = scan(
                &mut TextSource::new(bytes),
                format.as_bytes(),
                &classes,
                &ScanContext {
                    scope: "",
                    time_shift: 0,
                    time_precision: 0,
                },
            );
            let values: Vec<Option<u64>> = outcome
                .values
                .iter()
                .zip(&classes)
                .map(|(value, class)| {
                    value.as_ref().and_then(|value| match fit(value, *class) {
                        DigitalScalar::FourState(bits) => bits.to_u64(),
                        _ => None,
                    })
                })
                .collect();
            (outcome.count, values)
        };
        let one = words(&[0x0820_a08e]);
        assert_eq!(raw(&one, "%16u", &[32]), (1, vec![Some(0xa08e)]));
        assert_eq!(
            raw(&one, "%16u%u", &[32, 32]),
            (1, vec![Some(0xa08e), None])
        );
        assert_eq!(raw(&one, "%32u", &[64]), (1, vec![Some(0x0820_a08e)]));
        let three = words(&[1, 2, 3]);
        assert_eq!(raw(&three, "%*u%u", &[32]), (1, vec![Some(2)]));
        assert_eq!(raw(&three, "%*64u%u", &[32]), (1, vec![Some(3)]));
        assert_eq!(raw(&three, "%*33u%u", &[32]), (1, vec![Some(3)]));
        // Short words.
        assert_eq!(raw(&one[..2], "%u", &[32]), (-1, vec![None]));
        assert_eq!(raw(&one, "%u", &[64]), (-1, vec![None]));
        assert_eq!(raw(&three[..6], "%*u%u", &[32]), (-1, vec![None]));
        assert_eq!(
            raw(&three[..6], "%u%u", &[32, 32]),
            (1, vec![Some(1), None])
        );
    }

    #[test]
    fn raw_words_read_value_and_unknown_planes() {
        // `%z` of 4'b1xz0: value plane 0b1100, unknown plane 0b0110.
        let bytes = [0x0c, 0, 0, 0, 0x06, 0, 0, 0];
        let outcome = scan(
            &mut TextSource::new(&bytes),
            b"%z",
            &[class(4)],
            &ScanContext {
                scope: "",
                time_shift: 0,
                time_precision: 0,
            },
        );
        assert_eq!(outcome.count, 1);
        let DigitalScalar::FourState(value) = fit(outcome.values[0].as_ref().unwrap(), class(4))
        else {
            panic!("four-state")
        };
        assert_eq!(
            value.bits_msb_first(),
            [
                FourStateBit::One,
                FourStateBit::Unknown,
                FourStateBit::HighImpedance,
                FourStateBit::Zero
            ]
        );
        // A short word is the end of the input.
        let outcome = scan(
            &mut TextSource::new(&bytes[..3]),
            b"%u",
            &[class(32)],
            &ScanContext {
                scope: "",
                time_shift: 0,
                time_precision: 0,
            },
        );
        assert_eq!(outcome.count, -1);
    }
}

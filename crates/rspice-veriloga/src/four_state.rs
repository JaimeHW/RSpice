//! Four-state (`0 1 x z`) literal decoding for the IEEE 1364-2005 digital
//! subset that Verilog-AMS embeds.
//!
//! The continuous half of the language has no representation for `x` or `z`:
//! [`crate::numeric_literal`] therefore refuses any based literal whose digits
//! contain one, and that refusal stays exactly as it was. This module decodes
//! the same syntax into an explicit per-bit value so the *discrete* half can
//! carry it faithfully, without ever handing a number to the analog IR.
//!
//! The two decoders are deliberately separate. A single decoder returning
//! "either an integer or a bit vector" would put a four-state value one
//! `match` arm away from every analog consumer of a numeric literal; keeping
//! them apart means an analog expression cannot acquire one by accident.

use smol_str::SmolStr;

/// Radix of a based literal, retained so the source spelling survives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteralBase {
    Binary,
    Octal,
    Decimal,
    Hex,
}

impl LiteralBase {
    /// Bits contributed by one digit of this base.
    ///
    /// Decimal has no per-digit bit expansion, which is why IEEE 1364-2005
    /// section 3.5.1 permits an `x`/`z` decimal literal to consist of that one
    /// digit and nothing else.
    const fn bits_per_digit(self) -> Option<u32> {
        match self {
            Self::Binary => Some(1),
            Self::Octal => Some(3),
            Self::Hex => Some(4),
            Self::Decimal => None,
        }
    }

    const fn radix(self) -> u32 {
        match self {
            Self::Binary => 2,
            Self::Octal => 8,
            Self::Decimal => 10,
            Self::Hex => 16,
        }
    }
}

/// One bit of a four-state value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FourStateBit {
    Zero,
    One,
    /// `x` / `X`: unknown.
    Unknown,
    /// `z` / `Z` / `?`: high impedance. IEEE 1364-2005 section 3.5.1 makes `?`
    /// another spelling of `z` inside a literal, so both decode identically
    /// and only the retained raw text distinguishes them.
    HighImpedance,
}

impl FourStateBit {
    /// Canonical single-character spelling, used by diagnostics.
    pub const fn as_char(self) -> char {
        match self {
            Self::Zero => '0',
            Self::One => '1',
            Self::Unknown => 'x',
            Self::HighImpedance => 'z',
        }
    }
}

/// Largest declared width this front end will materialize bit-by-bit.
///
/// A literal is stored as one entry per bit, so an unbounded width is an
/// unbounded allocation driven by source text.
pub const MAX_FOUR_STATE_WIDTH: u32 = 65_536;

/// Width given to an unsized four-state literal (`'bx`).
///
/// IEEE 1364-2005 section 3.5.1 gives an unsized literal at least 32 bits;
/// this front end does not yet propagate a context width, so 32 is used
/// verbatim and the absence of an explicit width is retained separately.
pub const UNSIZED_FOUR_STATE_WIDTH: u32 = 32;

/// A decoded four-state literal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FourStateLiteral {
    /// Source spelling, exactly as written.
    pub raw: SmolStr,
    /// Width written by the author. `None` for an unsized literal.
    pub declared_width: Option<u32>,
    pub base: LiteralBase,
    /// Whether the author wrote the `s`/`S` signed marker.
    pub signed: bool,
    /// Bit values, most significant first. Always
    /// `declared_width.unwrap_or(UNSIZED_FOUR_STATE_WIDTH)` entries long.
    pub bits: Vec<FourStateBit>,
}

impl FourStateLiteral {
    pub fn width(&self) -> u32 {
        self.bits.len() as u32
    }

    /// Whether any bit is `x` or `z`.
    ///
    /// A literal that decodes to only `0`/`1` bits never reaches here: the
    /// lexer routes it to the integer decoder instead. The predicate exists
    /// for consumers that receive an already-decoded literal.
    pub fn has_unknown_bits(&self) -> bool {
        self.bits
            .iter()
            .any(|bit| matches!(bit, FourStateBit::Unknown | FourStateBit::HighImpedance))
    }

    /// Canonical `width'base` prefix used by diagnostics.
    pub fn spelling(&self) -> String {
        self.raw.to_string()
    }
}

/// Whether the digits of an already-lexed based literal require four-state
/// decoding.
///
/// Answers only the question "does this text contain an `x`, `z`, or `?`
/// digit"; validity is decided by [`decode`].
pub fn digits_are_four_state(raw: &str) -> bool {
    let Some((_, suffix)) = raw.split_once('\'') else {
        return false;
    };
    // Skip the optional signed marker and the base character so a hex `d`
    // digit is never mistaken for the decimal base marker, and a base
    // character is never mistaken for a digit.
    let mut characters = suffix.trim_start().chars();
    let first = characters.next();
    if matches!(first, Some('s' | 'S')) && characters.next().is_none() {
        return false;
    }
    if first.is_none() {
        return false;
    }
    characters
        .as_str()
        .chars()
        .any(|c| matches!(c, 'x' | 'X' | 'z' | 'Z' | '?'))
}

/// Decode a based literal whose digits contain at least one `x`, `z`, or `?`.
///
/// Returns a human-readable reason on failure; callers wrap it in their own
/// diagnostic so the message keeps the literal's source span.
pub fn decode(raw: &str) -> Result<FourStateLiteral, String> {
    let trimmed = raw.trim();
    let (size, suffix) = trimmed
        .split_once('\'')
        .ok_or_else(|| format!("'{raw}' is not a based literal"))?;
    if suffix.contains('\'') {
        return Err(format!("'{raw}' contains more than one base marker"));
    }

    let declared_width = decode_width(raw, size)?;

    let mut characters = suffix.trim_start().chars();
    let first = characters
        .next()
        .ok_or_else(|| format!("'{raw}' is missing a base and digits"))?;
    let (signed, base_character) = if matches!(first, 's' | 'S') {
        (
            true,
            characters
                .next()
                .ok_or_else(|| format!("'{raw}' is missing a base after the signed marker"))?,
        )
    } else {
        (false, first)
    };
    let base = match base_character {
        'b' | 'B' => LiteralBase::Binary,
        'o' | 'O' => LiteralBase::Octal,
        'd' | 'D' => LiteralBase::Decimal,
        'h' | 'H' => LiteralBase::Hex,
        other => return Err(format!("'{raw}' has invalid base '{other}'")),
    };

    let digits: String = characters
        .as_str()
        .chars()
        .filter(|character| *character != '_' && !character.is_whitespace())
        .collect();
    if digits.is_empty() {
        return Err(format!("'{raw}' is missing digits after its base"));
    }

    let mut decoded = decode_digits(raw, base, &digits)?;
    let target = declared_width.unwrap_or(UNSIZED_FOUR_STATE_WIDTH);
    resize(&mut decoded, target);

    Ok(FourStateLiteral {
        raw: trimmed.into(),
        declared_width,
        base,
        signed,
        bits: decoded,
    })
}

fn decode_width(raw: &str, size: &str) -> Result<Option<u32>, String> {
    let size: String = size
        .chars()
        .filter(|character| *character != '_' && !character.is_whitespace())
        .collect();
    if size.is_empty() {
        return Ok(None);
    }
    let width = size
        .parse::<u32>()
        .map_err(|_| format!("'{raw}' has an invalid width"))?;
    if width == 0 {
        return Err(format!("'{raw}' has a zero width"));
    }
    if width > MAX_FOUR_STATE_WIDTH {
        return Err(format!(
            "'{raw}' has width {width}; this compiler materializes at most \
             {MAX_FOUR_STATE_WIDTH} bits per four-state literal"
        ));
    }
    Ok(Some(width))
}

fn decode_digits(raw: &str, base: LiteralBase, digits: &str) -> Result<Vec<FourStateBit>, String> {
    let Some(bits_per_digit) = base.bits_per_digit() else {
        // IEEE 1364-2005 section 3.5.1: a decimal literal that is not a plain
        // number must be exactly one `x` or `z` digit, because a decimal digit
        // string has no per-digit bit expansion to place the unknown into.
        let mut characters = digits.chars();
        let single = characters
            .next()
            .ok_or_else(|| format!("'{raw}' is missing digits after its base"))?;
        if characters.next().is_some() {
            return Err(format!(
                "'{raw}' mixes an x/z digit with decimal digits; a decimal \
                 four-state literal must be exactly one x or z digit"
            ));
        }
        let bit = four_state_digit(single)
            .ok_or_else(|| format!("'{raw}' contains a digit outside base 10"))?;
        return Ok(vec![bit]);
    };

    let width = (digits.len() as u64).saturating_mul(u64::from(bits_per_digit));
    if width > u64::from(MAX_FOUR_STATE_WIDTH) {
        return Err(format!(
            "'{raw}' expands to {width} bits; this compiler materializes at \
             most {MAX_FOUR_STATE_WIDTH} bits per four-state literal"
        ));
    }

    let mut bits = Vec::with_capacity(width as usize);
    for character in digits.chars() {
        match four_state_digit(character) {
            // An x/z digit fills every bit it stands for, which is what makes
            // `8'hxF` mean four unknown bits followed by `1111`.
            Some(fill @ (FourStateBit::Unknown | FourStateBit::HighImpedance)) => {
                bits.extend(std::iter::repeat_n(fill, bits_per_digit as usize));
            }
            Some(_) => unreachable!("only x/z/? decode to a fill digit"),
            None => {
                let value = character.to_digit(base.radix()).ok_or_else(|| {
                    format!("'{raw}' contains a digit outside base {}", base.radix())
                })?;
                for offset in (0..bits_per_digit).rev() {
                    bits.push(if value & (1 << offset) == 0 {
                        FourStateBit::Zero
                    } else {
                        FourStateBit::One
                    });
                }
            }
        }
    }
    Ok(bits)
}

const fn four_state_digit(character: char) -> Option<FourStateBit> {
    match character {
        'x' | 'X' => Some(FourStateBit::Unknown),
        'z' | 'Z' | '?' => Some(FourStateBit::HighImpedance),
        _ => None,
    }
}

/// Apply the IEEE 1364-2005 section 3.5.1 padding rule.
///
/// A value narrower than its declared width is extended on the left with `0`,
/// except that a leading `x` or `z` extends with itself — the rule that makes
/// `8'bx` eight unknown bits rather than seven zeros and one unknown. A value
/// wider than its declared width is truncated from the left.
fn resize(bits: &mut Vec<FourStateBit>, target: u32) {
    let target = target as usize;
    if bits.len() > target {
        bits.drain(..bits.len() - target);
        return;
    }
    if bits.len() == target {
        return;
    }
    let fill = match bits.first() {
        Some(bit @ (FourStateBit::Unknown | FourStateBit::HighImpedance)) => *bit,
        _ => FourStateBit::Zero,
    };
    let mut padded = vec![fill; target - bits.len()];
    padded.append(bits);
    *bits = padded;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spell(raw: &str) -> String {
        decode(raw)
            .unwrap_or_else(|error| panic!("{raw}: {error}"))
            .bits
            .iter()
            .map(|bit| bit.as_char())
            .collect()
    }

    #[test]
    fn detects_only_four_state_digit_strings() {
        for raw in ["4'b10x1", "8'hFz", "'bx", "4'sb1?01", "1'dz"] {
            assert!(digits_are_four_state(raw), "{raw}");
        }
        // A hex `d` digit and a signed hex base must not be read as digits.
        for raw in ["8'hdd", "8'shFF", "4'b1010", "16'd65535", "'h837FF"] {
            assert!(!digits_are_four_state(raw), "{raw}");
        }
    }

    #[test]
    fn expands_each_base_one_digit_at_a_time() {
        assert_eq!(spell("4'b10x1"), "10x1");
        assert_eq!(spell("6'o5x"), "101xxx");
        assert_eq!(spell("8'hxF"), "xxxx1111");
        assert_eq!(spell("8'b1z01_0011"), "1z010011");
    }

    /// IEEE 1364-2005 section 3.5.1: a leading `x` or `z` extends with itself.
    #[test]
    fn left_extension_repeats_a_leading_unknown() {
        assert_eq!(spell("8'bx"), "xxxxxxxx");
        assert_eq!(spell("8'bz1"), "zzzzzzz1");
        assert_eq!(spell("8'b1x"), "0000001x");
    }

    #[test]
    fn oversized_values_truncate_from_the_left() {
        assert_eq!(spell("4'b1010_x011"), "x011");
        assert_eq!(spell("2'hxF"), "11");
    }

    #[test]
    fn unsized_literals_take_the_default_width() {
        let decoded = decode("'bx").expect("unsized four-state literal");
        assert_eq!(decoded.declared_width, None);
        assert_eq!(decoded.width(), UNSIZED_FOUR_STATE_WIDTH);
        assert!(decoded.bits.iter().all(|bit| *bit == FourStateBit::Unknown));
    }

    #[test]
    fn signed_marker_is_retained_without_changing_bits() {
        let decoded = decode("4'sb10x1").expect("signed four-state literal");
        assert!(decoded.signed);
        assert_eq!(decoded.base, LiteralBase::Binary);
        assert_eq!(decoded.raw, "4'sb10x1");
    }

    /// IEEE 1364-2005 section 3.5.1: a decimal four-state literal is exactly
    /// one `x` or `z` digit.
    #[test]
    fn decimal_four_state_literals_are_a_single_digit() {
        let decoded = decode("4'dx").expect("single-digit decimal x");
        assert_eq!(decoded.bits, vec![FourStateBit::Unknown; 4]);
        let error = decode("4'd1x").expect_err("mixed decimal digits must fail");
        assert!(
            error.contains("must be exactly one x or z digit"),
            "{error}"
        );
    }

    #[test]
    fn rejects_unrepresentable_forms() {
        for raw in ["0'bx", "4'qx", "8'b", "4'bx2", "70000'bx"] {
            assert!(decode(raw).is_err(), "{raw}");
        }
        let error = decode("70000'bx").expect_err("over-wide literal must fail");
        assert!(error.contains("materializes at most"), "{error}");
    }

    #[test]
    fn question_mark_decodes_as_high_impedance() {
        assert_eq!(spell("4'b1?0?"), "1z0z");
    }
}

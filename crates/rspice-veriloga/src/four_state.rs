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

/// Width given to an unsized four-state literal (`'bx`) when nothing wider
/// asks for it.
///
/// IEEE 1364-2005 section 3.5.1 gives an unsized literal *at least* 32 bits,
/// and section 5.4.1 gives it the size of its context when that is larger. So
/// this is a floor rather than a width: decoding uses it because decoding has
/// no context to consult, and the discrete-domain lowering — which does — asks
/// for [`FourStateLiteral::bits_at`] instead. The absence of an explicit width
/// is retained separately, because only an unsized literal may grow this way.
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

    /// This literal's bits at `width`, padded by the section 3.5.1 rule.
    ///
    /// For an *unsized* literal only. Section 5.4.1 gives one the size of its
    /// context whenever that exceeds the 32-bit floor, and section 3.5.1 says
    /// what fills the positions it gains: a leading `x` or `z` extends with
    /// itself, anything else with zero. `'bx` in a 40-bit context is forty
    /// `x`s, not eight zeros above thirty-two of them.
    ///
    /// A *sized* literal is exactly as wide as its author wrote it and must
    /// not come through here. It reaches a wider context as an ordinary
    /// unsigned operand, which section 5.4.1 zero-extends — so `4'bxxxx` in an
    /// eight-bit context is `8'b0000xxxx`, and the two rules disagree by
    /// design.
    pub fn bits_at(&self, width: u32) -> Vec<FourStateBit> {
        let mut bits = self.bits.clone();
        resize(&mut bits, width);
        bits
    }
}

/// Split a based literal's suffix into its signed marker, its base character,
/// and its digits.
///
/// IEEE 1364-2005 section 3.5.1 orders them exactly so: an optional `s`/`S`,
/// then the base, then the digits. Splitting in one place is what keeps a hex
/// `d` digit from being read as the decimal base marker and a base character
/// from being read as a digit — three separate scans of the same grammar would
/// be three chances to disagree about `8'shFF`.
///
/// `None` when the suffix has no base character at all.
fn split_base(suffix: &str) -> Option<(bool, char, &str)> {
    let mut characters = suffix.trim_start().chars();
    let first = characters.next()?;
    let (signed, base) = if matches!(first, 's' | 'S') {
        (true, characters.next()?)
    } else {
        (false, first)
    };
    Some((signed, base, characters.as_str()))
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
    let Some((_, _, digits)) = split_base(suffix) else {
        return false;
    };
    digits
        .chars()
        .any(|c| matches!(c, 'x' | 'X' | 'z' | 'Z' | '?'))
}

/// Whether a literal's source spelling carries the section 3.5.1 `s` marker.
///
/// Read from the raw text rather than from a [`FourStateLiteral`], because
/// [`decode`] answers only for the literals it can decode: a based *decimal*
/// literal whose digits are ordinary numerals — `4'sd9` — has no per-digit bit
/// expansion and is refused there, so asking the decoded form whether it was
/// marked would answer "no" for exactly the spellings that carry the marker
/// most often.
///
/// A literal with no base marker at all is unsigned by this test, which is not
/// the same question as section 5.4.2's: a *plain decimal* number is signed
/// without any marker. This answers only what the author wrote.
pub fn has_signed_marker(raw: &str) -> bool {
    raw.trim()
        .split_once('\'')
        .and_then(|(_, suffix)| split_base(suffix))
        .is_some_and(|(signed, _, _)| signed)
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

    let (signed, base_character, rest) =
        split_base(suffix).ok_or_else(|| format!("'{raw}' is missing a base and digits"))?;
    let base = match base_character {
        'b' | 'B' => LiteralBase::Binary,
        'o' | 'O' => LiteralBase::Octal,
        'd' | 'D' => LiteralBase::Decimal,
        'h' | 'H' => LiteralBase::Hex,
        other => return Err(format!("'{raw}' has invalid base '{other}'")),
    };

    let digits: String = rest
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
        return decode_decimal(raw, digits);
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

/// Largest decimal literal this decoder materializes.
///
/// A decimal digit string is a number rather than a per-digit expansion, so
/// decoding one means arithmetic, and arithmetic here is `u128`. Refusing past
/// that is a stated boundary; guessing past it would be a wrong constant.
const MAX_DECIMAL_LITERAL_BITS: u32 = 128;

/// Decode a base-10 digit string, IEEE 1364-2005 section 3.5.1.
///
/// Two forms, and they cannot be mixed. Ordinarily the digits are a plain
/// number, which is expanded to bits here and then padded or truncated to the
/// declared width by [`resize`] like any other literal. The exception is a
/// single `x` or `z` standing for the whole value: a decimal digit string has
/// no per-digit bit expansion to place an unknown into, so the standard permits
/// the unknown only as the entire digit string.
///
/// The plain-number form matters beyond `x`/`z` handling. It is what gives a
/// `4'd9` a *declared width of four*: without it the literal fails to decode,
/// and a caller that recovers a literal's size from its spelling has to fall
/// back on the 32-bit unsized floor for every based decimal in a design.
fn decode_decimal(raw: &str, digits: &str) -> Result<Vec<FourStateBit>, String> {
    if let Some(unknown) = digits.chars().find_map(four_state_digit) {
        if digits.chars().count() > 1 {
            return Err(format!(
                "'{raw}' mixes an x/z digit with decimal digits; a decimal \
                 four-state literal must be exactly one x or z digit"
            ));
        }
        return Ok(vec![unknown]);
    }
    if !digits.chars().all(|character| character.is_ascii_digit()) {
        return Err(format!("'{raw}' contains a digit outside base 10"));
    }
    let value = digits.parse::<u128>().map_err(|_| {
        format!(
            "'{raw}' is a decimal literal beyond {MAX_DECIMAL_LITERAL_BITS} bits; \
             this compiler materializes no wider decimal value"
        )
    })?;
    // Most significant bit first, and never empty: a zero is one `0` bit, which
    // `resize` then pads to the declared width.
    let significant = MAX_DECIMAL_LITERAL_BITS - value.leading_zeros();
    Ok((0..significant.max(1))
        .rev()
        .map(|index| {
            if value >> index & 1 == 1 {
                FourStateBit::One
            } else {
                FourStateBit::Zero
            }
        })
        .collect())
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

    /// IEEE 1364-2005 section 3.5.1: a based decimal literal is an ordinary
    /// number, expanded to its declared width. This is what gives `4'd9` a
    /// width of four rather than the 32-bit unsized floor a failed decode
    /// leaves behind.
    #[test]
    fn based_decimal_literals_decode_to_their_declared_width() {
        assert_eq!(spell("4'd9"), "1001");
        assert_eq!(spell("4'd0"), "0000");
        assert_eq!(spell("8'd255"), "11111111");
        assert_eq!(spell("4'sd9"), "1001");
        assert_eq!(decode("4'd9").expect("decodes").declared_width, Some(4));
        assert!(decode("4'sd9").expect("decodes").signed);
        assert!(!decode("4'd9").expect("decodes").signed);
        // An unsized decimal takes the 32-bit floor.
        let floored = decode("'d5").expect("decodes");
        assert_eq!(floored.declared_width, None);
        assert_eq!(floored.width(), UNSIZED_FOUR_STATE_WIDTH);
        // Section 3.5.1 truncates from the left, so twenty in four bits is four.
        assert_eq!(spell("4'd20"), "0100");
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

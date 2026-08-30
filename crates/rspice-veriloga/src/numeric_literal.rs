//! Exact parsing for Verilog-AMS numeric literals.
//!
//! The executable IR currently stores scalar constants as `f64`. Integer
//! syntax is nevertheless parsed as an integer first so type-sensitive
//! compile-time evaluation never depends on a rounded floating-point parse.

/// Exact value represented by a numeric literal before it enters the scalar
/// `f64` execution IR.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum NumericLiteralValue {
    Integer(i64),
    Real(f64),
}

const MAX_BASED_INTEGER_DIGITS: usize = 16_384;

impl NumericLiteralValue {
    /// Convert to the current scalar IR representation without losing an
    /// integer bit. A wider integer IR is required before this gate can be
    /// relaxed.
    pub(crate) fn as_exact_f64(self, raw: &str) -> Result<f64, String> {
        match self {
            Self::Integer(value) => exact_integer_as_f64(value).ok_or_else(|| {
                format!(
                    "integer literal '{raw}' cannot be represented exactly by the current scalar numeric IR"
                )
            }),
            Self::Real(value) => Ok(value),
        }
    }
}

/// Parse a lexer-validated numeric literal while retaining integer identity.
pub(crate) fn parse_numeric_literal(raw: &str) -> Result<NumericLiteralValue, String> {
    let raw = raw.trim();
    if raw.is_empty() {
        return Err("empty numeric literal".to_string());
    }

    if raw.contains('\'') {
        return parse_based_integer(raw).map(NumericLiteralValue::Integer);
    }

    let compact = remove_underscores(raw);
    let is_real = compact.contains('.')
        || compact.contains('e')
        || compact.contains('E')
        || scale_suffix(&compact).is_some();
    if !is_real {
        return compact
            .parse::<i64>()
            .map(NumericLiteralValue::Integer)
            .map_err(|_| format!("'{raw}' is outside the supported signed 64-bit integer range"));
    }

    parse_real_literal(&compact).map(NumericLiteralValue::Real)
}

/// Return an exact integer for integer syntax, or `None` for real syntax.
pub(crate) fn parse_integer_literal(raw: &str) -> Result<Option<i64>, String> {
    match parse_numeric_literal(raw)? {
        NumericLiteralValue::Integer(value) => Ok(Some(value)),
        NumericLiteralValue::Real(_) => Ok(None),
    }
}

/// Convert only integers exactly representable by the current `f64` IR.
pub(crate) fn exact_integer_as_f64(value: i64) -> Option<f64> {
    let converted = value as f64;
    if converted.is_finite()
        && converted >= i64::MIN as f64
        && converted < 9_223_372_036_854_775_808.0
        && converted as i64 == value
    {
        Some(converted)
    } else {
        None
    }
}

fn parse_real_literal(compact: &str) -> Result<f64, String> {
    let (number, scale) = match scale_suffix(compact) {
        Some((suffix_start, scale)) => (&compact[..suffix_start], scale),
        None => (compact, 1.0),
    };
    let value = number
        .parse::<f64>()
        .map_err(|_| format!("'{compact}' is not a valid real number"))?
        * scale;
    if !value.is_finite() {
        return Err(format!("'{compact}' is outside the finite real range"));
    }
    Ok(value)
}

fn scale_suffix(compact: &str) -> Option<(usize, f64)> {
    let (index, _) = compact.char_indices().find(|(_, character)| {
        matches!(
            character,
            'T' | 'G' | 'M' | 'k' | 'K' | 'm' | 'u' | 'n' | 'p' | 'f' | 'a'
        )
    })?;
    let suffix = &compact[index..];
    let scale = if suffix.eq_ignore_ascii_case("meg") {
        1.0e6
    } else {
        match suffix {
            "T" => 1.0e12,
            "G" => 1.0e9,
            "M" => 1.0e6,
            "k" | "K" => 1.0e3,
            "m" => 1.0e-3,
            "u" => 1.0e-6,
            "n" => 1.0e-9,
            "p" => 1.0e-12,
            "f" => 1.0e-15,
            "a" => 1.0e-18,
            _ => return None,
        }
    };
    Some((index, scale))
}

fn parse_based_integer(raw: &str) -> Result<i64, String> {
    let (size, suffix) = raw
        .split_once('\'')
        .ok_or_else(|| format!("'{raw}' is not a based integer literal"))?;
    if suffix.contains('\'') {
        return Err(format!("'{raw}' contains more than one base marker"));
    }

    let size = size.trim();
    let explicit_width = if size.is_empty() {
        None
    } else {
        let compact_size = remove_underscores(size);
        if compact_size.is_empty() {
            return Err(format!("'{raw}' has an empty width"));
        }
        let width = compact_size
            .parse::<u32>()
            .map_err(|_| format!("'{raw}' has an invalid width"))?;
        if width == 0 {
            return Err(format!("'{raw}' has a zero width"));
        }
        if width > 64 {
            return Err(format!(
                "'{raw}' has width {width}; integer widths above 64 bits are not representable by the current integer IR"
            ));
        }
        Some(width)
    };

    let mut chars = suffix.chars();
    let first = chars
        .next()
        .ok_or_else(|| format!("'{raw}' is missing a base and digits"))?;
    let (signed, base_character) = if matches!(first, 's' | 'S') {
        (
            true,
            chars
                .next()
                .ok_or_else(|| format!("'{raw}' is missing a base after the signed marker"))?,
        )
    } else {
        (false, first)
    };
    let radix = match base_character {
        'b' | 'B' => 2,
        'o' | 'O' => 8,
        'd' | 'D' => 10,
        'h' | 'H' => 16,
        _ => return Err(format!("'{raw}' has invalid base '{base_character}'")),
    };
    let digits = chars.as_str().trim();
    if digits.is_empty() {
        return Err(format!("'{raw}' is missing digits after its base"));
    }
    if digits
        .chars()
        .any(|character| matches!(character, 'x' | 'X' | 'z' | 'Z' | '?'))
    {
        return Err(format!(
            "'{raw}' contains an x/z/? digit, whose four-state semantics are not representable by the current integer IR"
        ));
    }
    let compact_digits = remove_underscores(digits);
    if compact_digits.is_empty() {
        return Err(format!("'{raw}' is missing digits after its base"));
    }
    if compact_digits.len() > MAX_BASED_INTEGER_DIGITS {
        return Err(format!(
            "'{raw}' contains more than the supported safety limit of {MAX_BASED_INTEGER_DIGITS} based digits"
        ));
    }

    let width_mask = explicit_width.map(|width| {
        if width == 64 {
            u64::MAX
        } else {
            (1_u64 << width) - 1
        }
    });
    let mut bits = 0_u64;
    for character in compact_digits.chars() {
        let digit = character
            .to_digit(radix)
            .ok_or_else(|| format!("'{raw}' contains a digit outside base {radix}"))?;
        bits = if let Some(mask) = width_mask {
            bits.wrapping_mul(u64::from(radix))
                .wrapping_add(u64::from(digit))
                & mask
        } else {
            bits.checked_mul(u64::from(radix))
                .and_then(|value| value.checked_add(u64::from(digit)))
                .ok_or_else(|| {
                    format!(
                        "'{raw}' requires more than 64 bits; unsized integers wider than the current integer IR are not supported"
                    )
                })?
        };
    }

    let width = match explicit_width {
        Some(width) => width,
        None => {
            let required = (64 - bits.leading_zeros()).max(1);
            let width = required.max(32);
            if width > 64 {
                return Err(format!(
                    "'{raw}' requires {width} bits; unsized integers wider than the current integer IR are not supported"
                ));
            }
            width
        }
    };
    let truncated = bits;

    if signed && truncated & (1_u64 << (width - 1)) != 0 {
        let extended = if width == 64 {
            truncated
        } else {
            truncated | (!0_u64 << width)
        };
        Ok(extended as i64)
    } else {
        i64::try_from(truncated).map_err(|_| {
            format!("'{raw}' is an unsigned value above the supported signed 64-bit integer range")
        })
    }
}

fn remove_underscores(value: &str) -> String {
    value
        .chars()
        .filter(|character| *character != '_')
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_based_integer_width_and_signedness_exactly() {
        for (raw, expected) in [
            ("8'b1111_0000", 240),
            ("12'o7_123", 3667),
            ("16'd65_535", 65_535),
            ("16'hCA_FE", 51_966),
            ("8'shFF", -1),
            ("4'sb1000", -8),
            ("4'h1F", 15),
            ("8'h1FF", 255),
            ("8'd511", 255),
            ("8'sh1FF", -1),
            ("8'h_FF__", 255),
            ("8'h1234_5678_9ABC_DEFF", 255),
            ("8'b1111_1111_0000_0000_1111_1111", 255),
            ("'shFF", 255),
            ("'shFFFF_FFFF", -1),
            ("32 'h 12ab_f001", 313_257_985),
            ("'h 837FF", 538_623),
        ] {
            assert_eq!(parse_integer_literal(raw).unwrap(), Some(expected), "{raw}");
        }
    }

    #[test]
    fn rejects_unrepresentable_integer_forms() {
        for raw in [
            "0'h0",
            "65'h1",
            "8'hx1",
            "8'b10?1",
            "8'oz",
            "8'h",
            "8's",
            "8'q1",
            "64'hFFFF_FFFF_FFFF_FFFF",
        ] {
            assert!(parse_numeric_literal(raw).is_err(), "{raw}");
        }
        let error = parse_numeric_literal("'h1_0000_0000_0000_0000")
            .expect_err("over-wide unsized integer must fail");
        assert!(error.contains("requires more than 64 bits"), "{error}");
    }

    #[test]
    fn exact_f64_gate_never_rounds_integers() {
        assert_eq!(
            exact_integer_as_f64(1_i64 << 60),
            Some((1_i64 << 60) as f64)
        );
        assert_eq!(exact_integer_as_f64((1_i64 << 53) + 1), None);
        assert!(
            NumericLiteralValue::Integer((1_i64 << 53) + 1)
                .as_exact_f64("9007199254740993")
                .is_err()
        );
    }
}

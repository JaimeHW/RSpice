//! `$test$plusargs` and `$value$plusargs` (IEEE 1364-2005 section 17.10).
//!
//! The plusargs are the run's [`crate::config::HdlRunConfig::plusargs`],
//! with or without their leading `+`. Matching is by prefix and the first
//! match wins, so the command line, which comes first, overrides the deck.

use rspice_veriloga::canonical_ir::digital::DigitalValueClass;
use rspice_veriloga::canonical_ir::digital_eval::DigitalScalar;
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::four_state::FourStateBit;

fn stripped(argument: &str) -> &str {
    argument.strip_prefix('+').unwrap_or(argument)
}

/// `$test$plusargs(name)`: whether any plusarg begins with `name`.
pub(crate) fn test(plusargs: &[String], name: &str) -> bool {
    plusargs
        .iter()
        .any(|argument| stripped(argument).starts_with(name))
}

/// `$value$plusargs(format, variable)`: the value of the first plusarg that
/// begins with the text before the format's conversion, converted by it and
/// sized to `class`. `None` when no plusarg matches; `Some(None)` when one
/// matches but its text does not convert (the variable is set to `x`).
pub(crate) fn value(
    plusargs: &[String],
    format: &str,
    class: DigitalValueClass,
) -> Result<Option<DigitalScalar>, String> {
    let Some(percent) = format.find('%') else {
        return Err(format!(
            "`$value$plusargs` format `{format}` has no conversion specification"
        ));
    };
    let prefix = &format[..percent];
    let mut specification = format[percent + 1..].chars();
    let conversion = loop {
        match specification.next() {
            Some(character) if character.is_ascii_digit() => continue,
            Some(character) => break character.to_ascii_lowercase(),
            None => {
                return Err(format!(
                    "`$value$plusargs` format `{format}` ends inside its conversion"
                ));
            }
        }
    };
    let Some(text) = plusargs
        .iter()
        .map(|argument| stripped(argument))
        .find_map(|argument| argument.strip_prefix(prefix))
    else {
        return Ok(None);
    };
    let radix = match conversion {
        'd' => Some(10),
        'o' => Some(8),
        'h' | 'x' => Some(16),
        'b' => Some(2),
        _ => None,
    };
    let scalar = if let Some(radix) = radix {
        integer(text, radix, class)
    } else {
        match conversion {
            'e' | 'f' | 'g' => match real_prefix(text.trim()) {
                Some(value) => real(value, class),
                None => unknown(class),
            },
            's' => string(text, class),
            other => {
                return Err(format!(
                    "`$value$plusargs` does not convert `%{other}`; IEEE 1364-2005 section \
                     17.10.2 defines %d %o %h %b %e %f %g %s"
                ));
            }
        }
    };
    Ok(Some(scalar))
}

/// The real the longest leading part of `text` spells, as Icarus reads a
/// real plusarg: `9.825units` is 9.825 (Icarus warns about the rest),
/// `not_a_num` is no real at all.
fn real_prefix(text: &str) -> Option<f64> {
    (1..=text.len())
        .rev()
        .filter(|end| text.is_char_boundary(*end))
        .find_map(|end| text[..end].parse::<f64>().ok())
}

fn unknown(class: DigitalValueClass) -> DigitalScalar {
    match class {
        DigitalValueClass::FourState { width, .. } => {
            DigitalScalar::FourState(FourStateValue::splat(width, FourStateBit::Unknown))
        }
        DigitalValueClass::Real => DigitalScalar::Real(0.0),
    }
}

fn integer(text: &str, radix: u32, class: DigitalValueClass) -> DigitalScalar {
    let text = text.trim();
    let (negative, digits) = match text.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, text.strip_prefix('+').unwrap_or(text)),
    };
    let digits: String = digits
        .chars()
        .filter(|character| *character != '_')
        .collect();
    if let DigitalValueClass::FourState { width, .. } = class
        && digits
            .chars()
            .any(|character| matches!(character, 'x' | 'X' | 'z' | 'Z'))
    {
        return match (negative, four_state_digits(&digits, radix, width)) {
            (false, Some(value)) => DigitalScalar::FourState(value),
            _ => unknown(class),
        };
    }
    let Ok(magnitude) = num_bigint::BigUint::parse_bytes(digits.as_bytes(), radix)
        .ok_or(())
        .and_then(|value| {
            if digits.is_empty() {
                Err(())
            } else {
                Ok(value)
            }
        })
    else {
        return unknown(class);
    };
    match class {
        DigitalValueClass::Real => {
            let value = magnitude
                .to_string()
                .parse::<f64>()
                .unwrap_or(f64::INFINITY);
            DigitalScalar::Real(if negative { -value } else { value })
        }
        DigitalValueClass::FourState { width, .. } => {
            let modulus = num_bigint::BigUint::from(1u8) << width as usize;
            let mut value = magnitude % &modulus;
            if negative && value != num_bigint::BigUint::from(0u8) {
                value = &modulus - value;
            }
            DigitalScalar::FourState(from_magnitude(width, &value))
        }
    }
}

/// Digits with an `x` or `z` among them, as Icarus reads them: in binary,
/// octal or hex each such digit sets every bit of its digit (`7x` under `%o`
/// is `111xxx`), and in decimal only a lone `x` or `z` is a value, setting
/// every bit. `None` for anything else, which reads as `x`.
fn four_state_digits(digits: &str, radix: u32, width: u32) -> Option<FourStateValue> {
    let digit_bits = match radix {
        2 => 1,
        8 => 3,
        16 => 4,
        _ => {
            return match digits {
                "x" | "X" => Some(FourStateValue::splat(width, FourStateBit::Unknown)),
                "z" | "Z" => Some(FourStateValue::splat(width, FourStateBit::HighImpedance)),
                _ => None,
            };
        }
    };
    let mut out = FourStateValue::zero(width);
    for (position, character) in digits.chars().rev().enumerate() {
        let digit = match character {
            'x' | 'X' => None,
            'z' | 'Z' => Some(None),
            other => Some(Some(other.to_digit(radix)?)),
        };
        for bit in 0..digit_bits {
            let Some(index) = u32::try_from(position)
                .ok()
                .and_then(|position| position.checked_mul(digit_bits))
                .and_then(|lowest| lowest.checked_add(bit))
                .filter(|index| *index < width)
            else {
                break;
            };
            out.set_bit(
                index,
                match digit {
                    None => FourStateBit::Unknown,
                    Some(None) => FourStateBit::HighImpedance,
                    Some(Some(value)) if value >> bit & 1 == 1 => FourStateBit::One,
                    Some(Some(_)) => FourStateBit::Zero,
                },
            );
        }
    }
    Some(out)
}

fn from_magnitude(width: u32, value: &num_bigint::BigUint) -> FourStateValue {
    let mut out = FourStateValue::zero(width);
    for index in 0..width {
        if value.bit(u64::from(index)) {
            out.set_bit(index, FourStateBit::One);
        }
    }
    out
}

fn real(value: f64, class: DigitalValueClass) -> DigitalScalar {
    match class {
        DigitalValueClass::Real => DigitalScalar::Real(value),
        DigitalValueClass::FourState { width, .. } => FourStateValue::from_real(width, value)
            .map_or_else(|_| unknown(class), DigitalScalar::FourState),
    }
}

/// A string packed eight bits per character, right-aligned, keeping the
/// rightmost characters when the variable is too narrow (IEEE 1364-2005
/// section 3.6.1's rule for a string assigned to a vector).
fn string(text: &str, class: DigitalValueClass) -> DigitalScalar {
    let DigitalValueClass::FourState { width, .. } = class else {
        return DigitalScalar::Real(0.0);
    };
    let mut out = FourStateValue::zero(width);
    for (position, byte) in text.bytes().rev().enumerate() {
        for bit in 0..8 {
            let index = position as u32 * 8 + bit;
            if index >= width {
                return DigitalScalar::FourState(out);
            }
            if byte >> bit & 1 == 1 {
                out.set_bit(index, FourStateBit::One);
            }
        }
    }
    DigitalScalar::FourState(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> Vec<String> {
        list.iter()
            .map(|argument| (*argument).to_string())
            .collect()
    }

    const INT: DigitalValueClass = DigitalValueClass::FourState {
        width: 32,
        signed: true,
    };

    #[test]
    fn the_first_prefix_match_wins() {
        let plusargs = args(&["+VERBOSE", "+N=12", "N=99", "+HEX=ff"]);
        assert!(test(&plusargs, "VERB"));
        assert!(!test(&plusargs, "QUIET"));
        assert_eq!(
            value(&plusargs, "N=%d", INT).unwrap(),
            Some(DigitalScalar::FourState(FourStateValue::from_u64(32, 12)))
        );
        assert_eq!(
            value(&plusargs, "HEX=%h", INT).unwrap(),
            Some(DigitalScalar::FourState(FourStateValue::from_u64(32, 255)))
        );
        assert_eq!(value(&plusargs, "MISSING=%d", INT).unwrap(), None);
        assert_eq!(
            value(&args(&["+R=2.5"]), "R=%f", DigitalValueClass::Real).unwrap(),
            Some(DigitalScalar::Real(2.5))
        );
        assert_eq!(
            value(&args(&["+N=-3"]), "N=%d", INT).unwrap(),
            Some(DigitalScalar::FourState(FourStateValue::from_integer(
                32, -3
            )))
        );
    }

    #[test]
    fn unknown_digits_and_trailing_text_read_as_icarus_reads_them() {
        // Icarus 13 (`vvp +a=1x +b=-1x +d=10x +e=2.5e +f=Z +g=fffff +h=.5x
        // +i=7x +hex=12_x_z`) into a 12-bit variable and a real.
        let class = DigitalValueClass::FourState {
            width: 12,
            signed: false,
        };
        let plusargs = args(&[
            "+a=1x",
            "+b=-1x",
            "+d=10x",
            "+e=2.5e",
            "+f=Z",
            "+g=fffff",
            "+h=.5x",
            "+i=7x",
            "+hex=12_x_z",
            "+bad=not_a_num",
        ]);
        let bits = |format: &str| {
            let Some(DigitalScalar::FourState(value)) = value(&plusargs, format, class).unwrap()
            else {
                panic!("{format} matches a four-state value");
            };
            value.spelling()
        };
        assert_eq!(bits("a=%d"), "xxxxxxxxxxxx");
        assert_eq!(bits("b=%h"), "xxxxxxxxxxxx");
        assert_eq!(bits("d=%b"), "00000000010x");
        assert_eq!(bits("f=%d"), "zzzzzzzzzzzz");
        assert_eq!(bits("g=%h"), "111111111111");
        assert_eq!(bits("i=%o"), "000000111xxx");
        assert_eq!(bits("hex=%h"), "0010xxxxzzzz");
        assert_eq!(bits("bad=%b"), "xxxxxxxxxxxx");
        let real = |format: &str| value(&plusargs, format, DigitalValueClass::Real).unwrap();
        assert_eq!(real("e=%f"), Some(DigitalScalar::Real(2.5)));
        assert_eq!(real("h=%f"), Some(DigitalScalar::Real(0.5)));
        assert_eq!(real("bad=%f"), Some(DigitalScalar::Real(0.0)));
    }

    #[test]
    fn a_string_packs_right_aligned() {
        let class = DigitalValueClass::FourState {
            width: 24,
            signed: false,
        };
        let Some(DigitalScalar::FourState(packed)) =
            value(&args(&["+NAME=ab"]), "NAME=%s", class).unwrap()
        else {
            panic!("string conversion");
        };
        assert_eq!(packed.to_u64(), Some(0x6162));
    }
}

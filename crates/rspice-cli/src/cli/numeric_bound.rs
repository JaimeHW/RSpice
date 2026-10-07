//! Preserve authored decimal boundaries before choosing an output coordinate type.
use super::args::spice_value;
use rspice_core::netlist::lexer::spice_suffix_scale;
use std::cmp::Ordering;
use std::str::FromStr;

const EXPONENT_RANGE: &str = "range boundary exponent exceeds the supported decimal range";

/// A validated SPICE value, with its exact decimal value and table projection.
#[derive(Clone, Debug)]
pub(crate) struct NumericBound {
    value: f64,
    negative: bool,
    // Normalized decimal digits, most significant first, with no trailing zero.
    digits: Vec<u8>,
    exponent: i64,
}

impl FromStr for NumericBound {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let value = spice_value(text)?;
        let (negative, mut digits, mut exponent, suffix) = decimal_prefix(text.trim())?;
        // The core owns the accepted suffix vocabulary. Its finite scale
        // constants have exact decimal spellings (including MIL's 2.54e-5).
        let scale = format!("{:e}", spice_suffix_scale(suffix).0);
        let (_, scale_digits, scale_exponent, _) = decimal_prefix(&scale)?;
        let coefficient = scale_digits
            .iter()
            .fold(0_u32, |n, digit| n * 10 + u32::from(*digit));
        exponent = exponent.checked_add(scale_exponent).ok_or(EXPONENT_RANGE)?;
        let leading = digits.iter().take_while(|digit| **digit == 0).count();
        digits.drain(..leading);
        if digits.is_empty() {
            return Ok(Self {
                value,
                negative: false,
                digits,
                exponent: 0,
            });
        }
        let mut carry = 0;
        for digit in digits.iter_mut().rev() {
            let product = u32::from(*digit) * coefficient + carry;
            *digit = (product % 10) as u8;
            carry = product / 10;
        }
        while carry != 0 {
            digits.insert(0, (carry % 10) as u8);
            carry /= 10;
        }
        while digits.last() == Some(&0) {
            digits.pop();
            exponent = exponent.checked_add(1).ok_or(EXPONENT_RANGE)?;
        }
        Ok(Self {
            value,
            negative,
            digits,
            exponent,
        })
    }
}

/// The whole token was already admitted by the core lexer. Split only the
/// decimal prefix; suffix recognition remains the core's responsibility.
fn decimal_prefix(text: &str) -> Result<(bool, Vec<u8>, i64, &str), String> {
    let negative = text.starts_with('-');
    let text = text.strip_prefix(['+', '-']).unwrap_or(text);
    let end = text
        .bytes()
        .take_while(|byte| byte.is_ascii_digit() || *byte == b'.')
        .count();
    let mantissa = &text[..end];
    let fractional = mantissa.find('.').map_or(0, |dot| mantissa.len() - dot - 1);
    let digits = mantissa
        .bytes()
        .filter(u8::is_ascii_digit)
        .map(|byte| byte - b'0')
        .collect();
    let mut suffix = &text[end..];
    let mut exponent = 0_i64;
    if suffix.starts_with(['e', 'E']) {
        suffix = &suffix[1..];
        let sign = usize::from(suffix.starts_with(['+', '-']));
        let count = sign
            + suffix[sign..]
                .bytes()
                .take_while(u8::is_ascii_digit)
                .count();
        let literal = &suffix[..count];
        exponent = literal.parse().map_err(|_| EXPONENT_RANGE)?;
        suffix = &suffix[count..];
    }
    Ok((
        negative,
        digits,
        exponent
            .checked_sub(fractional as i64)
            .ok_or(EXPONENT_RANGE)?,
        suffix,
    ))
}

impl NumericBound {
    pub(crate) fn value(&self) -> f64 {
        self.value
    }

    pub(crate) fn is_negative(&self) -> bool {
        self.negative
    }

    pub(crate) fn cmp(&self, other: &Self) -> Ordering {
        let sign = |value: &Self| {
            if value.digits.is_empty() {
                0
            } else if value.negative {
                -1
            } else {
                1
            }
        };
        let order = sign(self).cmp(&sign(other));
        if order != Ordering::Equal || self.digits.is_empty() {
            return order;
        }
        let magnitude = |value: &Self| value.exponent.saturating_add(value.digits.len() as i64);
        let order = magnitude(self).cmp(&magnitude(other)).then_with(|| {
            self.digits
                .iter()
                .copied()
                .chain(std::iter::repeat(0))
                .zip(other.digits.iter().copied().chain(std::iter::repeat(0)))
                .take(self.digits.len().max(other.digits.len()))
                .find_map(|(left, right)| (left != right).then(|| left.cmp(&right)))
                .unwrap_or(Ordering::Equal)
        });
        if self.negative {
            order.reverse()
        } else {
            order
        }
    }

    /// Floor the value on a 10^power grid and report whether it lands exactly.
    /// `None` means negative or beyond u64. Only the integer digits are read,
    /// so long fractional tails and extreme exponents never require big integers.
    pub(crate) fn grid_position(&self, power: i64) -> Option<(u64, bool)> {
        if self.negative {
            return None;
        }
        if self.digits.is_empty() {
            return Some((0, true));
        }
        let count = self
            .exponent
            .saturating_sub(power)
            .saturating_add(self.digits.len() as i64);
        if count <= 0 {
            return Some((0, false));
        }
        if count > 20 {
            return None;
        }
        let mut tick = 0_u64;
        for digit in self
            .digits
            .iter()
            .copied()
            .chain(std::iter::repeat(0))
            .take(count as usize)
        {
            tick = tick.checked_mul(10)?.checked_add(u64::from(digit))?;
        }
        Some((tick, count as usize >= self.digits.len()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suffixes_and_exponents_share_the_core_numeric_vocabulary() {
        for (input, power, expected) in [
            ("5.25n", -11, 525),
            ("1e-3k", 0, 1),
            ("1MIL", -7, 254),
            ("20.000000000000014s", -15, 20_000_000_000_000_014),
            ("18.446744073709551615e18", 0, u64::MAX),
        ] {
            let value: NumericBound = input.parse().unwrap();
            assert_eq!(
                value.grid_position(power),
                Some((expected, true)),
                "{input}"
            );
        }
    }

    #[test]
    fn fractional_tails_underflow_and_signed_order_are_exact() {
        let tail: NumericBound = "1.9999999999999999999999999999999999999999"
            .parse()
            .unwrap();
        assert_eq!(tail.grid_position(0), Some((1, false)));
        assert_eq!(tail.cmp(&"2".parse().unwrap()), Ordering::Less);
        let tiny: NumericBound = "1e-999".parse().unwrap();
        assert_eq!(tiny.grid_position(-15), Some((0, false)));
        assert_eq!(tiny.cmp(&"0".parse().unwrap()), Ordering::Greater);
        assert!(
            "1e-99999999999999999999999999999999"
                .parse::<NumericBound>()
                .is_err()
        );
        assert!("1e-9223372036854775808f".parse::<NumericBound>().is_err());
        assert_eq!(
            "-1e-999"
                .parse::<NumericBound>()
                .unwrap()
                .cmp(&"0".parse().unwrap()),
            Ordering::Less
        );
        assert_eq!(
            "-2".parse::<NumericBound>()
                .unwrap()
                .cmp(&"-1".parse().unwrap()),
            Ordering::Less
        );
        assert_eq!(
            "-0".parse::<NumericBound>()
                .unwrap()
                .cmp(&"0".parse().unwrap()),
            Ordering::Equal
        );
        assert_eq!(
            "1MIL"
                .parse::<NumericBound>()
                .unwrap()
                .cmp(&"25.4u".parse().unwrap()),
            Ordering::Equal
        );
    }
}

//! Shared runtime contract for the Verilog-AMS `integer` type.
//!
//! Verilog-AMS 2023 sections 3.2 and 4.2.1.1 define `integer` as a signed
//! 32-bit, two's-complement value and require real-to-integer conversion to
//! round to nearest with exact half cases away from zero. Section 4.2.11
//! defines both `<<` and `>>` as logical (zero-filling) shifts. Keeping those
//! rules here prevents the portable VM and the native/WASM backends from
//! inheriting their host language's cast and shift behavior.

use std::fmt;

pub(crate) const INTEGER_BITS: u32 = i32::BITS;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum IntegerBinaryOperation {
    Shl,
    Shr,
    BitAnd,
    BitOr,
    BitXor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum IntegerRuntimeError {
    NonFiniteOperand { value: f64 },
    OperandOutOfRange { value: f64 },
}

impl fmt::Display for IntegerRuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NonFiniteOperand { value } => {
                write!(f, "integer conversion requires a finite value, got {value}")
            }
            Self::OperandOutOfRange { value } => write!(
                f,
                "integer conversion of {value} rounds outside the signed 32-bit range"
            ),
        }
    }
}

/// Apply the Verilog-AMS real-to-integer conversion.
pub(crate) fn real_to_integer(value: f64) -> Result<i32, IntegerRuntimeError> {
    if !value.is_finite() {
        return Err(IntegerRuntimeError::NonFiniteOperand { value });
    }
    let rounded = value.round();
    if rounded < f64::from(i32::MIN) || rounded > f64::from(i32::MAX) {
        return Err(IntegerRuntimeError::OperandOutOfRange { value });
    }
    Ok(rounded as i32)
}

pub(crate) fn integer_binary(
    operation: IntegerBinaryOperation,
    left: f64,
    right: f64,
) -> Result<f64, IntegerRuntimeError> {
    let left = real_to_integer(left)?;
    let value = match operation {
        IntegerBinaryOperation::BitAnd => left & real_to_integer(right)?,
        IntegerBinaryOperation::BitOr => left | real_to_integer(right)?,
        IntegerBinaryOperation::BitXor => left ^ real_to_integer(right)?,
        IntegerBinaryOperation::Shl => {
            let count = shift_count(right)?;
            if count >= INTEGER_BITS {
                0
            } else {
                (left as u32).wrapping_shl(count) as i32
            }
        }
        IntegerBinaryOperation::Shr => {
            let count = shift_count(right)?;
            if count >= INTEGER_BITS {
                0
            } else {
                ((left as u32) >> count) as i32
            }
        }
    };
    Ok(f64::from(value))
}

pub(crate) fn shift_count(value: f64) -> Result<u32, IntegerRuntimeError> {
    // VAMS 4.2.11 treats the right operand as unsigned. A negative signed
    // integer therefore becomes a large unsigned count; every source bit is
    // shifted out and the logical result is zero.
    real_to_integer(value).map(|count| count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn real_conversion_rounds_nearest_with_half_away_from_zero() {
        for (input, expected) in [
            (35.2, 35),
            (35.5, 36),
            (35.7, 36),
            (-1.49, -1),
            (-1.5, -2),
            (1.5, 2),
        ] {
            assert_eq!(real_to_integer(input), Ok(expected), "input {input}");
        }
    }

    #[test]
    fn real_conversion_fails_closed_for_nonfinite_and_out_of_range_values() {
        for input in [
            f64::NAN,
            f64::INFINITY,
            f64::NEG_INFINITY,
            f64::from(i32::MAX) + 0.5,
            f64::from(i32::MIN) - 0.5,
        ] {
            assert!(real_to_integer(input).is_err(), "input {input}");
        }
    }

    #[test]
    fn logical_right_shift_zero_fills_a_negative_integer() {
        assert_eq!(
            integer_binary(IntegerBinaryOperation::Shr, -16.0, 2.0),
            Ok(1_073_741_820.0)
        );
    }

    #[test]
    fn left_shift_wraps_as_a_32_bit_twos_complement_result() {
        assert_eq!(
            integer_binary(IntegerBinaryOperation::Shl, 0x4000_0000_u32 as f64, 1.0),
            Ok(f64::from(i32::MIN))
        );
    }

    #[test]
    fn counts_at_or_beyond_the_width_shift_every_bit_out() {
        for count in [-1.0, 32.0, f64::from(i32::MAX)] {
            assert_eq!(
                integer_binary(IntegerBinaryOperation::Shl, 1.0, count),
                Ok(0.0),
                "count {count}"
            );
            assert_eq!(
                integer_binary(IntegerBinaryOperation::Shr, -1.0, count),
                Ok(0.0),
                "count {count}"
            );
        }
    }

    #[test]
    fn nonfinite_shift_counts_are_errors_instead_of_host_panics() {
        for count in [f64::NAN, f64::INFINITY] {
            assert!(integer_binary(IntegerBinaryOperation::Shl, 1.0, count).is_err());
        }
    }
}

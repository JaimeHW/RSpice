//! Shared runtime contract for the Verilog-AMS `integer` type.
//!
//! Verilog-AMS 2023 sections 3.2 and 4.2.1.1 define `integer` as a signed
//! 32-bit, two's-complement value and require real-to-integer conversion to
//! round to nearest with exact half cases away from zero. Section 4.2.11
//! defines both `<<` and `>>` as logical (zero-filling) shifts. Keeping those
//! rules here prevents the portable VM and the native/WASM backends from
//! inheriting their host language's cast and shift behavior.
//!
//! Ordinary integer arithmetic also stays here rather than relying on Rust's
//! build-mode-dependent overflow checks. Results use signed 32-bit,
//! two's-complement wrapping. Division truncates toward zero; the one signed
//! quotient that is not representable (`i32::MIN / -1`) wraps to `i32::MIN`.
//! Integer powers use exponentiation by squaring, so every accepted exponent
//! has a small, fixed upper bound on evaluation work.

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

/// Ordinary signed-32-bit arithmetic operations.
///
/// This is intentionally separate from [`IntegerBinaryOperation`]. Native and
/// WASM helper ABIs assign compact numeric codes to that existing bitwise enum;
/// adding arithmetic variants to it would silently reinterpret those codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "typed arithmetic execution is enabled in the following schema/backend tranche"
    )
)]
pub(crate) enum IntegerArithmeticOperation {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum IntegerRuntimeError {
    NonFiniteOperand { value: f64 },
    OperandOutOfRange { value: f64 },
    DivisionByZero,
    ModulusByZero,
    NegativeExponent { exponent: i32 },
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
            Self::DivisionByZero => write!(f, "signed 32-bit integer division by zero"),
            Self::ModulusByZero => write!(f, "signed 32-bit integer modulus by zero"),
            Self::NegativeExponent { exponent } => write!(
                f,
                "negative signed 32-bit integer exponent {exponent} is not supported"
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

/// Apply an ordinary signed-32-bit integer arithmetic operation.
///
/// The `f64` boundary is deliberate: all currently shipping evaluator and JIT
/// storage is scalar `f64`, and every `i32` is exactly representable there.
/// Both operands pass through the same checked Verilog-AMS integer conversion
/// as bitwise operations before arithmetic begins.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "typed arithmetic execution is enabled in the following schema/backend tranche"
    )
)]
pub(crate) fn integer_arithmetic(
    operation: IntegerArithmeticOperation,
    left: f64,
    right: f64,
) -> Result<f64, IntegerRuntimeError> {
    let left = real_to_integer(left)?;
    let right = real_to_integer(right)?;
    let value = match operation {
        IntegerArithmeticOperation::Add => left.wrapping_add(right),
        IntegerArithmeticOperation::Sub => left.wrapping_sub(right),
        IntegerArithmeticOperation::Mul => left.wrapping_mul(right),
        IntegerArithmeticOperation::Div => integer_div(left, right)?,
        IntegerArithmeticOperation::Mod => integer_mod(left, right)?,
        IntegerArithmeticOperation::Pow => integer_pow(left, right)?,
    };
    Ok(f64::from(value))
}

/// Apply signed-32-bit unary negation without host overflow behavior.
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "typed arithmetic execution is enabled in the following schema/backend tranche"
    )
)]
pub(crate) fn integer_neg(value: f64) -> Result<f64, IntegerRuntimeError> {
    real_to_integer(value).map(|value| f64::from(value.wrapping_neg()))
}

fn integer_div(left: i32, right: i32) -> Result<i32, IntegerRuntimeError> {
    if right == 0 {
        return Err(IntegerRuntimeError::DivisionByZero);
    }
    if left == i32::MIN && right == -1 {
        return Ok(i32::MIN);
    }
    Ok(left / right)
}

fn integer_mod(left: i32, right: i32) -> Result<i32, IntegerRuntimeError> {
    if right == 0 {
        return Err(IntegerRuntimeError::ModulusByZero);
    }
    if left == i32::MIN && right == -1 {
        return Ok(0);
    }
    Ok(left % right)
}

fn integer_pow(base: i32, exponent: i32) -> Result<i32, IntegerRuntimeError> {
    let mut exponent =
        u32::try_from(exponent).map_err(|_| IntegerRuntimeError::NegativeExponent { exponent })?;
    let mut base = base;
    let mut result = 1_i32;
    while exponent != 0 {
        if exponent & 1 != 0 {
            result = result.wrapping_mul(base);
        }
        exponent >>= 1;
        if exponent != 0 {
            base = base.wrapping_mul(base);
        }
    }
    Ok(result)
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
    fn real_conversion_checks_the_rounded_value_at_both_signed_boundaries() {
        for (input, expected) in [
            (f64::from(i32::MAX) + 0.49, i32::MAX),
            (f64::from(i32::MIN) - 0.49, i32::MIN),
            (f64::from(i32::MAX), i32::MAX),
            (f64::from(i32::MIN), i32::MIN),
            (-0.0, 0),
        ] {
            assert_eq!(real_to_integer(input), Ok(expected), "input {input}");
        }

        for input in [f64::from(i32::MAX) + 0.5, f64::from(i32::MIN) - 0.5] {
            assert!(matches!(
                real_to_integer(input),
                Err(IntegerRuntimeError::OperandOutOfRange { .. })
            ));
        }
    }

    #[test]
    fn arithmetic_operands_use_the_shared_checked_integer_conversion() {
        assert_eq!(
            integer_arithmetic(IntegerArithmeticOperation::Add, 1.5, -1.5),
            Ok(0.0)
        );
        assert_eq!(integer_neg(-1.5), Ok(2.0));

        for invalid in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, 1.0e300] {
            assert!(matches!(
                integer_arithmetic(IntegerArithmeticOperation::Add, invalid, 0.0),
                Err(IntegerRuntimeError::NonFiniteOperand { .. }
                    | IntegerRuntimeError::OperandOutOfRange { .. })
            ));
            assert!(matches!(
                integer_arithmetic(IntegerArithmeticOperation::Mul, 0.0, invalid),
                Err(IntegerRuntimeError::NonFiniteOperand { .. }
                    | IntegerRuntimeError::OperandOutOfRange { .. })
            ));
            assert!(integer_neg(invalid).is_err());
        }
    }

    #[test]
    fn add_subtract_and_multiply_wrap_at_signed_32_bit_boundaries() {
        let cases = [
            (IntegerArithmeticOperation::Add, i32::MAX, 1, i32::MIN),
            (IntegerArithmeticOperation::Add, i32::MIN, i32::MIN, 0),
            (IntegerArithmeticOperation::Add, i32::MAX, i32::MAX, -2),
            (IntegerArithmeticOperation::Sub, i32::MIN, 1, i32::MAX),
            (IntegerArithmeticOperation::Sub, i32::MAX, -1, i32::MIN),
            (IntegerArithmeticOperation::Sub, i32::MIN, i32::MAX, 1),
            (IntegerArithmeticOperation::Mul, i32::MAX, 2, -2),
            (IntegerArithmeticOperation::Mul, i32::MIN, -1, i32::MIN),
            (IntegerArithmeticOperation::Mul, i32::MIN, 2, 0),
        ];
        for (operation, left, right, expected) in cases {
            assert_eq!(
                integer_arithmetic(operation, f64::from(left), f64::from(right)),
                Ok(f64::from(expected)),
                "{operation:?}: {left}, {right}"
            );
        }
    }

    #[test]
    fn wrapping_arithmetic_matches_a_modulo_32_oracle_across_boundary_pairs() {
        fn signed32(value: i64) -> i32 {
            (value as u64 as u32) as i32
        }

        let values = [
            i32::MIN,
            i32::MIN + 1,
            -2,
            -1,
            0,
            1,
            2,
            i32::MAX - 1,
            i32::MAX,
        ];
        for left in values {
            for right in values {
                for (operation, wide_result) in [
                    (
                        IntegerArithmeticOperation::Add,
                        i64::from(left) + i64::from(right),
                    ),
                    (
                        IntegerArithmeticOperation::Sub,
                        i64::from(left) - i64::from(right),
                    ),
                    (
                        IntegerArithmeticOperation::Mul,
                        i64::from(left) * i64::from(right),
                    ),
                ] {
                    assert_eq!(
                        integer_arithmetic(operation, f64::from(left), f64::from(right)),
                        Ok(f64::from(signed32(wide_result))),
                        "{operation:?}: {left}, {right}"
                    );
                }
            }
        }
    }

    #[test]
    fn unary_negation_wraps_the_signed_minimum() {
        for (input, expected) in [
            (i32::MIN, i32::MIN),
            (i32::MAX, -i32::MAX),
            (-1, 1),
            (0, 0),
            (1, -1),
        ] {
            assert_eq!(integer_neg(f64::from(input)), Ok(f64::from(expected)));
        }
    }

    #[test]
    fn division_truncates_toward_zero_and_defines_the_overflow_edge() {
        for (left, right, expected) in [
            (7, 3, 2),
            (7, -3, -2),
            (-7, 3, -2),
            (-7, -3, 2),
            (0, -3, 0),
            (i32::MIN, -1, i32::MIN),
            (i32::MIN, 1, i32::MIN),
            (i32::MAX, -1, -i32::MAX),
        ] {
            assert_eq!(
                integer_arithmetic(
                    IntegerArithmeticOperation::Div,
                    f64::from(left),
                    f64::from(right)
                ),
                Ok(f64::from(expected)),
                "{left} / {right}"
            );
        }
        assert_eq!(
            integer_arithmetic(IntegerArithmeticOperation::Div, 1.0, 0.0),
            Err(IntegerRuntimeError::DivisionByZero)
        );
    }

    #[test]
    fn modulus_follows_the_dividend_sign_and_defines_the_overflow_edge() {
        for (left, right, expected) in [
            (7, 3, 1),
            (7, -3, 1),
            (-7, 3, -1),
            (-7, -3, -1),
            (0, -3, 0),
            (i32::MIN, -1, 0),
            (i32::MIN, 3, -2),
            (i32::MAX, -3, 1),
        ] {
            assert_eq!(
                integer_arithmetic(
                    IntegerArithmeticOperation::Mod,
                    f64::from(left),
                    f64::from(right)
                ),
                Ok(f64::from(expected)),
                "{left} % {right}"
            );
        }
        assert_eq!(
            integer_arithmetic(IntegerArithmeticOperation::Mod, 1.0, 0.0),
            Err(IntegerRuntimeError::ModulusByZero)
        );
    }

    #[test]
    fn power_wraps_with_bounded_exponentiation_by_squaring() {
        for (base, exponent, expected) in [
            (0, 0, 1),
            (0, 1, 0),
            (1, i32::MAX, 1),
            (-1, i32::MAX, -1),
            (-1, i32::MAX - 1, 1),
            (2, 30, 1 << 30),
            (2, 31, i32::MIN),
            (2, 32, 0),
            (-2, 31, i32::MIN),
            (i32::MAX, 2, 1),
        ] {
            assert_eq!(
                integer_arithmetic(
                    IntegerArithmeticOperation::Pow,
                    f64::from(base),
                    f64::from(exponent)
                ),
                Ok(f64::from(expected)),
                "{base} ** {exponent}"
            );
        }
    }

    #[test]
    fn negative_integer_exponents_fail_closed() {
        for exponent in [i32::MIN, -2, -1] {
            assert_eq!(
                integer_arithmetic(IntegerArithmeticOperation::Pow, 2.0, f64::from(exponent)),
                Err(IntegerRuntimeError::NegativeExponent { exponent })
            );
        }
    }

    #[test]
    fn arithmetic_errors_are_operation_specific_and_actionable() {
        for (error, expected) in [
            (
                IntegerRuntimeError::DivisionByZero,
                "signed 32-bit integer division by zero",
            ),
            (
                IntegerRuntimeError::ModulusByZero,
                "signed 32-bit integer modulus by zero",
            ),
            (
                IntegerRuntimeError::NegativeExponent { exponent: -3 },
                "negative signed 32-bit integer exponent -3 is not supported",
            ),
        ] {
            assert_eq!(error.to_string(), expected);
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

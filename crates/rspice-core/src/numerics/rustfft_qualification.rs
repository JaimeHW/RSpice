//! Resource qualification for RustFFT 6.4.1 forward plans.
//!
//! RustFFT's planner allocates internally through infallible APIs. This module
//! therefore rejects record lengths whose scalar, SSE, Neon, wasm-SIMD, or AVX
//! planner routes can expand into an unqualified Bluestein convolution before
//! a planner exists.

use thiserror::Error;

/// Maximum authored length accepted by the RustFFT 6.4.1 resource policy.
pub const MAX_QUALIFIED_RUSTFFT_LENGTH: usize = 1_048_576;
const MAX_QUALIFIED_BLUESTEIN_INNER_LENGTH: usize = 524_288;

/// A forward RustFFT plan cannot be constructed within the qualified bounds.
#[derive(Debug, Clone, PartialEq, Eq, Error)]
#[non_exhaustive]
pub enum RustfftQualificationError {
    /// The authored transform length exceeds the qualified record bound.
    #[error("rustfft forward length {length} exceeds the qualified limit {limit}")]
    LengthLimit {
        /// Requested forward-transform length.
        length: usize,
        /// Maximum qualified forward-transform length.
        limit: usize,
    },
    /// Checked planner-shape arithmetic could not be represented.
    #[error(
        "rustfft {route} Bluestein {operation} overflowed for base {base} in a {length}-sample record"
    )]
    ArithmeticOverflow {
        /// Requested forward-transform length.
        length: usize,
        /// Planner base whose derived shape overflowed.
        base: usize,
        /// Planner family or decomposition route.
        route: &'static str,
        /// Derived arithmetic operation that overflowed.
        operation: &'static str,
    },
    /// No AVX-f64 Bluestein candidate survived RustFFT's planner filter.
    #[error(
        "rustfft AVX f64 found no qualified {route} Bluestein candidate for base {base} in a {length}-sample record"
    )]
    AvxCandidateUnavailable {
        /// Requested forward-transform length.
        length: usize,
        /// Planner base requiring Bluestein expansion.
        base: usize,
        /// Planner family or decomposition route.
        route: &'static str,
    },
    /// At least one possible RustFFT planner route exceeds the inner bound.
    #[error(
        "rustfft planning for a {length}-sample record may route {route} {base} through Bluestein with minimum inner length {minimum_inner_length}, scalar/SSE/Neon/wasm-SIMD candidate {portable_inner_length}, and AVX f64 candidate {avx_f64_inner_length}; candidate {largest_inner_length} exceeds the qualified inner limit {limit}"
    )]
    BluesteinInnerLimit {
        /// Requested forward-transform length.
        length: usize,
        /// Planner base requiring Bluestein expansion.
        base: usize,
        /// Planner family or decomposition route.
        route: &'static str,
        /// Minimum legal Bluestein convolution length.
        minimum_inner_length: usize,
        /// RustFFT 6.4.1 scalar, SSE, Neon, and wasm-SIMD selected inner length.
        portable_inner_length: usize,
        /// RustFFT 6.4.1 AVX-f64 selected inner length.
        avx_f64_inner_length: usize,
        /// Larger of the possible selected inner lengths.
        largest_inner_length: usize,
        /// Maximum qualified inner-transform length.
        limit: usize,
    },
    /// The bounded prime-factor scan exhausted platform arithmetic.
    #[error("rustfft prime-factor scan exceeded this platform for length {length}")]
    FactorizationOverflow {
        /// Requested forward-transform length.
        length: usize,
    },
}

/// Qualify a forward transform length for RustFFT 6.4.1 planning.
///
/// This validates both the authored record bound and every scalar, SSE, Neon,
/// wasm-SIMD, or AVX-f64 Bluestein expansion reachable for the length. It does
/// not create, expose, or retain a plan. Call it immediately before entering
/// RustFFT's infallible planner API.
pub fn qualify_rustfft_forward_length(length: usize) -> Result<(), RustfftQualificationError> {
    if length > MAX_QUALIFIED_RUSTFFT_LENGTH {
        return Err(RustfftQualificationError::LengthLimit {
            length,
            limit: MAX_QUALIFIED_RUSTFFT_LENGTH,
        });
    }
    if length <= 1 {
        return Ok(());
    }

    // Scalar, SSE, Neon, and wasm-SIMD planners decompose composite records
    // and may use Bluestein for a prime base above the built-in butterflies
    // when Rader's p-1 transform itself contains a prime factor above 23.
    for_each_distinct_prime_factor(length, |prime| {
        if prime > 31 && has_prime_factor_above_23(prime - 1) {
            validate_bluestein_base(length, prime, "prime factor")?;
        }
        Ok(())
    })?;

    // AVX strips its fast 2/3/5/7/11 radixes and plans the entire remaining
    // product as one base. A composite base can therefore reach Bluestein even
    // when every individual prime factor was harmless above.
    let mut avx_other = length;
    for factor in [2, 3, 5, 7, 11] {
        while avx_other.is_multiple_of(factor) {
            avx_other /= factor;
        }
    }
    let avx_butterfly = matches!(avx_other, 1 | 13 | 17 | 19 | 23 | 29 | 31);
    let portable_rader = is_prime(avx_other) && has_only_two_and_three_factors(avx_other - 1);
    if !avx_butterfly && !portable_rader {
        validate_bluestein_base(length, avx_other, "AVX other-factor product")?;
    }
    Ok(())
}

fn validate_bluestein_base(
    length: usize,
    base: usize,
    route: &'static str,
) -> Result<(), RustfftQualificationError> {
    let minimum_inner_length = base
        .checked_mul(2)
        .and_then(|value| value.checked_sub(1))
        .ok_or(RustfftQualificationError::ArithmeticOverflow {
            length,
            base,
            route,
            operation: "minimum-inner calculation",
        })?;
    let power_of_two = minimum_inner_length.checked_next_power_of_two().ok_or(
        RustfftQualificationError::ArithmeticOverflow {
            length,
            base,
            route,
            operation: "power-of-two candidate",
        },
    )?;
    let three_quarters = power_of_two
        .checked_div(4)
        .and_then(|value| value.checked_mul(3))
        .ok_or(RustfftQualificationError::ArithmeticOverflow {
            length,
            base,
            route,
            operation: "three-quarter candidate",
        })?;
    let portable_inner_length = if three_quarters >= minimum_inner_length {
        three_quarters
    } else {
        power_of_two
    };
    let avx_f64_inner_length =
        rustfft_avx_f64_bluestein_inner(length, base, route, minimum_inner_length, power_of_two)?;
    let largest_inner_length = portable_inner_length.max(avx_f64_inner_length);
    if largest_inner_length > MAX_QUALIFIED_BLUESTEIN_INNER_LENGTH {
        return Err(RustfftQualificationError::BluesteinInnerLimit {
            length,
            base,
            route,
            minimum_inner_length,
            portable_inner_length,
            avx_f64_inner_length,
            largest_inner_length,
            limit: MAX_QUALIFIED_BLUESTEIN_INNER_LENGTH,
        });
    }
    Ok(())
}

fn rustfft_avx_f64_bluestein_inner(
    length: usize,
    base: usize,
    route: &'static str,
    minimum_inner_length: usize,
    power_of_two: usize,
) -> Result<usize, RustfftQualificationError> {
    // Mirror RustFFT 6.4.1's AVX-f64 2^n*3^m candidate generator and
    // benchmark filter, retaining only its smallest accepted candidate.
    let mut candidate = power_of_two;
    let mut factor_two = candidate.trailing_zeros();
    let mut factor_three = 0u32;
    let mut chosen = None;
    while factor_two >= 2 {
        let rejected =
            (factor_three < 1 && factor_two > 13) || (factor_three < 4 && factor_two > 14);
        if candidate >= minimum_inner_length && !rejected {
            chosen = Some(chosen.map_or(candidate, |prior: usize| prior.min(candidate)));
        }
        if candidate >= power_of_two {
            candidate >>= 1;
            factor_two -= 1;
        } else {
            candidate =
                candidate
                    .checked_mul(3)
                    .ok_or(RustfftQualificationError::ArithmeticOverflow {
                        length,
                        base,
                        route,
                        operation: "AVX f64 candidate",
                    })?;
            factor_three = factor_three.checked_add(1).ok_or(
                RustfftQualificationError::ArithmeticOverflow {
                    length,
                    base,
                    route,
                    operation: "AVX f64 factor count",
                },
            )?;
        }
    }
    chosen.ok_or(RustfftQualificationError::AvxCandidateUnavailable {
        length,
        base,
        route,
    })
}

fn for_each_distinct_prime_factor(
    original: usize,
    mut visit: impl FnMut(usize) -> Result<(), RustfftQualificationError>,
) -> Result<(), RustfftQualificationError> {
    let mut value = original;
    if value.is_multiple_of(2) {
        visit(2)?;
        while value.is_multiple_of(2) {
            value /= 2;
        }
    }
    let mut divisor = 3usize;
    while divisor <= value / divisor {
        if value.is_multiple_of(divisor) {
            visit(divisor)?;
            while value.is_multiple_of(divisor) {
                value /= divisor;
            }
        }
        divisor = divisor
            .checked_add(2)
            .ok_or(RustfftQualificationError::FactorizationOverflow { length: original })?;
    }
    if value > 1 {
        visit(value)?;
    }
    Ok(())
}

fn has_prime_factor_above_23(mut value: usize) -> bool {
    for factor in [2, 3, 5, 7, 11, 13, 17, 19, 23] {
        while value.is_multiple_of(factor) {
            value /= factor;
        }
    }
    value > 1
}

fn has_only_two_and_three_factors(mut value: usize) -> bool {
    for factor in [2, 3] {
        while value.is_multiple_of(factor) {
            value /= factor;
        }
    }
    value == 1
}

fn is_prime(value: usize) -> bool {
    if value < 2 {
        return false;
    }
    if value.is_multiple_of(2) {
        return value == 2;
    }
    let mut divisor = 3usize;
    while divisor <= value / divisor {
        if value.is_multiple_of(divisor) {
            return false;
        }
        let Some(next) = divisor.checked_add(2) else {
            return false;
        };
        divisor = next;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smooth_and_degenerate_lengths_are_qualified() {
        for length in [0, 1, 8, 786_432, MAX_QUALIFIED_RUSTFFT_LENGTH] {
            qualify_rustfft_forward_length(length)
                .unwrap_or_else(|error| panic!("length {length} should qualify: {error}"));
        }
    }

    #[test]
    fn authored_length_limit_is_typed() {
        assert!(matches!(
            qualify_rustfft_forward_length(MAX_QUALIFIED_RUSTFFT_LENGTH + 1),
            Err(RustfftQualificationError::LengthLimit { .. })
        ));
    }

    #[test]
    fn scalar_and_avx_bluestein_expansions_fail_closed() {
        let prime = qualify_rustfft_forward_length(1_048_573)
            .expect_err("the large prime requires an over-budget Bluestein transform");
        assert!(matches!(
            prime,
            RustfftQualificationError::BluesteinInnerLimit { .. }
        ));

        // Scalar planners see two individually manageable prime bases, while
        // AVX combines the non-fast 521 and 523 factors into one Bluestein base.
        let composite = qualify_rustfft_forward_length(521 * 523)
            .expect_err("the composite AVX other-factor base must be bounded");
        assert!(composite.to_string().contains("AVX other-factor product"));

        let avx_rader = qualify_rustfft_forward_length(267_037)
            .expect_err("the prime has no portable AVX Rader route");
        assert!(avx_rader.to_string().contains("AVX other-factor product"));

        let avx_filter = qualify_rustfft_forward_length(248_839)
            .expect_err("AVX f64 rejects the otherwise bounded pure-power-of-two candidate");
        assert!(avx_filter.to_string().contains("AVX f64 candidate 559872"));
    }
}

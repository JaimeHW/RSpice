//! Numerical methods shared by every analysis.
//!
//! What the analyses have in common below the level of any one of them: how a
//! derivative is discretized, how large a step may be, and where a step is not
//! allowed to land. These sit beneath the circuit store and the device models,
//! because those stamp into structures this module defines.
//!
//! `crate::solver` — sparse LU, the Newton loop, damping and continuation —
//! is the other half of this and sits one layer up, because it solves against
//! an assembled matrix rather than defining one.

pub(crate) mod eigenspectrum;
mod power_law;
pub(crate) use power_law::{power_product_binary_normalization, scaled_power_law};
pub mod integration;
pub mod rustfft_qualification;

use crate::Value;

/// Scalar operations for the small, stack-allocated device reductions.
pub(crate) trait DenseScalar:
    Copy + Default + std::ops::SubAssign + std::ops::Mul<Output = Self>
{
    fn magnitude(self) -> Value;
    fn finite(self) -> bool;
    fn scale_down(self, scale: Value) -> Self;
    fn scale_binary(self, exponent: i32) -> Self;
    fn quotient(self, divisor: Self) -> Self;
}

impl DenseScalar for Value {
    fn magnitude(self) -> Value {
        self.abs()
    }
    fn finite(self) -> bool {
        self.is_finite()
    }
    fn scale_down(self, scale: Value) -> Self {
        self / scale
    }
    fn scale_binary(self, exponent: i32) -> Self {
        libm::scalbn(self, exponent)
    }
    fn quotient(self, divisor: Self) -> Self {
        self / divisor
    }
}

impl DenseScalar for crate::Complex64 {
    fn magnitude(self) -> Value {
        self.re.abs().max(self.im.abs())
    }
    fn finite(self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }
    fn scale_down(self, scale: Value) -> Self {
        self / scale
    }
    fn scale_binary(self, exponent: i32) -> Self {
        Self::new(
            libm::scalbn(self.re, exponent),
            libm::scalbn(self.im, exponent),
        )
    }
    fn quotient(self, divisor: Self) -> Self {
        // Complex's generic division squares its denominator and can lose
        // a tiny numerator product even when that square is normal. Scale
        // before either product, without forming a reciprocal first.
        let scale = divisor.magnitude();
        let b = divisor / scale;
        let denominator = b.norm_sqr();
        let a = self / scale;
        let a = if a.finite() {
            a / denominator
        } else {
            // The normalized denominator lies in [1, 2]. Apply it first
            // when self/scale would overflow but the quotient may fit.
            (self / denominator) / scale
        };
        Self::new(a.re * b.re + a.im * b.im, a.im * b.re - a.re * b.im)
    }
}

/// Row-equilibrated partial-pivot solve for small private device systems.
/// No physical-unit pivot floor: a nonzero pivot is usable only when the
/// resulting finite solution passes a scale-independent backward-error check.
/// Inactive rows/columns beyond `dim` are ignored, and inactive outputs are zero.
pub(crate) fn solve_small_dense<T: DenseScalar, const N: usize>(
    matrix: &[[T; N]; N],
    rhs: &[T; N],
    dim: usize,
) -> Option<[T; N]> {
    solve_small_dense_many(matrix, &rhs.map(|value| [value]), dim)
        .map(|solution| solution.map(|row| row[0]))
}

/// Solve multiple RHS columns with one elimination; validate every solution.
pub(crate) fn solve_small_dense_many<T: DenseScalar, const N: usize, const R: usize>(
    matrix: &[[T; N]; N],
    rhs: &[[T; R]; N],
    dim: usize,
) -> Option<[[T; R]; N]> {
    if dim > N {
        return None;
    }
    let mut a = *matrix;
    let mut b = *rhs;
    for row in 0..dim {
        if b[row].iter().any(|v| !v.finite()) || a[row][..dim].iter().any(|v| !v.finite()) {
            return None;
        }
        let scale = a[row][..dim]
            .iter()
            .fold(0.0_f64, |s, v| s.max(v.magnitude()));
        if scale == 0.0 {
            return None;
        }
        for entry in &mut a[row][..dim] {
            *entry = entry.scale_down(scale);
        }
        for entry in &mut b[row] {
            *entry = entry.scale_down(scale);
            if !entry.finite() {
                return None;
            }
        }
    }
    for pivot in 0..dim {
        let mut best = pivot;
        for row in (pivot + 1)..dim {
            if a[row][pivot].magnitude() > a[best][pivot].magnitude() {
                best = row;
            }
        }
        if a[best][pivot].magnitude() == 0.0 || !a[best][pivot].finite() {
            return None;
        }
        a.swap(pivot, best);
        b.swap(pivot, best);
        let pivot_value = a[pivot][pivot];
        for row in (pivot + 1)..dim {
            let factor = a[row][pivot].quotient(pivot_value);
            if !factor.finite() {
                return None;
            }
            let (above, below) = a.split_at_mut(row);
            let pivot_row = &above[pivot];
            let target = &mut below[0];
            target[pivot] = T::default();
            for col in (pivot + 1)..dim {
                target[col] -= factor * pivot_row[col];
            }
            let (prior_rhs, later_rhs) = b.split_at_mut(row);
            for (entry, &pivot_entry) in later_rhs[0].iter_mut().zip(&prior_rhs[pivot]) {
                *entry -= factor * pivot_entry;
            }
        }
    }
    let mut solution = [[T::default(); R]; N];
    for row in (0..dim).rev() {
        for column in 0..R {
            let mut residual = b[row][column];
            for col in (row + 1)..dim {
                residual -= a[row][col] * solution[col][column];
            }
            solution[row][column] = residual.quotient(a[row][row]);
            if !solution[row][column].finite() {
                return None;
            }
        }
    }
    // Certify original equations using binary-scaled products. A common
    // floating row/x normalization can erase a small coefficient whose large
    // solution still makes a significant contribution to the residual.
    let tolerance = 64.0 * Value::EPSILON * dim.max(1) as Value;
    for column in 0..R {
        for row in 0..dim {
            let rhs_magnitude = rhs[row][column].magnitude();
            let mut exponent = if rhs_magnitude > 0.0 {
                libm::ilogb(rhs_magnitude)
            } else {
                i32::MIN
            };
            for col in 0..dim {
                let a = matrix[row][col].magnitude();
                let x = solution[col][column].magnitude();
                if a > 0.0 && x > 0.0 {
                    exponent = exponent.max(libm::ilogb(a) + libm::ilogb(x));
                }
            }
            if exponent == i32::MIN {
                continue;
            }
            let mut residual = rhs[row][column].scale_binary(-exponent);
            let mut bound = residual.magnitude();
            for col in 0..dim {
                let a = matrix[row][col];
                let x = solution[col][column];
                if a.magnitude() == 0.0 || x.magnitude() == 0.0 {
                    continue;
                }
                let a_exp = libm::ilogb(a.magnitude());
                let x_exp = libm::ilogb(x.magnitude());
                let a = a.scale_binary(-a_exp);
                let x = x.scale_binary(-x_exp);
                let shift = a_exp + x_exp - exponent;
                residual -= (a * x).scale_binary(shift);
                // Component-max complex magnitudes need the factor of two.
                bound += libm::scalbn(2.0 * a.magnitude() * x.magnitude(), shift);
            }
            if !residual.finite() || residual.magnitude() > tolerance * bound {
                return None;
            }
        }
    }
    Some(solution)
}

/// Evaluate a product/quotient times exp(exponent), retaining the ordinary
/// arithmetic path unless an intermediate loses range or subnormal precision.
#[inline]
pub(crate) fn scaled_exp_product(
    factors: &[crate::Value],
    divisors: &[crate::Value],
    exponent: crate::Value,
) -> crate::Value {
    if factors.contains(&0.0) {
        return 0.0;
    }
    let exponential = exponent.exp();
    let mut product = exponential;
    let mut ordinary = exponential.is_normal();
    for &factor in factors {
        product *= factor;
        ordinary &= product.is_normal();
    }
    for &divisor in divisors {
        product /= divisor;
        ordinary &= product.is_normal();
    }
    if ordinary {
        product
    } else {
        scaled_exp_product_fallback(factors, divisors, exponent, exponential, 0)
    }
}

/// Retain a product's binary scale when its value cannot be stored in f64.
/// Inputs must be finite and nonzero. A normal product keeps its ordinary
/// evaluation exactly; the normalized form also preserves subnormal precision.
pub(crate) fn product_binary_normalization(factors: &[Value], divisors: &[Value]) -> (Value, i32) {
    let product = scaled_exp_product(factors, divisors, 0.0);
    if product.is_normal() {
        return (product, 0);
    }
    debug_assert!(
        factors
            .iter()
            .chain(divisors)
            .all(|v| v.is_finite() && *v != 0.0)
    );
    let (mantissa, power) = product_binary_parts(factors, divisors);
    (
        mantissa,
        power.try_into().expect("bounded device normalization"),
    )
}

fn product_binary_parts(factors: &[Value], divisors: &[Value]) -> (Value, i64) {
    let mut mantissa = 1.0;
    let mut power = 0_i64;
    for &factor in factors {
        let e = libm::ilogb(factor);
        mantissa *= libm::scalbn(factor, -e);
        power += i64::from(e);
    }
    for &divisor in divisors {
        let e = libm::ilogb(divisor);
        mantissa /= libm::scalbn(divisor, -e);
        power -= i64::from(e);
    }
    (mantissa, power)
}

/// Combine a separately retained binary scale before rounding the final value.
pub(crate) fn scaled_exp_product_with_binary_scale(
    factors: &[Value],
    divisors: &[Value],
    exponent: Value,
    binary_scale: i32,
) -> Value {
    if binary_scale == 0 {
        return scaled_exp_product(factors, divisors, exponent);
    }
    if factors.contains(&0.0) {
        return 0.0;
    }
    scaled_exp_product_fallback(factors, divisors, exponent, exponent.exp(), binary_scale)
}

#[cold]
fn scaled_exp_product_fallback(
    factors: &[crate::Value],
    divisors: &[crate::Value],
    exponent: crate::Value,
    exponential: crate::Value,
    binary_scale: i32,
) -> crate::Value {
    if exponent.is_nan()
        || factors.iter().any(|value| !value.is_finite())
        || divisors
            .iter()
            .any(|value| !value.is_finite() || *value == 0.0)
    {
        return crate::Value::NAN;
    }
    let (mut mantissa, mut power) = product_binary_parts(factors, divisors);
    power += i64::from(binary_scale);
    if exponential.is_normal() {
        let e = libm::ilogb(exponential);
        mantissa *= libm::scalbn(exponential, -e);
        power += i64::from(e);
    } else {
        // No finite input factor can compensate an exponent beyond this
        // bound. It also keeps conversion/reduction within the integer range.
        let bound = ((factors.len() + divisors.len() + 2) as crate::Value * 1075.0
            + Value::from(binary_scale).abs())
            * std::f64::consts::LN_2;
        if exponent > bound {
            return crate::Value::INFINITY.copysign(mantissa);
        }
        if exponent < -bound {
            return 0.0_f64.copysign(mantissa);
        }
        let e = (exponent * std::f64::consts::LOG2_E).round() as i64;
        // Residual of ln(2) after rounding its high part to f64. FMA and the
        // low part prevent range reduction from discarding significant bits.
        const LN_2_LOW: crate::Value = 2.319_046_813_846_299_6e-17;
        let reduced = (-(e as crate::Value)).mul_add(std::f64::consts::LN_2, exponent)
            - e as crate::Value * LN_2_LOW;
        mantissa *= reduced.exp();
        power += e;
    }
    libm::scalbn(
        mantissa,
        power.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32,
    )
}

/// Infinity norm for residual and state vectors. Nonfinite entries yield
/// infinity, so an invalid equation cannot disappear from a maximum reduction.
pub(crate) fn infinity_norm(values: &[Value]) -> Value {
    let mut norm = 0.0_f64;
    for value in values {
        if !value.is_finite() {
            return Value::INFINITY;
        }
        norm = norm.max(value.abs());
    }
    norm
}

/// Scaled sum of squares, retaining ratios even when a norm exceeds f64 range.
#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct ScaledL2Norm {
    scale: Value,
    squared_sum: Value,
}

impl ScaledL2Norm {
    pub(crate) fn between(old: &[Value], new: &[Value]) -> Self {
        let mut norm = Self::default();
        for (&a, &b) in old.iter().zip(new) {
            let delta = b - a;
            // Opposite finite endpoints may have an unrepresentable delta.
            // Its half remains finite; a weight of four retains its square.
            let (magnitude, weight) = if delta.is_infinite() && a.is_finite() && b.is_finite() {
                ((0.5 * b - 0.5 * a).abs(), 4.0)
            } else {
                (delta.abs(), 1.0)
            };
            norm.accumulate(magnitude, weight);
        }
        norm
    }

    #[inline]
    fn accumulate(&mut self, magnitude: Value, weight: Value) {
        if !magnitude.is_finite() {
            self.scale = Value::INFINITY;
            self.squared_sum = 1.0;
        } else if magnitude > self.scale {
            self.squared_sum = weight + self.squared_sum * (self.scale / magnitude).powi(2);
            self.scale = magnitude;
        } else if magnitude != 0.0 {
            self.squared_sum += weight * (magnitude / self.scale).powi(2);
        }
    }

    pub(crate) fn ratio(self, previous: Self) -> Value {
        if self.scale == 0.0 {
            0.0
        } else if previous.scale == 0.0 {
            Value::INFINITY
        } else {
            (self.scale / previous.scale) * (self.squared_sum / previous.squared_sum).sqrt()
        }
    }
}

/// Componentwise Newton update test shared by scalar and SIMD entry points.
/// Each finite coordinate must satisfy `|new-old| <= abs_tol + rel_tol*scale`,
/// where `scale = max(|old|, |new|)`. Both tolerances must be finite and nonnegative.
pub(crate) fn solution_update_converged(
    old: &[Value],
    new: &[Value],
    abs_tol: Value,
    rel_tol: Value,
) -> bool {
    if old.len() != new.len()
        || !abs_tol.is_finite()
        || abs_tol < 0.0
        || !rel_tol.is_finite()
        || rel_tol < 0.0
    {
        return false;
    }

    #[cfg(feature = "simd")]
    let (old, new) = {
        use wide::f64x4;
        let end = if old.len() >= 16 {
            old.len() / 4 * 4
        } else {
            0
        };
        let absolute = f64x4::splat(abs_tol);
        let relative = f64x4::splat(rel_tol);
        let minimum = f64x4::splat(Value::from_bits(1));
        for start in (0..end).step_by(4) {
            let a = f64x4::from(&old[start..start + 4]);
            let b = f64x4::from(&new[start..start + 4]);
            if !(a.is_finite() & b.is_finite()).all() {
                return false;
            }
            let difference = (b - a).abs();
            if difference.is_finite().all() {
                let scale = a.abs().max(b.abs()).max(minimum);
                if !(difference / scale)
                    .simd_le(absolute / scale + relative)
                    .all()
                {
                    return false;
                }
            } else {
                // Finite opposite endpoints can have an overflowing difference.
                // Only those exceptional chunks need the scaled scalar path.
                for lane in start..start + 4 {
                    if !solution_component_converged(old[lane], new[lane], abs_tol, rel_tol) {
                        return false;
                    }
                }
            }
        }
        (&old[end..], &new[end..])
    };

    old.iter()
        .zip(new)
        .all(|(&a, &b)| solution_component_converged(a, b, abs_tol, rel_tol))
}

#[inline]
fn solution_component_converged(a: Value, b: Value, abs_tol: Value, rel_tol: Value) -> bool {
    if !a.is_finite() || !b.is_finite() {
        return false;
    }
    // Scaling avoids overflow in both the update and the tolerance sum.
    // The least subnormal only gives two exact zeros a nonzero denominator.
    let scale = a.abs().max(b.abs()).max(Value::from_bits(1));
    let difference = (b - a).abs();
    let relative_difference = if difference.is_finite() {
        difference / scale
    } else {
        (b / scale - a / scale).abs()
    };
    relative_difference <= abs_tol / scale + rel_tol
}

/// Authenticate an authored frequency ratio without accepting a fraction of
/// a cycle as floating-point tolerance. Zero is a separate constant case.
pub(crate) fn is_integral_cycle_count(cycles: Value) -> bool {
    cycles.is_finite()
        && cycles > 0.0
        && (cycles - cycles.round()).abs() <= (32.0 * Value::EPSILON * cycles.max(1.0)).min(1e-10)
        && cycles.round() >= 1.0
}

/// Smallest PWL feature interval, discarding interior knots in constant runs.
/// Keep both sides of ideal jumps and the complete width of a flat pulse;
/// redundant flat knots must not force an arbitrarily fine integration grid.
pub(crate) fn minimum_pwl_interval(
    points: impl IntoIterator<Item = (Value, Value)>,
) -> Option<Value> {
    let mut points = pwl_event_points(points, false);
    let mut previous = points.next()?;
    let mut minimum: Option<Value> = None;
    let mut changes = false;
    for point in points {
        changes |= point.1 != previous.1;
        let interval = (point.0 - previous.0).abs();
        if interval > 0.0 && interval.is_finite() {
            minimum = Some(minimum.map_or(interval, |value| value.min(interval)));
        }
        previous = point;
    }
    if changes { minimum } else { None }
}

/// PWL event knots, optionally retaining every authored point. Interior knots
/// of a constant run do not change the value or slope; retaining them as
/// physical events can demand a meaningless tiny integration step.
pub(crate) fn pwl_event_points(
    points: impl IntoIterator<Item = (Value, Value)>,
    retain_flat_knots: bool,
) -> impl Iterator<Item = (Value, Value)> {
    let mut points = points.into_iter().peekable();
    let mut previous: Option<(Value, Value)> = None;
    std::iter::from_fn(move || {
        loop {
            let point = points.next()?;
            let flat = !retain_flat_knots
                && previous.is_some_and(|previous| previous.1 == point.1)
                && points.peek().is_some_and(|next| next.1 == point.1);
            previous = Some(point);
            if !flat {
                return Some(point);
            }
        }
    })
}

/// Map a PWL clock into its repeated tail. The authored endpoint is retained
/// at exact repeat boundaries; the next representable instant belongs to the
/// next cycle. A tolerance in seconds would flatten small waveforms and hold
/// discontinuous endpoints past their seam.
pub(crate) fn pwl_repeated_time(
    time: Value,
    first: Value,
    last: Value,
    repeat_from: Option<Value>,
) -> Value {
    let Some(start) = repeat_from.filter(|start| start.is_finite()) else {
        return time;
    };
    let start = start.max(first);
    let period = last - start;
    if !time.is_finite() || time <= last || !period.is_finite() || period <= 0.0 {
        return time;
    }
    let elapsed = time - last;
    let remainder = elapsed.rem_euclid(period);
    // Multiplication can round an authored boundary differently from modulo.
    // Authenticate its represented clock instead of widening the seam into
    // an interval, which would include an actual point after the boundary.
    let cycle = (elapsed / period).round();
    if remainder == 0.0 || (cycle.is_finite() && time == last + cycle * period) {
        last
    } else {
        start + remainder
    }
}

/// Neumaier compensated accumulation. Callers scale their operands when an
/// unscaled sum could overflow; compensation recovers low-order terms lost
/// when finite contributions of opposite sign nearly cancel.
#[inline]
pub(crate) fn compensated_add(sum: &mut Value, correction: &mut Value, value: Value) {
    let next = *sum + value;
    *correction += if sum.abs() >= value.abs() {
        (*sum - next) + value
    } else {
        (value - next) + *sum
    };
    *sum = next;
}

/// The smallest timestep Xyce will take at `current_time`, and the scale its
/// breakpoint comparisons are measured against.
///
/// Xyce derives this from the floating-point resolution of the clock itself:
/// once a step is small enough that adding it to the current time changes only
/// the last few bits, advancing it means nothing. Twice this value is the
/// tolerance for deciding whether a transient has landed *on* a waveform
/// breakpoint, which is why a source waveform needs it as much as the step
/// controller does.
///
/// Multiplying before taking the magnitude would overflow near `Value::MAX`, so
/// the magnitude comes first; a non-finite clock has no resolution to speak of
/// and yields zero.
#[must_use]
pub fn xyce_hard_min_timestep(current_time: Value) -> Value {
    if current_time.is_finite() {
        current_time.abs() * (10.0 * Value::EPSILON)
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_dense_solves_preserve_row_scales_and_tiny_complex_pivots() {
        use crate::Complex64;
        for (first, second) in [
            (1.0, 1.0),
            (1e-300, 1e300),
            (1e300, 1e-300),
            (Value::from_bits(4), 1.0),
        ] {
            let matrix = [
                [2.0 * first, first, Value::NAN],
                [second, -3.0 * second, Value::NAN],
                [Value::NAN; 3],
            ];
            let rhs = [first, 11.0 * second, Value::NAN];
            let real = solve_small_dense(&matrix, &rhs, 2).unwrap();
            let complex_matrix = matrix.map(|row| row.map(|v| Complex64::new(v, v)));
            let complex_rhs = rhs.map(|v| Complex64::new(v, v));
            let complex = solve_small_dense(&complex_matrix, &complex_rhs, 2).unwrap();
            for ((a, b), expected) in real.into_iter().zip(complex).zip([2.0, -3.0, 0.0]) {
                assert!((a - expected).abs() < 1e-13);
                assert!((b - Complex64::new(expected, 0.0)).norm() < 1e-13);
            }
        }
        for (pivot, drive) in [(1e-200, 1.0), (1e-300, 1.0), (1e-150, 1e-200)] {
            let matrix = [
                [Complex64::new(1.0, 0.0), Complex64::default()],
                [Complex64::new(1.0, 0.0), Complex64::new(0.0, pivot)],
            ];
            let rhs = [Complex64::default(), Complex64::new(drive, 0.0)];
            let solution = solve_small_dense(&matrix, &rhs, 2).unwrap();
            assert_eq!(solution[0], rhs[0]);
            assert!((solution[1].im / (drive / pivot) + 1.0).abs() < 1e-14);
            assert_eq!(solution[1].re, 0.0);
        }
    }

    #[test]
    fn small_dense_multiple_rhs_preserve_independent_solution_scales() {
        let matrix = [[2e-200, 1e-200], [1e200, -3e200]];
        let rhs = [
            [1e-200, 0.0, -6e-200, 1e-300],
            [11e200, 0.0, 18e200, 11e100],
        ];
        let expected = [[2.0, 0.0, 0.0, 2e-100], [-3.0, 0.0, -6.0, -3e-100]];
        let actual = solve_small_dense_many(&matrix, &rhs, 2).unwrap();
        for (row, reference) in actual.iter().zip(expected) {
            for (&value, expected) in row.iter().zip(reference) {
                if expected == 0.0 {
                    assert!(value.abs() < 1e-13);
                } else {
                    assert!((value / expected - 1.0).abs() < 1e-13);
                }
            }
        }
        let independent = solve_small_dense_many(&[[1.0]], &[[1e300, 1e-300]], 1).unwrap();
        assert_eq!(independent, [[1e300, 1e-300]]);
        let mixed_rows =
            solve_small_dense_many(&[[1.0, 0.0], [0.0, 1.0]], &[[1e300], [1e-300]], 2).unwrap();
        assert_eq!(mixed_rows, [[1e300], [1e-300]]);
        // Row equilibration loses the 1e-300 coupling. Its actual product
        // with x[1]=1e300 is significant, so the original residual must fail.
        assert!(solve_small_dense(&[[1e300, 1e-300], [0.0, 1.0]], &[1.0, 1e300], 2).is_none());
        for column in 0..4 {
            let mut invalid = rhs;
            invalid[0][column] = Value::NAN;
            assert!(solve_small_dense_many(&matrix, &invalid, 2).is_none());
        }
    }

    #[test]
    fn small_dense_solves_reject_invalid_or_singular_systems() {
        use crate::Complex64;
        assert_eq!(
            solve_small_dense(&[[Value::NAN; 2]; 2], &[Value::NAN; 2], 0),
            Some([0.0; 2])
        );
        assert!(solve_small_dense(&[[1.0; 2]; 2], &[1.0; 2], 3).is_none());
        for rhs in [[0.0, 0.0], [1.0, 2.0]] {
            assert!(solve_small_dense(&[[1.0; 2]; 2], &rhs, 2).is_none());
            assert!(
                solve_small_dense(
                    &[[Complex64::new(1.0, 1.0); 2]; 2],
                    &rhs.map(|x| Complex64::new(x, 0.0)),
                    2
                )
                .is_none()
            );
        }
        for invalid in [Value::NAN, Value::INFINITY, Value::NEG_INFINITY] {
            for lane in 0..2 {
                let mut rhs = [1.0, 2.0];
                rhs[lane] = invalid;
                assert!(solve_small_dense(&[[1.0, 0.0], [0.0, 1.0]], &rhs, 2).is_none());
                for col in 0..2 {
                    let mut matrix = [[1.0, 0.0], [0.0, 1.0]];
                    matrix[lane][col] = invalid;
                    assert!(solve_small_dense(&matrix, &[1.0, 2.0], 2).is_none());
                    assert!(
                        solve_small_dense(
                            &matrix.map(|r| r.map(|x| Complex64::new(0.0, x))),
                            &[Complex64::new(1.0, 0.0); 2],
                            2
                        )
                        .is_none()
                    );
                }
            }
        }
        assert!(solve_small_dense(&[[Value::MIN_POSITIVE]], &[Value::MAX], 1).is_none());
    }

    #[test]
    fn residual_norms_preserve_finite_scale_and_reject_every_nonfinite_entry() {
        for scale in [Value::from_bits(1), 1e-300, 1e-200, 1.0, 1e200, 1e307] {
            let values = [3.0 * scale, -4.0 * scale];
            assert_eq!(infinity_norm(&values), 4.0 * scale);
        }
        assert_eq!(infinity_norm(&[]), 0.0);
        for lane in 0..10 {
            for invalid in [Value::NAN, Value::INFINITY, Value::NEG_INFINITY] {
                let mut values = [1.0; 10];
                values[lane] = invalid;
                assert_eq!(infinity_norm(&values), Value::INFINITY);
            }
        }
    }

    #[test]
    fn update_tolerances_are_consistent_at_extreme_scales_and_simd_boundaries() {
        let tiny = Value::from_bits(1);
        for length in [1, 15, 16, 17, 32, 33] {
            for (old, new, absolute, relative, expected) in [
                (0.0, 0.0, 0.0, 0.0, true),
                (0.0, tiny, 0.0, 0.0, false),
                (0.0, tiny, tiny, 0.0, true),
                (1.0, 1.5, 0.5, 0.0, true),
                (1.0, 1.5_f64.next_up(), 0.5, 0.0, false),
                (-Value::MAX, Value::MAX, Value::MAX, 0.5, false),
                (-Value::MAX, Value::MAX, Value::MAX, 1.0, true),
                (Value::MAX, Value::MAX.next_down(), 0.0, 1e-15, true),
                (Value::MAX, 0.0, 0.0, 1e-15, false),
            ] {
                assert_eq!(
                    solution_update_converged(
                        &vec![old; length],
                        &vec![new; length],
                        absolute,
                        relative
                    ),
                    expected,
                    "length={length}, old={old}, new={new}, abs={absolute}, rel={relative}"
                );
            }
        }
        assert!(!solution_update_converged(&[1.0], &[1.0, 2.0], 1e-12, 1e-6));
    }

    #[test]
    fn pwl_feature_width_ignores_redundant_holds_and_retains_ideal_pulses() {
        assert_eq!(
            minimum_pwl_interval([
                (0.0, 0.0),
                (1e-300, 0.0),
                (0.25, 0.0),
                (0.5, 1.0),
                (1.0, 0.0)
            ]),
            Some(0.25)
        );
        assert_eq!(
            minimum_pwl_interval([
                (0.0, 0.0),
                (0.5, 0.0),
                (0.5, 1.0),
                (0.6, 1.0),
                (0.6, 0.0),
                (1.0, 0.0)
            ]),
            Some(0.6 - 0.5)
        );
        assert_eq!(
            minimum_pwl_interval([(0.0, 1.0), (1e-300, 1.0), (1.0, 1.0)]),
            None
        );
    }

    #[test]
    fn xyce_hard_minimum_tracks_current_time_machine_precision() {
        let transition_time = 5.380_978_556_560e-4;
        assert_eq!(xyce_hard_min_timestep(0.0).to_bits(), 0.0f64.to_bits());
        assert_eq!(
            xyce_hard_min_timestep(transition_time).to_bits(),
            (transition_time * 10.0 * Value::EPSILON).to_bits()
        );
        assert_eq!(
            xyce_hard_min_timestep(-transition_time).to_bits(),
            (transition_time * 10.0 * Value::EPSILON).to_bits()
        );
        assert_eq!(xyce_hard_min_timestep(Value::NAN), 0.0);
        assert_eq!(xyce_hard_min_timestep(Value::INFINITY), 0.0);
        assert_eq!(xyce_hard_min_timestep(Value::NEG_INFINITY), 0.0);
    }

    #[test]
    fn xyce_hard_min_timestep_avoids_intermediate_overflow() {
        let minimum = xyce_hard_min_timestep(Value::MAX);

        assert!(minimum.is_finite());
        assert!(minimum > 0.0);
        assert_eq!(minimum, Value::MAX * (10.0 * Value::EPSILON));
    }
}

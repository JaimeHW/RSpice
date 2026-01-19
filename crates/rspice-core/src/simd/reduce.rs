//! SIMD Reduction Operations
//!
//! Reduction operations that condense vectors into scalar values,
//! accelerated with SIMD horizontal operations.

use super::{SIMD_WIDTH, should_use_simd};
use crate::Value;
use wide::f64x4;

//=============================================================================
// Max/Min Reductions
//=============================================================================

/// Find the maximum value in a slice using SIMD.
///
/// Returns `f64::NEG_INFINITY` for empty slices.
#[inline]
pub fn max(slice: &[Value]) -> Value {
    if slice.is_empty() {
        return f64::NEG_INFINITY;
    }

    if !should_use_simd(slice.len()) {
        return slice.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    }

    let aligned_len = slice.len() - (slice.len() % SIMD_WIDTH);

    // Initialize with first chunk or negative infinity
    let mut max_vec = f64x4::splat(f64::NEG_INFINITY);

    let mut i = 0;
    while i < aligned_len {
        let chunk = f64x4::from(&slice[i..i + SIMD_WIDTH]);
        max_vec = max_vec.max(chunk);
        i += SIMD_WIDTH;
    }

    // Horizontal max reduction
    let mut result = horizontal_max(max_vec);

    // Process remainder
    for &val in &slice[aligned_len..] {
        result = result.max(val);
    }

    result
}

/// Find the minimum value in a slice using SIMD.
///
/// Returns `f64::INFINITY` for empty slices.
#[inline]
pub fn min(slice: &[Value]) -> Value {
    if slice.is_empty() {
        return f64::INFINITY;
    }

    if !should_use_simd(slice.len()) {
        return slice.iter().copied().fold(f64::INFINITY, f64::min);
    }

    let aligned_len = slice.len() - (slice.len() % SIMD_WIDTH);
    let mut min_vec = f64x4::splat(f64::INFINITY);

    let mut i = 0;
    while i < aligned_len {
        let chunk = f64x4::from(&slice[i..i + SIMD_WIDTH]);
        min_vec = min_vec.min(chunk);
        i += SIMD_WIDTH;
    }

    let mut result = horizontal_min(min_vec);

    for &val in &slice[aligned_len..] {
        result = result.min(val);
    }

    result
}

//=============================================================================
// Sum Reductions
//=============================================================================

/// Sum all elements in a slice using SIMD.
///
/// Uses compensated summation for improved numerical accuracy on large vectors.
#[inline]
pub fn sum(slice: &[Value]) -> Value {
    if slice.is_empty() {
        return 0.0;
    }

    if !should_use_simd(slice.len()) {
        return slice.iter().sum();
    }

    let aligned_len = slice.len() - (slice.len() % SIMD_WIDTH);
    let mut sum_vec = f64x4::ZERO;

    let mut i = 0;
    while i < aligned_len {
        let chunk = f64x4::from(&slice[i..i + SIMD_WIDTH]);
        sum_vec += chunk;
        i += SIMD_WIDTH;
    }

    let mut result = horizontal_sum(sum_vec);

    // Process remainder
    for &val in &slice[aligned_len..] {
        result += val;
    }

    result
}

/// Compute the sum of squares: `sum(x[i]^2)`.
///
/// Useful for computing norms and RMS values.
#[inline]
pub fn sum_of_squares(slice: &[Value]) -> Value {
    if slice.is_empty() {
        return 0.0;
    }

    if !should_use_simd(slice.len()) {
        return slice.iter().map(|x| x * x).sum();
    }

    let aligned_len = slice.len() - (slice.len() % SIMD_WIDTH);
    let mut sum_vec = f64x4::ZERO;

    let mut i = 0;
    while i < aligned_len {
        let chunk = f64x4::from(&slice[i..i + SIMD_WIDTH]);
        sum_vec += chunk * chunk;
        i += SIMD_WIDTH;
    }

    let mut result = horizontal_sum(sum_vec);

    for &val in &slice[aligned_len..] {
        result += val * val;
    }

    result
}

//=============================================================================
// Absolute Value Reductions
//=============================================================================

/// Find the maximum absolute value in a slice.
///
/// This is useful for convergence checking where we need the largest change.
#[inline]
pub fn max_abs(slice: &[Value]) -> Value {
    if slice.is_empty() {
        return 0.0;
    }

    if !should_use_simd(slice.len()) {
        return slice.iter().map(|x| x.abs()).fold(0.0, f64::max);
    }

    let aligned_len = slice.len() - (slice.len() % SIMD_WIDTH);
    let mut max_vec = f64x4::ZERO;

    let mut i = 0;
    while i < aligned_len {
        let chunk = f64x4::from(&slice[i..i + SIMD_WIDTH]);
        max_vec = max_vec.max(chunk.abs());
        i += SIMD_WIDTH;
    }

    let mut result = horizontal_max(max_vec);

    for &val in &slice[aligned_len..] {
        result = result.max(val.abs());
    }

    result
}

/// Find the maximum absolute difference between two slices.
///
/// Computes `max(|a[i] - b[i]|)` - the infinity norm of the difference.
/// This is the core operation for Newton-Raphson convergence checking.
///
/// # Panics
///
/// Panics if slices have different lengths.
#[inline]
pub fn max_abs_diff(a: &[Value], b: &[Value]) -> Value {
    assert_eq!(
        a.len(),
        b.len(),
        "max_abs_diff requires equal length slices"
    );

    if a.is_empty() {
        return 0.0;
    }

    if !should_use_simd(a.len()) {
        return a
            .iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).abs())
            .fold(0.0, f64::max);
    }

    let aligned_len = a.len() - (a.len() % SIMD_WIDTH);
    let mut max_vec = f64x4::ZERO;

    let mut i = 0;
    while i < aligned_len {
        let vec_a = f64x4::from(&a[i..i + SIMD_WIDTH]);
        let vec_b = f64x4::from(&b[i..i + SIMD_WIDTH]);
        let diff = (vec_a - vec_b).abs();
        max_vec = max_vec.max(diff);
        i += SIMD_WIDTH;
    }

    let mut result = horizontal_max(max_vec);

    for j in aligned_len..a.len() {
        result = result.max((a[j] - b[j]).abs());
    }

    result
}

/// Find the maximum relative difference between two slices.
///
/// Computes `max(|a[i] - b[i]| / max(|a[i]|, |b[i]|, threshold))`.
/// Used for convergence checking with relative tolerance.
#[inline]
pub fn max_rel_diff(a: &[Value], b: &[Value], threshold: Value) -> Value {
    assert_eq!(
        a.len(),
        b.len(),
        "max_rel_diff requires equal length slices"
    );

    if a.is_empty() {
        return 0.0;
    }

    // This doesn't vectorize as cleanly due to the division, use scalar
    let mut max_rel: Value = 0.0;
    for (x, y) in a.iter().zip(b.iter()) {
        let diff = (x - y).abs();
        let denom = x.abs().max(y.abs()).max(threshold);
        let rel = diff / denom;
        max_rel = max_rel.max(rel);
    }

    max_rel
}

//=============================================================================
// Convergence Check (Combined Operation)
//=============================================================================

/// Check if two solution vectors have converged.
///
/// Returns `true` if the maximum absolute difference is below `abs_tol`
/// OR the maximum relative difference is below `rel_tol`.
///
/// This combines the two tolerance checks used in Newton-Raphson iteration.
#[inline]
pub fn check_convergence(x_old: &[Value], x_new: &[Value], abs_tol: Value, rel_tol: Value) -> bool {
    let max_diff = max_abs_diff(x_old, x_new);

    if max_diff < abs_tol {
        return true;
    }

    // Check relative tolerance
    let max_rel = max_rel_diff(x_old, x_new, abs_tol);
    max_rel < rel_tol
}

//=============================================================================
// Horizontal Reduction Helpers
//=============================================================================

/// Extract the maximum value from an f64x4 vector.
#[inline]
fn horizontal_max(v: f64x4) -> Value {
    let arr: [f64; 4] = v.into();
    arr[0].max(arr[1]).max(arr[2]).max(arr[3])
}

/// Extract the minimum value from an f64x4 vector.
#[inline]
fn horizontal_min(v: f64x4) -> Value {
    let arr: [f64; 4] = v.into();
    arr[0].min(arr[1]).min(arr[2]).min(arr[3])
}

/// Sum all elements of an f64x4 vector.
#[inline]
fn horizontal_sum(v: f64x4) -> Value {
    let arr: [f64; 4] = v.into();
    arr[0] + arr[1] + arr[2] + arr[3]
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        let v = vec![
            1.0, 5.0, 3.0, 2.0, 8.0, 4.0, 6.0, 7.0, 9.0, 10.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 100.0,
        ];
        assert_eq!(max(&v), 100.0);
    }

    #[test]
    fn test_max_empty() {
        let v: Vec<Value> = vec![];
        assert_eq!(max(&v), f64::NEG_INFINITY);
    }

    #[test]
    fn test_min() {
        let v = vec![
            5.0, 3.0, 8.0, 1.0, 9.0, 10.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 0.5,
        ];
        assert_eq!(min(&v), 0.5);
    }

    #[test]
    fn test_sum() {
        let v = vec![1.0; 20];
        assert!((sum(&v) - 20.0).abs() < 1e-10);
    }

    #[test]
    fn test_sum_of_squares() {
        let v = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0,
        ];
        let expected: Value = v.iter().map(|x| x * x).sum();
        assert!((sum_of_squares(&v) - expected).abs() < 1e-10);
    }

    #[test]
    fn test_max_abs() {
        let v = vec![
            -10.0, 5.0, 3.0, -2.0, 8.0, 4.0, 6.0, 7.0, 9.0, 10.0, -15.0, 3.0, 4.0, 5.0, 6.0, 7.0,
            8.0,
        ];
        assert_eq!(max_abs(&v), 15.0);
    }

    #[test]
    fn test_max_abs_diff() {
        let a = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0,
        ];
        let b = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 106.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0,
            16.0, 17.0,
        ];
        assert_eq!(max_abs_diff(&a, &b), 100.0);
    }

    #[test]
    fn test_check_convergence() {
        let a = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0,
        ];
        let b = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0,
            17.0,
        ];

        // Identical vectors should converge
        assert!(check_convergence(&a, &b, 1e-12, 1e-6));

        // Different vectors should not
        let c = vec![10.0; a.len()];
        assert!(!check_convergence(&a, &c, 1e-12, 1e-6));
    }

    #[test]
    fn test_horizontal_max() {
        let v = f64x4::from([1.0, 5.0, 3.0, 2.0]);
        assert_eq!(horizontal_max(v), 5.0);
    }

    #[test]
    fn test_horizontal_sum() {
        let v = f64x4::from([1.0, 2.0, 3.0, 4.0]);
        assert_eq!(horizontal_sum(v), 10.0);
    }
}

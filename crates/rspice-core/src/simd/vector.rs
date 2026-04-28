//! SIMD Vector Operations
//!
//! Basic vector operations accelerated with SIMD instructions.
//! All operations handle arbitrary-length slices by processing
//! SIMD-width chunks followed by scalar remainder.

use super::{SIMD_WIDTH, should_use_simd};
use crate::Value;
use wide::f64x4;

//=============================================================================
// Helper: Store f64x4 to slice
//=============================================================================

/// Store an f64x4 vector to a mutable slice.
/// The `wide` crate doesn't have a direct store method, so we convert to array.
#[inline]
fn store_f64x4(v: f64x4, dst: &mut [Value]) {
    let arr: [f64; 4] = v.into();
    dst[0] = arr[0];
    dst[1] = arr[1];
    dst[2] = arr[2];
    dst[3] = arr[3];
}

//=============================================================================
// Fill Operations
//=============================================================================

/// Fill a slice with zeros using SIMD.
///
/// This is a drop-in replacement for `slice.fill(0.0)` that guarantees
/// vectorization for slices above the SIMD threshold.
///
/// # Performance
///
/// - O(n/4) SIMD operations for the bulk
/// - O(n%4) scalar operations for remainder
/// - ~2-4x faster than scalar for large slices
#[inline]
pub fn fill_zero(slice: &mut [Value]) {
    if !should_use_simd(slice.len()) {
        slice.fill(0.0);
        return;
    }

    let zero = f64x4::ZERO;
    let aligned_len = slice.len() - (slice.len() % SIMD_WIDTH);

    // Process SIMD chunks
    let mut i = 0;
    while i < aligned_len {
        store_f64x4(zero, &mut slice[i..i + SIMD_WIDTH]);
        i += SIMD_WIDTH;
    }

    // Process remainder
    for val in &mut slice[aligned_len..] {
        *val = 0.0;
    }
}

/// Fill a slice with a constant value using SIMD.
///
/// # Arguments
///
/// * `slice` - The slice to fill
/// * `value` - The value to fill with
#[inline]
pub fn fill_value(slice: &mut [Value], value: Value) {
    if !should_use_simd(slice.len()) {
        slice.fill(value);
        return;
    }

    let vec_value = f64x4::splat(value);
    let aligned_len = slice.len() - (slice.len() % SIMD_WIDTH);

    let mut i = 0;
    while i < aligned_len {
        store_f64x4(vec_value, &mut slice[i..i + SIMD_WIDTH]);
        i += SIMD_WIDTH;
    }

    for val in &mut slice[aligned_len..] {
        *val = value;
    }
}

//=============================================================================
// Copy Operations
//=============================================================================

/// Copy from source to destination slice using SIMD.
///
/// # Panics
///
/// Panics if `src.len() != dst.len()`.
#[inline]
pub fn copy(dst: &mut [Value], src: &[Value]) {
    assert_eq!(
        dst.len(),
        src.len(),
        "SIMD copy requires equal length slices"
    );

    if !should_use_simd(src.len()) {
        dst.copy_from_slice(src);
        return;
    }

    let aligned_len = src.len() - (src.len() % SIMD_WIDTH);

    let mut i = 0;
    while i < aligned_len {
        let v = f64x4::from(&src[i..i + SIMD_WIDTH]);
        store_f64x4(v, &mut dst[i..i + SIMD_WIDTH]);
        i += SIMD_WIDTH;
    }

    // Remainder
    dst[aligned_len..].copy_from_slice(&src[aligned_len..]);
}

//=============================================================================
// Scale Operations
//=============================================================================

/// Scale all elements in a slice by a constant factor.
///
/// Computes `slice[i] *= scale` for all elements.
#[inline]
pub fn scale(slice: &mut [Value], scale: Value) {
    if !should_use_simd(slice.len()) {
        for v in slice.iter_mut() {
            *v *= scale;
        }
        return;
    }

    let scale_vec = f64x4::splat(scale);
    let aligned_len = slice.len() - (slice.len() % SIMD_WIDTH);

    let mut i = 0;
    while i < aligned_len {
        let chunk = &mut slice[i..i + SIMD_WIDTH];
        let v = f64x4::from(&*chunk) * scale_vec;
        store_f64x4(v, chunk);
        i += SIMD_WIDTH;
    }

    for val in &mut slice[aligned_len..] {
        *val *= scale;
    }
}

/// Add a scaled source vector to destination: `dst[i] += src[i] * scale`.
///
/// This is the classic AXPY operation (Y = aX + Y) commonly used
/// in linear algebra and circuit simulation.
#[inline]
pub fn axpy(dst: &mut [Value], src: &[Value], scale: Value) {
    assert_eq!(
        dst.len(),
        src.len(),
        "SIMD axpy requires equal length slices"
    );

    if !should_use_simd(src.len()) {
        for (d, s) in dst.iter_mut().zip(src.iter()) {
            *d += *s * scale;
        }
        return;
    }

    let scale_vec = f64x4::splat(scale);
    let aligned_len = src.len() - (src.len() % SIMD_WIDTH);

    let mut i = 0;
    while i < aligned_len {
        let src_chunk = &src[i..i + SIMD_WIDTH];
        let dst_chunk = &mut dst[i..i + SIMD_WIDTH];

        let s = f64x4::from(src_chunk);
        let d = f64x4::from(&*dst_chunk);
        let result = d + s * scale_vec;
        store_f64x4(result, dst_chunk);
        i += SIMD_WIDTH;
    }

    for j in aligned_len..src.len() {
        dst[j] += src[j] * scale;
    }
}

/// Element-wise addition: `dst[i] += src[i]`.
#[inline]
pub fn add(dst: &mut [Value], src: &[Value]) {
    assert_eq!(
        dst.len(),
        src.len(),
        "SIMD add requires equal length slices"
    );

    if !should_use_simd(src.len()) {
        for (d, s) in dst.iter_mut().zip(src.iter()) {
            *d += *s;
        }
        return;
    }

    let aligned_len = src.len() - (src.len() % SIMD_WIDTH);

    let mut i = 0;
    while i < aligned_len {
        let src_chunk = &src[i..i + SIMD_WIDTH];
        let dst_chunk = &mut dst[i..i + SIMD_WIDTH];

        let s = f64x4::from(src_chunk);
        let d = f64x4::from(&*dst_chunk);
        store_f64x4(d + s, dst_chunk);
        i += SIMD_WIDTH;
    }

    for j in aligned_len..src.len() {
        dst[j] += src[j];
    }
}

/// Element-wise subtraction: `dst[i] -= src[i]`.
#[inline]
pub fn sub(dst: &mut [Value], src: &[Value]) {
    assert_eq!(
        dst.len(),
        src.len(),
        "SIMD sub requires equal length slices"
    );

    if !should_use_simd(src.len()) {
        for (d, s) in dst.iter_mut().zip(src.iter()) {
            *d -= *s;
        }
        return;
    }

    let aligned_len = src.len() - (src.len() % SIMD_WIDTH);

    let mut i = 0;
    while i < aligned_len {
        let src_chunk = &src[i..i + SIMD_WIDTH];
        let dst_chunk = &mut dst[i..i + SIMD_WIDTH];

        let s = f64x4::from(src_chunk);
        let d = f64x4::from(&*dst_chunk);
        store_f64x4(d - s, dst_chunk);
        i += SIMD_WIDTH;
    }

    for j in aligned_len..src.len() {
        dst[j] -= src[j];
    }
}

/// Compute element-wise difference into a new vector: `out[i] = a[i] - b[i]`.
#[inline]
pub fn diff(a: &[Value], b: &[Value]) -> Vec<Value> {
    assert_eq!(a.len(), b.len(), "SIMD diff requires equal length slices");

    let mut result = vec![0.0; a.len()];

    if !should_use_simd(a.len()) {
        for i in 0..a.len() {
            result[i] = a[i] - b[i];
        }
        return result;
    }

    let aligned_len = a.len() - (a.len() % SIMD_WIDTH);

    let mut i = 0;
    while i < aligned_len {
        let vec_a = f64x4::from(&a[i..i + SIMD_WIDTH]);
        let vec_b = f64x4::from(&b[i..i + SIMD_WIDTH]);
        store_f64x4(vec_a - vec_b, &mut result[i..i + SIMD_WIDTH]);
        i += SIMD_WIDTH;
    }

    for j in aligned_len..a.len() {
        result[j] = a[j] - b[j];
    }

    result
}

//=============================================================================
// Tests
//=============================================================================


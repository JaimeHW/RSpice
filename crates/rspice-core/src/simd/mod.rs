//! SIMD-Accelerated Operations for High-Performance Circuit Simulation
//!
//! This module provides portable SIMD implementations of performance-critical
//! operations used throughout the simulation engine. All operations are designed
//! to work seamlessly as drop-in replacements for scalar code.
//!
//! # Architecture
//!
//! The module is organized into submodules by operation category:
//! - `vector`: Basic vector operations (fill, copy, scale)
//! - `reduce`: Reduction operations (max, sum, convergence checks)
//! - `integrate`: Numerical integration helpers
//!
//! # Platform Support
//!
//! Uses the `wide` crate for portable SIMD:
//! - x86/x86_64: SSE2/SSE4.1/AVX/AVX2 (auto-detected at runtime)
//! - ARM/AArch64: NEON
//! - Fallback: Scalar operations on unsupported platforms
//!
//! # Usage
//!
//! All functions in this module are designed to handle arbitrary-length slices,
//! automatically processing remainder elements after the SIMD-width chunks.
//!
//! ```ignore
//! use rspice_core::simd;
//!
//! let mut vec = vec![1.0; 1000];
//! simd::fill_zero(&mut vec);
//! ```

mod integrate;
mod math;
mod reduce;
mod vector;

// Re-export all public functions at module level for ergonomic access
pub use integrate::*;
pub use math::*;
pub use reduce::*;
pub use vector::*;

use crate::Value;

/// SIMD lane width for f64 operations.
///
/// We use 4-wide vectors (256-bit) which map to:
/// - AVX/AVX2 on x86_64 (single instruction)
/// - 2x NEON operations on ARM (still faster than scalar)
/// - SSE2 (2x operations, still vectorized)
pub const SIMD_WIDTH: usize = 4;

/// Minimum slice length to benefit from SIMD.
///
/// Below this threshold, the setup overhead exceeds the SIMD benefit.
/// Determined empirically - adjust based on profiling if needed.
pub const SIMD_THRESHOLD: usize = 16;

/// Check if a slice is large enough to benefit from SIMD acceleration.
#[inline]
pub fn should_use_simd(len: usize) -> bool {
    len >= SIMD_THRESHOLD
}

/// Align a slice length down to SIMD width boundary.
#[inline]
pub const fn align_down(len: usize) -> usize {
    len - (len % SIMD_WIDTH)
}

//=============================================================================
// Inline helper for common patterns
//=============================================================================

/// Process a slice in SIMD-width chunks, with a closure for each chunk.
///
/// This is internally used by the SIMD operations and optimizes the
/// common pattern of processing 4 elements at a time.
#[inline]
pub fn process_chunks<F>(slice: &[Value], mut f: F)
where
    F: FnMut(&[Value; SIMD_WIDTH]),
{
    let chunks = slice.len() / SIMD_WIDTH;
    for i in 0..chunks {
        let start = i * SIMD_WIDTH;
        // SAFETY: We've verified the slice is long enough
        let chunk: &[Value; SIMD_WIDTH] = slice[start..start + SIMD_WIDTH]
            .try_into()
            .expect("chunk size mismatch");
        f(chunk);
    }
}

/// Process a mutable slice in SIMD-width chunks.
#[inline]
pub fn process_chunks_mut<F>(slice: &mut [Value], mut f: F)
where
    F: FnMut(&mut [Value; SIMD_WIDTH]),
{
    let chunks = slice.len() / SIMD_WIDTH;
    for i in 0..chunks {
        let start = i * SIMD_WIDTH;
        // SAFETY: We've verified the slice is long enough
        let chunk: &mut [Value; SIMD_WIDTH] = (&mut slice[start..start + SIMD_WIDTH])
            .try_into()
            .expect("chunk size mismatch");
        f(chunk);
    }
}


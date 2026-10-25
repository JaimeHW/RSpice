//! Packed derivative lanes, the arithmetic type of generated model bodies.

use super::arithmetic;

/// Packed partial derivatives used by precompiled Verilog-A model bodies.
///
/// This lives in the shared leaf dependency so every generated device does not
/// have to parse its own copy. Widths emitted by the shipped catalog use the
/// loop-free fixed types below. This const-generic form is retained only as a
/// correctness fallback for future widths above the fixed ceiling.
#[doc(hidden)]
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct Lanes<const N: usize>(pub [f64; N]);

impl<const N: usize> core::ops::Add for Lanes<N> {
    type Output = Self;

    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] + rhs.0[i];
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Sub for Lanes<N> {
    type Output = Self;

    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] - rhs.0[i];
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Mul<f64> for Lanes<N> {
    type Output = Self;

    #[inline(always)]
    fn mul(self, rhs: f64) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] * rhs;
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Div<f64> for Lanes<N> {
    type Output = Self;

    #[inline(always)]
    fn div(self, rhs: f64) -> Self {
        let mut out = self.0;
        let mut i = 0;
        while i < N {
            out[i] = self.0[i] / rhs;
            i += 1;
        }
        Self(out)
    }
}

impl<const N: usize> core::ops::Index<usize> for Lanes<N> {
    type Output = f64;

    #[inline(always)]
    fn index(&self, index: usize) -> &f64 {
        &self.0[index]
    }
}

/// Defines one fixed-width derivative value with no source or IR loop in its
/// arithmetic implementations. LLVM can inline these scalar expressions
/// directly without rediscovering and rotating thousands of tiny loops in a
/// wide generated stamp.
macro_rules! define_fixed_lanes {
    ($name:ident, $width:literal, [$($index:tt),+ $(,)?]) => {
        #[doc(hidden)]
        #[repr(transparent)]
        #[derive(Clone, Copy)]
        pub struct $name(pub [f64; $width]);

        impl $name {
            /// Lane-wise form of [`arithmetic::product_div`].
            #[inline]
            pub fn product_div(self, scalar: f64, divisor: f64) -> Self {
                Self([$(arithmetic::product_div(self.0[$index], scalar, divisor)),+])
            }

            /// Lane-wise form of [`arithmetic::product_sum_div`].
            #[inline]
            pub fn product_sum_div(
                self,
                scalar: f64,
                right: Self,
                right_scalar: f64,
                divisor: f64,
            ) -> Self {
                Self([$(arithmetic::product_sum_div(
                    self.0[$index], scalar, right.0[$index], right_scalar, divisor
                )),+])
            }
        }

        impl core::ops::Add for $name {
            type Output = Self;

            #[inline(always)]
            fn add(self, rhs: Self) -> Self {
                Self([$((self.0[$index] + rhs.0[$index])),+])
            }
        }

        impl core::ops::Sub for $name {
            type Output = Self;

            #[inline(always)]
            fn sub(self, rhs: Self) -> Self {
                Self([$((self.0[$index] - rhs.0[$index])),+])
            }
        }

        impl core::ops::Mul<f64> for $name {
            type Output = Self;

            #[inline(always)]
            fn mul(self, rhs: f64) -> Self {
                Self([$((self.0[$index] * rhs)),+])
            }
        }

        impl core::ops::Div<f64> for $name {
            type Output = Self;

            #[inline(always)]
            fn div(self, rhs: f64) -> Self {
                Self([$((self.0[$index] / rhs)),+])
            }
        }

        impl core::ops::Index<usize> for $name {
            type Output = f64;

            #[inline(always)]
            fn index(&self, index: usize) -> &f64 {
                &self.0[index]
            }
        }
    };
}

define_fixed_lanes!(L2, 2, [0, 1]);
define_fixed_lanes!(L3, 3, [0, 1, 2]);
define_fixed_lanes!(L4, 4, [0, 1, 2, 3]);
define_fixed_lanes!(L5, 5, [0, 1, 2, 3, 4]);
define_fixed_lanes!(L6, 6, [0, 1, 2, 3, 4, 5]);
define_fixed_lanes!(L7, 7, [0, 1, 2, 3, 4, 5, 6]);
define_fixed_lanes!(L8, 8, [0, 1, 2, 3, 4, 5, 6, 7]);
define_fixed_lanes!(L9, 9, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
define_fixed_lanes!(L10, 10, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
define_fixed_lanes!(L11, 11, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
define_fixed_lanes!(L12, 12, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
define_fixed_lanes!(L13, 13, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
define_fixed_lanes!(L14, 14, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
define_fixed_lanes!(L15, 15, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
define_fixed_lanes!(
    L16,
    16,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
);
define_fixed_lanes!(
    L17,
    17,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]
);
define_fixed_lanes!(
    L18,
    18,
    [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]
);
define_fixed_lanes!(
    L19,
    19,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18
    ]
);
define_fixed_lanes!(
    L20,
    20,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19
    ]
);
define_fixed_lanes!(
    L21,
    21,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
    ]
);
define_fixed_lanes!(
    L22,
    22,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21
    ]
);
define_fixed_lanes!(
    L23,
    23,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22
    ]
);
define_fixed_lanes!(
    L24,
    24,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23
    ]
);
define_fixed_lanes!(
    L25,
    25,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24
    ]
);
define_fixed_lanes!(
    L26,
    26,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25
    ]
);
define_fixed_lanes!(
    L27,
    27,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26
    ]
);
define_fixed_lanes!(
    L28,
    28,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27
    ]
);
define_fixed_lanes!(
    L29,
    29,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28
    ]
);
define_fixed_lanes!(
    L30,
    30,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29
    ]
);
define_fixed_lanes!(
    L31,
    31,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30
    ]
);
define_fixed_lanes!(
    L32,
    32,
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31
    ]
);

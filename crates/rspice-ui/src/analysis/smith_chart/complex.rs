//! Complex Number Arithmetic
//!
//! High-precision complex number operations for RF/microwave calculations.
//! Designed to match the precision requirements of commercial tools.

use std::f64::consts::PI;

// =============================================================================
// Complex Number Type
// =============================================================================

/// Complex number with real and imaginary parts
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Complex {
    /// Real part
    pub re: f64,
    /// Imaginary part
    pub im: f64,
}

impl Complex {
    /// Create a new complex number
    pub const fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

    /// Create from polar coordinates
    pub fn from_polar(magnitude: f64, angle_rad: f64) -> Self {
        Self {
            re: magnitude * angle_rad.cos(),
            im: magnitude * angle_rad.sin(),
        }
    }

    /// Create pure real number
    pub const fn real(re: f64) -> Self {
        Self { re, im: 0.0 }
    }

    /// Create pure imaginary number
    pub const fn imag(im: f64) -> Self {
        Self { re: 0.0, im }
    }

    /// Zero complex number
    pub const ZERO: Self = Self { re: 0.0, im: 0.0 };

    /// One (real)
    pub const ONE: Self = Self { re: 1.0, im: 0.0 };

    /// Imaginary unit (i)
    pub const I: Self = Self { re: 0.0, im: 1.0 };

    // =========================================================================
    // Properties
    // =========================================================================

    /// Magnitude (absolute value)
    pub fn magnitude(&self) -> f64 {
        (self.re * self.re + self.im * self.im).sqrt()
    }

    /// Magnitude squared (avoids sqrt for comparisons)
    pub fn magnitude_squared(&self) -> f64 {
        self.re * self.re + self.im * self.im
    }

    /// Phase angle in radians (-π to π)
    pub fn phase(&self) -> f64 {
        self.im.atan2(self.re)
    }

    /// Phase angle in degrees (-180 to 180)
    pub fn phase_deg(&self) -> f64 {
        self.phase() * 180.0 / PI
    }

    /// Complex conjugate
    pub fn conjugate(&self) -> Self {
        Self {
            re: self.re,
            im: -self.im,
        }
    }

    /// Check if approximately zero
    pub fn is_zero(&self, epsilon: f64) -> bool {
        self.magnitude_squared() < epsilon * epsilon
    }

    /// Check if finite (not NaN or Inf)
    pub fn is_finite(&self) -> bool {
        self.re.is_finite() && self.im.is_finite()
    }

    // =========================================================================
    // Arithmetic Operations
    // =========================================================================

    /// Add two complex numbers
    pub fn add(&self, other: &Complex) -> Complex {
        Complex {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }

    /// Subtract two complex numbers
    pub fn sub(&self, other: &Complex) -> Complex {
        Complex {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }

    /// Multiply two complex numbers
    pub fn mul(&self, other: &Complex) -> Complex {
        Complex {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }

    /// Divide by another complex number
    pub fn div(&self, other: &Complex) -> Option<Complex> {
        let denom = other.magnitude_squared();
        if denom < 1e-30 {
            return None;
        }
        Some(Complex {
            re: (self.re * other.re + self.im * other.im) / denom,
            im: (self.im * other.re - self.re * other.im) / denom,
        })
    }

    /// Scale by real number
    pub fn scale(&self, s: f64) -> Complex {
        Complex {
            re: self.re * s,
            im: self.im * s,
        }
    }

    /// Negate
    pub fn neg(&self) -> Complex {
        Complex {
            re: -self.re,
            im: -self.im,
        }
    }

    /// Reciprocal (1/z)
    pub fn reciprocal(&self) -> Option<Complex> {
        Complex::ONE.div(self)
    }

    // =========================================================================
    // Advanced Operations
    // =========================================================================

    /// Square root (principal value)
    pub fn sqrt(&self) -> Complex {
        let r = self.magnitude();
        let theta = self.phase();
        Complex::from_polar(r.sqrt(), theta / 2.0)
    }

    /// Natural exponential e^z
    pub fn exp(&self) -> Complex {
        let exp_re = self.re.exp();
        Complex {
            re: exp_re * self.im.cos(),
            im: exp_re * self.im.sin(),
        }
    }

    /// Natural logarithm (principal value)
    pub fn ln(&self) -> Complex {
        Complex {
            re: self.magnitude().ln(),
            im: self.phase(),
        }
    }

    /// Power z^n (integer)
    pub fn powi(&self, n: i32) -> Complex {
        let r = self.magnitude().powi(n);
        let theta = self.phase() * n as f64;
        Complex::from_polar(r, theta)
    }

    /// Power z^w (complex)
    pub fn powc(&self, w: &Complex) -> Complex {
        // z^w = e^(w * ln(z))
        w.mul(&self.ln()).exp()
    }
}

// =============================================================================
// Operator Implementations
// =============================================================================

impl std::ops::Add for Complex {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Complex::add(&self, &rhs)
    }
}

impl std::ops::Sub for Complex {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Complex::sub(&self, &rhs)
    }
}

impl std::ops::Mul for Complex {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Complex::mul(&self, &rhs)
    }
}

impl std::ops::Neg for Complex {
    type Output = Self;
    fn neg(self) -> Self {
        Complex::neg(&self)
    }
}

impl std::ops::Mul<f64> for Complex {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        self.scale(rhs)
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.im >= 0.0 {
            write!(f, "{:.4}+j{:.4}", self.re, self.im)
        } else {
            write!(f, "{:.4}-j{:.4}", self.re, -self.im)
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

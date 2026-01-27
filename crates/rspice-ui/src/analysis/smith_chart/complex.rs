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

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    fn complex_approx_eq(a: &Complex, b: &Complex) -> bool {
        approx_eq(a.re, b.re) && approx_eq(a.im, b.im)
    }

    // =========================================================================
    // Construction Tests
    // =========================================================================

    #[test]
    fn test_new() {
        let c = Complex::new(3.0, 4.0);
        assert_eq!(c.re, 3.0);
        assert_eq!(c.im, 4.0);
    }

    #[test]
    fn test_from_polar() {
        let c = Complex::from_polar(5.0, 0.0);
        assert!(approx_eq(c.re, 5.0));
        assert!(approx_eq(c.im, 0.0));

        let c2 = Complex::from_polar(1.0, PI / 2.0);
        assert!(approx_eq(c2.re, 0.0));
        assert!(approx_eq(c2.im, 1.0));
    }

    #[test]
    fn test_constants() {
        assert_eq!(Complex::ZERO, Complex::new(0.0, 0.0));
        assert_eq!(Complex::ONE, Complex::new(1.0, 0.0));
        assert_eq!(Complex::I, Complex::new(0.0, 1.0));
    }

    // =========================================================================
    // Property Tests
    // =========================================================================

    #[test]
    fn test_magnitude() {
        let c = Complex::new(3.0, 4.0);
        assert!(approx_eq(c.magnitude(), 5.0));
    }

    #[test]
    fn test_magnitude_squared() {
        let c = Complex::new(3.0, 4.0);
        assert!(approx_eq(c.magnitude_squared(), 25.0));
    }

    #[test]
    fn test_phase() {
        let c = Complex::new(1.0, 1.0);
        assert!(approx_eq(c.phase(), PI / 4.0));
    }

    #[test]
    fn test_phase_deg() {
        let c = Complex::new(1.0, 1.0);
        assert!(approx_eq(c.phase_deg(), 45.0));
    }

    #[test]
    fn test_conjugate() {
        let c = Complex::new(3.0, 4.0);
        let conj = c.conjugate();
        assert_eq!(conj.re, 3.0);
        assert_eq!(conj.im, -4.0);
    }

    #[test]
    fn test_is_zero() {
        assert!(Complex::ZERO.is_zero(1e-10));
        assert!(!Complex::ONE.is_zero(1e-10));
    }

    #[test]
    fn test_is_finite() {
        assert!(Complex::new(1.0, 2.0).is_finite());
        assert!(!Complex::new(f64::NAN, 0.0).is_finite());
        assert!(!Complex::new(0.0, f64::INFINITY).is_finite());
    }

    // =========================================================================
    // Arithmetic Tests
    // =========================================================================

    #[test]
    fn test_add() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let c = a + b;
        assert_eq!(c.re, 4.0);
        assert_eq!(c.im, 6.0);
    }

    #[test]
    fn test_sub() {
        let a = Complex::new(5.0, 7.0);
        let b = Complex::new(2.0, 3.0);
        let c = a - b;
        assert_eq!(c.re, 3.0);
        assert_eq!(c.im, 4.0);
    }

    #[test]
    fn test_mul() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let c = a * b;
        // (1+2i)(3+4i) = 3 + 4i + 6i + 8i² = 3 + 10i - 8 = -5 + 10i
        assert!(approx_eq(c.re, -5.0));
        assert!(approx_eq(c.im, 10.0));
    }

    #[test]
    fn test_mul_i_squared() {
        let i = Complex::I;
        let i_sq = i * i;
        // i² = -1
        assert!(approx_eq(i_sq.re, -1.0));
        assert!(approx_eq(i_sq.im, 0.0));
    }

    #[test]
    fn test_div() {
        let a = Complex::new(1.0, 2.0);
        let b = Complex::new(3.0, 4.0);
        let c = a.div(&b).unwrap();
        // (1+2i)/(3+4i) = (1+2i)(3-4i)/25 = (3-4i+6i-8i²)/25 = (11+2i)/25
        assert!(approx_eq(c.re, 11.0 / 25.0));
        assert!(approx_eq(c.im, 2.0 / 25.0));
    }

    #[test]
    fn test_div_by_zero() {
        let a = Complex::new(1.0, 2.0);
        assert!(a.div(&Complex::ZERO).is_none());
    }

    #[test]
    fn test_scale() {
        let a = Complex::new(2.0, 3.0);
        let b = a.scale(2.0);
        assert_eq!(b.re, 4.0);
        assert_eq!(b.im, 6.0);
    }

    #[test]
    fn test_neg() {
        let a = Complex::new(2.0, 3.0);
        let b = -a;
        assert_eq!(b.re, -2.0);
        assert_eq!(b.im, -3.0);
    }

    #[test]
    fn test_reciprocal() {
        let a = Complex::new(1.0, 1.0);
        let r = a.reciprocal().unwrap();
        // 1/(1+i) = (1-i)/2
        assert!(approx_eq(r.re, 0.5));
        assert!(approx_eq(r.im, -0.5));
    }

    // =========================================================================
    // Advanced Operation Tests
    // =========================================================================

    #[test]
    fn test_sqrt() {
        let a = Complex::new(0.0, 2.0);
        let s = a.sqrt();
        // sqrt(2i) = 1+i
        assert!(approx_eq(s.re, 1.0));
        assert!(approx_eq(s.im, 1.0));
    }

    #[test]
    fn test_exp() {
        let a = Complex::new(0.0, PI);
        let e = a.exp();
        // e^(iπ) = -1
        assert!(approx_eq(e.re, -1.0));
        assert!(e.im.abs() < 1e-10);
    }

    #[test]
    fn test_ln() {
        let a = Complex::new(-1.0, 0.0);
        let l = a.ln();
        // ln(-1) = iπ
        assert!(approx_eq(l.re, 0.0));
        assert!(approx_eq(l.im, PI));
    }

    #[test]
    fn test_powi() {
        let a = Complex::new(1.0, 1.0);
        let a2 = a.powi(2);
        // (1+i)² = 2i
        assert!(approx_eq(a2.re, 0.0));
        assert!(approx_eq(a2.im, 2.0));
    }

    #[test]
    fn test_powi_negative() {
        let a = Complex::new(2.0, 0.0);
        let inv = a.powi(-1);
        assert!(approx_eq(inv.re, 0.5));
        assert!(approx_eq(inv.im, 0.0));
    }

    // =========================================================================
    // Display Test
    // =========================================================================

    #[test]
    fn test_display() {
        let a = Complex::new(1.5, 2.5);
        assert!(format!("{}", a).contains("+j"));

        let b = Complex::new(1.5, -2.5);
        assert!(format!("{}", b).contains("-j"));
    }

    // =========================================================================
    // Edge Cases
    // =========================================================================

    #[test]
    fn test_magnitude_of_real() {
        let a = Complex::real(5.0);
        assert!(approx_eq(a.magnitude(), 5.0));
    }

    #[test]
    fn test_magnitude_of_imaginary() {
        let a = Complex::imag(5.0);
        assert!(approx_eq(a.magnitude(), 5.0));
    }

    #[test]
    fn test_phase_of_positive_real() {
        let a = Complex::real(1.0);
        assert!(approx_eq(a.phase(), 0.0));
    }

    #[test]
    fn test_phase_of_negative_real() {
        let a = Complex::real(-1.0);
        assert!(approx_eq(a.phase().abs(), PI));
    }

    #[test]
    fn test_phase_of_positive_imag() {
        let a = Complex::imag(1.0);
        assert!(approx_eq(a.phase(), PI / 2.0));
    }

    #[test]
    fn test_roundtrip_polar() {
        let original = Complex::new(3.0, 4.0);
        let magnitude = original.magnitude();
        let phase = original.phase();
        let reconstructed = Complex::from_polar(magnitude, phase);
        assert!(complex_approx_eq(&original, &reconstructed));
    }
}

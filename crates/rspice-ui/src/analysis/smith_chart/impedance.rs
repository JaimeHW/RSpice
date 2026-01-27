//! Impedance and Admittance Types
//!
//! RF circuit analysis types for Smith chart calculations.
//! Supports normalized/denormalized impedance and reflection coefficient (Gamma).

use super::complex::Complex;
use std::f64::consts::PI;

// =============================================================================
// Constants
// =============================================================================

/// Default reference impedance (50Ω standard)
pub const Z0_DEFAULT: f64 = 50.0;

// =============================================================================
// Impedance Type
// =============================================================================

/// Complex impedance (Z = R + jX)
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Impedance {
    /// Resistance (real part) in ohms
    pub r: f64,
    /// Reactance (imaginary part) in ohms
    pub x: f64,
}

impl Impedance {
    /// Create new impedance
    pub fn new(r: f64, x: f64) -> Self {
        Self { r, x }
    }

    /// Create from complex number
    pub fn from_complex(c: Complex) -> Self {
        Self { r: c.re, x: c.im }
    }

    /// Convert to complex number
    pub fn to_complex(&self) -> Complex {
        Complex::new(self.r, self.x)
    }

    /// Create pure resistance
    pub fn resistance(r: f64) -> Self {
        Self { r, x: 0.0 }
    }

    /// Create pure reactance
    pub fn reactance(x: f64) -> Self {
        Self { r: 0.0, x }
    }

    /// Open circuit (infinite impedance, approximated)
    pub fn open() -> Self {
        Self { r: 1e12, x: 0.0 }
    }

    /// Short circuit (zero impedance)
    pub fn short() -> Self {
        Self { r: 0.0, x: 0.0 }
    }

    // =========================================================================
    // Normalization
    // =========================================================================

    /// Normalize to reference impedance
    pub fn normalize(&self, z0: f64) -> Self {
        Self {
            r: self.r / z0,
            x: self.x / z0,
        }
    }

    /// Denormalize from reference impedance
    pub fn denormalize(&self, z0: f64) -> Self {
        Self {
            r: self.r * z0,
            x: self.x * z0,
        }
    }

    // =========================================================================
    // Reflection Coefficient (Gamma)
    // =========================================================================

    /// Calculate reflection coefficient Γ = (Z - Z0) / (Z + Z0)
    pub fn to_gamma(&self, z0: f64) -> Complex {
        let z = self.to_complex();
        let z0_c = Complex::real(z0);

        let numerator = z.sub(&z0_c);
        let denominator = z.add(&z0_c);

        numerator.div(&denominator).unwrap_or(Complex::ZERO)
    }

    /// Create from reflection coefficient Γ: Z = Z0 * (1 + Γ) / (1 - Γ)
    pub fn from_gamma(gamma: Complex, z0: f64) -> Self {
        let one = Complex::ONE;
        let numerator = one.add(&gamma);
        let denominator = one.sub(&gamma);

        if let Some(ratio) = numerator.div(&denominator) {
            let z = ratio.scale(z0);
            Self::from_complex(z)
        } else {
            // Γ = 1 corresponds to open circuit
            Self::open()
        }
    }

    /// Calculate VSWR from reflection coefficient
    pub fn vswr(&self, z0: f64) -> f64 {
        let gamma_mag = self.to_gamma(z0).magnitude();
        if gamma_mag >= 1.0 {
            f64::INFINITY
        } else {
            (1.0 + gamma_mag) / (1.0 - gamma_mag)
        }
    }

    /// Calculate return loss in dB
    pub fn return_loss_db(&self, z0: f64) -> f64 {
        let gamma_mag = self.to_gamma(z0).magnitude();
        if gamma_mag < 1e-10 {
            f64::INFINITY // Perfect match
        } else {
            -20.0 * gamma_mag.log10()
        }
    }

    // =========================================================================
    // Conversions
    // =========================================================================

    /// Convert to admittance Y = 1/Z
    pub fn to_admittance(&self) -> Admittance {
        let z = self.to_complex();
        if let Some(y) = z.reciprocal() {
            Admittance::from_complex(y)
        } else {
            Admittance::open() // Short circuit Z -> Open circuit Y
        }
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Magnitude |Z|
    pub fn magnitude(&self) -> f64 {
        self.to_complex().magnitude()
    }

    /// Phase angle in degrees
    pub fn phase_deg(&self) -> f64 {
        self.to_complex().phase_deg()
    }

    /// Check if purely resistive
    pub fn is_resistive(&self, epsilon: f64) -> bool {
        self.x.abs() < epsilon
    }

    /// Check if purely reactive
    pub fn is_reactive(&self, epsilon: f64) -> bool {
        self.r.abs() < epsilon
    }

    /// Check if inductive (positive reactance)
    pub fn is_inductive(&self) -> bool {
        self.x > 0.0
    }

    /// Check if capacitive (negative reactance)
    pub fn is_capacitive(&self) -> bool {
        self.x < 0.0
    }

    // =========================================================================
    // Smith Chart Coordinates
    // =========================================================================

    /// Convert normalized impedance to Smith chart (x, y) coordinates
    ///
    /// The Smith chart uses the reflection coefficient plane where:
    /// - x coordinate = Re(Γ)
    /// - y coordinate = Im(Γ)
    /// - The chart is a unit circle
    pub fn to_smith_xy(&self, z0: f64) -> (f64, f64) {
        let gamma = self.to_gamma(z0);
        (gamma.re, gamma.im)
    }

    /// Create from Smith chart coordinates
    pub fn from_smith_xy(x: f64, y: f64, z0: f64) -> Self {
        let gamma = Complex::new(x, y);
        Self::from_gamma(gamma, z0)
    }
}

impl std::fmt::Display for Impedance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.x >= 0.0 {
            write!(f, "{:.2}+j{:.2}Ω", self.r, self.x)
        } else {
            write!(f, "{:.2}-j{:.2}Ω", self.r, -self.x)
        }
    }
}

// =============================================================================
// Admittance Type
// =============================================================================

/// Complex admittance (Y = G + jB)
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Admittance {
    /// Conductance (real part) in siemens
    pub g: f64,
    /// Susceptance (imaginary part) in siemens
    pub b: f64,
}

impl Admittance {
    /// Create new admittance
    pub fn new(g: f64, b: f64) -> Self {
        Self { g, b }
    }

    /// Create from complex number
    pub fn from_complex(c: Complex) -> Self {
        Self { g: c.re, b: c.im }
    }

    /// Convert to complex number
    pub fn to_complex(&self) -> Complex {
        Complex::new(self.g, self.b)
    }

    /// Create pure conductance
    pub fn conductance(g: f64) -> Self {
        Self { g, b: 0.0 }
    }

    /// Create pure susceptance
    pub fn susceptance(b: f64) -> Self {
        Self { g: 0.0, b }
    }

    /// Open circuit (zero admittance)
    pub fn open() -> Self {
        Self { g: 0.0, b: 0.0 }
    }

    /// Short circuit (infinite admittance, approximated)
    pub fn short() -> Self {
        Self { g: 1e12, b: 0.0 }
    }

    // =========================================================================
    // Normalization
    // =========================================================================

    /// Normalize to reference admittance Y0 = 1/Z0
    pub fn normalize(&self, z0: f64) -> Self {
        let y0 = 1.0 / z0;
        Self {
            g: self.g / y0,
            b: self.b / y0,
        }
    }

    /// Denormalize from reference admittance
    pub fn denormalize(&self, z0: f64) -> Self {
        let y0 = 1.0 / z0;
        Self {
            g: self.g * y0,
            b: self.b * y0,
        }
    }

    // =========================================================================
    // Conversions
    // =========================================================================

    /// Convert to impedance Z = 1/Y
    pub fn to_impedance(&self) -> Impedance {
        let y = self.to_complex();
        if let Some(z) = y.reciprocal() {
            Impedance::from_complex(z)
        } else {
            Impedance::open() // Open circuit Y -> Open circuit Z
        }
    }

    /// Calculate reflection coefficient for admittance
    /// Γ = (Y0 - Y) / (Y0 + Y)
    pub fn to_gamma(&self, z0: f64) -> Complex {
        let y0 = 1.0 / z0;
        let y = self.to_complex();
        let y0_c = Complex::real(y0);

        let numerator = y0_c.sub(&y);
        let denominator = y0_c.add(&y);

        numerator.div(&denominator).unwrap_or(Complex::ZERO)
    }

    /// Convert to Smith chart coordinates (for Y chart, same as Z but rotated 180°)
    pub fn to_smith_xy(&self, z0: f64) -> (f64, f64) {
        let gamma = self.to_gamma(z0);
        (gamma.re, gamma.im)
    }

    // =========================================================================
    // Properties
    // =========================================================================

    /// Magnitude |Y|
    pub fn magnitude(&self) -> f64 {
        self.to_complex().magnitude()
    }

    /// Phase angle in degrees
    pub fn phase_deg(&self) -> f64 {
        self.to_complex().phase_deg()
    }
}

impl std::fmt::Display for Admittance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.b >= 0.0 {
            write!(f, "{:.4}+j{:.4}S", self.g, self.b)
        } else {
            write!(f, "{:.4}-j{:.4}S", self.g, -self.b)
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
    const Z0: f64 = 50.0;

    fn approx_eq(a: f64, b: f64) -> bool {
        (a - b).abs() < EPSILON
    }

    // =========================================================================
    // Impedance Construction Tests
    // =========================================================================

    #[test]
    fn test_impedance_new() {
        let z = Impedance::new(100.0, 50.0);
        assert_eq!(z.r, 100.0);
        assert_eq!(z.x, 50.0);
    }

    #[test]
    fn test_impedance_from_complex() {
        let c = Complex::new(75.0, -25.0);
        let z = Impedance::from_complex(c);
        assert_eq!(z.r, 75.0);
        assert_eq!(z.x, -25.0);
    }

    #[test]
    fn test_impedance_special() {
        let open = Impedance::open();
        assert!(open.r > 1e10);

        let short = Impedance::short();
        assert_eq!(short.r, 0.0);
        assert_eq!(short.x, 0.0);
    }

    // =========================================================================
    // Normalization Tests
    // =========================================================================

    #[test]
    fn test_impedance_normalize() {
        let z = Impedance::new(100.0, 50.0);
        let zn = z.normalize(Z0);
        assert!(approx_eq(zn.r, 2.0));
        assert!(approx_eq(zn.x, 1.0));
    }

    #[test]
    fn test_impedance_denormalize() {
        let zn = Impedance::new(2.0, 1.0);
        let z = zn.denormalize(Z0);
        assert!(approx_eq(z.r, 100.0));
        assert!(approx_eq(z.x, 50.0));
    }

    #[test]
    fn test_normalize_denormalize_roundtrip() {
        let z = Impedance::new(75.0, -33.0);
        let zn = z.normalize(Z0);
        let z2 = zn.denormalize(Z0);
        assert!(approx_eq(z.r, z2.r));
        assert!(approx_eq(z.x, z2.x));
    }

    // =========================================================================
    // Reflection Coefficient Tests
    // =========================================================================

    #[test]
    fn test_impedance_matched_gamma() {
        // Z = Z0 should give Γ = 0
        let z = Impedance::new(50.0, 0.0);
        let gamma = z.to_gamma(Z0);
        assert!(approx_eq(gamma.re, 0.0));
        assert!(approx_eq(gamma.im, 0.0));
    }

    #[test]
    fn test_impedance_open_gamma() {
        // Open circuit should give Γ ≈ 1 (using approximation 1e12Ω)
        let z = Impedance::open();
        let gamma = z.to_gamma(Z0);
        // Use looser tolerance since open() is an approximation
        assert!((gamma.magnitude() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_impedance_short_gamma() {
        // Short circuit should give Γ = -1
        let z = Impedance::short();
        let gamma = z.to_gamma(Z0);
        assert!(approx_eq(gamma.re, -1.0));
        assert!(approx_eq(gamma.im, 0.0));
    }

    #[test]
    fn test_gamma_roundtrip() {
        let z_original = Impedance::new(75.0, 25.0);
        let gamma = z_original.to_gamma(Z0);
        let z_recovered = Impedance::from_gamma(gamma, Z0);
        assert!((z_original.r - z_recovered.r).abs() < 1e-6);
        assert!((z_original.x - z_recovered.x).abs() < 1e-6);
    }

    #[test]
    fn test_vswr_matched() {
        let z = Impedance::new(50.0, 0.0);
        assert!(approx_eq(z.vswr(Z0), 1.0));
    }

    #[test]
    fn test_vswr_2_to_1() {
        // 100Ω load on 50Ω line should give VSWR = 2
        let z = Impedance::new(100.0, 0.0);
        assert!((z.vswr(Z0) - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_return_loss_perfect_match() {
        let z = Impedance::new(50.0, 0.0);
        let rl = z.return_loss_db(Z0);
        assert!(rl > 100.0); // Very high return loss
    }

    // =========================================================================
    // Conversion Tests
    // =========================================================================

    #[test]
    fn test_impedance_to_admittance() {
        let z = Impedance::new(100.0, 0.0);
        let y = z.to_admittance();
        assert!(approx_eq(y.g, 0.01));
        assert!(approx_eq(y.b, 0.0));
    }

    #[test]
    fn test_impedance_admittance_roundtrip() {
        let z = Impedance::new(75.0, 33.0);
        let y = z.to_admittance();
        let z2 = y.to_impedance();
        assert!((z.r - z2.r).abs() < 1e-6);
        assert!((z.x - z2.x).abs() < 1e-6);
    }

    // =========================================================================
    // Smith Chart Coordinate Tests
    // =========================================================================

    #[test]
    fn test_smith_center() {
        // Z = Z0 is at center of Smith chart (0, 0)
        let z = Impedance::new(50.0, 0.0);
        let (x, y) = z.to_smith_xy(Z0);
        assert!(approx_eq(x, 0.0));
        assert!(approx_eq(y, 0.0));
    }

    #[test]
    fn test_smith_right_edge() {
        // Open circuit is at (1, 0)
        let z = Impedance::open();
        let (x, y) = z.to_smith_xy(Z0);
        assert!((x - 1.0).abs() < 0.01);
        assert!(approx_eq(y, 0.0));
    }

    #[test]
    fn test_smith_left_edge() {
        // Short circuit is at (-1, 0)
        let z = Impedance::short();
        let (x, y) = z.to_smith_xy(Z0);
        assert!(approx_eq(x, -1.0));
        assert!(approx_eq(y, 0.0));
    }

    #[test]
    fn test_smith_xy_roundtrip() {
        let z = Impedance::new(100.0, 75.0);
        let (x, y) = z.to_smith_xy(Z0);
        let z2 = Impedance::from_smith_xy(x, y, Z0);
        assert!((z.r - z2.r).abs() < 1e-6);
        assert!((z.x - z2.x).abs() < 1e-6);
    }

    // =========================================================================
    // Property Tests
    // =========================================================================

    #[test]
    fn test_is_inductive() {
        let z = Impedance::new(50.0, 100.0);
        assert!(z.is_inductive());
        assert!(!z.is_capacitive());
    }

    #[test]
    fn test_is_capacitive() {
        let z = Impedance::new(50.0, -100.0);
        assert!(z.is_capacitive());
        assert!(!z.is_inductive());
    }

    #[test]
    fn test_is_resistive() {
        let z = Impedance::resistance(100.0);
        assert!(z.is_resistive(1e-10));
    }

    // =========================================================================
    // Admittance Tests
    // =========================================================================

    #[test]
    fn test_admittance_new() {
        let y = Admittance::new(0.02, 0.01);
        assert_eq!(y.g, 0.02);
        assert_eq!(y.b, 0.01);
    }

    #[test]
    fn test_admittance_normalize() {
        let y = Admittance::new(0.04, 0.02);
        let yn = y.normalize(Z0);
        assert!(approx_eq(yn.g, 2.0)); // 0.04 / (1/50) = 2.0
        assert!(approx_eq(yn.b, 1.0));
    }

    #[test]
    fn test_admittance_gamma() {
        // Y = Y0 (matched) should give Γ = 0
        let y = Admittance::new(0.02, 0.0); // 1/50 = 0.02
        let gamma = y.to_gamma(Z0);
        assert!(approx_eq(gamma.magnitude(), 0.0));
    }

    // =========================================================================
    // Display Tests
    // =========================================================================

    #[test]
    fn test_impedance_display() {
        let z = Impedance::new(75.0, 25.0);
        assert!(format!("{}", z).contains("Ω"));
    }

    #[test]
    fn test_admittance_display() {
        let y = Admittance::new(0.02, 0.01);
        assert!(format!("{}", y).contains("S"));
    }
}

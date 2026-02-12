//! Pole-Zero Data Structures
//!
//! Core data types for pole-zero analysis and visualization.

use std::f64::consts::PI;

// =============================================================================
// Root Type
// =============================================================================

/// Type of complex root
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootType {
    /// Pole (denominator root)
    Pole,
    /// Zero (numerator root)
    Zero,
}

impl RootType {
    /// Display symbol
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Pole => "×",
            Self::Zero => "○",
        }
    }
}

// =============================================================================
// Complex Root
// =============================================================================

/// A complex root (pole or zero)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexRoot {
    /// Real part
    pub real: f64,
    /// Imaginary part
    pub imag: f64,
    /// Type (pole or zero)
    pub root_type: RootType,
    /// Optional label
    pub order: u32,
}

impl ComplexRoot {
    /// Create a pole
    pub fn pole(real: f64, imag: f64) -> Self {
        Self {
            real,
            imag,
            root_type: RootType::Pole,
            order: 1,
        }
    }

    /// Create a zero
    pub fn zero(real: f64, imag: f64) -> Self {
        Self {
            real,
            imag,
            root_type: RootType::Zero,
            order: 1,
        }
    }

    /// Create with order (for repeated roots)
    pub fn with_order(mut self, order: u32) -> Self {
        self.order = order;
        self
    }

    /// Is this a pole?
    pub fn is_pole(&self) -> bool {
        self.root_type == RootType::Pole
    }

    /// Is this a zero?
    pub fn is_zero(&self) -> bool {
        self.root_type == RootType::Zero
    }

    /// Magnitude from origin
    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    /// Phase angle from positive real axis
    pub fn phase(&self) -> f64 {
        self.imag.atan2(self.real)
    }

    /// Phase in degrees
    pub fn phase_deg(&self) -> f64 {
        self.phase() * 180.0 / PI
    }

    /// Natural frequency (radians/s)
    /// For a pole at -σ ± jω, natural frequency ωn = √(σ² + ω²)
    pub fn natural_frequency(&self) -> f64 {
        self.magnitude()
    }

    /// Damping ratio
    /// ζ = -σ / ωn
    pub fn damping_ratio(&self) -> f64 {
        if self.magnitude() == 0.0 {
            return 0.0;
        }
        -self.real / self.magnitude()
    }

    /// Q factor (quality factor)
    /// Q = 1 / (2ζ)
    pub fn q_factor(&self) -> Option<f64> {
        let zeta = self.damping_ratio();
        if zeta <= 0.0 || zeta >= 1.0 {
            return None;
        }
        Some(1.0 / (2.0 * zeta))
    }

    /// Is this root stable?
    /// For s-domain: real part must be negative
    /// For z-domain: magnitude must be less than 1
    pub fn is_stable_s_domain(&self) -> bool {
        self.real < 0.0
    }

    /// Is stable in z-domain?
    pub fn is_stable_z_domain(&self) -> bool {
        self.magnitude() < 1.0
    }

    /// Is on the jω axis (purely imaginary)?
    pub fn is_on_jw_axis(&self) -> bool {
        self.real.abs() < 1e-10
    }

    /// Is on the unit circle (z-domain marginal stability)?
    pub fn is_on_unit_circle(&self) -> bool {
        (self.magnitude() - 1.0).abs() < 1e-10
    }

    /// Conjugate
    pub fn conjugate(&self) -> Self {
        Self {
            imag: -self.imag,
            ..*self
        }
    }

    /// Is this a real root (no imaginary part)?
    pub fn is_real(&self) -> bool {
        self.imag.abs() < 1e-10
    }

    /// Is part of a complex conjugate pair?
    pub fn is_complex(&self) -> bool {
        !self.is_real()
    }
}

// =============================================================================
// Pole-Zero Data
// =============================================================================

/// Complete pole-zero data for a transfer function
#[derive(Debug, Clone)]
pub struct PoleZeroData {
    /// Name/label
    pub name: String,
    /// All roots (poles and zeros)
    pub roots: Vec<ComplexRoot>,
    /// Gain constant
    pub gain: f64,
    /// Is z-domain (discrete-time)?
    pub z_domain: bool,
}

impl Default for PoleZeroData {
    fn default() -> Self {
        Self {
            name: String::new(),
            roots: Vec::new(),
            gain: 1.0,
            z_domain: false,
        }
    }
}

impl PoleZeroData {
    /// Create new empty data
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    /// Create for z-domain analysis
    pub fn new_z_domain(name: &str) -> Self {
        Self {
            name: name.to_string(),
            z_domain: true,
            ..Default::default()
        }
    }

    /// Add a pole
    pub fn add_pole(&mut self, real: f64, imag: f64) {
        self.roots.push(ComplexRoot::pole(real, imag));
        // Add conjugate for complex poles
        if imag.abs() > 1e-10 {
            self.roots.push(ComplexRoot::pole(real, -imag));
        }
    }

    /// Add a zero
    pub fn add_zero(&mut self, real: f64, imag: f64) {
        self.roots.push(ComplexRoot::zero(real, imag));
        // Add conjugate for complex zeros
        if imag.abs() > 1e-10 {
            self.roots.push(ComplexRoot::zero(real, -imag));
        }
    }

    /// Add a real pole
    pub fn add_real_pole(&mut self, sigma: f64) {
        self.roots.push(ComplexRoot::pole(sigma, 0.0));
    }

    /// Add a real zero
    pub fn add_real_zero(&mut self, sigma: f64) {
        self.roots.push(ComplexRoot::zero(sigma, 0.0));
    }

    /// Add a complex conjugate pole pair
    pub fn add_pole_pair(&mut self, sigma: f64, omega: f64) {
        self.roots.push(ComplexRoot::pole(sigma, omega));
        self.roots.push(ComplexRoot::pole(sigma, -omega));
    }

    /// Add a complex conjugate zero pair
    pub fn add_zero_pair(&mut self, sigma: f64, omega: f64) {
        self.roots.push(ComplexRoot::zero(sigma, omega));
        self.roots.push(ComplexRoot::zero(sigma, -omega));
    }

    /// Number of roots
    pub fn len(&self) -> usize {
        self.roots.len()
    }

    /// Is empty
    pub fn is_empty(&self) -> bool {
        self.roots.is_empty()
    }

    /// Get all poles
    pub fn poles(&self) -> impl Iterator<Item = &ComplexRoot> {
        self.roots.iter().filter(|r| r.is_pole())
    }

    /// Get all zeros
    pub fn zeros(&self) -> impl Iterator<Item = &ComplexRoot> {
        self.roots.iter().filter(|r| r.is_zero())
    }

    /// Number of poles
    pub fn pole_count(&self) -> usize {
        self.roots.iter().filter(|r| r.is_pole()).count()
    }

    /// Number of zeros
    pub fn zero_count(&self) -> usize {
        self.roots.iter().filter(|r| r.is_zero()).count()
    }

    /// Is the system stable?
    pub fn is_stable(&self) -> bool {
        // Only poles affect stability
        for root in self.poles() {
            if self.z_domain {
                if !root.is_stable_z_domain() {
                    return false;
                }
            } else if !root.is_stable_s_domain() {
                return false;
            }
        }
        true
    }

    /// Is marginally stable?
    pub fn is_marginally_stable(&self) -> bool {
        for root in self.poles() {
            if self.z_domain {
                if root.is_on_unit_circle() {
                    return true;
                }
            } else if root.is_on_jw_axis() {
                return true;
            }
        }
        false
    }

    /// Find the dominant poles (closest to stability boundary)
    pub fn dominant_poles(&self) -> Vec<&ComplexRoot> {
        let mut poles: Vec<&ComplexRoot> = self
            .poles()
            .filter(|root| {
                if self.z_domain {
                    root.magnitude().is_finite()
                } else {
                    root.real.is_finite()
                }
            })
            .collect();

        if poles.is_empty() {
            return Vec::new();
        }

        if self.z_domain {
            // For z-domain: closest to unit circle
            poles.sort_by(|a, b| {
                let da = (1.0 - a.magnitude()).abs();
                let db = (1.0 - b.magnitude()).abs();
                da.total_cmp(&db)
            });
        } else {
            // For s-domain: closest to jω axis (least negative real part)
            poles.sort_by(|a, b| {
                let da = a.real.abs();
                let db = b.real.abs();
                da.total_cmp(&db)
            });
        }

        // Return poles with real part close to the dominant
        let Some(dominant) = poles.first().copied() else {
            return Vec::new();
        };

        let threshold = if self.z_domain {
            (1.0 - dominant.magnitude()).abs() + 0.01
        } else {
            dominant.real.abs() + 0.01
        };

        poles
            .into_iter()
            .filter(|p| {
                if self.z_domain {
                    let distance = (1.0 - p.magnitude()).abs();
                    distance.is_finite() && distance <= threshold
                } else {
                    p.real.is_finite() && p.real.abs() <= threshold
                }
            })
            .collect()
    }

    /// Real axis range
    pub fn real_range(&self) -> (f64, f64) {
        if self.roots.is_empty() {
            return (-2.0, 2.0);
        }
        let min = self.roots.iter().map(|r| r.real).fold(f64::MAX, f64::min);
        let max = self.roots.iter().map(|r| r.real).fold(f64::MIN, f64::max);
        let padding = (max - min).max(1.0) * 0.2;
        (min - padding, max + padding)
    }

    /// Imaginary axis range
    pub fn imag_range(&self) -> (f64, f64) {
        if self.roots.is_empty() {
            return (-2.0, 2.0);
        }
        let min = self.roots.iter().map(|r| r.imag).fold(f64::MAX, f64::min);
        let max = self.roots.iter().map(|r| r.imag).fold(f64::MIN, f64::max);
        let padding = (max - min).max(1.0) * 0.2;
        (min - padding, max + padding)
    }

    /// Order of the system (highest pole count)
    pub fn system_order(&self) -> usize {
        self.pole_count()
    }

    /// Relative degree (poles - zeros)
    pub fn relative_degree(&self) -> i32 {
        self.pole_count() as i32 - self.zero_count() as i32
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

    fn approx_eq_rel(a: f64, b: f64, rel_tol: f64) -> bool {
        if b.abs() < EPSILON {
            a.abs() < EPSILON
        } else {
            ((a - b) / b).abs() < rel_tol
        }
    }

    // =========================================================================
    // RootType Tests
    // =========================================================================

    #[test]
    fn test_root_type_symbols() {
        assert_eq!(RootType::Pole.symbol(), "×");
        assert_eq!(RootType::Zero.symbol(), "○");
    }

    // =========================================================================
    // ComplexRoot Tests
    // =========================================================================

    #[test]
    fn test_complex_root_pole() {
        let p = ComplexRoot::pole(-1.0, 2.0);
        assert!(p.is_pole());
        assert!(!p.is_zero());
        assert_eq!(p.real, -1.0);
        assert_eq!(p.imag, 2.0);
    }

    #[test]
    fn test_complex_root_zero() {
        let z = ComplexRoot::zero(0.0, 1.0);
        assert!(z.is_zero());
        assert!(!z.is_pole());
    }

    #[test]
    fn test_root_with_order() {
        let p = ComplexRoot::pole(-1.0, 0.0).with_order(2);
        assert_eq!(p.order, 2);
    }

    #[test]
    fn test_magnitude() {
        let p = ComplexRoot::pole(3.0, 4.0);
        assert!(approx_eq(p.magnitude(), 5.0));
    }

    #[test]
    fn test_phase() {
        let p = ComplexRoot::pole(1.0, 1.0);
        assert!(approx_eq(p.phase(), PI / 4.0));
    }

    #[test]
    fn test_phase_deg() {
        let p = ComplexRoot::pole(0.0, 1.0);
        assert!(approx_eq(p.phase_deg(), 90.0));
    }

    #[test]
    fn test_natural_frequency() {
        let p = ComplexRoot::pole(-3.0, 4.0);
        assert!(approx_eq(p.natural_frequency(), 5.0));
    }

    #[test]
    fn test_damping_ratio() {
        // Pole at -1 ± j√3 has ωn = 2, ζ = 0.5
        let p = ComplexRoot::pole(-1.0, 3.0_f64.sqrt());
        assert!(approx_eq_rel(p.damping_ratio(), 0.5, 0.01));
    }

    #[test]
    fn test_q_factor() {
        // ζ = 0.5 → Q = 1
        let p = ComplexRoot::pole(-1.0, 3.0_f64.sqrt());
        assert!(approx_eq_rel(p.q_factor().unwrap(), 1.0, 0.01));
    }

    #[test]
    fn test_q_factor_overdamped() {
        // Real pole (ζ ≥ 1) has no Q factor
        let p = ComplexRoot::pole(-1.0, 0.0);
        assert!(p.q_factor().is_none());
    }

    #[test]
    fn test_stable_s_domain() {
        assert!(ComplexRoot::pole(-1.0, 2.0).is_stable_s_domain());
        assert!(!ComplexRoot::pole(0.5, 1.0).is_stable_s_domain());
    }

    #[test]
    fn test_stable_z_domain() {
        assert!(ComplexRoot::pole(0.5, 0.3).is_stable_z_domain());
        assert!(!ComplexRoot::pole(0.9, 0.9).is_stable_z_domain());
    }

    #[test]
    fn test_on_jw_axis() {
        assert!(ComplexRoot::pole(0.0, 1.0).is_on_jw_axis());
        assert!(!ComplexRoot::pole(-0.1, 1.0).is_on_jw_axis());
    }

    #[test]
    fn test_on_unit_circle() {
        assert!(ComplexRoot::pole(0.6, 0.8).is_on_unit_circle());
        assert!(!ComplexRoot::pole(0.5, 0.5).is_on_unit_circle());
    }

    #[test]
    fn test_conjugate() {
        let p = ComplexRoot::pole(-1.0, 2.0);
        let conj = p.conjugate();
        assert_eq!(conj.real, -1.0);
        assert_eq!(conj.imag, -2.0);
    }

    #[test]
    fn test_is_real() {
        assert!(ComplexRoot::pole(-1.0, 0.0).is_real());
        assert!(!ComplexRoot::pole(-1.0, 0.5).is_real());
    }

    // =========================================================================
    // PoleZeroData Tests
    // =========================================================================

    #[test]
    fn test_pz_data_new() {
        let data = PoleZeroData::new("Test");
        assert!(data.is_empty());
        assert_eq!(data.name, "Test");
        assert!(!data.z_domain);
    }

    #[test]
    fn test_pz_data_z_domain() {
        let data = PoleZeroData::new_z_domain("Test");
        assert!(data.z_domain);
    }

    #[test]
    fn test_add_pole() {
        let mut data = PoleZeroData::new("Test");
        data.add_pole(-1.0, 2.0);
        // Should add conjugate pair
        assert_eq!(data.len(), 2);
        assert_eq!(data.pole_count(), 2);
    }

    #[test]
    fn test_add_real_pole() {
        let mut data = PoleZeroData::new("Test");
        data.add_real_pole(-1.0);
        assert_eq!(data.len(), 1);
    }

    #[test]
    fn test_add_zero() {
        let mut data = PoleZeroData::new("Test");
        data.add_zero(0.0, 1.0);
        assert_eq!(data.len(), 2); // Conjugate pair
        assert_eq!(data.zero_count(), 2);
    }

    #[test]
    fn test_add_pole_pair() {
        let mut data = PoleZeroData::new("Test");
        data.add_pole_pair(-1.0, 2.0);
        assert_eq!(data.pole_count(), 2);
    }

    #[test]
    fn test_is_stable_s_domain() {
        let mut data = PoleZeroData::new("Test");
        data.add_pole(-1.0, 2.0);
        assert!(data.is_stable());

        let mut unstable = PoleZeroData::new("Unstable");
        unstable.add_pole(0.5, 1.0);
        assert!(!unstable.is_stable());
    }

    #[test]
    fn test_is_stable_z_domain() {
        let mut data = PoleZeroData::new_z_domain("Test");
        data.add_pole(0.5, 0.3);
        assert!(data.is_stable());

        let mut unstable = PoleZeroData::new_z_domain("Unstable");
        unstable.add_pole(0.9, 0.6);
        assert!(!unstable.is_stable());
    }

    #[test]
    fn test_marginally_stable() {
        let mut data = PoleZeroData::new("Test");
        data.add_pole(0.0, 1.0); // On jω axis
        assert!(data.is_marginally_stable());
    }

    #[test]
    fn test_dominant_poles() {
        let mut data = PoleZeroData::new("Test");
        data.add_pole(-0.1, 1.0); // Dominant (closest to jω axis)
        data.add_pole(-10.0, 0.0); // Fast pole

        let dominant = data.dominant_poles();
        assert!(!dominant.is_empty());
        assert!(approx_eq_rel(dominant[0].real, -0.1, 0.01));
    }

    #[test]
    fn test_dominant_poles_ignores_non_finite_s_domain_roots() {
        let mut data = PoleZeroData::new("NonFinite");
        data.roots.push(ComplexRoot::pole(f64::NAN, 1.0));
        data.roots.push(ComplexRoot::pole(-0.05, 0.2));
        data.roots.push(ComplexRoot::pole(-5.0, 0.0));

        let dominant = data.dominant_poles();
        assert!(!dominant.is_empty());
        assert!(dominant.iter().all(|root| root.real.is_finite()));
        assert!(dominant
            .iter()
            .any(|root| approx_eq_rel(root.real, -0.05, 0.01)));
    }

    #[test]
    fn test_dominant_poles_ignores_non_finite_z_domain_roots() {
        let mut data = PoleZeroData::new_z_domain("NonFiniteZ");
        data.roots.push(ComplexRoot::pole(f64::NAN, 0.0));
        data.roots.push(ComplexRoot::pole(0.95, 0.01));
        data.roots.push(ComplexRoot::pole(0.2, 0.0));

        let dominant = data.dominant_poles();
        assert!(!dominant.is_empty());
        assert!(dominant.iter().all(|root| root.magnitude().is_finite()));
        assert!(dominant
            .iter()
            .any(|root| approx_eq_rel(root.real, 0.95, 0.01)));
    }

    #[test]
    fn test_system_order() {
        let mut data = PoleZeroData::new("Test");
        data.add_pole(-1.0, 2.0); // Adds 2 poles
        data.add_real_pole(-3.0);
        assert_eq!(data.system_order(), 3);
    }

    #[test]
    fn test_relative_degree() {
        let mut data = PoleZeroData::new("Test");
        data.add_pole(-1.0, 0.0);
        data.add_pole(-2.0, 0.0);
        data.add_real_zero(-0.5);
        assert_eq!(data.relative_degree(), 1);
    }

    #[test]
    fn test_real_range() {
        let mut data = PoleZeroData::new("Test");
        data.add_real_pole(-5.0);
        data.add_real_zero(-1.0);

        let (min, max) = data.real_range();
        assert!(min < -5.0);
        assert!(max > -1.0);
    }

    #[test]
    fn test_imag_range() {
        let mut data = PoleZeroData::new("Test");
        data.add_pole(-1.0, 3.0);

        let (min, max) = data.imag_range();
        assert!(min < -3.0);
        assert!(max > 3.0);
    }

    #[test]
    fn test_poles_iterator() {
        let mut data = PoleZeroData::new("Test");
        data.add_real_pole(-1.0);
        data.add_real_zero(-2.0);

        let poles: Vec<_> = data.poles().collect();
        assert_eq!(poles.len(), 1);
    }

    #[test]
    fn test_zeros_iterator() {
        let mut data = PoleZeroData::new("Test");
        data.add_real_pole(-1.0);
        data.add_real_zero(-2.0);

        let zeros: Vec<_> = data.zeros().collect();
        assert_eq!(zeros.len(), 1);
    }
}

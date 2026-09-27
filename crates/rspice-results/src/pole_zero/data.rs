//! Pole-zero root data and exact natural-frequency/damping queries.

use super::PoleZeroRootSetEvidence;

/// Type of complex root
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RootType {
    /// Pole (denominator root)
    Pole,
    /// Zero (numerator root)
    Zero,
}

/// A complex root (pole or zero)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexRoot {
    /// Real part
    pub real: f64,
    /// Imaginary part
    pub imag: f64,
    /// Type (pole or zero)
    pub root_type: RootType,
}

impl ComplexRoot {
    /// Create a pole
    pub fn pole(real: f64, imag: f64) -> Self {
        Self {
            real,
            imag,
            root_type: RootType::Pole,
        }
    }

    /// Create a zero
    pub fn zero(real: f64, imag: f64) -> Self {
        Self {
            real,
            imag,
            root_type: RootType::Zero,
        }
    }

    /// Is this a pole?
    pub fn is_pole(&self) -> bool {
        self.root_type == RootType::Pole
    }

    /// Is this root purely real?
    pub fn is_real(&self) -> bool {
        self.imag.abs() < 1e-10
    }

    /// Magnitude from origin
    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    /// Natural frequency (radians/s).
    ///
    /// For a pole at -σ ± jω this is ωn = √(σ² + ω²), i.e. the distance from
    /// the origin.
    pub fn natural_frequency(&self) -> f64 {
        self.magnitude()
    }

    /// Damping ratio ζ = -σ / ωn.
    ///
    /// Zero at the origin rather than NaN: a root there has no meaningful
    /// damping, and the readout should show 0 instead of propagating NaN into
    /// the table.
    pub fn damping_ratio(&self) -> f64 {
        if self.magnitude() == 0.0 {
            return 0.0;
        }
        -self.real / self.magnitude()
    }
}

/// Complete pole-zero data for a transfer function
#[derive(Debug, Clone)]
pub struct PoleZeroData {
    /// Name/label
    pub name: String,
    /// All roots (poles and zeros)
    pub roots: Vec<ComplexRoot>,
    /// Finite DC gain when the transfer has one.
    pub gain: Option<f64>,
    /// Completeness and numerical qualification evidence for the pole set.
    pub pole_evidence: PoleZeroRootSetEvidence,
    /// Completeness and numerical qualification evidence for the zero set.
    pub zero_evidence: PoleZeroRootSetEvidence,
}

impl Default for PoleZeroData {
    fn default() -> Self {
        Self {
            name: String::new(),
            roots: Vec::new(),
            gain: None,
            pole_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
            zero_evidence: PoleZeroRootSetEvidence::LegacyUnknown,
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
}

//! Pole-zero data: the roots of a transfer function and how to describe one.
//!
//! `result_document::pz` builds a [`PoleZeroData`] from a run's roots and reads
//! `roots` and `name` off it, asking each root for `is_real`,
//! `natural_frequency`, and `damping_ratio` in the table beside the plot. That
//! is the whole surface.
//!
//! What used to sit here was a small control-theory library on top of it:
//! z-domain construction, conjugate-pair and zero-adding helpers, stability and
//! marginal-stability predicates in both domains, dominant-pole ranking, Q
//! factor, system order, relative degree, and axis-range queries the viewer
//! computes for itself. None of it had a caller. The z-domain flag went with
//! the predicates that were its only readers — pole-zero runs here are
//! continuous-time.

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

    /// Is this a zero?
    #[cfg(test)]
    pub fn is_zero(&self) -> bool {
        self.root_type == RootType::Zero
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
    /// Gain constant
    pub gain: f64,
}

impl Default for PoleZeroData {
    fn default() -> Self {
        Self {
            name: String::new(),
            roots: Vec::new(),
            gain: 1.0,
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

    /// Add a real pole
    #[cfg(test)]
    pub fn add_real_pole(&mut self, sigma: f64) {
        self.roots.push(ComplexRoot::pole(sigma, 0.0));
    }
}

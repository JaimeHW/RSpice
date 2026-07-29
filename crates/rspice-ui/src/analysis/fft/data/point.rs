//! One spectrum bin.

// =============================================================================
// FFT Point
// =============================================================================

/// Single point in FFT spectrum
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FftPoint {
    /// Frequency in Hz
    pub frequency: f64,
    /// Magnitude (linear)
    pub magnitude: f64,
    /// Phase in radians
    pub phase: f64,
}

impl FftPoint {
    /// Create new point
    pub fn new(frequency: f64, magnitude: f64, phase: f64) -> Self {
        Self {
            frequency,
            magnitude,
            phase,
        }
    }

    /// Create from complex components
    pub fn from_complex(frequency: f64, real: f64, imag: f64) -> Self {
        Self {
            frequency,
            magnitude: (real * real + imag * imag).sqrt(),
            phase: imag.atan2(real),
        }
    }

    /// Magnitude in dB (20 * log10)
    pub fn magnitude_db(&self) -> f64 {
        if self.magnitude <= 0.0 {
            f64::NEG_INFINITY
        } else {
            20.0 * self.magnitude.log10()
        }
    }

}

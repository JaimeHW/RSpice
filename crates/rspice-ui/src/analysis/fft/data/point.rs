use std::f64::consts::PI;

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

    /// Magnitude in dBV (reference 1V)
    pub fn magnitude_dbv(&self) -> f64 {
        self.magnitude_db()
    }

    /// Magnitude in dBm (reference 1mW into 50Ω)
    pub fn magnitude_dbm(&self, z0: f64) -> f64 {
        if self.magnitude <= 0.0 {
            f64::NEG_INFINITY
        } else {
            let z0 = if z0.is_finite() && z0 > 0.0 { z0 } else { 50.0 };
            let power_mw = (self.magnitude * self.magnitude) / z0 * 1000.0;
            10.0 * power_mw.log10()
        }
    }

    /// Phase in degrees
    pub fn phase_deg(&self) -> f64 {
        self.phase * 180.0 / PI
    }
}

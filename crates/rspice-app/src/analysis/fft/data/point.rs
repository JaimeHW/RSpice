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
    #[cfg(test)]
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
            magnitude: real.hypot(imag),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complex_magnitude_avoids_intermediate_overflow() {
        let point = FftPoint::from_complex(1.0, f64::MAX / 4.0, f64::MAX / 4.0);

        assert!(point.magnitude.is_finite());
        assert_eq!(point.magnitude, (f64::MAX / 4.0).hypot(f64::MAX / 4.0));
    }
}

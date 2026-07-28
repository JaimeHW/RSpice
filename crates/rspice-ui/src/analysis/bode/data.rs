//! Bode data: frequency responses collected from an AC run.
//!
//! Held so the workspace can tell whether a Bode result exists. The Bode
//! viewer draws from the AC Bode summary in `state::simulation::ac_bode` and
//! computes its own margins, so nothing downstream reads these points.
//!
//! What used to live here was a frequency-response analysis library: DC gain,
//! 3 dB bandwidth, phase and magnitude ranges, interpolation at a frequency,
//! angular frequency, and a `StabilityMargins` type with its own gain/phase
//! margin search and formatters. All of it was unreachable, and the margin
//! search in particular was a second implementation of what
//! `result_document::bode` does against the summary.

/// Single frequency response data point
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrequencyPoint {
    /// Frequency in Hz
    pub frequency: f64,
    /// Magnitude (linear)
    pub magnitude: f64,
    /// Phase in radians
    pub phase: f64,
}

impl FrequencyPoint {
    /// Create from a complex value, converting to magnitude and phase.
    pub fn from_complex(frequency: f64, real: f64, imag: f64) -> Self {
        Self {
            frequency,
            magnitude: (real * real + imag * imag).sqrt(),
            phase: imag.atan2(real),
        }
    }
}

/// Complete frequency response data
#[derive(Debug, Clone, Default)]
pub struct FrequencyResponse {
    /// Data points, in sweep order
    pub points: Vec<FrequencyPoint>,
}

impl FrequencyResponse {
    /// Build from frequency, magnitude, and phase arrays.
    ///
    /// Zipped to the shortest of the three rather than asserted equal: a
    /// truncated result should carry what it has.
    pub fn from_arrays(freq: &[f64], mag: &[f64], phase: &[f64]) -> Self {
        let n = freq.len().min(mag.len()).min(phase.len());
        Self {
            points: (0..n)
                .map(|i| FrequencyPoint {
                    frequency: freq[i],
                    magnitude: mag[i],
                    phase: phase[i],
                })
                .collect(),
        }
    }

    /// Build from frequency and complex real/imaginary arrays.
    pub fn from_complex_arrays(freq: &[f64], real: &[f64], imag: &[f64]) -> Self {
        let n = freq.len().min(real.len()).min(imag.len());
        Self {
            points: (0..n)
                .map(|i| FrequencyPoint::from_complex(freq[i], real[i], imag[i]))
                .collect(),
        }
    }
}

/// Collection of frequency responses for Bode plot
#[derive(Debug, Clone, Default)]
pub struct BodeData {
    /// All frequency responses
    pub responses: Vec<FrequencyResponse>,
}

impl BodeData {
    /// Create new empty Bode data
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a frequency response
    pub fn add_response(&mut self, response: FrequencyResponse) {
        self.responses.push(response);
    }

    /// Number of responses
    pub fn response_count(&self) -> usize {
        self.responses.len()
    }
}

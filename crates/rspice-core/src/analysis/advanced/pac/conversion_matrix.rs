//! Conversion Matrix for PAC Analysis
//!
//! The conversion matrix is the core data structure for PAC analysis.
//! It captures the linear time-varying (LTV) transfer function that maps
//! input signals at one sideband to output signals at other sidebands.
//!
//! # Theory
//!
//! For a mixer with LO frequency f₀, the conversion matrix H(Δf) relates:
//!   V_out,k(Δf) = Σ_m H_{k,m}(Δf) · V_in,m(Δf)
//!
//! where k and m are sideband indices, and Δf is the input offset from f₀.
//!
//! Key properties:
//! - H[0,-1] is mixer conversion gain (RF→IF for k=0, m=-1 or +1)
//! - H[0,0] is IF-to-IF transfer (direct feedthrough)
//! - Diagonal elements H[k,k] represent harmonic transfer at same sideband

use crate::{Complex64, Value};
use std::collections::HashMap;

//=============================================================================
// Sideband Transfer Function
//=============================================================================

/// Transfer function from one sideband to another at a specific frequency
#[derive(Debug, Clone)]
pub struct SidebandTransfer {
    /// Input sideband index (relative to fundamental)
    pub input_sideband: i32,

    /// Output sideband index (relative to fundamental)
    pub output_sideband: i32,

    /// Frequency offset from fundamental (Hz)
    pub frequency_offset: Value,

    /// Complex transfer value (voltage gain)
    pub transfer: Complex64,
}

impl SidebandTransfer {
    /// Create a new sideband transfer
    pub fn new(
        input_sideband: i32,
        output_sideband: i32,
        frequency_offset: Value,
        transfer: Complex64,
    ) -> Self {
        Self {
            input_sideband,
            output_sideband,
            frequency_offset,
            transfer,
        }
    }

    /// Get magnitude in linear scale
    pub fn magnitude(&self) -> Value {
        self.transfer.norm()
    }

    /// Get magnitude in dB
    pub fn magnitude_db(&self) -> Value {
        20.0 * self.transfer.norm().log10()
    }

    /// Get phase in radians
    pub fn phase_rad(&self) -> Value {
        self.transfer.arg()
    }

    /// Get phase in degrees
    pub fn phase_deg(&self) -> Value {
        self.transfer.arg() * 180.0 / std::f64::consts::PI
    }

    /// Get the absolute input frequency
    pub fn input_frequency(&self, fundamental: Value) -> Value {
        (self.input_sideband as Value) * fundamental + self.frequency_offset
    }

    /// Get the absolute output frequency
    pub fn output_frequency(&self, fundamental: Value) -> Value {
        (self.output_sideband as Value) * fundamental + self.frequency_offset
    }
}

//=============================================================================
// Conversion Matrix
//=============================================================================

/// Conversion matrix for PAC analysis
///
/// Stores the complete set of transfer functions between all sidebands
/// at all analyzed frequencies. Provides efficient lookup by sideband pair.
#[derive(Debug, Clone)]
pub struct ConversionMatrix {
    /// Fundamental frequency (Hz)
    fundamental: Value,

    /// Sideband range
    sideband_min: i32,
    sideband_max: i32,

    /// Frequency offset points analyzed
    frequencies: Vec<Value>,

    /// Matrix elements: [freq_index][output_sideband][input_sideband]
    /// Stored as flattened Vec with computed indexing
    elements: Vec<Complex64>,

    /// Number of sidebands
    num_sidebands: usize,
}

impl ConversionMatrix {
    /// Create a new conversion matrix
    ///
    /// # Arguments
    /// * `fundamental` - LO/fundamental frequency in Hz
    /// * `sideband_min` - Minimum sideband index
    /// * `sideband_max` - Maximum sideband index
    /// * `frequencies` - Frequency offset points to analyze
    pub fn new(
        fundamental: Value,
        sideband_min: i32,
        sideband_max: i32,
        frequencies: Vec<Value>,
    ) -> Self {
        let num_sidebands = (sideband_max - sideband_min + 1) as usize;
        let num_elements = frequencies.len() * num_sidebands * num_sidebands;

        Self {
            fundamental,
            sideband_min,
            sideband_max,
            frequencies,
            elements: vec![Complex64::new(0.0, 0.0); num_elements],
            num_sidebands,
        }
    }

    /// Get the fundamental frequency
    pub fn fundamental(&self) -> Value {
        self.fundamental
    }

    /// Get the sideband range
    pub fn sideband_range(&self) -> (i32, i32) {
        (self.sideband_min, self.sideband_max)
    }

    /// Get the number of sidebands
    pub fn num_sidebands(&self) -> usize {
        self.num_sidebands
    }

    /// Get the frequency points
    pub fn frequencies(&self) -> &[Value] {
        &self.frequencies
    }

    /// Get the number of frequency points
    pub fn num_frequencies(&self) -> usize {
        self.frequencies.len()
    }

    /// Compute flat index from (freq_idx, out_sb, in_sb)
    fn index(&self, freq_idx: usize, out_sideband: i32, in_sideband: i32) -> Option<usize> {
        if freq_idx >= self.frequencies.len() {
            return None;
        }

        let out_idx = (out_sideband - self.sideband_min) as usize;
        let in_idx = (in_sideband - self.sideband_min) as usize;

        if out_idx >= self.num_sidebands || in_idx >= self.num_sidebands {
            return None;
        }

        Some(
            freq_idx * self.num_sidebands * self.num_sidebands
                + out_idx * self.num_sidebands
                + in_idx,
        )
    }

    /// Set a matrix element
    pub fn set(
        &mut self,
        freq_idx: usize,
        output_sideband: i32,
        input_sideband: i32,
        value: Complex64,
    ) {
        if let Some(idx) = self.index(freq_idx, output_sideband, input_sideband) {
            self.elements[idx] = value;
        }
    }

    /// Get a matrix element
    pub fn get(&self, freq_idx: usize, output_sideband: i32, input_sideband: i32) -> Complex64 {
        self.index(freq_idx, output_sideband, input_sideband)
            .map(|idx| self.elements[idx])
            .unwrap_or(Complex64::new(0.0, 0.0))
    }

    /// Get transfer from input sideband to output sideband as SidebandTransfer vec
    pub fn get_transfer(&self, input_sideband: i32, output_sideband: i32) -> Vec<SidebandTransfer> {
        self.frequencies
            .iter()
            .enumerate()
            .map(|(idx, &freq)| {
                SidebandTransfer::new(
                    input_sideband,
                    output_sideband,
                    freq,
                    self.get(idx, output_sideband, input_sideband),
                )
            })
            .collect()
    }

    /// Get conversion gain (typically RF to IF for mixers)
    ///
    /// For a down-converting mixer: input at sideband +1 (RF = LO + Δf),
    /// output at sideband 0 (IF = Δf)
    pub fn conversion_gain(&self, freq_idx: usize) -> Complex64 {
        // Standard convention: RF at sideband 1, IF at sideband 0
        self.get(freq_idx, 0, 1)
    }

    /// Get image rejection (RF at sideband -1 to IF at sideband 0)
    pub fn image_transfer(&self, freq_idx: usize) -> Complex64 {
        self.get(freq_idx, 0, -1)
    }

    /// Calculate image rejection ratio (IRR) in dB at a frequency point
    pub fn image_rejection_db(&self, freq_idx: usize) -> Value {
        let signal = self.conversion_gain(freq_idx).norm();
        let image = self.image_transfer(freq_idx).norm();

        if image > 0.0 {
            20.0 * (signal / image).log10()
        } else {
            f64::INFINITY
        }
    }

    /// Get all non-zero elements as a hashmap for sparse representation
    pub fn to_sparse(&self) -> HashMap<(usize, i32, i32), Complex64> {
        let mut sparse = HashMap::new();

        for freq_idx in 0..self.frequencies.len() {
            for out_sb in self.sideband_min..=self.sideband_max {
                for in_sb in self.sideband_min..=self.sideband_max {
                    let val = self.get(freq_idx, out_sb, in_sb);
                    if val.norm() > 1e-15 {
                        sparse.insert((freq_idx, out_sb, in_sb), val);
                    }
                }
            }
        }

        sparse
    }

    /// Get the diagonal elements (same-sideband transfer) across all frequencies
    pub fn diagonal(&self, sideband: i32) -> Vec<Complex64> {
        (0..self.frequencies.len())
            .map(|idx| self.get(idx, sideband, sideband))
            .collect()
    }

    /// Get all sidebands as a vector
    pub fn sideband_indices(&self) -> Vec<i32> {
        (self.sideband_min..=self.sideband_max).collect()
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_sideband_transfer_creation() {
        let transfer = SidebandTransfer::new(
            1,                        // input sideband
            0,                        // output sideband
            1e6,                      // frequency offset
            Complex64::new(0.5, 0.0), // 0.5 voltage gain
        );

        assert_eq!(transfer.input_sideband, 1);
        assert_eq!(transfer.output_sideband, 0);
        assert!((transfer.frequency_offset - 1e6).abs() < 1.0);
        assert!((transfer.magnitude() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_sideband_transfer_magnitude_db() {
        let transfer = SidebandTransfer::new(
            0,
            0,
            0.0,
            Complex64::new(0.1, 0.0), // -20 dB
        );

        assert!((transfer.magnitude_db() - (-20.0)).abs() < 0.01);
    }

    #[test]
    fn test_sideband_transfer_phase() {
        let transfer = SidebandTransfer::new(
            0,
            0,
            0.0,
            Complex64::new(0.0, 1.0), // 90 degrees
        );

        assert!((transfer.phase_rad() - PI / 2.0).abs() < 1e-10);
        assert!((transfer.phase_deg() - 90.0).abs() < 1e-6);
    }

    #[test]
    fn test_sideband_transfer_frequencies() {
        let transfer = SidebandTransfer::new(
            1,   // input at LO + 1*f0 + Δf
            0,   // output at Δf
            1e6, // Δf = 1 MHz
            Complex64::new(1.0, 0.0),
        );

        let fundamental = 1e9; // 1 GHz LO

        // Input frequency: 1 * 1e9 + 1e6 = 1.001 GHz
        assert!((transfer.input_frequency(fundamental) - 1.001e9).abs() < 1.0);

        // Output frequency: 0 * 1e9 + 1e6 = 1 MHz
        assert!((transfer.output_frequency(fundamental) - 1e6).abs() < 1.0);
    }

    #[test]
    fn test_conversion_matrix_creation() {
        let freqs = vec![1e6, 2e6, 3e6];
        let matrix = ConversionMatrix::new(1e9, -2, 2, freqs);

        assert!((matrix.fundamental() - 1e9).abs() < 1.0);
        assert_eq!(matrix.sideband_range(), (-2, 2));
        assert_eq!(matrix.num_sidebands(), 5);
        assert_eq!(matrix.num_frequencies(), 3);
    }

    #[test]
    fn test_conversion_matrix_set_get() {
        let freqs = vec![1e6, 2e6];
        let mut matrix = ConversionMatrix::new(1e9, -1, 1, freqs);

        let val = Complex64::new(0.7, 0.1);
        matrix.set(0, 0, 1, val); // RF to IF at first frequency

        let retrieved = matrix.get(0, 0, 1);
        assert!((retrieved.re - 0.7).abs() < 1e-10);
        assert!((retrieved.im - 0.1).abs() < 1e-10);

        // Other elements should be zero
        let zero = matrix.get(0, 1, 0);
        assert!((zero.norm()).abs() < 1e-10);
    }

    #[test]
    fn test_conversion_matrix_out_of_bounds() {
        let freqs = vec![1e6];
        let matrix = ConversionMatrix::new(1e9, -1, 1, freqs);

        // Out of bounds should return zero
        let val = matrix.get(0, 5, 0); // sideband 5 is out of range
        assert!((val.norm()).abs() < 1e-10);

        let val2 = matrix.get(10, 0, 0); // freq index out of range
        assert!((val2.norm()).abs() < 1e-10);
    }

    #[test]
    fn test_conversion_gain() {
        let freqs = vec![1e6];
        let mut matrix = ConversionMatrix::new(1e9, -1, 1, freqs);

        // Set RF to IF conversion gain
        matrix.set(0, 0, 1, Complex64::new(0.8, 0.0));

        let gain = matrix.conversion_gain(0);
        assert!((gain.re - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_image_rejection() {
        let freqs = vec![1e6];
        let mut matrix = ConversionMatrix::new(1e9, -1, 1, freqs);

        // Set signal and image conversions
        matrix.set(0, 0, 1, Complex64::new(1.0, 0.0)); // Signal: 0 dB
        matrix.set(0, 0, -1, Complex64::new(0.01, 0.0)); // Image: -40 dB

        let irr = matrix.image_rejection_db(0);
        assert!((irr - 40.0).abs() < 0.1); // 40 dB image rejection
    }

    #[test]
    fn test_get_transfer() {
        let freqs = vec![1e6, 2e6, 3e6];
        let mut matrix = ConversionMatrix::new(1e9, -1, 1, freqs);

        matrix.set(0, 0, 1, Complex64::new(0.5, 0.0));
        matrix.set(1, 0, 1, Complex64::new(0.6, 0.0));
        matrix.set(2, 0, 1, Complex64::new(0.7, 0.0));

        let transfers = matrix.get_transfer(1, 0);
        assert_eq!(transfers.len(), 3);
        assert!((transfers[0].magnitude() - 0.5).abs() < 1e-10);
        assert!((transfers[1].magnitude() - 0.6).abs() < 1e-10);
        assert!((transfers[2].magnitude() - 0.7).abs() < 1e-10);
    }

    #[test]
    fn test_diagonal() {
        let freqs = vec![1e6, 2e6];
        let mut matrix = ConversionMatrix::new(1e9, -1, 1, freqs);

        // Set diagonal for sideband 0
        matrix.set(0, 0, 0, Complex64::new(0.9, 0.0));
        matrix.set(1, 0, 0, Complex64::new(0.95, 0.0));

        let diag = matrix.diagonal(0);
        assert_eq!(diag.len(), 2);
        assert!((diag[0].re - 0.9).abs() < 1e-10);
        assert!((diag[1].re - 0.95).abs() < 1e-10);
    }

    #[test]
    fn test_sideband_indices() {
        let matrix = ConversionMatrix::new(1e9, -2, 2, vec![1e6]);
        let indices = matrix.sideband_indices();
        assert_eq!(indices, vec![-2, -1, 0, 1, 2]);
    }

    #[test]
    fn test_to_sparse() {
        let freqs = vec![1e6];
        let mut matrix = ConversionMatrix::new(1e9, -1, 1, freqs);

        matrix.set(0, 0, 1, Complex64::new(0.5, 0.0));
        matrix.set(0, 0, 0, Complex64::new(1.0, 0.0));

        let sparse = matrix.to_sparse();
        assert_eq!(sparse.len(), 2);
        assert!(sparse.contains_key(&(0, 0, 1)));
        assert!(sparse.contains_key(&(0, 0, 0)));
    }

    #[test]
    fn test_empty_matrix() {
        let matrix = ConversionMatrix::new(1e9, 0, 0, vec![]);
        assert_eq!(matrix.num_frequencies(), 0);
        assert_eq!(matrix.num_sidebands(), 1);
    }

    #[test]
    fn test_single_sideband() {
        let freqs = vec![1e6];
        let mut matrix = ConversionMatrix::new(1e9, 0, 0, freqs);

        matrix.set(0, 0, 0, Complex64::new(0.99, 0.0));
        let val = matrix.get(0, 0, 0);
        assert!((val.re - 0.99).abs() < 1e-10);
    }

    #[test]
    fn test_complex_transfer() {
        let transfer = SidebandTransfer::new(
            1,
            0,
            1e6,
            Complex64::new(0.707, 0.707), // 45 degrees, ~1.0 magnitude
        );

        assert!((transfer.magnitude() - 1.0).abs() < 0.01);
        assert!((transfer.phase_deg() - 45.0).abs() < 0.1);
    }

    #[test]
    fn test_infinite_image_rejection() {
        let freqs = vec![1e6];
        let mut matrix = ConversionMatrix::new(1e9, -1, 1, freqs);

        // Perfect image rejection (image = 0)
        matrix.set(0, 0, 1, Complex64::new(1.0, 0.0));
        matrix.set(0, 0, -1, Complex64::new(0.0, 0.0));

        let irr = matrix.image_rejection_db(0);
        assert!(irr.is_infinite());
    }
}

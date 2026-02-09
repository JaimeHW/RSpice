//! PAC Analysis Results
//!
//! Contains result types for PAC analysis including per-node voltage spectra,
//! conversion matrices, and helper methods for extracting RF metrics.

use super::conversion_matrix::{ConversionMatrix, SidebandTransfer};
use crate::{Complex64, Value};
use std::collections::HashMap;

//=============================================================================
// Per-Sideband Data
//=============================================================================

/// Data for a single sideband at a single frequency point
#[derive(Debug, Clone)]
pub struct PacSidebandData {
    /// Sideband index relative to fundamental
    pub sideband: i32,

    /// Frequency offset from fundamental (Hz)
    pub frequency_offset: Value,

    /// Absolute frequency = sideband * f₀ + frequency_offset
    pub absolute_frequency: Value,

    /// Complex voltage at each node: node_index -> complex voltage
    pub node_voltages: Vec<Complex64>,

    /// Complex current at each branch (for sources/inductors)
    pub branch_currents: Vec<Complex64>,
}

impl PacSidebandData {
    /// Create new sideband data
    pub fn new(
        sideband: i32,
        frequency_offset: Value,
        fundamental: Value,
        num_nodes: usize,
        num_branches: usize,
    ) -> Self {
        Self {
            sideband,
            frequency_offset,
            absolute_frequency: (sideband as Value) * fundamental + frequency_offset,
            node_voltages: vec![Complex64::new(0.0, 0.0); num_nodes],
            branch_currents: vec![Complex64::new(0.0, 0.0); num_branches],
        }
    }

    /// Get voltage magnitude at a node
    pub fn voltage_magnitude(&self, node: usize) -> Value {
        self.node_voltages
            .get(node)
            .map(|v| v.norm())
            .unwrap_or(0.0)
    }

    /// Get voltage phase at a node (radians)
    pub fn voltage_phase(&self, node: usize) -> Value {
        self.node_voltages.get(node).map(|v| v.arg()).unwrap_or(0.0)
    }

    /// Get voltage in dB relative to 1V
    pub fn voltage_db(&self, node: usize) -> Value {
        let mag = self.voltage_magnitude(node);
        if mag > 0.0 {
            20.0 * mag.log10()
        } else {
            f64::NEG_INFINITY
        }
    }

    /// Set voltage for a node
    pub fn set_voltage(&mut self, node: usize, voltage: Complex64) {
        if node < self.node_voltages.len() {
            self.node_voltages[node] = voltage;
        }
    }

    /// Set current for a branch
    pub fn set_current(&mut self, branch: usize, current: Complex64) {
        if branch < self.branch_currents.len() {
            self.branch_currents[branch] = current;
        }
    }
}

//=============================================================================
// PAC Result
//=============================================================================

/// Complete result of PAC analysis
///
/// Contains the full conversion matrix and per-node voltage spectra at all
/// analyzed frequency points and sidebands.
#[derive(Debug, Clone)]
pub struct PacResult {
    /// Fundamental frequency from PSS (Hz)
    pub fundamental_frequency: Value,

    /// Sweep frequency points (offset from fundamental)
    pub frequencies: Vec<Value>,

    /// Sideband range analyzed
    pub sideband_min: i32,
    pub sideband_max: i32,

    /// Node names for voltage lookup
    pub node_names: Vec<String>,

    /// Branch names for current lookup
    pub branch_names: Vec<String>,

    /// Full conversion matrix
    pub conversion_matrix: ConversionMatrix,

    /// Per-frequency, per-sideband voltage data
    /// Access: sideband_data[freq_idx][sideband_idx offset from sideband_min]
    sideband_data: Vec<Vec<PacSidebandData>>,

    /// Name of input source used for excitation
    pub input_source: Option<String>,

    /// Primary output node for convenience methods
    pub output_node: Option<String>,

    /// Number of Newton iterations for convergence
    pub iterations: usize,

    /// Final residual norm
    pub residual: Value,
}

impl PacResult {
    /// Create a new PAC result
    pub fn new(
        fundamental_frequency: Value,
        frequencies: Vec<Value>,
        sideband_min: i32,
        sideband_max: i32,
        node_names: Vec<String>,
        branch_names: Vec<String>,
    ) -> Self {
        let num_nodes = node_names.len();
        let num_branches = branch_names.len();

        // Initialize sideband data for each frequency point
        let sideband_data: Vec<Vec<PacSidebandData>> = frequencies
            .iter()
            .map(|&freq_offset| {
                (sideband_min..=sideband_max)
                    .map(|sb| {
                        PacSidebandData::new(
                            sb,
                            freq_offset,
                            fundamental_frequency,
                            num_nodes,
                            num_branches,
                        )
                    })
                    .collect()
            })
            .collect();

        let conversion_matrix = ConversionMatrix::new(
            fundamental_frequency,
            sideband_min,
            sideband_max,
            frequencies.clone(),
        );

        Self {
            fundamental_frequency,
            frequencies,
            sideband_min,
            sideband_max,
            node_names,
            branch_names,
            conversion_matrix,
            sideband_data,
            input_source: None,
            output_node: None,
            iterations: 0,
            residual: 0.0,
        }
    }

    /// Get the number of frequency points
    pub fn num_frequencies(&self) -> usize {
        self.frequencies.len()
    }

    /// Get the number of sidebands
    pub fn num_sidebands(&self) -> usize {
        (self.sideband_max - self.sideband_min + 1) as usize
    }

    /// Get sideband indices as vector
    pub fn sideband_indices(&self) -> Vec<i32> {
        (self.sideband_min..=self.sideband_max).collect()
    }

    /// Get node index by name (case-insensitive)
    pub fn node_index(&self, name: &str) -> Option<usize> {
        let upper = name.to_uppercase();
        self.node_names
            .iter()
            .position(|n| n.to_uppercase() == upper)
    }

    /// Internal: convert sideband to array index
    fn sideband_to_index(&self, sideband: i32) -> Option<usize> {
        if sideband >= self.sideband_min && sideband <= self.sideband_max {
            Some((sideband - self.sideband_min) as usize)
        } else {
            None
        }
    }

    /// Get sideband data for a specific frequency and sideband
    pub fn get_sideband_data(&self, freq_idx: usize, sideband: i32) -> Option<&PacSidebandData> {
        let sb_idx = self.sideband_to_index(sideband)?;
        self.sideband_data.get(freq_idx)?.get(sb_idx)
    }

    /// Get mutable sideband data
    pub fn get_sideband_data_mut(
        &mut self,
        freq_idx: usize,
        sideband: i32,
    ) -> Option<&mut PacSidebandData> {
        let sb_idx = self.sideband_to_index(sideband)?;
        self.sideband_data.get_mut(freq_idx)?.get_mut(sb_idx)
    }

    /// Get voltage at a node, frequency, and sideband
    pub fn voltage(&self, node: usize, freq_idx: usize, sideband: i32) -> Complex64 {
        self.get_sideband_data(freq_idx, sideband)
            .and_then(|sd| sd.node_voltages.get(node).copied())
            .unwrap_or(Complex64::new(0.0, 0.0))
    }

    /// Get voltage by node name
    pub fn voltage_by_name(&self, node_name: &str, freq_idx: usize, sideband: i32) -> Complex64 {
        self.node_index(node_name)
            .map(|idx| self.voltage(idx, freq_idx, sideband))
            .unwrap_or(Complex64::new(0.0, 0.0))
    }

    /// Get conversion gain from input sideband to output sideband
    ///
    /// This is the primary result for mixer analysis.
    pub fn conversion_gain(
        &self,
        input_sideband: i32,
        output_sideband: i32,
        freq_idx: usize,
    ) -> Complex64 {
        self.conversion_matrix
            .get(freq_idx, output_sideband, input_sideband)
    }

    /// Get conversion gain in dB
    pub fn conversion_gain_db(
        &self,
        input_sideband: i32,
        output_sideband: i32,
        freq_idx: usize,
    ) -> Value {
        let gain = self.conversion_gain(input_sideband, output_sideband, freq_idx);
        let mag = gain.norm();
        if mag > 0.0 {
            20.0 * mag.log10()
        } else {
            f64::NEG_INFINITY
        }
    }

    /// Get transfer function from one sideband to another across all frequencies
    pub fn get_transfer(&self, input_sideband: i32, output_sideband: i32) -> Vec<SidebandTransfer> {
        self.conversion_matrix
            .get_transfer(input_sideband, output_sideband)
    }

    /// Get image rejection ratio in dB at a frequency point
    pub fn image_rejection_db(&self, freq_idx: usize) -> Value {
        self.conversion_matrix.image_rejection_db(freq_idx)
    }

    /// Extract voltage spectrum at a node (all sidebands) for a frequency
    pub fn voltage_spectrum(&self, node: usize, freq_idx: usize) -> HashMap<i32, Complex64> {
        let mut spectrum = HashMap::new();
        for sb in self.sideband_min..=self.sideband_max {
            let v = self.voltage(node, freq_idx, sb);
            spectrum.insert(sb, v);
        }
        spectrum
    }

    /// Get magnitude vs frequency at a specific node and sideband
    pub fn magnitude_vs_frequency(&self, node: usize, sideband: i32) -> Vec<(Value, Value)> {
        self.frequencies
            .iter()
            .enumerate()
            .map(|(idx, &freq)| {
                let mag = self.voltage(node, idx, sideband).norm();
                (freq, mag)
            })
            .collect()
    }

    /// Get magnitude in dB vs frequency
    pub fn magnitude_db_vs_frequency(&self, node: usize, sideband: i32) -> Vec<(Value, Value)> {
        self.magnitude_vs_frequency(node, sideband)
            .into_iter()
            .map(|(f, mag)| {
                let db = if mag > 0.0 {
                    20.0 * mag.log10()
                } else {
                    f64::NEG_INFINITY
                };
                (f, db)
            })
            .collect()
    }

    /// Get phase vs frequency at a specific node and sideband (radians)
    pub fn phase_vs_frequency(&self, node: usize, sideband: i32) -> Vec<(Value, Value)> {
        self.frequencies
            .iter()
            .enumerate()
            .map(|(idx, &freq)| {
                let phase = self.voltage(node, idx, sideband).arg();
                (freq, phase)
            })
            .collect()
    }

    /// Set the input source name
    pub fn set_input_source(&mut self, name: &str) {
        self.input_source = Some(name.to_uppercase());
    }

    /// Set the output node name
    pub fn set_output_node(&mut self, name: &str) {
        self.output_node = Some(name.to_uppercase());
    }

    /// Set convergence info
    pub fn set_convergence_info(&mut self, iterations: usize, residual: Value) {
        self.iterations = iterations;
        self.residual = residual;
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_result() -> PacResult {
        let node_names = vec!["0".to_string(), "IN".to_string(), "OUT".to_string()];
        let branch_names = vec!["V1".to_string()];
        let frequencies = vec![1e6, 2e6, 3e6];

        PacResult::new(
            1e9, // 1 GHz fundamental
            frequencies,
            -2, // sideband min
            2,  // sideband max
            node_names,
            branch_names,
        )
    }

    #[test]
    fn test_pac_result_creation() {
        let result = create_test_result();

        assert!((result.fundamental_frequency - 1e9).abs() < 1.0);
        assert_eq!(result.num_frequencies(), 3);
        assert_eq!(result.num_sidebands(), 5);
        assert_eq!(result.sideband_min, -2);
        assert_eq!(result.sideband_max, 2);
    }

    #[test]
    fn test_node_index_lookup() {
        let result = create_test_result();

        assert_eq!(result.node_index("IN"), Some(1));
        assert_eq!(result.node_index("in"), Some(1)); // Case insensitive
        assert_eq!(result.node_index("OUT"), Some(2));
        assert_eq!(result.node_index("NONEXISTENT"), None);
    }

    #[test]
    fn test_sideband_indices() {
        let result = create_test_result();
        let indices = result.sideband_indices();
        assert_eq!(indices, vec![-2, -1, 0, 1, 2]);
    }

    #[test]
    fn test_sideband_data_creation() {
        let data = PacSidebandData::new(1, 1e6, 1e9, 3, 1);

        assert_eq!(data.sideband, 1);
        assert!((data.frequency_offset - 1e6).abs() < 1.0);
        assert!((data.absolute_frequency - 1.001e9).abs() < 1.0);
        assert_eq!(data.node_voltages.len(), 3);
    }

    #[test]
    fn test_sideband_data_voltage_access() {
        let mut data = PacSidebandData::new(0, 1e6, 1e9, 3, 1);

        data.set_voltage(1, Complex64::new(1.0, 0.5));

        assert!((data.voltage_magnitude(1) - 1.118).abs() < 0.01);
        assert!((data.voltage_phase(1) - 0.4636).abs() < 0.01);
    }

    #[test]
    fn test_sideband_data_voltage_db() {
        let mut data = PacSidebandData::new(0, 1e6, 1e9, 3, 1);

        data.set_voltage(1, Complex64::new(0.1, 0.0)); // -20 dB

        assert!((data.voltage_db(1) - (-20.0)).abs() < 0.01);
    }

    #[test]
    fn test_pac_result_voltage_access() {
        let mut result = create_test_result();

        // Set a voltage
        if let Some(sb_data) = result.get_sideband_data_mut(0, 1) {
            sb_data.set_voltage(2, Complex64::new(0.5, 0.5));
        }

        let v = result.voltage(2, 0, 1);
        assert!((v.re - 0.5).abs() < 1e-10);
        assert!((v.im - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_voltage_by_name() {
        let mut result = create_test_result();

        if let Some(sb_data) = result.get_sideband_data_mut(0, 0) {
            sb_data.set_voltage(2, Complex64::new(1.0, 0.0));
        }

        let v = result.voltage_by_name("OUT", 0, 0);
        assert!((v.re - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_conversion_gain() {
        let mut result = create_test_result();

        // Set conversion matrix element (RF->IF)
        result
            .conversion_matrix
            .set(0, 0, 1, Complex64::new(0.8, 0.0));

        let gain = result.conversion_gain(1, 0, 0);
        assert!((gain.re - 0.8).abs() < 1e-10);

        let gain_db = result.conversion_gain_db(1, 0, 0);
        assert!((gain_db - (-1.938)).abs() < 0.01); // 20*log10(0.8) ≈ -1.938
    }

    #[test]
    fn test_voltage_spectrum() {
        let mut result = create_test_result();

        // Set voltages at different sidebands
        for sb in -2..=2 {
            if let Some(sb_data) = result.get_sideband_data_mut(0, sb) {
                sb_data.set_voltage(1, Complex64::new(sb as f64 * 0.1, 0.0));
            }
        }

        let spectrum = result.voltage_spectrum(1, 0);
        assert_eq!(spectrum.len(), 5);
        assert!((spectrum[&1].re - 0.1).abs() < 1e-10);
        assert!((spectrum[&(-1)].re - (-0.1)).abs() < 1e-10);
    }

    #[test]
    fn test_magnitude_vs_frequency() {
        let mut result = create_test_result();

        // Set increasing magnitude at each frequency
        for (idx, _) in result.frequencies.clone().iter().enumerate() {
            if let Some(sb_data) = result.get_sideband_data_mut(idx, 0) {
                sb_data.set_voltage(1, Complex64::new((idx + 1) as f64 * 0.1, 0.0));
            }
        }

        let mag_vs_freq = result.magnitude_vs_frequency(1, 0);
        assert_eq!(mag_vs_freq.len(), 3);
        assert!((mag_vs_freq[0].1 - 0.1).abs() < 1e-10);
        assert!((mag_vs_freq[1].1 - 0.2).abs() < 1e-10);
        assert!((mag_vs_freq[2].1 - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_phase_vs_frequency() {
        let mut result = create_test_result();

        // Set 90 degree phase
        if let Some(sb_data) = result.get_sideband_data_mut(0, 0) {
            sb_data.set_voltage(1, Complex64::new(0.0, 1.0));
        }

        let phase_vs_freq = result.phase_vs_frequency(1, 0);
        assert!((phase_vs_freq[0].1 - std::f64::consts::PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_set_convergence_info() {
        let mut result = create_test_result();
        result.set_convergence_info(15, 1e-9);

        assert_eq!(result.iterations, 15);
        assert!((result.residual - 1e-9).abs() < 1e-15);
    }

    #[test]
    fn test_set_input_output() {
        let mut result = create_test_result();
        result.set_input_source("VRF");
        result.set_output_node("VOUT");

        assert_eq!(result.input_source, Some("VRF".to_string()));
        assert_eq!(result.output_node, Some("VOUT".to_string()));
    }

    #[test]
    fn test_image_rejection() {
        let mut result = create_test_result();

        result
            .conversion_matrix
            .set(0, 0, 1, Complex64::new(1.0, 0.0));
        result
            .conversion_matrix
            .set(0, 0, -1, Complex64::new(0.1, 0.0));

        let irr = result.image_rejection_db(0);
        assert!((irr - 20.0).abs() < 0.01); // 20 dB
    }

    #[test]
    fn test_out_of_bounds_access() {
        let result = create_test_result();

        // Out of range sideband
        let v = result.voltage(1, 0, 10);
        assert!((v.norm()).abs() < 1e-10);

        // Out of range freq
        let v2 = result.voltage(1, 100, 0);
        assert!((v2.norm()).abs() < 1e-10);
    }

    #[test]
    fn test_get_transfer() {
        let mut result = create_test_result();

        // Set transfer at each frequency
        for i in 0..3 {
            result
                .conversion_matrix
                .set(i, 0, 1, Complex64::new((i + 1) as f64 * 0.1, 0.0));
        }

        let transfers = result.get_transfer(1, 0);
        assert_eq!(transfers.len(), 3);
        assert!((transfers[0].magnitude() - 0.1).abs() < 1e-10);
        assert!((transfers[2].magnitude() - 0.3).abs() < 1e-10);
    }
}

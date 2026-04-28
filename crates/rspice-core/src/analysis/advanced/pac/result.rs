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


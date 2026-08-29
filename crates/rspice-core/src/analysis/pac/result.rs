//! PAC Analysis Results
//!
//! Contains result types for PAC analysis including per-node voltage spectra,
//! conversion matrices, and helper methods for extracting RF metrics.

use super::conversion_matrix::{
    ConversionMatrix, SidebandTransfer, complex_log10_magnitude, complex_magnitude,
};
use super::solver::PacError;
use crate::{Complex64, Value};

fn try_zeroed_complex_values(count: usize, context: &str) -> Result<Vec<Complex64>, PacError> {
    let mut values = Vec::new();
    values.try_reserve_exact(count).map_err(|error| {
        PacError::AllocationFailed(format!(
            "cannot reserve {count} complex values for {context}: {error}"
        ))
    })?;
    values.resize(count, Complex64::new(0.0, 0.0));
    Ok(values)
}

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
    ) -> Result<Self, PacError> {
        if !frequency_offset.is_finite() || frequency_offset < 0.0 {
            return Err(PacError::InvalidResult(format!(
                "frequency offset must be finite and non-negative, got {frequency_offset}"
            )));
        }
        if !fundamental.is_finite() || fundamental <= 0.0 {
            return Err(PacError::InvalidResult(format!(
                "fundamental frequency must be finite and positive, got {fundamental}"
            )));
        }
        let absolute_frequency = (sideband as Value).mul_add(fundamental, frequency_offset);
        if !absolute_frequency.is_finite() {
            return Err(PacError::InvalidResult(format!(
                "sideband {sideband} and offset {frequency_offset} Hz produce a non-representable absolute frequency"
            )));
        }
        Ok(Self {
            sideband,
            frequency_offset,
            absolute_frequency,
            node_voltages: try_zeroed_complex_values(num_nodes, "PAC node voltages")?,
            branch_currents: try_zeroed_complex_values(num_branches, "PAC branch currents")?,
        })
    }

    /// Get voltage magnitude at a node
    pub fn voltage_magnitude(&self, node: usize) -> Result<Value, PacError> {
        let voltage = self.node_voltages.get(node).copied().ok_or_else(|| {
            PacError::InvalidResult(format!(
                "node index {node} is outside this {}-node sideband record",
                self.node_voltages.len()
            ))
        })?;
        complex_magnitude(voltage, "PAC sideband voltage")
    }

    /// Get voltage phase at a node (radians)
    pub fn voltage_phase(&self, node: usize) -> Result<Value, PacError> {
        let voltage = self.node_voltages.get(node).copied().ok_or_else(|| {
            PacError::InvalidResult(format!(
                "node index {node} is outside this {}-node sideband record",
                self.node_voltages.len()
            ))
        })?;
        if !voltage.re.is_finite() || !voltage.im.is_finite() {
            return Err(PacError::InvalidResult(format!(
                "PAC sideband voltage has a non-finite complex value ({:+.6e}{:+.6e}j)",
                voltage.re, voltage.im
            )));
        }
        Ok(voltage.arg())
    }

    /// Get voltage in dB relative to 1V
    pub fn voltage_db(&self, node: usize) -> Result<Value, PacError> {
        let voltage = *self.node_voltages.get(node).ok_or_else(|| {
            PacError::InvalidResult(format!(
                "node index {node} is outside this {}-node sideband record",
                self.node_voltages.len()
            ))
        })?;
        Ok(20.0 * complex_log10_magnitude(voltage, "PAC sideband voltage")?)
    }

    /// Set voltage for a node
    pub fn set_voltage(&mut self, node: usize, voltage: Complex64) -> Result<(), PacError> {
        if !voltage.re.is_finite() || !voltage.im.is_finite() {
            return Err(PacError::InvalidResult(format!(
                "node voltage {node} is non-finite ({:+.6e}{:+.6e}j)",
                voltage.re, voltage.im
            )));
        }
        let node_count = self.node_voltages.len();
        let slot = self.node_voltages.get_mut(node).ok_or_else(|| {
            PacError::InvalidResult(format!(
                "node index {node} is outside this {node_count}-node sideband record"
            ))
        })?;
        *slot = voltage;
        Ok(())
    }

    /// Set current for a branch
    pub fn set_current(&mut self, branch: usize, current: Complex64) -> Result<(), PacError> {
        if !current.re.is_finite() || !current.im.is_finite() {
            return Err(PacError::InvalidResult(format!(
                "branch current {branch} is non-finite ({:+.6e}{:+.6e}j)",
                current.re, current.im
            )));
        }
        let branch_count = self.branch_currents.len();
        let slot = self.branch_currents.get_mut(branch).ok_or_else(|| {
            PacError::InvalidResult(format!(
                "branch index {branch} is outside this {branch_count}-branch sideband record"
            ))
        })?;
        *slot = current;
        Ok(())
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
    ) -> Result<Self, PacError> {
        Self::new_with_conversion_storage(
            fundamental_frequency,
            frequencies,
            sideband_min,
            sideband_max,
            node_names,
            branch_names,
            true,
        )
    }

    /// Create a PAC result whose per-node spectra are retained but whose
    /// output conversion matrix is deliberately unavailable.
    ///
    /// This is the result shape produced by analyses that do not request an
    /// output node. Conversion-metric accessors return [`PacError`] instead
    /// of fabricating zero-valued transfer data.
    pub(crate) fn new_without_conversion_matrix(
        fundamental_frequency: Value,
        frequencies: Vec<Value>,
        sideband_min: i32,
        sideband_max: i32,
        node_names: Vec<String>,
        branch_names: Vec<String>,
    ) -> Result<Self, PacError> {
        Self::new_with_conversion_storage(
            fundamental_frequency,
            frequencies,
            sideband_min,
            sideband_max,
            node_names,
            branch_names,
            false,
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn new_with_conversion_storage(
        fundamental_frequency: Value,
        frequencies: Vec<Value>,
        sideband_min: i32,
        sideband_max: i32,
        node_names: Vec<String>,
        branch_names: Vec<String>,
        materialize_conversion: bool,
    ) -> Result<Self, PacError> {
        let num_nodes = node_names.len();
        let num_branches = branch_names.len();

        let mut conversion_frequencies = Vec::new();
        conversion_frequencies
            .try_reserve_exact(frequencies.len())
            .map_err(|error| {
                PacError::AllocationFailed(format!(
                    "cannot reserve {} conversion frequency values: {error}",
                    frequencies.len()
                ))
            })?;
        conversion_frequencies.extend_from_slice(&frequencies);
        let conversion_matrix = if materialize_conversion {
            ConversionMatrix::new(
                fundamental_frequency,
                sideband_min,
                sideband_max,
                conversion_frequencies,
            )?
        } else {
            ConversionMatrix::without_elements(
                fundamental_frequency,
                sideband_min,
                sideband_max,
                conversion_frequencies,
            )?
        };
        let num_sidebands = conversion_matrix.num_sidebands();

        let mut sideband_data = Vec::new();
        sideband_data
            .try_reserve_exact(frequencies.len())
            .map_err(|error| {
                PacError::AllocationFailed(format!(
                    "cannot reserve {} PAC frequency rows: {error}",
                    frequencies.len()
                ))
            })?;
        for &frequency_offset in &frequencies {
            let mut per_frequency = Vec::new();
            per_frequency
                .try_reserve_exact(num_sidebands)
                .map_err(|error| {
                    PacError::AllocationFailed(format!(
                        "cannot reserve {num_sidebands} PAC sideband records: {error}"
                    ))
                })?;
            for offset in 0..num_sidebands {
                let sideband = i64::from(sideband_min)
                    .checked_add(i64::try_from(offset).map_err(|_| {
                        PacError::InvalidResult(
                            "sideband offset exceeds the i64 representation".to_string(),
                        )
                    })?)
                    .and_then(|value| i32::try_from(value).ok())
                    .ok_or_else(|| {
                        PacError::InvalidResult(
                            "sideband index exceeds the i32 representation".to_string(),
                        )
                    })?;
                per_frequency.push(PacSidebandData::new(
                    sideband,
                    frequency_offset,
                    fundamental_frequency,
                    num_nodes,
                    num_branches,
                )?);
            }
            sideband_data.push(per_frequency);
        }

        Ok(Self {
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
        })
    }

    /// Get the number of frequency points
    pub fn num_frequencies(&self) -> usize {
        self.frequencies.len()
    }

    /// Get the number of sidebands
    pub fn num_sidebands(&self) -> usize {
        self.conversion_matrix.num_sidebands()
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
            usize::try_from(i64::from(sideband) - i64::from(self.sideband_min)).ok()
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
    pub fn voltage(
        &self,
        node: usize,
        freq_idx: usize,
        sideband: i32,
    ) -> Result<Complex64, PacError> {
        let data = self
            .get_sideband_data(freq_idx, sideband)
            .ok_or_else(|| {
                PacError::InvalidResult(format!(
                    "PAC voltage coordinate (frequency {freq_idx}, sideband {sideband}) is outside the result axes"
                ))
            })?;
        let voltage = data.node_voltages.get(node).copied().ok_or_else(|| {
            PacError::InvalidResult(format!(
                "node index {node} is outside this {}-node PAC result",
                self.node_names.len()
            ))
        })?;
        if !voltage.re.is_finite() || !voltage.im.is_finite() {
            return Err(PacError::InvalidResult(format!(
                "PAC voltage coordinate (frequency {freq_idx}, sideband {sideband}, node {node}) is non-finite ({:+.6e}{:+.6e}j)",
                voltage.re, voltage.im
            )));
        }
        Ok(voltage)
    }

    /// Get conversion gain from input sideband to output sideband
    ///
    /// This is the primary result for mixer analysis.
    pub fn conversion_gain(
        &self,
        input_sideband: i32,
        output_sideband: i32,
        freq_idx: usize,
    ) -> Result<Complex64, PacError> {
        self.conversion_matrix
            .get(freq_idx, output_sideband, input_sideband)
    }

    /// Get conversion gain in dB
    pub fn conversion_gain_db(
        &self,
        input_sideband: i32,
        output_sideband: i32,
        freq_idx: usize,
    ) -> Result<Value, PacError> {
        let gain = self.conversion_gain(input_sideband, output_sideband, freq_idx)?;
        Ok(20.0 * complex_log10_magnitude(gain, "PAC conversion gain")?)
    }

    /// Get transfer function from one sideband to another across all frequencies
    pub fn get_transfer(
        &self,
        input_sideband: i32,
        output_sideband: i32,
    ) -> Result<Vec<SidebandTransfer>, PacError> {
        self.conversion_matrix
            .get_transfer(input_sideband, output_sideband)
    }

    /// Get image rejection ratio in dB at a frequency point
    pub fn image_rejection_db(&self, freq_idx: usize) -> Result<Value, PacError> {
        self.conversion_matrix.image_rejection_db(freq_idx)
    }

    /// Get magnitude vs frequency at a specific node and sideband
    pub fn magnitude_vs_frequency(
        &self,
        node: usize,
        sideband: i32,
    ) -> Result<Vec<(Value, Value)>, PacError> {
        let mut values = Vec::new();
        values
            .try_reserve_exact(self.frequencies.len())
            .map_err(|error| {
                PacError::AllocationFailed(format!(
                    "cannot reserve {} PAC magnitude points: {error}",
                    self.frequencies.len()
                ))
            })?;
        for (idx, &frequency) in self.frequencies.iter().enumerate() {
            values.push((
                frequency,
                complex_magnitude(self.voltage(node, idx, sideband)?, "PAC voltage spectrum")?,
            ));
        }
        Ok(values)
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

    #[test]
    fn result_without_output_does_not_materialize_conversion_storage() {
        let result = PacResult::new_without_conversion_matrix(
            1.0e6,
            vec![1.0e3],
            -1,
            1,
            vec!["OUT".to_string()],
            Vec::new(),
        )
        .expect("small PAC result axes are valid");
        assert_eq!(result.num_sidebands(), 3);
        assert!(!result.conversion_matrix.is_materialized());
        assert!(result.conversion_gain(0, 0, 0).is_err());
        assert!(result.conversion_gain_db(0, 0, 0).is_err());
        assert!(result.get_transfer(0, 0).is_err());
        assert!(result.image_rejection_db(0).is_err());
        assert_eq!(
            result.get_sideband_data(0, -1).unwrap().absolute_frequency,
            -999_000.0
        );
    }

    #[test]
    fn sideband_data_rejects_nonrepresentable_absolute_frequency() {
        let error = PacSidebandData::new(1, Value::MAX, Value::MAX, 1, 0)
            .expect_err("non-representable absolute-frequency metadata must fail closed");
        assert!(
            error
                .to_string()
                .contains("non-representable absolute frequency"),
            "{error}"
        );
    }

    #[test]
    fn node_access_and_publication_fail_closed() {
        let mut data =
            PacSidebandData::new(0, 1.0e3, 1.0e6, 1, 1).expect("small sideband record is valid");
        assert!(data.voltage_magnitude(1).is_err());
        assert!(data.voltage_phase(1).is_err());
        assert!(data.voltage_db(1).is_err());
        assert!(data.set_voltage(1, Complex64::new(1.0, 0.0)).is_err());
        assert!(
            data.set_voltage(0, Complex64::new(Value::NAN, 0.0))
                .is_err()
        );
        assert!(data.set_current(1, Complex64::new(1.0, 0.0)).is_err());
        assert!(
            data.set_current(0, Complex64::new(0.0, Value::INFINITY))
                .is_err()
        );
        data.node_voltages[0] = Complex64::new(Value::NAN, 0.0);
        assert!(data.voltage_magnitude(0).is_err());
        assert!(data.voltage_phase(0).is_err());
        assert!(data.voltage_db(0).is_err());
        data.node_voltages[0] = Complex64::new(Value::MAX, Value::MAX);
        assert!(data.voltage_magnitude(0).is_err());
        assert!(data.voltage_db(0).is_ok());

        let mut result = PacResult::new(
            1.0e6,
            vec![1.0e3],
            -1,
            1,
            vec!["OUT".to_string()],
            Vec::new(),
        )
        .expect("small PAC result is valid");
        assert!(result.voltage(1, 0, 0).is_err());
        assert!(result.voltage(0, 1, 0).is_err());
        assert!(result.voltage(0, 0, 2).is_err());
        assert!(result.magnitude_vs_frequency(1, 0).is_err());
        assert!(result.magnitude_vs_frequency(0, 2).is_err());
        result
            .get_sideband_data_mut(0, 0)
            .expect("sideband exists")
            .node_voltages[0] = Complex64::new(Value::NAN, 0.0);
        assert!(result.voltage(0, 0, 0).is_err());
        assert!(result.magnitude_vs_frequency(0, 0).is_err());
    }

    #[test]
    fn conversion_gain_db_is_scale_safe() {
        let mut result = PacResult::new(
            1.0e6,
            vec![1.0e3],
            -1,
            1,
            vec!["OUT".to_string()],
            Vec::new(),
        )
        .expect("small PAC result is valid");
        result
            .conversion_matrix
            .set(0, 0, 1, Complex64::new(Value::MAX, Value::MAX))
            .expect("finite conversion value is accepted");
        let db = result
            .conversion_gain_db(1, 0, 0)
            .expect("maximum finite components have a representable dB gain");
        let expected = 20.0 * (Value::MAX.log10() + 0.5 * 2.0_f64.log10());
        assert!(
            (db - expected).abs() < 1.0e-12,
            "db={db}, expected={expected}"
        );
    }
}

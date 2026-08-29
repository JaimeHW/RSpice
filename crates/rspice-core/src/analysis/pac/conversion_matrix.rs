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
//! - H`[0,0]` is IF-to-IF transfer (direct feedthrough)
//! - Diagonal elements H`[k,k]` represent harmonic transfer at same sideband

use super::solver::PacError;
use crate::{Complex64, Value};

pub(super) fn complex_magnitude(value: Complex64, context: &str) -> Result<Value, PacError> {
    if !value.re.is_finite() || !value.im.is_finite() {
        return Err(PacError::InvalidResult(format!(
            "{context} has a non-finite complex value ({:+.6e}{:+.6e}j)",
            value.re, value.im
        )));
    }
    let magnitude = value.norm();
    if !magnitude.is_finite() {
        return Err(PacError::InvalidResult(format!(
            "{context} has a non-representable linear magnitude"
        )));
    }
    Ok(magnitude)
}

pub(super) fn complex_log10_magnitude(value: Complex64, context: &str) -> Result<Value, PacError> {
    if !value.re.is_finite() || !value.im.is_finite() {
        return Err(PacError::InvalidResult(format!(
            "{context} has a non-finite complex value ({:+.6e}{:+.6e}j)",
            value.re, value.im
        )));
    }
    let scale = value.re.abs().max(value.im.abs());
    if scale == 0.0 {
        return Ok(Value::NEG_INFINITY);
    }
    let re = value.re / scale;
    let im = value.im / scale;
    let log10_magnitude = scale.log10() + 0.5 * re.mul_add(re, im * im).log10();
    if !log10_magnitude.is_finite() {
        return Err(PacError::InvalidResult(format!(
            "{context} has a non-representable logarithmic magnitude"
        )));
    }
    Ok(log10_magnitude)
}

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
    pub fn magnitude(&self) -> Result<Value, PacError> {
        complex_magnitude(self.transfer, "sideband transfer")
    }

    /// Get magnitude in dB
    pub fn magnitude_db(&self) -> Result<Value, PacError> {
        Ok(20.0 * complex_log10_magnitude(self.transfer, "sideband transfer")?)
    }

    /// Get phase in radians
    pub fn phase_rad(&self) -> Result<Value, PacError> {
        if !self.transfer.re.is_finite() || !self.transfer.im.is_finite() {
            return Err(PacError::InvalidResult(format!(
                "sideband transfer has a non-finite complex value ({:+.6e}{:+.6e}j)",
                self.transfer.re, self.transfer.im
            )));
        }
        Ok(self.transfer.arg())
    }

    /// Get phase in degrees
    pub fn phase_deg(&self) -> Result<Value, PacError> {
        Ok(self.phase_rad()? * 180.0 / std::f64::consts::PI)
    }

    /// Get the absolute output frequency
    pub fn output_frequency(&self, fundamental: Value) -> Result<Value, PacError> {
        if !fundamental.is_finite() || fundamental <= 0.0 {
            return Err(PacError::InvalidResult(format!(
                "fundamental frequency must be finite and positive, got {fundamental}"
            )));
        }
        let frequency = (self.output_sideband as Value).mul_add(fundamental, self.frequency_offset);
        if !frequency.is_finite() {
            return Err(PacError::InvalidResult(format!(
                "sideband {} and offset {} Hz produce a non-representable output frequency",
                self.output_sideband, self.frequency_offset
            )));
        }
        Ok(frequency)
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

    /// Matrix elements: `[freq_index][output_sideband][input_sideband]`
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
    ) -> Result<Self, PacError> {
        Self::new_with_storage(fundamental, sideband_min, sideband_max, frequencies, true)
    }

    pub(crate) fn without_elements(
        fundamental: Value,
        sideband_min: i32,
        sideband_max: i32,
        frequencies: Vec<Value>,
    ) -> Result<Self, PacError> {
        Self::new_with_storage(fundamental, sideband_min, sideband_max, frequencies, false)
    }

    fn new_with_storage(
        fundamental: Value,
        sideband_min: i32,
        sideband_max: i32,
        frequencies: Vec<Value>,
        materialize: bool,
    ) -> Result<Self, PacError> {
        if !fundamental.is_finite() || fundamental <= 0.0 {
            return Err(PacError::InvalidResult(format!(
                "fundamental frequency must be finite and positive, got {fundamental}"
            )));
        }
        if frequencies.is_empty() {
            return Err(PacError::InvalidResult(
                "frequency axis must contain at least one point".to_string(),
            ));
        }
        if let Some((index, frequency)) = frequencies
            .iter()
            .copied()
            .enumerate()
            .find(|(_, frequency)| !frequency.is_finite() || *frequency < 0.0)
        {
            return Err(PacError::InvalidResult(format!(
                "frequency offset {index} must be finite and non-negative, got {frequency}"
            )));
        }
        let span = i64::from(sideband_max) - i64::from(sideband_min);
        if span < 0 {
            return Err(PacError::InvalidResult(format!(
                "sideband range [{sideband_min}, {sideband_max}] is empty"
            )));
        }
        let num_sidebands = usize::try_from(span + 1).map_err(|_| {
            PacError::InvalidResult("sideband count exceeds this platform".to_string())
        })?;
        for &sideband in &[sideband_min, sideband_max] {
            for (index, &offset) in frequencies.iter().enumerate() {
                let absolute = (sideband as Value).mul_add(fundamental, offset);
                if !absolute.is_finite() {
                    return Err(PacError::InvalidResult(format!(
                        "frequency offset {index} ({offset} Hz) and sideband {sideband} produce a non-representable absolute frequency"
                    )));
                }
            }
        }
        let per_frequency = num_sidebands.checked_mul(num_sidebands).ok_or_else(|| {
            PacError::InvalidResult("conversion-matrix sideband area overflows usize".to_string())
        })?;
        let num_elements = frequencies
            .len()
            .checked_mul(per_frequency)
            .ok_or_else(|| {
                PacError::InvalidResult(
                    "conversion-matrix element count overflows usize".to_string(),
                )
            })?;
        let mut elements = Vec::new();
        if materialize {
            elements.try_reserve_exact(num_elements).map_err(|error| {
                PacError::AllocationFailed(format!(
                    "cannot reserve {num_elements} complex conversion values: {error}"
                ))
            })?;
            elements.resize(num_elements, Complex64::new(0.0, 0.0));
        }

        Ok(Self {
            fundamental,
            sideband_min,
            sideband_max,
            frequencies,
            elements,
            num_sidebands,
        })
    }

    /// Get the fundamental frequency
    pub fn fundamental(&self) -> Value {
        self.fundamental
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

    /// Whether conversion elements were requested and allocated.
    ///
    /// PAC runs without an output node retain node spectra only; their
    /// conversion matrix keeps its axes but deliberately has no elements.
    pub(crate) fn is_materialized(&self) -> bool {
        !self.elements.is_empty()
    }

    /// Compute flat index from (freq_idx, out_sb, in_sb)
    fn index(&self, freq_idx: usize, out_sideband: i32, in_sideband: i32) -> Option<usize> {
        if freq_idx >= self.frequencies.len() {
            return None;
        }

        let out_idx =
            usize::try_from(i64::from(out_sideband) - i64::from(self.sideband_min)).ok()?;
        let in_idx = usize::try_from(i64::from(in_sideband) - i64::from(self.sideband_min)).ok()?;

        if out_idx >= self.num_sidebands || in_idx >= self.num_sidebands {
            return None;
        }

        let per_frequency = self.num_sidebands.checked_mul(self.num_sidebands)?;
        freq_idx
            .checked_mul(per_frequency)?
            .checked_add(out_idx.checked_mul(self.num_sidebands)?)?
            .checked_add(in_idx)
    }

    fn checked_element_index(
        &self,
        freq_idx: usize,
        output_sideband: i32,
        input_sideband: i32,
    ) -> Result<usize, PacError> {
        if !self.is_materialized() {
            return Err(PacError::InvalidResult(
                "conversion data is unavailable because PAC was run without an output".to_string(),
            ));
        }
        self.index(freq_idx, output_sideband, input_sideband)
            .ok_or_else(|| {
                PacError::InvalidResult(format!(
                    "conversion coordinate (frequency {freq_idx}, output sideband {output_sideband}, input sideband {input_sideband}) is outside the result axes"
                ))
            })
    }

    /// Set a matrix element
    pub fn set(
        &mut self,
        freq_idx: usize,
        output_sideband: i32,
        input_sideband: i32,
        value: Complex64,
    ) -> Result<(), PacError> {
        if !value.re.is_finite() || !value.im.is_finite() {
            return Err(PacError::InvalidResult(format!(
                "conversion coordinate (frequency {freq_idx}, output sideband {output_sideband}, input sideband {input_sideband}) has a non-finite transfer ({:+.6e}{:+.6e}j)",
                value.re, value.im
            )));
        }
        let index = self.checked_element_index(freq_idx, output_sideband, input_sideband)?;
        let element = self.elements.get_mut(index).ok_or_else(|| {
            PacError::InvalidResult(
                "conversion storage is inconsistent with its declared axes".to_string(),
            )
        })?;
        *element = value;
        Ok(())
    }

    /// Get a matrix element
    pub fn get(
        &self,
        freq_idx: usize,
        output_sideband: i32,
        input_sideband: i32,
    ) -> Result<Complex64, PacError> {
        let index = self.checked_element_index(freq_idx, output_sideband, input_sideband)?;
        self.elements.get(index).copied().ok_or_else(|| {
            PacError::InvalidResult(
                "conversion storage is inconsistent with its declared axes".to_string(),
            )
        })
    }

    /// Get transfer from input sideband to output sideband as SidebandTransfer vec
    pub fn get_transfer(
        &self,
        input_sideband: i32,
        output_sideband: i32,
    ) -> Result<Vec<SidebandTransfer>, PacError> {
        let mut transfers = Vec::new();
        transfers
            .try_reserve_exact(self.frequencies.len())
            .map_err(|error| {
                PacError::AllocationFailed(format!(
                    "cannot reserve {} sideband transfers: {error}",
                    self.frequencies.len()
                ))
            })?;
        for (idx, &freq) in self.frequencies.iter().enumerate() {
            transfers.push(SidebandTransfer::new(
                input_sideband,
                output_sideband,
                freq,
                self.get(idx, output_sideband, input_sideband)?,
            ));
        }
        Ok(transfers)
    }

    /// Get conversion gain (typically RF to IF for mixers)
    ///
    /// For a down-converting mixer: input at sideband +1 (RF = LO + Δf),
    /// output at sideband 0 (IF = Δf)
    pub fn conversion_gain(&self, freq_idx: usize) -> Result<Complex64, PacError> {
        // Standard convention: RF at sideband 1, IF at sideband 0
        self.get(freq_idx, 0, 1)
    }

    /// Get image rejection (RF at sideband -1 to IF at sideband 0)
    pub fn image_transfer(&self, freq_idx: usize) -> Result<Complex64, PacError> {
        self.get(freq_idx, 0, -1)
    }

    /// Calculate image rejection ratio (IRR) in dB at a frequency point
    pub fn image_rejection_db(&self, freq_idx: usize) -> Result<Value, PacError> {
        let signal =
            complex_log10_magnitude(self.conversion_gain(freq_idx)?, "PAC signal transfer")?;
        let image = complex_log10_magnitude(self.image_transfer(freq_idx)?, "PAC image transfer")?;

        if signal == Value::NEG_INFINITY && image == Value::NEG_INFINITY {
            Err(PacError::InvalidResult(format!(
                "image rejection is undefined at frequency index {freq_idx} because both signal and image transfers are zero"
            )))
        } else if image == Value::NEG_INFINITY {
            Ok(f64::INFINITY)
        } else {
            let rejection = 20.0 * (signal - image);
            if rejection.is_nan() {
                return Err(PacError::InvalidResult(format!(
                    "image rejection is non-representable at frequency index {freq_idx}"
                )));
            }
            Ok(rejection)
        }
    }

    /// Get the diagonal elements (same-sideband transfer) across all frequencies
    pub fn diagonal(&self, sideband: i32) -> Result<Vec<Complex64>, PacError> {
        let mut diagonal = Vec::new();
        diagonal
            .try_reserve_exact(self.frequencies.len())
            .map_err(|error| {
                PacError::AllocationFailed(format!(
                    "cannot reserve {} diagonal conversion values: {error}",
                    self.frequencies.len()
                ))
            })?;
        for idx in 0..self.frequencies.len() {
            diagonal.push(self.get(idx, sideband, sideband)?);
        }
        Ok(diagonal)
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

    #[test]
    fn constructor_rejects_overflowing_axes_and_derived_frequencies() {
        let range_error = ConversionMatrix::new(1.0, i32::MIN, i32::MAX, vec![1.0])
            .expect_err("an unaddressable conversion matrix must fail before allocation");
        assert!(
            range_error.to_string().contains("sideband area")
                || range_error.to_string().contains("sideband count"),
            "{range_error}"
        );

        let frequency_error = ConversionMatrix::new(Value::MAX, 0, 1, vec![Value::MAX])
            .expect_err("non-representable absolute frequencies must fail closed");
        assert!(
            frequency_error
                .to_string()
                .contains("non-representable absolute frequency"),
            "{frequency_error}"
        );
    }

    #[test]
    fn unmaterialized_matrix_retains_safe_axes_without_elements() {
        let mut matrix = ConversionMatrix::without_elements(1.0e6, -1, 1, vec![1.0e3])
            .expect("small axes are valid");
        assert!(!matrix.is_materialized());
        let set_error = matrix
            .set(0, 0, 0, Complex64::new(7.0, -2.0))
            .expect_err("unmaterialized conversion data must reject writes");
        assert!(set_error.to_string().contains("unavailable"), "{set_error}");
        let get_error = matrix
            .get(0, 0, 0)
            .expect_err("unmaterialized conversion data must reject reads");
        assert!(get_error.to_string().contains("unavailable"), "{get_error}");
        assert!(matrix.conversion_gain(0).is_err());
        assert!(matrix.image_transfer(0).is_err());
        assert!(matrix.image_rejection_db(0).is_err());
        assert!(matrix.diagonal(0).is_err());
        assert!(matrix.get_transfer(0, 0).is_err());
    }

    #[test]
    fn matrix_rejects_invalid_coordinates_and_nonfinite_values() {
        let mut matrix =
            ConversionMatrix::new(1.0e6, -1, 1, vec![1.0e3]).expect("small axes are valid");
        assert!(
            matrix
                .set(0, 0, 0, Complex64::new(Value::INFINITY, 0.0))
                .is_err()
        );
        assert!(matrix.set(1, 0, 0, Complex64::new(1.0, 0.0)).is_err());
        assert!(matrix.get(0, i32::MAX, i32::MIN).is_err());
    }

    #[test]
    fn logarithmic_metrics_are_scale_safe_for_maximum_finite_components() {
        let maximum = Complex64::new(Value::MAX, Value::MAX);
        let expected_db = 20.0 * (Value::MAX.log10() + 0.5 * 2.0_f64.log10());
        let transfer = SidebandTransfer::new(1, 0, 1.0e3, maximum);
        assert!(
            transfer.magnitude().is_err(),
            "an unrepresentable linear magnitude must fail closed"
        );
        let transfer_db = transfer
            .magnitude_db()
            .expect("finite components have a representable dB magnitude");
        assert!((transfer_db - expected_db).abs() < 1.0e-12);

        let mut matrix =
            ConversionMatrix::new(1.0e6, -1, 1, vec![1.0e3]).expect("small axes are valid");
        matrix
            .set(0, 0, 1, maximum)
            .expect("finite signal transfer is accepted");
        matrix
            .set(0, 0, -1, maximum)
            .expect("finite image transfer is accepted");
        assert_eq!(
            matrix
                .image_rejection_db(0)
                .expect("equal finite transfers define zero-dB rejection"),
            0.0
        );

        let invalid = SidebandTransfer::new(0, 0, 1.0e3, Complex64::new(Value::NAN, 0.0));
        assert!(invalid.magnitude().is_err());
        assert!(invalid.magnitude_db().is_err());
        assert!(invalid.phase_rad().is_err());
        assert!(invalid.phase_deg().is_err());
    }
}

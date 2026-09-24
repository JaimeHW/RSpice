//! One `.FFT` spectrum, as the transient solve that computed it returned it.
//!
//! The three coefficient columns are the engine's own `TransientFftBin`
//! columns. Magnitude and phase are deliberately not carried: the retained
//! complex waveform derives them with the same `hypot`/`atan2` the engine
//! used, and a second copy could disagree with the first.

use serde::{Deserialize, Serialize};

use crate::state::FftSpectrumEvidence;

/// A recorded spectrum plus the key that pairs it with its FFT analysis.
///
/// The three columns serialize as exact bit patterns, not decimal text: an
/// execution artifact's identity is a digest over these numbers, and a decimal
/// round trip is not guaranteed to return the bits the engine produced.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecordedFftSpectrum {
    /// The canonical `.fft` card of the request that produced this spectrum,
    /// as `FftRequest::engine_key` spells it on both ends of the pairing.
    pub request_key: String,
    pub evidence: FftSpectrumEvidence,
    /// Bin-centre frequencies, DC through Nyquist. Empty when incomplete.
    #[serde(with = "exact_f64_vec")]
    pub frequency: Vec<f64>,
    #[serde(with = "exact_f64_vec")]
    pub real: Vec<f64>,
    #[serde(with = "exact_f64_vec")]
    pub imaginary: Vec<f64>,
}

mod exact_f64_vec {
    use serde::ser::SerializeSeq;
    use serde::{Deserialize, Deserializer, Serializer, de::Error as _};

    pub(super) fn serialize<S>(values: &[f64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut sequence = serializer.serialize_seq(Some(values.len()))?;
        for value in values {
            sequence.serialize_element(&format!("{:016x}", value.to_bits()))?;
        }
        sequence.end()
    }

    pub(super) fn deserialize<'de, D>(deserializer: D) -> Result<Vec<f64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Vec::<String>::deserialize(deserializer)?
            .into_iter()
            .map(|encoded| {
                u64::from_str_radix(&encoded, 16)
                    .map(f64::from_bits)
                    .map_err(|_| D::Error::custom("invalid exact f64 bit pattern"))
            })
            .collect()
    }
}

impl RecordedFftSpectrum {
    /// Whether the three columns agree with the evidence that describes them.
    ///
    /// A complete spectrum holds one finite coefficient per one-sided bin on a
    /// strictly increasing frequency axis; an incomplete one holds none, which
    /// is the engine's own contract for a record that ran short.
    pub fn validate(&self) -> Result<(), String> {
        self.evidence.validate()?;
        if self.request_key.trim().is_empty() {
            return Err("a recorded FFT spectrum has no request key".into());
        }
        if self.real.len() != self.frequency.len() || self.imaginary.len() != self.frequency.len() {
            return Err("a recorded FFT spectrum has mismatched coefficient columns".into());
        }
        if !self.evidence.status.is_complete() {
            if !self.frequency.is_empty() {
                return Err("an incomplete FFT record cannot carry coefficients".into());
            }
            return Ok(());
        }
        if self.frequency.len() != self.evidence.bin_count() {
            return Err(format!(
                "a recorded FFT spectrum has {} coefficients for a {}-point transform",
                self.frequency.len(),
                self.evidence.point_count
            ));
        }
        if self
            .frequency
            .iter()
            .chain(&self.real)
            .chain(&self.imaginary)
            .any(|value| !value.is_finite())
        {
            return Err("a recorded FFT spectrum contains a non-finite value".into());
        }
        if self.frequency[0] != 0.0 || self.frequency.windows(2).any(|pair| pair[1] <= pair[0]) {
            return Err(
                "a recorded FFT spectrum frequency axis does not rise from DC to Nyquist".into(),
            );
        }
        Ok(())
    }

    /// How many floating-point values this spectrum retains.
    #[must_use]
    pub fn numeric_value_count(&self) -> usize {
        self.frequency
            .len()
            .saturating_add(self.real.len())
            .saturating_add(self.imaginary.len())
    }
}

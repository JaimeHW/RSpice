//! Retained evidence of one `.FFT` spectrum the transient engine computed.
//!
//! The Studio computes no transform: every number below is what
//! `rspice_core::engine::TransientFftResult` returned from inside the solve
//! that carried the `.fft` card. These are UI-owned mirrors of the engine's
//! types, so a retained result stays readable when a core type moves, and
//! [`FftSpectrumEvidence::validate`] restates the engine's own status rule by
//! calling the engine's function rather than re-deriving it.

use rspice_core::engine::{
    TransientFftHarmonic, TransientFftMetrics, TransientFftResult, TransientFftStatus,
};
use rspice_core::netlist::{FftFormat, XyceFftMode};
use serde::{Deserialize, Serialize};

/// Whether the transient retained enough accepted history for the transform.
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum FftSpectrumStatusEvidence {
    #[default]
    Complete,
    IncompleteHistory {
        available_start_s: f64,
        available_stop_s: f64,
    },
}

impl FftSpectrumStatusEvidence {
    #[must_use]
    pub const fn is_complete(self) -> bool {
        matches!(self, Self::Complete)
    }

    const fn core(self) -> TransientFftStatus {
        match self {
            Self::Complete => TransientFftStatus::Complete,
            Self::IncompleteHistory {
                available_start_s,
                available_stop_s,
            } => TransientFftStatus::IncompleteHistory {
                available_start: available_start_s,
                available_stop: available_stop_s,
            },
        }
    }
}

impl From<TransientFftStatus> for FftSpectrumStatusEvidence {
    fn from(status: TransientFftStatus) -> Self {
        match status {
            TransientFftStatus::Complete => Self::Complete,
            TransientFftStatus::IncompleteHistory {
                available_start,
                available_stop,
            } => Self::IncompleteHistory {
                available_start_s: available_start,
                available_stop_s: available_stop,
            },
        }
    }
}

/// Effective `.FFT FORMAT`, as the engine resolved it for this spectrum.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FftSpectrumFormatEvidence {
    #[default]
    Normalized,
    Unnormalized,
}

impl From<FftFormat> for FftSpectrumFormatEvidence {
    fn from(format: FftFormat) -> Self {
        match format {
            FftFormat::Normalized => Self::Normalized,
            FftFormat::Unnormalized => Self::Unnormalized,
        }
    }
}

impl FftSpectrumFormatEvidence {
    #[must_use]
    pub const fn core(self) -> FftFormat {
        match self {
            Self::Normalized => FftFormat::Normalized,
            Self::Unnormalized => FftFormat::Unnormalized,
        }
    }

    /// The keyword `.FFT FORMAT=` spells this with.
    #[must_use]
    pub const fn keyword(self) -> &'static str {
        match self {
            Self::Normalized => "NORM",
            Self::Unnormalized => "UNORM",
        }
    }
}

/// Effective `.OPTIONS FFT FFT_MODE` compatibility mode of this spectrum.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FftSpectrumModeEvidence {
    #[default]
    HspiceCompatible,
    SpectreCompatible,
}

impl From<XyceFftMode> for FftSpectrumModeEvidence {
    fn from(mode: XyceFftMode) -> Self {
        match mode {
            XyceFftMode::HspiceCompatible => Self::HspiceCompatible,
            XyceFftMode::SpectreCompatible => Self::SpectreCompatible,
        }
    }
}

impl FftSpectrumModeEvidence {
    /// How the inspector names the window convention this mode selects.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::HspiceCompatible => "HSPICE (symmetric)",
            Self::SpectreCompatible => "Spectre (periodic)",
        }
    }
}

/// One entry of the magnitude-ranked harmonic list `FFTOUT=1` requests.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FftHarmonicEvidence {
    pub rank: usize,
    pub bin: usize,
    pub frequency_hz: f64,
    pub magnitude: f64,
    pub magnitude_db: f64,
    pub phase_degrees: f64,
}

impl From<&TransientFftHarmonic> for FftHarmonicEvidence {
    fn from(harmonic: &TransientFftHarmonic) -> Self {
        Self {
            rank: harmonic.rank,
            bin: harmonic.bin,
            frequency_hz: harmonic.frequency,
            magnitude: harmonic.magnitude,
            magnitude_db: harmonic.magnitude_db,
            phase_degrees: harmonic.phase_degrees,
        }
    }
}

/// The Xyce-compatible figures `.OPTIONS FFT FFTOUT=1` requests.
///
/// Retained only when the run asked for them: the engine computes none
/// otherwise, and an invented zero would read as a measured one.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FftMetricsEvidence {
    pub fundamental_magnitude: f64,
    pub thd_ratio: f64,
    pub thd_db: f64,
    pub sndr_db: f64,
    pub enob_bits: f64,
    pub snr_db: f64,
    pub sfdr_db: f64,
    pub sfdr_spur_bin: Option<usize>,
    pub sfdr_spur_frequency_hz: Option<f64>,
    pub largest_harmonics: Vec<FftHarmonicEvidence>,
}

impl From<&TransientFftMetrics> for FftMetricsEvidence {
    fn from(metrics: &TransientFftMetrics) -> Self {
        Self {
            fundamental_magnitude: metrics.fundamental_magnitude,
            thd_ratio: metrics.thd_ratio,
            thd_db: metrics.thd_db,
            sndr_db: metrics.sndr_db,
            enob_bits: metrics.enob_bits,
            snr_db: metrics.snr_db,
            sfdr_db: metrics.sfdr_db,
            sfdr_spur_bin: metrics.sfdr_spur_bin,
            sfdr_spur_frequency_hz: metrics.sfdr_spur_frequency,
            largest_harmonics: metrics
                .largest_harmonics
                .iter()
                .map(FftHarmonicEvidence::from)
                .collect(),
        }
    }
}

impl FftMetricsEvidence {
    fn validate(&self) -> Result<(), String> {
        for value in [
            self.fundamental_magnitude,
            self.thd_ratio,
            self.thd_db,
            self.sndr_db,
            self.enob_bits,
            self.snr_db,
            self.sfdr_db,
        ] {
            if !value.is_finite() {
                return Err("FFT metric evidence contains a non-finite figure".into());
            }
        }
        if self.sfdr_spur_bin.is_none() != self.sfdr_spur_frequency_hz.is_none() {
            return Err("FFT spur evidence states a bin without its frequency".into());
        }
        if self
            .sfdr_spur_frequency_hz
            .is_some_and(|frequency| !frequency.is_finite() || frequency < 0.0)
        {
            return Err("FFT spur evidence has an invalid frequency".into());
        }
        for (position, harmonic) in self.largest_harmonics.iter().enumerate() {
            if harmonic.rank != position + 1 {
                return Err("ranked FFT harmonics are not in descending rank order".into());
            }
            if harmonic.bin == 0 {
                return Err("a ranked FFT harmonic cannot be the DC bin".into());
            }
            if !harmonic.frequency_hz.is_finite()
                || harmonic.frequency_hz < 0.0
                || !harmonic.magnitude.is_finite()
                || harmonic.magnitude < 0.0
                || !harmonic.magnitude_db.is_finite()
                || !harmonic.phase_degrees.is_finite()
            {
                return Err("a ranked FFT harmonic contains an invalid value".into());
            }
        }
        Ok(())
    }
}

/// Everything about one recorded `.FFT` spectrum except its coefficients.
///
/// The coefficients travel as an ordinary complex waveform, so this record is
/// what identifies the request, states the transform the engine performed, and
/// carries the figures no waveform can express.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FftSpectrumEvidence {
    pub status: FftSpectrumStatusEvidence,
    /// Display spelling of the scalar column the engine resolved.
    pub output: String,
    /// The engine's physical quantity class: `voltage`, `current`, `parameter`.
    pub physical_type: String,
    pub start_time_s: f64,
    pub stop_time_s: f64,
    pub sample_interval_s: f64,
    pub point_count: usize,
    /// Whether the solver was configured to land on each requested sample time.
    pub accurate_sampling: bool,
    pub format: FftSpectrumFormatEvidence,
    pub mode: FftSpectrumModeEvidence,
    /// The canonical `.FFT WINDOW=` keyword the parser retained.
    pub window: String,
    /// HSPICE `ALFA`, echoed by the engine and read by no window it implements.
    pub alpha: f64,
    pub coherent_gain: f64,
    pub frequency_resolution_hz: f64,
    pub fundamental_bin: usize,
    pub minimum_metric_bin: usize,
    pub maximum_metric_bin: usize,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metrics: Option<FftMetricsEvidence>,
}

impl From<&TransientFftResult> for FftSpectrumEvidence {
    fn from(result: &TransientFftResult) -> Self {
        Self {
            status: result.status.into(),
            output: result.output_name.clone(),
            physical_type: result.physical_type.to_owned(),
            start_time_s: result.start_time,
            stop_time_s: result.stop_time,
            sample_interval_s: result.sample_interval,
            point_count: result.point_count,
            accurate_sampling: result.accurate_sampling,
            format: result.format.into(),
            mode: result.mode.into(),
            window: result.window_name.clone(),
            alpha: result.alpha,
            coherent_gain: result.coherent_gain,
            frequency_resolution_hz: result.frequency_resolution,
            fundamental_bin: result.fundamental_bin,
            minimum_metric_bin: result.minimum_metric_bin,
            maximum_metric_bin: result.maximum_metric_bin,
            metrics: result.metrics.as_ref().map(FftMetricsEvidence::from),
        }
    }
}

impl FftSpectrumEvidence {
    /// How many one-sided coefficients a complete spectrum of this request has.
    #[must_use]
    pub const fn bin_count(&self) -> usize {
        self.point_count / 2 + 1
    }

    /// The last uniform sample time the transform needs.
    ///
    /// The finite record stops one spacing before `stop_time_s`, which is the
    /// grid `rspice_core::engine::TransientFftStatus::validate_history` checks
    /// and the boundary an incomplete history failed to reach.
    #[must_use]
    pub fn last_sample_time_s(&self) -> f64 {
        self.start_time_s + (self.point_count.saturating_sub(1) as f64) * self.sample_interval_s
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.output.trim().is_empty() {
            return Err("FFT spectrum evidence has no resolved output".into());
        }
        if !matches!(
            self.physical_type.as_str(),
            "voltage" | "current" | "parameter"
        ) {
            return Err(format!(
                "FFT spectrum evidence names an unknown physical quantity class '{}'",
                self.physical_type
            ));
        }
        if self.point_count < 4 || !self.point_count.is_power_of_two() {
            return Err(
                "FFT spectrum evidence has a transform length that is not a power of two of at \
                 least four"
                    .into(),
            );
        }
        if !self.start_time_s.is_finite()
            || self.start_time_s < 0.0
            || !self.stop_time_s.is_finite()
            || self.stop_time_s <= self.start_time_s
        {
            return Err("FFT spectrum evidence has an invalid record interval".into());
        }
        let expected_interval = (self.stop_time_s - self.start_time_s) / self.point_count as f64;
        if !self.sample_interval_s.is_finite()
            || (self.sample_interval_s - expected_interval).abs()
                > expected_interval.abs() * 1.0e-12
        {
            return Err(
                "FFT spectrum evidence sample spacing does not match its record and length".into(),
            );
        }
        let expected_resolution = 1.0 / (self.stop_time_s - self.start_time_s);
        if !self.frequency_resolution_hz.is_finite()
            || (self.frequency_resolution_hz - expected_resolution).abs()
                > expected_resolution.abs() * 1.0e-12
        {
            return Err("FFT spectrum evidence bin width does not match its record".into());
        }
        if !self.coherent_gain.is_finite() || self.coherent_gain <= 0.0 {
            return Err("FFT spectrum evidence has an invalid coherent gain".into());
        }
        if !self.alpha.is_finite() {
            return Err("FFT spectrum evidence has a non-finite ALFA".into());
        }
        if self.window.trim().is_empty() || self.window.to_ascii_uppercase() != self.window {
            return Err("FFT spectrum evidence has no canonical window keyword".into());
        }
        let nyquist_bin = self.point_count / 2;
        if self.fundamental_bin == 0
            || self.fundamental_bin > nyquist_bin
            || self.maximum_metric_bin > nyquist_bin
            || self.maximum_metric_bin < self.minimum_metric_bin
        {
            return Err("FFT spectrum evidence has inconsistent metric bins".into());
        }
        // The engine owns the incomplete-history rule; restate it by calling it.
        self.status.core().validate_history(
            self.start_time_s,
            self.stop_time_s,
            self.point_count,
        )?;
        if let Some(metrics) = &self.metrics {
            if !self.status.is_complete() {
                return Err("an incomplete FFT cannot retain metrics".into());
            }
            metrics.validate()?;
            if metrics
                .largest_harmonics
                .iter()
                .any(|harmonic| harmonic.bin > nyquist_bin)
            {
                return Err("a ranked FFT harmonic lies above the Nyquist bin".into());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) fn fixture() -> FftSpectrumEvidence {
        FftSpectrumEvidence {
            status: FftSpectrumStatusEvidence::Complete,
            output: "V(OUT)".to_owned(),
            physical_type: "voltage".to_owned(),
            start_time_s: 0.0,
            stop_time_s: 8.0e-3,
            sample_interval_s: 8.0e-3 / 256.0,
            point_count: 256,
            accurate_sampling: true,
            format: FftSpectrumFormatEvidence::Unnormalized,
            mode: FftSpectrumModeEvidence::HspiceCompatible,
            window: "RECT".to_owned(),
            alpha: 3.0,
            coherent_gain: 1.0,
            frequency_resolution_hz: 125.0,
            fundamental_bin: 1,
            minimum_metric_bin: 1,
            maximum_metric_bin: 128,
            metrics: None,
        }
    }

    #[test]
    fn recorded_fft_evidence_round_trips_and_states_its_grid() {
        let evidence = fixture();
        evidence.validate().expect("the fixture is valid evidence");
        assert_eq!(evidence.bin_count(), 129);
        assert!((evidence.last_sample_time_s() - 255.0 * (8.0e-3 / 256.0)).abs() < 1.0e-18);
        let text = serde_json::to_string(&evidence).expect("evidence serializes");
        let restored: FftSpectrumEvidence = serde_json::from_str(&text).expect("evidence restores");
        assert_eq!(evidence, restored);
    }

    #[test]
    fn recorded_fft_evidence_refuses_a_grid_the_engine_could_not_have_produced() {
        let mut broken = fixture();
        broken.point_count = 100;
        assert!(broken.validate().is_err());

        let mut broken = fixture();
        broken.sample_interval_s *= 2.0;
        assert!(broken.validate().is_err());

        let mut broken = fixture();
        broken.window = "rect".to_owned();
        assert!(broken.validate().is_err());

        let mut broken = fixture();
        broken.coherent_gain = 0.0;
        assert!(broken.validate().is_err());
    }

    #[test]
    fn an_incomplete_fft_history_keeps_the_engines_own_rule() {
        let mut incomplete = fixture();
        incomplete.status = FftSpectrumStatusEvidence::IncompleteHistory {
            available_start_s: 0.0,
            available_stop_s: 1.0e-3,
        };
        incomplete
            .validate()
            .expect("a record that truly ran short is valid evidence");
        assert!(!incomplete.status.is_complete());

        // The whole requested record was available, so the engine would have
        // called this complete; its own validator is what says so.
        incomplete.status = FftSpectrumStatusEvidence::IncompleteHistory {
            available_start_s: 0.0,
            available_stop_s: 8.0e-3,
        };
        assert!(incomplete.validate().is_err());
    }

    #[test]
    fn metrics_are_refused_on_an_incomplete_record_and_out_of_order_ranks() {
        let mut spectrum = fixture();
        spectrum.metrics = Some(FftMetricsEvidence {
            fundamental_magnitude: 1.0,
            thd_ratio: 0.01,
            thd_db: -40.0,
            sndr_db: 40.0,
            enob_bits: 6.35,
            snr_db: 41.0,
            sfdr_db: 45.0,
            sfdr_spur_bin: Some(16),
            sfdr_spur_frequency_hz: Some(2000.0),
            largest_harmonics: vec![FftHarmonicEvidence {
                rank: 2,
                bin: 8,
                frequency_hz: 1000.0,
                magnitude: 1.0,
                magnitude_db: 0.0,
                phase_degrees: -90.0,
            }],
        });
        assert!(spectrum.validate().is_err());
    }
    #[test]
    fn native_scalar_units_keep_fft_normalization_and_unknown_quantities() {
        use crate::state::{AnalysisResult, AnalysisResultPayload, AnalysisType};
        for (physical, format, target, expected) in [
            (
                "voltage",
                FftSpectrumFormatEvidence::Unnormalized,
                "mV",
                Some(250.0),
            ),
            (
                "current",
                FftSpectrumFormatEvidence::Unnormalized,
                "mA",
                Some(250.0),
            ),
            (
                "voltage",
                FftSpectrumFormatEvidence::Normalized,
                "%",
                Some(25.0),
            ),
            (
                "parameter",
                FftSpectrumFormatEvidence::Unnormalized,
                "V",
                None,
            ),
        ] {
            let mut spectrum = fixture();
            spectrum.physical_type = physical.into();
            spectrum.format = format;
            spectrum.metrics = Some(FftMetricsEvidence {
                fundamental_magnitude: 0.25,
                thd_ratio: 0.01,
                thd_db: -40.0,
                sndr_db: 40.0,
                enob_bits: 6.35,
                snr_db: 41.0,
                sfdr_db: 45.0,
                sfdr_spur_bin: None,
                sfdr_spur_frequency_hz: None,
                largest_harmonics: vec![],
            });
            spectrum.validate().unwrap();
            let mut result = AnalysisResult::new(1, AnalysisType::Fourier, "FFT")
                .with_result_payload(AnalysisResultPayload::FftSpectrum { spectrum });
            result.retain_native_scalar_units();
            let scalar = result.scalar_evidence("FFT.fundamental_magnitude");
            match expected {
                Some(expected) => {
                    assert_eq!(scalar[0].value_in_unit(target).unwrap(), Some(expected))
                }
                None => assert!(scalar[0].value_in_unit(target).is_err()),
            }
            assert_eq!(
                result.scalar_evidence("fft_enob_bits")[0]
                    .value_in_unit("bits")
                    .unwrap(),
                Some(6.35)
            );
            assert!(
                result.scalar_evidence("fft_thd_db")[0]
                    .value_in_unit("V")
                    .is_err()
            );
        }
    }
}

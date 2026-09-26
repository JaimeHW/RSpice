//! Physical units of analysis-native scalar evidence.

use super::AnalysisResultPayload;
use rspice_core::analysis::MeasurementUnit;

impl AnalysisResultPayload {
    pub fn produced_scalar_unit(&self, name: &str) -> MeasurementUnit {
        let symbol = match self {
            Self::PssFloquet { .. } => match name {
                "pss_period" => "s",
                "pss_fundamental_frequency" => "Hz",
                "pss_mode_count" => "count",
                _ => return MeasurementUnit::Unknown,
            },
            Self::Pstb { .. } => match name {
                "pstb_period" => "s",
                "pstb_fundamental_frequency" => "Hz",
                "pstb_mode_count" | "pstb_unstable_mode_count" => "count",
                "pstb_max_multiplier_magnitude" => "1",
                "pstb_min_stability_margin_db" => "dB",
                _ => return MeasurementUnit::Unknown,
            },
            Self::DcMismatch { evidence } => &evidence.output_unit,
            Self::FftSpectrum { spectrum } => match name {
                "fft_thd_db" | "fft_sndr_db" | "fft_snr_db" | "fft_sfdr_db" => "dB",
                "fft_enob_bits" => "bits",
                "fft_fundamental_magnitude" => {
                    return rspice_core::execution::transient_fft_output_unit(
                        &spectrum.physical_type,
                        crate::fft::fft_format_to_core(spectrum.format),
                    )
                    .ok()
                    .and_then(|unit| MeasurementUnit::known(&unit.symbol()).ok())
                    .unwrap_or(MeasurementUnit::Unknown);
                }
                _ => return MeasurementUnit::Unknown,
            },
            // Generic scalar results have no producer quantity metadata.
            _ => return MeasurementUnit::Unknown,
        };
        MeasurementUnit::known(symbol).unwrap_or(MeasurementUnit::Unknown)
    }
}

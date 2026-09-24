//! Physical units for explicit study selections, taken from their producers.
use super::*;
use rspice_core::analysis::MeasurementUnit;

pub(super) fn unit(symbol: &str) -> MeasurementUnit {
    MeasurementUnit::known(symbol).unwrap_or(MeasurementUnit::Unknown)
}

pub(super) fn fft_unit(evidence: &crate::state::FftSpectrumEvidence) -> MeasurementUnit {
    rspice_core::execution::transient_fft_output_unit(
        &evidence.physical_type,
        evidence.format.core(),
    )
    .map_or(MeasurementUnit::Unknown, |physical| {
        unit(&physical.symbol())
    })
}

impl SimulationResult {
    pub(super) fn noise_study_unit(&self, key: &str) -> MeasurementUnit {
        let Self::Noise {
            output_unit,
            summary,
            ..
        } = self
        else {
            return MeasurementUnit::Unknown;
        };
        if key.eq_ignore_ascii_case("noise_figure_db") {
            unit("dB")
        } else if key.eq_ignore_ascii_case("input_noise") {
            summary
                .as_ref()
                .and_then(|summary| summary.input_quantity)
                .map_or(MeasurementUnit::Unknown, |quantity| {
                    unit(quantity.density_unit())
                })
        } else {
            output_unit.clone().unwrap_or(MeasurementUnit::Unknown)
        }
    }

    pub(super) fn study_scalar_unit(&self, key: &str) -> MeasurementUnit {
        let folded = key.to_ascii_lowercase();
        match self {
            Self::DcOp(op) => {
                // Match the value lookup's exact-map precedence before aliases.
                if named_value(&op.node_voltages, key).is_some() {
                    unit("V")
                } else if named_value(&op.branch_currents, key).is_some() {
                    unit("A")
                } else if parse_wrapped_identifier(key, "V").is_some() {
                    unit("V")
                } else if parse_wrapped_identifier(key, "I").is_some() {
                    unit("A")
                } else {
                    MeasurementUnit::Unknown
                }
            }
            Self::TransferFunction {
                input_quantity,
                output_quantity,
                normalization,
                ..
            } => {
                if matches!(folded.as_str(), "gain" | "transfer_gain" | "tf.gain") {
                    if *normalization
                        == crate::simulation::multi_run::TfNormalization::RelativeToNominal
                    {
                        unit("1")
                    } else {
                        match (input_quantity, output_quantity) {
                            (
                                TransferFunctionQuantity::Voltage,
                                TransferFunctionQuantity::Current,
                            ) => unit("S"),
                            (
                                TransferFunctionQuantity::Current,
                                TransferFunctionQuantity::Voltage,
                            ) => unit("ohm"),
                            _ => unit("1"),
                        }
                    }
                } else {
                    unit("ohm")
                }
            }
            Self::DcMismatch { evidence } => unit(&evidence.output_unit),
            Self::SensitivityStudy { .. } => {
                if key.starts_with("normalized:") {
                    unit("1")
                } else {
                    MeasurementUnit::Unknown
                }
            }
            Self::PoleZero { .. } => {
                if matches!(folded.as_str(), "num_poles" | "num_zeros") {
                    unit("count")
                } else {
                    // The result does not retain its input/output quantities.
                    MeasurementUnit::Unknown
                }
            }
            Self::Fft { spectrum, .. } => match folded.as_str() {
                "fft.dc" | "fft.fundamental_magnitude" => fft_unit(&spectrum.evidence),
                "fft.thd_ratio" => unit("1"),
                "fft.enob_bits" => unit("bits"),
                "fft.sfdr_spur_frequency_hz" => unit("Hz"),
                "fft.thd_db" | "fft.sndr_db" | "fft.snr_db" | "fft.sfdr_db" => unit("dB"),
                _ => MeasurementUnit::Unknown,
            },
            Self::Transient {
                periodic_state: Some(_),
                ..
            } => match folded.as_str() {
                "pss.period" => unit("s"),
                "pss.frequency" => unit("Hz"),
                "pss.iterations" => unit("count"),
                _ => MeasurementUnit::Unknown,
            },
            Self::Qpss {
                operating_point, ..
            } => match folded.as_str() {
                "qpss.iterations" => unit("count"),
                "qpss.normalized_residual" => unit("1"),
                "qpss.oscillator_frequency_hz"
                    if operating_point.oscillator_frequency_hz().is_some() =>
                {
                    unit("Hz")
                }
                _ => MeasurementUnit::Unknown,
            },
            Self::Pstb { waveforms, .. } => match folded.as_str() {
                "pstb.period" => unit("s"),
                "pstb.fundamental_frequency" => unit("Hz"),
                "pstb.min_stability_margin_db" => unit("dB"),
                "pstb.mode_count" | "pstb.unstable_mode_count" => unit("count"),
                "pstb.max_multiplier_magnitude"
                | "pstb.stability_threshold"
                | "pstb.detect_subharmonics" => unit("1"),
                _ => last_waveform_by_name(waveforms, key)
                    .map_or(MeasurementUnit::Unknown, |wave| unit(&wave.y_unit)),
            },
            Self::Ac { waveforms, .. } => named_value(waveforms, key)
                .map_or(MeasurementUnit::Unknown, |wave| unit(&wave.y_unit)),
            Self::Noise { summary, .. } => match folded.as_str() {
                "noise.output_rms" => unit("V"),
                "noise.input_rms" => summary
                    .as_ref()
                    .and_then(|summary| summary.input_quantity)
                    .map_or(MeasurementUnit::Unknown, |quantity| match quantity {
                        rspice_core::analysis::noise::NoiseInputQuantity::Voltage => unit("V"),
                        rspice_core::analysis::noise::NoiseInputQuantity::Current => unit("A"),
                    }),
                _ => MeasurementUnit::Unknown,
            },
            Self::Qpnoise { response, .. } => {
                let Some((name, arguments)) = folded.split_once('(') else {
                    return MeasurementUnit::Unknown;
                };
                if name == "qpnoise.contributor_share_percent" {
                    return unit("%");
                }
                if name == "qpnoise.input_rms" {
                    return response.metadata.input_source.as_ref().map_or(
                        MeasurementUnit::Unknown,
                        |input| match input.quantity {
                            rspice_core::engine::QpxfQuantity::Voltage => unit("V"),
                            rspice_core::engine::QpxfQuantity::Current => unit("A"),
                        },
                    );
                }
                let output = arguments
                    .trim_end_matches(')')
                    .split(',')
                    .next()
                    .and_then(|index| index.trim().parse::<usize>().ok())
                    .and_then(|index| index.checked_sub(1))
                    .and_then(|index| response.metadata.request.outputs.get(index));
                output.map_or(MeasurementUnit::Unknown, |output| {
                    match output.observation {
                        rspice_core::engine::QpnoiseObservation::Voltage { .. } => unit("V"),
                        rspice_core::engine::QpnoiseObservation::BranchCurrent { .. } => unit("A"),
                    }
                })
            }
            _ => MeasurementUnit::Unknown,
        }
    }
}

#[cfg(test)]
mod tests;

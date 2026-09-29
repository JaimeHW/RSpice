//! Canonical correlation CSV from exact samples of an already-selected trace.

use csv::{Terminator, WriterBuilder};
use rspice_results::analysis_type::AnalysisType;
use rspice_results::waveform::RetainedWaveform;

#[derive(Debug)]
pub enum CorrelationCsvError {
    InvalidSampleCounts { coordinates: usize, values: usize },
    SampleLimit { samples: usize, limit: usize },
    NonFiniteSample,
    UnsupportedAnalysis(AnalysisType),
    Header(csv::Error),
    Row { row: usize, source: csv::Error },
    Finalize(std::io::Error),
}

impl std::fmt::Display for CorrelationCsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSampleCounts { .. } => f.write_str(
                "The selected waveform must retain equally sized, non-empty X and Y vectors",
            ),
            Self::SampleLimit { samples, limit } => write!(
                f,
                "The selected waveform has {samples} samples; correlation datasets are limited to {limit}",
            ),
            Self::NonFiniteSample => {
                f.write_str("The selected waveform contains a non-finite sample")
            }
            Self::UnsupportedAnalysis(analysis_type) => write!(
                f,
                "{} does not expose a correlation-compatible waveform axis",
                analysis_type.display_name(),
            ),
            Self::Header(source) => write!(
                f,
                "Canonical correlation CSV header cannot be written: {source}"
            ),
            Self::Row { row, source } => write!(
                f,
                "Canonical correlation CSV row {row} cannot be written: {source}"
            ),
            Self::Finalize(source) => {
                write!(f, "Canonical correlation CSV cannot be finalized: {source}")
            }
        }
    }
}

impl std::error::Error for CorrelationCsvError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Header(source) | Self::Row { source, .. } => Some(source),
            Self::Finalize(source) => Some(source),
            _ => None,
        }
    }
}

/// Preserve the selected trace's samples and the established correlation schema.
/// The caller supplies its domain's row limit; selection and provenance stay there.
pub fn encode_correlation_csv(
    analysis_type: AnalysisType,
    trace: &RetainedWaveform,
    max_rows: usize,
) -> Result<Vec<u8>, CorrelationCsvError> {
    if trace.x.is_empty() || trace.x.len() != trace.y.len() {
        return Err(CorrelationCsvError::InvalidSampleCounts {
            coordinates: trace.x.len(),
            values: trace.y.len(),
        });
    }
    if trace.x.len() > max_rows {
        return Err(CorrelationCsvError::SampleLimit {
            samples: trace.x.len(),
            limit: max_rows,
        });
    }
    if trace
        .x
        .iter()
        .chain(trace.y.iter())
        .any(|value| !value.is_finite())
    {
        return Err(CorrelationCsvError::NonFiniteSample);
    }
    let (axis, axis_unit) = waveform_axis(analysis_type)?;
    let value_unit = waveform_value_unit(&trace.name);
    let condition_header = format!("condition:{axis}[{axis_unit}]");
    let mut writer = WriterBuilder::new()
        .has_headers(false)
        .terminator(Terminator::Any(b'\n'))
        .from_writer(Vec::new());
    writer
        .write_record([
            "id",
            "quantity",
            "value",
            "unit",
            "uncertainty",
            "weight",
            condition_header.as_str(),
        ])
        .map_err(CorrelationCsvError::Header)?;
    for (index, (&x, &y)) in trace.x.iter().zip(trace.y.iter()).enumerate() {
        let id = format!("sample-{index:08}");
        let value = y.to_string();
        let coordinate = x.to_string();
        writer
            .write_record([
                id.as_str(),
                trace.name.as_str(),
                value.as_str(),
                value_unit,
                "0",
                "1",
                coordinate.as_str(),
            ])
            .map_err(|source| CorrelationCsvError::Row {
                row: index + 1,
                source,
            })?;
    }
    writer
        .into_inner()
        .map_err(|error| CorrelationCsvError::Finalize(error.into_error()))
}

fn waveform_axis(
    analysis_type: AnalysisType,
) -> Result<(&'static str, &'static str), CorrelationCsvError> {
    use AnalysisType as Kind;
    match analysis_type {
        Kind::Transient | Kind::TransientNoise | Kind::Envelope | Kind::Pss | Kind::Qpss => {
            Ok(("time", "s"))
        }
        Kind::Ac
        | Kind::Disto
        | Kind::Noise
        | Kind::Pac
        | Kind::Pnoise
        | Kind::Pxf
        | Kind::Pstb
        | Kind::Stb
        | Kind::SParameter
        | Kind::Fourier
        | Kind::HarmonicBalance
        | Kind::Hbsp
        | Kind::Hbnoise
        | Kind::Psp
        | Kind::Qpac
        | Kind::Qpnoise
        | Kind::Qpxf => Ok(("frequency", "Hz")),
        Kind::DcSweep
        | Kind::Sensitivity
        | Kind::MonteCarlo
        | Kind::Parametric
        | Kind::Corner
        | Kind::Optimization
        | Kind::Soa
        | Kind::DcMismatch => Ok(("sweep", "1")),
        Kind::DcOp | Kind::PoleZero | Kind::Tf => {
            Err(CorrelationCsvError::UnsupportedAnalysis(analysis_type))
        }
    }
}

fn waveform_value_unit(trace_name: &str) -> &'static str {
    let normalized = trace_name.trim().to_ascii_lowercase();
    if normalized.starts_with("phase(") {
        "deg"
    } else if normalized.starts_with("db(") {
        "dB"
    } else {
        let unwrapped = normalized.trim_matches('|');
        if unwrapped.starts_with("v(") {
            "V"
        } else if unwrapped.starts_with("i(") {
            "A"
        } else {
            "1"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_bytes_preserve_quoting_sample_order_and_f64_values() {
        let trace = RetainedWaveform::new(
            "phase(V(\"out,ref\"))",
            vec![10.0, 1.0],
            vec![-0.0, f64::from_bits(1.0_f64.to_bits() + 1)],
        );
        let bytes = encode_correlation_csv(AnalysisType::Ac, &trace, 2).unwrap();
        assert_eq!(
            bytes,
            concat!(
                "id,quantity,value,unit,uncertainty,weight,condition:frequency[Hz]\n",
                "sample-00000000,\"phase(V(\"\"out,ref\"\"))\",-0,deg,0,1,10\n",
                "sample-00000001,\"phase(V(\"\"out,ref\"\"))\",1.0000000000000002,deg,0,1,1\n",
            )
            .as_bytes()
        );
        let error = encode_correlation_csv(AnalysisType::Ac, &trace, 1).unwrap_err();
        assert!(matches!(
            error,
            CorrelationCsvError::SampleLimit {
                samples: 2,
                limit: 1
            }
        ));
        assert_eq!(
            error.to_string(),
            "The selected waveform has 2 samples; correlation datasets are limited to 1"
        );
    }

    #[test]
    fn retained_waveform_export_rejects_unusable_or_unbounded_sources() {
        let mismatched = RetainedWaveform::new("V(out)", vec![0.0], Vec::<f64>::new());
        let error = encode_correlation_csv(AnalysisType::Transient, &mismatched, 1).unwrap_err();
        assert!(matches!(
            error,
            CorrelationCsvError::InvalidSampleCounts {
                coordinates: 1,
                values: 0
            }
        ));
        assert!(error.to_string().contains("equally sized"));
        let non_finite = RetainedWaveform::new("V(out)", vec![0.0], vec![f64::NAN]);
        let error = encode_correlation_csv(AnalysisType::Transient, &non_finite, 1).unwrap_err();
        assert!(matches!(error, CorrelationCsvError::NonFiniteSample));
        assert!(error.to_string().contains("non-finite"));
        let scalar = RetainedWaveform::new("V(out)", vec![0.0], vec![1.0]);
        let error = encode_correlation_csv(AnalysisType::DcOp, &scalar, 1).unwrap_err();
        assert!(matches!(
            error,
            CorrelationCsvError::UnsupportedAnalysis(AnalysisType::DcOp)
        ));
        assert!(error.to_string().contains("does not expose"));
    }
}

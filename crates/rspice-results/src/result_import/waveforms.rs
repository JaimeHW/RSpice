//! Bounded assembly of imported scalar and complex waveforms, before presentation.

use super::ResultImportFormat;
use crate::analysis_type::AnalysisType;
use crate::waveform::RetainedWaveform;
use std::collections::HashSet;
use std::sync::Arc;

/// Unassembled source samples; no import authority is conferred by this record.
#[derive(Debug)]
pub struct ImportedSignal {
    pub name: String,
    pub real: Vec<f64>,
    pub imag: Option<Vec<f64>>,
    pub unit: Option<String>,
}

/// Resource policy supplied by the importing host.
#[derive(Debug, Clone, Copy)]
pub struct WaveformImportLimits {
    pub min_rows: usize,
    pub max_rows: usize,
    pub max_columns: usize,
    pub max_values: usize,
    pub max_signal_name_bytes: usize,
}

/// Exact imported samples, ready for the host's presentation and transaction.
#[derive(Debug)]
pub struct ImportedWaveforms {
    pub coordinate_name: String,
    pub sample_count: usize,
    pub waveforms: Vec<RetainedWaveform>,
}

fn adapter_error(format: ResultImportFormat, detail: impl std::fmt::Display) -> String {
    format!("{} import: {detail}", format.canonical_id())
}

/// Check source shape, precision and resource policy, preserving validation order.
pub fn assemble_imported_waveforms(
    format: ResultImportFormat,
    analysis_type: AnalysisType,
    coordinate_name: impl Into<String>,
    coordinate: Vec<f64>,
    signals: Vec<ImportedSignal>,
    limits: WaveformImportLimits,
) -> Result<ImportedWaveforms, String> {
    let WaveformImportLimits {
        min_rows,
        max_rows,
        max_columns,
        max_values,
        max_signal_name_bytes,
    } = limits;
    let coordinate_name = coordinate_name.into();
    validate_name(
        format,
        "coordinate",
        &coordinate_name,
        max_signal_name_bytes,
    )?;
    if coordinate.len() < min_rows {
        return Err(adapter_error(
            format,
            format_args!(
                "the '{coordinate_name}' coordinate carries no samples, so the source holds no \
                 result to import"
            ),
        ));
    }
    if coordinate.len() > max_rows {
        return Err(adapter_error(
            format,
            format_args!(
                "the coordinate contains {} samples; the limit is {max_rows}",
                coordinate.len()
            ),
        ));
    }
    if signals.is_empty() {
        return Err(adapter_error(
            format,
            "the source contains no importable signals",
        ));
    }
    if signals.len() + 1 > max_columns {
        return Err(adapter_error(
            format,
            format_args!(
                "the source contains {} columns; the limit is {max_columns}",
                signals.len() + 1
            ),
        ));
    }
    validate_finite(format, &coordinate_name, &coordinate)?;
    validate_coordinate(format, analysis_type, &coordinate)?;

    let retained_values = coordinate
        .len()
        .checked_mul(1 + signals.len().saturating_mul(2))
        .ok_or_else(|| adapter_error(format, "retained-value count overflow"))?;
    if retained_values > max_values {
        return Err(adapter_error(
            format,
            format_args!(
                "the source expands to {retained_values} numeric values; the limit is {max_values}"
            ),
        ));
    }

    let coordinate = Arc::new(coordinate);
    let mut names = HashSet::with_capacity(signals.len());
    let mut waveforms = Vec::with_capacity(signals.len());
    for signal in signals {
        validate_name(format, "signal", &signal.name, max_signal_name_bytes)?;
        if !names.insert(signal.name.to_ascii_lowercase()) {
            return Err(adapter_error(
                format,
                format_args!("duplicate signal identity '{}'", signal.name),
            ));
        }
        if signal.real.len() != coordinate.len() {
            return Err(adapter_error(
                format,
                format_args!(
                    "signal '{}' has {} samples; expected {}",
                    signal.name,
                    signal.real.len(),
                    coordinate.len()
                ),
            ));
        }
        validate_finite(format, &signal.name, &signal.real)?;
        let mut waveform = if let Some(imag) = signal.imag {
            if imag.len() != coordinate.len() {
                return Err(adapter_error(
                    format,
                    format_args!(
                        "signal '{}' imaginary component has {} samples; expected {}",
                        signal.name,
                        imag.len(),
                        coordinate.len()
                    ),
                ));
            }
            validate_finite(
                format,
                &format!("{} imaginary component", signal.name),
                &imag,
            )?;
            let magnitude = signal
                .real
                .iter()
                .zip(&imag)
                .map(|(real, imag)| real.hypot(*imag))
                .collect::<Vec<_>>();
            RetainedWaveform::new(
                format!("|{}|", signal.name),
                Arc::clone(&coordinate),
                magnitude,
            )
            .with_complex_components(signal.name, signal.real, imag)
        } else {
            RetainedWaveform::new(signal.name, Arc::clone(&coordinate), signal.real)
        };
        if let Some(unit) = signal.unit {
            waveform = waveform.with_unit(unit);
        }
        waveforms.push(waveform);
    }

    Ok(ImportedWaveforms {
        coordinate_name,
        sample_count: coordinate.len(),
        waveforms,
    })
}

/// Check a source identity using the importing host's length limit.
pub fn validate_name(
    format: ResultImportFormat,
    kind: &str,
    name: &str,
    max_signal_name_bytes: usize,
) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err(adapter_error(format, format_args!("{kind} name is empty")));
    }
    if name.len() > max_signal_name_bytes {
        return Err(adapter_error(
            format,
            format_args!("{kind} name exceeds {max_signal_name_bytes} bytes"),
        ));
    }
    if name.chars().any(char::is_control) {
        return Err(adapter_error(
            format,
            format_args!("{kind} name contains a control character"),
        ));
    }
    Ok(())
}

fn validate_finite(
    format: ResultImportFormat,
    identity: &str,
    values: &[f64],
) -> Result<(), String> {
    if let Some((index, value)) = values
        .iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite())
    {
        return Err(adapter_error(
            format,
            format_args!("'{identity}' contains non-finite value {value} at sample {index}"),
        ));
    }
    Ok(())
}

fn validate_coordinate(
    format: ResultImportFormat,
    analysis_type: AnalysisType,
    coordinate: &[f64],
) -> Result<(), String> {
    let mut direction = None;
    for (index, pair) in coordinate.windows(2).enumerate() {
        let step = pair[1].total_cmp(&pair[0]);
        if step.is_eq() {
            return Err(adapter_error(
                format,
                format_args!(
                    "coordinate repeats {} at samples {} and {}",
                    pair[0],
                    index,
                    index + 1
                ),
            ));
        }
        if let Some(expected) = direction {
            if step != expected {
                return Err(adapter_error(
                    format,
                    format_args!("coordinate reverses direction at sample {}", index + 1),
                ));
            }
        } else {
            direction = Some(step);
        }
    }
    if analysis_type == AnalysisType::Ac && coordinate.iter().any(|value| *value <= 0.0) {
        return Err(adapter_error(
            format,
            "frequency coordinates must all be greater than zero",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assembly_preserves_source_buffers_and_rejects_ambiguous_identity() {
        let limits = WaveformImportLimits {
            min_rows: 1,
            max_rows: 2,
            max_columns: 3,
            max_values: 10,
            max_signal_name_bytes: 32,
        };
        let real = vec![-0.0, f64::from_bits(1)];
        let real_ptr = real.as_ptr();
        let imag = vec![0.0, 1.0];
        let imag_ptr = imag.as_ptr();
        let scalar = vec![2.0, 3.0];
        let scalar_ptr = scalar.as_ptr();
        let imported = assemble_imported_waveforms(
            ResultImportFormat::NumpyNpz,
            AnalysisType::Ac,
            "frequency",
            vec![1.0, 2.0],
            vec![
                ImportedSignal {
                    name: "V(out)".into(),
                    real,
                    imag: Some(imag),
                    unit: Some("V".into()),
                },
                ImportedSignal {
                    name: "I(in)".into(),
                    real: scalar,
                    imag: None,
                    unit: Some("A".into()),
                },
            ],
            limits,
        )
        .unwrap();
        let complex = imported.waveforms[0].complex.as_ref().unwrap();
        assert_eq!(complex.real.as_ptr(), real_ptr);
        assert_eq!(complex.imag.as_ptr(), imag_ptr);
        assert_eq!(complex.real[0].to_bits(), (-0.0_f64).to_bits());
        assert_eq!(complex.real[1].to_bits(), 1);
        assert_eq!(complex.source_name, "V(out)");
        assert_eq!(imported.waveforms[0].name, "|V(out)|");
        assert_eq!(imported.waveforms[0].unit.as_deref(), Some("V"));
        assert_eq!(imported.waveforms[1].y.as_ptr(), scalar_ptr);
        assert!(Arc::ptr_eq(
            &imported.waveforms[0].x,
            &imported.waveforms[1].x
        ));
        let signals = ["V(out)", "v(OUT)"].map(|name| ImportedSignal {
            name: name.into(),
            real: vec![1.0],
            imag: None,
            unit: None,
        });
        assert_eq!(
            assemble_imported_waveforms(
                ResultImportFormat::NumpyNpz,
                AnalysisType::Transient,
                "time",
                vec![0.0],
                signals.into(),
                limits,
            )
            .unwrap_err(),
            "numpy-npz import: duplicate signal identity 'v(OUT)'"
        );
    }
}

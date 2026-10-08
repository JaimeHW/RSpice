//! Validate borrowed waveform tables before a format allocates output arrays.

use rspice_results::result_import::waveforms::WaveformImportLimits;
use rspice_results::waveform::RetainedWaveform;

#[derive(Debug)]
pub enum WaveformExportError {
    NoSamples,
    RowLimit {
        rows: usize,
        minimum: usize,
        maximum: usize,
    },
    ColumnLimit {
        columns: usize,
        limit: usize,
    },
    ValueLimit {
        values: Option<usize>,
        limit: usize,
    },
    DifferentCoordinates {
        signal: String,
    },
    SampleCount {
        signal: String,
        component: &'static str,
        actual: usize,
        expected: usize,
    },
}

impl std::fmt::Display for WaveformExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSamples => f.write_str("No waveform samples available to export."),
            Self::RowLimit {
                rows,
                minimum,
                maximum,
            } => write!(
                f,
                "This result has {rows} samples; reopening the export requires {minimum}..={maximum} samples."
            ),
            Self::ColumnLimit { columns, limit } => write!(
                f,
                "This result has {columns} columns; the import limit is {limit}. Hide traces before exporting."
            ),
            Self::ValueLimit {
                values: Some(values),
                limit,
            } => write!(
                f,
                "Reopening this export retains {values} numeric values; the limit is {limit}. Hide traces before exporting."
            ),
            Self::ValueLimit { values: None, .. } => {
                f.write_str("Export numeric-value accounting overflowed.")
            }
            Self::DifferentCoordinates { signal } => write!(
                f,
                "This export is a rectangular table: every signal must use the same coordinate samples. '{signal}' carries its own x-axis samples."
            ),
            Self::SampleCount {
                signal,
                component,
                actual,
                expected,
            } => write!(
                f,
                "'{signal}' has {actual} {component} samples against {expected} coordinate samples; the export cannot pad or truncate them."
            ),
        }
    }
}

impl std::error::Error for WaveformExportError {}

pub(crate) fn check_export_shape(
    rows: usize,
    columns: usize,
    retained_columns: Option<usize>,
    limits: WaveformImportLimits,
) -> Result<(), WaveformExportError> {
    if rows == 0 || columns < 2 {
        return Err(WaveformExportError::NoSamples);
    }
    if rows < limits.min_rows || rows > limits.max_rows {
        return Err(WaveformExportError::RowLimit {
            rows,
            minimum: limits.min_rows,
            maximum: limits.max_rows,
        });
    }
    if columns > limits.max_columns {
        return Err(WaveformExportError::ColumnLimit {
            columns,
            limit: limits.max_columns,
        });
    }
    let values = retained_columns.and_then(|columns| rows.checked_mul(columns));
    if values.is_none_or(|values| values > limits.max_values) {
        return Err(WaveformExportError::ValueLimit {
            values,
            limit: limits.max_values,
        });
    }
    Ok(())
}

/// Apply the receiving host's sample limits before cloning any source array.
/// A complex signal retains its rectangular components and a magnitude; the
/// shared coordinate is counted once, regardless of the number of signals.
pub(crate) fn shared_coordinate<'a, W: AsRef<RetainedWaveform>>(
    waveforms: &[&'a W],
    limits: WaveformImportLimits,
) -> Result<&'a [f64], WaveformExportError> {
    let reference = waveforms
        .iter()
        .map(|waveform| (*waveform).as_ref())
        .max_by_key(|waveform| waveform.x.len())
        .ok_or(WaveformExportError::NoSamples)?;
    let retained_columns = waveforms.iter().try_fold(1usize, |count, waveform| {
        count.checked_add(if waveform.as_ref().complex.is_some() {
            3
        } else {
            1
        })
    });
    check_export_shape(
        reference.x.len(),
        waveforms.len().saturating_add(1),
        retained_columns,
        limits,
    )?;
    for waveform in waveforms {
        let waveform = (*waveform).as_ref();
        if waveform.x.as_ref() != reference.x.as_ref() {
            return Err(WaveformExportError::DifferentCoordinates {
                signal: waveform.name.clone(),
            });
        }
        let check = |component, actual| {
            if actual == reference.x.len() {
                Ok(())
            } else {
                Err(WaveformExportError::SampleCount {
                    signal: waveform.name.clone(),
                    component,
                    actual,
                    expected: reference.x.len(),
                })
            }
        };
        check("display", waveform.y.len())?;
        if let Some(complex) = &waveform.complex {
            check("real", complex.real.len())?;
            check("imaginary", complex.imag.len())?;
        }
    }
    Ok(reference.x.as_slice())
}

#[cfg(test)]
mod tests {
    use super::super::EXPORT_TEST_LIMITS;
    use super::*;

    #[cfg(feature = "result-hdf5")]
    #[test]
    fn every_binary_projection_checks_retained_samples_before_encoding() {
        use rspice_results::{analysis_result::AnalysisResult, analysis_type::AnalysisType};
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "Complex", 0.0)
            .with_waveforms(vec![
                RetainedWaveform::new("magnitude", vec![0.0, 1.0], vec![5.0, 5.0])
                    .with_complex_components("out", vec![3.0, 4.0], vec![4.0, 3.0]),
            ]);
        let waveforms = analysis.waveforms.iter().collect::<Vec<_>>();
        for max_values in [7, 8] {
            let limits = WaveformImportLimits {
                max_values,
                ..EXPORT_TEST_LIMITS
            };
            for result in [
                crate::hdf5::result::prepare_hdf5(&analysis, &waveforms, limits)
                    .map(|_| ())
                    .map_err(|e| e.to_string()),
                crate::matlab::result::prepare_matlab(&analysis, &waveforms, limits)
                    .map(|_| ())
                    .map_err(|e| e.to_string()),
                crate::numpy::result::prepare_numpy(&analysis, &waveforms, limits)
                    .map(|_| ())
                    .map_err(|e| e.to_string()),
            ] {
                if max_values == 8 {
                    result.unwrap();
                } else {
                    assert!(result.unwrap_err().contains("retains 8 numeric values"));
                }
            }
        }
    }

    #[test]
    fn export_preflight_counts_every_retained_array_at_the_exact_boundary() {
        for (complex, values) in [
            (vec![false], 4),
            (vec![true], 8),
            (vec![false, false], 6),
            (vec![false, true], 10),
            (vec![true, true], 14),
        ] {
            let waveforms = complex
                .iter()
                .enumerate()
                .map(|(index, complex)| {
                    let waveform = RetainedWaveform::new(
                        format!("signal{index}"),
                        vec![-0.0, 1.0],
                        vec![3.0, 4.0],
                    );
                    if *complex {
                        waveform.with_complex_components(
                            format!("source{index}"),
                            vec![3.0, 4.0],
                            vec![-0.0, 2.0],
                        )
                    } else {
                        waveform
                    }
                })
                .collect::<Vec<_>>();
            let borrowed = waveforms.iter().collect::<Vec<_>>();
            for limit in [values - 1, values] {
                let result = shared_coordinate(
                    &borrowed,
                    WaveformImportLimits {
                        max_values: limit,
                        ..EXPORT_TEST_LIMITS
                    },
                );
                if limit == values {
                    assert_eq!(result.unwrap()[0].to_bits(), (-0.0_f64).to_bits());
                } else {
                    assert!(
                        matches!(result, Err(WaveformExportError::ValueLimit { values: Some(actual), .. }) if actual == values)
                    );
                }
            }
        }
    }

    #[test]
    fn export_preflight_rejects_shape_errors_and_host_bounds() {
        let valid = RetainedWaveform::new("valid", vec![0.0, 1.0], vec![1.0, 2.0]);
        for malformed in [
            RetainedWaveform::new("display", vec![0.0, 1.0], vec![1.0]),
            valid
                .clone()
                .with_complex_components("real", vec![1.0], vec![1.0, 2.0]),
            valid
                .clone()
                .with_complex_components("imag", vec![1.0, 2.0], vec![1.0]),
        ] {
            assert!(matches!(
                shared_coordinate(&[&malformed], EXPORT_TEST_LIMITS),
                Err(WaveformExportError::SampleCount { .. })
            ));
        }
        assert!(matches!(
            shared_coordinate(
                &[&valid],
                WaveformImportLimits {
                    max_rows: 1,
                    ..EXPORT_TEST_LIMITS
                }
            ),
            Err(WaveformExportError::RowLimit { .. })
        ));
        assert!(matches!(
            shared_coordinate(
                &[&valid],
                WaveformImportLimits {
                    max_columns: 1,
                    ..EXPORT_TEST_LIMITS
                }
            ),
            Err(WaveformExportError::ColumnLimit { .. })
        ));
        assert!(check_export_shape(2, 2, Some(usize::MAX), EXPORT_TEST_LIMITS).is_err());
    }
}

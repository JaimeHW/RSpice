//! Validate borrowed waveform tables before a format allocates output arrays.

use std::borrow::Cow;

use rspice_results::result_import::waveforms::WaveformImportLimits;
use rspice_results::waveform::{RetainedWaveform, WaveformValues};

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
    /// A signal's samples could not be read from its results store, or are more than the
    /// loaded results may hold.
    Unreadable {
        signal: String,
        reason: String,
    },
}

impl std::fmt::Display for WaveformExportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSamples => f.write_str("No waveform samples available to export."),
            Self::Unreadable { signal, reason } => {
                write!(f, "'{signal}' could not be exported: {reason}")
            }
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

/// The waveform whose coordinate every exported signal shares, once the receiving host's sample
/// limits and the table's shape are checked: no sample is read, so a writer can refuse a table
/// before it writes a byte and then read the coordinate a block at a time.
///
/// A complex signal retains its rectangular components and a magnitude; the shared coordinate
/// is counted once, regardless of the number of signals.
pub(crate) fn checked_coordinate<'a, W: AsRef<RetainedWaveform>>(
    waveforms: &[&'a W],
    limits: WaveformImportLimits,
) -> Result<&'a RetainedWaveform, WaveformExportError> {
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
        if waveform.x != reference.x {
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
    Ok(reference)
}

/// A column read in row order a block at a time, for a writer that writes a table row by row: a
/// column of any length costs one block of memory. A column held in memory is read in place.
pub(crate) struct RowReader<'a> {
    signal: &'a str,
    values: &'a WaveformValues,
    first: usize,
    block: Cow<'a, [f64]>,
    failure: Option<WaveformExportError>,
}

impl<'a> RowReader<'a> {
    pub(crate) fn new(signal: &'a str, values: &'a WaveformValues) -> Self {
        Self {
            signal,
            values,
            first: 0,
            block: values.in_memory().map_or(Cow::Borrowed(&[]), Cow::Borrowed),
            failure: None,
        }
    }

    /// Sample `row`. A row whose block cannot be read is unavailable (NaN), and the failure is
    /// kept for [`Self::failure`]; no block is read after it.
    pub(crate) fn at(&mut self, row: usize) -> f64 {
        if let Some(value) = row
            .checked_sub(self.first)
            .and_then(|at| self.block.get(at))
        {
            return *value;
        }
        if self.failure.is_some() || row >= self.values.len() {
            return f64::NAN;
        }
        let end = row
            .saturating_add(rspice_results::waveform::BLOCK_SAMPLES)
            .min(self.values.len());
        match self.values.blocks(row..end).next() {
            Some(Ok(block)) => {
                self.first = row;
                self.block = block;
                self.block[0]
            }
            Some(Err(failure)) => {
                self.failure = Some(WaveformExportError::Unreadable {
                    signal: self.signal.to_owned(),
                    reason: failure.to_string(),
                });
                f64::NAN
            }
            None => f64::NAN,
        }
    }

    /// Whether a row could not be read.
    #[cfg(feature = "spice-raw")]
    pub(crate) fn failed(&self) -> bool {
        self.failure.is_some()
    }

    /// Why a row could not be read, if one could not.
    pub(crate) fn failure(&mut self) -> Option<WaveformExportError> {
        self.failure.take()
    }
}

/// One analysis an export writes, with the traces of it the export writes: the one analysis of
/// most exports, one member of a `.step` or Monte Carlo family of an export of every member.
pub struct ExportMember<'a, W> {
    pub analysis: &'a rspice_results::analysis_result::AnalysisResult<W>,
    pub waveforms: Vec<&'a W>,
}

impl<'a, W> ExportMember<'a, W> {
    pub fn new(
        analysis: &'a rspice_results::analysis_result::AnalysisResult<W>,
        waveforms: Vec<&'a W>,
    ) -> Self {
        Self {
            analysis,
            waveforms,
        }
    }
}

impl<W: AsRef<RetainedWaveform>> ExportMember<'_, W> {
    /// What the member is called in a file of every member: its parameter values, `r=2k`, as
    /// every route names a member, or its analysis's label when it is no member of a family.
    pub fn label(&self) -> String {
        self.analysis
            .family_member()
            .map_or_else(|| self.analysis.label.clone(), |member| member.label())
    }

    /// [`Self::label`] as a name inside a file, which a path separator or a `..` would split or
    /// escape: each becomes `_`.
    pub fn file_label(&self) -> String {
        let label = self.label().replace(['/', '\\'], "_").replace("..", "_");
        if label.trim().is_empty() {
            "member".to_owned()
        } else {
            label
        }
    }
}

/// The windows a column of `rows` samples is read in, in order: one block each.
pub(crate) fn windows(rows: usize) -> impl Iterator<Item = std::ops::Range<usize>> {
    (0..rows)
        .step_by(rspice_results::waveform::BLOCK_SAMPLES)
        .map(move |first| {
            first
                ..first
                    .saturating_add(rspice_results::waveform::BLOCK_SAMPLES)
                    .min(rows)
        })
}

/// Samples `window` of `values`, the signal `signal`: borrowed from memory, or one block read
/// from its results store, refused with the store's reason when it cannot be read.
pub(crate) fn window<'a>(
    signal: &str,
    values: &'a WaveformValues,
    window: std::ops::Range<usize>,
) -> Result<Cow<'a, [f64]>, WaveformExportError> {
    let end = window.end.min(values.len());
    match values.blocks(window.start.min(end)..end).next() {
        Some(Ok(block)) => Ok(block),
        Some(Err(failure)) => Err(WaveformExportError::Unreadable {
            signal: signal.to_owned(),
            reason: failure.to_string(),
        }),
        None => Ok(Cow::Borrowed(&[])),
    }
}

/// The samples of `values`, the signal `signal`, for a writer that takes them as one array:
/// borrowed when they are in memory, read whole from their results store when the loaded results
/// may hold them, refused with the store's reason otherwise.
#[cfg(feature = "native-bundle")]
pub(crate) fn export_samples<'a>(
    signal: &str,
    values: &'a WaveformValues,
) -> Result<Cow<'a, [f64]>, WaveformExportError> {
    values
        .read_whole()
        .map_err(|failure| WaveformExportError::Unreadable {
            signal: signal.to_owned(),
            reason: failure.to_string(),
        })
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
            let members = [ExportMember::new(&analysis, waveforms.clone())];
            for result in [
                crate::hdf5::result::check_hdf5(&members, limits)
                    .map(|_| ())
                    .map_err(|e| e.to_string()),
                crate::matlab::result::check_matlab(&members, limits)
                    .map(|_| ())
                    .map_err(|e| e.to_string()),
                crate::numpy::result::check_npz(&members, limits)
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
                let result = checked_coordinate(
                    &borrowed,
                    WaveformImportLimits {
                        max_values: limit,
                        ..EXPORT_TEST_LIMITS
                    },
                );
                if limit == values {
                    assert_eq!(result.unwrap().x.len(), 2);
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
                checked_coordinate(&[&malformed], EXPORT_TEST_LIMITS),
                Err(WaveformExportError::SampleCount { .. })
            ));
        }
        assert!(matches!(
            checked_coordinate(
                &[&valid],
                WaveformImportLimits {
                    max_rows: 1,
                    ..EXPORT_TEST_LIMITS
                }
            ),
            Err(WaveformExportError::RowLimit { .. })
        ));
        assert!(matches!(
            checked_coordinate(
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

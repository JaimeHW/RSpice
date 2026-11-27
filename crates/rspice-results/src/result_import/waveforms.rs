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
    /// Retained f64 values: the shared coordinate plus each real trace, or
    /// both rectangular components and the magnitude of each complex trace.
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
    format!("{} import: {detail}", format.label())
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

    // The coordinate is shared. Each real trace owns one array; each complex
    // trace retains both components and a derived magnitude array.
    let retained_columns = signals.iter().try_fold(1usize, |count, signal| {
        count.checked_add(if signal.imag.is_some() { 3 } else { 1 })
    });
    let retained_values = retained_columns
        .and_then(|columns| coordinate.len().checked_mul(columns))
        .ok_or_else(|| adapter_error(format, "too many values to count"))?;
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
        validate_signal_values(format, &signal.name, &signal.real)?;
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
            validate_signal_values(
                format,
                &format!("{} imaginary component", signal.name),
                &imag,
            )?;
            if signal
                .real
                .iter()
                .zip(&imag)
                .any(|(real, imag)| real.is_nan() != imag.is_nan())
            {
                return Err(adapter_error(
                    format,
                    format_args!(
                        "signal '{}' complex components have inconsistent sample availability",
                        signal.name
                    ),
                ));
            }
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
    validate_finite_from(format, identity, values, 0)
}

/// [`validate_finite`] for `values` that start at sample `first`.
fn validate_finite_from(
    format: ResultImportFormat,
    identity: &str,
    values: &[f64],
    first: usize,
) -> Result<(), String> {
    if let Some((index, value)) = values
        .iter()
        .enumerate()
        .find(|(_, value)| !value.is_finite())
    {
        let index = first + index;
        return Err(adapter_error(
            format,
            format_args!("'{identity}' contains non-finite value {value} at sample {index}"),
        ));
    }
    Ok(())
}

fn validate_signal_values(
    format: ResultImportFormat,
    identity: &str,
    values: &[f64],
) -> Result<(), String> {
    validate_signal_values_from(format, identity, values, 0)
}

/// [`validate_signal_values`] for `values` that start at sample `first`.
fn validate_signal_values_from(
    format: ResultImportFormat,
    identity: &str,
    values: &[f64],
    first: usize,
) -> Result<(), String> {
    if let Some(index) = values.iter().position(|value| value.is_infinite()) {
        let index = first + index;
        return Err(adapter_error(
            format,
            format_args!("'{identity}' contains an infinite value at sample {index}"),
        ));
    }
    Ok(())
}

fn validate_coordinate(
    format: ResultImportFormat,
    analysis_type: AnalysisType,
    coordinate: &[f64],
) -> Result<(), String> {
    let mut check = CoordinateCheck::default();
    for value in coordinate {
        check.step(format, *value)?;
    }
    if analysis_type == AnalysisType::Ac && coordinate.iter().any(|value| *value <= 0.0) {
        return Err(frequency_above_zero(format));
    }
    Ok(())
}

/// Why an AC coordinate with a sample at or below zero is refused.
fn frequency_above_zero(format: ResultImportFormat) -> String {
    adapter_error(
        format,
        "frequency coordinates must all be greater than zero",
    )
}

/// A coordinate's samples checked one at a time: none repeats, and all run one way.
#[derive(Debug, Default)]
struct CoordinateCheck {
    previous: Option<f64>,
    direction: Option<std::cmp::Ordering>,
    index: usize,
}

impl CoordinateCheck {
    fn step(&mut self, format: ResultImportFormat, value: f64) -> Result<(), String> {
        let index = self.index;
        self.index += 1;
        let Some(previous) = self.previous.replace(value) else {
            return Ok(());
        };
        let step = value.total_cmp(&previous);
        // Signed zeros have different total ordering but the same coordinate.
        if value == previous {
            return Err(adapter_error(
                format,
                format_args!(
                    "coordinate repeats {previous} at samples {} and {index}",
                    index - 1
                ),
            ));
        }
        match self.direction {
            Some(expected) if step != expected => Err(adapter_error(
                format,
                format_args!("coordinate reverses direction at sample {index}"),
            )),
            Some(_) => Ok(()),
            None => {
                self.direction = Some(step);
                Ok(())
            }
        }
    }
}

/// The checks [`assemble_imported_waveforms`] makes, made on a source's rows as they arrive, a
/// block at a time: a source kept in a results store as it is read is never held whole. The
/// rules and their words are the assembler's.
#[derive(Debug)]
pub struct ImportedRowsCheck {
    format: ResultImportFormat,
    analysis_type: AnalysisType,
    limits: WaveformImportLimits,
    coordinate_name: String,
    signals: Vec<String>,
    coordinate: CoordinateCheck,
    rows: usize,
}

impl ImportedRowsCheck {
    /// Check the source's names and width before a row arrives. `signals` names each signal in
    /// source order.
    pub fn new(
        format: ResultImportFormat,
        analysis_type: AnalysisType,
        coordinate_name: impl Into<String>,
        signals: &[&str],
        limits: WaveformImportLimits,
    ) -> Result<Self, String> {
        let coordinate_name = coordinate_name.into();
        validate_name(
            format,
            "coordinate",
            &coordinate_name,
            limits.max_signal_name_bytes,
        )?;
        if signals.is_empty() {
            return Err(adapter_error(
                format,
                "the source contains no importable signals",
            ));
        }
        if signals.len() + 1 > limits.max_columns {
            return Err(adapter_error(
                format,
                format_args!(
                    "the source contains {} columns; the limit is {}",
                    signals.len() + 1,
                    limits.max_columns
                ),
            ));
        }
        let mut names = HashSet::with_capacity(signals.len());
        for name in signals {
            validate_name(format, "signal", name, limits.max_signal_name_bytes)?;
            if !names.insert(name.to_ascii_lowercase()) {
                return Err(adapter_error(
                    format,
                    format_args!("duplicate signal identity '{name}'"),
                ));
            }
        }
        Ok(Self {
            format,
            analysis_type,
            limits,
            coordinate_name,
            signals: signals.iter().map(|name| (*name).to_owned()).collect(),
            coordinate: CoordinateCheck::default(),
            rows: 0,
        })
    }

    /// Check the next rows: their coordinate, and each signal's real parts and, for a complex
    /// source, imaginary parts, in source order. Every slice holds the same number of rows.
    pub fn rows(
        &mut self,
        coordinate: &[f64],
        real: &[Vec<f64>],
        imag: &[Vec<f64>],
    ) -> Result<(), String> {
        let format = self.format;
        let first = self.rows;
        validate_finite_from(format, &self.coordinate_name, coordinate, first)?;
        for value in coordinate {
            self.coordinate.step(format, *value)?;
        }
        if self.analysis_type == AnalysisType::Ac && coordinate.iter().any(|value| *value <= 0.0) {
            return Err(frequency_above_zero(format));
        }
        for (index, name) in self.signals.iter().enumerate() {
            let real = &real[index];
            validate_signal_values_from(format, name, real, first)?;
            if let Some(imag) = imag.get(index) {
                validate_signal_values_from(
                    format,
                    &format!("{name} imaginary component"),
                    imag,
                    first,
                )?;
                if real
                    .iter()
                    .zip(imag)
                    .any(|(real, imag)| real.is_nan() != imag.is_nan())
                {
                    return Err(adapter_error(
                        format,
                        format_args!(
                            "signal '{name}' complex components have inconsistent sample \
                             availability"
                        ),
                    ));
                }
            }
        }
        self.rows += coordinate.len();
        let retained = imag.len().saturating_mul(2).saturating_add(real.len() + 1);
        let values = self.rows.saturating_mul(retained);
        if values > self.limits.max_values {
            return Err(adapter_error(
                format,
                format_args!(
                    "the source expands to {values} numeric values; the limit is {}",
                    self.limits.max_values
                ),
            ));
        }
        Ok(())
    }

    /// The rows the source held, once every one has arrived and been checked.
    pub fn finish(self) -> Result<usize, String> {
        let format = self.format;
        let coordinate_name = &self.coordinate_name;
        if self.rows < self.limits.min_rows {
            return Err(adapter_error(
                format,
                format_args!(
                    "the '{coordinate_name}' coordinate carries no samples, so the source holds \
                     no result to import"
                ),
            ));
        }
        if self.rows > self.limits.max_rows {
            return Err(adapter_error(
                format,
                format_args!(
                    "the coordinate contains {} samples; the limit is {}",
                    self.rows, self.limits.max_rows
                ),
            ));
        }
        Ok(self.rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A source checked a block at a time is refused for what the assembler refuses it for, in
    /// the same words, with each sample numbered from the source's first row.
    #[test]
    fn rows_checked_in_blocks_are_refused_as_the_assembler_refuses_them() {
        let limits = WaveformImportLimits {
            min_rows: 1,
            max_rows: 100,
            max_columns: 10,
            max_values: 1_000,
            max_signal_name_bytes: 64,
        };
        let format = ResultImportFormat::SpiceRaw;
        let assembled = |coordinate: Vec<f64>, real: Vec<f64>| {
            assemble_imported_waveforms(
                format,
                AnalysisType::Ac,
                "frequency",
                coordinate,
                vec![ImportedSignal {
                    name: "V(out)".to_owned(),
                    real,
                    imag: None,
                    unit: None,
                }],
                limits,
            )
            .map(|_| ())
        };
        let streamed = |coordinate: Vec<f64>, real: Vec<f64>| {
            let mut check =
                ImportedRowsCheck::new(format, AnalysisType::Ac, "frequency", &["V(out)"], limits)?;
            for first in (0..coordinate.len()).step_by(2) {
                let last = (first + 2).min(coordinate.len());
                check.rows(&coordinate[first..last], &[real[first..last].to_vec()], &[])?;
            }
            check.finish().map(|_| ())
        };
        for (coordinate, real) in [
            (vec![1.0, 2.0, 3.0, 4.0, 5.0], vec![0.0; 5]),
            (vec![1.0, 2.0, 3.0, 3.0, 5.0], vec![0.0; 5]),
            (vec![1.0, 2.0, 3.0, 2.5, 1.0], vec![0.0; 5]),
            (vec![1.0, 2.0, 3.0, f64::NAN, 5.0], vec![0.0; 5]),
            (
                vec![1.0, 2.0, 3.0, 4.0, 5.0],
                vec![0.0, 0.0, 0.0, f64::INFINITY, 0.0],
            ),
            (vec![-1.0, 2.0, 3.0], vec![0.0; 3]),
            (Vec::new(), Vec::new()),
        ] {
            assert_eq!(
                streamed(coordinate.clone(), real.clone()),
                assembled(coordinate.clone(), real.clone()),
                "{coordinate:?}"
            );
        }
        assert_eq!(
            ImportedRowsCheck::new(format, AnalysisType::Ac, "frequency", &["a", "A"], limits)
                .unwrap_err(),
            assembled_names_error(limits)
        );
    }

    fn assembled_names_error(limits: WaveformImportLimits) -> String {
        let signal = |name: &str| ImportedSignal {
            name: name.to_owned(),
            real: vec![0.0],
            imag: None,
            unit: None,
        };
        assemble_imported_waveforms(
            ResultImportFormat::SpiceRaw,
            AnalysisType::Ac,
            "frequency",
            vec![1.0],
            vec![signal("a"), signal("A")],
            limits,
        )
        .unwrap_err()
    }

    #[test]
    fn signed_zero_is_one_coordinate_but_keeps_its_sample_bits() {
        let import = |kind, coordinate: Vec<f64>| {
            let rows = coordinate.len();
            assemble_imported_waveforms(
                ResultImportFormat::Hdf5,
                kind,
                "axis",
                coordinate,
                vec![ImportedSignal {
                    name: "out".into(),
                    real: vec![-0.0; rows],
                    imag: None,
                    unit: None,
                }],
                WaveformImportLimits {
                    min_rows: 1,
                    max_rows: 3,
                    max_columns: 2,
                    max_values: 6,
                    max_signal_name_bytes: 32,
                },
            )
        };
        for kind in [AnalysisType::Transient, AnalysisType::DcSweep] {
            for coordinate in [vec![-0.0, 0.0], vec![0.0, -0.0]] {
                let error = import(kind, coordinate).unwrap_err();
                assert!(error.contains("coordinate repeats"), "{error}");
            }
            for coordinate in [vec![-1.0, -0.0, 1.0], vec![1.0, -0.0, -1.0]] {
                let restored = import(kind, coordinate).unwrap();
                assert_eq!(
                    restored.waveforms[0].x.sample(1).map(f64::to_bits),
                    Some((-0.0_f64).to_bits())
                );
                assert!(
                    restored.waveforms[0]
                        .y
                        .samples(..)
                        .all(|v| v.to_bits() == (-0.0_f64).to_bits())
                );
            }
        }
    }

    #[test]
    fn retained_value_limits_count_shared_coordinates_and_each_owned_sample_array() {
        for (complex, expected) in [
            (vec![false], 4),
            (vec![true], 8),
            (vec![false, false], 6),
            (vec![true, false], 10),
            (vec![true, true], 14),
        ] {
            for maximum in [expected - 1, expected] {
                let signals = complex
                    .iter()
                    .enumerate()
                    .map(|(index, complex)| ImportedSignal {
                        name: format!("signal{index}"),
                        real: vec![3.0, 4.0],
                        imag: complex.then(|| vec![4.0, 3.0]),
                        unit: None,
                    })
                    .collect();
                let result = assemble_imported_waveforms(
                    ResultImportFormat::Hdf5,
                    AnalysisType::Ac,
                    "frequency",
                    vec![1.0, 2.0],
                    signals,
                    WaveformImportLimits {
                        min_rows: 1,
                        max_rows: 2,
                        max_columns: 3,
                        max_values: maximum,
                        max_signal_name_bytes: 32,
                    },
                );
                if maximum == expected {
                    let decoded = result.unwrap();
                    let actual = decoded.waveforms[0].x.len()
                        + decoded
                            .waveforms
                            .iter()
                            .map(|waveform| {
                                waveform.y.len()
                                    + waveform
                                        .complex
                                        .as_ref()
                                        .map_or(0, |parts| parts.real.len() + parts.imag.len())
                            })
                            .sum::<usize>();
                    assert_eq!(actual, expected);
                } else {
                    let error = result.unwrap_err();
                    assert!(
                        error.contains(&format!("expands to {expected} numeric values")),
                        "{error}"
                    );
                }
            }
        }
    }

    #[test]
    fn unavailable_samples_survive_assembly_and_calculation_without_becoming_zeroes() {
        let limits = WaveformImportLimits {
            min_rows: 1,
            max_rows: 5,
            max_columns: 3,
            max_values: 25,
            max_signal_name_bytes: 32,
        };
        let make_signal = |real, imag| ImportedSignal {
            name: "V(out)".into(),
            real,
            imag,
            unit: Some("V".into()),
        };
        for imag in [None, Some(vec![0.0, f64::NAN, 0.0])] {
            let imported = assemble_imported_waveforms(
                ResultImportFormat::SpiceRaw,
                AnalysisType::Transient,
                "time",
                vec![0.0, 1.0, 2.0],
                vec![make_signal(vec![1.0, f64::NAN, -0.0], imag)],
                limits,
            )
            .unwrap();
            let waveform = &imported.waveforms[0];
            assert!(waveform.has_missing_samples());
            assert_eq!(waveform.sample(1), None);
            assert_eq!(waveform.sample(0), Some(1.0));
            assert_eq!(waveform, &waveform.clone());
            let value = crate::calculator::retained::waveform_value(
                waveform,
                crate::saved_output::ComplexExpressionPolicy::Rectangular,
            )
            .unwrap();
            match value {
                crate::calculator::value::CalcValue::Real(
                    crate::calculator::value::RealValue::Waveform(_, values),
                ) => assert!(values[1].is_nan()),
                crate::calculator::value::CalcValue::Complex(
                    crate::calculator::value::ComplexValue::Waveform(_, values),
                ) => assert!(values[1].re.is_nan() && values[1].im.is_nan()),
                _ => panic!("expected sampled values"),
            }
            #[cfg(feature = "engine-evidence")]
            {
                let mut analysis = crate::analysis_result::AnalysisResult::new(
                    1,
                    AnalysisType::Transient,
                    "TRAN",
                    0.0,
                )
                .with_waveforms(imported.waveforms);
                assert!(
                    analysis.validate_retained_evidence().is_err(),
                    "native solver NaNs must still be rejected"
                );
                analysis.import_source = Some(crate::result_import::ResultImportSource {
                    source_name: "gaps.raw".into(),
                    format: ResultImportFormat::SpiceRaw,
                    coordinate: None,
                    family_member: None,
                    event_dump: None,
                });
                analysis.validate_retained_evidence().unwrap();
            }
        }
        for (real, imag) in [
            (vec![f64::INFINITY], None),
            (vec![0.0], Some(vec![f64::NAN])),
            (vec![f64::NAN], Some(vec![0.0])),
        ] {
            assert!(
                assemble_imported_waveforms(
                    ResultImportFormat::SpiceRaw,
                    AnalysisType::Transient,
                    "time",
                    vec![0.0],
                    vec![make_signal(real, imag)],
                    limits
                )
                .is_err()
            );
        }
    }

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
        let (real, imag) = (
            complex.real.in_memory().unwrap(),
            complex.imag.in_memory().unwrap(),
        );
        assert_eq!(real.as_ptr(), real_ptr);
        assert_eq!(imag.as_ptr(), imag_ptr);
        assert_eq!(real[0].to_bits(), (-0.0_f64).to_bits());
        assert_eq!(real[1].to_bits(), 1);
        assert_eq!(complex.source_name, "V(out)");
        assert_eq!(imported.waveforms[0].name, "|V(out)|");
        assert_eq!(imported.waveforms[0].unit.as_deref(), Some("V"));
        assert_eq!(
            imported.waveforms[1].y.in_memory().unwrap().as_ptr(),
            scalar_ptr
        );
        assert!(crate::waveform::WaveformValues::ptr_eq(
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
            "NumPy .npz import: duplicate signal identity 'v(OUT)'"
        );
    }
}

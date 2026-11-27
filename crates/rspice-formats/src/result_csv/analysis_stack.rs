//! Long-form CSV preserves each selected analysis and its independent coordinate grid.
//!
//! One row per sample of each trace component, written as the samples are read: a block of a
//! component's coordinate and samples at a time, so a run kept in a results store exports
//! whatever its length. A member of a `.step` sweep states the value each swept parameter takes
//! there in a column of that parameter's name, and a Monte Carlo trial its number in `trial`;
//! an analysis that is no member, or does not sweep that parameter, leaves the cell empty.

use std::io::Write;

use crate::table::sanitize_column_label;
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::provenance::AnalysisResultFamilyMember;
use rspice_results::waveform::{BLOCK_SAMPLES, RetainedWaveform, WaveformValues};
use rspice_vocabulary::product::DatasetId;

#[derive(Debug)]
pub enum AnalysisStackCsvError {
    NoSamples,
    SampleCountMismatch {
        trace: String,
        component: String,
        coordinates: usize,
        samples: usize,
    },
    NonFiniteSample {
        trace: String,
        component: String,
        sample_index: usize,
    },
    /// A trace's samples could not be read from its results store.
    Unreadable {
        trace: String,
        reason: String,
    },
    /// The destination refused a write.
    Write(String),
}

impl std::fmt::Display for AnalysisStackCsvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSamples => f.write_str("No waveform samples available to export."),
            Self::SampleCountMismatch {
                trace,
                component,
                coordinates,
                samples,
            } => write!(
                f,
                "Displayed trace '{}' has {} coordinates and {} {component} samples; export refused instead of truncating evidence.",
                sanitize_column_label(trace),
                coordinates,
                samples,
            ),
            Self::NonFiniteSample {
                trace,
                component,
                sample_index,
            } => write!(
                f,
                "Displayed trace '{}' contains a non-finite {component} sample at index {sample_index}.",
                sanitize_column_label(trace),
            ),
            Self::Unreadable { trace, reason } => write!(
                f,
                "Displayed trace '{}' could not be read: {reason}",
                sanitize_column_label(trace),
            ),
            Self::Write(reason) => write!(f, "The table could not be written: {reason}"),
        }
    }
}

impl std::error::Error for AnalysisStackCsvError {}

/// The field separator of a long-form table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackDelimiter {
    Comma,
    Tab,
}

/// One analysis of the stack and the traces of it the table writes.
pub struct StackAnalysis<'a, W> {
    pub analysis: &'a AnalysisResult<W>,
    pub waveforms: Vec<&'a W>,
}

/// What a long-form table holds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisStackFacts {
    /// Traces written, a complex one counted once.
    pub traces: usize,
    /// Sample rows written.
    pub rows: usize,
    /// The parameter columns, in the order the header names them.
    pub parameters: Vec<String>,
}

/// One component of one trace: the trace's name, the component, and its samples.
type Component<'a> = (&'a str, &'static str, &'a WaveformValues);

fn components<'a, W: AsRef<RetainedWaveform>>(
    waveforms: &[&'a W],
) -> impl Iterator<Item = (&'a WaveformValues, Component<'a>)> {
    waveforms.iter().flat_map(|waveform| {
        let waveform = (*waveform).as_ref();
        std::iter::once((
            &waveform.x,
            (waveform.name.as_str(), "display", &waveform.y),
        ))
        .chain(waveform.complex.iter().flat_map(move |complex| {
            [
                (
                    &waveform.x,
                    (complex.source_name.as_str(), "real", &complex.real),
                ),
                (
                    &waveform.x,
                    (complex.source_name.as_str(), "imaginary", &complex.imag),
                ),
            ]
        }))
    })
}

/// The parameter columns of `analyses`: every swept parameter's name in the order the members
/// first state them, then `trial` when a Monte Carlo trial is among them.
fn parameters<W: AsRef<RetainedWaveform>>(analyses: &[StackAnalysis<'_, W>]) -> Vec<String> {
    let mut names = Vec::<String>::new();
    let mut trial = false;
    for stacked in analyses {
        match stacked.analysis.family_member() {
            Some(AnalysisResultFamilyMember::StepPoint { coordinate, .. }) => {
                for axis in coordinate {
                    if !names.contains(&axis.name) {
                        names.push(axis.name.clone());
                    }
                }
            }
            Some(AnalysisResultFamilyMember::MonteCarloTrial { .. }) => trial = true,
            None => {}
        }
    }
    if trial && !names.iter().any(|name| name == "trial") {
        names.push("trial".to_owned());
    }
    names
}

/// The cells `analysis` states under `parameters`.
fn parameter_cells<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
    parameters: &[String],
) -> Vec<String> {
    parameters
        .iter()
        .map(|name| match analysis.family_member() {
            Some(AnalysisResultFamilyMember::StepPoint { coordinate, .. }) => coordinate
                .iter()
                .find(|axis| &axis.name == name)
                .map_or_else(String::new, |axis| axis.value.to_string()),
            Some(AnalysisResultFamilyMember::MonteCarloTrial { index }) if name == "trial" => {
                index.to_string()
            }
            _ => String::new(),
        })
        .collect()
}

/// Whether `analyses` make a long-form table, checked without reading a sample: each
/// component's samples match its coordinate's count, and there is a row to write.
pub fn check_analysis_stack<W: AsRef<RetainedWaveform>>(
    analyses: &[StackAnalysis<'_, W>],
) -> Result<AnalysisStackFacts, AnalysisStackCsvError> {
    let mut rows = 0_usize;
    let mut traces = 0;
    for stacked in analyses {
        traces += stacked.waveforms.len();
        for (x, (trace, component, y)) in components(&stacked.waveforms) {
            if x.len() != y.len() {
                return Err(AnalysisStackCsvError::SampleCountMismatch {
                    trace: trace.to_owned(),
                    component: component.to_owned(),
                    coordinates: x.len(),
                    samples: y.len(),
                });
            }
            rows = rows.saturating_add(x.len());
        }
    }
    if rows == 0 {
        return Err(AnalysisStackCsvError::NoSamples);
    }
    Ok(AnalysisStackFacts {
        traces,
        rows,
        parameters: parameters(analyses),
    })
}

/// `analyses` as one long-form table written to `out` as their samples are read. A sample that
/// cannot be written or read stops the write; what reached `out` is then a partial table its
/// caller discards.
pub fn write_analysis_stack<W: AsRef<RetainedWaveform>>(
    dataset_id: DatasetId,
    analyses: &[StackAnalysis<'_, W>],
    delimiter: StackDelimiter,
    out: &mut dyn Write,
) -> Result<AnalysisStackFacts, AnalysisStackCsvError> {
    let mut facts = check_analysis_stack(analyses)?;
    let written = |error: csv::Error| AnalysisStackCsvError::Write(error.to_string());
    let mut table = csv::WriterBuilder::new()
        .delimiter(match delimiter {
            StackDelimiter::Comma => b',',
            StackDelimiter::Tab => b'\t',
        })
        .flexible(false)
        .from_writer(out);
    let header = [
        "dataset_id",
        "analysis_sequence",
        "analysis_label",
        "analysis_type",
    ]
    .into_iter()
    .map(str::to_owned)
    .chain(facts.parameters.iter().cloned())
    .chain(
        ["trace", "component", "sample_index", "x", "y"]
            .into_iter()
            .map(str::to_owned),
    )
    .collect::<Vec<_>>();
    table.write_record(&header).map_err(written)?;
    let dataset = dataset_id.to_string();
    let mut rows = 0;
    let mut record = Vec::with_capacity(header.len());
    for stacked in analyses {
        let analysis = stacked.analysis;
        let sequence = analysis.id.to_string();
        let cells = parameter_cells(analysis, &facts.parameters);
        let analysis_type = analysis.analysis_type.short_label();
        for (x, (trace, component, y)) in components(&stacked.waveforms) {
            let unreadable = |failure: rspice_results::waveform::ColumnReadFailure| {
                AnalysisStackCsvError::Unreadable {
                    trace: trace.to_owned(),
                    reason: failure.to_string(),
                }
            };
            let mut first = 0;
            while first < x.len() {
                let end = first.saturating_add(BLOCK_SAMPLES).min(x.len());
                let xs = read_block(x, first..end).map_err(unreadable)?;
                let ys = read_block(y, first..end).map_err(unreadable)?;
                for (offset, (&x, &y)) in xs.iter().zip(ys.iter()).enumerate() {
                    let sample_index = first + offset;
                    if !x.is_finite() || y.is_infinite() {
                        return Err(AnalysisStackCsvError::NonFiniteSample {
                            trace: trace.to_owned(),
                            component: component.to_owned(),
                            sample_index,
                        });
                    }
                    record.clear();
                    record.extend([
                        dataset.clone(),
                        sequence.clone(),
                        analysis.label.clone(),
                        analysis_type.to_owned(),
                    ]);
                    record.extend(cells.iter().cloned());
                    record.extend([
                        trace.to_owned(),
                        component.to_owned(),
                        sample_index.to_string(),
                        format!("{x:.17e}"),
                        if y.is_nan() {
                            String::new()
                        } else {
                            format!("{y:.17e}")
                        },
                    ]);
                    table.write_record(&record).map_err(written)?;
                    rows += 1;
                }
                first = end;
            }
        }
    }
    table
        .flush()
        .map_err(|error| AnalysisStackCsvError::Write(error.to_string()))?;
    facts.rows = rows;
    Ok(facts)
}

/// Samples `window` of `values`: borrowed from memory, or one block read from its store.
fn read_block(
    values: &WaveformValues,
    window: std::ops::Range<usize>,
) -> Result<std::borrow::Cow<'_, [f64]>, rspice_results::waveform::ColumnReadFailure> {
    match values.blocks(window).next() {
        Some(block) => block,
        None => Ok(std::borrow::Cow::Borrowed(&[])),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_results::analysis_type::AnalysisType;

    fn dataset() -> DatasetId {
        "d24e84c0-0000-4000-8000-000000000001".parse().unwrap()
    }

    fn written(
        analyses: &[StackAnalysis<'_, RetainedWaveform>],
    ) -> Result<(Vec<csv::StringRecord>, AnalysisStackFacts), AnalysisStackCsvError> {
        let mut bytes = Vec::new();
        let facts = write_analysis_stack(dataset(), analyses, StackDelimiter::Comma, &mut bytes)?;
        let rows = csv::Reader::from_reader(bytes.as_slice())
            .records()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        Ok((rows, facts))
    }

    #[test]
    fn independent_grids_and_component_errors_preserve_exact_rows_and_diagnostics() {
        let first = AnalysisResult::new(1, AnalysisType::Transient, "A,\"one\"", 0.0)
            .with_waveforms(vec![RetainedWaveform::new("V(a)", vec![0.0], vec![-0.0])]);
        let second = AnalysisResult::new(2, AnalysisType::Ac, "B", 0.0).with_waveforms(vec![
            RetainedWaveform::new("|V(b)|", vec![10.0, 20.0], vec![5.0, 5.0])
                .with_complex_components("V(b)", vec![3.0, 4.0], vec![4.0, 3.0]),
        ]);
        let (rows, facts) = written(&[
            StackAnalysis {
                analysis: &first,
                waveforms: first.waveforms.iter().collect(),
            },
            StackAnalysis {
                analysis: &second,
                waveforms: second.waveforms.iter().collect(),
            },
        ])
        .unwrap();
        assert_eq!(facts.rows, 7);
        assert_eq!(facts.traces, 2);
        assert!(facts.parameters.is_empty());
        assert_eq!(&rows[0][2], "A,\"one\"");
        assert_eq!(&rows[0][8], "-0.00000000000000000e0");
        assert_eq!(&rows[1][1], "2");
        assert_eq!(&rows[1][5], "display");
        assert_eq!(&rows[3][4], "V(b)");
        assert_eq!(&rows[3][5], "real");
        assert_eq!(&rows[5][5], "imaginary");
        assert_eq!(&rows[5][7], "1.00000000000000000e1");
        assert_eq!(&rows[6][8], "3.00000000000000000e0");

        let ragged =
            AnalysisResult::new(1, AnalysisType::Transient, "A", 0.0).with_waveforms(vec![
                RetainedWaveform::new(" V(a,\nb) ", vec![f64::NAN], vec![]),
            ]);
        let error = written(&[StackAnalysis {
            analysis: &ragged,
            waveforms: ragged.waveforms.iter().collect(),
        }])
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "Displayed trace 'V(a b)' has 1 coordinates and 0 display samples; export refused instead of truncating evidence."
        );
        let infinite =
            AnalysisResult::new(1, AnalysisType::Transient, "A", 0.0).with_waveforms(vec![
                RetainedWaveform::new("V(a,b)", vec![0.0, 1.0], vec![2.0, f64::INFINITY]),
            ]);
        let error = written(&[StackAnalysis {
            analysis: &infinite,
            waveforms: infinite.waveforms.iter().collect(),
        }])
        .unwrap_err();
        assert_eq!(
            error.to_string(),
            "Displayed trace 'V(a b)' contains a non-finite display sample at index 1."
        );
    }

    /// Each member of a `.step` states its parameter values in a column per swept parameter,
    /// and a tab-separated table carries the same cells.
    #[test]
    fn every_member_states_its_parameter_values_in_a_column_per_swept_parameter() {
        let members = [1000.0, 2000.0, 4000.0]
            .into_iter()
            .enumerate()
            .map(|(index, r)| {
                let mut analysis = AnalysisResult::new(
                    u64::try_from(index).unwrap() + 1,
                    AnalysisType::Transient,
                    "TRAN",
                    0.0,
                )
                .with_waveforms(vec![RetainedWaveform::new(
                    "V(out)",
                    vec![0.0, 1.0],
                    vec![r, -r],
                )]);
                analysis.import_source = Some(rspice_results::result_import::ResultImportSource {
                    source_name: "step.raw".into(),
                    format: rspice_results::result_import::ResultImportFormat::SpiceRaw,
                    coordinate: None,
                    family_member: Some(AnalysisResultFamilyMember::StepPoint {
                        index,
                        coordinate: vec![
                            rspice_results::provenance::AnalysisResultStepCoordinate {
                                name: "r".to_owned(),
                                value: r,
                                unit: String::new(),
                            },
                        ],
                    }),
                    event_dump: None,
                });
                analysis
            })
            .collect::<Vec<_>>();
        let stacked = members
            .iter()
            .map(|analysis| StackAnalysis {
                analysis,
                waveforms: analysis.waveforms.iter().collect(),
            })
            .collect::<Vec<_>>();
        let (rows, facts) = written(&stacked).unwrap();
        assert_eq!(facts.parameters, ["r"]);
        assert_eq!(rows.len(), 6);
        for (row, (r, y)) in rows.iter().zip([
            ("1000", "1.00000000000000000e3"),
            ("1000", "-1.00000000000000000e3"),
            ("2000", "2.00000000000000000e3"),
            ("2000", "-2.00000000000000000e3"),
            ("4000", "4.00000000000000000e3"),
            ("4000", "-4.00000000000000000e3"),
        ]) {
            assert_eq!(&row[4], r);
            assert_eq!(&row[5], "V(out)");
            assert_eq!(&row[9], y);
        }
        let mut tab = Vec::new();
        write_analysis_stack(dataset(), &stacked, StackDelimiter::Tab, &mut tab).unwrap();
        let tab = String::from_utf8(tab).unwrap();
        let mut lines = tab.lines();
        assert_eq!(
            lines.next(),
            Some(
                "dataset_id\tanalysis_sequence\tanalysis_label\tanalysis_type\tr\ttrace\tcomponent\tsample_index\tx\ty"
            )
        );
        assert!(
            lines
                .next()
                .unwrap()
                .contains("\t1000\tV(out)\tdisplay\t0\t")
        );
    }
}

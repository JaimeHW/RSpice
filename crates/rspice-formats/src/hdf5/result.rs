//! HDF5 projections through the canonical writer shared with the CLI.

use rspice_core::io::{
    Hdf5Attribute, Hdf5Column, Hdf5Coordinate, Hdf5Document, Hdf5Error, Hdf5Table,
};
use rspice_results::analysis_result::AnalysisResult;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::waveform::RetainedWaveform;

// Preserve the product's existing import limits so exported files can be reopened.
const MAX_COLUMNS: usize = 1_024;
const MAX_ROWS: usize = 1_000_000;

#[derive(Debug)]
pub enum Hdf5ProjectionError {
    UnsupportedAnalysis {
        analysis: AnalysisType,
        label: String,
    },
    NoSamples,
    RowLimit {
        rows: usize,
    },
    ColumnLimit {
        columns: usize,
    },
    DifferentCoordinates {
        signal: String,
    },
    InvalidTable(Hdf5Error),
}

impl std::fmt::Display for Hdf5ProjectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedAnalysis { label, .. } => write!(
                f,
                "An HDF5 dataset carries one sampled analysis: a transient, a DC sweep or an AC \
                 sweep. '{label}' is none of those, and publishing it under one of those section names \
                 would make every reader call it something it is not. Export CSV, or an RSpice \
                 bundle, which carries this analysis whole."
            ),
            Self::NoSamples => f.write_str("No waveform samples available to export."),
            Self::RowLimit { rows } => write!(
                f,
                "This result has {rows} samples; RSpice reads at most {MAX_ROWS} from an HDF5 source, \
                 so publishing it would produce a file this build could not reopen."
            ),
            Self::ColumnLimit { columns } => write!(
                f,
                "This result has {columns} columns; RSpice reads at most {MAX_COLUMNS} from an HDF5 \
                 source. Hide traces, or export an RSpice bundle."
            ),
            Self::DifferentCoordinates { signal } => write!(
                f,
                "An HDF5 section is one table, so every column must stand on the same \
                 coordinate samples. '{signal}' carries its own x-axis samples. Export this result \
                 as CSV or an RSpice bundle instead."
            ),
            Self::InvalidTable(source) => write!(
                f,
                "This result cannot be published as an HDF5 dataset: {source}."
            ),
        }
    }
}

impl std::error::Error for Hdf5ProjectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::InvalidTable(source) => Some(source),
            _ => None,
        }
    }
}

/// One prepared document, and what the reader is owed about it.
#[derive(Debug)]
pub struct Hdf5Export {
    pub document: Hdf5Document,
    pub section: &'static str,
    pub coordinate_name: String,
    pub rows: usize,
    pub columns: usize,
    /// Columns published with a zero imaginary part because the displayed
    /// trace retained none.
    pub zeroed_imaginary: Vec<String>,
}

/// The section name an analysis publishes under, and whether that section is
/// spectral (complex columns) rather than sampled (real ones).
const fn section_for(analysis: AnalysisType) -> Option<(&'static str, bool)> {
    match analysis {
        AnalysisType::Transient => Some(("transient", false)),
        AnalysisType::DcSweep => Some(("dc_sweep", false)),
        AnalysisType::Ac => Some(("ac", true)),
        _ => None,
    }
}

/// The `signal_NNNN_type` attribute: what kind of quantity a column holds.
fn quantity(name: &str) -> String {
    match crate::waveform_io::result::signal_type_from_waveform_name(name) {
        crate::waveform_io::SignalType::Voltage => "voltage",
        crate::waveform_io::SignalType::Current => "current",
        _ => "value",
    }
    .to_owned()
}

pub fn prepare_hdf5<W: AsRef<RetainedWaveform>>(
    analysis: &AnalysisResult<W>,
    waveforms: &[&W],
) -> Result<Hdf5Export, Hdf5ProjectionError> {
    let Some((section, spectral)) = section_for(analysis.analysis_type) else {
        return Err(Hdf5ProjectionError::UnsupportedAnalysis {
            analysis: analysis.analysis_type,
            label: analysis.label.clone(),
        });
    };
    let reference = waveforms
        .iter()
        .map(|waveform| (*waveform).as_ref())
        .filter(|waveform| !waveform.x.is_empty())
        .max_by_key(|waveform| waveform.x.len())
        .ok_or(Hdf5ProjectionError::NoSamples)?;
    let coordinate = reference.x.as_ref().to_vec();
    if coordinate.len() > MAX_ROWS {
        return Err(Hdf5ProjectionError::RowLimit {
            rows: coordinate.len(),
        });
    }
    if waveforms.len() + 1 > MAX_COLUMNS {
        return Err(Hdf5ProjectionError::ColumnLimit {
            columns: waveforms.len() + 1,
        });
    }

    let mut columns = Vec::with_capacity(waveforms.len());
    let mut zeroed_imaginary = Vec::new();
    for waveform in waveforms {
        let waveform = (*waveform).as_ref();
        // A section is one table, so a column that does not stand on the
        // shared coordinate has no honest place in it.
        if waveform.x.as_ref() != coordinate.as_slice() {
            return Err(Hdf5ProjectionError::DifferentCoordinates {
                signal: waveform.name.clone(),
            });
        }
        if !spectral {
            columns.push(Hdf5Column::Real {
                name: waveform.name.clone(),
                quantity: quantity(&waveform.name),
                unit: waveform.unit.clone(),
                values: waveform.y.as_ref().to_vec(),
            });
            continue;
        }
        match &waveform.complex {
            Some(complex) => columns.push(Hdf5Column::Complex {
                name: complex.source_name.clone(),
                unit: waveform.unit.clone(),
                real: complex.real.as_ref().to_vec(),
                imag: complex.imag.as_ref().to_vec(),
            }),
            None => {
                zeroed_imaginary.push(waveform.name.clone());
                columns.push(Hdf5Column::Complex {
                    name: waveform.name.clone(),
                    unit: waveform.unit.clone(),
                    real: waveform.y.as_ref().to_vec(),
                    imag: vec![0.0; coordinate.len()],
                });
            }
        }
    }
    if columns.is_empty() {
        return Err(Hdf5ProjectionError::NoSamples);
    }

    let coordinate_name = crate::waveform_io::result::axis_signal_for_analysis(analysis)
        .0
        .to_owned();
    let rows = coordinate.len();
    let count = columns.len();
    let mut document = Hdf5Document::new(analysis.label.clone());
    document
        .add_table(&Hdf5Table {
            group: section.to_owned(),
            section_type: section.to_owned(),
            coordinate: if spectral {
                Hdf5Coordinate::Frequency(coordinate)
            } else {
                Hdf5Coordinate::Independent {
                    name: coordinate_name.clone(),
                    values: coordinate,
                }
            },
            columns,
        })
        .map_err(Hdf5ProjectionError::InvalidTable)?;
    let group = document.groups.last_mut().expect("table was added");
    if spectral && coordinate_name != "frequency" {
        group.set_attr(
            "independent_name",
            Hdf5Attribute::Text(coordinate_name.clone()),
        );
    }
    if let Some(unit) = analysis.waveform_coordinate_unit() {
        group.set_attr("coordinate_unit", Hdf5Attribute::Text(unit.to_owned()));
    }
    Ok(Hdf5Export {
        document,
        section,
        coordinate_name,
        rows,
        columns: count,
        zeroed_imaginary,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdf5::{Hdf5Limits, decode_hdf5};
    use rspice_results::result_import::{
        ResultImportCoordinate, ResultImportFormat, ResultImportSource,
    };

    #[test]
    fn retained_hdf5_round_trip_preserves_coordinate_identity() {
        for (kind, name, unit) in [
            (AnalysisType::Transient, "Elapsed time", Some("s")),
            (AnalysisType::Transient, "Clock", None),
            (AnalysisType::DcSweep, "bias", Some("A")),
            (AnalysisType::DcSweep, "ambient", Some("K")),
            (AnalysisType::DcSweep, "control", None),
            (AnalysisType::Ac, "Test frequency", Some("Hz")),
            (AnalysisType::Ac, "Tone", None),
        ] {
            let mut waveform = RetainedWaveform::new("out", vec![1.0, 2.0], vec![-0.0, 4.0]);
            if kind == AnalysisType::Ac {
                waveform =
                    waveform.with_complex_components("out", vec![-0.0, 4.0], vec![2.0, -1.0]);
            }
            let mut analysis = AnalysisResult::new(1, kind, "Retained coordinate", 0.0)
                .with_waveforms(vec![waveform]);
            analysis.import_source = Some(ResultImportSource {
                source_name: "source.h5".into(),
                format: ResultImportFormat::Hdf5,
                coordinate: Some(ResultImportCoordinate {
                    name: name.into(),
                    unit: unit.map(str::to_owned),
                }),
            });
            let export =
                prepare_hdf5(&analysis, &analysis.waveforms.iter().collect::<Vec<_>>()).unwrap();
            let mut bytes = Vec::new();
            rspice_core::io::write_hdf5(&mut bytes, &export.document).unwrap();
            let decoded = decode_hdf5(
                &bytes,
                Hdf5Limits {
                    max_columns: 10,
                    max_values: 100,
                    coordinate_names: &["time", "frequency", "x"],
                },
                "hdf5",
            )
            .unwrap();
            assert_eq!(decoded.coordinate_name, name);
            assert_eq!(export.coordinate_name, name);
            assert_eq!(decoded.coordinate_unit.as_deref(), unit);
            assert_eq!(decoded.coordinate, [1.0, 2.0]);
            assert_eq!(decoded.signals[0].unit, None);
            assert_eq!(decoded.signals[0].real[0].to_bits(), (-0.0_f64).to_bits());
            if kind == AnalysisType::Ac {
                assert_eq!(
                    decoded.signals[0].imag.as_deref(),
                    Some([2.0, -1.0].as_slice())
                );
            }
        }
    }
}

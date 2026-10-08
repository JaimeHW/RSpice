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
}

/// The section name an analysis publishes under.
const fn section_for(analysis: AnalysisType) -> Option<&'static str> {
    match analysis {
        AnalysisType::Transient => Some("transient"),
        AnalysisType::DcSweep => Some("dc_sweep"),
        AnalysisType::Ac => Some("ac"),
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
    let Some(section) = section_for(analysis.analysis_type) else {
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

    // The native AC layout requires complex columns. Typed tables represent
    // mixed real/complex traces on any coordinate without fabricating phase
    // or replacing retained rectangular samples with display magnitudes.
    let spectral = analysis.analysis_type == AnalysisType::Ac
        && waveforms
            .iter()
            .all(|waveform| waveform.as_ref().complex.is_some());
    let typed_table = !spectral
        && (analysis.analysis_type == AnalysisType::Ac
            || waveforms
                .iter()
                .any(|waveform| waveform.as_ref().complex.is_some()));
    let column_count: usize = waveforms
        .iter()
        .map(|waveform| {
            if typed_table && waveform.as_ref().complex.is_some() {
                2
            } else {
                1
            }
        })
        .sum();
    if column_count + 1 > MAX_COLUMNS {
        return Err(Hdf5ProjectionError::ColumnLimit {
            columns: column_count + 1,
        });
    }
    let mut columns = Vec::with_capacity(column_count);
    for waveform in waveforms {
        let waveform = (*waveform).as_ref();
        // A section is one table, so a column that does not stand on the
        // shared coordinate has no honest place in it.
        if waveform.x.as_ref() != coordinate.as_slice() {
            return Err(Hdf5ProjectionError::DifferentCoordinates {
                signal: waveform.name.clone(),
            });
        }
        match &waveform.complex {
            Some(complex) if spectral => columns.push(Hdf5Column::Complex {
                name: complex.source_name.clone(),
                unit: waveform.unit.clone(),
                real: complex.real.as_ref().to_vec(),
                imag: complex.imag.as_ref().to_vec(),
            }),
            Some(complex) => {
                for (label, part, values) in
                    [("Re", "real", &complex.real), ("Im", "imag", &complex.imag)]
                {
                    columns.push(Hdf5Column::Real {
                        name: format!("{label}({})", complex.source_name),
                        quantity: format!("complex_{part}:{}", quantity(&complex.source_name)),
                        unit: waveform.unit.clone(),
                        values: values.as_ref().to_vec(),
                    });
                }
            }
            None => columns.push(Hdf5Column::Real {
                name: waveform.name.clone(),
                quantity: quantity(&waveform.name),
                unit: waveform.unit.clone(),
                values: waveform.y.as_ref().to_vec(),
            }),
        }
    }
    if columns.is_empty() {
        return Err(Hdf5ProjectionError::NoSamples);
    }

    let coordinate_name = crate::waveform_io::result::axis_signal_for_analysis(analysis)
        .0
        .to_owned();
    let rows = coordinate.len();
    let mut document = Hdf5Document::new(analysis.label.clone());
    document
        .add_table(&Hdf5Table {
            group: section.to_owned(),
            section_type: if typed_table { "table" } else { section }.to_owned(),
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
    if typed_table {
        group.set_attr("analysis", Hdf5Attribute::Text(section.to_owned()));
        group.set_attr(
            "coordinate_type",
            Hdf5Attribute::Text(
                match analysis.analysis_type {
                    AnalysisType::Transient => "time",
                    AnalysisType::Ac => "frequency",
                    _ => "value",
                }
                .to_owned(),
            ),
        );
    }
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
        columns: waveforms.len(),
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
    fn hdf5_preserves_each_signal_representation_in_every_waveform_domain() {
        for kind in [
            AnalysisType::Transient,
            AnalysisType::DcSweep,
            AnalysisType::Ac,
        ] {
            let analysis = AnalysisResult::new(1, kind, "Mixed representations", 0.0)
                .with_waveforms(vec![
                    RetainedWaveform::new("|out|", vec![1.0, 2.0], vec![99.0, 99.0])
                        .with_complex_components("out", vec![-0.0, 4.0], vec![2.0, -1.0])
                        .with_unit("mA"),
                    RetainedWaveform::new("Re(out)", vec![1.0, 2.0], vec![3.0, -0.0]),
                ]);
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
            assert_eq!(decoded.signals.len(), 2);
            assert_eq!(decoded.signals[0].name, "out");
            assert_eq!(decoded.signals[0].real[0].to_bits(), (-0.0_f64).to_bits());
            assert_eq!(decoded.signals[0].real[1], 4.0);
            assert_eq!(
                decoded.signals[0].imag.as_deref(),
                Some([2.0, -1.0].as_slice())
            );
            assert_eq!(decoded.signals[0].unit.as_deref(), Some("mA"));
            assert_eq!(decoded.signals[1].name, "Re(out)");
            assert_eq!(decoded.signals[1].real[1].to_bits(), (-0.0_f64).to_bits());
            assert!(decoded.signals[1].imag.is_none());
            assert_eq!(decoded.signals[1].unit, None);
        }
    }

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

    #[test]
    fn typed_complex_tables_obey_the_readers_physical_column_limit() {
        let mut analysis = AnalysisResult::new(1, AnalysisType::Transient, "Complex columns", 0.0)
            .with_waveforms(
                (0..512)
                    .map(|index| {
                        RetainedWaveform::new(
                            format!("magnitude{index}"),
                            vec![0.0, 1.0],
                            vec![5.0, 5.0],
                        )
                        .with_complex_components(
                            format!("signal{index}"),
                            vec![3.0, 3.0],
                            vec![4.0, 4.0],
                        )
                    })
                    .collect(),
            );
        let error =
            prepare_hdf5(&analysis, &analysis.waveforms.iter().collect::<Vec<_>>()).unwrap_err();
        assert!(matches!(
            error,
            Hdf5ProjectionError::ColumnLimit { columns: 1025 }
        ));
        analysis.waveforms.last_mut().unwrap().complex = None;
        let export =
            prepare_hdf5(&analysis, &analysis.waveforms.iter().collect::<Vec<_>>()).unwrap();
        let mut bytes = Vec::new();
        rspice_core::io::write_hdf5(&mut bytes, &export.document).unwrap();
        let decoded = decode_hdf5(
            &bytes,
            Hdf5Limits {
                max_columns: MAX_COLUMNS,
                max_values: 4096,
                coordinate_names: &["time"],
            },
            "hdf5",
        )
        .unwrap();
        assert_eq!(export.columns, 512);
        assert_eq!(decoded.signals.len(), 512);
        assert!(decoded.signals[0].imag.is_some());
        assert!(decoded.signals[511].imag.is_none());
    }
}

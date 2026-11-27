//! HDF5 files of already-selected retained waveforms, through the canonical writer shared with
//! the CLI, written as their samples are read.
//!
//! Every dataset is stored contiguously at an address its length fixes, so the file's metadata
//! is written first and each column after it a block at a time
//! ([`rspice_core::io::write_hdf5_streamed`]): nothing holds a column, and a run kept in a
//! results store exports whatever its length.
//!
//! One analysis is the section group RSpice reopens: `transient`, `dc_sweep` or `ac`. Every
//! member of a `.step` or Monte Carlo family is a group of its own, named by its parameter
//! values, `r=2k`, laid out as that section is and stating the member's parameters as
//! attributes.

use std::io::Write;

use rspice_core::io::{
    Hdf5Attribute, Hdf5Column, Hdf5Coordinate, Hdf5Document, Hdf5Error, Hdf5StreamError,
    Hdf5Table, Hdf5Values,
};
use rspice_results::analysis_type::AnalysisType;
use rspice_results::provenance::AnalysisResultFamilyMember;
use rspice_results::waveform::{RetainedWaveform, WaveformValues};

use crate::waveform_io::result::{
    ExportMember, WaveformExportError, checked_coordinate, window, windows,
};
use rspice_results::result_import::waveforms::WaveformImportLimits;

#[derive(Debug)]
pub enum Hdf5ProjectionError {
    UnsupportedAnalysis {
        analysis: AnalysisType,
        label: String,
    },
    Waveforms(WaveformExportError),
    ColumnLimit {
        columns: usize,
        limit: usize,
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
            Self::Waveforms(source) => source.fmt(f),
            Self::ColumnLimit { columns, limit } => write!(
                f,
                "This HDF5 table has {columns} physical columns; the import limit is {limit}. Hide traces before exporting."
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
            Self::Waveforms(source) => Some(source),
            _ => None,
        }
    }
}

/// What a written file holds, and what the reader is owed about it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hdf5Facts {
    /// The section the analysis is: `transient`, `dc_sweep` or `ac`.
    pub section: &'static str,
    pub coordinate_name: String,
    /// Rows of the first member.
    pub rows: usize,
    /// Signals of the first member.
    pub columns: usize,
    /// The members the file holds, a group each: more than one for every member of a family.
    pub members: usize,
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

/// A document whose datasets are all streamed, and the samples each streamed dataset is, in
/// document order, by the signal they are read for.
struct Planned<'a> {
    document: Hdf5Document,
    datasets: Vec<(&'a str, &'a WaveformValues)>,
    facts: Hdf5Facts,
}

fn plan<'a, W: AsRef<RetainedWaveform>>(
    members: &[ExportMember<'a, W>],
    limits: WaveformImportLimits,
) -> Result<Planned<'a>, Hdf5ProjectionError> {
    let Some(first) = members.first() else {
        return Err(Hdf5ProjectionError::Waveforms(WaveformExportError::NoSamples));
    };
    let family = members.len() > 1;
    let title = match first.analysis.family_member() {
        Some(member) if family => first
            .analysis
            .label
            .strip_suffix(&format!(" \u{00b7} {}", member.label()))
            .unwrap_or(&first.analysis.label)
            .to_owned(),
        _ => first.analysis.label.clone(),
    };
    let mut document = Hdf5Document::new(title);
    if family {
        document.set_attr("member_count", Hdf5Attribute::Integer(members.len() as i64));
    }
    let mut datasets = Vec::new();
    let mut facts = None;
    let mut groups = std::collections::BTreeSet::new();
    for member in members {
        let analysis = member.analysis;
        let waveforms = &member.waveforms;
        let Some(section) = section_for(analysis.analysis_type) else {
            return Err(Hdf5ProjectionError::UnsupportedAnalysis {
                analysis: analysis.analysis_type,
                label: analysis.label.clone(),
            });
        };
        let reference =
            checked_coordinate(waveforms, limits).map_err(Hdf5ProjectionError::Waveforms)?;
        let rows = reference.x.len();

        // The native AC layout requires complex columns. Typed tables represent
        // mixed real/complex traces on any coordinate without fabricating phase
        // or replacing retained rectangular samples with display magnitudes.
        let spectral = analysis.analysis_type == AnalysisType::Ac
            && waveforms
                .iter()
                .all(|waveform| (*waveform).as_ref().complex.is_some());
        let typed_table = !spectral
            && (analysis.analysis_type == AnalysisType::Ac
                || waveforms
                    .iter()
                    .any(|waveform| (*waveform).as_ref().complex.is_some()));
        let column_count: usize = waveforms
            .iter()
            .map(|waveform| {
                if typed_table && (*waveform).as_ref().complex.is_some() {
                    2
                } else {
                    1
                }
            })
            .sum();
        if column_count + 1 > limits.max_columns {
            return Err(Hdf5ProjectionError::ColumnLimit {
                columns: column_count + 1,
                limit: limits.max_columns,
            });
        }
        // The columns hold no values: each dataset is streamed from the samples beside it.
        let mut columns = Vec::with_capacity(column_count);
        let mut streamed = vec![(reference.name.as_str(), &reference.x)];
        for waveform in waveforms {
            let waveform = (*waveform).as_ref();
            match &waveform.complex {
                Some(complex) if spectral => {
                    columns.push(Hdf5Column::Complex {
                        name: complex.source_name.clone(),
                        unit: waveform.unit.clone(),
                        real: Vec::new(),
                        imag: Vec::new(),
                    });
                    streamed.push((complex.source_name.as_str(), &complex.real));
                    streamed.push((complex.source_name.as_str(), &complex.imag));
                }
                Some(complex) => {
                    for (label, part, values) in
                        [("Re", "real", &complex.real), ("Im", "imag", &complex.imag)]
                    {
                        columns.push(Hdf5Column::Real {
                            name: format!("{label}({})", complex.source_name),
                            quantity: format!(
                                "complex_{part}:{}",
                                quantity(&complex.source_name)
                            ),
                            unit: waveform.unit.clone(),
                            values: Vec::new(),
                        });
                        streamed.push((complex.source_name.as_str(), values));
                    }
                }
                None => {
                    columns.push(Hdf5Column::Real {
                        name: waveform.name.clone(),
                        quantity: quantity(&waveform.name),
                        unit: waveform.unit.clone(),
                        values: Vec::new(),
                    });
                    streamed.push((waveform.name.as_str(), &waveform.y));
                }
            }
        }

        let coordinate_name = crate::waveform_io::result::axis_signal_for_analysis(analysis)
            .0
            .to_owned();
        let group_name = if family {
            let mut name = member.file_label();
            if !groups.insert(name.clone()) {
                name = format!("{name} ({})", groups.len() + 1);
                groups.insert(name.clone());
            }
            name
        } else {
            section.to_owned()
        };
        document
            .add_table(&Hdf5Table {
                group: group_name,
                section_type: if typed_table { "table" } else { section }.to_owned(),
                coordinate: if spectral {
                    Hdf5Coordinate::Frequency(Vec::new())
                } else {
                    Hdf5Coordinate::Independent {
                        name: coordinate_name.clone(),
                        values: Vec::new(),
                    }
                },
                columns,
            })
            .map_err(Hdf5ProjectionError::InvalidTable)?;
        let group = document.groups.last_mut().expect("table was added");
        for (_, values) in &mut group.datasets {
            *values = Hdf5Values::Streamed(rows);
        }
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
        if family {
            group.set_attr("member", Hdf5Attribute::Text(member.label()));
            match analysis.family_member() {
                Some(AnalysisResultFamilyMember::StepPoint { index, coordinate }) => {
                    group.set_attr("step_index", Hdf5Attribute::Integer(*index as i64));
                    for (at, axis) in coordinate.iter().enumerate() {
                        let prefix = format!("parameter_{at:04}");
                        group.set_attr(
                            &format!("{prefix}_name"),
                            Hdf5Attribute::Text(axis.name.clone()),
                        );
                        group.set_attr(&format!("{prefix}_value"), Hdf5Attribute::Real(axis.value));
                        if !axis.unit.is_empty() {
                            group.set_attr(
                                &format!("{prefix}_unit"),
                                Hdf5Attribute::Text(axis.unit.clone()),
                            );
                        }
                    }
                }
                Some(AnalysisResultFamilyMember::MonteCarloTrial { index }) => {
                    group.set_attr("trial", Hdf5Attribute::Integer(*index as i64));
                }
                None => {}
            }
        }
        datasets.extend(streamed);
        facts.get_or_insert(Hdf5Facts {
            section,
            coordinate_name,
            rows,
            columns: waveforms.len(),
            members: members.len(),
        });
    }
    Ok(Planned {
        document,
        datasets,
        facts: facts.expect("a file of one member or more"),
    })
}

/// Whether `members` make an HDF5 file, checked without reading a sample.
pub fn check_hdf5<W: AsRef<RetainedWaveform>>(
    members: &[ExportMember<'_, W>],
    limits: WaveformImportLimits,
) -> Result<Hdf5Facts, Hdf5ProjectionError> {
    plan(members, limits).map(|planned| planned.facts)
}

/// `members` as an HDF5 file written to `out` as their samples are read: one analysis as its
/// section group, or every member of a family as a group each. A column that cannot be read
/// stops the write with its failure; what reached `out` is then a partial file its caller
/// discards.
pub fn write_hdf5<W: AsRef<RetainedWaveform>>(
    members: &[ExportMember<'_, W>],
    limits: WaveformImportLimits,
    out: &mut dyn Write,
) -> Result<Hdf5Facts, Hdf5ProjectionError> {
    let planned = plan(members, limits)?;
    let datasets = &planned.datasets;
    let mut out = std::io::BufWriter::with_capacity(1 << 16, out);
    let mut bytes = Vec::new();
    rspice_core::io::write_hdf5_streamed(&mut out, &planned.document, |index, out| {
        let (signal, values) = datasets[index];
        for rows in windows(values.len()) {
            let block =
                window(signal, values, rows).map_err(Hdf5ProjectionError::Waveforms)?;
            bytes.clear();
            for value in block.iter() {
                bytes.extend_from_slice(&value.to_le_bytes());
            }
            out.write_all(&bytes)
                .map_err(|error| Hdf5ProjectionError::InvalidTable(Hdf5Error::Write(error)))?;
        }
        Ok(())
    })
    .map_err(|error| match error {
        Hdf5StreamError::Document(error) => Hdf5ProjectionError::InvalidTable(error),
        Hdf5StreamError::Source(error) => error,
    })?;
    out.flush()
        .map_err(|error| Hdf5ProjectionError::InvalidTable(Hdf5Error::Write(error)))?;
    Ok(planned.facts)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hdf5::{Hdf5Limits, decode_hdf5};
    use crate::waveform_io::result::EXPORT_TEST_LIMITS;
    use rspice_results::analysis_result::AnalysisResult;
    use rspice_results::result_import::{
        ResultImportCoordinate, ResultImportFormat, ResultImportSource,
    };

    fn written(
        analysis: &AnalysisResult,
        limits: WaveformImportLimits,
    ) -> Result<(Vec<u8>, Hdf5Facts), Hdf5ProjectionError> {
        let mut bytes = Vec::new();
        let facts = write_hdf5(
            &[ExportMember::new(analysis, analysis.waveforms.iter().collect())],
            limits,
            &mut bytes,
        )?;
        Ok((bytes, facts))
    }

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
            let (bytes, _) = written(&analysis, EXPORT_TEST_LIMITS).unwrap();
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

    /// The streamed file is the one the in-memory writer makes of the same document holding its
    /// samples, byte for byte: each dataset streams the samples the document names it for.
    #[test]
    fn a_streamed_file_is_the_in_memory_writers_bytes() {
        let analysis = AnalysisResult::new(1, AnalysisType::Ac, "AC", 0.0).with_waveforms(vec![
            RetainedWaveform::new("|V(out)|", vec![1.0, 2.0, 3.0], vec![5.0, 5.0, 5.0])
                .with_complex_components("V(out)", vec![0.5, -0.25, 3.0], vec![1.0, 2.0, 4.0])
                .with_unit("V"),
            RetainedWaveform::new("V(in)", vec![1.0, 2.0, 3.0], vec![7.0, 8.0, 9.0]),
        ]);
        let (streamed, _) = written(&analysis, EXPORT_TEST_LIMITS).unwrap();
        let members = [ExportMember::new(&analysis, analysis.waveforms.iter().collect())];
        let mut planned = plan(&members, EXPORT_TEST_LIMITS).unwrap();
        let mut samples = planned
            .datasets
            .iter()
            .map(|(_, values)| values.read_whole().unwrap().into_owned());
        for group in &mut planned.document.groups {
            for (_, values) in &mut group.datasets {
                *values = Hdf5Values::Real(samples.next().unwrap());
            }
        }
        assert!(samples.next().is_none());
        let mut whole = Vec::new();
        rspice_core::io::write_hdf5(&mut whole, &planned.document).unwrap();
        assert_eq!(streamed, whole);
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
                family_member: None,
                event_dump: None,
            });
            let (bytes, facts) = written(&analysis, EXPORT_TEST_LIMITS).unwrap();
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
            assert_eq!(facts.coordinate_name, name);
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
        let error = written(&analysis, EXPORT_TEST_LIMITS).unwrap_err();
        assert!(matches!(
            error,
            Hdf5ProjectionError::ColumnLimit {
                columns: 1025,
                limit: 1024
            }
        ));
        analysis.waveforms.last_mut().unwrap().complex = None;
        let (bytes, facts) = written(&analysis, EXPORT_TEST_LIMITS).unwrap();
        let decoded = decode_hdf5(
            &bytes,
            Hdf5Limits {
                max_columns: EXPORT_TEST_LIMITS.max_columns,
                max_values: 4096,
                coordinate_names: &["time"],
            },
            "hdf5",
        )
        .unwrap();
        assert_eq!(facts.columns, 512);
        assert_eq!(decoded.signals.len(), 512);
        assert!(decoded.signals[0].imag.is_some());
        assert!(decoded.signals[511].imag.is_none());
    }

    /// Every member of a family is a group named by its parameter values, stating them, and
    /// holding that member's samples.
    #[test]
    fn every_member_is_a_group_named_by_its_parameter_values() {
        let members = [1000.0, 2000.0, 4000.0]
            .into_iter()
            .enumerate()
            .map(|(index, r)| {
                let mut analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0)
                    .with_waveforms(vec![RetainedWaveform::new(
                        "V(out)",
                        vec![0.0, 1.0 + index as f64],
                        vec![r, -r],
                    )]);
                crate::waveform_io::result::step_member(&mut analysis, index, &[("r", r)]);
                analysis
            })
            .collect::<Vec<_>>();
        let export = members
            .iter()
            .map(|analysis| ExportMember::new(analysis, analysis.waveforms.iter().collect()))
            .collect::<Vec<_>>();
        let mut bytes = Vec::new();
        let facts = write_hdf5(&export, EXPORT_TEST_LIMITS, &mut bytes).unwrap();
        assert_eq!(facts.members, 3);
        let file = rustyhdf5::File::from_bytes(bytes).unwrap();
        let mut groups = file.root().groups().unwrap();
        groups.sort();
        assert_eq!(groups, ["r=1k", "r=2k", "r=4k"]);
        for (name, r, end) in [("r=1k", 1000.0, 1.0), ("r=2k", 2000.0, 2.0), ("r=4k", 4000.0, 3.0)] {
            let group = file.group(name).unwrap();
            let attrs = group.attrs().unwrap();
            assert!(matches!(
                attrs.get("parameter_0000_name"),
                Some(rustyhdf5::AttrValue::String(name)) if name == "r"
            ));
            assert!(matches!(
                attrs.get("parameter_0000_value"),
                Some(rustyhdf5::AttrValue::F64(value)) if *value == r
            ));
            assert_eq!(
                group.dataset("independent").unwrap().read_f64().unwrap(),
                [0.0, end]
            );
            assert_eq!(
                group.dataset("signal_0000").unwrap().read_f64().unwrap(),
                [r, -r]
            );
        }
    }
}

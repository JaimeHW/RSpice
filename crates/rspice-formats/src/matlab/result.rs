//! MATLAB v5 files of already-selected retained waveforms, written as their samples are read.
//!
//! Every element states its size before its values, and every size is known from the columns'
//! lengths, so a file is written front to back a block of each column at a time: nothing holds a
//! column, and a run kept in a results store exports whatever its length up to MAT v5's own
//! limit of 4 GiB a variable.
//!
//! One analysis is a column vector per signal beside its coordinate, the layout RSpice reopens.
//! Every member of a `.step` or Monte Carlo family is a structure per member, named by its
//! parameter values, `r_2k`, whose fields are that member's coordinate and signals.

use std::io::Write;

use super::publication::{MatNameAllocator, created_on, header_text, note_entry};
use super::{MatWriteError, row_dimension};
use crate::waveform_io::result::{
    ExportMember, WaveformExportError, checked_coordinate, window, windows,
};
use rspice_results::analysis_type::AnalysisType;
use rspice_results::result_import::waveforms::WaveformImportLimits;
use rspice_results::waveform::{RetainedWaveform, WaveformValues};

#[derive(Debug)]
pub enum MatlabProjectionError {
    UnsupportedAnalysis {
        analysis: AnalysisType,
        label: String,
    },
    Waveforms(WaveformExportError),
    /// A variable MAT v5 cannot state, or a write the destination refused.
    File(MatWriteError),
}

impl std::fmt::Display for MatlabProjectionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedAnalysis { label, .. } => write!(
                f,
                "A MATLAB v5 file carries one sampled analysis: a transient on 'time', a DC sweep on \
                 'sweep' or an AC sweep on 'frequency'. '{label}' is none of those, and MAT has no header \
                 in which a variable could say what it is, so the file would reopen as an anonymous \
                 sweep. Export CSV, or an RSpice bundle, which carries this analysis whole."
            ),
            Self::Waveforms(source) => source.fmt(f),
            Self::File(source) => source.fmt(f),
        }
    }
}

impl std::error::Error for MatlabProjectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Waveforms(source) => Some(source),
            _ => None,
        }
    }
}

/// What a written file holds, and what the reader is owed about it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatlabFacts {
    /// The variable (or field) the coordinate is published under.
    pub coordinate: &'static str,
    /// Rows of the first member.
    pub rows: usize,
    /// Top-level variables: a column per signal and the coordinate, or a structure per member.
    pub variables: usize,
    /// The members the file holds: more than one for every member of a family.
    pub members: usize,
    /// Every published variable, mapped back to what it came from.
    pub note: String,
    /// The note did not fit the header's descriptive-text field.
    pub note_truncated: bool,
    /// The header's descriptive text.
    pub header_text: String,
}

/// The variable name an analysis publishes its coordinate under.
///
/// These are exactly the names `result_import_adapters::parse_matlab_v5`
/// recognises a coordinate by, so a file written here reopens as the analysis
/// it was.
const fn coordinate_variable(analysis: AnalysisType) -> Option<&'static str> {
    match analysis {
        AnalysisType::Transient => Some("time"),
        AnalysisType::DcSweep => Some("sweep"),
        AnalysisType::Ac => Some("frequency"),
        _ => None,
    }
}

/// One column the file writes: its MATLAB name, the signal it came from, its samples.
struct PlannedColumn<'a> {
    name: String,
    signal: &'a str,
    real: &'a WaveformValues,
    imag: Option<&'a WaveformValues>,
}

/// One member's columns, the coordinate first, and the structure it is kept in, if any.
struct PlannedMember<'a> {
    structure: Option<String>,
    rows: usize,
    columns: Vec<PlannedColumn<'a>>,
}

struct Plan<'a> {
    members: Vec<PlannedMember<'a>>,
    facts: MatlabFacts,
}

fn plan<'a, W: AsRef<RetainedWaveform>>(
    members: &[ExportMember<'a, W>],
    limits: WaveformImportLimits,
) -> Result<Plan<'a>, MatlabProjectionError> {
    let Some(first) = members.first() else {
        return Err(MatlabProjectionError::Waveforms(
            WaveformExportError::NoSamples,
        ));
    };
    let family = members.len() > 1;
    let mut structures = MatNameAllocator::new("", members.len());
    let mut entries = Vec::new();
    let mut planned = Vec::with_capacity(members.len());
    let mut coordinate_name = "";
    for member in members {
        let analysis = member.analysis;
        let Some(coordinate) = coordinate_variable(analysis.analysis_type) else {
            return Err(MatlabProjectionError::UnsupportedAnalysis {
                analysis: analysis.analysis_type,
                label: analysis.label.clone(),
            });
        };
        coordinate_name = coordinate;
        let reference = checked_coordinate(&member.waveforms, limits)
            .map_err(MatlabProjectionError::Waveforms)?;
        let rows = reference.x.len();
        row_dimension(coordinate, rows).map_err(MatlabProjectionError::File)?;
        // The coordinate claims its name first: it is written first, and the
        // importer takes the first variable whose name it recognises.
        let mut names = MatNameAllocator::new(coordinate, member.waveforms.len());
        let structure = family.then(|| structures.allocate(&member.label()));
        // One member of a family says which one it is first, where a note cut to fit the header
        // still keeps it; a file of every member says what each structure is.
        match &structure {
            Some(structure) => entries.push(note_entry(structure, &member.label(), None)),
            None => entries.extend(analysis.family_member().map(|member| member.label())),
        }
        let coordinate_source =
            crate::waveform_io::result::axis_signal_for_analysis_type(analysis.analysis_type).0;
        if !family {
            entries.push(note_entry(coordinate, coordinate_source, None));
        }
        let mut columns = vec![PlannedColumn {
            name: coordinate.to_owned(),
            signal: &reference.name,
            real: &reference.x,
            imag: None,
        }];
        for waveform in &member.waveforms {
            let waveform = (*waveform).as_ref();
            let (source, real, imag) = match &waveform.complex {
                Some(complex) => (
                    complex.source_name.as_str(),
                    &complex.real,
                    Some(&complex.imag),
                ),
                None => (waveform.name.as_str(), &waveform.y, None),
            };
            let name = names.allocate(source);
            if !family {
                entries.push(note_entry(&name, source, waveform.unit.as_deref()));
            }
            columns.push(PlannedColumn {
                name,
                signal: source,
                real,
                imag,
            });
        }
        match &structure {
            Some(structure) => {
                let fields = columns
                    .iter()
                    .map(|column| (column.name.as_str(), rows, column.imag.is_some()))
                    .collect::<Vec<_>>();
                super::struct_size(structure, &fields).map_err(MatlabProjectionError::File)?;
            }
            None => {
                for column in &columns {
                    super::column_size(&column.name, rows, column.imag.is_some())
                        .map_err(MatlabProjectionError::File)?;
                }
            }
        }
        planned.push(PlannedMember {
            structure,
            rows,
            columns,
        });
    }
    // Every published variable is named, not just the first few: the unit is
    // recoverable from nowhere else, so a note that covered some of them
    // would be a note that quietly lost the rest.
    let note = entries.join("; ");
    let (header_text, note_truncated) = header_text(&created_on(first.analysis.timestamp), &note);
    let variables = if family {
        planned.len()
    } else {
        planned[0].columns.len()
    };
    let facts = MatlabFacts {
        coordinate: coordinate_name,
        rows: planned[0].rows,
        variables,
        members: members.len(),
        note,
        note_truncated,
        header_text,
    };
    Ok(Plan {
        members: planned,
        facts,
    })
}

/// Whether `members` make a MATLAB v5 file, checked without reading a sample.
pub fn check_matlab<W: AsRef<RetainedWaveform>>(
    members: &[ExportMember<'_, W>],
    limits: WaveformImportLimits,
) -> Result<MatlabFacts, MatlabProjectionError> {
    plan(members, limits).map(|plan| plan.facts)
}

/// `members` as a MATLAB v5 file written to `out` as their samples are read: one member's
/// coordinate and signals as column vectors, or every member of a family as a structure of
/// them. A column that cannot be read stops the write with its failure; what reached `out` is
/// then a partial file its caller discards.
pub fn write_matlab<W: AsRef<RetainedWaveform>>(
    members: &[ExportMember<'_, W>],
    limits: WaveformImportLimits,
    out: &mut dyn Write,
) -> Result<MatlabFacts, MatlabProjectionError> {
    let plan = plan(members, limits)?;
    let mut out = std::io::BufWriter::with_capacity(1 << 16, out);
    let file = |error: std::io::Error| {
        MatlabProjectionError::File(MatWriteError::Write(error.to_string()))
    };
    super::write_header(&mut out, &plan.facts.header_text).map_err(file)?;
    for member in &plan.members {
        let rows = row_dimension("", member.rows).map_err(MatlabProjectionError::File)?;
        if let Some(structure) = &member.structure {
            let fields = member
                .columns
                .iter()
                .map(|column| (column.name.as_str(), member.rows, column.imag.is_some()))
                .collect::<Vec<_>>();
            super::write_struct_start(&mut out, structure, &fields)
                .map_err(MatlabProjectionError::File)?;
        }
        for column in &member.columns {
            let name = if member.structure.is_some() {
                ""
            } else {
                column.name.as_str()
            };
            super::write_column_start(&mut out, name, rows, column.imag.is_some()).map_err(file)?;
            write_part(&mut out, column.signal, column.real, member.rows)?;
            if let Some(imag) = column.imag {
                super::write_part_tag(&mut out, rows).map_err(file)?;
                write_part(&mut out, column.signal, imag, member.rows)?;
            }
        }
    }
    out.flush().map_err(file)?;
    Ok(plan.facts)
}

/// One part of a column, its `rows` values as little-endian doubles, a block at a time.
fn write_part(
    out: &mut dyn Write,
    signal: &str,
    values: &WaveformValues,
    rows: usize,
) -> Result<(), MatlabProjectionError> {
    let mut bytes = Vec::new();
    for rows in windows(rows) {
        let block = window(signal, values, rows).map_err(MatlabProjectionError::Waveforms)?;
        bytes.clear();
        for value in block.iter() {
            bytes.extend_from_slice(&value.to_le_bytes());
        }
        out.write_all(&bytes).map_err(|error| {
            MatlabProjectionError::File(MatWriteError::Write(error.to_string()))
        })?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::matlab::reader::{MatlabReadLimits, decode_matlab_v5};
    use crate::waveform_io::result::EXPORT_TEST_LIMITS;
    use rspice_results::analysis_result::AnalysisResult;

    fn written(analysis: &AnalysisResult) -> Vec<u8> {
        let mut bytes = Vec::new();
        write_matlab(
            &[ExportMember::new(
                analysis,
                analysis.waveforms.iter().collect(),
            )],
            EXPORT_TEST_LIMITS,
            &mut bytes,
        )
        .unwrap();
        bytes
    }

    #[test]
    fn matlab_preserves_each_signal_representation_in_every_waveform_domain() {
        for kind in [
            AnalysisType::Transient,
            AnalysisType::DcSweep,
            AnalysisType::Ac,
        ] {
            let analysis = AnalysisResult::new(1, kind, "Mixed representations", 0.0)
                .with_waveforms(vec![
                    RetainedWaveform::new("magnitude", vec![1.0, 2.0], vec![99.0, 99.0])
                        .with_complex_components("out", vec![-0.0, 4.0], vec![2.0, -1.0]),
                    RetainedWaveform::new("scalar", vec![1.0, 2.0], vec![3.0, -0.0]),
                ]);
            let decoded = decode_matlab_v5(
                &written(&analysis),
                MatlabReadLimits {
                    max_variables: 10,
                    max_expanded_bytes: 0,
                    min_rows: 1,
                    coordinate_names: &["time", "frequency", "sweep"],
                },
                "matlab-v5",
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
            assert_eq!(decoded.signals[1].name, "scalar");
            assert_eq!(decoded.signals[1].real[1].to_bits(), (-0.0_f64).to_bits());
            assert!(decoded.signals[1].imag.is_none());
        }
    }

    /// The streamed file is the one the in-memory writer makes of the same variables, byte for
    /// byte.
    #[test]
    fn a_streamed_file_is_the_in_memory_writers_bytes() {
        let analysis = AnalysisResult::new(1, AnalysisType::Ac, "AC", 0.0).with_waveforms(vec![
            RetainedWaveform::new("|out|", vec![1.0, 10.0], vec![1.0, 0.5])
                .with_complex_components("V(out)", vec![1.0, 0.3], vec![0.0, -0.4])
                .with_unit("V"),
        ]);
        let facts = check_matlab(
            &[ExportMember::new(
                &analysis,
                analysis.waveforms.iter().collect(),
            )],
            EXPORT_TEST_LIMITS,
        )
        .unwrap();
        let expected = crate::matlab::write_mat_v5(
            &facts.header_text,
            &[
                crate::matlab::MatVariable {
                    name: "frequency".to_owned(),
                    real: vec![1.0, 10.0],
                    imag: None,
                },
                crate::matlab::MatVariable {
                    name: "V_out_".to_owned(),
                    real: vec![1.0, 0.3],
                    imag: Some(vec![0.0, -0.4]),
                },
            ],
        )
        .unwrap();
        assert_eq!(written(&analysis), expected);
        assert!(
            facts.note.contains("V_out_ = V(out) in V"),
            "{}",
            facts.note
        );
    }

    /// Every member of a family is a structure named by its parameter values, holding that
    /// member's coordinate and signals.
    #[test]
    fn every_member_is_a_structure_named_by_its_parameter_values() {
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
        let facts = write_matlab(&export, EXPORT_TEST_LIMITS, &mut bytes).unwrap();
        assert_eq!((facts.members, facts.variables), (3, 3));
        assert!(
            facts.note.starts_with("r_1k = r=1k; r_2k = r=2k"),
            "{}",
            facts.note
        );
        let structures = mat_structures(&bytes);
        assert_eq!(
            structures
                .iter()
                .map(|(name, fields)| (
                    name.as_str(),
                    fields
                        .iter()
                        .map(|(field, _)| field.as_str())
                        .collect::<Vec<_>>()
                ))
                .collect::<Vec<_>>(),
            [
                ("r_1k", vec!["time", "V_out_"]),
                ("r_2k", vec!["time", "V_out_"]),
                ("r_4k", vec!["time", "V_out_"]),
            ]
        );
        assert_eq!(structures[1].1[0].1, [0.0, 2.0]);
        assert_eq!(structures[2].1[1].1, [4000.0, -4000.0]);
    }

    /// A structure's name and its real column fields, each by name.
    type Structure = (String, Vec<(String, Vec<f64>)>);

    /// Each top-level structure of a MAT v5 file and its real column fields, read by the
    /// element layout *MAT-File Format* states.
    fn mat_structures(bytes: &[u8]) -> Vec<Structure> {
        let u32_at = |at: usize| u32::from_le_bytes(bytes[at..at + 4].try_into().unwrap()) as usize;
        let pad = |length: usize| (8 - length % 8) % 8;
        let mut structures = Vec::new();
        let mut at = 128;
        while at < bytes.len() {
            assert_eq!(u32_at(at), 14, "an miMATRIX");
            let end = at + 8 + u32_at(at + 4);
            assert_eq!(u32_at(at + 16) & 0xff, 2, "a structure");
            let name_length = u32_at(at + 44);
            let name = String::from_utf8(bytes[at + 48..at + 48 + name_length].to_vec()).unwrap();
            let mut cursor = at + 48 + name_length + pad(name_length);
            let field_width = u32_at(cursor + 8);
            let names_length = u32_at(cursor + 20);
            let names = bytes[cursor + 24..cursor + 24 + names_length]
                .chunks(field_width)
                .map(|cell| {
                    String::from_utf8(cell.iter().copied().take_while(|b| *b != 0).collect())
                        .unwrap()
                })
                .collect::<Vec<_>>();
            cursor += 24 + names_length + pad(names_length);
            let mut fields = Vec::new();
            for field in names {
                assert_eq!(u32_at(cursor), 14, "a field is an miMATRIX");
                let next = cursor + 8 + u32_at(cursor + 4);
                let rows = u32_at(cursor + 32);
                // Flags, dimensions, an empty name, then the real part's tag.
                let values = cursor + 8 + 16 + 16 + 8 + 8;
                let real = (0..rows)
                    .map(|row| {
                        f64::from_le_bytes(
                            bytes[values + row * 8..values + row * 8 + 8]
                                .try_into()
                                .unwrap(),
                        )
                    })
                    .collect();
                fields.push((field, real));
                cursor = next;
            }
            assert_eq!(cursor, end);
            structures.push((name, fields));
            at = end;
        }
        structures
    }
}

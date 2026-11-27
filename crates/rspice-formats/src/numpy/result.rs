//! NumPy files of already-selected retained waveforms, written as their samples are read.
//!
//! An `.npy` matrix is written a row at a time, each column read a block at a time; an `.npz`
//! archive a member at a time, each member's column a block at a time, into a ZIP whose members'
//! sizes and checksums follow their bytes. Neither holds a column, so a run kept in a results
//! store exports whatever its length.

use std::io::Write;

use npyz::WriterBuilder as _;
use num_complex::Complex64;

use super::{MAX_COLUMNS, NumpyWriteError};
use crate::waveform_io::result::{
    ExportMember, RowReader, check_export_shape, checked_coordinate, window, windows,
};
use crate::zip::StreamedZip;
use rspice_results::analysis_type::AnalysisType;
use rspice_results::result_import::waveforms::WaveformImportLimits;
use rspice_results::waveform::{RetainedWaveform, WaveformValues};

/// What a written NumPy file holds, as its export reports it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumpyFacts {
    /// The coordinate's name: column 0 of an `.npy`, an array of an `.npz`.
    pub coordinate_name: &'static str,
    /// Rows of the first member written.
    pub rows: usize,
    /// The signals of the first member written, in the file's order.
    pub signals: Vec<String>,
    /// Some value is complex: an `.npy` is then complex128 throughout.
    pub complex: bool,
    /// The arrays the file holds: one for an `.npy`, every member of an `.npz`.
    pub arrays: usize,
    /// The analyses the file holds: more than one for every member of a family.
    pub members: usize,
}

impl NumpyFacts {
    /// Columns of an `.npy`: the coordinate and every signal.
    pub fn columns(&self) -> usize {
        self.signals.len() + 1
    }
}

/// The coordinate name each analysis domain publishes under.
///
/// These three are exactly the spellings RSpice's own NPZ reader recognises as
/// a coordinate, and the ones it maps back onto an analysis domain. Publishing
/// any other name would produce an archive this product could not reopen.
fn coordinate_name(analysis_type: AnalysisType) -> &'static str {
    match analysis_type {
        AnalysisType::Transient => "time",
        AnalysisType::Ac => "frequency",
        _ => "sweep",
    }
}

/// One signal column: its name, its real (or only) samples and its imaginary ones.
type Column<'a> = (&'a str, &'a WaveformValues, Option<&'a WaveformValues>);

fn columns<'a, W: AsRef<RetainedWaveform>>(waveforms: &[&'a W]) -> Vec<Column<'a>> {
    waveforms
        .iter()
        .map(|waveform| {
            let waveform = (*waveform).as_ref();
            match &waveform.complex {
                Some(complex) => (
                    complex.source_name.as_str(),
                    &complex.real,
                    Some(&complex.imag),
                ),
                None => (waveform.name.as_str(), &waveform.y, None),
            }
        })
        .collect()
}

/// The header npyz writes for an array of `shape`, of complex128 or float64.
fn npy_header(shape: &[u64], complex: bool) -> Vec<u8> {
    let mut bytes = Vec::new();
    // A header written to memory cannot fail. The writer is dropped holding no values, which
    // it notes and ignores.
    if complex {
        drop(
            npyz::WriteOptions::<Complex64>::new()
                .default_dtype()
                .shape(shape)
                .writer(&mut bytes)
                .begin_nd(),
        );
    } else {
        drop(
            npyz::WriteOptions::<f64>::new()
                .default_dtype()
                .shape(shape)
                .writer(&mut bytes)
                .begin_nd(),
        );
    }
    bytes
}

/// Whether `waveforms` of `analysis` make one `.npy` matrix, checked without reading a sample.
pub fn check_npy<W: AsRef<RetainedWaveform>>(
    analysis: &rspice_results::analysis_result::AnalysisResult<W>,
    waveforms: &[&W],
    limits: WaveformImportLimits,
) -> Result<NumpyFacts, NumpyWriteError> {
    let reference =
        checked_coordinate(waveforms, limits).map_err(NumpyWriteError::PublicationBounds)?;
    let rows = reference.x.len();
    let signals = columns(waveforms);
    let complex = signals.iter().any(|(_, _, imag)| imag.is_some());
    let columns = signals.len() + 1;
    if columns > MAX_COLUMNS {
        return Err(NumpyWriteError::MatrixColumnLimit { columns });
    }
    // One complex column promotes the whole matrix, including the signals that were real, and
    // reopening it keeps three arrays per promoted signal.
    if complex {
        let retained = signals
            .len()
            .checked_mul(3)
            .and_then(|count| count.checked_add(1));
        check_export_shape(rows, columns, retained, limits)
            .map_err(NumpyWriteError::PublicationBounds)?;
    }
    rows.checked_mul(columns)
        .ok_or(NumpyWriteError::MatrixSizeOverflow { rows, columns })?;
    Ok(NumpyFacts {
        coordinate_name: coordinate_name(analysis.analysis_type),
        rows,
        signals: signals
            .iter()
            .map(|(name, ..)| (*name).to_owned())
            .collect(),
        complex,
        arrays: 1,
        members: 1,
    })
}

/// `waveforms` of `analysis` as one 2-D `.npy` array, C order, the coordinate in column 0 and
/// the signals after it in their order, written to `out` a row at a time as the samples are
/// read. A column that cannot be read stops the write with its failure; what reached `out` is
/// then a partial file its caller discards.
pub fn write_npy<W: AsRef<RetainedWaveform>>(
    analysis: &rspice_results::analysis_result::AnalysisResult<W>,
    waveforms: &[&W],
    limits: WaveformImportLimits,
    out: &mut dyn Write,
) -> Result<NumpyFacts, NumpyWriteError> {
    let facts = check_npy(analysis, waveforms, limits)?;
    let reference =
        checked_coordinate(waveforms, limits).map_err(NumpyWriteError::PublicationBounds)?;
    let mut out = std::io::BufWriter::with_capacity(1 << 16, out);
    out.write_all(&npy_header(
        &[facts.rows as u64, facts.columns() as u64],
        facts.complex,
    ))
    .map_err(NumpyWriteError::Io)?;
    let mut scale = RowReader::new(&reference.name, &reference.x);
    let mut readers = columns(waveforms)
        .into_iter()
        .map(|(name, real, imag)| {
            (
                RowReader::new(name, real),
                imag.map(|imag| RowReader::new(name, imag)),
            )
        })
        .collect::<Vec<_>>();
    let width = if facts.complex { 16 } else { 8 };
    let mut row_bytes = Vec::with_capacity(facts.columns() * width);
    for row in 0..facts.rows {
        row_bytes.clear();
        row_bytes.extend_from_slice(&scale.at(row).to_le_bytes());
        if facts.complex {
            row_bytes.extend_from_slice(&0.0_f64.to_le_bytes());
        }
        for (real, imag) in &mut readers {
            row_bytes.extend_from_slice(&real.at(row).to_le_bytes());
            if facts.complex {
                let imag = imag.as_mut().map_or(0.0, |imag| imag.at(row));
                row_bytes.extend_from_slice(&imag.to_le_bytes());
            }
        }
        if let Some(failure) = std::iter::once(&mut scale)
            .chain(
                readers
                    .iter_mut()
                    .flat_map(|(real, imag)| std::iter::once(real).chain(imag.as_mut())),
            )
            .find_map(RowReader::failure)
        {
            return Err(NumpyWriteError::PublicationBounds(failure));
        }
        out.write_all(&row_bytes).map_err(NumpyWriteError::Io)?;
    }
    out.flush().map_err(NumpyWriteError::Io)?;
    Ok(facts)
}

/// The archive key of `name` in `member`'s part of an archive of `members` members: the name
/// itself in an archive of one, else under the member's parameter values, `r=2k/V(out)`.
fn archive_key<W: AsRef<RetainedWaveform>>(
    member: &ExportMember<'_, W>,
    members: usize,
    name: &str,
) -> String {
    if members > 1 {
        format!("{}/{name}", member.file_label())
    } else {
        name.to_owned()
    }
}

/// Whether `members` make an `.npz` archive, checked without reading a sample: one named array
/// per signal and one for each member's coordinate, the names distinct.
pub fn check_npz<W: AsRef<RetainedWaveform>>(
    members: &[ExportMember<'_, W>],
    limits: WaveformImportLimits,
) -> Result<NumpyFacts, NumpyWriteError> {
    if members.is_empty() {
        return Err(NumpyWriteError::EmptyArchive);
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut arrays = 0_usize;
    let mut facts = None;
    for member in members {
        let reference = checked_coordinate(&member.waveforms, limits)
            .map_err(NumpyWriteError::PublicationBounds)?;
        let signals = columns(&member.waveforms);
        if signals.len() + 1 > MAX_COLUMNS {
            return Err(NumpyWriteError::ArchiveMemberLimit {
                members: signals.len() + 1,
            });
        }
        let coordinate = coordinate_name(member.analysis.analysis_type);
        for name in std::iter::once(coordinate).chain(signals.iter().map(|(name, ..)| *name)) {
            let key = archive_key(member, members.len(), name);
            super::archive::archive_member_name(&key)?;
            if !seen.insert(key.to_ascii_lowercase()) {
                return Err(NumpyWriteError::DuplicateMemberName(key));
            }
            arrays += 1;
        }
        facts.get_or_insert_with(|| NumpyFacts {
            coordinate_name: coordinate,
            rows: reference.x.len(),
            signals: signals
                .iter()
                .map(|(name, ..)| (*name).to_owned())
                .collect(),
            complex: false,
            arrays: 0,
            members: members.len(),
        });
        if signals.iter().any(|(_, _, imag)| imag.is_some())
            && let Some(facts) = facts.as_mut()
        {
            facts.complex = true;
        }
    }
    let mut facts = facts.expect("an archive of one member or more");
    facts.arrays = arrays;
    Ok(facts)
}

/// `members` as an `.npz` archive written to `out` as their samples are read: each member's
/// coordinate and then each of its signals as a 1-D array, complex128 where the signal is
/// complex, a block of samples at a time. An archive of one member names its arrays by their
/// signals, as RSpice reopens it; an archive of every member of a family keeps each member's
/// arrays under its parameter values, `r=2k/time`, `r=2k/V(out)`.
pub fn write_npz<W: AsRef<RetainedWaveform>>(
    members: &[ExportMember<'_, W>],
    limits: WaveformImportLimits,
    out: &mut dyn Write,
) -> Result<NumpyFacts, NumpyWriteError> {
    let facts = check_npz(members, limits)?;
    let mut buffered = std::io::BufWriter::with_capacity(1 << 16, out);
    let mut archive = StreamedZip::new(&mut buffered);
    for member in members {
        let reference = checked_coordinate(&member.waveforms, limits)
            .map_err(NumpyWriteError::PublicationBounds)?;
        let rows = reference.x.len();
        let coordinate = coordinate_name(member.analysis.analysis_type);
        let key = archive_key(member, members.len(), coordinate);
        write_array(
            &mut archive,
            &key,
            (&reference.name, &reference.x, None),
            rows,
        )?;
        for column in columns(&member.waveforms) {
            let key = archive_key(member, members.len(), column.0);
            write_array(&mut archive, &key, column, rows)?;
        }
    }
    archive.finish().map_err(NumpyWriteError::Io)?;
    Ok(facts)
}

/// One `.npy` member of an archive, `key.npy`, written a block at a time.
fn write_array(
    archive: &mut StreamedZip<'_>,
    key: &str,
    (signal, real, imag): Column<'_>,
    rows: usize,
) -> Result<(), NumpyWriteError> {
    let header = npy_header(&[rows as u64], imag.is_some());
    let width = if imag.is_some() { 16 } else { 8 };
    archive
        .start(
            &format!("{key}.npy"),
            header.len() as u64 + rows as u64 * width,
        )
        .map_err(NumpyWriteError::Io)?;
    archive.write_all(&header).map_err(NumpyWriteError::Io)?;
    let mut bytes = Vec::new();
    for rows in windows(rows) {
        bytes.clear();
        let real_block =
            window(signal, real, rows.clone()).map_err(NumpyWriteError::PublicationBounds)?;
        match imag {
            Some(imag) => {
                let imag_block =
                    window(signal, imag, rows).map_err(NumpyWriteError::PublicationBounds)?;
                for (real, imag) in real_block.iter().zip(imag_block.iter()) {
                    bytes.extend_from_slice(&real.to_le_bytes());
                    bytes.extend_from_slice(&imag.to_le_bytes());
                }
            }
            None => {
                for value in real_block.iter() {
                    bytes.extend_from_slice(&value.to_le_bytes());
                }
            }
        }
        archive.write_all(&bytes).map_err(NumpyWriteError::Io)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::waveform_io::result::EXPORT_TEST_LIMITS;
    use rspice_results::analysis_result::AnalysisResult;

    #[test]
    fn complex_matrix_promotion_counts_every_imported_magnitude_and_component() {
        let analysis = AnalysisResult::new(1, AnalysisType::Transient, "Mixed", 0.0)
            .with_waveforms(vec![
                RetainedWaveform::new("magnitude", vec![0.0, 1.0], vec![5.0, 5.0])
                    .with_complex_components("out", vec![3.0, 4.0], vec![4.0, 3.0]),
                RetainedWaveform::new("real", vec![0.0, 1.0], vec![-0.0, 2.0]),
            ]);
        let waveforms = analysis.waveforms.iter().collect::<Vec<_>>();
        for max_values in [10, 13, 14] {
            let limits = WaveformImportLimits {
                max_values,
                ..EXPORT_TEST_LIMITS
            };
            // NPZ retains each signal's own dtype and needs ten values.
            let mut archive = Vec::new();
            write_npz(
                &[ExportMember::new(&analysis, waveforms.clone())],
                limits,
                &mut archive,
            )
            .unwrap();
            let decoded = crate::numpy::archive::decode_npz(
                &archive,
                crate::numpy::archive::NpzReadLimits {
                    max_members: 3,
                    max_expanded_bytes: 4096,
                    max_numeric_values: max_values,
                },
                &["time"],
                "numpy_npz",
            )
            .unwrap();
            assert_eq!(decoded.signals.len(), 2);
            assert_eq!(
                decoded.signals.iter().filter(|s| s.imag.is_some()).count(),
                1
            );
            // NPY's single complex dtype promotes the real signal as well.
            let mut matrix = Vec::new();
            let written = write_npy(&analysis, &waveforms, limits, &mut matrix);
            if max_values < 14 {
                assert!(
                    written
                        .unwrap_err()
                        .to_string()
                        .contains("retains 14 numeric values")
                );
            } else {
                written.unwrap();
                let array =
                    crate::numpy::reader::decode_npy(&matrix, max_values, "numpy_npy").unwrap();
                let decoded =
                    crate::numpy::reader::npy_matrix_to_dataset(array, 2, 3, "numpy_npy").unwrap();
                assert_eq!(decoded.coordinate, [0.0, 1.0]);
                assert_eq!(decoded.signals.len(), 2);
                assert!(decoded.signals.iter().all(|s| s.imag.is_some()));
                assert_eq!(decoded.signals[1].real[0].to_bits(), (-0.0_f64).to_bits());
            }
        }
    }

    /// An archive of every member of a family keeps each member's arrays under its parameter
    /// values, and each holds that member's samples.
    #[test]
    fn an_archive_of_every_member_keeps_each_under_its_parameter_values() {
        let members = [1000.0, 2000.0, 4000.0]
            .into_iter()
            .enumerate()
            .map(|(index, r)| {
                let mut analysis = AnalysisResult::new(1, AnalysisType::Transient, "TRAN", 0.0)
                    .with_waveforms(vec![RetainedWaveform::new(
                        "V(out)",
                        vec![0.0, 1.0, 2.0 + index as f64],
                        vec![r, r + 1.0, r + 2.0],
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
        let facts = write_npz(&export, EXPORT_TEST_LIMITS, &mut bytes).unwrap();
        assert_eq!((facts.members, facts.arrays), (3, 6));
        let files = crate::zip::read_zip_files(&bytes, 10, 1 << 20).unwrap();
        let names = files
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            [
                "r=1k/time.npy",
                "r=1k/V(out).npy",
                "r=2k/time.npy",
                "r=2k/V(out).npy",
                "r=4k/time.npy",
                "r=4k/V(out).npy",
            ]
        );
        let values = |bytes: &[u8]| {
            npyz::NpyFile::new(bytes)
                .unwrap()
                .into_vec::<f64>()
                .unwrap()
        };
        assert_eq!(values(&files[5].1), [4000.0, 4001.0, 4002.0]);
        assert_eq!(values(&files[4].1), [0.0, 1.0, 4.0]);
    }
}

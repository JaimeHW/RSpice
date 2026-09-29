//! Named NumPy arrays in NPZ archives.

use std::collections::{BTreeSet, HashSet};
use std::io::{Cursor, Read};
use std::path::Path;

use super::reader::{NumpyReadError, NumpyReadFailure, read_error};
use super::{MAX_COLUMNS, NamedArray, encode_complex_array, encode_real_array};
use crate::numeric::{DecodedNumericDataset, DecodedNumericSignal, stated_coordinate_names};
use crate::zip::deterministic_stored_zip;
use num_complex::Complex64;

/// A member name RSpice's own archive reader will accept.
///
/// The reader refuses a member whose name is absolute, contains `..`, or
/// contains a backslash, and it identifies an array by the file stem, so two
/// signals whose names differ only in case collide. Every one of those is
/// refused here, by name, rather than published as an archive this product
/// would not reopen.
fn archive_member_name(name: &str) -> Result<String, String> {
    if name.trim().is_empty() {
        return Err(
            "A signal with no name cannot become an archive member; export CSV instead.".to_owned(),
        );
    }
    if name.starts_with('/') || name.contains("..") || name.contains('\\') {
        return Err(format!(
            "'{name}' cannot be an archive member name: RSpice refuses an archive member that is \
             absolute, contains '..', or contains a backslash. Export CSV or an RSpice bundle."
        ));
    }
    Ok(format!("{name}.npy"))
}

/// One `.npy` member per signal, plus the coordinate, in a stored ZIP.
pub fn encode_npz(
    coordinate_name: &str,
    coordinate: &[f64],
    signals: &[NamedArray<'_>],
) -> Result<Vec<u8>, String> {
    let members = signals.len().checked_add(1).ok_or_else(|| {
        format!("This result needs too many archive members; RSpice reads at most {MAX_COLUMNS}.")
    })?;
    if members > MAX_COLUMNS {
        return Err(format!(
            "This result needs {members} archive members; RSpice reads at most {MAX_COLUMNS}."
        ));
    }
    if coordinate.is_empty() || signals.is_empty() {
        return Err("A NumPy archive needs coordinate samples and at least one signal.".into());
    }
    for signal in signals {
        if signal.real.len() != coordinate.len()
            || signal
                .imag
                .is_some_and(|imag| imag.len() != coordinate.len())
        {
            return Err(format!(
                "'{}' has {} samples against {} coordinate samples; the export is refused rather than padded or truncated.",
                signal.name,
                signal.real.len(),
                coordinate.len()
            ));
        }
    }
    let rows = [coordinate.len() as u64];
    let mut names = Vec::with_capacity(members);
    let mut seen = BTreeSet::new();
    names.push(archive_member_name(coordinate_name)?);
    seen.insert(coordinate_name.to_ascii_lowercase());
    for signal in signals {
        let member = archive_member_name(signal.name)?;
        if !seen.insert(signal.name.to_ascii_lowercase()) {
            return Err(format!(
                "Two signals both claim the archive member '{}'. An archive names its arrays, so \
                 the names have to differ; RSpice compares them without regard to case.",
                signal.name
            ));
        }
        names.push(member);
    }

    let mut payloads = Vec::with_capacity(members);
    payloads.push(encode_real_array(&rows, coordinate)?);
    for signal in signals {
        let bytes = match signal.imag {
            Some(imag) => {
                let values = signal
                    .real
                    .iter()
                    .zip(imag)
                    .map(|(re, im)| Complex64::new(*re, *im))
                    .collect::<Vec<_>>();
                encode_complex_array(&rows, &values)?
            }
            None => encode_real_array(&rows, signal.real)?,
        };
        payloads.push(bytes);
    }

    let entries = names
        .iter()
        .map(String::as_str)
        .zip(payloads.iter().map(Vec::as_slice))
        .collect::<Vec<_>>();
    deterministic_stored_zip(&entries)
}

/// Limits imposed by the caller's import transaction before decoding an NPZ.
#[derive(Debug, Clone, Copy)]
pub struct NpzReadLimits {
    pub max_members: usize,
    pub max_expanded_bytes: u64,
    pub max_numeric_values: usize,
}

/// Decode named NPY members after validating the archive and its bounds.
fn decode_npz_arrays(
    bytes: &[u8],
    limits: NpzReadLimits,
    format: &str,
) -> Result<Vec<(String, super::reader::NpyArray)>, NumpyReadError> {
    let max_members = limits.max_members;
    let max_expanded_bytes = limits.max_expanded_bytes;
    let max_numeric_values = limits.max_numeric_values;
    let mut archive = zip::ZipArchive::new(Cursor::new(bytes))
        .map_err(|error| read_error(format, NumpyReadFailure::Archive(error)))?;
    if archive.len() > max_members {
        return Err(read_error(
            format,
            NumpyReadFailure::MemberCount {
                members: archive.len(),
                limit: max_members,
            },
        ));
    }
    let mut arrays = Vec::new();
    let mut names = HashSet::new();
    let mut expanded = 0_u64;
    let mut decoded_expanded = 0_u64;
    for index in 0..archive.len() {
        let member = archive.by_index(index).map_err(|error| {
            read_error(
                format,
                NumpyReadFailure::ArchiveMember {
                    index,
                    source: error,
                },
            )
        })?;
        if member.is_dir() {
            continue;
        }
        let member_name = member.name().to_owned();
        if member_name.starts_with('/') || member_name.contains("..") || member_name.contains('\\')
        {
            return Err(read_error(
                format,
                NumpyReadFailure::UnsafeMember(member_name),
            ));
        }
        if !member_name.to_ascii_lowercase().ends_with(".npy") {
            return Err(read_error(
                format,
                NumpyReadFailure::UnsupportedMember(member_name),
            ));
        }
        expanded = expanded
            .checked_add(member.size())
            .ok_or_else(|| read_error(format, NumpyReadFailure::ExpandedSizeOverflow))?;
        if expanded > max_expanded_bytes {
            return Err(read_error(
                format,
                NumpyReadFailure::ExpandedByteLimit {
                    expanded,
                    limit: max_expanded_bytes,
                    decoded: false,
                },
            ));
        }
        let stem = Path::new(&member_name)
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or_else(|| {
                read_error(
                    format,
                    NumpyReadFailure::InvalidMemberName(member_name.clone()),
                )
            })?
            .to_owned();
        if !names.insert(stem.to_ascii_lowercase()) {
            return Err(read_error(format, NumpyReadFailure::DuplicateArray(stem)));
        }
        let mut member_bytes = Vec::with_capacity(usize::try_from(member.size()).unwrap_or(0));
        member
            .take(max_expanded_bytes.saturating_add(1))
            .read_to_end(&mut member_bytes)
            .map_err(|error| {
                read_error(
                    format,
                    NumpyReadFailure::MemberRead {
                        name: member_name,
                        source: error,
                    },
                )
            })?;
        decoded_expanded = decoded_expanded
            .checked_add(member_bytes.len() as u64)
            .ok_or_else(|| read_error(format, NumpyReadFailure::ExpandedSizeOverflow))?;
        if decoded_expanded > max_expanded_bytes {
            return Err(read_error(
                format,
                NumpyReadFailure::ExpandedByteLimit {
                    expanded: decoded_expanded,
                    limit: max_expanded_bytes,
                    decoded: true,
                },
            ));
        }
        arrays.push((
            stem,
            super::reader::decode_npy(&member_bytes, max_numeric_values, format)?,
        ));
    }
    Ok(arrays)
}

/// Decode an NPZ and its named coordinate/signal interpretation.
pub fn decode_npz(
    bytes: &[u8],
    limits: NpzReadLimits,
    coordinate_names: &[&str],
    format: &str,
) -> Result<DecodedNumericDataset, NumpyReadError> {
    let mut arrays = decode_npz_arrays(bytes, limits, format)?;
    let coordinate_index = arrays
        .iter()
        .position(|(name, _)| {
            coordinate_names
                .iter()
                .any(|candidate| name.eq_ignore_ascii_case(candidate))
        })
        .ok_or_else(|| {
            read_error(
                format,
                NumpyReadFailure::MissingCoordinate {
                    expected: stated_coordinate_names(coordinate_names),
                },
            )
        })?;
    let (coordinate_name, coordinate_array) = arrays.remove(coordinate_index);
    if coordinate_array.is_complex() {
        return Err(read_error(
            format,
            NumpyReadFailure::ComplexCoordinate {
                name: coordinate_name,
            },
        ));
    }
    let coordinate = super::reader::npy_vector(&coordinate_array, format, &coordinate_name)?.0;
    let mut signals = Vec::with_capacity(arrays.len());
    for (name, array) in arrays {
        let (real, imag) = super::reader::npy_vector(&array, format, &name)?;
        signals.push(DecodedNumericSignal {
            name,
            real,
            imag,
            unit: None,
        });
    }
    Ok(DecodedNumericDataset {
        domain: crate::WaveformDomain::from_coordinate_name(&coordinate_name),
        coordinate_name,
        coordinate,
        signals,
    })
}

#[cfg(test)]
mod tests {
    use super::{
        NamedArray, NpzReadLimits, NumpyReadFailure, decode_npz, decode_npz_arrays, encode_npz,
    };
    use crate::zip::deterministic_stored_zip;

    #[test]
    fn rejects_unbalanced_columns_before_writing_archive_members() {
        let signal = NamedArray {
            name: "V(out)",
            real: &[1.0],
            imag: None,
        };
        assert!(encode_npz("time", &[0.0, 1.0], &[signal]).is_err());
    }

    #[test]
    fn npz_reader_rejects_unsafe_duplicate_and_oversized_members() {
        let npy = crate::numpy::encode_real_array(&[1], &[1.0]).expect("NPY fixture");
        let limits = NpzReadLimits {
            max_members: 2,
            max_expanded_bytes: npy.len() as u64 * 2,
            max_numeric_values: 2,
        };
        let unsafe_archive =
            deterministic_stored_zip(&[("../time.npy", npy.as_slice())]).expect("ZIP fixture");
        let error =
            decode_npz_arrays(&unsafe_archive, limits, "numpy_npz").expect_err("unsafe member");
        assert!(
            matches!(&error.reason, NumpyReadFailure::UnsafeMember(name) if name == "../time.npy")
        );
        assert_eq!(
            error.to_string(),
            "numpy_npz import: unsafe archive member '../time.npy'"
        );

        let duplicate_archive =
            deterministic_stored_zip(&[("time.npy", npy.as_slice()), ("TIME.npy", npy.as_slice())])
                .expect("ZIP fixture");
        let error = decode_npz_arrays(&duplicate_archive, limits, "numpy_npz")
            .expect_err("duplicate identity");
        assert!(matches!(&error.reason, NumpyReadFailure::DuplicateArray(name) if name == "TIME"));
        assert_eq!(
            error.to_string(),
            "numpy_npz import: archive repeats array identity 'TIME'"
        );

        let small_limit = NpzReadLimits {
            max_expanded_bytes: npy.len() as u64 - 1,
            ..limits
        };
        let error = decode_npz_arrays(&duplicate_archive, small_limit, "numpy_npz")
            .expect_err("expanded limit");
        assert!(
            matches!(&error.reason, NumpyReadFailure::ExpandedByteLimit { expanded, limit, decoded: false } if *expanded == npy.len() as u64 && *limit == small_limit.max_expanded_bytes)
        );
        assert_eq!(
            error.to_string(),
            "numpy_npz import: NPZ expanded-byte limit exceeded"
        );
    }

    #[test]
    fn npz_coordinate_selection_preserves_member_order_and_complex_samples() {
        let coordinate = crate::numpy::encode_real_array(&[2], &[0.0, 1.0]).unwrap();
        let signal = crate::numpy::encode_complex_array(
            &[2],
            &[
                num_complex::Complex64::new(1.0, -0.0),
                num_complex::Complex64::new(2.0, 3.0),
            ],
        )
        .unwrap();
        let bytes = deterministic_stored_zip(&[
            ("T.npy", &coordinate),
            ("time.npy", &coordinate),
            ("out.npy", &signal),
        ])
        .unwrap();
        let limits = NpzReadLimits {
            max_members: 3,
            max_expanded_bytes: 4096,
            max_numeric_values: 6,
        };
        let decoded = decode_npz(&bytes, limits, &["time", "t"], "numpy_npz").unwrap();
        assert_eq!(decoded.coordinate_name, "T");
        assert_eq!(decoded.domain, crate::WaveformDomain::Transient);
        assert_eq!(decoded.coordinate, [0.0, 1.0]);
        assert_eq!(
            decoded
                .signals
                .iter()
                .map(|s| s.name.as_str())
                .collect::<Vec<_>>(),
            ["time", "out"]
        );
        assert_eq!(decoded.signals[1].real, [1.0, 2.0]);
        let imag = decoded.signals[1].imag.as_ref().unwrap();
        assert_eq!(imag[0].to_bits(), (-0.0_f64).to_bits());
        assert_eq!(imag[1], 3.0);
        assert!(decoded.signals.iter().all(|s| s.unit.is_none()));

        let complex_coordinate =
            deterministic_stored_zip(&[("T.npy", &signal), ("out.npy", &coordinate)]).unwrap();
        let error =
            decode_npz(&complex_coordinate, limits, &["time", "t"], "numpy_npz").unwrap_err();
        assert!(
            matches!(&error.reason, NumpyReadFailure::ComplexCoordinate { name } if name == "T")
        );
        assert_eq!(
            error.to_string(),
            "numpy_npz import: NPZ coordinate array cannot be complex"
        );
    }

    #[test]
    fn malformed_npz_retains_the_zip_parser_cause() {
        use std::error::Error as _;
        let limits = NpzReadLimits {
            max_members: 2,
            max_expanded_bytes: 1024,
            max_numeric_values: 2,
        };
        let error = decode_npz(b"invalid", limits, &["time", "t"], "numpy_npz").unwrap_err();
        assert!(matches!(&error.reason, NumpyReadFailure::Archive(_)));
        assert!(error.reason.source().unwrap().is::<zip::result::ZipError>());
    }
}

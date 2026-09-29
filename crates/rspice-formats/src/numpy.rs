//! NumPy NPY byte encoding for real and complex arrays.

pub mod archive;
pub mod matrix;
pub mod reader;

use npyz::WriterBuilder as _;
use num_complex::Complex64;

/// Maximum coordinate-plus-signal columns or members accepted by RSpice's NumPy readers.
pub const MAX_COLUMNS: usize = 1_024;

/// A named real or rectangular complex column borrowed from a result table.
#[derive(Debug, Clone, Copy)]
pub struct NamedArray<'a> {
    pub name: &'a str,
    pub real: &'a [f64],
    pub imag: Option<&'a [f64]>,
}

#[derive(Debug)]
pub enum NumpyWriteError {
    Io(std::io::Error),
    Zip(crate::zip::StoredZipError),
    MatrixColumnLimit {
        columns: Option<usize>,
    },
    ArchiveMemberLimit {
        members: Option<usize>,
    },
    EmptyMatrix,
    EmptyArchive,
    SampleCount {
        name: String,
        real: usize,
        imag: Option<usize>,
        coordinate: usize,
    },
    MatrixSizeOverflow {
        rows: usize,
        columns: usize,
    },
    EmptyMemberName,
    InvalidMemberName(String),
    DuplicateMemberName(String),
}

impl std::fmt::Display for NumpyWriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(source) => write!(f, "The NumPy array could not be written: {source}"),
            Self::Zip(source) => source.fmt(f),
            Self::MatrixColumnLimit { columns: None } => {
                f.write_str("NumPy matrix has too many columns")
            }
            Self::MatrixColumnLimit {
                columns: Some(columns),
            } => write!(
                f,
                "This result has {columns} columns; RSpice reads at most {MAX_COLUMNS} from a NumPy source."
            ),
            Self::ArchiveMemberLimit { members: None } => write!(
                f,
                "This result needs too many archive members; RSpice reads at most {MAX_COLUMNS}."
            ),
            Self::ArchiveMemberLimit {
                members: Some(members),
            } => write!(
                f,
                "This result needs {members} archive members; RSpice reads at most {MAX_COLUMNS}."
            ),
            Self::EmptyMatrix => {
                f.write_str("A NumPy matrix needs coordinate samples and at least one signal.")
            }
            Self::EmptyArchive => {
                f.write_str("A NumPy archive needs coordinate samples and at least one signal.")
            }
            Self::SampleCount {
                name,
                real,
                coordinate,
                ..
            } => write!(
                f,
                "'{name}' has {real} samples against {coordinate} coordinate samples; the export is refused rather than padded or truncated."
            ),
            Self::MatrixSizeOverflow { .. } => {
                f.write_str("NumPy matrix exceeds the supported sample count")
            }
            Self::EmptyMemberName => f.write_str(
                "A signal with no name cannot become an archive member; export CSV instead.",
            ),
            Self::InvalidMemberName(name) => write!(
                f,
                "'{name}' cannot be an archive member name: RSpice refuses an archive member that is \
             absolute, contains '..', or contains a backslash. Export CSV or an RSpice bundle."
            ),
            Self::DuplicateMemberName(name) => write!(
                f,
                "Two signals both claim the archive member '{name}'. An archive names its arrays, so \
                 the names have to differ; RSpice compares them without regard to case."
            ),
        }
    }
}

impl std::error::Error for NumpyWriteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(source) => Some(source),
            Self::Zip(source) => Some(source),
            _ => None,
        }
    }
}

/// Encode real samples with the specified NumPy array shape.
pub fn encode_real_array(shape: &[u64], values: &[f64]) -> Result<Vec<u8>, NumpyWriteError> {
    let mut bytes = Vec::new();
    let mut writer = npyz::WriteOptions::<f64>::new()
        .default_dtype()
        .shape(shape)
        .writer(&mut bytes)
        .begin_nd()
        .map_err(NumpyWriteError::Io)?;
    writer
        .extend(values.iter().copied())
        .map_err(NumpyWriteError::Io)?;
    writer.finish().map_err(NumpyWriteError::Io)?;
    Ok(bytes)
}

/// Encode rectangular complex samples with the specified NumPy array shape.
pub fn encode_complex_array(
    shape: &[u64],
    values: &[Complex64],
) -> Result<Vec<u8>, NumpyWriteError> {
    let mut bytes = Vec::new();
    let mut writer = npyz::WriteOptions::<Complex64>::new()
        .default_dtype()
        .shape(shape)
        .writer(&mut bytes)
        .begin_nd()
        .map_err(NumpyWriteError::Io)?;
    writer
        .extend(values.iter().copied())
        .map_err(NumpyWriteError::Io)?;
    writer.finish().map_err(NumpyWriteError::Io)?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_shape_failure_retains_the_writer_io_cause() {
        use std::error::Error as _;
        let error = encode_real_array(&[2], &[1.0]).unwrap_err();
        let NumpyWriteError::Io(source) = &error else {
            panic!("expected writer error: {error}");
        };
        assert!(error.source().unwrap().is::<std::io::Error>());
        assert_eq!(
            error.to_string(),
            format!("The NumPy array could not be written: {source}")
        );
    }
}

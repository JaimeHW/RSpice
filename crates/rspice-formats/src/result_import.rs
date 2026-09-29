//! Format identification for result sources already bounded by the host.

use rspice_results::result_import::ResultImportFormat;
use std::path::Path;

#[derive(Debug)]
pub enum ResultFormatError {
    Ambiguous {
        extension: String,
        extension_format: ResultImportFormat,
        signature_format: ResultImportFormat,
    },
    UnknownBinary(std::str::Utf8Error),
    UnsupportedExtension(String),
    NoHeader,
    MissingDelimiter,
    MixedDelimiters {
        commas: usize,
        tabs: usize,
    },
}

impl std::fmt::Display for ResultFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ambiguous {
                extension,
                extension_format,
                signature_format,
            } => write!(
                f,
                "the .{extension} extension identifies '{}' but the file signature identifies '{}'; refusing an ambiguous import",
                extension_format.canonical_id(),
                signature_format.canonical_id()
            ),
            Self::UnknownBinary(_) => {
                f.write_str("the result format is unknown and has no recognized binary signature")
            }
            Self::UnsupportedExtension(extension) => write!(
                f,
                "unsupported .{extension} result format; select a UTF-8 CSV or TSV file"
            ),
            Self::NoHeader => f.write_str("the selected file contains no header row"),
            Self::MissingDelimiter => {
                f.write_str("could not infer CSV or TSV delimiter from the header")
            }
            Self::MixedDelimiters { .. } => f.write_str(
                "the header mixes comma and tab delimiters; use a consistent CSV or TSV file",
            ),
        }
    }
}

impl std::error::Error for ResultFormatError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::UnknownBinary(source) => Some(source),
            _ => None,
        }
    }
}

pub fn identify_result_import_format(
    source_name: &str,
    bytes: &[u8],
) -> Result<ResultImportFormat, ResultFormatError> {
    let extension = Path::new(source_name)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase);
    let strong_signature = strong_result_format_signature(bytes);
    let by_extension = match extension.as_deref() {
        Some("rspiceresult") => Some(ResultImportFormat::RSpiceResultBundle),
        Some("rspicedata") => Some(ResultImportFormat::RSpiceDatasetBundle),
        Some("csv") => Some(ResultImportFormat::CsvRfc4180),
        Some("tsv" | "tab") => Some(ResultImportFormat::Tsv),
        Some("ts") => Some(ResultImportFormat::TouchstoneV2),
        Some("snp") => Some(if text_looks_touchstone_v2(bytes) {
            ResultImportFormat::TouchstoneV2
        } else {
            ResultImportFormat::TouchstoneV1
        }),
        Some(extension)
            if extension.starts_with('s')
                && extension.ends_with('p')
                && extension.len() > 2
                && extension[1..extension.len() - 1]
                    .bytes()
                    .all(|byte| byte.is_ascii_digit()) =>
        {
            Some(ResultImportFormat::TouchstoneV1)
        }
        Some("h5" | "hdf5") => Some(ResultImportFormat::Hdf5),
        Some("arrow" | "feather") => Some(ResultImportFormat::ArrowIpc),
        Some("parquet") => Some(ResultImportFormat::Parquet),
        Some("npy") => Some(ResultImportFormat::NumpyNpy),
        Some("npz") => Some(ResultImportFormat::NumpyNpz),
        Some("mat") => Some(if bytes.starts_with(b"\x89HDF\r\n\x1a\n") {
            ResultImportFormat::MatlabV73
        } else {
            ResultImportFormat::MatlabV5
        }),
        Some("raw") => Some(ResultImportFormat::SpiceRaw),
        Some("psfascii") => Some(ResultImportFormat::PsfAscii),
        Some("vcd") => Some(ResultImportFormat::Vcd),
        Some("fst") => Some(ResultImportFormat::Fst),
        _ => None,
    };

    if let (Some(extension_format), Some(signature_format)) = (by_extension, strong_signature)
        && extension_format != signature_format
        && !matches!(
            (extension_format, signature_format),
            (ResultImportFormat::MatlabV73, ResultImportFormat::Hdf5)
        )
    {
        return Err(ResultFormatError::Ambiguous {
            extension: extension.unwrap_or_default(),
            extension_format,
            signature_format,
        });
    }
    if let Some(format) = by_extension.or(strong_signature) {
        return Ok(format);
    }

    let text = std::str::from_utf8(bytes).map_err(ResultFormatError::UnknownBinary)?;
    if text_looks_touchstone_v2(bytes) {
        return Ok(ResultImportFormat::TouchstoneV2);
    }
    if text.lines().any(|line| line.trim_start().starts_with('#'))
        && text.to_ascii_lowercase().contains(" s ")
    {
        return Ok(ResultImportFormat::TouchstoneV1);
    }
    let delimiter = infer_delimiter(source_name, text)?;
    Ok(if delimiter == b'\t' {
        ResultImportFormat::Tsv
    } else {
        ResultImportFormat::CsvRfc4180
    })
}

fn strong_result_format_signature(bytes: &[u8]) -> Option<ResultImportFormat> {
    if bytes.starts_with(b"\x89HDF\r\n\x1a\n") {
        Some(ResultImportFormat::Hdf5)
    } else if bytes.starts_with(b"MATLAB 5.0 MAT-file") {
        Some(ResultImportFormat::MatlabV5)
    } else if bytes.starts_with(b"\x93NUMPY") {
        Some(ResultImportFormat::NumpyNpy)
    } else if bytes.starts_with(b"PK\x03\x04") {
        None
    } else if bytes.starts_with(b"PAR1") && bytes.ends_with(b"PAR1") {
        Some(ResultImportFormat::Parquet)
    } else if bytes.starts_with(b"ARROW1") || bytes.ends_with(b"ARROW1") {
        Some(ResultImportFormat::ArrowIpc)
    } else if crate::fst::looks_like_fst(bytes) {
        Some(ResultImportFormat::Fst)
    } else {
        let prefix = std::str::from_utf8(bytes.get(..bytes.len().min(8_192))?).ok()?;
        if prefix.contains("$timescale") && prefix.contains("$scope") {
            Some(ResultImportFormat::Vcd)
        } else if prefix.lines().any(|line| line.starts_with("Plotname:"))
            && prefix
                .lines()
                .any(|line| line.starts_with("No. Variables:"))
        {
            Some(ResultImportFormat::SpiceRaw)
        } else {
            None
        }
    }
}

fn text_looks_touchstone_v2(bytes: &[u8]) -> bool {
    std::str::from_utf8(bytes.get(..bytes.len().min(8_192)).unwrap_or(bytes)).is_ok_and(|prefix| {
        prefix.lines().any(|line| {
            line.trim_start()
                .to_ascii_lowercase()
                .starts_with("[version]")
        })
    })
}

fn infer_delimiter(source_name: &str, text: &str) -> Result<u8, ResultFormatError> {
    let extension = Path::new(source_name)
        .extension()
        .and_then(|extension| extension.to_str())
        .map(str::to_ascii_lowercase);
    match extension.as_deref() {
        Some("csv") => return Ok(b','),
        Some("tsv") => return Ok(b'\t'),
        Some(extension) => {
            return Err(ResultFormatError::UnsupportedExtension(
                extension.to_owned(),
            ));
        }
        None => {}
    }

    let header = text
        .lines()
        .find(|line| !line.trim().is_empty())
        .ok_or(ResultFormatError::NoHeader)?;
    let commas = header.bytes().filter(|byte| *byte == b',').count();
    let tabs = header.bytes().filter(|byte| *byte == b'\t').count();
    match (commas, tabs) {
        (0, 0) => Err(ResultFormatError::MissingDelimiter),
        (commas, tabs) if commas > 0 && tabs > 0 => {
            Err(ResultFormatError::MixedDelimiters { commas, tabs })
        }
        (_, 0) => Ok(b','),
        (0, _) => Ok(b'\t'),
        _ => unreachable!(),
    }
}

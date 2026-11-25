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
    /// Neither text nor a container this importer knows.
    UnknownBinary,
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
                "the .{extension} extension names a {} file but the file's contents are {}; refusing an ambiguous import",
                extension_format.label(),
                signature_format.label()
            ),
            Self::UnknownBinary => f.write_str(
                "the file is neither text nor a result container this importer reads",
            ),
            Self::UnsupportedExtension(extension) => write!(
                f,
                "this importer does not read .{extension} files; choose a file of a listed type"
            ),
            Self::NoHeader => f.write_str("the selected file contains no header row"),
            Self::MissingDelimiter => f.write_str(
                "the header row has no commas, semicolons or tabs to tell its columns apart",
            ),
            Self::MixedDelimiters { .. } => f.write_str(
                "the header mixes tabs with commas or semicolons; save the table with one delimiter",
            ),
        }
    }
}

impl std::error::Error for ResultFormatError {}

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
        // Touchstone 2.0 lets a file keep its `.sNp` name, and ADS writes it that way: the
        // `[Version]` keyword, not the extension, says which version a file is.
        Some(extension)
            if extension == "snp"
                || (extension.starts_with('s')
                    && extension.ends_with('p')
                    && extension.len() > 2
                    && extension[1..extension.len() - 1]
                        .bytes()
                        .all(|byte| byte.is_ascii_digit())) =>
        {
            Some(if text_looks_touchstone_v2(bytes) {
                ResultImportFormat::TouchstoneV2
            } else {
                ResultImportFormat::TouchstoneV1
            })
        }
        Some("h5" | "hdf5") => Some(ResultImportFormat::Hdf5),
        Some("arrow" | "feather") => Some(ResultImportFormat::ArrowIpc),
        Some("parquet") => Some(ResultImportFormat::Parquet),
        Some("npy") => Some(ResultImportFormat::NumpyNpy),
        Some("npz") => Some(ResultImportFormat::NumpyNpz),
        Some("mat") => Some(if bytes.starts_with(b"\x89HDF\r\n\x1a\n") {
            ResultImportFormat::MatlabV73
        } else if !bytes.starts_with(crate::matlab::HEADER_SIGNATURE.as_bytes())
            && crate::matlab::reader::looks_like_matlab_v4(bytes)
        {
            // Level 4 has no signature: its first variable's header is what says so.
            ResultImportFormat::MatlabV4
        } else {
            ResultImportFormat::MatlabV5
        }),
        Some("raw") => Some(ResultImportFormat::SpiceRaw),
        Some("qraw") => Some(ResultImportFormat::QspiceRaw),
        // HSPICE numbers its outputs by `.alter`: `deck.tr0`, `deck.tr1` and on.
        Some(extension)
            if ["tr", "ac", "sw"]
                .iter()
                .any(|kind| numbered(extension, kind)) =>
        {
            Some(ResultImportFormat::Hspice)
        }
        Some(extension)
            if ["mt", "ma", "ms"]
                .iter()
                .any(|kind| numbered(extension, kind)) =>
        {
            Some(ResultImportFormat::HspiceMeasure)
        }
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
                // QSPICE writes its ASCII `.qraw` with a rawfile's header.
                | (ResultImportFormat::QspiceRaw, ResultImportFormat::SpiceRaw)
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

    let text = crate::delimited::decode_source_text(bytes)
        .map_err(|_| ResultFormatError::UnknownBinary)?;
    let text = text.as_ref();
    if text_looks_touchstone_v2(text.as_bytes()) {
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

/// `extension` is `kind` followed by a number, as `tr0` or `mt12`.
fn numbered(extension: &str, kind: &str) -> bool {
    extension.strip_prefix(kind).is_some_and(|number| {
        !number.is_empty() && number.bytes().all(|byte| byte.is_ascii_digit())
    })
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
        // Excel saves a sheet as "Unicode Text" or "Text (Tab delimited)" under `.txt`.
        Some("txt") | None => {}
        Some(extension) => {
            return Err(ResultFormatError::UnsupportedExtension(
                extension.to_owned(),
            ));
        }
    }

    let (_, table) = crate::delimited::skip_preamble(text);
    let header = table.lines().next().ok_or(ResultFormatError::NoHeader)?;
    if let Some(delimiter) = crate::delimited::header_delimiter(table) {
        return Ok(delimiter);
    }
    let commas = header
        .bytes()
        .filter(|byte| matches!(byte, b',' | b';'))
        .count();
    let tabs = header.bytes().filter(|byte| *byte == b'\t').count();
    Err(if commas == 0 && tabs == 0 {
        ResultFormatError::MissingDelimiter
    } else {
        ResultFormatError::MixedDelimiters { commas, tabs }
    })
}

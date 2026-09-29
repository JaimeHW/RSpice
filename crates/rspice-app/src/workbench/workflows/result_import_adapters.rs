//! Bounded adapters for structured and simulator-native result formats.
//!
//! Every entry point consumes the already size-limited byte slice owned by the
//! import transaction.  Adapters reject ambiguous mappings rather than
//! inventing domain or signal identity.

use super::*;
use rspice_results::result_import::waveforms::ImportedSignal;
use std::io::Cursor;

const MAX_ARCHIVE_MEMBERS: usize = 1_024;
const MAX_ARCHIVE_EXPANDED_BYTES: u64 = MAX_RESULT_DATASET_BYTES;
const MAX_SIGNAL_NAME_BYTES: usize = 1_024;
const MAX_RESULT_VALUES: usize = MAX_RESULT_DATASET_BYTES as usize / std::mem::size_of::<f64>();

fn adapter_error(format: ResultImportFormat, detail: impl std::fmt::Display) -> String {
    format!("{} import: {detail}", format.canonical_id())
}

fn analysis_from_coordinate(name: &str) -> AnalysisType {
    imported_analysis_type(rspice_formats::WaveformDomain::from_coordinate_name(name))
}

fn parse_analysis(format: ResultImportFormat, value: &str) -> Result<AnalysisType, String> {
    value
        .parse::<rspice_formats::WaveformDomain>()
        .map(imported_analysis_type)
        .map_err(|error| adapter_error(format, error))
}

fn waveform_import_limits() -> rspice_results::result_import::waveforms::WaveformImportLimits {
    rspice_results::result_import::waveforms::WaveformImportLimits {
        min_rows: MIN_RESULT_ROWS,
        max_rows: MAX_RESULT_ROWS,
        max_columns: MAX_RESULT_COLUMNS,
        max_values: MAX_RESULT_VALUES,
        max_signal_name_bytes: MAX_SIGNAL_NAME_BYTES,
    }
}

fn finish_dataset(
    format: ResultImportFormat,
    analysis_type: AnalysisType,
    coordinate_name: impl Into<String>,
    coordinate: Vec<f64>,
    signals: Vec<ImportedSignal>,
) -> Result<ParsedResultDataset, String> {
    let imported = rspice_results::result_import::waveforms::assemble_imported_waveforms(
        format,
        analysis_type,
        coordinate_name,
        coordinate,
        signals,
        waveform_import_limits(),
    )?;
    Ok(present_imported_waveforms(format, analysis_type, imported))
}

fn present_imported_waveforms(
    format: ResultImportFormat,
    analysis_type: AnalysisType,
    imported: rspice_results::result_import::waveforms::ImportedWaveforms,
) -> ParsedResultDataset {
    ParsedResultDataset {
        source_format: format,
        analysis_type,
        coordinate_name: imported.coordinate_name,
        sample_count: imported.sample_count,
        waveforms: imported
            .waveforms
            .into_iter()
            .enumerate()
            .map(|(index, data)| WaveformData {
                data,
                color: trace_color(index).to_owned(),
                visible: true,
                display_cache: None,
            })
            .collect(),
        family_metadata: None,
        delimiter: 0,
        notes: Vec::new(),
        event_payload: None,
    }
}

// -------------------------------------------------------------------------
// Native RSpice bundles

pub(super) fn parse_native_bundle(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    use rspice_formats::native_bundle::{
        NativeBundleKind, NativeBundleReadLimits, decode_native_bundle,
    };

    let kind = match format {
        ResultImportFormat::RSpiceResultBundle => NativeBundleKind::Result,
        ResultImportFormat::RSpiceDatasetBundle => NativeBundleKind::Dataset,
        _ => unreachable!(),
    };
    let dataset = decode_native_bundle(
        bytes,
        kind,
        NativeBundleReadLimits {
            max_members: MAX_ARCHIVE_MEMBERS,
            max_expanded_bytes: MAX_ARCHIVE_EXPANDED_BYTES,
            max_member_bytes: MAX_RESULT_DATASET_BYTES,
        },
    )
    .map_err(|error| adapter_error(format, error))?;
    let analysis = parse_analysis(format, &dataset.analysis)?;
    let mut signals = Vec::with_capacity(dataset.signals.len());
    for signal in dataset.signals {
        let (real, imag) = match (signal.values, signal.real, signal.imag) {
            (Some(values), None, None) => (values, None),
            (None, Some(real), Some(imag)) => (real, Some(imag)),
            _ => {
                return Err(adapter_error(
                    format,
                    format_args!(
                        "signal '{}' must provide either values or both real and imag",
                        signal.name
                    ),
                ));
            }
        };
        signals.push(ImportedSignal {
            name: signal.name,
            real,
            imag,
            unit: signal.unit,
        });
    }
    finish_dataset(
        format,
        analysis,
        dataset.coordinate.name,
        dataset.coordinate.values,
        signals,
    )
}

// -------------------------------------------------------------------------
// HDF5 and MATLAB 7.3

pub(super) fn parse_hdf5(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let decoded = rspice_formats::hdf5::decode_hdf5(bytes, hdf5_limits(), format.canonical_id())
        .map_err(|error| error.to_string())?;
    finish_hdf5(format, decoded)
}

pub(super) fn parse_matlab_v73(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let decoded =
        rspice_formats::hdf5::decode_matlab_v73(bytes, hdf5_limits(), format.canonical_id())
            .map_err(|error| error.to_string())?;
    finish_hdf5(format, decoded)
}

fn hdf5_limits() -> rspice_formats::hdf5::Hdf5Limits<'static> {
    rspice_formats::hdf5::Hdf5Limits {
        max_columns: MAX_RESULT_COLUMNS,
        max_values: MAX_RESULT_VALUES,
        coordinate_names: &RESULT_COORDINATE_NAMES,
    }
}

fn finish_hdf5(
    format: ResultImportFormat,
    decoded: rspice_formats::hdf5::DecodedHdf5,
) -> Result<ParsedResultDataset, String> {
    match decoded {
        rspice_formats::hdf5::DecodedHdf5::Section {
            family,
            coordinate_name,
            coordinate,
            signals,
        } => {
            let analysis = match family {
                rspice_formats::hdf5::Hdf5SectionFamily::Transient => AnalysisType::Transient,
                rspice_formats::hdf5::Hdf5SectionFamily::DcSweep => AnalysisType::DcSweep,
                rspice_formats::hdf5::Hdf5SectionFamily::Ac => AnalysisType::Ac,
            };
            let signals = imported_signals(signals);
            finish_dataset(format, analysis, coordinate_name, coordinate, signals)
        }
        rspice_formats::hdf5::DecodedHdf5::Root {
            coordinate_name,
            coordinate,
            columns,
        } => {
            let signals = combine_real_imag_columns(format, columns)?;
            finish_dataset(
                format,
                analysis_from_coordinate(&coordinate_name),
                coordinate_name,
                coordinate,
                signals,
            )
        }
    }
}

/// Whether `name` is one of the coordinate names a headerless source may use.
fn is_coordinate_name(name: &str) -> bool {
    RESULT_COORDINATE_NAMES
        .iter()
        .any(|candidate| name.eq_ignore_ascii_case(candidate))
}

/// [`RESULT_COORDINATE_NAMES`] as a refusal spells them, so the sentence a
/// reader is shown cannot drift from the list the reader actually accepts.
fn stated_coordinate_names() -> String {
    rspice_formats::hdf5::stated_coordinate_names(&RESULT_COORDINATE_NAMES)
}

// -------------------------------------------------------------------------
// Arrow IPC and Parquet

pub(super) fn parse_arrow_ipc(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let table =
        rspice_formats::columnar::decode_arrow_ipc(bytes, columnar_limits(), format.canonical_id())
            .map_err(|error| error.to_string())?;
    finish_columnar_table(format, table)
}

pub(super) fn parse_parquet(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let table =
        rspice_formats::columnar::decode_parquet(bytes, columnar_limits(), format.canonical_id())
            .map_err(|error| error.to_string())?;
    finish_columnar_table(format, table)
}

fn columnar_limits() -> rspice_formats::columnar::ColumnarLimits {
    rspice_formats::columnar::ColumnarLimits {
        max_columns: MAX_RESULT_COLUMNS,
        max_rows: MAX_RESULT_ROWS,
        max_values: MAX_RESULT_VALUES,
    }
}

fn finish_columnar_table(
    format: ResultImportFormat,
    table: rspice_formats::columnar::DecodedColumnarTable,
) -> Result<ParsedResultDataset, String> {
    let rspice_formats::columnar::DecodedColumnarTable {
        metadata,
        mut columns,
    } = table;
    let coordinate_name = metadata
        .get("rspice.coordinate")
        .cloned()
        .unwrap_or_else(|| columns[0].0.clone());
    let coordinate_index = columns
        .iter()
        .position(|(name, _)| name == &coordinate_name)
        .ok_or_else(|| {
            adapter_error(
                format,
                format_args!("schema metadata names missing coordinate '{coordinate_name}'"),
            )
        })?;
    let coordinate = columns.remove(coordinate_index).1;
    let analysis = metadata
        .get("rspice.analysis")
        .map(|value| parse_analysis(format, value))
        .transpose()?
        .unwrap_or_else(|| analysis_from_coordinate(&coordinate_name));
    let signals = combine_real_imag_columns(format, columns)?;
    finish_dataset(format, analysis, coordinate_name, coordinate, signals)
}

fn imported_signals(
    signals: Vec<rspice_formats::numeric::DecodedNumericSignal>,
) -> Vec<ImportedSignal> {
    signals
        .into_iter()
        .map(|signal| ImportedSignal {
            name: signal.name,
            real: signal.real,
            imag: signal.imag,
            unit: signal.unit,
        })
        .collect()
}

fn combine_real_imag_columns(
    format: ResultImportFormat,
    columns: Vec<(String, Vec<f64>)>,
) -> Result<Vec<ImportedSignal>, String> {
    rspice_formats::numeric::combine_real_imag_columns(columns)
        .map(imported_signals)
        .map_err(|error| adapter_error(format, error))
}

// -------------------------------------------------------------------------
// NumPy NPY and NPZ

pub(super) fn parse_npy(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let array =
        rspice_formats::numpy::reader::decode_npy(bytes, MAX_RESULT_VALUES, format.canonical_id())?;
    let (coordinate, signals) = rspice_formats::numpy::reader::npy_matrix_to_dataset(
        array,
        MAX_RESULT_ROWS,
        MAX_RESULT_COLUMNS,
        format.canonical_id(),
    )?;
    let signals = imported_signals(signals);
    finish_dataset(format, AnalysisType::DcSweep, "sample", coordinate, signals)
}

pub(super) fn parse_npz(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let mut arrays = rspice_formats::numpy::archive::decode_npz_arrays(
        bytes,
        rspice_formats::numpy::archive::NpzReadLimits {
            max_members: MAX_ARCHIVE_MEMBERS,
            max_expanded_bytes: MAX_ARCHIVE_EXPANDED_BYTES,
            max_numeric_values: MAX_RESULT_VALUES,
        },
        format.canonical_id(),
    )?;
    let coordinate_index = arrays
        .iter()
        .position(|(name, _)| is_coordinate_name(name))
        .ok_or_else(|| {
            adapter_error(
                format,
                format_args!(
                    "NPZ requires one coordinate array named {}",
                    stated_coordinate_names()
                ),
            )
        })?;
    let (coordinate_name, coordinate_array) = arrays.remove(coordinate_index);
    if coordinate_array.is_complex() {
        return Err(adapter_error(
            format,
            "NPZ coordinate array cannot be complex",
        ));
    }
    let coordinate = rspice_formats::numpy::reader::npy_vector(
        &coordinate_array,
        format.canonical_id(),
        &coordinate_name,
    )?
    .0;
    let mut signals = Vec::with_capacity(arrays.len());
    for (name, array) in arrays {
        let (real, imag) =
            rspice_formats::numpy::reader::npy_vector(&array, format.canonical_id(), &name)?;
        signals.push(ImportedSignal {
            name,
            real,
            imag,
            unit: None,
        });
    }
    finish_dataset(
        format,
        analysis_from_coordinate(&coordinate_name),
        coordinate_name,
        coordinate,
        signals,
    )
}

// -------------------------------------------------------------------------
// MATLAB v5

pub(super) fn parse_matlab_v5(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let parsed = rspice_formats::matlab::reader::MatFile::parse(
        bytes,
        MAX_RESULT_COLUMNS.saturating_mul(2),
        format.canonical_id(),
    )?;
    let arrays = parsed.arrays();
    let coordinate_index = arrays
        .iter()
        .position(|array| is_coordinate_name(array.name()));
    if let Some(coordinate_index) = coordinate_index {
        let coordinate_array = &arrays[coordinate_index];
        let (coordinate, coordinate_imag) = coordinate_array.values(format.canonical_id())?;
        if coordinate_imag.is_some() || !coordinate_array.is_vector() {
            return Err(adapter_error(
                format,
                "MATLAB coordinate variable must be a real vector",
            ));
        }
        let mut signals = Vec::new();
        for (index, array) in arrays.iter().enumerate() {
            if index == coordinate_index {
                continue;
            }
            if !array.is_vector() {
                return Err(adapter_error(
                    format,
                    format_args!("MATLAB variable '{}' is not a vector", array.name()),
                ));
            }
            let (real, imag) = array.values(format.canonical_id())?;
            signals.push(ImportedSignal {
                name: array.name().to_owned(),
                real,
                imag,
                unit: None,
            });
        }
        return finish_dataset(
            format,
            analysis_from_coordinate(coordinate_array.name()),
            coordinate_array.name(),
            coordinate,
            signals,
        );
    }

    if arrays.len() != 1 {
        return Err(adapter_error(
            format,
            format_args!(
                "MATLAB file requires a coordinate variable named {}",
                stated_coordinate_names()
            ),
        ));
    }
    let array = &arrays[0];
    let size = array.size();
    if size.len() != 2 || size[0] < MIN_RESULT_ROWS || size[1] < 2 {
        return Err(adapter_error(
            format,
            "without a named coordinate, MATLAB data must be one rows-by-columns table with the coordinate in column one",
        ));
    }
    let (real, imag) = array.values(format.canonical_id())?;
    if imag.is_some() {
        return Err(adapter_error(
            format,
            "a complex MATLAB table requires separate named coordinate and signal variables",
        ));
    }
    let rows = size[0];
    let columns = size[1];
    let coordinate = real[..rows].to_vec();
    let signals = (1..columns)
        .map(|column| ImportedSignal {
            name: format!("{}.{}", array.name(), column),
            real: real[column * rows..(column + 1) * rows].to_vec(),
            imag: None,
            unit: None,
        })
        .collect();
    finish_dataset(format, AnalysisType::DcSweep, "x", coordinate, signals)
}

// -------------------------------------------------------------------------
// SPICE RAW

pub(super) fn parse_spice_raw(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let mut limits = rspice_core::ResourceLimits::default();
    limits.max_external_data_bytes = MAX_RESULT_DATASET_BYTES as usize;
    limits.max_external_data_values = MAX_RESULT_VALUES;
    let parsed = rspice_core::io::parse_raw_reader_with_limits(&mut Cursor::new(bytes), limits)
        .map_err(|error| adapter_error(format, error))?;
    let mut waveforms = parsed.waveforms.into_iter();
    let scale = waveforms
        .next()
        .ok_or_else(|| adapter_error(format, "rawfile contains no variables"))?;
    let coordinate_name = scale.name;
    let coordinate = scale.y;
    let mut signals: Vec<ImportedSignal> = Vec::new();
    for waveform in waveforms {
        signals.push(ImportedSignal {
            name: waveform.name,
            real: waveform.y,
            imag: waveform.y_imag,
            unit: None,
        });
    }
    let plot = parsed.header.plotname.to_ascii_lowercase();
    let analysis = if parsed.header.is_complex || plot.contains("ac") {
        AnalysisType::Ac
    } else if plot.contains("tran") || coordinate_name.to_ascii_lowercase().contains("time") {
        AnalysisType::Transient
    } else {
        AnalysisType::DcSweep
    };
    finish_dataset(format, analysis, coordinate_name, coordinate, signals)
}

// -------------------------------------------------------------------------
// Cadence PSF ASCII

pub(super) fn parse_psf_ascii(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    use rspice_formats::psf::{PsfReadLimits, decode_psf_ascii};

    let decoded = decode_psf_ascii(
        bytes,
        PsfReadLimits {
            max_columns: MAX_RESULT_COLUMNS,
            max_rows: MAX_RESULT_ROWS,
        },
    )
    .map_err(|error| adapter_error(format, error))?;
    let signals = decoded
        .signal_names
        .into_iter()
        .zip(decoded.signal_values)
        .map(|(name, real)| ImportedSignal {
            name,
            real,
            imag: None,
            unit: None,
        })
        .collect();
    finish_dataset(
        format,
        imported_analysis_type(decoded.domain),
        decoded.coordinate_name,
        decoded.coordinate,
        signals,
    )
}

// -------------------------------------------------------------------------

#[path = "result_import_adapters/digital.rs"]
mod digital;

pub(super) use digital::{parse_fst, parse_vcd};
pub(super) use rspice_formats::fst::looks_like_fst;

#[cfg(test)]
use digital::preflight_fst_for_test as preflight_fst;

#[cfg(test)]
#[path = "result_import_adapters/tests.rs"]
mod tests;

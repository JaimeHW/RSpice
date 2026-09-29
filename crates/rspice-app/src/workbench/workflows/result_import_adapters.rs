//! Bounded adapters for structured and simulator-native result formats.
//!
//! Every entry point consumes the already size-limited byte slice owned by the
//! import transaction.  Adapters reject ambiguous mappings rather than
//! inventing domain or signal identity.

use super::*;
use rspice_results::result_import::waveforms::ImportedSignal;

const MAX_ARCHIVE_MEMBERS: usize = 1_024;
const MAX_ARCHIVE_EXPANDED_BYTES: u64 = MAX_RESULT_DATASET_BYTES;
const MAX_SIGNAL_NAME_BYTES: usize = 1_024;
const MAX_RESULT_VALUES: usize = MAX_RESULT_DATASET_BYTES as usize / std::mem::size_of::<f64>();

fn adapter_error(format: ResultImportFormat, detail: impl std::fmt::Display) -> String {
    format!("{} import: {detail}", format.canonical_id())
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
    finish_numeric_dataset(format, dataset)
}

// -------------------------------------------------------------------------
// HDF5 and MATLAB 7.3

pub(super) fn parse_hdf5(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let decoded = rspice_formats::hdf5::decode_hdf5(bytes, hdf5_limits(), format.canonical_id())
        .map_err(|error| error.to_string())?;
    finish_numeric_dataset(format, decoded)
}

pub(super) fn parse_matlab_v73(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let decoded =
        rspice_formats::hdf5::decode_matlab_v73(bytes, hdf5_limits(), format.canonical_id())
            .map_err(|error| error.to_string())?;
    finish_numeric_dataset(format, decoded)
}

fn hdf5_limits() -> rspice_formats::hdf5::Hdf5Limits<'static> {
    rspice_formats::hdf5::Hdf5Limits {
        max_columns: MAX_RESULT_COLUMNS,
        max_values: MAX_RESULT_VALUES,
        coordinate_names: &RESULT_COORDINATE_NAMES,
    }
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
    finish_numeric_dataset(format, table)
}

pub(super) fn parse_parquet(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let table =
        rspice_formats::columnar::decode_parquet(bytes, columnar_limits(), format.canonical_id())
            .map_err(|error| error.to_string())?;
    finish_numeric_dataset(format, table)
}

fn columnar_limits() -> rspice_formats::columnar::ColumnarLimits {
    rspice_formats::columnar::ColumnarLimits {
        max_columns: MAX_RESULT_COLUMNS,
        max_rows: MAX_RESULT_ROWS,
        max_values: MAX_RESULT_VALUES,
    }
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

// -------------------------------------------------------------------------
// NumPy NPY and NPZ

pub(super) fn parse_npy(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let array =
        rspice_formats::numpy::reader::decode_npy(bytes, MAX_RESULT_VALUES, format.canonical_id())
            .map_err(|error| error.to_string())?;
    let decoded = rspice_formats::numpy::reader::npy_matrix_to_dataset(
        array,
        MAX_RESULT_ROWS,
        MAX_RESULT_COLUMNS,
        format.canonical_id(),
    )
    .map_err(|error| error.to_string())?;
    finish_numeric_dataset(format, decoded)
}

pub(super) fn parse_npz(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let decoded = rspice_formats::numpy::archive::decode_npz(
        bytes,
        rspice_formats::numpy::archive::NpzReadLimits {
            max_members: MAX_ARCHIVE_MEMBERS,
            max_expanded_bytes: MAX_ARCHIVE_EXPANDED_BYTES,
            max_numeric_values: MAX_RESULT_VALUES,
        },
        &RESULT_COORDINATE_NAMES,
        format.canonical_id(),
    )
    .map_err(|error| error.to_string())?;
    finish_numeric_dataset(format, decoded)
}

fn finish_numeric_dataset(
    format: ResultImportFormat,
    decoded: rspice_formats::numeric::DecodedNumericDataset,
) -> Result<ParsedResultDataset, String> {
    finish_dataset(
        format,
        imported_analysis_type(decoded.domain),
        decoded.coordinate_name,
        decoded.coordinate,
        imported_signals(decoded.signals),
    )
}

// -------------------------------------------------------------------------
// MATLAB v5

pub(super) fn parse_matlab_v5(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let decoded = rspice_formats::matlab::reader::decode_matlab_v5(
        bytes,
        rspice_formats::matlab::reader::MatlabReadLimits {
            max_variables: MAX_RESULT_COLUMNS.saturating_mul(2),
            min_rows: MIN_RESULT_ROWS,
            coordinate_names: &RESULT_COORDINATE_NAMES,
        },
        format.canonical_id(),
    )
    .map_err(|error| error.to_string())?;
    finish_numeric_dataset(format, decoded)
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
    let decoded = rspice_formats::spice_raw::decode_spice_raw(bytes, limits)
        .map_err(|error| adapter_error(format, error))?;
    finish_numeric_dataset(format, decoded)
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

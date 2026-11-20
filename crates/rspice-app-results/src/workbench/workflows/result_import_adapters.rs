//! Bounded adapters for structured and simulator-native result formats.
//!
//! Every entry point consumes the already size-limited byte slice owned by the
//! import transaction.  Adapters reject ambiguous mappings rather than
//! inventing domain or signal identity.

use super::*;
use rspice_results::result_import::waveforms::ImportedSignal;

/// The fewest bytes an archive member takes: its local header (30) and its directory entry (46).
const LEAST_ARCHIVE_MEMBER_BYTES: usize = 76;

/// The most members the archive being read can hold.
fn archive_members() -> usize {
    (source_capacity() / MOST_VALUES_PER_SOURCE_BYTE / LEAST_ARCHIVE_MEMBER_BYTES).max(1)
}

/// The most bytes the archive being read can expand to.
fn archive_expanded_bytes() -> u64 {
    u64::try_from(source_capacity()).unwrap_or(u64::MAX)
}

fn adapter_error(format: ResultImportFormat, detail: impl std::fmt::Display) -> String {
    format!("{} import: {detail}", format.canonical_id())
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

pub(super) fn present_imported_waveforms(
    format: ResultImportFormat,
    analysis_type: AnalysisType,
    imported: rspice_results::result_import::waveforms::ImportedWaveforms,
) -> ParsedResultDataset {
    ParsedResultDataset {
        source_format: format,
        analysis_type,
        coordinate_name: imported.coordinate_name,
        coordinate_unit: None,
        sample_count: imported.sample_count,
        waveforms: imported
            .waveforms
            .into_iter()
            .map(|data| RetainedTrace {
                data,
                color: String::new(),
                visible: true,
            })
            .collect(),
        family_metadata: None,
        delimiter: 0,
        notes: Vec::new(),
        event_payload: None,
        event_dump: None,
        plots: None,
    }
}

// -------------------------------------------------------------------------
// Native RSpice bundles

pub(super) fn parse_native_bundle(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let _source = super::ReadingSource::of(bytes.len());
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
            max_members: archive_members(),
            max_expanded_bytes: archive_expanded_bytes(),
            max_member_bytes: archive_expanded_bytes(),
            max_rows: source_capacity(),
            max_columns: source_capacity(),
            max_numeric_values: source_capacity(),
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
    let _source = super::ReadingSource::of(bytes.len());
    let decoded = rspice_formats::hdf5::decode_hdf5(bytes, hdf5_limits(), format.canonical_id())
        .map_err(|error| error.to_string())?;
    finish_numeric_dataset(format, decoded)
}

pub(super) fn parse_matlab_v73(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let _source = super::ReadingSource::of(bytes.len());
    let decoded =
        rspice_formats::hdf5::decode_matlab_v73(bytes, hdf5_limits(), format.canonical_id())
            .map_err(|error| error.to_string())?;
    finish_numeric_dataset(format, decoded)
}

fn hdf5_limits() -> rspice_formats::hdf5::Hdf5Limits<'static> {
    rspice_formats::hdf5::Hdf5Limits {
        max_columns: source_capacity(),
        max_values: source_capacity(),
        coordinate_names: &RESULT_COORDINATE_NAMES,
    }
}

// -------------------------------------------------------------------------
// Arrow IPC and Parquet

pub(super) fn parse_arrow_ipc(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let _source = super::ReadingSource::of(bytes.len());
    let table =
        rspice_formats::columnar::decode_arrow_ipc(bytes, columnar_limits(), format.canonical_id())
            .map_err(|error| error.to_string())?;
    finish_numeric_dataset(format, table)
}

pub(super) fn parse_parquet(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let _source = super::ReadingSource::of(bytes.len());
    let table =
        rspice_formats::columnar::decode_parquet(bytes, columnar_limits(), format.canonical_id())
            .map_err(|error| error.to_string())?;
    finish_numeric_dataset(format, table)
}

fn columnar_limits() -> rspice_formats::columnar::ColumnarLimits {
    rspice_formats::columnar::ColumnarLimits {
        max_columns: source_capacity(),
        max_rows: source_capacity(),
        max_values: source_capacity(),
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
    let _source = super::ReadingSource::of(bytes.len());
    let array =
        rspice_formats::numpy::reader::decode_npy(bytes, source_capacity(), format.canonical_id())
            .map_err(|error| error.to_string())?;
    let decoded = rspice_formats::numpy::reader::npy_matrix_to_dataset(
        array,
        source_capacity(),
        source_capacity(),
        format.canonical_id(),
    )
    .map_err(|error| error.to_string())?;
    finish_numeric_dataset(format, decoded)
}

pub(super) fn parse_npz(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let _source = super::ReadingSource::of(bytes.len());
    let decoded = rspice_formats::numpy::archive::decode_npz(
        bytes,
        rspice_formats::numpy::archive::NpzReadLimits {
            max_members: archive_members(),
            max_expanded_bytes: archive_expanded_bytes(),
            max_numeric_values: source_capacity(),
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
    let mut parsed = finish_dataset(
        format,
        imported_analysis_type(decoded.domain),
        decoded.coordinate_name,
        decoded.coordinate,
        imported_signals(decoded.signals),
    )?;
    parsed.coordinate_unit = decoded.coordinate_unit;
    Ok(parsed)
}

// -------------------------------------------------------------------------
// MATLAB v5

pub(super) fn parse_matlab_v5(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let _source = super::ReadingSource::of(bytes.len());
    let decoded = rspice_formats::matlab::reader::decode_matlab_v5(
        bytes,
        rspice_formats::matlab::reader::MatlabReadLimits {
            max_variables: source_capacity(),
            max_expanded_bytes: archive_expanded_bytes(),
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

/// Read a rawfile, with the simulator's log beside it when there is one: an
/// LTspice `stepped` rawfile holds every `.step` run in one plot, and its
/// log is what names each run's parameter values.
pub(super) fn parse_spice_raw(
    bytes: &[u8],
    log: Option<&[u8]>,
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let _source = super::ReadingSource::of(bytes.len());
    let mut limits = rspice_core::ResourceLimits::default();
    limits.max_external_data_bytes = usize::MAX;
    limits.max_external_data_values = source_capacity();
    limits.max_result_values = source_capacity();
    let decoded = rspice_formats::spice_raw::decode_spice_raw_plots_with_log(bytes, log, limits)
        .map_err(|error| adapter_error(format, error))?;
    // A rawfile holds one plot per analysis. The reader reviews the first one
    // drawn over a coordinate (an operating point is a scalar result, not the
    // waveform the file was written for) and the rest come in beside it.
    let reviewed = decoded
        .iter()
        .position(|plot| !plot.scalar)
        .unwrap_or_default();
    let several = decoded.len() > 1;
    let mut reviewed_parsed = None;
    let mut further = Vec::with_capacity(decoded.len().saturating_sub(1));
    for (index, plot) in decoded.into_iter().enumerate() {
        // Each run of a stepped plot is its own analysis, titled by its run,
        // and the runs of the file draw as one family.
        let family_member =
            plot.step.map(|step| {
                rspice_results::provenance::AnalysisResultFamilyMember::StepPoint {
                    index: step.index,
                    coordinate: step
                        .coordinate
                        .into_iter()
                        .map(|(name, value)| {
                            rspice_results::provenance::AnalysisResultStepCoordinate { name, value }
                        })
                        .collect(),
                }
            });
        let plot_name = match &family_member {
            Some(member) => member.titled(&plot.plot_name),
            None => plot.plot_name,
        };
        let mut parsed = finish_numeric_dataset(format, plot.dataset).map_err(|error| {
            if several {
                format!("{plot_name} plot: {error}")
            } else {
                error
            }
        })?;
        parsed.event_payload = plot.events.and_then(|events| {
            rspice_results::events::imported_event_payload(
                &rspice_core::execution::VcdEventHistories {
                    digital_traces: events.digital_traces,
                    digital_buses: events.digital_buses,
                    real_traces: events.real_traces,
                    ..Default::default()
                },
            )
        });
        if index == reviewed {
            reviewed_parsed = Some((plot_name, family_member, parsed));
        } else {
            further.push(
                crate::workbench::documents::result_document::import::ResultImportPlot {
                    name: plot_name,
                    family_member,
                    analysis_type: parsed.analysis_type,
                    coordinate_name: parsed.coordinate_name,
                    coordinate_unit: parsed.coordinate_unit,
                    sample_count: parsed.sample_count,
                    waveforms: parsed.waveforms,
                    event_payload: parsed.event_payload,
                },
            );
        }
    }
    let (reviewed_name, reviewed_member, mut parsed) =
        reviewed_parsed.ok_or_else(|| adapter_error(format, "rawfile contains no variables"))?;
    if several {
        parsed.plots = Some(Arc::new(ResultImportPlots {
            reviewed: reviewed_name,
            reviewed_member,
            further,
        }));
    }
    Ok(parsed)
}

// -------------------------------------------------------------------------
// Cadence PSF ASCII

pub(super) fn parse_psf_ascii(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let _source = super::ReadingSource::of(bytes.len());
    use rspice_formats::psf::{PsfReadLimits, decode_psf_ascii};

    let decoded = decode_psf_ascii(
        bytes,
        PsfReadLimits {
            max_columns: source_capacity(),
            max_rows: source_capacity(),
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

#[cfg(test)]
use digital::preflight_fst_for_test as preflight_fst;

#[cfg(test)]
#[path = "result_import_adapters/tests.rs"]
mod tests;

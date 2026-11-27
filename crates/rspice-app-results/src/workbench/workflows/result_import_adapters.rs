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

pub(super) fn adapter_error(format: ResultImportFormat, detail: impl std::fmt::Display) -> String {
    format!("{} import: {detail}", format.label())
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
        NativeBundleKind, NativeBundleRead, NativeBundleReadLimits, read_native_bundle,
    };

    let kind = match format {
        ResultImportFormat::RSpiceResultBundle => NativeBundleKind::Result,
        ResultImportFormat::RSpiceDatasetBundle => NativeBundleKind::Dataset,
        _ => unreachable!(),
    };
    let NativeBundleRead {
        dataset,
        member,
        family,
        unread_members,
    } = read_native_bundle(
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
    let mut parsed = finish_numeric_dataset(format, dataset)?;
    if !unread_members.is_empty() {
        let (holds, them) = if unread_members.len() == 1 {
            ("a member", "it was")
        } else {
            ("members", "they were")
        };
        parsed.notes.push(format!(
            "The bundle also holds {holds} this version of RSpice does not read ({}); {them} \
             left out.",
            unread_members.join(", ")
        ));
    }
    // A bundle of one member of a `.step` sweep or Monte Carlo study reads back as that member,
    // titled the way every route titles one: `Transient · r=2k`; a bundle of every member reads
    // back as each of them, in family order, drawn as one family.
    let analysis_title = |analysis_type: AnalysisType| match analysis_type {
        AnalysisType::Ac => "AC",
        AnalysisType::DcSweep => "DC sweep",
        _ => "Transient",
    };
    let title = |member: Option<&rspice_results::provenance::AnalysisResultFamilyMember>,
                 analysis_type: AnalysisType| {
        let analysis = analysis_title(analysis_type);
        member.map_or_else(|| analysis.to_owned(), |member| member.titled(analysis))
    };
    let member = member.map(|member| member.into_family_member());
    let several = !family.is_empty();
    let mut further = Vec::with_capacity(family.len());
    for (position, read) in family.into_iter().enumerate() {
        let family_member = read.member.map(|member| member.into_family_member());
        let plot = finish_numeric_dataset(format, read.dataset)
            .map_err(|error| format!("member {} of the bundle: {error}", position + 2))?;
        further.push(
            crate::workbench::documents::result_document::import::ResultImportPlot {
                name: title(family_member.as_ref(), plot.analysis_type),
                family_member,
                analysis_type: plot.analysis_type,
                coordinate_name: plot.coordinate_name,
                coordinate_unit: plot.coordinate_unit,
                sample_count: plot.sample_count,
                waveforms: plot.waveforms,
                event_payload: None,
            },
        );
    }
    if member.is_some() || several {
        parsed.plots = Some(Arc::new(ResultImportPlots {
            reviewed: title(member.as_ref(), parsed.analysis_type),
            reviewed_member: member,
            further,
        }));
    }
    Ok(parsed)
}

// -------------------------------------------------------------------------
// HDF5 and MATLAB 7.3

pub(super) fn parse_hdf5(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let _source = super::ReadingSource::of(bytes.len());
    let decoded = rspice_formats::hdf5::decode_hdf5(bytes, hdf5_limits(), format.label())
        .map_err(|error| error.to_string())?;
    finish_numeric_dataset(format, decoded)
}

pub(super) fn parse_matlab_v73(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let _source = super::ReadingSource::of(bytes.len());
    let decoded = rspice_formats::hdf5::decode_matlab_v73(bytes, hdf5_limits(), format.label())
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
        rspice_formats::columnar::decode_arrow_ipc(bytes, columnar_limits(), format.label())
            .map_err(|error| error.to_string())?;
    finish_numeric_dataset(format, table)
}

pub(super) fn parse_parquet(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let _source = super::ReadingSource::of(bytes.len());
    let table = rspice_formats::columnar::decode_parquet(bytes, columnar_limits(), format.label())
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
    let array = rspice_formats::numpy::reader::decode_npy(bytes, source_capacity(), format.label())
        .map_err(|error| error.to_string())?;
    let decoded = rspice_formats::numpy::reader::npy_matrix_to_dataset(
        array,
        source_capacity(),
        source_capacity(),
        format.label(),
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
        format.label(),
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
    use rspice_formats::matlab::reader::{decode_matlab_v4, decode_matlab_v5_workspace};
    let decode = if format == ResultImportFormat::MatlabV4 {
        decode_matlab_v4
    } else {
        decode_matlab_v5_workspace
    };
    let decoded = decode(
        bytes,
        rspice_formats::matlab::reader::MatlabReadLimits {
            max_variables: source_capacity(),
            max_expanded_bytes: archive_expanded_bytes(),
            min_rows: MIN_RESULT_ROWS,
            coordinate_names: &RESULT_COORDINATE_NAMES,
        },
        format.label(),
    )
    .map_err(|error| error.to_string())?;
    let mut parsed = finish_numeric_dataset(format, decoded.dataset)?;
    parsed.notes = decoded.notes;
    Ok(parsed)
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
    let decoded =
        rspice_formats::spice_raw::decode_spice_raw_plots_with_log(bytes, log, raw_limits())
            .map_err(|error| adapter_error(format, error))?;
    let runs = decoded.iter().filter(|plot| plot.step.is_some()).count();
    let mut parsed = assemble_plots(format, decoded)?;
    if runs > 0 {
        match log {
            Some(log) => {
                let named = rspice_core::io::ltspice_raw::ltspice_log_step_values(log).len();
                if named != runs {
                    parsed.notes.push(format!(
                        "The log beside the rawfile names {named} {} but the rawfile holds \
                         {runs}, so the runs' step values could not be named and the runs are \
                         numbered instead.",
                        if named == 1 { "run" } else { "runs" }
                    ));
                }
            }
            None => parsed.notes.push(UNNAMED_RUNS_NOTE.to_owned()),
        }
    }
    Ok(parsed)
}

fn raw_limits() -> rspice_core::ResourceLimits {
    let mut limits = rspice_core::ResourceLimits::default();
    limits.max_external_data_bytes = usize::MAX;
    limits.max_external_data_values = source_capacity();
    limits.max_result_values = source_capacity();
    limits
}

// -------------------------------------------------------------------------
// QSPICE .qraw

pub(super) fn parse_qspice_raw(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let _source = super::ReadingSource::of(bytes.len());
    let decoded = rspice_formats::spice_raw::qspice::decode_qspice_raw_plots(bytes, raw_limits())
        .map_err(|error| adapter_error(format, error))?;
    assemble_plots(format, decoded)
}

// -------------------------------------------------------------------------
// HSPICE

pub(super) fn parse_hspice(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let _source = super::ReadingSource::of(bytes.len());
    let decoded = rspice_formats::hspice::decode_hspice(bytes)
        .map_err(|error| adapter_error(format, error))?;
    let swept = decoded.tables.len() > 1 || decoded.tables[0].sweep.is_some();
    let notes = decoded.notes;
    let plots = decoded
        .tables
        .into_iter()
        .enumerate()
        .map(|(index, table)| rspice_formats::spice_raw::DecodedRawPlot {
            plot_name: match table.dataset.domain {
                rspice_formats::WaveformDomain::Transient => "Transient Analysis",
                rspice_formats::WaveformDomain::Ac => "AC Analysis",
                rspice_formats::WaveformDomain::DcSweep => "DC Sweep",
            }
            .to_owned(),
            scalar: false,
            dataset: table.dataset,
            events: None,
            step: swept.then(|| rspice_formats::spice_raw::RawStep {
                index,
                coordinate: table.sweep.into_iter().collect(),
            }),
        })
        .collect();
    let mut parsed = assemble_plots(format, plots)?;
    parsed.notes.extend(notes);
    Ok(parsed)
}

/// A measurement table, one value per measurement and row: a scalar result over its rows, as
/// a rawfile's operating point is.
pub(super) fn parse_hspice_measure(
    bytes: &[u8],
    format: ResultImportFormat,
) -> Result<ParsedResultDataset, String> {
    let _source = super::ReadingSource::of(bytes.len());
    let decoded = rspice_formats::hspice::decode_hspice_measure(bytes)
        .map_err(|error| adapter_error(format, error))?;
    finish_numeric_dataset(format, decoded.dataset)
}

/// The plots a simulator's file holds, as one import: the reader reviews the first one drawn
/// over a coordinate (an operating point is a scalar result, not the waveform the file was
/// written for) and the rest come in beside it. Each run of a stepped or swept plot is its own
/// analysis, titled by its run, and the runs of the file draw as one family.
fn assemble_plots(
    format: ResultImportFormat,
    decoded: Vec<rspice_formats::spice_raw::DecodedRawPlot>,
) -> Result<ParsedResultDataset, String> {
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
        let family_member = plot.step.map(|step| {
            rspice_results::provenance::AnalysisResultFamilyMember::StepPoint {
                index: step.index,
                coordinate: step
                    .coordinate
                    .into_iter()
                    .map(|(name, value)| {
                        let unit = imported_step_unit(&name);
                        rspice_results::provenance::AnalysisResultStepCoordinate {
                            name,
                            value,
                            unit,
                        }
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
        reviewed_parsed.ok_or_else(|| adapter_error(format, "the file holds no variables"))?;
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
        .zip(decoded.signal_imag)
        .map(|((name, real), imag)| ImportedSignal {
            name,
            real,
            imag,
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

/// The unit an imported `.step` coordinate is measured in. A file names a swept
/// parameter and a swept source alike, so only the temperature states its unit.
pub(crate) fn imported_step_unit(name: &str) -> String {
    if name.eq_ignore_ascii_case("temp") {
        "°C".to_owned()
    } else {
        String::new()
    }
}

#[cfg(test)]
#[path = "result_import_adapters/tests.rs"]
mod tests;

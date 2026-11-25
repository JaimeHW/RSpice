//! SPICE rawfile publication: the displayed analysis as one rawfile plot.
//!
//! The bytes are [`rspice_formats::spice_raw::result`]'s, which writes through
//! the core RAW writer the CLI's `--format raw` uses. A file published here
//! therefore loads in ngspice and LTspice as an ordinary rawfile, and reopens
//! in this product with its units and exact names.
//!
//! # What a rawfile carries
//!
//! One analysis: a transient, an AC sweep as a complex plot of its retained
//! rectangular samples, or a DC sweep. The traces are the ones the export
//! publishes, and a transient's digital and real lanes — the plotted ones, or
//! every one when the export writes all signals — follow the analysis plot as
//! the engine's own per-node event plots, so a mixed run's logic travels
//! exactly, strength and all, where a VCD keeps four bit states.
//!
//! # Written as it is read
//!
//! Where the file is one this process writes, the rawfile is written on a
//! Results job as its samples are read, a block of each column at a time, so
//! a run kept in a results store exports whatever its length. A browser
//! download takes the encoded bytes.

use super::job::{ResultExportJob, start_export_job};
use super::{
    ALL_TRACES_HIDDEN_MESSAGE, ExportedSignals, NO_ACTIVE_ANALYSIS_MESSAGE, NO_SAMPLES_MESSAGE,
    event_nodes, exported_waveforms, note_result_export_success,
};
use crate::workbench::documents::result_document::view_context::ResolvedResultView;
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};
use crate::workbench::workflows::result_import_workflow::exported_waveform_limits;

use rspice_core::io::RawFormat;
use rspice_formats::spice_raw::result::{
    RawPlotFacts, check_result_raw, encode_result_raw, write_result_raw,
};

const EXTENSION: &str = "raw";

/// No media type is registered for a SPICE rawfile. Binary values are bytes;
/// ASCII values are text, and claiming more than that would be an invention.
const fn mime_type(format: RawFormat) -> &'static str {
    match format {
        RawFormat::Binary => "application/octet-stream",
        RawFormat::Ascii => "text/plain;charset=utf-8",
    }
}

const fn label(format: RawFormat) -> &'static str {
    match format {
        RawFormat::Binary => "SPICE raw",
        RawFormat::Ascii => "SPICE raw (ASCII)",
    }
}

/// Whether the event lane of `node` is on the plot: its `D()` or `E()`
/// column's visibility, the reader's override winning, and a node the run
/// keeps only as event history off the plot until the reader plots it — the
/// rule the strip draws its lanes by.
pub(super) fn plotted_event_lane(
    state: ResultsReads<'_>,
    dataset_id: crate::product::DatasetId,
    analysis: &crate::state::AnalysisResult,
    node: &str,
) -> bool {
    use crate::workbench::documents::result_document::AnalysisPresentationKey;
    use rspice_results::result_presentation::SourceWaveformPresentationKey;

    let analysis_key = AnalysisPresentationKey::new(dataset_id, analysis);
    ["D", "E"].into_iter().any(|prefix| {
        let column = format!("{prefix}({node})");
        let retained = analysis
            .waveforms
            .iter()
            .find(|waveform| waveform.name == column)
            .is_some_and(|waveform| waveform.visible);
        state.results.session.waveform_visibility(
            &SourceWaveformPresentationKey::new(analysis_key, &column),
            retained,
        )
    })
}

pub(super) fn export_raw(
    state: &mut ResultsParts<'_>,
    io: &(impl ExportWorkflowIo + ?Sized),
    displayed: &ResolvedResultView,
    format: RawFormat,
    signals: ExportedSignals,
) -> Option<ResultExportJob> {
    if io.writes_as_it_reads() {
        return export_raw_as_read(state, io, displayed, format, signals);
    }
    let reads = state.results_reads();
    let prepared = match displayed.primary_analysis(reads) {
        Some(analysis) => {
            let waveforms = exported_waveforms(reads, displayed.dataset_id, analysis, signals);
            if waveforms.is_empty() {
                Err(if analysis.waveforms.is_empty() {
                    NO_SAMPLES_MESSAGE.to_owned()
                } else {
                    ALL_TRACES_HIDDEN_MESSAGE.to_owned()
                })
            } else {
                let lanes = |node: &str| {
                    signals == ExportedSignals::All
                        || plotted_event_lane(reads, displayed.dataset_id, analysis, node)
                };
                encode_result_raw(
                    analysis,
                    &waveforms,
                    &lanes,
                    format,
                    exported_waveform_limits(),
                )
                .map_err(|error| error.to_string())
            }
        }
        None => Err(NO_ACTIVE_ANALYSIS_MESSAGE.to_owned()),
    };
    let prepared = match prepared {
        Ok(prepared) => prepared,
        Err(message) => {
            state.prompt(message);
            return None;
        }
    };

    let label = label(format);
    let default_name = format!("waveforms.{EXTENSION}");
    let (published_path, export) = match io.show_save_dialog(SaveDialogConfig {
        title: "Export SPICE Rawfile",
        default_name: &default_name,
        filter_name: "SPICE Rawfile",
        filter_extensions: &[EXTENSION],
    }) {
        Ok(Some(mut path)) => {
            crate::io::file_exchange::ensure_file_extension(&mut path, EXTENSION);
            let export = io.observe_destination(&path).and_then(|destination| {
                io.write_bytes_file_observed(&destination, &prepared.bytes, mime_type(format))
            });
            (path, export)
        }
        Ok(None) => return None,
        Err(error) => (std::path::PathBuf::from(default_name), Err(error)),
    };
    match export {
        Ok(()) => note_result_export_success(
            state,
            crate::workbench::workflows::export_workflow::export_completion_message(
                label,
                &published_path,
                Some(completion_detail(&prepared.facts)),
                io,
            ),
        ),
        Err(error) => {
            state.report_failure(format!("{label} export failed"), error.to_string());
        }
    }
    None
}

/// The rawfile written on a job as its samples are read: refused before the
/// picker opens when it cannot be written, and started once the reader picks
/// where it goes.
fn export_raw_as_read(
    state: &mut ResultsParts<'_>,
    io: &(impl ExportWorkflowIo + ?Sized),
    displayed: &ResolvedResultView,
    format: RawFormat,
    signals: ExportedSignals,
) -> Option<ResultExportJob> {
    let reads = state.results_reads();
    let Some(analysis) = displayed.primary_analysis(reads) else {
        state.prompt(NO_ACTIVE_ANALYSIS_MESSAGE);
        return None;
    };
    let waveforms = exported_waveforms(reads, displayed.dataset_id, analysis, signals);
    let refusal = if waveforms.is_empty() {
        Some(if analysis.waveforms.is_empty() {
            NO_SAMPLES_MESSAGE.to_owned()
        } else {
            ALL_TRACES_HIDDEN_MESSAGE.to_owned()
        })
    } else {
        check_result_raw(analysis, &waveforms, exported_waveform_limits())
            .err()
            .map(|error| error.to_string())
    };
    if let Some(refusal) = refusal {
        state.prompt(refusal);
        return None;
    }
    // What the job writes is fixed now: the analysis, the signals by their
    // place in it, and the event lanes that ride along.
    let chosen = analysis
        .waveforms
        .iter()
        .enumerate()
        .filter(|(_, waveform)| {
            waveforms
                .iter()
                .any(|chosen| std::ptr::eq(*chosen, *waveform))
        })
        .map(|(index, _)| index)
        .collect::<Vec<_>>();
    let lanes = event_nodes(analysis)
        .into_iter()
        .filter(|node| {
            signals == ExportedSignals::All
                || plotted_event_lane(reads, displayed.dataset_id, analysis, node)
        })
        .map(str::to_owned)
        .collect::<std::collections::BTreeSet<_>>();
    let bytes = estimated_bytes(&waveforms, format);
    let analysis = analysis.clone();

    let label = label(format);
    let default_name = format!("waveforms.{EXTENSION}");
    let destination = match io.show_save_dialog(SaveDialogConfig {
        title: "Export SPICE Rawfile",
        default_name: &default_name,
        filter_name: "SPICE Rawfile",
        filter_extensions: &[EXTENSION],
    }) {
        Ok(Some(mut path)) => {
            crate::io::file_exchange::ensure_file_extension(&mut path, EXTENSION);
            io.observe_destination(&path)
        }
        Ok(None) => return None,
        Err(error) => Err(error),
    };
    let destination = match destination {
        Ok(destination) => destination,
        Err(error) => {
            state.report_failure(format!("{label} export failed"), error);
            return None;
        }
    };
    Some(start_export_job(label, destination, bytes, move |out| {
        let waveforms = chosen
            .iter()
            .map(|&index| &analysis.waveforms[index])
            .collect::<Vec<_>>();
        write_result_raw(
            &analysis,
            &waveforms,
            &|node| lanes.contains(node),
            format,
            exported_waveform_limits(),
            out,
        )
        .map(|facts| completion_detail(&facts))
        .map_err(|error| error.to_string())
    }))
}

/// About how many bytes the analysis plot of `waveforms` takes, for the
/// progress of its export: a binary value is eight bytes, an ASCII one about
/// twenty-five characters.
fn estimated_bytes(waveforms: &[&crate::state::RetainedTrace], format: RawFormat) -> u64 {
    let rows = waveforms.first().map_or(0, |waveform| waveform.x.len()) as u64;
    let values = waveforms.iter().fold(1_u64, |values, waveform| {
        values + if waveform.complex.is_some() { 2 } else { 1 }
    });
    let value_bytes = match format {
        RawFormat::Binary => 8,
        RawFormat::Ascii => 25,
    };
    rows.saturating_mul(values).saturating_mul(value_bytes)
}

/// What the file holds, in the terms a rawfile reader lists it by.
fn completion_detail(export: &RawPlotFacts) -> String {
    let mut detail = format!(
        "plot '{}': {} points x {} variables on '{}'{}",
        export.plot_name,
        export.points,
        export.variables,
        export.scale_name,
        if export.complex { ", complex" } else { "" }
    );
    if export.event_plots > 0 {
        detail.push_str(&format!(
            "; {} event node plot{} after it",
            export.event_plots,
            if export.event_plots == 1 { "" } else { "s" }
        ));
        if export.bus_plots > 0 {
            detail.push_str(&format!(
                " and {} bus plot{}",
                export.bus_plots,
                if export.bus_plots == 1 { "" } else { "s" }
            ));
        }
    }
    detail
}

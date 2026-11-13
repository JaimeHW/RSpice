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
//! publishes, and a transient's digital and real lanes follow the analysis
//! plot as the engine's own per-node event plots, so a mixed run's logic
//! travels exactly, strength and all, where a VCD keeps four bit states.

use super::{
    ALL_TRACES_HIDDEN_MESSAGE, NO_ACTIVE_ANALYSIS_MESSAGE, NO_SAMPLES_MESSAGE, exported_waveforms,
    note_result_export_failure, note_result_export_success,
};
use crate::workbench::documents::result_document::view_context::ResolvedResultView;
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};
use crate::workbench::workflows::result_import_workflow::waveform_import_limits;

use rspice_core::io::RawFormat;
use rspice_formats::spice_raw::result::{RawExport, encode_result_raw};

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
) {
    let reads = state.results_reads();
    let prepared = match displayed.primary_analysis(reads) {
        Some(analysis) => {
            let waveforms = exported_waveforms(reads, displayed.dataset_id, analysis);
            if waveforms.is_empty() {
                Err(if analysis.waveforms.is_empty() {
                    NO_SAMPLES_MESSAGE.to_owned()
                } else {
                    ALL_TRACES_HIDDEN_MESSAGE.to_owned()
                })
            } else {
                let lanes =
                    |node: &str| plotted_event_lane(reads, displayed.dataset_id, analysis, node);
                encode_result_raw(
                    analysis,
                    &waveforms,
                    &lanes,
                    format,
                    waveform_import_limits(),
                )
                .map_err(|error| error.to_string())
                .and_then(|mut export| {
                    export.bytes = super::admit_result_export_bytes(export.bytes)?;
                    Ok(export)
                })
            }
        }
        None => Err(NO_ACTIVE_ANALYSIS_MESSAGE.to_owned()),
    };
    let prepared = match prepared {
        Ok(prepared) => prepared,
        Err(message) => {
            state.prompt(message);
            return;
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
        Ok(None) => return,
        Err(error) => (std::path::PathBuf::from(default_name), Err(error)),
    };
    match export {
        Ok(()) => note_result_export_success(
            state,
            label,
            crate::workbench::workflows::export_workflow::export_completion_message(
                label,
                &published_path,
                Some(completion_detail(&prepared)),
                io,
            ),
        ),
        Err(error) => {
            note_result_export_failure(state, format!("{label} export failed: {error}"));
            state.report_failure(format!("{label} export failed"), error.to_string());
        }
    }
}

/// What the file holds, in the terms a rawfile reader lists it by.
fn completion_detail(export: &RawExport) -> String {
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

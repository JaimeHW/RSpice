//! The displayed analysis's waveform table as CSV or TSV, written on a job as its samples are
//! read.
//!
//! The bytes are [`rspice_formats::waveform_io::result::write_delimited_waveforms`]', the table
//! the dataset writer makes of the same traces, byte for byte, written a chunk of rows at a time,
//! so a run kept in a results store exports whatever its length and no column is read whole. A
//! browser download takes the table the dataset writer makes.

use super::job::{ResultExportJob, start_export_job};
use super::{
    ALL_TRACES_HIDDEN_MESSAGE, ExportedSignals, NO_ACTIVE_ANALYSIS_MESSAGE, NO_SAMPLES_MESSAGE,
    TabularExportFormat, exported_waveforms, failed_run_refusal,
};
use crate::workbench::documents::result_document::ResultsParts;
use crate::workbench::documents::result_document::view_context::ResolvedResultView;
use crate::workbench::workflows::export_workflow::{ExportWorkflowIo, SaveDialogConfig};

use rspice_formats::waveform_io::result::{
    DelimitedExportError, check_delimited_waveforms, write_delimited_waveforms,
};

/// Start writing the displayed analysis's table as `format` says, once the reader picks where
/// it goes: refused, as the dataset export refuses it, before the picker opens when it cannot
/// be written.
pub(super) fn export_delimited_as_read(
    state: &mut ResultsParts<'_>,
    io: &(impl ExportWorkflowIo + ?Sized),
    displayed: &ResolvedResultView,
    format: TabularExportFormat,
    signals: ExportedSignals,
) -> Option<ResultExportJob> {
    let (label, extension, title, filter, delimiter) = match format {
        TabularExportFormat::Csv => ("CSV", "csv", "Export Waveform CSV", "CSV Files", ','),
        TabularExportFormat::Tsv => ("TSV", "tsv", "Export Waveform TSV", "TSV Files", '\t'),
        TabularExportFormat::TouchstoneWhereCompatible => {
            unreachable!("Touchstone is written from its dataset")
        }
    };
    let reads = state.results_reads();
    if displayed.analysis_indices.is_empty() {
        let refusal = displayed
            .run(reads)
            .and_then(failed_run_refusal)
            .unwrap_or_else(|| NO_SAMPLES_MESSAGE.to_owned());
        state.prompt(refusal);
        return None;
    }
    let Some(analysis) = displayed.primary_analysis(reads) else {
        state.prompt(NO_ACTIVE_ANALYSIS_MESSAGE);
        return None;
    };
    let waveforms = exported_waveforms(reads, displayed.dataset_id, analysis, signals);
    if waveforms.is_empty() && !analysis.waveforms.is_empty() {
        state.prompt(ALL_TRACES_HIDDEN_MESSAGE);
        return None;
    }
    match check_delimited_waveforms(analysis, &waveforms) {
        Ok(()) => {}
        Err(DelimitedExportError::Projection(error)) => {
            state.prompt(error.to_string());
            return None;
        }
        Err(error) => {
            state.report_failure(format!("{label} export failed"), error.to_string());
            return None;
        }
    }
    // What the job writes is fixed now: the analysis and the signals by their place in it.
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
    let rows = waveforms.first().map_or(0, |waveform| waveform.x.len()) as u64;
    let values = waveforms.iter().fold(1_u64, |values, waveform| {
        values + if waveform.complex.is_some() { 2 } else { 1 }
    });
    let samples = rows.saturating_mul(values);
    let analysis = analysis.clone();

    let default_name = format!("waveforms.{extension}");
    let destination = match io.show_save_dialog(SaveDialogConfig {
        title,
        default_name: &default_name,
        filter_name: filter,
        filter_extensions: &[extension],
    }) {
        Ok(Some(mut path)) => {
            crate::io::file_exchange::ensure_file_extension(&mut path, extension);
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
    Some(start_export_job(label, destination, samples, move |out| {
        let waveforms = chosen
            .iter()
            .map(|&index| &analysis.waveforms[index])
            .collect::<Vec<_>>();
        write_delimited_waveforms(&analysis, &waveforms, delimiter, out)
            .map(|table| format!("{} signals, {} points", table.signals, table.points))
            .map_err(|error| error.to_string())
    }))
}

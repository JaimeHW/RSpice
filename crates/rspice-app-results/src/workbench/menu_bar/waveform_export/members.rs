//! The analyses an export writes, fixed for the job that writes them, and where it writes them.
//!
//! An export of the displayed result writes the one analysis the form chose, or every displayed
//! one: each member of a `.step` sweep or Monte Carlo study, in strip order. What it writes is
//! decided on the frame the reader chooses Export, checked there without reading a sample, and
//! handed to the job as each analysis and its traces by their place in it.

use std::io::Write;

use super::job::{ResultExportJob, start_export_job};
use super::{
    ALL_TRACES_HIDDEN_MESSAGE, ExportedSignals, NO_ACTIVE_ANALYSIS_MESSAGE, NO_SAMPLES_MESSAGE,
    exported_waveforms, note_result_export_success,
};
use crate::state::{AnalysisResult, RetainedTrace};
use crate::workbench::documents::result_document::view_context::ResolvedResultView;
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};
use crate::workbench::workflows::export_workflow::{
    ExportWorkflowIo, ObservedExportDestination, SaveDialogConfig, export_completion_message,
};
use rspice_formats::waveform_io::result::ExportMember;

/// One analysis an export writes and the traces of it the export writes.
pub(super) type Member<'a> = (&'a AnalysisResult, Vec<&'a RetainedTrace>);

/// The displayed analyses the export writes, each with its exported traces: an analysis with
/// none to write refuses the export, saying why.
pub(super) fn exported_members<'a>(
    reads: ResultsReads<'a>,
    displayed: &'a ResolvedResultView,
    signals: ExportedSignals,
) -> Result<Vec<Member<'a>>, String> {
    let mut members = Vec::with_capacity(displayed.analysis_indices.len());
    for analysis in displayed.analyses(reads) {
        let waveforms = exported_waveforms(reads, displayed.dataset_id, analysis, signals);
        if waveforms.is_empty() {
            return Err(if analysis.waveforms.is_empty() {
                NO_SAMPLES_MESSAGE.to_owned()
            } else {
                ALL_TRACES_HIDDEN_MESSAGE.to_owned()
            });
        }
        members.push((analysis, waveforms));
    }
    if members.is_empty() {
        return Err(NO_ACTIVE_ANALYSIS_MESSAGE.to_owned());
    }
    Ok(members)
}

/// The members as the writers take them.
pub(super) fn export_members<'a>(members: &'a [Member<'a>]) -> Vec<ExportMember<'a, RetainedTrace>> {
    members
        .iter()
        .map(|(analysis, waveforms)| ExportMember::new(analysis, waveforms.clone()))
        .collect()
}

/// What an export writes, fixed for its job: each analysis, and its traces by their place in it.
#[derive(Debug, Clone)]
pub(super) struct OwnedMembers(Vec<(AnalysisResult, Vec<usize>)>);

impl OwnedMembers {
    pub(super) fn of(members: &[Member<'_>]) -> Self {
        Self(
            members
                .iter()
                .map(|(analysis, waveforms)| {
                    let chosen = analysis
                        .waveforms
                        .iter()
                        .enumerate()
                        .filter(|(_, waveform)| {
                            waveforms.iter().any(|chosen| std::ptr::eq(*chosen, *waveform))
                        })
                        .map(|(index, _)| index)
                        .collect();
                    ((*analysis).clone(), chosen)
                })
                .collect(),
        )
    }

    /// The members as the writers take them.
    pub(super) fn members(&self) -> Vec<ExportMember<'_, RetainedTrace>> {
        self.0
            .iter()
            .map(|(analysis, chosen)| {
                ExportMember::new(
                    analysis,
                    chosen.iter().map(|&index| &analysis.waveforms[index]).collect(),
                )
            })
            .collect()
    }
}

/// About how many bytes the members' samples take at `value_bytes` a sample, for an export's
/// progress: the coordinate and each trace, a complex one twice.
pub(super) fn estimated_bytes(members: &[Member<'_>], value_bytes: u64) -> u64 {
    members
        .iter()
        .map(|(_, waveforms)| {
            let rows = waveforms.first().map_or(0, |waveform| waveform.x.len()) as u64;
            let values = waveforms.iter().fold(1_u64, |values, waveform| {
                values + if waveform.complex.is_some() { 2 } else { 1 }
            });
            rows.saturating_mul(values).saturating_mul(value_bytes)
        })
        .fold(0_u64, u64::saturating_add)
}

/// What the save picker asks for an export's file, and what the file is.
pub(super) struct ExportFile {
    /// The format, as the completion line and a failure name it.
    pub label: &'static str,
    pub title: &'static str,
    pub default_name: String,
    pub filter_name: &'static str,
    /// The extension [`publish_export`]'s picker offers and the chosen path is given.
    pub extension: &'static str,
    pub mime_type: &'static str,
    /// The file is text: a browser download takes it as such.
    pub text: bool,
}

/// Ask where `file` goes, then write it with `write`, which returns what it holds in the words
/// the completion line quotes: on a Results job as its samples are read where this process
/// writes the file, the job returned; into the bytes a browser downloads otherwise.
pub(super) fn publish_export(
    state: &mut ResultsParts<'_>,
    io: &(impl ExportWorkflowIo + ?Sized),
    file: ExportFile,
    bytes: u64,
    write: impl FnOnce(&mut dyn Write) -> Result<String, String> + Send + 'static,
) -> Option<ResultExportJob> {
    let destination = choose_destination(
        state,
        io,
        &file,
        &[file.extension],
        |path: &mut std::path::PathBuf| {
            crate::io::file_exchange::ensure_file_extension(path, file.extension);
        },
    )?;
    write_export(state, io, &file, destination, bytes, write)
}

/// Ask where `file` goes, offering `extensions`, with `settle` giving the chosen path the name
/// the file is written under. `None` when the reader cancels, or the destination cannot be
/// written, which is said.
pub(super) fn choose_destination(
    state: &mut ResultsParts<'_>,
    io: &(impl ExportWorkflowIo + ?Sized),
    file: &ExportFile,
    extensions: &[&str],
    settle: impl FnOnce(&mut std::path::PathBuf),
) -> Option<ObservedExportDestination> {
    let destination = match io.show_save_dialog(SaveDialogConfig {
        title: file.title,
        default_name: &file.default_name,
        filter_name: file.filter_name,
        filter_extensions: extensions,
    }) {
        Ok(Some(mut path)) => {
            settle(&mut path);
            io.observe_destination(&path)
        }
        Ok(None) => return None,
        Err(error) => Err(error),
    };
    destination
        .map_err(|error| state.report_failure(format!("{} export failed", file.label), error))
        .ok()
}

/// Write `file` to `destination` with `write`, as [`publish_export`] does once it is chosen.
pub(super) fn write_export(
    state: &mut ResultsParts<'_>,
    io: &(impl ExportWorkflowIo + ?Sized),
    file: &ExportFile,
    destination: ObservedExportDestination,
    bytes: u64,
    write: impl FnOnce(&mut dyn Write) -> Result<String, String> + Send + 'static,
) -> Option<ResultExportJob> {
    let failed = format!("{} export failed", file.label);
    if io.writes_as_it_reads() {
        return Some(start_export_job(file.label, destination, bytes, write));
    }
    let mut written = Vec::new();
    let published = write(&mut written).and_then(|detail| {
        if file.text {
            let text = String::from_utf8(written).map_err(|error| error.to_string())?;
            io.write_text_file_observed(&destination, &text)
        } else {
            io.write_bytes_file_observed(&destination, &written, file.mime_type)
        }
        .map(|()| detail)
    });
    match published {
        Ok(detail) => note_result_export_success(
            state,
            export_completion_message(file.label, destination.path(), Some(detail), io),
        ),
        Err(error) => state.report_failure(failed, error),
    }
    None
}

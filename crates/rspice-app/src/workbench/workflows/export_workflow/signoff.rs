//! Exporting the sign-off report: the summary where the reader puts it, and
//! the CSV of every finding beside it under the same name.

use crate::diagnostics::{LogSeverity, LogSource};
use crate::workbench::app_state::AppState;
use crate::workbench::publication_snapshot::publication_timestamp_utc;

use super::{
    ExportWorkflowIo, ObservedExportDestination, SaveDialogConfig, export_completion_message,
};

/// Export the sign-off report, and say in the Console how it went.
pub(crate) fn export_signoff_report(state: &mut AppState, io: &(impl ExportWorkflowIo + ?Sized)) {
    match write_signoff_report(state, io) {
        Ok(Some(message)) => {
            state.record(LogSeverity::Info, LogSource::Studio, message);
        }
        Ok(None) => {}
        Err(error) => {
            state.record(
                LogSeverity::Error,
                LogSource::Studio,
                format!("Sign-off report export failed: {error}"),
            );
        }
    }
}

/// Check Design runs first wherever no current check describes the design,
/// so a design that cannot be checked says so before a file is asked for.
/// The report is built again once the file is chosen, from a check of the
/// design as it then stands: a design, a deck or a waiver that moved while
/// the dialog was open is reported as it is, never as it was.
///
/// `None` where the reader cancelled, or declined to replace a file the
/// export writes beside the one they chose.
fn write_signoff_report(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
) -> Result<Option<String>, String> {
    const TITLE: &str = "Export sign-off report";
    state.signoff_report(&publication_timestamp_utc())?;
    let root = state.workspace.content.simulation_root_reference();
    let default_name = format!("{}_signoff.summary", root.cell);
    let Some(chosen) = io.show_save_dialog(SaveDialogConfig {
        title: TITLE,
        default_name: &default_name,
        filter_name: "Sign-off summary",
        filter_extensions: &["summary"],
    })?
    else {
        return Ok(None);
    };
    // The detail is written beside the summary as `.csv`, so a summary named
    // as a CSV would be overwritten by it.
    let mut path = chosen.clone();
    if path
        .extension()
        .is_none_or(|extension| extension.eq_ignore_ascii_case("csv"))
    {
        path.set_extension("summary");
    }
    let report = state.signoff_report(&publication_timestamp_utc())?;
    let detail_path = path.with_extension("csv");
    let summary = io.observe_destination(&path)?;
    let detail = io.observe_destination(&detail_path)?;
    // The save dialog asked about the file it returned and nothing else. The
    // detail beside it, or a summary renamed from the name chosen, would
    // replace a file of that name unasked, so the reader is asked once,
    // before either is written. Each write still refuses a file that moved
    // after it was observed.
    let unasked = [&summary, &detail]
        .into_iter()
        .filter(|destination| destination.path() != chosen && destination.is_occupied())
        .map(ObservedExportDestination::path)
        .collect::<Vec<_>>();
    if !unasked.is_empty() && !io.confirm_replace(TITLE, &unasked) {
        return Ok(None);
    }
    io.write_text_file_observed(&detail, &report.detail)?;
    io.write_text_file_observed(&summary, &report.summary)?;
    Ok(Some(export_completion_message(
        "Sign-off report",
        &path,
        Some(format!(
            "{}; findings in {}",
            if report.passed { "PASS" } else { "FAIL" },
            detail_path.display()
        )),
        io,
    )))
}

#[cfg(test)]
mod tests;

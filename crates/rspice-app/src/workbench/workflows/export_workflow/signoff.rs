//! Exporting the sign-off report: the summary where the reader puts it, and
//! the CSV of every finding beside it under the same name.

use crate::diagnostics::ConsoleMessage;
use crate::workbench::app_state::AppState;
use crate::workbench::publication_snapshot::publication_timestamp_utc;

use super::{ExportWorkflowIo, SaveDialogConfig, export_completion_message};

/// Export the sign-off report, and say in the Console how it went.
pub(crate) fn export_signoff_report(state: &mut AppState, io: &(impl ExportWorkflowIo + ?Sized)) {
    match write_signoff_report(state, io) {
        Ok(Some(message)) => state.push_user_message(ConsoleMessage::info(message)),
        Ok(None) => {}
        Err(error) => state.push_user_message(ConsoleMessage::error(format!(
            "Sign-off report export failed: {error}"
        ))),
    }
}

/// Check Design runs first wherever no current check describes the design,
/// so a design that cannot be checked says so before a file is asked for.
/// The report is built again once the file is chosen, from a check of the
/// design as it then stands: a design, a deck or a waiver that moved while
/// the dialog was open is reported as it is, never as it was.
///
/// `None` where the reader cancelled.
fn write_signoff_report(
    state: &mut AppState,
    io: &(impl ExportWorkflowIo + ?Sized),
) -> Result<Option<String>, String> {
    state.signoff_report(&publication_timestamp_utc())?;
    let root = state.workspace.content.simulation_root_reference();
    let default_name = format!("{}_signoff.summary", root.cell);
    let Some(mut path) = io.show_save_dialog(SaveDialogConfig {
        title: "Export sign-off report",
        default_name: &default_name,
        filter_name: "Sign-off summary",
        filter_extensions: &["summary"],
    })?
    else {
        return Ok(None);
    };
    // The detail is written beside the summary as `.csv`, so a summary named
    // as a CSV would be overwritten by it.
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

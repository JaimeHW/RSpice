//! An export written as it is read, on a Results job beside the window.
//!
//! The file is staged beside its destination and put in its place only when the last byte is
//! written, against the destination the reader accepted after the picker returned: Cancel or a
//! failure leaves the destination as it was and nothing beside it. The status bar shows how far
//! it got and cancels it.

use std::path::PathBuf;

use crate::workbench::documents::result_document::ResultsParts;
use crate::workbench::results_job::{JobContext, JobOutcome, JobState, JobStop, ResultsJob};
use crate::workbench::workflows::export_workflow::{
    NativeExportWorkflowIo, ObservedExportDestination, export_completion_message, publish_streamed,
};

use super::note_result_export_success;

/// Bytes written between two reports of an export's progress and two looks at its Cancel.
const CHECK_BYTES: u64 = 1 << 20;

/// An export being written, while it is written.
#[derive(Debug, Clone, PartialEq)]
pub struct ResultExportJob {
    job: ResultsJob<String>,
    label: &'static str,
    path: PathBuf,
}

impl ResultExportJob {
    /// How far the export got, while it runs.
    pub fn state(&self) -> JobState {
        self.job.state()
    }

    /// Stop the export: the destination stays as it was.
    pub fn cancel(&self) {
        self.job.cancel();
    }

    /// Wait up to `patience` for the export to end; whether it did.
    pub fn wait(&self, patience: std::time::Duration) -> bool {
        self.job.wait(patience)
    }
}

/// Start writing the export `label` at `destination` as `write` makes it. `write` writes the
/// file and returns what it holds, in the words the completion line quotes; `bytes` is about
/// how many it writes, for the progress the status bar shows.
pub(super) fn start_export_job(
    label: &'static str,
    destination: ObservedExportDestination,
    bytes: u64,
    write: impl FnOnce(&mut dyn std::io::Write) -> Result<String, String> + Send + 'static,
) -> ResultExportJob {
    let path = destination.path().to_path_buf();
    let job = ResultsJob::start("result-export", move |job| {
        job.step("Exporting", bytes);
        let mut detail = None;
        let mut failure = None;
        let published = publish_streamed(&destination, |out| {
            let mut out = JobWriter {
                out,
                job,
                written: 0,
                checked: 0,
                bytes,
            };
            match write(&mut out) {
                Ok(written) => {
                    detail = Some(written);
                    Ok(())
                }
                Err(reason) => {
                    failure = Some(reason);
                    Err(std::io::Error::other("the export stopped"))
                }
            }
        });
        match (published, detail) {
            (Ok(()), Some(detail)) => Ok(detail),
            _ if job.cancelled() => Err(JobStop::Cancelled),
            (Err(reason), _) => Err(JobStop::Failed(failure.unwrap_or(reason))),
            (Ok(()), None) => Err(JobStop::Failed("the export wrote nothing".to_owned())),
        }
    });
    ResultExportJob { job, label, path }
}

/// The end of `export`, if it ended: a written file is said, a failure reported, and a cancelled
/// export said to have written nothing. Returns the export while it still runs.
pub fn finish_export_job(
    state: &mut ResultsParts<'_>,
    export: ResultExportJob,
) -> Option<ResultExportJob> {
    if matches!(export.job.state(), JobState::Running { .. }) {
        return Some(export);
    }
    let label = export.label;
    match export.job.take_outcome() {
        Some(JobOutcome::Done(detail)) => note_result_export_success(
            state,
            export_completion_message(label, &export.path, Some(detail), &NativeExportWorkflowIo),
        ),
        Some(JobOutcome::Failed(reason)) => {
            state.report_failure(format!("{label} export failed"), reason);
        }
        Some(JobOutcome::Cancelled) | None => state.prompt(format!(
            "The {label} export was cancelled; {} was left as it was.",
            export.path.display()
        )),
    }
    None
}

/// The staged file, written through a job: progress reported and Cancel looked at every
/// [`CHECK_BYTES`].
struct JobWriter<'a> {
    out: &'a mut dyn std::io::Write,
    job: &'a JobContext,
    written: u64,
    checked: u64,
    /// About how many bytes the export writes: progress never reports more.
    bytes: u64,
}

impl std::io::Write for JobWriter<'_> {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        if self.written - self.checked >= CHECK_BYTES {
            self.checked = self.written;
            self.job.done(self.written.min(self.bytes));
            if self.job.cancelled() {
                return Err(std::io::Error::other("the export was cancelled"));
            }
        }
        let written = self.out.write(bytes)?;
        self.written += written as u64;
        Ok(written)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.out.flush()
    }
}

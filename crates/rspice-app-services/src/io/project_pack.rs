//! Pack project with results, and opening a pack, as Results jobs.
//!
//! A pack is the project file and its results folder as one archive to share
//! ([`rspice_formats::project_results::pack_project`]). Writing one, or opening one into a folder
//! of its own beside it, reads every byte of the results once, so each runs as a job with
//! progress and Cancel, and a cancelled one leaves nothing.

use std::path::{Path, PathBuf};

use rspice_formats::project_results::{folder_to_unpack, pack_project, unpack_project};

use crate::workbench::results_job::{JobContext, JobState, JobStop, ResultsJob};

pub use rspice_formats::project_results::PACK_EXTENSION;

/// Whether `path` names a pack.
pub fn is_pack(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| extension.eq_ignore_ascii_case(PACK_EXTENSION))
}

/// Writing a pack, or opening one.
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectPackJob {
    /// Writing the pack `archive`.
    Packing {
        job: ResultsJob<()>,
        archive: PathBuf,
    },
    /// Opening the pack `archive`; the job ends with the project file it unpacked.
    Opening {
        job: ResultsJob<PathBuf>,
        archive: PathBuf,
    },
}

impl ProjectPackJob {
    /// What a frame shows of the job.
    pub fn state(&self) -> JobState {
        match self {
            Self::Packing { job, .. } => job.state(),
            Self::Opening { job, .. } => job.state(),
        }
    }

    /// Stop the job; it leaves nothing.
    pub fn cancel(&self) {
        match self {
            Self::Packing { job, .. } => job.cancel(),
            Self::Opening { job, .. } => job.cancel(),
        }
    }

    /// Wait at most `patience` for the job to end. Returns whether it did.
    pub fn wait(&self, patience: std::time::Duration) -> bool {
        match self {
            Self::Packing { job, .. } => job.wait(patience),
            Self::Opening { job, .. } => job.wait(patience),
        }
    }
}

/// `progress` for a pack step of `job` named `label`: it states the step's bytes once they are
/// known, reports the bytes done, and stops when the job is cancelled.
fn reporting<'a>(job: &'a JobContext, label: &'a str) -> impl FnMut(u64, u64) -> bool + 'a {
    let mut known = None;
    move |done, total| {
        if known != Some(total) {
            job.step(label, total);
            known = Some(total);
        }
        job.done(done);
        !job.cancelled()
    }
}

/// Start writing the project file `project` and its results folder as the pack `archive`.
pub fn start_project_pack(project: PathBuf, archive: PathBuf) -> ProjectPackJob {
    let written = archive.clone();
    let job = ResultsJob::start("project-pack", move |job| {
        match pack_project(&project, &written, &mut reporting(job, "Packing project")) {
            Ok(Some(())) => Ok(()),
            Ok(None) => Err(JobStop::Cancelled),
            Err(reason) => Err(JobStop::Failed(reason)),
        }
    });
    ProjectPackJob::Packing { job, archive }
}

/// Start opening the pack `archive` into a folder of its own beside it.
pub fn start_pack_open(archive: PathBuf) -> ProjectPackJob {
    let read = archive.clone();
    let job = ResultsJob::start("pack-open", move |job| {
        let folder = folder_to_unpack(&read);
        match unpack_project(&read, &folder, &mut reporting(job, "Opening pack")) {
            Ok(Some(project)) => Ok(project),
            Ok(None) => Err(JobStop::Cancelled),
            Err(reason) => Err(JobStop::Failed(reason)),
        }
    });
    ProjectPackJob::Opening { job, archive }
}

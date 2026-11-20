//! Pack project with results, and opening a pack, as the File menu runs them.

use std::path::{Path, PathBuf};

use rspice_app_services::io::project_pack::{ProjectPackJob, start_pack_open, start_project_pack};
use rspice_app_services::workbench::results_job::{JobOutcome, JobState};

use crate::workbench::app_state::AppState;

impl AppState {
    /// Write the open project, as it is saved, and its results folder as one pack the reader
    /// picks, as a job the status bar shows.
    pub(crate) fn pack_project_with_results(&mut self) {
        if self.platform.project_pack.is_some() {
            self.prompt("A pack is being written or opened already.");
            return;
        }
        let Some(project) = self
            .platform
            .project_lifecycle
            .canonical_native_path()
            .map(Path::to_path_buf)
        else {
            self.prompt(
                "Save the project first: a pack holds the project file and the results beside it.",
            );
            return;
        };
        if rspice_app_services::workbench::lifecycle::project_lifecycle::has_unsaved_changes_fresh(
            self,
        ) {
            self.prompt("Save the project before packing it: a pack holds the project as saved.");
            return;
        }
        let Some(archive) = pick_pack_destination(&project) else {
            return;
        };
        self.start_project_pack(project, archive);
    }

    /// Start writing `project` and its results folder as the pack `archive`.
    pub(crate) fn start_project_pack(&mut self, project: PathBuf, archive: PathBuf) {
        self.platform.project_pack = Some(start_project_pack(project, archive));
    }

    /// Open the pack `archive`: unpacked into a folder of its own beside it, as a job, then
    /// opened as the project it holds. Returns whether the job started.
    pub(crate) fn open_pack(&mut self, archive: PathBuf) -> bool {
        if self.platform.project_pack.is_some() {
            self.prompt("A pack is being written or opened already.");
            return false;
        }
        self.platform.project_pack = Some(start_pack_open(archive));
        true
    }

    /// Stop the pack being written or opened; it leaves nothing.
    pub(crate) fn cancel_project_pack(&mut self) {
        if let Some(pack) = &self.platform.project_pack {
            pack.cancel();
        }
    }

    /// The pack job that ended since the last frame, if one did: a written pack is said, and an
    /// opened one opens as its project. While one runs, asks for the frames that show it.
    pub(crate) fn poll_project_pack(&mut self, ctx: &egui::Context) {
        let Some(pack) = &self.platform.project_pack else {
            return;
        };
        if matches!(pack.state(), JobState::Running { .. }) {
            ctx.request_repaint_after(std::time::Duration::from_millis(100));
            return;
        }
        match self.platform.project_pack.take() {
            Some(ProjectPackJob::Packing { job, archive }) => match job.take_outcome() {
                Some(JobOutcome::Done(())) => self.prompt(format!(
                    "Packed the project with its results as {}.",
                    archive.display()
                )),
                Some(JobOutcome::Failed(reason)) => self.report_failure("Pack failed", reason),
                Some(JobOutcome::Cancelled) | None => {
                    self.prompt("The pack was cancelled; nothing was written.");
                }
            },
            Some(ProjectPackJob::Opening { job, .. }) => match job.take_outcome() {
                Some(JobOutcome::Done(project)) => {
                    self.load_project_from_path(&project);
                }
                Some(JobOutcome::Failed(reason)) => {
                    self.report_failure("Project open failed", reason);
                }
                Some(JobOutcome::Cancelled) | None => {
                    self.prompt("Opening the pack was cancelled; nothing was unpacked.");
                }
            },
            None => {}
        }
    }
}

/// Where the reader puts the pack of `project`: none in a test, which never opens a dialog.
#[cfg(test)]
fn pick_pack_destination(_project: &Path) -> Option<PathBuf> {
    None
}

#[cfg(not(test))]
fn pick_pack_destination(project: &Path) -> Option<PathBuf> {
    use rspice_app_services::io::project_pack::PACK_EXTENSION;
    let stem = project.file_stem().map_or_else(
        || "project".into(),
        |stem| stem.to_string_lossy().into_owned(),
    );
    let mut archive = rfd::FileDialog::new()
        .add_filter("RSpice Project with Results", &[PACK_EXTENSION])
        .set_title("Pack Project with Results")
        .set_file_name(format!("{stem}.{PACK_EXTENSION}"))
        .save_file()?;
    if !rspice_app_services::io::project_pack::is_pack(&archive) {
        archive.set_extension(PACK_EXTENSION);
    }
    Some(archive)
}

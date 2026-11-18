//! Host file access and editor snapshots around the canonical project codec.

pub use crate::state::project_snapshot::ProjectSnapshot;
pub use rspice_formats::project_results::*;
pub use rspice_project::ProjectIoError;
use rspice_project::persistence::ProjectRead;
use rspice_vocabulary::product::ContentDigest;
#[cfg(any(not(target_arch = "wasm32"), test))]
use std::path::PathBuf;
use std::{fs::File, path::Path};

pub const PROJECT_FILTER: (&str, &[&str]) = ("RSpice Project", &["rspiceproj", "json"]);

#[cfg(not(target_arch = "wasm32"))]
pub fn show_open_project_dialog() -> Result<PathBuf, ProjectIoError> {
    rfd::FileDialog::new()
        .add_filter(PROJECT_FILTER.0, PROJECT_FILTER.1)
        .add_filter("All Files", &["*"])
        .set_title("Open RSpice Project")
        .pick_file()
        .ok_or(ProjectIoError::Cancelled)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn show_save_project_dialog(default_name: Option<&str>) -> Result<PathBuf, ProjectIoError> {
    let mut dialog = rfd::FileDialog::new()
        .add_filter(PROJECT_FILTER.0, PROJECT_FILTER.1)
        .set_title("Save RSpice Project");

    dialog = dialog.set_file_name(default_name.unwrap_or("untitled.rspiceproj"));

    let mut path = dialog.save_file().ok_or(ProjectIoError::Cancelled)?;
    let has_extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("rspiceproj"));
    if !has_extension {
        path.set_extension("rspiceproj");
    }
    Ok(path)
}

pub fn serialize_project_file(project: &ProjectSnapshot) -> Result<String, ProjectIoError> {
    rspice_project::serialize_project_file(&project.file)
}

pub fn load_project_text(
    contents: &str,
    source_path: Option<&Path>,
) -> Result<ProjectSnapshot, ProjectIoError> {
    rspice_project::decode_project_text(
        contents,
        source_path,
        crate::state::workspace::WorkspaceSourceFiles,
    )
    .map(ProjectSnapshot::from_decoded)
}

pub fn load_project_file(path: &Path) -> Result<ProjectSnapshot, ProjectIoError> {
    load_project_file_with_digest(path).map(|(project, _)| project)
}

/// Read, hash, parse, migrate, and validate one read of the file. The
/// returned digest is therefore the exact persistence identity accepted by
/// the caller, not a later re-read that could race an external editor.
pub(crate) fn load_project_file_with_digest(
    path: &Path,
) -> Result<(ProjectSnapshot, ContentDigest), ProjectIoError> {
    let read = read_project_file(path)?;
    let digest = read.digest();
    let project = read.decode(crate::state::workspace::WorkspaceSourceFiles)?;
    Ok((ProjectSnapshot::from_decoded(project), digest))
}

/// Read the project file at `path` once, parsing it as it is read, so its
/// size costs memory only in the project it holds.
pub(crate) fn read_project_file(path: &Path) -> Result<ProjectRead, ProjectIoError> {
    if !path.exists() {
        return Err(ProjectIoError::NotFound(path.to_path_buf()));
    }
    ProjectRead::read(File::open(path)?, Some(path))
}

/// The most of one file a browser build can hold: its memory is addressed
/// in 32 bits.
#[cfg(target_arch = "wasm32")]
pub const BROWSER_FILE_BYTES: u64 = u32::MAX as u64;

/// What the browser says of a file larger than it can hold.
#[cfg(target_arch = "wasm32")]
pub const BROWSER_FILE_TOO_LARGE: &str =
    "This file is larger than a browser can hold. Open it in RSpice for desktop.";

#[cfg(any(test, feature = "test-support"))]
pub mod fixtures;
#[cfg(test)]
mod tests;

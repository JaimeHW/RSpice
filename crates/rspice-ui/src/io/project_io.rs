//! Project file I/O.
//!
//! `.rspiceproj` stores the product-level workspace: project identity,
//! libraries/cells/views, open documents, and schematic buffers. Individual
//! schematic export remains available through `.rsch`; project files are the
//! native professional workflow container.

use std::fs::{self, File};
use std::io::{BufReader, BufWriter, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::state::{LibraryManager, ProjectWorkspace};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl ProjectVersion {
    pub const fn current() -> Self {
        Self {
            major: 1,
            minor: 0,
            patch: 0,
        }
    }

    pub fn is_compatible(self) -> bool {
        self.major == Self::current().major
    }
}

impl Default for ProjectVersion {
    fn default() -> Self {
        Self::current()
    }
}

impl std::fmt::Display for ProjectVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFile {
    pub version: ProjectVersion,
    pub workspace: ProjectWorkspace,
    pub libraries: LibraryManager,
}

impl ProjectFile {
    pub fn new(workspace: ProjectWorkspace, libraries: LibraryManager) -> Self {
        Self {
            version: ProjectVersion::current(),
            workspace,
            libraries,
        }
    }

    pub fn validate(&self) -> Result<(), ProjectIoError> {
        if !self.version.is_compatible() {
            return Err(ProjectIoError::IncompatibleVersion {
                file_version: self.version.to_string(),
                app_version: ProjectVersion::current().to_string(),
            });
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub enum ProjectIoError {
    Cancelled,
    NotFound(PathBuf),
    IncompatibleVersion {
        file_version: String,
        app_version: String,
    },
    ParseError(String),
    SerializeError(String),
    Io(String),
}

impl std::fmt::Display for ProjectIoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => f.write_str("Operation cancelled"),
            Self::NotFound(path) => write!(f, "Project file not found: {}", path.display()),
            Self::IncompatibleVersion {
                file_version,
                app_version,
            } => write!(
                f,
                "Project version {} is not compatible with app version {}",
                file_version, app_version
            ),
            Self::ParseError(error) => write!(f, "Project parse error: {}", error),
            Self::SerializeError(error) => write!(f, "Project serialize error: {}", error),
            Self::Io(error) => write!(f, "Project I/O error: {}", error),
        }
    }
}

impl std::error::Error for ProjectIoError {}

impl From<std::io::Error> for ProjectIoError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

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

#[cfg(target_arch = "wasm32")]
pub fn show_open_project_dialog() -> Result<PathBuf, ProjectIoError> {
    Err(ProjectIoError::Io(
        "Project dialogs are not supported on web".to_string(),
    ))
}

#[cfg(target_arch = "wasm32")]
pub fn show_save_project_dialog(_default_name: Option<&str>) -> Result<PathBuf, ProjectIoError> {
    Err(ProjectIoError::Io(
        "Project dialogs are not supported on web".to_string(),
    ))
}

pub fn save_project_file(project: &ProjectFile, path: &Path) -> Result<(), ProjectIoError> {
    if path.exists() {
        let backup = path.with_extension("rspiceproj.bak");
        if let Err(error) = fs::copy(path, backup) {
            log::warn!("Failed to create project backup: {}", error);
        }
    }

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let temp_path = path.with_extension("rspiceproj.tmp");
    let file = File::create(&temp_path)?;
    let mut writer = BufWriter::new(file);

    serde_json::to_writer_pretty(&mut writer, project)
        .map_err(|error| ProjectIoError::SerializeError(error.to_string()))?;
    writer.flush()?;
    fs::rename(temp_path, path)?;

    Ok(())
}

pub fn load_project_file(path: &Path) -> Result<ProjectFile, ProjectIoError> {
    if !path.exists() {
        return Err(ProjectIoError::NotFound(path.to_path_buf()));
    }

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut project: ProjectFile =
        serde_json::from_reader(reader).map_err(|e| ProjectIoError::ParseError(e.to_string()))?;
    project.validate()?;
    project.workspace.project.set_path(path.to_path_buf());
    Ok(project)
}

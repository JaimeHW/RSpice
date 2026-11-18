//! Where a library, and each of its cellviews, stands: the one set of words
//! the rail, the table, the inspector and the Overview all say.

use super::format::LibraryMode;
use super::path::LibraryPathError;

/// The mark a status earns beside its word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusMark {
    None,
    Caution,
    Error,
}

/// Why a folder or a file is not there to read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Missing {
    /// Nothing at the folder the path resolves to, shown in the host's form.
    NoFolder(String),
    /// Nothing at one file of a readable folder.
    NoFile(String),
    /// The path names no folder on this computer.
    Path(LibraryPathError),
}

impl Missing {
    pub fn detail(&self) -> String {
        match self {
            Self::NoFolder(folder) => format!("No folder at {folder}"),
            Self::NoFile(file) => format!("No file at {file}"),
            Self::Path(error) => error.to_string(),
        }
    }
}

/// Where one library stands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceStatus {
    /// Embedded: read from the project file with the rest of the project.
    InProject,
    /// Every file read and matching what is loaded. `moved_on` counts the
    /// cellviews whose file is newer than the project last saved with;
    /// `read_for_session` names the folder a browser read for this session.
    Found {
        mode: LibraryMode,
        moved_on: usize,
        read_for_session: Option<String>,
    },
    /// Files on disk differ from what is loaded.
    Changed {
        cellviews: usize,
    },
    /// Cellviews with unsaved edits here changed on disk too.
    ChangedSinceEditing {
        cellviews: usize,
    },
    /// Its layouts were drawn for another technology pin, so they are not
    /// loaded.
    TechnologyDiffers {
        library: String,
        project: String,
    },
    /// In a browser, which cannot read files by path.
    NotLoaded,
    NotFound(Missing),
    /// Damaged, not a library, or a newer format: the reader's reason.
    Unreadable(String),
    /// The folder's manifest names another library.
    DifferentLibrary {
        holds: String,
    },
}

impl ReferenceStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::InProject => "In project",
            Self::Found { .. } => "Found",
            Self::Changed { .. } => "Changed",
            Self::ChangedSinceEditing { .. } => CHANGED_SINCE_EDITING,
            Self::TechnologyDiffers { .. } => "Technology differs",
            Self::NotLoaded => "Not loaded",
            Self::NotFound(_) => "Not found",
            Self::Unreadable(_) => "Unreadable",
            Self::DifferentLibrary { .. } => "Different library",
        }
    }

    pub fn mark(&self) -> StatusMark {
        match self {
            Self::InProject | Self::Found { .. } => StatusMark::None,
            Self::Changed { .. } | Self::TechnologyDiffers { .. } | Self::NotLoaded => {
                StatusMark::Caution
            }
            Self::ChangedSinceEditing { .. }
            | Self::NotFound(_)
            | Self::Unreadable(_)
            | Self::DifferentLibrary { .. } => StatusMark::Error,
        }
    }

    pub fn detail(&self) -> Option<String> {
        match self {
            Self::InProject => None,
            Self::Found {
                mode,
                moved_on,
                read_for_session,
            } => Some(match (read_for_session, moved_on) {
                (Some(folder), _) => format!("Read from {folder} for this session"),
                (None, 0) => mode.label().to_owned(),
                (None, count) => format!(
                    "{} \u{00b7} {} newer than the project's save",
                    mode.label(),
                    cellviews(*count)
                ),
            }),
            Self::Changed { cellviews: count } => Some(format!("{} changed", cellviews(*count))),
            Self::ChangedSinceEditing { cellviews: count } => Some(format!(
                "{} changed on disk since you started editing; Save refuses them until you choose",
                cellviews(*count)
            )),
            Self::TechnologyDiffers { library, project } => Some(format!(
                "Layouts drawn for {library} \u{00b7} the project uses {project}"
            )),
            Self::NotLoaded => Some("A browser cannot read files by path".to_owned()),
            Self::NotFound(missing) => Some(missing.detail()),
            Self::Unreadable(reason) => Some(reason.clone()),
            Self::DifferentLibrary { holds } => Some(format!("The folder holds library '{holds}'")),
        }
    }
}

/// The word a cellview with unsaved edits reads when its file changed too.
pub const CHANGED_SINCE_EDITING: &str = "Changed on disk since you started editing";

/// Where one cellview, or a cell's or the library's own file, stands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CellViewStatus {
    /// Read and matching what is loaded. `saved_with` is the revision the
    /// project last saved with, when the file has moved on since.
    Found {
        revision: u64,
        saved_with: Option<u64>,
    },
    Changed {
        loaded: u64,
        on_disk: u64,
    },
    ChangedSinceEditing {
        loaded: u64,
        on_disk: u64,
    },
    /// A layout drawn for another technology pin, not loaded.
    TechnologyDiffers,
    NotFound(Missing),
    Unreadable(String),
}

impl CellViewStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Found { .. } => "Found",
            Self::Changed { .. } => "Changed",
            Self::ChangedSinceEditing { .. } => CHANGED_SINCE_EDITING,
            Self::TechnologyDiffers => "Technology differs",
            Self::NotFound(_) => "Not found",
            Self::Unreadable(_) => "Unreadable",
        }
    }

    pub fn mark(&self) -> StatusMark {
        match self {
            Self::Found { .. } => StatusMark::None,
            Self::Changed { .. } | Self::TechnologyDiffers => StatusMark::Caution,
            Self::ChangedSinceEditing { .. } | Self::NotFound(_) | Self::Unreadable(_) => {
                StatusMark::Error
            }
        }
    }

    pub fn detail(&self) -> Option<String> {
        match self {
            Self::Found {
                revision,
                saved_with: Some(saved),
            } => Some(format!(
                "Revision {revision} \u{00b7} project saved with {saved}"
            )),
            Self::Found { revision, .. } => Some(format!("Revision {revision}")),
            Self::Changed { loaded, on_disk } | Self::ChangedSinceEditing { loaded, on_disk } => {
                Some(format!(
                    "Revision {on_disk} on disk \u{00b7} {loaded} loaded"
                ))
            }
            Self::TechnologyDiffers => {
                Some("Drawn for another technology, so it is not loaded".to_owned())
            }
            Self::NotFound(missing) => Some(missing.detail()),
            Self::Unreadable(reason) => Some(reason.clone()),
        }
    }

    pub fn is_changed(&self) -> bool {
        matches!(
            self,
            Self::Changed { .. } | Self::ChangedSinceEditing { .. }
        )
    }
}

fn cellviews(count: usize) -> String {
    format!("{count} cellview{}", if count == 1 { "" } else { "s" })
}

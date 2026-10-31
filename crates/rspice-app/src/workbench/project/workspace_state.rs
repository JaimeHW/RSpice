//! What the Project workspace and the project launcher keep between frames:
//! the launcher's sort, filter and page, which project page is showing,
//! which projection the Library/Cellview Manager shows, and the cell or view
//! deletion it is reviewing.
//!
//! None of it is engineering data. Libraries, cells and views stay in the
//! project's library manager; this is only which view of them is in front.

use serde::{Deserialize, Serialize};

/// Sort order offered by the project launcher. The recent-files store is
/// maintained newest-first and records the last-opened timestamp, so
/// `LastOpened` preserves its durable ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProjectLauncherSort {
    #[default]
    LastOpened,
    Name,
    Owner,
}

impl ProjectLauncherSort {
    pub const fn label(self) -> &'static str {
        match self {
            Self::LastOpened => "Last opened",
            Self::Name => "Name",
            Self::Owner => "Owner",
        }
    }
}

/// Project groups backed by durable recent-project metadata.
///
/// Recovery has its own complete launcher page and governed templates are not
/// exposed until a template-package service exists, so neither is represented
/// as a decorative filter here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProjectLauncherFilter {
    #[default]
    All,
    Recent,
    Pinned,
    Shared,
}

impl ProjectLauncherFilter {
    pub const ALL: [Self; 4] = [Self::All, Self::Recent, Self::Pinned, Self::Shared];

    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Recent => "Recent",
            Self::Pinned => "Pinned",
            Self::Shared => "Shared",
        }
    }
}

/// Startup pages that have complete local executors in the desktop product.
/// Legal, identity, licensing, cloud, template, and extension-host pages are
/// intentionally absent until their governed services exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProjectLauncherPage {
    #[default]
    Projects,
    Recovery,
    SafeMode,
}

impl ProjectLauncherPage {
    pub const ALL: [Self; 3] = [Self::Projects, Self::Recovery, Self::SafeMode];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Projects => "Projects",
            Self::Recovery => "Recovery",
            Self::SafeMode => "Safe mode",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ProjectPage {
    #[serde(alias = "Dashboard", alias = "Activity")]
    #[default]
    Overview,
    Library,
    Configuration,
    #[serde(alias = "Technology")]
    Dependencies,
    Recovery,
}

impl ProjectPage {
    pub const ALL: [Self; 5] = [
        Self::Overview,
        Self::Library,
        Self::Configuration,
        Self::Dependencies,
        Self::Recovery,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::Library => "Library",
            Self::Configuration => "Configuration",
            Self::Dependencies => "Dependencies",
            Self::Recovery => "Recovery",
        }
    }
}

/// Persistent page selection inside the canonical Library/Cellview Manager.
///
/// The specialist surface is another projection over the project-owned
/// `LibraryManager`; this enum retains only which projection is visible and
/// never duplicates library, symbol, or form content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum LibraryCellviewPage {
    #[default]
    Libraries,
    SymbolForm,
}

impl LibraryCellviewPage {
    pub const ALL: [Self; 2] = [Self::Libraries, Self::SymbolForm];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Libraries => "Libraries",
            Self::SymbolForm => "Symbol & form",
        }
    }
}

#[cfg(test)]
mod tests;

/// The exact cell or view a deletion names.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum LibraryDeletionTarget {
    Cell {
        library: String,
        cell: String,
    },
    View {
        library: String,
        cell: String,
        view: String,
    },
}

impl LibraryDeletionTarget {
    pub(crate) fn library(&self) -> &str {
        match self {
            Self::Cell { library, .. } | Self::View { library, .. } => library,
        }
    }

    pub(crate) fn cell(&self) -> &str {
        match self {
            Self::Cell { cell, .. } | Self::View { cell, .. } => cell,
        }
    }

    pub(crate) fn view(&self) -> Option<&str> {
        match self {
            Self::Cell { .. } => None,
            Self::View { view, .. } => Some(view),
        }
    }

    pub(crate) fn display_path(&self) -> String {
        match self {
            Self::Cell { library, cell } => format!("{library}/{cell}"),
            Self::View {
                library,
                cell,
                view,
            } => format!("{library}/{cell}/{view}"),
        }
    }

    pub(crate) const fn kind_label(&self) -> &'static str {
        match self {
            Self::Cell { .. } => "Cell",
            Self::View { .. } => "View",
        }
    }
}

/// What becomes of the placements of a library object that is being deleted.
///
/// Removing a master is a library decision; what the drawings that placed it
/// should do about it is a separate design decision, and one the reader has to
/// make rather than inherit. Nothing is deleted until they have.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum DeletionInstanceResolution {
    /// Leave every placement drawn where it is. Each keeps naming the master
    /// it was placed from and reads as unresolved until it is rebound or
    /// removed, which is what a design review needs to be able to see.
    #[default]
    KeepUnresolved,
    /// Take every placement of the object out of every drawing in the project,
    /// as a single step that can be undone as a single step.
    RemoveInstances,
}

/// The cell or view deletion under review, against the library revision it
/// opened at, and the reader's answer about the placements once confirmed.
#[derive(Debug, Clone, Default)]
pub(crate) struct LibraryDeletionReviewState {
    pub(crate) target: Option<LibraryDeletionTarget>,
    pub(crate) expected_library_revision: u64,
    pub(crate) error: Option<String>,
    /// The choice the reader confirmed for the object's placements. Set when
    /// the deletion is staged and taken exactly once by the staged deletion,
    /// so the two halves of one decision cannot come apart.
    pub(crate) resolution: Option<DeletionInstanceResolution>,
}

impl LibraryDeletionReviewState {
    pub(crate) fn close(&mut self) {
        *self = Self::default();
    }
}

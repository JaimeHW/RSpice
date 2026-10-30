//! What the Project workspace and the project launcher keep between frames:
//! the launcher's sort, filter and page, which project page is showing, and
//! which projection the Library/Cellview Manager shows.
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

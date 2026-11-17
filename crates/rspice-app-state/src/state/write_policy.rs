//! What this process may write: the safe-mode policy it was started under and
//! the documents another live-session participant holds the write lease on.

use std::collections::HashMap;

/// Safe-mode controls requested for this process. Each option is one the
/// running process enforces; an option with nothing behind it is not offered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalSafeModeOptions {
    pub isolate_prior_documents: bool,
    pub reset_layout: bool,
    pub open_project_read_only: bool,
}

impl Default for LocalSafeModeOptions {
    fn default() -> Self {
        Self {
            isolate_prior_documents: true,
            reset_layout: false,
            open_project_read_only: false,
        }
    }
}

impl LocalSafeModeOptions {
    pub const fn has_effect(self) -> bool {
        self.isolate_prior_documents || self.reset_layout || self.open_project_read_only
    }
}

/// Why this window opened the project read-only: it cannot write the file,
/// or another window has the project open.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpenedReadOnly {
    /// The project's folder takes no new file, as a read-only share, medium
    /// or folder answers, so no save can be published there.
    FolderNotWritable { folder: std::path::PathBuf },
    /// The project file is marked read-only.
    FileReadOnly,
    /// Another RSpice window has the project open: who and since when, when
    /// that window's record could be read.
    OpenElsewhere { holder: Option<ProjectHolder> },
}

/// The window that holds a project open.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectHolder {
    pub user: String,
    pub machine: String,
    pub since_unix_ms: u64,
}

impl ProjectHolder {
    /// Who holds the project, as a sentence names them: "ada on BENCH-7".
    pub fn display_name(&self) -> String {
        match (self.user.is_empty(), self.machine.is_empty()) {
            (false, false) => format!("{} on {}", self.user, self.machine),
            (false, true) => self.user.clone(),
            (true, false) => format!("a window on {}", self.machine),
            (true, true) => "another RSpice window".to_owned(),
        }
    }
}

impl OpenedReadOnly {
    /// What the read-only notice says.
    pub fn sentence(&self) -> String {
        match self {
            Self::FolderNotWritable { folder } => format!(
                "Opened read-only: the folder \u{201c}{}\u{201d} cannot be written.",
                folder.display()
            ),
            Self::FileReadOnly => {
                "Opened read-only: the project file is marked read-only.".to_owned()
            }
            Self::OpenElsewhere {
                holder: Some(holder),
            } => format!(
                "Opened read-only: {} has had this project open since {}.",
                holder.display_name(),
                crate::time_compat::utc_stamp(holder.since_unix_ms)
            ),
            Self::OpenElsewhere { holder: None } => {
                "Opened read-only: another RSpice window has this project open.".to_owned()
            }
        }
    }

    /// Whether the reader may take the project for writing anyway: only when
    /// the file could be written, and another window is what holds it.
    pub const fn can_open_anyway(&self) -> bool {
        matches!(self, Self::OpenElsewhere { .. })
    }
}

/// Current-launch safe-mode state. The serialized pre-safe-mode session is
/// retained only so application persistence can keep it byte-for-byte
/// equivalent for the next normal launch.
#[derive(Debug, Clone, Default)]
pub struct LocalSafeModeState {
    pub active: bool,
    pub draft: LocalSafeModeOptions,
    pub applied: LocalSafeModeOptions,
    preserved_session: Option<String>,
    /// Why the open project was opened read-only, when this window could not
    /// or should not write it. Set and cleared by the project lifecycle.
    pub opened_read_only: Option<OpenedReadOnly>,
}

impl LocalSafeModeState {
    pub fn activate(&mut self, options: LocalSafeModeOptions, session: String) {
        self.active = true;
        self.draft = options;
        self.applied = options;
        self.preserved_session = Some(session);
    }

    pub fn preserved_session(&self) -> Option<&str> {
        self.preserved_session.as_deref()
    }

    /// Whether safe mode, rather than the project's file, keeps the project
    /// read-only.
    pub const fn safe_mode_read_only(&self) -> bool {
        self.active && self.applied.open_project_read_only
    }

    /// Whether no write to the project may be made from this window.
    pub const fn project_read_only(&self) -> bool {
        (self.active && self.applied.open_project_read_only) || self.opened_read_only.is_some()
    }

    /// Why the project is read-only, in the words every refused write gives.
    pub fn read_only_reason(&self) -> Option<String> {
        if self.active && self.applied.open_project_read_only {
            Some("Safe mode opened this project read-only.".to_owned())
        } else {
            self.opened_read_only.as_ref().map(OpenedReadOnly::sentence)
        }
    }
}

/// Documents another live-session participant currently holds the write
/// lease on, projected each frame by the live-session engine. Runtime-only:
/// empty whenever no live session is attached. The netlist entry doubles as
/// that document's edit gate; schematic buffers gate through their own
/// `read_only` flag and use these entries for the deny message.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct LiveWriteLocks {
    /// Locked schematic buffers, by cell-view key → holder display name.
    pub schematic_views: HashMap<String, String>,
    /// Who holds netlist write authority when this install does not.
    pub netlist: Option<String>,
    /// This install is a live-session guest mirroring the host's project,
    /// so documents without a held lease are read-only wholesale.
    pub mirror: bool,
    /// Whether the session policy lets this mirror be saved as a copy.
    /// Meaningful only while `mirror` is set.
    pub mirror_save_copy_allowed: bool,
}

//! What this process may write: the safe-mode policy it was started under and
//! the documents another live-session participant holds the write lease on.

use std::collections::HashMap;

/// Safe-mode controls requested for this process. Capability-gated options are
/// rejected before activation, so every applied bit represents an enforced
/// runtime policy rather than presentation-only state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalSafeModeOptions {
    pub disable_third_party_extensions: bool,
    pub disable_gpu_acceleration: bool,
    pub isolate_prior_documents: bool,
    pub reset_layout: bool,
    pub open_project_read_only: bool,
}

impl Default for LocalSafeModeOptions {
    fn default() -> Self {
        Self {
            disable_third_party_extensions: true,
            // eframe selects the graphics adapter before this launcher exists.
            // The UI capability-gates this option until a software adapter can
            // actually be selected for the current platform.
            disable_gpu_acceleration: false,
            isolate_prior_documents: true,
            reset_layout: false,
            open_project_read_only: false,
        }
    }
}

impl LocalSafeModeOptions {
    pub const fn has_effect(self) -> bool {
        self.disable_third_party_extensions
            || self.disable_gpu_acceleration
            || self.isolate_prior_documents
            || self.reset_layout
            || self.open_project_read_only
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
}

impl LocalSafeModeState {
    pub(crate) fn activate(&mut self, options: LocalSafeModeOptions, session: String) {
        self.active = true;
        self.draft = options;
        self.applied = options;
        self.preserved_session = Some(session);
    }

    pub(crate) fn preserved_session(&self) -> Option<&str> {
        self.preserved_session.as_deref()
    }

    pub(crate) const fn project_read_only(&self) -> bool {
        self.active && self.applied.open_project_read_only
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

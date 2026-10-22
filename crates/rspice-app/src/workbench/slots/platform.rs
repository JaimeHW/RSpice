//! The platform services' runtime state: what opening, saving and recovering
//! a project keeps between frames.

/// What the platform services keep open between frames.
#[derive(Debug, Clone, Default)]
pub(crate) struct PlatformState {
    /// A saved-file open found a bound, eligible autosave checkpoint; the
    /// restore dialog resolves it before either exact byte snapshot is loaded.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) pending_autosave_restore:
        Option<crate::workbench::lifecycle::recovery_checkpoint::AutosaveRestoreCandidate>,
}

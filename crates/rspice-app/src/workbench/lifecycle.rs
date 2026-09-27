//! Opening, checkpointing, and recovering a working session.
//!
//! `project_lifecycle` owns the transactional document registry and the
//! accepted baseline; `project_checkpoint` and `recovery_checkpoint` write
//! the crash-recovery record; `recovery` reads it back. `session` and
//! `window_session` are the runtime state a restart has to reconstruct — open
//! views, selection, symbol editor, window bounds — none of which the project
//! file itself persists.

pub(crate) mod project_checkpoint;
pub(crate) mod project_lifecycle;
pub(crate) mod recovery;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod recovery_checkpoint;
pub(crate) mod session;
pub(crate) mod window_session;

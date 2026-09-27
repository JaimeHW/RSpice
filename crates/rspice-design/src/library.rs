//! Library operation contracts and collaboration-lock authority.

mod locks;
mod mutation;

pub use locks::{
    ProjectLibraryEditLock, ProjectLibraryEditLockScope, ProjectLibraryLockAuthority,
    ProjectLibraryLockBlockReason, ProjectLibraryLockSnapshot,
};
pub use mutation::{ProjectLibraryMutation, validate_library_audit_text};

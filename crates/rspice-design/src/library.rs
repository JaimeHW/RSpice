//! Library records, catalog revisions, mutations, and collaboration-lock authority.

mod catalog;
mod cell;
mod edit;
mod locks;
mod mutation;
mod record;
mod view;

pub use locks::{
    ProjectLibraryEditLock, ProjectLibraryEditLockScope, ProjectLibraryLockAuthority,
    ProjectLibraryLockBlockReason, ProjectLibraryLockSnapshot,
};
pub use mutation::{ProjectLibraryMutation, validate_library_audit_text};

pub use catalog::LibraryCatalog;
pub use cell::Cell;
pub use edit::LibraryEdit;
pub use record::Library;
pub use view::{View, ViewType};

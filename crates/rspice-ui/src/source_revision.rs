//! Framework-independent mutation identity for source owners and derived caches.

use std::sync::Arc;

/// Retaining a revision prevents identity reuse after its source is replaced.
/// Clones share identity until the owner exposes a mutable borrow.
#[derive(Debug, Clone, Default)]
pub(crate) struct SourceRevision(Arc<()>);

impl SourceRevision {
    pub(crate) fn advance(&mut self) {
        // Unobserved consecutive writes need no additional allocation.
        Arc::make_mut(&mut self.0);
    }
}

impl PartialEq for SourceRevision {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for SourceRevision {}

//! Framework-independent mutation identity for source owners and derived caches.

use std::sync::Arc;

/// Retaining a revision prevents identity reuse after its source is replaced.
/// Clones share identity until the owner exposes a mutable borrow.
#[derive(Debug, Clone, Default)]
pub struct SourceRevision(Arc<()>);

impl SourceRevision {
    pub fn advance(&mut self) {
        // Unobserved consecutive writes need no additional allocation.
        Arc::make_mut(&mut self.0);
    }

    /// Whether anything besides this holds the revision: an owner whose
    /// revision nothing else holds may change its source again without
    /// telling the change apart from the one before.
    pub fn is_observed(&self) -> bool {
        Arc::strong_count(&self.0) > 1
    }
}

impl PartialEq for SourceRevision {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl Eq for SourceRevision {}

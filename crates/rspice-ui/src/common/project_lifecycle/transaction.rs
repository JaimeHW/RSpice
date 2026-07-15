//! One-at-a-time project lifecycle transactions.
//!
//! Native operations normally complete synchronously, while browser pickers
//! and writable handles complete on a later frame.  Both paths use the same
//! token so a late completion can never commit into a different project.

use crate::product::ContentDigest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TransactionKind {
    SaveActive,
    SaveAll,
    SaveProjectCopy,
    OpenProject,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct TransactionId(uuid::Uuid);

impl TransactionId {
    pub(crate) fn new() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct LifecycleTransaction {
    pub(crate) id: TransactionId,
    pub(crate) kind: TransactionKind,
    /// Content observed when a destructive replacement was authorized.  A
    /// picker completing after another edit must not overwrite that edit.
    pub(crate) replacement_guard: Option<ContentDigest>,
}

impl LifecycleTransaction {
    pub(crate) fn save(kind: TransactionKind) -> Self {
        debug_assert!(matches!(
            kind,
            TransactionKind::SaveActive
                | TransactionKind::SaveAll
                | TransactionKind::SaveProjectCopy
        ));
        Self {
            id: TransactionId::new(),
            kind,
            replacement_guard: None,
        }
    }

    pub(crate) fn replacement(content: ContentDigest) -> Self {
        Self {
            id: TransactionId::new(),
            kind: TransactionKind::OpenProject,
            replacement_guard: Some(content),
        }
    }
}

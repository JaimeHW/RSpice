//! Pending document transactions and committed history share one owner.
use super::super::document::SchematicDocument;
use super::super::history::{SchematicHistory, SchematicSnapshot};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_OPERATION_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone)]
struct PendingOperation {
    id: u64,
    before_snapshot: SchematicSnapshot,
    /// The name of the content `before_snapshot` was taken from, when it
    /// has one.
    before_state: Option<u64>,
    description: String,
    nesting_depth: usize,
}

#[derive(Debug, Clone, Default)]
pub(super) struct EditHistory {
    pub(super) committed: SchematicHistory,
    pending: Option<PendingOperation>,
}

impl EditHistory {
    pub(super) fn initialize(&mut self) {
        self.committed.initialize();
        self.pending = None;
    }

    #[cfg(any(test, feature = "test-support"))]
    pub(super) fn clear(&mut self) {
        self.committed.clear();
        self.pending = None;
    }

    /// Begin or join an operation on `document`, whose content is named
    /// `state`.
    pub(super) fn begin(
        &mut self,
        document: &SchematicDocument,
        state: Option<u64>,
        description: impl Into<String>,
    ) -> u64 {
        if !self.committed.is_initialized() {
            self.initialize();
        }
        self.begin_named_from(SchematicSnapshot::capture(document), state, description)
    }

    /// Begin or join an operation from a baseline taken earlier, whose
    /// content has no name the history can return to.
    pub(super) fn begin_from(
        &mut self,
        before_snapshot: SchematicSnapshot,
        description: impl Into<String>,
    ) -> u64 {
        self.begin_named_from(before_snapshot, None, description)
    }

    /// Begin or join an operation from a baseline taken earlier, from the
    /// content named `before_state`.
    pub(super) fn begin_named_from(
        &mut self,
        mut before_snapshot: SchematicSnapshot,
        before_state: Option<u64>,
        description: impl Into<String>,
    ) -> u64 {
        if !self.committed.is_initialized() {
            self.initialize();
        }
        self.committed
            .capture_sheet_assignments(&mut before_snapshot);
        if let Some(pending) = self.pending.as_mut() {
            pending.nesting_depth = pending.nesting_depth.saturating_add(1);
            log::debug!(
                "nested undo operation joined outer transaction {:?}",
                pending.description
            );
            return pending.id;
        }
        let id = NEXT_OPERATION_ID.fetch_add(1, Ordering::Relaxed);
        self.pending = Some(PendingOperation {
            id,
            before_snapshot,
            before_state,
            description: description.into(),
            nesting_depth: 0,
        });
        id
    }

    pub(super) fn end(&mut self, document: &SchematicDocument) -> bool {
        let after_snapshot = SchematicSnapshot::capture(document);
        if let Some(pending) = self.pending.as_mut()
            && pending.nesting_depth > 0
        {
            pending.nesting_depth -= 1;
            return false;
        }
        let pending = match self.pending.take() {
            Some(pending) => pending,
            None => {
                log::warn!("end_operation called without begin_operation");
                return false;
            }
        };
        self.committed.commit(
            pending.before_snapshot,
            pending.before_state,
            &after_snapshot,
            pending.description,
        )
    }

    pub(super) fn cancel(&mut self) -> Option<(u64, SchematicSnapshot, Option<u64>)> {
        let pending = self.pending.take()?;
        self.committed
            .adopt_restored_sheet_assignments(&pending.before_snapshot.sheet_assignments);
        Some((pending.id, pending.before_snapshot, pending.before_state))
    }

    /// The name of the content the pending operation began on, when the
    /// next [`Self::end`] closes it rather than a scope nested in it.
    pub(super) fn closing_state(&self) -> Option<Option<u64>> {
        self.pending
            .as_ref()
            .filter(|pending| pending.nesting_depth == 0)
            .map(|pending| pending.before_state)
    }

    /// The document as the pending operation found it.
    pub(super) fn pending_baseline(&self) -> Option<&SchematicSnapshot> {
        self.pending
            .as_ref()
            .map(|pending| &pending.before_snapshot)
    }

    pub(super) fn pending_operation_id(&self) -> Option<u64> {
        self.pending.as_ref().map(|pending| pending.id)
    }
}

#[cfg(test)]
mod tests;

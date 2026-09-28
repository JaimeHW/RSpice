//! Pending document transactions and committed history share one owner.
use super::super::document::SchematicDocument;
use super::super::history::{SchematicHistory, SchematicSnapshot};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_OPERATION_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone)]
struct PendingOperation {
    id: u64,
    before_snapshot: SchematicSnapshot,
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

    pub(super) fn clear(&mut self) {
        self.committed.clear();
        self.pending = None;
    }

    pub(super) fn begin(
        &mut self,
        document: &SchematicDocument,
        description: impl Into<String>,
    ) -> u64 {
        if !self.committed.is_initialized() {
            self.initialize();
        }
        self.begin_from(SchematicSnapshot::capture(document), description)
    }

    pub(super) fn begin_from(
        &mut self,
        mut before_snapshot: SchematicSnapshot,
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
            &after_snapshot,
            pending.description,
        )
    }

    pub(super) fn cancel(&mut self) -> Option<(u64, SchematicSnapshot)> {
        let pending = self.pending.take()?;
        self.committed
            .adopt_restored_sheet_assignments(&pending.before_snapshot.sheet_assignments);
        Some((pending.id, pending.before_snapshot))
    }

    pub(super) fn pending_operation_id(&self) -> Option<u64> {
        self.pending.as_ref().map(|pending| pending.id)
    }
}

#[cfg(test)]
mod tests;

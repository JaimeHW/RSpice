//! Live schematic document, identity, revisions and edit history.

use super::component_type::ComponentType;
use super::document::SchematicDocument;
use super::history::{SchematicHistory, SchematicSnapshot};
use history::EditHistory;
mod history;
use super::identity::{DocumentRepair, SchematicIdentity};
use rspice_design_model::design_management::SheetId;
mod annotations;
mod bindings;
pub mod bulk_edit;
mod buses;
mod hierarchy;
mod layout;
mod model_bindings;
mod movement;
pub mod named_net;
mod object_edits;
mod placement;
mod probes;
mod projection;
pub mod properties;
pub mod references;
pub mod sheet_topology;
mod stimuli;
mod symbols;
mod validated_revisions;
mod wires;

use std::collections::BTreeMap;

/// The authoritative editable design. View interaction and pending gestures
/// remain with the editor; persisted content is read through an immutable view.
#[derive(Debug, Clone)]
pub struct Schematic {
    document: SchematicDocument,
    identity: SchematicIdentity,
    topology_version: u64,
    content_version: u64,
    history: EditHistory,
}

impl AsRef<SchematicDocument> for Schematic {
    fn as_ref(&self) -> &SchematicDocument {
        self.document()
    }
}

impl serde::Serialize for Schematic {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serde::Serialize::serialize(self.document(), serializer)
    }
}

impl<'de> serde::Deserialize<'de> for Schematic {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::from_loaded_document(serde::Deserialize::deserialize(
            deserializer,
        )?))
    }
}

impl Default for Schematic {
    fn default() -> Self {
        Self::from_document(SchematicDocument::default())
    }
}

impl Schematic {
    pub fn from_document(document: SchematicDocument) -> Self {
        Self {
            document,
            identity: SchematicIdentity::with_cursor(1),
            topology_version: 0,
            content_version: 0,
            history: EditHistory::default(),
        }
    }

    /// Loading retains the legacy zero allocator cursor until repair/allocation.
    pub fn from_loaded_document(document: SchematicDocument) -> Self {
        Self {
            identity: SchematicIdentity::default(),
            ..Self::from_document(document)
        }
    }

    pub fn document(&self) -> &SchematicDocument {
        &self.document
    }
    pub fn into_document(self) -> SchematicDocument {
        self.document
    }
    pub fn history(&self) -> &SchematicHistory {
        &self.history.committed
    }
    pub const fn topology_version(&self) -> u64 {
        self.topology_version
    }
    pub const fn content_version(&self) -> u64 {
        self.content_version
    }
    pub const fn identity_cursor(&self) -> u64 {
        self.identity.cursor()
    }

    pub fn allocate_id(&mut self) -> u64 {
        self.identity.allocate(&self.document)
    }
    pub fn generate_name(&mut self, kind: ComponentType) -> String {
        self.identity.generate_name(kind)
    }

    /// External symbol/catalog changes also invalidate document geometry caches.
    pub fn invalidate_topology(&mut self) {
        self.topology_version = self.topology_version.wrapping_add(1);
    }

    pub fn repair_document(&mut self) -> DocumentRepair {
        let repaired = self.identity.repair_document(&mut self.document);
        self.topology_version = self
            .topology_version
            .wrapping_add(repaired.topology_changes);
        repaired
    }

    pub fn reconcile_grid_pitch(&mut self) -> i32 {
        let grid_size = self.document.document_policy.grid_pitch.canvas_grid_size();
        self.document.grid_size = grid_size;
        grid_size
    }

    /// Project-save candidates discard runtime conductor attachment records.
    pub fn strip_runtime_connections_for_save(&mut self) {
        self.document.connections.clear();
    }

    pub fn initialize_history(&mut self) {
        self.history.initialize();
    }
    pub fn clear_history(&mut self) {
        self.history.clear();
    }
    pub fn clear_redo(&mut self) {
        self.history.committed.clear_redo();
    }

    pub fn set_live_sheet_assignments(&mut self, assignments: BTreeMap<u64, SheetId>) {
        self.history
            .committed
            .set_live_sheet_assignments(assignments);
    }
    pub fn take_restored_sheet_assignments(&mut self) -> BTreeMap<u64, SheetId> {
        self.history.committed.take_restored_sheet_assignments()
    }

    /// Begin or join an atomic document edit, preserving the outer baseline.
    pub fn begin_operation(&mut self, description: impl Into<String>) -> u64 {
        self.history.begin(&self.document, description)
    }

    /// Begin a scope using a baseline captured by a live property-edit session.
    pub fn begin_operation_from(
        &mut self,
        before: SchematicSnapshot,
        description: impl Into<String>,
    ) -> u64 {
        self.history.begin_from(before, description)
    }

    pub fn end_operation(&mut self) -> bool {
        let committed = self.history.end(&self.document);
        if committed {
            self.content_version = self.content_version.wrapping_add(1);
        }
        committed
    }

    pub fn pending_operation_id(&self) -> Option<u64> {
        self.history.pending_operation_id()
    }

    /// Restore an aborted edit; editor cancellation metadata is held by its caller.
    pub fn cancel_operation(&mut self) -> Option<CancelledOperation> {
        let (operation_id, before) = self.history.cancel()?;
        let repaired = if before.is_equal_document(&self.document) {
            None
        } else {
            self.apply_snapshot(&before);
            self.reconcile_grid_pitch();
            Some(self.repair_document())
        };
        Some(CancelledOperation {
            operation_id,
            repaired,
        })
    }

    pub fn apply_snapshot(&mut self, snapshot: &SchematicSnapshot) {
        if snapshot.apply(&mut self.document) {
            self.invalidate_topology();
        }
    }

    pub fn undo(&mut self) -> Option<DocumentRepair> {
        if !self.history.committed.can_undo() {
            return None;
        }
        let current = SchematicSnapshot::capture(&self.document);
        let (snapshot, _) = self.history.committed.undo(current)?;
        self.apply_snapshot(&snapshot);
        self.reconcile_grid_pitch();
        let repaired = self.repair_document();
        self.content_version = self.content_version.wrapping_add(1);
        Some(repaired)
    }

    pub fn redo(&mut self) -> Option<DocumentRepair> {
        if !self.history.committed.can_redo() {
            return None;
        }
        let current = SchematicSnapshot::capture(&self.document);
        let (snapshot, _) = self.history.committed.redo(current)?;
        self.apply_snapshot(&snapshot);
        self.reconcile_grid_pitch();
        let repaired = self.repair_document();
        self.content_version = self.content_version.wrapping_add(1);
        Some(repaired)
    }

    #[cfg(feature = "schematic-test-fixtures")]
    pub fn set_identity_for_test(&mut self, identity: SchematicIdentity) {
        self.identity = identity;
    }

    /// Explicit dependency-test support; absent from production feature builds.
    #[cfg(feature = "schematic-test-fixtures")]
    pub fn document_mut_for_test(&mut self) -> &mut SchematicDocument {
        &mut self.document
    }
}

/// Identifies the editor baseline to restore and any runtime repair to reconcile.
pub struct CancelledOperation {
    pub operation_id: u64,
    pub repaired: Option<DocumentRepair>,
}

/// A completed document scope and its result. Nested scopes leave the outer
/// transaction pending, so mutation and history publication are distinct facts.
pub struct DocumentEdit<T> {
    pub value: T,
    pub committed: bool,
}

//! Live schematic document, identity, revisions and edit history.

use super::component_type::ComponentType;
use super::document::SchematicDocument;
use super::history::{Restored, SchematicHistory, SchematicSnapshot};
use history::EditHistory;
mod history;
use super::identity::{DocumentRepair, SchematicIdentity};
use rspice_design_model::design_management::SheetId;
mod annotations;
mod bindings;
#[cfg(any(test, feature = "schematic-test-fixtures"))]
pub use bindings::{binding_revalidation_count, reset_binding_revalidation_count};
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
pub use projection::{projected_point_sheet, projected_sheet_offset};
pub mod properties;
pub mod references;
pub mod sheet_topology;
mod stimuli;
mod symbols;
mod validated_revisions;
mod waivers;
mod wires;

use std::collections::BTreeMap;

/// The authoritative editable design. View interaction and pending gestures
/// remain with the editor; persisted content is read through an immutable view.
#[derive(Debug, Clone)]
pub struct Schematic {
    /// Stamped, so every edit — through any method below, a history step or
    /// a cancelled gesture — names the content it leaves behind, and evidence
    /// derived from the document can be reused exactly until it does.
    document: crate::stamped::Stamped<SchematicDocument>,
    identity: SchematicIdentity,
    topology_version: u64,
    content_version: u64,
    history: EditHistory,
    /// Where the content stands in its history.
    position: Position,
}

/// The name of the content the last step through the history left, with
/// the stamp it left it under. Equal stamps say the content is still that
/// one; any other edit moves the stamp and leaves the content unnamed.
#[derive(Debug, Clone, Copy)]
struct Position {
    state: u64,
    stamp: u64,
    /// Whether the history gave `state`, rather than the stamp standing in
    /// for a name it could not give.
    named: bool,
}

impl Position {
    /// No content: no stamp is ever 0, so nothing stands here.
    const UNNAMED: Self = Self {
        state: 0,
        stamp: 0,
        named: false,
    };
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
            document: crate::stamped::Stamped::new(document),
            identity: SchematicIdentity::with_cursor(1),
            topology_version: 0,
            content_version: 0,
            history: EditHistory::default(),
            position: Position::UNNAMED,
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
        self.document.into_inner()
    }
    /// The stamp of the document as it stands: two schematics with equal
    /// stamps hold equal documents, and every edit issues a new one. See
    /// [`crate::stamped`].
    pub const fn content_stamp(&self) -> u64 {
        self.document.stamp()
    }
    pub fn history(&self) -> &SchematicHistory {
        &self.history.committed
    }

    /// The name of the content as it stands, when the history gave it: the
    /// content is exactly what the last commit, undo, redo or cancel left
    /// under that name. `None` once anything else moved it, or when the
    /// step could not say which content it left. Two equal names are equal
    /// content: a commit names what it leaves afresh, and an undo or redo
    /// gives back the name of the content it restores. A drawing undone
    /// back to the content it was saved with therefore carries the name it
    /// was saved under, and one edited anew carries a name no save has seen.
    pub fn history_state(&self) -> Option<u64> {
        (self.position.named && self.position.stamp == self.content_stamp())
            .then_some(self.position.state)
    }

    /// The name of the content as it stands, for a step that may return to
    /// it. Content that moved outside the history since its last step is
    /// named by its stamp, and every step recorded before loses the name it
    /// returns to: a step restores the drawing it holds, not what moved
    /// beside it.
    fn name_current(&mut self) -> u64 {
        if self.position.stamp != self.content_stamp() {
            self.history.committed.forget_states();
            self.stand_unnamed();
        }
        self.position.state
    }

    /// Stand at content the history gave no name, named by its stamp for a
    /// later step to return to.
    fn stand_unnamed(&mut self) {
        let stamp = self.content_stamp();
        self.position = Position {
            state: stamp,
            stamp,
            named: false,
        };
    }

    /// Stand at the content named `state` as it stands now, or at content
    /// with no name.
    fn arrive(&mut self, state: Option<u64>) {
        match state {
            Some(state) => {
                self.position = Position {
                    state,
                    stamp: self.content_stamp(),
                    named: true,
                };
            }
            None => self.stand_unnamed(),
        }
    }

    /// Stand at `state` after restoring `restored`, when the content is
    /// exactly that snapshot's: a repair after the restore can move it.
    fn arrive_restored(&mut self, restored: &SchematicSnapshot, state: Option<u64>) {
        let exact = state.filter(|_| restored.is_equal_document(&self.document));
        self.arrive(exact);
    }

    /// Close the pending operation's scope: a commit names the content it
    /// leaves afresh, and an operation that changed nothing leaves the
    /// content it began on.
    fn close_scope(&mut self, closing: Option<Option<u64>>, committed: bool) {
        match closing {
            Some(_) if committed => self.arrive(Some(crate::stamped::issue())),
            Some(before_state) => self.arrive(before_state),
            None => {}
        }
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
        self.identity.generate_name(&self.document, kind)
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
        // Written only when it moves, so reconciling a document that already
        // agrees with its policy is not an edit of it.
        if self.document.grid_size != grid_size {
            self.document.grid_size = grid_size;
        }
        grid_size
    }

    /// Project-save candidates discard runtime conductor attachment records.
    pub fn strip_runtime_connections_for_save(&mut self) {
        self.document.connections.clear();
    }

    pub fn initialize_history(&mut self) {
        self.history.initialize();
    }
    #[cfg(any(test, feature = "test-support"))]
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
        let state = self.name_current();
        self.history.begin(&self.document, Some(state), description)
    }

    /// Copy the baseline [`Self::begin_operation`] would capture, ahead of an
    /// edit that may follow, for [`Self::begin_prepared_operation`].
    pub fn prepare_operation(&self) -> PreparedBaseline {
        PreparedBaseline {
            stamp: self.content_stamp(),
            snapshot: SchematicSnapshot::capture(&self.document),
        }
    }

    /// [`Self::begin_operation`], taking its baseline from `prepared` while
    /// the document still carries the stamp it was copied under. Equal stamps
    /// are equal content, so that copy is the one `begin_operation` would
    /// capture now; a baseline the document has moved on from is dropped and
    /// the document captured afresh.
    pub fn begin_prepared_operation(
        &mut self,
        prepared: Option<PreparedBaseline>,
        description: impl Into<String>,
    ) -> u64 {
        match prepared {
            Some(prepared) if prepared.stamp == self.content_stamp() => {
                let state = self.name_current();
                self.history
                    .begin_named_from(prepared.snapshot, Some(state), description)
            }
            _ => self.begin_operation(description),
        }
    }

    /// Begin a scope using a baseline captured by a live property-edit session.
    pub fn begin_operation_from(
        &mut self,
        before: SchematicSnapshot,
        description: impl Into<String>,
    ) -> u64 {
        // The baseline was taken before the content moved to where it
        // stands, so the content it holds has no name to return to; what
        // moved since moved outside the history.
        let _ = self.name_current();
        self.history.begin_from(before, description)
    }

    /// [`Self::begin_operation_from`] for a baseline taken from the
    /// content named `state`, which [`Self::name_current`] gave just before
    /// it moved.
    fn begin_named_operation_from(
        &mut self,
        before: SchematicSnapshot,
        state: u64,
        description: impl Into<String>,
    ) -> u64 {
        self.history
            .begin_named_from(before, Some(state), description)
    }

    pub fn end_operation(&mut self) -> bool {
        let closing = self.history.closing_state();
        let committed = self.history.end(&self.document);
        if committed {
            self.content_version = self.content_version.wrapping_add(1);
        }
        self.close_scope(closing, committed);
        committed
    }

    pub fn pending_operation_id(&self) -> Option<u64> {
        self.history.pending_operation_id()
    }

    /// Restore an aborted edit; editor cancellation metadata is held by its caller.
    pub fn cancel_operation(&mut self) -> Option<CancelledOperation> {
        let (operation_id, before, before_state) = self.history.cancel()?;
        let repaired = if before.is_equal_document(&self.document) {
            self.arrive(before_state);
            None
        } else {
            self.apply_snapshot(&before);
            self.reconcile_grid_pitch();
            let repaired = self.repair_document();
            self.arrive_restored(&before, before_state);
            Some(repaired)
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
        let state = self.name_current();
        let current = SchematicSnapshot::capture(&self.document);
        let restored = self.history.committed.undo(current, Some(state))?;
        Some(self.restore(&restored))
    }

    pub fn redo(&mut self) -> Option<DocumentRepair> {
        if !self.history.committed.can_redo() {
            return None;
        }
        let state = self.name_current();
        let current = SchematicSnapshot::capture(&self.document);
        let restored = self.history.committed.redo(current, Some(state))?;
        Some(self.restore(&restored))
    }

    /// Make the content what an undo or redo restores, and stand at its name.
    fn restore(&mut self, restored: &Restored) -> DocumentRepair {
        self.apply_snapshot(&restored.snapshot);
        self.reconcile_grid_pitch();
        let repaired = self.repair_document();
        self.content_version = self.content_version.wrapping_add(1);
        self.arrive_restored(&restored.snapshot, restored.state);
        repaired
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

/// A document's undo baseline, copied before the edit it is for has begun:
/// at the press of a pointer gesture, so the gesture's first move does not
/// pay for the copy. It keeps the stamp of the content it copied, and
/// [`Schematic::begin_prepared_operation`] uses it only while the document
/// still carries that stamp.
#[derive(Debug, Clone)]
pub struct PreparedBaseline {
    stamp: u64,
    snapshot: SchematicSnapshot,
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

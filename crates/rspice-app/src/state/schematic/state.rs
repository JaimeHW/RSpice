//! Schematic State
//!
//! Main state container for the schematic editor.

use rspice_schematic_editor::session::EditorSession;
use std::path::PathBuf;

#[cfg(test)]
use rspice_design::schematic::identity::SchematicIdentity;
use serde::{Deserialize, Serialize};

use super::bus::{Bus, BusTap};
use super::clipboard::ClipboardData;
use super::component::{Component, LibraryCellInstance};
use super::component_type::ComponentType;
#[cfg(test)]
use super::design_note::DesignNote;
use super::document::SchematicDocument;
#[cfg(test)]
use super::documentation_shape::DocumentationShape;
#[cfg(test)]
use super::net_label::{Junction, NetLabel};
#[cfg(test)]
use super::rotation::Rotation;
use super::selection::Selection;
#[cfg(test)]
use super::snap::SnapEngine;
use super::tool::Tool;
#[cfg(test)]
use rspice_design::schematic::wire::{Wire, WireConnection};
use rspice_design_model::Point;

mod components;
mod editor_ops;
mod hierarchy_ops;
mod identity;
mod junction_ops;
mod selection_ops;
#[cfg(test)]
mod serialization_tests;
mod stimulus_placement;
mod undo;
mod viewport;

pub use stimulus_placement::PendingStimulusPlacement;

// =============================================================================
// Constants
// =============================================================================

pub use rspice_design::schematic::movement::{MoveSelectionError, MoveSelectionMode};

pub use rspice_design::schematic::stretch::{
    StretchOrthogonalPolicy, StretchSelectionError, StretchTarget,
};

// =============================================================================
// SchematicState
// =============================================================================

/// Schematic editor state over one persisted design document.
#[derive(Debug, Clone)]
pub struct SchematicState {
    pub(in crate::state::schematic) design: rspice_design::schematic::owned::Schematic,

    pub session: SchematicSession,
}

/// Borrow a stored design with the session belonging to the same document.
#[derive(Clone, Copy)]
pub(crate) struct SchematicEditorRef<'a> {
    pub(crate) design: &'a rspice_design::schematic::owned::Schematic,
    pub(crate) session: Option<&'a SchematicSession>,
}

impl<'a> SchematicEditorRef<'a> {
    pub(crate) fn document(&self) -> &'a SchematicDocument {
        self.design.document()
    }

    pub(crate) fn clone_editor(&self) -> SchematicState {
        self.with_design(self.design.clone())
    }

    pub(crate) fn with_design(
        self,
        design: rspice_design::schematic::owned::Schematic,
    ) -> SchematicState {
        SchematicState::from_parts(design, self.session.cloned().unwrap_or_default())
    }

    pub(crate) fn is_dirty(&self) -> bool {
        self.session.is_some_and(|session| session.is_dirty)
    }

    pub(crate) fn read_only(&self) -> bool {
        self.session.is_some_and(|session| session.read_only)
    }
}

impl AsRef<SchematicDocument> for SchematicEditorRef<'_> {
    fn as_ref(&self) -> &SchematicDocument {
        self.document()
    }
}

/// Exclusive editor access to a stored design and its per-document session.
/// Both values return to their borrowed slots when editing ends, including unwind.
pub(crate) struct SchematicEditorMut<'a> {
    pub(crate) editor: SchematicState,
    design: &'a mut rspice_design::schematic::owned::Schematic,
    session: &'a mut SchematicSession,
}

impl Drop for SchematicEditorMut<'_> {
    fn drop(&mut self) {
        std::mem::swap(self.design, &mut self.editor.design);
        std::mem::swap(self.session, &mut self.editor.session);
    }
}

impl SchematicState {
    pub(crate) fn editor_ref(&self) -> SchematicEditorRef<'_> {
        SchematicEditorRef {
            design: &self.design,
            session: Some(&self.session),
        }
    }

    pub(crate) fn from_parts(
        design: rspice_design::schematic::owned::Schematic,
        session: SchematicSession,
    ) -> Self {
        Self { design, session }
    }

    pub(crate) fn into_parts(
        self,
    ) -> (rspice_design::schematic::owned::Schematic, SchematicSession) {
        (self.design, self.session)
    }

    pub(crate) fn borrow_parts<'a>(
        design: &'a mut rspice_design::schematic::owned::Schematic,
        session: &'a mut SchematicSession,
    ) -> SchematicEditorMut<'a> {
        let editor = Self::from_parts(std::mem::take(design), std::mem::take(session));
        SchematicEditorMut {
            editor,
            design,
            session,
        }
    }
}

/// App bookkeeping around the per-document editor session.
#[derive(Debug, Clone, Default)]
pub struct SchematicSession {
    pub editor: EditorSession,

    /// Current schematic file path (for save without dialog)
    pub current_file: Option<PathBuf>,

    /// Flag indicating unsaved changes (runtime state, not persisted)
    pub is_dirty: bool,

    /// The open view belongs to a read-only library — inspection only.
    /// Set by the workspace loader; every edit path refuses while it holds.
    pub read_only: bool,

    /// Flag indicating the undo history should be reset (e.g., after loading a file).
    /// Set to true when a file is loaded, cleared after history is reset.
    pub needs_history_reset: bool,

    /// Editor selection and dirty baseline for the current document transaction.
    pub(in crate::state::schematic) operation_cancel:
        Option<super::undo_history::OperationCancelState>,
}

impl Default for SchematicState {
    fn default() -> Self {
        Self::from_document(SchematicDocument::default())
    }
}

impl SchematicState {
    /// Read-only access to persisted schematic content.
    pub(crate) fn document(&self) -> &SchematicDocument {
        self.design.document()
    }

    pub(crate) fn document_and_editor(&mut self) -> (&SchematicDocument, &mut EditorSession) {
        (self.design.document(), &mut self.session.editor)
    }

    /// Fixtures can model invalid or externally changed content without a
    /// mutable document accessor in production.
    #[cfg(test)]
    pub(crate) fn document_mut_for_test(&mut self) -> &mut SchematicDocument {
        self.design.document_mut_for_test()
    }

    /// Create fresh editor state around an owned document. Saved-file loading
    /// retains its separate legacy runtime defaults in Deserialize.
    pub(crate) fn from_document(document: SchematicDocument) -> Self {
        let mut session = SchematicSession::default();
        session.editor.snap_engine.grid_size = document.grid_size;
        Self {
            design: rspice_design::schematic::owned::Schematic::from_document(document),
            session,
        }
    }
}

// Delegate the design owner's wire layout without copying content. Runtime
// fields retain their original serde(skip) defaults when loading saved data.
impl Serialize for SchematicState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.design.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for SchematicState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self {
            design: rspice_design::schematic::owned::Schematic::deserialize(deserializer)?,
            session: SchematicSession::default(),
        })
    }
}

impl SchematicState {
    /// Reconcile every in-document runtime projection of the authoritative
    /// project-portable grid pitch.
    ///
    /// Call this after restoring history or installing session-owned snap
    /// target preferences into a newly activated document.
    pub(crate) fn reconcile_grid_pitch_runtime(&mut self) {
        let grid_size = self.design.reconcile_grid_pitch();
        self.session.editor.snap_engine.grid_size = grid_size;
    }
}

impl SchematicState {
    pub fn history(&self) -> &rspice_design::schematic::history::SchematicHistory {
        self.design.history()
    }
    pub(crate) fn clear_schematic_redo(&mut self) {
        self.design.clear_redo();
    }
    pub(crate) fn set_live_sheet_assignments(
        &mut self,
        assignments: std::collections::BTreeMap<u64, crate::state::SheetId>,
    ) {
        self.design.set_live_sheet_assignments(assignments);
    }
    pub(crate) fn take_restored_sheet_assignments(
        &mut self,
    ) -> std::collections::BTreeMap<u64, crate::state::SheetId> {
        self.design.take_restored_sheet_assignments()
    }
    pub(crate) fn set_pending_was_dirty(&mut self, was_dirty: bool) {
        if let Some(cancel) = &mut self.session.operation_cancel
            && Some(cancel.operation_id()) == self.design.pending_operation_id()
        {
            cancel.set_was_dirty(was_dirty);
        }
    }
}

impl AsRef<SchematicDocument> for SchematicState {
    fn as_ref(&self) -> &SchematicDocument {
        self.document()
    }
}

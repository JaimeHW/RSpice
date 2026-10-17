//! The armed Move or Stretch preview, kept across frames while the transform
//! and what it acts on stand still.
//!
//! A preview applies the transform to a copy of the whole document and then
//! compares the copy with the original. Done every frame, that was a deep
//! copy and a full comparison per repaint while the pointer rested.

use std::sync::Arc;

use rspice_design::schematic::{
    movement::MoveSelectionMode,
    selection::Selection,
    stretch::{StretchOrthogonalPolicy, StretchTarget},
};
use rspice_design_model::{Point, design_management::SheetId};
use rspice_schematic_editor::view::preview::TransformCandidateDiff;

/// What a canvas derivation reads from the drawing besides its own inputs:
/// the drawing as it stands, the symbols that give its pins and bounds, and
/// the sheet whose objects it may reach. Shared by every memo that holds
/// something derived from the open drawing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DrawingInputs {
    pub(crate) topology_version: u64,
    pub(crate) content_version: u64,
    pub(crate) symbol_revision: u64,
    pub(crate) active_sheet: Option<SheetId>,
    pub(crate) sheet_catalog_revision: Option<u64>,
}

/// Everything one transform preview was computed from.
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TransformPreviewKey {
    Move {
        delta: Point,
        mode: MoveSelectionMode,
        selection: Selection,
        drawing: DrawingInputs,
    },
    Stretch {
        delta: Point,
        target: StretchTarget,
        policy: StretchOrthogonalPolicy,
        drawing: DrawingInputs,
    },
}

/// What the preview found: the changes to paint, `None` when the transform
/// would change nothing, or why it is refused.
pub(crate) type TransformPreviewOutcome = Result<Option<Arc<TransformCandidateDiff>>, String>;

#[derive(Clone, Default)]
pub struct TransformPreviewMemo {
    entry: Option<(TransformPreviewKey, TransformPreviewOutcome)>,
}

impl std::fmt::Debug for TransformPreviewMemo {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("TransformPreviewMemo")
            .field("key", &self.entry.as_ref().map(|(key, _)| key))
            .finish()
    }
}

impl TransformPreviewMemo {
    /// The outcome computed from exactly `key`, if it is held.
    pub(crate) fn current(&self, key: &TransformPreviewKey) -> Option<TransformPreviewOutcome> {
        self.entry
            .as_ref()
            .filter(|(held, _)| held == key)
            .map(|(_, outcome)| outcome.clone())
    }

    pub(crate) fn store(&mut self, key: TransformPreviewKey, outcome: TransformPreviewOutcome) {
        self.entry = Some((key, outcome));
    }
}

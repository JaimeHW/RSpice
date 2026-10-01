//! Editor intents bound to the source view that produced them.

use rspice_app_types::product::ProjectId;
use rspice_design::occurrence::DocumentOccurrence;
use rspice_design::schematic::selection::Selection;
use rspice_design_model::{cell_view::CellViewRef, design_management::SheetId};

use crate::session::selection::SchematicKeyboardFocus;
use crate::view::pointer_target::PointerTarget;

/// Expected source identity, not permission to mutate the design.
/// The app compares this with its current owners before applying a request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorRequestSource {
    pub project: ProjectId,
    pub document: CellViewRef,
    pub occurrence: Option<DocumentOccurrence>,
    pub design_epoch: u64,
    pub document_epoch: u64,
    pub content_version: u64,
    pub topology_version: u64,
    pub symbol_revision: u64,
    pub sheet: Option<(Option<SheetId>, u64)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EditorAction {
    DeleteSelection,
    Focus(SchematicKeyboardFocus),
    SelectPointer {
        target: Option<PointerTarget>,
        additive: bool,
        alt_held: bool,
    },
}

#[derive(Debug, Clone)]
pub struct EditorRequest {
    pub source: EditorRequestSource,
    pub selection: Selection,
    pub action: EditorAction,
}

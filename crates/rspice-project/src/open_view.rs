//! Persisted open-document identity, occurrence, and access marking.

use rspice_design::{library::ViewType, occurrence::DocumentOccurrence};
use rspice_design_model::cell_view::CellViewRef;
use serde::{Deserialize, Serialize};

/// One open view tab in the workspace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenCellView {
    pub reference: CellViewRef,
    pub view_type: ViewType,
    pub dirty: bool,
    /// The exact occurrence this document is editing. Its terminal master is
    /// always `reference`; saves written before documents owned an occurrence
    /// deserialize unrooted and are rooted at `reference` on load.
    #[serde(default)]
    pub occurrence: DocumentOccurrence,
    /// Whether this document was opened as a read-only hierarchy reference.
    /// The marking belongs to the tab, so returning to it later still refuses
    /// writes.
    #[serde(default)]
    pub read_only_reference: bool,
}

impl OpenCellView {
    pub fn new(reference: CellViewRef, view_type: ViewType) -> Self {
        Self {
            occurrence: DocumentOccurrence::rooted(reference.clone()),
            reference,
            view_type,
            dirty: false,
            read_only_reference: false,
        }
    }
}

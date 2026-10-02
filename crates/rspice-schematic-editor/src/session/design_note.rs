//! Armed design-note placement data and its expected document identity.

use crate::requests::EditorRequestSource;

use rspice_design::schematic::design_note::{
    DesignNote, DesignNoteError, DesignNoteKind, DesignNoteLayer, normalize_design_note_text,
    validate_kind_text,
};

/// Frozen, validated one-shot placement payload. The UI also binds the
/// canonical source so a delayed click cannot mutate a different document or occurrence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingDesignNotePlacement {
    pub kind: DesignNoteKind,
    pub text: String,
    pub layer: DesignNoteLayer,
    pub topology_version: u64,
    /// Exact non-electrical document content frozen when the tool is armed.
    /// Topology versioning deliberately ignores notes, so this closes the
    /// concurrent note-edit gap without treating documentation as electrical.
    pub expected_design_notes: Vec<DesignNote>,
    pub source: Option<EditorRequestSource>,
}

impl PendingDesignNotePlacement {
    pub fn new(
        kind: DesignNoteKind,
        text: impl Into<String>,
        topology_version: u64,
        expected_design_notes: &[DesignNote],
    ) -> Result<Self, DesignNoteError> {
        let text = normalize_design_note_text(&text.into())?;
        validate_kind_text(kind, &text)?;
        Ok(Self {
            kind,
            text,
            layer: DesignNoteLayer::DrawingAnnotation,
            topology_version,
            expected_design_notes: expected_design_notes.to_vec(),
            source: None,
        })
    }

    pub fn with_source(mut self, source: EditorRequestSource) -> Self {
        self.source = Some(source);
        self
    }
}

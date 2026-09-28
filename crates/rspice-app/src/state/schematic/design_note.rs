//! Design-note placement authority and undo-bound editing.

use super::{Point, SchematicState};
pub use rspice_design::schematic::design_note::{
    DesignNote, DesignNoteError, DesignNoteKind, DesignNoteLayer, DesignNoteRenderContext,
    DesignReviewMutation, DesignReviewState, RequirementTarget,
};
use rspice_design::schematic::design_note::{normalize_design_note_text, validate_kind_text};

/// Frozen, validated one-shot placement payload. The UI also binds the
/// application epochs and active cell/view so a delayed click cannot mutate a
/// different document.
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
    pub document_authority: Option<DesignNotePlacementAuthority>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DesignNotePlacementAuthority {
    pub design_execution_epoch: u64,
    pub active_schematic_epoch: u64,
    pub view_path: String,
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
            document_authority: None,
        })
    }

    pub fn with_document_authority(
        mut self,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        view_path: String,
    ) -> Self {
        self.document_authority = Some(DesignNotePlacementAuthority {
            design_execution_epoch,
            active_schematic_epoch,
            view_path,
        });
        self
    }
}

impl SchematicState {
    pub fn validate_pending_design_note(
        &self,
        pending: &PendingDesignNotePlacement,
    ) -> Result<(), DesignNoteError> {
        if self.session.read_only {
            return Err(DesignNoteError::ReadOnly);
        }
        if pending.topology_version != self.topology_version() {
            return Err(DesignNoteError::StaleDocument);
        }
        if pending.expected_design_notes != self.design.document().design_notes {
            return Err(DesignNoteError::StaleDocument);
        }
        if pending.layer != DesignNoteLayer::DrawingAnnotation {
            return Err(DesignNoteError::InvalidLayer);
        }
        let normalized = normalize_design_note_text(&pending.text)?;
        if normalized != pending.text {
            return Err(DesignNoteError::NonCanonicalText);
        }
        validate_kind_text(pending.kind, &pending.text)
    }

    /// Commit one design note as one undo transaction. The object is
    /// intentionally excluded from electrical connectivity collections.
    pub fn place_pending_design_note(
        &mut self,
        pos: Point,
        pending: PendingDesignNotePlacement,
    ) -> Result<u64, DesignNoteError> {
        self.validate_pending_design_note(&pending)?;
        let edit = self
            .design
            .place_design_note(pos, pending.kind, pending.text)?;
        self.session.selection.clear();
        self.session.selection.select_design_note(edit.value);
        self.finish_document_edit(edit.committed);
        if edit.committed {
            Ok(edit.value)
        } else {
            Err(DesignNoteError::ReadOnly)
        }
    }

    /// Apply one review mutation against an exact note snapshot. This closes
    /// delayed-dialog and cross-document races without treating comments as
    /// electrical topology changes.
    pub fn apply_design_review_mutation(
        &mut self,
        note_id: u64,
        expected_design_notes: &[DesignNote],
        mutation: DesignReviewMutation,
    ) -> Result<(), DesignNoteError> {
        if self.session.read_only {
            return Err(DesignNoteError::ReadOnly);
        }
        let committed =
            self.design
                .apply_design_review_mutation(note_id, expected_design_notes, mutation)?;
        self.finish_document_edit(committed);
        if committed {
            Ok(())
        } else {
            Err(DesignNoteError::ReadOnly)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::ProjectId;
    use crate::state::{ValidatedRevisionRequest, ValidationFindingCounts};

    #[test]
    fn review_mutations_are_atomic_stale_guarded_and_undoable() {
        let mut schematic = SchematicState::default();
        schematic.design.document_mut_for_test().design_notes.push(
            DesignNote::new(
                42,
                Point::origin(),
                DesignNoteKind::ReviewNote,
                "Confirm the bias model",
            )
            .unwrap(),
        );
        let expected = schematic.design.document().design_notes.clone();
        schematic
            .apply_design_review_mutation(
                42,
                &expected,
                DesignReviewMutation::Assign {
                    assignee: Some("M. Chen".to_owned()),
                },
            )
            .unwrap();
        assert_eq!(
            schematic.design.document().design_notes[0]
                .review
                .as_ref()
                .unwrap()
                .assignee
                .as_deref(),
            Some("M. Chen")
        );
        assert!(schematic.undo());
        assert!(
            schematic.design.document().design_notes[0]
                .review
                .as_ref()
                .unwrap()
                .assignee
                .is_none()
        );

        schematic.design.document_mut_for_test().design_notes[0].text =
            "Concurrent edit".to_owned();
        let before = schematic.design.document().design_notes.clone();
        assert_eq!(
            schematic.apply_design_review_mutation(
                42,
                &expected,
                DesignReviewMutation::Reply {
                    author: "J. Whitfield".to_owned(),
                    body: "This must not commit.".to_owned(),
                    created_unix_ms: 103,
                },
            ),
            Err(DesignNoteError::StaleDocument)
        );
        assert_eq!(schematic.design.document().design_notes, before);
    }

    #[test]
    fn placement_is_non_electrical_and_one_undo_record() {
        let mut schematic = SchematicState::default();
        let pending = PendingDesignNotePlacement::new(
            DesignNoteKind::PlainText,
            "Bias network",
            schematic.topology_version(),
            &schematic.design.document().design_notes,
        )
        .unwrap();
        let topology = schematic.topology_version();
        let id = schematic
            .place_pending_design_note(Point::new(4, 7), pending)
            .unwrap();
        assert_eq!(schematic.design.document().design_notes.len(), 1);
        assert_eq!(schematic.design.document().design_notes[0].id, id);
        assert!(schematic.design.document().components.is_empty());
        assert!(schematic.design.document().wires.is_empty());
        assert_eq!(schematic.topology_version(), topology);
        assert!(schematic.can_undo());
        assert!(schematic.undo());
        assert!(schematic.design.document().design_notes.is_empty());
    }

    #[test]
    fn placed_review_note_anchors_to_latest_validated_revision() {
        let mut schematic = SchematicState::default();
        let validation_receipt_digest = schematic.validated_design_content_digest().unwrap();
        schematic
            .append_validated_revision(ValidatedRevisionRequest {
                project_id: ProjectId::new().to_string(),
                project_revision: 1,
                view_identity: "user/top/schematic".to_owned(),
                revision_note: "Validated review anchor".to_owned(),
                author: "Test engineer".to_owned(),
                validation_receipt_digest,
                finding_counts: ValidationFindingCounts::default(),
                dependencies: Vec::new(),
                advisory_dispositions: Vec::new(),
            })
            .unwrap();
        let expected_anchor = schematic.design.document().validated_revisions.records()[0]
            .revision_digest()
            .to_string();
        let pending = PendingDesignNotePlacement::new(
            DesignNoteKind::ReviewNote,
            "Confirm the retained model binding",
            schematic.topology_version(),
            &schematic.design.document().design_notes,
        )
        .unwrap();
        schematic
            .place_pending_design_note(Point::new(4, 7), pending)
            .unwrap();
        assert_eq!(
            schematic.design.document().design_notes[0]
                .review
                .as_ref()
                .and_then(|review| review.anchored_revision.as_deref()),
            Some(expected_anchor.as_str())
        );
    }

    #[test]
    fn armed_note_contract_rejects_non_electrical_document_drift() {
        let mut schematic = SchematicState::default();
        let pending = PendingDesignNotePlacement::new(
            DesignNoteKind::PlainText,
            "Bias network",
            schematic.topology_version(),
            &schematic.design.document().design_notes,
        )
        .unwrap();
        schematic.design.document_mut_for_test().design_notes.push(
            DesignNote::new(
                99,
                Point::new(1, 1),
                DesignNoteKind::PlainText,
                "Concurrent note",
            )
            .unwrap(),
        );

        assert_eq!(
            schematic.validate_pending_design_note(&pending),
            Err(DesignNoteError::StaleDocument)
        );
    }
}

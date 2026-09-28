//! Durable annotation placement, review and candidate repair operations.
use super::super::{
    design_note::{DesignNote, DesignNoteError, DesignNoteKind, DesignReviewMutation},
    documentation_shape::{
        DocumentationShape, DocumentationShapeError, DocumentationShapeGeometry,
    },
    net_label::NetLabel,
};
use super::{DocumentEdit, Schematic};
use rspice_design_model::Point;
impl Schematic {
    pub fn place_design_note(
        &mut self,
        pos: Point,
        kind: DesignNoteKind,
        text: String,
    ) -> Result<DocumentEdit<u64>, DesignNoteError> {
        let id = self.allocate_id();
        let mut note = DesignNote::new(id, pos, kind, text)?;
        if kind == DesignNoteKind::ReviewNote {
            let anchor = self
                .document
                .validated_revisions
                .records()
                .last()
                .map(|revision| revision.revision_digest().to_string());
            note.anchor_review_to_revision(anchor)?;
        }
        self.begin_operation("place design note");
        self.document.design_notes.push(note);
        Ok(DocumentEdit {
            value: id,
            committed: self.end_operation(),
        })
    }
    pub fn apply_design_review_mutation(
        &mut self,
        note_id: u64,
        expected_design_notes: &[DesignNote],
        mutation: DesignReviewMutation,
    ) -> Result<bool, DesignNoteError> {
        if self.document.design_notes != expected_design_notes {
            return Err(DesignNoteError::StaleDocument);
        }
        let index = self
            .document
            .design_notes
            .iter()
            .position(|note| note.id == note_id)
            .ok_or(DesignNoteError::ReviewRecordNotFound)?;
        let mut candidate = self.document.design_notes[index].clone();
        mutation.clone().apply(&mut candidate)?;
        candidate.validate()?;
        let label = mutation.undo_label();
        self.begin_operation(label);
        self.document.design_notes[index] = candidate;
        Ok(self.end_operation())
    }
    pub fn place_documentation_shape(
        &mut self,
        geometry: DocumentationShapeGeometry,
    ) -> Result<DocumentEdit<u64>, DocumentationShapeError> {
        let id = self.allocate_id();
        let shape = DocumentationShape::new(id, geometry)?;
        self.begin_operation("draw documentation shape");
        self.document.documentation_shapes.push(shape);
        Ok(DocumentEdit {
            value: id,
            committed: self.end_operation(),
        })
    }
    pub fn set_placed_port_contract(&mut self, id: u64, name: &str, params: &str) {
        let component = self
            .document
            .components
            .iter_mut()
            .find(|component| component.id == id)
            .expect("newly allocated port exists");
        name.clone_into(&mut component.value);
        params.clone_into(&mut component.params);
    }
    pub fn repair_candidate_net_labels(
        &mut self,
        canonical: &str,
        labels: &[(u64, String)],
    ) -> Result<(), String> {
        NetLabel::validate_name(canonical, self.document.document_policy.net_naming)
            .map_err(|error| format!("Canonical global name is invalid: {error}."))?;
        for (id, expected) in labels {
            let label = self
                .document
                .net_labels
                .iter_mut()
                .find(|label| label.id == *id)
                .ok_or_else(|| format!("Net label #{id} no longer exists."))?;
            if label.name != *expected {
                return Err(format!(
                    "Net label #{id} changed from '{expected}' to '{}'. Refresh the report.",
                    label.name
                ));
            }
            canonical.clone_into(&mut label.name);
        }
        Ok(())
    }
}

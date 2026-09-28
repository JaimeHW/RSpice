//! Guarded property edits and their document-local history boundaries.

use crate::state::{
    DesignNote, DesignNoteKind, DesignReviewState, DocumentationShape, DocumentationShapeGeometry,
    NetLabel, Point, SchematicGridPitch, SchematicState,
};

impl SchematicState {
    pub(crate) fn edit_net_label_properties(
        &mut self,
        expected: NetLabel,
        name: String,
        position: Point,
    ) -> Result<bool, String> {
        let read_only = self.session.read_only;
        let Some(change) = self
            .design
            .prepare_net_label_properties(expected, name, position)?
        else {
            return Ok(false);
        };
        if read_only {
            return Ok(false);
        }
        let committed = change.commit();
        self.finish_document_edit(committed);
        Ok(committed)
    }

    pub(crate) fn edit_design_note_properties(
        &mut self,
        expected: DesignNote,
        kind: DesignNoteKind,
        text: String,
        review_state: Option<DesignReviewState>,
    ) -> Result<bool, String> {
        let read_only = self.session.read_only;
        let Some(change) =
            self.design
                .prepare_design_note_properties(expected, kind, text, review_state)?
        else {
            return Ok(false);
        };
        if read_only {
            return Ok(false);
        }
        let committed = change.commit();
        self.finish_document_edit(committed);
        Ok(committed)
    }

    pub(crate) fn edit_documentation_shape_properties(
        &mut self,
        expected: DocumentationShape,
        geometry: DocumentationShapeGeometry,
    ) -> Result<bool, String> {
        let read_only = self.session.read_only;
        let Some(change) = self
            .design
            .prepare_documentation_shape_properties(expected, geometry)?
        else {
            return Ok(false);
        };
        if read_only {
            return Ok(false);
        }
        let committed = change.commit();
        self.finish_document_edit(committed);
        Ok(committed)
    }

    pub(crate) fn rename_net_label(
        &mut self,
        expected: NetLabel,
        name: String,
    ) -> Result<bool, String> {
        let read_only = self.session.read_only;
        let Some(change) = self.design.prepare_net_label_rename(expected, name)? else {
            return Ok(false);
        };
        if read_only {
            return Ok(false);
        }
        let committed = change.commit();
        self.finish_document_edit(committed);
        Ok(committed)
    }

    pub(crate) fn change_grid_pitch(&mut self, pitch: SchematicGridPitch) -> bool {
        if self.session.read_only {
            return false;
        }
        let committed = self.design.change_grid_pitch(pitch);
        self.finish_document_edit(committed);
        committed
    }
}

impl SchematicState {
    pub(crate) fn bind_component_value(
        &mut self,
        component_id: u64,
        binding_expression: String,
        description: String,
    ) -> bool {
        if self.session.read_only {
            return false;
        }
        let committed =
            self.design
                .bind_component_value(component_id, binding_expression, description);
        self.finish_document_edit(committed);
        committed
    }
}

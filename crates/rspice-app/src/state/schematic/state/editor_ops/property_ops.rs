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
        let Some(current) = self
            .document
            .net_labels
            .iter()
            .find(|label| label.id == expected.id)
        else {
            return Err("The selected net label no longer exists.".to_owned());
        };
        if current != &expected {
            return Err("The selected net label changed before commit.".to_owned());
        }
        if expected.name == name && expected.pos == position {
            return Ok(false);
        }
        Ok(
            self.with_undo("edit net label properties", move |schematic| {
                if let Some(label) = schematic
                    .document
                    .net_labels
                    .iter_mut()
                    .find(|label| label.id == expected.id)
                {
                    label.name = name;
                    label.pos = position;
                    schematic.is_dirty = true;
                    schematic.bump_topology_version();
                }
            }),
        )
    }

    pub(crate) fn edit_design_note_properties(
        &mut self,
        expected: DesignNote,
        kind: DesignNoteKind,
        text: String,
        review_state: Option<DesignReviewState>,
    ) -> Result<bool, String> {
        let Some(current) = self
            .document
            .design_notes
            .iter()
            .find(|note| note.id == expected.id)
        else {
            return Err("The selected design note no longer exists.".to_owned());
        };
        if current != &expected {
            return Err("The selected design note changed before commit.".to_owned());
        }
        let mut candidate = expected.clone();
        candidate
            .update(kind, text)
            .map_err(|error| error.to_string())?;
        if let Some(review_state) = review_state {
            candidate
                .set_review_state(review_state)
                .map_err(|error| error.to_string())?;
        }
        if candidate == expected {
            return Ok(false);
        }
        Ok(
            self.with_undo("edit design note properties", move |schematic| {
                if let Some(note) = schematic
                    .document
                    .design_notes
                    .iter_mut()
                    .find(|note| note.id == expected.id)
                {
                    *note = candidate;
                    schematic.is_dirty = true;
                }
            }),
        )
    }

    pub(crate) fn edit_documentation_shape_properties(
        &mut self,
        expected: DocumentationShape,
        geometry: DocumentationShapeGeometry,
    ) -> Result<bool, String> {
        let Some(current) = self
            .document
            .documentation_shapes
            .iter()
            .find(|shape| shape.id == expected.id)
        else {
            return Err("The selected documentation shape no longer exists.".to_owned());
        };
        if current != &expected {
            return Err("The selected documentation shape changed before commit.".to_owned());
        }
        let candidate =
            DocumentationShape::new(expected.id, geometry).map_err(|error| error.to_string())?;
        if candidate == expected {
            return Ok(false);
        }
        Ok(
            self.with_undo("edit documentation shape properties", move |schematic| {
                if let Some(shape) = schematic
                    .document
                    .documentation_shapes
                    .iter_mut()
                    .find(|shape| shape.id == expected.id)
                {
                    *shape = candidate;
                    schematic.is_dirty = true;
                }
            }),
        )
    }

    pub(crate) fn rename_net_label(
        &mut self,
        expected: NetLabel,
        name: String,
    ) -> Result<bool, String> {
        let Some(current) = self
            .document
            .net_labels
            .iter()
            .find(|label| label.id == expected.id)
        else {
            return Err("The selected net label no longer exists.".to_owned());
        };
        if current != &expected {
            return Err("The selected net label changed before commit.".to_owned());
        }
        Ok(self.with_undo("rename net label", move |schematic| {
            if let Some(label) = schematic
                .document
                .net_labels
                .iter_mut()
                .find(|label| label.id == expected.id)
            {
                label.name = name;
                schematic.is_dirty = true;
                schematic.bump_topology_version();
            }
        }))
    }

    pub(crate) fn change_grid_pitch(&mut self, pitch: SchematicGridPitch) -> bool {
        self.with_undo("change schematic grid pitch", |document| {
            document.document.document_policy.grid_pitch = pitch;
            document.document.grid_size = pitch.canvas_grid_size();
            document.is_dirty = true;
            document.bump_topology_version();
        })
    }
}

impl SchematicState {
    pub(crate) fn bind_component_value(
        &mut self,
        component_id: u64,
        binding_expression: String,
        description: String,
    ) -> bool {
        self.with_undo(description, move |schematic| {
            let index = schematic
                .document
                .components
                .iter()
                .position(|component| component.id == component_id)
                .expect("staged component identity was validated before the transaction");
            schematic.document.components[index].value = binding_expression;
            schematic.is_dirty = true;
            schematic.bump_topology_version();
        })
    }
}

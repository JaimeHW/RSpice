//! Property candidates validate before an editor opens their document scope.
use super::super::{
    design_note::{DesignNote, DesignNoteKind, DesignReviewState},
    document_policy::SchematicGridPitch,
    documentation_shape::{DocumentationShape, DocumentationShapeGeometry},
    net_label::NetLabel,
};
use super::Schematic;
use rspice_design_model::Point;

enum PropertyChange {
    Label {
        id: u64,
        name: String,
        position: Point,
    },
    LabelName {
        id: u64,
        name: String,
    },
    Note(DesignNote),
    Shape(DocumentationShape),
}
pub struct PreparedPropertyEdit<'a> {
    design: &'a mut Schematic,
    change: PropertyChange,
    description: &'static str,
}
impl PreparedPropertyEdit<'_> {
    pub fn commit(self) -> bool {
        let Self {
            design,
            change,
            description,
        } = self;
        design.begin_operation(description);
        match change {
            PropertyChange::Label { id, name, position } => {
                if let Some(label) = design
                    .document
                    .net_labels
                    .iter_mut()
                    .find(|label| label.id == id)
                {
                    label.name = name;
                    label.pos = position;
                    design.invalidate_topology();
                }
            }
            PropertyChange::LabelName { id, name } => {
                if let Some(label) = design
                    .document
                    .net_labels
                    .iter_mut()
                    .find(|label| label.id == id)
                {
                    label.name = name;
                    design.invalidate_topology();
                }
            }
            PropertyChange::Note(candidate) => {
                if let Some(note) = design
                    .document
                    .design_notes
                    .iter_mut()
                    .find(|note| note.id == candidate.id)
                {
                    *note = candidate;
                }
            }
            PropertyChange::Shape(candidate) => {
                if let Some(shape) = design
                    .document
                    .documentation_shapes
                    .iter_mut()
                    .find(|shape| shape.id == candidate.id)
                {
                    *shape = candidate;
                }
            }
        }
        design.end_operation()
    }
}
impl Schematic {
    pub fn prepare_net_label_properties(
        &mut self,
        expected: NetLabel,
        name: String,
        position: Point,
    ) -> Result<Option<PreparedPropertyEdit<'_>>, String> {
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
            return Ok(None);
        }
        Ok(Some(PreparedPropertyEdit {
            design: self,
            change: PropertyChange::Label {
                id: expected.id,
                name,
                position,
            },
            description: "edit net label properties",
        }))
    }
    pub fn prepare_design_note_properties(
        &mut self,
        expected: DesignNote,
        kind: DesignNoteKind,
        text: String,
        review_state: Option<DesignReviewState>,
    ) -> Result<Option<PreparedPropertyEdit<'_>>, String> {
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
            return Ok(None);
        }
        Ok(Some(PreparedPropertyEdit {
            design: self,
            change: PropertyChange::Note(candidate),
            description: "edit design note properties",
        }))
    }
    pub fn prepare_documentation_shape_properties(
        &mut self,
        expected: DocumentationShape,
        geometry: DocumentationShapeGeometry,
    ) -> Result<Option<PreparedPropertyEdit<'_>>, String> {
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
            return Ok(None);
        }
        Ok(Some(PreparedPropertyEdit {
            design: self,
            change: PropertyChange::Shape(candidate),
            description: "edit documentation shape properties",
        }))
    }
    pub fn prepare_net_label_rename(
        &mut self,
        expected: NetLabel,
        name: String,
    ) -> Result<Option<PreparedPropertyEdit<'_>>, String> {
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
        Ok(Some(PreparedPropertyEdit {
            design: self,
            change: PropertyChange::LabelName {
                id: expected.id,
                name,
            },
            description: "rename net label",
        }))
    }
    pub fn change_grid_pitch(&mut self, pitch: SchematicGridPitch) -> bool {
        self.begin_operation("change schematic grid pitch");
        self.document.document_policy.grid_pitch = pitch;
        self.document.grid_size = pitch.canvas_grid_size();
        self.invalidate_topology();
        self.end_operation()
    }
    pub fn bind_component_value(
        &mut self,
        component_id: u64,
        binding_expression: String,
        description: String,
    ) -> bool {
        self.begin_operation(description);
        let index = self
            .document
            .components
            .iter()
            .position(|component| component.id == component_id)
            .expect("staged component identity was validated before the transaction");
        self.document.components[index].value = binding_expression;
        self.invalidate_topology();
        self.end_operation()
    }
}

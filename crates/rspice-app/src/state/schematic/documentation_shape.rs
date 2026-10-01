//! Documentation-shape placement authority and unfinished editor gestures.

#[cfg(test)]
use super::Point;
use super::SchematicState;
pub use rspice_design::schematic::documentation_shape::{
    DocumentationShape, DocumentationShapeError, DocumentationShapeGeometry,
    DocumentationShapeKind, DocumentationShapeLayer, arc_parameters, geometry_from_points,
};

pub use rspice_schematic_editor::session::documentation_shape::PendingDocumentationShapePlacement;

impl SchematicState {
    pub fn validate_pending_documentation_shape(
        &self,
        pending: &PendingDocumentationShapePlacement,
    ) -> Result<(), DocumentationShapeError> {
        if self.session.read_only {
            return Err(DocumentationShapeError::ReadOnly);
        }
        if pending.topology_version != self.topology_version()
            || pending.expected_shapes != self.design.document().documentation_shapes
        {
            return Err(DocumentationShapeError::StaleDocument);
        }
        if pending.layer != DocumentationShapeLayer::DrawingDocumentation {
            return Err(DocumentationShapeError::InvalidLayer);
        }
        Ok(())
    }

    pub fn commit_documentation_shape(
        &mut self,
        pending: PendingDocumentationShapePlacement,
        geometry: DocumentationShapeGeometry,
    ) -> Result<u64, DocumentationShapeError> {
        self.validate_pending_documentation_shape(&pending)?;
        if geometry.kind() != pending.kind {
            return Err(DocumentationShapeError::DegenerateGeometry);
        }
        let edit = self.design.place_documentation_shape(geometry)?;
        self.session.editor.selection.clear();
        self.session
            .editor
            .selection
            .select_documentation_shape(edit.value);
        self.finish_document_edit(edit.committed);
        if edit.committed {
            Ok(edit.value)
        } else {
            Err(DocumentationShapeError::ReadOnly)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn placement_is_non_electrical_and_one_undo_record() {
        let mut schematic = SchematicState::default();
        let topology = schematic.topology_version();
        let pending = PendingDocumentationShapePlacement::new(
            DocumentationShapeKind::Rectangle,
            topology,
            &schematic.design.document().documentation_shapes,
        );
        let id = schematic
            .commit_documentation_shape(
                pending,
                DocumentationShapeGeometry::Rectangle {
                    first: Point::origin(),
                    opposite: Point::new(20, 10),
                },
            )
            .unwrap();
        assert_eq!(schematic.design.document().documentation_shapes[0].id, id);
        assert!(schematic.design.document().components.is_empty());
        assert!(schematic.design.document().wires.is_empty());
        assert_eq!(schematic.topology_version(), topology);
        assert!(schematic.undo());
        assert!(schematic.design.document().documentation_shapes.is_empty());
    }

    #[test]
    fn armed_contract_rejects_documentation_shape_drift() {
        let mut schematic = SchematicState::default();
        let pending = PendingDocumentationShapePlacement::new(
            DocumentationShapeKind::Line,
            schematic.topology_version(),
            &schematic.design.document().documentation_shapes,
        );
        schematic
            .design
            .document_mut_for_test()
            .documentation_shapes
            .push(
                DocumentationShape::new(
                    44,
                    DocumentationShapeGeometry::Line {
                        start: Point::origin(),
                        end: Point::new(1, 1),
                    },
                )
                .unwrap(),
            );
        assert_eq!(
            schematic.validate_pending_documentation_shape(&pending),
            Err(DocumentationShapeError::StaleDocument)
        );
    }
}

//! Documentation-shape placement authority and unfinished editor gestures.

use super::{Point, SchematicState};
use rspice_design::schematic::documentation_shape::MAX_DOCUMENTATION_POLYGON_POINTS;
pub use rspice_design::schematic::documentation_shape::{
    DocumentationShape, DocumentationShapeError, DocumentationShapeGeometry,
    DocumentationShapeKind, DocumentationShapeLayer, arc_parameters, geometry_from_points,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentationShapePlacementAuthority {
    pub design_execution_epoch: u64,
    pub active_schematic_epoch: u64,
    pub view_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingDocumentationShapePlacement {
    pub kind: DocumentationShapeKind,
    pub layer: DocumentationShapeLayer,
    pub topology_version: u64,
    pub expected_shapes: Vec<DocumentationShape>,
    pub document_authority: Option<DocumentationShapePlacementAuthority>,
}

impl PendingDocumentationShapePlacement {
    pub fn new(
        kind: DocumentationShapeKind,
        topology_version: u64,
        expected_shapes: &[DocumentationShape],
    ) -> Self {
        Self {
            kind,
            layer: DocumentationShapeLayer::DrawingDocumentation,
            topology_version,
            expected_shapes: expected_shapes.to_vec(),
            document_authority: None,
        }
    }

    pub fn with_document_authority(
        mut self,
        design_execution_epoch: u64,
        active_schematic_epoch: u64,
        view_path: String,
    ) -> Self {
        self.document_authority = Some(DocumentationShapePlacementAuthority {
            design_execution_epoch,
            active_schematic_epoch,
            view_path,
        });
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DocumentationShapeDrawing {
    pub points: Vec<Point>,
    /// Grid-resolved placement cursor used when the focused canvas is driven
    /// without a pointing device.
    pub keyboard_cursor: Option<Point>,
    pub keyboard_active: bool,
}

impl DocumentationShapeDrawing {
    pub fn clear(&mut self) {
        self.points.clear();
        self.keyboard_cursor = None;
        self.keyboard_active = false;
    }

    pub fn add_point(
        &mut self,
        kind: DocumentationShapeKind,
        point: Point,
    ) -> Result<bool, DocumentationShapeError> {
        if self.points.last() == Some(&point) {
            return Ok(false);
        }
        if kind == DocumentationShapeKind::Polygon
            && self.points.len() >= MAX_DOCUMENTATION_POLYGON_POINTS
        {
            return Err(DocumentationShapeError::TooManyPoints);
        }
        self.points.push(point);
        Ok(kind.commits_automatically() && self.points.len() == kind.minimum_points())
    }

    pub fn geometry(
        &self,
        kind: DocumentationShapeKind,
    ) -> Result<DocumentationShapeGeometry, DocumentationShapeError> {
        geometry_from_points(kind, &self.points)
    }
}

impl SchematicState {
    pub fn validate_pending_documentation_shape(
        &self,
        pending: &PendingDocumentationShapePlacement,
    ) -> Result<(), DocumentationShapeError> {
        if self.read_only {
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
        self.selection.clear();
        self.selection.select_documentation_shape(edit.value);
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

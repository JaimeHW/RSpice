//! Armed documentation-shape placement and unfinished drawing gestures.

use rspice_design::schematic::documentation_shape::{
    DocumentationShape, DocumentationShapeError, DocumentationShapeGeometry,
    DocumentationShapeKind, DocumentationShapeLayer, MAX_DOCUMENTATION_POLYGON_POINTS,
    geometry_from_points,
};
use rspice_design_model::Point;

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

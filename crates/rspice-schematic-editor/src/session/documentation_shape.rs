//! Armed documentation-shape placement and unfinished drawing gestures.

use crate::requests::EditorRequestSource;

use rspice_design::schematic::documentation_shape::{
    DocumentationShape, DocumentationShapeError, DocumentationShapeGeometry,
    DocumentationShapeKind, DocumentationShapeLayer, MAX_DOCUMENTATION_POLYGON_POINTS,
    geometry_from_points,
};
use rspice_design_model::Point;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingDocumentationShapePlacement {
    pub kind: DocumentationShapeKind,
    pub layer: DocumentationShapeLayer,
    pub topology_version: u64,
    pub expected_shapes: Vec<DocumentationShape>,
    pub source: Option<EditorRequestSource>,
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
            source: None,
        }
    }

    pub fn with_source(mut self, source: EditorRequestSource) -> Self {
        self.source = Some(source);
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

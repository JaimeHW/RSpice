//! Local canvas draft for an armed schematic transform.

use rspice_design_model::Point;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TransformCanvasSession {
    pub anchor: Option<Point>,
    pub preview_delta: Point,
    pub pointer_drag: bool,
    pub preview_error: Option<String>,
}

impl TransformCanvasSession {
    /// A rejected candidate leaves its feedback visible while accepting a new gesture.
    pub fn reset_candidate(&mut self) {
        self.anchor = None;
        self.preview_delta = Point::origin();
        self.pointer_drag = false;
    }
}

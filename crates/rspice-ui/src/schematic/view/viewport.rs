use egui::{Pos2, Rect};

use crate::state::Point;

/// Viewport transformation helper
pub(super) struct Viewport {
    pub(super) offset: Pos2,
    pub(super) zoom: f32,
    pub(super) bounds: Rect,
}

impl Viewport {
    /// Convert schematic coordinates to screen coordinates
    pub(super) fn schematic_to_screen(&self, point: Point) -> Pos2 {
        Pos2::new(
            self.bounds.min.x + self.offset.x + (point.x as f32) * self.zoom,
            self.bounds.min.y + self.offset.y + (point.y as f32) * self.zoom,
        )
    }

    /// The visible region in schematic coordinates, expanded by `margin`
    /// world units on every side (labels and symbol bodies overhang their
    /// anchor points). Used for viewport culling.
    pub(super) fn visible_world_rect(&self, margin: f32) -> (f32, f32, f32, f32) {
        let inv = 1.0 / self.zoom.max(f32::EPSILON);
        let x0 = -self.offset.x * inv;
        let y0 = -self.offset.y * inv;
        let x1 = (self.bounds.width() - self.offset.x) * inv;
        let y1 = (self.bounds.height() - self.offset.y) * inv;
        (x0 - margin, y0 - margin, x1 + margin, y1 + margin)
    }
}

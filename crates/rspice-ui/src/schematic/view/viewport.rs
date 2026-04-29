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
}

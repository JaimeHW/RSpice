//! Persisted view state.
//!
//! The pan, zoom, and grid a schematic reopens at.

use super::*;

impl SchematicState {
    // =========================================================================
    // Viewport Management
    // =========================================================================

    /// Zoom to fit all schematic content in the viewport.
    ///
    /// Sets zoom and pan so all components and wires are visible with comfortable margins.
    ///
    /// Parameters:
    /// - `viewport_width`: Width of the schematic canvas in pixels
    /// - `viewport_height`: Height of the schematic canvas in pixels
    pub fn zoom_to_fit(&mut self, viewport_width: f64, viewport_height: f64) {
        // Calculate bounding box of all content (in schematic pixel coordinates)
        self.zoom_to_fit_bounds(self.content_bounds(), viewport_width, viewport_height);
    }

    pub(crate) fn zoom_to_fit_bounds(
        &mut self,
        bounds: Option<(i32, i32, i32, i32)>,
        viewport_width: f64,
        viewport_height: f64,
    ) {
        let Some((min_x, min_y, max_x, max_y)) = bounds else {
            // No content - reset to default view
            self.zoom = 1.0;
            self.pan = (0.0, 0.0);
            return;
        };

        // content_bounds returns schematic pixel coordinates, not grid cell indices
        // So we use them directly without multiplying by grid_size
        let min_px = min_x as f64;
        let min_py = min_y as f64;
        let max_px = max_x as f64;
        let max_py = max_y as f64;

        // Content size in schematic pixels
        let content_width = (max_px - min_px).max(1.0);
        let content_height = (max_py - min_py).max(1.0);

        // Add margin (10% of content size, minimum 50 pixels) for a comfortable fit
        let margin = (content_width.max(content_height) * 0.10).max(50.0);

        let total_width = content_width + margin * 2.0;
        let total_height = content_height + margin * 2.0;

        // Calculate zoom to fit (use the smaller scale to ensure everything fits)
        let zoom_x = viewport_width / total_width;
        let zoom_y = viewport_height / total_height;
        let fit_zoom = zoom_x.min(zoom_y);

        // Use the same 25%–800% contract as direct canvas zoom. Keeping one
        // range avoids a fit operation silently producing a scale that the
        // wheel and toolbar cannot subsequently reach.
        self.zoom = fit_zoom.clamp(0.25, 8.0);

        // Calculate pan to center the content in the viewport
        // Screen position formula: screen = bounds.min + pan + schematic * zoom
        // We want the center of content to appear at center of viewport:
        // viewport_width/2 = pan + center_schematic * zoom
        // pan = viewport_width/2 - center_schematic * zoom
        let center_schematic_x = (min_px + max_px) / 2.0;
        let center_schematic_y = (min_py + max_py) / 2.0;

        self.pan = (
            viewport_width / 2.0 - center_schematic_x * self.zoom,
            viewport_height / 2.0 - center_schematic_y * self.zoom,
        );

        log::debug!(
            "zoom_to_fit: content=[{:.0},{:.0}]-[{:.0},{:.0}], viewport={:.0}x{:.0}, zoom={:.2}, pan=({:.0},{:.0})",
            min_px,
            min_py,
            max_px,
            max_py,
            viewport_width,
            viewport_height,
            self.zoom,
            self.pan.0,
            self.pan.1
        );
    }

    /// Frame an exact world-space rectangle with a stable screen-space inset.
    ///
    /// Drawing-sheet paper uses physical dimensions which are not necessarily
    /// integral schematic units (for example, US Letter is 215.9 mm wide).
    /// Keeping this path in `f64` avoids coercing those edges while preserving
    /// the editor's shared 25%-800% zoom contract.
    pub(crate) fn zoom_to_fit_world_rect(
        &mut self,
        bounds: (f64, f64, f64, f64),
        viewport_width: f64,
        viewport_height: f64,
        screen_inset: f64,
    ) {
        let (min_x, min_y, max_x, max_y) = bounds;
        let width = (max_x - min_x).max(f64::EPSILON);
        let height = (max_y - min_y).max(f64::EPSILON);
        let inset_x = screen_inset
            .max(0.0)
            .min((viewport_width - 1.0).max(0.0) * 0.5);
        let inset_y = screen_inset
            .max(0.0)
            .min((viewport_height - 1.0).max(0.0) * 0.5);
        let usable_width = (viewport_width - inset_x * 2.0).max(1.0);
        let usable_height = (viewport_height - inset_y * 2.0).max(1.0);
        self.zoom = (usable_width / width)
            .min(usable_height / height)
            .clamp(0.25, 8.0);

        let center_x = (min_x + max_x) * 0.5;
        let center_y = (min_y + max_y) * 0.5;
        self.pan = (
            viewport_width * 0.5 - center_x * self.zoom,
            viewport_height * 0.5 - center_y * self.zoom,
        );
    }

    /// Pan so `target` (schematic pixel coordinates) sits at the viewport
    /// center, keeping the current zoom. Same screen mapping as
    /// `zoom_to_fit`: screen = bounds.min + pan + schematic * zoom.
    pub fn center_view_on(&mut self, target: Point, viewport_width: f64, viewport_height: f64) {
        self.pan = (
            viewport_width / 2.0 - f64::from(target.x) * self.zoom,
            viewport_height / 2.0 - f64::from(target.y) * self.zoom,
        );
    }

    /// Calculate the bounding box of all schematic content.
    /// Returns (min_x, min_y, max_x, max_y) in schematic pixel coordinates, or None if empty.
    /// Note: These are pixel coordinates snapped to grid, not grid cell indices.
    pub fn content_bounds(&self) -> Option<(i32, i32, i32, i32)> {
        if self.components.is_empty()
            && self.wires.is_empty()
            && self.junctions.is_empty()
            && self.design_notes.is_empty()
            && self.documentation_shapes.is_empty()
            && self.probes.is_empty()
        {
            return None;
        }

        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;

        // Include component bounds (with approximate size for the symbol)
        for comp in &self.components {
            let (comp_min_x, comp_min_y, comp_max_x, comp_max_y) = comp.bounding_box();
            min_x = min_x.min(comp_min_x);
            min_y = min_y.min(comp_min_y);
            max_x = max_x.max(comp_max_x);
            max_y = max_y.max(comp_max_y);
        }

        // Include wire endpoints
        for wire in &self.wires {
            for point in &wire.points {
                min_x = min_x.min(point.x);
                min_y = min_y.min(point.y);
                max_x = max_x.max(point.x);
                max_y = max_y.max(point.y);
            }
        }

        // Include junctions
        for junction in &self.junctions {
            min_x = min_x.min(junction.pos.x);
            min_y = min_y.min(junction.pos.y);
            max_x = max_x.max(junction.pos.x);
            max_y = max_y.max(junction.pos.y);
        }

        for note in &self.design_notes {
            let lines = note.text.lines().count().max(1) as i32;
            let columns = note
                .text
                .lines()
                .map(|line| line.chars().count())
                .max()
                .unwrap_or(1)
                .min(i32::MAX as usize) as i32;
            min_x = min_x.min(note.pos.x);
            min_y = min_y.min(note.pos.y);
            max_x = max_x.max(note.pos.x.saturating_add(columns.saturating_mul(8)));
            max_y = max_y.max(note.pos.y.saturating_add(lines.saturating_mul(15)));
        }

        for shape in &self.documentation_shapes {
            let (min, max) = shape.bounds();
            min_x = min_x.min(min.x);
            min_y = min_y.min(min.y);
            max_x = max_x.max(max.x);
            max_y = max_y.max(max.y);
        }

        for probe in &self.probes {
            let (min, max) = probe.world_bounds();
            min_x = min_x.min(min.x);
            min_y = min_y.min(min.y);
            max_x = max_x.max(max.x);
            max_y = max_y.max(max.y);
        }

        Some((min_x, min_y, max_x, max_y))
    }
}

#[cfg(test)]
mod tests {
    use super::SchematicState;

    #[test]
    fn physical_world_rect_fit_preserves_fractional_edges_and_screen_inset() {
        let mut state = SchematicState::default();
        state.zoom_to_fit_world_rect((-140.0, -40.0, 723.6, 1077.6), 1200.0, 800.0, 24.0);

        let left = state.pan.0 + -140.0 * state.zoom;
        let top = state.pan.1 + -40.0 * state.zoom;
        let right = state.pan.0 + 723.6 * state.zoom;
        let bottom = state.pan.1 + 1077.6 * state.zoom;

        assert!(left >= 24.0 - 1.0e-9);
        assert!(top >= 24.0 - 1.0e-9);
        assert!(right <= 1200.0 - 24.0 + 1.0e-9);
        assert!(bottom <= 800.0 - 24.0 + 1.0e-9);
        assert!(((left + right) * 0.5 - 600.0).abs() < 1.0e-9);
        assert!(((top + bottom) * 0.5 - 400.0).abs() < 1.0e-9);
    }
}

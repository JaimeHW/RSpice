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
        let Some((min_x, min_y, max_x, max_y)) = self.content_bounds() else {
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

        // Clamp zoom to reasonable bounds (0.25x to 4x)
        self.zoom = fit_zoom.clamp(0.25, 4.0);

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
        if self.components.is_empty() && self.wires.is_empty() && self.junctions.is_empty() {
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

        Some((min_x, min_y, max_x, max_y))
    }
}

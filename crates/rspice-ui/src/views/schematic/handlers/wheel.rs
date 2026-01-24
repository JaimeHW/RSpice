//! Wheel Event Handlers for Schematic Editor
//!
//! Provides testable zoom handling logic for the schematic editor.
//! Zoom follows industry-standard behavior with cursor-centered scaling.

/// Zoom parameters for the schematic editor
pub struct ZoomConfig {
    /// Minimum zoom level (zoomed out)
    pub min_zoom: f64,
    /// Maximum zoom level (zoomed in)
    pub max_zoom: f64,
    /// Zoom multiplier per scroll step
    pub zoom_factor: f64,
}

impl Default for ZoomConfig {
    fn default() -> Self {
        Self {
            min_zoom: 0.1,
            max_zoom: 10.0,
            zoom_factor: 1.1,
        }
    }
}

/// Calculate new zoom level and pan offset for cursor-centered zoom
///
/// This provides standard CAD tool behavior where the point under the
/// cursor stays in place while the zoom level changes.
///
/// # Arguments
/// * `wheel_delta` - Wheel delta (positive = zoom in, negative = zoom out)
/// * `current_zoom` - Current zoom level
/// * `current_pan` - Current pan offset (px, py)
/// * `cursor_screen` - Cursor position in screen pixels (x, y)
/// * `config` - Zoom configuration parameters
///
/// # Returns
/// (new_zoom, new_pan) tuple
pub fn calculate_cursor_centered_zoom(
    wheel_delta: f64,
    current_zoom: f64,
    current_pan: (f64, f64),
    cursor_screen: (f64, f64),
    config: &ZoomConfig,
) -> (f64, (f64, f64)) {
    let direction = if wheel_delta > 0.0 { 1.0 } else { -1.0 };
    let factor = config.zoom_factor.powf(direction);

    // Calculate new zoom, clamped to limits
    let new_zoom = (current_zoom * factor).clamp(config.min_zoom, config.max_zoom);

    // If zoom didn't change (hit limit), keep current pan
    if (new_zoom - current_zoom).abs() < 1e-10 {
        return (current_zoom, current_pan);
    }

    let (cx, cy) = cursor_screen;
    let (px, py) = current_pan;

    // Calculate the world point under cursor before zoom
    let world_x = (cx - px) / current_zoom;
    let world_y = (cy - py) / current_zoom;

    // Calculate new pan to keep that world point under cursor
    let new_px = cx - world_x * new_zoom;
    let new_py = cy - world_y * new_zoom;

    (new_zoom, (new_px, new_py))
}

/// Calculate zoom level for "fit to view" operation
///
/// Calculates zoom and pan to fit all content with padding.
///
/// # Arguments
/// * `content_bounds` - (min_x, min_y, max_x, max_y) in grid units
/// * `viewport_size` - (width, height) in pixels
/// * `grid_size` - Grid size in pixels
/// * `padding_fraction` - Fraction of viewport to leave as padding (0.1 = 10%)
///
/// # Returns
/// (zoom, pan) to center and fit content
pub fn calculate_fit_to_view(
    content_bounds: (i32, i32, i32, i32),
    viewport_size: (f64, f64),
    grid_size: i32,
    padding_fraction: f64,
) -> (f64, (f64, f64)) {
    let (min_x, min_y, max_x, max_y) = content_bounds;
    let (vw, vh) = viewport_size;

    // Calculate content size in pixels (at zoom 1.0)
    let content_w = ((max_x - min_x + 1) * grid_size) as f64;
    let content_h = ((max_y - min_y + 1) * grid_size) as f64;

    if content_w <= 0.0 || content_h <= 0.0 {
        return (1.0, (0.0, 0.0));
    }

    // Calculate zoom to fit with padding
    let usable_w = vw * (1.0 - padding_fraction * 2.0);
    let usable_h = vh * (1.0 - padding_fraction * 2.0);

    let zoom = (usable_w / content_w).min(usable_h / content_h).max(0.1);

    // Calculate pan to center content
    let content_center_x = ((min_x + max_x) as f64 / 2.0) * grid_size as f64;
    let content_center_y = ((min_y + max_y) as f64 / 2.0) * grid_size as f64;

    let pan_x = vw / 2.0 - content_center_x * zoom;
    let pan_y = vh / 2.0 - content_center_y * zoom;

    (zoom, (pan_x, pan_y))
}

/// Predefined zoom levels for zoom-to-level feature
pub const ZOOM_LEVELS: [f64; 9] = [0.25, 0.5, 0.75, 1.0, 1.5, 2.0, 3.0, 4.0, 6.0];

/// Get the next zoom level in the predefined list
///
/// # Arguments
/// * `current_zoom` - Current zoom level
/// * `zoom_in` - True to find next higher level, false for lower
///
/// # Returns
/// Next zoom level, or current if at limit
pub fn next_zoom_level(current_zoom: f64, zoom_in: bool) -> f64 {
    if zoom_in {
        ZOOM_LEVELS
            .iter()
            .find(|&&z| z > current_zoom + 0.01)
            .copied()
            .unwrap_or(*ZOOM_LEVELS.last().unwrap())
    } else {
        ZOOM_LEVELS
            .iter()
            .rev()
            .find(|&&z| z < current_zoom - 0.01)
            .copied()
            .unwrap_or(*ZOOM_LEVELS.first().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // =============================================================================
    // Cursor-Centered Zoom Tests
    // =============================================================================

    #[test]
    fn test_zoom_in_increases_zoom() {
        let config = ZoomConfig::default();
        let (new_zoom, _) = calculate_cursor_centered_zoom(
            1.0, // positive = zoom in
            1.0,
            (0.0, 0.0),
            (400.0, 300.0),
            &config,
        );
        assert!(new_zoom > 1.0);
    }

    #[test]
    fn test_zoom_out_decreases_zoom() {
        let config = ZoomConfig::default();
        let (new_zoom, _) = calculate_cursor_centered_zoom(
            -1.0, // negative = zoom out
            1.0,
            (0.0, 0.0),
            (400.0, 300.0),
            &config,
        );
        assert!(new_zoom < 1.0);
    }

    #[test]
    fn test_zoom_clamped_to_min() {
        let config = ZoomConfig {
            min_zoom: 0.5,
            max_zoom: 2.0,
            zoom_factor: 1.1,
        };
        // Start at min, try to zoom out
        let (new_zoom, _) =
            calculate_cursor_centered_zoom(-100.0, 0.5, (0.0, 0.0), (400.0, 300.0), &config);
        assert!((new_zoom - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_zoom_clamped_to_max() {
        let config = ZoomConfig {
            min_zoom: 0.5,
            max_zoom: 2.0,
            zoom_factor: 1.1,
        };
        // Start at max, try to zoom in
        let (new_zoom, _) =
            calculate_cursor_centered_zoom(100.0, 2.0, (0.0, 0.0), (400.0, 300.0), &config);
        assert!((new_zoom - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_cursor_stays_under_point_after_zoom() {
        let config = ZoomConfig::default();
        let cursor_screen = (200.0, 150.0);
        let current_zoom = 1.0;
        let current_pan = (50.0, 50.0);

        // Calculate world point under cursor before zoom
        let world_x_before = (cursor_screen.0 - current_pan.0) / current_zoom;
        let world_y_before = (cursor_screen.1 - current_pan.1) / current_zoom;

        let (new_zoom, new_pan) =
            calculate_cursor_centered_zoom(1.0, current_zoom, current_pan, cursor_screen, &config);

        // Calculate world point under cursor after zoom
        let world_x_after = (cursor_screen.0 - new_pan.0) / new_zoom;
        let world_y_after = (cursor_screen.1 - new_pan.1) / new_zoom;

        // World point should be the same
        assert!((world_x_before - world_x_after).abs() < 0.01);
        assert!((world_y_before - world_y_after).abs() < 0.01);
    }

    // =============================================================================
    // Fit to View Tests
    // =============================================================================

    #[test]
    fn test_fit_to_view_centers_content() {
        let (zoom, (px, py)) = calculate_fit_to_view(
            (0, 0, 10, 10), // 11x11 grid units
            (800.0, 600.0), // viewport
            10,             // grid size
            0.1,            // 10% padding
        );

        // Zoom should be positive and reasonable
        assert!(zoom > 0.0);
        assert!(zoom < 10.0);

        // Content should be roughly centered
        // (exact values depend on content/viewport ratio)
        assert!(px.abs() < 500.0);
        assert!(py.abs() < 500.0);
    }

    #[test]
    fn test_fit_to_view_empty_content() {
        let (zoom, pan) = calculate_fit_to_view(
            (0, 0, -1, -1), // invalid bounds (empty)
            (800.0, 600.0),
            10,
            0.1,
        );

        // Should return default
        assert_eq!(zoom, 1.0);
        assert_eq!(pan, (0.0, 0.0));
    }

    // =============================================================================
    // Zoom Levels Tests
    // =============================================================================

    #[test]
    fn test_next_zoom_level_up() {
        assert_eq!(next_zoom_level(1.0, true), 1.5);
        assert_eq!(next_zoom_level(0.5, true), 0.75);
    }

    #[test]
    fn test_next_zoom_level_down() {
        assert_eq!(next_zoom_level(1.0, false), 0.75);
        assert_eq!(next_zoom_level(2.0, false), 1.5);
    }

    #[test]
    fn test_next_zoom_level_at_max() {
        assert_eq!(next_zoom_level(6.0, true), 6.0);
    }

    #[test]
    fn test_next_zoom_level_at_min() {
        assert_eq!(next_zoom_level(0.25, false), 0.25);
    }

    #[test]
    fn test_zoom_levels_are_sorted() {
        for i in 1..ZOOM_LEVELS.len() {
            assert!(ZOOM_LEVELS[i] > ZOOM_LEVELS[i - 1]);
        }
    }

    // =============================================================================
    // Config Tests
    // =============================================================================

    #[test]
    fn test_default_config_valid() {
        let config = ZoomConfig::default();
        assert!(config.min_zoom > 0.0);
        assert!(config.max_zoom > config.min_zoom);
        assert!(config.zoom_factor > 1.0);
    }
}

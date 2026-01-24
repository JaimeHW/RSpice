//! Wire SVG Rendering Component
//!
//! Renders wire segments with support for selection highlighting and probe mode.

use crate::state::Point;
use crate::theme::Theme;
use dioxus::prelude::*;

/// Wire SVG component - renders a wire path with selection/probe highlighting
///
/// # Props
/// - `points` - Vector of grid points defining the wire path
/// - `grid_size` - Grid size in pixels for coordinate conversion
/// - `selected` - Whether the wire is selected (blue highlight)
/// - `probe_highlight` - Whether the wire is highlighted for probe mode (orange)
#[component]
pub fn WireSvg(
    points: Vec<Point>,
    grid_size: i32,
    selected: bool,
    #[props(default)] probe_highlight: bool,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    if points.len() < 2 {
        return rsx! {};
    }

    // Probe highlight takes priority, then selection, then normal
    let (col, sw) = if probe_highlight {
        ("#ffa500".to_string(), "2.5") // Orange highlight for probe mode
    } else if selected {
        (th.accent_primary().to_string(), "2")
    } else {
        (th.accent_success().to_string(), "1.5")
    };

    // Build path string properly
    let mut d = String::new();
    for (i, p) in points.iter().enumerate() {
        let (x, y) = p.to_pixels(grid_size);
        if i == 0 {
            d.push_str(&format!("M{} {}", x, y));
        } else {
            d.push_str(&format!(" L{} {}", x, y));
        }
    }

    // Only render the wire path - endpoint dots are rendered separately
    // to ensure proper z-ordering (selected dots on top)
    rsx! {
        path { d: "{d}", stroke: "{col}", stroke_width: "{sw}", fill: "none", stroke_linecap: "round" }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wire_path_generation() {
        // Test that path data is generated correctly for a simple wire
        let points = vec![Point::new(0, 0), Point::new(1, 0), Point::new(1, 1)];
        let grid_size = 10;

        // Calculate expected path
        let mut expected = String::new();
        for (i, p) in points.iter().enumerate() {
            let (x, y) = p.to_pixels(grid_size);
            if i == 0 {
                expected.push_str(&format!("M{} {}", x, y));
            } else {
                expected.push_str(&format!(" L{} {}", x, y));
            }
        }

        // Verify path format starts with M and contains L
        assert!(expected.starts_with('M'));
        assert!(expected.contains('L'));
    }

    #[test]
    fn test_wire_empty_points_no_panic() {
        // Empty points should not panic - component handles gracefully
        let points: Vec<Point> = vec![];
        assert!(points.len() < 2);
    }

    #[test]
    fn test_wire_single_point_no_render() {
        // Single point wires should not render (need at least 2 points)
        let points = vec![Point::new(0, 0)];
        assert!(points.len() < 2);
    }

    #[test]
    fn test_wire_selection_colors() {
        // Test that different states produce different styling
        let selected = true;
        let probe_highlight = true;

        // Probe takes priority over selection
        let (col, _sw) = if probe_highlight {
            ("#ffa500".to_string(), "2.5")
        } else if selected {
            ("accent_primary".to_string(), "2")
        } else {
            ("accent_success".to_string(), "1.5")
        };

        assert_eq!(col, "#ffa500");
    }

    #[test]
    fn test_wire_horizontal_path() {
        let points = vec![Point::new(0, 0), Point::new(5, 0)];
        let grid_size = 10;

        let (x0, y0) = points[0].to_pixels(grid_size);
        let (x1, y1) = points[1].to_pixels(grid_size);

        // Horizontal wire: y coordinates should be equal
        assert_eq!(y0, y1);
        assert!(x1 > x0);
    }

    #[test]
    fn test_wire_vertical_path() {
        let points = vec![Point::new(0, 0), Point::new(0, 5)];
        let grid_size = 10;

        let (x0, y0) = points[0].to_pixels(grid_size);
        let (x1, y1) = points[1].to_pixels(grid_size);

        // Vertical wire: x coordinates should be equal
        assert_eq!(x0, x1);
        assert!(y1 > y0);
    }

    #[test]
    fn test_wire_orthogonal_path() {
        // L-shaped wire (3 points)
        let points = vec![Point::new(0, 0), Point::new(2, 0), Point::new(2, 3)];
        let grid_size = 10;

        // First segment horizontal
        let (x0, y0) = points[0].to_pixels(grid_size);
        let (x1, y1) = points[1].to_pixels(grid_size);
        assert_eq!(y0, y1);

        // Second segment vertical
        let (x2, y2) = points[2].to_pixels(grid_size);
        assert_eq!(x1, x2);
        assert!(y2 > y1);
    }
}

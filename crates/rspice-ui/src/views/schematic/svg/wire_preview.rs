//! Wire Preview SVG Rendering Component
//!
//! Renders the live preview of a wire being drawn, showing the orthogonal
//! routing path from the last committed point to the current cursor position.

use crate::state::SchematicState;
use crate::theme::Theme;
use dioxus::prelude::*;

/// Wire preview SVG - shows orthogonal preview path from last point to cursor
///
/// This component renders a dashed line preview showing where the wire will
/// be placed when the user clicks. It supports orthogonal routing with a
/// corner junction indicator for L-shaped paths.
///
/// # Props
/// - `schematic` - Signal to the schematic state containing wire drawing info
#[component]
pub fn WirePreviewSvg(schematic: Signal<SchematicState>) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let s = schematic.read();
    let preview_path = s.wire_drawing.get_preview_path();
    let gs = s.grid_size;

    if preview_path.len() < 2 {
        return rsx! {};
    }

    // Build SVG path data for orthogonal wire preview
    let path_data: String = preview_path
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let (px, py) = p.to_pixels(gs);
            if i == 0 {
                format!("M{px},{py}")
            } else {
                format!("L{px},{py}")
            }
        })
        .collect();

    let stroke_color = th.accent_primary();

    rsx! {
        path {
            d: "{path_data}",
            stroke: "{stroke_color}",
            stroke_width: "2",
            stroke_dasharray: "4,2",
            fill: "none",
            opacity: "0.7",
        }
        // Show corner junction if path has 3 points (L-shaped)
        if preview_path.len() == 3 {
            {
                let corner = preview_path[1];
                let (cx, cy) = corner.to_pixels(gs);
                rsx! {
                    circle {
                        cx: "{cx}",
                        cy: "{cy}",
                        r: "3",
                        fill: "{stroke_color}",
                        opacity: "0.5",
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::state::Point;

    #[test]
    fn test_preview_path_format() {
        // Test path data format generation
        let points = vec![Point::new(0, 0), Point::new(2, 0), Point::new(2, 2)];
        let gs = 10;

        let path_data: String = points
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let (px, py) = p.to_pixels(gs);
                if i == 0 {
                    format!("M{px},{py}")
                } else {
                    format!("L{px},{py}")
                }
            })
            .collect();

        assert!(path_data.starts_with("M0,0"));
        assert!(path_data.contains("L20,0"));
        assert!(path_data.contains("L20,20"));
    }

    #[test]
    fn test_preview_single_segment() {
        // Two-point path (single segment)
        let points = vec![Point::new(0, 0), Point::new(3, 0)];
        let gs = 10;

        let path_data: String = points
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let (px, py) = p.to_pixels(gs);
                if i == 0 {
                    format!("M{px},{py}")
                } else {
                    format!("L{px},{py}")
                }
            })
            .collect();

        assert_eq!(path_data, "M0,0L30,0");
    }

    #[test]
    fn test_preview_l_shaped_path() {
        // Three-point L-shaped path
        let points = vec![Point::new(0, 0), Point::new(2, 0), Point::new(2, 3)];

        // Should have corner junction
        assert_eq!(points.len(), 3);

        let corner = points[1];
        assert_eq!(corner.x, 2);
        assert_eq!(corner.y, 0);
    }

    #[test]
    fn test_preview_empty_path() {
        // Empty path should not generate path data
        let points: Vec<Point> = vec![];
        assert!(points.len() < 2);
    }

    #[test]
    fn test_preview_corner_position() {
        // Test corner junction position in L-shaped path
        let points = vec![Point::new(1, 1), Point::new(4, 1), Point::new(4, 5)];
        let gs = 10;

        let corner = points[1];
        let (cx, cy) = corner.to_pixels(gs);

        assert_eq!(cx, 40.0);
        assert_eq!(cy, 10.0);
    }

    #[test]
    fn test_preview_negative_coordinates() {
        // Test with negative coordinates
        let points = vec![Point::new(-2, -1), Point::new(0, -1)];
        let gs = 10;

        let (x0, y0) = points[0].to_pixels(gs);
        let (x1, y1) = points[1].to_pixels(gs);

        assert_eq!(x0, -20.0);
        assert_eq!(y0, -10.0);
        assert_eq!(x1, 0.0);
        assert_eq!(y1, -10.0);
    }

    #[test]
    fn test_preview_different_grid_size() {
        // Test with different grid sizes
        let point = Point::new(3, 2);

        let (x5, y5) = point.to_pixels(5);
        let (x15, y15) = point.to_pixels(15);

        assert_eq!(x5, 15.0);
        assert_eq!(y5, 10.0);
        assert_eq!(x15, 45.0);
        assert_eq!(y15, 30.0);
    }
}

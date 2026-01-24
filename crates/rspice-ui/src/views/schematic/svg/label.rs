//! Net Label SVG Rendering Component
//!
//! Renders net labels as flag symbols with the net name.

use crate::state::Point;
use crate::theme::Theme;
use dioxus::prelude::*;

/// Net label SVG component - flag symbol with name
///
/// Renders a net label at the specified grid position showing:
/// - A connection point circle
/// - A flag pole
/// - A flag with the net name
///
/// # Props
/// - `pos` - Grid position of the label
/// - `name` - Net name to display
/// - `grid_size` - Grid size in pixels for coordinate conversion
#[component]
pub fn NetLabelSvg(pos: Point, name: String, grid_size: i32) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let (cx, cy) = pos.to_pixels(grid_size);

    // Calculate text width for background (approximate 7px per character + padding)
    let text_width = (name.len() * 7) as i32 + 10;

    rsx! {
        g { transform: "translate({cx},{cy})",
            // Connection point circle
            circle { cx: "0", cy: "0", r: "3", fill: "{th.accent_primary()}" }
            // Flag pole
            line { x1: "0", y1: "0", x2: "0", y2: "-15", stroke: "{th.accent_primary()}", stroke_width: "2" }
            // Flag background (filled with opacity)
            rect { x: "2", y: "-22", width: "{text_width}", height: "14", rx: "2", fill: "{th.accent_primary()}", opacity: "0.15" }
            // Flag border
            rect { x: "2", y: "-22", width: "{text_width}", height: "14", rx: "2", stroke: "{th.accent_primary()}", stroke_width: "1", fill: "none" }
            // Net name text
            text { x: "6", y: "-12", font_size: "10", fill: "{th.accent_primary()}", font_weight: "600", font_family: "monospace", "{name}" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_text_width_calculation() {
        // Test that text width scales with name length
        let short_name = "A";
        let long_name = "VCC_SUPPLY";

        let short_width = (short_name.len() * 7) as i32 + 10;
        let long_width = (long_name.len() * 7) as i32 + 10;

        assert!(long_width > short_width);
        assert_eq!(short_width, 17); // 1*7 + 10
        assert_eq!(long_width, 80); // 10*7 + 10
    }

    #[test]
    fn test_label_position_conversion() {
        let pos = Point::new(5, 3);
        let grid_size = 10;

        let (cx, cy) = pos.to_pixels(grid_size);

        assert_eq!(cx, 50.0); // 5 * 10
        assert_eq!(cy, 30.0); // 3 * 10
    }

    #[test]
    fn test_label_zero_position() {
        let pos = Point::new(0, 0);
        let grid_size = 10;

        let (cx, cy) = pos.to_pixels(grid_size);

        assert_eq!(cx, 0.0);
        assert_eq!(cy, 0.0);
    }

    #[test]
    fn test_label_negative_position() {
        let pos = Point::new(-3, -2);
        let grid_size = 10;

        let (cx, cy) = pos.to_pixels(grid_size);

        assert_eq!(cx, -30.0);
        assert_eq!(cy, -20.0);
    }

    #[test]
    fn test_label_empty_name() {
        // Empty name should still calculate width
        let empty_name = "";
        let width = (empty_name.len() * 7) as i32 + 10;

        assert_eq!(width, 10); // Just the padding
    }

    #[test]
    fn test_label_special_characters() {
        // Net names can contain underscores, numbers
        let name = "NET_01_VDD";
        let width = (name.len() * 7) as i32 + 10;

        assert_eq!(width, 80); // 10*7 + 10
    }

    #[test]
    fn test_label_different_grid_sizes() {
        let pos = Point::new(2, 2);

        let (cx_small, cy_small) = pos.to_pixels(5);
        let (cx_large, cy_large) = pos.to_pixels(20);

        assert_eq!(cx_small, 10.0);
        assert_eq!(cy_small, 10.0);
        assert_eq!(cx_large, 40.0);
        assert_eq!(cy_large, 40.0);
    }
}

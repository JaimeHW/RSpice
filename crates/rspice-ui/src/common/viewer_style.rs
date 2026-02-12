use egui::Color32;

/// Shared background fill for viewer control header bars.
///
/// Keep this aligned with the viewer workspace strip/chip surface so control
/// bars blend with the surrounding panel frame.
pub fn viewer_header_bg_color() -> Color32 {
    Color32::from_rgb(23, 26, 32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewer_header_bg_color_matches_workspace_surface() {
        assert_eq!(viewer_header_bg_color(), Color32::from_rgb(23, 26, 32));
    }
}

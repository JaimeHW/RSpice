/// Configuration for SVG export
#[derive(Debug, Clone)]
pub struct SvgExportConfig {
    /// Grid size in SVG units
    pub grid_size: f64,
    /// Stroke width for wires
    pub wire_stroke_width: f64,
    /// Stroke width for components
    pub component_stroke_width: f64,
    /// Wire color
    pub wire_color: String,
    /// Component color
    pub component_color: String,
    /// Text color
    pub text_color: String,
    /// Font size for labels
    pub font_size: f64,
    /// Include grid in output
    pub include_grid: bool,
    /// Margin around content
    pub margin: f64,
}

impl Default for SvgExportConfig {
    fn default() -> Self {
        Self {
            grid_size: 10.0,
            wire_stroke_width: 2.0,
            component_stroke_width: 2.0,
            wire_color: "#00FF00".to_string(),
            component_color: "#FFFFFF".to_string(),
            text_color: "#AAAAAA".to_string(),
            font_size: 12.0,
            include_grid: false,
            margin: 50.0,
        }
    }
}

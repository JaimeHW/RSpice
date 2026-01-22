//! PDF/Print Export for Schematic Documentation
//!
//! Provides professional schematic export with vector graphics, title blocks,
//! page sizing, and print-quality output. Follows industry standards for
//! engineering documentation output.
//!
//! # Usage
//!
//! ```ignore
//! let config = PdfExportConfig::a4_landscape()
//!     .with_title_block(TitleBlock::new("Project", "Sheet 1"))
//!     .with_margin(15.0);
//!
//! export_schematic_to_pdf(&schematic, &config, "output.pdf")?;
//! ```

use serde::{Deserialize, Serialize};

// =============================================================================
// Page Sizes (ISO and ANSI standards)
// =============================================================================

/// Standard page sizes for schematic export
#[derive(Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize)]
pub enum PageSize {
    // ISO A-series (metric)
    /// A4 (210 × 297 mm)
    #[default]
    A4,
    /// A3 (297 × 420 mm)
    A3,
    /// A2 (420 × 594 mm)
    A2,
    /// A1 (594 × 841 mm)
    A1,
    /// A0 (841 × 1189 mm)
    A0,

    // ANSI series (imperial)
    /// Letter (8.5 × 11 in)
    Letter,
    /// Legal (8.5 × 14 in)
    Legal,
    /// Tabloid (11 × 17 in)
    Tabloid,
    /// ANSI C (17 × 22 in)
    AnsiC,
    /// ANSI D (22 × 34 in)
    AnsiD,
    /// ANSI E (34 × 44 in)
    AnsiE,

    /// Custom size (width, height in mm)
    Custom(f64, f64),
}

impl PageSize {
    /// Get page dimensions in millimeters (width, height) for portrait orientation
    pub fn dimensions_mm(&self) -> (f64, f64) {
        match self {
            PageSize::A4 => (210.0, 297.0),
            PageSize::A3 => (297.0, 420.0),
            PageSize::A2 => (420.0, 594.0),
            PageSize::A1 => (594.0, 841.0),
            PageSize::A0 => (841.0, 1189.0),
            PageSize::Letter => (215.9, 279.4),  // 8.5 × 11 in
            PageSize::Legal => (215.9, 355.6),   // 8.5 × 14 in
            PageSize::Tabloid => (279.4, 431.8), // 11 × 17 in
            PageSize::AnsiC => (431.8, 558.8),   // 17 × 22 in
            PageSize::AnsiD => (558.8, 863.6),   // 22 × 34 in
            PageSize::AnsiE => (863.6, 1117.6),  // 34 × 44 in
            PageSize::Custom(w, h) => (*w, *h),
        }
    }

    /// Get page dimensions in points (72 dpi) for portrait orientation
    pub fn dimensions_pt(&self) -> (f64, f64) {
        let (w_mm, h_mm) = self.dimensions_mm();
        (mm_to_pt(w_mm), mm_to_pt(h_mm))
    }

    /// Get display name
    pub fn name(&self) -> &'static str {
        match self {
            PageSize::A4 => "A4",
            PageSize::A3 => "A3",
            PageSize::A2 => "A2",
            PageSize::A1 => "A1",
            PageSize::A0 => "A0",
            PageSize::Letter => "Letter",
            PageSize::Legal => "Legal",
            PageSize::Tabloid => "Tabloid",
            PageSize::AnsiC => "ANSI C",
            PageSize::AnsiD => "ANSI D",
            PageSize::AnsiE => "ANSI E",
            PageSize::Custom(_, _) => "Custom",
        }
    }
}

/// Page orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Orientation {
    Portrait,
    #[default]
    Landscape,
}

// =============================================================================
// Title Block
// =============================================================================

/// Title block configuration for schematic sheets
///
/// Follows industry standards for engineering documentation with fields for
/// project info, revision history, and approval signatures.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TitleBlock {
    /// Project or company name
    pub project_name: String,
    /// Sheet title or description
    pub sheet_title: String,
    /// Sheet number (e.g., "1 of 3")
    pub sheet_number: String,
    /// Revision number or letter
    pub revision: String,
    /// Date string
    pub date: String,
    /// Designer/drafter name
    pub drawn_by: String,
    /// Checker name
    pub checked_by: String,
    /// Approver name
    pub approved_by: String,
    /// Additional notes or description
    pub notes: String,
    /// Company logo path (optional)
    pub logo_path: Option<String>,
    /// Custom fields as (label, value) pairs
    pub custom_fields: Vec<(String, String)>,
}

impl TitleBlock {
    /// Create a new title block with basic information
    pub fn new(project: &str, title: &str) -> Self {
        Self {
            project_name: project.to_string(),
            sheet_title: title.to_string(),
            ..Default::default()
        }
    }

    /// Set sheet number
    pub fn with_sheet(mut self, number: &str) -> Self {
        self.sheet_number = number.to_string();
        self
    }

    /// Set revision
    pub fn with_revision(mut self, rev: &str) -> Self {
        self.revision = rev.to_string();
        self
    }

    /// Set date
    pub fn with_date(mut self, date: &str) -> Self {
        self.date = date.to_string();
        self
    }

    /// Set drawn by
    pub fn with_drawn_by(mut self, name: &str) -> Self {
        self.drawn_by = name.to_string();
        self
    }

    /// Set current date automatically
    pub fn with_current_date(mut self) -> Self {
        // Use a simple format - in production, use chrono crate
        self.date = "Auto".to_string();
        self
    }

    /// Add custom field
    pub fn with_custom_field(mut self, label: &str, value: &str) -> Self {
        self.custom_fields
            .push((label.to_string(), value.to_string()));
        self
    }

    /// Check if title block is empty (should not be rendered)
    pub fn is_empty(&self) -> bool {
        self.project_name.is_empty() && self.sheet_title.is_empty() && self.sheet_number.is_empty()
    }

    /// Calculate required height for title block in mm
    pub fn height_mm(&self) -> f64 {
        if self.is_empty() {
            0.0
        } else {
            25.0 // Standard title block height
        }
    }
}

// =============================================================================
// Export Configuration
// =============================================================================

/// PDF export configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PdfExportConfig {
    /// Page size
    pub page_size: PageSize,
    /// Page orientation
    pub orientation: Orientation,
    /// Title block configuration
    pub title_block: Option<TitleBlock>,
    /// Page margin in mm
    pub margin_mm: f64,
    /// Whether to include grid lines
    pub show_grid: bool,
    /// Whether to include border/frame
    pub show_border: bool,
    /// Background color (hex string, e.g. "#ffffff")
    pub background_color: String,
    /// Line width scale factor (1.0 = normal)
    pub line_scale: f64,
    /// Text scale factor (1.0 = normal)
    pub text_scale: f64,
    /// DPI for rasterized elements (if any)
    pub dpi: u32,
    /// Whether to embed fonts
    pub embed_fonts: bool,
    /// Color mode
    pub color_mode: ColorMode,
    /// Fit schematic to page (vs actual size)
    pub fit_to_page: bool,
    /// Schematic scale (when not fit to page)
    pub scale: f64,
}

/// Color mode for export
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ColorMode {
    /// Full color
    #[default]
    Color,
    /// Grayscale
    Grayscale,
    /// Black and white (no grays)
    BlackWhite,
}

impl Default for PdfExportConfig {
    fn default() -> Self {
        Self::a4_landscape()
    }
}

impl PdfExportConfig {
    /// Create A4 landscape configuration (common for schematics)
    pub fn a4_landscape() -> Self {
        Self {
            page_size: PageSize::A4,
            orientation: Orientation::Landscape,
            title_block: None,
            margin_mm: 10.0,
            show_grid: false,
            show_border: true,
            background_color: "#ffffff".to_string(),
            line_scale: 1.0,
            text_scale: 1.0,
            dpi: 300,
            embed_fonts: true,
            color_mode: ColorMode::Color,
            fit_to_page: true,
            scale: 1.0,
        }
    }

    /// Create A3 landscape configuration
    pub fn a3_landscape() -> Self {
        Self {
            page_size: PageSize::A3,
            orientation: Orientation::Landscape,
            ..Self::a4_landscape()
        }
    }

    /// Create Letter landscape configuration
    pub fn letter_landscape() -> Self {
        Self {
            page_size: PageSize::Letter,
            orientation: Orientation::Landscape,
            ..Self::a4_landscape()
        }
    }

    /// Set title block
    pub fn with_title_block(mut self, block: TitleBlock) -> Self {
        self.title_block = Some(block);
        self
    }

    /// Set margin
    pub fn with_margin(mut self, margin_mm: f64) -> Self {
        self.margin_mm = margin_mm;
        self
    }

    /// Set line scale
    pub fn with_line_scale(mut self, scale: f64) -> Self {
        self.line_scale = scale;
        self
    }

    /// Enable grid display
    pub fn with_grid(mut self, show: bool) -> Self {
        self.show_grid = show;
        self
    }

    /// Set color mode
    pub fn with_color_mode(mut self, mode: ColorMode) -> Self {
        self.color_mode = mode;
        self
    }

    /// Disable fit-to-page and use fixed scale
    pub fn with_fixed_scale(mut self, scale: f64) -> Self {
        self.fit_to_page = false;
        self.scale = scale;
        self
    }

    /// Get effective page dimensions in mm (accounting for orientation)
    pub fn page_dimensions_mm(&self) -> (f64, f64) {
        let (w, h) = self.page_size.dimensions_mm();
        match self.orientation {
            Orientation::Portrait => (w, h),
            Orientation::Landscape => (h, w),
        }
    }

    /// Get drawable area dimensions in mm (accounting for margins and title block)
    pub fn drawable_area_mm(&self) -> (f64, f64) {
        let (page_w, page_h) = self.page_dimensions_mm();
        let title_height = self.title_block.as_ref().map_or(0.0, |t| t.height_mm());

        (
            page_w - 2.0 * self.margin_mm,
            page_h - 2.0 * self.margin_mm - title_height,
        )
    }

    /// Get effective page dimensions in points
    pub fn page_dimensions_pt(&self) -> (f64, f64) {
        let (w_mm, h_mm) = self.page_dimensions_mm();
        (mm_to_pt(w_mm), mm_to_pt(h_mm))
    }
}

// =============================================================================
// SVG Export
// =============================================================================

/// Generate SVG content for schematic export
///
/// This produces a standalone SVG that can be converted to PDF or used directly.
/// The SVG is vector-based for print quality output.
pub struct SvgExporter {
    config: PdfExportConfig,
    content: String,
}

impl SvgExporter {
    /// Create new SVG exporter with configuration
    pub fn new(config: PdfExportConfig) -> Self {
        Self {
            config,
            content: String::new(),
        }
    }

    /// Generate SVG header
    pub fn begin_document(&mut self) {
        let (w_mm, h_mm) = self.config.page_dimensions_mm();

        self.content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" 
     width="{w_mm}mm" height="{h_mm}mm" 
     viewBox="0 0 {w_mm} {h_mm}">
<defs>
    <style type="text/css">
        .wire {{ stroke: #1a1a1a; stroke-width: 0.35; fill: none; stroke-linecap: round; stroke-linejoin: round; }}
        .component {{ stroke: #1a1a1a; stroke-width: 0.25; fill: none; }}
        .component-fill {{ fill: #ffffff; }}
        .text-label {{ font-family: Arial, sans-serif; font-size: 2.5mm; fill: #333333; }}
        .text-value {{ font-family: 'Courier New', monospace; font-size: 2mm; fill: #666666; }}
        .title-block {{ stroke: #333333; stroke-width: 0.3; fill: #fafafa; }}
        .border {{ stroke: #000000; stroke-width: 0.5; fill: none; }}
        .grid {{ stroke: #e5e5e5; stroke-width: 0.1; }}
    </style>
</defs>
<!-- Background -->
<rect width="100%" height="100%" fill="{bg}"/>
"#,
            bg = self.config.background_color
        );
    }

    /// Add page border
    pub fn add_border(&mut self) {
        if !self.config.show_border {
            return;
        }

        let (w, h) = self.config.page_dimensions_mm();
        let m = self.config.margin_mm;

        self.content.push_str(&format!(
            r#"<rect class="border" x="{m}" y="{m}" width="{}" height="{}"/>
"#,
            w - 2.0 * m,
            h - 2.0 * m
        ));
    }

    /// Add title block
    pub fn add_title_block(&mut self) {
        let title_block = match &self.config.title_block {
            Some(t) if !t.is_empty() => t,
            _ => return,
        };

        let (page_w, page_h) = self.config.page_dimensions_mm();
        let m = self.config.margin_mm;
        let tb_height = title_block.height_mm();
        let tb_y = page_h - m - tb_height;
        let tb_width = page_w - 2.0 * m;

        // Title block background
        self.content.push_str(&format!(
            r#"<rect class="title-block" x="{m}" y="{tb_y}" width="{tb_width}" height="{tb_height}"/>
"#
        ));

        // Title block content
        let text_y = tb_y + 8.0;
        let col1_x = m + 5.0;
        let col2_x = m + tb_width * 0.4;
        let col3_x = m + tb_width * 0.7;

        // Project name (large)
        if !title_block.project_name.is_empty() {
            self.content.push_str(&format!(
                r#"<text x="{col1_x}" y="{text_y}" style="font-size: 4mm; font-weight: bold;">{}</text>
"#,
                escape_xml(&title_block.project_name)
            ));
        }

        // Sheet title
        if !title_block.sheet_title.is_empty() {
            self.content.push_str(&format!(
                r#"<text x="{col1_x}" y="{}" style="font-size: 3mm;">{}</text>
"#,
                text_y + 6.0,
                escape_xml(&title_block.sheet_title)
            ));
        }

        // Sheet number
        if !title_block.sheet_number.is_empty() {
            self.content.push_str(&format!(
                r#"<text x="{col2_x}" y="{text_y}" style="font-size: 2.5mm;">Sheet: {}</text>
"#,
                escape_xml(&title_block.sheet_number)
            ));
        }

        // Revision
        if !title_block.revision.is_empty() {
            self.content.push_str(&format!(
                r#"<text x="{col2_x}" y="{}" style="font-size: 2.5mm;">Rev: {}</text>
"#,
                text_y + 5.0,
                escape_xml(&title_block.revision)
            ));
        }

        // Date
        if !title_block.date.is_empty() {
            self.content.push_str(&format!(
                r#"<text x="{col3_x}" y="{text_y}" style="font-size: 2.5mm;">Date: {}</text>
"#,
                escape_xml(&title_block.date)
            ));
        }

        // Drawn by
        if !title_block.drawn_by.is_empty() {
            self.content.push_str(&format!(
                r#"<text x="{col3_x}" y="{}" style="font-size: 2mm;">Drawn: {}</text>
"#,
                text_y + 5.0,
                escape_xml(&title_block.drawn_by)
            ));
        }
    }

    /// Add grid lines  
    pub fn add_grid(&mut self, grid_spacing_mm: f64) {
        if !self.config.show_grid {
            return;
        }

        let (w, h) = self.config.page_dimensions_mm();
        let m = self.config.margin_mm;

        self.content.push_str("<g class=\"grid\">\n");

        // Vertical lines
        let mut x = m;
        while x <= w - m {
            self.content.push_str(&format!(
                "<line x1=\"{x}\" y1=\"{m}\" x2=\"{x}\" y2=\"{}\"/>\n",
                h - m
            ));
            x += grid_spacing_mm;
        }

        // Horizontal lines
        let mut y = m;
        while y <= h - m {
            self.content.push_str(&format!(
                "<line x1=\"{m}\" y1=\"{y}\" x2=\"{}\" y2=\"{y}\"/>\n",
                w - m
            ));
            y += grid_spacing_mm;
        }

        self.content.push_str("</g>\n");
    }

    /// Add a wire as polyline
    pub fn add_wire(&mut self, points: &[(f64, f64)]) {
        if points.len() < 2 {
            return;
        }

        let points_str: String = points
            .iter()
            .map(|(x, y)| format!("{:.3},{:.3}", x, y))
            .collect::<Vec<_>>()
            .join(" ");

        self.content.push_str(&format!(
            "<polyline class=\"wire\" points=\"{points_str}\"/>\n"
        ));
    }

    /// Add text label
    pub fn add_text(&mut self, x: f64, y: f64, text: &str, class: &str) {
        self.content.push_str(&format!(
            "<text class=\"{class}\" x=\"{x:.3}\" y=\"{y:.3}\">{}</text>\n",
            escape_xml(text)
        ));
    }

    /// Add raw SVG content (for component symbols)
    pub fn add_raw_svg(&mut self, svg: &str) {
        self.content.push_str(svg);
        self.content.push('\n');
    }

    /// Finalize document
    pub fn end_document(&mut self) {
        self.content.push_str("</svg>\n");
    }

    /// Get the generated SVG content
    pub fn svg_content(&self) -> &str {
        &self.content
    }

    /// Export to string
    pub fn export_svg(mut self) -> String {
        self.end_document();
        self.content
    }
}

// =============================================================================
// Utility Functions
// =============================================================================

/// Convert millimeters to points (72 dpi)
pub fn mm_to_pt(mm: f64) -> f64 {
    mm * 72.0 / 25.4
}

/// Convert points to millimeters
pub fn pt_to_mm(pt: f64) -> f64 {
    pt * 25.4 / 72.0
}

/// Convert inches to millimeters
pub fn in_to_mm(inches: f64) -> f64 {
    inches * 25.4
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

/// Calculate scale factor to fit content in available area
pub fn calculate_fit_scale(
    content_width: f64,
    content_height: f64,
    available_width: f64,
    available_height: f64,
) -> f64 {
    let scale_x = available_width / content_width;
    let scale_y = available_height / content_height;
    scale_x.min(scale_y).min(1.0) // Don't scale up, only down
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_page_sizes() {
        assert_eq!(PageSize::A4.dimensions_mm(), (210.0, 297.0));
        assert_eq!(PageSize::Letter.name(), "Letter");
    }

    #[test]
    fn test_orientation() {
        let config = PdfExportConfig::a4_landscape();
        let (w, h) = config.page_dimensions_mm();
        assert!(w > h); // Landscape = wider than tall
    }

    #[test]
    fn test_title_block() {
        let tb = TitleBlock::new("RSpice", "Power Supply")
            .with_revision("A")
            .with_date("2024-01-15")
            .with_drawn_by("Engineer");

        assert_eq!(tb.project_name, "RSpice");
        assert_eq!(tb.revision, "A");
        assert!(!tb.is_empty());
    }

    #[test]
    fn test_drawable_area() {
        let config = PdfExportConfig::a4_landscape()
            .with_margin(10.0)
            .with_title_block(TitleBlock::new("Test", "Sheet"));

        let (w, h) = config.drawable_area_mm();

        // A4 landscape is 297x210, minus 20mm margins = 277x190
        // Minus title block ~25mm
        assert!(w > 270.0);
        assert!(h > 150.0);
    }

    #[test]
    fn test_svg_export() {
        let config = PdfExportConfig::a4_landscape();
        let mut exporter = SvgExporter::new(config);

        exporter.begin_document();
        exporter.add_border();
        exporter.add_wire(&[(10.0, 10.0), (50.0, 10.0), (50.0, 30.0)]);
        exporter.add_text(20.0, 20.0, "R1", "text-label");
        let svg = exporter.export_svg();

        assert!(svg.contains("<svg"));
        assert!(svg.contains("</svg>"));
        assert!(svg.contains("polyline"));
        assert!(svg.contains("R1"));
    }

    #[test]
    fn test_mm_conversions() {
        // 1 inch = 25.4mm, 1 inch = 72pt
        assert!((mm_to_pt(25.4) - 72.0).abs() < 0.1);
        assert!((pt_to_mm(72.0) - 25.4).abs() < 0.1);
    }

    #[test]
    fn test_xml_escaping() {
        assert_eq!(escape_xml("a < b"), "a &lt; b");
        assert_eq!(escape_xml("R&D"), "R&amp;D");
    }
}

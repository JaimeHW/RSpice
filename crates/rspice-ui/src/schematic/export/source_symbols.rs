use std::fmt::Write;

use super::SvgExportConfig;

pub(super) fn write_vsource_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // Circle with + and -
    writeln!(
        svg,
        r#"<circle class="component" cx="{}" cy="{}" r="15"/>"#,
        cx, cy
    )
    .unwrap();
    writeln!(
        svg,
        r#"<text class="text" x="{}" y="{}">+</text>"#,
        cx - 3.0,
        cy - 3.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<text class="text" x="{}" y="{}">−</text>"#,
        cx - 3.0,
        cy + 12.0
    )
    .unwrap();
    // Lead lines
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy - 15.0,
        cx,
        cy - 25.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy + 15.0,
        cx,
        cy + 25.0
    )
    .unwrap();
}

pub(super) fn write_ground_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // Three horizontal lines
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy,
        cx,
        cy + 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 12.0,
        cy + 10.0,
        cx + 12.0,
        cy + 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 8.0,
        cy + 15.0,
        cx + 8.0,
        cy + 15.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy + 20.0,
        cx + 4.0,
        cy + 20.0
    )
    .unwrap();
}

pub(super) fn write_current_source_symbol(
    svg: &mut String,
    cx: f64,
    cy: f64,
    _config: &SvgExportConfig,
) {
    // Current source symbol (circle with arrow)

    // Circle
    writeln!(
        svg,
        r#"<circle class="component" cx="{}" cy="{}" r="15" fill="none"/>"#,
        cx, cy
    )
    .unwrap();

    // Arrow pointing up
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy + 10.0,
        cx,
        cy - 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<polygon class="component" fill="white" points="{},{} {},{} {},{}"/>"#,
        cx,
        cy - 10.0,
        cx - 4.0,
        cy - 4.0,
        cx + 4.0,
        cy - 4.0
    )
    .unwrap();

    // Lead lines
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy - 15.0,
        cx,
        cy - 25.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy + 15.0,
        cx,
        cy + 25.0
    )
    .unwrap();
}

pub(super) fn write_behavioral_source_symbol(
    svg: &mut String,
    cx: f64,
    cy: f64,
    _config: &SvgExportConfig,
) {
    // Behavioral source: circle with an expression (asterisk) glyph.
    writeln!(
        svg,
        r#"<circle class="component" cx="{}" cy="{}" r="15"/>"#,
        cx, cy
    )
    .unwrap();

    // Asterisk: horizontal bar plus two crossing diagonals
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 5.0,
        cy,
        cx + 5.0,
        cy
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 2.5,
        cy - 4.3,
        cx + 2.5,
        cy + 4.3
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 2.5,
        cy + 4.3,
        cx + 2.5,
        cy - 4.3
    )
    .unwrap();

    // Lead lines
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy - 15.0,
        cx,
        cy - 25.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy + 15.0,
        cx,
        cy + 25.0
    )
    .unwrap();
}

// =============================================================================
// JFET Symbols
// =============================================================================

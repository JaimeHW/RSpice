use std::fmt::Write;

use super::SvgExportConfig;

pub(super) fn write_nmos_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // N-channel MOSFET symbol (industry standard)
    // Vertical channel with gate on left, drain at top, source at bottom

    // Gate lead (horizontal line to gate)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 20.0,
        cy,
        cx - 8.0,
        cy
    )
    .unwrap();

    // Gate vertical line
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 8.0,
        cy - 12.0,
        cx - 8.0,
        cy + 12.0
    )
    .unwrap();

    // Channel (three segments)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 12.0,
        cx - 4.0,
        cy - 4.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 2.0,
        cx - 4.0,
        cy + 2.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy + 4.0,
        cx - 4.0,
        cy + 12.0
    )
    .unwrap();

    // Drain connection
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 10.0,
        cx + 8.0,
        cy - 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 8.0,
        cy - 10.0,
        cx + 8.0,
        cy - 20.0
    )
    .unwrap();

    // Source connection
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy + 10.0,
        cx + 8.0,
        cy + 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 8.0,
        cy + 10.0,
        cx + 8.0,
        cy + 20.0
    )
    .unwrap();

    // Arrow pointing into channel (NMOS indicator)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy,
        cx + 4.0,
        cy
    )
    .unwrap();
    // Arrow head
    writeln!(
        svg,
        r#"<polygon class="component" fill="white" points="{},{} {},{} {},{}"/>"#,
        cx + 4.0,
        cy,
        cx,
        cy - 3.0,
        cx,
        cy + 3.0
    )
    .unwrap();
}

pub(super) fn write_pmos_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // P-channel MOSFET symbol (same as NMOS but with bubble on gate)

    // Gate lead with bubble
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 20.0,
        cy,
        cx - 14.0,
        cy
    )
    .unwrap();
    writeln!(
        svg,
        r#"<circle class="component" cx="{}" cy="{}" r="3" fill="none"/>"#,
        cx - 11.0,
        cy
    )
    .unwrap();

    // Gate vertical line
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 8.0,
        cy - 12.0,
        cx - 8.0,
        cy + 12.0
    )
    .unwrap();

    // Channel (three segments)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 12.0,
        cx - 4.0,
        cy - 4.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 2.0,
        cx - 4.0,
        cy + 2.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy + 4.0,
        cx - 4.0,
        cy + 12.0
    )
    .unwrap();

    // Drain and Source connections
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 10.0,
        cx + 8.0,
        cy - 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 8.0,
        cy - 10.0,
        cx + 8.0,
        cy - 20.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy + 10.0,
        cx + 8.0,
        cy + 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 8.0,
        cy + 10.0,
        cx + 8.0,
        cy + 20.0
    )
    .unwrap();

    // Arrow pointing out from channel (PMOS indicator)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy,
        cx + 4.0,
        cy
    )
    .unwrap();
    writeln!(
        svg,
        r#"<polygon class="component" fill="white" points="{},{} {},{} {},{}"/>"#,
        cx - 4.0,
        cy,
        cx,
        cy - 3.0,
        cx,
        cy + 3.0
    )
    .unwrap();
}

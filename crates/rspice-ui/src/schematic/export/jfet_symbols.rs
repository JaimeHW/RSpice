use std::fmt::Write;

use super::SvgExportConfig;

pub(super) fn write_njfet_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // N-channel JFET symbol (industry standard)
    // Arrow points inward from gate for N-channel

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

    // Gate arrow (pointing in for NJFET)
    writeln!(
        svg,
        r#"<polygon class="component" fill="white" points="{},{} {},{} {},{}"/>"#,
        cx - 8.0,
        cy,
        cx - 12.0,
        cy - 3.0,
        cx - 12.0,
        cy + 3.0
    )
    .unwrap();

    // Channel (vertical line)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 15.0,
        cx - 4.0,
        cy + 15.0
    )
    .unwrap();

    // Drain connection (top)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 10.0,
        cx + 10.0,
        cy - 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 10.0,
        cy - 10.0,
        cx + 10.0,
        cy - 20.0
    )
    .unwrap();

    // Source connection (bottom)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy + 10.0,
        cx + 10.0,
        cy + 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 10.0,
        cy + 10.0,
        cx + 10.0,
        cy + 20.0
    )
    .unwrap();
}

pub(super) fn write_pjfet_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // P-channel JFET symbol (arrow points outward from gate)

    // Gate lead
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 20.0,
        cy,
        cx - 8.0,
        cy
    )
    .unwrap();

    // Gate arrow (pointing out for PJFET)
    writeln!(
        svg,
        r#"<polygon class="component" fill="white" points="{},{} {},{} {},{}"/>"#,
        cx - 4.0,
        cy,
        cx - 8.0,
        cy - 3.0,
        cx - 8.0,
        cy + 3.0
    )
    .unwrap();

    // Channel (vertical line)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 15.0,
        cx - 4.0,
        cy + 15.0
    )
    .unwrap();

    // Drain connection (top)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy - 10.0,
        cx + 10.0,
        cy - 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 10.0,
        cy - 10.0,
        cx + 10.0,
        cy - 20.0
    )
    .unwrap();

    // Source connection (bottom)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 4.0,
        cy + 10.0,
        cx + 10.0,
        cy + 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 10.0,
        cy + 10.0,
        cx + 10.0,
        cy + 20.0
    )
    .unwrap();
}

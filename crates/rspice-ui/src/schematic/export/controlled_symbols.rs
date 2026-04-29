use std::fmt::Write;

use super::SvgExportConfig;

pub(super) fn write_vcvs_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // Voltage-Controlled Voltage Source - diamond with + and -
    // Standard controlled source symbol (diamond shape)

    // Diamond shape
    writeln!(
        svg,
        r#"<polygon class="component" fill="none" points="{},{} {},{} {},{} {},{}"/>"#,
        cx,
        cy - 15.0, // top
        cx + 15.0,
        cy, // right
        cx,
        cy + 15.0, // bottom
        cx - 15.0,
        cy // left
    )
    .unwrap();

    // Plus sign (upper half)
    writeln!(
        svg,
        r#"<text class="text" x="{}" y="{}" font-size="12">+</text>"#,
        cx - 4.0,
        cy - 3.0
    )
    .unwrap();

    // Minus sign (lower half)
    writeln!(
        svg,
        r#"<text class="text" x="{}" y="{}" font-size="12">−</text>"#,
        cx - 4.0,
        cy + 10.0
    )
    .unwrap();

    // Lead lines (vertical)
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

pub(super) fn write_vccs_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // Voltage-Controlled Current Source - diamond with arrow

    // Diamond shape
    writeln!(
        svg,
        r#"<polygon class="component" fill="none" points="{},{} {},{} {},{} {},{}"/>"#,
        cx,
        cy - 15.0, // top
        cx + 15.0,
        cy, // right
        cx,
        cy + 15.0, // bottom
        cx - 15.0,
        cy // left
    )
    .unwrap();

    // Arrow inside (pointing up)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx,
        cy + 8.0,
        cx,
        cy - 8.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<polygon class="component" fill="white" points="{},{} {},{} {},{}"/>"#,
        cx,
        cy - 8.0,
        cx - 4.0,
        cy - 2.0,
        cx + 4.0,
        cy - 2.0
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

pub(super) fn write_ccvs_symbol(svg: &mut String, cx: f64, cy: f64, config: &SvgExportConfig) {
    // Current-Controlled Voltage Source - same as VCVS
    write_vcvs_symbol(svg, cx, cy, config);
}

pub(super) fn write_cccs_symbol(svg: &mut String, cx: f64, cy: f64, config: &SvgExportConfig) {
    // Current-Controlled Current Source - same as VCCS
    write_vccs_symbol(svg, cx, cy, config);
}

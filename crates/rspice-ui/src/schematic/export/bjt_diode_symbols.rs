use std::fmt::Write;

use super::SvgExportConfig;

pub(super) fn write_npn_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // NPN BJT symbol (industry standard)

    // Base lead
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 20.0,
        cy,
        cx - 6.0,
        cy
    )
    .unwrap();

    // Emitter base line (vertical)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 6.0,
        cy - 10.0,
        cx - 6.0,
        cy + 10.0
    )
    .unwrap();

    // Collector line (angled up-right)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 6.0,
        cy - 6.0,
        cx + 10.0,
        cy - 18.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 10.0,
        cy - 18.0,
        cx + 10.0,
        cy - 20.0
    )
    .unwrap();

    // Emitter line (angled down-right) with arrow
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 6.0,
        cy + 6.0,
        cx + 10.0,
        cy + 18.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 10.0,
        cy + 18.0,
        cx + 10.0,
        cy + 20.0
    )
    .unwrap();

    // Arrow on emitter (pointing out for NPN)
    writeln!(
        svg,
        r#"<polygon class="component" fill="white" points="{},{} {},{} {},{}"/>"#,
        cx + 10.0,
        cy + 18.0,
        cx + 4.0,
        cy + 14.0,
        cx + 6.0,
        cy + 20.0
    )
    .unwrap();
}

pub(super) fn write_pnp_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // PNP BJT symbol (arrow points into emitter)

    // Base lead
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 20.0,
        cy,
        cx - 6.0,
        cy
    )
    .unwrap();

    // Emitter base line (vertical)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 6.0,
        cy - 10.0,
        cx - 6.0,
        cy + 10.0
    )
    .unwrap();

    // Collector line (angled up-right)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 6.0,
        cy - 6.0,
        cx + 10.0,
        cy - 18.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 10.0,
        cy - 18.0,
        cx + 10.0,
        cy - 20.0
    )
    .unwrap();

    // Emitter line (angled down-right)
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 6.0,
        cy + 6.0,
        cx + 10.0,
        cy + 18.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 10.0,
        cy + 18.0,
        cx + 10.0,
        cy + 20.0
    )
    .unwrap();

    // Arrow on emitter (pointing in for PNP)
    writeln!(
        svg,
        r#"<polygon class="component" fill="white" points="{},{} {},{} {},{}"/>"#,
        cx - 6.0,
        cy + 6.0,
        cx,
        cy + 10.0,
        cx - 2.0,
        cy + 4.0
    )
    .unwrap();
}

pub(super) fn write_diode_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // Diode symbol (triangle with bar)

    // Anode lead
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 20.0,
        cy,
        cx - 8.0,
        cy
    )
    .unwrap();

    // Triangle (pointing right)
    writeln!(
        svg,
        r#"<polygon class="component" fill="none" points="{},{} {},{} {},{}"/>"#,
        cx - 8.0,
        cy - 10.0,
        cx + 8.0,
        cy,
        cx - 8.0,
        cy + 10.0
    )
    .unwrap();

    // Cathode bar
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 8.0,
        cy - 10.0,
        cx + 8.0,
        cy + 10.0
    )
    .unwrap();

    // Cathode lead
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 8.0,
        cy,
        cx + 20.0,
        cy
    )
    .unwrap();
}

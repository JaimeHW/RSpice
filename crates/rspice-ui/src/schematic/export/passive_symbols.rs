use std::fmt::Write;

use super::SvgExportConfig;

pub(super) fn write_resistor_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // Zigzag resistor symbol
    let scale = 1.5;
    writeln!(
        svg,
        r#"<path class="component" d="M {} {} L {} {} L {} {} L {} {} L {} {} L {} {} L {} {} L {} {}"/>"#,
        cx - 20.0 * scale, cy,
        cx - 15.0 * scale, cy,
        cx - 12.0 * scale, cy - 8.0,
        cx - 6.0 * scale, cy + 8.0,
        cx, cy - 8.0,
        cx + 6.0 * scale, cy + 8.0,
        cx + 12.0 * scale, cy,
        cx + 20.0 * scale, cy
    ).unwrap();
}

pub(super) fn write_capacitor_symbol(
    svg: &mut String,
    cx: f64,
    cy: f64,
    _config: &SvgExportConfig,
) {
    // Two parallel lines
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 20.0,
        cy,
        cx - 3.0,
        cy
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 3.0,
        cy - 10.0,
        cx - 3.0,
        cy + 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 3.0,
        cy - 10.0,
        cx + 3.0,
        cy + 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 3.0,
        cy,
        cx + 20.0,
        cy
    )
    .unwrap();
}

pub(super) fn write_inductor_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // Arc-based inductor
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 20.0,
        cy,
        cx - 12.0,
        cy
    )
    .unwrap();
    // Three bumps
    writeln!(
        svg,
        r#"<path class="component" d="M {} {} A 4 4 0 0 1 {} {}"/>"#,
        cx - 12.0,
        cy,
        cx - 4.0,
        cy
    )
    .unwrap();
    writeln!(
        svg,
        r#"<path class="component" d="M {} {} A 4 4 0 0 1 {} {}"/>"#,
        cx - 4.0,
        cy,
        cx + 4.0,
        cy
    )
    .unwrap();
    writeln!(
        svg,
        r#"<path class="component" d="M {} {} A 4 4 0 0 1 {} {}"/>"#,
        cx + 4.0,
        cy,
        cx + 12.0,
        cy
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx + 12.0,
        cy,
        cx + 20.0,
        cy
    )
    .unwrap();
}

pub(super) fn write_transformer_symbol(
    svg: &mut String,
    cx: f64,
    cy: f64,
    config: &SvgExportConfig,
) {
    let stroke = &config.component_color;
    let stroke_width = config.component_stroke_width;

    let left_x = cx - 18.0;
    let right_x = cx + 18.0;
    let top_y = cy - 40.0;
    let bottom_y = cy + 40.0;
    let coil_top = cy - 24.0;
    let coil_bottom = cy + 24.0;
    let core_left = cx - 4.0;
    let core_right = cx + 4.0;

    writeln!(
        svg,
        "<line class=\"component\" x1=\"{left_x}\" y1=\"{top_y}\" x2=\"{left_x}\" y2=\"{coil_top}\" stroke=\"{stroke}\" stroke-width=\"{stroke_width}\"/>"
    )
    .unwrap();
    writeln!(
        svg,
        "<line class=\"component\" x1=\"{left_x}\" y1=\"{coil_bottom}\" x2=\"{left_x}\" y2=\"{bottom_y}\" stroke=\"{stroke}\" stroke-width=\"{stroke_width}\"/>"
    )
    .unwrap();
    writeln!(
        svg,
        "<line class=\"component\" x1=\"{right_x}\" y1=\"{top_y}\" x2=\"{right_x}\" y2=\"{coil_top}\" stroke=\"{stroke}\" stroke-width=\"{stroke_width}\"/>"
    )
    .unwrap();
    writeln!(
        svg,
        "<line class=\"component\" x1=\"{right_x}\" y1=\"{coil_bottom}\" x2=\"{right_x}\" y2=\"{bottom_y}\" stroke=\"{stroke}\" stroke-width=\"{stroke_width}\"/>"
    )
    .unwrap();

    for idx in 0..4 {
        let center_y = cy - 18.0 + (idx as f64 * 12.0);
        writeln!(
            svg,
            "<path class=\"component\" d=\"M {} {} C {} {}, {} {}, {} {}\" stroke=\"{}\" stroke-width=\"{}\" fill=\"none\"/>",
            left_x,
            center_y - 6.0,
            left_x - 8.0,
            center_y - 6.0,
            left_x - 8.0,
            center_y + 6.0,
            left_x,
            center_y + 6.0,
            stroke,
            stroke_width
        )
        .unwrap();
        writeln!(
            svg,
            "<path class=\"component\" d=\"M {} {} C {} {}, {} {}, {} {}\" stroke=\"{}\" stroke-width=\"{}\" fill=\"none\"/>",
            right_x,
            center_y - 6.0,
            right_x + 8.0,
            center_y - 6.0,
            right_x + 8.0,
            center_y + 6.0,
            right_x,
            center_y + 6.0,
            stroke,
            stroke_width
        )
        .unwrap();
    }

    writeln!(
        svg,
        "<line class=\"component\" x1=\"{core_left}\" y1=\"{coil_top}\" x2=\"{core_left}\" y2=\"{coil_bottom}\" stroke=\"{stroke}\" stroke-width=\"{stroke_width}\"/>"
    )
    .unwrap();
    writeln!(
        svg,
        "<line class=\"component\" x1=\"{core_right}\" y1=\"{coil_top}\" x2=\"{core_right}\" y2=\"{coil_bottom}\" stroke=\"{stroke}\" stroke-width=\"{stroke_width}\"/>"
    )
    .unwrap();

    writeln!(
        svg,
        "<circle cx=\"{}\" cy=\"{}\" r=\"2\" fill=\"{}\"/>",
        left_x - 5.0,
        top_y + 6.0,
        stroke
    )
    .unwrap();
    writeln!(
        svg,
        "<circle cx=\"{}\" cy=\"{}\" r=\"2\" fill=\"{}\"/>",
        right_x + 5.0,
        top_y + 6.0,
        stroke
    )
    .unwrap();
}

pub(super) fn write_coupled_inductor_symbol(
    svg: &mut String,
    cx: f64,
    cy: f64,
    config: &SvgExportConfig,
) {
    write_inductor_symbol(svg, cx, cy, config);
    writeln!(
        svg,
        r#"<text class="text" x="{}" y="{}" text-anchor="middle">K</text>"#,
        cx,
        cy - 14.0
    )
    .unwrap();
}

pub(super) fn write_tline_symbol(svg: &mut String, cx: f64, cy: f64, _config: &SvgExportConfig) {
    // Lossless transmission line: coax cylinder with four pin stubs.

    // Pin stubs to the a/b port terminals
    for (x1, x2) in [(cx - 30.0, cx - 13.0), (cx + 13.0, cx + 30.0)] {
        for dy in [-10.0, 10.0] {
            writeln!(
                svg,
                r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
                x1,
                cy + dy,
                x2,
                cy + dy
            )
            .unwrap();
        }
    }

    // Cylinder body
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 13.0,
        cy - 10.0,
        cx + 13.0,
        cy - 10.0
    )
    .unwrap();
    writeln!(
        svg,
        r#"<line class="component" x1="{}" y1="{}" x2="{}" y2="{}"/>"#,
        cx - 13.0,
        cy + 10.0,
        cx + 13.0,
        cy + 10.0
    )
    .unwrap();

    // Near end: full ellipse; far end: convex cap
    writeln!(
        svg,
        r#"<ellipse class="component" cx="{}" cy="{}" rx="3.5" ry="10"/>"#,
        cx - 13.0,
        cy
    )
    .unwrap();
    writeln!(
        svg,
        r#"<path class="component" d="M {} {} A 3.5 10 0 0 1 {} {}"/>"#,
        cx + 13.0,
        cy - 10.0,
        cx + 13.0,
        cy + 10.0
    )
    .unwrap();
}

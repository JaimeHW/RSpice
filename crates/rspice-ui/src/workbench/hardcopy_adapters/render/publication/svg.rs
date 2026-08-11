//! Emitting a page as SVG.
//!
//! Fonts are embedded rather than referenced, and hatch patterns are emitted
//! as definitions keyed by fill, so an exported page renders identically
//! wherever it is opened — nothing depends on the viewing machine having a
//! font or a pattern library. Colours pass through the plan's print-safe
//! mapping first, so what is emitted is what a printer will reproduce.

use super::*;

pub(super) fn render_page_svg(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    page: &PreviewPage,
) -> Result<String, HardcopyRenderError> {
    let geometry = page.geometry();
    let (page_width, page_height) = geometry.physical_size();
    let width_um = page_width.micrometres();
    let height_um = page_height.micrometres();
    let width_mm = width_um as f64 / 1_000.0;
    let height_mm = height_um as f64 / 1_000.0;
    let clip = geometry.content_rect();
    let mut output = String::with_capacity(checked_vector_capacity(
        32_768,
        1,
        0,
        scene.primitives.len(),
        96,
    )?);
    let bleed_um = match plan.setup().physical_page().bleed() {
        Bleed::None => 0,
        Bleed::Uniform(value) => value.micrometres(),
    };
    write!(
        output,
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width_mm:.3}mm\" height=\"{height_mm:.3}mm\" viewBox=\"0 0 {width_um} {height_um}\" data-rspice-bleed-um=\"{bleed_um}\" data-rspice-trim=\"{bleed_um} {bleed_um} {} {}\">",
        width_um.saturating_sub(bleed_um.saturating_mul(2)),
        height_um.saturating_sub(bleed_um.saturating_mul(2)),
    )
    .expect("write to string");
    output.push_str("<title>");
    escape_xml_into(scene.metadata.title(), &mut output);
    output.push_str("</title>");
    output.push_str("<metadata>");
    escape_xml_into(
        &format!(
            "RSpice hardcopy plan {}; source digest {}; plan digest {}",
            plan.id(),
            plan.source().content_digest(),
            plan.content_digest()
        ),
        &mut output,
    );
    output.push_str("</metadata>");
    if plan.setup().render().fonts().embed_fonts() {
        write_embedded_svg_fonts(&mut output);
    }
    write!(
        output,
        "<defs><clipPath id=\"content-clip\"><rect x=\"{}\" y=\"{}\" width=\"{}\" height=\"{}\"/></clipPath></defs>",
        clip.x.micrometres(),
        clip.y.micrometres(),
        clip.width.micrometres(),
        clip.height.micrometres()
    )
    .expect("write to string");
    let primitives = page_primitives(scene, page)?;
    write_svg_hatch_defs(&mut output, plan, primitives, &scene.legend);
    if let Some(background) = background_color(plan) {
        write!(
            output,
            "<rect x=\"0\" y=\"0\" width=\"{width_um}\" height=\"{height_um}\" fill=\"{}\"/>",
            svg_color(background)
        )
        .expect("write to string");
    }
    output.push_str("<g clip-path=\"url(#content-clip)\">");
    let transform = page_transform(page);
    for primitive in primitives {
        write_svg_primitive(&mut output, plan, transform, primitive);
    }
    output.push_str("</g>");
    write_svg_decorations(&mut output, plan, scene, page);
    output.push_str("</svg>");
    Ok(output)
}

pub(super) fn write_embedded_svg_fonts(output: &mut String) {
    let encoded = |bytes: &[u8]| base64::engine::general_purpose::STANDARD.encode(bytes);
    output.push_str("<style>");
    for (family, weight, bytes) in PUBLICATION_FACES {
        write!(
            output,
            "@font-face{{font-family:'{family}';font-style:normal;font-weight:{weight};src:url(data:font/ttf;base64,{}) format('truetype');}}",
            encoded(bytes)
        )
        .expect("write to string");
    }
    output.push_str("</style>");
}

pub(super) fn write_svg_primitive(
    output: &mut String,
    plan: &HardcopyPlan,
    transform: PageTransform,
    primitive: &ScenePrimitive,
) {
    match primitive {
        ScenePrimitive::Line { from, to, stroke } => {
            let (x1, y1) = transform.point(*from);
            let (x2, y2) = transform.point(*to);
            let stroke = resolve_stroke(plan, transform, *stroke);
            write!(
                output,
                "<line x1=\"{x1:.3}\" y1=\"{y1:.3}\" x2=\"{x2:.3}\" y2=\"{y2:.3}\""
            )
            .expect("write to string");
            write_svg_stroke(output, stroke);
            output.push_str(" fill=\"none\"/>");
        }
        ScenePrimitive::Polyline {
            points,
            closed,
            stroke,
            fill,
        } => {
            output.push_str(if *closed {
                "<polygon points=\""
            } else {
                "<polyline points=\""
            });
            for (index, point) in points.iter().enumerate() {
                let (x, y) = transform.point(*point);
                if index != 0 {
                    output.push(' ');
                }
                write!(output, "{x:.3},{y:.3}").expect("write to string");
            }
            output.push('"');
            write_svg_stroke(output, resolve_stroke(plan, transform, *stroke));
            write_svg_fill(output, plan, *fill);
            output.push_str("/>");
        }
        ScenePrimitive::Rect { rect, stroke, fill } => {
            let (x, y) = transform.point(ScenePoint::new(rect.x, rect.y));
            let width = transform.length(rect.width);
            let height = transform.length(rect.height);
            write!(
                output,
                "<rect x=\"{x:.3}\" y=\"{y:.3}\" width=\"{width:.3}\" height=\"{height:.3}\""
            )
            .expect("write to string");
            if let Some(stroke) = stroke {
                write_svg_stroke(output, resolve_stroke(plan, transform, *stroke));
            } else {
                output.push_str(" stroke=\"none\"");
            }
            write_svg_fill(output, plan, *fill);
            output.push_str("/>");
        }
        ScenePrimitive::Circle {
            center,
            radius,
            stroke,
            fill,
        } => {
            let (cx, cy) = transform.point(*center);
            let radius = transform.length(*radius);
            write!(
                output,
                "<circle cx=\"{cx:.3}\" cy=\"{cy:.3}\" r=\"{radius:.3}\""
            )
            .expect("write to string");
            if let Some(stroke) = stroke {
                write_svg_stroke(output, resolve_stroke(plan, transform, *stroke));
            } else {
                output.push_str(" stroke=\"none\"");
            }
            write_svg_fill(output, plan, *fill);
            output.push_str("/>");
        }
        ScenePrimitive::RasterImage {
            rect,
            png,
            content_digest,
            alternative_text,
        } => {
            let (x, y) = transform.point(ScenePoint::new(rect.x, rect.y));
            let width = transform.length(rect.width);
            let height = transform.length(rect.height);
            write!(
                output,
                "<image x=\"{x:.3}\" y=\"{y:.3}\" width=\"{width:.3}\" height=\"{height:.3}\" preserveAspectRatio=\"xMidYMid meet\" href=\"data:image/png;base64,{}\" data-content-digest=\"{}\" aria-label=\"",
                base64::engine::general_purpose::STANDARD.encode(png),
                content_digest,
            )
            .expect("write to string");
            escape_xml_into(alternative_text, output);
            output.push_str("\"/>");
        }
        ScenePrimitive::Text {
            origin,
            text,
            font,
            size,
            color,
            anchor,
            rotation,
        } => {
            let (x, y) = transform.point(*origin);
            let size = transform.length(*size);
            let (family, weight) = svg_font(*font);
            let anchor = match anchor {
                TextAnchor::Start => "start",
                TextAnchor::Middle => "middle",
                TextAnchor::End => "end",
            };
            let rotation = match rotation {
                SceneTextRotation::Upright => String::new(),
                SceneTextRotation::Clockwise90 => {
                    format!(" transform=\"rotate(90 {x:.3} {y:.3})\"")
                }
                SceneTextRotation::CounterClockwise90 => {
                    format!(" transform=\"rotate(-90 {x:.3} {y:.3})\"")
                }
            };
            write!(
                output,
                "<text x=\"{x:.3}\" y=\"{y:.3}\" fill=\"{}\" font-family=\"{family}\" font-weight=\"{weight}\" font-size=\"{size:.3}\" text-anchor=\"{anchor}\"{rotation}>",
                svg_color(resolve_color(plan, *color)),
            )
            .expect("write to string");
            escape_xml_into(text, output);
            output.push_str("</text>");
        }
        ScenePrimitive::ClippedGroup {
            source_origin,
            destination_origin,
            primitives,
            ..
        } => {
            let transform = transform.remap(*source_origin, *destination_origin);
            for primitive in primitives {
                write_svg_primitive(output, plan, transform, primitive);
            }
        }
    }
}

pub(super) fn write_svg_stroke(output: &mut String, stroke: ResolvedStroke) {
    write!(
        output,
        " stroke=\"{}\" stroke-width=\"{:.3}\" stroke-linecap=\"round\" stroke-linejoin=\"round\"",
        svg_color(stroke.color),
        stroke.width_um
    )
    .expect("write to string");
    let dash = if let Some((dash, gap)) = stroke.exact_dash_um {
        Some(format!("{dash:.3} {gap:.3}"))
    } else if let Some(spacing) = stroke.exact_dot_spacing_um {
        Some(format!("{:.3} {spacing:.3}", stroke.width_um))
    } else {
        svg_dash_pattern(stroke.pattern, stroke.width_um)
    };
    if let Some(dash) = dash {
        write!(output, " stroke-dasharray=\"{dash}\"").expect("write to string");
    }
}

pub(super) fn svg_dash_pattern(pattern: StrokePattern, width: f64) -> Option<String> {
    match pattern {
        StrokePattern::Solid => None,
        StrokePattern::Dashed => Some(format!("{:.3} {:.3}", width * 6.0, width * 3.0)),
        StrokePattern::Dotted => Some(format!("{:.3} {:.3}", width, width * 2.5)),
        StrokePattern::DashDot => Some(format!(
            "{:.3} {:.3} {:.3} {:.3}",
            width * 6.0,
            width * 2.5,
            width,
            width * 2.5
        )),
    }
}

pub(super) fn write_svg_hatch_defs(
    output: &mut String,
    plan: &HardcopyPlan,
    primitives: &[ScenePrimitive],
    legend: &[LegendEntry],
) {
    let mut seen = Vec::<String>::new();
    let mut fills = Vec::new();
    for primitive in primitives {
        collect_primitive_fills(primitive, &mut fills);
    }
    fills.extend(legend.iter().filter_map(|entry| entry.fill));
    for fill in fills {
        let SceneFill::CrossHatch {
            color,
            line_width,
            spacing,
        } = fill
        else {
            continue;
        };
        let id = hatch_pattern_id(fill);
        if seen.iter().any(|existing| existing == &id) {
            continue;
        }
        seen.push(id.clone());
        let tile = spacing.micrometres() as f64 * std::f64::consts::SQRT_2;
        let color = svg_color(resolve_color(plan, color));
        write!(
            output,
            "<defs><pattern id=\"{id}\" patternUnits=\"userSpaceOnUse\" width=\"{tile:.6}\" height=\"{tile:.6}\"><path d=\"M 0 0 L {tile:.6} {tile:.6} M 0 {tile:.6} L {tile:.6} 0\" fill=\"none\" stroke=\"{color}\" stroke-width=\"{}\"/></pattern></defs>",
            line_width.micrometres(),
        )
        .expect("write to string");
    }
}

fn collect_primitive_fills(primitive: &ScenePrimitive, fills: &mut Vec<SceneFill>) {
    if let Some(fill) = primitive_fill(primitive) {
        fills.push(fill);
    }
    if let ScenePrimitive::ClippedGroup { primitives, .. } = primitive {
        for primitive in primitives {
            collect_primitive_fills(primitive, fills);
        }
    }
}

pub(super) fn primitive_fill(primitive: &ScenePrimitive) -> Option<SceneFill> {
    match primitive {
        ScenePrimitive::Polyline { fill, .. }
        | ScenePrimitive::Rect { fill, .. }
        | ScenePrimitive::Circle { fill, .. } => *fill,
        ScenePrimitive::Line { .. }
        | ScenePrimitive::RasterImage { .. }
        | ScenePrimitive::Text { .. }
        | ScenePrimitive::ClippedGroup { .. } => None,
    }
}

pub(super) fn hatch_pattern_id(fill: SceneFill) -> String {
    match fill {
        SceneFill::CrossHatch {
            color,
            line_width,
            spacing,
        } => {
            let encoded = serde_json::to_vec(&color).expect("semantic color serialization");
            let digest = Sha256::digest(encoded);
            format!(
                "hatch-{:02x}{:02x}{:02x}{:02x}-{}-{}",
                digest[0],
                digest[1],
                digest[2],
                digest[3],
                line_width.micrometres(),
                spacing.micrometres()
            )
        }
        SceneFill::Solid { .. } => "solid".to_owned(),
    }
}

pub(super) fn write_svg_fill(output: &mut String, plan: &HardcopyPlan, fill: Option<SceneFill>) {
    match fill {
        Some(SceneFill::Solid { color }) => write!(
            output,
            " fill=\"{}\"",
            svg_color(resolve_color(plan, color))
        )
        .expect("write to string"),
        Some(fill @ SceneFill::CrossHatch { .. }) => {
            write!(output, " fill=\"url(#{})\"", hatch_pattern_id(fill)).expect("write to string")
        }
        None => output.push_str(" fill=\"none\""),
    }
}

pub(super) fn svg_font(font: SceneFont) -> (&'static str, &'static str) {
    match font {
        SceneFont::Sans => (PUBLICATION_SANS, "400"),
        SceneFont::SansSemibold => (PUBLICATION_SANS, "600"),
        SceneFont::Monospace => (PUBLICATION_MONO, "400"),
    }
}

pub(super) fn escape_xml_into(value: &str, output: &mut String) {
    for character in value.chars() {
        match character {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&apos;"),
            character => output.push(character),
        }
    }
}

pub(super) fn write_svg_decorations(
    output: &mut String,
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    page: &PreviewPage,
) {
    let geometry = page.geometry();
    let printable = geometry.printable_rect();
    let content = geometry.content_rect();
    let ink = svg_color(resolve_color(plan, SemanticColor::Foreground));
    let secondary = svg_color(resolve_color(plan, SemanticColor::Secondary));
    if plan.setup().decorations().includes_header() {
        let baseline = printable.y.micrometres() + DECORATION_TEXT_UM;
        let page_right = printable.x.micrometres() + printable.width.micrometres();
        write!(
            output,
            "<text x=\"{}\" y=\"{baseline}\" fill=\"{ink}\" font-family=\"{PUBLICATION_SANS}\" font-weight=\"600\" font-size=\"{DECORATION_TEXT_UM}\">",
            printable.x.micrometres()
        )
        .expect("write to string");
        escape_xml_into(scene.metadata.title(), output);
        output.push_str("</text>");
        write!(
            output,
            "<text x=\"{page_right}\" y=\"{baseline}\" fill=\"{secondary}\" font-family=\"{PUBLICATION_MONO}\" font-size=\"{DECORATION_TEXT_UM}\" text-anchor=\"end\">"
        )
        .expect("write to string");
        escape_xml_into(
            &format!(
                "rev {} · page {} / {} · {}",
                plan.source().revision().get(),
                page.number(),
                plan.pagination().pages().len(),
                page.coordinate()
            ),
            output,
        );
        output.push_str("</text>");
        if let Some(line) = scene.metadata.header_lines.first() {
            let center = printable.x.micrometres() + printable.width.micrometres() / 2;
            write!(
                output,
                "<text x=\"{center}\" y=\"{baseline}\" fill=\"{secondary}\" font-family=\"{PUBLICATION_SANS}\" font-size=\"{DECORATION_TEXT_UM}\" text-anchor=\"middle\">"
            )
            .expect("write to string");
            escape_xml_into(line, output);
            output.push_str("</text>");
        }
    }
    if plan.setup().decorations().includes_provenance() {
        let baseline = printable.y.micrometres() + printable.height.micrometres() - 2_000;
        write!(
            output,
            "<text x=\"{}\" y=\"{baseline}\" fill=\"{secondary}\" font-family=\"{PUBLICATION_MONO}\" font-size=\"2200\">",
            printable.x.micrometres()
        )
        .expect("write to string");
        let text = scene
            .metadata
            .provenance_lines
            .first()
            .cloned()
            .unwrap_or_else(|| {
                format!(
                    "source {} · plan {}",
                    plan.source().content_digest(),
                    plan.content_digest()
                )
            });
        escape_xml_into(&text, output);
        output.push_str("</text>");
    }
    if plan.setup().decorations().includes_legends() && !scene.legend.is_empty() {
        let legend_band = geometry.legend_band().micrometres();
        let columns = printable.width.micrometres() / LEGEND_COLUMN_UM;
        let rows = legend_band.saturating_sub(LEGEND_VERTICAL_PADDING_UM) / LEGEND_ROW_UM;
        let legend_top = content
            .y
            .micrometres()
            .saturating_add(content.height.micrometres());
        let box_height = rows
            .saturating_mul(LEGEND_ROW_UM)
            .saturating_add(LEGEND_VERTICAL_PADDING_UM);
        write!(
            output,
            "<g data-rspice-decoration=\"legend\"><rect x=\"{}\" y=\"{legend_top}\" width=\"{}\" height=\"{box_height}\" fill=\"#ffffff\" fill-opacity=\"0.92\" stroke=\"{secondary}\" stroke-width=\"150\"/>",
            printable.x.micrometres(),
            printable.width.micrometres(),
        )
        .expect("write to string");
        for (index, entry) in scene.legend.iter().enumerate() {
            let index = index as u64;
            let column = index / rows.max(1);
            let row = index % rows.max(1);
            if column >= columns {
                break;
            }
            let x = printable
                .x
                .micrometres()
                .saturating_add(column.saturating_mul(LEGEND_COLUMN_UM))
                .saturating_add(2_000);
            let y = legend_top
                .saturating_add(3_500)
                .saturating_add(row.saturating_mul(LEGEND_ROW_UM));
            let stroke = resolve_stroke(plan, page_transform(page), entry.stroke);
            if entry.fill.is_some() {
                write!(
                    output,
                    "<rect x=\"{x}\" y=\"{}\" width=\"12000\" height=\"3000\"",
                    y.saturating_sub(2_000)
                )
                .expect("write to string");
                write_svg_stroke(output, stroke);
                write_svg_fill(output, plan, entry.fill);
                output.push_str("/>");
            } else {
                write!(
                    output,
                    "<line x1=\"{x}\" y1=\"{y}\" x2=\"{}\" y2=\"{y}\"",
                    x + 12_000
                )
                .expect("write to string");
                write_svg_stroke(output, stroke);
                output.push_str("/>");
            }
            write!(
                output,
                "<text x=\"{}\" y=\"{}\" fill=\"{ink}\" font-family=\"{PUBLICATION_SANS}\" font-size=\"2600\">",
                x + 15_000,
                y + 900
            )
            .expect("write to string");
            escape_xml_into(&entry.label, output);
            output.push_str("</text>");
        }
        output.push_str("</g>");
    }
    write_svg_watermark(output, plan, scene, page);
    write_svg_trim_marks(output, plan, page);
    write_svg_registration_marks(output, plan, page);
}

pub(super) fn write_svg_trim_marks(output: &mut String, plan: &HardcopyPlan, page: &PreviewPage) {
    let Bleed::Uniform(bleed) = plan.setup().physical_page().bleed() else {
        return;
    };
    let geometry = page.geometry();
    let (width, height) = geometry.physical_size();
    let inset = bleed.micrometres();
    let right = width.micrometres().saturating_sub(inset);
    let bottom = height.micrometres().saturating_sub(inset);
    let gap = (inset / 4).max(100);
    let length = (inset.saturating_mul(3) / 4).clamp(500, 5_000);
    let color = svg_color(resolve_color(plan, SemanticColor::Foreground));
    write!(
        output,
        "<path data-rspice-decoration=\"trim-marks\" d=\"M {} {inset} H {} M {inset} {} V {} M {} {inset} H {} M {right} {} V {} M {} {bottom} H {} M {inset} {} V {} M {} {bottom} H {} M {right} {} V {}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"150\"/>",
        inset.saturating_sub(gap.saturating_add(length)),
        inset.saturating_sub(gap),
        inset.saturating_sub(gap.saturating_add(length)),
        inset.saturating_sub(gap),
        right.saturating_add(gap),
        right.saturating_add(gap).saturating_add(length),
        inset.saturating_sub(gap.saturating_add(length)),
        inset.saturating_sub(gap),
        inset.saturating_sub(gap.saturating_add(length)),
        inset.saturating_sub(gap),
        bottom.saturating_add(gap),
        bottom.saturating_add(gap).saturating_add(length),
        right.saturating_add(gap),
        right.saturating_add(gap).saturating_add(length),
        bottom.saturating_add(gap),
        bottom.saturating_add(gap).saturating_add(length),
    )
    .expect("write to string");
}

pub(super) fn write_svg_watermark(
    output: &mut String,
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    page: &PreviewPage,
) {
    let text = match plan.setup().decorations().watermark() {
        Watermark::None => return,
        Watermark::Draft => "DRAFT",
        Watermark::Confidential => "CONFIDENTIAL",
        Watermark::Custom(text) => text,
    };
    let (width, height) = page.geometry().physical_size();
    let cx = width.micrometres() / 2;
    let cy = height.micrometres() / 2;
    let ink = svg_color(resolve_color(plan, SemanticColor::Secondary));
    let size = width.micrometres().min(height.micrometres()) / 10;
    write!(
        output,
        "<text x=\"{cx}\" y=\"{cy}\" fill=\"{ink}\" fill-opacity=\"0.16\" font-family=\"{PUBLICATION_SANS}\" font-weight=\"600\" font-size=\"{size}\" text-anchor=\"middle\" transform=\"rotate(-35 {cx} {cy})\">"
    )
    .expect("write to string");
    escape_xml_into(text, output);
    output.push_str("</text>");
    let _ = scene;
}

pub(super) fn write_svg_registration_marks(
    output: &mut String,
    plan: &HardcopyPlan,
    page: &PreviewPage,
) {
    if !plan.setup().tiling().registration_marks_and_coordinates() {
        return;
    }
    let rect = page.geometry().content_rect();
    let color = svg_color(resolve_color(plan, SemanticColor::Secondary));
    let left = rect.x.micrometres();
    let right = left + rect.width.micrometres();
    let top = rect.y.micrometres();
    let bottom = top + rect.height.micrometres();
    for (x, y) in [(left, top), (right, top), (left, bottom), (right, bottom)] {
        write!(
            output,
            "<path d=\"M {} {y} H {} M {x} {} V {}\" fill=\"none\" stroke=\"{color}\" stroke-width=\"150\"/>",
            x.saturating_sub(2_000),
            x + 2_000,
            y.saturating_sub(2_000),
            y + 2_000
        )
        .expect("write to string");
    }
    write!(
        output,
        "<text x=\"{}\" y=\"{}\" fill=\"{color}\" font-family=\"{PUBLICATION_MONO}\" font-size=\"2200\">{}</text>",
        left + 2_500,
        top + 3_000,
        page.coordinate()
    )
    .expect("write to string");
}

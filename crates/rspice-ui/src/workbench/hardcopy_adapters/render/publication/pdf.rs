//! Emitting a page as PDF.
//!
//! Fonts are embedded as subsets and the byte stream carries no timestamp or
//! producer metadata, so the same scene produces byte-identical output on
//! every run — which is what lets a printed artifact be checked against a
//! digest. Conformance level is taken from the plan rather than inferred, so a
//! document claiming PDF/A carries what that level actually requires.

use super::*;

pub(super) struct PdfFonts {
    pub(super) sans: Font,
    pub(super) semibold: Font,
    pub(super) mono: Font,
}

impl PdfFonts {
    fn load() -> Result<Self, HardcopyRenderError> {
        Ok(Self {
            sans: load_pdf_font(IBM_PLEX_SANS_REGULAR, "IBM Plex Sans Regular")?,
            semibold: load_pdf_font(IBM_PLEX_SANS_SEMIBOLD, "IBM Plex Sans Semibold")?,
            mono: load_pdf_font(IBM_PLEX_MONO_REGULAR, "IBM Plex Mono Regular")?,
        })
    }

    fn get(&self, font: SceneFont) -> &Font {
        match font {
            SceneFont::Sans => &self.sans,
            SceneFont::SansSemibold => &self.semibold,
            SceneFont::Monospace => &self.mono,
        }
    }
}

pub(super) fn load_pdf_font(
    bytes: &'static [u8],
    name: &'static str,
) -> Result<Font, HardcopyRenderError> {
    Font::new(bytes.into(), 0).ok_or(HardcopyRenderError::InvalidEmbeddedFont(name))
}

pub(super) fn render_pdf(
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    archival: bool,
) -> Result<Vec<u8>, HardcopyRenderError> {
    if archival && scene.metadata.publication_timestamp.is_none() {
        return Err(HardcopyRenderError::PdfARequiresPublicationTimestamp);
    }
    let configuration = if archival {
        ConfigurationBuilder::new()
            .with_archival_validator(Archival::A2_B)
            .finish()
            .map_err(|error| HardcopyRenderError::PdfAConfiguration(format!("{error:?}")))?
    } else {
        ConfigurationBuilder::new()
            .finish()
            .map_err(|error| HardcopyRenderError::PdfSerialization(format!("{error:?}")))?
    };
    let settings = SerializeSettings {
        pretty: true,
        compress_content_streams: false,
        no_device_cs: archival,
        xmp_metadata: archival,
        configuration,
        enable_tagging: false,
        ..SerializeSettings::default()
    };
    let fonts = PdfFonts::load()?;
    let mut document = Document::new_with(settings);
    let mut metadata = Metadata::new()
        .title(scene.metadata.title.clone())
        .description(format!(
            "RSpice hardcopy; source {}; plan {}",
            plan.source().content_digest(),
            plan.content_digest()
        ))
        .creator(scene.metadata.creator.clone())
        .producer(if archival {
            "RSpice validated PDF/A-2b hardcopy renderer".to_owned()
        } else {
            "RSpice vector hardcopy renderer".to_owned()
        })
        .document_id(format!("rspice-hardcopy-{}", plan.id()))
        .language("en-US".to_owned())
        .page_layout(PageLayout::OneColumn);
    if !scene.metadata.authors.is_empty() {
        metadata = metadata.authors(scene.metadata.authors.clone());
    }
    if let Some(timestamp) = scene.metadata.publication_timestamp {
        metadata = metadata.creation_date(timestamp.as_krilla());
    }
    document.set_metadata(metadata);

    for preview_page in plan.pagination().pages() {
        let geometry = preview_page.geometry();
        let (physical_width, physical_height) = geometry.physical_size();
        let page_width = um_to_points(physical_width.micrometres());
        let page_height = um_to_points(physical_height.micrometres());
        let mut page_settings = PageSettings::from_wh(page_width, page_height)
            .ok_or(HardcopyRenderError::RasterDimensionOverflow)?;
        let full = Rect::from_xywh(0.0, 0.0, page_width, page_height)
            .ok_or(HardcopyRenderError::RasterDimensionOverflow)?;
        let content = geometry.content_rect();
        let art_box = pdf_rect(content)?;
        page_settings = page_settings.with_art_box(Some(art_box));
        if let Bleed::Uniform(bleed) = plan.setup().physical_page().bleed() {
            let inset = um_to_points(bleed.micrometres());
            let trim = Rect::from_xywh(
                inset,
                inset,
                page_width - 2.0 * inset,
                page_height - 2.0 * inset,
            )
            .ok_or(HardcopyRenderError::RasterDimensionOverflow)?;
            page_settings = page_settings
                .with_bleed_box(Some(full))
                .with_trim_box(Some(trim));
        }
        let mut page = document.start_page_with(page_settings);
        let mut surface = page.surface();
        if let Some(background) = background_color(plan) {
            set_pdf_fill(&mut surface, background, NormalizedF32::ONE);
            surface.set_stroke(None);
            let mut builder = PathBuilder::new();
            builder.push_rect(full);
            if let Some(path) = builder.finish() {
                surface.draw_path(&path);
            }
        }
        let mut clip_builder = PathBuilder::new();
        clip_builder.push_rect(pdf_rect(content)?);
        let clip = clip_builder
            .finish()
            .ok_or(HardcopyRenderError::EmptyPrimitiveGeometry)?;
        surface.push_clip_path(&clip, &FillRule::NonZero);
        let transform = page_transform(preview_page);
        for primitive in page_primitives(scene, preview_page)? {
            draw_pdf_primitive(&mut surface, &fonts, plan, transform, primitive)?;
        }
        surface.pop();
        draw_pdf_decorations(&mut surface, &fonts, plan, scene, preview_page);
        draw_pdf_trim_marks(&mut surface, plan, preview_page);
        surface.finish();
        page.finish();
    }
    document
        .finish()
        .map_err(|error| HardcopyRenderError::PdfSerialization(error.to_string()))
}

pub(super) fn um_to_points(value: u64) -> f32 {
    (value as f64 * POINTS_PER_INCH / MICROMETRES_PER_INCH as f64) as f32
}

pub(super) fn coordinate_to_points(value: f64) -> f32 {
    (value * POINTS_PER_INCH / MICROMETRES_PER_INCH as f64) as f32
}

pub(super) fn draw_pdf_trim_marks(
    surface: &mut krilla::surface::Surface<'_>,
    plan: &HardcopyPlan,
    page: &PreviewPage,
) {
    let Bleed::Uniform(bleed) = plan.setup().physical_page().bleed() else {
        return;
    };
    let (width, height) = page.geometry().physical_size();
    let inset = bleed.micrometres();
    let right = width.micrometres().saturating_sub(inset);
    let bottom = height.micrometres().saturating_sub(inset);
    let gap = (inset / 4).max(100);
    let length = (inset.saturating_mul(3) / 4).min(5_000).max(500);
    let segments = [
        (
            inset.saturating_sub(gap.saturating_add(length)),
            inset,
            inset.saturating_sub(gap),
            inset,
        ),
        (
            inset,
            inset.saturating_sub(gap.saturating_add(length)),
            inset,
            inset.saturating_sub(gap),
        ),
        (
            right.saturating_add(gap),
            inset,
            right.saturating_add(gap).saturating_add(length),
            inset,
        ),
        (
            right,
            inset.saturating_sub(gap.saturating_add(length)),
            right,
            inset.saturating_sub(gap),
        ),
        (
            inset.saturating_sub(gap.saturating_add(length)),
            bottom,
            inset.saturating_sub(gap),
            bottom,
        ),
        (
            inset,
            bottom.saturating_add(gap),
            inset,
            bottom.saturating_add(gap).saturating_add(length),
        ),
        (
            right.saturating_add(gap),
            bottom,
            right.saturating_add(gap).saturating_add(length),
            bottom,
        ),
        (
            right,
            bottom.saturating_add(gap),
            right,
            bottom.saturating_add(gap).saturating_add(length),
        ),
    ];
    let mut builder = PathBuilder::new();
    for (x1, y1, x2, y2) in segments {
        builder.move_to(um_to_points(x1), um_to_points(y1));
        builder.line_to(um_to_points(x2), um_to_points(y2));
    }
    surface.set_fill(None);
    set_pdf_stroke(
        surface,
        ResolvedStroke {
            color: resolve_color(plan, SemanticColor::Foreground),
            width_um: 150.0,
            pattern: StrokePattern::Solid,
            exact_dash_um: None,
            exact_dot_spacing_um: None,
        },
    );
    if let Some(path) = builder.finish() {
        surface.draw_path(&path);
    }
}

pub(super) fn pdf_rect(rect: PageRect) -> Result<Rect, HardcopyRenderError> {
    Rect::from_xywh(
        um_to_points(rect.x.micrometres()),
        um_to_points(rect.y.micrometres()),
        um_to_points(rect.width.micrometres()),
        um_to_points(rect.height.micrometres()),
    )
    .ok_or(HardcopyRenderError::EmptyPrimitiveGeometry)
}

pub(super) fn set_pdf_fill(
    surface: &mut krilla::surface::Surface<'_>,
    color: Rgb8,
    opacity: NormalizedF32,
) {
    surface.set_fill(Some(Fill {
        paint: rgb::Color::new(color.red, color.green, color.blue).into(),
        opacity,
        rule: FillRule::NonZero,
    }));
}

pub(super) fn set_pdf_scene_fill(
    surface: &mut krilla::surface::Surface<'_>,
    plan: &HardcopyPlan,
    fill: Option<SceneFill>,
) {
    match fill {
        Some(SceneFill::Solid { color }) => {
            set_pdf_fill(surface, resolve_color(plan, color), NormalizedF32::ONE)
        }
        Some(SceneFill::CrossHatch { .. }) | None => surface.set_fill(None),
    }
}

pub(super) fn draw_pdf_cross_hatch(
    surface: &mut krilla::surface::Surface<'_>,
    plan: &HardcopyPlan,
    scale: f64,
    clip: &krilla::geom::Path,
    bounds: (f32, f32, f32, f32),
    fill: Option<SceneFill>,
) {
    let Some(SceneFill::CrossHatch {
        color,
        line_width,
        spacing,
    }) = fill
    else {
        return;
    };
    let (x, y, width, height) = bounds;
    let tile =
        coordinate_to_points(spacing.micrometres() as f64 * scale) * std::f32::consts::SQRT_2;
    if tile <= 0.0 || width <= 0.0 || height <= 0.0 {
        return;
    }
    let mut builder = PathBuilder::new();
    let mut offset = -height;
    let mut lines = 0_u64;
    while offset <= width {
        builder.move_to(x + offset, y);
        builder.line_to(x + offset + height, y + height);
        builder.move_to(x + offset, y + height);
        builder.line_to(x + offset + height, y);
        offset += tile;
        lines += 2;
        if lines > MAX_RENDER_WORK_UNITS {
            return;
        }
    }
    let Some(path) = builder.finish() else {
        return;
    };
    surface.push_clip_path(clip, &FillRule::NonZero);
    surface.set_fill(None);
    set_pdf_stroke(
        surface,
        ResolvedStroke {
            color: resolve_color(plan, color),
            width_um: line_width.micrometres() as f64 * scale,
            pattern: StrokePattern::Solid,
            exact_dash_um: None,
            exact_dot_spacing_um: None,
        },
    );
    surface.draw_path(&path);
    surface.pop();
}

pub(super) fn set_pdf_stroke(surface: &mut krilla::surface::Surface<'_>, stroke: ResolvedStroke) {
    let width = coordinate_to_points(stroke.width_um);
    let dash = if let Some((dash, gap)) = stroke.exact_dash_um {
        Some(vec![coordinate_to_points(dash), coordinate_to_points(gap)])
    } else if let Some(spacing) = stroke.exact_dot_spacing_um {
        Some(vec![width, coordinate_to_points(spacing)])
    } else {
        match stroke.pattern {
            StrokePattern::Solid => None,
            StrokePattern::Dashed => Some(vec![width * 6.0, width * 3.0]),
            StrokePattern::Dotted => Some(vec![width, width * 2.5]),
            StrokePattern::DashDot => Some(vec![width * 6.0, width * 2.5, width, width * 2.5]),
        }
    };
    surface.set_stroke(Some(Stroke {
        paint: rgb::Color::new(stroke.color.red, stroke.color.green, stroke.color.blue).into(),
        width,
        line_cap: LineCap::Round,
        line_join: LineJoin::Round,
        dash: dash.map(|array| StrokeDash { array, offset: 0.0 }),
        ..Stroke::default()
    }));
}

pub(super) fn draw_pdf_primitive(
    surface: &mut krilla::surface::Surface<'_>,
    fonts: &PdfFonts,
    plan: &HardcopyPlan,
    transform: PageTransform,
    primitive: &ScenePrimitive,
) -> Result<(), HardcopyRenderError> {
    match primitive {
        ScenePrimitive::Line { from, to, stroke } => {
            let (x1, y1) = transform.point(*from);
            let (x2, y2) = transform.point(*to);
            let mut builder = PathBuilder::new();
            builder.move_to(coordinate_to_points(x1), coordinate_to_points(y1));
            builder.line_to(coordinate_to_points(x2), coordinate_to_points(y2));
            surface.set_fill(None);
            set_pdf_stroke(surface, resolve_stroke(plan, transform, *stroke));
            if let Some(path) = builder.finish() {
                surface.draw_path(&path);
            }
        }
        ScenePrimitive::Polyline {
            points,
            closed,
            stroke,
            fill,
        } => {
            let mut builder = PathBuilder::new();
            let mut minimum_x = f32::INFINITY;
            let mut minimum_y = f32::INFINITY;
            let mut maximum_x = f32::NEG_INFINITY;
            let mut maximum_y = f32::NEG_INFINITY;
            for (index, point) in points.iter().enumerate() {
                let (x, y) = transform.point(*point);
                let x = coordinate_to_points(x);
                let y = coordinate_to_points(y);
                minimum_x = minimum_x.min(x);
                minimum_y = minimum_y.min(y);
                maximum_x = maximum_x.max(x);
                maximum_y = maximum_y.max(y);
                if index == 0 {
                    builder.move_to(x, y);
                } else {
                    builder.line_to(x, y);
                }
            }
            if *closed {
                builder.close();
            }
            set_pdf_scene_fill(surface, plan, *fill);
            set_pdf_stroke(surface, resolve_stroke(plan, transform, *stroke));
            if let Some(path) = builder.finish() {
                surface.draw_path(&path);
                draw_pdf_cross_hatch(
                    surface,
                    plan,
                    transform.scale_factor(),
                    &path,
                    (
                        minimum_x,
                        minimum_y,
                        maximum_x - minimum_x,
                        maximum_y - minimum_y,
                    ),
                    *fill,
                );
            }
        }
        ScenePrimitive::Rect { rect, stroke, fill } => {
            let (x, y) = transform.point(ScenePoint::new(rect.x, rect.y));
            let pdf_rect = Rect::from_xywh(
                coordinate_to_points(x),
                coordinate_to_points(y),
                coordinate_to_points(transform.length(rect.width)),
                coordinate_to_points(transform.length(rect.height)),
            );
            set_pdf_scene_fill(surface, plan, *fill);
            match stroke {
                Some(stroke) => set_pdf_stroke(surface, resolve_stroke(plan, transform, *stroke)),
                None => surface.set_stroke(None),
            }
            if let Some(pdf_rect_value) = pdf_rect {
                let mut builder = PathBuilder::new();
                builder.push_rect(pdf_rect_value);
                if let Some(path) = builder.finish() {
                    surface.draw_path(&path);
                    draw_pdf_cross_hatch(
                        surface,
                        plan,
                        transform.scale_factor(),
                        &path,
                        (
                            coordinate_to_points(x),
                            coordinate_to_points(y),
                            coordinate_to_points(transform.length(rect.width)),
                            coordinate_to_points(transform.length(rect.height)),
                        ),
                        *fill,
                    );
                }
            }
        }
        ScenePrimitive::Circle {
            center,
            radius,
            stroke,
            fill,
        } => {
            let (cx, cy) = transform.point(*center);
            let cx = coordinate_to_points(cx);
            let cy = coordinate_to_points(cy);
            let radius = coordinate_to_points(transform.length(*radius));
            let k = radius * 0.552_284_8;
            let mut builder = PathBuilder::new();
            builder.move_to(cx + radius, cy);
            builder.cubic_to(cx + radius, cy + k, cx + k, cy + radius, cx, cy + radius);
            builder.cubic_to(cx - k, cy + radius, cx - radius, cy + k, cx - radius, cy);
            builder.cubic_to(cx - radius, cy - k, cx - k, cy - radius, cx, cy - radius);
            builder.cubic_to(cx + k, cy - radius, cx + radius, cy - k, cx + radius, cy);
            builder.close();
            set_pdf_scene_fill(surface, plan, *fill);
            match stroke {
                Some(stroke) => set_pdf_stroke(surface, resolve_stroke(plan, transform, *stroke)),
                None => surface.set_stroke(None),
            }
            if let Some(path) = builder.finish() {
                surface.draw_path(&path);
                draw_pdf_cross_hatch(
                    surface,
                    plan,
                    transform.scale_factor(),
                    &path,
                    (cx - radius, cy - radius, radius * 2.0, radius * 2.0),
                    *fill,
                );
            }
        }
        ScenePrimitive::RasterImage { rect, png, .. } => {
            let image = Image::from_png(png.clone().into(), false)
                .map_err(HardcopyRenderError::InvalidEmbeddedFigure)?;
            let (x, y) = transform.point(ScenePoint::new(rect.x, rect.y));
            let size = Size::from_wh(
                coordinate_to_points(transform.length(rect.width)),
                coordinate_to_points(transform.length(rect.height)),
            )
            .ok_or(HardcopyRenderError::EmptyPrimitiveGeometry)?;
            surface.push_transform(&Transform::from_translate(
                coordinate_to_points(x),
                coordinate_to_points(y),
            ));
            surface.draw_image(image, size);
            surface.pop();
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
            let x = coordinate_to_points(x);
            let y = coordinate_to_points(y);
            let rotation_degrees = match rotation {
                SceneTextRotation::Upright => None,
                SceneTextRotation::Clockwise90 => Some(90.0),
                SceneTextRotation::CounterClockwise90 => Some(-90.0),
            };
            if let Some(degrees) = rotation_degrees {
                surface.push_transform(&Transform::from_rotate_at(degrees, x, y));
            }
            draw_pdf_text(
                surface,
                fonts.get(*font),
                x,
                y,
                coordinate_to_points(transform.length(*size)),
                text,
                resolve_color(plan, *color),
                *anchor,
                !plan.setup().render().fonts().preserve_searchable_text(),
                NormalizedF32::ONE,
            );
            if rotation_degrees.is_some() {
                surface.pop();
            }
        }
        ScenePrimitive::ClippedGroup {
            source_origin,
            destination_origin,
            primitives,
            ..
        } => {
            let transform = transform.remap(*source_origin, *destination_origin);
            for primitive in primitives {
                draw_pdf_primitive(surface, fonts, plan, transform, primitive)?;
            }
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(super) fn draw_pdf_text(
    surface: &mut krilla::surface::Surface<'_>,
    font: &Font,
    mut x: f32,
    y: f32,
    size: f32,
    text: &str,
    color: Rgb8,
    anchor: TextAnchor,
    outlined: bool,
    opacity: NormalizedF32,
) {
    let estimated_width = text.chars().count() as f32 * size * 0.55;
    match anchor {
        TextAnchor::Start => {}
        TextAnchor::Middle => x -= estimated_width / 2.0,
        TextAnchor::End => x -= estimated_width,
    }
    set_pdf_fill(surface, color, opacity);
    surface.set_stroke(None);
    surface.draw_text(
        Point::from_xy(x, y),
        font.clone(),
        size,
        text,
        outlined,
        TextDirection::LeftToRight,
    );
}

pub(super) fn draw_pdf_decorations(
    surface: &mut krilla::surface::Surface<'_>,
    fonts: &PdfFonts,
    plan: &HardcopyPlan,
    scene: &HardcopyScene,
    page: &PreviewPage,
) {
    let outlined = !plan.setup().render().fonts().preserve_searchable_text();
    let geometry = page.geometry();
    let printable = geometry.printable_rect();
    let content = geometry.content_rect();
    let ink = resolve_color(plan, SemanticColor::Foreground);
    let secondary = resolve_color(plan, SemanticColor::Secondary);
    if plan.setup().decorations().includes_header() {
        let baseline = um_to_points(printable.y.micrometres() + DECORATION_TEXT_UM);
        let left = um_to_points(printable.x.micrometres());
        let right = um_to_points(printable.x.micrometres() + printable.width.micrometres());
        let font_size = um_to_points(DECORATION_TEXT_UM);
        draw_pdf_text(
            surface,
            &fonts.semibold,
            left,
            baseline,
            font_size,
            scene.metadata.title(),
            ink,
            TextAnchor::Start,
            outlined,
            NormalizedF32::ONE,
        );
        draw_pdf_text(
            surface,
            &fonts.mono,
            right,
            baseline,
            font_size,
            &format!(
                "rev {} · page {} / {} · {}",
                plan.source().revision().get(),
                page.number(),
                plan.pagination().pages().len(),
                page.coordinate()
            ),
            secondary,
            TextAnchor::End,
            outlined,
            NormalizedF32::ONE,
        );
        if let Some(line) = scene.metadata.header_lines.first() {
            draw_pdf_text(
                surface,
                &fonts.sans,
                (left + right) / 2.0,
                baseline,
                font_size,
                line,
                secondary,
                TextAnchor::Middle,
                outlined,
                NormalizedF32::ONE,
            );
        }
    }
    if plan.setup().decorations().includes_provenance() {
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
        draw_pdf_text(
            surface,
            &fonts.mono,
            um_to_points(printable.x.micrometres()),
            um_to_points(printable.y.micrometres() + printable.height.micrometres() - 2_000),
            um_to_points(2_200),
            &text,
            secondary,
            TextAnchor::Start,
            outlined,
            NormalizedF32::ONE,
        );
    }
    if plan.setup().decorations().includes_legends() && !scene.legend.is_empty() {
        let legend_band = geometry.legend_band().micrometres();
        let columns = printable.width.micrometres() / LEGEND_COLUMN_UM;
        let rows = legend_band.saturating_sub(LEGEND_VERTICAL_PADDING_UM) / LEGEND_ROW_UM;
        let legend_top = content
            .y
            .micrometres()
            .saturating_add(content.height.micrometres());
        let background = Rect::from_xywh(
            um_to_points(printable.x.micrometres()),
            um_to_points(legend_top),
            um_to_points(printable.width.micrometres()),
            um_to_points(legend_band),
        );
        if let Some(background) = background {
            set_pdf_fill(
                surface,
                Rgb8::new(255, 255, 255),
                NormalizedF32::new(0.92).expect("valid opacity"),
            );
            set_pdf_stroke(
                surface,
                ResolvedStroke {
                    color: secondary,
                    width_um: 150.0,
                    pattern: StrokePattern::Solid,
                    exact_dash_um: None,
                    exact_dot_spacing_um: None,
                },
            );
            let mut builder = PathBuilder::new();
            builder.push_rect(background);
            if let Some(path) = builder.finish() {
                surface.draw_path(&path);
            }
        }
        for (index, entry) in scene.legend.iter().enumerate() {
            let index = index as u64;
            let column = index / rows.max(1);
            let row = index % rows.max(1);
            if column >= columns {
                break;
            }
            let x_um = printable
                .x
                .micrometres()
                .saturating_add(column.saturating_mul(LEGEND_COLUMN_UM))
                .saturating_add(2_000);
            let y_um = legend_top
                .saturating_add(3_500)
                .saturating_add(row.saturating_mul(LEGEND_ROW_UM));
            let mut builder = PathBuilder::new();
            let transform = page_transform(page);
            if entry.fill.is_some() {
                let swatch_y = y_um.saturating_sub(2_000);
                let Some(swatch) = Rect::from_xywh(
                    um_to_points(x_um),
                    um_to_points(swatch_y),
                    um_to_points(12_000),
                    um_to_points(3_000),
                ) else {
                    continue;
                };
                builder.push_rect(swatch);
                set_pdf_scene_fill(surface, plan, entry.fill);
                set_pdf_stroke(surface, resolve_stroke(plan, transform, entry.stroke));
                if let Some(path) = builder.finish() {
                    surface.draw_path(&path);
                    draw_pdf_cross_hatch(
                        surface,
                        plan,
                        1.0,
                        &path,
                        (
                            um_to_points(x_um),
                            um_to_points(swatch_y),
                            um_to_points(12_000),
                            um_to_points(3_000),
                        ),
                        entry.fill,
                    );
                }
            } else {
                builder.move_to(um_to_points(x_um), um_to_points(y_um));
                builder.line_to(um_to_points(x_um + 12_000), um_to_points(y_um));
                surface.set_fill(None);
                set_pdf_stroke(surface, resolve_stroke(plan, transform, entry.stroke));
                if let Some(path) = builder.finish() {
                    surface.draw_path(&path);
                }
            }
            draw_pdf_text(
                surface,
                &fonts.sans,
                um_to_points(x_um + 15_000),
                um_to_points(y_um + 900),
                um_to_points(2_600),
                &entry.label,
                ink,
                TextAnchor::Start,
                outlined,
                NormalizedF32::ONE,
            );
        }
    }
    draw_pdf_watermark(surface, fonts, plan, page);
    draw_pdf_registration_marks(surface, fonts, plan, page);
}

pub(super) fn draw_pdf_watermark(
    surface: &mut krilla::surface::Surface<'_>,
    fonts: &PdfFonts,
    plan: &HardcopyPlan,
    page: &PreviewPage,
) {
    let text = match plan.setup().decorations().watermark() {
        Watermark::None => return,
        Watermark::Draft => "DRAFT",
        Watermark::Confidential => "CONFIDENTIAL",
        Watermark::Custom(text) => text,
    };
    let (width, height) = page.geometry().physical_size();
    let cx = um_to_points(width.micrometres()) / 2.0;
    let cy = um_to_points(height.micrometres()) / 2.0;
    let size = um_to_points(width.micrometres().min(height.micrometres()) / 10);
    surface.push_transform(&Transform::from_rotate_at(-35.0, cx, cy));
    draw_pdf_text(
        surface,
        &fonts.semibold,
        cx,
        cy,
        size,
        text,
        resolve_color(plan, SemanticColor::Secondary),
        TextAnchor::Middle,
        !plan.setup().render().fonts().preserve_searchable_text(),
        NormalizedF32::new(0.16).expect("valid opacity"),
    );
    surface.pop();
}

pub(super) fn draw_pdf_registration_marks(
    surface: &mut krilla::surface::Surface<'_>,
    fonts: &PdfFonts,
    plan: &HardcopyPlan,
    page: &PreviewPage,
) {
    if !plan.setup().tiling().registration_marks_and_coordinates() {
        return;
    }
    let rect = page.geometry().content_rect();
    let left = rect.x.micrometres();
    let right = left + rect.width.micrometres();
    let top = rect.y.micrometres();
    let bottom = top + rect.height.micrometres();
    let mut builder = PathBuilder::new();
    for (x, y) in [(left, top), (right, top), (left, bottom), (right, bottom)] {
        builder.move_to(um_to_points(x.saturating_sub(2_000)), um_to_points(y));
        builder.line_to(um_to_points(x + 2_000), um_to_points(y));
        builder.move_to(um_to_points(x), um_to_points(y.saturating_sub(2_000)));
        builder.line_to(um_to_points(x), um_to_points(y + 2_000));
    }
    surface.set_fill(None);
    set_pdf_stroke(
        surface,
        ResolvedStroke {
            color: resolve_color(plan, SemanticColor::Secondary),
            width_um: 150.0,
            pattern: StrokePattern::Solid,
            exact_dash_um: None,
            exact_dot_spacing_um: None,
        },
    );
    if let Some(path) = builder.finish() {
        surface.draw_path(&path);
    }
    draw_pdf_text(
        surface,
        &fonts.mono,
        um_to_points(left + 2_500),
        um_to_points(top + 3_000),
        um_to_points(2_200),
        page.coordinate(),
        resolve_color(plan, SemanticColor::Secondary),
        TextAnchor::Start,
        !plan.setup().render().fonts().preserve_searchable_text(),
        NormalizedF32::ONE,
    );
}

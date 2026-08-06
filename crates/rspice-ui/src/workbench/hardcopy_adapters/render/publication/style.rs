//! Shared publication styling for vector and raster hardcopy backends.
//!
//! This module keeps color, dash, hatch, and text policies backend-neutral so
//! SVG, PDF, preview, and printer output remain visually equivalent.

use super::*;

pub(super) fn page_primitives<'a>(
    scene: &'a HardcopyScene,
    page: &PreviewPage,
) -> Result<&'a [ScenePrimitive], HardcopyRenderError> {
    if scene.aggregate_sections.is_empty() {
        return Ok(&scene.primitives);
    }
    let section = scene
        .aggregate_sections
        .iter()
        .find(|section| section.ordinal == page.section_ordinal())
        .ok_or(HardcopyRenderError::AggregatePaginationMismatch)?;
    scene
        .primitives
        .get(section.primitive_start..section.primitive_end)
        .ok_or(HardcopyRenderError::AggregatePaginationMismatch)
}

impl PageTransform {
    pub(super) fn point(self, point: ScenePoint) -> (f64, f64) {
        (
            self.axis_for(
                self.content_rect.x.micrometres(),
                point.x.micrometres(),
                self.window.x.micrometres(),
                self.source_origin.x.micrometres(),
                self.destination_origin.x.micrometres(),
            ),
            self.axis_for(
                self.content_rect.y.micrometres(),
                point.y.micrometres(),
                self.window.y.micrometres(),
                self.source_origin.y.micrometres(),
                self.destination_origin.y.micrometres(),
            ),
        )
    }

    pub(super) fn axis_for(
        self,
        page_origin: u64,
        source: u64,
        window_origin: u64,
        source_origin: u64,
        destination_origin: u64,
    ) -> f64 {
        page_origin as f64
            + (source as f64 - source_origin as f64 + destination_origin as f64)
                * self.scale.numerator() as f64
                / self.scale.denominator() as f64
            - window_origin as f64
    }

    pub(super) fn remap(self, source_origin: ScenePoint, destination_origin: ScenePoint) -> Self {
        Self {
            source_origin,
            destination_origin,
            ..self
        }
    }

    pub(super) fn length(self, value: Length) -> f64 {
        value.micrometres() as f64 * self.scale.numerator() as f64 / self.scale.denominator() as f64
    }

    pub(super) fn scale_factor(self) -> f64 {
        self.scale.numerator() as f64 / self.scale.denominator() as f64
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct ResolvedStroke {
    pub(super) color: Rgb8,
    pub(super) width_um: f64,
    pub(super) pattern: StrokePattern,
    pub(super) exact_dash_um: Option<(f64, f64)>,
    pub(super) exact_dot_spacing_um: Option<f64>,
}

pub(super) fn page_transform(page: &PreviewPage) -> PageTransform {
    PageTransform {
        content_rect: page.geometry().content_rect(),
        window: page.scaled_content_window(),
        scale: page.scale(),
        source_origin: ScenePoint::new(Length::ZERO, Length::ZERO),
        destination_origin: ScenePoint::new(Length::ZERO, Length::ZERO),
    }
}

pub(super) fn resolve_stroke(
    plan: &HardcopyPlan,
    transform: PageTransform,
    style: StrokeStyle,
) -> ResolvedStroke {
    let mapping = plan.setup().render().color_mapping();
    let pattern = if mapping == ColorMapping::GrayscaleWithDashMarkerRedundancy
        && style.pattern == StrokePattern::Solid
    {
        style
            .series_index
            .map(auto_trace_pattern)
            .unwrap_or(StrokePattern::Solid)
    } else {
        style.pattern
    };
    ResolvedStroke {
        color: resolve_color(plan, style.color),
        width_um: transform.length(style.width).max(80.0),
        pattern,
        exact_dash_um: style
            .exact_dash
            .map(|(dash, gap)| (transform.length(dash), transform.length(gap))),
        exact_dot_spacing_um: style
            .exact_dot_spacing
            .map(|spacing| transform.length(spacing)),
    }
}

pub(in crate::workbench::hardcopy_adapters::render) fn auto_trace_pattern(
    index: u16,
) -> StrokePattern {
    match index % 4 {
        0 => StrokePattern::Solid,
        1 => StrokePattern::Dashed,
        2 => StrokePattern::Dotted,
        _ => StrokePattern::DashDot,
    }
}

pub(super) fn base_semantic_color(color: SemanticColor) -> Rgb8 {
    const TRACES: [Rgb8; 10] = [
        Rgb8::new(0, 113, 188),
        Rgb8::new(213, 94, 0),
        Rgb8::new(0, 158, 115),
        Rgb8::new(204, 121, 167),
        Rgb8::new(230, 159, 0),
        Rgb8::new(86, 180, 233),
        Rgb8::new(111, 78, 161),
        Rgb8::new(0, 0, 0),
        Rgb8::new(56, 131, 74),
        Rgb8::new(180, 65, 65),
    ];
    match color {
        SemanticColor::Foreground => Rgb8::new(25, 32, 38),
        SemanticColor::Secondary => Rgb8::new(91, 102, 112),
        SemanticColor::Grid => Rgb8::new(181, 188, 194),
        SemanticColor::Accent => Rgb8::new(196, 139, 0),
        SemanticColor::Warning => Rgb8::new(190, 67, 54),
        SemanticColor::Success => Rgb8::new(0, 125, 82),
        SemanticColor::Trace(index) => TRACES[usize::from(index) % TRACES.len()],
        SemanticColor::Exact(value) => value,
    }
}

pub(super) fn resolve_color(plan: &HardcopyPlan, color: SemanticColor) -> Rgb8 {
    let source = if plan.setup().render().background() == BackgroundMode::WorkspaceBackground {
        match color {
            SemanticColor::Foreground => Rgb8::new(224, 230, 234),
            SemanticColor::Secondary => Rgb8::new(166, 176, 184),
            SemanticColor::Grid => Rgb8::new(66, 78, 86),
            _ => base_semantic_color(color),
        }
    } else {
        base_semantic_color(color)
    };
    match plan.setup().render().color_mapping() {
        ColorMapping::ScreenColors => source,
        ColorMapping::PrintSafeEngineeringPalette => print_safe(source),
        ColorMapping::GrayscaleWithDashMarkerRedundancy => {
            let luminance = ((u32::from(source.red) * 54
                + u32::from(source.green) * 183
                + u32::from(source.blue) * 19)
                / 256) as u8;
            Rgb8::new(luminance, luminance, luminance)
        }
        ColorMapping::Monochrome => Rgb8::new(0, 0, 0),
    }
}

pub(super) fn print_safe(color: Rgb8) -> Rgb8 {
    // Constrain saturated screen colors to a reproducible, contrast-safe sRGB
    // print palette. Exact black stays black; near-white ink is darkened.
    if color == Rgb8::new(0, 0, 0) {
        return color;
    }
    let clamp = |value: u8| value.clamp(18, 220);
    let mut resolved = Rgb8::new(clamp(color.red), clamp(color.green), clamp(color.blue));
    let luminance = (u32::from(resolved.red) * 54
        + u32::from(resolved.green) * 183
        + u32::from(resolved.blue) * 19)
        / 256;
    if luminance > 196 {
        resolved.red = resolved.red.saturating_sub(42);
        resolved.green = resolved.green.saturating_sub(42);
        resolved.blue = resolved.blue.saturating_sub(42);
    }
    resolved
}

pub(super) fn background_color(plan: &HardcopyPlan) -> Option<Rgb8> {
    match plan.setup().render().background() {
        BackgroundMode::White => Some(Rgb8::new(255, 255, 255)),
        BackgroundMode::WorkspaceBackground => Some(Rgb8::new(17, 24, 28)),
        BackgroundMode::Transparent => None,
    }
}

pub(super) fn svg_color(color: Rgb8) -> String {
    format!("#{:02x}{:02x}{:02x}", color.red, color.green, color.blue)
}

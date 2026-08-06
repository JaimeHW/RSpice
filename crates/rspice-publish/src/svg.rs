//! Scene → SVG emission.
//!
//! Scenes arrive fully resolved in integral micrometres, so this module is a
//! mechanical translation: paths become `<path>` data, full-turn arcs become
//! `<circle>`, text becomes `<text>`. Paint roles become prefixed CSS classes
//! (`s-` stroke, `f-` fill, `t-` text) so one sealed scene renders correctly
//! in the page's light, dark, and print palettes; explicit RGBA survives
//! verbatim as author intent.

use std::fmt::Write as _;

use rspice_publication_contract::{
    GroupTag, Paint, PaintRole, PathPrimitive, PathSegment, Point, Primitive, PrimitiveGroup,
    Scene, Stroke, StrokePattern, TextAnchor, TextPrimitive,
};

use crate::escape_html;

/// Full turn in the contract's millidegree unit.
const FULL_TURN_MILLIDEG: i64 = 360_000;
/// The page palette defines this many trace series colors; higher indices
/// wrap deterministically.
const TRACE_PALETTE_SIZE: u8 = 8;

fn fmt_um(value: i64) -> String {
    value.to_string()
}

fn fmt_f64(value: f64) -> String {
    format!("{value}")
}

fn role_token(role: PaintRole) -> String {
    match role {
        PaintRole::SheetFrame => "sheet-frame".to_string(),
        PaintRole::SymbolBody => "symbol-body".to_string(),
        PaintRole::Wire => "wire".to_string(),
        PaintRole::Bus => "bus".to_string(),
        PaintRole::Junction => "junction".to_string(),
        PaintRole::Pin => "pin".to_string(),
        PaintRole::NetLabel => "net-label".to_string(),
        PaintRole::ReferenceDesignator => "reference-designator".to_string(),
        PaintRole::ComponentValue => "component-value".to_string(),
        PaintRole::Annotation => "annotation".to_string(),
        PaintRole::PlotFrame => "plot-frame".to_string(),
        PaintRole::PlotGrid => "plot-grid".to_string(),
        PaintRole::PlotAxisText => "plot-axis-text".to_string(),
        PaintRole::TraceSeries(index) => format!("trace-{}", index % TRACE_PALETTE_SIZE),
    }
}

fn rgba_css([r, g, b, a]: [u8; 4]) -> String {
    if a == 255 {
        format!("rgb({r} {g} {b})")
    } else {
        format!("rgb({r} {g} {b} / {})", fmt_f64(f64::from(a) / 255.0))
    }
}

/// Accumulates one element's class list and explicit presentation
/// attributes, guaranteeing a single well-formed `class` attribute.
#[derive(Default)]
struct Presentation {
    classes: Vec<String>,
    attributes: String,
}

impl Presentation {
    fn stroke(&mut self, stroke: &Stroke) {
        match stroke.paint {
            Paint::Role(role) => self.classes.push(format!("s-{}", role_token(role))),
            Paint::Rgba(color) => {
                let _ = write!(self.attributes, " stroke=\"{}\"", rgba_css(color));
            }
        }
        let _ = write!(
            self.attributes,
            " stroke-width=\"{}\"",
            fmt_um(stroke.width_um as i64)
        );
        match stroke.pattern {
            StrokePattern::Solid => {}
            StrokePattern::Dashed => {
                let dash = stroke.width_um.saturating_mul(4).max(1);
                let gap = stroke.width_um.saturating_mul(2).max(1);
                let _ = write!(self.attributes, " stroke-dasharray=\"{dash} {gap}\"");
            }
            StrokePattern::Dotted => {
                let dot = stroke.width_um.max(1);
                let gap = stroke.width_um.saturating_mul(3).max(1);
                let _ = write!(self.attributes, " stroke-dasharray=\"{dot} {gap}\"");
            }
        }
    }

    fn fill(&mut self, fill: Option<&Paint>) {
        match fill {
            None => self.attributes.push_str(" fill=\"none\""),
            Some(Paint::Role(role)) => self.classes.push(format!("f-{}", role_token(*role))),
            Some(Paint::Rgba(color)) => {
                let _ = write!(self.attributes, " fill=\"{}\"", rgba_css(*color));
            }
        }
    }

    fn text_paint(&mut self, paint: Paint) {
        match paint {
            Paint::Role(role) => self.classes.push(format!("t-{}", role_token(role))),
            Paint::Rgba(color) => {
                let _ = write!(self.attributes, " fill=\"{}\"", rgba_css(color));
            }
        }
    }

    fn markup(self) -> String {
        if self.classes.is_empty() {
            self.attributes
        } else {
            format!(" class=\"{}\"{}", self.classes.join(" "), self.attributes)
        }
    }
}

/// Convert one arc segment into SVG path arc commands, or `None` when the
/// sweep covers a full turn and the caller should emit a `<circle>` instead.
fn arc_path_data(
    center: Point,
    radius_um: u64,
    start_millideg: i32,
    sweep_millideg: i32,
) -> Option<String> {
    if i64::from(sweep_millideg).abs() >= FULL_TURN_MILLIDEG {
        return None;
    }
    let radius = radius_um as f64;
    let start = f64::from(start_millideg) / 1000.0;
    let sweep = f64::from(sweep_millideg) / 1000.0;
    let end = start + sweep;
    let center_x = center.x_um as f64;
    let center_y = center.y_um as f64;
    // Scene y grows downward while contract angles are counter-clockwise
    // positive in conventional orientation, hence the negated sine terms and
    // the inverted SVG sweep flag below.
    let (sx, sy) = (
        center_x + radius * start.to_radians().cos(),
        center_y - radius * start.to_radians().sin(),
    );
    let (ex, ey) = (
        center_x + radius * end.to_radians().cos(),
        center_y - radius * end.to_radians().sin(),
    );
    let large_arc = i32::from(sweep.abs() > 180.0);
    let sweep_flag = i32::from(sweep < 0.0);
    Some(format!(
        "M {} {} A {} {} 0 {} {} {} {}",
        fmt_f64(sx),
        fmt_f64(sy),
        fmt_um(radius_um as i64),
        fmt_um(radius_um as i64),
        large_arc,
        sweep_flag,
        fmt_f64(ex),
        fmt_f64(ey),
    ))
}

fn path_markup(path: &PathPrimitive) -> String {
    let mut presentation = Presentation::default();
    if let Some(stroke) = &path.stroke {
        presentation.stroke(stroke);
    }
    presentation.fill(path.fill.as_ref());
    let presentation = presentation.markup();

    // A lone full-turn arc renders as a circle for exactness.
    if let [
        PathSegment::Arc {
            center,
            radius_um,
            sweep_millideg,
            ..
        },
    ] = path.segments.as_slice()
        && i64::from(*sweep_millideg).abs() >= FULL_TURN_MILLIDEG
    {
        return format!(
            "<circle cx=\"{}\" cy=\"{}\" r=\"{}\"{presentation}/>",
            fmt_um(center.x_um),
            fmt_um(center.y_um),
            fmt_um(*radius_um as i64),
        );
    }

    let mut data = String::new();
    for segment in &path.segments {
        if !data.is_empty() {
            data.push(' ');
        }
        match segment {
            PathSegment::MoveTo { to } => {
                let _ = write!(data, "M {} {}", fmt_um(to.x_um), fmt_um(to.y_um));
            }
            PathSegment::LineTo { to } => {
                let _ = write!(data, "L {} {}", fmt_um(to.x_um), fmt_um(to.y_um));
            }
            PathSegment::Arc {
                center,
                radius_um,
                start_millideg,
                sweep_millideg,
            } => {
                let arc = arc_path_data(*center, *radius_um, *start_millideg, *sweep_millideg)
                    .unwrap_or_default();
                data.push_str(&arc);
            }
            PathSegment::Close => data.push('Z'),
        }
    }
    format!("<path d=\"{data}\"{presentation}/>")
}

fn text_markup(text: &TextPrimitive) -> String {
    let anchor = match text.anchor {
        TextAnchor::Start => "start",
        TextAnchor::Middle => "middle",
        TextAnchor::End => "end",
    };
    let mut presentation = Presentation::default();
    presentation.text_paint(text.paint);
    let presentation = presentation.markup();
    let rotation = if text.rotation_millideg == 0 {
        String::new()
    } else {
        // SVG rotation is clockwise-positive in screen space; the contract
        // is counter-clockwise positive.
        format!(
            " transform=\"rotate({} {} {})\"",
            fmt_f64(-f64::from(text.rotation_millideg) / 1000.0),
            fmt_um(text.origin.x_um),
            fmt_um(text.origin.y_um),
        )
    };
    format!(
        "<text x=\"{}\" y=\"{}\" font-size=\"{}\" text-anchor=\"{anchor}\"{presentation}{rotation}>{}</text>",
        fmt_um(text.origin.x_um),
        fmt_um(text.origin.y_um),
        fmt_um(text.height_um as i64),
        escape_html(&text.text),
    )
}

fn group_attributes(tag: Option<&GroupTag>) -> String {
    match tag {
        None => String::new(),
        Some(GroupTag::Instance { reference }) => {
            format!(
                " class=\"instance\" data-instance=\"{}\"",
                escape_html(reference)
            )
        }
        Some(GroupTag::Net { name }) => {
            format!(" class=\"net\" data-net=\"{}\"", escape_html(name))
        }
        Some(GroupTag::SheetFrame) => " class=\"frame\"".to_string(),
        Some(GroupTag::Annotation) => " class=\"note\"".to_string(),
    }
}

fn group_markup(group: &PrimitiveGroup) -> String {
    let mut markup = format!("<g{}>", group_attributes(group.tag.as_ref()));
    for primitive in &group.primitives {
        match primitive {
            Primitive::Path(path) => markup.push_str(&path_markup(path)),
            Primitive::Text(text) => markup.push_str(&text_markup(text)),
        }
    }
    markup.push_str("</g>");
    markup
}

/// Render one scene as an inline SVG element sized for print fidelity:
/// user units are micrometres and the physical size is declared in
/// millimetres, so a sheet prints at exactly its authored dimensions.
pub fn scene_svg(scene: &Scene, accessible_title: &str) -> String {
    let width_mm = fmt_f64(scene.width_um as f64 / 1000.0);
    let height_mm = fmt_f64(scene.height_um as f64 / 1000.0);
    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {} {}\" width=\"{width_mm}mm\" height=\"{height_mm}mm\" role=\"img\" aria-label=\"{}\" fill=\"none\" stroke-linecap=\"round\" stroke-linejoin=\"round\" font-family=\"'Cascadia Code', Consolas, 'DejaVu Sans Mono', monospace\">",
        fmt_um(scene.width_um as i64),
        fmt_um(scene.height_um as i64),
        escape_html(accessible_title),
    );
    for group in &scene.groups {
        svg.push_str(&group_markup(group));
    }
    svg.push_str("</svg>");
    svg
}

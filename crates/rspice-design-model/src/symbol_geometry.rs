//! Shared authored symbol shapes, transforms and text metrics.

use serde::{Deserialize, Serialize};

use crate::Point;

/// Drawn height of a symbol text run, in symbol coordinate units.
///
/// The sizes are the ones the rest of a symbol already draws at, so authored
/// text sits in the same type hierarchy as the artwork around it: a pin name,
/// an instance name, and a heading above both.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolTextSize {
    Small,
    #[default]
    Normal,
    Large,
}

impl SymbolTextSize {
    pub const ALL: [Self; 3] = [Self::Small, Self::Normal, Self::Large];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Normal => "normal",
            Self::Large => "large",
        }
    }

    /// Cap height in symbol units. Renderers scale it to their viewport, so
    /// a text run keeps its proportion against the body at every zoom.
    pub const fn height(self) -> i32 {
        match self {
            Self::Small => 5,
            Self::Normal => 9,
            Self::Large => 14,
        }
    }
}

/// Which side of its anchor a symbol text run hangs off.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolTextAlign {
    #[default]
    Left,
    Center,
    Right,
}

impl SymbolTextAlign {
    pub const ALL: [Self; 3] = [Self::Left, Self::Center, Self::Right];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Left => "left",
            Self::Center => "center",
            Self::Right => "right",
        }
    }

    /// The alignment a mirror across the run produces. A run is never read
    /// backward, so a mirror can only move it to the other side of its
    /// anchor.
    pub const fn mirrored(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Center => Self::Center,
            Self::Right => Self::Left,
        }
    }

    /// Where a run reading `rotation`'s way sits once `orient` — the placed
    /// instance's own transform — has carried its advance direction into
    /// world space.
    ///
    /// The single owner of the rule, because every surface that draws
    /// authored text spells the answer differently and none of them may
    /// disagree about it.
    pub fn placement(
        self,
        rotation: SymbolTextRotation,
        orient: impl Fn(Point) -> Point,
    ) -> SymbolTextPlacement {
        let Some(step) = self.run_step(rotation) else {
            return SymbolTextPlacement::On;
        };
        let run = orient(step);
        if run.x.abs() >= run.y.abs() {
            if run.x >= 0 {
                SymbolTextPlacement::After
            } else {
                SymbolTextPlacement::Before
            }
        } else if run.y >= 0 {
            SymbolTextPlacement::Below
        } else {
            SymbolTextPlacement::Above
        }
    }

    /// The direction the run advances away from its anchor, or `None` when
    /// it straddles the anchor and no orientation can move it off. A run
    /// read upward starts at its anchor and climbs from it.
    const fn run_step(self, rotation: SymbolTextRotation) -> Option<Point> {
        match (self, rotation) {
            (Self::Center, _) => None,
            (Self::Left, SymbolTextRotation::R0) => Some(Point::new(1, 0)),
            (Self::Right, SymbolTextRotation::R0) => Some(Point::new(-1, 0)),
            (Self::Left, SymbolTextRotation::R90) => Some(Point::new(0, -1)),
            (Self::Right, SymbolTextRotation::R90) => Some(Point::new(0, 1)),
        }
    }
}

/// Which way a text run reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolTextRotation {
    /// Left to right.
    #[default]
    R0,
    /// Bottom to top: the run turned a quarter counter-clockwise, as a label
    /// set beside a column of pins reads.
    R90,
}

impl SymbolTextRotation {
    pub const ALL: [Self; 2] = [Self::R0, Self::R90];

    pub const fn label(self) -> &'static str {
        match self {
            Self::R0 => "0\u{b0}",
            Self::R90 => "90\u{b0}",
        }
    }

    pub fn is_r0(&self) -> bool {
        *self == Self::R0
    }

    /// The other reading direction: what a quarter turn makes of a run,
    /// since glyphs are never set upside down.
    pub const fn turned(self) -> Self {
        match self {
            Self::R0 => Self::R90,
            Self::R90 => Self::R0,
        }
    }
}

/// Which side of its anchor an oriented text run ends up on.
///
/// A run keeps its reading under every turn or mirror of a placed instance,
/// so an orientation can only move the run around its anchor; this names
/// where it landed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolTextPlacement {
    /// Straddling the anchor, wherever the instance is turned.
    On,
    After,
    Before,
    Below,
    Above,
}

/// The box a text run occupies in symbol units, centred across its anchor:
/// vertically for a run read left to right, horizontally for one read
/// upward.
///
/// The single owner of symbol type metrics. IBM Plex Mono advances 600/1000
/// em, so a glyph of cap height `h` is `3h/5` wide; every renderer sets that
/// same face and size, so what this measures is what they draw.
pub fn symbol_text_bounds(
    anchor: Point,
    text: &str,
    size: SymbolTextSize,
    align: SymbolTextAlign,
    rotation: SymbolTextRotation,
) -> (Point, Point) {
    let glyphs = i32::try_from(text.chars().count()).unwrap_or(i32::MAX);
    let span = glyphs.saturating_mul(size.height() * 3 / 5);
    let (before, after) = match align {
        SymbolTextAlign::Left => (0, span),
        SymbolTextAlign::Center => (span / 2, span / 2),
        SymbolTextAlign::Right => (span, 0),
    };
    let half_height = size.height() / 2;
    match rotation {
        SymbolTextRotation::R0 => (
            Point::new(
                anchor.x.saturating_sub(before),
                anchor.y.saturating_sub(half_height),
            ),
            Point::new(
                anchor.x.saturating_add(after),
                anchor.y.saturating_add(half_height),
            ),
        ),
        // Read upward, what follows the anchor is above it.
        SymbolTextRotation::R90 => (
            Point::new(
                anchor.x.saturating_sub(half_height),
                anchor.y.saturating_sub(after),
            ),
            Point::new(
                anchor.x.saturating_add(half_height),
                anchor.y.saturating_add(before),
            ),
        ),
    }
}

/// How heavy a shape's outline is, against the weight a renderer draws a
/// symbol body at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolStroke {
    Thin,
    #[default]
    Normal,
    Heavy,
}

impl SymbolStroke {
    pub const ALL: [Self; 3] = [Self::Thin, Self::Normal, Self::Heavy];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Thin => "thin",
            Self::Normal => "normal",
            Self::Heavy => "heavy",
        }
    }

    /// The outline's width as a multiple of the body weight.
    pub const fn scale(self) -> f32 {
        match self {
            Self::Thin => 0.5,
            Self::Normal => 1.0,
            Self::Heavy => 2.0,
        }
    }
}

/// Whether a shape's outline is drawn whole or broken.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolDash {
    #[default]
    Solid,
    Dashed,
    Dotted,
}

impl SymbolDash {
    pub const ALL: [Self; 3] = [Self::Solid, Self::Dashed, Self::Dotted];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Solid => "solid",
            Self::Dashed => "dashed",
            Self::Dotted => "dotted",
        }
    }

    /// The drawn and the skipped length of one step of the pattern, in
    /// symbol units, so a pattern keeps its proportion against the body at
    /// every zoom; `None` for a whole line.
    pub const fn pattern(self) -> Option<(f32, f32)> {
        match self {
            Self::Solid => None,
            Self::Dashed => Some((6.0, 4.0)),
            Self::Dotted => Some((1.0, 3.0)),
        }
    }
}

/// Whether a closed shape's inside is painted in its outline's ink.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SymbolFill {
    #[default]
    None,
    Solid,
}

impl SymbolFill {
    pub const ALL: [Self; 2] = [Self::None, Self::Solid];

    pub const fn label(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Solid => "solid",
        }
    }
}

/// How a drawn shape's outline and inside are inked. The default is the one
/// weight, whole and unfilled, every symbol drew before shapes had a style.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct SymbolStyle {
    pub stroke: SymbolStroke,
    pub dash: SymbolDash,
    pub fill: SymbolFill,
}

impl SymbolStyle {
    /// Whether this is the style a shape takes when nothing was chosen.
    pub fn is_plain(&self) -> bool {
        *self == Self::default()
    }
}

/// The largest gap, in device pixels, a straight piece may leave from the
/// curve it stands for.
const CHORD_ERROR_PIXELS: f64 = 0.05;
/// The longest straight piece of a curve, in device pixels.
const LONGEST_CHORD_PIXELS: f64 = 2.5;
/// Enough pieces for a curve far larger than any screen.
const MOST_CHORDS: f64 = 2048.0;

/// An ellipse, or the part of one, exactly as the drawing it came from stood
/// it.
///
/// Nothing here is rounded to whole symbol units and no points are kept: each
/// painter makes its strokes for the size it draws at, or writes a true arc.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolCurve {
    pub center: [f64; 2],
    pub radii: [f64; 2],
    /// Where the curve starts, in degrees from +x, rising clockwise on a
    /// screen whose y runs down.
    pub start_degrees: f64,
    /// How far it runs clockwise from there, in degrees: above zero, and 360
    /// for the whole ellipse.
    pub sweep_degrees: f64,
}

// A curve is built from finite numbers and validated with its document, so
// the reflexivity `Eq` promises holds.
impl Eq for SymbolCurve {}

impl SymbolCurve {
    /// The curve an authored whole-unit arc draws.
    pub fn of_arc(center: Point, radius: i32, start_degrees: i32, sweep_degrees: i32) -> Self {
        let (start, sweep) = (f64::from(start_degrees), f64::from(sweep_degrees));
        Self {
            center: [f64::from(center.x), f64::from(center.y)],
            radii: [f64::from(radius); 2],
            start_degrees: if sweep < 0.0 { start + sweep } else { start },
            sweep_degrees: sweep.abs(),
        }
    }

    pub fn is_whole(&self) -> bool {
        self.sweep_degrees >= 360.0
    }

    /// Whether the curve is a circle or part of one.
    pub fn is_circular(&self) -> bool {
        self.radii[0] == self.radii[1]
    }

    pub fn point_at(&self, degrees: f64) -> [f64; 2] {
        let (sin, cos) = degrees.to_radians().sin_cos();
        [
            self.center[0] + self.radii[0] * cos,
            self.center[1] + self.radii[1] * sin,
        ]
    }

    pub fn start_point(&self) -> [f64; 2] {
        self.point_at(self.start_degrees)
    }

    pub fn end_point(&self) -> [f64; 2] {
        self.point_at(self.start_degrees + self.sweep_degrees)
    }

    /// Points along the curve, end to end, close enough together that no
    /// straight piece shows at `pixels_per_unit` device pixels a symbol unit.
    /// A whole ellipse ends on the point it starts from.
    pub fn tessellated(&self, pixels_per_unit: f64) -> Vec<[f64; 2]> {
        let radius = self.radii[0].max(self.radii[1]) * pixels_per_unit;
        let sweep = self.sweep_degrees.to_radians();
        let by_length = sweep * radius / LONGEST_CHORD_PIXELS;
        let by_error = if radius > CHORD_ERROR_PIXELS {
            sweep / (2.0 * (1.0 - CHORD_ERROR_PIXELS / radius).acos())
        } else {
            1.0
        };
        let chords = by_length.max(by_error).ceil().clamp(4.0, MOST_CHORDS) as usize;
        (0..=chords)
            .map(|chord| {
                self.point_at(
                    self.start_degrees + self.sweep_degrees * chord as f64 / chords as f64,
                )
            })
            .collect()
    }

    /// The least box around the curve, as its least and greatest corners.
    pub fn bounds(&self) -> ([f64; 2], [f64; 2]) {
        let from = self.start_degrees;
        let to = from + self.sweep_degrees;
        let mut quarter = (from / 90.0).ceil() as i64;
        let mut least = self.start_point();
        let mut greatest = least;
        let mut include = |point: [f64; 2]| {
            for axis in 0..2 {
                least[axis] = least[axis].min(point[axis]);
                greatest[axis] = greatest[axis].max(point[axis]);
            }
        };
        include(self.end_point());
        while (quarter as f64) * 90.0 <= to {
            include(self.point_at(quarter as f64 * 90.0));
            quarter += 1;
        }
        (least, greatest)
    }

    /// [`Self::bounds`] grown out to whole symbol units.
    pub fn whole_unit_bounds(&self) -> (Point, Point) {
        let (least, greatest) = self.bounds();
        (
            Point::new(least[0].floor() as i32, least[1].floor() as i32),
            Point::new(greatest[0].ceil() as i32, greatest[1].ceil() as i32),
        )
    }

    /// This curve once the whole-unit transform carrying `(x, y)` to
    /// `origin + x * across + y * down` has placed it. `across` and `down`
    /// are a quarter turn, a mirror or both, so the curve stays an ellipse
    /// square to the axes.
    #[must_use]
    pub fn placed(&self, origin: Point, across: Point, down: Point) -> Self {
        let (ax, ay) = (f64::from(across.x), f64::from(across.y));
        let (dx, dy) = (f64::from(down.x), f64::from(down.y));
        let [x, y] = self.center;
        // Where a point of the unit circle at `degrees` lands.
        let turned = |degrees: f64| {
            let (sin, cos) = degrees.to_radians().sin_cos();
            (ay * cos + dy * sin)
                .atan2(ax * cos + dx * sin)
                .to_degrees()
        };
        let mirrored = ax * dy - ay * dx < 0.0;
        let start = if mirrored {
            turned(self.start_degrees + self.sweep_degrees)
        } else {
            turned(self.start_degrees)
        };
        Self {
            center: [
                f64::from(origin.x) + ax * x + dx * y,
                f64::from(origin.y) + ay * x + dy * y,
            ],
            radii: if across.x != 0 {
                self.radii
            } else {
                [self.radii[1], self.radii[0]]
            },
            start_degrees: start.rem_euclid(360.0),
            sweep_degrees: self.sweep_degrees,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum SymbolShape {
    Polyline {
        points: Vec<Point>,
        closed: bool,
        #[serde(default, skip_serializing_if = "SymbolStyle::is_plain")]
        style: SymbolStyle,
    },
    /// An axis-aligned box between two opposite corners, `min` never right
    /// of or below `max`. Its own kind rather than a closed polyline, so an
    /// edit that drags one corner keeps it a rectangle.
    Rectangle {
        min: Point,
        max: Point,
        #[serde(default, skip_serializing_if = "SymbolStyle::is_plain")]
        style: SymbolStyle,
    },
    Circle {
        center: Point,
        radius: i32,
        #[serde(default, skip_serializing_if = "SymbolStyle::is_plain")]
        style: SymbolStyle,
    },
    Arc {
        center: Point,
        radius: i32,
        start_degrees: i32,
        sweep_degrees: i32,
        #[serde(default, skip_serializing_if = "SymbolStyle::is_plain")]
        style: SymbolStyle,
    },
    /// An ellipse or arc a drawing brought in, unrounded.
    Curve {
        curve: SymbolCurve,
        #[serde(default, skip_serializing_if = "SymbolStyle::is_plain")]
        style: SymbolStyle,
    },
    Arrow {
        tip: Point,
        rotation_quarters: i32,
    },
    Dot {
        center: Point,
        radius: i32,
    },
    /// A run of authored glyphs. The anchor is geometry and moves with every
    /// transform. The run reads left to right, or upward once turned a
    /// quarter in the editor; on a placed instance it keeps that reading
    /// however the instance is turned or mirrored, and only moves round its
    /// anchor, which is what keeps a turned symbol legible.
    Text {
        anchor: Point,
        text: String,
        size: SymbolTextSize,
        align: SymbolTextAlign,
        #[serde(default, skip_serializing_if = "SymbolTextRotation::is_r0")]
        rotation: SymbolTextRotation,
    },
    /// One of the symbol's parameters lettered as `name=value`: a placed
    /// instance letters its own value of `parameter`, or `default` where it
    /// sets none, and the editor letters `default`. It moves, turns and
    /// mirrors as a text run does. A hidden label stays in the symbol, so it
    /// can be shown again; only placed instances leave it out.
    Label {
        anchor: Point,
        parameter: String,
        #[serde(default, skip_serializing_if = "String::is_empty")]
        default: String,
        size: SymbolTextSize,
        align: SymbolTextAlign,
        #[serde(default, skip_serializing_if = "SymbolTextRotation::is_r0")]
        rotation: SymbolTextRotation,
        #[serde(default = "label_shown", skip_serializing_if = "is_shown")]
        shown: bool,
    },
}

const fn label_shown() -> bool {
    true
}

fn is_shown(shown: &bool) -> bool {
    *shown
}

/// What a label letters for `parameter` holding `value`.
pub fn symbol_label_text(parameter: &str, value: &str) -> String {
    format!("{parameter}={value}")
}

/// What a label letters where no instance's value is known: its default,
/// or a mark holding the place of one.
pub fn symbol_label_sample(parameter: &str, default: &str) -> String {
    symbol_label_text(parameter, if default.is_empty() { "?" } else { default })
}

/// A rectangle's outline, clockwise on screen from its top-left corner: the
/// order every renderer draws it in, so a box reads the same everywhere.
pub fn rectangle_corners(min: Point, max: Point) -> [Point; 4] {
    [min, Point::new(max.x, min.y), max, Point::new(min.x, max.y)]
}

impl SymbolShape {
    /// The rectangle with `a` and `b` as opposite corners, in either order.
    pub fn rectangle(a: Point, b: Point) -> Self {
        SymbolShape::Rectangle {
            min: Point::new(a.x.min(b.x), a.y.min(b.y)),
            max: Point::new(a.x.max(b.x), a.y.max(b.y)),
            style: SymbolStyle::default(),
        }
    }

    /// The curve `curve`, drawn in the plain style.
    pub fn curve(curve: SymbolCurve) -> Self {
        SymbolShape::Curve {
            curve,
            style: SymbolStyle::default(),
        }
    }

    /// How the shape's outline is inked, or `None` for a shape that has
    /// no outline to style: an arrowhead and a dot are solid ink, and a
    /// text run or a label is glyphs.
    pub fn style(&self) -> Option<SymbolStyle> {
        match self {
            SymbolShape::Polyline { style, .. }
            | SymbolShape::Rectangle { style, .. }
            | SymbolShape::Circle { style, .. }
            | SymbolShape::Arc { style, .. }
            | SymbolShape::Curve { style, .. } => Some(*style),
            SymbolShape::Arrow { .. }
            | SymbolShape::Dot { .. }
            | SymbolShape::Text { .. }
            | SymbolShape::Label { .. } => None,
        }
    }

    pub fn style_mut(&mut self) -> Option<&mut SymbolStyle> {
        match self {
            SymbolShape::Polyline { style, .. }
            | SymbolShape::Rectangle { style, .. }
            | SymbolShape::Circle { style, .. }
            | SymbolShape::Arc { style, .. }
            | SymbolShape::Curve { style, .. } => Some(style),
            SymbolShape::Arrow { .. }
            | SymbolShape::Dot { .. }
            | SymbolShape::Text { .. }
            | SymbolShape::Label { .. } => None,
        }
    }

    /// Whether the shape encloses an inside a fill can paint: a rectangle,
    /// a circle, or a polyline closed on itself. A fill on any other shape
    /// is never drawn.
    pub fn is_closed(&self) -> bool {
        match self {
            SymbolShape::Rectangle { .. } | SymbolShape::Circle { .. } => true,
            SymbolShape::Polyline { points, closed, .. } => *closed && points.len() > 2,
            _ => false,
        }
    }

    /// Whether the shape's inside is painted.
    pub fn is_filled(&self) -> bool {
        self.is_closed()
            && self
                .style()
                .is_some_and(|style| style.fill == SymbolFill::Solid)
    }

    pub fn translate(&mut self, delta: Point) {
        self.map_points(delta, Point::new(1, 0), Point::new(0, 1));
    }

    pub fn rotate_cw(&mut self) {
        self.map_points(Point::origin(), Point::new(0, 1), Point::new(-1, 0));
        match self {
            SymbolShape::Arc { start_degrees, .. } => {
                *start_degrees = (*start_degrees as i64 + 90).rem_euclid(360) as i32;
            }
            SymbolShape::Arrow {
                rotation_quarters, ..
            } => {
                *rotation_quarters = (*rotation_quarters as i64 + 1).rem_euclid(4) as i32;
            }
            // A quarter turn stands a run on end or lays it down. A run laid
            // along +y would read downward, which no glyph does, so standing
            // one up also moves it to the other side of its anchor.
            SymbolShape::Text {
                align, rotation, ..
            }
            | SymbolShape::Label {
                align, rotation, ..
            } => {
                if rotation.is_r0() {
                    *align = align.mirrored();
                }
                *rotation = rotation.turned();
            }
            _ => {}
        }
    }

    pub fn mirror_h(&mut self) {
        self.map_points(Point::origin(), Point::new(-1, 0), Point::new(0, 1));
        match self {
            SymbolShape::Arc {
                start_degrees,
                sweep_degrees,
                ..
            } => {
                *start_degrees = (180_i64 - *start_degrees as i64 - *sweep_degrees as i64)
                    .rem_euclid(360) as i32;
            }
            SymbolShape::Arrow {
                rotation_quarters, ..
            } => {
                *rotation_quarters = (2 - *rotation_quarters).rem_euclid(4);
            }
            // The anchor has already moved; a run read left to right has to
            // change which side of it it hangs off, or a mirrored label lands
            // across the body it was set beside. One read upward runs along
            // the mirror and stays where it is.
            SymbolShape::Text {
                align,
                rotation: SymbolTextRotation::R0,
                ..
            }
            | SymbolShape::Label {
                align,
                rotation: SymbolTextRotation::R0,
                ..
            } => *align = align.mirrored(),
            _ => {}
        }
    }

    pub fn mirror_v(&mut self) {
        self.map_points(Point::origin(), Point::new(1, 0), Point::new(0, -1));
        match self {
            SymbolShape::Arc {
                start_degrees,
                sweep_degrees,
                ..
            } => {
                *start_degrees =
                    (-(*start_degrees as i64) - *sweep_degrees as i64).rem_euclid(360) as i32;
            }
            SymbolShape::Arrow {
                rotation_quarters, ..
            } => {
                *rotation_quarters = (-*rotation_quarters).rem_euclid(4);
            }
            // The mirror runs across a run read upward, which keeps reading
            // upward from the moved anchor on the other side of it.
            SymbolShape::Text {
                align,
                rotation: SymbolTextRotation::R90,
                ..
            }
            | SymbolShape::Label {
                align,
                rotation: SymbolTextRotation::R90,
                ..
            } => *align = align.mirrored(),
            _ => {}
        }
    }

    /// Carries every point of the shape to `origin + x * across + y * down`.
    fn map_points(&mut self, origin: Point, across: Point, down: Point) {
        let transform = |point: Point| {
            origin
                + Point::new(
                    point.x * across.x + point.y * down.x,
                    point.x * across.y + point.y * down.y,
                )
        };
        match self {
            SymbolShape::Polyline { points, .. } => {
                for point in points {
                    *point = transform(*point);
                }
            }
            // A turn or a mirror swaps which corner is least; name them again.
            SymbolShape::Rectangle { min, max, .. } => {
                let (a, b) = (transform(*min), transform(*max));
                *min = Point::new(a.x.min(b.x), a.y.min(b.y));
                *max = Point::new(a.x.max(b.x), a.y.max(b.y));
            }
            SymbolShape::Circle { center, .. }
            | SymbolShape::Arc { center, .. }
            | SymbolShape::Dot { center, .. } => {
                *center = transform(*center);
            }
            SymbolShape::Curve { curve, .. } => *curve = curve.placed(origin, across, down),
            SymbolShape::Arrow { tip, .. } => {
                *tip = transform(*tip);
            }
            SymbolShape::Text { anchor, .. } | SymbolShape::Label { anchor, .. } => {
                *anchor = transform(*anchor);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: [f64; 2], b: [f64; 2]) -> bool {
        (a[0] - b[0]).abs() < 1e-6 && (a[1] - b[1]).abs() < 1e-6
    }

    #[test]
    fn a_curve_shows_no_straight_piece_at_any_size() {
        let curve = SymbolCurve {
            center: [2.5, -7.25],
            radii: [10.0, 10.0],
            start_degrees: 200.0,
            sweep_degrees: 225.0,
        };
        for pixels_per_unit in [0.5, 1.0, 2.0, 8.0, 40.0] {
            let points = curve.tessellated(pixels_per_unit);
            assert!(close(points[0], curve.start_point()));
            assert!(close(*points.last().unwrap(), curve.end_point()));
            for piece in points.windows(2) {
                let chord = (piece[1][0] - piece[0][0]).hypot(piece[1][1] - piece[0][1]);
                let pixels = chord * pixels_per_unit;
                assert!(pixels <= 3.0, "{pixels} px at {pixels_per_unit}");
                let middle = [
                    (piece[0][0] + piece[1][0]) / 2.0 - curve.center[0],
                    (piece[0][1] + piece[1][1]) / 2.0 - curve.center[1],
                ];
                let error = (10.0 - middle[0].hypot(middle[1])) * pixels_per_unit;
                assert!(error <= 0.06, "{error} px at {pixels_per_unit}");
            }
        }
    }

    #[test]
    fn a_placed_curve_is_the_curve_carried_point_by_point() {
        let curve = SymbolCurve {
            center: [3.75, -2.5],
            radii: [10.0, 4.5],
            start_degrees: 31.0,
            sweep_degrees: 137.0,
        };
        let frames = [
            (Point::new(1, 0), Point::new(0, 1)),
            (Point::new(0, 1), Point::new(-1, 0)),
            (Point::new(-1, 0), Point::new(0, -1)),
            (Point::new(0, -1), Point::new(1, 0)),
            (Point::new(-1, 0), Point::new(0, 1)),
            (Point::new(1, 0), Point::new(0, -1)),
            (Point::new(0, 1), Point::new(1, 0)),
            (Point::new(0, -1), Point::new(-1, 0)),
        ];
        let origin = Point::new(40, -30);
        for (across, down) in frames {
            let placed = curve.placed(origin, across, down);
            let carry = |[x, y]: [f64; 2]| {
                [
                    f64::from(origin.x) + x * f64::from(across.x) + y * f64::from(down.x),
                    f64::from(origin.y) + x * f64::from(across.y) + y * f64::from(down.y),
                ]
            };
            let (from, to) = (carry(curve.start_point()), carry(curve.end_point()));
            let mirrored = across.x * down.y - across.y * down.x < 0;
            let (first, last) = if mirrored { (to, from) } else { (from, to) };
            assert!(close(placed.start_point(), first), "{across:?} {down:?}");
            assert!(close(placed.end_point(), last), "{across:?} {down:?}");
            let middle = carry(curve.point_at(curve.start_degrees + curve.sweep_degrees / 2.0));
            assert!(close(
                placed.point_at(placed.start_degrees + placed.sweep_degrees / 2.0),
                middle
            ));
        }
    }

    #[test]
    fn a_curves_bounds_are_its_own_and_not_its_ellipses() {
        let quarter = SymbolCurve {
            center: [0.0, 0.0],
            radii: [10.0, 10.0],
            start_degrees: 10.0,
            sweep_degrees: 20.0,
        };
        let (least, greatest) = quarter.bounds();
        assert!(close(
            least,
            [quarter.end_point()[0], quarter.start_point()[1]]
        ));
        assert!(close(
            greatest,
            [quarter.start_point()[0], quarter.end_point()[1]]
        ));
        let (least, greatest) = SymbolCurve {
            sweep_degrees: 360.0,
            ..quarter
        }
        .bounds();
        assert!(close(least, [-10.0, -10.0]));
        assert!(close(greatest, [10.0, 10.0]));
    }

    #[test]
    fn repeated_shape_transforms_keep_angles_in_canonical_ranges() {
        let mut arc = SymbolShape::Arc {
            center: Point::origin(),
            radius: 10,
            start_degrees: 0,
            sweep_degrees: 180,
            style: Default::default(),
        };
        let mut arrow = SymbolShape::Arrow {
            tip: Point::origin(),
            rotation_quarters: 0,
        };

        for _ in 0..10_000 {
            arc.rotate_cw();
            arrow.rotate_cw();
        }

        let SymbolShape::Arc { start_degrees, .. } = arc else {
            unreachable!();
        };
        let SymbolShape::Arrow {
            rotation_quarters, ..
        } = arrow
        else {
            unreachable!();
        };
        assert!((0..360).contains(&start_degrees));
        assert!((0..4).contains(&rotation_quarters));
    }

    /// A turned or mirrored rectangle is the same box carried over, still
    /// named by its least and greatest corners.
    #[test]
    fn transformed_rectangles_stay_rectangles_with_ordered_corners() {
        let original = SymbolShape::rectangle(Point::new(30, 20), Point::new(-10, -5));
        assert_eq!(
            original,
            SymbolShape::Rectangle {
                min: Point::new(-10, -5),
                max: Point::new(30, 20),
                style: Default::default(),
            }
        );

        let mut turned = original.clone();
        turned.rotate_cw();
        assert_eq!(
            turned,
            SymbolShape::Rectangle {
                min: Point::new(-20, -10),
                max: Point::new(5, 30),
                style: Default::default(),
            }
        );
        let mut mirrored = original.clone();
        mirrored.mirror_h();
        assert_eq!(
            mirrored,
            SymbolShape::Rectangle {
                min: Point::new(-30, -5),
                max: Point::new(10, 20),
                style: Default::default(),
            }
        );
        let mut flipped = original.clone();
        flipped.mirror_v();
        assert_eq!(
            flipped,
            SymbolShape::Rectangle {
                min: Point::new(-10, -20),
                max: Point::new(30, 5),
                style: Default::default(),
            }
        );
        for _ in 0..3 {
            turned.rotate_cw();
        }
        assert_eq!(turned, original);
        assert_eq!(
            rectangle_corners(Point::new(-10, -5), Point::new(30, 20)),
            [
                Point::new(-10, -5),
                Point::new(30, -5),
                Point::new(30, 20),
                Point::new(-10, 20),
            ]
        );
    }

    /// T13: four quarter turns and two horizontal mirrors are the identity on
    /// a text run, alignment included — otherwise a symbol drifts every time
    /// it is turned back to where it started.
    #[test]
    fn text_shapes_round_trip_four_rotations_and_two_mirrors() {
        let original = SymbolShape::Text {
            anchor: Point::new(30, -10),
            text: "AMP".to_owned(),
            size: SymbolTextSize::Large,
            align: SymbolTextAlign::Right,
            rotation: Default::default(),
        };

        let mut rotated = original.clone();
        for _ in 0..4 {
            rotated.rotate_cw();
        }
        let mut mirrored = original.clone();
        mirrored.mirror_h();
        let once = mirrored.clone();
        mirrored.mirror_h();
        let mut flipped = original.clone();
        flipped.mirror_v();
        flipped.mirror_v();

        assert_eq!(rotated, original);
        assert_eq!(mirrored, original);
        assert_eq!(flipped, original);
        assert_eq!(
            once,
            SymbolShape::Text {
                anchor: Point::new(-30, -10),
                text: "AMP".to_owned(),
                size: SymbolTextSize::Large,
                align: SymbolTextAlign::Left,
                rotation: Default::default(),
            },
            "one mirror moves the anchor and the side the run hangs off"
        );
    }

    /// A quarter turn carries a run round with the drawing: one read left to
    /// right stands up to read upward, and one read upward lies down again.
    /// The size never follows the turn, and four turns give the run back.
    #[test]
    fn rotating_a_text_shape_turns_its_reading() {
        let text = |anchor, align, rotation| SymbolShape::Text {
            anchor,
            text: "OTA".to_owned(),
            size: SymbolTextSize::Small,
            align,
            rotation,
        };
        let mut shape = text(
            Point::new(20, 5),
            SymbolTextAlign::Left,
            SymbolTextRotation::R0,
        );

        shape.rotate_cw();
        // It read rightward from its anchor, so after the turn it lies below
        // the anchor and reads up to it.
        assert_eq!(
            shape,
            text(
                Point::new(-5, 20),
                SymbolTextAlign::Right,
                SymbolTextRotation::R90
            )
        );
        shape.rotate_cw();
        assert_eq!(
            shape,
            text(
                Point::new(-20, -5),
                SymbolTextAlign::Right,
                SymbolTextRotation::R0
            )
        );
        shape.rotate_cw();
        shape.rotate_cw();
        assert_eq!(
            shape,
            text(
                Point::new(20, 5),
                SymbolTextAlign::Left,
                SymbolTextRotation::R0
            )
        );
    }

    /// Turned or mirrored, a run covers exactly the box the same transform
    /// carries its old box to, for every alignment and either reading: it
    /// moves round its anchor with the rest of the drawing and is never read
    /// downward or backward.
    #[test]
    fn a_turned_or_mirrored_run_covers_where_its_box_went() {
        fn bounds(shape: &SymbolShape) -> (Point, Point) {
            let SymbolShape::Text {
                anchor,
                text,
                size,
                align,
                rotation,
            } = shape
            else {
                unreachable!("only text is measured here");
            };
            symbol_text_bounds(*anchor, text, *size, *align, *rotation)
        }
        fn carried((min, max): (Point, Point), map: fn(Point) -> Point) -> (Point, Point) {
            let (a, b) = (map(min), map(max));
            (
                Point::new(a.x.min(b.x), a.y.min(b.y)),
                Point::new(a.x.max(b.x), a.y.max(b.y)),
            )
        }
        type Transform = (&'static str, fn(&mut SymbolShape), fn(Point) -> Point);
        let transforms: [Transform; 3] = [
            ("turned", SymbolShape::rotate_cw, |p| Point::new(-p.y, p.x)),
            ("mirrored", SymbolShape::mirror_h, |p| Point::new(-p.x, p.y)),
            ("flipped", SymbolShape::mirror_v, |p| Point::new(p.x, -p.y)),
        ];
        for align in SymbolTextAlign::ALL {
            for rotation in SymbolTextRotation::ALL {
                let shape = SymbolShape::Text {
                    anchor: Point::new(20, 5),
                    text: "AMP".to_owned(),
                    size: SymbolTextSize::Normal,
                    align,
                    rotation,
                };
                for (name, transform, map) in transforms {
                    let mut moved = shape.clone();
                    transform(&mut moved);
                    assert_eq!(
                        bounds(&moved),
                        carried(bounds(&shape), map),
                        "{align:?} {rotation:?} {name}"
                    );
                }
            }
        }
    }

    /// A run read upward is measured up from its anchor when it starts
    /// there, down from it when it ends there, and centred across it.
    #[test]
    fn a_run_read_upward_is_measured_along_its_reading() {
        let up = |align| {
            symbol_text_bounds(
                Point::new(0, 0),
                "AMP",
                SymbolTextSize::Normal,
                align,
                SymbolTextRotation::R90,
            )
        };

        assert_eq!(
            up(SymbolTextAlign::Left),
            (Point::new(-4, -15), Point::new(4, 0))
        );
        assert_eq!(
            up(SymbolTextAlign::Right),
            (Point::new(-4, 0), Point::new(4, 15))
        );
        assert_eq!(
            up(SymbolTextAlign::Center),
            (Point::new(-4, -7), Point::new(4, 7))
        );
    }

    /// Where a run read upward lands on a placed instance: above its anchor
    /// when it starts there, below when it ends there, and carried round the
    /// anchor by the instance's turn.
    #[test]
    fn a_run_read_upward_lands_where_the_instance_carries_it() {
        let upward = SymbolTextRotation::R90;
        let still = |point: Point| point;
        let turned = |point: Point| Point::new(-point.y, point.x);

        assert_eq!(
            SymbolTextAlign::Left.placement(upward, still),
            SymbolTextPlacement::Above
        );
        assert_eq!(
            SymbolTextAlign::Right.placement(upward, still),
            SymbolTextPlacement::Below
        );
        assert_eq!(
            SymbolTextAlign::Center.placement(upward, still),
            SymbolTextPlacement::On
        );
        assert_eq!(
            SymbolTextAlign::Left.placement(upward, turned),
            SymbolTextPlacement::After
        );
        assert_eq!(
            SymbolTextAlign::Left.placement(SymbolTextRotation::R0, still),
            SymbolTextPlacement::After
        );
    }

    /// A label turns and mirrors exactly as a text run with the same anchor,
    /// alignment and reading does, keeping its parameter, default and
    /// visibility.
    #[test]
    fn a_label_moves_as_a_text_run_does() {
        let label = |anchor, align, rotation| SymbolShape::Label {
            anchor,
            parameter: "W".to_owned(),
            default: "1u".to_owned(),
            size: SymbolTextSize::Small,
            align,
            rotation,
            shown: false,
        };
        let run = |anchor, align, rotation| SymbolShape::Text {
            anchor,
            text: "W=1u".to_owned(),
            size: SymbolTextSize::Small,
            align,
            rotation,
        };
        let parts = |shape: &SymbolShape| match shape {
            SymbolShape::Label {
                anchor,
                align,
                rotation,
                ..
            }
            | SymbolShape::Text {
                anchor,
                align,
                rotation,
                ..
            } => (*anchor, *align, *rotation),
            other => unreachable!("{other:?}"),
        };
        let transforms: [fn(&mut SymbolShape); 4] = [
            SymbolShape::rotate_cw,
            SymbolShape::mirror_h,
            SymbolShape::mirror_v,
            |shape| shape.translate(Point::new(3, -4)),
        ];
        for align in SymbolTextAlign::ALL {
            for rotation in SymbolTextRotation::ALL {
                for transform in transforms {
                    let mut moved_label = label(Point::new(20, 5), align, rotation);
                    let mut moved_run = run(Point::new(20, 5), align, rotation);
                    transform(&mut moved_label);
                    transform(&mut moved_run);
                    assert_eq!(parts(&moved_label), parts(&moved_run));
                    let (anchor, align, rotation) = parts(&moved_label);
                    assert_eq!(moved_label, label(anchor, align, rotation));
                }
            }
        }
    }

    /// A label shown on instances, with no default, reading across, writes
    /// only what names it; the rest is written when it differs.
    #[test]
    fn a_label_writes_only_what_differs_from_the_plain_one() {
        let label = |default: &str, shown| SymbolShape::Label {
            anchor: Point::new(0, 0),
            parameter: "W".to_owned(),
            default: default.to_owned(),
            size: SymbolTextSize::Normal,
            align: SymbolTextAlign::Left,
            rotation: SymbolTextRotation::R0,
            shown,
        };
        let plain = label("", true);
        let written = serde_json::to_string(&plain).unwrap();
        assert!(!written.contains("default"), "{written}");
        assert!(!written.contains("shown"), "{written}");
        assert!(!written.contains("rotation"), "{written}");
        assert_eq!(
            serde_json::from_str::<SymbolShape>(&written).unwrap(),
            plain
        );

        let hidden = label("1u", false);
        let written = serde_json::to_string(&hidden).unwrap();
        assert!(written.contains(r#""default":"1u""#), "{written}");
        assert!(written.contains(r#""shown":false"#), "{written}");
        assert_eq!(
            serde_json::from_str::<SymbolShape>(&written).unwrap(),
            hidden
        );
        assert_eq!(hidden.style(), None);
        assert!(!hidden.is_closed());
    }

    #[test]
    fn a_label_letters_its_name_and_a_value() {
        assert_eq!(symbol_label_text("W", "2u"), "W=2u");
        assert_eq!(symbol_label_sample("W", "1u"), "W=1u");
        assert_eq!(symbol_label_sample("W", ""), "W=?");
    }

    /// A run read left to right is written as it always was; only a turned
    /// one names its reading.
    #[test]
    fn only_a_turned_run_writes_its_reading() {
        let text = |rotation| SymbolShape::Text {
            anchor: Point::new(0, 0),
            text: "A".to_owned(),
            size: SymbolTextSize::Normal,
            align: SymbolTextAlign::Left,
            rotation,
        };
        let plain = serde_json::to_string(&text(SymbolTextRotation::R0)).unwrap();
        let turned = serde_json::to_string(&text(SymbolTextRotation::R90)).unwrap();

        assert!(!plain.contains("rotation"), "{plain}");
        assert!(turned.contains(r#""rotation":"r90""#), "{turned}");
        assert_eq!(
            serde_json::from_str::<SymbolShape>(&turned).unwrap(),
            text(SymbolTextRotation::R90)
        );
        assert_eq!(
            serde_json::from_str::<SymbolShape>(&plain).unwrap(),
            text(SymbolTextRotation::R0)
        );
    }

    #[test]
    fn text_runs_are_measured_from_the_side_they_hang_off() {
        let anchor = Point::new(0, 0);

        let across = |align| {
            symbol_text_bounds(
                anchor,
                "AMP",
                SymbolTextSize::Normal,
                align,
                SymbolTextRotation::R0,
            )
        };
        let left = across(SymbolTextAlign::Left);
        let right = across(SymbolTextAlign::Right);
        let centered = across(SymbolTextAlign::Center);

        // Plex Mono at a 9-unit cap height advances 5 units per glyph.
        assert_eq!(left, (Point::new(0, -4), Point::new(15, 4)));
        assert_eq!(right, (Point::new(-15, -4), Point::new(0, 4)));
        assert_eq!(centered, (Point::new(-7, -4), Point::new(7, 4)));
    }

    fn styled() -> SymbolStyle {
        SymbolStyle {
            stroke: SymbolStroke::Heavy,
            dash: SymbolDash::Dotted,
            fill: SymbolFill::Solid,
        }
    }

    fn every_styled_kind() -> Vec<SymbolShape> {
        vec![
            SymbolShape::Polyline {
                points: vec![Point::new(0, 0), Point::new(20, 0), Point::new(0, 10)],
                closed: true,
                style: SymbolStyle::default(),
            },
            SymbolShape::rectangle(Point::new(0, 0), Point::new(20, 10)),
            SymbolShape::Circle {
                center: Point::new(5, 5),
                radius: 4,
                style: SymbolStyle::default(),
            },
            SymbolShape::Arc {
                center: Point::new(5, 5),
                radius: 4,
                start_degrees: 0,
                sweep_degrees: 90,
                style: SymbolStyle::default(),
            },
            SymbolShape::curve(SymbolCurve::of_arc(Point::new(5, 5), 4, 0, 90)),
        ]
    }

    /// A shape in the plain style writes no style at all, so a document
    /// reads back exactly as it was written before shapes had one; a chosen
    /// style is written and read back whole.
    #[test]
    fn only_a_chosen_style_is_written() {
        for shape in every_styled_kind() {
            let plain = serde_json::to_string(&shape).unwrap();
            assert!(!plain.contains("style"), "{plain}");
            assert_eq!(serde_json::from_str::<SymbolShape>(&plain).unwrap(), shape);

            let mut chosen = shape.clone();
            *chosen.style_mut().unwrap() = styled();
            let written = serde_json::to_string(&chosen).unwrap();
            assert!(
                written.contains(r#""style":{"stroke":"heavy","dash":"dotted","fill":"solid"}"#),
                "{written}"
            );
            assert_eq!(
                serde_json::from_str::<SymbolShape>(&written).unwrap(),
                chosen
            );
        }
        // Only the part of a style that differs need be written.
        let thin: SymbolShape = serde_json::from_str(
            r#"{"Circle":{"center":{"x":0,"y":0},"radius":3,"style":{"stroke":"thin"}}}"#,
        )
        .unwrap();
        assert_eq!(
            thin.style(),
            Some(SymbolStyle {
                stroke: SymbolStroke::Thin,
                ..SymbolStyle::default()
            })
        );
    }

    /// A turn, a mirror or a move carries the geometry and leaves the ink
    /// alone.
    #[test]
    fn a_moved_turned_or_mirrored_shape_keeps_its_style() {
        for mut shape in every_styled_kind() {
            *shape.style_mut().unwrap() = styled();
            shape.rotate_cw();
            shape.mirror_h();
            shape.mirror_v();
            shape.translate(Point::new(7, -3));
            assert_eq!(shape.style(), Some(styled()), "{shape:?}");
        }
    }

    /// Only a shape with an inside takes a fill: a rectangle, a circle and a
    /// polyline closed round at least three corners. A line, an arc and a
    /// curve have no inside to paint, and an arrowhead, a dot and a text run
    /// have no outline to style at all.
    #[test]
    fn only_a_shape_with_an_inside_is_filled() {
        let fill = |mut shape: SymbolShape| {
            if let Some(style) = shape.style_mut() {
                style.fill = SymbolFill::Solid;
            }
            shape
        };
        let kinds = every_styled_kind();
        for shape in &kinds[..3] {
            assert!(shape.is_closed() && !shape.is_filled(), "{shape:?}");
            assert!(fill(shape.clone()).is_filled(), "{shape:?}");
        }
        for shape in &kinds[3..] {
            assert!(!fill(shape.clone()).is_filled(), "{shape:?}");
        }
        let open = SymbolShape::Polyline {
            points: vec![Point::new(0, 0), Point::new(20, 0), Point::new(0, 10)],
            closed: false,
            style: SymbolStyle::default(),
        };
        let two_corners = SymbolShape::Polyline {
            points: vec![Point::new(0, 0), Point::new(20, 0)],
            closed: true,
            style: SymbolStyle::default(),
        };
        assert!(!fill(open).is_filled());
        assert!(!fill(two_corners).is_filled());
        for solid in [
            SymbolShape::Arrow {
                tip: Point::origin(),
                rotation_quarters: 0,
            },
            SymbolShape::Dot {
                center: Point::origin(),
                radius: 2,
            },
        ] {
            assert_eq!(solid.style(), None);
            assert!(!solid.is_filled());
        }
    }
}

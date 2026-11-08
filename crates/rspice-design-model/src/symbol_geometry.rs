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

    /// The alignment a horizontal mirror produces. Glyphs stay upright under
    /// every transform, so a mirror can only move the run to the other side
    /// of its anchor.
    pub const fn mirrored(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Center => Self::Center,
            Self::Right => Self::Left,
        }
    }

    /// Where the run sits once `orient` — the placed instance's own
    /// transform — has carried its advance direction into world space.
    ///
    /// The single owner of the rule, because every surface that draws
    /// authored text spells the answer differently and none of them may
    /// disagree about it.
    pub fn placement(self, orient: impl Fn(Point) -> Point) -> SymbolTextPlacement {
        let Some(step) = self.run_step() else {
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
    /// it straddles the anchor and no orientation can move it off.
    const fn run_step(self) -> Option<Point> {
        match self {
            Self::Left => Some(Point::new(1, 0)),
            Self::Center => None,
            Self::Right => Some(Point::new(-1, 0)),
        }
    }
}

/// Which side of its anchor an oriented text run ends up on.
///
/// Glyphs stay upright under every transform, so an orientation can only move
/// the run around its anchor; this names where it landed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolTextPlacement {
    /// Straddling the anchor, wherever the instance is turned.
    On,
    After,
    Before,
    Below,
    Above,
}

/// The box a text run occupies in symbol units, vertically centred on its
/// anchor.
///
/// The single owner of symbol type metrics. IBM Plex Mono advances 600/1000
/// em, so a glyph of cap height `h` is `3h/5` wide; every renderer sets that
/// same face and size, so what this measures is what they draw.
pub fn symbol_text_bounds(
    anchor: Point,
    text: &str,
    size: SymbolTextSize,
    align: SymbolTextAlign,
) -> (Point, Point) {
    let glyphs = i32::try_from(text.chars().count()).unwrap_or(i32::MAX);
    let span = glyphs.saturating_mul(size.height() * 3 / 5);
    let (before, after) = match align {
        SymbolTextAlign::Left => (0, span),
        SymbolTextAlign::Center => (span / 2, span / 2),
        SymbolTextAlign::Right => (span, 0),
    };
    let half_height = size.height() / 2;
    (
        Point::new(
            anchor.x.saturating_sub(before),
            anchor.y.saturating_sub(half_height),
        ),
        Point::new(
            anchor.x.saturating_add(after),
            anchor.y.saturating_add(half_height),
        ),
    )
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
    },
    Circle {
        center: Point,
        radius: i32,
    },
    Arc {
        center: Point,
        radius: i32,
        start_degrees: i32,
        sweep_degrees: i32,
    },
    /// An ellipse or arc a drawing brought in, unrounded.
    Curve(SymbolCurve),
    Arrow {
        tip: Point,
        rotation_quarters: i32,
    },
    Dot {
        center: Point,
        radius: i32,
    },
    /// A run of authored glyphs. The anchor is geometry and moves with every
    /// transform; the glyphs themselves stay upright and read left to right
    /// however the shape or the instance is turned, which is what makes a
    /// mirrored or rotated symbol still legible.
    Text {
        anchor: Point,
        text: String,
        size: SymbolTextSize,
        align: SymbolTextAlign,
    },
}

impl SymbolShape {
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
            // The anchor has already moved; the run has to change which side
            // of it it hangs off, or a mirrored label lands across the body
            // it was set beside.
            SymbolShape::Text { align, .. } => *align = align.mirrored(),
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
            SymbolShape::Circle { center, .. }
            | SymbolShape::Arc { center, .. }
            | SymbolShape::Dot { center, .. } => {
                *center = transform(*center);
            }
            SymbolShape::Curve(curve) => *curve = curve.placed(origin, across, down),
            SymbolShape::Arrow { tip, .. } => {
                *tip = transform(*tip);
            }
            SymbolShape::Text { anchor, .. } => {
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
            },
            "one mirror moves the anchor and the side the run hangs off"
        );
    }

    /// A quarter turn moves the anchor and nothing else: the glyphs stay
    /// upright, so neither the alignment nor the size may follow it round.
    #[test]
    fn rotating_a_text_shape_moves_only_its_anchor() {
        let mut shape = SymbolShape::Text {
            anchor: Point::new(20, 5),
            text: "OTA".to_owned(),
            size: SymbolTextSize::Small,
            align: SymbolTextAlign::Left,
        };

        shape.rotate_cw();

        assert_eq!(
            shape,
            SymbolShape::Text {
                anchor: Point::new(-5, 20),
                text: "OTA".to_owned(),
                size: SymbolTextSize::Small,
                align: SymbolTextAlign::Left,
            }
        );
    }

    #[test]
    fn text_runs_are_measured_from_the_side_they_hang_off() {
        let anchor = Point::new(0, 0);

        let left = symbol_text_bounds(anchor, "AMP", SymbolTextSize::Normal, SymbolTextAlign::Left);
        let right = symbol_text_bounds(
            anchor,
            "AMP",
            SymbolTextSize::Normal,
            SymbolTextAlign::Right,
        );
        let centered = symbol_text_bounds(
            anchor,
            "AMP",
            SymbolTextSize::Normal,
            SymbolTextAlign::Center,
        );

        // Plex Mono at a 9-unit cap height advances 5 units per glyph.
        assert_eq!(left, (Point::new(0, -4), Point::new(15, 4)));
        assert_eq!(right, (Point::new(-15, -4), Point::new(0, 4)));
        assert_eq!(centered, (Point::new(-7, -4), Point::new(7, 4)));
    }
}

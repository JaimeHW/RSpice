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
        self.map_points(|point| point + delta);
    }

    pub fn rotate_cw(&mut self) {
        self.map_points(|point| Point::new(-point.y, point.x));
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
        self.map_points(|point| Point::new(-point.x, point.y));
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
        self.map_points(|point| Point::new(point.x, -point.y));
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

    fn map_points(&mut self, transform: impl Fn(Point) -> Point) {
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

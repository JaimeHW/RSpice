//! Where a placed instance puts its symbol on the sheet that places it.
//!
//! Editing a symbol in place draws it through its instance's transform and
//! reads the pointer back through the inverse, so every point the editor
//! hit-tests, snaps and stores stays in the symbol's own coordinates.

use rspice_design::schematic::{component::Component, rotation::Rotation};
use rspice_design_model::Point;

/// The map from symbol coordinates to sheet coordinates that one instance
/// applies: its position, its orientation, and the symbol origin the
/// position is measured from.
///
/// The default is the identity, which is how the symbol editor draws a
/// symbol in its own view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SymbolPlacement {
    /// Where the symbol origin lands on the sheet.
    pub position: Point,
    /// The symbol point that lands on `position`.
    pub origin: Point,
    pub rotation: Rotation,
    pub mirror_h: bool,
    pub mirror_v: bool,
}

impl SymbolPlacement {
    /// The map `instance` applies to a symbol whose origin is `origin`.
    pub fn of_instance(instance: &Component, origin: Point) -> Self {
        Self {
            position: instance.pos,
            origin,
            rotation: instance.rotation,
            mirror_h: instance.mirror_h,
            mirror_v: instance.mirror_v,
        }
    }

    pub fn is_identity(self) -> bool {
        self == Self::default()
    }

    /// Carry a symbol-space direction into sheet space: mirror first, then
    /// rotate, as [`Component::transform_point`] does.
    pub fn orient(self, vector: Point) -> Point {
        let (x, y) = self.orient_f(vector.x as f32, vector.y as f32);
        Point::new(x.round() as i32, y.round() as i32)
    }

    /// Where symbol point `point` lands on the sheet.
    pub fn place(self, point: Point) -> Point {
        self.position + self.orient(point - self.origin)
    }

    /// [`Self::place`] without rounding, for drawing.
    pub fn place_f(self, point: Point) -> (f32, f32) {
        self.place_xy(point.x as f32, point.y as f32)
    }

    /// Where the symbol-space point `(x, y)` lands, for a point between
    /// grid units.
    pub fn place_xy(self, x: f32, y: f32) -> (f32, f32) {
        let (x, y) = self.orient_f(x - self.origin.x as f32, y - self.origin.y as f32);
        (self.position.x as f32 + x, self.position.y as f32 + y)
    }

    /// The symbol point a sheet point stands on, before rounding.
    pub fn unplace_f(self, x: f32, y: f32) -> (f32, f32) {
        let (x, y) = self.unorient_f(x - self.position.x as f32, y - self.position.y as f32);
        (self.origin.x as f32 + x, self.origin.y as f32 + y)
    }

    pub fn orient_f(self, x: f32, y: f32) -> (f32, f32) {
        let x = if self.mirror_h { -x } else { x };
        let y = if self.mirror_v { -y } else { y };
        match self.rotation {
            Rotation::R0 => (x, y),
            Rotation::R90 => (-y, x),
            Rotation::R180 => (-x, -y),
            Rotation::R270 => (y, -x),
        }
    }

    /// Undo the rotation, then the mirror, which is its own inverse.
    fn unorient_f(self, x: f32, y: f32) -> (f32, f32) {
        let (x, y) = match self.rotation {
            Rotation::R0 => (x, y),
            Rotation::R90 => (y, -x),
            Rotation::R180 => (-x, -y),
            Rotation::R270 => (-y, x),
        };
        (
            if self.mirror_h { -x } else { x },
            if self.mirror_v { -y } else { y },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::schematic::component_type::ComponentType;

    /// Every orientation an instance can take maps a symbol point where the
    /// instance's own transform does, and the inverse brings it back.
    #[test]
    fn placement_matches_the_instance_transform_and_inverts() {
        let origin = Point::new(5, -10);
        let point = Point::new(-30, 20);
        for rotation in [Rotation::R0, Rotation::R90, Rotation::R180, Rotation::R270] {
            for (mirror_h, mirror_v) in [(false, false), (true, false), (false, true), (true, true)]
            {
                let instance = Component::new(1, ComponentType::CellInstance, Point::new(100, 60))
                    .with_rotation(rotation)
                    .with_mirror_h(mirror_h)
                    .with_mirror_v(mirror_v);
                let placement = SymbolPlacement::of_instance(&instance, origin);
                let expected = instance.pos + instance.transform_point(point - origin);
                assert_eq!(
                    placement.place(point),
                    expected,
                    "{rotation:?} {mirror_h} {mirror_v}"
                );
                let (x, y) = placement.unplace_f(expected.x as f32, expected.y as f32);
                assert_eq!(
                    Point::new(x.round() as i32, y.round() as i32),
                    point,
                    "{rotation:?} {mirror_h} {mirror_v}"
                );
            }
        }
    }
}

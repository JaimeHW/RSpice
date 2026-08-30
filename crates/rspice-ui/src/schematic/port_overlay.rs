//! Runtime direction marks layered over the canonical interface-port body.

use crate::state::PortDirection;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct OverlayPoint {
    pub(super) x: f32,
    pub(super) y: f32,
}

impl OverlayPoint {
    const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct OverlaySegment {
    pub(super) start: OverlayPoint,
    pub(super) end: OverlayPoint,
}

const fn segment(x1: f32, y1: f32, x2: f32, y2: f32) -> OverlaySegment {
    OverlaySegment {
        start: OverlayPoint::new(x1, y1),
        end: OverlayPoint::new(x2, y2),
    }
}

const IN: &[OverlaySegment] = &[
    segment(-1.5, 0.0, 5.0, 0.0),
    segment(5.0, 0.0, 2.0, -2.5),
    segment(5.0, 0.0, 2.0, 2.5),
];
const OUT: &[OverlaySegment] = &[
    segment(5.0, 0.0, -1.5, 0.0),
    segment(-1.5, 0.0, 1.5, -2.5),
    segment(-1.5, 0.0, 1.5, 2.5),
];
const IN_OUT: &[OverlaySegment] = &[
    segment(-1.5, 0.0, 5.0, 0.0),
    segment(5.0, 0.0, 2.0, -2.5),
    segment(5.0, 0.0, 2.0, 2.5),
    segment(-1.5, 0.0, 1.5, -2.5),
    segment(-1.5, 0.0, 1.5, 2.5),
];
const SUPPLY: &[OverlaySegment] = &[segment(2.0, -3.0, 2.0, 3.0), segment(-1.0, -3.0, 5.0, -3.0)];

pub(super) const fn direction_segments(direction: PortDirection) -> &'static [OverlaySegment] {
    match direction {
        PortDirection::In => IN,
        PortDirection::Out => OUT,
        PortDirection::InOut => IN_OUT,
        PortDirection::Supply => SUPPLY,
    }
}

/// EDA transform order: mirror in symbol-local coordinates, then rotate.
pub(super) fn transform_point(
    point: OverlayPoint,
    rotation_degrees: i32,
    mirror_h: bool,
    mirror_v: bool,
) -> OverlayPoint {
    let x = if mirror_h { -point.x } else { point.x };
    let y = if mirror_v { -point.y } else { point.y };
    let (x, y) = match rotation_degrees.rem_euclid(360) {
        0 => (x, y),
        90 => (-y, x),
        180 => (-x, -y),
        270 => (y, -x),
        degrees => {
            let radians = (degrees as f32).to_radians();
            let (sine, cosine) = radians.sin_cos();
            (x * cosine - y * sine, x * sine + y * cosine)
        }
    };
    OverlayPoint::new(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_port_direction_has_explicit_overlay_geometry() {
        assert_eq!(direction_segments(PortDirection::In).len(), 3);
        assert_eq!(direction_segments(PortDirection::Out).len(), 3);
        assert_eq!(direction_segments(PortDirection::InOut).len(), 5);
        assert_eq!(direction_segments(PortDirection::Supply).len(), 2);
    }

    #[test]
    fn overlay_transform_applies_mirror_before_rotation() {
        let point = OverlayPoint::new(5.0, 2.0);
        assert_eq!(transform_point(point, 0, false, false), point);
        assert_eq!(
            transform_point(point, 90, true, false),
            OverlayPoint::new(-2.0, -5.0)
        );
        assert_eq!(
            transform_point(point, 270, false, true),
            OverlayPoint::new(-2.0, -5.0)
        );
    }

    #[test]
    fn every_direction_preserves_geometry_through_all_cardinal_transforms() {
        for direction in [
            PortDirection::In,
            PortDirection::Out,
            PortDirection::InOut,
            PortDirection::Supply,
        ] {
            for rotation in [0, 90, 180, 270] {
                for (mirror_h, mirror_v) in
                    [(false, false), (true, false), (false, true), (true, true)]
                {
                    for segment in direction_segments(direction) {
                        let transformed_start =
                            transform_point(segment.start, rotation, mirror_h, mirror_v);
                        let transformed_end =
                            transform_point(segment.end, rotation, mirror_h, mirror_v);
                        let length = |start: OverlayPoint, end: OverlayPoint| {
                            (end.x - start.x).hypot(end.y - start.y)
                        };
                        assert!(
                            (length(segment.start, segment.end)
                                - length(transformed_start, transformed_end))
                            .abs()
                                < f32::EPSILON
                        );
                    }
                }
            }
        }
    }
}

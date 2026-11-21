//! Runtime direction marks layered over the canonical interface-port body,
//! in world units: the canvas and the printed page draw the same arrow.

use rspice_design_model::port::PortDirection;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OverlayPoint {
    pub x: f32,
    pub y: f32,
}

impl OverlayPoint {
    const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OverlaySegment {
    pub start: OverlayPoint,
    pub end: OverlayPoint,
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

pub const fn direction_segments(direction: PortDirection) -> &'static [OverlaySegment] {
    match direction {
        PortDirection::In => IN,
        PortDirection::Out => OUT,
        PortDirection::InOut => IN_OUT,
        PortDirection::Supply => SUPPLY,
    }
}

/// EDA transform order: mirror in symbol-local coordinates, then rotate.
pub fn transform_point(
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

//! Shared presentation geometry for buses and typed bus taps.
//!
//! The canvas, hit testing, selection, the SVG export and the printed page
//! all consume this one tap route, so the visible angled breakout can never
//! disagree with interaction geometry or with paper. Electrical connectivity
//! remains defined only by the durable `bus_point` and `connection_point`
//! endpoints.
//!
//! A bus draws as three parallel strands; the drawings that set them as
//! paths (the SVG and the page) offset its centreline here.

use super::bus::{BusTap, BusTapOrientation};
use rspice_design_model::Point;

/// Centre-to-centre spacing of a bus's strands, in world units.
pub const BUS_STRAND_OFFSET_WORLD: f32 = 4.0;
/// A bus strand's nominal stroke width, in world units.
pub const BUS_STRAND_WIDTH_WORLD: f32 = 1.2;
/// A tap's nominal stroke width, in world units.
pub const BUS_TAP_WIDTH_WORLD: f32 = 2.0;

const TAP_LEAD: i32 = 10;
const BREAKOUT_OFFSET: i32 = 4;

pub fn bus_tap_route_points(tap: &BusTap) -> Vec<Point> {
    let orientation = resolved_orientation(tap);
    // A final lead cannot fit when the target is at, behind, or no farther
    // than one lead length from the source in the requested direction.
    // A direct segment is the only non-retracing representation in that
    // geometry and is preferable to drawing a decorative loop.
    if orientation_progress(tap, orientation) <= i64::from(TAP_LEAD) {
        return vec![tap.bus_point, tap.connection_point];
    }
    let knee = match orientation {
        BusTapOrientation::Left => Point::new(
            tap.connection_point.x.saturating_add(TAP_LEAD),
            tap.connection_point.y,
        ),
        BusTapOrientation::Right => Point::new(
            tap.connection_point.x.saturating_sub(TAP_LEAD),
            tap.connection_point.y,
        ),
        BusTapOrientation::Up => Point::new(
            tap.connection_point.x,
            tap.connection_point.y.saturating_add(TAP_LEAD),
        ),
        BusTapOrientation::Down => Point::new(
            tap.connection_point.x,
            tap.connection_point.y.saturating_sub(TAP_LEAD),
        ),
        BusTapOrientation::Automatic => unreachable!("automatic orientation is resolved"),
    };

    let mut points = vec![tap.bus_point];
    let first_is_angled = tap.bus_point.x != knee.x && tap.bus_point.y != knee.y;
    if !first_is_angled {
        let breakout = match orientation {
            BusTapOrientation::Left | BusTapOrientation::Right => Point::new(
                midpoint_away_from_start(tap.bus_point.x, knee.x),
                tap.bus_point.y.saturating_sub(BREAKOUT_OFFSET),
            ),
            BusTapOrientation::Up | BusTapOrientation::Down => Point::new(
                tap.bus_point.x.saturating_add(BREAKOUT_OFFSET),
                midpoint_away_from_start(tap.bus_point.y, knee.y),
            ),
            BusTapOrientation::Automatic => unreachable!("automatic orientation is resolved"),
        };
        push_unique(&mut points, breakout);
    }
    push_unique(&mut points, knee);
    push_unique(&mut points, tap.connection_point);
    points
}

fn orientation_progress(tap: &BusTap, orientation: BusTapOrientation) -> i64 {
    match orientation {
        BusTapOrientation::Left => i64::from(tap.bus_point.x) - i64::from(tap.connection_point.x),
        BusTapOrientation::Right => i64::from(tap.connection_point.x) - i64::from(tap.bus_point.x),
        BusTapOrientation::Up => i64::from(tap.bus_point.y) - i64::from(tap.connection_point.y),
        BusTapOrientation::Down => i64::from(tap.connection_point.y) - i64::from(tap.bus_point.y),
        BusTapOrientation::Automatic => unreachable!("automatic orientation is resolved"),
    }
}

fn resolved_orientation(tap: &BusTap) -> BusTapOrientation {
    if tap.orientation != BusTapOrientation::Automatic {
        return tap.orientation;
    }
    let dx = i64::from(tap.connection_point.x) - i64::from(tap.bus_point.x);
    let dy = i64::from(tap.connection_point.y) - i64::from(tap.bus_point.y);
    if dx.unsigned_abs() >= dy.unsigned_abs() {
        if dx < 0 {
            BusTapOrientation::Left
        } else {
            BusTapOrientation::Right
        }
    } else if dy < 0 {
        BusTapOrientation::Up
    } else {
        BusTapOrientation::Down
    }
}

fn midpoint_away_from_start(start: i32, end: i32) -> i32 {
    let midpoint = ((i64::from(start) + i64::from(end)) / 2) as i32;
    if midpoint == start && end != start {
        end
    } else {
        midpoint
    }
}

fn push_unique(points: &mut Vec<Point>, point: Point) {
    if points.last() != Some(&point) {
        points.push(point);
    }
}

/// A polyline moved `offset` along its left normal, each inner vertex on
/// the miter of its two segments (the miter capped at four times the
/// offset), so a bus's outer strands stay parallel through its corners.
pub fn offset_polyline(points: &[(f64, f64)], offset: f64) -> Vec<(f64, f64)> {
    if points.len() < 2 || offset == 0.0 {
        return points.to_vec();
    }
    let normal = |start: (f64, f64), end: (f64, f64)| {
        let (dx, dy) = (end.0 - start.0, end.1 - start.1);
        let length = (dx * dx + dy * dy).sqrt().max(f64::EPSILON);
        (-dy / length, dx / length)
    };
    points
        .iter()
        .enumerate()
        .map(|(index, &point)| {
            let vector = if index == 0 {
                normal(points[0], points[1])
            } else if index + 1 == points.len() {
                normal(points[index - 1], points[index])
            } else {
                let previous = normal(points[index - 1], points[index]);
                let next = normal(points[index], points[index + 1]);
                let sum = (previous.0 + next.0, previous.1 + next.1);
                let length = (sum.0 * sum.0 + sum.1 * sum.1).sqrt();
                if length <= f64::EPSILON {
                    next
                } else {
                    let miter = (sum.0 / length, sum.1 / length);
                    let denominator = (miter.0 * next.0 + miter.1 * next.1).abs().max(0.25);
                    (miter.0 / denominator, miter.1 / denominator)
                }
            };
            (point.0 + vector.0 * offset, point.1 + vector.1 * offset)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schematic::bus::{Bus, BusDeclaration, BusSlice};

    fn tap(orientation: BusTapOrientation, connection: Point) -> BusTap {
        let bus = Bus::segment(
            1,
            Point::new(0, 0),
            Point::new(40, 0),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
        BusTap::new(
            2,
            &bus,
            Point::new(20, 0),
            connection,
            BusSlice::parse("DATA[3]").unwrap(),
            orientation,
        )
        .unwrap()
    }

    #[test]
    fn right_tap_has_angled_breakout_and_horizontal_final_run() {
        let points = bus_tap_route_points(&tap(BusTapOrientation::Right, Point::new(40, 10)));
        assert!(points.len() >= 3);
        let first = &points[0..2];
        assert_ne!(first[0].x, first[1].x);
        assert_ne!(first[0].y, first[1].y);
        let final_segment = &points[points.len() - 2..];
        assert_eq!(final_segment[0].y, final_segment[1].y);
        assert!(final_segment[1].x > final_segment[0].x);
    }

    #[test]
    fn automatic_orientation_is_stable_from_endpoint_geometry() {
        let points = bus_tap_route_points(&tap(BusTapOrientation::Automatic, Point::new(40, 10)));
        let final_segment = &points[points.len() - 2..];
        assert_eq!(final_segment[0].y, final_segment[1].y);
        assert!(final_segment[1].x > final_segment[0].x);
    }

    #[test]
    fn each_orientation_has_a_non_retracing_breakout_when_space_allows() {
        for (orientation, connection) in [
            (BusTapOrientation::Left, Point::new(0, 0)),
            (BusTapOrientation::Right, Point::new(40, 0)),
            (BusTapOrientation::Up, Point::new(20, -20)),
            (BusTapOrientation::Down, Point::new(20, 20)),
        ] {
            let points = bus_tap_route_points(&tap(orientation, connection));
            assert!(points.len() >= 3, "{orientation:?}: {points:?}");
            assert!(points.windows(2).all(|pair| pair[0] != pair[1]));
            assert_ne!(points[0].x, points[1].x, "{orientation:?}: {points:?}");
            assert_ne!(points[0].y, points[1].y, "{orientation:?}: {points:?}");
        }
    }

    #[test]
    fn short_and_exact_lead_routes_degrade_to_one_direct_segment() {
        for distance in [TAP_LEAD - 1, TAP_LEAD] {
            for (orientation, connection) in [
                (BusTapOrientation::Left, Point::new(20 - distance, 0)),
                (BusTapOrientation::Right, Point::new(20 + distance, 0)),
                (BusTapOrientation::Up, Point::new(20, -distance)),
                (BusTapOrientation::Down, Point::new(20, distance)),
            ] {
                let points = bus_tap_route_points(&tap(orientation, connection));
                assert_eq!(points, vec![Point::new(20, 0), connection]);
            }
        }
    }

    #[test]
    fn extreme_coordinates_do_not_overflow_route_geometry() {
        let bus = Bus::segment(
            1,
            Point::new(i32::MIN, i32::MIN),
            Point::new(i32::MAX, i32::MAX),
            Some(BusDeclaration::parse("DATA[7:0]").unwrap()),
        )
        .unwrap();
        let tap = BusTap::new(
            2,
            &bus,
            Point::new(i32::MIN, i32::MIN),
            Point::new(i32::MAX, i32::MAX),
            BusSlice::parse("DATA[3]").unwrap(),
            BusTapOrientation::Automatic,
        )
        .unwrap();

        let points = bus_tap_route_points(&tap);

        assert_eq!(points.first(), Some(&tap.bus_point));
        assert_eq!(points.last(), Some(&tap.connection_point));
    }
}

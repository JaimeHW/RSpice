//! Shared schematic hit geometry.

use rspice_design_model::Point;

pub fn point_in_rect(point: Point, min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> bool {
    point.x >= min_x && point.x <= max_x && point.y >= min_y && point.y <= max_y
}

pub fn rects_intersect(
    min: Point,
    max: Point,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
) -> bool {
    max.x >= min_x && min.x <= max_x && max.y >= min_y && min.y <= max_y
}

pub fn segment_intersects_rect(
    start: Point,
    end: Point,
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
) -> bool {
    if point_in_rect(start, min_x, min_y, max_x, max_y)
        || point_in_rect(end, min_x, min_y, max_x, max_y)
    {
        return true;
    }

    let dx = f64::from(end.x) - f64::from(start.x);
    let dy = f64::from(end.y) - f64::from(start.y);
    let mut enter = 0.0_f64;
    let mut leave = 1.0_f64;
    for (p, q) in [
        (-dx, f64::from(start.x) - f64::from(min_x)),
        (dx, f64::from(max_x) - f64::from(start.x)),
        (-dy, f64::from(start.y) - f64::from(min_y)),
        (dy, f64::from(max_y) - f64::from(start.y)),
    ] {
        if p == 0.0 {
            if q < 0.0 {
                return false;
            }
            continue;
        }
        let ratio = q / p;
        if p < 0.0 {
            enter = enter.max(ratio);
        } else {
            leave = leave.min(ratio);
        }
        if enter > leave {
            return false;
        }
    }
    true
}

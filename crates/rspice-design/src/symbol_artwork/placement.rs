//! A symbol's artwork as one placed instance draws it.

use super::types::{PathCommand, Symbol};

/// Straight pieces a cubic curve is drawn with.
const CURVE_SEGMENTS: u16 = 16;

/// One stroked run or filled outline of a symbol's artwork, in schematic
/// units about the instance's position.
#[derive(Debug, Clone, PartialEq)]
pub struct PlacedRun {
    /// The run's points in drawing order; a closed path repeats its first.
    pub points: Vec<(f32, f32)>,
    /// Whether the run is a filled outline rather than a stroke.
    pub filled: bool,
}

impl Symbol {
    /// The artwork as an instance draws it at `target_width` x
    /// `target_height`: scaled to fill that box, mirrored, then turned by
    /// `rotation_degrees` about the symbol's centre, with every curve drawn
    /// as straight pieces. The canvas paints this geometry, and whatever
    /// judges where a native part's ink lies reads the same.
    pub fn placed_runs(
        &self,
        target_width: f32,
        target_height: f32,
        rotation_degrees: i32,
        mirror_h: bool,
        mirror_v: bool,
    ) -> Vec<PlacedRun> {
        let radians = rotation_degrees as f32 * std::f32::consts::PI / 180.0;
        let (sine, cosine) = (radians.sin(), radians.cos());
        let (centre_x, centre_y) = self.center();
        let scale_x = target_width / self.width().max(0.001);
        let scale_y = target_height / self.height().max(0.001);
        // Scaled before mirroring, mirrored before turning, so the symbol
        // fills its box exactly and its terminals land on the grid.
        let place = |x: f32, y: f32| {
            let scaled_x = (x - centre_x) * scale_x;
            let scaled_y = (y - centre_y) * scale_y;
            let mirrored_x = if mirror_h { -scaled_x } else { scaled_x };
            let mirrored_y = if mirror_v { -scaled_y } else { scaled_y };
            (
                mirrored_x * cosine - mirrored_y * sine,
                mirrored_x * sine + mirrored_y * cosine,
            )
        };
        let mut runs = Vec::new();
        for path in &self.paths {
            let mut points: Vec<(f32, f32)> = Vec::new();
            let mut pen = (0.0_f32, 0.0_f32);
            let mut finish = |points: &mut Vec<(f32, f32)>| {
                if points.len() >= 2 {
                    runs.push(PlacedRun {
                        points: std::mem::take(points),
                        filled: path.filled,
                    });
                } else {
                    points.clear();
                }
            };
            for command in &path.commands {
                match *command {
                    PathCommand::MoveTo(x, y) => {
                        finish(&mut points);
                        points.push(place(x, y));
                        pen = (x, y);
                    }
                    PathCommand::LineTo(x, y) => {
                        points.push(place(x, y));
                        pen = (x, y);
                    }
                    PathCommand::CurveTo { ctrl1, ctrl2, end } => {
                        for step in 0..=CURVE_SEGMENTS {
                            let t = f32::from(step) / f32::from(CURVE_SEGMENTS);
                            let (x, y) = cubic_point(pen, ctrl1, ctrl2, end, t);
                            let point = place(x, y);
                            // The curve's start repeats the point it leaves.
                            if step == 0
                                && points.last().is_some_and(|last| {
                                    (last.0 - point.0).abs() < 0.1 && (last.1 - point.1).abs() < 0.1
                                })
                            {
                                continue;
                            }
                            points.push(point);
                        }
                        pen = end;
                    }
                    PathCommand::Close => {
                        if let Some(first) = points.first().copied() {
                            points.push(first);
                        }
                    }
                }
            }
            finish(&mut points);
        }
        runs
    }
}

/// The point at `t` along the cubic curve from `start` to `end`.
fn cubic_point(
    start: (f32, f32),
    ctrl1: (f32, f32),
    ctrl2: (f32, f32),
    end: (f32, f32),
    t: f32,
) -> (f32, f32) {
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let mt3 = mt2 * mt;
    (
        mt3 * start.0 + 3.0 * mt2 * t * ctrl1.0 + 3.0 * mt * t2 * ctrl2.0 + t3 * end.0,
        mt3 * start.1 + 3.0 * mt2 * t * ctrl1.1 + 3.0 * mt * t2 * ctrl2.1 + t3 * end.1,
    )
}

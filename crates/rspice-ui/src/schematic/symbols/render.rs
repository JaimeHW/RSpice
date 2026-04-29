use egui::{Painter, Pos2, Stroke};
use std::f32::consts::PI;

use super::types::{PathCommand, Symbol};

/// Render a symbol to an egui painter with position, scale, rotation, mirroring, and stroke
pub fn draw_symbol(
    painter: &Painter,
    symbol: &Symbol,
    center: Pos2,
    scale: f32,
    rotation_degrees: i32,
    mirror_h: bool,
    mirror_v: bool,
    stroke: Stroke,
) {
    // Debug: log when drawing
    log::trace!(
        "Drawing SVG symbol '{}' at {:?}, {} paths, scale={}",
        symbol.name,
        center,
        symbol.paths.len(),
        scale
    );

    let rotation_rad = (rotation_degrees as f32) * PI / 180.0;
    let cos_r = rotation_rad.cos();
    let sin_r = rotation_rad.sin();

    // Symbol center for rotation pivot
    let (cx, cy) = symbol.center();

    // Non-uniform scaling to exactly fill target dimensions:
    // This ensures terminal leads reach their expected positions on the grid.
    // For transistors, gate lead must reach -hw and D/S leads must reach ±hh.
    let view_scale_x = symbol.target_width / symbol.width().max(0.001);
    let view_scale_y = symbol.target_height / symbol.height().max(0.001);
    let total_scale_x = scale * view_scale_x;
    let total_scale_y = scale * view_scale_y;

    // Debug: log symbol dimensions
    log::debug!(
        "Symbol '{}' bounds=({:.1},{:.1},{:.1},{:.1}), target=({:.1}x{:.1}), scale=({:.2},{:.2})",
        symbol.name,
        symbol.bounds.0,
        symbol.bounds.1,
        symbol.bounds.2,
        symbol.bounds.3,
        symbol.target_width,
        symbol.target_height,
        total_scale_x,
        total_scale_y
    );

    let mut total_points_drawn = 0;

    for path in &symbol.paths {
        let mut points: Vec<Pos2> = Vec::new();
        let mut current_pos = (0.0f32, 0.0f32);

        for cmd in &path.commands {
            match cmd {
                PathCommand::MoveTo(x, y) => {
                    // Draw accumulated points first
                    if points.len() >= 2 {
                        draw_path_segment(painter, &points, stroke);
                        total_points_drawn += points.len();
                    }
                    points.clear();

                    // Transform point with non-uniform scaling and mirroring
                    let (tx, ty) = transform_point_nonuniform(
                        *x,
                        *y,
                        cx,
                        cy,
                        cos_r,
                        sin_r,
                        total_scale_x,
                        total_scale_y,
                        mirror_h,
                        mirror_v,
                        center,
                    );
                    points.push(Pos2::new(tx, ty));
                    current_pos = (*x, *y);
                }
                PathCommand::LineTo(x, y) => {
                    let (tx, ty) = transform_point_nonuniform(
                        *x,
                        *y,
                        cx,
                        cy,
                        cos_r,
                        sin_r,
                        total_scale_x,
                        total_scale_y,
                        mirror_h,
                        mirror_v,
                        center,
                    );
                    points.push(Pos2::new(tx, ty));
                    current_pos = (*x, *y);
                }
                PathCommand::CurveTo { ctrl1, ctrl2, end } => {
                    // Approximate bezier with line segments
                    // Start from t=0 to ensure we don't skip the starting connection
                    let segments = 16; // Use more segments for smoother curves
                    for i in 0..=segments {
                        let t = i as f32 / segments as f32;
                        let (bx, by) = cubic_bezier(current_pos, *ctrl1, *ctrl2, *end, t);
                        let (tx, ty) = transform_point_nonuniform(
                            bx,
                            by,
                            cx,
                            cy,
                            cos_r,
                            sin_r,
                            total_scale_x,
                            total_scale_y,
                            mirror_h,
                            mirror_v,
                            center,
                        );
                        // Skip adding t=0 point if it would duplicate the previous MoveTo point
                        if i == 0 && !points.is_empty() {
                            // Check if we'd be adding a duplicate
                            let last = points.last().unwrap();
                            let new_point = Pos2::new(tx, ty);
                            if (last.x - new_point.x).abs() < 0.1
                                && (last.y - new_point.y).abs() < 0.1
                            {
                                continue;
                            }
                        }
                        points.push(Pos2::new(tx, ty));
                    }
                    current_pos = *end;
                }
                PathCommand::Close => {
                    if let Some(first) = points.first() {
                        points.push(*first);
                    }
                }
            }
        }

        // Draw remaining points
        if points.len() >= 2 {
            draw_path_segment(painter, &points, stroke);
            total_points_drawn += points.len();
        }
    }

    // Debug: if no points were drawn, log a warning
    if total_points_drawn == 0 {
        log::warn!("SVG symbol '{}' had 0 points to draw!", symbol.name);
    }

    // Note: Lead lines are handled by the SVG artwork itself.
    // The SVG symbols are designed with lead lines that extend to terminal positions.
    // Uniform scaling preserves these leads proportionally.
}

/// Draw a sequence of connected line segments using individual line calls
fn draw_path_segment(painter: &Painter, points: &[Pos2], stroke: Stroke) {
    if points.len() < 2 {
        return;
    }

    // Draw circles FIRST (behind lines) at vertices where direction changes
    // This fills gaps at corners without visible circle edges (lines cover them)
    let radius = stroke.width / 2.0;

    // Check middle vertices for direction changes
    for i in 1..points.len().saturating_sub(1) {
        let prev = points[i - 1];
        let curr = points[i];
        let next = points[i + 1];

        // Calculate direction vectors
        let d1 = (curr.x - prev.x, curr.y - prev.y);
        let d2 = (next.x - curr.x, next.y - curr.y);

        // Normalize and check if directions are different (not collinear)
        let len1 = (d1.0 * d1.0 + d1.1 * d1.1).sqrt();
        let len2 = (d2.0 * d2.0 + d2.1 * d2.1).sqrt();

        if len1 > 0.001 && len2 > 0.001 {
            let n1 = (d1.0 / len1, d1.1 / len1);
            let n2 = (d2.0 / len2, d2.1 / len2);

            // Dot product - if close to 1, segments are collinear
            let dot = n1.0 * n2.0 + n1.1 * n2.1;

            // Only draw at sharp corners (angle > ~30 degrees)
            if dot.abs() < 0.87 {
                painter.circle_filled(curr, radius, stroke.color);
            }
        }
    }

    // Draw line segments ON TOP of circles
    for pair in points.windows(2) {
        painter.line_segment([pair[0], pair[1]], stroke);
    }
}

/// Transform a point with rotation around symbol center and non-uniform scale.
/// Non-uniform scaling is applied BEFORE rotation to ensure the symbol fills
/// exactly its target dimensions. This guarantees terminals land on grid lines.
/// Mirroring is applied BEFORE rotation but AFTER scaling.
#[allow(dead_code)]
fn transform_point_nonuniform(
    x: f32,
    y: f32,
    cx: f32,
    cy: f32,
    cos_r: f32,
    sin_r: f32,
    scale_x: f32,
    scale_y: f32,
    mirror_h: bool,
    mirror_v: bool,
    center: Pos2,
) -> (f32, f32) {
    // Translate to symbol origin
    let dx = x - cx;
    let dy = y - cy;

    // Apply non-uniform scaling BEFORE rotation
    // This ensures the symbol exactly fills its target dimensions
    let sx = dx * scale_x;
    let sy = dy * scale_y;

    // Apply mirroring BEFORE rotation
    // mirror_h flips horizontally (negate x), mirror_v flips vertically (negate y)
    let mx = if mirror_h { -sx } else { sx };
    let my = if mirror_v { -sy } else { sy };

    // Rotate the mirrored and scaled point
    let rx = mx * cos_r - my * sin_r;
    let ry = mx * sin_r + my * cos_r;

    // Translate to screen position
    (center.x + rx, center.y + ry)
}

/// Evaluate cubic bezier at parameter t
fn cubic_bezier(
    p0: (f32, f32),
    p1: (f32, f32),
    p2: (f32, f32),
    p3: (f32, f32),
    t: f32,
) -> (f32, f32) {
    let t2 = t * t;
    let t3 = t2 * t;
    let mt = 1.0 - t;
    let mt2 = mt * mt;
    let mt3 = mt2 * mt;

    let x = mt3 * p0.0 + 3.0 * mt2 * t * p1.0 + 3.0 * mt * t2 * p2.0 + t3 * p3.0;
    let y = mt3 * p0.1 + 3.0 * mt2 * t * p1.1 + 3.0 * mt * t2 * p2.1 + t3 * p3.1;
    (x, y)
}

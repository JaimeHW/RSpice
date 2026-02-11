use egui::{Painter, Pos2, Stroke};

use crate::state::Rotation;

pub(super) fn rotation_to_index(rotation: Rotation) -> i32 {
    match rotation {
        Rotation::R0 => 0,
        Rotation::R90 => 1,
        Rotation::R180 => 2,
        Rotation::R270 => 3,
    }
}

// =============================================================================
// Component Symbol Drawing Functions
// =============================================================================

pub(super) fn draw_resistor_symbol(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    rotation: i32,
    stroke: Stroke,
) {
    let len = 30.0 * scale;
    let width = 8.0 * scale;
    let segments = 6;

    let (dx, dy) = rotation_to_delta(rotation);
    let perp = (-dy, dx);

    // Start point
    let start = Pos2::new(pos.x - len / 2.0 * dx, pos.y - len / 2.0 * dy);

    // Draw zigzag
    let seg_len = len / (segments as f32);
    let mut points = vec![start];

    for i in 1..segments {
        let t = i as f32 * seg_len;
        let offset = if i % 2 == 1 { width } else { -width };
        points.push(Pos2::new(
            start.x + t * dx + offset * perp.0,
            start.y + t * dy + offset * perp.1,
        ));
    }
    points.push(Pos2::new(pos.x + len / 2.0 * dx, pos.y + len / 2.0 * dy));

    for i in 0..points.len() - 1 {
        painter.line_segment([points[i], points[i + 1]], stroke);
    }
}

pub(super) fn draw_capacitor_symbol(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    rotation: i32,
    stroke: Stroke,
) {
    let gap = 4.0 * scale;
    let plate_len = 16.0 * scale;

    let (dx, dy) = rotation_to_delta(rotation);
    let perp = (-dy, dx);

    // Two parallel plates
    let p1a = Pos2::new(
        pos.x - gap / 2.0 * dx + plate_len / 2.0 * perp.0,
        pos.y - gap / 2.0 * dy + plate_len / 2.0 * perp.1,
    );
    let p1b = Pos2::new(
        pos.x - gap / 2.0 * dx - plate_len / 2.0 * perp.0,
        pos.y - gap / 2.0 * dy - plate_len / 2.0 * perp.1,
    );

    let p2a = Pos2::new(
        pos.x + gap / 2.0 * dx + plate_len / 2.0 * perp.0,
        pos.y + gap / 2.0 * dy + plate_len / 2.0 * perp.1,
    );
    let p2b = Pos2::new(
        pos.x + gap / 2.0 * dx - plate_len / 2.0 * perp.0,
        pos.y + gap / 2.0 * dy - plate_len / 2.0 * perp.1,
    );

    painter.line_segment([p1a, p1b], stroke);
    painter.line_segment([p2a, p2b], stroke);

    // Lead lines
    let lead_len = 10.0 * scale;
    painter.line_segment(
        [
            Pos2::new(
                pos.x - (gap / 2.0 + lead_len) * dx,
                pos.y - (gap / 2.0 + lead_len) * dy,
            ),
            Pos2::new(pos.x - gap / 2.0 * dx, pos.y - gap / 2.0 * dy),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(pos.x + gap / 2.0 * dx, pos.y + gap / 2.0 * dy),
            Pos2::new(
                pos.x + (gap / 2.0 + lead_len) * dx,
                pos.y + (gap / 2.0 + lead_len) * dy,
            ),
        ],
        stroke,
    );
}

pub(super) fn draw_inductor_symbol(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    rotation: i32,
    stroke: Stroke,
) {
    let len = 30.0 * scale;
    let radius = 5.0 * scale;
    let num_coils = 4;

    let (dx, dy) = rotation_to_delta(rotation);

    // Draw series of arcs
    let coil_spacing = len / (num_coils as f32);
    for i in 0..num_coils {
        let cx = pos.x - len / 2.0 * dx + (i as f32 + 0.5) * coil_spacing * dx;
        let cy = pos.y - len / 2.0 * dy + (i as f32 + 0.5) * coil_spacing * dy;

        // Approximate arc with circle (simplified)
        painter.circle_stroke(Pos2::new(cx, cy), radius, stroke);
    }
}

pub(super) fn draw_vsource_symbol(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    _rotation: i32,
    stroke: Stroke,
) {
    let radius = 12.0 * scale;

    // Circle
    painter.circle_stroke(pos, radius, stroke);

    // Plus sign at top
    let plus_y = pos.y - 5.0 * scale;
    painter.line_segment(
        [
            Pos2::new(pos.x - 4.0 * scale, plus_y),
            Pos2::new(pos.x + 4.0 * scale, plus_y),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(pos.x, plus_y - 4.0 * scale),
            Pos2::new(pos.x, plus_y + 4.0 * scale),
        ],
        stroke,
    );

    // Minus at bottom
    let minus_y = pos.y + 5.0 * scale;
    painter.line_segment(
        [
            Pos2::new(pos.x - 4.0 * scale, minus_y),
            Pos2::new(pos.x + 4.0 * scale, minus_y),
        ],
        stroke,
    );
}

pub(super) fn draw_isource_symbol(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    _rotation: i32,
    stroke: Stroke,
) {
    let radius = 12.0 * scale;

    // Circle
    painter.circle_stroke(pos, radius, stroke);

    // Arrow
    let arrow_len = 10.0 * scale;
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y + arrow_len / 2.0),
            Pos2::new(pos.x, pos.y - arrow_len / 2.0),
        ],
        stroke,
    );

    // Arrow head
    let head_size = 3.0 * scale;
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - arrow_len / 2.0),
            Pos2::new(pos.x - head_size, pos.y - arrow_len / 2.0 + head_size),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - arrow_len / 2.0),
            Pos2::new(pos.x + head_size, pos.y - arrow_len / 2.0 + head_size),
        ],
        stroke,
    );
}

pub(super) fn draw_ground_symbol(painter: &Painter, pos: Pos2, scale: f32, stroke: Stroke) {
    let width = 14.0 * scale;

    // Three horizontal lines, decreasing in width
    for i in 0..3 {
        let y = pos.y + (i as f32) * 4.0 * scale;
        let w = width - (i as f32) * 4.0 * scale;
        painter.line_segment(
            [Pos2::new(pos.x - w / 2.0, y), Pos2::new(pos.x + w / 2.0, y)],
            stroke,
        );
    }

    // Vertical lead
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - 6.0 * scale),
            Pos2::new(pos.x, pos.y),
        ],
        stroke,
    );
}

pub(super) fn draw_diode_symbol(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    rotation: i32,
    stroke: Stroke,
) {
    let size = 10.0 * scale;
    let (dx, dy) = rotation_to_delta(rotation);
    let perp = (-dy, dx);

    // Triangle
    let tip = Pos2::new(pos.x + size * dx, pos.y + size * dy);
    let base1 = Pos2::new(
        pos.x - size * dx + size * perp.0,
        pos.y - size * dy + size * perp.1,
    );
    let base2 = Pos2::new(
        pos.x - size * dx - size * perp.0,
        pos.y - size * dy - size * perp.1,
    );

    painter.line_segment([tip, base1], stroke);
    painter.line_segment([base1, base2], stroke);
    painter.line_segment([base2, tip], stroke);

    // Bar at cathode
    painter.line_segment(
        [
            Pos2::new(tip.x + size * perp.0, tip.y + size * perp.1),
            Pos2::new(tip.x - size * perp.0, tip.y - size * perp.1),
        ],
        stroke,
    );
}

pub(super) fn draw_nmos_symbol(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    _rotation: i32,
    stroke: Stroke,
) {
    let size = 12.0 * scale;

    // Channel (vertical line)
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - size),
            Pos2::new(pos.x, pos.y + size),
        ],
        stroke,
    );

    // Gate (vertical line to the left)
    painter.line_segment(
        [
            Pos2::new(pos.x - size * 0.5, pos.y - size * 0.7),
            Pos2::new(pos.x - size * 0.5, pos.y + size * 0.7),
        ],
        stroke,
    );

    // Gate lead
    painter.line_segment(
        [
            Pos2::new(pos.x - size, pos.y),
            Pos2::new(pos.x - size * 0.5, pos.y),
        ],
        stroke,
    );

    // Drain/Source leads
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - size),
            Pos2::new(pos.x + size, pos.y - size),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y + size),
            Pos2::new(pos.x + size, pos.y + size),
        ],
        stroke,
    );
}

pub(super) fn draw_pmos_symbol(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    rotation: i32,
    stroke: Stroke,
) {
    // Same as NMOS but with bubble on gate
    draw_nmos_symbol(painter, pos, scale, rotation, stroke);

    let size = 12.0 * scale;
    let bubble_radius = 2.5 * scale;
    painter.circle_stroke(
        Pos2::new(pos.x - size * 0.5 - bubble_radius, pos.y),
        bubble_radius,
        stroke,
    );
}

pub(super) fn draw_npn_symbol(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    _rotation: i32,
    stroke: Stroke,
) {
    let size = 12.0 * scale;

    // Base line (vertical)
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - size * 0.6),
            Pos2::new(pos.x, pos.y + size * 0.6),
        ],
        stroke,
    );

    // Base lead
    painter.line_segment(
        [Pos2::new(pos.x - size, pos.y), Pos2::new(pos.x, pos.y)],
        stroke,
    );

    // Collector
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - size * 0.3),
            Pos2::new(pos.x + size, pos.y - size),
        ],
        stroke,
    );

    // Emitter with arrow
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y + size * 0.3),
            Pos2::new(pos.x + size, pos.y + size),
        ],
        stroke,
    );
}

pub(super) fn draw_pnp_symbol(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    _rotation: i32,
    stroke: Stroke,
) {
    let size = 12.0 * scale;

    // Base line (vertical)
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - size * 0.6),
            Pos2::new(pos.x, pos.y + size * 0.6),
        ],
        stroke,
    );

    // Base lead
    painter.line_segment(
        [Pos2::new(pos.x - size, pos.y), Pos2::new(pos.x, pos.y)],
        stroke,
    );

    // Emitter (arrow pointing in)
    painter.line_segment(
        [
            Pos2::new(pos.x + size, pos.y - size),
            Pos2::new(pos.x, pos.y - size * 0.3),
        ],
        stroke,
    );

    // Collector
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y + size * 0.3),
            Pos2::new(pos.x + size, pos.y + size),
        ],
        stroke,
    );
}

// =============================================================================
// Controlled Source Symbols
// =============================================================================

/// Draw VCVS (Voltage-Controlled Voltage Source) - diamond with + and -
pub(super) fn draw_vcvs_symbol(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    _rotation: i32,
    stroke: Stroke,
) {
    draw_diamond_source(painter, pos, scale, stroke);
    // Add + and - for voltage output
    let size = 12.0 * scale;
    painter.text(
        Pos2::new(pos.x - size * 0.3, pos.y - size * 0.3),
        egui::Align2::CENTER_CENTER,
        "+",
        egui::FontId::proportional(10.0 * scale),
        stroke.color,
    );
    painter.text(
        Pos2::new(pos.x + size * 0.3, pos.y + size * 0.3),
        egui::Align2::CENTER_CENTER,
        "−",
        egui::FontId::proportional(12.0 * scale),
        stroke.color,
    );
}

/// Draw VCCS (Voltage-Controlled Current Source) - diamond with arrow
pub(super) fn draw_vccs_symbol(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    _rotation: i32,
    stroke: Stroke,
) {
    draw_diamond_source(painter, pos, scale, stroke);
    // Add arrow for current output
    let arr_len = 8.0 * scale;
    let head = 3.0 * scale;
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y + arr_len / 2.0),
            Pos2::new(pos.x, pos.y - arr_len / 2.0),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - arr_len / 2.0),
            Pos2::new(pos.x - head, pos.y - arr_len / 2.0 + head),
        ],
        stroke,
    );
    painter.line_segment(
        [
            Pos2::new(pos.x, pos.y - arr_len / 2.0),
            Pos2::new(pos.x + head, pos.y - arr_len / 2.0 + head),
        ],
        stroke,
    );
}

/// Draw CCVS (Current-Controlled Voltage Source) - diamond with + and -
pub(super) fn draw_ccvs_symbol(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    rotation: i32,
    stroke: Stroke,
) {
    // CCVS looks like VCVS but input is current-sensing
    draw_vcvs_symbol(painter, pos, scale, rotation, stroke);
}

/// Draw CCCS (Current-Controlled Current Source) - diamond with arrow
pub(super) fn draw_cccs_symbol(
    painter: &Painter,
    pos: Pos2,
    scale: f32,
    rotation: i32,
    stroke: Stroke,
) {
    // CCCS looks like VCCS but input is current-sensing
    draw_vccs_symbol(painter, pos, scale, rotation, stroke);
}

/// Helper: Draw diamond shape for controlled sources
pub(super) fn draw_diamond_source(painter: &Painter, pos: Pos2, scale: f32, stroke: Stroke) {
    let size = 12.0 * scale;
    let top = Pos2::new(pos.x, pos.y - size);
    let right = Pos2::new(pos.x + size, pos.y);
    let bottom = Pos2::new(pos.x, pos.y + size);
    let left = Pos2::new(pos.x - size, pos.y);

    painter.line_segment([top, right], stroke);
    painter.line_segment([right, bottom], stroke);
    painter.line_segment([bottom, left], stroke);
    painter.line_segment([left, top], stroke);
}

/// Convert rotation index (0-3 for 0°, 90°, 180°, 270°) to direction deltas
pub(super) fn rotation_to_delta(rotation: i32) -> (f32, f32) {
    match rotation % 4 {
        0 => (1.0, 0.0),  // 0°: pointing right
        1 => (0.0, 1.0),  // 90°: pointing down
        2 => (-1.0, 0.0), // 180°: pointing left
        3 => (0.0, -1.0), // 270°: pointing up
        _ => (1.0, 0.0),
    }
}

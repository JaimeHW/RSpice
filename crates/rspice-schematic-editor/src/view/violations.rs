//! Read-only check markers and active-sheet location resolution.

use super::{design_view::DesignView, viewport::Viewport};
use egui::{Painter, Pos2, Shape, Stroke, pos2, vec2};
use rspice_design::drc::{DrcLocation, DrcSeverity, DrcViolation};
use rspice_design_model::Point;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::active_palette;

/// Marker half-extent in screen px (badges do not scale with zoom — they
/// are annotations, not geometry).
const HALF: f32 = 7.0;

/// Draw violation badges and the hover card for the closest hovered one.
/// The app supplies only check results current for this design view.
pub fn draw_violation_markers(
    painter: &Painter,
    viewport: &Viewport,
    view: &DesignView<'_>,
    violations: &[DrcViolation],
    canvas_hover: Option<(f64, f64)>,
) {
    let palette = active_palette();
    let cursor = cursor_screen_pos(view, viewport, canvas_hover);
    let mut hovered: Option<(Pos2, &DrcViolation)> = None;

    for violation in violations {
        let Some(world) = anchor(view, &violation.location) else {
            continue;
        };
        let pos = viewport.schematic_to_screen(world);
        if !viewport.bounds.expand(HALF * 2.0).contains(pos) {
            continue;
        }

        let color = severity_color(violation.severity, &palette);
        draw_badge(painter, pos, color, palette.canvas_bg);

        if let Some(cursor) = cursor
            && cursor.distance(pos) <= HALF + 4.0
        {
            hovered = Some((pos, violation));
        }
    }

    if let Some((pos, violation)) = hovered {
        draw_hover_card(painter, viewport, pos, violation);
    }
}

fn severity_color(
    severity: DrcSeverity,
    palette: &rspice_ui_kit::palette::Palette,
) -> egui::Color32 {
    match severity {
        DrcSeverity::Critical | DrcSeverity::Error => palette.err,
        DrcSeverity::Warning => palette.warn,
        DrcSeverity::Info => palette.text_dim,
    }
}

/// Resolve a violation to a schematic-space anchor, when it has one.
/// `Node` and `Global` violations have no single spot — they live in the
/// console summary and the ERC pill only.
pub fn anchor(view: &DesignView<'_>, location: &DrcLocation) -> Option<Point> {
    match location {
        DrcLocation::Point { x, y } => Some(Point::new(*x as i32, *y as i32)),
        DrcLocation::Component { id, .. } => view
            .document
            .components
            .iter()
            .find(|component| component.id == *id && view.object_is_visible(component.id))
            .map(|c| c.pos),
        DrcLocation::Wire { id } => view
            .document
            .wires
            .iter()
            .find(|wire| wire.id == *id && view.object_is_visible(wire.id))
            .and_then(|w| {
                let first = w.points.first()?;
                let second = w.points.get(1).unwrap_or(first);
                Some(Point::new(
                    ((i64::from(first.x) + i64::from(second.x)) / 2) as i32,
                    ((i64::from(first.y) + i64::from(second.y)) / 2) as i32,
                ))
            }),
        DrcLocation::Bus { id } => view
            .document
            .buses
            .iter()
            .find(|bus| bus.id == *id && view.object_is_visible(bus.id))
            .and_then(|bus| bus.points.first().copied()),
        DrcLocation::BusTap { id } => view
            .document
            .bus_taps
            .iter()
            .find(|tap| tap.id == *id && view.object_is_visible(tap.id))
            .map(|tap| tap.connection_point),
        DrcLocation::NetLabel { name } => view
            .document
            .net_labels
            .iter()
            .find(|label| label.name == *name && view.object_is_visible(label.id))
            .map(|l| l.pos),
        DrcLocation::Node { .. } | DrcLocation::Global | DrcLocation::SymbolPin { .. } => None,
    }
}

/// The cursor in screen space, derived from the grid-unit hover the canvas
/// reports each frame.
fn cursor_screen_pos(
    view: &DesignView<'_>,
    viewport: &Viewport,
    canvas_hover: Option<(f64, f64)>,
) -> Option<Pos2> {
    let (gx, gy) = canvas_hover?;
    let grid = view.document.grid_size.max(1) as f32;
    Some(pos2(
        viewport.bounds.min.x + viewport.offset.x + (gx as f32) * grid * viewport.zoom,
        viewport.bounds.min.y + viewport.offset.y + (gy as f32) * grid * viewport.zoom,
    ))
}

/// A warning triangle with an exclamation tick, glyph in the canvas color
/// for contrast against the severity fill.
fn draw_badge(painter: &Painter, center: Pos2, fill: egui::Color32, ink: egui::Color32) {
    let points = vec![
        center + vec2(0.0, -HALF),
        center + vec2(HALF, HALF * 0.78),
        center + vec2(-HALF, HALF * 0.78),
    ];
    painter.add(Shape::convex_polygon(points, fill, Stroke::new(1.0, ink)));
    painter.line_segment(
        [center + vec2(0.0, -3.0), center + vec2(0.0, 1.0)],
        Stroke::new(1.5, ink),
    );
    painter.circle_filled(center + vec2(0.0, 3.4), 0.9, ink);
}

/// The hover card: severity + violation type, the message, and the
/// suggested fix, in an elevated box clamped to the canvas.
fn draw_hover_card(painter: &Painter, viewport: &Viewport, marker: Pos2, violation: &DrcViolation) {
    let palette = active_palette();

    let title = format!(
        "{} · {}",
        violation.severity.display_name(),
        violation.violation_type.description()
    );
    let fix = format!("Fix: {}", violation.violation_type.suggested_fix());

    let title_galley = painter.layout_no_wrap(
        title,
        theme::mono(11.0, FontWeight::Medium),
        severity_color(violation.severity, &palette),
    );
    let message_galley = painter.layout_no_wrap(
        truncate(&violation.message, 78),
        theme::sans(11.0, FontWeight::Regular),
        palette.text,
    );
    let fix_galley = painter.layout_no_wrap(
        truncate(&fix, 78),
        theme::sans(11.0, FontWeight::Regular),
        palette.text_faint,
    );

    let (pad_x, pad_y, line_gap) = (9.0, 7.0, 4.0);
    let width = title_galley
        .size()
        .x
        .max(message_galley.size().x)
        .max(fix_galley.size().x)
        + pad_x * 2.0;
    let height = title_galley.size().y
        + message_galley.size().y
        + fix_galley.size().y
        + line_gap * 2.0
        + pad_y * 2.0;

    let mut origin = marker + vec2(12.0, -height - 6.0);
    let bounds = viewport.bounds;
    if origin.x + width > bounds.right() - 4.0 {
        origin.x = marker.x - width - 12.0;
    }
    origin.y = origin.y.clamp(
        bounds.top() + 4.0,
        (bounds.bottom() - height - 4.0).max(bounds.top() + 4.0),
    );

    let rect = egui::Rect::from_min_size(origin, vec2(width, height));
    painter.rect(
        rect,
        3.0,
        palette.bg_elevated,
        Stroke::new(1.0, palette.border_strong),
        egui::StrokeKind::Inside,
    );

    let mut y = origin.y + pad_y;
    for galley in [title_galley, message_galley, fix_galley] {
        let galley_height = galley.size().y;
        painter.galley(pos2(origin.x + pad_x, y), galley, palette.text);
        y += galley_height + line_gap;
    }
}

/// Hard-truncate long strings for the single-line card rows.
fn truncate(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_owned();
    }
    let mut out: String = text.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}

#[cfg(test)]
mod tests;

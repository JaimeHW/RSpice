//! Dimmed ancestor connectivity, filtered against each ancestor's own sheet.
//! Parent symbols deliberately use footprints; child symbol IDs can collide.

use super::super::{
    design_view::DesignView,
    drawing::{chain_conductors, paint_conductor},
    viewport::Viewport,
};
use super::{CULL_MARGIN, polyline_intersects_view};
use egui::{Painter, Rect, Stroke};
use rspice_design_model::Point;
const PARENT_CONTEXT_OPACITY: f32 = 0.22;
const PARENT_CONTEXT_STROKE_WIDTH: f32 = 1.1;

pub fn draw_parent_context<'a>(
    painter: &Painter,
    viewport: &Viewport,
    sheets: impl IntoIterator<Item = DesignView<'a>>,
) {
    let palette = rspice_ui_kit::tokens::active_palette();
    let conductor = palette.wire.gamma_multiply(PARENT_CONTEXT_OPACITY);
    let outline = palette.symbol.gamma_multiply(PARENT_CONTEXT_OPACITY);
    let stroke = Stroke::new(PARENT_CONTEXT_STROKE_WIDTH * viewport.zoom, conductor);
    let (wx0, wy0, wx1, wy1) = viewport.visible_world_rect(CULL_MARGIN);

    for sheet in sheets {
        let wires = sheet
            .document
            .wires
            .iter()
            .filter(|wire| {
                sheet.object_is_visible(wire.id)
                    && polyline_intersects_view(&wire.points, wx0, wy0, wx1, wy1)
            })
            .map(|wire| wire.points.as_slice());
        for chain in chain_conductors(wires) {
            paint_conductor(
                painter,
                chain
                    .iter()
                    .map(|point| viewport.schematic_to_screen(*point))
                    .collect(),
                stroke,
            );
        }
        for bus in &sheet.document.buses {
            if !sheet.object_is_visible(bus.id)
                || !polyline_intersects_view(&bus.points, wx0, wy0, wx1, wy1)
            {
                continue;
            }
            paint_conductor(
                painter,
                bus.points
                    .iter()
                    .map(|point| viewport.schematic_to_screen(*point))
                    .collect(),
                stroke,
            );
        }
        for junction in &sheet.document.junctions {
            let (jx, jy) = (junction.pos.x as f32, junction.pos.y as f32);
            if !sheet.object_is_visible(junction.id) || jx < wx0 || jx > wx1 || jy < wy0 || jy > wy1
            {
                continue;
            }
            painter.circle_filled(
                viewport.schematic_to_screen(junction.pos),
                (2.0 * viewport.zoom).max(1.0),
                conductor,
            );
        }
        for component in &sheet.document.components {
            let (min_x, min_y, max_x, max_y) = component.bounding_box();
            if !sheet.object_is_visible(component.id)
                || (max_x as f32) < wx0
                || (min_x as f32) > wx1
                || (max_y as f32) < wy0
                || (min_y as f32) > wy1
            {
                continue;
            }
            painter.rect_stroke(
                Rect::from_two_pos(
                    viewport.schematic_to_screen(Point::new(min_x, min_y)),
                    viewport.schematic_to_screen(Point::new(max_x, max_y)),
                ),
                2.0,
                Stroke::new(1.0, outline),
                egui::StrokeKind::Inside,
            );
        }
    }
}

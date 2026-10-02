//! Wire route and electrical acquisition feedback.

use super::super::{
    coordinates::screen_to_schematic,
    drawing::{nearest_wire_screen_hit, paint_conductor},
    snap_resolution::{conductor_attachment_pitch, resolve_target_pointer},
    symbol_context::SchematicSymbolContext,
    viewport::Viewport,
};
use super::PreviewView;
use crate::session::snap::{SnapResult, SnapTarget, SnapTargetType};
use egui::{Painter, Pos2, Rect, Stroke};
use rspice_design_model::Point;

const WIRE_PREVIEW_STROKE_WIDTH: f32 = 1.5;

pub fn draw_wire_preview(
    painter: &Painter,
    view: &PreviewView<'_>,
    viewport: &Viewport,
    snap_feedback: Option<&SnapResult>,
) {
    if !view.can_edit {
        return;
    }
    let wire_active = view.editor.wire_drawing.active;

    if wire_active {
        let drawing = &view.editor.wire_drawing;
        let to_screen = |point: &Point| viewport.schematic_to_screen(*point);
        let committed: Vec<Pos2> = drawing.points.iter().map(to_screen).collect();
        // The whole route a click would commit, corner included, goes down
        // first in the hint tone as one mitered path, and the committed
        // prefix is painted over it in full tone with a butt end at the last
        // vertex. No end edge lies on another stroke's edge, which is what
        // turns egui's alpha-feathered anti-aliasing into a lighter hairline.
        let route: Vec<Pos2> = drawing.get_full_path().iter().map(to_screen).collect();

        if let Some(start) = committed.first().copied() {
            let wire_color = rspice_ui_kit::tokens::active_palette().accent;
            let width = WIRE_PREVIEW_STROKE_WIDTH * viewport.zoom;
            paint_conductor(
                painter,
                route,
                Stroke::new(width, wire_color.gamma_multiply(0.6)),
            );
            paint_conductor(painter, committed, Stroke::new(width, wire_color));
            painter.circle_filled(start, 4.0 * viewport.zoom, wire_color);
        }
    }

    if let Some(result) = snap_feedback {
        draw_wire_snap_feedback(painter, view, viewport, result);
    }
}

/// Resolve exactly what a wire click would commit. Visual conductor
/// acquisition owns the gesture before generic target priority, and a
/// non-representable diagonal acquisition fails closed instead of showing a
/// preview that cannot be committed.
pub fn resolve_wire_preview_snap(
    view: &PreviewView<'_>,
    symbol_context: &SchematicSymbolContext,
    viewport: &Viewport,
    pointer: egui::Pos2,
) -> Option<SnapResult> {
    if !view.editor.snap_engine.enabled {
        return Some(resolve_target_pointer(
            &view.design,
            &view.editor.snap_engine,
            symbol_context,
            viewport,
            pointer,
        ));
    }
    let active_wires = view
        .design
        .objects_on_active_sheet(&view.design.document.wires, |wire| wire.id);
    let Some(hit) = nearest_wire_screen_hit(
        viewport,
        active_wires.as_ref(),
        pointer,
        6.0,
        conductor_attachment_pitch(&view.editor.snap_engine, view.design.document.grid_size),
    ) else {
        return Some(resolve_target_pointer(
            &view.design,
            &view.editor.snap_engine,
            symbol_context,
            viewport,
            pointer,
        ));
    };
    let attachment = hit.attachment?;
    let wire = active_wires.iter().find(|wire| wire.id == hit.wire_id)?;
    let raw = screen_to_schematic(viewport, pointer);
    let distance = (f64::from(raw.x) - f64::from(attachment.x))
        .hypot(f64::from(raw.y) - f64::from(attachment.y));
    let target = if wire.points.first() == Some(&attachment) {
        view.editor
            .snap_engine
            .snap_to_wire_endpoints
            .then(|| SnapTarget::wire_endpoint(attachment, wire.id, true, distance))
    } else if wire.points.last() == Some(&attachment) {
        view.editor
            .snap_engine
            .snap_to_wire_endpoints
            .then(|| SnapTarget::wire_endpoint(attachment, wire.id, false, distance))
    } else {
        let segment_index = wire
            .segments()
            .position(|segment| segment.contains_point(attachment))
            .unwrap_or_default();
        view.editor
            .snap_engine
            .snap_to_wire_segments
            .then(|| SnapTarget::wire_segment(attachment, wire.id, segment_index, distance))
    };
    Some(match target {
        Some(target) => SnapResult::with_target(target, raw),
        None => resolve_target_pointer(
            &view.design,
            &view.editor.snap_engine,
            symbol_context,
            viewport,
            pointer,
        ),
    })
}

fn draw_wire_snap_feedback(
    painter: &Painter,
    view: &PreviewView<'_>,
    viewport: &Viewport,
    result: &SnapResult,
) {
    if !result.show_indicator {
        return;
    }
    let Some(copy) = wire_snap_feedback_copy(view, result) else {
        return;
    };

    let palette = rspice_ui_kit::tokens::active_palette();
    let center = viewport.schematic_to_screen(result.snapped_position);
    painter.circle_stroke(center, 5.0, Stroke::new(1.25, palette.accent));
    painter.line_segment(
        [center - egui::vec2(2.5, 0.0), center + egui::vec2(2.5, 0.0)],
        Stroke::new(1.0, palette.accent),
    );
    painter.line_segment(
        [center - egui::vec2(0.0, 2.5), center + egui::vec2(0.0, 2.5)],
        Stroke::new(1.0, palette.accent),
    );

    let galley = painter.layout_no_wrap(
        copy,
        rspice_ui_kit::theme::mono(
            rspice_ui_kit::tokens::FS_0,
            rspice_ui_kit::theme::FontWeight::Regular,
        ),
        palette.text,
    );
    let requested = center + egui::vec2(10.0, -galley.size().y * 0.5);
    let background_size = galley.size() + egui::vec2(8.0, 5.0);
    let clip = painter.clip_rect().shrink(3.0);
    let origin = egui::pos2(
        requested.x.clamp(
            clip.left(),
            (clip.right() - background_size.x).max(clip.left()),
        ),
        requested.y.clamp(
            clip.top(),
            (clip.bottom() - background_size.y).max(clip.top()),
        ),
    );
    let background = Rect::from_min_size(origin, background_size);
    painter.rect_filled(background, 3.0, palette.bg_elevated.gamma_multiply(0.96));
    painter.rect_stroke(
        background,
        3.0,
        Stroke::new(1.0, palette.border_strong),
        egui::StrokeKind::Inside,
    );
    painter.galley(origin + egui::vec2(4.0, 2.5), galley, palette.text);
}

/// Copy shown beside the live acquisition marker. Net names are included only
/// when an authored label proves them; unknown nets are never guessed from IDs
/// or a potentially stale generated-net cache.
fn wire_snap_feedback_copy(view: &PreviewView<'_>, result: &SnapResult) -> Option<String> {
    if !result.show_indicator {
        return None;
    }
    let target = result.target.as_ref()?;
    let target_copy = match &target.target_type {
        SnapTargetType::Terminal {
            component_id,
            terminal_name,
        } => {
            let component = view
                .design
                .document
                .components
                .iter()
                .find(|component| component.id == *component_id);
            let owner = component
                .filter(|component| !component.name.trim().is_empty())
                .map(|component| component.name.as_str())
                .map(str::to_owned)
                .unwrap_or_else(|| format!("component #{component_id}"));
            format!("Pin {owner}.{terminal_name}")
        }
        SnapTargetType::Junction => "Junction".to_owned(),
        SnapTargetType::WireEndpoint { wire_id, is_start } => format!(
            "Wire #{wire_id} {} endpoint",
            if *is_start { "start" } else { "end" }
        ),
        SnapTargetType::WireSegment {
            wire_id,
            segment_index,
        } => format!("Wire #{wire_id} segment {}", segment_index + 1),
        SnapTargetType::Grid => return None,
    };

    Some(match net_name_at_snap_target(view, target.position) {
        Some(net_name) => format!("{target_copy} | net {net_name}"),
        None => target_copy,
    })
}

fn net_name_at_snap_target<'a>(view: &PreviewView<'a>, target: Point) -> Option<&'a str> {
    view.design
        .document
        .net_labels
        .iter()
        .find(|label| {
            view.design.object_is_visible(label.id)
                && label.pos == target
                && !label.name.trim().is_empty()
        })
        .map(|label| label.name.as_str())
        .or_else(|| {
            view.design
                .document
                .wires
                .iter()
                .filter(|wire| {
                    view.design.object_is_visible(wire.id) && wire.contains_point(target)
                })
                .find_map(|wire| {
                    view.design
                        .document
                        .net_labels
                        .iter()
                        .find(|label| {
                            view.design.object_is_visible(label.id)
                                && !label.name.trim().is_empty()
                                && wire.contains_point(label.pos)
                        })
                        .map(|label| label.name.as_str())
                })
        })
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::session::EditorSession;
    use crate::view::design_view::DesignView;
    use rspice_design::schematic::{
        component::Component, component_type::ComponentType, document::SchematicDocument,
        net_label::NetLabel, wire::Wire,
    };

    fn preview<'a>(document: &'a SchematicDocument, editor: &'a EditorSession) -> PreviewView<'a> {
        PreviewView {
            design: DesignView {
                document,
                canvas_cache: None,
                sheet_catalog: None,
                review_markers: Default::default(),
            },
            editor,
            can_edit: true,
        }
    }
    #[test]
    fn wire_preview_attaches_on_the_grid_along_the_conductor_and_exactly_in_free_mode() {
        let mut document = SchematicDocument::default();
        let mut editor = EditorSession::default();
        document
            .wires
            .push(Wire::segment(5, Point::new(0, 10), Point::new(20, 10)));
        let symbol_context = SchematicSymbolContext::default();
        let viewport = Viewport {
            offset: egui::Pos2::ZERO,
            zoom: 2.0,
            bounds: egui::Rect::from_min_size(egui::Pos2::ZERO, egui::Vec2::splat(400.0)),
        };
        let pointer = viewport.schematic_to_screen(Point::new(7, 10)) + egui::vec2(0.0, 2.0);

        assert_eq!(document.grid_size, 10);
        let result = resolve_wire_preview_snap(
            &preview(&document, &editor),
            &symbol_context,
            &viewport,
            pointer,
        )
        .expect("representable conductor acquisition");
        assert_eq!(
            result.snapped_position,
            Point::new(10, 10),
            "grid mode quantizes the attachment along the conductor"
        );
        assert_eq!(
            result.target_type(),
            Some(&SnapTargetType::WireSegment {
                wire_id: 5,
                segment_index: 0,
            })
        );
        assert!(result.show_indicator);

        editor.snap_engine.snap_to_grid = false;
        let free = resolve_wire_preview_snap(
            &preview(&document, &editor),
            &symbol_context,
            &viewport,
            pointer,
        )
        .expect("representable conductor acquisition");
        assert_eq!(
            free.snapped_position,
            Point::new(7, 10),
            "Free mode keeps the exact visual attachment"
        );
    }

    #[test]
    fn wire_snap_feedback_uses_retained_net_name_and_never_invents_one() {
        let mut document = SchematicDocument::default();
        let editor = EditorSession::default();
        let component = Component::new(7, ComponentType::Resistor, Point::new(20, 20))
            .with_name_value("R7", "1k");
        let terminal_position = component.terminal_positions()[0].1;
        document.components.push(component);
        document
            .net_labels
            .push(NetLabel::new(1, terminal_position, "VOUT"));
        let terminal =
            editor
                .snap_engine
                .find_snap_target(terminal_position, &document.components, &[], &[]);
        let expected = format!(
            "Pin R7.{} | net VOUT",
            terminal.terminal_name().expect("terminal target")
        );
        assert_eq!(
            wire_snap_feedback_copy(&preview(&document, &editor), &terminal).as_deref(),
            Some(expected.as_str())
        );

        let wire = Wire::segment(41, Point::new(60, 20), Point::new(80, 20));
        let unknown_wire = editor.snap_engine.find_snap_target(
            Point::new(80, 20),
            &[],
            std::slice::from_ref(&wire),
            &[],
        );
        let copy = wire_snap_feedback_copy(&preview(&document, &editor), &unknown_wire)
            .expect("target copy");
        assert_eq!(copy, "Wire #41 end endpoint");
        assert!(!copy.contains("net"));
    }
}

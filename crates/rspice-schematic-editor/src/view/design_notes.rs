//! Exact canvas geometry and paint for durable non-electrical design notes.

use std::sync::Arc;

use egui::{Color32, Context, Galley, Painter, Pos2, Rect, Stroke, vec2};

use rspice_design::schematic::design_note::{DesignNote, DesignNoteKind, DesignNoteRenderContext};
use rspice_design_model::Point;
use rspice_ui_kit::theme::{self, FontWeight};

use super::viewport::Viewport;

const FONT_WORLD_SIZE: f32 = 12.0;
const LINE_ADVANCE_WORLD: f32 = 15.0;
const OFFSET_X_WORLD: f32 = 6.0;
const OFFSET_Y_WORLD: f32 = 3.0;
const GLYPH_WIDTH_WORLD: f32 = 7.2;
const POINTER_SLOP: f32 = 3.0;
const MIN_VISIBLE_FONT_SIZE: f32 = 4.0;

#[derive(Clone)]
struct DesignNoteScreenLayout {
    lines: Vec<(Arc<Galley>, Pos2)>,
    bounds: Rect,
    anchor: Pos2,
}

fn screen_layout(
    ctx: &Context,
    viewport: &Viewport,
    note: &DesignNote,
    text: &str,
    color: Color32,
) -> Option<DesignNoteScreenLayout> {
    let font_size = (((FONT_WORLD_SIZE * viewport.zoom) * 4.0).round() * 0.25).max(1.0);
    if font_size < MIN_VISIBLE_FONT_SIZE {
        return None;
    }
    let anchor = viewport.schematic_to_screen(note.pos);
    let text_origin = anchor
        + vec2(
            OFFSET_X_WORLD * viewport.zoom,
            OFFSET_Y_WORLD * viewport.zoom,
        );
    let mut lines = Vec::new();
    let mut bounds = Rect::from_min_size(text_origin, egui::Vec2::ZERO);
    for (index, source) in text.split('\n').enumerate() {
        let source = if source.is_empty() { " " } else { source };
        let galley = ctx.fonts_mut(|fonts| {
            fonts.layout_no_wrap(
                source.to_owned(),
                theme::mono(font_size, FontWeight::Regular),
                color,
            )
        });
        let position = text_origin + vec2(0.0, index as f32 * LINE_ADVANCE_WORLD * viewport.zoom);
        let rect = Rect::from_min_size(position, galley.size());
        bounds = if lines.is_empty() {
            rect
        } else {
            bounds.union(rect)
        };
        lines.push((galley, position));
    }
    Some(DesignNoteScreenLayout {
        lines,
        bounds: bounds.expand(POINTER_SLOP),
        anchor,
    })
}

fn note_color(kind: DesignNoteKind, selected: bool) -> Color32 {
    let palette = rspice_ui_kit::tokens::active_palette();
    if selected {
        return palette.accent;
    }
    match kind {
        DesignNoteKind::PlainText => palette.text_dim,
        DesignNoteKind::PropertyDisplay => palette.net_label,
        DesignNoteKind::RequirementLink => palette.accent,
        DesignNoteKind::ReviewNote => palette.warn,
    }
}

pub fn draw_design_note(
    painter: &Painter,
    viewport: &Viewport,
    note: &DesignNote,
    context: &DesignNoteRenderContext<'_>,
    selected: bool,
    hovered: bool,
) {
    let palette = rspice_ui_kit::tokens::active_palette();
    let color = note_color(note.kind, selected);
    let text = note.rendered_text(context);
    let Some(layout) = screen_layout(painter.ctx(), viewport, note, &text, color) else {
        return;
    };

    if selected {
        painter.rect_filled(layout.bounds, 2.0, palette.accent_dim);
        painter.rect_stroke(
            layout.bounds,
            2.0,
            Stroke::new(1.0, palette.accent),
            egui::StrokeKind::Inside,
        );
    } else if hovered {
        painter.rect_filled(layout.bounds, 2.0, palette.bg_hover.gamma_multiply(0.72));
    }

    let marker_radius = (2.0 * viewport.zoom).clamp(1.75, 3.5);
    match note.kind {
        DesignNoteKind::PlainText => {
            painter.circle_filled(layout.anchor, marker_radius, color);
        }
        DesignNoteKind::PropertyDisplay => {
            painter.rect_stroke(
                Rect::from_center_size(layout.anchor, egui::Vec2::splat(marker_radius * 2.0)),
                0.0,
                Stroke::new(1.0, color),
                egui::StrokeKind::Inside,
            );
        }
        DesignNoteKind::RequirementLink => {
            painter.circle_stroke(layout.anchor, marker_radius, Stroke::new(1.0, color));
        }
        DesignNoteKind::ReviewNote => {
            let size = marker_radius * 1.3;
            painter.line_segment(
                [
                    layout.anchor + vec2(-size, -size),
                    layout.anchor + vec2(size, size),
                ],
                Stroke::new(1.0, color),
            );
            painter.line_segment(
                [
                    layout.anchor + vec2(size, -size),
                    layout.anchor + vec2(-size, size),
                ],
                Stroke::new(1.0, color),
            );
        }
    }
    for (galley, position) in layout.lines {
        painter.galley(position, galley, color);
    }
    if note.kind == DesignNoteKind::RequirementLink {
        painter.line_segment(
            [layout.bounds.left_bottom(), layout.bounds.right_bottom()],
            Stroke::new(1.0, color),
        );
    }
}

pub fn design_note_at(
    ctx: &Context,
    viewport: &Viewport,
    notes: &[DesignNote],
    context: &DesignNoteRenderContext<'_>,
    pointer: Pos2,
) -> Option<u64> {
    notes.iter().rev().find_map(|note| {
        let text = note.rendered_text(context);
        screen_layout(ctx, viewport, note, &text, note_color(note.kind, false))
            .filter(|layout| {
                layout.bounds.contains(pointer)
                    || Rect::from_center_size(layout.anchor, egui::Vec2::splat(9.0))
                        .contains(pointer)
            })
            .map(|_| note.id)
    })
}

/// Intrinsic world bounds for culling, zoom-to-fit, and marquee selection.
pub fn world_bounds(note: &DesignNote, rendered_text: &str) -> (Point, Point) {
    let lines: Vec<&str> = rendered_text.split('\n').collect();
    let longest = lines
        .iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(1) as f32;
    let width = (longest.max(1.0) * GLYPH_WIDTH_WORLD).ceil() as i32;
    let height = ((lines.len().max(1) as f32) * LINE_ADVANCE_WORLD).ceil() as i32;
    (
        note.pos,
        Point::new(
            note.pos
                .x
                .saturating_add(OFFSET_X_WORLD.ceil() as i32 + width),
            note.pos
                .y
                .saturating_add(OFFSET_Y_WORLD.ceil() as i32 + height),
        ),
    )
}

pub fn conservative_world_bounds(note: &DesignNote) -> (Point, Point) {
    let source = note
        .text
        .replace("${view}", &"W".repeat(64))
        .replace("${component_count}", "00000000")
        .replace("${conductor_count}", "00000000");
    let source = if note.kind == DesignNoteKind::RequirementLink {
        format!("REQ · {source}")
    } else {
        source
    };
    world_bounds(note, &source)
}

#[cfg(test)]
fn hit_bounds(
    ctx: &Context,
    viewport: &Viewport,
    note: &DesignNote,
    context: &DesignNoteRenderContext<'_>,
) -> Option<Rect> {
    let text = note.rendered_text(context);
    screen_layout(ctx, viewport, note, &text, note_color(note.kind, false))
        .map(|layout| layout.bounds)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn initialized_context() -> Context {
        let ctx = Context::default();
        rspice_ui_kit::Theme::default().apply(&ctx);
        let _ = ctx.run_ui(egui::RawInput::default(), |_| {});
        ctx
    }

    #[test]
    fn multiline_shaped_bounds_are_the_pointer_hit_authority() {
        let ctx = initialized_context();
        let viewport = Viewport {
            offset: Pos2::ZERO,
            zoom: 2.0,
            bounds: Rect::from_min_size(Pos2::ZERO, egui::Vec2::splat(500.0)),
        };
        let context = DesignNoteRenderContext {
            view_path: "work/top/schematic",
            component_count: 0,
            conductor_count: 0,
        };
        let note = DesignNote::new(
            31,
            Point::new(50, 40),
            DesignNoteKind::PlainText,
            "Bias network\nKeep clear",
        )
        .unwrap();
        let notes = vec![note.clone()];
        let bounds = hit_bounds(&ctx, &viewport, &note, &context).expect("visible note");

        assert_eq!(
            design_note_at(&ctx, &viewport, &notes, &context, bounds.center()),
            Some(31)
        );
        assert_eq!(
            design_note_at(
                &ctx,
                &viewport,
                &notes,
                &context,
                bounds.right_bottom() + egui::Vec2::splat(0.1)
            ),
            None
        );
    }

    #[test]
    fn reverse_paint_order_wins_for_overlapping_notes() {
        let ctx = initialized_context();
        let viewport = Viewport {
            offset: Pos2::ZERO,
            zoom: 1.0,
            bounds: Rect::from_min_size(Pos2::ZERO, egui::Vec2::splat(500.0)),
        };
        let context = DesignNoteRenderContext {
            view_path: "work/top/schematic",
            component_count: 0,
            conductor_count: 0,
        };
        let notes = vec![
            DesignNote::new(1, Point::new(20, 20), DesignNoteKind::PlainText, "same").unwrap(),
            DesignNote::new(2, Point::new(20, 20), DesignNoteKind::ReviewNote, "same").unwrap(),
        ];
        let bounds = hit_bounds(&ctx, &viewport, &notes[1], &context).unwrap();
        assert_eq!(
            design_note_at(&ctx, &viewport, &notes, &context, bounds.center()),
            Some(2)
        );
    }
}

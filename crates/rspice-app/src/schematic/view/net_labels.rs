//! Net-label canvas geometry, painting, and exact pointer hit testing.
//!
//! The mockup presents net names as compact amber monospace text offset just
//! above their electrical attachment point.  One layout function owns both
//! paint and hit geometry so selection never depends on an approximate
//! character-count box.
//!
//! An off-sheet connector is the same object with a direction tab drawn after
//! its name. The tab reads the way the schematic's port glyph does: the arrow
//! points at the label when signal arrives on this sheet and away from it when
//! signal leaves for another. Because the tab is part of the one decoration
//! rect, selection, hover and marquee bounds follow it with no second path.

use std::sync::Arc;

use egui::{Color32, Context, Galley, Painter, Pos2, Rect, Stroke, vec2};

use crate::state::{CrossSheetPortDirection, NetLabel, Point};
use crate::ui::theme::{self, FontWeight};

use super::viewport::Viewport;

const LABEL_FONT_WORLD_SIZE: f32 = 11.0;
const LABEL_OFFSET_X_WORLD: f32 = 7.0;
const LABEL_OFFSET_Y_WORLD: f32 = -5.0;
const LABEL_LINE_HEIGHT_WORLD: f32 = 13.0;
const LABEL_GLYPH_WIDTH_WORLD: f32 = 6.7;
const POINTER_SLOP: f32 = 2.5;
const MIN_VISIBLE_FONT_SIZE: f32 = 4.0;
const PREVIEW_ALPHA: f32 = 0.55;
const INVALID_LABEL_PLACEHOLDER: &str = "<unnamed>";
/// Gap between the shaped name and the connector tab, in world units.
const CONNECTOR_TAB_GAP_WORLD: f32 = 3.0;
/// Width of the connector tab, in world units. The height follows the name so
/// the pair reads as one object at every zoom.
const CONNECTOR_TAB_WIDTH_WORLD: f32 = 12.0;

#[derive(Clone)]
struct NetLabelScreenLayout {
    galley: Arc<Galley>,
    attachment: Pos2,
    text_rect: Rect,
    /// Present only for an off-sheet connector, and always inside
    /// `decoration_rect`.
    connector_tab: Option<(Rect, CrossSheetPortDirection)>,
    decoration_rect: Rect,
    attachment_hit_rect: Rect,
}

fn quantized_font_size(zoom: f32) -> f32 {
    (((LABEL_FONT_WORLD_SIZE * zoom) * 4.0).round() * 0.25).max(1.0)
}

fn screen_layout(
    ctx: &Context,
    viewport: &Viewport,
    label: &NetLabel,
    color: Color32,
) -> Option<NetLabelScreenLayout> {
    let font_size = quantized_font_size(viewport.zoom);
    if font_size < MIN_VISIBLE_FONT_SIZE {
        return None;
    }
    let display_name = if label.name.trim().is_empty() {
        INVALID_LABEL_PLACEHOLDER
    } else {
        &label.name
    };
    let galley = ctx.fonts_mut(|fonts| {
        fonts.layout_no_wrap(
            display_name.to_owned(),
            theme::mono(font_size, FontWeight::Medium),
            color,
        )
    });
    let anchor = viewport.schematic_to_screen(label.pos);
    let text_min = anchor
        + vec2(
            LABEL_OFFSET_X_WORLD * viewport.zoom,
            LABEL_OFFSET_Y_WORLD * viewport.zoom - galley.size().y,
        );
    let text_rect = Rect::from_min_size(text_min, galley.size());
    let attachment_radius = (2.25 * viewport.zoom).clamp(2.0, 4.0);
    let attachment_rect =
        Rect::from_center_size(anchor, egui::Vec2::splat(attachment_radius * 2.0));
    let connector_tab = label.kind.off_sheet_direction().map(|direction| {
        let rect = Rect::from_min_size(
            text_rect.right_top() + vec2(CONNECTOR_TAB_GAP_WORLD * viewport.zoom, 0.0),
            egui::Vec2::new(
                (CONNECTOR_TAB_WIDTH_WORLD * viewport.zoom).max(1.0),
                text_rect.height(),
            ),
        );
        (rect, direction)
    });
    let painted = match connector_tab {
        Some((tab, _)) => text_rect.union(tab),
        None => text_rect,
    };
    Some(NetLabelScreenLayout {
        galley,
        attachment: anchor,
        text_rect,
        connector_tab,
        decoration_rect: painted.expand(POINTER_SLOP),
        attachment_hit_rect: attachment_rect.expand(POINTER_SLOP),
    })
}

/// Paint the direction tab of an off-sheet connector.
///
/// The marks are the schematic port glyph's, scaled to the tab: an arrow whose
/// head sits at the end the signal travels toward, both heads when the net is
/// bidirectional, and a barred stem for a supply.
fn paint_connector_tab(
    painter: &Painter,
    rect: Rect,
    direction: CrossSheetPortDirection,
    color: Color32,
) {
    let stroke = Stroke::new((rect.height() * 0.085).clamp(0.8, 1.6), color);
    painter.rect_stroke(rect, 2.0, stroke, egui::StrokeKind::Inside);

    let center = rect.center();
    let reach = rect.width() * 0.28;
    let head = rect.width() * 0.16;
    let rise = rect.height() * 0.20;
    let left = Pos2::new(center.x - reach, center.y);
    let right = Pos2::new(center.x + reach, center.y);

    if matches!(direction, CrossSheetPortDirection::Supply) {
        painter.line_segment(
            [
                Pos2::new(left.x, center.y - rise),
                Pos2::new(right.x, center.y - rise),
            ],
            stroke,
        );
        painter.line_segment(
            [
                Pos2::new(center.x, center.y - rise),
                Pos2::new(center.x, center.y + rise),
            ],
            stroke,
        );
        return;
    }

    painter.line_segment([left, right], stroke);
    if matches!(
        direction,
        CrossSheetPortDirection::Input | CrossSheetPortDirection::InOut
    ) {
        painter.line_segment([left, Pos2::new(left.x + head, center.y - rise)], stroke);
        painter.line_segment([left, Pos2::new(left.x + head, center.y + rise)], stroke);
    }
    if matches!(
        direction,
        CrossSheetPortDirection::Output | CrossSheetPortDirection::InOut
    ) {
        painter.line_segment([right, Pos2::new(right.x - head, center.y - rise)], stroke);
        painter.line_segment([right, Pos2::new(right.x - head, center.y + rise)], stroke);
    }
}

pub(super) fn draw_net_label(
    painter: &Painter,
    viewport: &Viewport,
    label: &NetLabel,
    selected: bool,
    hovered: bool,
    preview: bool,
) {
    let palette = crate::ui::tokens::active_palette();
    let text_color = if label.name.trim().is_empty() {
        palette.err
    } else if preview {
        palette.net_label.gamma_multiply(PREVIEW_ALPHA)
    } else if selected {
        palette.accent
    } else {
        palette.net_label
    };
    let Some(layout) = screen_layout(painter.ctx(), viewport, label, text_color) else {
        return;
    };

    if selected {
        painter.rect_filled(layout.decoration_rect, 2.0, palette.accent_dim);
        painter.rect_stroke(
            layout.decoration_rect,
            2.0,
            Stroke::new(1.0, palette.accent),
            egui::StrokeKind::Inside,
        );
    } else if hovered && !preview {
        painter.rect_filled(
            layout.decoration_rect,
            2.0,
            palette.bg_hover.gamma_multiply(0.72),
        );
    }

    let attachment_radius = (2.25 * viewport.zoom).clamp(2.0, 4.0);
    painter.add(egui::Shape::convex_polygon(
        vec![
            layout.attachment + vec2(0.0, -attachment_radius),
            layout.attachment + vec2(attachment_radius, 0.0),
            layout.attachment + vec2(0.0, attachment_radius),
            layout.attachment + vec2(-attachment_radius, 0.0),
        ],
        text_color,
        Stroke::NONE,
    ));
    painter.galley(layout.text_rect.min, layout.galley, text_color);
    if let Some((tab, direction)) = layout.connector_tab {
        paint_connector_tab(painter, tab, direction, text_color);
    }
}

/// Return the topmost label whose exact shaped-text bounds contain the pointer.
/// Reverse order matches paint order when labels overlap.
pub(super) fn net_label_at(
    ctx: &Context,
    viewport: &Viewport,
    labels: &[NetLabel],
    pointer: Pos2,
) -> Option<u64> {
    let color = crate::ui::tokens::active_palette().net_label;
    labels.iter().rev().find_map(|label| {
        screen_layout(ctx, viewport, label, color)
            .filter(|layout| {
                layout.decoration_rect.contains(pointer)
                    || layout.attachment_hit_rect.contains(pointer)
            })
            .map(|_| label.id)
    })
}

#[cfg(test)]
pub(super) fn hit_bounds(ctx: &Context, viewport: &Viewport, label: &NetLabel) -> Option<Rect> {
    screen_layout(
        ctx,
        viewport,
        label,
        crate::ui::tokens::active_palette().net_label,
    )
    .map(|layout| layout.decoration_rect.union(layout.attachment_hit_rect))
}

/// Intrinsic world-space bounds used by fit-to-content and marquee selection.
/// Pointer hit testing deliberately uses the exact shaped screen galley above.
pub(super) fn world_bounds(label: &NetLabel) -> (Point, Point) {
    let display_name = if label.name.trim().is_empty() {
        INVALID_LABEL_PLACEHOLDER
    } else {
        &label.name
    };
    let glyphs = display_name.chars().count() as f32;
    let text_min_x = label.pos.x as f32 + LABEL_OFFSET_X_WORLD;
    let min_x = label.pos.x as f32;
    let tab_width = if label.kind.off_sheet_direction().is_some() {
        CONNECTOR_TAB_GAP_WORLD + CONNECTOR_TAB_WIDTH_WORLD
    } else {
        0.0
    };
    let max_x = text_min_x + glyphs * LABEL_GLYPH_WIDTH_WORLD + tab_width;
    let text_max_y = label.pos.y as f32 + LABEL_OFFSET_Y_WORLD;
    let max_y = label.pos.y as f32;
    let min_y = text_max_y - LABEL_LINE_HEIGHT_WORLD;
    (
        Point::new(min_x.floor() as i32, min_y.floor() as i32),
        Point::new(max_x.ceil() as i32, max_y.ceil() as i32),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::{Id, LayerId, Order, Vec2};

    fn viewport() -> Viewport {
        Viewport {
            offset: Pos2::ZERO,
            zoom: 2.0,
            bounds: Rect::from_min_size(Pos2::ZERO, Vec2::splat(400.0)),
        }
    }

    fn initialized_context() -> Context {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let _ = ctx.run_ui(egui::RawInput::default(), |_| {});
        ctx
    }

    #[test]
    fn shaped_text_bounds_are_the_single_pointer_hit_authority() {
        let ctx = initialized_context();
        let viewport = viewport();
        let label = NetLabel::new(41, Point::new(40, 30), "afe_out");
        let color = crate::ui::tokens::active_palette().net_label;
        let layout = screen_layout(&ctx, &viewport, &label, color).expect("visible label");
        let hit_bounds = layout.decoration_rect.union(layout.attachment_hit_rect);

        assert_eq!(
            net_label_at(
                &ctx,
                &viewport,
                std::slice::from_ref(&label),
                layout.text_rect.center()
            ),
            Some(41)
        );
        assert_eq!(
            net_label_at(
                &ctx,
                &viewport,
                std::slice::from_ref(&label),
                hit_bounds.right_bottom() + Vec2::splat(0.1),
            ),
            None
        );
    }

    #[test]
    fn selected_and_preview_treatments_do_not_change_label_geometry() {
        let ctx = initialized_context();
        let viewport = viewport();
        let label = NetLabel::new(7, Point::new(20, 20), "sensor_p");
        let painter = Painter::new(
            ctx.clone(),
            LayerId::new(Order::Foreground, Id::new("net-label-treatment-test")),
            viewport.bounds,
        );
        let before = hit_bounds(&ctx, &viewport, &label).expect("visible label");

        draw_net_label(&painter, &viewport, &label, true, false, false);
        draw_net_label(&painter, &viewport, &label, false, false, true);

        let after = hit_bounds(&ctx, &viewport, &label).expect("visible label");
        assert_eq!(before, after);
    }

    #[test]
    fn intrinsic_bounds_include_the_mockup_text_offset() {
        let label = NetLabel::new(3, Point::new(100, 80), "OUT");
        let (min, max) = world_bounds(&label);
        assert_eq!(min.x, label.pos.x);
        assert!(max.x > min.x);
        assert_eq!(max.y, label.pos.y);
        assert!(min.y < max.y);
    }

    #[test]
    fn invalid_empty_label_remains_visible_and_pointer_addressable() {
        let ctx = initialized_context();
        let viewport = viewport();
        let label = NetLabel::new(9, Point::new(30, 30), "   ");
        let bounds = hit_bounds(&ctx, &viewport, &label).expect("invalid label stays visible");

        assert_eq!(
            net_label_at(
                &ctx,
                &viewport,
                std::slice::from_ref(&label),
                bounds.center()
            ),
            Some(label.id)
        );
        let (min, max) = world_bounds(&label);
        assert!(max.x - min.x > 1);
    }

    #[test]
    fn the_connector_tab_is_inside_the_one_decoration_rect_that_hit_testing_reads() {
        let ctx = initialized_context();
        let viewport = viewport();
        let connector = NetLabel::off_sheet(
            5,
            Point::new(50, 40),
            "BIAS",
            crate::state::CrossSheetPortDirection::Output,
        );
        let color = crate::ui::tokens::active_palette().net_label;
        let layout = screen_layout(&ctx, &viewport, &connector, color).expect("visible connector");
        let (tab, _) = layout
            .connector_tab
            .expect("an off-sheet connector has a tab");

        assert!(tab.left() > layout.text_rect.right());
        assert!(layout.decoration_rect.contains_rect(tab));
        assert_eq!(
            net_label_at(
                &ctx,
                &viewport,
                std::slice::from_ref(&connector),
                tab.center()
            ),
            Some(5),
            "the painted tab must be pointer-addressable"
        );
        assert_eq!(
            net_label_at(
                &ctx,
                &viewport,
                std::slice::from_ref(&connector),
                Pos2::new(layout.decoration_rect.right() + 1.0, tab.center().y),
            ),
            None,
            "one pixel outside the decoration rect must miss"
        );

        let local = NetLabel::new(5, Point::new(50, 40), "BIAS");
        let plain = screen_layout(&ctx, &viewport, &local, color).expect("visible label");
        assert!(plain.connector_tab.is_none());
        assert!(plain.decoration_rect.width() < layout.decoration_rect.width());
        assert_eq!(plain.text_rect, layout.text_rect);
    }

    #[test]
    fn intrinsic_bounds_reserve_the_connector_tab_so_marquee_and_fit_agree_with_paint() {
        let local = NetLabel::new(1, Point::new(0, 0), "BIAS");
        let connector = NetLabel::off_sheet(
            1,
            Point::new(0, 0),
            "BIAS",
            crate::state::CrossSheetPortDirection::Input,
        );
        let (local_min, local_max) = world_bounds(&local);
        let (connector_min, connector_max) = world_bounds(&connector);

        assert_eq!(local_min, connector_min);
        assert!(connector_max.x > local_max.x);
    }

    #[test]
    fn every_direction_paints_ink_inside_the_tab_and_leaves_the_gap_clear() {
        for direction in crate::state::NetLabelKind::DIRECTIONS {
            let viewport = Viewport {
                offset: Pos2::new(30.0, 90.0),
                zoom: 3.0,
                bounds: Rect::from_min_size(Pos2::ZERO, Vec2::new(240.0, 120.0)),
            };
            let connector = NetLabel::off_sheet(2, Point::new(0, 0), "BIAS", direction);
            let mut tab = Rect::NOTHING;
            let canvas = crate::ui::raster::render(Vec2::new(240.0, 120.0), |ui, background| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE.fill(background))
                    .show(ui, |ui| {
                        let painter = ui.painter();
                        let color = crate::ui::tokens::active_palette().net_label;
                        tab = screen_layout(painter.ctx(), &viewport, &connector, color)
                            .expect("visible connector")
                            .connector_tab
                            .expect("an off-sheet connector has a tab")
                            .0;
                        draw_net_label(painter, &viewport, &connector, false, false, false);
                    });
            });

            let inside: Vec<egui::Color32> = canvas.pixels_in(tab).collect();
            assert!(!inside.is_empty(), "{direction:?}: the tab is off canvas");
            assert!(
                inside.iter().any(|pixel| *pixel != canvas.background()),
                "{direction:?}: the connector tab painted nothing"
            );

            let gap = Rect::from_min_max(
                Pos2::new(
                    tab.left() - CONNECTOR_TAB_GAP_WORLD * viewport.zoom + 3.0,
                    tab.top(),
                ),
                Pos2::new(tab.left() - 1.0, tab.bottom()),
            );
            let between: Vec<egui::Color32> = canvas.pixels_in(gap).collect();
            assert!(!between.is_empty(), "{direction:?}: the gap is off canvas");
            assert!(
                between.iter().all(|pixel| *pixel == canvas.background()),
                "{direction:?}: the tab must stand apart from the name"
            );
        }
    }
}

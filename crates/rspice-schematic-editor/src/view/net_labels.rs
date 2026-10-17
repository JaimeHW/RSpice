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

use rspice_design::schematic::net_label::NetLabel;
use rspice_design_model::{Point, design_management::CrossSheetPortDirection};
use rspice_ui_kit::theme::{self, FontWeight};

use super::drawing::ConductorInk;
use super::lod::{self, TextClass, TextLod};
use super::viewport::Viewport;

const LABEL_FONT_WORLD_SIZE: f32 = 11.0;
const LABEL_OFFSET_X_WORLD: f32 = 7.0;
const LABEL_OFFSET_Y_WORLD: f32 = -5.0;
const LABEL_LINE_HEIGHT_WORLD: f32 = 13.0;
const LABEL_GLYPH_WIDTH_WORLD: f32 = 6.7;
const POINTER_SLOP: f32 = 2.5;
const PREVIEW_ALPHA: f32 = 0.55;
const INVALID_LABEL_PLACEHOLDER: &str = "<unnamed>";
/// Gap between the shaped name and the connector tab, in world units.
const CONNECTOR_TAB_GAP_WORLD: f32 = 3.0;
/// Width of the connector tab, in world units. The height follows the name so
/// the pair reads as one object at every zoom.
const CONNECTOR_TAB_WIDTH_WORLD: f32 = 12.0;

/// What stands where the name is drawn.
#[derive(Clone)]
enum LabelText {
    /// The shaped name.
    Shaped(Arc<Galley>),
    /// The name is below the legibility floor: a bar of this rect stands in
    /// for it, so the reader still sees that a name is attached here.
    Greeked(Rect),
}

#[derive(Clone)]
struct NetLabelScreenLayout {
    text: LabelText,
    attachment: Pos2,
    /// Half the diagonal of the attachment diamond, in points.
    attachment_radius: f32,
    /// The text box: the shaped galley's rect, or where it would be.
    text_rect: Rect,
    /// Present only for an off-sheet connector, and always inside
    /// `decoration_rect`.
    connector_tab: Option<(Rect, CrossSheetPortDirection)>,
    decoration_rect: Rect,
    attachment_hit_rect: Rect,
}

fn display_name(label: &NetLabel) -> &str {
    if label.name.trim().is_empty() {
        INVALID_LABEL_PLACEHOLDER
    } else {
        &label.name
    }
}

/// The one layout paint and hit testing share. A name below the legibility
/// floor greeks rather than disappearing, so a label is drawn, hoverable,
/// selectable and draggable at every zoom.
fn screen_layout(
    ctx: &Context,
    viewport: &Viewport,
    label: &NetLabel,
    color: Color32,
) -> NetLabelScreenLayout {
    let ppp = ctx.pixels_per_point();
    let zoom = viewport.zoom;
    let anchor = viewport.schematic_to_screen(label.pos);
    let name = display_name(label);
    let (text, text_rect) =
        match lod::text_lod(LABEL_FONT_WORLD_SIZE, zoom, ppp, TextClass::Electrical) {
            TextLod::Draw(font_size) => {
                let galley = ctx.fonts_mut(|fonts| {
                    fonts.layout_no_wrap(
                        name.to_owned(),
                        theme::mono(font_size, FontWeight::Medium),
                        color,
                    )
                });
                let text_min = anchor
                    + vec2(
                        LABEL_OFFSET_X_WORLD * zoom,
                        LABEL_OFFSET_Y_WORLD * zoom - galley.size().y,
                    );
                let rect = Rect::from_min_size(text_min, galley.size());
                (LabelText::Shaped(galley), rect)
            }
            greek @ (TextLod::Greek(_) | TextLod::Hide) => {
                // An electrical name greeks rather than hides; `Hide` cannot
                // come back for it, and would take the thinnest bar if it did.
                let bar_height = match greek {
                    TextLod::Greek(bar) => bar.height_pt,
                    TextLod::Draw(_) | TextLod::Hide => 1.0 / ppp.max(f32::EPSILON),
                };
                let box_height = LABEL_LINE_HEIGHT_WORLD * zoom;
                let text_min = anchor
                    + vec2(
                        LABEL_OFFSET_X_WORLD * zoom,
                        LABEL_OFFSET_Y_WORLD * zoom - box_height,
                    );
                let width = lod::greek_bar_width_pt(
                    name.chars().count(),
                    LABEL_GLYPH_WIDTH_WORLD,
                    zoom,
                    ppp,
                );
                let rect = Rect::from_min_size(text_min, vec2(width, box_height));
                let bar = Rect::from_min_size(
                    Pos2::new(rect.left(), rect.center().y - bar_height * 0.5),
                    vec2(width, bar_height),
                );
                (LabelText::Greeked(bar), rect)
            }
        };
    let attachment_radius = lod::diamond_half_pt(zoom);
    let attachment_rect =
        Rect::from_center_size(anchor, egui::Vec2::splat(attachment_radius * 2.0));
    let greeked = matches!(text, LabelText::Greeked(_));
    let connector_tab = label.kind.off_sheet_direction().map(|direction| {
        let width = if greeked {
            lod::greek_tab_width_pt(CONNECTOR_TAB_WIDTH_WORLD, zoom, ppp)
        } else {
            (CONNECTOR_TAB_WIDTH_WORLD * zoom).max(1.0)
        };
        let rect = Rect::from_min_size(
            text_rect.right_top() + vec2(CONNECTOR_TAB_GAP_WORLD * zoom, 0.0),
            egui::Vec2::new(width, text_rect.height()),
        );
        (rect, direction)
    });
    let painted = match (&text, connector_tab) {
        (LabelText::Greeked(bar), Some((tab, _))) => bar.union(tab),
        (LabelText::Greeked(bar), None) => *bar,
        (LabelText::Shaped(_), Some((tab, _))) => text_rect.union(tab),
        (LabelText::Shaped(_), None) => text_rect,
    };
    NetLabelScreenLayout {
        text,
        attachment: anchor,
        attachment_radius,
        text_rect,
        connector_tab,
        decoration_rect: painted.expand(POINTER_SLOP),
        attachment_hit_rect: attachment_rect.expand(POINTER_SLOP),
    }
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

pub fn draw_net_label(
    painter: &Painter,
    viewport: &Viewport,
    label: &NetLabel,
    selected: bool,
    hovered: bool,
    preview: bool,
) {
    draw_net_label_attached(
        painter,
        viewport,
        label,
        NetLabelPaint {
            selected,
            hovered,
            preview,
            attachment: None,
        },
    );
}

/// How a net label is painted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetLabelPaint {
    pub selected: bool,
    pub hovered: bool,
    /// A ghost of a label being placed or moved.
    pub preview: bool,
    /// The ink of the conductor the label attaches to, when it is drawn
    /// other than plain: the attachment diamond takes it, as a junction
    /// would. The text keeps its own colour.
    pub attachment: Option<ConductorInk>,
}

/// The fill behind a selected label's text, over the canvas.
const SELECTION_FILL_ALPHA: f32 = 0.12;

pub fn draw_net_label_attached(
    painter: &Painter,
    viewport: &Viewport,
    label: &NetLabel,
    paint: NetLabelPaint,
) {
    let NetLabelPaint {
        selected,
        hovered,
        preview,
        attachment,
    } = paint;
    let palette = rspice_ui_kit::tokens::active_palette();
    // Text never takes the selection colour: a selected label keeps its
    // text colour and gets the selection rect.
    let text_color = if label.name.trim().is_empty() {
        palette.err
    } else if preview {
        palette.net_label.gamma_multiply(PREVIEW_ALPHA)
    } else {
        palette.net_label
    };
    let layout = screen_layout(painter.ctx(), viewport, label, text_color);

    if selected {
        painter.rect_filled(
            layout.decoration_rect,
            2.0,
            palette.canvas_select.gamma_multiply(SELECTION_FILL_ALPHA),
        );
        painter.rect_stroke(
            layout.decoration_rect,
            2.0,
            Stroke::new(1.0, palette.canvas_select),
            egui::StrokeKind::Inside,
        );
    } else if hovered && !preview {
        painter.rect_filled(
            layout.decoration_rect,
            2.0,
            palette.bg_hover.gamma_multiply(0.72),
        );
    }

    let attachment_radius = layout.attachment_radius;
    let diamond = match attachment {
        Some(ink) if !preview => ink.colour(&palette),
        Some(_) | None => text_color,
    };
    painter.add(egui::Shape::convex_polygon(
        vec![
            layout.attachment + vec2(0.0, -attachment_radius),
            layout.attachment + vec2(attachment_radius, 0.0),
            layout.attachment + vec2(0.0, attachment_radius),
            layout.attachment + vec2(-attachment_radius, 0.0),
        ],
        diamond,
        Stroke::NONE,
    ));
    match layout.text {
        LabelText::Shaped(galley) => {
            painter.galley(layout.text_rect.min, galley, text_color);
            if let Some((tab, direction)) = layout.connector_tab {
                paint_connector_tab(painter, tab, direction, text_color);
            }
        }
        LabelText::Greeked(bar) => {
            let ink = text_color.gamma_multiply(lod::GREEK_ALPHA);
            painter.rect_filled(bar, 0.0, ink);
            if let Some((tab, _)) = layout.connector_tab {
                // The tab keeps its outline; its arrow is below legibility.
                painter.rect_stroke(
                    tab,
                    0.0,
                    Stroke::new(1.0 / painter.pixels_per_point(), ink),
                    egui::StrokeKind::Inside,
                );
            }
        }
    }
}

/// Return the topmost label whose exact shaped-text bounds contain the pointer.
/// Reverse order matches paint order when labels overlap.
pub fn net_label_at(
    ctx: &Context,
    viewport: &Viewport,
    labels: &[NetLabel],
    pointer: Pos2,
) -> Option<u64> {
    let color = rspice_ui_kit::tokens::active_palette().net_label;
    labels.iter().rev().find_map(|label| {
        let layout = screen_layout(ctx, viewport, label, color);
        (layout.decoration_rect.contains(pointer) || layout.attachment_hit_rect.contains(pointer))
            .then_some(label.id)
    })
}

#[cfg(any(test, feature = "test-support"))]
pub fn hit_bounds(ctx: &Context, viewport: &Viewport, label: &NetLabel) -> Option<Rect> {
    let layout = screen_layout(
        ctx,
        viewport,
        label,
        rspice_ui_kit::tokens::active_palette().net_label,
    );
    Some(layout.decoration_rect.union(layout.attachment_hit_rect))
}

/// Intrinsic world-space bounds used by fit-to-content and marquee selection.
/// Pointer hit testing deliberately uses the exact shaped screen galley above.
pub fn world_bounds(label: &NetLabel) -> (Point, Point) {
    let glyphs = display_name(label).chars().count() as f32;
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
        rspice_ui_kit::Theme::default().apply(&ctx);
        let _ = ctx.run_ui(egui::RawInput::default(), |_| {});
        ctx
    }

    #[test]
    fn shaped_text_bounds_are_the_single_pointer_hit_authority() {
        let ctx = initialized_context();
        let viewport = viewport();
        let label = NetLabel::new(41, Point::new(40, 30), "afe_out");
        let color = rspice_ui_kit::tokens::active_palette().net_label;
        let layout = screen_layout(&ctx, &viewport, &label, color);
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

    /// Zoomed out to a quarter, a net name is below the legibility floor. It
    /// greeks into a bar, keeps its attachment diamond at the floor size, and
    /// stays under the pointer: bar, tab and diamond are all hit.
    #[test]
    fn a_greeked_label_keeps_its_diamond_and_stays_pickable() {
        let viewport = Viewport {
            offset: Pos2::new(40.0, 60.0),
            zoom: 0.25,
            bounds: Rect::from_min_size(Pos2::ZERO, Vec2::new(160.0, 100.0)),
        };
        let label = NetLabel::off_sheet(
            11,
            Point::new(0, 0),
            "VOUT",
            rspice_design_model::design_management::CrossSheetPortDirection::Output,
        );
        let mut layout = None;
        let canvas = rspice_ui_kit::raster::render(Vec2::new(160.0, 100.0), |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| {
                    let painter = ui.painter();
                    let color = rspice_ui_kit::tokens::active_palette().net_label;
                    layout = Some(screen_layout(painter.ctx(), &viewport, &label, color));
                    draw_net_label(painter, &viewport, &label, false, false, false);
                });
        });
        let layout = layout.expect("laid out");
        let LabelText::Greeked(bar) = layout.text else {
            panic!("a quarter-zoom net name greeks");
        };
        assert_eq!(layout.attachment_radius, 3.0, "the diamond is at its floor");
        let (tab, _) = layout.connector_tab.expect("connector tab");
        assert!(tab.width() >= 4.0, "the tab is at least 4 dpx wide");

        let ink = |rect: Rect| {
            canvas
                .pixels_in(rect)
                .filter(|pixel| *pixel != canvas.background())
                .count()
        };
        let diamond = Rect::from_center_size(layout.attachment, Vec2::splat(4.0));
        assert!(ink(diamond) >= 4, "the diamond is painted");
        assert!(ink(bar.expand(0.5)) >= 3, "the bar is painted");

        let ctx = initialized_context();
        let labels = std::slice::from_ref(&label);
        for (what, point) in [
            ("bar", bar.center()),
            ("tab", tab.center()),
            ("diamond", layout.attachment),
        ] {
            assert_eq!(
                net_label_at(&ctx, &viewport, labels, point),
                Some(11),
                "the greeked {what} is pickable"
            );
        }
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
            rspice_design_model::design_management::CrossSheetPortDirection::Output,
        );
        let color = rspice_ui_kit::tokens::active_palette().net_label;
        let layout = screen_layout(&ctx, &viewport, &connector, color);
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
        let plain = screen_layout(&ctx, &viewport, &local, color);
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
            rspice_design_model::design_management::CrossSheetPortDirection::Input,
        );
        let (local_min, local_max) = world_bounds(&local);
        let (connector_min, connector_max) = world_bounds(&connector);

        assert_eq!(local_min, connector_min);
        assert!(connector_max.x > local_max.x);
    }

    #[test]
    fn every_direction_paints_ink_inside_the_tab_and_leaves_the_gap_clear() {
        for direction in rspice_design::schematic::net_label::NetLabelKind::DIRECTIONS {
            let viewport = Viewport {
                offset: Pos2::new(30.0, 90.0),
                zoom: 3.0,
                bounds: Rect::from_min_size(Pos2::ZERO, Vec2::new(240.0, 120.0)),
            };
            let connector = NetLabel::off_sheet(2, Point::new(0, 0), "BIAS", direction);
            let mut tab = Rect::NOTHING;
            let canvas =
                rspice_ui_kit::raster::render(Vec2::new(240.0, 120.0), |ui, background| {
                    egui::CentralPanel::default()
                        .frame(egui::Frame::NONE.fill(background))
                        .show(ui, |ui| {
                            let painter = ui.painter();
                            let color = rspice_ui_kit::tokens::active_palette().net_label;
                            tab = screen_layout(painter.ctx(), &viewport, &connector, color)
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

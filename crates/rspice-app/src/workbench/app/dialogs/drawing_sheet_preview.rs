//! Shared physical drawing-sheet preview for supporting setup surfaces.

use egui::{Align2, Color32, FontId, Pos2, Rect, Sense, Shape, Stroke, Ui, pos2, vec2};

use crate::schematic::view::drawing_sheet::{
    DrawingSheetPrintablePreview, DrawingSheetPrintablePreviewKind, drawing_sheet_printable_preview,
};
use crate::state::{
    DrawingSheetFrameLayer, DrawingSheetFrameMark, DrawingSheetRect, SchematicSheetFormat,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::app_state::AppState;

/// Immutable schematic geometry captured before a modal borrows its own state.
///
/// The sheet-format manager draws it on the open sheet's card, so that card
/// shows the drawing the format would frame. Surfaces comparing formats
/// rather than the active drawing omit it.
#[derive(Debug, Clone, Default)]
pub(super) struct DrawingSheetPreviewContent {
    objects: Vec<DrawingSheetPrintablePreview>,
    /// The active sheet's number, as its title block letters it, which
    /// decides whether the sheet carries the same-revision note.
    page_label: String,
}

impl DrawingSheetPreviewContent {
    pub(super) fn from_state(state: &AppState) -> Self {
        Self {
            objects: drawing_sheet_printable_preview(state),
            page_label: crate::schematic::view::drawing_sheet::resolve_active_drawing_sheet(state)
                .page_label,
        }
    }

    pub(super) fn is_empty(&self) -> bool {
        self.objects.is_empty()
    }

    fn outside_drawing_count(&self, drawing: DrawingSheetRect) -> usize {
        let in_drawing = |x: f64, y: f64| {
            let (origin_x, origin_y) = crate::schematic::view::drawing_sheet::DRAWING_SHEET_ORIGIN;
            let units_per_mm = crate::schematic::view::drawing_sheet::DRAWING_SHEET_UNITS_PER_MM;
            let x_um = ((x - origin_x) / units_per_mm * 1_000.0).round();
            let y_um = ((y - origin_y) / units_per_mm * 1_000.0).round();
            x_um >= drawing.x_um as f64
                && y_um >= drawing.y_um as f64
                && x_um <= (drawing.x_um as f64 + drawing.width_um as f64)
                && y_um <= (drawing.y_um as f64 + drawing.height_um as f64)
        };
        self.objects
            .iter()
            .filter(|object| {
                !in_drawing(object.bounds.min_x, object.bounds.min_y)
                    || !in_drawing(object.bounds.max_x, object.bounds.max_y)
            })
            .count()
    }
}

pub(super) fn drawing_sheet_preview(
    ui: &mut Ui,
    format: &SchematicSheetFormat,
    height: f32,
    label: &str,
) {
    drawing_sheet_preview_impl(ui, format, height, label, None, None);
}

/// The desk (backdrop) color the preview paints behind the paper, exposed so
/// a fused surface can extend the same desk across its whole preview region.
pub(super) fn sheet_desk_color(ctx: &egui::Context) -> Color32 {
    SheetPreviewPalette::for_text_color(Tokens::get(ctx).color.text).desk
}

/// One sheet drawn at the scale its whole set shares.
///
/// The ruler is the bounding page of the set — its widest page by its tallest —
/// so every sheet in a strip is measured against the same rule and an A3 among
/// A4s still reads as the larger page. Ruling on the longest edge alone squared
/// the ruler instead, which drew a landscape page at barely half the width of
/// the card that carried it and left the rest as desk.
pub(super) fn drawing_sheet_preview_at_common_scale(
    ui: &mut Ui,
    format: &SchematicSheetFormat,
    height: f32,
    label: &str,
    content: Option<&DrawingSheetPreviewContent>,
    common_extent_um: (u64, u64),
) {
    drawing_sheet_preview_impl(ui, format, height, label, content, Some(common_extent_um));
}

/// The share of the ruler kept clear around the drawn page, matching the 6 %
/// pad a single preview leaves around its own paper.
pub(super) const DRAWING_SHEET_COMMON_RULER_PAD: f32 = 1.12;

fn drawing_sheet_preview_impl(
    ui: &mut Ui,
    format: &SchematicSheetFormat,
    height: f32,
    label: &str,
    content: Option<&DrawingSheetPreviewContent>,
    common_extent_um: Option<(u64, u64)>,
) {
    let t = Tokens::get(ui.ctx());
    let palette = SheetPreviewPalette::for_text_color(t.color.text);
    let width = ui.available_width().max(1.0);
    let total_height = height.max(72.0);
    let (stage, response) = ui.allocate_exact_size(vec2(width, total_height), Sense::hover());

    let caption_height = if label.trim().is_empty() { 0.0 } else { 20.0 };
    // The desk covers the whole stage, caption track included, so the preview
    // reads as one surface: a desk that stopped short of the caption left a
    // band of dialog background under the sheet and set the format line on it.
    ui.painter().rect_filled(stage, 0.0, palette.desk);
    let canvas = Rect::from_min_max(
        stage.min,
        pos2(
            stage.right(),
            (stage.bottom() - caption_height).max(stage.top()),
        ),
    );

    let Ok(geometry) = format.geometry() else {
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Image,
                true,
                format!("{label}. Invalid physical sheet."),
            )
        });
        ui.painter().text(
            canvas.center(),
            Align2::CENTER_CENTER,
            "Invalid physical sheet",
            theme::sans(tokens::FS_0, FontWeight::Medium),
            t.color.err,
        );
        paint_caption(ui, stage, caption_height, label, t.color.text_dim);
        return;
    };
    let accessible_description = preview_accessible_description(format, &geometry, content, label);
    response.widget_info(|| {
        egui::WidgetInfo::labeled(
            egui::WidgetType::Image,
            true,
            accessible_description.clone(),
        )
    });

    let paper = geometry.paper;
    let pad_um = paper.width_um.max(paper.height_um) as f32 * 0.06;
    let min_x_um = -pad_um;
    let min_y_um = -pad_um;
    let max_x_um = paper.width_um as f32 + pad_um;
    let max_y_um = paper.height_um as f32 + pad_um;
    let extent_um = vec2(max_x_um - min_x_um, max_y_um - min_y_um);
    let available = canvas.shrink2(vec2(10.0, 9.0));
    let scale = common_extent_um.map_or_else(
        || {
            (available.width() / extent_um.x)
                .min(available.height() / extent_um.y)
                .max(f32::EPSILON)
        },
        |(ruler_width_um, ruler_height_um)| {
            let ruler_width = ruler_width_um.max(1) as f32 * DRAWING_SHEET_COMMON_RULER_PAD;
            let ruler_height = ruler_height_um.max(1) as f32 * DRAWING_SHEET_COMMON_RULER_PAD;
            (available.width() / ruler_width)
                .min(available.height() / ruler_height)
                .max(f32::EPSILON)
        },
    );
    let extent_screen = Rect::from_center_size(available.center(), extent_um * scale);
    let map = |x_um: f32, y_um: f32| -> Pos2 {
        pos2(
            extent_screen.left() + (x_um - min_x_um) * scale,
            extent_screen.top() + (y_um - min_y_um) * scale,
        )
    };
    let map_rect = |rect: DrawingSheetRect| -> Rect {
        Rect::from_min_max(
            map(rect.x_um as f32, rect.y_um as f32),
            map(
                rect.x_um as f32 + rect.width_um as f32,
                rect.y_um as f32 + rect.height_um as f32,
            ),
        )
    };

    let paper_rect = map_rect(paper);
    ui.painter().rect_filled(paper_rect, 0.0, palette.paper);
    ui.painter().rect_stroke(
        paper_rect,
        0.0,
        Stroke::new(1.25, palette.edge),
        egui::StrokeKind::Inside,
    );

    let drawing_rect = map_rect(geometry.drawing_area);
    paint_frame(ui, format, &geometry, &map, scale, palette);

    if let Some(content) = content {
        paint_content_silhouette(ui, content, drawing_rect, &map, palette);
    }

    if let Some(title_block) = geometry.title_block {
        paint_title_block(ui, map_rect(title_block), format, &geometry, palette);
    }
    // The same-revision note the canvas and the plot letter, as the value
    // lines are drawn: a stroke on its line, over its length.
    if let Some(note) =
        content.and_then(|content| format.same_revision_note(&geometry, &content.page_label))
    {
        let width_um =
            crate::state::drawing_sheet_mono_advance_um(note.lettering_um) * note.text.len() as u64;
        let (x, y) = (note.x_um as f32, note.seat_um as f32);
        ui.painter().line_segment(
            [map(x, y), map(x + width_um as f32, y)],
            Stroke::new(0.65, palette.block_ink),
        );
    }

    paint_caption(ui, stage, caption_height, label, t.color.text_dim);
}

fn preview_accessible_description(
    format: &SchematicSheetFormat,
    geometry: &crate::state::DrawingSheetGeometry,
    content: Option<&DrawingSheetPreviewContent>,
    label: &str,
) -> String {
    let unit = format.display_unit;
    let mut description = format!(
        "{label}. {} drawing sheet; paper {}; drawing area {}.",
        format.authored_size.label(),
        unit.format_size_um(geometry.paper.width_um, geometry.paper.height_um),
        unit.format_size_um(
            geometry.drawing_area.width_um,
            geometry.drawing_area.height_um
        ),
    );
    if let Some(zones) = geometry.zones {
        description.push_str(&format!(
            " Zone grid {} columns by {} rows.",
            zones.columns, zones.rows
        ));
    } else {
        description.push_str(" No zone grid.");
    }
    if let Some(content) = content {
        let outside = content.outside_drawing_count(geometry.drawing_area);
        if outside == 0 {
            description.push_str(" All previewed content is inside the drawing area.");
        } else {
            description.push_str(&format!(
                " {outside} previewed content {} outside the drawing area.",
                if outside == 1 { "item is" } else { "items are" }
            ));
        }
    }
    description
}

fn paint_content_silhouette(
    ui: &Ui,
    content: &DrawingSheetPreviewContent,
    drawing_rect: Rect,
    map: &impl Fn(f32, f32) -> Pos2,
    palette: SheetPreviewPalette,
) {
    let map_world = |x: f64, y: f64| {
        let (origin_x, origin_y) = crate::schematic::view::drawing_sheet::DRAWING_SHEET_ORIGIN;
        let units_per_mm = crate::schematic::view::drawing_sheet::DRAWING_SHEET_UNITS_PER_MM;
        let sheet_x_mm = (x - origin_x) / units_per_mm;
        let sheet_y_mm = (y - origin_y) / units_per_mm;
        map((sheet_x_mm * 1_000.0) as f32, (sheet_y_mm * 1_000.0) as f32)
    };
    for object in &content.objects {
        let bounds = Rect::from_two_pos(
            map_world(object.bounds.min_x, object.bounds.min_y),
            map_world(object.bounds.max_x, object.bounds.max_y),
        );
        let outside = !drawing_rect.contains(bounds.min) || !drawing_rect.contains(bounds.max);
        let color = if outside {
            palette.warning
        } else {
            palette.zone_ink
        };
        let stroke = Stroke::new(if outside { 1.2 } else { 0.8 }, color);
        match object.kind {
            DrawingSheetPrintablePreviewKind::Component => {
                ui.painter()
                    .rect_stroke(bounds, 0.0, stroke, egui::StrokeKind::Inside);
            }
            DrawingSheetPrintablePreviewKind::Conductor => {
                for segment in object.path.windows(2) {
                    ui.painter().line_segment(
                        [
                            map_world(f64::from(segment[0].x), f64::from(segment[0].y)),
                            map_world(f64::from(segment[1].x), f64::from(segment[1].y)),
                        ],
                        stroke,
                    );
                }
            }
            DrawingSheetPrintablePreviewKind::Junction => {
                ui.painter()
                    .circle_filled(bounds.center(), bounds.width().max(2.0) * 0.5, color);
            }
            DrawingSheetPrintablePreviewKind::NetLabel => {
                ui.painter().circle_filled(bounds.left_bottom(), 1.4, color);
                let baseline_y = (bounds.bottom() - 1.0).max(bounds.top());
                ui.painter().line_segment(
                    [
                        pos2(bounds.left() + 2.0, baseline_y),
                        pos2(bounds.right(), baseline_y),
                    ],
                    stroke,
                );
            }
            DrawingSheetPrintablePreviewKind::DesignNote => {
                paint_dashed_rect(ui, bounds, stroke, 2.0, 2.0);
                for fraction in [0.35, 0.65] {
                    let y = egui::lerp(bounds.top()..=bounds.bottom(), fraction);
                    ui.painter().line_segment(
                        [pos2(bounds.left() + 1.5, y), pos2(bounds.right() - 1.5, y)],
                        Stroke::new(stroke.width * 0.75, color),
                    );
                }
            }
            DrawingSheetPrintablePreviewKind::DocumentationShape => {
                ui.painter()
                    .rect_stroke(bounds, 0.0, stroke, egui::StrokeKind::Inside);
                ui.painter()
                    .line_segment([bounds.left_top(), bounds.right_bottom()], stroke);
            }
        }
    }
}

#[derive(Clone, Copy)]
struct SheetPreviewPalette {
    desk: Color32,
    paper: Color32,
    edge: Color32,
    border: Color32,
    zone: Color32,
    zone_ink: Color32,
    block: Color32,
    block_ink: Color32,
    warning: Color32,
}

impl SheetPreviewPalette {
    fn for_text_color(text: Color32) -> Self {
        if text.r() > 160 {
            Self {
                desk: Color32::from_rgb(4, 8, 11),
                paper: Color32::from_rgb(11, 18, 23),
                edge: Color32::from_rgb(77, 89, 96),
                border: Color32::from_rgb(111, 127, 137),
                zone: Color32::from_rgb(65, 78, 86),
                zone_ink: Color32::from_rgb(142, 153, 159),
                block: Color32::from_rgb(16, 24, 29),
                block_ink: Color32::from_rgb(197, 205, 210),
                warning: Color32::from_rgb(224, 145, 48),
            }
        } else {
            Self {
                desk: Color32::from_rgb(213, 218, 221),
                paper: Color32::WHITE,
                edge: Color32::from_rgb(153, 162, 167),
                border: Color32::from_rgb(77, 84, 89),
                zone: Color32::from_rgb(141, 149, 154),
                zone_ink: Color32::from_rgb(69, 75, 79),
                block: Color32::from_rgb(243, 245, 246),
                block_ink: Color32::from_rgb(31, 37, 40),
                warning: Color32::from_rgb(138, 79, 0),
            }
        }
    }
}

/// Paint the sheet's frame from the artwork the canvas and the plot draw,
/// scaled to the preview. Lines keep their physical proportions down to a
/// hairline, and references are lettered only where the band can carry
/// legible text.
fn paint_frame(
    ui: &Ui,
    format: &SchematicSheetFormat,
    geometry: &crate::state::DrawingSheetGeometry,
    map: &impl Fn(f32, f32) -> Pos2,
    scale: f32,
    palette: SheetPreviewPalette,
) {
    let at = |(x_um, y_um): (i64, i64)| map(x_um as f32, y_um as f32);
    let width = |width_um: u64| (width_um as f32 * scale).max(0.55);
    let lettered = geometry.border_band_um as f32 * scale >= 4.5;
    for item in &format.frame_artwork(geometry).items {
        let ink = match item.layer {
            DrawingSheetFrameLayer::Border | DrawingSheetFrameLayer::CentringMarks => {
                palette.border
            }
            DrawingSheetFrameLayer::ZoneLabels | DrawingSheetFrameLayer::SizeDesignation => {
                palette.zone_ink
            }
            DrawingSheetFrameLayer::Zones => palette.zone,
            // Trimming marks belong to output that is cut to size.
            DrawingSheetFrameLayer::TrimMarks => continue,
        };
        match &item.mark {
            DrawingSheetFrameMark::Line { from, to, width_um } => {
                ui.painter()
                    .line_segment([at(*from), at(*to)], Stroke::new(width(*width_um), ink));
            }
            DrawingSheetFrameMark::Rect { rect, width_um } => {
                let (right, bottom) = (
                    rect.x_um + rect.width_um as i64,
                    rect.y_um + rect.height_um as i64,
                );
                ui.painter().rect_stroke(
                    Rect::from_min_max(at((rect.x_um, rect.y_um)), at((right, bottom))),
                    0.0,
                    Stroke::new(width(*width_um), ink),
                    egui::StrokeKind::Middle,
                );
            }
            DrawingSheetFrameMark::FilledRect { .. } => {}
            DrawingSheetFrameMark::FilledPolygon { points } => {
                ui.painter().add(Shape::convex_polygon(
                    points.iter().copied().map(at).collect(),
                    ink,
                    Stroke::NONE,
                ));
            }
            DrawingSheetFrameMark::Text {
                centre,
                text,
                lettering_um,
            } => {
                if lettered {
                    let font_size = (*lettering_um as f32 * scale / 0.698).clamp(5.0, 8.0);
                    ui.painter().text(
                        at(*centre),
                        Align2::CENTER_CENTER,
                        text,
                        FontId::monospace(font_size),
                        ink,
                    );
                }
            }
        }
    }
}

fn paint_title_block(
    ui: &Ui,
    title: Rect,
    format: &SchematicSheetFormat,
    geometry: &crate::state::DrawingSheetGeometry,
    palette: SheetPreviewPalette,
) {
    // The cells the canvas and the plot draw, scaled to the preview.
    let Some(layout) = crate::state::drawing_sheet_title_layout(format, geometry) else {
        return;
    };
    let template = geometry.effective_title_block_template;
    let line_stroke = Stroke::new(0.55, palette.zone);
    let authored = title;
    let outline = [
        authored.left_top(),
        authored.right_top(),
        authored.right_bottom(),
        authored.left_bottom(),
    ];
    // The plot's widths in proportion: ISO 7200's 0.7 mm outline and 0.35 mm
    // rules draw at 1 and 0.55 points, and the ASME form's thick and thin
    // lines in the same ratio.
    let outline_width =
        layout.outline_um as f32 / crate::state::DRAWING_SHEET_TITLE_OUTLINE_UM as f32;
    let rule_stroke = |width_um: u64| {
        Stroke::new(
            0.55 * width_um as f32 / crate::state::DRAWING_SHEET_TITLE_RULE_UM as f32,
            palette.zone,
        )
    };
    ui.painter().add(Shape::convex_polygon(
        outline.to_vec(),
        palette.block,
        Stroke::new(outline_width, palette.border),
    ));
    let x_at =
        |x_um: u64| authored.left() + authored.width() * x_um as f32 / layout.width_um as f32;
    let y_at =
        |y_um: u64| authored.top() + authored.height() * y_um as f32 / layout.height_um as f32;
    let mut field_grid = authored;
    field_grid.min.x = x_at(layout.grid_x_um);
    if let Some(logo) = format.title_block_logo(template) {
        ui.painter().line_segment(
            [
                pos2(field_grid.left(), authored.top()),
                pos2(field_grid.left(), authored.bottom()),
            ],
            line_stroke,
        );
        let reserved = Rect::from_min_max(
            authored.left_top(),
            pos2(field_grid.left(), authored.bottom()),
        );
        let inset = (reserved.width().min(reserved.height()) * 0.08).max(0.5);
        let content = reserved.shrink(inset);
        if content.width() > 0.0 && content.height() > 0.0 {
            let basis = f32::from(crate::state::DRAWING_SHEET_MANAGED_LOGO_COORDINATE_BASIS);
            for primitive in logo.primitives() {
                let points = primitive
                    .points()
                    .iter()
                    .map(|point| {
                        pos2(
                            egui::lerp(
                                content.left()..=content.right(),
                                f32::from(point.x()) / basis,
                            ),
                            egui::lerp(
                                content.top()..=content.bottom(),
                                f32::from(point.y()) / basis,
                            ),
                        )
                    })
                    .collect::<Vec<_>>();
                if primitive.filled() {
                    ui.painter().add(Shape::convex_polygon(
                        points,
                        palette.block_ink,
                        line_stroke,
                    ));
                } else if primitive.closed() {
                    ui.painter().add(Shape::closed_line(points, line_stroke));
                } else {
                    ui.painter().add(Shape::line(points, line_stroke));
                }
            }
        }
    }

    for rule in &layout.rules {
        ui.painter().line_segment(
            [
                pos2(x_at(rule.from_x_um), y_at(rule.from_y_um)),
                pos2(x_at(rule.to_x_um), y_at(rule.to_y_um)),
            ],
            rule_stroke(rule.width_um),
        );
    }
    // At preview scale, field text would become an illegible grey smear.
    // A short stroke on each value's line still communicates the populated
    // form without claiming values the preview cannot resolve.
    for (value_index, (cell, value)) in layout
        .cells
        .iter()
        .flat_map(|cell| cell.values.iter().map(move |value| (cell, value)))
        .enumerate()
    {
        let (cell_left, cell_right) = (x_at(cell.x_um), x_at(cell.x_um + cell.width_um));
        let cell_width = cell_right - cell_left;
        let cell_height = y_at(cell.y_um + cell.height_um) - y_at(cell.y_um);
        if cell_width >= 7.0 && cell_height >= 4.0 {
            let inset = (cell_width * 0.12).clamp(1.0, 4.0);
            let width_ratio = 0.42 + (value_index % 3) as f32 * 0.12;
            let end_x = (cell_left + cell_width * width_ratio).min(cell_right - inset);
            let y = y_at(value.seat_um);
            ui.painter().line_segment(
                [pos2(cell_left + inset, y), pos2(end_x, y)],
                Stroke::new(0.65, palette.block_ink),
            );
        }
    }
}

fn paint_dashed_rect(ui: &Ui, rect: Rect, stroke: Stroke, dash: f32, gap: f32) {
    paint_dashed_line(ui, rect.left_top(), rect.right_top(), stroke, dash, gap);
    paint_dashed_line(ui, rect.right_top(), rect.right_bottom(), stroke, dash, gap);
    paint_dashed_line(
        ui,
        rect.right_bottom(),
        rect.left_bottom(),
        stroke,
        dash,
        gap,
    );
    paint_dashed_line(ui, rect.left_bottom(), rect.left_top(), stroke, dash, gap);
}

fn paint_dashed_line(ui: &Ui, start: Pos2, end: Pos2, stroke: Stroke, dash: f32, gap: f32) {
    let delta = end - start;
    let length = delta.length();
    if length <= f32::EPSILON {
        return;
    }
    let direction = delta / length;
    let mut offset = 0.0;
    while offset < length {
        let segment_end = (offset + dash).min(length);
        ui.painter().line_segment(
            [start + direction * offset, start + direction * segment_end],
            stroke,
        );
        offset += dash + gap;
    }
}

fn paint_caption(ui: &Ui, stage: Rect, caption_height: f32, label: &str, color: Color32) {
    if caption_height <= 0.0 {
        return;
    }
    ui.painter().text(
        pos2(stage.center().x, stage.bottom() - caption_height * 0.5),
        Align2::CENTER_CENTER,
        label,
        FontId::monospace(tokens::FS_0),
        color,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        DesignNote, DesignNoteKind, DocumentationShape, DocumentationShapeGeometry, Junction,
        NetLabel, Point, SchematicProbe,
    };

    #[test]
    fn the_preview_letters_the_zones_the_sheet_model_letters() {
        // The preview keeps no alphabet of its own: it letters the
        // references the canvas and the plot letter, from the sheet model.
        let format = SchematicSheetFormat::from_standard(
            crate::state::DrawingSheetStandard::IsoA0,
            crate::state::SchematicPageOrientation::Landscape,
        );
        let geometry = format.geometry().unwrap();
        let expected = format
            .frame_artwork(&geometry)
            .layer(DrawingSheetFrameLayer::ZoneLabels)
            .filter_map(|mark| match mark {
                DrawingSheetFrameMark::Text { text, .. } => Some(text.clone()),
                _ => None,
            })
            .collect::<std::collections::BTreeSet<_>>();
        assert!(expected.contains("Q") && expected.contains("R") && expected.contains("24"));
        // Wide enough that the 5 mm band carries legible references.
        let input = egui::RawInput {
            screen_rect: Some(Rect::from_min_size(Pos2::ZERO, vec2(6_000.0, 4_500.0))),
            ..Default::default()
        };
        let output = egui::Context::default().run_ui(input, |ui| {
            drawing_sheet_preview(ui, &format, 4_000.0, "");
        });
        let lettered = output
            .shapes
            .iter()
            .filter_map(|clipped| match &clipped.shape {
                Shape::Text(text) => Some(text.galley.text().to_owned()),
                _ => None,
            })
            .collect::<std::collections::BTreeSet<_>>();
        assert!(
            expected.is_subset(&lettered),
            "the preview letters {lettered:?}, not the model's {expected:?}"
        );
    }

    #[test]
    fn preview_covers_every_hardcopy_printable_class_but_not_probe_flags() {
        let mut state = AppState::default();
        state
            .schematic
            .document_mut_for_test()
            .junctions
            .push(Junction::new(51, Point::new(1_100, 100)));
        state
            .schematic
            .document_mut_for_test()
            .net_labels
            .push(NetLabel::new(52, Point::new(1_100, 140), "OUTSIDE"));
        state.schematic.document_mut_for_test().design_notes.push(
            DesignNote::new(
                53,
                Point::new(1_100, 180),
                DesignNoteKind::PlainText,
                "outside note",
            )
            .expect("valid note"),
        );
        state
            .schematic
            .document_mut_for_test()
            .documentation_shapes
            .push(
                DocumentationShape::new(
                    54,
                    DocumentationShapeGeometry::Line {
                        start: Point::new(1_100, 220),
                        end: Point::new(1_140, 240),
                    },
                )
                .expect("valid documentation line"),
            );
        state.schematic.document_mut_for_test().probes.push(
            SchematicProbe::new(
                55,
                Point::new(1_100, 260),
                "V(out)",
                Some("V(out)".to_owned()),
            )
            .expect("valid probe"),
        );

        let content = DrawingSheetPreviewContent::from_state(&state);
        let drawing = SchematicSheetFormat::default()
            .geometry()
            .expect("default sheet geometry")
            .drawing_area;

        assert_eq!(content.objects.len(), 4);
        assert_eq!(content.outside_drawing_count(drawing), 4);
        assert_eq!(
            content
                .objects
                .iter()
                .map(|object| object.kind)
                .collect::<Vec<_>>(),
            vec![
                DrawingSheetPrintablePreviewKind::Junction,
                DrawingSheetPrintablePreviewKind::NetLabel,
                DrawingSheetPrintablePreviewKind::DesignNote,
                DrawingSheetPrintablePreviewKind::DocumentationShape,
            ]
        );
    }
}

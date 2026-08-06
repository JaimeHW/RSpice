//! Shared physical drawing-sheet preview for supporting setup surfaces.

use egui::{Align2, Color32, FontId, Pos2, Rect, Sense, Shape, Stroke, Ui, pos2, vec2};

use crate::schematic::view::drawing_sheet::{
    DrawingSheetPrintablePreview, DrawingSheetPrintablePreviewKind, drawing_sheet_printable_preview,
};
use crate::state::{
    DrawingSheetRect, DrawingSheetTitleBlockRotation, DrawingSheetTitleBlockTemplate,
    DrawingSheetZoneEdges, DrawingSheetZoneGrid, DrawingSheetZoneLabels, SchematicSheetFormat,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::app_state::AppState;

/// Immutable schematic geometry captured before a modal borrows its own state.
///
/// Page Setup uses this snapshot to answer the same physical-page question in
/// its preview and overflow summary. Supporting surfaces omit it when they are
/// comparing formats rather than the active drawing.
#[derive(Debug, Clone, Default)]
pub(super) struct DrawingSheetPreviewContent {
    objects: Vec<DrawingSheetPrintablePreview>,
}

impl DrawingSheetPreviewContent {
    pub(super) fn from_state(state: &AppState) -> Self {
        Self {
            objects: drawing_sheet_printable_preview(state),
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
    drawing_sheet_preview_impl(ui, format, height, label, None, false, None);
}

/// The desk (backdrop) color the preview paints behind the paper, exposed so
/// a fused surface can extend the same desk across its whole preview region.
pub(super) fn sheet_desk_color(ctx: &egui::Context) -> Color32 {
    SheetPreviewPalette::for_text_color(Tokens::get(ctx).color.text).desk
}

pub(super) fn drawing_sheet_preview_with_content(
    ui: &mut Ui,
    format: &SchematicSheetFormat,
    height: f32,
    label: &str,
    content: &DrawingSheetPreviewContent,
    invalid: bool,
) {
    drawing_sheet_preview_impl(ui, format, height, label, Some(content), invalid, None);
}

pub(super) fn drawing_sheet_preview_at_common_scale(
    ui: &mut Ui,
    format: &SchematicSheetFormat,
    height: f32,
    label: &str,
    content: Option<&DrawingSheetPreviewContent>,
    common_edge_um: u64,
) {
    drawing_sheet_preview_impl(
        ui,
        format,
        height,
        label,
        content,
        false,
        Some(common_edge_um),
    );
}

fn drawing_sheet_preview_impl(
    ui: &mut Ui,
    format: &SchematicSheetFormat,
    height: f32,
    label: &str,
    content: Option<&DrawingSheetPreviewContent>,
    invalid: bool,
    common_edge_um: Option<u64>,
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
    let scale = common_edge_um.map_or_else(
        || {
            (available.width() / extent_um.x)
                .min(available.height() / extent_um.y)
                .max(f32::EPSILON)
        },
        |common_edge_um| {
            let ruler_extent_um = common_edge_um.max(1) as f32 * 1.12;
            (available.width().min(available.height()) / ruler_extent_um).max(f32::EPSILON)
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
    if invalid {
        ui.painter().rect_filled(
            paper_rect,
            0.0,
            Color32::from_rgba_unmultiplied(
                palette.warning.r(),
                palette.warning.g(),
                palette.warning.b(),
                10,
            ),
        );
        paint_dashed_rect(ui, paper_rect, Stroke::new(1.4, palette.warning), 6.0, 3.0);
    } else {
        ui.painter().rect_stroke(
            paper_rect,
            0.0,
            Stroke::new(1.25, palette.edge),
            egui::StrokeKind::Inside,
        );
    }

    let printable_rect = map_rect(geometry.printable);
    paint_dashed_rect(
        ui,
        printable_rect,
        Stroke::new(0.8, palette.margin),
        4.0,
        3.0,
    );

    let drawing_rect = map_rect(geometry.drawing_area);
    if geometry.border_band_um > 0 {
        ui.painter().rect_stroke(
            drawing_rect,
            0.0,
            Stroke::new(1.0, palette.border),
            egui::StrokeKind::Inside,
        );
        if let Some(zones) = geometry.zones {
            paint_zone_band(ui, printable_rect, drawing_rect, zones, palette);
        }
    }

    if let Some(content) = content {
        paint_content_silhouette(ui, content, drawing_rect, &map, palette);
    }

    if let Some(title_block) = geometry.title_block {
        paint_title_block(
            ui,
            map_rect(title_block),
            geometry.effective_title_block_template,
            format,
            palette,
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
        "{label}. {} drawing sheet; paper {}; printable area {}; drawing area {}.",
        format.authored_size.label(),
        unit.format_size_um(geometry.paper.width_um, geometry.paper.height_um),
        unit.format_size_um(geometry.printable.width_um, geometry.printable.height_um),
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
    margin: Color32,
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
                margin: Color32::from_rgb(48, 61, 69),
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
                margin: Color32::from_rgb(182, 190, 194),
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

fn paint_zone_band(
    ui: &Ui,
    printable: Rect,
    drawing: Rect,
    zones: DrawingSheetZoneGrid,
    palette: SheetPreviewPalette,
) {
    let columns = usize::from(zones.columns);
    let rows = usize::from(zones.rows);
    if columns == 0 || rows == 0 {
        return;
    }
    let top = matches!(
        zones.edges,
        DrawingSheetZoneEdges::All | DrawingSheetZoneEdges::TopAndLeft
    );
    let bottom = matches!(
        zones.edges,
        DrawingSheetZoneEdges::All | DrawingSheetZoneEdges::BottomAndRight
    );
    let left = matches!(
        zones.edges,
        DrawingSheetZoneEdges::All | DrawingSheetZoneEdges::TopAndLeft
    );
    let right = matches!(
        zones.edges,
        DrawingSheetZoneEdges::All | DrawingSheetZoneEdges::BottomAndRight
    );
    let stroke = Stroke::new(0.75, palette.zone);

    for index in 1..columns {
        let x = egui::lerp(
            drawing.left()..=drawing.right(),
            index as f32 / columns as f32,
        );
        if top {
            ui.painter()
                .line_segment([pos2(x, printable.top()), pos2(x, drawing.top())], stroke);
        }
        if bottom {
            ui.painter().line_segment(
                [pos2(x, drawing.bottom()), pos2(x, printable.bottom())],
                stroke,
            );
        }
    }
    for index in 1..rows {
        let y = egui::lerp(drawing.top()..=drawing.bottom(), index as f32 / rows as f32);
        if left {
            ui.painter()
                .line_segment([pos2(printable.left(), y), pos2(drawing.left(), y)], stroke);
        }
        if right {
            ui.painter().line_segment(
                [pos2(drawing.right(), y), pos2(printable.right(), y)],
                stroke,
            );
        }
    }

    if zones.labels == DrawingSheetZoneLabels::Coordinates {
        return;
    }
    let horizontal_band =
        (drawing.top() - printable.top()).max(printable.bottom() - drawing.bottom());
    let vertical_band =
        (drawing.left() - printable.left()).max(printable.right() - drawing.right());
    let column_step = drawing.width() / columns as f32;
    let row_step = drawing.height() / rows as f32;
    let font_size = horizontal_band
        .max(vertical_band)
        .mul_add(0.8, 0.0)
        .clamp(5.0, 8.0);
    let font = FontId::monospace(font_size);

    if horizontal_band >= 4.5 && column_step >= 7.0 {
        let top_y = (printable.top() + drawing.top()) * 0.5;
        let bottom_y = (drawing.bottom() + printable.bottom()) * 0.5;
        for index in 0..columns {
            let x = drawing.left() + column_step * (index as f32 + 0.5);
            let label = zone_column_label(index, zones.labels);
            if top {
                ui.painter().text(
                    pos2(x, top_y),
                    Align2::CENTER_CENTER,
                    &label,
                    font.clone(),
                    palette.zone_ink,
                );
            }
            if bottom {
                ui.painter().text(
                    pos2(x, bottom_y),
                    Align2::CENTER_CENTER,
                    &label,
                    font.clone(),
                    palette.zone_ink,
                );
            }
        }
    }
    if vertical_band >= 4.5 && row_step >= 7.0 {
        let left_x = (printable.left() + drawing.left()) * 0.5;
        let right_x = (drawing.right() + printable.right()) * 0.5;
        for index in 0..rows {
            let y = drawing.top() + row_step * (index as f32 + 0.5);
            let label = zone_row_label(index, zones.labels);
            if left {
                ui.painter().text(
                    pos2(left_x, y),
                    Align2::CENTER_CENTER,
                    &label,
                    font.clone(),
                    palette.zone_ink,
                );
            }
            if right {
                ui.painter().text(
                    pos2(right_x, y),
                    Align2::CENTER_CENTER,
                    &label,
                    font.clone(),
                    palette.zone_ink,
                );
            }
        }
    }
}

fn zone_column_label(index: usize, labels: DrawingSheetZoneLabels) -> String {
    match labels {
        DrawingSheetZoneLabels::AlphaNumeric => (index + 1).to_string(),
        DrawingSheetZoneLabels::NumericAlpha => alphabetic_zone_label(index),
        DrawingSheetZoneLabels::Coordinates => String::new(),
    }
}

fn zone_row_label(index: usize, labels: DrawingSheetZoneLabels) -> String {
    match labels {
        DrawingSheetZoneLabels::AlphaNumeric => alphabetic_zone_label(index),
        DrawingSheetZoneLabels::NumericAlpha => (index + 1).to_string(),
        DrawingSheetZoneLabels::Coordinates => String::new(),
    }
}

fn alphabetic_zone_label(index: usize) -> String {
    const LETTERS: &[u8] = b"ABCDEFGHJKLMNPRSTUVWXY";
    LETTERS.get(index).map_or_else(
        || (index + 1).to_string(),
        |letter| char::from(*letter).to_string(),
    )
}

fn paint_title_block(
    ui: &Ui,
    title: Rect,
    template: DrawingSheetTitleBlockTemplate,
    format: &SchematicSheetFormat,
    palette: SheetPreviewPalette,
) {
    let visible_fields = format
        .title_block
        .fields
        .values()
        .filter(|field| field.visible)
        .count()
        .max(1);
    let Some(rows) = format.title_block_rows(template) else {
        return;
    };
    let columns = visible_fields.div_ceil(rows).clamp(1, 5);
    let line_stroke = Stroke::new(0.55, palette.zone);
    let angle = match format.title_block.rotation {
        DrawingSheetTitleBlockRotation::Upright => 0.0,
        DrawingSheetTitleBlockRotation::Clockwise90 => std::f32::consts::FRAC_PI_2,
        DrawingSheetTitleBlockRotation::CounterClockwise90 => -std::f32::consts::FRAC_PI_2,
    };
    let authored = if angle == 0.0 {
        title
    } else {
        Rect::from_center_size(title.center(), vec2(title.height(), title.width()))
    };
    let transform = |point: Pos2| {
        let offset = point - authored.center();
        let (sin, cos) = angle.sin_cos();
        authored.center()
            + vec2(
                offset.x * cos - offset.y * sin,
                offset.x * sin + offset.y * cos,
            )
    };
    let outline = [
        transform(authored.left_top()),
        transform(authored.right_top()),
        transform(authored.right_bottom()),
        transform(authored.left_bottom()),
    ];
    ui.painter().add(Shape::convex_polygon(
        outline.to_vec(),
        palette.block,
        Stroke::new(1.0, palette.border),
    ));
    let mut field_grid = authored;
    if let Some(logo) = format.title_block_logo(template)
        && let Some((template_width_um, _)) = format.title_block_dimensions_um(template)
    {
        let reserved_width =
            authored.width() * (logo.reserved_width_um() as f32 / template_width_um as f32);
        field_grid.min.x = (authored.left() + reserved_width).min(authored.right());
        ui.painter().line_segment(
            [
                transform(pos2(field_grid.left(), authored.top())),
                transform(pos2(field_grid.left(), authored.bottom())),
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
                        transform(pos2(
                            egui::lerp(
                                content.left()..=content.right(),
                                f32::from(point.x()) / basis,
                            ),
                            egui::lerp(
                                content.top()..=content.bottom(),
                                f32::from(point.y()) / basis,
                            ),
                        ))
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

    for row in 1..rows {
        let y = egui::lerp(
            field_grid.top()..=field_grid.bottom(),
            row as f32 / rows as f32,
        );
        ui.painter().line_segment(
            [
                transform(pos2(field_grid.left(), y)),
                transform(pos2(field_grid.right(), y)),
            ],
            line_stroke,
        );
    }
    for field_index in 0..visible_fields {
        let row = field_index / columns;
        let column = field_index % columns;
        if column == 0 {
            continue;
        }
        let x = egui::lerp(
            field_grid.left()..=field_grid.right(),
            column as f32 / columns as f32,
        );
        let top = egui::lerp(
            field_grid.top()..=field_grid.bottom(),
            row as f32 / rows as f32,
        );
        let bottom = egui::lerp(
            field_grid.top()..=field_grid.bottom(),
            (row + 1) as f32 / rows as f32,
        );
        ui.painter().line_segment(
            [transform(pos2(x, top)), transform(pos2(x, bottom))],
            line_stroke,
        );
    }

    // At preview scale, field text would become an illegible grey smear.
    // Short ruled silhouettes still communicate the populated title-grid
    // structure without claiming values the preview cannot resolve.
    let cell_width = field_grid.width() / columns as f32;
    let cell_height = field_grid.height() / rows as f32;
    if cell_width >= 7.0 && cell_height >= 4.0 {
        for field_index in 0..visible_fields.min(rows * columns) {
            let row = field_index / columns;
            let column = field_index % columns;
            let cell_left = field_grid.left() + cell_width * column as f32;
            let cell_top = field_grid.top() + cell_height * row as f32;
            let inset = (cell_width * 0.12).clamp(1.0, 4.0);
            let width_ratio = 0.42 + (field_index % 3) as f32 * 0.12;
            let end_x = (cell_left + cell_width * width_ratio).min(cell_left + cell_width - inset);
            let y = cell_top + cell_height * 0.58;
            ui.painter().line_segment(
                [
                    transform(pos2(cell_left + inset, y)),
                    transform(pos2(end_x, y)),
                ],
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
    fn zone_labels_use_the_engineering_alphabet_and_numeric_overflow() {
        assert_eq!(alphabetic_zone_label(0), "A");
        assert_eq!(alphabetic_zone_label(8), "J");
        assert_eq!(alphabetic_zone_label(21), "Y");
        assert_eq!(alphabetic_zone_label(22), "23");
    }

    #[test]
    fn preview_covers_every_hardcopy_printable_class_but_not_probe_flags() {
        let mut state = AppState::default();
        state
            .schematic
            .junctions
            .push(Junction::new(51, Point::new(1_100, 100)));
        state
            .schematic
            .net_labels
            .push(NetLabel::new(52, Point::new(1_100, 140), "OUTSIDE"));
        state.schematic.design_notes.push(
            DesignNote::new(
                53,
                Point::new(1_100, 180),
                DesignNoteKind::PlainText,
                "outside note",
            )
            .expect("valid note"),
        );
        state.schematic.documentation_shapes.push(
            DocumentationShape::new(
                54,
                DocumentationShapeGeometry::Line {
                    start: Point::new(1_100, 220),
                    end: Point::new(1_140, 240),
                },
            )
            .expect("valid documentation line"),
        );
        state.schematic.probes.push(
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

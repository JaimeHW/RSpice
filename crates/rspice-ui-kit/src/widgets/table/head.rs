//! A table's head: the band rows that name a run of columns once, then the
//! row of column heads.
//!
//! Each head and span scrolls with its columns and the frozen ones stay put,
//! so a head always sits over its column. A span is one label centred over
//! the part of its run in view, between hairlines at the run's edges; the
//! frozen columns' band cells are blank. A sortable head answers a click and
//! carries a small mark for the way its column runs.

use egui::{Rect, Sense, Ui, pos2};

use crate::panels::elide_text;
use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};

use super::geometry::Columns;
use super::{Align, CELL_PAD, DataTable, Sort, SortDirection, View, Words, paint_words};

/// The width of a sort mark, and the room it takes beside a head's words.
const SORT_MARK_W: f32 = 7.0;
const SORT_MARK_ROOM: f32 = SORT_MARK_W + 4.0;
/// The width of the strip over a column's right edge that a press drags.
const EDGE_GRIP: f32 = 6.0;

/// Where a table's head goes this frame.
pub(super) struct HeadPlace<'g> {
    /// The band rows and the head row, allocated before the body.
    pub(super) block: Rect,
    pub(super) response: &'g egui::Response,
    /// Height of the head row.
    pub(super) height: f32,
    /// Height of each band row over it.
    pub(super) band_height: f32,
    pub(super) view: View,
    pub(super) geometry: &'g Columns,
    pub(super) sort: Option<Sort>,
    /// The table has state, so a head click can sort it.
    pub(super) sortable: bool,
}

/// What the reader did to a table's head this frame.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct HeadAnswer {
    /// The column whose head was clicked to sort by.
    pub(super) sort: Option<usize>,
    /// The column whose right edge was dragged, and the width it is dragged
    /// to.
    pub(super) resize: Option<(usize, f32)>,
}

impl DataTable<'_> {
    /// Paint the band rows and the head row, and report a head clicked to
    /// sort by and an edge dragged to size its column.
    pub(super) fn head(&self, ui: &mut Ui, place: HeadPlace<'_>) -> HeadAnswer {
        let t = Tokens::get(ui.ctx());
        let HeadPlace {
            block,
            response,
            height,
            band_height,
            view,
            geometry,
            sort,
            sortable,
        } = place;
        response.widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Label,
                ui.is_enabled(),
                format!(
                    "Table columns: {}",
                    self.columns
                        .iter()
                        .map(|column| column.head.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
            )
        });
        for (band, spans) in self.bands.iter().enumerate() {
            let top = block.top() + band as f32 * band_height;
            self.band(ui, band, spans, view, geometry, top..=top + band_height);
        }
        let rect = Rect::from_x_y_ranges(block.x_range(), block.bottom() - height..=block.bottom());
        #[cfg(any(test, feature = "test-support"))]
        crate::probe::probe(ui, "table.head", rect);
        if !ui.is_rect_visible(rect) {
            return HeadAnswer::default();
        }
        ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
        let mut clicked = None;
        let mut edges = Vec::new();
        for index in geometry.in_view(view.offset, view.width) {
            let Some(column) = self.columns.get(index) else {
                continue;
            };
            let (cell, region) = view.cell(geometry, index, rect.y_range());
            let clip = region.intersect(ui.clip_rect());
            #[cfg(any(test, feature = "test-support"))]
            {
                let page = self.page_column(index);
                crate::probe::probe(ui, &format!("table.head.{page}"), cell);
                if self.pinned == Some(page) {
                    crate::probe::probe(ui, "table.pinned", cell);
                }
            }
            let sorts = sortable && column.sortable;
            let direction = sort
                .filter(|sort| sort.column == index)
                .map(|sort| sort.direction);
            if sorts || column.tooltip.is_some() {
                let head = ui.interact(
                    cell.intersect(clip),
                    ui.id().with((self.id, "head", index)),
                    if sorts {
                        Sense::click()
                    } else {
                        Sense::hover()
                    },
                );
                if sorts {
                    head.widget_info(|| {
                        egui::WidgetInfo::labeled(
                            egui::WidgetType::Button,
                            ui.is_enabled(),
                            match direction {
                                Some(SortDirection::Ascending) => {
                                    format!("{}, sorted ascending", column.head)
                                }
                                Some(SortDirection::Descending) => {
                                    format!("{}, sorted descending", column.head)
                                }
                                None => format!("Sort by {}", column.head),
                            },
                        )
                    });
                    if head.hovered() {
                        ui.painter()
                            .with_clip_rect(clip)
                            .rect_filled(cell, 0.0, t.color.bg_hover);
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }
                    if theme::focus_is_visible(ui.ctx(), &head) {
                        theme::paint_focus_ring(ui, &head, cell.intersect(clip));
                    }
                    if head.clicked() {
                        clicked = Some(index);
                    }
                }
                if let Some(tooltip) = &column.tooltip {
                    let _ = head.on_hover_text(tooltip);
                }
            }
            let mut span = self.text_span(index, cell);
            if direction.is_some() {
                match column.align {
                    Align::Left => span.max = (span.max - SORT_MARK_ROOM).max(span.min),
                    Align::Right => span.min = (span.min + SORT_MARK_ROOM).min(span.max),
                }
            }
            let landed = paint_words(
                ui,
                cell,
                clip,
                span,
                Words {
                    text: &column.head,
                    font: theme::sans(tokens::FS_0, FontWeight::Medium),
                    align: column.align,
                    color: t.color.text_dim,
                    problem: None,
                },
            )
            .map(|painted| painted.landed);
            if let Some(direction) = direction {
                let centre_x = match (column.align, landed) {
                    (Align::Left, Some(words)) => {
                        words.right() + SORT_MARK_ROOM - SORT_MARK_W * 0.5
                    }
                    (Align::Right, Some(words)) => {
                        words.left() - SORT_MARK_ROOM + SORT_MARK_W * 0.5
                    }
                    (Align::Left, None) => span.min + SORT_MARK_W * 0.5,
                    (Align::Right, None) => span.max - SORT_MARK_W * 0.5,
                };
                paint_sort_mark(
                    &ui.painter().with_clip_rect(clip),
                    pos2(centre_x, cell.center().y),
                    direction,
                    t.color.text_dim,
                );
            }
            if index > 0 {
                ui.painter().with_clip_rect(clip).vline(
                    cell.left(),
                    cell.y_range(),
                    egui::Stroke::new(1.0, t.color.border),
                );
            }
            // Every edge but the table's last can be dragged, where it shows.
            if sortable && index + 1 < self.columns.len() && clip.x_range().contains(cell.right()) {
                edges.push((index, cell));
            }
        }
        // The edges after the heads, so a press on an edge drags it while a
        // click there still sorts: an edge senses dragging alone, and takes
        // no keyboard stop.
        let mut resize = None;
        for (index, cell) in edges {
            let grip = Rect::from_x_y_ranges(
                cell.right() - EDGE_GRIP * 0.5..=cell.right() + EDGE_GRIP * 0.5,
                cell.y_range(),
            )
            .intersect(ui.clip_rect());
            #[cfg(any(test, feature = "test-support"))]
            crate::probe::probe(ui, &format!("table.edge.{}", self.page_column(index)), grip);
            let edge = ui.interact(grip, ui.id().with((self.id, "edge", index)), Sense::DRAG);
            if edge.hovered() || edge.dragged() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeColumn);
                ui.painter().vline(
                    cell.right(),
                    cell.y_range(),
                    egui::Stroke::new(1.0, t.color.border_strong),
                );
            }
            let delta = edge.drag_delta().x;
            if edge.dragged() && delta != 0.0 {
                resize = Some((index, geometry.width(index) + delta));
            }
        }
        ui.painter().hline(
            rect.x_range(),
            rect.bottom() - 0.5,
            egui::Stroke::new(1.0, t.color.border),
        );
        HeadAnswer {
            sort: clicked,
            resize,
        }
    }

    /// Paint one band row: each span's label once, centred over the part of
    /// its run in view, between hairlines at the run's edges.
    fn band(
        &self,
        ui: &mut Ui,
        band: usize,
        spans: &[super::Span],
        view: View,
        geometry: &Columns,
        y: std::ops::RangeInclusive<f32>,
    ) {
        let t = Tokens::get(ui.ctx());
        let row = Rect::from_x_y_ranges(view.left..=view.left + view.width, y);
        #[cfg(any(test, feature = "test-support"))]
        crate::probe::probe(ui, "table.band", row);
        if !ui.is_rect_visible(row) {
            return;
        }
        ui.painter().rect_filled(row, 0.0, t.color.bg_panel_2);
        let (_, scrolled) = view.regions(geometry, row.y_range());
        let clip = scrolled.intersect(ui.clip_rect());
        let painter = ui.painter().with_clip_rect(clip);
        let stroke = egui::Stroke::new(1.0, t.color.border);
        let font = theme::sans(tokens::FS_0, FontWeight::Medium);
        for (index, span) in spans.iter().enumerate() {
            if span.label.trim().is_empty() {
                continue;
            }
            let Some(place) = geometry.span_at(span, view.offset, view.width) else {
                continue;
            };
            let extent = Rect::from_x_y_ranges(
                view.left + place.left..=view.left + place.right,
                row.y_range(),
            );
            #[cfg(any(test, feature = "test-support"))]
            crate::probe::probe(ui, &format!("table.span.{band}"), extent);
            painter.vline(extent.left(), extent.y_range(), stroke);
            painter.vline(extent.right(), extent.y_range(), stroke);
            painter.hline(extent.x_range(), extent.bottom() - 0.5, stroke);
            let shown = view.left + place.shown.start..view.left + place.shown.end;
            let room = (shown.end - shown.start - 2.0 * CELL_PAD).max(0.0);
            let elided = painter
                .layout_no_wrap(span.label.clone(), font.clone(), t.color.text_dim)
                .size()
                .x
                > room;
            let words = elide_text(ui, &span.label, &font, room);
            let galley = painter.layout_no_wrap(words, font.clone(), t.color.text_dim);
            let centre = (shown.start + shown.end) * 0.5;
            let landed = Rect::from_min_size(
                pos2(
                    centre - galley.size().x * 0.5,
                    row.center().y - galley.size().y * 0.5,
                ),
                galley.size(),
            );
            let label = ui.interact(
                extent.intersect(clip),
                ui.id().with((self.id, "span", band, index)),
                Sense::hover(),
            );
            label.widget_info(|| {
                egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &span.label)
            });
            if elided {
                let _ = label.on_hover_text(&span.label);
            }
            #[cfg(any(test, feature = "test-support"))]
            crate::probe::probe(ui, &format!("table.span.{band}.label"), landed);
            painter.galley(landed.min, galley, t.color.text_dim);
        }
    }
}

/// A small triangle: up for a column running ascending, down for
/// descending.
fn paint_sort_mark(
    painter: &egui::Painter,
    centre: egui::Pos2,
    direction: SortDirection,
    color: egui::Color32,
) {
    let half = SORT_MARK_W * 0.5;
    let rise = 2.0;
    let points = match direction {
        SortDirection::Ascending => vec![
            pos2(centre.x - half, centre.y + rise),
            pos2(centre.x + half, centre.y + rise),
            pos2(centre.x, centre.y - rise),
        ],
        SortDirection::Descending => vec![
            pos2(centre.x - half, centre.y - rise),
            pos2(centre.x + half, centre.y - rise),
            pos2(centre.x, centre.y + rise),
        ],
    };
    painter.add(egui::Shape::convex_polygon(
        points,
        color,
        egui::Stroke::NONE,
    ));
}

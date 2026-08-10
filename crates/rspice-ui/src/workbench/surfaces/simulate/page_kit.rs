//! Shared composition for the Simulation Studio setup pages.
//!
//! Every setup page is the same shape: a heading, then bounded cards laid out
//! in one- or two-column rows over the workspace surface. The primitives live
//! here so a page states what it owns and never re-derives the geometry.
//!
//! A card is a *bounded box* — a full 1 px border, the panel fill, and the
//! token corner radius. It is deliberately not a tile in a divider sheet: the
//! pages sit on the workspace background with a gutter on all four sides, and
//! a card that drew only some of its edges would leave hairlines dangling in
//! mid-air. Cards size to their content and never stretch to match a taller
//! neighbour, so a short card beside a tall one ends where its content ends
//! rather than in a slab of blank surface.

use egui::{Align, Color32, Layout, Rect, Response, Sense, Stroke, Ui, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

use super::super::super::design_system::elide_text;

/// Gutter around a setup page and between its cards.
pub(super) const PAGE_GUTTER: f32 = 10.0;
/// Card head: the title row and its status.
pub(super) const CARD_HEAD_H: f32 = 34.0;
/// Ledger head row.
pub(super) const TABLE_HEAD_H: f32 = 27.0;
/// Horizontal padding inside a card.
pub(super) const CARD_PAD_X: f32 = 10.0;
/// Below this width the two columns of a card row stack.
pub(super) const TWO_COLUMN_BREAKPOINT: f32 = 780.0;

/// Tone of a card's status, or of a single ledger cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Tone {
    Neutral,
    Ok,
    Warn,
    Error,
    Accent,
}

impl Tone {
    pub(super) fn color(self, ui: &Ui) -> Color32 {
        let c = Tokens::get(ui.ctx()).color;
        match self {
            Self::Neutral => c.text_faint,
            Self::Ok => c.ok,
            Self::Warn => c.warn,
            Self::Error => c.err,
            Self::Accent => c.accent,
        }
    }
}

/// Wrap a setup page.
///
/// The gutter is on all four sides, including the top: with no top padding the
/// first card's border lands exactly on the title row's border and its corners
/// read as sliced off.
pub(super) fn setup_page<R>(ui: &mut Ui, body: impl FnOnce(&mut Ui) -> R) -> R {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    egui::Frame::new()
        .fill(t.color.bg_app)
        .inner_margin(egui::Margin::same(PAGE_GUTTER as i8))
        .show(ui, |ui| {
            ui.set_width(width - PAGE_GUTTER * 2.0);
            ui.spacing_mut().item_spacing.y = PAGE_GUTTER;
            body(ui)
        })
        .inner
}

/// One bounded card with a head.
pub(super) fn card<R>(
    ui: &mut Ui,
    title: &str,
    status: Option<(&str, Tone)>,
    body: impl FnOnce(&mut Ui) -> R,
) -> R {
    card_with_head(ui, |ui| card_head(ui, title, status), body)
}

/// A card whose head carries controls rather than only a status.
pub(super) fn card_with_head<R>(
    ui: &mut Ui,
    head: impl FnOnce(&mut Ui),
    body: impl FnOnce(&mut Ui) -> R,
) -> R {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    egui::Frame::new()
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .corner_radius(t.radius)
        .show(ui, |ui| {
            ui.set_width(width - 2.0);
            ui.spacing_mut().item_spacing.y = 0.0;
            head(ui);
            body(ui)
        })
        .inner
}

/// Title on the left, status on the right, separated from the body by the
/// card's own hairline.
pub(super) fn card_head(ui: &mut Ui, title: &str, status: Option<(&str, Tone)>) {
    card_head_row(ui, title, status, |_| {});
}

/// Card head with trailing controls. The controls are laid out right to left,
/// so the first one added sits furthest right.
pub(super) fn card_head_row(
    ui: &mut Ui,
    title: &str,
    status: Option<(&str, Tone)>,
    controls: impl FnOnce(&mut Ui),
) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), CARD_HEAD_H), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let mut trailing = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(Rect::from_min_max(
                egui::pos2(rect.left() + CARD_PAD_X, rect.top()),
                egui::pos2(rect.right() - CARD_PAD_X, rect.bottom()),
            ))
            .layout(Layout::right_to_left(Align::Center)),
    );
    trailing.spacing_mut().item_spacing.x = 6.0;
    controls(&mut trailing);
    let consumed = trailing.min_rect().width();
    let status_right =
        rect.right() - CARD_PAD_X - if consumed > 0.0 { consumed + 8.0 } else { 0.0 };
    let mut title_right = status_right;
    if let Some((text, tone)) = status {
        let font = theme::mono(tokens::FS_0, FontWeight::Regular);
        let color = tone.color(ui);
        // Measure through the painter rather than `fonts_mut`: an exclusive
        // font lock held while the same `Ui` is read deadlocks the frame.
        let width = ui
            .painter()
            .layout_no_wrap(text.to_owned(), font.clone(), color)
            .size()
            .x;
        ui.painter().text(
            egui::pos2(status_right, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            text,
            font,
            tone.color(ui),
        );
        title_right = status_right - width - 10.0;
    }
    paint_text(
        ui,
        Rect::from_min_max(
            egui::pos2(rect.left() + CARD_PAD_X, rect.top()),
            egui::pos2(title_right.max(rect.left() + CARD_PAD_X), rect.bottom()),
        ),
        title,
        theme::sans(tokens::FS_1, FontWeight::SemiBold),
        t.color.text,
    );
}

/// Two cards side by side, each sized to its own content.
///
/// Below [`TWO_COLUMN_BREAKPOINT`] they stack, because two half-width cards
/// would clip their own controls before the page would.
///
/// The shared subject is threaded through rather than captured: both halves
/// mutate the same application state, and two closures that each captured it
/// would need unique access at the same time.
pub(super) fn card_row<T>(
    ui: &mut Ui,
    subject: &mut T,
    left: impl FnOnce(&mut Ui, &mut T),
    right: impl FnOnce(&mut Ui, &mut T),
) {
    let available = ui.available_width();
    if available < TWO_COLUMN_BREAKPOINT {
        left(ui, subject);
        ui.add_space(PAGE_GUTTER);
        right(ui, subject);
        return;
    }
    let column = ((available - PAGE_GUTTER) * 0.5).max(1.0);
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = PAGE_GUTTER;
        ui.allocate_ui_with_layout(vec2(column, 0.0), Layout::top_down(Align::Min), |ui| {
            ui.set_width(column);
            left(ui, subject);
        });
        ui.allocate_ui_with_layout(vec2(column, 0.0), Layout::top_down(Align::Min), |ui| {
            ui.set_width(column);
            right(ui, subject);
        });
    });
}

/// A ledger head row: uppercase column labels over the table's own fill.
pub(super) fn ledger_head(ui: &mut Ui, fractions: &[f32], labels: &[&str]) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) =
        ui.allocate_exact_size(vec2(ui.available_width(), TABLE_HEAD_H), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    for (cell, label) in column_rects(rect, fractions).into_iter().zip(labels) {
        paint_text(
            ui,
            cell.shrink2(vec2(CARD_PAD_X * 0.8, 0.0)),
            &label.to_uppercase(),
            theme::sans(tokens::FS_0, FontWeight::Medium),
            t.color.text_faint,
        );
    }
}

/// A group caption inside a ledger.
///
/// Used where a table holds more than one kind of row and a flat list would
/// read as unrelated numbers. It states the criterion in the notation the
/// engine uses, so it is deliberately not uppercased.
pub(super) fn ledger_group(ui: &mut Ui, caption: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 23.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_panel_2);
    ui.painter()
        .hline(rect.x_range(), rect.top(), Stroke::new(1.0, t.color.border));
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    paint_text(
        ui,
        rect.shrink2(vec2(CARD_PAD_X * 0.8, 0.0)),
        caption,
        theme::mono(tokens::FS_0, FontWeight::Medium),
        t.color.text_faint,
    );
}

/// A read-only ledger row. Returns the row response so a page can select it.
pub(super) fn ledger_row(
    ui: &mut Ui,
    fractions: &[f32],
    cells: &[(&str, Tone)],
    selected: bool,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), t.metrics.row_h), Sense::click());
    if selected {
        ui.painter().rect_filled(rect, 0.0, t.color.accent_dim);
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    for (cell, (text, tone)) in column_rects(rect, fractions).into_iter().zip(cells) {
        let color = if *tone == Tone::Neutral {
            t.color.text_dim
        } else {
            tone.color(ui)
        };
        paint_text(
            ui,
            cell.shrink2(vec2(CARD_PAD_X * 0.8, 0.0)),
            text,
            theme::mono(tokens::FS_0, FontWeight::Regular),
            color,
        );
    }
    response
}

/// Reserve one ledger row and hand back its column rects, for a row that has
/// to host controls rather than text.
pub(super) fn ledger_row_cells(ui: &mut Ui, fractions: &[f32]) -> (Rect, Vec<Rect>) {
    let t = Tokens::get(ui.ctx());
    let height = t.metrics.ctl_h + 8.0;
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    (rect, column_rects(rect, fractions))
}

/// A child `Ui` over an exact rect, for placing a control inside a reserved
/// ledger cell.
pub(super) fn cell_ui(ui: &mut Ui, rect: Rect) -> Ui {
    ui.new_child(
        egui::UiBuilder::new()
            .max_rect(rect)
            .layout(Layout::left_to_right(Align::Center)),
    )
}

/// Split a row into columns by fraction of its width. Fractions that do not
/// sum to one leave the remainder in the last column, which is what a caller
/// wants when the last column is the free one.
pub(super) fn column_rects(rect: Rect, fractions: &[f32]) -> Vec<Rect> {
    let mut rects = Vec::with_capacity(fractions.len());
    let mut left = rect.left();
    for (index, fraction) in fractions.iter().enumerate() {
        let right = if index + 1 == fractions.len() {
            rect.right()
        } else {
            left + rect.width() * fraction
        };
        rects.push(Rect::from_min_max(
            egui::pos2(left, rect.top()),
            egui::pos2(right, rect.bottom()),
        ));
        left = right;
    }
    rects
}

/// A card's closing prose. Explains a rule the controls above cannot state.
pub(super) fn card_note(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::new()
        .inner_margin(egui::Margin {
            left: CARD_PAD_X as i8,
            right: CARD_PAD_X as i8,
            top: 8,
            bottom: 9,
        })
        .show(ui, |ui| {
            ui.add(
                egui::Label::new(
                    egui::RichText::new(text)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_faint),
                )
                .wrap(),
            );
        });
}

/// The body region of a card: horizontal padding and a comfortable rhythm for
/// stacked form rows.
pub(super) fn card_body<R>(ui: &mut Ui, body: impl FnOnce(&mut Ui) -> R) -> R {
    egui::Frame::new()
        .inner_margin(egui::Margin {
            left: CARD_PAD_X as i8,
            right: CARD_PAD_X as i8,
            top: 8,
            bottom: 9,
        })
        .show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 6.0;
            body(ui)
        })
        .inner
}

/// A control painted into a field's reserved width.
///
/// Taken as a trait object rather than a generic so a caller can pass two
/// different control kinds in the same row without the row becoming generic
/// over both.
pub(super) type FieldControl<'a> = &'a mut dyn FnMut(&mut Ui, f32);

/// A labeled field: its caption and the control that fills its column.
pub(super) type Field<'a> = (&'a str, FieldControl<'a>);

/// A labeled field laid out as a caption over its control, two per row.
///
/// The shared inspector form row weights its columns for a label/value pair;
/// these pages hold two equal fields per row, and the narrow column clipped
/// option text.
pub(super) fn field_pair(ui: &mut Ui, left: Field<'_>, right: Option<Field<'_>>) {
    let gap = 11.0;
    let available = ui.available_width();
    let column = if right.is_some() {
        ((available - gap) * 0.5).max(1.0)
    } else {
        available
    };
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = gap;
        field(ui, column, left.0, left.1);
        if let Some((label, control)) = right {
            field(ui, column, label, control);
        }
    });
}

fn field(ui: &mut Ui, width: f32, label: &str, control: FieldControl<'_>) {
    let t = Tokens::get(ui.ctx());
    ui.allocate_ui_with_layout(vec2(width, 0.0), Layout::top_down(Align::Min), |ui| {
        ui.set_width(width);
        ui.spacing_mut().item_spacing.y = 4.0;
        let (label_rect, _) = ui.allocate_exact_size(vec2(width, 14.0), Sense::hover());
        paint_text(
            ui,
            label_rect,
            label,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        control(ui, width);
    });
}

/// A read-only property row: label left, contract right, wrapping against a
/// common left edge because these values are sentences rather than numbers.
pub(super) fn rule_row(ui: &mut Ui, label: &str, contract: &str) {
    let t = Tokens::get(ui.ctx());
    let gap = 10.0;
    let available = ui.available_width();
    let label_width = (available * 0.38).max(1.0);
    ui.horizontal_top(|ui| {
        ui.spacing_mut().item_spacing.x = gap;
        ui.allocate_ui_with_layout(vec2(label_width, 0.0), Layout::top_down(Align::Min), |ui| {
            ui.set_width(label_width);
            ui.add(
                egui::Label::new(
                    egui::RichText::new(label)
                        .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                        .color(t.color.text_dim),
                )
                .wrap(),
            );
        });
        ui.add(
            egui::Label::new(
                egui::RichText::new(contract)
                    .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                    .color(t.color.text),
            )
            .wrap(),
        );
    });
}

/// Paint text clipped to a rect, eliding rather than overflowing its cell.
pub(super) fn paint_text(ui: &Ui, rect: Rect, text: &str, font: egui::FontId, color: Color32) {
    if !ui.is_rect_visible(rect) || rect.width() <= 0.0 {
        return;
    }
    let shown = elide_text(ui, text, &font, rect.width());
    ui.painter().text(
        egui::pos2(rect.left(), rect.center().y),
        egui::Align2::LEFT_CENTER,
        shown,
        font,
        color,
    );
}

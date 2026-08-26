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
    /// Secondary text: a label, a caption, prose beside a control. One step
    /// above [`Self::Neutral`], which is the metadata register.
    Dim,
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
            Self::Dim => c.text_dim,
            Self::Ok => c.ok,
            Self::Warn => c.warn,
            Self::Error => c.err,
            Self::Accent => c.accent,
        }
    }
}

/// One line of studio prose, in the studio's own type.
///
/// `ui.label` with a bare string or an unfonted `RichText` paints in egui's
/// default text style: 13 px of a family the theme assigns no weight to, which
/// is a step and a half above every caption beside it and a register the token
/// scale does not name. Ten lines shipped that way — a search's empty state, a
/// predicate hint, two field labels, four plan-unavailable notices — each one
/// slightly larger than the text it sat next to for no reason a reader could
/// find.
///
/// Wrapped rather than elided: these are sentences, and half of one says less
/// than none.
pub(super) fn note_line(ui: &mut Ui, text: &str, tone: Tone) -> egui::Response {
    ui.add(
        egui::Label::new(
            egui::RichText::new(text)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(tone.color(ui)),
        )
        .wrap(),
    )
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

/// The controls that narrow the ledger below, on the table's own full width.
///
/// Deliberately not in the card head. The head already carries the registry's
/// status, which is a sentence rather than a chip, and it is painted right to
/// left from whatever the controls leave: below roughly 900 px a filter and a
/// class picker there push the status over the card's own title and off its
/// left edge. The table's own width has room for both at every width the pages
/// support.
pub(super) fn filter_row(ui: &mut Ui, controls: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    let height = t.metrics.ctl_h + 14.0;
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let mut row = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(Rect::from_min_max(
                egui::pos2(rect.left() + CARD_PAD_X, rect.top()),
                egui::pos2(rect.right() - CARD_PAD_X, rect.bottom()),
            ))
            .layout(Layout::left_to_right(Align::Center)),
    );
    row.spacing_mut().item_spacing.x = 6.0;
    controls(&mut row);
}

/// A filter field, sized to a search box rather than to the row it sits in.
///
/// The field is view state only. A table that offers one must still say what
/// it is hiding — narrowed to nothing it has to read as a filter that matched
/// nothing, never as an empty registry.
pub(super) fn filter_field(
    ui: &mut Ui,
    id_salt: &'static str,
    placeholder: &str,
    query: &mut String,
) {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().clamp(110.0, 250.0);
    let response = ui.add_sized(
        [width, t.metrics.ctl_h],
        egui::TextEdit::singleline(query)
            .id_salt(id_salt)
            .hint_text(placeholder)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .margin(egui::Margin {
                left: 25,
                right: 8,
                top: 3,
                bottom: 3,
            }),
    );
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_label(placeholder.to_owned());
    });
    super::super::super::design_system::WorkbenchIcon::Search.paint(
        ui.painter(),
        Rect::from_center_size(
            egui::pos2(response.rect.left() + 14.0, response.rect.center().y),
            vec2(13.0, 13.0),
        ),
        t.color.text_faint,
    );
}

/// Whether a registry row survives the filter above it.
///
/// Matched against the cells the row paints and nothing else, so a row that
/// stays is always one the reader can see the reason for. Matching a field the
/// table does not render would leave rows that appear to match nothing.
pub(super) fn row_matches(query: &str, cells: &[&str]) -> bool {
    let query = query.trim().to_ascii_lowercase();
    query.is_empty()
        || cells
            .iter()
            .any(|cell| cell.to_ascii_lowercase().contains(&query))
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
    // The first cell is the row's identifier in every caller, so it is the
    // name a screen reader should announce for the selection.
    let row_label = cells
        .first()
        .map(|(text, _)| (*text).to_owned())
        .unwrap_or_default();
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            row_label.clone(),
        )
    });
    theme::paint_focus_ring(ui, &response, rect);
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

/// Column split of a receipts ledger: sequence, what was done, the revision it
/// moved, and the digest of the state it adopted.
const RECEIPT_COLUMNS: [f32; 4] = [0.10, 0.34, 0.20, 0.36];
/// Receipts shown. The log is evidence, not a history the page has to hold.
const RECEIPT_LIMIT: usize = 6;

/// One receipt, already rendered into the words its ledger shows.
///
/// The card takes rows rather than a receipt type because the two logs it
/// serves are different types — a run-set transaction and a plan configuration
/// change — and both are the same evidence to the person reading them.
pub(super) struct ReceiptRow {
    pub(super) sequence: String,
    pub(super) action: String,
    pub(super) tone: Tone,
    pub(super) revision: String,
    pub(super) digest: String,
}

/// The receipts card. Newest first, capped, with the cap owned here so five
/// surfaces cannot disagree about how much evidence a card holds.
///
/// `rows` are chronological; the card reverses them. An empty log is a state
/// worth naming, so it gets its own note rather than an empty table.
pub(super) fn receipts_card(
    ui: &mut Ui,
    title: &str,
    empty_status: &str,
    notes: (&str, &str),
    rows: &[ReceiptRow],
) {
    let (empty_note, closing_note) = notes;
    let status = if rows.is_empty() {
        empty_status.to_owned()
    } else {
        format!("{} recorded", rows.len())
    };
    card(ui, title, Some((status.as_str(), Tone::Neutral)), |ui| {
        if rows.is_empty() {
            card_note(ui, empty_note);
            return;
        }
        ledger_head(ui, &RECEIPT_COLUMNS, &["#", "Action", "Revision", "Digest"]);
        for row in rows.iter().rev().take(RECEIPT_LIMIT) {
            ledger_row(
                ui,
                &RECEIPT_COLUMNS,
                &[
                    (row.sequence.as_str(), Tone::Neutral),
                    (row.action.as_str(), row.tone),
                    (row.revision.as_str(), Tone::Neutral),
                    (row.digest.as_str(), Tone::Neutral),
                ],
                false,
            );
        }
        card_note(ui, closing_note);
    });
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

/// One choice in a [`command_popup`]: what it is called, and why it cannot be
/// taken. A disabled choice stays listed rather than vanishing — the authored
/// domain is the point, and a list that silently shortens teaches nothing.
pub(super) struct PopupChoice {
    pub(super) label: String,
    pub(super) unavailable: Option<&'static str>,
}

/// A design-system button that opens a list of named choices; returns the index
/// of the one taken.
///
/// egui's `menu_button` paints its own trigger in egui's default chrome and
/// fills the list with `ui.button`, so a card head that used it carried two
/// controls this workbench does not otherwise draw. The list geometry here is
/// the select's option list, which is the only drop-down these pages have.
///
/// A row is reachable from the keyboard, and taking one there does what taking
/// one with the pointer does. Only the available rows sense clicks, which is
/// also what makes them focusable, so tabbing walks the choices that can be
/// taken and skips the ones that state why they cannot. The explicit close is
/// the half egui does not do: a `CloseOnClick` popup is dismissed by a *pointer*
/// click and by nothing else, so Space or Enter on the focused row used to add
/// the dimension and leave the list standing over the page it had just changed.
pub(super) fn command_popup(
    ui: &mut Ui,
    id_salt: &str,
    button: crate::ui::widgets::Button<'_>,
    empty_note: &str,
    choices: &[PopupChoice],
) -> Option<usize> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let response = button.show(ui);
    let popup_id = ui.make_persistent_id(("rspice.command-popup", id_salt));
    if response.clicked() {
        egui::Popup::toggle_id(ui.ctx(), popup_id);
    }
    let mut picked = None;
    let widest = choices
        .iter()
        .map(|choice| {
            ui.painter()
                .layout_no_wrap(
                    choice.label.clone(),
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    c.text,
                )
                .size()
                .x
        })
        .fold(response.rect.width(), f32::max)
        + 24.0;
    egui::Popup::from_response(&response)
        .id(popup_id)
        .open_memory(None)
        .layout(Layout::top_down_justified(Align::LEFT))
        .align(egui::RectAlign::BOTTOM_START)
        .width(widest)
        .close_behavior(egui::PopupCloseBehavior::CloseOnClick)
        .show(|ui| {
            ui.set_min_width(widest);
            ui.spacing_mut().item_spacing.y = 0.0;
            if choices.is_empty() {
                card_note(ui, empty_note);
                return;
            }
            for (index, choice) in choices.iter().enumerate() {
                let height = t.metrics.ctl_h.max(24.0);
                let (row, row_response) = ui.allocate_exact_size(
                    vec2(ui.available_width(), height),
                    if choice.unavailable.is_some() {
                        Sense::hover()
                    } else {
                        Sense::click()
                    },
                );
                let row_response = match choice.unavailable {
                    Some(reason) => row_response.on_hover_text(reason),
                    None => row_response,
                };
                row_response.widget_info(|| {
                    egui::WidgetInfo::labeled(
                        egui::WidgetType::Button,
                        ui.is_enabled() && choice.unavailable.is_none(),
                        &choice.label,
                    )
                });
                if row_response.hovered() {
                    ui.painter().rect_filled(row, t.radius, c.bg_hover);
                }
                ui.painter().text(
                    egui::pos2(row.left() + 8.0, row.center().y),
                    egui::Align2::LEFT_CENTER,
                    &choice.label,
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    if choice.unavailable.is_some() {
                        c.text_faint
                    } else {
                        c.text
                    },
                );
                theme::paint_focus_ring(ui, &row_response, row);
                if choice.unavailable.is_none()
                    && row_response
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                {
                    picked = Some(index);
                }
            }
        });
    if picked.is_some() {
        egui::Popup::close_id(ui.ctx(), popup_id);
    }
    picked
}

/// A bare on/off control for a cell whose row already names it.
///
/// The two tables that carry a boolean per row — the resolved point table's
/// include column and the variable importer's accept column — state the
/// subject in the cells beside the control, so a switch with a label of its
/// own would repeat what the row has already said. The same holds for a form
/// field, where the caption is painted above the cell the control fills.
///
/// It still has to *announce* one. A self-painted control publishes only what
/// it is given, and a column of controls that publish nothing is a column a
/// reader cannot tell apart — which is what twenty-seven nameless tick boxes
/// in the point table were. `name` is that announcement and there is no
/// spelling of this that omits it.
///
/// Returns the response so a caller can hang the row's own tooltip on it;
/// [`egui::Response::changed`] is set exactly when the reader moved the value.
pub(super) fn switch_cell(ui: &mut Ui, name: &str, value: &mut bool) -> Response {
    use super::ANALYSIS_SWITCH_WIDTH as SWITCH_W;

    let t = Tokens::get(ui.ctx());
    let (rect, mut response) =
        ui.allocate_exact_size(vec2(SWITCH_W, t.metrics.ctl_h), Sense::click());
    // The enabled bit is read once and carried into the paint, the toggle and
    // the announcement alike: a self-painted control gets none of egui's
    // disabled styling, and its response still reports clicks inside a
    // disabled `Ui`.
    let enabled = ui.is_enabled();
    if enabled && response.clicked() {
        *value = !*value;
        response.mark_changed();
    }
    super::paint_switch(
        ui,
        rect.center(),
        *value,
        enabled && response.hovered(),
        rect,
    );
    let announced = *value;
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Checkbox, enabled, announced, name)
    });
    theme::paint_focus_ring(ui, &response, rect);
    if enabled {
        response.on_hover_cursor(egui::CursorIcon::PointingHand)
    } else {
        response
    }
}

/// A labelled on/off control, painted with this design system's own switch.
///
/// Four settings rows on these pages were still `ui.checkbox`, which is egui's
/// tick box and not a control this studio ships. One row shape serves all of
/// them now, so another cannot be written in the other style without somebody
/// noticing.
///
/// Every boolean the studio paints is one of these, the bare [`switch_cell`]
/// where the row beside it already carries the label, or — where an editor
/// column is too narrow to hold two — the design system's own
/// [`crate::ui::widgets::switch_row`], which paints the same switch on a tree
/// row. That is the mockup's rule for the simulation stage and its advanced
/// options — `label.switch` throughout, with `.check-row` appearing only in
/// the project launcher and the results and platform workflows — and the
/// studio now holds it in full. [`tests::the_studio_paints_no_tick_box`] is
/// what keeps it: the doc here once claimed a tick box on four rows and a
/// switch on every other, which was true of the four and false of the rest,
/// and it went on being false at narrow widths for as long as the ban named
/// only egui's spellings. A claim about how many of something ships drifts
/// back to being wrong the moment nothing checks it.
///
/// Returns whether the reader changed the value. The label elides rather than
/// wrapping, because these rows sit inside cards whose width is the page's.
pub(super) fn switch_row(ui: &mut Ui, label: &str, value: &mut bool) -> bool {
    /// Between the switch and the text it labels.
    const LABEL_GAP: f32 = 8.0;
    use super::ANALYSIS_SWITCH_WIDTH as SWITCH_W;

    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let available = ui.available_width().max(SWITCH_W + LABEL_GAP);
    let text = elide_text(ui, label, &font, available - SWITCH_W - LABEL_GAP);
    let galley = ui.fonts_mut(|fonts| fonts.layout_no_wrap(text, font, Color32::WHITE));
    let height = t.metrics.ctl_h.max(galley.size().y);
    let width = (SWITCH_W + LABEL_GAP + galley.size().x).min(available);
    let (rect, mut response) = ui.allocate_exact_size(vec2(width, height), Sense::click());

    // A self-painted control gets none of egui's disabled styling, and the
    // response still reports hover and clicks inside a disabled `Ui`. So the
    // enabled bit is read once and carried into the paint, the announcement
    // and the toggle alike.
    let enabled = ui.is_enabled();
    let changed = enabled && response.clicked();
    if changed {
        *value = !*value;
        response.mark_changed();
    }
    super::paint_switch(
        ui,
        egui::pos2(rect.left() + SWITCH_W * 0.5, rect.center().y),
        *value,
        enabled && response.hovered(),
        rect,
    );
    ui.painter().galley(
        egui::pos2(
            rect.left() + SWITCH_W + LABEL_GAP,
            rect.center().y - galley.size().y * 0.5,
        ),
        galley,
        if enabled {
            t.color.text
        } else {
            t.color.text_faint
        },
    );
    let announced = *value;
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Checkbox, enabled, announced, label)
    });
    theme::paint_focus_ring(ui, &response, rect);
    if enabled {
        response.on_hover_cursor(egui::CursorIcon::PointingHand);
    }
    changed
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    /// What the trigger of the harness popup announces.
    const TRIGGER: &str = "Add something";
    /// The choice that can be taken.
    const AVAILABLE: &str = "Available choice";
    /// The choice that states why it cannot.
    const BLOCKED: &str = "Blocked choice";

    /// One [`super::command_popup`] driven by keyboard and pointer.
    ///
    /// Rendered on its own rather than through a page, because what is under
    /// test is the popup's own interaction contract and a route around it
    /// would put a page's layout between the assertion and the row.
    ///
    /// Each pass keeps the accessibility tree it published, so a row is found
    /// by the string a screen reader is given and the focused widget is named
    /// by looking its id up in that same tree — an AccessKit node id is the
    /// egui id's value, which is what makes the two tables joinable.
    struct PopupHarness {
        ctx: egui::Context,
        choices: Vec<super::PopupChoice>,
        picked: Option<usize>,
        nodes: Vec<(u64, String, egui::Rect)>,
    }

    impl PopupHarness {
        /// A harness with one takeable choice and one refused one, settled.
        fn new() -> Self {
            let ctx = egui::Context::default();
            crate::ui::Theme::default().apply(&ctx);
            ctx.enable_accesskit();
            let mut popup = Self {
                ctx,
                choices: vec![
                    super::PopupChoice {
                        label: AVAILABLE.to_owned(),
                        unavailable: None,
                    },
                    super::PopupChoice {
                        label: BLOCKED.to_owned(),
                        unavailable: Some("already declared"),
                    },
                ],
                picked: None,
                nodes: Vec::new(),
            };
            // Twice: the first pass builds the font set and the second lays
            // out against it, so a rectangle measured here is the one the
            // control ends up in.
            popup.pass(Vec::new());
            popup.pass(Vec::new());
            popup
        }

        fn pass(&mut self, events: Vec<egui::Event>) {
            let choices = &self.choices;
            let mut picked = None;
            let output = self.ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        egui::vec2(420.0, 320.0),
                    )),
                    events,
                    ..egui::RawInput::default()
                },
                |ui| {
                    picked = super::command_popup(
                        ui,
                        "page-kit.popup-keyboard",
                        crate::ui::widgets::Button::new(TRIGGER),
                        "nothing can be added",
                        choices,
                    );
                },
            );
            self.picked = picked;
            self.nodes = output
                .platform_output
                .accesskit_update
                .map(|update| {
                    update
                        .nodes
                        .iter()
                        .filter_map(|(id, node)| {
                            let label = node.label()?.to_owned();
                            let bounds = node.bounds()?;
                            Some((
                                id.0,
                                label,
                                egui::Rect::from_min_max(
                                    egui::pos2(bounds.x0 as f32, bounds.y0 as f32),
                                    egui::pos2(bounds.x1 as f32, bounds.y1 as f32),
                                ),
                            ))
                        })
                        .collect()
                })
                .unwrap_or_default();
        }

        /// Press and release the pointer over the control announcing `label`.
        fn click(&mut self, label: &str) {
            let at = self
                .nodes
                .iter()
                .find(|(_, announced, _)| announced.as_str() == label)
                .unwrap_or_else(|| panic!("no control announces {label:?}: {:?}", self.announced()))
                .2
                .center();
            self.pass(vec![
                egui::Event::PointerMoved(at),
                egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed: true,
                    modifiers: egui::Modifiers::default(),
                },
                egui::Event::PointerButton {
                    pos: at,
                    button: egui::PointerButton::Primary,
                    pressed: false,
                    modifiers: egui::Modifiers::default(),
                },
            ]);
            self.pass(Vec::new());
        }

        fn key(&mut self, key: egui::Key) {
            self.pass(vec![egui::Event::Key {
                key,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::default(),
            }]);
        }

        /// What the widget holding keyboard focus announces, if anything.
        fn focused(&self) -> Option<String> {
            let id = self.ctx.memory(egui::Memory::focused)?;
            self.nodes
                .iter()
                .find(|(node, _, _)| *node == id.value())
                .map(|(_, label, _)| label.clone())
        }

        fn announced(&self) -> Vec<&str> {
            self.nodes
                .iter()
                .map(|(_, label, _)| label.as_str())
                .collect()
        }
    }

    /// A popup row is taken from the keyboard, and taking it dismisses the list.
    ///
    /// The rows were click-sensed and announced correctly from the start, so
    /// egui already turned Space and Enter on the focused row into a click.
    /// What it does not do is close the list: a `CloseOnClick` popup is
    /// dismissed by a *pointer* click and by nothing else, so the keyboard
    /// path added the dimension and left the choices standing over the page.
    #[test]
    fn a_command_popup_row_is_taken_from_the_keyboard() {
        let mut popup = PopupHarness::new();
        popup.click(TRIGGER);
        assert!(
            popup.announced().contains(&AVAILABLE),
            "the trigger opens the list; it announced {:?}",
            popup.announced()
        );

        // Whether the click left focus on the trigger is egui's business, so
        // the walk is bounded rather than counted.
        for _ in 0..4 {
            if popup.focused().as_deref() == Some(AVAILABLE) {
                break;
            }
            popup.key(egui::Key::Tab);
        }
        assert_eq!(
            popup.focused().as_deref(),
            Some(AVAILABLE),
            "tabbing reaches the takeable row; focus sat on {:?}",
            popup.focused()
        );

        popup.key(egui::Key::Enter);
        assert_eq!(
            popup.picked,
            Some(0),
            "Enter on the focused row takes that choice"
        );
        popup.pass(Vec::new());
        assert!(
            !popup.announced().contains(&AVAILABLE),
            "and the list closes behind it; it still announced {:?}",
            popup.announced()
        );
    }

    /// A refused row is listed, and keyboard focus never lands on it.
    ///
    /// The list keeps every choice the domain has, including the ones that
    /// cannot be taken — a list that silently shortens teaches nothing. But a
    /// refused row is not a control: it senses hover only, which is also what
    /// keeps it out of the tab order, so the keyboard walks exactly the rows
    /// that can be acted on.
    #[test]
    fn an_unavailable_command_popup_row_is_listed_but_never_focused() {
        let mut popup = PopupHarness::new();
        popup.click(TRIGGER);
        assert!(
            popup.announced().contains(&BLOCKED),
            "a refused choice stays listed; it announced {:?}",
            popup.announced()
        );

        let mut visited = Vec::new();
        for _ in 0..6 {
            popup.key(egui::Key::Tab);
            if let Some(label) = popup.focused() {
                visited.push(label);
            }
        }
        assert!(
            visited.iter().any(|label| label.as_str() == AVAILABLE),
            "the walk reached the takeable row; it visited {visited:?}"
        );
        assert!(
            !visited.iter().any(|label| label.as_str() == BLOCKED),
            "and never the refused one; it visited {visited:?}"
        );
    }

    /// Every `.rs` file the Simulation Studio ships, read at test time.
    ///
    /// Walked rather than listed with `include_str!`, so a page added to the
    /// studio is scanned the day it lands rather than the day someone
    /// remembers to add it here.
    fn studio_sources() -> Vec<(PathBuf, String)> {
        fn walk(directory: &Path, out: &mut Vec<PathBuf>) {
            let entries = std::fs::read_dir(directory)
                .unwrap_or_else(|error| panic!("read {}: {error}", directory.display()));
            for entry in entries {
                let path = entry.expect("directory entry").path();
                if path.is_dir() {
                    walk(&path, out);
                } else if path.extension().is_some_and(|extension| extension == "rs") {
                    out.push(path);
                }
            }
        }

        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/workbench/surfaces");
        let mut paths = vec![root.join("simulate.rs")];
        walk(&root.join("simulate"), &mut paths);
        paths.sort();
        let sources: Vec<(PathBuf, String)> = paths
            .into_iter()
            .map(|path| {
                let source = std::fs::read_to_string(&path)
                    .unwrap_or_else(|error| panic!("read {}: {error}", path.display()));
                (path, source)
            })
            .collect();
        // A test-only file is free to author whatever it needs to author. Which
        // ones those are is read from the module that declared them, not from a
        // list here that would go stale.
        let roots: Vec<PathBuf> = sources
            .iter()
            .flat_map(|(path, source)| crate::source_guard::test_only_roots(path, source))
            .collect();
        sources
            .iter()
            .filter(|(path, _)| crate::source_guard::ships(path, &roots))
            .map(|(path, source)| {
                (
                    path.clone(),
                    without_line_comments(&crate::source_guard::without_test_items(source)),
                )
            })
            .collect()
    }

    /// `source` with every `//` tail removed, line for line.
    ///
    /// A doc comment that quotes a control constructor is prose about it, not a
    /// call to it, and two of them are how this module explains why the raw
    /// ones are not used. Line numbers are kept, because the failure names the
    /// line a match sits on.
    fn without_line_comments(source: &str) -> String {
        source
            .lines()
            .map(|line| line.split("//").next().unwrap_or_default())
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Every `call` in `source`, as `(line, argument list)`.
    fn calls<'a>(source: &'a str, call: &str) -> Vec<(usize, &'a str)> {
        let mut found = Vec::new();
        for (index, _) in source.match_indices(call) {
            let rest = &source[index + call.len()..];
            let mut depth = 1usize;
            let mut end = rest.len();
            for (offset, character) in rest.char_indices() {
                match character {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 {
                            end = offset;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            found.push((1 + source[..index].matches('\n').count(), &rest[..end]));
        }
        found
    }

    /// Nothing the studio paints falls back to egui's default text style.
    ///
    /// `Ui::label` with a bare string, or with a `RichText` that never names a
    /// font, paints at 13 px in a family the theme assigns no weight to. That
    /// is a register the token scale does not have: the line sits a step and a
    /// half above the captions beside it, and nothing chose it. Ten shipped
    /// that way — a search's empty state, a predicate hint, two field labels,
    /// four plan-unavailable notices — and each was a place someone reached
    /// for egui instead of [`super::note_line`].
    ///
    /// The receiver is not always spelled `ui`. A control placed in a reserved
    /// ledger cell is drawn on a child `Ui` named for the cell, and a two-up
    /// row is drawn on `columns[0]`, so a scan anchored on `ui.label(` had a
    /// hole exactly where the studio does its most bespoke layout — the
    /// advanced-options editor row painted both of its text cells at egui's
    /// default through a child called `hint` and one called `name`. Any
    /// receiver counts here.
    ///
    /// An empty argument list is skipped: `kind.label()` and its two dozen
    /// siblings are enum getters that hand back the word a row states, not
    /// calls that paint one. `set_label(` never matches — the dot belongs to
    /// `set`, not to `label`.
    ///
    /// The check is on the source rather than the render because a font is a
    /// literal at its call site, and a rendered galley cannot say whether the
    /// size it carries was chosen or inherited.
    #[test]
    fn the_studio_names_the_font_of_every_line_it_paints() {
        let sources = studio_sources();
        assert!(
            sources.len() >= 20,
            "the studio scan found only {} files; the walk has stopped reaching them",
            sources.len()
        );

        let mut scanned = 0usize;
        let mut untyped = Vec::new();
        for (path, source) in &sources {
            for (line, argument) in calls(source, ".label(") {
                if argument.trim().is_empty() {
                    continue;
                }
                scanned += 1;
                if !argument.contains(".font(") {
                    untyped.push(format!("{}:{line}", path.display()));
                }
            }
        }
        assert!(
            scanned >= 30,
            "the scan matched only {scanned} label calls; the pattern has stopped matching"
        );
        assert!(
            untyped.is_empty(),
            "these lines paint in egui's default text style — give them a token \
             size through `page_kit::note_line` or a `RichText::font`:\n  {}",
            untyped.join("\n  ")
        );
    }

    /// The studio authors no raw egui action control.
    ///
    /// `ui.button` paints egui's own chrome, which is a different height, fill
    /// and focus ring from every other control on the page — the one place
    /// that reached past [`crate::ui::widgets::Button`], the executed-deck
    /// record in the inspector, shipped a control that did not look like the
    /// surface it sat on.
    ///
    /// `ui.checkbox` is deliberately not here. The studio's tick boxes belong
    /// to named row constructors that wrap it, and whether those rows stay tick
    /// boxes at all is a question about the design rather than about the call
    /// site; a ban here would answer it by accident.
    #[test]
    fn the_studio_authors_no_raw_egui_action_control() {
        let raw: Vec<String> = studio_sources()
            .iter()
            .flat_map(|(path, source)| {
                ["ui.button(", "ui.selectable_label(", "ui.link("]
                    .into_iter()
                    .flat_map(move |call| {
                        calls(source, call)
                            .into_iter()
                            .map(move |(line, _)| format!("{}:{line}: {call}", path.display()))
                    })
            })
            .collect();
        assert!(
            raw.is_empty(),
            "these lines author a raw egui control; the design system has one for \
             each of them:\n  {}",
            raw.join("\n  ")
        );
    }

    /// The studio paints no tick box at all.
    ///
    /// This used to be a census. It named the five call sites left and asserted
    /// exactly those, because the doc on [`super::switch_row`] had claimed the
    /// studio painted a tick box on four rows and a switch on every other —
    /// true of the four it had just converted, false of everything else — and
    /// a claim about how many of something ships drifts back to being wrong
    /// the next time one is added.
    ///
    /// The five are gone, so the census is a ban: the point table's include
    /// column, the workflow dialogs' `workflow_switch`, and the importer's
    /// scope control and per-row accept column are all [`super::switch_row`]
    /// or [`super::switch_cell`] now. Every boolean in the mockup's simulation
    /// stage and its advanced options is a `label.switch`, and there is no
    /// longer a number here for a sixth to hide behind.
    ///
    /// The ban had a hole under it for as long as it named only egui's own
    /// spellings. Below 420 points the analysis form's boolean row falls back
    /// to the design system's row, which painted a tick box on a tree row —
    /// a call in a file this scan does read, spelt in a way it could not see,
    /// so every narrow editor column showed a control the rest of the studio
    /// had stopped using. That row paints the switch now and is named
    /// `switch_row` for it, so `check_row(` and `.checkbox(` name nothing
    /// anywhere in the crate and are banned here to keep it that way.
    #[test]
    fn the_studio_paints_no_tick_box() {
        let sites: Vec<String> = studio_sources()
            .iter()
            .flat_map(|(path, source)| {
                [
                    "ui.checkbox(",
                    "egui::Checkbox::",
                    "widgets::tick_box(",
                    "check_row(",
                    ".checkbox(",
                ]
                .into_iter()
                .flat_map(move |call| {
                    calls(source, call).into_iter().map(move |(line, _)| {
                        format!(
                            "{}:{line}: {call}",
                            path.file_name().unwrap_or_default().to_string_lossy()
                        )
                    })
                })
            })
            .collect();
        assert!(
            sites.is_empty(),
            "the studio's booleans are switches; these are tick boxes:\n  {}",
            sites.join("\n  ")
        );
    }
}

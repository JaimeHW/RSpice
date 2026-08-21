//! The painters every page of this workspace draws with.
//!
//! One card, one property row, one table header, one selectable data row, one
//! empty state. They live together because the pages have to agree: a table
//! whose header measured its columns differently from the rows under it puts
//! every cell in the wrong place, and two spellings of "this page has nothing
//! on it" is two designs.
//!
//! Nothing here reads or writes application state. Every function takes what
//! it paints and returns either nothing or the response the caller senses,
//! which is what lets a page compose them without any of them reaching for the
//! project.
//!
//! # Painted text publishes no accessibility node
//!
//! Most of this draws through `ui.painter()`, which is what keeps a table of
//! two hundred rows affordable and is also why a painted cell is invisible to
//! a screen reader. Every painter here either publishes a node itself — see
//! [`page_empty_state`] — or is used by a caller that declares one on the
//! response it hands back. A painter that did neither would be legible only to
//! readers who can see it.

use super::*;

pub(super) fn section_title(
    ui: &mut Ui,
    title: &str,
    subtitle: &str,
    actions: impl FnOnce(&mut Ui),
) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(12, 0))
        .show(ui, |ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), 30.0),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.spacing_mut().item_spacing.x = 12.0;
                    ui.label(
                        RichText::new(title)
                            .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
                            .color(t.color.text),
                    );
                    ui.label(
                        RichText::new(subtitle)
                            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                            .color(t.color.text_faint),
                    );
                },
            );
        });
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .inner_margin(egui::Margin::symmetric(12, 2))
        .show(ui, |ui| {
            ui.allocate_ui_with_layout(
                egui::vec2(ui.available_width(), 32.0),
                Layout::left_to_right(Align::Center),
                actions,
            );
            ui.painter().hline(
                ui.max_rect().x_range(),
                ui.max_rect().bottom() - 0.5,
                Stroke::new(1.0, t.color.border),
            );
        });
}

pub(super) fn card(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::same(7))
        .show(ui, |ui| {
            // Cards are structural panes, not shrink-to-fit labels.
            ui.set_min_width(ui.available_width().max(1.0));
            content(ui);
        });
}

pub(super) fn detail_pane(
    ui: &mut Ui,
    title: &str,
    meta: Option<&str>,
    content: impl FnOnce(&mut Ui),
) {
    let t = Tokens::get(ui.ctx());
    egui::Frame::NONE
        .fill(t.color.bg_panel)
        .stroke(Stroke::new(1.0, t.color.border))
        .inner_margin(egui::Margin::ZERO)
        .show(ui, |ui| {
            egui::Frame::NONE
                .inner_margin(egui::Margin::symmetric(12, 6))
                .show(ui, |ui| card_title(ui, title, meta));
            egui::Frame::NONE
                .inner_margin(egui::Margin::symmetric(12, 8))
                .show(ui, content);
        });
}

pub(super) fn card_title(ui: &mut Ui, title: &str, meta: Option<&str>) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(title)
                .font(theme::sans(tokens::FS_0, FontWeight::SemiBold))
                .color(t.color.text_dim),
        );
        if let Some(meta) = meta {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                ui.label(
                    RichText::new(meta)
                        .small()
                        .monospace()
                        .color(t.color.text_faint),
                );
            });
        }
    });
    ui.separator();
}

pub(super) fn property(ui: &mut Ui, name: &str, value: &str, origin: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(egui::vec2(ui.available_width(), 22.0), Sense::hover());
    let name_width = rect.width() * 0.30;
    let value_width = rect.width() * 0.34;
    let origin_width = (rect.width() - name_width - value_width).max(1.0);
    let inset = 3.0;

    let name = elide(ui, name, (name_width - inset * 2.0).max(1.0), false);
    let value = elide(ui, value, (value_width - inset * 2.0).max(1.0), true);
    let origin = elide(ui, origin, (origin_width - inset * 2.0).max(1.0), false);
    ui.painter().text(
        egui::pos2(rect.left() + inset, rect.center().y),
        egui::Align2::LEFT_CENTER,
        name,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter().text(
        egui::pos2(rect.left() + name_width + inset, rect.center().y),
        egui::Align2::LEFT_CENTER,
        value,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text,
    );
    ui.painter().text(
        egui::pos2(rect.right() - inset, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        origin,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
}

pub(super) fn empty_state(ui: &mut Ui, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    ui.add_space(10.0);
    ui.vertical_centered(|ui| {
        ui.label(RichText::new(title).strong().color(t.color.text_dim));
        ui.label(RichText::new(detail).small().color(t.color.text_faint));
    });
    ui.add_space(10.0);
}

/// A page with nothing on it, and the reason there is nothing.
///
/// Every glyph here is painter text, which publishes no accessibility node at
/// all — so this used to be a whole page a screen reader found empty *and
/// silent*, including the pages that carry a refusal: a store that would not
/// open, and a catalog past the instant its publisher signed. The node is
/// declared outright, carrying both sentences whole. The painted detail elides
/// to the panel's width; what is announced does not, for the same reason
/// `announced_widget` next door announces the full sentence.
pub(super) fn page_empty_state(ui: &mut Ui, title: &str, detail: &str) {
    let t = Tokens::get(ui.ctx());
    let size = egui::vec2(
        ui.available_width().max(1.0),
        ui.available_height().max(180.0),
    );
    let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
    let announcement = format!("{title}. {detail}");
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), &announcement)
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Label);
        node.set_label(announcement.clone());
    });
    let panel = rect.shrink2(egui::vec2(12.0, 12.0));
    ui.painter().rect(
        panel,
        3.0,
        t.color.bg_inset,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    let accent = egui::Rect::from_center_size(
        egui::pos2(panel.center().x, panel.center().y - 34.0),
        egui::vec2(34.0, 3.0),
    );
    ui.painter().rect_filled(accent, 2.0, t.color.accent);
    ui.painter().text(
        egui::pos2(panel.center().x, panel.center().y - 12.0),
        egui::Align2::CENTER_CENTER,
        title,
        theme::sans(tokens::FS_1, FontWeight::SemiBold),
        t.color.text_dim,
    );
    ui.painter().text(
        egui::pos2(panel.center().x, panel.center().y + 14.0),
        egui::Align2::CENTER_CENTER,
        elide(ui, detail, (panel.width() - 48.0).max(1.0), false),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
}

pub(super) fn table_header(ui: &mut Ui, columns: &[(&str, f32)]) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), HEADER_H), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
    let mut x = rect.left() + 5.0;
    for (label, fraction) in columns {
        let width = rect.width() * fraction;
        ui.painter().text(
            egui::pos2(x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            *label,
            theme::sans(tokens::FS_0, FontWeight::SemiBold),
            t.color.text_faint,
        );
        x += width;
    }
}

pub(super) fn selectable_data_row(
    ui: &mut Ui,
    selected: bool,
    columns: &[(&str, f32, bool)],
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), ROW_H), Sense::click());
    if selected {
        ui.painter()
            .rect_filled(rect, 0.0, t.color.accent.linear_multiply(0.14));
        ui.painter().vline(
            rect.left(),
            rect.y_range(),
            Stroke::new(2.0, t.color.accent),
        );
    } else if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }
    paint_columns(ui, rect, columns);
    // The first column is the row's identifier in every caller, so it is the
    // name a screen reader should announce for the selection.
    let row_label = columns
        .first()
        .map(|(value, _, _)| (*value).to_owned())
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

pub(super) fn paint_columns(ui: &Ui, rect: egui::Rect, columns: &[(&str, f32, bool)]) {
    let t = Tokens::get(ui.ctx());
    let mut x = rect.left() + 5.0;
    for (value, fraction, mono) in columns {
        let width = rect.width() * fraction;
        let clipped = elide(ui, value, width - 9.0, *mono);
        ui.painter().text(
            egui::pos2(x, rect.center().y),
            egui::Align2::LEFT_CENTER,
            clipped,
            if *mono {
                theme::mono(tokens::FS_0, FontWeight::Regular)
            } else {
                theme::sans(tokens::FS_0, FontWeight::Regular)
            },
            t.color.text_dim,
        );
        x += width;
    }
}

/// Clip a cell's text to its column.
///
/// This module carried its own copy that dropped one character at a time and
/// laid the whole string out again after each, so a name that had to lose
/// thirty characters cost thirty text layouts — paid per cell, per row, on a
/// table the size of the corpus. The design system's owner bisects instead,
/// and cuts on grapheme boundaries rather than `char`s.
pub(super) fn elide(ui: &Ui, value: &str, max_width: f32, mono: bool) -> String {
    let font = if mono {
        theme::mono(tokens::FS_0, FontWeight::Regular)
    } else {
        theme::sans(tokens::FS_0, FontWeight::Regular)
    };
    crate::workbench::design_system::elide_text(ui, value, &font, max_width)
}

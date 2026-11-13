//! Searchable select — the design-system dropdown for a long, grouped list:
//! the same well as [`super::select`], opening a list with a search field at
//! its top and each group under its heading.
//!
//! A reader who knows the name types a few letters; one who does not reads
//! the groups. An option matches when its label, its detail or its group's
//! heading contains the query, ignoring case.

use egui::{Id, Sense, Stroke, Ui, WidgetInfo, WidgetType, vec2};

use super::select::paint_select_chevron;
use crate::theme::{self, FontWeight};
use crate::tokens::{self, Tokens};

/// Where a value starts inside the well and inside a list row.
const VALUE_INSET: f32 = 8.0;
/// The well's right end the chevron keeps clear of the value.
const CHEVRON_ROOM: f32 = 24.0;
/// Rows the list shows before it scrolls.
const VISIBLE_ROWS: usize = 12;
/// The least gap between an option's label and its detail.
const DETAIL_GAP: f32 = 16.0;
/// One option's row under a fine pointer: a dense list's line, as the
/// results table and the marker list set theirs, not a field's height.
const ROW_H: f32 = 22.0;

/// One heading and the options under it.
pub struct SelectGroup<'a> {
    pub heading: &'a str,
    /// Each option's label, set in the field's mono face, and a few words
    /// of detail set after it in the faint tone.
    pub options: Vec<(String, &'a str)>,
}

fn matches(query: &str, heading: &str, label: &str, detail: &str) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty()
        || [heading, label, detail]
            .iter()
            .any(|text| text.to_lowercase().contains(&query))
}

/// Render the well showing `selected` and, while open, the grouped list
/// filtered by `query`. Returns `(group, option)` when one is picked.
pub fn searchable_select(
    ui: &mut Ui,
    id_salt: &str,
    accessible_label: &str,
    selected: &str,
    groups: &[SelectGroup<'_>],
    query: &mut String,
    width: f32,
) -> Option<(usize, usize)> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let value_font = theme::field_mono();
    let (rect, response) = ui.allocate_exact_size(vec2(width, t.metrics.ctl_h), Sense::click());
    response.widget_info(|| {
        WidgetInfo::labeled(WidgetType::ComboBox, ui.is_enabled(), accessible_label)
    });
    let hover =
        ui.ctx()
            .animate_bool_with_time(response.id, response.hovered(), ui.style().animation_time);
    let shown = super::middle_elided(
        ui,
        selected,
        &value_font,
        c.text,
        (rect.width() - VALUE_INSET - CHEVRON_ROOM).max(1.0),
    );
    let painter = ui.painter();
    painter.rect(
        rect,
        t.radius,
        c.bg_inset,
        Stroke::new(1.0, theme::mix(c.border, c.border_strong, hover)),
        egui::StrokeKind::Inside,
    );
    painter.text(
        egui::pos2(rect.left() + VALUE_INSET, rect.center().y),
        egui::Align2::LEFT_CENTER,
        shown,
        value_font.clone(),
        c.text,
    );
    paint_select_chevron(
        painter,
        egui::pos2(rect.right() - 11.0, rect.center().y),
        c.text_dim,
    );
    theme::paint_focus_ring_outset(ui, &response, rect);

    let popup_id = Id::new(("rspice.searchable_select", id_salt));
    let search_id = popup_id.with("search");
    if response.clicked() {
        egui::Popup::toggle_id(ui.ctx(), popup_id);
        query.clear();
        ui.memory_mut(|memory| memory.request_focus(search_id));
    }
    let popup_open = egui::Popup::is_id_open(ui.ctx(), popup_id);
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_value(selected);
        node.set_expanded(popup_open);
    });

    let mut picked = None;
    egui::Popup::from_response(&response)
        .id(popup_id)
        .open_memory(None)
        .layout(egui::Layout::top_down_justified(egui::Align::LEFT))
        .align(egui::RectAlign::BOTTOM_START)
        .width(rect.width())
        // A click in the search field must not close the list.
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside)
        .show(|ui| {
            ui.set_min_width(rect.width());
            ui.spacing_mut().item_spacing.y = 0.0;
            let search = ui.add_sized(
                vec2(ui.available_width(), t.metrics.ctl_h),
                egui::TextEdit::singleline(query)
                    .id(search_id)
                    .font(theme::field_sans())
                    .hint_text("Search")
                    .vertical_align(egui::Align::Center)
                    .margin(egui::Margin::symmetric(8, 4)),
            );
            super::form::name_control(ui, &search, "Search");
            let row_height = if t.metrics.ctl_h >= tokens::TOUCH_TARGET {
                // Under a finger every row is a whole control tall.
                t.metrics.ctl_h
            } else {
                ROW_H
            };
            let first_match = groups.iter().enumerate().find_map(|(g, group)| {
                group
                    .options
                    .iter()
                    .position(|(label, detail)| matches(query, group.heading, label, detail))
                    .map(|o| (g, o))
            });
            // Enter in the search field picks the first match.
            if search.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter)) {
                picked = first_match;
            }
            ui.add_space(4.0);
            egui::ScrollArea::vertical()
                .id_salt(("rspice.searchable_select.options", id_salt))
                .max_height(row_height * VISIBLE_ROWS as f32)
                .show(ui, |ui| {
                    let mut any = false;
                    for (g, group) in groups.iter().enumerate() {
                        let shown: Vec<usize> = (0..group.options.len())
                            .filter(|&o| {
                                let (label, detail) = &group.options[o];
                                matches(query, group.heading, label, detail)
                            })
                            .collect();
                        if shown.is_empty() {
                            continue;
                        }
                        any = true;
                        heading(ui, group.heading);
                        for o in shown {
                            let (label, detail) = &group.options[o];
                            if option_row(ui, label, detail, *label == selected, row_height) {
                                picked = Some((g, o));
                            }
                        }
                    }
                    if !any {
                        heading(ui, "Nothing matches");
                    }
                });
        });
    if picked.is_some() {
        egui::Popup::close_id(ui.ctx(), popup_id);
    }
    picked
}

/// A group's heading, which a screen reader reads between its options as the
/// list shows it.
fn heading(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_0, FontWeight::Medium);
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), ROW_H), Sense::hover());
    response.widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), text));
    ui.painter().text(
        egui::pos2(rect.left() + VALUE_INSET, rect.bottom() - 5.0),
        egui::Align2::LEFT_BOTTOM,
        text,
        font,
        t.color.text_faint,
    );
}

fn option_row(ui: &mut Ui, label: &str, detail: &str, current: bool, height: f32) -> bool {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let (row, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::click());
    response.widget_info(|| {
        WidgetInfo::selected(WidgetType::SelectableLabel, ui.is_enabled(), current, label)
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::ListBoxOption);
        node.set_selected(current);
    });
    let painter = ui.painter();
    if response.hovered() {
        painter.rect_filled(row, t.radius, c.bg_hover);
    }
    let label_font = theme::field_mono();
    let label_width = painter
        .layout_no_wrap(label.to_owned(), label_font.clone(), c.text)
        .size()
        .x;
    painter.text(
        egui::pos2(row.left() + VALUE_INSET, row.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        label_font,
        if current { c.accent } else { c.text },
    );
    let detail_left = row.left() + VALUE_INSET + label_width + DETAIL_GAP;
    let detail_room = row.right() - VALUE_INSET - detail_left;
    if detail_room > 24.0 {
        let detail_font = theme::sans(tokens::FS_0, FontWeight::Regular);
        let shown = super::middle_elided(ui, detail, &detail_font, c.text_faint, detail_room);
        ui.painter().text(
            egui::pos2(row.right() - VALUE_INSET, row.center().y),
            egui::Align2::RIGHT_CENTER,
            shown,
            detail_font,
            c.text_faint,
        );
    }
    theme::paint_focus_ring(ui, &response, row);
    response
        .on_hover_cursor(egui::CursorIcon::PointingHand)
        .clicked()
}

#[cfg(test)]
mod tests {
    use super::matches;

    #[test]
    fn a_query_matches_a_label_its_detail_or_its_group_ignoring_case() {
        assert!(matches("", "Timing", "rise(x, …)", "rise time"));
        assert!(matches("RISE", "Timing", "rise(x, …)", "rise time"));
        assert!(matches(
            "slope",
            "Timing",
            "slewRate(x, …)",
            "slope of an edge"
        ));
        assert!(matches(
            "timing",
            "Timing",
            "cross(x, level, …)",
            "nth crossing"
        ));
        assert!(!matches("margin", "Timing", "rise(x, …)", "rise time"));
    }
}

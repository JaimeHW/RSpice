//! Select — the design-system dropdown: an inset UI-text well with a chevron,
//! opening a token-styled option list. Returns the picked index.

use egui::{Id, Sense, Shape, Stroke, Ui, WidgetInfo, WidgetType, vec2};

use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};

/// Render a select control showing `selected`; returns `Some(index)` when
/// an option is picked this frame.
pub fn select(
    ui: &mut Ui,
    id_salt: &str,
    accessible_label: &str,
    selected: &str,
    options: &[String],
    width: f32,
) -> Option<usize> {
    let t = Tokens::get(ui.ctx());
    let c = t.color;

    let (rect, response) = ui.allocate_exact_size(vec2(width, t.metrics.ctl_h), Sense::click());
    response.widget_info(|| {
        WidgetInfo::labeled(WidgetType::ComboBox, ui.is_enabled(), accessible_label)
    });
    let hover = ui
        .ctx()
        .animate_bool_with_time(response.id, response.hovered(), 0.16);
    let painter = ui.painter();
    painter.rect(
        rect,
        t.radius,
        c.bg_inset,
        Stroke::new(1.0, theme::mix(c.border, c.border_strong, hover)),
        egui::StrokeKind::Inside,
    );
    painter
        .with_clip_rect(egui::Rect::from_min_max(
            rect.left_top(),
            egui::pos2((rect.right() - 24.0).max(rect.left()), rect.bottom()),
        ))
        .text(
            egui::pos2(rect.left() + 8.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            selected,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            c.text,
        );
    let cx = rect.right() - 11.0;
    let cy = rect.center().y - 0.5;
    painter.add(Shape::line(
        vec![
            egui::pos2(cx - 3.5, cy - 1.5),
            egui::pos2(cx, cy + 2.5),
            egui::pos2(cx + 3.5, cy - 1.5),
        ],
        Stroke::new(1.4, c.text_dim),
    ));
    theme::paint_focus_ring(ui, &response, rect);

    let popup_id = ui.make_persistent_id(("rspice.select", id_salt));
    if response.clicked() {
        ui.memory_mut(|memory| memory.toggle_popup(popup_id));
    }

    let mut picked = keyboard_selection(ui, &response, popup_id, selected, options);
    let popup_open = ui.memory(|memory| memory.is_popup_open(popup_id));
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_value(selected);
        node.set_expanded(popup_open);
    });
    egui::popup_below_widget(
        ui,
        popup_id,
        &response,
        egui::PopupCloseBehavior::CloseOnClick,
        |ui| {
            ui.set_min_width(width.max(rect.width()));
            ui.spacing_mut().item_spacing.y = 0.0;
            let listbox = ui.scope(|ui| {
                for (index, option) in options.iter().enumerate() {
                    let is_current = option == selected;
                    // The owning surface may raise `ctl_h` for a coarse pointer;
                    // option rows must honor the same target rather than falling
                    // back to a mouse-only 24 px hit area.
                    let option_height = t.metrics.ctl_h.max(24.0);
                    let (row, row_response) = ui.allocate_exact_size(
                        vec2(ui.available_width(), option_height),
                        Sense::click(),
                    );
                    row_response.widget_info(|| {
                        WidgetInfo::selected(
                            WidgetType::SelectableLabel,
                            ui.is_enabled(),
                            is_current,
                            option,
                        )
                    });
                    ui.ctx().accesskit_node_builder(row_response.id, |node| {
                        node.set_role(egui::accesskit::Role::ListBoxOption);
                        node.set_selected(is_current);
                    });
                    let painter = ui.painter();
                    if row_response.hovered() {
                        painter.rect_filled(row, t.radius, c.bg_hover);
                    }
                    painter.text(
                        egui::pos2(row.left() + 8.0, row.center().y),
                        egui::Align2::LEFT_CENTER,
                        option,
                        theme::sans(tokens::FS_0, FontWeight::Regular),
                        if is_current { c.accent } else { c.text },
                    );
                    theme::paint_focus_ring(ui, &row_response, row);
                    if row_response
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        picked = Some(index);
                    }
                }
            });
            ui.ctx()
                .accesskit_node_builder(listbox.response.id, |node| {
                    node.set_role(egui::accesskit::Role::ListBox);
                    node.set_label(accessible_label);
                });
        },
    );
    picked
}

fn keyboard_selection(
    ui: &Ui,
    response: &egui::Response,
    state_id: Id,
    selected: &str,
    options: &[String],
) -> Option<usize> {
    if !response.has_focus() || options.is_empty() {
        return None;
    }
    let current = options
        .iter()
        .position(|option| option == selected)
        .unwrap_or_default();
    if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowDown)) {
        clear_typeahead(ui, state_id);
        return Some((current + 1).min(options.len() - 1));
    }
    if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp)) {
        clear_typeahead(ui, state_id);
        return Some(current.saturating_sub(1));
    }
    if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Home)) {
        clear_typeahead(ui, state_id);
        return Some(0);
    }
    if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::End)) {
        clear_typeahead(ui, state_id);
        return Some(options.len() - 1);
    }
    if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::PageDown)) {
        clear_typeahead(ui, state_id);
        return Some((current + 10).min(options.len() - 1));
    }
    if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::PageUp)) {
        clear_typeahead(ui, state_id);
        return Some(current.saturating_sub(10));
    }

    let (typed, now) = ui.input(|input| {
        let typed = input
            .events
            .iter()
            .filter_map(|event| match event {
                egui::Event::Text(text) => Some(text.as_str()),
                _ => None,
            })
            .collect::<String>();
        (typed, input.time)
    });
    let typed = typed.trim();
    if typed.is_empty() || typed.chars().any(char::is_control) {
        return None;
    }

    let query = ui.data_mut(|data| {
        let state = data.get_temp_mut_or_default::<SelectTypeahead>(state_id);
        if now - state.last_input_time > TYPEAHEAD_RESET_SECONDS {
            state.query.clear();
        }
        state.last_input_time = now;
        state.query.push_str(&typed.to_lowercase());
        if state.query.len() > 1
            && state
                .query
                .chars()
                .all(|character| character == state.query.chars().next().unwrap_or_default())
        {
            state.query.truncate(1);
        }
        state.query.clone()
    });
    matching_option(options, current, &query)
}

const TYPEAHEAD_RESET_SECONDS: f64 = 0.75;

#[derive(Clone, Debug, Default)]
struct SelectTypeahead {
    query: String,
    last_input_time: f64,
}

fn clear_typeahead(ui: &Ui, state_id: Id) {
    ui.data_mut(|data| data.remove::<SelectTypeahead>(state_id));
}

fn matching_option(options: &[String], current: usize, query: &str) -> Option<usize> {
    let query = query.to_lowercase();
    (1..=options.len())
        .map(|offset| (current + offset) % options.len())
        .find(|index| options[*index].to_lowercase().starts_with(&query))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options() -> Vec<String> {
        ["Browser", "Comfortable", "Compact", "System"]
            .into_iter()
            .map(str::to_owned)
            .collect()
    }

    #[test]
    fn typeahead_search_wraps_after_the_current_option() {
        assert_eq!(matching_option(&options(), 3, "c"), Some(1));
        assert_eq!(matching_option(&options(), 1, "c"), Some(2));
    }

    #[test]
    fn typeahead_search_is_case_insensitive_and_fail_closed() {
        assert_eq!(matching_option(&options(), 0, "SyS"), Some(3));
        assert_eq!(matching_option(&options(), 0, "missing"), None);
    }
}

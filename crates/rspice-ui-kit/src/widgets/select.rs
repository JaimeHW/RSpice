//! Select — the design-system dropdown: an inset UI-text well with a chevron,
//! opening a token-styled option list. Returns the picked index.

use egui::{Id, Sense, Shape, Stroke, Ui, WidgetInfo, WidgetType, vec2};

use crate::theme;
use crate::tokens::Tokens;

/// Where a value starts inside the well and inside a drop-down row.
const VALUE_INSET: f32 = 8.0;
/// The well's right end the chevron keeps clear of the value.
const CHEVRON_ROOM: f32 = 24.0;
/// Options a drop-down shows before its list scrolls.
const SELECT_VISIBLE_OPTIONS: usize = 12;

pub struct SelectOutput {
    pub picked: Option<usize>,
    pub response: egui::Response,
}

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
    select_with_response(ui, id_salt, accessible_label, selected, options, width).picked
}

/// Select control with individually unavailable choices. Disabled options
/// remain visible to preserve the authored domain and expose a precise reason,
/// but cannot be selected by pointer or keyboard.
pub fn select_with_disabled(
    ui: &mut Ui,
    id_salt: &str,
    accessible_label: &str,
    selected: &str,
    options: &[String],
    disabled: &[(usize, &'static str)],
    width: f32,
) -> Option<usize> {
    select_with_font(
        ui,
        id_salt,
        accessible_label,
        selected,
        options,
        width,
        false,
        disabled,
    )
    .picked
}

pub fn select_with_response(
    ui: &mut Ui,
    id_salt: &str,
    accessible_label: &str,
    selected: &str,
    options: &[String],
    width: f32,
) -> SelectOutput {
    select_with_font(
        ui,
        id_salt,
        accessible_label,
        selected,
        options,
        width,
        false,
        &[],
    )
}

/// Mockup-authored engineering-field select. Field values and options use the
/// same mono face as adjacent exact-value inputs without changing toolbar or
/// navigation selects globally.
pub fn select_mono_with_response(
    ui: &mut Ui,
    id_salt: &str,
    accessible_label: &str,
    selected: &str,
    options: &[String],
    width: f32,
) -> SelectOutput {
    select_with_font(
        ui,
        id_salt,
        accessible_label,
        selected,
        options,
        width,
        true,
        &[],
    )
}

// The caller independently supplies the control identity, content, style,
// width and unavailable choices. Keep the existing rendering contract.
#[expect(clippy::too_many_arguments)]
fn select_with_font(
    ui: &mut Ui,
    id_salt: &str,
    accessible_label: &str,
    selected: &str,
    options: &[String],
    width: f32,
    mono: bool,
    disabled: &[(usize, &'static str)],
) -> SelectOutput {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    let value_font = if mono {
        theme::field_mono()
    } else {
        theme::field_sans()
    };

    let width = width.max(if t.metrics.ctl_h >= 44.0 { 44.0 } else { 0.0 });
    let (rect, response) = ui.allocate_exact_size(vec2(width, t.metrics.ctl_h), Sense::click());
    response.widget_info(|| {
        WidgetInfo::labeled(WidgetType::ComboBox, ui.is_enabled(), accessible_label)
    });
    let hover =
        ui.ctx()
            .animate_bool_with_time(response.id, response.hovered(), ui.style().animation_time);
    // A value wider than the well is shortened from the middle and named
    // whole on hover: cut at the chevron, it read as complete.
    let shown = super::middle_elided(
        ui,
        selected,
        &value_font,
        c.text,
        (rect.width() - VALUE_INSET - CHEVRON_ROOM).max(1.0),
    );
    let response = if shown == selected {
        response
    } else {
        response.on_hover_text(selected)
    };
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
            egui::pos2(
                (rect.right() - CHEVRON_ROOM).max(rect.left()),
                rect.bottom(),
            ),
        ))
        .text(
            egui::pos2(rect.left() + VALUE_INSET, rect.center().y),
            egui::Align2::LEFT_CENTER,
            shown,
            value_font.clone(),
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
    theme::paint_focus_ring_outset(ui, &response, rect);

    let popup_id = ui.make_persistent_id(("rspice.select", id_salt));
    if response.clicked() {
        egui::Popup::toggle_id(ui.ctx(), popup_id);
    }

    let mut picked = keyboard_selection(ui, &response, popup_id, selected, options, disabled);
    let popup_open = egui::Popup::is_id_open(ui.ctx(), popup_id);
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_value(selected);
        node.set_expanded(popup_open);
    });
    // Spelled out rather than left on `popup_below_widget`, which egui 0.34
    // deprecated: this is that helper's body verbatim -- justified top-down
    // layout, memory-tracked open state, bottom-start alignment, and the
    // trigger's own width -- so the drop-down keeps its exact geometry.
    egui::Popup::from_response(&response)
        .id(popup_id)
        .open_memory(None)
        .layout(egui::Layout::top_down_justified(egui::Align::LEFT))
        .align(egui::RectAlign::BOTTOM_START)
        .width(response.rect.width())
        .close_behavior(egui::PopupCloseBehavior::CloseOnClick)
        .show(|ui| {
            ui.set_min_width(ui.available_width());
            ui.set_min_width(width.max(rect.width()));
            ui.spacing_mut().item_spacing.y = 0.0;
            // A long list scrolls inside the drop-down rather than running
            // the drop-down off the screen; a short one is laid out as is.
            let option_rows = t.metrics.ctl_h.max(24.0) * SELECT_VISIBLE_OPTIONS as f32;
            let listbox = egui::ScrollArea::vertical()
                .id_salt(("rspice.select.options", id_salt))
                .max_height(option_rows)
                .show(ui, |ui| {
                    ui.scope(|ui| {
                        for (index, option) in options.iter().enumerate() {
                            let is_current = option == selected;
                            let unavailable = disabled.iter().find_map(|(candidate, reason)| {
                                (*candidate == index).then_some(*reason)
                            });
                            // The owning surface may raise `ctl_h` for a coarse pointer;
                            // option rows must honor the same target rather than falling
                            // back to a mouse-only 24 px hit area.
                            let option_height = t.metrics.ctl_h.max(24.0);
                            let (row, row_response) = ui.allocate_exact_size(
                                vec2(ui.available_width(), option_height),
                                if unavailable.is_some() {
                                    Sense::hover()
                                } else {
                                    Sense::click()
                                },
                            );
                            let shown = super::middle_elided(
                                ui,
                                option,
                                &value_font,
                                c.text,
                                (row.width() - 2.0 * VALUE_INSET).max(1.0),
                            );
                            let row_response = if let Some(reason) = unavailable {
                                row_response.on_hover_text(reason)
                            } else if shown != *option {
                                row_response.on_hover_text(option)
                            } else {
                                row_response
                            };
                            row_response.widget_info(|| {
                                WidgetInfo::selected(
                                    WidgetType::SelectableLabel,
                                    ui.is_enabled() && unavailable.is_none(),
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
                                egui::pos2(row.left() + VALUE_INSET, row.center().y),
                                egui::Align2::LEFT_CENTER,
                                shown,
                                value_font.clone(),
                                if unavailable.is_some() {
                                    c.text_faint
                                } else if is_current {
                                    c.accent
                                } else {
                                    c.text
                                },
                            );
                            theme::paint_focus_ring(ui, &row_response, row);
                            if unavailable.is_none()
                                && row_response
                                    .on_hover_cursor(egui::CursorIcon::PointingHand)
                                    .clicked()
                            {
                                picked = Some(index);
                            }
                        }
                    })
                })
                .inner;
            ui.ctx()
                .accesskit_node_builder(listbox.response.id, |node| {
                    node.set_role(egui::accesskit::Role::ListBox);
                    node.set_label(accessible_label);
                });
        });
    SelectOutput { picked, response }
}

fn keyboard_selection(
    ui: &Ui,
    response: &egui::Response,
    state_id: Id,
    selected: &str,
    options: &[String],
    disabled: &[(usize, &'static str)],
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
        return next_available_index(options.len(), current, true, disabled);
    }
    if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::ArrowUp)) {
        clear_typeahead(ui, state_id);
        return next_available_index(options.len(), current, false, disabled);
    }
    if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::Home)) {
        clear_typeahead(ui, state_id);
        return (0..options.len()).find(|index| option_is_available(*index, disabled));
    }
    if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::End)) {
        clear_typeahead(ui, state_id);
        return (0..options.len())
            .rev()
            .find(|index| option_is_available(*index, disabled));
    }
    if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::PageDown)) {
        clear_typeahead(ui, state_id);
        let target = (current + 10).min(options.len() - 1);
        return (target..options.len())
            .find(|index| option_is_available(*index, disabled))
            .or_else(|| {
                (current + 1..target)
                    .rev()
                    .find(|index| option_is_available(*index, disabled))
            });
    }
    if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, egui::Key::PageUp)) {
        clear_typeahead(ui, state_id);
        let target = current.saturating_sub(10);
        return (0..=target)
            .rev()
            .find(|index| option_is_available(*index, disabled))
            .or_else(|| (target + 1..current).find(|index| option_is_available(*index, disabled)));
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
    matching_option(options, current, &query, disabled)
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

fn option_is_available(index: usize, disabled: &[(usize, &'static str)]) -> bool {
    !disabled.iter().any(|(candidate, _)| *candidate == index)
}

fn next_available_index(
    len: usize,
    current: usize,
    forward: bool,
    disabled: &[(usize, &'static str)],
) -> Option<usize> {
    if forward {
        (current.saturating_add(1)..len).find(|index| option_is_available(*index, disabled))
    } else {
        (0..current)
            .rev()
            .find(|index| option_is_available(*index, disabled))
    }
}

fn matching_option(
    options: &[String],
    current: usize,
    query: &str,
    disabled: &[(usize, &'static str)],
) -> Option<usize> {
    let query = query.to_lowercase();
    (1..=options.len())
        .map(|offset| (current + offset) % options.len())
        .find(|index| {
            option_is_available(*index, disabled)
                && options[*index].to_lowercase().starts_with(&query)
        })
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

    /// A value wider than its well is shortened from the middle, both ends
    /// kept, and painted inside the well rather than cut at the chevron.
    #[test]
    fn a_long_value_is_shortened_from_the_middle_to_fit_its_well() {
        let ctx = egui::Context::default();
        crate::Theme::default().apply(&ctx);
        let long = "Corner characterization \u{00b7} revision 1";
        let options = vec![long.to_owned()];
        let mut painted = Vec::new();
        for _ in 0..2 {
            let output = ctx.run_ui(egui::RawInput::default(), |ui| {
                let _ = select(ui, "fit", "Plan", long, &options, 140.0);
            });
            painted = output
                .shapes
                .iter()
                .filter_map(|clipped| match &clipped.shape {
                    egui::Shape::Text(text) => {
                        Some((text.galley.text().to_owned(), text.galley.size().x))
                    }
                    _ => None,
                })
                .collect();
        }
        let (shown, width) = painted
            .iter()
            .find(|(text, _)| text.starts_with("Corner"))
            .cloned()
            .unwrap_or_else(|| panic!("the value is painted: {painted:?}"));
        assert!(
            shown.starts_with("Corner") && shown.ends_with(" 1") && shown.contains('\u{2026}'),
            "{shown:?}"
        );
        assert!(
            width <= 140.0 - VALUE_INSET - CHEVRON_ROOM,
            "{shown:?} is {width} wide"
        );
    }

    /// A long list scrolls inside its drop-down: the drop-down never grows
    /// past twelve options, so it cannot run off the screen.
    #[test]
    fn a_long_list_scrolls_inside_its_drop_down() {
        let ctx = egui::Context::default();
        crate::Theme::default().apply(&ctx);
        let options = (0..40)
            .map(|index| format!("Option {index}"))
            .collect::<Vec<_>>();
        let mut popup = None;
        for _ in 0..3 {
            let _ = ctx.run_ui(
                egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(
                        egui::Pos2::ZERO,
                        vec2(800.0, 2000.0),
                    )),
                    ..Default::default()
                },
                |ui| {
                    let id = ui.make_persistent_id(("rspice.select", "long"));
                    egui::Popup::open_id(ui.ctx(), id);
                    popup = Some(id);
                    let _ = select(ui, "long", "Long", "Option 0", &options, 200.0);
                },
            );
        }
        let rect = ctx
            .memory(|memory| memory.area_rect(popup.expect("the frame ran")))
            .expect("the drop-down is shown");
        let row = Tokens::get(&ctx).metrics.ctl_h.max(24.0);
        assert!(
            rect.height() <= row * (SELECT_VISIBLE_OPTIONS as f32 + 1.0),
            "the drop-down is {} tall for {} options",
            rect.height(),
            options.len()
        );
    }

    #[test]
    fn typeahead_search_wraps_after_the_current_option() {
        assert_eq!(matching_option(&options(), 3, "c", &[]), Some(1));
        assert_eq!(matching_option(&options(), 1, "c", &[]), Some(2));
    }

    #[test]
    fn typeahead_search_is_case_insensitive_and_fail_closed() {
        assert_eq!(matching_option(&options(), 0, "SyS", &[]), Some(3));
        assert_eq!(matching_option(&options(), 0, "missing", &[]), None);
    }

    #[test]
    fn keyboard_and_typeahead_navigation_skip_unavailable_choices() {
        let disabled = [(1, "requires retained evidence")];
        assert_eq!(next_available_index(4, 0, true, &disabled), Some(2));
        assert_eq!(next_available_index(4, 2, false, &disabled), Some(0));
        assert_eq!(matching_option(&options(), 0, "c", &disabled), Some(2));
        let all_c_choices_disabled = [
            (1, "requires retained evidence"),
            (2, "requires retained evidence"),
        ];
        assert_eq!(
            matching_option(&options(), 0, "c", &all_c_choices_disabled),
            None
        );
    }
}

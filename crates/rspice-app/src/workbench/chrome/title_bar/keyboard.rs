//! The menus' keyboard: F10 and the arrow keys across the bar, the arrow
//! keys, Home, End, Escape and Tab inside an open menu, and type-to-find on
//! both. Each menu records its rows while it is drawn, so the keys move
//! among the rows the reader can see, greyed ones included: a greyed row
//! takes the focus so it can say why it is grey, and only refuses to run.

use super::*;

pub(super) const MENU_RENDER_STATE_ID: &str = "workbench.title_menu.render_state";
const MENU_RETURN_FOCUS_ID: &str = "workbench.title_menu.return_focus";
const MENU_TYPEAHEAD_ID: &str = "workbench.title_menu.typeahead";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum InitialMenuFocus {
    First,
    Last,
}

#[derive(Debug, Clone)]
pub(super) struct MenuRowRecord {
    pub(super) id: Id,
    pub(super) label: String,
    pub(super) response: Response,
}

#[derive(Debug, Clone, Default)]
pub(super) struct MenuRenderState {
    pub(super) rows: Vec<MenuRowRecord>,
    pub(super) initial_focus: Option<InitialMenuFocus>,
    pub(super) initial_focus_claimed: bool,
}

#[derive(Debug, Clone, Default)]
struct MenuTypeaheadState {
    value: String,
    last_input_time: f64,
}

pub(super) fn begin_menu_render(ctx: &Context, popup_id: Id) {
    let initial_focus = ctx.data_mut(|data| {
        let id = popup_id.with("initial_focus");
        let focus = data.get_temp::<InitialMenuFocus>(id);
        data.remove::<InitialMenuFocus>(id);
        focus
    });
    ctx.data_mut(|data| {
        data.insert_temp(
            Id::new(MENU_RENDER_STATE_ID),
            MenuRenderState {
                initial_focus,
                ..MenuRenderState::default()
            },
        );
    });
}

pub(super) fn record_menu_row(ctx: &Context, response: &Response, label: &str) {
    let request_row_focus = ctx.data_mut(|data| {
        let id = Id::new(MENU_RENDER_STATE_ID);
        let Some(mut state) = data.get_temp::<MenuRenderState>(id) else {
            return false;
        };
        let request_row_focus = match state.initial_focus {
            Some(InitialMenuFocus::First) if !state.initial_focus_claimed => {
                state.initial_focus_claimed = true;
                true
            }
            Some(InitialMenuFocus::Last) => {
                // Every row requests focus while rendering; the last row
                // therefore owns the final request.
                state.initial_focus_claimed = true;
                true
            }
            Some(InitialMenuFocus::First) | None => false,
        };
        state.rows.push(MenuRowRecord {
            id: response.id,
            label: label.to_owned(),
            response: response.clone(),
        });
        data.insert_temp(id, state);
        request_row_focus
    });
    // `ctx.data_mut` is backed by egui's Memory lock. Requesting focus from
    // inside that closure recursively acquires the same lock and deadlocks on
    // native and WASM builds; apply the recorded focus request after release.
    if request_row_focus {
        response.request_focus();
        response.scroll_to_me(Some(Align::Center));
    }
}

pub(super) fn finish_menu_render(ctx: &Context) -> MenuRenderState {
    ctx.data_mut(|data| {
        data.remove_temp::<MenuRenderState>(Id::new(MENU_RENDER_STATE_ID))
            .unwrap_or_default()
    })
}

fn request_focus(ctx: &Context, id: Id) {
    ctx.memory_mut(|memory| memory.request_focus(id));
}

/// Keep egui's own focus moves off the keys a menu row answers itself: an
/// arrow would hop the focus on from the row the menu just moved it to, to
/// whatever widget lies that way, under the menu or beside it, and Escape
/// would drop the focus before the menu could hand it back.
pub(super) fn claim_menu_keys(ctx: &Context, id: Id) {
    ctx.memory_mut(|memory| {
        memory.set_focus_lock_filter(
            id,
            egui::EventFilter {
                tab: false,
                horizontal_arrows: true,
                vertical_arrows: true,
                escape: true,
            },
        );
    });
}

pub(super) fn handle_popup_keyboard(
    ctx: &Context,
    popup_id: Id,
    trigger: &Response,
    rows: &[MenuRowRecord],
) {
    let focused = ctx.memory(|memory| memory.focused());
    let focused_index = focused.and_then(|id| rows.iter().position(|row| row.id == id));
    if let Some(index) = focused_index {
        claim_menu_keys(ctx, rows[index].id);
    }

    if step_menu_row_focus(ctx, rows, focused_index) {
        return;
    }

    if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape)) {
        Popup::close_id(ctx, popup_id);
        trigger.request_focus();
        return;
    }
    if ctx.input(|input| input.key_pressed(Key::Tab)) {
        Popup::close_id(ctx, popup_id);
        return;
    }

    if let Some(index) = focused_index {
        typeahead_menu_row_focus(ctx, rows, index);
    }
}

/// Up, Down, Home and End among a menu's rows. Returns whether one of them
/// was pressed.
pub(super) fn step_menu_row_focus(
    ctx: &Context,
    rows: &[MenuRowRecord],
    focused_index: Option<usize>,
) -> bool {
    if rows.is_empty() {
        return false;
    }
    let destination = if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::ArrowDown)) {
        Some(focused_index.map_or(0, |index| (index + 1) % rows.len()))
    } else if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::ArrowUp)) {
        Some(focused_index.map_or(rows.len() - 1, |index| {
            (index + rows.len() - 1) % rows.len()
        }))
    } else if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Home)) {
        Some(0)
    } else if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::End)) {
        Some(rows.len() - 1)
    } else {
        None
    };
    let Some(index) = destination else {
        return false;
    };
    rows[index].response.request_focus();
    rows[index].response.scroll_to_me(Some(Align::Center));
    // egui read the same arrow when the pass began and would move the focus
    // on from the row just chosen at its end; a row that has not held the
    // focus for a pass yet cannot claim the arrows, so the move is cancelled.
    ctx.memory_mut(|memory| memory.move_focus(egui::FocusDirection::None));
    true
}

/// Typing a row's first letters moves the focus to it.
pub(super) fn typeahead_menu_row_focus(
    ctx: &Context,
    rows: &[MenuRowRecord],
    focused_index: usize,
) {
    let Some(query) = menu_typeahead_query(ctx) else {
        return;
    };
    if let Some(row) = rows
        .iter()
        .cycle()
        .skip(focused_index + 1)
        .take(rows.len())
        .find(|row| row.label.to_lowercase().starts_with(&query))
    {
        row.response.request_focus();
        row.response.scroll_to_me(Some(Align::Center));
    }
}

pub(super) fn handle_menu_bar_keyboard(
    ctx: &Context,
    triggers: &[MenuTriggerResponse],
    keyboard_enabled: bool,
) {
    if !keyboard_enabled || triggers.is_empty() {
        return;
    }
    let focused = ctx.memory(|memory| memory.focused());
    let focused_trigger = focused.and_then(|id| {
        triggers
            .iter()
            .position(|trigger| trigger.response.id == id)
    });
    let open_trigger = triggers
        .iter()
        .position(|trigger| Popup::is_id_open(ctx, trigger.popup_id));

    if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::F10)) {
        if focused_trigger.is_some() || open_trigger.is_some() {
            Popup::close_all(ctx);
            let return_focus = ctx.data_mut(|data| {
                data.get_temp::<Option<Id>>(Id::new(MENU_RETURN_FOCUS_ID))
                    .flatten()
            });
            if let Some(id) = return_focus {
                request_focus(ctx, id);
            }
        } else {
            ctx.data_mut(|data| {
                data.insert_temp(Id::new(MENU_RETURN_FOCUS_ID), focused);
            });
            triggers[0].response.request_focus();
        }
        return;
    }

    if let Some(open_index) = open_trigger {
        let direction =
            if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::ArrowRight)) {
                Some(1_isize)
            } else if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::ArrowLeft)) {
                Some(-1_isize)
            } else {
                None
            };
        if let Some(direction) = direction {
            let next =
                (open_index as isize + direction).rem_euclid(triggers.len() as isize) as usize;
            Popup::open_id(ctx, triggers[next].popup_id);
            ctx.data_mut(|data| {
                data.insert_temp(
                    triggers[next].popup_id.with("initial_focus"),
                    if direction < 0 {
                        InitialMenuFocus::Last
                    } else {
                        InitialMenuFocus::First
                    },
                );
            });
            triggers[next].response.request_focus();
            return;
        }
    }

    let Some(current) = focused_trigger else {
        return;
    };
    let destination = if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::ArrowRight))
    {
        Some((current + 1) % triggers.len())
    } else if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::ArrowLeft)) {
        Some((current + triggers.len() - 1) % triggers.len())
    } else if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Home)) {
        Some(0)
    } else if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::End)) {
        Some(triggers.len() - 1)
    } else {
        None
    };
    if let Some(index) = destination {
        triggers[index].response.request_focus();
        return;
    }

    if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape)) {
        let return_focus = ctx.data_mut(|data| {
            data.get_temp::<Option<Id>>(Id::new(MENU_RETURN_FOCUS_ID))
                .flatten()
        });
        if let Some(id) = return_focus {
            request_focus(ctx, id);
        } else {
            ctx.memory_mut(|memory| {
                memory.surrender_focus(focused.unwrap_or(triggers[current].response.id))
            });
        }
        return;
    }

    if let Some(query) = menu_typeahead_query(ctx)
        && let Some(index) = triggers
            .iter()
            .enumerate()
            .cycle()
            .skip(current + 1)
            .take(triggers.len())
            .find_map(|(index, trigger)| {
                trigger
                    .key
                    .label()
                    .to_lowercase()
                    .starts_with(&query)
                    .then_some(index)
            })
    {
        triggers[index].response.request_focus();
    }
}

fn menu_typeahead_query(ctx: &Context) -> Option<String> {
    let typed = ctx.input(|input| {
        input.events.iter().rev().find_map(|event| match event {
            egui::Event::Text(text) if text.chars().any(|character| !character.is_whitespace()) => {
                Some(text.to_lowercase())
            }
            _ => None,
        })
    })?;
    let now = ctx.input(|input| input.time);
    ctx.data_mut(|data| {
        let id = Id::new(MENU_TYPEAHEAD_ID);
        let mut state = data.get_temp::<MenuTypeaheadState>(id).unwrap_or_default();
        if now - state.last_input_time > 0.7 {
            state.value.clear();
        }
        state.value.push_str(&typed);
        state.last_input_time = now;
        let repeated = state.value.chars().all(|character| {
            state
                .value
                .chars()
                .next()
                .is_some_and(|first| character == first)
        });
        let query = if repeated {
            state
                .value
                .chars()
                .next()
                .map(|character| character.to_string())
        } else {
            Some(state.value.clone())
        };
        data.insert_temp(id, state);
        query
    })
}

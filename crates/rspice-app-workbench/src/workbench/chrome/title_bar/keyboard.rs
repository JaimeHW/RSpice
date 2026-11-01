//! The menus' keyboard: F10 and the arrow keys across the bar, the arrow
//! keys, Home, End, Escape and Tab inside an open menu, and type-to-find on
//! both. Each menu records its rows while it is drawn, so the keys move
//! among the rows the reader can see, greyed ones included: a greyed row
//! takes the focus so it can say why it is grey, and only refuses to run.
//!
//! On the Windows and Linux desktop the menus also answer access keys, the
//! way those desktops' own menus do: a tap of Alt takes the focus to the bar
//! and underlines a letter in every menu's name and row, Alt and a menu's
//! letter opens it from anywhere, and a row's letter chooses the row while
//! its menu is open.

use super::*;

pub(super) const MENU_RENDER_STATE_ID: &str = "workbench.title_menu.render_state";
const MENU_RETURN_FOCUS_ID: &str = "workbench.title_menu.return_focus";
const MENU_TYPEAHEAD_ID: &str = "workbench.title_menu.typeahead";
const ACCESS_CUES_ID: &str = "workbench.title_menu.access_cues";
const ALT_TAP_ID: &str = "workbench.title_menu.alt_tap";
/// Asks a submenu's row to open its submenu when it is next drawn.
pub(super) const SUBMENU_OPEN_REQUEST: &str = "open_request";

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
    /// The row's access key, if it has one.
    pub(super) access: Option<char>,
    pub(super) activation: RowActivation,
}

/// What typing a row's access key does.
#[derive(Debug, Clone)]
pub(super) enum RowActivation {
    /// Chooses the row.
    Choose(MenuChoice),
    /// The row is greyed: it takes the focus, the way the arrows leave it.
    Greyed,
    /// The row opens a submenu, with its first row focused.
    OpenSubmenu,
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

pub(super) fn record_menu_row(
    ctx: &Context,
    response: &Response,
    label: &str,
    access: Option<char>,
    activation: RowActivation,
) {
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
            access,
            activation,
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

/// Keys inside an open menu. Returns the row its access key chose, which
/// the caller closes the menu on and runs.
pub(super) fn handle_popup_keyboard(
    ctx: &Context,
    popup_id: Id,
    trigger: &Response,
    rows: &[MenuRowRecord],
) -> Option<MenuChoice> {
    let focused = ctx.memory(|memory| memory.focused());
    let focused_index = focused.and_then(|id| rows.iter().position(|row| row.id == id));
    if let Some(index) = focused_index {
        claim_menu_keys(ctx, rows[index].id);
    }

    if step_menu_row_focus(ctx, rows, focused_index) {
        return None;
    }

    if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::Escape)) {
        Popup::close_id(ctx, popup_id);
        trigger.request_focus();
        return None;
    }
    if ctx.input(|input| input.key_pressed(Key::Tab)) {
        Popup::close_id(ctx, popup_id);
        return None;
    }

    if let Some(typed) = take_row_access(ctx, rows, focused_index) {
        return typed;
    }
    if let Some(index) = focused_index {
        typeahead_menu_row_focus(ctx, rows, index);
    }
    None
}

/// Whether the menus answer access keys in the window `ctx` draws.
pub(super) fn access_keys_apply_here(ctx: &Context) -> bool {
    application_menus::access_keys_apply(
        crate::workbench::app_state::session::shortcuts::runtime_command_platform(ctx),
        ctx.os(),
    )
}

/// Whether the access keys are underlined: while Alt alone is held, and
/// from a tap of Alt, an Alt chord or F10 until the keyboard leaves the bar.
pub(super) fn access_cues_shown(ctx: &Context) -> bool {
    access_keys_apply_here(ctx)
        && (ctx.input(|input| alt_alone(input.modifiers))
            || ctx
                .data(|data| data.get_temp::<bool>(Id::new(ACCESS_CUES_ID)))
                .unwrap_or(false))
}

fn set_access_cues(ctx: &Context, shown: bool) {
    let id = Id::new(ACCESS_CUES_ID);
    let was = ctx.data(|data| data.get_temp::<bool>(id)).unwrap_or(false);
    if was != shown {
        ctx.data_mut(|data| data.insert_temp(id, shown));
        // The bar and the rows have been painted for this pass already.
        ctx.request_repaint();
    }
}

const fn alt_alone(modifiers: Modifiers) -> bool {
    modifiers.alt && !modifiers.ctrl && !modifiers.shift && !modifiers.command && !modifiers.mac_cmd
}

/// The key that types `access`, a letter or a digit.
fn access_key_of(access: char) -> Option<Key> {
    Key::from_name(&access.to_ascii_uppercase().to_string())
}

/// Take the first press, with Alt alone if `alt` and with neither Alt, Ctrl
/// nor Command otherwise, of a key `wanted` accepts, with the letter it
/// typed, and return what `wanted` made of it.
fn take_access_press<T>(ctx: &Context, alt: bool, wanted: impl Fn(Key) -> Option<T>) -> Option<T> {
    ctx.input_mut(|input| {
        let (index, key, found) = input.events.iter().enumerate().find_map(|(index, event)| {
            let egui::Event::Key {
                key,
                pressed: true,
                modifiers,
                ..
            } = event
            else {
                return None;
            };
            let modifiers_fit = if alt {
                alt_alone(*modifiers)
            } else {
                !modifiers.alt && !modifiers.ctrl && !modifiers.command && !modifiers.mac_cmd
            };
            if !modifiers_fit {
                return None;
            }
            wanted(*key).map(|found| (index, *key, found))
        })?;
        input.events.remove(index);
        let typed = key.name().to_lowercase();
        if let Some(text) = input.events.iter().position(
            |event| matches!(event, egui::Event::Text(text) if text.to_lowercase() == typed),
        ) {
            input.events.remove(text);
        }
        Some(found)
    })
}

/// A row's access key, typed while its menu is open: an available command's
/// row is chosen (`Some(Some(choice))`), a submenu's row opens its submenu
/// and a greyed row takes the focus (`Some(None)`). Two rows sharing a key
/// would take the focus in turn. `None` when no row's key was typed.
pub(super) fn take_row_access(
    ctx: &Context,
    rows: &[MenuRowRecord],
    focused_index: Option<usize>,
) -> Option<Option<MenuChoice>> {
    if rows.is_empty() || !access_keys_apply_here(ctx) {
        return None;
    }
    let keys: Vec<Option<Key>> = rows
        .iter()
        .map(|row| row.access.and_then(access_key_of))
        .collect();
    let typed = take_access_press(ctx, false, |key| keys.contains(&Some(key)).then_some(key))?;
    let answering: Vec<usize> = (0..rows.len())
        .filter(|&index| keys[index] == Some(typed))
        .collect();
    let after = focused_index.map_or(0, |index| index + 1);
    let index = answering
        .iter()
        .copied()
        .find(|&index| index >= after)
        .unwrap_or(answering[0]);
    let row = &rows[index];
    Some(match &row.activation {
        RowActivation::Choose(choice) if answering.len() == 1 => Some(choice.clone()),
        RowActivation::Choose(_) | RowActivation::Greyed => {
            row.response.request_focus();
            row.response.scroll_to_me(Some(Align::Center));
            None
        }
        RowActivation::OpenSubmenu => {
            row.response.request_focus();
            ctx.data_mut(|data| data.insert_temp(row.id.with(SUBMENU_OPEN_REQUEST), true));
            None
        }
    })
}

/// Keep a submenu's letters to it while one of its rows has the focus: the
/// menu it opens from would otherwise choose its own row of that letter.
pub(super) fn hold_access_letters(ctx: &Context) {
    if !access_keys_apply_here(ctx) {
        return;
    }
    ctx.input_mut(|input| {
        input.events.retain(|event| {
            !matches!(
                event,
                egui::Event::Key {
                    key,
                    pressed: true,
                    modifiers,
                    ..
                } if !modifiers.alt
                    && !modifiers.ctrl
                    && !modifiers.command
                    && access_key_of_key(*key).is_some()
            )
        });
    });
}

/// The letter or digit `key` types, for a key that types one.
fn access_key_of_key(key: Key) -> Option<char> {
    let mut name = key.name().chars();
    let character = name.next()?;
    (name.next().is_none() && character.is_ascii_alphanumeric()).then_some(character)
}

#[derive(Debug, Clone, Copy, Default)]
struct AltTap {
    down: bool,
    clean: bool,
}

/// Whether Alt was tapped: pressed and let go alone, with no key, click,
/// wheel or text between, and the window focused throughout. Alt and a
/// digit (the workspaces), Alt and a drag (the canvas pans) and Alt+Tab are
/// no tap. Read from the pass's raw input, which keeps a key that a
/// shortcut has already taken.
fn alt_tapped(ctx: &Context) -> bool {
    let (modifiers, interrupted) = ctx.input(|input| {
        let interrupted = !input.raw.focused
            || input.pointer.any_down()
            || input.raw.events.iter().any(|event| {
                !matches!(
                    event,
                    egui::Event::PointerMoved(_)
                        | egui::Event::MouseMoved(_)
                        | egui::Event::PointerGone
                )
            });
        (input.modifiers, interrupted)
    });
    let id = Id::new(ALT_TAP_ID);
    let mut tap = ctx
        .data(|data| data.get_temp::<AltTap>(id))
        .unwrap_or_default();
    let mut tapped = false;
    if modifiers.alt {
        if !tap.down {
            tap = AltTap {
                down: true,
                clean: true,
            };
        }
        if !alt_alone(modifiers) || interrupted {
            tap.clean = false;
        }
    } else {
        tapped = tap.down && tap.clean && !interrupted;
        tap.down = false;
    }
    ctx.data_mut(|data| data.insert_temp(id, tap));
    tapped
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
    let access = access_keys_apply_here(ctx);
    // Followed every pass, so a tap begun under a dialog is not finished
    // once it closes.
    let tapped = access && alt_tapped(ctx);
    if !keyboard_enabled || triggers.is_empty() {
        if access {
            set_access_cues(ctx, false);
        }
        return;
    }
    let entered = bar_keys(ctx, triggers, access, tapped);
    if access {
        // The underlines last while the keyboard is in the bar or its menus.
        let focused = ctx.memory(|memory| memory.focused());
        let in_bar = focused
            .is_some_and(|id| triggers.iter().any(|trigger| trigger.response.id == id))
            || triggers
                .iter()
                .any(|trigger| Popup::is_id_open(ctx, trigger.popup_id));
        let shown = ctx
            .data(|data| data.get_temp::<bool>(Id::new(ACCESS_CUES_ID)))
            .unwrap_or(false);
        let pressed = ctx.input(|input| input.pointer.any_pressed());
        set_access_cues(ctx, (entered || shown) && in_bar && !pressed);
    }
}

/// Open the menu of `triggers[index]` from the keyboard, its first row
/// focused, keeping where the focus was if it came from outside the bar.
fn open_menu_from_keyboard(
    ctx: &Context,
    triggers: &[MenuTriggerResponse],
    index: usize,
    focused: Option<Id>,
    in_bar: bool,
) {
    if !in_bar {
        ctx.data_mut(|data| {
            data.insert_temp(Id::new(MENU_RETURN_FOCUS_ID), focused);
        });
    }
    Popup::open_id(ctx, triggers[index].popup_id);
    ctx.data_mut(|data| {
        data.insert_temp(
            triggers[index].popup_id.with("initial_focus"),
            InitialMenuFocus::First,
        );
    });
    triggers[index].response.request_focus();
}

/// The bar's own keys. Returns whether the keyboard came into the bar by
/// one of the keys that show the access keys: F10, a tap of Alt or an Alt
/// chord.
fn bar_keys(ctx: &Context, triggers: &[MenuTriggerResponse], access: bool, tapped: bool) -> bool {
    let focused = ctx.memory(|memory| memory.focused());
    let focused_trigger = focused.and_then(|id| {
        triggers
            .iter()
            .position(|trigger| trigger.response.id == id)
    });
    let open_trigger = triggers
        .iter()
        .position(|trigger| Popup::is_id_open(ctx, trigger.popup_id));

    let in_bar = focused_trigger.is_some() || open_trigger.is_some();
    // F10, and on Windows and Linux a tap of Alt, takes the focus to the
    // bar, or gives it back.
    if ctx.input_mut(|input| input.consume_key(Modifiers::NONE, Key::F10)) || tapped {
        if in_bar {
            Popup::close_all(ctx);
            let return_focus = ctx.data_mut(|data| {
                data.get_temp::<Option<Id>>(Id::new(MENU_RETURN_FOCUS_ID))
                    .flatten()
            });
            if let Some(id) = return_focus {
                request_focus(ctx, id);
            } else if let Some(index) = focused_trigger {
                // Nothing held the focus before: the bar lets it go.
                ctx.memory_mut(|memory| memory.surrender_focus(triggers[index].response.id));
            }
        } else {
            ctx.data_mut(|data| {
                data.insert_temp(Id::new(MENU_RETURN_FOCUS_ID), focused);
            });
            triggers[0].response.request_focus();
            return true;
        }
        return false;
    }

    // Alt and a menu's letter opens it, from anywhere.
    if access
        && let Some(index) = take_access_press(ctx, true, |key| {
            triggers
                .iter()
                .position(|trigger| trigger.key.access().and_then(access_key_of) == Some(key))
        })
    {
        open_menu_from_keyboard(ctx, triggers, index, focused, in_bar);
        return true;
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
            return false;
        }
    }

    let Some(current) = focused_trigger else {
        return false;
    };
    // With the bar focused and no menu open, a menu's letter opens it.
    if access
        && open_trigger.is_none()
        && let Some(index) = take_access_press(ctx, false, |key| {
            triggers
                .iter()
                .position(|trigger| trigger.key.access().and_then(access_key_of) == Some(key))
        })
    {
        open_menu_from_keyboard(ctx, triggers, index, focused, in_bar);
        return false;
    }
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
        return false;
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
        return false;
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
    false
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

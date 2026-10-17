//! The Design menu's Arrange submenu: the commands that turn, flip, line up
//! and space out the selection, which no menu listed before.

use egui::containers::menu::{MenuState, SubMenu};

use super::*;

/// The submenu's rows, group by group: turn and flip, line up, space out.
pub(super) const ARRANGE_GROUPS: [&[Command]; 3] = [
    &[
        Command::RotateSelection,
        Command::RotateSelectionCounterClockwise,
        Command::MirrorSelectionHorizontal,
        Command::MirrorSelectionVertical,
    ],
    &[
        Command::AlignSelectionLeft,
        Command::AlignSelectionCenter,
        Command::AlignSelectionRight,
        Command::AlignSelectionTop,
        Command::AlignSelectionMiddle,
        Command::AlignSelectionBottom,
    ],
    &[
        Command::DistributeSelectionHorizontal,
        Command::DistributeSelectionVertical,
    ],
];
pub(super) const ARRANGE_ROW_ID: &str = "workbench.title_menu.arrange";

/// The Arrange row, and the submenu it opens, whose contents `rows` draws.
///
/// The submenu's rows are recorded apart from the Design menu's, so the
/// Design menu's arrow keys step over the row rather than into its
/// children. Right arrow, Enter or Space opens it with its first available
/// row focused; Left arrow or Escape closes it back onto the row.
pub(super) fn arrange_submenu(ui: &mut Ui, rows: impl FnOnce(&mut Ui)) {
    let row_height = ui.spacing().interact_size.y.max(MENU_ROW_HEIGHT);
    let large_targets = row_height >= MENU_TOUCH_ROW_HEIGHT;
    let (_, rect) = ui.allocate_space(Vec2::new(ui.available_width(), row_height));
    let response = ui.interact(rect, Id::new(ARRANGE_ROW_ID), Sense::click());
    let submenu_id = SubMenu::id_from_widget_id(response.id);
    let parent_menu = MenuState::from_ui(ui, |_, stack| stack.id);
    let focus_first_id = Id::new(ARRANGE_ROW_ID).with("focus_first");

    if response.has_focus() {
        claim_menu_keys(ui.ctx(), response.id);
    }
    let opened_by_key = response.has_focus()
        && ui.input_mut(|input| input.consume_key(Modifiers::NONE, Key::ArrowRight));
    if opened_by_key {
        // Marked shown as well as opened, or the menu would drop an open
        // item it has never drawn.
        MenuState::from_id(ui.ctx(), parent_menu, |state| {
            state.open_item = Some(submenu_id);
        });
        MenuState::mark_shown(ui.ctx(), submenu_id);
    }
    if opened_by_key || (response.clicked() && !response.clicked_by(PointerButton::Primary)) {
        ui.data_mut(|data| data.insert_temp(focus_first_id, true));
    }

    let label = "Arrange";
    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label));
    record_menu_row(ui.ctx(), &response, label, true);
    let is_open = MenuState::from_id(ui.ctx(), parent_menu, |state| {
        state.open_item == Some(submenu_id)
    });
    paint_menu_row(
        ui,
        rect,
        is_open || response.hovered() || response.has_focus(),
        true,
        WorkbenchIcon::Grid,
        label,
        MenuRowTrailing::Submenu,
    );
    theme::paint_focus_ring_outset(ui, &response, rect);

    let shown = SubMenu::new().show(ui, &response, |ui| {
        configure_menu_popup(ui, large_targets);
        let focus_first = ui.data_mut(|data| data.remove_temp::<bool>(focus_first_id));
        let ctx = ui.ctx().clone();
        let recorded = render_nested_menu_rows(
            &ctx,
            focus_first.is_some().then_some(InitialMenuFocus::First),
            || rows(ui),
        );
        handle_submenu_keyboard(&ctx, parent_menu, submenu_id, &response, &recorded.rows);
    });
    if shown.is_none() {
        ui.data_mut(|data| data.remove_temp::<bool>(focus_first_id));
    }
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::MenuItem);
        node.set_label(label);
        node.set_expanded(shown.is_some());
        node.set_controls([submenu_id.accesskit_id()]);
    });
}

/// The submenu's width: wide enough that no row's label is cut short by its
/// key.
pub(super) fn arrange_submenu_width(ui: &Ui, app: &RSpiceApp) -> f32 {
    let label_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let shortcut_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let measure = |text: &str, font: &egui::FontId| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font.clone(), egui::Color32::WHITE)
            .size()
            .x
    };
    ARRANGE_GROUPS
        .iter()
        .flat_map(|group| group.iter())
        .map(|&command| {
            let shortcut = shortcut_for_occurrence(
                app.state.ui.preferences.shortcuts(),
                crate::workbench::app_state::runtime_command_platform(ui.ctx()),
                ui.ctx().os(),
                command,
                None,
            );
            let keys = if shortcut.is_empty() {
                0.0
            } else {
                measure(&shortcut, &shortcut_font) + 7.0
            };
            (31.0 + measure(command.spec().label, &label_font) + 7.0 + keys + 1.0).ceil()
        })
        .fold(MENU_CONTENT_WIDTH, f32::max)
}

/// Render `rows` as a menu of their own: what they record is kept apart from
/// the enclosing menu's records, which resume afterwards.
fn render_nested_menu_rows(
    ctx: &Context,
    initial_focus: Option<InitialMenuFocus>,
    rows: impl FnOnce(),
) -> MenuRenderState {
    let id = Id::new(MENU_RENDER_STATE_ID);
    let enclosing = ctx.data_mut(|data| data.remove_temp::<MenuRenderState>(id));
    ctx.data_mut(|data| {
        data.insert_temp(
            id,
            MenuRenderState {
                initial_focus,
                ..MenuRenderState::default()
            },
        );
    });
    rows();
    let nested = finish_menu_render(ctx);
    if let Some(enclosing) = enclosing {
        ctx.data_mut(|data| data.insert_temp(id, enclosing));
    }
    nested
}

/// Keys inside an open submenu, while one of its rows has the focus.
fn handle_submenu_keyboard(
    ctx: &Context,
    parent_menu: Id,
    submenu_id: Id,
    row: &Response,
    rows: &[MenuRowRecord],
) {
    let enabled_rows = rows.iter().filter(|row| row.enabled).collect::<Vec<_>>();
    let focused = ctx.memory(|memory| memory.focused());
    let Some(focused_index) =
        focused.and_then(|id| enabled_rows.iter().position(|row| row.id == id))
    else {
        return;
    };
    claim_menu_keys(ctx, enabled_rows[focused_index].id);
    if ctx.input_mut(|input| {
        input.consume_key(Modifiers::NONE, Key::ArrowLeft)
            || input.consume_key(Modifiers::NONE, Key::Escape)
    }) {
        MenuState::from_id(ctx, parent_menu, |state| {
            if state.open_item == Some(submenu_id) {
                state.open_item = None;
            }
        });
        row.request_focus();
        return;
    }
    if !step_menu_row_focus(ctx, &enabled_rows, Some(focused_index)) {
        typeahead_menu_row_focus(ctx, &enabled_rows, focused_index);
    }
}

/// Keep egui's own focus moves off the keys these rows answer themselves:
/// an arrow would hop the focus to whatever widget lies that way, under the
/// menu or beside it, and Escape would drop the focus before the submenu
/// could close back onto its row.
fn claim_menu_keys(ctx: &Context, id: Id) {
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

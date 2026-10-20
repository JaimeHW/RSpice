//! Menus as data, and the one function that draws them.
//!
//! A menu is written down as a list of entries of four kinds: a command
//! (optionally under a label of its own), a submenu, a separator, and a block
//! shown only in one context. Resolving the list against the application's
//! state gives the rows a reader sees; [`show_menu_rows`] draws those rows,
//! and a test reads the same rows to assert a menu's exact contents.

use egui::containers::menu::{MenuState, SubMenu};
use egui::os::OperatingSystem;

use super::*;
use crate::workbench::commands::vocabulary::CommandPlatform;

/// One entry of a menu, as the menu is written down.
#[derive(Clone, Copy)]
pub(super) enum MenuEntry {
    /// A command's row.
    Command(MenuCommand),
    /// A row that opens a menu of its own entries beside it.
    Submenu {
        label: &'static str,
        icon: WorkbenchIcon,
        entries: &'static [MenuEntry],
    },
    /// A rule between two groups of rows.
    Separator,
    /// Entries shown only while the context holds.
    Only(MenuContext, &'static [MenuEntry]),
}

/// A command's row as written down: which command, under what label, with
/// which key.
#[derive(Clone, Copy)]
pub(super) struct MenuCommand {
    pub(super) command: Command,
    label: MenuLabel,
    /// `None` shows the command's own key; `Some("")` shows none.
    shortcut: Option<&'static str>,
    /// The row opens the command palette narrowed to this scope instead of
    /// running the command.
    palette_scope: Option<&'static str>,
}

#[derive(Clone, Copy)]
enum MenuLabel {
    /// The label the command vocabulary gives the command.
    Command,
    Fixed(&'static str),
}

/// Where a block of entries is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum MenuContext {
    /// A source editor is the document on screen.
    SourceEditor,
    /// A source editor whose language service answers go to definition,
    /// find references and rename is the document on screen.
    NavigableSourceEditor,
    /// No source editor is the document on screen: the page shows a drawing.
    Drawing,
    /// RSpice runs as a desktop application, not in a browser or on a
    /// tablet or phone.
    DesktopApplication,
}

impl MenuContext {
    fn holds(self, app: &RSpiceApp, platform: CommandPlatform) -> bool {
        let editor = app.state.ui.active_source_editor;
        match self {
            Self::SourceEditor => editor.is_some(),
            Self::NavigableSourceEditor => editor.is_some_and(|editor| editor.navigable),
            Self::Drawing => editor.is_none(),
            Self::DesktopApplication => file_menu_shows_exit(platform),
        }
    }
}

/// The command's row under the vocabulary's label.
pub(super) const fn command(command: Command) -> MenuEntry {
    MenuEntry::Command(MenuCommand {
        command,
        label: MenuLabel::Command,
        shortcut: None,
        palette_scope: None,
    })
}

/// The command's row under a label of the menu's own.
pub(super) const fn command_as(command: Command, label: &'static str) -> MenuEntry {
    MenuEntry::Command(MenuCommand {
        command,
        label: MenuLabel::Fixed(label),
        shortcut: None,
        palette_scope: None,
    })
}

/// The command's row under a label of the menu's own, with no key shown.
pub(super) const fn command_without_key(command: Command, label: &'static str) -> MenuEntry {
    MenuEntry::Command(MenuCommand {
        command,
        label: MenuLabel::Fixed(label),
        shortcut: Some(""),
        palette_scope: None,
    })
}

/// A row that opens the command palette narrowed to `scope`.
pub(super) const fn palette_scope(label: &'static str, scope: &'static str) -> MenuEntry {
    MenuEntry::Command(MenuCommand {
        command: Command::CommandPalette,
        label: MenuLabel::Fixed(label),
        shortcut: Some("Ctrl+K"),
        palette_scope: Some(scope),
    })
}

/// A menu row as it reads for the application's state right now.
#[derive(Debug, Clone)]
pub(super) enum MenuRow {
    Command(CommandRow),
    Separator,
    Submenu {
        label: &'static str,
        icon: WorkbenchIcon,
        rows: Vec<MenuRow>,
    },
}

#[derive(Debug, Clone)]
pub(super) struct CommandRow {
    pub(super) command: Command,
    pub(super) label: &'static str,
    /// The key painted at the row's right edge, empty for none.
    pub(super) shortcut: String,
    pub(super) availability: CommandAvailability,
    /// Whether the state the command switches is on, for a command that
    /// switches one: the row shows a check mark while it is.
    pub(super) checked: Option<bool>,
    palette_scope: Option<&'static str>,
}

/// The rows `entries` show in the window `ctx` draws.
pub(super) fn menu_rows(entries: &[MenuEntry], app: &RSpiceApp, ctx: &Context) -> Vec<MenuRow> {
    resolve_menu_rows(
        entries,
        app,
        crate::workbench::app_state::runtime_command_platform(ctx),
        ctx.os(),
    )
}

/// The rows `entries` show for `app` on `platform`, hidden ones included so
/// a test can see what a menu leaves out.
pub(super) fn resolve_menu_rows(
    entries: &[MenuEntry],
    app: &RSpiceApp,
    platform: CommandPlatform,
    os: OperatingSystem,
) -> Vec<MenuRow> {
    let mut rows = Vec::with_capacity(entries.len());
    push_menu_rows(&mut rows, entries, app, platform, os);
    rows
}

fn push_menu_rows(
    rows: &mut Vec<MenuRow>,
    entries: &[MenuEntry],
    app: &RSpiceApp,
    platform: CommandPlatform,
    os: OperatingSystem,
) {
    for entry in entries {
        match *entry {
            MenuEntry::Command(item) => rows.push(MenuRow::Command(CommandRow {
                command: item.command,
                label: match item.label {
                    MenuLabel::Command => item.command.spec().label,
                    MenuLabel::Fixed(label) => label,
                },
                shortcut: shortcut_for_occurrence(
                    app.state.ui.preferences.shortcuts(),
                    platform,
                    os,
                    item.command,
                    item.shortcut,
                ),
                availability: item.command.availability(app),
                checked: item.command.checked(app),
                palette_scope: item.palette_scope,
            })),
            MenuEntry::Submenu {
                label,
                icon,
                entries,
            } => rows.push(MenuRow::Submenu {
                label,
                icon,
                rows: resolve_menu_rows(entries, app, platform, os),
            }),
            MenuEntry::Separator => rows.push(MenuRow::Separator),
            MenuEntry::Only(context, entries) => {
                if context.holds(app, platform) {
                    push_menu_rows(rows, entries, app, platform, os);
                }
            }
        }
    }
}

/// The row a reader chose from a menu, carried out once the menu is drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct MenuChoice {
    command: Command,
    palette_scope: Option<&'static str>,
}

impl MenuChoice {
    pub(super) fn run(self, app: &mut RSpiceApp) {
        if let Some(scope) = self.palette_scope {
            app.state.dialogs.command_palette.open_in_scope(scope);
        } else {
            self.command.execute(app);
        }
    }
}

/// The rows a reader sees, in order: hidden rows dropped, and a separator
/// kept only between two groups that each show a row, so no rule leads or
/// ends a menu and a group whose rows are all hidden leaves no second rule
/// beside the first.
pub(super) fn visible_rows(rows: &[MenuRow]) -> Vec<&MenuRow> {
    let mut visible = Vec::with_capacity(rows.len());
    let mut pending_rule = None;
    for row in rows {
        match row {
            MenuRow::Separator => {
                if !visible.is_empty() {
                    pending_rule = Some(row);
                }
            }
            MenuRow::Command(command) if command.availability == CommandAvailability::Hidden => {}
            MenuRow::Command(_) | MenuRow::Submenu { .. } => {
                if let Some(rule) = pending_rule.take() {
                    visible.push(rule);
                }
                visible.push(row);
            }
        }
    }
    visible
}

/// Draw `rows` into an open menu and return the row the reader chose, if
/// any. Hidden rows are not drawn.
pub(super) fn show_menu_rows(ui: &mut Ui, rows: &[MenuRow]) -> Option<MenuChoice> {
    let mut choice = None;
    for row in visible_rows(rows) {
        let chosen = match row {
            MenuRow::Command(row) => command_row(ui, row),
            MenuRow::Separator => {
                menu_separator(ui);
                None
            }
            MenuRow::Submenu { label, icon, rows } => submenu_row(ui, label, *icon, rows),
        };
        choice = choice.or(chosen);
    }
    choice
}

fn command_row(ui: &mut Ui, row: &CommandRow) -> Option<MenuChoice> {
    let availability = row.availability;
    if availability == CommandAvailability::Hidden {
        return None;
    }
    let label = row.label;
    let enabled = availability.is_available();
    let row_height = ui.spacing().interact_size.y.max(MENU_ROW_HEIGHT);
    // A greyed row is still a live widget: it takes the focus, so the keys
    // walk every row a reader can see and the row can say why it is grey.
    // Choosing it does nothing.
    let (rect, response) =
        ui.allocate_exact_size(Vec2::new(ui.available_width(), row_height), Sense::click());
    response.widget_info(|| match row.checked {
        Some(checked) => {
            egui::WidgetInfo::selected(egui::WidgetType::Checkbox, enabled, checked, label)
        }
        None => egui::WidgetInfo::labeled(egui::WidgetType::Button, enabled, label),
    });
    ui.ctx().accesskit_node_builder(response.id, |node| {
        match row.checked {
            Some(checked) => {
                node.set_role(egui::accesskit::Role::MenuItemCheckBox);
                node.set_toggled(checked.into());
            }
            None => node.set_role(egui::accesskit::Role::MenuItem),
        }
        node.set_label(label);
        if let CommandAvailability::Disabled(reason) = availability {
            node.set_description(reason);
        }
        if enabled && !row.shortcut.is_empty() {
            node.set_keyboard_shortcut(row.shortcut.as_str());
        }
    });
    record_menu_row(ui.ctx(), &response, label);

    // The pointer lights only a row it can choose; the keyboard's focus
    // lights any row, so the reader sees where it is.
    let highlighted = response.has_focus() || (enabled && response.hovered());
    paint_menu_row(
        ui,
        rect,
        highlighted,
        enabled,
        row.checked
            .map_or(MenuRowLead::Icon(command_icon(row.command)), MenuRowLead::Check),
        label,
        MenuRowTrailing::Shortcut(&row.shortcut),
    );
    theme::paint_focus_ring_outset(ui, &response, rect);

    let response = match availability {
        CommandAvailability::Disabled(reason) => response.on_hover_text(sentence(reason)),
        CommandAvailability::Available | CommandAvailability::Hidden => response,
    };
    if enabled && response.clicked() {
        ui.close();
        return Some(MenuChoice {
            command: row.command,
            palette_scope: row.palette_scope,
        });
    }
    None
}

/// A greyed row's reason as its tooltip reads it: a sentence.
fn sentence(reason: &str) -> String {
    let mut characters = reason.chars();
    characters.next().map_or_else(String::new, |first| {
        first.to_uppercase().chain(characters).collect()
    })
}

const MENU_SEPARATOR_HEIGHT: f32 = 9.0;

fn menu_separator(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(
        Vec2::new(ui.available_width(), MENU_SEPARATOR_HEIGHT),
        Sense::hover(),
    );
    ui.painter().hline(
        (rect.left() + 5.0)..=(rect.right() - 5.0),
        rect.top() + 4.5,
        egui::Stroke::new(1.0, t.color.border),
    );
}

/// The identity of the submenu row labelled `label`.
pub(super) fn submenu_row_id(label: &str) -> Id {
    Id::new(("workbench.title_menu.submenu", label))
}

/// A submenu's row, and the submenu it opens, drawing `rows`.
///
/// The submenu's rows are recorded apart from the enclosing menu's, so that
/// menu's arrow keys step over the row rather than into its children. Right
/// arrow, Enter or Space opens it with its first row focused; Left
/// arrow or Escape closes it back onto the row.
fn submenu_row(
    ui: &mut Ui,
    label: &'static str,
    icon: WorkbenchIcon,
    rows: &[MenuRow],
) -> Option<MenuChoice> {
    let row_height = ui.spacing().interact_size.y.max(MENU_ROW_HEIGHT);
    let large_targets = row_height >= MENU_TOUCH_ROW_HEIGHT;
    let (_, rect) = ui.allocate_space(Vec2::new(ui.available_width(), row_height));
    let row_id = submenu_row_id(label);
    let response = ui.interact(rect, row_id, Sense::click());
    let submenu_id = SubMenu::id_from_widget_id(response.id);
    let parent_menu = MenuState::from_ui(ui, |_, stack| stack.id);
    let focus_first_id = row_id.with("focus_first");

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

    response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, label));
    record_menu_row(ui.ctx(), &response, label);
    let is_open = MenuState::from_id(ui.ctx(), parent_menu, |state| {
        state.open_item == Some(submenu_id)
    });
    paint_menu_row(
        ui,
        rect,
        is_open || response.hovered() || response.has_focus(),
        true,
        MenuRowLead::Icon(icon),
        label,
        MenuRowTrailing::Submenu,
    );
    theme::paint_focus_ring_outset(ui, &response, rect);

    let shown = SubMenu::new().show(ui, &response, |ui| {
        configure_menu_popup(ui, large_targets, rows);
        let focus_first = ui.data_mut(|data| data.remove_temp::<bool>(focus_first_id));
        let ctx = ui.ctx().clone();
        let mut choice = None;
        let recorded = render_nested_menu_rows(
            &ctx,
            focus_first.is_some().then_some(InitialMenuFocus::First),
            || choice = show_menu_rows(ui, rows),
        );
        handle_submenu_keyboard(&ctx, parent_menu, submenu_id, &response, &recorded.rows);
        choice
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
    shown.and_then(|shown| shown.inner)
}

/// The height `rows` take in an open menu whose rows `ui` sizes.
pub(super) fn menu_rows_height(ui: &Ui, rows: &[MenuRow]) -> f32 {
    let row_height = ui.spacing().interact_size.y.max(MENU_ROW_HEIGHT);
    visible_rows(rows)
        .into_iter()
        .map(|row| match row {
            MenuRow::Separator => MENU_SEPARATOR_HEIGHT,
            MenuRow::Command(_) | MenuRow::Submenu { .. } => row_height,
        })
        .sum()
}

/// A menu's content width: as wide as its longest row needs, within the
/// narrowest and widest a menu is.
pub(super) fn menu_rows_width(ui: &Ui, rows: &[MenuRow]) -> f32 {
    menu_rows_fit(ui, rows).clamp(MENU_CONTENT_WIDTH, MENU_MAX_CONTENT_WIDTH)
}

/// The content width the longest of `rows` needs for its label, with its
/// key or chevron, to be painted whole.
pub(super) fn menu_rows_fit(ui: &Ui, rows: &[MenuRow]) -> f32 {
    let label_font = theme::sans(tokens::FS_0, FontWeight::Regular);
    let shortcut_font = theme::mono(tokens::FS_0, FontWeight::Regular);
    let measure = |text: &str, font: &egui::FontId| {
        ui.painter()
            .layout_no_wrap(text.to_owned(), font.clone(), egui::Color32::WHITE)
            .size()
            .x
    };
    rows.iter()
        .filter_map(|row| match row {
            MenuRow::Command(row) if row.availability != CommandAvailability::Hidden => {
                let keys = if row.shortcut.is_empty() {
                    0.0
                } else {
                    measure(&row.shortcut, &shortcut_font) + 7.0
                };
                Some(measure(row.label, &label_font) + keys)
            }
            MenuRow::Submenu { label, .. } => Some(measure(label, &label_font) + 12.0 + 7.0),
            MenuRow::Command(_) | MenuRow::Separator => None,
        })
        .map(|content| (31.0 + content + 7.0 + 1.0).ceil())
        .fold(0.0, f32::max)
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
    let focused = ctx.memory(|memory| memory.focused());
    let Some(focused_index) = focused.and_then(|id| rows.iter().position(|row| row.id == id))
    else {
        return;
    };
    claim_menu_keys(ctx, rows[focused_index].id);
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
    if !step_menu_row_focus(ctx, rows, Some(focused_index)) {
        typeahead_menu_row_focus(ctx, rows, focused_index);
    }
}

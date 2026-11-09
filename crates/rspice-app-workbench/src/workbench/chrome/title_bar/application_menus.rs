//! The application menus the title bar offers, and how it sizes and draws
//! an open one. What each menu holds is the host's; the title bar asks for
//! a menu's entries by name.

use super::*;
use menu_entries::MenuRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ApplicationMenu {
    File,
    Edit,
    View,
    Design,
    Simulate,
    Results,
    Verify,
    Tools,
    Window,
    Help,
}

impl ApplicationMenu {
    pub const fn label(self) -> &'static str {
        match self {
            Self::File => "File",
            Self::Edit => "Edit",
            Self::View => "View",
            Self::Design => "Design",
            Self::Simulate => "Simulate",
            Self::Results => "Results",
            Self::Verify => "Verify",
            Self::Tools => "Tools",
            Self::Window => "Window",
            Self::Help => "Help",
        }
    }

    /// The letter that opens the menu with Alt on Windows and Linux, and
    /// alone while the bar has the focus: each menu's own.
    pub const fn access_key(self) -> char {
        match self {
            Self::File => 'F',
            Self::Edit => 'E',
            Self::View => 'V',
            Self::Design => 'D',
            Self::Simulate => 'S',
            Self::Results => 'R',
            Self::Verify => 'Y',
            Self::Tools => 'T',
            Self::Window => 'W',
            Self::Help => 'H',
        }
    }
}

/// Whether the menus answer access keys: on the Windows and Linux desktop.
/// The browser keeps Alt and a letter for its own menus, and a macOS menu
/// has none.
pub fn access_keys_apply(
    platform: crate::state::command_catalog::CommandPlatform,
    os: egui::os::OperatingSystem,
) -> bool {
    platform == crate::state::command_catalog::CommandPlatform::Desktop
        && os != egui::os::OperatingSystem::Mac
}

/// Whether the bar opens with the RSpice menu, its brand turned into the
/// application menu the way a macOS application's menu bar begins: on the
/// macOS desktop. The browser has its own, and Windows and Linux keep About,
/// Preferences and Exit in Help, Edit and File.
pub fn shows_application_menu(
    platform: crate::state::command_catalog::CommandPlatform,
    os: egui::os::OperatingSystem,
) -> bool {
    platform == crate::state::command_catalog::CommandPlatform::Desktop
        && os == egui::os::OperatingSystem::Mac
}

/// Where `label` underlines its access key `key`: the first letter that is
/// `key` and starts a word ("Save &as"), else the first that is `key` at
/// all ("E&xit"), ignoring case. A char index, or `None` when the label has
/// no such letter.
pub fn access_key_index(label: &str, key: char) -> Option<usize> {
    let matches = |character: char| character.to_lowercase().eq(key.to_lowercase());
    let characters: Vec<char> = label.chars().collect();
    let starts_word = |index: usize| index == 0 || !characters[index - 1].is_alphanumeric();
    (0..characters.len())
        .find(|&index| matches(characters[index]) && starts_word(index))
        .or_else(|| characters.iter().position(|&character| matches(character)))
}

pub const ALL_MENUS: [ApplicationMenu; 10] = [
    ApplicationMenu::File,
    ApplicationMenu::Edit,
    ApplicationMenu::View,
    ApplicationMenu::Design,
    ApplicationMenu::Simulate,
    ApplicationMenu::Results,
    ApplicationMenu::Verify,
    ApplicationMenu::Tools,
    ApplicationMenu::Window,
    ApplicationMenu::Help,
];

/// Size an open menu for `rows` (as wide as its longest row needs, within
/// the menu's bounds) and give its rows their height.
pub(super) fn configure_menu_popup(ui: &mut Ui, large_targets: bool, rows: &[MenuRow]) {
    let width = menu_entries::menu_rows_width(ui, rows);
    ui.set_min_width(width);
    ui.set_max_width(width);
    ui.spacing_mut().item_spacing.y = 0.0;
    ui.spacing_mut().interact_size.y = if large_targets {
        MENU_TOUCH_ROW_HEIGHT
    } else {
        MENU_ROW_HEIGHT
    };
}

/// The source editor's right-click menu, drawn with the menu bar's own rows
/// and separators so its row height, text edge and rules match the menus
/// above it. A long press opens it on touch.
pub fn source_editor_context_menu(
    response: &Response,
    app: &mut impl WorkbenchHost<Menu = ApplicationMenu, MenuEntry = MenuEntry>,
) {
    let ctx = response.ctx.clone();
    let large_targets = ctx.content_rect().width() <= 820.0
        || Tokens::get(&ctx).metrics.ctl_h >= MENU_TOUCH_ROW_HEIGHT;
    let popup_id = Popup::default_response_id(response);
    let shown = crate::ui::widgets::context_menu_popup(response).show(|ui| {
        let rows = menu_rows(app.source_editor_menu(), app, ui.ctx());
        configure_menu_popup(ui, large_targets, &rows);
        ui.ctx().accesskit_node_builder(popup_id, |node| {
            node.set_role(egui::accesskit::Role::Menu);
            node.set_label("Source editor menu");
        });
        begin_menu_render(ui.ctx(), popup_id);
        let choice = menu_entries::show_menu_rows(ui, &rows);
        let render_state = finish_menu_render(ui.ctx());
        let typed = handle_popup_keyboard(ui.ctx(), popup_id, response, &render_state.rows);
        if typed.is_some() {
            ui.close();
        }
        choice.or(typed)
    });
    if let Some(choice) = shown.and_then(|shown| shown.inner) {
        choice.run(app);
    }
}

pub fn shortcut_for_occurrence(
    host: &impl Host,
    platform: crate::state::command_catalog::CommandPlatform,
    operating_system: egui::os::OperatingSystem,
    command: crate::state::command_catalog::CommandKey,
    shortcut_override: Option<&str>,
) -> String {
    shortcut_override.map_or_else(
        || host.shortcut_label(command, platform, operating_system),
        str::to_owned,
    )
}

pub(super) fn menu_popup_max_height(ui: &Ui) -> f32 {
    menu_popup_height_for_viewport(ui.ctx().content_rect().height())
}

pub fn menu_popup_height_for_viewport(viewport_height: f32) -> f32 {
    // Preserve room for the title bar and a small bottom escape margin while
    // guaranteeing enough space to navigate at least three standard rows. A
    // menu otherwise takes the window's height before it scrolls: File, the
    // longest, scrolled its Exit out of sight under a fixed 560 pt.
    (viewport_height - 64.0).max(MENU_ROW_HEIGHT * 3.0)
}

pub const fn file_menu_shows_exit(
    platform: crate::state::command_catalog::CommandPlatform,
) -> bool {
    matches!(
        platform,
        crate::state::command_catalog::CommandPlatform::Desktop
    )
}

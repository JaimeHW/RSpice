//! The application menus the title bar offers, and how it sizes and draws
//! an open one. What each menu holds is the host's; the title bar asks for
//! a menu's entries by name.

use super::*;
use menu_entries::MenuRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ApplicationMenu {
    File,
    Edit,
    View,
    Design,
    Simulate,
    Results,
    Verify,
    Models,
    Automation,
    Window,
    Help,
}

impl ApplicationMenu {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::File => "File",
            Self::Edit => "Edit",
            Self::View => "View",
            Self::Design => "Design",
            Self::Simulate => "Simulate",
            Self::Results => "Results",
            Self::Verify => "Verify",
            Self::Models => "Models",
            Self::Automation => "Automation",
            Self::Window => "Window",
            Self::Help => "Help",
        }
    }
}

pub(crate) const ALL_MENUS: [ApplicationMenu; 11] = [
    ApplicationMenu::File,
    ApplicationMenu::Edit,
    ApplicationMenu::View,
    ApplicationMenu::Design,
    ApplicationMenu::Simulate,
    ApplicationMenu::Results,
    ApplicationMenu::Verify,
    ApplicationMenu::Models,
    ApplicationMenu::Automation,
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
pub(in crate::workbench) fn source_editor_context_menu(
    response: &Response,
    app: &mut impl WorkbenchHost<Menu = ApplicationMenu, MenuEntry = MenuEntry>,
) {
    let ctx = response.ctx.clone();
    let large_targets = ctx.content_rect().width() <= 820.0
        || Tokens::get(&ctx).metrics.ctl_h >= MENU_TOUCH_ROW_HEIGHT;
    let popup_id = Popup::default_response_id(response);
    let shown = Popup::context_menu(response).show(|ui| {
        let rows = menu_rows(app.source_editor_menu(), app, ui.ctx());
        configure_menu_popup(ui, large_targets, &rows);
        ui.ctx().accesskit_node_builder(popup_id, |node| {
            node.set_role(egui::accesskit::Role::Menu);
            node.set_label("Source editor menu");
        });
        begin_menu_render(ui.ctx(), popup_id);
        let choice = menu_entries::show_menu_rows(ui, &rows);
        let render_state = finish_menu_render(ui.ctx());
        handle_popup_keyboard(ui.ctx(), popup_id, response, &render_state.rows);
        choice
    });
    if let Some(choice) = shown.and_then(|shown| shown.inner) {
        choice.run(app);
    }
}

pub(crate) fn shortcut_for_occurrence(
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

pub(crate) fn menu_popup_height_for_viewport(viewport_height: f32) -> f32 {
    // Preserve room for the title bar and a small bottom escape margin while
    // guaranteeing enough space to navigate at least three standard rows.
    (viewport_height - 64.0).clamp(MENU_ROW_HEIGHT * 3.0, 560.0)
}

pub(crate) const fn file_menu_shows_exit(
    platform: crate::state::command_catalog::CommandPlatform,
) -> bool {
    matches!(
        platform,
        crate::state::command_catalog::CommandPlatform::Desktop
    )
}

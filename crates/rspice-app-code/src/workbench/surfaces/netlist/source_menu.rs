//! The source editor's right-click menu's rows on the code pages (the deck
//! and the Verilog-A view), read from the code feature's context
//! lent with its menus' answers while the menu is open.

use egui::os::OperatingSystem;

use crate::state::command_catalog::{CommandKey, CommandPlatform};
use crate::workbench::chrome::title_bar::menu_entries::{
    MenuContext, MenuEntry, MenuLookup, MenuRow, menu_rows,
};
use crate::workbench::code::CodeFeature;
use crate::workbench::design_system::WorkbenchIcon;
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::framework::feature::Cx;

/// What the source editor menu's rows read, answered from the code
/// feature's context lent with its menus' answers.
pub struct SourceMenuLookup<'c, 'a>(pub &'c Cx<'a, CodeFeature>);

impl MenuLookup for SourceMenuLookup<'_, '_> {
    fn menu_label(&self, key: CommandKey) -> std::borrow::Cow<'static, str> {
        crate::workbench::code::source_menu_label(
            key,
            self.0.model,
            &self.0.state.netlist,
            self.0.wb.messages(),
        )
        .map_or_else(|| self.0.commands.label(key).into(), Into::into)
    }

    fn shortcut_label(
        &self,
        key: CommandKey,
        platform: CommandPlatform,
        os: OperatingSystem,
    ) -> String {
        self.0
            .model
            .preferences
            .shortcuts()
            .resolved_label(self.0.commands, key, platform, os)
    }

    fn availability(&self, key: CommandKey) -> CommandAvailability {
        self.0.foreign_availability(key)
    }

    /// None of the menu's commands switches a state on and off.
    fn checked(&self, _key: CommandKey) -> Option<bool> {
        None
    }

    /// Undo and Redo wear their toolbar buttons' icons, as on Edit; none of
    /// the menu's other commands has a button whose icon its row would wear.
    fn command_icon(&self, key: CommandKey) -> Option<WorkbenchIcon> {
        use crate::workbench::framework::command_registry::ids;
        if key == ids::UNDO {
            Some(WorkbenchIcon::Undo)
        } else if key == ids::REDO {
            Some(WorkbenchIcon::Redo)
        } else {
            None
        }
    }

    fn holds(&self, context: MenuContext, platform: CommandPlatform, os: OperatingSystem) -> bool {
        context.holds(&*self.0.wb, &*self.0.model, &*self.0.platform, platform, os)
    }

    fn recent_files(&self) -> &[crate::services::recent_files::RecentFile] {
        &self.0.platform.recent_files
    }
}

/// The rows `entries` show in the window `ctx` draws, read from `cx`, the
/// code feature's context lent with its menus' answers.
pub(in crate::workbench::surfaces) fn rows(
    entries: &[MenuEntry],
    cx: &Cx<'_, CodeFeature>,
    ctx: &egui::Context,
) -> Vec<MenuRow> {
    menu_rows(entries, &SourceMenuLookup(cx), ctx)
}

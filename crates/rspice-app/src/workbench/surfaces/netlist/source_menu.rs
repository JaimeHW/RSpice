//! The deck editor's right-click menu's rows: the source editor's, read from
//! the code feature's context lent with its menus' answers while the menu
//! is open.

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
struct SourceMenuLookup<'c, 'a>(&'c Cx<'a, CodeFeature>);

impl MenuLookup for SourceMenuLookup<'_, '_> {
    fn menu_label(&self, key: CommandKey) -> std::borrow::Cow<'static, str> {
        self.0.commands.label(key).into()
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

    /// None of the menu's commands has a toolbar button whose icon its row
    /// would wear.
    fn command_icon(&self, _key: CommandKey) -> Option<WorkbenchIcon> {
        None
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
pub(super) fn rows(
    entries: &[MenuEntry],
    cx: &Cx<'_, CodeFeature>,
    ctx: &egui::Context,
) -> Vec<MenuRow> {
    menu_rows(entries, &SourceMenuLookup(cx), ctx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::RSpiceApp;
    use crate::workbench::chrome::title_bar::menu_entries::{menu_keys, resolve_menu_rows};
    use crate::workbench::documents::text_editor_commands::ActiveSourceEditor;
    use crate::workbench::features;
    use crate::workbench::framework::feature::Feature;
    use crate::workbench::framework::host::Host;

    /// The commands the code feature declares its menus read are the source
    /// editor menu's, in its order.
    #[test]
    fn the_code_feature_reads_the_source_editor_menus_commands() {
        let app = RSpiceApp::test_instance();
        let mut keys = Vec::new();
        menu_keys(app.source_editor_menu(), &mut keys);
        assert_eq!(keys, CodeFeature::MENU_AVAILABILITY);
    }

    /// The menu drawn from the code feature's context shows the rows the
    /// host's menu shows, label, key, availability, check mark, icon and
    /// access letter alike: with no editor in front, with an editor that
    /// can be written and one that only reads, with or without a language
    /// service behind it, on each platform.
    #[test]
    fn the_menu_reads_the_rows_the_host_reads() {
        let mut app = RSpiceApp::test_instance();
        let entries = app.source_editor_menu();
        let editors = [
            None,
            Some((true, true)),
            Some((true, false)),
            Some((false, true)),
            Some((false, false)),
        ];
        let places = [
            (CommandPlatform::Desktop, OperatingSystem::Windows),
            (CommandPlatform::Desktop, OperatingSystem::Mac),
            (CommandPlatform::Browser, OperatingSystem::Nix),
            (CommandPlatform::Phone, OperatingSystem::Android),
        ];
        for project_open in [false, true] {
            if project_open {
                app.state.open_project_session();
            }
            for editor in editors {
                app.state.workbench.active_source_editor =
                    editor.map(|(editable, navigable)| ActiveSourceEditor {
                        id: egui::Id::new("source menu editor"),
                        editable,
                        navigable,
                    });
                for (platform, os) in places {
                    let host = format!("{:?}", resolve_menu_rows(entries, &app, platform, os));
                    let cx = features::cx_with_menu_answers::<CodeFeature>(&mut app);
                    let code = format!(
                        "{:?}",
                        resolve_menu_rows(entries, &SourceMenuLookup(&cx), platform, os)
                    );
                    assert_eq!(
                        code, host,
                        "editor {editor:?}, project open {project_open}, {platform:?} {os:?}"
                    );
                }
            }
        }
    }
}

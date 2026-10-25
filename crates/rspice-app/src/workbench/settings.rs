//! The settings feature: Preferences, the workspace layout manager and the
//! keyboard shortcut workflows.
//!
//! Its commands and its dialog step run in a [`Cx`] over the feature's own
//! dialog state, the model, whose preferences it edits, the session, whose
//! route opens Preferences, and the platform services' state, whose autosave
//! interval and recent files Preferences keeps.

pub(crate) mod commands;

use crate::state::command_catalog::CommandKey;
use crate::workbench::app::dialogs::preferences::PreferencesDialogs;
use crate::workbench::app::dialogs::preferences::shortcut_preferences::COMMAND_REFERENCE;
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::framework::dialogs::DialogKey;
use crate::workbench::framework::feature::{
    CommandSet, Cx, Feature, FeatureDialog, FeatureRegistry, Reads,
};
use crate::workbench::shortcuts::artifacts::VSCODE_COMMAND_MAPPINGS;
use crate::workbench::slots::SettingsState;
use crate::workbench::state::WorkbenchSession;
use crate::workbench::{RouteTransitionSource, SurfaceId, SurfaceRoute};

use commands::SettingsCommand;

/// The settings feature.
pub(crate) struct SettingsFeature;

impl Feature for SettingsFeature {
    type State = SettingsState;
    type Session = WorkbenchSession;

    fn register(registry: &mut FeatureRegistry<Self>) {
        registry.commands::<SettingsCommand>();
        registry.dialogs(DIALOGS);
    }
}

/// The dialog step the feature draws, under the key the shell's dialog order
/// lists it by: Preferences, and the windows it opens.
const DIALOGS: &[FeatureDialog<SettingsFeature>] = &[FeatureDialog {
    key: DialogKey("settings.preferences"),
    is_open: |settings| {
        settings.preferences_open
            || settings.workspace_layout_manager.open
            || settings.shortcut_portability.application_modal_open()
            || settings.shortcut_editor.open
    },
    render: render_preferences,
}];

/// Draw Preferences and the windows it opens for this frame.
fn render_preferences(cx: &mut Cx<'_, SettingsFeature>, ctx: &egui::Context) {
    let settings = &mut *cx.state;
    PreferencesDialogs {
        preferences_open: &mut settings.preferences_open,
        workspace_layout_manager: &mut settings.workspace_layout_manager,
        shortcut_portability: &mut settings.shortcut_portability,
        shortcut_editor: &mut settings.shortcut_editor,
        shortcut_policy_candidate: &mut settings.shortcut_policy_candidate,
        browser_spoken_feedback: &mut settings.browser_spoken_feedback,
        shortcut_library_persistence: &mut settings.shortcut_library_persistence,
        shortcut_library_publication_continuation: &mut settings
            .shortcut_library_publication_continuation,
        preferences_file: &mut settings.preferences_file,
        model: cx.model,
        workbench: cx.wb,
        platform: cx.platform,
        cloud: cx.peers.cloud(),
    }
    .render(ctx);
}

/// Every command key the feature names as data rather than declares, with
/// what names it: the commands the VS Code map binds, and the command the
/// shortcut editor leaves out. The registry checks each is a live key.
pub(crate) fn named_command_keys() -> impl Iterator<Item = (&'static str, CommandKey)> {
    VSCODE_COMMAND_MAPPINGS
        .iter()
        .map(|mapping| ("the VS Code shortcut map", mapping.rspice_command))
        .chain([("the shortcut editor", COMMAND_REFERENCE)])
}

impl CommandSet for SettingsCommand {
    type Feature = SettingsFeature;
    const REGISTERED: &'static [Self] = &Self::ALL;

    fn spec(self) -> crate::workbench::framework::command_registry::CommandSpec {
        self.declared_spec()
    }

    /// Preferences and the layout manager open from anywhere.
    fn availability(self, _reads: &Reads<'_, SettingsFeature>) -> CommandAvailability {
        CommandAvailability::Available
    }

    fn execute(self, cx: &mut Cx<'_, SettingsFeature>) {
        match self {
            Self::Preferences => {
                let route = SurfaceRoute::surface(SurfaceId::Preferences);
                if let Err(error) = cx.wb.navigate(route, RouteTransitionSource::User) {
                    cx.wb.prompt(error.to_string());
                }
            }
            Self::WorkspaceLayouts => {
                let preset = cx
                    .model
                    .preferences
                    .workspace()
                    .map(|workspace| workspace.preset())
                    .unwrap_or_default();
                cx.state.workspace_layout_manager.open(preset);
            }
        }
    }
}

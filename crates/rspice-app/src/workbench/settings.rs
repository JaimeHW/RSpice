//! The settings feature: Preferences, the workspace layout manager and the
//! keyboard shortcut workflows.
//!
//! Its commands and frame steps run in a [`Cx`] over the feature's own dialog
//! state, the model, whose preferences it edits, the session, whose route
//! opens Preferences, and the platform services' state, whose autosave
//! interval the preferences file keeps.

pub(crate) mod commands;

use crate::state::command_catalog::CommandKey;
use crate::workbench::app::dialogs::preferences::shortcut_preferences::COMMAND_REFERENCE;
use crate::workbench::app_state::session::preferences_file::PreferencesFileParts;
use crate::workbench::framework::command_registry::CommandAvailability;
use crate::workbench::framework::feature::{CommandSet, Cx, Feature, FeatureRegistry, Reads};
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
    }
}

impl SettingsFeature {
    /// The settings frame step that keeps the preferences file current: a
    /// change is noticed, and written once it settles.
    pub(in crate::workbench) fn persist_preferences_file(
        cx: &mut Cx<'_, Self>,
        ctx: &egui::Context,
    ) {
        Self::preferences_file(cx).persist_preferences_file(ctx);
    }

    /// Write any preference change not yet in the file now, rather than
    /// when it settles: the shell's last step before the session is saved.
    pub(in crate::workbench) fn flush_preferences_file(cx: &mut Cx<'_, Self>) {
        Self::preferences_file(cx).flush_preferences_file();
    }

    /// What the preferences file reads and writes, from the context.
    fn preferences_file<'c>(cx: &'c mut Cx<'_, Self>) -> PreferencesFileParts<'c> {
        PreferencesFileParts {
            file: &mut cx.state.preferences_file,
            browser_spoken_feedback: &mut cx.state.browser_spoken_feedback,
            model: &mut *cx.model,
            workbench: &mut *cx.wb,
            platform: &mut *cx.platform,
        }
    }
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

//! The commands the project feature declares, as the registry lists them:
//! the project launcher, the project's file commands, its cells and views,
//! the Project workspace's pages, and the design's sheets, revision history
//! and check-and-save.
//!
//! What each command does is the feature's own; what it declares here is all
//! the shell's order, menus, palette and shortcut profile read of it.
//!
//! Beside them is what the project feature hands the shell to carry out,
//! which the feature's own context cannot.

use egui::Key;
use rspice_app_state::workbench::app_state::model::WorkspacePruned;

use crate::state::command_catalog::{
    CommandKey, CommandPlatform, ShortcutBinding, ShortcutChord, ShortcutContext, ShortcutKind,
};
use crate::state::{CellViewRef, ViewType};
use crate::workbench::framework::command_registry::{CommandFlags, CommandOwner, CommandSpec, ids};
use crate::workbench::framework::requests::{Drain, Request};
use crate::workbench::project::configuration_state::ConfigurationSelection;
use crate::workbench::project::workspace_state::ProjectPage;

/// One command of the project feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProjectCommand {
    ProjectLauncher,
    RecentProjects,
    OpenProject,
    NewProject,
    Save,
    SaveAs,
    SaveAll,
    CloseProject,
    NewCell,
    /// A cell view of another kind beside the views the selected cell has.
    NewView,
    OpenSchematic,
    /// Open the Project workspace on one of its pages.
    Page(ProjectPage),
    DesignManagement,
    RevisionHistory,
    CheckAndSave,
}

const ALL: &[CommandPlatform] = &CommandPlatform::ALL;
const DESKTOP: &[CommandPlatform] = &[CommandPlatform::Desktop];
const HOST: &[CommandPlatform] = &[
    CommandPlatform::Browser,
    CommandPlatform::Tablet,
    CommandPlatform::Phone,
];

const fn primary(chord: ShortcutChord, platforms: &'static [CommandPlatform]) -> ShortcutBinding {
    ShortcutBinding {
        chord,
        platforms,
        kind: ShortcutKind::Primary,
    }
}

const fn alternate(chord: ShortcutChord, platforms: &'static [CommandPlatform]) -> ShortcutBinding {
    ShortcutBinding {
        chord,
        platforms,
        kind: ShortcutKind::Alternate,
    }
}

const fn chord(
    key: Key,
    primary_modifier: bool,
    alt: bool,
    shift: bool,
    label: &'static str,
) -> ShortcutChord {
    ShortcutChord::new(key, primary_modifier, alt, shift, label)
}

const NONE: &[ShortcutBinding] = &[];
const PROJECT_LAUNCHER: &[ShortcutBinding] = &[primary(
    chord(Key::O, true, false, true, "Ctrl+Shift+O"),
    ALL,
)];
const OPEN_PROJECT: &[ShortcutBinding] = &[
    primary(chord(Key::O, true, false, false, "Ctrl+O"), DESKTOP),
    alternate(chord(Key::O, true, true, false, "Ctrl+Alt+O"), HOST),
];
const NEW_PROJECT: &[ShortcutBinding] = &[
    primary(chord(Key::N, true, false, true, "Ctrl+Shift+N"), DESKTOP),
    alternate(chord(Key::N, true, true, true, "Ctrl+Alt+Shift+N"), HOST),
];
const SAVE_PROJECT: &[ShortcutBinding] = &[
    primary(chord(Key::S, true, false, false, "Ctrl+S"), DESKTOP),
    alternate(chord(Key::S, true, true, false, "Ctrl+Alt+S"), HOST),
];
const SAVE_PROJECT_AS: &[ShortcutBinding] = &[primary(
    chord(Key::S, true, true, true, "Ctrl+Shift+Alt+S"),
    ALL,
)];
const SAVE_ALL: &[ShortcutBinding] = &[primary(
    chord(Key::S, true, false, true, "Ctrl+Shift+S"),
    ALL,
)];
const CLOSE_PROJECT: &[ShortcutBinding] = &[primary(
    chord(Key::W, true, false, true, "Ctrl+Shift+W"),
    DESKTOP,
)];
const CHECK_AND_SAVE: &[ShortcutBinding] = &[primary(
    chord(Key::E, true, false, true, "Ctrl+Shift+E"),
    ALL,
)];

impl ProjectCommand {
    /// Every command of the feature.
    pub(crate) const ALL: [Self; 19] = [
        Self::ProjectLauncher,
        Self::RecentProjects,
        Self::OpenProject,
        Self::NewProject,
        Self::Save,
        Self::SaveAs,
        Self::SaveAll,
        Self::CloseProject,
        Self::NewCell,
        Self::NewView,
        Self::OpenSchematic,
        Self::Page(ProjectPage::Overview),
        Self::Page(ProjectPage::Library),
        Self::Page(ProjectPage::Configuration),
        Self::Page(ProjectPage::Dependencies),
        Self::Page(ProjectPage::Recovery),
        Self::DesignManagement,
        Self::RevisionHistory,
        Self::CheckAndSave,
    ];

    /// The key a shortcut profile names this command by.
    pub(crate) const fn key(self) -> CommandKey {
        match self {
            Self::ProjectLauncher => ids::PROJECT_LAUNCHER,
            Self::RecentProjects => CommandKey::new("recent-projects"),
            Self::OpenProject => CommandKey::new("open-project"),
            Self::NewProject => ids::NEW_PROJECT,
            Self::Save => ids::SAVE,
            Self::SaveAs => CommandKey::new("save-project-as"),
            Self::SaveAll => CommandKey::new("save-all"),
            Self::CloseProject => CommandKey::new("close-project"),
            Self::NewCell => ids::NEW_CELL,
            Self::NewView => CommandKey::new("new-view"),
            Self::OpenSchematic => CommandKey::new("open-schematic"),
            Self::Page(ProjectPage::Overview) => CommandKey::new("project-overview"),
            Self::Page(ProjectPage::Library) => CommandKey::new("project-library"),
            Self::Page(ProjectPage::Configuration) => {
                CommandKey::new("project-testbench-configuration")
            }
            Self::Page(ProjectPage::Dependencies) => CommandKey::new("project-dependencies"),
            Self::Page(ProjectPage::Recovery) => ids::PROJECT_RECOVERY,
            Self::DesignManagement => CommandKey::new("design-management"),
            Self::RevisionHistory => CommandKey::new("revision-history"),
            Self::CheckAndSave => CommandKey::new("check-and-save"),
        }
    }

    /// The command `key` names; `None` for a key the feature does not
    /// declare.
    pub(crate) fn from_key(key: CommandKey) -> Option<Self> {
        Self::ALL.into_iter().find(|command| command.key() == key)
    }

    /// What the command declares about itself. Every one is in the palette.
    /// The launcher and the file commands wait for a project operation in
    /// flight, and so does everything that writes the project; the commands
    /// that act on a project need one open, except Save, which also saves a
    /// standalone schematic, and the ones that open or start a project. The
    /// launcher, recent projects and the pages commit a pending inspector
    /// edit before they navigate, and the two design commands that edit the
    /// drawing are refused on a read-only one. Open project, New project and
    /// Save leave their primary chord to the browser.
    pub(crate) const fn declared_spec(self) -> CommandSpec {
        let waits =
            CommandFlags::PALETTE_VISIBLE.with(CommandFlags::BLOCKED_BY_PROJECT_OPERATION, true);
        let navigates = waits.with(CommandFlags::COMMITS_INSPECTOR_EDIT, true);
        let reserved = waits.with(CommandFlags::PRIMARY_RESERVED_BY_BROWSER, true);
        let open = waits.with(CommandFlags::REQUIRES_OPEN_PROJECT, true);
        let page = CommandFlags::PALETTE_VISIBLE
            .with(CommandFlags::REQUIRES_OPEN_PROJECT, true)
            .with(CommandFlags::COMMITS_INSPECTOR_EDIT, true);
        let edits = open
            .with(CommandFlags::COMMITS_INSPECTOR_EDIT, true)
            .with(CommandFlags::EDITS_ACTIVE_DOCUMENT, true);
        let global = ShortcutContext::Global;
        let design = ShortcutContext::DesignWorkspace;
        let (label, group, flags, context, bindings) = match self {
            Self::ProjectLauncher => (
                "Project launcher…",
                "File",
                navigates,
                global,
                PROJECT_LAUNCHER,
            ),
            Self::RecentProjects => ("Recent projects…", "File", navigates, global, NONE),
            Self::OpenProject => ("Open project…", "File", reserved, global, OPEN_PROJECT),
            Self::NewProject => ("New project…", "File", reserved, global, NEW_PROJECT),
            Self::Save => ("Save", "File", reserved, global, SAVE_PROJECT),
            Self::SaveAs => ("Save as…", "File", open, global, SAVE_PROJECT_AS),
            Self::SaveAll => ("Save all", "File", open, global, SAVE_ALL),
            Self::CloseProject => ("Close project…", "File", open, global, CLOSE_PROJECT),
            Self::NewCell => ("New cell…", "File", open, global, NONE),
            Self::NewView => ("New view…", "File", open, global, NONE),
            Self::OpenSchematic => ("Open schematic…", "File", waits, global, NONE),
            Self::Page(ProjectPage::Overview) => {
                ("Open project overview", "Project", page, global, NONE)
            }
            Self::Page(ProjectPage::Library) => {
                ("Open project library", "Project", page, global, NONE)
            }
            Self::Page(ProjectPage::Configuration) => (
                "Open testbench configuration",
                "Project",
                page,
                global,
                NONE,
            ),
            Self::Page(ProjectPage::Dependencies) => {
                ("Open dependency manifest", "Project", page, global, NONE)
            }
            Self::Page(ProjectPage::Recovery) => {
                ("Open recovery center", "Project", page, global, NONE)
            }
            Self::DesignManagement => (
                "Sheets, variants and annotation\u{2026}",
                "Design",
                edits,
                design,
                NONE,
            ),
            Self::RevisionHistory => ("Revision history\u{2026}", "Design", open, design, NONE),
            Self::CheckAndSave => ("Check and save", "Design", edits, design, CHECK_AND_SAVE),
        };
        CommandSpec {
            key: self.key(),
            label,
            group,
            flags,
            context,
            bindings,
            owner: CommandOwner::Project,
        }
    }
}

/// What the project feature hands the shell, which carries it out over the
/// whole application as the feature once did in place: the Configurations
/// command of the shell's own table, opening the selected view in the
/// workspace that edits it, using a configuration for runs, and what a
/// library catalog change left outside the model. Each place that raises one
/// raises one, carrying everything it once did after that point, in order.
/// Taken straight after the call that raised it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ProjectRequest {
    ConfigurationSets,
    OpenView(CellViewRef, ViewType),
    UseForRuns(ConfigurationSelection),
    /// What a catalog change left outside the model, then the change's own
    /// line in the log, in the order the change once wrote both.
    CatalogChanged {
        pruned: WorkspacePruned,
        notice: String,
    },
}

impl Request for ProjectRequest {
    const DRAIN: Drain = Drain::AfterContribution;
}

//! The commands the models feature declares, as the registry lists them: the
//! Models & PDKs workspace's pages, the device-model editor and its
//! workflows, and the PDK and model library paths.
//!
//! What each command does is the feature's own; what it declares here is all
//! the shell's order, menus, palette and shortcut profile read of it.

use crate::state::ModelsPage;
use crate::state::command_catalog::{CommandKey, ShortcutContext};
use crate::workbench::framework::command_registry::{CommandFlags, CommandOwner, CommandSpec, ids};

/// One command of the models feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelsCommand {
    /// Open the Models workspace on one of its pages.
    Page(ModelsPage),
    /// Create a project model: the device type first, then its card with the
    /// type's parameters at their defaults.
    ModelNew,
    ModelCreateProjectCopy,
    ModelEditor,
    ModelSaveRevision,
    ModelValidate,
    PdkSettings,
    RescanModelLibraries,
}

impl ModelsCommand {
    /// Every command of the feature.
    pub const ALL: [Self; 11] = [
        Self::Page(ModelsPage::Models),
        Self::Page(ModelsPage::Corners),
        Self::Page(ModelsPage::Bins),
        Self::Page(ModelsPage::Include),
        Self::ModelNew,
        Self::ModelCreateProjectCopy,
        Self::ModelEditor,
        Self::ModelSaveRevision,
        Self::ModelValidate,
        Self::PdkSettings,
        Self::RescanModelLibraries,
    ];

    /// The key a shortcut profile names this command by.
    pub const fn key(self) -> CommandKey {
        match self {
            Self::Page(ModelsPage::Models) => ids::MODELS_CATALOG,
            Self::Page(ModelsPage::Corners) => CommandKey::new("corner-sections"),
            Self::Page(ModelsPage::Bins) => CommandKey::new("model-bins"),
            Self::Page(ModelsPage::Include) => CommandKey::new("include-graph"),
            Self::ModelNew => CommandKey::new("model-new"),
            Self::ModelCreateProjectCopy => CommandKey::new("model-create-project-copy"),
            Self::ModelEditor => CommandKey::new("model-editor"),
            Self::ModelSaveRevision => CommandKey::new("model-save-revision"),
            Self::ModelValidate => CommandKey::new("model-validate"),
            Self::PdkSettings => ids::PDK_SETTINGS,
            Self::RescanModelLibraries => CommandKey::new("rescan-model-libraries"),
        }
    }

    /// The command `key` names; `None` for a key the feature does not
    /// declare.
    pub fn from_key(key: CommandKey) -> Option<Self> {
        Self::ALL.into_iter().find(|command| command.key() == key)
    }

    /// What the command declares about itself. Every one is in the palette,
    /// has no binding of its own and needs an open project. The pages commit
    /// a pending inspector edit before they navigate, and the commands that
    /// write the project or its model libraries wait for a project operation
    /// in flight.
    pub const fn declared_spec(self) -> CommandSpec {
        let page = CommandFlags::PALETTE_VISIBLE
            .with(CommandFlags::REQUIRES_OPEN_PROJECT, true)
            .with(CommandFlags::COMMITS_INSPECTOR_EDIT, true);
        let open = CommandFlags::PALETTE_VISIBLE.with(CommandFlags::REQUIRES_OPEN_PROJECT, true);
        let writes = open.with(CommandFlags::BLOCKED_BY_PROJECT_OPERATION, true);
        let (label, flags) = match self {
            // A page goes by the name its tab shows. "Library Manager" is the
            // Project workspace's Library page, as Virtuoso names it.
            Self::Page(ModelsPage::Models) => ("Models", page),
            Self::Page(ModelsPage::Corners) => ("Corners & sections", page),
            Self::Page(ModelsPage::Bins) => ("Bins & geometry", page),
            Self::Page(ModelsPage::Include) => ("Include graph", page),
            Self::ModelNew => ("New model…", writes),
            Self::ModelCreateProjectCopy => ("Create editable project copy", writes),
            Self::ModelEditor => ("Edit model…", writes),
            Self::ModelSaveRevision => ("Save model revision", writes),
            Self::ModelValidate => ("Validate model", open),
            Self::PdkSettings => ("PDK and model paths…", open),
            Self::RescanModelLibraries => ("Rescan model libraries", writes),
        };
        CommandSpec {
            key: self.key(),
            label,
            group: "Models",
            flags,
            context: ShortcutContext::Global,
            bindings: &[],
            owner: CommandOwner::Models,
        }
    }
}

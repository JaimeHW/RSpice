//! The commands the models feature declares, as the registry lists them: the
//! Models & PDKs workspace's pages, the model browser, the device-model
//! editor and its workflows, measurement correlation, and the PDK and model
//! library paths.
//!
//! What each command does is the feature's own; what it declares here is all
//! the shell's order, menus, palette and shortcut profile read of it.

use crate::state::ModelsPage;
use crate::state::command_catalog::{CommandKey, ShortcutContext};
use crate::workbench::framework::command_registry::{CommandFlags, CommandOwner, CommandSpec, ids};

/// One command of the models feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ModelsCommand {
    /// Open the Models workspace on one of its pages.
    Page(ModelsPage),
    ModelBrowser,
    ModelCreateProjectCopy,
    ModelEditor,
    ModelCorrelation,
    ModelSaveRevision,
    ModelValidate,
    ModelRunQualificationTests,
    ModelCompareRelease,
    PdkSettings,
    RescanModelLibraries,
}

impl ModelsCommand {
    /// Every command of the feature.
    pub(crate) const ALL: [Self; 16] = [
        Self::Page(ModelsPage::Models),
        Self::Page(ModelsPage::Symbols),
        Self::Page(ModelsPage::Corners),
        Self::Page(ModelsPage::Bins),
        Self::Page(ModelsPage::Include),
        Self::Page(ModelsPage::Qualification),
        Self::ModelBrowser,
        Self::ModelCreateProjectCopy,
        Self::ModelEditor,
        Self::ModelCorrelation,
        Self::ModelSaveRevision,
        Self::ModelValidate,
        Self::ModelRunQualificationTests,
        Self::ModelCompareRelease,
        Self::PdkSettings,
        Self::RescanModelLibraries,
    ];

    /// The key a shortcut profile names this command by.
    pub(crate) const fn key(self) -> CommandKey {
        match self {
            Self::Page(ModelsPage::Models) => ids::MODELS_CATALOG,
            Self::Page(ModelsPage::Symbols) => CommandKey::new("symbols-cdf"),
            Self::Page(ModelsPage::Corners) => CommandKey::new("corner-sections"),
            Self::Page(ModelsPage::Bins) => CommandKey::new("model-bins"),
            Self::Page(ModelsPage::Include) => CommandKey::new("include-graph"),
            Self::Page(ModelsPage::Qualification) => ids::MODEL_QUALIFICATION,
            Self::ModelBrowser => CommandKey::new("model-browser"),
            Self::ModelCreateProjectCopy => CommandKey::new("model-create-project-copy"),
            Self::ModelEditor => CommandKey::new("model-editor"),
            Self::ModelCorrelation => CommandKey::new("model-correlation"),
            Self::ModelSaveRevision => CommandKey::new("model-save-revision"),
            Self::ModelValidate => CommandKey::new("model-validate"),
            Self::ModelRunQualificationTests => CommandKey::new("model-run-tests"),
            Self::ModelCompareRelease => CommandKey::new("model-compare"),
            Self::PdkSettings => ids::PDK_SETTINGS,
            Self::RescanModelLibraries => CommandKey::new("rescan-model-libraries"),
        }
    }

    /// The command `key` names; `None` for a key the feature does not
    /// declare.
    pub(crate) fn from_key(key: CommandKey) -> Option<Self> {
        Self::ALL.into_iter().find(|command| command.key() == key)
    }

    /// What the command declares about itself. Every one is in the palette,
    /// has no binding of its own and needs an open project, except
    /// measurement correlation, which says why it cannot open instead. The
    /// pages commit a pending inspector edit before they navigate, and the
    /// commands that write the project or its model libraries wait for a
    /// project operation in flight.
    pub(crate) const fn declared_spec(self) -> CommandSpec {
        let page = CommandFlags::PALETTE_VISIBLE
            .with(CommandFlags::REQUIRES_OPEN_PROJECT, true)
            .with(CommandFlags::COMMITS_INSPECTOR_EDIT, true);
        let open = CommandFlags::PALETTE_VISIBLE.with(CommandFlags::REQUIRES_OPEN_PROJECT, true);
        let writes = open.with(CommandFlags::BLOCKED_BY_PROJECT_OPERATION, true);
        let (label, flags) = match self {
            Self::Page(ModelsPage::Models) => ("Model & library catalog", page),
            Self::Page(ModelsPage::Symbols) => ("Symbols & CDF", page),
            Self::Page(ModelsPage::Corners) => ("Corners & sections", page),
            Self::Page(ModelsPage::Bins) => ("Bins & geometry", page),
            Self::Page(ModelsPage::Include) => ("Include graph", page),
            Self::Page(ModelsPage::Qualification) => ("Model qualification", page),
            Self::ModelBrowser => ("Model browser…", open),
            Self::ModelCreateProjectCopy => ("Create editable project copy", writes),
            Self::ModelEditor => ("Device model and parameter editor…", writes),
            Self::ModelCorrelation => ("Measurement correlation…", CommandFlags::PALETTE_VISIBLE),
            Self::ModelSaveRevision => ("Save model revision", writes),
            Self::ModelValidate => ("Validate model", open),
            Self::ModelRunQualificationTests => ("Run qualification tests", writes),
            Self::ModelCompareRelease => ("Compare with release", open),
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

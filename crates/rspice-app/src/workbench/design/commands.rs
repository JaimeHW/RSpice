//! The commands the design feature declares, as the registry lists them:
//! so far the Stimulus Library's verbs.
//!
//! What each command does is the feature's own; what it declares here is all
//! the shell's order, menus, palette and shortcut profile read of it.

use egui::Key;

use crate::state::command_catalog::{
    CommandKey, CommandPlatform, ShortcutBinding, ShortcutChord, ShortcutContext, ShortcutKind,
};
use crate::workbench::framework::command_registry::{CommandFlags, CommandOwner, CommandSpec};

/// One command of the design feature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DesignCommand {
    /// Create a definition and select it.
    NewStimulusDefinition,
    /// Copy the selected definition into an editable duplicate.
    DuplicateStimulusDefinition,
    /// Remove the selected definition.
    DeleteStimulusDefinition,
    /// Publish the selected definition's draft.
    ApplyStimulusDraft,
    /// Throw the selected definition's draft away.
    RevertStimulusDraft,
    /// Audit every definition in the library.
    ValidateStimulusLibrary,
    /// Arm a placement with the selected definition's saved revision.
    PlaceSourceFromDefinition,
    /// Show the selected definition's first adopter on the drawing.
    ShowAdopterOnSchematic,
}

const DESKTOP: &[CommandPlatform] = &[CommandPlatform::Desktop];

const fn primary(chord: ShortcutChord, platforms: &'static [CommandPlatform]) -> ShortcutBinding {
    ShortcutBinding {
        chord,
        platforms,
        kind: ShortcutKind::Primary,
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
/// Publishing the draft the library instrument is editing.
///
/// Desktop only, because the browser spends `Ctrl+Enter` on Run — it has no
/// F5 to give that command — and a chord that meant "publish this definition"
/// on one platform and "start a simulation" on another would be the worst of
/// both.
const STIMULUS_APPLY_DRAFT: &[ShortcutBinding] = &[primary(
    chord(Key::Enter, true, false, false, "Ctrl+Enter"),
    DESKTOP,
)];

impl DesignCommand {
    /// Every command the registry lists, in the feature's own order.
    pub(crate) const ALL: [Self; 8] = [
        Self::NewStimulusDefinition,
        Self::DuplicateStimulusDefinition,
        Self::DeleteStimulusDefinition,
        Self::ApplyStimulusDraft,
        Self::RevertStimulusDraft,
        Self::ValidateStimulusLibrary,
        Self::PlaceSourceFromDefinition,
        Self::ShowAdopterOnSchematic,
    ];

    /// The key a shortcut profile names this command by.
    pub(crate) const fn key(self) -> CommandKey {
        match self {
            Self::NewStimulusDefinition => CommandKey::new("stimulus-new"),
            Self::DuplicateStimulusDefinition => CommandKey::new("stimulus-duplicate"),
            Self::DeleteStimulusDefinition => CommandKey::new("stimulus-delete"),
            Self::ApplyStimulusDraft => CommandKey::new("stimulus-apply"),
            Self::RevertStimulusDraft => CommandKey::new("stimulus-revert"),
            Self::ValidateStimulusLibrary => CommandKey::new("stimulus-validate"),
            Self::PlaceSourceFromDefinition => CommandKey::new("stimulus-place"),
            Self::ShowAdopterOnSchematic => CommandKey::new("stimulus-show-adopter"),
        }
    }

    /// What the command declares about itself. Every Stimulus Library verb
    /// is in the palette, needs an open project and keeps the global
    /// shortcut context; only Apply has a chord of its own.
    pub(crate) const fn declared_spec(self) -> CommandSpec {
        let open = CommandFlags::PALETTE_VISIBLE.with(CommandFlags::REQUIRES_OPEN_PROJECT, true);
        let (label, bindings) = match self {
            Self::NewStimulusDefinition => ("New stimulus definition", NONE),
            Self::DuplicateStimulusDefinition => ("Duplicate stimulus definition", NONE),
            Self::DeleteStimulusDefinition => ("Delete stimulus definition", NONE),
            Self::ApplyStimulusDraft => ("Apply stimulus draft", STIMULUS_APPLY_DRAFT),
            Self::RevertStimulusDraft => ("Revert stimulus draft", NONE),
            Self::ValidateStimulusLibrary => ("Validate stimulus library", NONE),
            Self::PlaceSourceFromDefinition => ("Place source from definition", NONE),
            Self::ShowAdopterOnSchematic => ("Show adopter on schematic", NONE),
        };
        CommandSpec {
            key: self.key(),
            label,
            group: "Stimulus",
            flags: open,
            context: ShortcutContext::Global,
            bindings,
            owner: CommandOwner::Design,
        }
    }
}

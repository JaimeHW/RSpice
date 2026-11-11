//! The commands the design feature declares, as the registry lists them:
//! so far the Stimulus Library's verbs, the dialogs the Design, Edit and
//! View menus open over the schematic, and the schematic's placements.
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
    /// Open Grid, snap and wire routing.
    GridSnapRouting,
    /// Open Display options.
    VisibilityOptions,
    /// Open the object table.
    EngineeringTableView,
    /// Open the connectivity and bus manager.
    ConnectivityManager,
    /// Open Bulk edit over the selection.
    SelectionBulkEdit,
    /// Open Design rule setup.
    RuleSetup,
    /// Open Review comments.
    ReviewComments,
    /// Rename the one selected object.
    RenameSelection,
    /// Open Replace instance on the selected instance.
    ReplaceInstance,
    /// Open Create hierarchy over the selection.
    CreateHierarchy,
    /// Pick an instance from the component shelf.
    PlaceInstance,
    /// Arm the wire tool.
    PlaceWire,
    /// Arm the bus tool.
    PlaceBus,
    /// Open Place bus tap.
    PlaceBusTap,
    /// Arm the junction tool.
    PlaceJunction,
    /// Arm the net label tool.
    PlaceLabel,
    /// Arm the off-sheet connector tool.
    PlaceOffSheetConnector,
    /// Arm the probe tool.
    PlaceProbe,
    /// Open Place text or note.
    PlaceText,
    /// Open Draw documentation shape.
    PlaceShape,
}

const EVERYWHERE: &[CommandPlatform] = &CommandPlatform::ALL;
const DESKTOP: &[CommandPlatform] = &[CommandPlatform::Desktop];
const DESKTOP_BROWSER: &[CommandPlatform] = &[CommandPlatform::Desktop, CommandPlatform::Browser];

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
const STIMULUS: &str = "Stimulus";
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
// The browser reopens a closed tab on Ctrl+Shift+T, so the key is the
// desktop's only.
const ENGINEERING_TABLE_VIEW: &[ShortcutBinding] = &[primary(
    chord(Key::T, true, false, true, "Ctrl+Shift+T"),
    DESKTOP,
)];
const RENAME_SELECTION: &[ShortcutBinding] = &[primary(
    chord(Key::F2, false, false, false, "F2"),
    DESKTOP_BROWSER,
)];
const PLACE_INSTANCE: &[ShortcutBinding] = &[primary(
    chord(Key::I, false, false, true, "Shift+I"),
    EVERYWHERE,
)];
const PLACE_WIRE: &[ShortcutBinding] =
    &[primary(chord(Key::W, false, false, false, "W"), EVERYWHERE)];
const PLACE_BUS: &[ShortcutBinding] =
    &[primary(chord(Key::B, false, false, false, "B"), EVERYWHERE)];
const PLACE_BUS_TAP: &[ShortcutBinding] =
    &[primary(chord(Key::T, false, false, false, "T"), EVERYWHERE)];
const PLACE_JUNCTION: &[ShortcutBinding] =
    &[primary(chord(Key::J, false, false, false, "J"), EVERYWHERE)];
const PLACE_LABEL: &[ShortcutBinding] =
    &[primary(chord(Key::N, false, false, false, "N"), EVERYWHERE)];
const PLACE_PROBE: &[ShortcutBinding] =
    &[primary(chord(Key::P, false, false, false, "P"), EVERYWHERE)];
const PLACE_TEXT: &[ShortcutBinding] = &[primary(
    chord(Key::T, false, false, true, "Shift+T"),
    EVERYWHERE,
)];

impl DesignCommand {
    /// Every command the registry lists, in the feature's own order.
    pub(crate) const ALL: [Self; 28] = [
        Self::NewStimulusDefinition,
        Self::DuplicateStimulusDefinition,
        Self::DeleteStimulusDefinition,
        Self::ApplyStimulusDraft,
        Self::RevertStimulusDraft,
        Self::ValidateStimulusLibrary,
        Self::PlaceSourceFromDefinition,
        Self::ShowAdopterOnSchematic,
        Self::GridSnapRouting,
        Self::VisibilityOptions,
        Self::EngineeringTableView,
        Self::ConnectivityManager,
        Self::SelectionBulkEdit,
        Self::RuleSetup,
        Self::ReviewComments,
        Self::RenameSelection,
        Self::ReplaceInstance,
        Self::CreateHierarchy,
        Self::PlaceInstance,
        Self::PlaceWire,
        Self::PlaceBus,
        Self::PlaceBusTap,
        Self::PlaceJunction,
        Self::PlaceLabel,
        Self::PlaceOffSheetConnector,
        Self::PlaceProbe,
        Self::PlaceText,
        Self::PlaceShape,
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
            Self::GridSnapRouting => CommandKey::new("grid-snap-routing"),
            Self::VisibilityOptions => CommandKey::new("visibility-options"),
            Self::EngineeringTableView => CommandKey::new("engineering-table-manager"),
            Self::ConnectivityManager => CommandKey::new("design-connectivity-tools"),
            Self::SelectionBulkEdit => CommandKey::new("design-bulk-tools"),
            Self::RuleSetup => CommandKey::new("design-rule-setup"),
            Self::ReviewComments => CommandKey::new("review-comments"),
            Self::RenameSelection => CommandKey::new("rename-selection"),
            Self::ReplaceInstance => CommandKey::new("replace-instance"),
            Self::CreateHierarchy => CommandKey::new("create-hierarchy"),
            Self::PlaceInstance => CommandKey::new("place-instance"),
            Self::PlaceWire => CommandKey::new("place-wire"),
            Self::PlaceBus => CommandKey::new("place-bus"),
            Self::PlaceBusTap => CommandKey::new("place-bus-tap"),
            Self::PlaceJunction => CommandKey::new("place-junction"),
            Self::PlaceLabel => CommandKey::new("place-label"),
            Self::PlaceOffSheetConnector => CommandKey::new("place-off-sheet-connector"),
            Self::PlaceProbe => CommandKey::new("place-probe"),
            Self::PlaceText => CommandKey::new("place-text"),
            Self::PlaceShape => CommandKey::new("place-shape"),
        }
    }

    /// Whether the command arms a placement on the schematic.
    pub(crate) const fn places(self) -> bool {
        matches!(
            self,
            Self::PlaceInstance
                | Self::PlaceWire
                | Self::PlaceBus
                | Self::PlaceBusTap
                | Self::PlaceJunction
                | Self::PlaceLabel
                | Self::PlaceOffSheetConnector
                | Self::PlaceProbe
                | Self::PlaceText
                | Self::PlaceShape
        )
    }

    /// The command a shortcut profile names by `key`, if the feature
    /// declares it.
    pub(crate) fn from_key(key: CommandKey) -> Option<Self> {
        Self::ALL.into_iter().find(|command| command.key() == key)
    }

    /// What the command declares about itself. Every Stimulus Library verb
    /// needs an open project and keeps the global shortcut context; only
    /// Apply has a chord of its own. The project dialogs also wait for a
    /// project operation, and every schematic dialog but the object table is
    /// a chord of the design workspace. Rename, Replace instance and Create
    /// hierarchy edit the drawing, so each commits the inspector's pending
    /// edit and is refused on a read-only schematic; only Rename has a chord.
    /// Every placement but the probe edits the drawing and is one Repeat
    /// last command runs again.
    pub(crate) const fn declared_spec(self) -> CommandSpec {
        let palette = CommandFlags::PALETTE_VISIBLE;
        let open = palette.with(CommandFlags::REQUIRES_OPEN_PROJECT, true);
        let project = open.with(CommandFlags::BLOCKED_BY_PROJECT_OPERATION, true);
        let global = ShortcutContext::Global;
        let design = ShortcutContext::DesignWorkspace;
        let edits = palette
            .with(CommandFlags::EDITS_ACTIVE_DOCUMENT, true)
            .with(CommandFlags::COMMITS_INSPECTOR_EDIT, true);
        let project_edits = project
            .with(CommandFlags::EDITS_ACTIVE_DOCUMENT, true)
            .with(CommandFlags::COMMITS_INSPECTOR_EDIT, true);
        let edit = ShortcutContext::EditContext;
        let canvas = ShortcutContext::EngineeringCanvas;
        let place = edits.with(CommandFlags::REPEATABLE, true);
        let (label, group, flags, context, bindings) = match self {
            Self::NewStimulusDefinition => {
                ("New stimulus definition", STIMULUS, open, global, NONE)
            }
            Self::DuplicateStimulusDefinition => (
                "Duplicate stimulus definition",
                STIMULUS,
                open,
                global,
                NONE,
            ),
            Self::DeleteStimulusDefinition => {
                ("Delete stimulus definition", STIMULUS, open, global, NONE)
            }
            Self::ApplyStimulusDraft => (
                "Apply stimulus draft",
                STIMULUS,
                open,
                global,
                STIMULUS_APPLY_DRAFT,
            ),
            Self::RevertStimulusDraft => ("Revert stimulus draft", STIMULUS, open, global, NONE),
            Self::ValidateStimulusLibrary => {
                ("Validate stimulus library", STIMULUS, open, global, NONE)
            }
            Self::PlaceSourceFromDefinition => {
                ("Place source from definition", STIMULUS, open, global, NONE)
            }
            Self::ShowAdopterOnSchematic => {
                ("Show adopter on schematic", STIMULUS, open, global, NONE)
            }
            Self::GridSnapRouting => (
                "Grid, snap and wire routing\u{2026}",
                "View",
                palette,
                design,
                NONE,
            ),
            Self::VisibilityOptions => ("Display options\u{2026}", "View", palette, design, NONE),
            Self::EngineeringTableView => (
                "Object table\u{2026}",
                "Design",
                palette,
                global,
                ENGINEERING_TABLE_VIEW,
            ),
            Self::ConnectivityManager => (
                "Connectivity and buses\u{2026}",
                "Design",
                project,
                design,
                NONE,
            ),
            Self::SelectionBulkEdit => ("Bulk edit\u{2026}", "Edit", project, design, NONE),
            Self::RuleSetup => ("Design rule setup\u{2026}", "Design", project, design, NONE),
            Self::ReviewComments => ("Review comments\u{2026}", "Design", project, design, NONE),
            Self::RenameSelection => ("Rename…", "Edit", edits, edit, RENAME_SELECTION),
            Self::ReplaceInstance => ("Replace instance\u{2026}", "Edit", edits, canvas, NONE),
            Self::CreateHierarchy => (
                "Create hierarchy\u{2026}",
                "Design",
                project_edits,
                canvas,
                NONE,
            ),
            Self::PlaceInstance => ("Place instance…", "Design", place, canvas, PLACE_INSTANCE),
            Self::PlaceWire => ("Draw wire", "Design", place, canvas, PLACE_WIRE),
            Self::PlaceBus => ("Draw bus", "Design", place, canvas, PLACE_BUS),
            Self::PlaceBusTap => ("Place bus tap", "Design", place, canvas, PLACE_BUS_TAP),
            Self::PlaceJunction => ("Place junction", "Design", place, canvas, PLACE_JUNCTION),
            Self::PlaceLabel => ("Place net label", "Design", place, canvas, PLACE_LABEL),
            // Every plausible mnemonic in the placement family is taken; the
            // toolbar and the palette are its routes.
            Self::PlaceOffSheetConnector => ("Off-sheet connector", "Design", place, canvas, NONE),
            Self::PlaceProbe => ("Place probe", "Design", palette, canvas, PLACE_PROBE),
            Self::PlaceText => (
                "Place text or note\u{2026}",
                "Design",
                place,
                canvas,
                PLACE_TEXT,
            ),
            Self::PlaceShape => (
                "Draw documentation shape\u{2026}",
                "Design",
                place,
                canvas,
                NONE,
            ),
        };
        CommandSpec {
            key: self.key(),
            label,
            group,
            flags,
            context,
            bindings,
            owner: CommandOwner::Design,
        }
    }
}

//! The commands the design feature declares, as the registry lists them:
//! so far the Stimulus Library's verbs, the dialogs the Design, Edit and
//! View menus open over the schematic, the schematic's placements, the
//! canvas's select tool, grid and Fit content, the moves up and down the
//! hierarchy, the design checks with their results' tour and clear, and
//! the schematic's annotations.
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
    /// Open Find and replace over the schematic.
    FindReplace,
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
    /// Open Create pins.
    PlacePin,
    /// Open Place text or note.
    PlaceText,
    /// Open Draw documentation shape.
    PlaceShape,
    /// Arm the resistor placement.
    PlaceResistor,
    /// Arm the capacitor placement.
    PlaceCapacitor,
    /// Arm the inductor placement.
    PlaceInductor,
    /// Arm the diode placement.
    PlaceDiode,
    /// Arm the ground placement.
    PlaceGround,
    /// Arm the voltage source placement.
    PlaceVoltageSource,
    /// Arm the current source placement.
    PlaceCurrentSource,
    /// Arm the select tool on the schematic or the symbol in front.
    SelectTool,
    /// Fit the schematic's content, rather than its drawing sheet, to the
    /// canvas.
    FitSchematicContent,
    /// Step the canvas grid on: the schematic's through its styles, the
    /// symbol editor's on or off.
    CycleGrid,
    /// Draw the schematic grid as dots.
    GridDots,
    /// Draw the schematic grid as lines.
    GridLines,
    /// Draw no schematic grid.
    GridOff,
    /// Return to the parent of the occurrence in front.
    AscendHierarchy,
    /// Open Descend with options on the selected instance.
    DescendHierarchy,
    /// Direct edit-in-place hierarchy gesture used by the toolbar and
    /// Shift+E. Descend with options above owns the explicit transaction.
    DescendHierarchyDirect,
    /// Withdraw the last check's results.
    ClearChecks,
    /// Step the finding tour forward.
    NextViolation,
    /// Step the finding tour back.
    PreviousViolation,
    /// Show or hide the DC operating point on the schematic.
    AnnotateOperatingPoint,
    /// Show or hide the transient's voltages at results cursor A on the
    /// schematic.
    AnnotateTransientVoltages,
    /// The open cellview's own findings: what an author runs while drawing.
    /// A symbol view checks its pins against the cell's interface.
    CheckCellview,
    /// The whole design, walked from the simulation root under its
    /// configuration, with every cellview of the project's own libraries the
    /// walk never reaches.
    CheckDesign,
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
// A browser keeps Ctrl+H for its history, so there the menu reaches it.
const FIND_REPLACE: &[ShortcutBinding] = &[primary(
    chord(Key::H, true, false, false, "Ctrl+H"),
    DESKTOP,
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
const PLACE_PIN: &[ShortcutBinding] = &[primary(
    chord(Key::P, false, false, true, "Shift+P"),
    EVERYWHERE,
)];
const PLACE_RESISTOR: &[ShortcutBinding] =
    &[primary(chord(Key::R, false, false, false, "R"), EVERYWHERE)];
const PLACE_CAPACITOR: &[ShortcutBinding] =
    &[primary(chord(Key::C, false, false, false, "C"), EVERYWHERE)];
const PLACE_INDUCTOR: &[ShortcutBinding] =
    &[primary(chord(Key::L, false, false, false, "L"), EVERYWHERE)];
const PLACE_DIODE: &[ShortcutBinding] =
    &[primary(chord(Key::D, false, false, false, "D"), EVERYWHERE)];
const PLACE_GROUND: &[ShortcutBinding] = &[primary(
    chord(Key::G, false, false, true, "Shift+G"),
    EVERYWHERE,
)];
const PLACE_VOLTAGE_SOURCE: &[ShortcutBinding] =
    &[primary(chord(Key::V, false, false, false, "V"), EVERYWHERE)];
const PLACE_CURRENT_SOURCE: &[ShortcutBinding] =
    &[primary(chord(Key::I, false, false, false, "I"), EVERYWHERE)];
const FIT_SCHEMATIC_CONTENT: &[ShortcutBinding] = &[primary(
    chord(Key::F, false, false, true, "Shift+F"),
    EVERYWHERE,
)];
const CYCLE_GRID: &[ShortcutBinding] =
    &[primary(chord(Key::G, false, false, false, "G"), EVERYWHERE)];
const ASCEND_HIERARCHY: &[ShortcutBinding] = &[primary(
    chord(Key::H, false, false, true, "Shift+H"),
    EVERYWHERE,
)];
const DESCEND_HIERARCHY: &[ShortcutBinding] =
    &[primary(chord(Key::H, false, false, false, "H"), EVERYWHERE)];
const DESCEND_HIERARCHY_DIRECT: &[ShortcutBinding] = &[primary(
    chord(Key::E, false, false, true, "Shift+E"),
    EVERYWHERE,
)];
const NEXT_VIOLATION: &[ShortcutBinding] = &[
    primary(
        chord(Key::CloseBracket, false, false, false, "]"),
        EVERYWHERE,
    ),
    alternate(chord(Key::F8, false, false, false, "F8"), EVERYWHERE),
];
const PREVIOUS_VIOLATION: &[ShortcutBinding] = &[
    primary(
        chord(Key::OpenBracket, false, false, false, "["),
        EVERYWHERE,
    ),
    alternate(chord(Key::F8, false, false, true, "Shift+F8"), EVERYWHERE),
];
const CHECK_CELLVIEW: &[ShortcutBinding] = &[primary(
    chord(Key::E, true, false, false, "Ctrl+E"),
    EVERYWHERE,
)];

impl DesignCommand {
    /// Every command the registry lists, in the feature's own order.
    pub(crate) const ALL: [Self; 53] = [
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
        Self::FindReplace,
        Self::PlaceInstance,
        Self::PlaceWire,
        Self::PlaceBus,
        Self::PlaceBusTap,
        Self::PlaceJunction,
        Self::PlaceLabel,
        Self::PlaceOffSheetConnector,
        Self::PlaceProbe,
        Self::PlacePin,
        Self::PlaceText,
        Self::PlaceShape,
        Self::PlaceResistor,
        Self::PlaceCapacitor,
        Self::PlaceInductor,
        Self::PlaceDiode,
        Self::PlaceGround,
        Self::PlaceVoltageSource,
        Self::PlaceCurrentSource,
        Self::SelectTool,
        Self::FitSchematicContent,
        Self::CycleGrid,
        Self::GridDots,
        Self::GridLines,
        Self::GridOff,
        Self::AscendHierarchy,
        Self::DescendHierarchy,
        Self::DescendHierarchyDirect,
        Self::ClearChecks,
        Self::NextViolation,
        Self::PreviousViolation,
        Self::AnnotateOperatingPoint,
        Self::AnnotateTransientVoltages,
        Self::CheckCellview,
        Self::CheckDesign,
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
            Self::FindReplace => CommandKey::new("find-replace"),
            Self::PlaceInstance => CommandKey::new("place-instance"),
            Self::PlaceWire => CommandKey::new("place-wire"),
            Self::PlaceBus => CommandKey::new("place-bus"),
            Self::PlaceBusTap => CommandKey::new("place-bus-tap"),
            Self::PlaceJunction => CommandKey::new("place-junction"),
            Self::PlaceLabel => CommandKey::new("place-label"),
            Self::PlaceOffSheetConnector => CommandKey::new("place-off-sheet-connector"),
            Self::PlaceProbe => CommandKey::new("place-probe"),
            Self::PlacePin => CommandKey::new("place-pin"),
            Self::PlaceText => CommandKey::new("place-text"),
            Self::PlaceShape => CommandKey::new("place-shape"),
            Self::PlaceResistor => CommandKey::new("place-resistor"),
            Self::PlaceCapacitor => CommandKey::new("place-capacitor"),
            Self::PlaceInductor => CommandKey::new("place-inductor"),
            Self::PlaceDiode => CommandKey::new("place-diode"),
            Self::PlaceGround => CommandKey::new("place-ground"),
            Self::PlaceVoltageSource => CommandKey::new("place-voltage-source"),
            Self::PlaceCurrentSource => CommandKey::new("place-current-source"),
            Self::SelectTool => CommandKey::new("select-tool"),
            Self::FitSchematicContent => CommandKey::new("canvas-fit-content"),
            Self::CycleGrid => CommandKey::new("cycle-grid"),
            Self::GridDots => CommandKey::new("grid-dots"),
            Self::GridLines => CommandKey::new("grid-lines"),
            Self::GridOff => CommandKey::new("grid-off"),
            Self::AscendHierarchy => CommandKey::new("ascend-hierarchy"),
            Self::DescendHierarchy => CommandKey::new("descend-hierarchy"),
            Self::DescendHierarchyDirect => CommandKey::new("descend-selected-instance"),
            Self::ClearChecks => CommandKey::new("clear-checks"),
            Self::NextViolation => CommandKey::new("next-violation"),
            Self::PreviousViolation => CommandKey::new("previous-violation"),
            Self::AnnotateOperatingPoint => CommandKey::new("annotate-operating-point"),
            Self::AnnotateTransientVoltages => CommandKey::new("annotate-transient-voltages"),
            Self::CheckCellview => CommandKey::new("check-cellview"),
            Self::CheckDesign => CommandKey::new("check-design"),
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
                | Self::PlacePin
                | Self::PlaceText
                | Self::PlaceShape
                | Self::PlaceResistor
                | Self::PlaceCapacitor
                | Self::PlaceInductor
                | Self::PlaceDiode
                | Self::PlaceGround
                | Self::PlaceVoltageSource
                | Self::PlaceCurrentSource
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
    /// a chord of the design workspace. Rename, Replace instance, Create
    /// hierarchy and Find and replace edit the drawing, so each commits the
    /// inspector's pending edit and is refused on a read-only schematic;
    /// Rename and Find and replace have chords. Find and replace also needs
    /// an open project and waits for a project operation.
    /// Every placement but the probe edits the drawing and is one Repeat
    /// last command runs again. The select tool, the grid and Fit content
    /// change only what the canvas shows, so none needs a project.
    /// Moving up or down the hierarchy commits the inspector's pending edit
    /// and is a chord of the design workspace; the direct descent needs an
    /// open project and stays out of the palette. The finding tour keeps
    /// the violation-navigation context, and clearing the checks and the
    /// annotation switches the global one. Both design checks commit the
    /// inspector's pending edit and are chords of the design workspace;
    /// checking the whole design also needs an open project.
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
        let commits = palette.with(CommandFlags::COMMITS_INSPECTOR_EDIT, true);
        let direct = CommandFlags::NONE
            .with(CommandFlags::REQUIRES_OPEN_PROJECT, true)
            .with(CommandFlags::COMMITS_INSPECTOR_EDIT, true);
        let violations = ShortcutContext::ViolationNavigation;
        let checks_project = open.with(CommandFlags::COMMITS_INSPECTOR_EDIT, true);
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
            Self::FindReplace => (
                "Find and replace\u{2026}",
                "Edit",
                project_edits,
                canvas,
                FIND_REPLACE,
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
            Self::PlacePin => ("Create pins\u{2026}", "Design", place, canvas, PLACE_PIN),
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
            Self::PlaceResistor => ("Place resistor", "Design", place, canvas, PLACE_RESISTOR),
            Self::PlaceCapacitor => ("Place capacitor", "Design", place, canvas, PLACE_CAPACITOR),
            Self::PlaceInductor => ("Place inductor", "Design", place, canvas, PLACE_INDUCTOR),
            Self::PlaceDiode => ("Place diode", "Design", place, canvas, PLACE_DIODE),
            Self::PlaceGround => ("Place ground", "Design", place, canvas, PLACE_GROUND),
            Self::PlaceVoltageSource => (
                "Place voltage source",
                "Design",
                place,
                canvas,
                PLACE_VOLTAGE_SOURCE,
            ),
            Self::PlaceCurrentSource => (
                "Place current source",
                "Design",
                place,
                canvas,
                PLACE_CURRENT_SOURCE,
            ),
            Self::SelectTool => ("Select tool", "Design", palette, edit, NONE),
            Self::FitSchematicContent => {
                ("Fit content", "View", palette, edit, FIT_SCHEMATIC_CONTENT)
            }
            Self::CycleGrid => ("Cycle grid display", "View", palette, canvas, CYCLE_GRID),
            Self::GridDots => ("Dot grid", "View", palette, canvas, NONE),
            Self::GridLines => ("Line grid", "View", palette, canvas, NONE),
            Self::GridOff => ("No grid", "View", palette, canvas, NONE),
            Self::AscendHierarchy => (
                "Return to parent",
                "Design",
                commits,
                design,
                ASCEND_HIERARCHY,
            ),
            // Two gestures, two rows: this one opens the descend dialog, and
            // the direct gesture below goes straight to the master. Sharing one
            // title made them indistinguishable everywhere the palette, the
            // shortcut editor and the command reference list them side by side.
            Self::DescendHierarchy => (
                "Descend with options\u{2026}",
                "Design",
                commits,
                design,
                DESCEND_HIERARCHY,
            ),
            Self::DescendHierarchyDirect => (
                "Descend",
                "Design",
                direct,
                design,
                DESCEND_HIERARCHY_DIRECT,
            ),
            Self::ClearChecks => ("Clear check results", "Design", palette, global, NONE),
            Self::NextViolation => (
                "Next violation",
                "Design",
                palette,
                violations,
                NEXT_VIOLATION,
            ),
            Self::PreviousViolation => (
                "Previous violation",
                "Design",
                palette,
                violations,
                PREVIOUS_VIOLATION,
            ),
            Self::AnnotateOperatingPoint => (
                "Annotate DC operating point",
                "Simulate",
                palette,
                global,
                NONE,
            ),
            Self::AnnotateTransientVoltages => (
                "Annotate transient voltages",
                "Simulate",
                palette,
                global,
                NONE,
            ),
            Self::CheckCellview => ("Check cellview", "Design", commits, design, CHECK_CELLVIEW),
            // Ctrl+Alt+E is AltGr+E, the euro sign, on most European
            // layouts, and every other chord of the check family is taken;
            // the Design menu, the toolbar and the palette are its routes.
            Self::CheckDesign => ("Check design", "Design", checks_project, design, NONE),
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

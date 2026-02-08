//! Keyboard Shortcuts Module
//!
//! Centralized keyboard shortcut handling for the application.
//! Provides a clean command-based interface for shortcut processing,
//! making shortcuts testable and maintainable.
//!
//! Follows commercial EDA conventions (particularly Cadence Virtuoso):
//! - R: Rotate
//! - H: Mirror Horizontal  
//! - Y: Mirror Vertical (since V is Voltage source)
//! - E: Edit Properties
//! - W: Wire tool
//! - S: Select tool
//! - etc.

use egui::{Context, Key, Modifiers};

// =============================================================================
// Shortcut Command
// =============================================================================

/// Commands that can be triggered by keyboard shortcuts
///
/// This enum provides a clean abstraction over keyboard input,
/// making the application logic independent of specific key bindings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShortcutCommand {
    // -------------------------------------------------------------------------
    // File Commands (require Ctrl)
    // -------------------------------------------------------------------------
    /// Create new schematic (Ctrl+N)
    FileNew,
    /// Open schematic file (Ctrl+O)
    FileOpen,
    /// Save schematic (Ctrl+S)
    FileSave,
    /// Save schematic as (Ctrl+Shift+S)
    FileSaveAs,
    /// Export schematic (Ctrl+E)
    FileExport,

    // -------------------------------------------------------------------------
    // Edit Commands (require Ctrl)
    // -------------------------------------------------------------------------
    /// Undo last action (Ctrl+Z)
    EditUndo,
    /// Redo last undone action (Ctrl+Y or Ctrl+Shift+Z)
    EditRedo,
    /// Copy selection (Ctrl+C)
    EditCopy,
    /// Paste clipboard (Ctrl+V)
    EditPaste,
    /// Cut selection (Ctrl+X)
    EditCut,
    /// Delete selection (Delete)
    EditDelete,
    /// Select all (Ctrl+A)
    EditSelectAll,
    /// Duplicate selection (Ctrl+D)
    EditDuplicate,

    // -------------------------------------------------------------------------
    // View Commands
    // -------------------------------------------------------------------------
    /// Zoom in (Ctrl+=)
    ViewZoomIn,
    /// Zoom out (Ctrl+-)
    ViewZoomOut,
    /// Zoom to fit (Ctrl+0)
    ViewZoomFit,
    /// Toggle project browser (Ctrl+Shift+L)
    ViewToggleBrowser,
    /// Toggle console (Ctrl+`)
    ViewToggleConsole,
    /// Toggle waveform viewer (Ctrl+W)
    ViewToggleWaveform,

    // -------------------------------------------------------------------------
    // Tool Selection (no modifiers, schematic mode only)
    // -------------------------------------------------------------------------
    /// Switch to select tool (S)
    ToolSelect,
    /// Switch to wire tool (W)
    ToolWire,
    /// Switch to probe tool (P)
    ToolProbe,

    // -------------------------------------------------------------------------
    // Component Placement (no modifiers, schematic mode only)
    // -------------------------------------------------------------------------
    /// Place resistor (R) - only when nothing selected
    PlaceResistor,
    /// Place capacitor (C)
    PlaceCapacitor,
    /// Place inductor (L)
    PlaceInductor,
    /// Place voltage source (V)
    PlaceVoltageSource,
    /// Place current source (I)
    PlaceCurrentSource,
    /// Place ground (G)
    PlaceGround,
    /// Place diode (D)
    PlaceDiode,
    /// Place NMOS transistor (M)
    PlaceNmos,
    /// Place NPN BJT (Q)
    PlaceNpnBjt,

    // -------------------------------------------------------------------------
    // Transform Commands (no modifiers, schematic mode only)
    // -------------------------------------------------------------------------
    /// Rotate selection/preview 90° CW (R) - when something selected
    Rotate,
    /// Mirror horizontal - flip about Y axis (H)
    MirrorHorizontal,
    /// Mirror vertical - flip about X axis (Y, since V is voltage source)
    MirrorVertical,

    // -------------------------------------------------------------------------
    // Property/Dialog Commands
    // -------------------------------------------------------------------------
    /// Edit properties of selection (E)
    EditProperties,
    /// Open simulation dialog (F5)
    OpenSimulationDialog,
    /// Run simulation (F9)
    RunSimulation,

    // -------------------------------------------------------------------------
    // General Commands
    // -------------------------------------------------------------------------
    /// Cancel current operation (Escape)
    Cancel,
    /// Show shortcuts help (F1)
    ShowShortcutsHelp,
}

impl ShortcutCommand {
    /// Get display name for the command
    pub fn display_name(self) -> &'static str {
        match self {
            Self::FileNew => "New",
            Self::FileOpen => "Open",
            Self::FileSave => "Save",
            Self::FileSaveAs => "Save As",
            Self::FileExport => "Export",
            Self::EditUndo => "Undo",
            Self::EditRedo => "Redo",
            Self::EditCopy => "Copy",
            Self::EditPaste => "Paste",
            Self::EditCut => "Cut",
            Self::EditDelete => "Delete",
            Self::EditSelectAll => "Select All",
            Self::EditDuplicate => "Duplicate",
            Self::ViewZoomIn => "Zoom In",
            Self::ViewZoomOut => "Zoom Out",
            Self::ViewZoomFit => "Zoom to Fit",
            Self::ViewToggleBrowser => "Toggle Browser",
            Self::ViewToggleConsole => "Toggle Console",
            Self::ViewToggleWaveform => "Toggle Waveform",
            Self::ToolSelect => "Select Tool",
            Self::ToolWire => "Wire Tool",
            Self::ToolProbe => "Probe Tool",
            Self::PlaceResistor => "Place Resistor",
            Self::PlaceCapacitor => "Place Capacitor",
            Self::PlaceInductor => "Place Inductor",
            Self::PlaceVoltageSource => "Place Voltage Source",
            Self::PlaceCurrentSource => "Place Current Source",
            Self::PlaceGround => "Place Ground",
            Self::PlaceDiode => "Place Diode",
            Self::PlaceNmos => "Place NMOS",
            Self::PlaceNpnBjt => "Place NPN BJT",
            Self::Rotate => "Rotate",
            Self::MirrorHorizontal => "Mirror Horizontal",
            Self::MirrorVertical => "Mirror Vertical",
            Self::EditProperties => "Edit Properties",
            Self::OpenSimulationDialog => "Simulation Setup",
            Self::RunSimulation => "Run Simulation",
            Self::Cancel => "Cancel",
            Self::ShowShortcutsHelp => "Shortcuts Help",
        }
    }

    /// Get the keyboard shortcut string for display
    pub fn shortcut_string(self) -> &'static str {
        match self {
            Self::FileNew => "Ctrl+N",
            Self::FileOpen => "Ctrl+O",
            Self::FileSave => "Ctrl+S",
            Self::FileSaveAs => "Ctrl+Shift+S",
            Self::FileExport => "Ctrl+E",
            Self::EditUndo => "Ctrl+Z",
            Self::EditRedo => "Ctrl+Y",
            Self::EditCopy => "Ctrl+C",
            Self::EditPaste => "Ctrl+V",
            Self::EditCut => "Ctrl+X",
            Self::EditDelete => "Delete",
            Self::EditSelectAll => "Ctrl+A",
            Self::EditDuplicate => "Ctrl+D",
            Self::ViewZoomIn => "Ctrl+=",
            Self::ViewZoomOut => "Ctrl+-",
            Self::ViewZoomFit => "Ctrl+0",
            Self::ViewToggleBrowser => "Ctrl+Shift+L",
            Self::ViewToggleConsole => "Ctrl+`",
            Self::ViewToggleWaveform => "Ctrl+W",
            Self::ToolSelect => "S",
            Self::ToolWire => "W",
            Self::ToolProbe => "P",
            Self::PlaceResistor => "R",
            Self::PlaceCapacitor => "C",
            Self::PlaceInductor => "L",
            Self::PlaceVoltageSource => "V",
            Self::PlaceCurrentSource => "I",
            Self::PlaceGround => "G",
            Self::PlaceDiode => "D",
            Self::PlaceNmos => "M",
            Self::PlaceNpnBjt => "Q",
            Self::Rotate => "R",
            Self::MirrorHorizontal => "H",
            Self::MirrorVertical => "Y",
            Self::EditProperties => "E",
            Self::OpenSimulationDialog => "F5",
            Self::RunSimulation => "F9",
            Self::Cancel => "Escape",
            Self::ShowShortcutsHelp => "F1",
        }
    }

    /// Check if this command modifies the schematic
    pub fn modifies_schematic(self) -> bool {
        matches!(
            self,
            Self::EditUndo
                | Self::EditRedo
                | Self::EditPaste
                | Self::EditCut
                | Self::EditDelete
                | Self::EditDuplicate
                | Self::Rotate
                | Self::MirrorHorizontal
                | Self::MirrorVertical
                | Self::PlaceResistor
                | Self::PlaceCapacitor
                | Self::PlaceInductor
                | Self::PlaceVoltageSource
                | Self::PlaceCurrentSource
                | Self::PlaceGround
                | Self::PlaceDiode
                | Self::PlaceNmos
                | Self::PlaceNpnBjt
        )
    }
}

// =============================================================================
// Shortcut Context
// =============================================================================

/// Context in which shortcuts are processed
///
/// Some shortcuts behave differently depending on context
/// (e.g., R places a resistor when nothing is selected,
/// but rotates when something is selected).
#[derive(Debug, Clone, Copy, Default)]
pub struct ShortcutContext {
    /// Whether a text input field has focus
    pub has_text_focus: bool,
    /// Whether any components/wires are selected
    pub has_selection: bool,
    /// Whether any dialog is currently open
    pub dialog_open: bool,
}

// =============================================================================
// Shortcut Processor
// =============================================================================

/// Process keyboard shortcuts and return triggered commands
///
/// This function checks the current keyboard state and returns
/// any shortcut command that should be executed.
///
/// # Arguments
/// * `ctx` - egui context for reading input
/// * `shortcut_ctx` - Application context for contextual shortcuts
///
/// # Returns
/// The triggered command, if any
pub fn process_shortcuts(ctx: &Context, shortcut_ctx: ShortcutContext) -> Option<ShortcutCommand> {
    // Don't process schematic shortcuts when typing in a text field
    let process_schematic_shortcuts = !shortcut_ctx.has_text_focus;

    // Check Ctrl shortcuts first (work regardless of text focus)
    if let Some(cmd) = process_ctrl_shortcuts(ctx) {
        return Some(cmd);
    }

    // Function key shortcuts (work regardless of focus)
    if let Some(cmd) = process_function_key_shortcuts(ctx) {
        return Some(cmd);
    }

    // Schematic shortcuts only when not in text field
    if process_schematic_shortcuts {
        if let Some(cmd) = process_schematic_shortcuts_impl(ctx, shortcut_ctx) {
            return Some(cmd);
        }
    }

    None
}

/// Process Ctrl+ shortcuts
fn process_ctrl_shortcuts(ctx: &Context) -> Option<ShortcutCommand> {
    ctx.input(|i| {
        if !i.modifiers.ctrl {
            return None;
        }

        // Ctrl+Shift combinations
        if i.modifiers.shift {
            if i.key_pressed(Key::S) {
                return Some(ShortcutCommand::FileSaveAs);
            }
            if i.key_pressed(Key::Z) {
                return Some(ShortcutCommand::EditRedo);
            }
            if i.key_pressed(Key::L) {
                return Some(ShortcutCommand::ViewToggleBrowser);
            }
        }

        // Ctrl combinations (no shift)
        if i.key_pressed(Key::N) {
            return Some(ShortcutCommand::FileNew);
        }
        if i.key_pressed(Key::O) {
            return Some(ShortcutCommand::FileOpen);
        }
        if i.key_pressed(Key::S) && !i.modifiers.shift {
            return Some(ShortcutCommand::FileSave);
        }
        if i.key_pressed(Key::Z) && !i.modifiers.shift {
            return Some(ShortcutCommand::EditUndo);
        }
        if i.key_pressed(Key::Y) {
            return Some(ShortcutCommand::EditRedo);
        }
        if i.key_pressed(Key::C) {
            return Some(ShortcutCommand::EditCopy);
        }
        if i.key_pressed(Key::V) {
            return Some(ShortcutCommand::EditPaste);
        }
        if i.key_pressed(Key::X) {
            return Some(ShortcutCommand::EditCut);
        }
        if i.key_pressed(Key::A) {
            return Some(ShortcutCommand::EditSelectAll);
        }
        if i.key_pressed(Key::D) {
            return Some(ShortcutCommand::EditDuplicate);
        }
        if i.key_pressed(Key::Backtick) {
            return Some(ShortcutCommand::ViewToggleConsole);
        }

        None
    })
}

/// Process function key shortcuts
fn process_function_key_shortcuts(ctx: &Context) -> Option<ShortcutCommand> {
    ctx.input(|i| {
        if i.key_pressed(Key::F1) {
            return Some(ShortcutCommand::ShowShortcutsHelp);
        }
        if i.key_pressed(Key::F5) {
            return Some(ShortcutCommand::OpenSimulationDialog);
        }
        if i.key_pressed(Key::F9) {
            return Some(ShortcutCommand::RunSimulation);
        }
        if i.key_pressed(Key::Escape) {
            return Some(ShortcutCommand::Cancel);
        }
        if i.key_pressed(Key::Delete) {
            return Some(ShortcutCommand::EditDelete);
        }
        None
    })
}

/// Process schematic-specific shortcuts (no modifiers)
fn process_schematic_shortcuts_impl(
    ctx: &Context,
    _shortcut_ctx: ShortcutContext,
) -> Option<ShortcutCommand> {
    ctx.input(|i| {
        // Skip if any modifier is pressed (except shift for some cases)
        if i.modifiers.ctrl || i.modifiers.alt {
            return None;
        }

        // Tool selection shortcuts
        if i.key_pressed(Key::S) {
            return Some(ShortcutCommand::ToolSelect);
        }
        if i.key_pressed(Key::W) {
            return Some(ShortcutCommand::ToolWire);
        }
        if i.key_pressed(Key::P) {
            return Some(ShortcutCommand::ToolProbe);
        }

        // R key: Rotate if selection, otherwise (could place resistor in future)
        if i.key_pressed(Key::R) {
            // Always rotate (R for resistor uses menu/toolbar)
            return Some(ShortcutCommand::Rotate);
        }

        // Transform shortcuts
        if i.key_pressed(Key::H) {
            return Some(ShortcutCommand::MirrorHorizontal);
        }
        if i.key_pressed(Key::Y) && !i.modifiers.ctrl {
            return Some(ShortcutCommand::MirrorVertical);
        }

        // Properties shortcut
        if i.key_pressed(Key::E) {
            return Some(ShortcutCommand::EditProperties);
        }

        // Component placement shortcuts
        if i.key_pressed(Key::G) {
            return Some(ShortcutCommand::PlaceGround);
        }
        if i.key_pressed(Key::V) {
            return Some(ShortcutCommand::PlaceVoltageSource);
        }
        if i.key_pressed(Key::I) {
            return Some(ShortcutCommand::PlaceCurrentSource);
        }
        if i.key_pressed(Key::C) {
            return Some(ShortcutCommand::PlaceCapacitor);
        }
        if i.key_pressed(Key::L) {
            return Some(ShortcutCommand::PlaceInductor);
        }
        if i.key_pressed(Key::D) {
            return Some(ShortcutCommand::PlaceDiode);
        }
        if i.key_pressed(Key::M) {
            return Some(ShortcutCommand::PlaceNmos);
        }
        if i.key_pressed(Key::Q) {
            return Some(ShortcutCommand::PlaceNpnBjt);
        }

        None
    })
}

// =============================================================================
// Shortcut Category (for help dialog organization)
// =============================================================================

/// Category of shortcuts for help dialog organization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortcutCategory {
    File,
    Edit,
    View,
    Tools,
    ComponentPlacement,
    Transform,
    Simulation,
    General,
}

impl ShortcutCategory {
    /// Get display name
    pub fn display_name(self) -> &'static str {
        match self {
            Self::File => "File",
            Self::Edit => "Edit",
            Self::View => "View",
            Self::Tools => "Tools",
            Self::ComponentPlacement => "Component Placement",
            Self::Transform => "Transform",
            Self::Simulation => "Simulation",
            Self::General => "General",
        }
    }

    /// Get all shortcuts in this category
    pub fn shortcuts(self) -> &'static [ShortcutCommand] {
        match self {
            Self::File => &[
                ShortcutCommand::FileNew,
                ShortcutCommand::FileOpen,
                ShortcutCommand::FileSave,
                ShortcutCommand::FileSaveAs,
                ShortcutCommand::FileExport,
            ],
            Self::Edit => &[
                ShortcutCommand::EditUndo,
                ShortcutCommand::EditRedo,
                ShortcutCommand::EditCopy,
                ShortcutCommand::EditPaste,
                ShortcutCommand::EditCut,
                ShortcutCommand::EditDelete,
                ShortcutCommand::EditSelectAll,
                ShortcutCommand::EditDuplicate,
            ],
            Self::View => &[
                ShortcutCommand::ViewZoomIn,
                ShortcutCommand::ViewZoomOut,
                ShortcutCommand::ViewZoomFit,
                ShortcutCommand::ViewToggleBrowser,
                ShortcutCommand::ViewToggleConsole,
                ShortcutCommand::ViewToggleWaveform,
            ],
            Self::Tools => &[
                ShortcutCommand::ToolSelect,
                ShortcutCommand::ToolWire,
                ShortcutCommand::ToolProbe,
            ],
            Self::ComponentPlacement => &[
                ShortcutCommand::PlaceGround,
                ShortcutCommand::PlaceVoltageSource,
                ShortcutCommand::PlaceCurrentSource,
                ShortcutCommand::PlaceCapacitor,
                ShortcutCommand::PlaceInductor,
                ShortcutCommand::PlaceDiode,
                ShortcutCommand::PlaceNmos,
                ShortcutCommand::PlaceNpnBjt,
            ],
            Self::Transform => &[
                ShortcutCommand::Rotate,
                ShortcutCommand::MirrorHorizontal,
                ShortcutCommand::MirrorVertical,
            ],
            Self::Simulation => &[
                ShortcutCommand::OpenSimulationDialog,
                ShortcutCommand::RunSimulation,
            ],
            Self::General => &[
                ShortcutCommand::EditProperties,
                ShortcutCommand::Cancel,
                ShortcutCommand::ShowShortcutsHelp,
            ],
        }
    }

    /// All categories in display order
    pub const ALL: [ShortcutCategory; 8] = [
        Self::File,
        Self::Edit,
        Self::View,
        Self::Tools,
        Self::ComponentPlacement,
        Self::Transform,
        Self::Simulation,
        Self::General,
    ];
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // ShortcutCommand Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_shortcut_command_display_names() {
        assert_eq!(ShortcutCommand::FileNew.display_name(), "New");
        assert_eq!(ShortcutCommand::EditUndo.display_name(), "Undo");
        assert_eq!(ShortcutCommand::ToolSelect.display_name(), "Select Tool");
        assert_eq!(ShortcutCommand::Rotate.display_name(), "Rotate");
    }

    #[test]
    fn test_shortcut_command_shortcut_strings() {
        assert_eq!(ShortcutCommand::FileNew.shortcut_string(), "Ctrl+N");
        assert_eq!(ShortcutCommand::EditCopy.shortcut_string(), "Ctrl+C");
        assert_eq!(ShortcutCommand::ToolWire.shortcut_string(), "W");
        assert_eq!(ShortcutCommand::Rotate.shortcut_string(), "R");
        assert_eq!(ShortcutCommand::Cancel.shortcut_string(), "Escape");
    }

    #[test]
    fn test_shortcut_command_modifies_schematic() {
        // Commands that modify schematic
        assert!(ShortcutCommand::EditDelete.modifies_schematic());
        assert!(ShortcutCommand::EditPaste.modifies_schematic());
        assert!(ShortcutCommand::Rotate.modifies_schematic());
        assert!(ShortcutCommand::PlaceGround.modifies_schematic());

        // Commands that don't modify schematic
        assert!(!ShortcutCommand::FileNew.modifies_schematic());
        assert!(!ShortcutCommand::EditCopy.modifies_schematic());
        assert!(!ShortcutCommand::ToolSelect.modifies_schematic());
        assert!(!ShortcutCommand::Cancel.modifies_schematic());
    }

    // -------------------------------------------------------------------------
    // ShortcutCategory Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_shortcut_category_display_names() {
        assert_eq!(ShortcutCategory::File.display_name(), "File");
        assert_eq!(ShortcutCategory::Edit.display_name(), "Edit");
        assert_eq!(
            ShortcutCategory::ComponentPlacement.display_name(),
            "Component Placement"
        );
    }

    #[test]
    fn test_shortcut_category_shortcuts_not_empty() {
        for category in ShortcutCategory::ALL {
            let shortcuts = category.shortcuts();
            assert!(
                !shortcuts.is_empty(),
                "Category {:?} has no shortcuts",
                category
            );
        }
    }

    #[test]
    fn test_shortcut_category_file_contains_expected() {
        let file_shortcuts = ShortcutCategory::File.shortcuts();
        assert!(file_shortcuts.contains(&ShortcutCommand::FileNew));
        assert!(file_shortcuts.contains(&ShortcutCommand::FileOpen));
        assert!(file_shortcuts.contains(&ShortcutCommand::FileSave));
    }

    #[test]
    fn test_shortcut_category_edit_contains_expected() {
        let edit_shortcuts = ShortcutCategory::Edit.shortcuts();
        assert!(edit_shortcuts.contains(&ShortcutCommand::EditUndo));
        assert!(edit_shortcuts.contains(&ShortcutCommand::EditRedo));
        assert!(edit_shortcuts.contains(&ShortcutCommand::EditCopy));
        assert!(edit_shortcuts.contains(&ShortcutCommand::EditPaste));
        assert!(edit_shortcuts.contains(&ShortcutCommand::EditDelete));
    }

    #[test]
    fn test_shortcut_category_transform_contains_expected() {
        let transform_shortcuts = ShortcutCategory::Transform.shortcuts();
        assert!(transform_shortcuts.contains(&ShortcutCommand::Rotate));
        assert!(transform_shortcuts.contains(&ShortcutCommand::MirrorHorizontal));
        assert!(transform_shortcuts.contains(&ShortcutCommand::MirrorVertical));
    }

    // -------------------------------------------------------------------------
    // ShortcutContext Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_shortcut_context_default() {
        let ctx = ShortcutContext::default();
        assert!(!ctx.has_text_focus);
        assert!(!ctx.has_selection);
        assert!(!ctx.dialog_open);
    }

    // -------------------------------------------------------------------------
    // All Categories Cover All Commands Test
    // -------------------------------------------------------------------------

    #[test]
    fn test_all_categories_cover_commands() {
        // Collect all commands from categories
        let mut covered: std::collections::HashSet<ShortcutCommand> =
            std::collections::HashSet::new();
        for category in ShortcutCategory::ALL {
            for cmd in category.shortcuts() {
                covered.insert(*cmd);
            }
        }

        // List of all commands that should be covered
        let all_commands = [
            ShortcutCommand::FileNew,
            ShortcutCommand::FileOpen,
            ShortcutCommand::FileSave,
            ShortcutCommand::FileSaveAs,
            ShortcutCommand::FileExport,
            ShortcutCommand::EditUndo,
            ShortcutCommand::EditRedo,
            ShortcutCommand::EditCopy,
            ShortcutCommand::EditPaste,
            ShortcutCommand::EditCut,
            ShortcutCommand::EditDelete,
            ShortcutCommand::EditSelectAll,
            ShortcutCommand::EditDuplicate,
            ShortcutCommand::ViewZoomIn,
            ShortcutCommand::ViewZoomOut,
            ShortcutCommand::ViewZoomFit,
            ShortcutCommand::ViewToggleBrowser,
            ShortcutCommand::ViewToggleConsole,
            ShortcutCommand::ViewToggleWaveform,
            ShortcutCommand::ToolSelect,
            ShortcutCommand::ToolWire,
            ShortcutCommand::ToolProbe,
            ShortcutCommand::PlaceGround,
            ShortcutCommand::PlaceVoltageSource,
            ShortcutCommand::PlaceCurrentSource,
            ShortcutCommand::PlaceCapacitor,
            ShortcutCommand::PlaceInductor,
            ShortcutCommand::PlaceDiode,
            ShortcutCommand::PlaceNmos,
            ShortcutCommand::PlaceNpnBjt,
            ShortcutCommand::Rotate,
            ShortcutCommand::MirrorHorizontal,
            ShortcutCommand::MirrorVertical,
            ShortcutCommand::EditProperties,
            ShortcutCommand::OpenSimulationDialog,
            ShortcutCommand::RunSimulation,
            ShortcutCommand::Cancel,
            ShortcutCommand::ShowShortcutsHelp,
        ];

        for cmd in all_commands {
            assert!(
                covered.contains(&cmd),
                "Command {:?} not covered by any category",
                cmd
            );
        }
    }
}

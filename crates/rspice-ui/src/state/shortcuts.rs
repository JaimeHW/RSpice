//! Keyboard Shortcuts System
//!
//! Professional-grade keyboard shortcut management for RSpice.
//! Follows industry patterns from Cadence Virtuoso and Keysight ADS.
//!
//! # Architecture
//!
//! The shortcut system consists of:
//! - `ShortcutAction`: Enumeration of all possible actions
//! - `KeyBinding`: A key + modifiers combination
//! - `ShortcutRegistry`: Maps key bindings to actions with customization support
//!
//! # Example
//!
//! ```ignore
//! use rspice_ui::state::shortcuts::{ShortcutAction, ShortcutRegistry};
//!
//! let registry = ShortcutRegistry::default();
//! let action = registry.get_action("KeyS", true, false, false);
//! assert_eq!(action, Some(ShortcutAction::Save));
//! ```

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//=============================================================================
// Shortcut Actions
//=============================================================================

/// All keyboard-triggerable actions in the application.
///
/// Organized by category following commercial EDA tool conventions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShortcutAction {
    //-------------------------------------------------------------------------
    // File Operations
    //-------------------------------------------------------------------------
    /// Create new schematic (Ctrl+N)
    NewSchematic,
    /// Open file (Ctrl+O)
    Open,
    /// Save current document (Ctrl+S)
    Save,
    /// Save as new file (Ctrl+Shift+S)
    SaveAs,
    /// Close current document (Ctrl+W)
    Close,

    //-------------------------------------------------------------------------
    // Edit Operations
    //-------------------------------------------------------------------------
    /// Undo last action (Ctrl+Z)
    Undo,
    /// Redo last undone action (Ctrl+Y or Ctrl+Shift+Z)
    Redo,
    /// Cut selection (Ctrl+X)
    Cut,
    /// Copy selection (Ctrl+C)
    Copy,
    /// Paste from clipboard (Ctrl+V)
    Paste,
    /// Delete selection (Del or Backspace)
    Delete,
    /// Select all (Ctrl+A)
    SelectAll,
    /// Duplicate selection (Ctrl+D)
    Duplicate,

    //-------------------------------------------------------------------------
    // View Operations
    //-------------------------------------------------------------------------
    /// Zoom in (Ctrl++ or Ctrl+=)
    ZoomIn,
    /// Zoom out (Ctrl+-)
    ZoomOut,
    /// Fit schematic to view (Space or Home)
    ZoomFit,
    /// Zoom to 100% (Ctrl+0)
    ZoomReset,
    /// Toggle grid visibility (G)
    ToggleGrid,
    /// Toggle waveform panel (Ctrl+W)
    ToggleWaveform,
    /// Toggle console panel (Ctrl+`)
    ToggleConsole,
    /// Toggle library browser (Ctrl+L)
    ToggleLibrary,

    //-------------------------------------------------------------------------
    // Schematic Tools
    //-------------------------------------------------------------------------
    /// Select/pointer tool (Esc or S)
    SelectTool,
    /// Wire drawing mode (W)
    WireTool,
    /// Component placement mode (C)
    ComponentTool,
    /// Net label placement (L)
    LabelTool,
    /// Text annotation (T)
    TextTool,
    /// Rotate component 90° CCW (R)
    Rotate,
    /// Mirror component horizontal (X)
    MirrorX,
    /// Mirror component vertical (Y)
    MirrorY,
    /// Edit component properties (E or Enter)
    EditProperties,

    //-------------------------------------------------------------------------
    // Simulation
    //-------------------------------------------------------------------------
    /// Run simulation (F5)
    RunSimulation,
    /// Stop simulation (Shift+F5)
    StopSimulation,
    /// Open simulation dialog (F2)
    SimulationDialog,
    /// Run DRC (Ctrl+D)
    RunDrc,

    //-------------------------------------------------------------------------
    // Navigation
    //-------------------------------------------------------------------------
    /// Find/search components (Ctrl+F)
    Find,
    /// Go to next search result (F3)
    FindNext,
    /// Go to previous search result (Shift+F3)
    FindPrevious,

    //-------------------------------------------------------------------------
    // Help
    //-------------------------------------------------------------------------
    /// Show keyboard shortcuts (Ctrl+/)
    ShowShortcuts,
    /// Show about dialog
    ShowAbout,
}

impl ShortcutAction {
    /// Get the display name for this action
    pub fn display_name(&self) -> &'static str {
        match self {
            // File
            Self::NewSchematic => "New Schematic",
            Self::Open => "Open",
            Self::Save => "Save",
            Self::SaveAs => "Save As",
            Self::Close => "Close",
            // Edit
            Self::Undo => "Undo",
            Self::Redo => "Redo",
            Self::Cut => "Cut",
            Self::Copy => "Copy",
            Self::Paste => "Paste",
            Self::Delete => "Delete",
            Self::SelectAll => "Select All",
            Self::Duplicate => "Duplicate",
            // View
            Self::ZoomIn => "Zoom In",
            Self::ZoomOut => "Zoom Out",
            Self::ZoomFit => "Zoom to Fit",
            Self::ZoomReset => "Zoom 100%",
            Self::ToggleGrid => "Toggle Grid",
            Self::ToggleWaveform => "Toggle Waveform",
            Self::ToggleConsole => "Toggle Console",
            Self::ToggleLibrary => "Toggle Library",
            // Tools
            Self::SelectTool => "Select Tool",
            Self::WireTool => "Wire Tool",
            Self::ComponentTool => "Component Tool",
            Self::LabelTool => "Label Tool",
            Self::TextTool => "Text Tool",
            Self::Rotate => "Rotate",
            Self::MirrorX => "Mirror Horizontal",
            Self::MirrorY => "Mirror Vertical",
            Self::EditProperties => "Edit Properties",
            // Simulation
            Self::RunSimulation => "Run Simulation",
            Self::StopSimulation => "Stop Simulation",
            Self::SimulationDialog => "Simulation Setup",
            Self::RunDrc => "Run DRC",
            // Navigation
            Self::Find => "Find",
            Self::FindNext => "Find Next",
            Self::FindPrevious => "Find Previous",
            // Help
            Self::ShowShortcuts => "Keyboard Shortcuts",
            Self::ShowAbout => "About",
        }
    }

    /// Get the category for this action (for grouping in UI)
    pub fn category(&self) -> ShortcutCategory {
        match self {
            Self::NewSchematic | Self::Open | Self::Save | Self::SaveAs | Self::Close => {
                ShortcutCategory::File
            }
            Self::Undo
            | Self::Redo
            | Self::Cut
            | Self::Copy
            | Self::Paste
            | Self::Delete
            | Self::SelectAll
            | Self::Duplicate => ShortcutCategory::Edit,
            Self::ZoomIn
            | Self::ZoomOut
            | Self::ZoomFit
            | Self::ZoomReset
            | Self::ToggleGrid
            | Self::ToggleWaveform
            | Self::ToggleConsole
            | Self::ToggleLibrary => ShortcutCategory::View,
            Self::SelectTool
            | Self::WireTool
            | Self::ComponentTool
            | Self::LabelTool
            | Self::TextTool
            | Self::Rotate
            | Self::MirrorX
            | Self::MirrorY
            | Self::EditProperties => ShortcutCategory::Tools,
            Self::RunSimulation | Self::StopSimulation | Self::SimulationDialog | Self::RunDrc => {
                ShortcutCategory::Simulation
            }
            Self::Find | Self::FindNext | Self::FindPrevious => ShortcutCategory::Navigation,
            Self::ShowShortcuts | Self::ShowAbout => ShortcutCategory::Help,
        }
    }
}

/// Category for grouping shortcuts in the help dialog
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShortcutCategory {
    File,
    Edit,
    View,
    Tools,
    Simulation,
    Navigation,
    Help,
}

impl ShortcutCategory {
    /// Get display name for the category
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::File => "File",
            Self::Edit => "Edit",
            Self::View => "View",
            Self::Tools => "Tools",
            Self::Simulation => "Simulation",
            Self::Navigation => "Navigation",
            Self::Help => "Help",
        }
    }

    /// Get all categories in display order
    pub fn all() -> &'static [ShortcutCategory] {
        &[
            Self::File,
            Self::Edit,
            Self::View,
            Self::Tools,
            Self::Simulation,
            Self::Navigation,
            Self::Help,
        ]
    }
}

//=============================================================================
// Key Bindings
//=============================================================================

/// A keyboard key binding (key code + modifiers).
///
/// Uses Web KeyboardEvent.code format for cross-platform compatibility.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyBinding {
    /// The key code (e.g., "KeyS", "F5", "Escape")
    pub key: String,
    /// Ctrl (or Cmd on macOS)
    pub ctrl: bool,
    /// Shift
    pub shift: bool,
    /// Alt (or Option on macOS)
    pub alt: bool,
}

impl KeyBinding {
    /// Create a new key binding
    pub fn new(key: impl Into<String>, ctrl: bool, shift: bool, alt: bool) -> Self {
        Self {
            key: key.into(),
            ctrl,
            shift,
            alt,
        }
    }

    /// Create a simple key binding (no modifiers)
    pub fn key(key: impl Into<String>) -> Self {
        Self::new(key, false, false, false)
    }

    /// Create a Ctrl+key binding
    pub fn ctrl(key: impl Into<String>) -> Self {
        Self::new(key, true, false, false)
    }

    /// Create a Ctrl+Shift+key binding
    pub fn ctrl_shift(key: impl Into<String>) -> Self {
        Self::new(key, true, true, false)
    }

    /// Create a Shift+key binding
    pub fn shift(key: impl Into<String>) -> Self {
        Self::new(key, false, true, false)
    }

    /// Get a human-readable display string
    pub fn display(&self) -> String {
        let mut parts = Vec::new();
        if self.ctrl {
            parts.push("Ctrl");
        }
        if self.shift {
            parts.push("Shift");
        }
        if self.alt {
            parts.push("Alt");
        }
        parts.push(self.key_display());
        parts.join("+")
    }

    /// Get display name for the key
    fn key_display(&self) -> &str {
        // Convert key codes to display names
        match self.key.as_str() {
            "KeyA" => "A",
            "KeyB" => "B",
            "KeyC" => "C",
            "KeyD" => "D",
            "KeyE" => "E",
            "KeyF" => "F",
            "KeyG" => "G",
            "KeyH" => "H",
            "KeyI" => "I",
            "KeyJ" => "J",
            "KeyK" => "K",
            "KeyL" => "L",
            "KeyM" => "M",
            "KeyN" => "N",
            "KeyO" => "O",
            "KeyP" => "P",
            "KeyQ" => "Q",
            "KeyR" => "R",
            "KeyS" => "S",
            "KeyT" => "T",
            "KeyU" => "U",
            "KeyV" => "V",
            "KeyW" => "W",
            "KeyX" => "X",
            "KeyY" => "Y",
            "KeyZ" => "Z",
            "Digit0" => "0",
            "Digit1" => "1",
            "Digit2" => "2",
            "Digit3" => "3",
            "Digit4" => "4",
            "Digit5" => "5",
            "Digit6" => "6",
            "Digit7" => "7",
            "Digit8" => "8",
            "Digit9" => "9",
            "F1" => "F1",
            "F2" => "F2",
            "F3" => "F3",
            "F4" => "F4",
            "F5" => "F5",
            "F6" => "F6",
            "F7" => "F7",
            "F8" => "F8",
            "F9" => "F9",
            "F10" => "F10",
            "F11" => "F11",
            "F12" => "F12",
            "Escape" => "Esc",
            "Space" => "Space",
            "Enter" => "Enter",
            "Backspace" => "Backspace",
            "Delete" => "Del",
            "Home" => "Home",
            "End" => "End",
            "PageUp" => "PgUp",
            "PageDown" => "PgDn",
            "ArrowUp" => "↑",
            "ArrowDown" => "↓",
            "ArrowLeft" => "←",
            "ArrowRight" => "→",
            "Tab" => "Tab",
            "Minus" => "-",
            "Equal" => "=",
            "Backquote" => "`",
            "Slash" => "/",
            other => other,
        }
    }
}

//=============================================================================
// Shortcut Registry
//=============================================================================

/// Global shortcut registry managing all key bindings.
///
/// Supports customization and serialization for user preferences.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutRegistry {
    /// Primary bindings: action -> key binding
    bindings: HashMap<ShortcutAction, Vec<KeyBinding>>,
    /// Reverse lookup: key binding -> action
    #[serde(skip)]
    reverse_lookup: HashMap<KeyBinding, ShortcutAction>,
}

impl Default for ShortcutRegistry {
    fn default() -> Self {
        let mut registry = Self {
            bindings: HashMap::new(),
            reverse_lookup: HashMap::new(),
        };
        registry.load_defaults();
        registry.rebuild_reverse_lookup();
        registry
    }
}

impl ShortcutRegistry {
    /// Create a new empty registry
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            reverse_lookup: HashMap::new(),
        }
    }

    /// Load default key bindings following industry conventions
    fn load_defaults(&mut self) {
        use ShortcutAction::*;

        // File operations
        self.set_binding(NewSchematic, KeyBinding::ctrl("KeyN"));
        self.set_binding(Open, KeyBinding::ctrl("KeyO"));
        self.set_binding(Save, KeyBinding::ctrl("KeyS"));
        self.set_binding(SaveAs, KeyBinding::ctrl_shift("KeyS"));
        self.set_binding(Close, KeyBinding::ctrl("KeyW"));

        // Edit operations
        self.set_binding(Undo, KeyBinding::ctrl("KeyZ"));
        self.set_binding(Redo, KeyBinding::ctrl("KeyY"));
        self.add_binding(Redo, KeyBinding::ctrl_shift("KeyZ")); // Alternative
        self.set_binding(Cut, KeyBinding::ctrl("KeyX"));
        self.set_binding(Copy, KeyBinding::ctrl("KeyC"));
        self.set_binding(Paste, KeyBinding::ctrl("KeyV"));
        self.set_binding(Delete, KeyBinding::key("Delete"));
        self.add_binding(Delete, KeyBinding::key("Backspace"));
        self.set_binding(SelectAll, KeyBinding::ctrl("KeyA"));
        self.set_binding(Duplicate, KeyBinding::ctrl("KeyD"));

        // View operations
        self.set_binding(ZoomIn, KeyBinding::ctrl("Equal"));
        self.add_binding(ZoomIn, KeyBinding::ctrl("Minus").with_key("Equal")); // Numpad
        self.set_binding(ZoomOut, KeyBinding::ctrl("Minus"));
        self.set_binding(ZoomFit, KeyBinding::key("Space"));
        self.add_binding(ZoomFit, KeyBinding::key("Home"));
        self.set_binding(ZoomReset, KeyBinding::ctrl("Digit0"));
        self.set_binding(ToggleGrid, KeyBinding::key("KeyG"));
        self.set_binding(ToggleConsole, KeyBinding::ctrl("Backquote"));
        self.set_binding(ToggleLibrary, KeyBinding::ctrl("KeyL"));

        // Schematic tools (single key shortcuts for efficiency)
        self.set_binding(SelectTool, KeyBinding::key("Escape"));
        self.add_binding(SelectTool, KeyBinding::key("KeyS"));
        self.set_binding(WireTool, KeyBinding::key("KeyW"));
        self.set_binding(ComponentTool, KeyBinding::key("KeyC"));
        self.set_binding(LabelTool, KeyBinding::key("KeyL"));
        self.set_binding(TextTool, KeyBinding::key("KeyT"));
        self.set_binding(Rotate, KeyBinding::key("KeyR"));
        self.set_binding(MirrorX, KeyBinding::key("KeyX"));
        self.set_binding(MirrorY, KeyBinding::key("KeyY"));
        self.set_binding(EditProperties, KeyBinding::key("KeyE"));
        self.add_binding(EditProperties, KeyBinding::key("Enter"));

        // Simulation
        self.set_binding(RunSimulation, KeyBinding::key("F5"));
        self.set_binding(StopSimulation, KeyBinding::shift("F5"));
        self.set_binding(SimulationDialog, KeyBinding::key("F2"));
        self.set_binding(RunDrc, KeyBinding::ctrl("KeyD"));

        // Navigation
        self.set_binding(Find, KeyBinding::ctrl("KeyF"));
        self.set_binding(FindNext, KeyBinding::key("F3"));
        self.set_binding(FindPrevious, KeyBinding::shift("F3"));

        // Help
        self.set_binding(ShowShortcuts, KeyBinding::ctrl("Slash"));
        self.set_binding(ShowAbout, KeyBinding::key("F1"));
    }

    /// Set the primary binding for an action (replaces existing)
    pub fn set_binding(&mut self, action: ShortcutAction, binding: KeyBinding) {
        self.bindings.insert(action, vec![binding]);
    }

    /// Add an additional binding for an action
    pub fn add_binding(&mut self, action: ShortcutAction, binding: KeyBinding) {
        self.bindings.entry(action).or_default().push(binding);
    }

    /// Get all bindings for an action
    pub fn get_bindings(&self, action: ShortcutAction) -> &[KeyBinding] {
        self.bindings
            .get(&action)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get the primary (first) binding for an action
    pub fn get_primary_binding(&self, action: ShortcutAction) -> Option<&KeyBinding> {
        self.bindings.get(&action).and_then(|v| v.first())
    }

    /// Look up action from key event
    pub fn get_action(
        &self,
        key: &str,
        ctrl: bool,
        shift: bool,
        alt: bool,
    ) -> Option<ShortcutAction> {
        let binding = KeyBinding::new(key, ctrl, shift, alt);
        self.reverse_lookup.get(&binding).copied()
    }

    /// Rebuild the reverse lookup table
    pub fn rebuild_reverse_lookup(&mut self) {
        self.reverse_lookup.clear();
        for (action, bindings) in &self.bindings {
            for binding in bindings {
                self.reverse_lookup.insert(binding.clone(), *action);
            }
        }
    }

    /// Get all actions grouped by category
    pub fn actions_by_category(
        &self,
    ) -> Vec<(ShortcutCategory, Vec<(ShortcutAction, &[KeyBinding])>)> {
        let mut result = Vec::new();

        for category in ShortcutCategory::all() {
            let actions: Vec<_> = self
                .bindings
                .iter()
                .filter(|(action, _)| action.category() == *category)
                .map(|(action, bindings)| (*action, bindings.as_slice()))
                .collect();

            if !actions.is_empty() {
                result.push((*category, actions));
            }
        }

        result
    }

    /// Check if a key event matches any shortcut (for preventing default browser behavior)
    pub fn is_shortcut(&self, key: &str, ctrl: bool, shift: bool, alt: bool) -> bool {
        self.get_action(key, ctrl, shift, alt).is_some()
    }
}

impl KeyBinding {
    /// Create a new binding with a different key (for building alternatives)
    fn with_key(mut self, key: impl Into<String>) -> Self {
        self.key = key.into();
        self
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_registry_has_all_standard_shortcuts() {
        let registry = ShortcutRegistry::default();

        // File operations
        assert_eq!(
            registry.get_action("KeyS", true, false, false),
            Some(ShortcutAction::Save)
        );
        assert_eq!(
            registry.get_action("KeyO", true, false, false),
            Some(ShortcutAction::Open)
        );
        assert_eq!(
            registry.get_action("KeyN", true, false, false),
            Some(ShortcutAction::NewSchematic)
        );

        // Edit operations
        assert_eq!(
            registry.get_action("KeyZ", true, false, false),
            Some(ShortcutAction::Undo)
        );
        assert_eq!(
            registry.get_action("KeyY", true, false, false),
            Some(ShortcutAction::Redo)
        );
        assert_eq!(
            registry.get_action("KeyC", true, false, false),
            Some(ShortcutAction::Copy)
        );
        assert_eq!(
            registry.get_action("KeyV", true, false, false),
            Some(ShortcutAction::Paste)
        );
        assert_eq!(
            registry.get_action("KeyX", true, false, false),
            Some(ShortcutAction::Cut)
        );

        // Delete has two bindings
        assert_eq!(
            registry.get_action("Delete", false, false, false),
            Some(ShortcutAction::Delete)
        );
        assert_eq!(
            registry.get_action("Backspace", false, false, false),
            Some(ShortcutAction::Delete)
        );
    }

    #[test]
    fn test_schematic_tool_shortcuts() {
        let registry = ShortcutRegistry::default();

        // Single-key shortcuts for efficiency
        assert_eq!(
            registry.get_action("KeyW", false, false, false),
            Some(ShortcutAction::WireTool)
        );
        assert_eq!(
            registry.get_action("KeyR", false, false, false),
            Some(ShortcutAction::Rotate)
        );
        assert_eq!(
            registry.get_action("Escape", false, false, false),
            Some(ShortcutAction::SelectTool)
        );
    }

    #[test]
    fn test_simulation_shortcuts() {
        let registry = ShortcutRegistry::default();

        assert_eq!(
            registry.get_action("F5", false, false, false),
            Some(ShortcutAction::RunSimulation)
        );
        assert_eq!(
            registry.get_action("F5", false, true, false),
            Some(ShortcutAction::StopSimulation)
        );
        assert_eq!(
            registry.get_action("F2", false, false, false),
            Some(ShortcutAction::SimulationDialog)
        );
    }

    #[test]
    fn test_alternative_bindings() {
        let registry = ShortcutRegistry::default();

        // Redo has two bindings: Ctrl+Y and Ctrl+Shift+Z
        assert_eq!(
            registry.get_action("KeyY", true, false, false),
            Some(ShortcutAction::Redo)
        );
        assert_eq!(
            registry.get_action("KeyZ", true, true, false),
            Some(ShortcutAction::Redo)
        );

        // ZoomFit has Space and Home
        assert_eq!(
            registry.get_action("Space", false, false, false),
            Some(ShortcutAction::ZoomFit)
        );
        assert_eq!(
            registry.get_action("Home", false, false, false),
            Some(ShortcutAction::ZoomFit)
        );
    }

    #[test]
    fn test_unknown_key_returns_none() {
        let registry = ShortcutRegistry::default();

        assert_eq!(registry.get_action("KeyQ", false, false, false), None);
        assert_eq!(registry.get_action("F12", false, false, false), None);
    }

    #[test]
    fn test_modifier_mismatch_returns_none() {
        let registry = ShortcutRegistry::default();

        // Ctrl+S is Save, but just S alone should not be Save
        assert_eq!(
            registry.get_action("KeyS", false, false, false),
            Some(ShortcutAction::SelectTool) // S alone is select tool
        );

        // W alone is wire tool, but Ctrl+W is close
        assert_eq!(
            registry.get_action("KeyW", true, false, false),
            Some(ShortcutAction::Close)
        );
        assert_eq!(
            registry.get_action("KeyW", false, false, false),
            Some(ShortcutAction::WireTool)
        );
    }

    #[test]
    fn test_key_binding_display() {
        assert_eq!(KeyBinding::ctrl("KeyS").display(), "Ctrl+S");
        assert_eq!(KeyBinding::ctrl_shift("KeyS").display(), "Ctrl+Shift+S");
        assert_eq!(KeyBinding::key("F5").display(), "F5");
        assert_eq!(KeyBinding::shift("F5").display(), "Shift+F5");
        assert_eq!(KeyBinding::key("Delete").display(), "Del");
        assert_eq!(KeyBinding::key("Escape").display(), "Esc");
    }

    #[test]
    fn test_action_display_name() {
        assert_eq!(ShortcutAction::Save.display_name(), "Save");
        assert_eq!(
            ShortcutAction::RunSimulation.display_name(),
            "Run Simulation"
        );
        assert_eq!(ShortcutAction::WireTool.display_name(), "Wire Tool");
    }

    #[test]
    fn test_action_category() {
        assert_eq!(ShortcutAction::Save.category(), ShortcutCategory::File);
        assert_eq!(ShortcutAction::Undo.category(), ShortcutCategory::Edit);
        assert_eq!(ShortcutAction::ZoomIn.category(), ShortcutCategory::View);
        assert_eq!(ShortcutAction::WireTool.category(), ShortcutCategory::Tools);
        assert_eq!(
            ShortcutAction::RunSimulation.category(),
            ShortcutCategory::Simulation
        );
        assert_eq!(
            ShortcutAction::Find.category(),
            ShortcutCategory::Navigation
        );
        assert_eq!(
            ShortcutAction::ShowShortcuts.category(),
            ShortcutCategory::Help
        );
    }

    #[test]
    fn test_actions_by_category() {
        let registry = ShortcutRegistry::default();
        let grouped = registry.actions_by_category();

        // Should have all categories
        assert_eq!(grouped.len(), 7);

        // Check that File category has expected actions
        let file_actions: Vec<_> = grouped
            .iter()
            .find(|(cat, _)| *cat == ShortcutCategory::File)
            .map(|(_, actions)| actions.iter().map(|(a, _)| *a).collect())
            .unwrap_or_default();

        assert!(file_actions.contains(&ShortcutAction::Save));
        assert!(file_actions.contains(&ShortcutAction::Open));
        assert!(file_actions.contains(&ShortcutAction::NewSchematic));
    }

    #[test]
    fn test_custom_binding() {
        let mut registry = ShortcutRegistry::new();

        // Add a custom binding
        registry.set_binding(ShortcutAction::Save, KeyBinding::ctrl("KeyQ"));
        registry.rebuild_reverse_lookup();

        assert_eq!(
            registry.get_action("KeyQ", true, false, false),
            Some(ShortcutAction::Save)
        );

        // Original Ctrl+S should not work
        assert_eq!(registry.get_action("KeyS", true, false, false), None);
    }

    #[test]
    fn test_is_shortcut() {
        let registry = ShortcutRegistry::default();

        assert!(registry.is_shortcut("KeyS", true, false, false)); // Ctrl+S
        assert!(registry.is_shortcut("F5", false, false, false)); // F5
        assert!(!registry.is_shortcut("KeyQ", false, false, false)); // Q alone is not mapped
    }

    #[test]
    fn test_serialization() {
        let registry = ShortcutRegistry::default();

        // Serialize to JSON
        let json = serde_json::to_string(&registry).expect("serialization failed");

        // Deserialize back
        let mut restored: ShortcutRegistry =
            serde_json::from_str(&json).expect("deserialization failed");
        restored.rebuild_reverse_lookup();

        // Should have same behavior
        assert_eq!(
            restored.get_action("KeyS", true, false, false),
            Some(ShortcutAction::Save)
        );
    }

    #[test]
    fn test_get_primary_binding() {
        let registry = ShortcutRegistry::default();

        // Redo has multiple bindings, primary should be Ctrl+Y
        let primary = registry.get_primary_binding(ShortcutAction::Redo);
        assert!(primary.is_some());
        assert_eq!(primary.unwrap().display(), "Ctrl+Y");
    }

    #[test]
    fn test_category_display_names() {
        assert_eq!(ShortcutCategory::File.display_name(), "File");
        assert_eq!(ShortcutCategory::Simulation.display_name(), "Simulation");
    }

    #[test]
    fn test_all_categories_returns_complete_list() {
        let all = ShortcutCategory::all();
        assert_eq!(all.len(), 7);
        assert!(all.contains(&ShortcutCategory::File));
        assert!(all.contains(&ShortcutCategory::Help));
    }
}

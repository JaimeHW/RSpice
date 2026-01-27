//! Dialog Visibility State
//!
//! Single-responsibility module for tracking which dialogs are open.
//! Separated from dialog content/parameters for cleaner architecture.

use serde::{Deserialize, Serialize};

// =============================================================================
// Dialog Visibility
// =============================================================================

/// Tracks which dialogs are currently open
///
/// This struct ONLY manages visibility (open/closed state).
/// Dialog content and parameters are managed by AnalysisDialogState.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DialogVisibility {
    /// Simulation setup dialog (main analysis configuration)
    pub simulation_dialog: bool,

    /// Simulation options dialog (solver settings)
    pub simulation_options: bool,

    /// About dialog (application info)
    pub about: bool,

    /// Preferences dialog (user settings)
    pub preferences: bool,

    /// Shortcuts help dialog (keyboard shortcuts reference)
    pub shortcuts_help: bool,

    /// Component properties dialog
    pub properties: bool,

    /// Design variables dialog
    pub design_variables: bool,

    /// Netlist preview dialog
    pub netlist_preview: bool,

    /// Export dialog (PDF, image, etc.)
    pub export: bool,

    /// DRC results dialog
    pub drc_results: bool,
}

impl DialogVisibility {
    /// Create with all dialogs closed
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any dialog is currently open
    pub fn any_open(&self) -> bool {
        self.simulation_dialog
            || self.simulation_options
            || self.about
            || self.preferences
            || self.shortcuts_help
            || self.properties
            || self.design_variables
            || self.netlist_preview
            || self.export
            || self.drc_results
    }

    /// Close all dialogs
    pub fn close_all(&mut self) {
        self.simulation_dialog = false;
        self.simulation_options = false;
        self.about = false;
        self.preferences = false;
        self.shortcuts_help = false;
        self.properties = false;
        self.design_variables = false;
        self.netlist_preview = false;
        self.export = false;
        self.drc_results = false;
    }

    /// Toggle a dialog's visibility, optionally closing others first
    pub fn toggle_exclusive(&mut self, dialog: DialogKind) {
        self.close_all();
        self.set(dialog, true);
    }

    /// Set a specific dialog's visibility
    pub fn set(&mut self, dialog: DialogKind, open: bool) {
        match dialog {
            DialogKind::Simulation => self.simulation_dialog = open,
            DialogKind::SimulationOptions => self.simulation_options = open,
            DialogKind::About => self.about = open,
            DialogKind::Preferences => self.preferences = open,
            DialogKind::ShortcutsHelp => self.shortcuts_help = open,
            DialogKind::Properties => self.properties = open,
            DialogKind::DesignVariables => self.design_variables = open,
            DialogKind::NetlistPreview => self.netlist_preview = open,
            DialogKind::Export => self.export = open,
            DialogKind::DrcResults => self.drc_results = open,
        }
    }

    /// Check if a specific dialog is open
    pub fn is_open(&self, dialog: DialogKind) -> bool {
        match dialog {
            DialogKind::Simulation => self.simulation_dialog,
            DialogKind::SimulationOptions => self.simulation_options,
            DialogKind::About => self.about,
            DialogKind::Preferences => self.preferences,
            DialogKind::ShortcutsHelp => self.shortcuts_help,
            DialogKind::Properties => self.properties,
            DialogKind::DesignVariables => self.design_variables,
            DialogKind::NetlistPreview => self.netlist_preview,
            DialogKind::Export => self.export,
            DialogKind::DrcResults => self.drc_results,
        }
    }
}

// =============================================================================
// Dialog Kind Enum
// =============================================================================

/// Enumeration of all dialog types
///
/// Used for programmatic dialog manipulation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DialogKind {
    /// Simulation setup dialog
    Simulation,
    /// Simulation options (solver settings)
    SimulationOptions,
    /// About dialog
    About,
    /// Preferences dialog
    Preferences,
    /// Keyboard shortcuts help
    ShortcutsHelp,
    /// Component properties
    Properties,
    /// Design variables
    DesignVariables,
    /// Netlist preview
    NetlistPreview,
    /// Export dialog
    Export,
    /// DRC results
    DrcResults,
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visibility_default_all_closed() {
        let vis = DialogVisibility::default();
        assert!(!vis.any_open());
        assert!(!vis.simulation_dialog);
        assert!(!vis.about);
        assert!(!vis.preferences);
    }

    #[test]
    fn test_visibility_any_open() {
        let mut vis = DialogVisibility::default();
        assert!(!vis.any_open());

        vis.simulation_dialog = true;
        assert!(vis.any_open());

        vis.simulation_dialog = false;
        vis.about = true;
        assert!(vis.any_open());
    }

    #[test]
    fn test_visibility_close_all() {
        let mut vis = DialogVisibility::default();
        vis.simulation_dialog = true;
        vis.simulation_options = true;
        vis.about = true;
        vis.preferences = true;
        vis.shortcuts_help = true;

        vis.close_all();

        assert!(!vis.any_open());
        assert!(!vis.simulation_dialog);
        assert!(!vis.simulation_options);
        assert!(!vis.about);
        assert!(!vis.preferences);
        assert!(!vis.shortcuts_help);
    }

    #[test]
    fn test_visibility_set_and_is_open() {
        let mut vis = DialogVisibility::default();

        vis.set(DialogKind::Simulation, true);
        assert!(vis.is_open(DialogKind::Simulation));
        assert!(!vis.is_open(DialogKind::About));

        vis.set(DialogKind::Simulation, false);
        assert!(!vis.is_open(DialogKind::Simulation));
    }

    #[test]
    fn test_visibility_toggle_exclusive() {
        let mut vis = DialogVisibility::default();
        vis.simulation_dialog = true;
        vis.about = true;

        vis.toggle_exclusive(DialogKind::Preferences);

        assert!(!vis.simulation_dialog);
        assert!(!vis.about);
        assert!(vis.preferences);
    }

    #[test]
    fn test_dialog_kind_all_variants() {
        // Ensure all variants can be used with set/is_open
        let mut vis = DialogVisibility::default();

        let all_kinds = [
            DialogKind::Simulation,
            DialogKind::SimulationOptions,
            DialogKind::About,
            DialogKind::Preferences,
            DialogKind::ShortcutsHelp,
            DialogKind::Properties,
            DialogKind::DesignVariables,
            DialogKind::NetlistPreview,
            DialogKind::Export,
            DialogKind::DrcResults,
        ];

        for kind in all_kinds {
            assert!(!vis.is_open(kind));
            vis.set(kind, true);
            assert!(vis.is_open(kind));
            vis.set(kind, false);
            assert!(!vis.is_open(kind));
        }
    }
}

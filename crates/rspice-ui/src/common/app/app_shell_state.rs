//! App Shell State Types
//!
//! Shared shell/panel/confirmation state for `RSpiceApp`.

/// Active tab in the unified bottom panel
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum BottomPanelTab {
    /// Log output and history
    #[default]
    Log,
    /// Waveform viewer / results
    Waveform,
    /// Automation / scripting console
    Automation,
}

impl BottomPanelTab {
    /// Display name for tab
    pub fn name(&self) -> &'static str {
        match self {
            Self::Log => "Log",
            Self::Waveform => "Waveform",
            Self::Automation => "Automation",
        }
    }

    /// All available tabs in display order
    pub fn all() -> &'static [BottomPanelTab] {
        &[Self::Log, Self::Waveform, Self::Automation]
    }
}

/// Panel visibility state
#[derive(Debug, Clone)]
pub struct PanelVisibility {
    /// Project browser (Library/Cell/View tree)
    pub project_browser: bool,
    /// Results browser (Simulation runs/analyses tree)
    pub results_browser: bool,
    /// Properties panel (right side)
    pub properties: bool,
    /// Unified bottom panel visible
    pub bottom_panel: bool,
    /// Active tab in bottom panel
    pub active_bottom_tab: BottomPanelTab,
    /// Cross-probe signal browser
    pub signal_browser: bool,
    /// Scripting/Automation console
    pub script_console: bool,
}

impl Default for PanelVisibility {
    fn default() -> Self {
        Self {
            project_browser: false,
            results_browser: false,
            properties: true,
            bottom_panel: true, // Visible by default with Log tab
            active_bottom_tab: BottomPanelTab::Log,
            signal_browser: false,
            script_console: false,
        }
    }
}

/// Resizable panel heights (in pixels)
#[derive(Debug, Clone)]
pub struct PanelSizes {
    /// Waveform panel height
    pub waveform_height: f32,
    /// Console panel height
    pub console_height: f32,
    /// Project browser width
    pub browser_width: f32,
    /// Properties panel width
    pub properties_width: f32,
}

impl Default for PanelSizes {
    fn default() -> Self {
        Self {
            waveform_height: 300.0,
            console_height: 120.0,
            browser_width: 220.0,
            properties_width: 250.0,
        }
    }
}

/// Actions that can trigger a save confirmation dialog
///
/// Commercial EDA tools like Cadence Virtuoso always prompt the user before
/// discarding unsaved work. This enum captures the pending action so it can
/// be executed after the user responds to the confirmation dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationAction {
    /// Create new schematic (discard current)
    FileNew,
    /// Open another schematic (discard current)
    FileOpen,
    /// Close the application
    Exit,
}

impl ConfirmationAction {
    /// Get the dialog title for this action
    pub fn dialog_title(&self) -> &'static str {
        match self {
            ConfirmationAction::FileNew => "Create New Schematic",
            ConfirmationAction::FileOpen => "Open Schematic",
            ConfirmationAction::Exit => "Exit RSpice",
        }
    }

    /// Get the prompt message for this action
    pub fn prompt_message(&self) -> &'static str {
        "The current schematic has unsaved changes.\nDo you want to save before continuing?"
    }
}

/// State for the save confirmation dialog
///
/// When visible is true, the confirmation dialog is shown. The pending_action
/// field stores what should happen after the user responds.
#[derive(Debug, Clone, Default)]
pub struct ConfirmationDialogState {
    /// Whether the dialog is currently visible
    pub visible: bool,
    /// The action pending user confirmation
    pub pending_action: Option<ConfirmationAction>,
}

impl ConfirmationDialogState {
    /// Open the confirmation dialog for a specific action
    pub fn show(&mut self, action: ConfirmationAction) {
        self.visible = true;
        self.pending_action = Some(action);
    }

    /// Close the dialog and clear pending action
    pub fn close(&mut self) {
        self.visible = false;
        self.pending_action = None;
    }

    /// Check if dialog is open for a specific action
    pub fn is_showing(&self, action: ConfirmationAction) -> bool {
        self.visible && self.pending_action == Some(action)
    }
}

/// User response to a save confirmation dialog
///
/// Standard Yes/No/Cancel pattern matching commercial EDA tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationResponse {
    /// Save changes and proceed with action
    Yes,
    /// Discard changes and proceed with action
    No,
    /// Cancel the action, keep changes
    Cancel,
}

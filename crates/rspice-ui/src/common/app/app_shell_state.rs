//! App Shell State Types
//!
//! Shared shell/panel/confirmation state for `RSpiceApp`.

/// Actions that can trigger a save confirmation dialog
///
/// Commercial EDA tools like Cadence Virtuoso always prompt the user before
/// discarding unsaved work. This enum captures the pending action so it can
/// be executed after the user responds to the confirmation dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationAction {
    /// Create new project (discard current workspace)
    ProjectNew,
    /// Open another project (discard current workspace)
    ProjectOpen,
    /// Create new schematic (discard current)
    FileNew,
    /// Open another schematic (discard current)
    FileOpen,
    /// Open a file from the recent-files list (path stored alongside the
    /// pending action in [`ConfirmationDialogState::pending_path`])
    OpenRecent,
    /// Load a bundled example circuit (name stored alongside the pending
    /// action in [`ConfirmationDialogState::pending_example`]).
    OpenExample,
    /// Import a SPICE deck into the Netlist workspace.
    ImportNetlist,
    /// Close the application
    Exit,
}

impl ConfirmationAction {
    /// Get the dialog title for this action
    pub fn dialog_title(&self) -> &'static str {
        match self {
            ConfirmationAction::ProjectNew => "Create New Project",
            ConfirmationAction::ProjectOpen => "Open Project",
            ConfirmationAction::FileNew => "Create New Schematic",
            ConfirmationAction::FileOpen => "Open Schematic",
            ConfirmationAction::OpenRecent => "Open Recent File",
            ConfirmationAction::OpenExample => "Open Example",
            ConfirmationAction::ImportNetlist => "Import SPICE Deck",
            ConfirmationAction::Exit => "Exit RSpice",
        }
    }

    /// Get the prompt message for this action
    pub fn prompt_message(&self) -> &'static str {
        match self {
            ConfirmationAction::ProjectNew | ConfirmationAction::ProjectOpen => {
                "The current project has unsaved changes.\nDo you want to save before continuing?"
            }
            ConfirmationAction::FileNew
            | ConfirmationAction::FileOpen
            | ConfirmationAction::OpenRecent
            | ConfirmationAction::OpenExample
            | ConfirmationAction::ImportNetlist
            | ConfirmationAction::Exit => {
                "The current design has unsaved changes.\nDo you want to save before continuing?"
            }
        }
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
    /// Target path for path-carrying actions ([`ConfirmationAction::OpenRecent`]).
    pub pending_path: Option<std::path::PathBuf>,
    /// Example name for [`ConfirmationAction::OpenExample`].
    pub pending_example: Option<String>,
}

impl ConfirmationDialogState {
    /// Open the confirmation dialog for a specific action
    pub fn show(&mut self, action: ConfirmationAction) {
        self.visible = true;
        self.pending_action = Some(action);
        self.pending_path = None;
        self.pending_example = None;
    }

    /// Open the confirmation dialog for an action that targets a known path.
    pub fn show_with_path(&mut self, action: ConfirmationAction, path: std::path::PathBuf) {
        self.visible = true;
        self.pending_action = Some(action);
        self.pending_path = Some(path);
        self.pending_example = None;
    }

    /// Open the confirmation dialog for an action that targets a bundled example.
    pub fn show_with_example(&mut self, action: ConfirmationAction, name: String) {
        self.visible = true;
        self.pending_action = Some(action);
        self.pending_path = None;
        self.pending_example = Some(name);
    }

    /// Close the dialog and clear pending action
    pub fn close(&mut self) {
        self.visible = false;
        self.pending_action = None;
        self.pending_path = None;
        self.pending_example = None;
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

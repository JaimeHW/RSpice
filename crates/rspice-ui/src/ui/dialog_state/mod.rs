//! Dialog State Module
//!
//! Properly separated state management for application dialogs.
//! Follows commercial EDA architecture with clear separation of concerns:
//!
//! - `visibility`: Dialog open/close state only
//! - `analysis`: Analysis-specific parameter states
//! - `interaction`: Runtime interaction state (drag, selection)
//!
//! This separation ensures:
//! - Single responsibility per module
//! - Easy testing of individual concerns
//! - Clean serialization boundaries

mod analysis;
mod interaction;
mod visibility;

pub use analysis::AnalysisDialogState;
pub use interaction::InteractionState;
pub use visibility::DialogVisibility;

// =============================================================================
// Unified DialogState
// =============================================================================

/// Complete dialog state container
///
/// This aggregates all dialog-related state into a single type for
/// convenient access from the application, while maintaining proper
/// separation of concerns internally.
#[derive(Debug, Clone, Default)]
pub struct DialogState {
    /// Which dialogs are currently open
    pub visibility: DialogVisibility,

    /// Analysis configuration parameters for all analysis types
    pub analysis: AnalysisDialogState,

    /// Runtime interaction state (drag, selection)
    pub interaction: InteractionState,
}

impl DialogState {
    /// Create a new dialog state with all defaults
    pub fn new() -> Self {
        Self::default()
    }

    /// Check if any dialog is currently open
    pub fn any_open(&self) -> bool {
        self.visibility.any_open()
    }

    /// Close all dialogs
    pub fn close_all(&mut self) {
        self.visibility.close_all();
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialog_state_default() {
        let state = DialogState::default();
        assert!(!state.any_open());
    }

    #[test]
    fn test_dialog_state_any_open() {
        let mut state = DialogState::default();
        assert!(!state.any_open());

        state.visibility.simulation_dialog = true;
        assert!(state.any_open());
    }

    #[test]
    fn test_dialog_state_close_all() {
        let mut state = DialogState::default();
        state.visibility.simulation_dialog = true;
        state.visibility.about = true;
        state.visibility.preferences = true;

        state.close_all();
        assert!(!state.any_open());
        assert!(!state.visibility.simulation_dialog);
        assert!(!state.visibility.about);
        assert!(!state.visibility.preferences);
    }
}

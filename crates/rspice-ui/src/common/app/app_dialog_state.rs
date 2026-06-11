//! App Dialog State
//!
//! Modal/dialog payload used by `AppState`. Analysis configuration lives
//! in `SimSetupState`, not here.

use super::ConfirmationDialogState;

/// A library deletion awaiting user confirmation.
#[derive(Debug, Clone)]
pub enum LibraryDeleteTarget {
    /// Delete a whole cell (with all its views).
    Cell {
        /// Owning library.
        library: String,
        /// Cell to delete.
        cell: String,
    },
    /// Delete a single view.
    View {
        /// Owning library.
        library: String,
        /// Owning cell.
        cell: String,
        /// View to delete.
        view: String,
    },
}

/// Dialog visibility state

#[derive(Debug, Clone, Default)]
pub struct DialogState {
    /// About dialog
    pub about: bool,
    /// Shortcuts help dialog
    pub shortcuts_help: bool,
    /// New Cell creation dialog
    pub new_cell_dialog: bool,
    /// New Cell name input
    pub new_cell_name: String,
    /// New Cell target library
    pub new_cell_library: String,
    /// New Cell description
    pub new_cell_description: String,
    /// New Cell view types to create
    pub new_cell_create_schematic: bool,
    /// Create symbol view for new cell
    pub new_cell_create_symbol: bool,
    /// Create testbench view for new cell
    pub new_cell_create_testbench: bool,
    /// New Cell validation error message
    pub new_cell_error: Option<String>,
    /// New View creation dialog
    pub new_view_dialog: bool,
    /// New View target library
    pub new_view_library: String,
    /// New View target cell
    pub new_view_cell: String,
    /// New View name input
    pub new_view_name: String,
    /// New View type selection
    pub new_view_type: crate::state::ViewType,
    /// New View validation error message
    pub new_view_error: Option<String>,
    /// Library deletion awaiting confirmation
    pub library_delete_confirm: Option<LibraryDeleteTarget>,

    /// Starting position of selection drag (grid coords)
    pub drag_start: Option<(i32, i32)>,
    /// Last drag position for computing delta (grid coords)
    pub last_drag_pos: Option<(i32, i32)>,

    /// DRC results (cached from last run; surfaced by the schematic view)
    pub drc_results: Option<crate::services::drc::DrcResult>,
    /// `topology_version` when the last check ran — canvas markers hide and
    /// the ERC pill reads "stale" once the design changes underneath them.
    pub drc_checked_version: u64,

    /// Waveform calculator dialog open
    pub waveform_calculator_dialog: bool,

    /// Preferences dialog open
    pub preferences_open: bool,

    /// Verilog-A model loading dialog state
    pub veriloga_dialog: crate::panels::VerilogALoadDialogState,

    /// Runtime interaction state for drag, hover, etc.
    pub interaction: super::InteractionState,

    /// State for save confirmation modal (unsaved changes warning)
    pub confirmation_dialog: ConfirmationDialogState,
}

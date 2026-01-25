//! Dialogs Module
//!
//! Domain-specific dialog components for the RSpice UI application.
//! These are separated from primitive UI components to improve organization.
//!
//! ## Modules
//!
//! - `about_dialog` - About/version information dialog
//! - `component_edit` - Component property editor dialog
//! - `confirm_modal` - Generic confirmation modal
//! - `model_editor` - Advanced device model editor dialog
//! - `model_types` - Device model types and parameter definitions
//! - `preferences_dialog` - User preferences configuration
//! - `resizable_dialog` - Reusable resizable dialog wrapper (commercial-grade)
//! - `search_dialog` - Find/search components dialog
//! - `shortcuts_help` - Keyboard shortcuts reference
//! - `simulation_dialog` - Tabbed simulation configuration dialog
//! - `simulation_options` - Simulation options configuration
//! - `veriloga_dialog` - Verilog-A module editor dialog
//! - `veriloga_inspector` - Verilog-A code inspector

pub mod about_dialog;
pub mod component_edit;
pub mod confirm_modal;
pub mod model_editor;
pub mod model_types;
pub mod preferences_dialog;
pub mod resizable_dialog;
pub mod search_dialog;
pub mod shortcuts_help;
pub mod simulation_dialog;
pub mod simulation_options;
pub mod veriloga_dialog;
pub mod veriloga_inspector;

// Re-export main dialog components
pub use about_dialog::AboutDialog;
pub use component_edit::ComponentEditModal;
pub use confirm_modal::{SaveDialogResult, UnsavedChangesModal};
pub use model_editor::ModelEditorState;
pub use model_types::{
    BjtModelType, DeviceModelType, DiodeModelType, ModelParameter, MosfetModelType,
    ParameterCategory, TechnologyNode,
};
pub use preferences_dialog::PreferencesDialog;
pub use resizable_dialog::{
    DialogSizeConstraints, ResizableDialog, ResizableDialogProps, ResizeEdge,
};
pub use search_dialog::{SearchDialog, SearchResult, SearchResultType, SearchType};
pub use shortcuts_help::ShortcutsHelpDialog;
pub use simulation_dialog::SimulationDialog;
pub use simulation_options::{
    AcOptions, AdvancedOptions, ConvergenceOptions, DcConvergenceAid, DcOptions, FrequencyScale,
    IntegrationMethod, MatrixSolver, OptionCategory, PivotStrategy, SimulationOptions,
    SimulationOptionsDialog, SimulationOptionsDialogProps, TransientOptions,
};
pub use veriloga_dialog::VerilogAImportDialog;
pub use veriloga_inspector::{ParameterInfo, VerilogAInspector, VerilogAModelInfo};
